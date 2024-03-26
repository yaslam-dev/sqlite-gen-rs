# sqlite-gen-rs

**Blazing fast Sqlite extension that generates a series written in rust.**

# Useage

1. Compile the code using `cargo build`.
2. install sqlite; enter this command in sqlite terminal `.load ./target/<Debug or Release>/libsqlite_csv`. This command will load the extension in SQLite.
3. To generate a Virtual table run this command `select value, start, stop, step from generate_series_rs(1,1000,2)`.
4. The Syntax for generate_series_rs is `start`, `stop`, and `step`. In the above example, the start is 1; the stop is 1000 and the step is 2.
