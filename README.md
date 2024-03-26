# sqlite-gen-rs

**Blazing fast Sqlite extension that generates a series written in rust.**

# Useage

1. Compile the code using `cargo build`.
2. install sqlite; enter this command in sqlite terminal `.load ./target/debug/libsqlite_gen_rs`. This command will load the extension in SQLite.
3. To generate a Virtual table run this command `select value, start, stop, step from generate_series_rs(1,1000,2)`.
4. The Syntax for generate_series_rs is `start`, `stop`, and `step`. In the above example, the start is 1; the stop is 1000 and the step is 2.

# Benchmark

```js
SQLite version 3.45.2 2024-03-12 11:06:23
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
sqlite> .load ./target/debug/libsqlite_gen_rs
sqlite> .timer on
sqlite> select count(value) from generate_series_rs(1,1e7,1);
10000000
Run Time: real 0.695 user 0.665762 sys 0.002378
sqlite>
```
