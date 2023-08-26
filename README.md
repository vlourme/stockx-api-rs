# StockX Search API

The StockX Search API is a high-speed server utilized by our [RapidAPI wrapper](https://rapidapi.com/vlourme/api/stockx-search) for StockX data retrieval. On localhost condition, the server is able to respond in less than 10ms.

## Notice

This code is not meant to be used standalone. We have a proprietary method for fetching StockX data, and our scraping code is not publicly available. You can access the hosted version of this API through RapidAPI, and we offer a generous free-tier.

## Request Example

```python
import requests

response = requests.get(
    url="https://stockx-search.p.rapidapi.com/search",
    headers={
        "X-RapidAPI-Key": "[API-KEY]",
        "X-RapidAPI-Host": "stockx-search.p.rapidapi.com"
    },
    params={
        "q": "Dunk Low Crushed D.C.",
        "page": "1"
    })

print(response.json())
```

To obtain an API key, you can register on [RapidAPI](https://rapidapi.com). Our free-tier allows up to 10,000 requests per month.

## Response Example

```json
[
  {
    "id": "4375befd-097a-48a1-8046-8054f3c95cb1",
    "title": "Nike SB Dunk Low Crushed D.C.",
    "brand": "Nike",
    "color": "Olive Grey/Mantra Orange/Rattan",
    "description": "The Nike SB Dunk Low Crushed DC comes in a mix of Olive Grey ... [truncated] ... The Nike SB Dunk Low Crushed DC made its debut on October 7th, 2022, and retailed for $110.",
    "category": "Apparel & Accessories > Shoes",
    "currency": "EUR",
    "image": "https://images.stockx.com/images/Nike-SB-Dunk-Low-Crushed-DC-Product.jpg",
    "link": "https://stockx.com/nike-sb-dunk-low-crushed-dc",
    "avg_price": 211.3,
    "available_sizes": 20,
    "labels": ["sneakers", "Nike SB SB Dunk Low", "DH7782-001"],
    "variants": [
      {
        "product_id": "4375befd-097a-48a1-8046-8054f3c95cb1",
        "variant_id": "0840a1e0-653b-4a49-9611-0b193d9f0a1f",
        "size": "7",
        "price": 166.0
      },
      {
        "product_id": "4375befd-097a-48a1-8046-8054f3c95cb1",
        "variant_id": "0dbcf5f7-ef9e-43d2-9873-50a3b1d9fe7a",
        "size": "9.5",
        "price": 210.0
      }
      // ... more variants ...
    ]
  }
  // ... more matching products ...
]
```
