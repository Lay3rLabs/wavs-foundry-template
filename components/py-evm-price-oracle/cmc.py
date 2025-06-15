# Use urllib.request with HTTP
import json
import time
import urllib.request
from dataclasses import dataclass

import encodings.idna

@dataclass
class PriceFeedData:
    symbol: str
    price: float
    timestamp: int

    def to_json(self) -> str:
        return json.dumps({
            "symbol": self.symbol,
            "price": self.price,
            "timestamp": self.timestamp
        })


def fetch_crypto_price(id: int) -> PriceFeedData:
    # Create URL - using HTTP instead of HTTPS
    url = f"http://api.coinmarketcap.com/data-api/v3/cryptocurrency/detail?id={id}&range=1h"

    # Set up the request
    headers = {
        "Accept": "application/json",
        "Content-Type": "application/json",
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36",
        "Cookie": f"myrandom_cookie={int(time.time())}"
    }

    # Create request object
    req = urllib.request.Request(url, headers=headers)

    try:
        # Make the request
        with urllib.request.urlopen(req) as response:
            body = response.read()

        # Parse the JSON
        root = json.loads(body)

        # Extract data
        symbol = root['data']['symbol']
        price = root['data']['statistics']['price']
        timestamp = root['status']['timestamp']

        # Create and return the price feed data
        return PriceFeedData(
            symbol=symbol,
            price=price,
            timestamp=timestamp
        )
    except Exception as e:
        raise RuntimeError(f"Failed to fetch price: {str(e)}")


if __name__ == "__main__":
    # Example usage
    price_data = fetch_crypto_price(1)
    print(price_data.to_json())
