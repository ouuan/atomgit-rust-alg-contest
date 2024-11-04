## 代码设计

`split` 后使用 `HashSet` 统计非重复元素个数。

## 运行结果

```console
$ cargo run -q
count: 5

$ cargo test -q

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
