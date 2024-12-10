use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use std::array::from_fn;

pub fn our_lfsr(a: [u32; 5]) -> u32 {
    let mut sum = [
        (a[0] as u64) << 15,
        (a[1] as u64) << 17,
        (a[2] as u64) << 21,
        (a[3] as u64) << 20,
        (a[4] as u64) << 8,
        a[4] as u64,
    ]
    .into_iter()
    .sum::<u64>();
    sum = (sum >> 31) + (sum % (1 << 31));
    sum = (sum >> 31) + (sum % (1 << 31));
    sum as u32
}

fn rotate31(x: u32, k: u32) -> u32 {
    ((x << k) | (x >> (31 - k))) & !(1 << 31)
}

fn mul31(x: u32, k: u32) -> u32 {
    (u64::from(x) * (1 << k) % ((1 << 31) - 1)) as u32
}

fn add31(a: u32, b: u32) -> u32 {
    let sum = a.wrapping_add(b);
    (sum & !(1 << 31)).wrapping_add(sum >> 31)
}

pub fn naive_lfsr(a: [u32; 5]) -> u32 {
    [
        rotate31(a[0], 15),
        rotate31(a[1], 17),
        rotate31(a[2], 21),
        rotate31(a[3], 20),
        rotate31(a[4], 8),
        a[4],
    ]
    .into_iter()
    .fold(0, add31)
}

pub fn mod_lfsr(a: [u32; 5]) -> u32 {
    [
        mul31(a[0], 15),
        mul31(a[1], 17),
        mul31(a[2], 21),
        mul31(a[3], 20),
        mul31(a[4], 8),
        a[4],
    ]
    .into_iter()
    .fold(0, add31)
}

fn lfsr_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let data = from_fn(|_| rng.gen_range(0..(1 << 31)));
    assert_eq!(our_lfsr(data), naive_lfsr(data));
    assert_eq!(our_lfsr(data), mod_lfsr(data));
    c.bench_function("our", |b| b.iter(|| our_lfsr(black_box(data))));
    c.bench_function("naive", |b| b.iter(|| naive_lfsr(black_box(data))));
    c.bench_function("mod", |b| b.iter(|| mod_lfsr(black_box(data))));
}

criterion_group!(benches, lfsr_benchmark);
criterion_main!(benches);
