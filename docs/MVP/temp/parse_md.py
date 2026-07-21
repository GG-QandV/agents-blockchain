#!/usr/bin/env python3
"""Parse Alchemy docs — files are already markdown, just organize by section."""

import os, re, json

BASE = "https://www.alchemy.com/docs"
TEMP_HTML = "/home/gg/projects/agents_blockchain/docs/MVP/temp/html"
OUT_DIR = "/home/gg/projects/agents_blockchain/docs/MVP"
URLS_FILE = "/home/gg/projects/agents_blockchain/docs/MVP/temp/llms/all_urls.json"

os.makedirs(OUT_DIR, exist_ok=True)

with open(URLS_FILE) as f:
    urls = json.load(f)

def url_to_safe(url):
    path = url.replace(f"{BASE}/", "").replace(".md", "")
    safe = re.sub(r'[^\w\-/]', '_', path).strip("_").replace("/", "--")
    return safe if safe else "index"

def classify_section(url_path):
    name = url_path.lower()
    # Quick categorization by URL structure
    if name.startswith("data/") or any(kw in name for kw in ["/token-", "/nft-", "/portfolio", "/transfer", "/price",
                                  "/webhook", "/simulation", "/stellar-data", "/utility-api"]):
        return "02-data"
    if name.startswith("wallets/") or any(kw in name for kw in ["wallet", "account-kit", "signer", "/gas-", "/bundler",
                                  "/paymaster", "/entrypoint", "/session-key", "privy", "turnkey",
                                  "light-account", "modular-account", "eip-4337", "aa-sdk", "account-kit"]):
        return "03-wallets"
    if any(kw in name for kw in ["agent-", "/cli", "mcp", "cursor", "build-with-ai"]):
        return "04-build-with-ai"
    # Check reference pages context
    if name.startswith("reference/"):
        ref = name.split("/", 1)[1]
        if any(kw in ref for kw in ["chain", "trace", "debug", "subscription", "ethereum",
                                      "solana", "arbitrum", "polygon", "base", "mev",
                                      "rollup", "yellowstone", "newheads", "newpending",
                                      "alchemy-mined", "alchemy-pending", "monad",
                                      "account-subscribe", "program-subscribe", "logs-subscribe",
                                      "signature-subscribe", "root-subscribe", "slot-subscribe"]):
            return "01-chains"
        if any(kw in ref for kw in ["token", "nft", "portfolio", "transfer", "price",
                                      "webhook", "simulation", "notify-api", "pricing",
                                      "compute-unit", "feature-support", "error-reference",
                                      "batch-request", "throughput", "gas-limit"]):
            return "02-data" if any(kw in ref for kw in ["token", "nft", "portfolio", "transfer", "price", "webhook", "simulation"]) else "05-tools-resources"
        if any(kw in ref for kw in ["wallet", "gas-manager", "bundler", "entrypoint"]):
            return "03-wallets"
        return "05-tools-resources"
    if any(kw in name for kw in ["chain", "ethereum", "solana", "arbitrum", "polygon", "base",
                                  "optimism", "trace-api", "debug-api", "subscription", "rollup",
                                  "mev", "yellowstone", "flashblocks", "alchemy-mined",
                                  "alchemy-pending", "newpending", "newheads"]):
        return "01-chains"
    # Pages in root /docs/ with reference-like content
    if any(kw in name for kw in ["pricing", "compute-unit", "dashboard", "sandbox", "alert",
                                  "request-log", "sso", "role", "quickstart", "tutorial",
                                  "blockchain-basics", "glossary", "solidity", "hardhat",
                                  "smart-contract", "erc-", "snapshot", "faq", "error",
                                  "admin-api", "activity-log", "throughput", "best-practices",
                                  "getting-started", "understanding-transaction",
                                  "websocket", "how-to-", "what-is-",
                                  "choose", "deploy", "integrat", "build", "debug",
                                  "hello-world", "set-up"]) and "chain" not in name:
        return "05-tools-resources"
    if any(kw in name for kw in ["chain", "rpc", "json-rpc"]):
        return "01-chains"
    if any(kw in name for kw in ["token", "nft", "transfer", "price", "data"]):
        return "02-data"
    if any(kw in name for kw in ["wallet", "account"]):
        return "03-wallets"
    if any(kw in name for kw in ["agent", "ai", "cli", "mcp"]):
        return "04-build-with-ai"
    return "05-tools-resources"  # default to tools

def extract_title(content):
    """Extract first # heading from markdown content."""
    for line in content.split("\n"):
        if line.startswith("# ") and not line.startswith("##"):
            return line[2:].strip()
    return ""

section_names = {
    "00-introduction": "Introduction",
    "01-chains": "Chains",
    "02-data": "Data",
    "03-wallets": "Wallets",
    "04-build-with-ai": "Build with AI",
    "05-tools-resources": "Tools & Resources",
}

section_counts = {}
errors = []

for url in urls:
    safe = url_to_safe(url)
    html_path = os.path.join(TEMP_HTML, f"{safe}.html")
    if not os.path.exists(html_path):
        errors.append(url)
        continue
    
    with open(html_path, errors="ignore") as f:
        content = f.read()
    
    if len(content) < 50:
        errors.append(url)
        continue
    
    url_path = url.replace(f"{BASE}/", "").replace(".md", "")
    section = classify_section(url_path)
    title = extract_title(content) or url_path.split("/")[-1].replace("-", " ").title()
    
    section_dir = os.path.join(OUT_DIR, section)
    os.makedirs(section_dir, exist_ok=True)
    
    safe_fn = re.sub(r'[^\w\-]', '_', safe).strip("_")
    if not safe_fn:
        safe_fn = "index"
    
    out_path = os.path.join(section_dir, f"{safe_fn}.md")
    
    with open(out_path, "w") as f:
        f.write(f"# {title}\n\n")
        f.write(f"> Source: [{url}]({url})\n\n")
        f.write(content)
    
    section_counts[section] = section_counts.get(section, 0) + 1

# Write README per section
for section, name in section_names.items():
    section_dir = os.path.join(OUT_DIR, section)
    if not os.path.exists(section_dir):
        continue
    files = sorted([f for f in os.listdir(section_dir) if f.endswith(".md") and f != "README.md"])
    if not files:
        continue
    with open(os.path.join(section_dir, "README.md"), "w") as f:
        f.write(f"# {name}\n\nPages: {len(files)}\n\n")
        f.write("| # | Page |\n|---|---|\n")
        for i, fn in enumerate(files, 1):
            display = fn.replace(".md", "").replace("_", " ").replace("--", " → ").title()
            f.write(f"| {i} | [{display}]({fn}) |\n")

# Master index
total = sum(section_counts.values())
with open(os.path.join(OUT_DIR, "README.md"), "w") as f:
    f.write("# Alchemy Documentation\n\n> Scraped from [https://www.alchemy.com/docs](https://www.alchemy.com/docs)\n\n## Sections\n\n")
    for section, name in section_names.items():
        section_dir = os.path.join(OUT_DIR, section)
        if os.path.exists(section_dir):
            files = [f for f in os.listdir(section_dir) if f.endswith(".md") and f != "README.md"]
            if files:
                f.write(f"- [{name}]({section}/) — {len(files)} pages\n")
    f.write(f"\n---\n**Total: {total} pages**\n")

if errors:
    miss_path = os.path.join(OUT_DIR, "temp", "missing_urls.json")
    with open(miss_path, "w") as f:
        json.dump(errors, f, indent=2)
    print(f"Missing {len(errors)} URLs → {miss_path}")

print(f"\nDone! {total} pages written:")
for s, c in sorted(section_counts.items()):
    print(f"  {section_names.get(s, s)}: {c} pages")
