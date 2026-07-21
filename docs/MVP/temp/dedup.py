#!/usr/bin/env python3
"""
Deduplicate Alchemy docs: find identical content blocks and replace copies
with references to the canonical source. Saves a report of changes.
"""

import os, re, hashlib, json
from collections import defaultdict
import shutil

BASE = "/home/gg/projects/agents_blockchain/docs/MVP"
TEMP_HTML = "/home/gg/projects/agents_blockchain/docs/MVP/temp"
BACKUP_DIR = os.path.join(TEMP_HTML, "backup_dedup")
os.makedirs(BACKUP_DIR, exist_ok=True)

MIN_BLOCK_CHARS = 120  # min chars for a block to be considered for dedup
MIN_HITS = 3  # min files a block must appear in to be deduped

def norm(text):
    t = text.lower()
    t = re.sub(r'[^a-z0-9\s]', '', t)
    t = re.sub(r'\s+', ' ', t).strip()
    return t

def get_files():
    files = []
    for root, dirs, fnames in os.walk(BASE):
        if '/temp/' in root:
            continue
        for f in fnames:
            if f.endswith('.md') and f != 'README.md':
                files.append(os.path.join(root, f))
    return sorted(files)

def extract_blocks(text):
    """Split text into paragraph blocks, preserving position info."""
    blocks = []
    # Split by double newline (paragraphs)
    paras = text.split('\n\n')
    idx = 0
    for para in paras:
        stripped = para.strip()
        if stripped:
            blocks.append({
                'text': stripped,
                'start': idx,
                'end': idx + len(para),
                'norm': norm(stripped),
                'len': len(norm(stripped))
            })
        idx += len(para) + 2  # +2 for '\n\n'
    return blocks, text

def find_duplicate_groups(files):
    """Find all duplicate blocks across files."""
    block_map = defaultdict(list)  # norm_hash -> [(filepath, block_idx, text)]
    
    for fp in files:
        with open(fp, errors='ignore') as f:
            text = f.read()
        blocks, _ = extract_blocks(text)
        for bi, block in enumerate(blocks):
            if block['len'] < MIN_BLOCK_CHARS:
                continue
            h = hashlib.md5(block['norm'].encode()).hexdigest()
            block_map[h].append((fp, bi, block))
    
    # Filter to groups that appear in >= MIN_HITS files AND have at least 1 duplicate
    dups = {}
    for h, occurrences in block_map.items():
        unique_files = set(o[0] for o in occurrences)
        if len(unique_files) >= MIN_HITS and len(occurrences) > 1:
            dups[h] = occurrences
    return dups

def choose_canonical(occurrences):
    """Choose which file keeps the original block (the one with most specific path)."""
    # Prefer longer paths (more specific/deeper in tree) as canonical,
    # or the one with the simplest/most generic name as reference point
    # Strategy: pick the file in the most general section as canonical
    # (so quickstart canonical goes in 05-tools-resources)
    section_priority = {
        '05-tools-resources': 0,
        '04-build-with-ai': 1,
        '00-introduction': 2,
        '01-chains': 3,
        '02-data': 4,
        '03-wallets': 5,
    }
    def sort_key(occ):
        fp = occ[0]
        # Extract section folder
        match = re.search(r'(\d{2}-[^/]+)', fp)
        section = match.group(1) if match else ''
        prio = section_priority.get(section, 99)
        return (prio, fp)
    
    return min(occurrences, key=sort_key)

def build_relative_ref(from_file, to_file):
    """Build a relative markdown link from from_file to to_file."""
    from_dir = os.path.dirname(from_file)
    rel = os.path.relpath(to_file, from_dir)
    # Get title of target for link text
    return rel

def get_title(filepath):
    with open(filepath, errors='ignore') as f:
        for line in f:
            if line.startswith('# '):
                return line[2:].strip()
    return os.path.basename(filepath).replace('.md', '').replace('_', ' ').replace('--', ' → ').title()

def deduplicate():
    files = get_files()
    print(f"Scanning {len(files)} files for duplicates...")
    
    # First pass: find duplicates
    dups = find_duplicate_groups(files)
    print(f"Found {len(dups)} duplicate block groups (appear in >= {MIN_HITS} files)")
    
    # Choose canonical for each group
    canonicals = {}
    for h, occurrences in dups.items():
        canon = choose_canonical(occurrences)
        canonicals[h] = canon
    
    # Second pass: replace duplicates
    changes_made = []
    stats = defaultdict(int)  # file -> count of replacements
    
    for h, occurrences in dups.items():
        canon_fp, canon_bi, canon_block = canonicals[h]
        canon_text = canon_block['text']
        canon_title = get_title(canon_fp)
        canon_rel = os.path.relpath(canon_fp, BASE)
        
        for fp, bi, block in occurrences:
            if fp == canon_fp:
                continue  # skip canonical
            
            # Read the file
            with open(fp, errors='ignore') as f:
                text = f.read()
            
            # Backup first time
            backup_fp = os.path.join(BACKUP_DIR, os.path.relpath(fp, BASE).replace('/', '__'))
            if not os.path.exists(backup_fp):
                os.makedirs(os.path.dirname(backup_fp) or '.', exist_ok=True)
                shutil.copy2(fp, backup_fp)
            
            # Replace the block with a reference
            block_text = block['text']
            anchor = os.path.basename(canon_fp).replace('.md', '')
            
            # Create concise reference text
            ref_text = f"\n> 📄 **This content also appears in [{canon_title}]({canon_rel})** — see there for full details.\n"
            
            # Do the replacement
            if block_text in text:
                new_text = text.replace(block_text, ref_text.strip(), 1)
                with open(fp, 'w') as f:
                    f.write(new_text)
                changes_made.append({
                    'hash': h,
                    'file': fp,
                    'canonical': canon_fp,
                    'block_preview': block_text[:80] + '...' if len(block_text) > 80 else block_text
                })
                stats[fp] = stats.get(fp, 0) + 1
            else:
                # Block may not match exactly if text has variations; skip
                pass
    
    # Report
    print(f"\n=== Deduplication Report ===")
    print(f"Files modified: {len(stats)}")
    print(f"Blocks replaced: {len(changes_made)}")
    
    # Show files with most changes
    print(f"\nTop files by changes:")
    for fp, count in sorted(stats.items(), key=lambda x: -x[1])[:20]:
        rel = os.path.relpath(fp, BASE)
        print(f"  {count:3d} → {rel}")
    
    # Save full report
    report_path = os.path.join(BASE, "temp", "dedup_report.json")
    with open(report_path, 'w') as f:
        json.dump({
            'total_blocks_found': len(dups),
            'files_modified': len(stats),
            'blocks_replaced': len(changes_made),
            'changes': changes_made,
            'stats': {os.path.relpath(k, BASE): v for k, v in sorted(stats.items(), key=lambda x: -x[1])}
        }, f, indent=2)
    print(f"\nFull report: {report_path}")

if __name__ == '__main__':
    deduplicate()
