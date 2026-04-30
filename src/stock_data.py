import csv
import random
import time
from datetime import datetime, timedelta

# ── CONFIG ──────────────────────────────────────────────────────────────
NUM_SYMBOLS   = 100          # number of unique stock tickers
ROWS_PER_SYM  = 22_500       # rows per symbol  →  100 × 22,500 = 2,250,000 total
OUTPUT_FILE   = "stock_data.csv"
RANDOM_SEED   = 42           # fixed seed so you get the same data every run

# Generate 100 fake ticker symbols like STOCK000, STOCK001 … STOCK099
symbols = [f"STOCK{i:03d}" for i in range(NUM_SYMBOLS)]

# Starting prices for each symbol — random between $10 and $500
random.seed(RANDOM_SEED)
base_prices = {sym: random.uniform(10.0, 500.0) for sym in symbols}

# Starting timestamp — Jan 1 2023 00:00:00
base_time = datetime(2023, 1, 1, 0, 0, 0)

print(f"Generating {NUM_SYMBOLS * ROWS_PER_SYM:,} rows for {NUM_SYMBOLS} symbols...")
t0 = time.time()

with open(OUTPUT_FILE, "w", newline="") as f:
    writer = csv.writer(f)

    # Write the header row — must match the field names in StockRow struct
    writer.writerow(["symbol", "timestamp", "price"])

    for sym in symbols:
        price     = base_prices[sym]   # start at the base price for this symbol
        timestamp = base_time          # start at the base time

        for _ in range(ROWS_PER_SYM):
            # Simulate a small random price move each tick
            # random.gauss(0, 1) gives a value centered around 0
            # multiplying by 0.5 keeps moves small (±$0.50 typical)
            price += random.gauss(0, 1) * 0.5

            # Clamp price so it never goes below $1.00 (no negative stock prices)
            price = max(price, 1.0)

            # Advance timestamp by 1 second each row
            timestamp += timedelta(seconds=1)

            # Write the row: symbol, ISO timestamp string, price rounded to 4 decimals
            writer.writerow([sym, timestamp.strftime("%Y-%m-%dT%H:%M:%S"), round(price, 4)])

elapsed = time.time() - t0
print(f"Done! Written to '{OUTPUT_FILE}' in {elapsed:.2f}s")