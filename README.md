# Rust Full Websocket Example

### This is a boilerplate Rust Websocket Server with:

- socket.io-like message format:
  ```json
  {
    "event": "SOME_EVENT_NAME",
    "data"?: <ARBITRARY_DATA>
  }
  ```
- **JSON serialization** / **deserialization** to / from structs with [serde](https://docs.rs/serde/latest/serde/) and [serde_json](https://docs.rs/serde_json/latest/serde_json/)
  - See [msg_in.rs](src/msg/msg_in.rs) for a list of all incoming events
- **async** / **await** event handlers with [tokio-tungstenite](https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/)
- **argument parsing** with [clap](https://docs.rs/clap/latest/clap/)
- **logging** with [log](https://docs.rs/log/latest/log/) and [env_logger](https://docs.rs/env_logger/latest/env_logger/)

### To run:

```sh
cargo run
```

### To test:

- Open a new tab in Chrome
- Copy-paste the code from [test-server.js](scripts/test-server.js)

### To modify:

- Add your own incoming and outgoing messages to [msg_in.rs](src/msg/msg_in.rs) and [msg_out.rs](src/msg/msg_out.rs) respectively
- Add your own message handlers to `src/handler`
- Register message handlers in [main.rs](src/main.rs)::handle_event
- Add args in [args.rs](src/args.rs)

### Misc:

- This was based off of [autobahn-server](https://github.com/snapview/tokio-tungstenite/blob/master/examples/autobahn-server.rs).
- This code can be slightly modified to become a CLIENT instead of a SERVER. See [autobahn-client](https://github.com/snapview/tokio-tungstenite/blob/master/examples/autobahn-client.rs).
