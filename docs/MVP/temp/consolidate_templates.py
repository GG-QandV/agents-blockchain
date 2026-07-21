#!/usr/bin/env python3
"""
Consolidate template documentation groups.

For each template group (quickstart, FAQ, overview), this script:
1. Reads all files in the group
2. Extracts common template vs per-chain differences
3. Generates consolidated documents

Strategy:
- Compare blocks paragraph-by-paragraph across files
- Blocks present in ALL files → common template
- Blocks with variations → parameterized template (shows per-chain values)
- Blocks unique to certain files → per-chain specifics in a table
"""

import os, re, json, hashlib
from collections import defaultdict, OrderedDict
from difflib import SequenceMatcher
import shutil

BASE = "/home/gg/projects/agents_blockchain/docs/MVP"
BACKUP = os.path.join(BASE, "temp", "backup_consolidate")
os.makedirs(BACKUP, exist_ok=True)

def norm(text):
    t = text.lower().strip()
    t = re.sub(r'[^a-z0-9\s]', '', t)
    return re.sub(r'\s+', ' ', t).strip()

def get_paragraphs(text):
    """Split text into paragraph-level blocks, preserving structure."""
    blocks = []
    # Split by double newlines (paragraph breaks)
    raw = text.split('\n\n')
    for para in raw:
        s = para.strip()
        if s:
            blocks.append(s)
    return blocks

def analyze_group(files, group_name):
    """
    Analyze a group of template files.
    Returns: {common_blocks, param_blocks, per_chain_data}
    """
    if len(files) < 2:
        return None
    
    # Read all files
    file_data = {}
    for fp in files:
        with open(fp, errors='ignore') as f:
            text = f.read()
        title = ""
        for line in text.split('\n'):
            if line.startswith('# '):
                title = line[2:].strip()
                break
        paragraphs = get_paragraphs(text)
        file_data[fp] = {
            'title': title,
            'text': text,
            'paragraphs': paragraphs,
            'norm_paras': [norm(p) for p in paragraphs],
            'chain': extract_chain_name(fp, title)
        }
    
    # Find common blocks and variations
    all_paras = list(file_data.values())
    first_paras = all_paras[0]['norm_paras']
    
    # For each block position, check if all files have the same content
    min_len = min(len(d['norm_paras']) for d in all_paras)
    max_len = max(len(d['norm_paras']) for d in all_paras)
    
    common = []
    param_blocks = []
    per_chain_data = defaultdict(dict)
    
    # Classify each position
    for i in range(min_len):
        texts_at_i = [d['norm_paras'][i] for d in all_paras]
        unique = len(set(texts_at_i))
        
        if unique == 1:
            # Exactly the same across all files
            common.append({
                'idx': i,
                'text': all_paras[0]['paragraphs'][i],
                'type': 'common'
            })
        else:
            # Different across files — capture per-chain values
            param_blocks.append({
                'idx': i,
                'type': 'variable',
                'values': {d['chain']: d['paragraphs'][i] for d in all_paras}
            })
    
    # Find per-chain unique paragraphs (beyond common min length)
    for d in all_paras:
        extra = d['paragraphs'][min_len:]
        for para in extra:
            per_chain_data[d['chain']].setdefault('extra', []).append(para)
    
    return {
        'common': common,
        'variable': param_blocks,
        'per_chain': dict(per_chain_data),
        'chains': [d['chain'] for d in all_paras],
        'min_len': min_len,
        'max_len': max_len,
    }

def extract_chain_name(fp, title):
    """Extract chain name from file path or title."""
    basename = os.path.basename(fp).replace('.md', '')
    # Remove prefixes like 'reference--', 'chains--'
    name = re.sub(r'^(reference|chains|data|wallets)--', '', basename)
    # Remove suffixes like '-api-quickstart', '-api-faq', '-api-overview'
    name = re.sub(r'(-api-quickstart|-api-faq|-api-overview)$', '', name)
    name = name.replace('--', '/').replace('-', ' ').title().strip()
    if not name:
        name = title
    return name

def find_template_groups():
    """Find all template file groups (quickstart, FAQ, overview)."""
    all_files = []
    for root, dirs, fnames in os.walk(BASE):
        if '/temp/' in root:
            continue
        for f in fnames:
            if f.endswith('.md') and f != 'README.md':
                all_files.append(os.path.join(root, f))
    
    groups = OrderedDict()
    
    patterns = {
        'api-quickstart': '-api-quickstart.md',
        'api-faq': '-api-faq.md',
        'api-overview': '-api-overview.md',
        'utxo-websockets': 'utxo-websockets.md',
        'utxo-overview': 'utxo.md',
    }
    
    for group_name, suffix in patterns.items():
        group_files = [f for f in all_files if f.endswith(suffix) and 'README' not in f]
        if len(group_files) >= 2:
            groups[group_name] = sorted(group_files)
    
    return groups


def generate_consolidated(group_name, analysis, files):
    """
    Generate a consolidated markdown document for a template group.
    """
    chains = analysis['chains']
    
    lines = []
    lines.append(f"# {group_name.replace('-', ' ').title()}")
    lines.append(f"")
    lines.append(f"> Consolidated from {len(files)} chain-specific template pages.")
    lines.append(f"> Applies to: {', '.join(chains)}")
    lines.append(f"")
    lines.append("---")
    lines.append("")
    
    # Section 1: Common template
    if analysis['common']:
        lines.append("## Common Template")
        lines.append("")
        lines.append("The following content is **identical** across all chains:")
        lines.append("")
        for block in analysis['common']:
            lines.append(block['text'])
            lines.append("")
    
    # Section 2: Variable parameters (per-chain values)
    if analysis['variable']:
        lines.append("## Per-Chain Parameters")
        lines.append("")
        lines.append("These blocks differ per chain. Below is the variation per chain:")
        lines.append("")
        
        for vblock in analysis['variable']:
            # Show as a compact table if values are short (< 200 chars)
            max_val_len = max(len(v) for v in vblock['values'].values())
            
            if max_val_len < 120:
                # Table format
                lines.append(f"### Step {vblock['idx']}")
                lines.append("")
                lines.append("| Chain | Value |")
                lines.append("|-------|-------|")
                for chain in chains:
                    val = vblock['values'].get(chain, '')
                    # Clean up: take first line
                    first_line = val.split('\n')[0][:100]
                    lines.append(f"| {chain} | {first_line} |")
                lines.append("")
            else:
                # Full text per chain
                preview_text = list(vblock['values'].values())[0][:200]
                lines.append(f"### Variable Block (position {vblock['idx']})")
                lines.append("")
                lines.append(f"Preview: _{preview_text}..._")
                lines.append("")
                lines.append("<details>")
                lines.append("<summary>Show per-chain values</summary>")
                lines.append("")
                for chain in chains:
                    lines.append(f"**{chain}:**")
                    lines.append("")
                    lines.append(vblock['values'].get(chain, ''))
                    lines.append("")
                lines.append("</details>")
                lines.append("")
    
    # Section 3: Per-chain unique extras
    has_extra = any(analysis['per_chain'].get(c, {}).get('extra', []) for c in chains)
    if has_extra:
        lines.append("## Per-Chain Unique Content")
        lines.append("")
        lines.append("Some chains have additional content not present in others:")
        lines.append("")
        for chain in chains:
            extras = analysis['per_chain'].get(chain, {}).get('extra', [])
            if extras:
                lines.append(f"### {chain}")
                lines.append("")
                for extra in extras:
                    lines.append(extra)
                    lines.append("")
    
    # Section 4: Source files
    lines.append("---")
    lines.append("")
    lines.append("## Source Files")
    lines.append("")
    lines.append("These individual chain pages were consolidated into this document:")
    lines.append("")
    for fp in files:
        rel = os.path.relpath(fp, BASE)
        lines.append(f"- [{rel}]({rel})")
    lines.append("")
    
    return '\n'.join(lines)


def consolidate_group(group_name, files):
    """Process one template group."""
    print(f"\n{'='*60}")
    print(f"Group: {group_name} ({len(files)} files)")
    print(f"{'='*60}")
    
    # Take first 3 files as sample for analysis (use representative chains)
    analysis = analyze_group(files, group_name)
    if not analysis:
        print("  SKIP: need at least 2 files")
        return
    
    print(f"  Chains sampled: {', '.join(analysis['chains'][:5])}{'...' if len(analysis['chains']) > 5 else ''}")
    print(f"  Common blocks: {len(analysis['common'])}")
    print(f"  Variable blocks: {len(analysis['variable'])}")
    
    # Generate consolidated document
    out_dir = os.path.join(BASE, '00-consolidated')
    os.makedirs(out_dir, exist_ok=True)
    out_path = os.path.join(out_dir, f"{group_name}.md")
    
    content = generate_consolidated(group_name, analysis, files)
    with open(out_path, 'w') as f:
        f.write(content)
    
    print(f"  → {os.path.relpath(out_path, BASE)} ({len(content)} chars)")
    
    # Mark original files as consolidated (rename with .consolidated suffix)
    for fp in files:
        bak_path = os.path.join(BACKUP, os.path.relpath(fp, BASE).replace('/', '__'))
        shutil.copy2(fp, bak_path)
        # Add a note at the top pointing to the consolidated doc
        with open(fp) as f:
            text = f.read()
        # Prepend header
        rel_out = os.path.relpath(out_path, os.path.dirname(fp))
        header = f"> ⚠️ **This page is a template variant.** The consolidated content is in [{group_name}]({rel_out}).\n> Below is the original chain-specific version.\n\n"
        with open(fp, 'w') as f:
            f.write(header + text)
    
    return analysis


groups = find_template_groups()
print(f"Found {len(groups)} template groups:")
for name, files in groups.items():
    print(f"  {name}: {len(files)} files")

for name, files in groups.items():
    consolidate_group(name, files)

print(f"\nDone! Consolidated templates in {BASE}/00-consolidated/")
