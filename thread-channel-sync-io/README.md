# Thread channel sync IO


# Run

Two threads are running, the main thread and file_io_thread. Main
thread is sleeping for 5ns while waiting for file_io_tread to finish.
Doing this allow us to see the two thread interleaving execution.

Note: file_io_thread is basically sync-io except main is now file_io_thread()
and `println!`'s are now `log::info!`.

```
wink@3900x 22-11-23T22:38:25.140Z:~/prgs/rust/myrepos/exper-file-io/thread-channel-sync-io (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/wink/prgs/rust/myrepos/exper-file-io/target/debug/thread-channel-sync-io`
[2022-11-23T22:38:32.199822606Z INFO  thread_channel_sync_io   13  1] main:+
[2022-11-23T22:38:32.199878120Z INFO  thread_channel_sync_io   21  1] main:  working while worker_thread is not finished
[2022-11-23T22:38:32.199883400Z INFO  thread_channel_sync_io   24  1] main:  sleeping 5ns
[2022-11-23T22:38:32.199942221Z INFO  thread_channel_sync_io   24  1] main:  sleeping 5ns
[2022-11-23T22:38:32.199957790Z INFO  thread_channel_sync_io   16  2] worker_thread:+
[2022-11-23T22:38:32.199994629Z INFO  thread_channel_sync_io::file_io_thread   70  2] file_io_thread:+
[2022-11-23T22:38:32.199999097Z INFO  thread_channel_sync_io   24  1] main:  sleeping 5ns
[2022-11-23T22:38:32.200014476Z INFO  thread_channel_sync_io::file_io_thread   75  2] Delete hello.txt
[2022-11-23T22:38:32.200059220Z INFO  thread_channel_sync_io   24  1] main:  sleeping 5ns
[2022-11-23T22:38:32.200078236Z INFO  thread_channel_sync_io::file_io_thread   79  2] Creating hello.txt
[2022-11-23T22:38:32.200116477Z INFO  thread_channel_sync_io   24  1] main:  sleeping 5ns
[2022-11-23T22:38:32.200155340Z INFO  thread_channel_sync_io::file_io_thread   24  2] hello.txt contains: "Hello World"
[2022-11-23T22:38:32.200173454Z INFO  thread_channel_sync_io   24  1] main:  sleeping 5ns
[2022-11-23T22:38:32.200186849Z INFO  thread_channel_sync_io::file_io_thread   35  2] open_existing_and_read_buf_at_a_time: file_name=hello.txt buf.len=5
[2022-11-23T22:38:32.200238035Z INFO  thread_channel_sync_io   24  1] main:  sleeping 5ns
[2022-11-23T22:38:32.200227015Z INFO  thread_channel_sync_io::file_io_thread   49  2] read 5 buf=[48, 65, 6C, 6C, 6F]
[2022-11-23T22:38:32.200252362Z INFO  thread_channel_sync_io::file_io_thread   49  2] read 5 buf=[20, 57, 6F, 72, 6C]
[2022-11-23T22:38:32.200273432Z INFO  thread_channel_sync_io::file_io_thread   49  2] read 1 buf=[64]
[2022-11-23T22:38:32.200294892Z INFO  thread_channel_sync_io   24  1] main:  sleeping 5ns
[2022-11-23T22:38:32.200294241Z INFO  thread_channel_sync_io::file_io_thread   91  2] len_read=11 len_last_buf=1
[2022-11-23T22:38:32.200310371Z INFO  thread_channel_sync_io::file_io_thread   93  2] file_io_thread:-
[2022-11-23T22:38:32.200324167Z INFO  thread_channel_sync_io   18  2] worker_thread:- file_io_thread=Ok(())
[2022-11-23T22:38:32.200382546Z INFO  thread_channel_sync_io   28  1] main:  joined  worker_thread.join()=Ok(())
[2022-11-23T22:38:32.200389209Z INFO  thread_channel_sync_io   30  1] main:-
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
