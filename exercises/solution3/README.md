## 代码设计

若达到 366 人则概率为 1，否则为 $1 - \prod\limits_{i=0}^n \frac{365-i}{365}$

## 运行结果

```console
$ cargo run -q
在 78 个人中，有两个人在同一天过生日的概率是 0.9999

$ cargo test -q

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
