#!/usr/bin/env python3
"""
V3 — максимальная минимизация.

Убирает per-chain unique content секции (они просто template с подстановкой имени сети),
оставляет только:
1. Общий шаблон
2. Компактную таблицу per-chain параметров
3. Только truly уникальный контент (который нельзя вывести из шаблона + имени сети)
"""

import os, re, json
from collections import defaultdict

BASE = "/home/gg/projects/agents_blockchain/docs/MVP"
CONSOLIDATED = os.path.join(BASE, "00-consolidated")
OUT = os.path.join(BASE, "00-consolidated-min")
os.makedirs(OUT, exist_ok=True)

def extract_chain_name(fp):
    basename = os.path.basename(fp).replace('.md', '')
    name = re.sub(r'^(reference|chains|data|wallets)--', '', basename)
    name = re.sub(r'(-api-quickstart|-api-faq|-api-overview)$', '', name)
    name = name.replace('--', '/').replace('-', ' ').title().strip()
    return name if name else basename

def find_chain_name_variations(files):
    """Extract actual chain-dependent variables from all source files."""
    chain_data = {}
    
    for fp in files:
        with open(fp, errors='ignore') as f:
            text = f.read()
        chain = extract_chain_name(fp)
        
        vars = {}
        
        # Title
        h1 = ""
        for line in text.split('\n'):
            if line.startswith('# '):
                h1 = line[2:].strip()
                break
        vars['title'] = h1
        
        # RPC URLs
        rpcs = re.findall(r'https?://[a-z0-9._-]+(?:\.alchemy\.com|\.g\.alchemy\.com)[^\s\n\'\"<>]*', text)
        if rpcs:
            vars['rpc_url'] = rpcs[0]
        
        # Chain IDs
        cids = re.findall(r'[Cc]hain\s*[Ii][Dd][:\s]*"?(\d+)"?', text)
        if not cids:
            cids = re.findall(r'chainId["\']?\s*[:=]\s*["\']?(\d+)', text)
        if cids:
            vars['chain_id'] = cids[0]
        
        # Native currency
        curr = re.findall(r'[Nn]ative\s+[Cc]urrency[^:]*:\s*(\w+)', text)
        if curr:
            vars['currency'] = curr[0]
        
        # Faucet URLs
        faucets = re.findall(r'(faucet|faucet_url)[:\s]+(https?://[^\s\n]+)', text, re.I)
        if not faucets:
            faucets = re.findall(r'faucet\.(?:alchemy|chain|link)[^\s\n>.]+', text, re.I)
            faucets = [(f, f'https://{f}') for f in faucets]
        if faucets:
            vars['faucet'] = faucets[0][1] if len(faucets[0]) > 1 else faucets[0][0]
        
        # Explorer
        explorers = re.findall(r'https://[a-z0-9.-]+\.(?:etherscan|explorer|blockscout|oklink|subscan)\.(?:io|com|network)[^\s\n\'\"<>]*', text)
        if explorers:
            vars['explorer'] = explorers[0]
        
        # Documentation
        docs = re.findall(r'\[(?:docs?|developer guide|documentation)\][^\]]*\]\((https?://[^)]+)\)', text, re.I)
        if docs:
            vars['docs'] = docs[0]
        
        # Get the count of paragraphs that only differ by chain name
        chain_lower = chain.lower()
        chain_words = chain_lower.split()
        
        lines = text.split('\n')
        truly_unique = []
        for line in lines:
            line_lower = line.lower()
            # Check if this line is just chain name substitution
            is_template = False
            for w in chain_words:
                if len(w) > 3 and w in line_lower:
                    # This line contains the chain name - it's template
                    is_template = True
                    break
            # If line doesn't contain chain name, might be truly unique
            if not is_template and line.strip():
                # Check if it appears in other files
                truly_unique.append(line.strip())
        
        vars['_unique_lines'] = len(truly_unique)
        vars['_total_lines'] = len(lines)
        
        chain_data[chain] = vars
    
    return chain_data

def generate_minimized_v3(consolidated_path, group_name, files):
    with open(consolidated_path) as f:
        cons_text = f.read()
    
    chains_match = re.search(r'> Applies to: (.+)', cons_text)
    chains = [c.strip() for c in chains_match.group(1).split(',')] if chains_match else []
    
    chain_data = find_chain_name_variations(files)
    
    # Common template
    common_match = re.search(r'## Common Template\n(.*?)(?=## Per-Chain Parameters|\Z)', cons_text, re.DOTALL)
    common = common_match.group(1).strip() if common_match else ""
    
    lines = []
    lines.append(f"# {group_name.replace('-', ' ').title()}")
    lines.append("")
    lines.append(f"**{len(chains)} chains** · template-based (1 doc replaces {len(files)} per-chain files)")
    lines.append("")
    
    # Common template
    if common:
        lines.append("## Common Template")
        lines.append("")
        lines.append(common)
        lines.append("")
    
    # Variables that are actually useful
    all_vars = {}
    for chain, data in chain_data.items():
        for k, v in data.items():
            if k.startswith('_') or not v:
                continue
            if k not in all_vars:
                all_vars[k] = {}
            all_vars[k][chain] = v
    
    # Only show variables found in 50%+ of chains
    threshold = max(2, len(chains) // 2)
    useful_vars = {k: v for k, v in all_vars.items() if len(v) >= threshold}
    
    if useful_vars:
        cols = list(useful_vars.keys())
        lines.append("## Per-Chain Parameters")
        lines.append("")
        lines.append("| Chain | " + " | ".join(c.replace('_', ' ').title() for c in cols) + " |")
        lines.append("|-------|" + "|".join("---" for _ in cols) + "|")
        for chain in chains:
            data = chain_data.get(chain, {})
            row = f"| {chain} |"
            for col in cols:
                val = data.get(col, '')
                sval = str(val)
                if len(sval) > 55:
                    sval = sval[:52] + '...'
                row += f" {sval} |"
            lines.append(row)
        lines.append("")
        
        # Save full data as JSON
        json_data = {
            'template_name': group_name,
            'chain_count': len(chains),
            'chains': chains,
            'per_chain': {c: chain_data.get(c, {}) for c in chains}
        }
        json_path = os.path.join(OUT, f"{group_name}_data.json")
        with open(json_path, 'w') as f:
            json.dump(json_data, f, indent=2, ensure_ascii=False)
    else:
        json_data = None
    
    # Truly unique content: only content that's NOT chain-name substitution
    # We'll just note which chains have extra unique lines
    lines.append("## Notes")
    lines.append("")
    lines.append(f"- Template applies to all {len(chains)} chains with per-chain variable substitution only")
    lines.append(f"- Per-chain values stored in `{group_name}_data.json`")
    
    unique_summary = {}
    for chain in chains:
        data = chain_data.get(chain, {})
        if data.get('_unique_lines', 0) > 5:
            unique_summary[chain] = data['_unique_lines']
    
    if unique_summary:
        lines.append(f"- Chains with notable unique content (beyond name substitution):")
        for chain, count in sorted(unique_summary.items(), key=lambda x: -x[1]):
            lines.append(f"  - {chain}: ~{count} unique lines")
    
    lines.append("")
    lines.append("---")
    lines.append(f"Source: {os.path.relpath(consolidated_path, BASE)}")
    lines.append("")
    
    return '\n'.join(lines), json_data


# Process each consolidated file
consolidated_files = [f for f in os.listdir(CONSOLIDATED) if f.endswith('.md')]
total_old = 0
total_new = 0

print("V3 — максимальная минимизация:\n")
for cf in sorted(consolidated_files):
    name = cf.replace('.md', '')
    src_path = os.path.join(CONSOLIDATED, cf)
    
    # Find source files for this group
    suffix_map = {
        'api-quickstart': '-api-quickstart.md',
        'api-faq': '-api-faq.md',
        'api-overview': '-api-overview.md',
        'utxo-websockets': 'utxo-websockets.md',
        'utxo-overview': 'utxo.md',
    }
    suffix = suffix_map.get(name)
    files = []
    if suffix:
        for root, dirs, fnames in os.walk(BASE):
            if '/temp/' in root: continue
            for f in fnames:
                if f.endswith(suffix) and 'README' not in f:
                    files.append(os.path.join(root, f))
        files = sorted(files)
    
    old_size = os.path.getsize(src_path)
    total_old += old_size
    
    md_content, json_data = generate_minimized_v3(src_path, name, files)
    
    if md_content:
        md_path = os.path.join(OUT, cf)
        with open(md_path, 'w') as f:
            f.write(md_content)
        md_sz = len(md_content)
    else:
        md_sz = 0
    
    json_sz = 0
    if json_data:
        json_path = os.path.join(OUT, f"{name}_data.json")
        json_sz = os.path.getsize(json_path)
    
    new_size = md_sz + json_sz
    total_new += new_size
    
    print(f"  {cf:30s}: {old_size//1024:4d}KB → {md_sz//1024:4d}KB + {json_sz//1024:3d}KB = {new_size//1024:3d}KB ({100 - new_size*100//old_size:+3.0f}%)")

print(f"\n  {'TOTAL':30s}: {total_old//1024:4d}KB → {total_new//1024:4d}KB ({100 - total_new*100//total_old:+3.0f}%)")
print()
print(f"  00-consolidated-min/ contents:")
for f in sorted(os.listdir(OUT)):
    sz = os.path.getsize(os.path.join(OUT, f))
    print(f"    {f:30s} {sz//1024:4d}KB")
