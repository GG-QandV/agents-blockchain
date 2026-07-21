#!/usr/bin/env python3
"""Download remaining missing URLs."""

import os, re, json, time, ssl
from urllib.request import Request, urlopen

TEMP_HTML = "/home/gg/projects/agents_blockchain/docs/MVP/temp/html"
MISSING = "/home/gg/projects/agents_blockchain/docs/MVP/temp/missing_urls.json"

with open(MISSING) as f:
    urls = json.load(f)

ssl_ctx = ssl._create_unverified_context()
HEADERS = {"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"}

def fetch_url(url):
    path_part = url.replace("https://www.alchemy.com/docs/", "").replace(".md", "")
    safe = re.sub(r'[^\w\-/]', '_', path_part).strip("_").replace("/", "--")
    if not safe: safe = "index"
    fpath = os.path.join(TEMP_HTML, f"{safe}.html")
    if os.path.exists(fpath) and os.path.getsize(fpath) > 100:
        return fpath
    for attempt in range(3):
        try:
            req = Request(url, headers=HEADERS)
            with urlopen(req, timeout=30, context=ssl_ctx) as resp:
                data = resp.read()
            if len(data) > 200:
                with open(fpath, "wb") as f:
                    f.write(data)
                return fpath
        except:
            if attempt < 2:
                time.sleep(2)
    return None

total = len(urls)
ok = 0
for i, url in enumerate(urls):
    if fetch_url(url):
        ok += 1
    if (i+1) % 20 == 0:
        print(f"  [{100*(i+1)//total}%] {i+1}/{total} — {ok} ok")

print(f"\nDownloaded {ok}/{total} missing pages")
