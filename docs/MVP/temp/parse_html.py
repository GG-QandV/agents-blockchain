#!/usr/bin/env python3
"""Parse downloaded HTML to .md files organized by section."""

import os, re, json, ssl, time, sys
from urllib.request import Request, urlopen
from bs4 import BeautifulSoup

BASE = "https://www.alchemy.com/docs"
TEMP_HTML = "/home/gg/projects/agents_blockchain/docs/MVP/temp/html"
OUT_DIR = "/home/gg/projects/agents_blockchain/docs/MVP"
URLS_FILE = "/home/gg/projects/agents_blockchain/docs/MVP/temp/llms/all_urls.json"

os.makedirs(OUT_DIR, exist_ok=True)

with open(URLS_FILE) as f:
    urls = json.load(f)

# Build URL-to-safe-filename mapping
def url_to_safe(url):
    path = url.replace(f"{BASE}/", "").replace(".md", "")
    safe = re.sub(r'[^\w\-/]', '_', path).strip("_").replace("/", "--")
    return safe if safe else "index"

url_html_map = {}
for url in urls:
    safe = url_to_safe(url)
    html_path = os.path.join(TEMP_HTML, f"{safe}.html")
    url_html_map[url] = html_path if os.path.exists(html_path) else None

print(f"Have HTML for {sum(1 for v in url_html_map.values() if v)}/{len(urls)} URLs")

# Section classification
def classify_section(url, url_path):
    """Determine which section a page belongs to."""
    name = url_path.lower()
    if any(kw in name for kw in ["agent", "cli", "mcp", "cursor", "ai-"]):
        return "04-build-with-ai"
    if any(kw in name for kw in ["wallet", "account-kit", "signer", "gas", "bundler", 
                                  "paymaster", "entrypoint", "session-key", "privy", "turnkey",
                                  "light-account", "modular-account", "eip-4337"]):
        return "03-wallets"
    if any(kw in name for kw in ["token-api", "nft-api", "portfolio", "transfer", "price",
                                  "webhook", "simulation", "stellar-data", "utility-api"]):
        return "02-data"
    if any(kw in name for kw in ["pricing", "compute-unit", "dashboard", "sandbox", "alert",
                                  "request-log", "sso", "role", "quickstart", "tutorial",
                                  "blockchain-basics", "glossary", "solidity", "hardhat",
                                  "smart-contract", "erc-", "snapshot", "faq", "error",
                                  "admin-api", "activity-log", "throughput", "best-practices"]):
        return "05-tools-resources"
    if any(kw in name for kw in ["chain", "ethereum", "solana", "arbitrum", "polygon", "base",
                                  "optimism", "trace-api", "debug-api", "rollup", "subscription",
                                  "websocket", "yellowstone", "monad", "alchemy-mined",
                                  "alchemy-pending", "newpending", "newheads", "mev"]):
        return "01-chains"
    if url_path.startswith("reference/"):
        ref_name = url_path.split("/", 1)[1].lower()
        if any(kw in ref_name for kw in ["chain", "trace", "debug", "subscription", "ethereum",
                                          "solana", "arbitrum", "polygon"]):
            return "01-chains"
        if any(kw in ref_name for kw in ["token", "nft", "portfolio", "transfer", "price",
                                          "webhook", "simulation", "notify"]):
            return "02-data"
        if any(kw in ref_name for kw in ["wallet", "gas", "bundler", "entrypoint"]):
            return "03-wallets"
        return "05-tools-resources"
    if url_path.startswith("data/"):
        return "02-data"
    if url_path.startswith("wallets/"):
        return "03-wallets"
    if url_path.startswith("chains/"):
        return "01-chains"
    if url_path.startswith("alchemy-") or url_path.startswith("build-with"):
        return "04-build-with-ai"
    if url_path.startswith("reference/"):
        return "05-tools-resources"
    # Check content-based classification
    for section, keywords in {
        "01-chains": ["chain", "ethereum", "rpc", "json-rpc"],
        "02-data": ["data", "token", "nft", "transfer"],
        "03-wallets": ["wallet", "account"],
        "04-build-with-ai": ["agent", "ai"],
    }.items():
        for kw in keywords:
            if kw in name:
                return section
    return "00-introduction"


def extract_content(html_path, url):
    """Extract main content from Alchemy docs HTML page."""
    with open(html_path, errors="ignore") as f:
        soup = BeautifulSoup(f.read(), "html.parser")
    
    # Remove non-content elements
    for tag in soup.find_all(["script", "style", "nav", "footer", "noscript"]):
        tag.decompose()
    for tag in soup.find_all(class_=re.compile(r"(nav|sidebar|toc|menu|banner|announce)", re.I)):
        tag.decompose()
    
    article = (
        soup.find("article")
        or soup.find("main")
        or soup.find(class_=re.compile(r"(content|doc|article|main|prose)", re.I))
    )
    if not article:
        article = soup.body or soup
    
    parts = []
    for el in article.find_all(["h1", "h2", "h3", "h4", "h5", "h6", "p", "li", "pre", "hr"]):
        tag = el.name
        text = el.get_text(strip=True)
        if not text and tag != "hr":
            continue
        
        if tag == "hr":
            parts.append("---")
        elif tag.startswith("h"):
            level = tag[1]
            parts.append(f"{'#' * int(level)} {text}")
        elif tag == "li":
            # Check for nested lists
            parts.append(f"- {text}")
        elif tag == "pre":
            code = el.get_text()
            lang = ""
            for c in el.get("class", []):
                for pfx in ["language-", "lang-"]:
                    if c.startswith(pfx):
                        lang = c.replace(pfx, "")
                        break
            parts.append(f"```{lang}\n{code}\n```")
        elif tag == "p":
            # Check for images, links inside
            parts.append(text)
    
    # Get title
    title = ""
    h1 = article.find("h1")
    if h1:
        title = h1.get_text(strip=True)
    
    content = "\n\n".join(filter(None, parts))
    return title, content


# Process each HTML file
section_counts = {}
missing_urls = []

for url in urls:
    html_path = url_html_map.get(url)
    if not html_path:
        missing_urls.append(url)
        continue
    
    url_path = url.replace(f"{BASE}/", "").replace(".md", "")
    section = classify_section(url, url_path)
    
    title, content = extract_content(html_path, url)
    if not title and not content:
        missing_urls.append(url)
        continue
    
    section_dir = os.path.join(OUT_DIR, section)
    os.makedirs(section_dir, exist_ok=True)
    
    safe_fn = url_to_safe(url)
    safe_fn = re.sub(r'[^\w\-]', '_', safe_fn).strip("_")
    if not safe_fn:
        safe_fn = "index"
    
    out_path = os.path.join(section_dir, f"{safe_fn}.md")
    
    with open(out_path, "w") as f:
        f.write(f"# {title}\n\n")
        f.write(f"> Source: [{url}]({url})\n\n")
        f.write(content)
    
    section_counts[section] = section_counts.get(section, 0) + 1

# Write README per section
section_names = {
    "00-introduction": "Introduction",
    "01-chains": "Chains",
    "02-data": "Data",
    "03-wallets": "Wallets",
    "04-build-with-ai": "Build with AI",
    "05-tools-resources": "Tools & Resources",
}

for section, name in section_names.items():
    section_dir = os.path.join(OUT_DIR, section)
    if not os.path.exists(section_dir):
        continue
    files = sorted([f for f in os.listdir(section_dir) if f.endswith(".md") and f != "README.md"])
    if not files:
        continue
    with open(os.path.join(section_dir, "README.md"), "w") as f:
        f.write(f"# {name}\n\n")
        f.write(f"Pages: {len(files)}\n\n")
        f.write("| # | Page |\n|---|---|\n")
        for i, fn in enumerate(files, 1):
            display = fn.replace(".md", "").replace("_", " ").replace("--", " → ").title()
            f.write(f"| {i} | [{display}]({fn}) |\n")

# Master index
total_pages = sum(section_counts.values())
with open(os.path.join(OUT_DIR, "README.md"), "w") as f:
    f.write("# Alchemy Documentation\n\n")
    f.write("> Scraped from [https://www.alchemy.com/docs](https://www.alchemy.com/docs)\n\n")
    f.write("## Sections\n\n")
    for section, name in section_names.items():
        section_dir = os.path.join(OUT_DIR, section)
        if os.path.exists(section_dir):
            files = [f for f in os.listdir(section_dir) if f.endswith(".md") and f != "README.md"]
            if files:
                f.write(f"- [{name}]({section}/) — {len(files)} pages\n")
    f.write(f"\n---\n**Total: {total_pages} pages**\n")

# Save missing URLs for retry
if missing_urls:
    miss_path = os.path.join(OUT_DIR, "temp", "missing_urls.json")
    with open(miss_path, "w") as f:
        json.dump(missing_urls, f, indent=2)
    print(f"\nMissing {len(missing_urls)} URLs — saved to {miss_path}")

print(f"\nDone! {total_pages} pages written to {OUT_DIR}")
for s, c in sorted(section_counts.items()):
    print(f"  {section_names.get(s, s)}: {c} pages")
