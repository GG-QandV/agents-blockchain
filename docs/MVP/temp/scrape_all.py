#!/usr/bin/env python3
"""Scrape Alchemy Docs: download all pages, extract content, write .md files."""

import os, re, json, time, subprocess, sys
from urllib.parse import urlparse, unquote
from bs4 import BeautifulSoup

BASE = "https://www.alchemy.com/docs"
TEMP_HTML = "/home/gg/projects/agents_blockchain/docs/MVP/temp/html"
TEMP_LLMS = "/home/gg/projects/agents_blockchain/docs/MVP/temp/llms"
OUT_DIR = "/home/gg/projects/agents_blockchain/docs/MVP"

os.makedirs(TEMP_HTML, exist_ok=True)
os.makedirs(TEMP_LLMS, exist_ok=True)
os.makedirs(OUT_DIR, exist_ok=True)

# Step 1: Fetch all section llms.txt files
SECTION_URLS = [
    ("chains", f"{BASE}/chains/llms.txt"),
    ("data", f"{BASE}/data/llms.txt"),
    ("wallets", f"{BASE}/wallets/llms.txt"),
    ("build-with-ai", f"{BASE}/build-with-ai/llms.txt"),
    ("tools-resources", f"{BASE}/get-started/llms.txt"),
    ("top", f"{BASE}/llms.txt"),
]

llms_texts = {}
for name, url in SECTION_URLS:
    fpath = os.path.join(TEMP_LLMS, f"{name}.txt")
    if not os.path.exists(fpath) or os.path.getsize(fpath) == 0:
        print(f"Fetching {url}...")
        r = subprocess.run(["curl", "-skL", "--max-time", "30", url], capture_output=True, text=True)
        with open(fpath, "w") as f:
            f.write(r.stdout)
    with open(fpath) as f:
        llms_texts[name] = f.read()

# Step 2: Extract all unique page URLs from llms.txt files
# Match markdown links: [text](url)
MD_LINK = re.compile(r'\[([^\]]+)\]\(([^)]+)\)')

all_urls = set()
for name, text in llms_texts.items():
    for match in MD_LINK.findall(text):
        link_text, url = match
        url = url.strip()
        if not url.startswith("https://www.alchemy.com/docs/"):
            continue
        if url.endswith("/llms.txt"):
            continue
        # Only take .md page URLs (skip section-level llms.txt links)
        if ".md" in url:
            all_urls.add(url)

urls = sorted(all_urls)
print(f"Total unique doc pages to fetch: {len(urls)}")
with open(os.path.join(TEMP_LLMS, "all_urls.json"), "w") as f:
    json.dump(urls, f, indent=2)

# Step 3: Download each page as HTML using curl (parallel batches)
def fetch_url(url):
    """Fetch a URL and save to temp HTML dir, return path or None."""
    # Create a safe filename from URL
    path = url.replace(f"{BASE}/", "").replace(".md", "")
    # Clean path for filesystem
    safe = re.sub(r'[^\w\-/]', '_', path)
    safe = safe.strip("_").replace("/", "--")
    if not safe:
        safe = "index"
    fpath = os.path.join(TEMP_HTML, f"{safe}.html")
    if os.path.exists(fpath) and os.path.getsize(fpath) > 100:
        return fpath  # already downloaded
    try:
        r = subprocess.run(
            ["curl", "-skL", "--max-time", "45", "-H", "User-Agent: Mozilla/5.0", url],
            capture_output=True, text=True, timeout=60
        )
        if r.returncode == 0 and len(r.stdout) > 500:
            with open(fpath, "w") as f:
                f.write(r.stdout)
            return fpath
    except Exception as e:
        print(f"  Error fetching {url}: {e}")
    return None

batch_size = 8
print(f"\nDownloading {len(urls)} pages in batches of {batch_size}...")
for i in range(0, len(urls), batch_size):
    batch = urls[i:i+batch_size]
    results = []
    for url in batch:
        results.append(fetch_url(url))
    n_done = sum(1 for r in results if r)
    n_total = len(batch)
    pct = min(100, (i + n_total) / len(urls) * 100)
    print(f"  [{pct:.0f}%] batch {i//batch_size + 1}/{(len(urls)-1)//batch_size + 1}: {n_done}/{n_total} ok")
    time.sleep(0.3)  # rate limiting

print("\nAll pages downloaded. Extracting content...")

# Step 4: Extract content from HTML and write .md files
def extract_content(html_path, url):
    """Extract main content from Alchemy docs HTML page."""
    with open(html_path, errors="ignore") as f:
        soup = BeautifulSoup(f.read(), "html.parser")
    
    # Remove nav, footer, scripts, styles
    for tag in soup.find_all(["script", "style", "nav", "footer", "header", "noscript"]):
        tag.decompose()
    for tag in soup.find_all(class_=re.compile(r"(nav|footer|sidebar|menu|toc|header|banner|announce)", re.I)):
        tag.decompose()
    
    # Try to find main content area
    article = (
        soup.find("article")
        or soup.find("main")
        or soup.find(class_=re.compile(r"(content|doc|article|main|prose)", re.I))
        or soup.find("div", class_=re.compile(r"(content|doc|article|main|prose)", re.I))
    )
    if not article:
        article = soup.body or soup
    
    # Get text, preserving structure
    lines = []
    for el in article.find_all(["h1", "h2", "h3", "h4", "h5", "h6", "p", "li", "pre", "code", "table", "tr", "th", "td", "hr", "br"]):
        tag = el.name
        text = el.get_text(strip=True)
        if not text and tag not in ("hr", "br"):
            continue
        
        if tag == "hr":
            lines.append("---")
        elif tag.startswith("h"):
            level = tag[1]
            lines.append(f"{'#' * int(level)} {text}")
        elif tag == "li":
            lines.append(f"- {text}")
        elif tag == "pre":
            code = el.get_text()
            lang = ""
            code_class = el.get("class", [])
            for c in code_class:
                if c.startswith("language-") or c.startswith("lang-"):
                    lang = c.replace("language-", "").replace("lang-", "")
                    break
            lines.append(f"```{lang}")
            lines.append(code)
            lines.append("```")
        elif tag in ("th", "td"):
            pass  # handled by table
        elif tag == "table":
            # Simple table extraction
            rows = el.find_all("tr")
            table_lines = []
            for row in rows:
                cells = row.find_all(["th", "td"])
                cell_texts = [c.get_text(strip=True) for c in cells]
                table_lines.append("| " + " | ".join(cell_texts) + " |")
                if row == rows[0] and rows[0].find_all("th"):
                    # header separator
                    table_lines.append("| " + " | ".join(["---"] * len(cells)) + " |")
            lines.extend(table_lines)
            lines.append("")
        elif tag == "br":
            lines.append("")
        else:
            lines.append(text)
    
    title = ""
    h1 = article.find("h1")
    if h1:
        title = h1.get_text(strip=True)
    
    content = "\n\n".join(filter(None, lines))
    return title, content


# Process all downloaded HTML files
html_files = [f for f in os.listdir(TEMP_HTML) if f.endswith(".html")]
print(f"Processing {len(html_files)} HTML files...")

# Group by section for output files
section_pages = {
    "00-introduction": [],
    "01-chains": [],
    "02-data": [],
    "03-wallets": [],
    "04-build-with-ai": [],
    "05-tools-resources": [],
}

section_keywords = {
    "01-chains": ["chain", "ethereum", "solana", "arbitrum", "polygon", "base", "optimism", 
                  "trace-api", "debug-api", "rollup", "subscription", "websocket",
                  "alchemy-minedtransactions", "alchemy-pendingtransactions"],
    "02-data": ["data", "token-api", "nft-api", "portfolio", "transfer", "price", 
                "webhook", "simulation", "utility"],
    "03-wallets": ["wallet", "account", "signer", "gas", "bundler", "paymaster", 
                   "entrypoint", "session-key", "privy", "turnkey"],
    "04-build-with-ai": ["agent", "cli", "mcp", "ai", "cursor"],
    "05-tools-resources": ["pricing", "compute-unit", "dashboard", "quickstart", "tutorial",
                           "blockchain", "solidity", "hardhat", "smart-contract", "erc",
                           "snapshot", "glossary", "faq", "error", "admin-api"],
}


def classify_section(url_path):
    """Determine which section a page belongs to."""
    for section, keywords in section_keywords.items():
        for kw in keywords:
            if kw.lower() in url_path.lower():
                return section
    # Fallback by URL prefix
    if url_path.startswith("chains") or url_path.startswith("reference/"):
        return "01-chains"
    elif url_path.startswith("data"):
        return "02-data"
    elif url_path.startswith("wallets"):
        return "03-wallets"
    elif url_path.startswith("alchemy-agent") or url_path.startswith("alchemy-cli") or url_path.startswith("alchemy-mcp"):
        return "04-build-with-ai"
    elif url_path.startswith("get-started") or url_path.startswith("tutorials") or url_path.startswith("snapshots"):
        return "05-tools-resources"
    return "00-introduction"


for html_file in html_files:
    html_path = os.path.join(TEMP_HTML, html_file)
    # Find original URL
    # Reconstruct URL from filename
    safe_name = html_file.replace(".html", "")
    # This is approximate; we'll use the URL list
    url = None
    for u in urls:
        u_safe = re.sub(r'[^\w\-/]', '_', u.replace(f"{BASE}/", "").replace(".md", "")).strip("_").replace("/", "--")
        if u_safe == safe_name:
            url = u
            break
    if not url:
        continue
    
    title, content = extract_content(html_path, url)
    if not content:
        continue
    
    # Determine output path
    url_path = url.replace(f"{BASE}/", "").replace(".md", "")
    section = classify_section(url_path)
    
    # Create output subdirectory
    section_dir = os.path.join(OUT_DIR, section)
    os.makedirs(section_dir, exist_ok=True)
    
    # Create filename from URL path
    safe_filename = url_path.replace("/", "--").strip("-")
    if not safe_filename:
        safe_filename = "index"
    safe_filename = re.sub(r'[^\w\-]', '_', safe_filename)
    
    out_path = os.path.join(section_dir, f"{safe_filename}.md")
    
    with open(out_path, "w") as f:
        f.write(f"# {title}\n\n")
        f.write(f"> Source: {url}\n\n")
        f.write(content)
    
    print(f"  -> {out_path}")

# Step 5: Create index files per section
for section, pages in section_pages.items():
    section_dir = os.path.join(OUT_DIR, section)
    if not os.path.exists(section_dir):
        continue
    md_files = sorted([f for f in os.listdir(section_dir) if f.endswith(".md")])
    if not md_files:
        continue
    
    index_path = os.path.join(section_dir, "README.md")
    with open(index_path, "w") as f:
        section_name = section.split("-", 1)[1].replace("-", " ").title()
        f.write(f"# {section_name}\n\n")
        f.write("## Pages\n\n")
        for mf in md_files:
            if mf == "README.md":
                continue
            name = mf.replace(".md", "").replace("_", " ").replace("--", " → ").title()
            f.write(f"- [{name}]({mf})\n")

# Step 6: Create master index
master_index = []
for section, pages in section_pages.items():
    section_dir = os.path.join(OUT_DIR, section)
    if not os.path.exists(section_dir):
        continue
    md_files = sorted([f for f in os.listdir(section_dir) if f.endswith(".md") and f != "README.md"])
    master_index.append((section, len(md_files)))

with open(os.path.join(OUT_DIR, "README.md"), "w") as f:
    f.write("# Alchemy Documentation\n\n")
    f.write("Full documentation scraped from https://www.alchemy.com/docs\n\n")
    f.write("## Sections\n\n")
    for section, count in master_index:
        section_name = section.split("-", 1)[1].replace("-", " ").title()
        f.write(f"- [{section_name}]({section}/) — {count} pages\n")
    f.write(f"\n---\n")
    f.write(f"\nTotal pages: {sum(c for _, c in master_index)}\n")

print("\nDone! All documentation saved to", OUT_DIR)
print(f"Total pages processed: {sum(c for _, c in master_index)}")
