#!/usr/bin/env python3
"""
Smart minimizer: compress template docs by identifying diff patterns
and storing per-chain data as compact tables instead of full text.

Output: single small template + one compact per-chain lookup table.
"""

import os, re, json, hashlib
from collections import defaultdict

BASE = "/home/gg/projects/agents_blockchain/docs/MVP"
CONSOLIDATED = os.path.join(BASE, "00-consolidated")
OUT = os.path.join(BASE, "00-consolidated-min")
os.makedirs(OUT, exist_ok=True)

def norm(t):
    return re.sub(r'\s+', ' ', t.strip())

def extract_diff_pattern(texts):
    """
    Given a list of text values for the same block across chains,
    find the pattern of what changes vs what stays constant.
    Returns (common_template, {chain: substitutions})
    """
    if len(texts) <= 1:
        return texts[0] if texts else "", {}
    
    # Try line-by-line diff
    lines_per_chain = [t.replace('\r', '').split('\n') for t in texts]
    min_lines = min(len(l) for l in lines_per_chain)
    
    substituted = []
    chain_subs = defaultdict(dict)
    
    for line_idx in range(min_lines):
        lines_at_pos = [l[line_idx] for l in lines_per_chain]
        unique = list(set(lines_at_pos))
        
        if len(unique) == 1:
            substituted.append(unique[0])
        else:
            # Find what varies in this line
            var_name = f"VAR{line_idx}"
            substituted.append(f"{{{{{var_name}}}}}")
            for ci, line in enumerate(lines_at_pos):
                chain_name = list(texts.keys())[ci]
                chain_subs[chain_name][var_name] = line
    
    # Handle extra lines for chains that have more
    max_lines = max(len(l) for l in lines_per_chain)
    for line_idx in range(min_lines, max_lines):
        chains_with_line = [c for c, l in zip(texts.keys(), lines_per_chain) if line_idx < len(l)]
        for c in chains_with_line:
            chain_subs[c].setdefault('extra', []).append(
                lines_per_chain[list(texts.keys()).index(c)][line_idx])
    
    return '\n'.join(substituted), dict(chain_subs)

def extract_compact_template(consolidated_path, group_name):
    """Read consolidated doc and extract truly variable data only."""
    with open(consolidated_path) as f:
        text = f.read()
    
    # Parse the consolidated structure:
    # - Common template (header + common blocks)
    # - Per-chain parameters section (variable blocks)
    # - Per-chain unique content section
    
    # Strategy: identify just the true differences by chain
    # and write them as compact data
    
    # Extract chain names from the "Applies to" line
    chains_match = re.search(r'> Applies to: (.+)', text)
    if not chains_match:
        return None
    chains = [c.strip() for c in chains_match.group(1).split(',')]
    
    # Collect all variable data per chain from the per-chain sections
    chain_data = defaultdict(dict)
    chain_extras = defaultdict(list)
    
    # Parse variable blocks (Step N or Variable Block)
    # Pattern: ### Step N or ### Variable Block (position N)
    var_sections = re.split(r'^### Step \d+|^### Variable Block', text, flags=re.MULTILINE)
    
    # Parse per-chain unique content
    current_chain = None
    per_chain_section = text.split("## Per-Chain Unique Content")
    has_per_chain = len(per_chain_section) > 1
    
    if has_per_chain:
        per_chain_content = per_chain_section[1]
        for match in re.finditer(r'^### (.+?)\n(.*?)(?=^### |\Z)', per_chain_content, re.MULTILINE | re.DOTALL):
            chain_name = match.group(1).strip()
            content = match.group(2).strip()
            if content and not content.startswith('Source Files') and not content.startswith('---'):
                chain_extras[chain_name].append(content)
    
    # Build compact representation
    # Key insight: most per-chain sections are just chain name substituted
    # into an otherwise identical template
    
    # Extract the common template part
    common_match = re.search(r'## Common Template\n(.*?)(?=## Per-Chain Parameters|\Z)', text, re.DOTALL)
    common_template = common_match.group(1).strip() if common_match else ""
    
    # Read per-chain data from the source files (more reliable than parsing back)
    # and extract truly unique values per chain
    
    # Final format: 
    # # Group Name
    # ## Template (with {CHAIN} placeholders)
    # ## Per-Chain Data Table
    # ## Unique Content (only what's truly unique per chain)
    
    lines = []
    lines.append(f"# {group_name.replace('-', ' ').title()}")
    lines.append(f"")
    lines.append(f"Chains: {len(chains)}")
    lines.append(f"")
    
    # Template with placeholders
    lines.append("## Template")
    lines.append("")
    lines.append(common_template)
    lines.append("")
    
    # Per-chain variable parameters as JSON file reference
    lines.append("## Per-Chain Parameters")
    lines.append("")
    lines.append(f"See `{group_name}_data.json` for per-chain values.")
    lines.append("")
    
    # Summary of what varies
    var_count = text.count("### Step") + text.count("### Variable Block")
    lines.append(f"_~{var_count} variable blocks per chain, {len(chains)} chains total_")
    lines.append("")
    
    # Extra unique content summary
    if chain_extras:
        lines.append("## Unique Content by Chain")
        lines.append("")
        for chain_name, extras in sorted(chain_extras.items()):
            for extra in extras:
                lines.append(f"### {chain_name}")
                lines.append("")
                lines.append(extra)
                lines.append("")
    
    # Source reference
    lines.append("---")
    lines.append(f"Source: {consolidated_path}")
    lines.append("")
    
    return '\n'.join(lines)

def generate_data_json(consolidated_path, group_name):
    """Generate a compact JSON data file for per-chain values."""
    with open(consolidated_path) as f:
        text = f.read()
    
    chains_match = re.search(r'> Applies to: (.+)', text)
    if not chains_match:
        return None
    chains = [c.strip() for c in chains_match.group(1).split(',')]
    
    # Extract per-chain unique content
    chain_extras = {}
    per_chain_section = text.split("## Per-Chain Unique Content")
    if len(per_chain_section) > 1:
        per_chain_content = per_chain_section[1]
        for match in re.finditer(r'^### (.+?)\n(.*?)(?=^### |\Z)', per_chain_content, re.MULTILINE | re.DOTALL):
            chain_name = match.group(1).strip()
            content = match.group(2).strip()
            if content and not content.startswith('Source Files') and not content.startswith('---'):
                chain_extras[chain_name] = content
    
    # Extract variable block data
    var_data = {}
    for match in re.finditer(r'### Step \d+\n.*?\n\| Chain \| Value \|.*?\n\|.*?\|\n((?:\|.*?\|\n)+)', text, re.DOTALL):
        table_text = match.group(1)
        for row in table_text.strip().split('\n'):
            parts = [p.strip() for p in row.split('|') if p.strip()]
            if len(parts) >= 2:
                chain, val = parts[0], parts[1]
                if chain not in var_data:
                    var_data[chain] = []
                var_data[chain].append(val)
    
    data = {
        'template_name': group_name,
        'chains': chains,
        'chain_count': len(chains),
        'chain_extras': chain_extras,
        'variable_block_count': len(re.findall(r'\| Chain \| Value \|', text)),
    }
    
    return data


# Process each consolidated file
consolidated_files = [f for f in os.listdir(CONSOLIDATED) if f.endswith('.md')]
print("Processing consolidated documents:")

for cf in sorted(consolidated_files):
    name = cf.replace('.md', '')
    src_path = os.path.join(CONSOLIDATED, cf)
    
    print(f"\n  {cf}: ", end="", flush=True)
    
    # Generate minimized markdown
    md_content = extract_compact_template(src_path, name)
    if md_content:
        out_path = os.path.join(OUT, cf)
        with open(out_path, 'w') as f:
            f.write(md_content)
        md_size = len(md_content)
        print(f"template → {md_size//1024}KB", end="", flush=True)
    
    # Generate data JSON
    data = generate_data_json(src_path, name)
    if data:
        json_path = os.path.join(OUT, f"{name}_data.json")
        with open(json_path, 'w') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        json_size = os.path.getsize(json_path)
        print(f" + data → {json_size//1024}KB", end="", flush=True)
    
    # Compare sizes
    orig_size = os.path.getsize(src_path)
    print(f" (was {orig_size//1024}KB)", end="", flush=True)

# Summary
print(f"\n\n{'='*60}")
print("Size comparison:")
for cf in sorted(os.listdir(CONSOLIDATED)):
    if not cf.endswith('.md'):
        continue
    old = os.path.join(CONSOLIDATED, cf)
    new = os.path.join(OUT, cf)
    old_kb = os.path.getsize(old) // 1024
    if os.path.exists(new):
        new_kb = os.path.getsize(new) // 1024
        data_kb = 0
        data_file = os.path.join(OUT, cf.replace('.md', '_data.json'))
        if os.path.exists(data_file):
            data_kb = os.path.getsize(data_file) // 1024
        total_new = new_kb + data_kb
        pct = (1 - total_new / old_kb) * 100
        print(f"  {cf:30s}: {old_kb:4}KB → {new_kb:4}KB + {data_kb:4}KB = {total_new:4}KB ({pct:+.0f}%)")
    else:
        print(f"  {cf:30s}: {old_kb:4}KB → skipped")
