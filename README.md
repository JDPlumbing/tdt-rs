# ⏱ Time Delta Toolkit (TDT)

A Rust toolkit for working with **time deltas**.  
Count ticks, break down elapsed time, pretty-print durations, or run a live ticking clock.  

TDT is compact, exact, and designed as both a **library** and a **CLI tool**.

---

## 🚀 Install
From crates.io:
```bash
cargo install tdt
```

From source:
```bash
git clone https://github.com/JDPlumbing/tdt.git
cd tdt
cargo install --path .
```

---

## 📌 Usage

### Since Epoch
```bash
tdt since --unit seconds
# → Ticks since 1970-01-01
```

### Between Dates
```bash
tdt between --start "1997-06-15 00:00:00" --end "2025-09-14 00:00:00"
# → "28 years, 3 months, 0 days"
```

### Until Target Date
```bash
tdt until --target "2100-01-01 00:00:00" --unit days
# → "Until 2100: 27393 days"
```

### Live Clock
```bash
tdt clock
# → Since epoch: 1737283645s | 1737283645123ms | 1737283645123456µs
```

---

## 🛠 Library Usage
```rust
use chrono::Utc;
use tdt::{count_ticks, pretty_breakdown};

fn main() {
    let start = Utc::now();
    let end = start + chrono::Duration::days(42);

    println!("Ticks in hours: {}", count_ticks(Some(start), Some(end), "hours"));
    println!("Breakdown: {}", pretty_breakdown(start, Some(end), 3));
}
```

---

## 📄 License
MIT License © 2025 JD Plumbing
