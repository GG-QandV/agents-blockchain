#!/usr/bin/env python3
"""Generate structured table of contents from all .md files."""

import os, re
from collections import defaultdict

BASE = "/home/gg/projects/agents_blockchain/docs/MVP"

def get_files():
    files = []
    for root, dirs, fnames in os.walk(BASE):
        if '/temp/' in root:
            continue
        for f in fnames:
            if f.endswith('.md') and f != 'README.md':
                files.append(os.path.join(root, f))
    return sorted(files)

def extract_headings(fp):
    headings = []
    in_code_block = False
    with open(fp, errors='ignore') as f:
        for line in f:
            if line.strip().startswith('```'):
                in_code_block = not in_code_block
                continue
            if in_code_block:
                continue
            m = re.match(r'^(#{1,4})\s+(.+)', line)
            if m:
                level = len(m.group(1))
                text = m.group(2).strip()
                if text:
                    # Skip source URL lines and dedup headers
                    if text.startswith('Source:') or text.startswith('>'):
                        continue
                    headings.append((level, text))
    return headings

def get_h1(fp):
    with open(fp, errors='ignore') as f:
        for line in f:
            m = re.match(r'^#\s+(.+)', line)
            if m:
                return m.group(1).strip()
    return os.path.basename(fp).replace('.md', '').replace('_', ' ').title()

# Sections display order
SECTION_ORDER = [
    '00-consolidated',
    '01-chains',
    '02-data',
    '03-wallets',
    '04-build-with-ai',
    '05-tools-resources',
]
SECTION_NAMES = {
    '00-consolidated': '📦 Consolidated Templates',
    '01-chains': '⛓️ Chains',
    '02-data': '📊 Data',
    '03-wallets': '👛 Wallets',
    '04-build-with-ai': '🤖 Build with AI',
    '05-tools-resources': '🛠️ Tools & Resources',
}

files = get_files()
print(f"Scanning {len(files)} files...")

# Group by section
by_section = defaultdict(list)
for fp in files:
    rel = os.path.relpath(fp, BASE)
    section = rel.split('/')[0] if '/' in rel else ''
    by_section[section].append(fp)

lines = []
lines.append("# Alchemy Documentation — Table of Contents")
lines.append("")
lines.append(f"**{len(files)} files** · auto-generated")
lines.append("")
lines.append("---")
lines.append("")

for section in SECTION_ORDER:
    section_files = by_section.get(section, [])
    if not section_files:
        continue
    
    name = SECTION_NAMES.get(section, section)
    lines.append(f"## {name}")
    lines.append("")
    
    for fp in section_files:
        rel = os.path.relpath(fp, BASE)
        h1 = get_h1(fp)
        headings = extract_headings(fp)
        
        lines.append(f"### {h1}")
        lines.append(f"`📄 {rel}`")
        lines.append("")
        
        if headings:
            for level, text in headings:
                if level == 1:
                    continue  # skip h1, already shown
                indent = "  " * (level - 2)
                lines.append(f"{indent}- {text}")
            lines.append("")
    
    lines.append("---")
    lines.append("")

# Stats
lines.append("## Stats")
lines.append("")
total = len(files)
total_entries = sum(len(extract_headings(fp)) for fp in files)
lines.append(f"- Files: {total}")
lines.append(f"- Headings (h2+): {total_entries}")
for section in SECTION_ORDER:
    sf = by_section.get(section, [])
    if sf:
        name = SECTION_NAMES.get(section, section)
        lines.append(f"- {name}: {len(sf)} files")
lines.append("")

out_path = os.path.join(BASE, "TOC.md")
with open(out_path, 'w') as f:
    f.write('\n'.join(lines))

print(f"Written: {out_path} ({len(lines)} lines)")
