use criterion::{criterion_group, criterion_main, Criterion};
use chrono::{Utc, TimeZone};
use tdt::core::TimeDelta;

// ---------- Constructors (measure creation cost) ----------
fn bench_constructors(c: &mut Criterion) {
    c.bench_function("construct: TimeDelta::from_now()", |b| {
        b.iter(|| {
            let _ = TimeDelta::from_now();
        })
    });

    let start = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let end   = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();

    c.bench_function("construct: TimeDelta::between(2000..2025)", |b| {
        b.iter(|| {
            let _ = TimeDelta::between(start, end);
        })
    });

    let since = Utc.with_ymd_and_hms(2010, 1, 1, 0, 0, 0).unwrap();
    c.bench_function("construct: TimeDelta::until_now(2010..now)", |b| {
        b.iter(|| {
            let _ = TimeDelta::until_now(since);
        })
    });
}

// ---------- Ops on a fixed window (stable, comparable numbers) ----------
fn bench_ops_fixed(c: &mut Criterion) {
    let start = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let end   = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let td = TimeDelta::between(start, end);

    c.bench_function("ticks(days) 2000..2025", |b| {
        b.iter(|| td.ticks("days"))
    });

    c.bench_function("ticks(seconds) 2000..2025", |b| {
        b.iter(|| td.ticks("seconds"))
    });

    c.bench_function("pretty(3) 2000..2025", |b| {
        b.iter(|| td.pretty(3))
    });
}

// ---------- Ops using now (more jittery, still useful) ----------
fn bench_ops_now(c: &mut Criterion) {
    let since = Utc.with_ymd_and_hms(2010, 1, 1, 0, 0, 0).unwrap();
    let td = TimeDelta::until_now(since);

    c.bench_function("ticks(milliseconds) 2010..now", |b| {
        b.iter(|| td.ticks("milliseconds"))
    });
}

criterion_group!(benches, bench_constructors, bench_ops_fixed, bench_ops_now);
criterion_main!(benches);
