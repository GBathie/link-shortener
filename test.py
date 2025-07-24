import requests

if __name__ == "__main__":
    r = requests.post(
        "http://localhost:3000/shorten", json={"target_url": "https://www.google.com"}
    )
    print(r.status_code)
    print(r.json())
    r = requests.post(
        "http://localhost:3000/shorten", json={"target_url": "https://discord.com/channels/@me"}
    )
    print(r.status_code)
    print(r.json())
