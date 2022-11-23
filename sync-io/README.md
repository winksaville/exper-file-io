# Synchronous file IO

Based on [Rust By Example open](https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html)

# Run

If hello.txt exists deletes it and then creates it and reads it using read_to_string.
Then reads it again into a `[u8]` buffer and prints the data as its reading in hex.
Finally, prints the total read and the length of the last buffer.
```
wink@3900x 22-11-23T20:58:35.928Z:~/prgs/rust/myrepos/exper-file-io/sync-io (main)
$ cargo run
   Compiling sync-io v0.1.0 (/home/wink/prgs/rust/myrepos/exper-file-io/sync-io)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `/home/wink/prgs/rust/myrepos/exper-file-io/target/debug/sync-io`
Creating hello.txt
hello.txt contains:
Hello World
open_existing_and_read_buf_at_a_time: file_name=hello.txt buf.len=5
read 5 buf=[48, 65, 6C, 6C, 6F]
read 5 buf=[20, 57, 6F, 72, 6C]
read 1 buf=[64]
len_read=11 len_last_buf=1
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
