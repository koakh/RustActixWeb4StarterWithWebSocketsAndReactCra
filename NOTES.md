# NOTES

## TLDR MD Notes

- `'Rust - ActixWeb v4.0 - Add React Project'`

## TLDR Debug

just stop service and launch `Debug executable 'actixweb4-starter'`

## TLDR Run Project

```shell
# win1 server
$ make startServer
```

```shell
# win2 actixweb web client
$ make startClient
```

goto <http://192.168.1.120:1337>

```shell
# win3 react web client
$ make startReactClient
```

goto <https://c3edu.online:3000/>

> used https

> require node v16.13.2

## Useful commands

```shell
# launch cargo run
$ cargo run -- $(ls /var/log/docker/c3-*.log)
$ sudo target/debug/tail $(ls /var/log/docker/c3-*.log)

$ curl -X GET localhost:8080
$ curl -X POST localhost:8080/filter \
  -H "Content-Type: application/json" \
  -d '{ "filterFile": "c3-microcloud-backend", "filterLine": "mongo" }'
$ curl -X POST localhost:8080/ws-echo \
  -H "Content-Type: application/json" \
  -d '{ "message": "c3-microcloud-backend" }'

# send to log
$ sudo sh -c "echo HttpModule >> /var/log/docker/c3-microcloud-backend.log"
```

## Links

- [Actix-Web: Run service every 10 seconds](https://stackoverflow.com/questions/64026629/actix-web-run-service-every-10-seconds)

## Discord question #1: awesome tip

solution is using `actix-web = "4.0.0-beta.19"` beta that uses tokio 1.x runtime

error: thread 'main' panicked at 'there is no reactor running, must be called from the context of a Tokio 1.x runtime'
answear: you might wanna try using newest beta version becuase it depends on 1.x tokio

Hello fellows

Im trying to create a simple actix web server with `linemux::MuxedLines`
in the end is something like a rest + tail +websockets service
we receive file changes with MuxedLines  and output it to stdout and websockets

the problem that I'm having is
how can I add this simple code block to #[actix_web::main]

when I try wrongly to add lines `lines.add_file(&f).await?;` I have the thread 'main' panicked at 'there is no reactor running, must be called from the context of a Tokio 1.x runtime'

simple working poc app to listen passed arguments, and tail it based on filter
ex tail file1 file2 file3

```rust
use linemux::MuxedLines;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
  let args = Cli::from_args();
  let mut lines = MuxedLines::new()?;
  let filter = "mongo";
  for f in args {
    lines.add_file(&f).await?;
  }

  while let Ok(Some(line)) = lines.next_line().await {
    if filter && line.source().display().to_string().to_lowercase().contains(filter) {
      println!("source: {}, line: {}", line.source().display(), line.line());
    }
  }
  Ok(())
}
```

## Discord question #2: use reference of AppStateGlobal member

hello I'm wrap my head a problem about actix web and references

fix with `clone the arc` -> `let ref_filter_file = &data.filter_file;`

```rust
let initial_filter_file = String::from("backend");
let initial_filter_line = String::from("mongo");
let data = web::Data::new(AppStateGlobal {
  counter: Mutex::new(0),
  filter_file: Arc::new(Mutex::new(String::from(initial_filter_file.clone()))),
  filter_line: Arc::new(Mutex::new(String::from(initial_filter_line.clone()))),
});

// spawn loop in parallel thread with async
let ref_filter_file = &data.filter_file;
spawn(async move {
  while let Ok(Some(line)) = lines.next_line().await {
    println!("data: {:?}", &ref_filter_file);
  }
});
```

the problem is how can I create a readonly reference to a AppState member ex to use inside spawn(async move loop
any help is appreciatted

what I want is to change the AppState that have the filters in a rest endpoint and use the if with readonly reference to output it or not 

The0x539 — Hoje às 17:45
**clone the arc**

## RegEx

- [regex](https://docs.rs/regex/latest/regex/)

```rust
use regex::Regex;
let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
assert!(re.is_match("2014-01-01"));
```

`^.*c3-microcloud-.*\.log$`

```shell
/var/log/docker/c3-microcloud-backend.log
/var/log/docker/c3-microcloud-frontend.log
/var/log/docker/c3-system-service-mongo.log
/var/log/docker/docker.log
/var/log/docker/c3-microcloud-cloud-client.log
/var/log/docker/c3-system-service-kiwix.log
/var/log/docker/c3-system-service-syncthing.log
```

## WebSockets beta

- [unable to use websocket with version 4 beta](https://github.com/actix/actix-web/discussions/2140)

```shell
actix = "0.11.1"
actix-web = { version = "4.0.0-beta.5", features = ["rustls"] }
actix-rt = "2.2.0"
actix-multipart = "0.4.0-beta.4"
actix-cors = "0.6.0-beta.1"
actix-service = "2.0.0-beta.5"
actix-session = "0.5.0-beta.1"
actix-files = "0.6.0-beta.4"
actix-web-actors = "4.0.0-beta.4"
actix-codec = "0.4.0-beta.1"
actix-utils = "3.0.0-beta.4"
```

[websocket](https://github.com/actix/examples/tree/master/websockets/websocket) example with latest beta crates work......

```toml
[dependencies]
actix = "0.12.0"
actix-codec = "0.4.2"
actix-web = "4.0.0-beta.19"
actix-web-actors = "4.0.0-beta.10"
```

## WebSocket : Client Connection Error

```shell
(index):20 WebSocket connection to 'ws://c3edu.online:8080/ws/' failed: Error during WebSocket handshake: Unexpected response code: 500
```

enable actix debug level `std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");`

```shell
[2022-01-12T12:11:02Z DEBUG actix_web::data] Failed to extract `Data<actix::address::Addr<tail::websocket::server::Server>>` for `/ws/` handler. For the Data extractor to work correctly, wrap the data with `Data::new()` and pass it to `App::app_data()`. Ensure that types align in both the set and retrieve calls.
```

now we see useful info

solution we must wrap everything with Data::new() ex

```rust
.app_data(Data::new(AppState {
  server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
  request_count: Cell::new(0),
  // filter,
}))
// global data
.app_data(Data::new(data.clone()))
// inject ws_server in context
.app_data(Data::new(ws_server.clone()))
```

## React HTTPS : `ERR_CERT_AUTHORITY_INVALID`

the trick is open https url on chrome or chrome debugger at <https://192.168.1.120:8543/>, and accept certificate like we do normally with advance in invalid certificates, and it start to work in chrome debug mode, and in non chrome debugger.

## Custom React Compnent

- [React container that will auto scroll to bottom](https://bestofreactjs.com/repo/compulim-react-scroll-to-bottom-react-custom-scrollbar)

- [Auto scroll to the bottom in a react chat application](https://www.cluemediator.com/auto-scroll-to-the-bottom-in-a-react-chat-application)

## Upload Files

```shell
$ sudo sh -c "tar -zcf /tmp/logs.tgz --absolute-names \
  /var/log/docker/c3-microcloud-backend.log \
  /var/log/docker/c3-microcloud-frontend.log \
  /var/log/docker/c3-microcloud-backend.log \
  /var/log/docker/c3-microcloud-cloud-client.log \
  /var/log/docker/c3-microcloud-frontend.log \
  /var/log/docker/c3-microcloud-fronten.log \
  /var/log/docker/c3-system-service-kiwix.log \
  /var/log/docker/c3-system-service-mongo.log \
  /var/log/docker/c3-system-service-syncthing.log"
```
