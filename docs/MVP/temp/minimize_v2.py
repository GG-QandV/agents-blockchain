#!/usr/bin/env python3
"""
V2 Minimizer: extract actual diff variables from templates.
Analyzes all chain files to find real parameters (RPC URLs, chain IDs, etc.)
and generates truly parameterized template + compact data table.
"""

import os, re, json
from collections import defaultdict

BASE = "/home/gg/projects/agents_blockchain/docs/MVP"
CONSOLIDATED = os.path.join(BASE, "00-consolidated")
OUT = os.path.join(BASE, "00-consolidated-min")
os.makedirs(OUT, exist_ok=True)

def norm(t):
    return re.sub(r'\s+', ' ', t.strip())

def extract_variables_from_files(files, group_name):
    """
    Read all source template files for a group and extract
    actual variable parameters per chain.
    """
    chain_params = {}
    
    for fp in files:
        with open(fp, errors='ignore') as f:
            text = f.read()
        chain_name = extract_chain_name(fp, group_name)
        
        # Extract useful variables
        params = {}
        
        # Chain name variations
        h1 = ""
        for line in text.split('\n'):
            if line.startswith('# '):
                h1 = line[2:].strip()
                break
        params['title'] = h1
        
        # RPC endpoint URL
        rpc_match = re.search(r'https://[a-z0-9.-]+\.alchemy\.com/[^\s\n]+', text)
        if rpc_match:
            params['rpc_url'] = rpc_match.group(0)
        
        # Chain ID
        chain_id_match = re.search(r'[Cc]hain\s*[Ii][Dd][:\s]+(\d+)', text)
        if not chain_id_match:
            chain_id_match = re.search(r'chainId["\']?\s*[:=]\s*["\']?(\d+)', text)
        if chain_id_match:
            params['chain_id'] = chain_id_match.group(1)
        
        # Documentation links
        docs_match = re.search(r'\[(docs?|developer|documentation)[^\]]*\]\((https?://[^)]+)\)', text, re.I)
        if docs_match:
            params['docs_url'] = docs_match.group(2)
        
        # Explorer URL
        explorer_match = re.search(r'https://[a-z0-9.-]+\.(etherscan|explorer|blockscout|oklink|subscan)\.(io|com|network)[^\s\n]*', text)
        if explorer_match:
            params['explorer_url'] = explorer_match.group(0)
        
        # Faucet URL
        faucet_match = re.search(r'(faucet|faucet_url)[:\s]+(https?://[^\s\n]+)', text, re.I)
        if not faucet_match:
            faucet_match = re.search(r'https://[a-z0-9.-]+\.(faucet|testnet\.faucet)[^\s\n]*', text, re.I)
        if faucet_match:
            params['faucet_url'] = faucet_match.group(0) if not faucet_match.group(2) else faucet_match.group(2)
        
        # Token/symbol
        symbol_match = re.search(r'[Nn]ative\s+[Cc]urrency[^:]*:\s*(\w+)', text)
        if not symbol_match:
            symbol_match = re.search(r'(ETH|BTC|SOL|MATIC|AVAX|BNB|ARB|OP|ATOM|DOT|ADA)[^a-z]', text)
        if symbol_match:
            params['native_token'] = symbol_match.group(1)
        
        chain_params[chain_name] = params
    
    return chain_params


def extract_chain_name(fp, group_name):
    basename = os.path.basename(fp).replace('.md', '')
    name = re.sub(r'^(reference|chains|data|wallets)--', '', basename)
    name = re.sub(r'(-api-quickstart|-api-faq|-api-overview|--utxo|-utxo)$', '', name)
    name = name.replace('--', '/').replace('-', ' ').title().strip()
    return name if name else basename

def count_real_variables(chain_params):
    """Count how many actual variables were found per chain."""
    var_counts = defaultdict(int)
    for chain, params in chain_params.items():
        for k, v in params.items():
            if v:
                var_counts[k] += 1
    return dict(var_counts)


def find_source_files(group_name):
    """Find all source template files for a group by scanning the MVP directories."""
    suffix_map = {
        'api-quickstart': '-api-quickstart.md',
        'api-faq': '-api-faq.md',
        'api-overview': '-api-overview.md',
        'utxo-websockets': 'utxo-websockets.md',
        'utxo-overview': 'utxo.md',
    }
    suffix = suffix_map.get(group_name)
    if not suffix:
        return []
    
    files = []
    for root, dirs, fnames in os.walk(BASE):
        if '/temp/' in root:
            continue
        for f in fnames:
            if f.endswith(suffix) and 'README' not in f:
                files.append(os.path.join(root, f))
    return sorted(files)


def generate_minimized(consolidated_path, group_name, files):
    """Generate truly minimized template + data."""
    chain_params = extract_variables_from_files(files, group_name)
    
    if not chain_params:
        # Fallback: just do basic minimizer
        return None, None
    
    # Read consolidated doc for the template
    with open(consolidated_path) as f:
        cons_text = f.read()
    
    # Extract chain names
    chains_match = re.search(r'> Applies to: (.+)', cons_text)
    chains = [c.strip() for c in chains_match.group(1).split(',')] if chains_match else sorted(chain_params.keys())
    
    # Extract common template
    common_match = re.search(r'## Common Template\n(.*?)(?=## Per-Chain Parameters|\Z)', cons_text, re.DOTALL)
    common_template = common_match.group(1).strip() if common_match else ""
    
    # Count found variables
    var_stats = count_real_variables(chain_params)
    found_count = sum(var_stats.values())
    
    # Generate minimized template
    lines = []
    lines.append(f"# {group_name.replace('-', ' ').title()}")
    lines.append(f"")
    lines.append(f"Chains: {len(chains)}")
    lines.append(f"Template type: template-based ({len(chains)} variants → 1 template + 1 data table)")
    lines.append(f"")
    
    if common_template:
        lines.append("## Common Template")
        lines.append("")
        lines.append(common_template)
        lines.append("")
    
    # Variables found
    useful_vars = {k: v for k, v in var_stats.items() if v >= 2}
    if useful_vars:
        lines.append("## Per-Chain Variables")
        lines.append("")
        lines.append("| Variable | Found In |")
        lines.append("|----------|----------|")
        for var, count in sorted(useful_vars.items()):
            pct = 100 * count // len(chains)
            lines.append(f"| `{var}` | {count}/{len(chains)} ({pct}%) |")
        lines.append("")
        lines.append("Full data in `{}_data.json`".format(group_name))
        lines.append("")
    
    # Per-chain data table (compact)
    lines.append("## Per-Chain Data Table")
    lines.append("")
    
    # Build column headers from useful variables
    cols = list(useful_vars.keys()) if useful_vars else ['title']
    header = "| Chain | " + " | ".join(c.replace('_', ' ').title() for c in cols) + " |"
    sep = "|-------|" + "|".join("---" for _ in cols) + "|"
    lines.append(header)
    lines.append(sep)
    
    for chain_name in chains:
        params = chain_params.get(chain_name, {})
        row = f"| {chain_name} |"
        for col in cols:
            val = params.get(col, '')
            # Shorten long URLs
            if len(str(val)) > 60:
                val = str(val)[:57] + '...'
            row += f" {val} |"
        lines.append(row)
    
    lines.append("")
    
    # Unique content per chain (only what's truly unique)
    lines.append("## Per-Chain Unique Content (extra beyond template)")
    lines.append("")
    
    # Count unique extras from the consolidated document
    per_chain_section = cons_text.split("## Per-Chain Unique Content")
    if len(per_chain_section) > 1:
        for match in re.finditer(r'^### (.+?)\n(.*?)(?=^### |\Z)', per_chain_section[1], re.MULTILINE | re.DOTALL):
            chain_name = match.group(1).strip()
            content = match.group(2).strip()
            if content and not content.startswith('Source Files') and not content.startswith('---') and len(content) > 100:
                # Only include if truly unique (not just chain name substitution)
                lines.append(f"<details><summary>{chain_name}</summary>\n\n{content}\n\n</details>")
                lines.append("")
    
    # Source
    lines.append("---")
    lines.append(f"Source: {os.path.relpath(consolidated_path, BASE)}")
    lines.append("")
    
    md_content = '\n'.join(lines)
    
    # Generate data JSON with proper per-chain values
    json_data = {
        'template_name': group_name,
        'chain_count': len(chains),
        'chains': chains,
        'per_chain': {}
    }
    for chain_name in chains:
        params = chain_params.get(chain_name, {})
        clean = {k: v for k, v in params.items() if v}
        if clean:
            json_data['per_chain'][chain_name] = clean
    
    return md_content, json_data


# Process all consolidated files
consolidated_files = [f for f in os.listdir(CONSOLIDATED) if f.endswith('.md')]
print("V2 Minimizing consolidated documents:\n")

total_old = 0
total_new = 0

for cf in sorted(consolidated_files):
    name = cf.replace('.md', '')
    src_path = os.path.join(CONSOLIDATED, cf)
    files = find_source_files(name)
    
    print(f"  {cf:30s}: ", end="", flush=True)
    
    md_content, json_data = generate_minimized(src_path, name, files)
    
    old_size = os.path.getsize(src_path)
    total_old += old_size
    
    if md_content:
        md_path = os.path.join(OUT, cf)
        with open(md_path, 'w') as f:
            f.write(md_content)
        md_size = len(md_content)
    else:
        md_size = 0
    
    if json_data:
        json_path = os.path.join(OUT, f"{name}_data.json")
        with open(json_path, 'w') as f:
            json.dump(json_data, f, indent=2, ensure_ascii=False)
        json_size = os.path.getsize(json_path)
    else:
        json_size = 0
    
    new_size = md_size + json_size
    total_new += new_size
    
    if old_size > 0:
        pct = (1 - new_size / old_size) * 100
        print(f" {old_size//1024:4d}KB → {md_size//1024:4d}KB + {json_size//1024:3d}KB = {new_size//1024:4d}KB ({pct:+.0f}%)")
    else:
        print(f" skipped")

print(f"\n  {'TOTAL':30s}: {total_old//1024:4d}KB → {total_new//1024:4d}KB ({100 - total_new*100//total_old:+.0f}%)")
