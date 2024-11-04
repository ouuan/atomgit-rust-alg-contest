## 代码设计

首先解析输入的进制，然后使用秦九韶算法计算出数值，再从低位向高位逐位计算输出，最后逆序输出。

## 运行结果

```console
$ cargo run -q
9(10) -> 11

$ cargo test -q

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
