# NOTES

actixweb sample

change `.bind("127.0.0.1:8080")?` to `.bind("0.0.0.0:8080")?` in `main.rs`

```shell
# win1
$ cd websocket
# run server
$ cargo run --bin websocket-server

# win2
$ cd websocket
# run client with binserve
$ binserve
```
