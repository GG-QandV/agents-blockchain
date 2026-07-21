#!/usr/bin/env python3
"""Download Alchemy docs pages via urllib (no curl proxy issues)."""

import os, re, json, sys, time, ssl
from urllib.request import Request, urlopen
from urllib.error import URLError, HTTPError
from bs4 import BeautifulSoup

TEMP_HTML = "/home/gg/projects/agents_blockchain/docs/MVP/temp/html"
os.makedirs(TEMP_HTML, exist_ok=True)

with open("/home/gg/projects/agents_blockchain/docs/MVP/temp/llms/all_urls.json") as f:
    urls = json.load(f)

ssl_ctx = ssl._create_unverified_context()
HEADERS = {
    "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
    "Accept": "text/html,application/xhtml+xml",
}

def fetch_url(url, retries=2):
    path_part = url.replace("https://www.alchemy.com/docs/", "").replace(".md", "")
    safe = re.sub(r'[^\w\-/]', '_', path_part).strip("_").replace("/", "--")
    if not safe:
        safe = "index"
    fpath = os.path.join(TEMP_HTML, f"{safe}.html")
    if os.path.exists(fpath) and os.path.getsize(fpath) > 200:
        return fpath
    for attempt in range(retries):
        try:
            req = Request(url, headers=HEADERS)
            with urlopen(req, timeout=30, context=ssl_ctx) as resp:
                data = resp.read()
            if len(data) > 200:
                with open(fpath, "wb") as f:
                    f.write(data)
                return fpath
        except Exception as e:
            if attempt < retries - 1:
                time.sleep(1)
    return None

batch_size = 10
total = len(urls)
print(f"Downloading {total} pages in batches of {batch_size}...")
ok_count = 0

for i in range(0, total, batch_size):
    batch = urls[i:i+batch_size]
    for url in batch:
        result = fetch_url(url)
        if result:
            ok_count += 1
    pct = min(100, (i + batch_size) / total * 100)
    print(f"  [{pct:.0f}%] {i+batch_size}/{total} — {ok_count} ok so far")
    time.sleep(0.2)

print(f"\nDone! {ok_count}/{total} pages downloaded to {TEMP_HTML}")
