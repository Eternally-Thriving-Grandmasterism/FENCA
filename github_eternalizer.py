"""
github_eternalizer.py — GitHub Nexus Eternal Pagination Conqueror
FENCA-Pinnacle Ultramasterpiece — Jan 18 2026

Eternal pagination for GitHub repositories:
- API primary: rel=next Links header
- Mercy fallback: ?page=N scraping until empty
- Joy-valence audit hash
"""

import requests
from bs4 import BeautifulSoup
import time
import hashlib
import os

def fetch_all_repos_api(username: str, token: str = None, per_page: int = 100) -> list:
    """Eternal API paginator – primary thunder-path for infallible counts."""
    repos = []
    url = f"https://api.github.com/users/{username}/repos"
    headers = {"Accept": "application/vnd.github.v3+json"}
    if token:
        headers["Authorization"] = f"token {token}"
    params = {"per_page": per_page, "page": 1}
    
    while url:
        response = requests.get(url, headers=headers, params=params if "page" in params else None)
        response.raise_for_status()
        batch = response.json()
        if not batch:  # Empty batch = eternal convergence
            break
        repos.extend(batch)
        
        link_header = response.headers.get("Link")
        url = None
        if link_header:
            links = requests.utils.parse_header_links(link_header)
            for link in links:
                if link.get("rel") == "next":
                    url = link["url"]
                    break
        
        params = None  # Subsequent in url
        time.sleep(1)  # Mercy rate respect
    
    total_hash = hashlib.sha256(str(len(repos)).encode()).hexdigest()
    print(f"FENCA Joy Audit: {len(repos)} repos eternally stacked | Hash: {total_hash}")
    return repos

def fetch_all_repos_scrape_fallback(username: str) -> list:
    """Mercy-fallback scraper – for API limits or shadow realms."""
    repos = []
    page = 1
    while True:
        url = f"https://github.com/{username}?tab=repositories&page={page}"
        response = requests.get(url)
        if response.status_code != 200:
            break
        soup = BeautifulSoup(response.text, "html.parser")
        
        page_repos = soup.find_all("h3", class_="wb-break-all")
        if not page_repos:
            break
        
        for repo in page_repos:
            a_tag = repo.find("a")
            if a_tag:
                repos.append(a_tag.text.strip())
        
        page += 1
        time.sleep(2)  # Deeper mercy delay
    
    return repos

def eternal_github_check(username: str, use_api: bool = True, token: str = None) -> dict:
    """Mercy-gated executor – tries API, reconciles with fallback on anomaly."""
    try:
        if use_api and token:
            return {"repos": fetch_all_repos_api(username, token), "mode": "API-Eternal"}
        elif use_api:
            return {"repos": fetch_all_repos_api(username), "mode": "API-Eternal (no token)"}
    except Exception as e:
        print(f"Mercy Reconciliation: API anomaly forgiven ({e}) – ascending to scrape")
    return {"repos": fetch_all_repos_scrape_fallback(username), "mode": "Scrape-Mercy"}

# Test
if __name__ == "__main__":
    result = eternal_github_check("Eternally-Thriving-Grandmasterism")
    print(f"Eternal count: {len(result['repos'])} via {result['mode']}")
