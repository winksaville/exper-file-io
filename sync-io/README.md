# Synchronous file IO

Based on [Rust By Example open](https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html)

# Run

Currently to run successfully you must create hello.txt, if you don't:
```
wink@3900x 22-11-23T17:31:28.340Z:~/prgs/rust/myrepos/exper-file-io/sync-io (main)
$ cargo run
   Compiling sync-io v0.1.0 (/home/wink/prgs/rust/myrepos/exper-file-io/sync-io)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `/home/wink/prgs/rust/myrepos/exper-file-io/target/debug/sync-io`
thread 'main' panicked at 'couldn't open hello.txt: No such file or directory (os error 2)', sync-io/src/main.rs:12:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

And if you do it succeeds:
```
wink@3900x 22-11-23T17:32:55.804Z:~/prgs/rust/myrepos/exper-file-io/sync-io (main)
$ echo "Hello World!" > hello.txt
wink@3900x 22-11-23T17:32:59.324Z:~/prgs/rust/myrepos/exper-file-io/sync-io (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/wink/prgs/rust/myrepos/exper-file-io/target/debug/sync-io`
hello.txt contains:
Hello World!
```



## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
