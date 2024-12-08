# Rust Contest

[Rust数据结构与算法学习赛 - 开放原子大赛 - 训练营](https://opencamp.cn/atomgit/camp/rust)

## 代码结构

可复用的代码作为独立的 crate 放在 [`crates`](./crates) 中，在题目代码中引用。

```
crates
├── baby-mimalloc
├── chinese-convert
├── date
├── dsu
├── max-match-segmentation
├── pinyin
├── prime
├── resident-id
└── zuc
```

## T1 

枚举当前已知质数至根号来筛素数，这样虽然筛素数复杂度不如埃拉托斯特尼筛法、欧拉线性筛，但这种筛法可以直接扩展，不需要提前预知筛的范围，适合本题这样答案范围未知的情况，且并未成为复杂度瓶颈（ $O\left(\frac{n \sqrt n}{\log n}\right)$）。

一边筛，一边枚举平方，检查减去平方后是否是质数。总复杂度 $O(n \sqrt n)$（$n$ 为答案的大小）。

## T2

实现 `serde` 的相关 trait，在反序列化过程中计算，实现 zero copy 数据处理，使用并查集计算连通集数量。

## T3

公式计算星期，农历新年打表。

## T4

基于自动机实现了通用的最大正向匹配算法，以及为字符串实现特化的最大正向/反向/双向匹配，支持分词和转换。

使用 pypinyin 的转换表。

## T5

可实现 `trait Rule` 来新增政策。`struct LinearRule` 可通过配置若干参数实现 `Rule`。

## T6

使用和 T4 同样的分词算法，使用 opencc 的转换表。

## T7

存储行政区划时没有使用级联列表，而是分别查询前两位、前四位和全部六位，如果前四位没查到，则根据是 01/02/90 来判断是市辖区/县/省或自治区直辖县级行政区划。

## T8

使用 Pollard Rho 算法进行质因数分解。

## T9

使用 Miller Rabin 算法判断质数。

## T10

采用与 RustCrypto 兼容的实现，实现了 `cipher` 的相关 trait。

## T11

见 [`crates/baby-mimalloc`](./crates/baby-mimalloc)。可以使用 `cargo doc --open` 或 `RUSTDOCFLAGS='--cfg docsrs' cargo +nightly doc --all-feature` 查看文档。
