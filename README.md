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
