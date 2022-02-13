# Rust Full Websocket Example

### This is a boilerplate Rust Websocket Server with:

- socket.io-like message format:
  ```json
  {
    "event": "SOME_EVENT_NAME",
    "data"?: <ARBITRARY_DATA>
  }
  ```
- JSON serialization / deserialization to / from structs
- async / await support for event handlers
- argument parsing
- logging

### To run:

```sh
cargo run
```

### To test:

- Open a new tab in Chrome
- Copy-paste the code from [scripts/test-server.js](scripts/test-server.js)
