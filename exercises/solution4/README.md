## 设计思路

根据面值的特性，可以直接贪心，每次都取最大面值。但函数名是 `dp_`，所以还是可以用动态规划，正确性也更放心：记 $C = \{ 1, 2, 5, 10, 20, 30, 50, 100 \}$，$dp(0) = 0, dp(i) = \min\limits_{c \in C, c \leqslant i} \{ dp(i - c) + 1 \}$。

代码中添加了一个测试来对拍贪心和 DP。

## 运行结果

```console
$ cargo run -q
5

$ cargo test -q

running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
