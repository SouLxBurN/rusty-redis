# Rusty Redis Project

Initially Followed https://build-your-own.org/redis and attempted to build my own redis in Rust instead of C.

## Structure

Project is diveded into three subprojects
- Core
    - Contains server commands and serializing/deserializing code.
- Server
    - Rusty Redis server binary, and accompanied storage structures.
- Client
    - Client implementation in order to connect to Rusty Redis.

## Server Startup
Start the server by running
```
cargo run --bin server
```

## Connecting to the Server
Import rusty_redis_client crate. Add the following to your Cargo.toml. Updating the path accordingly.
```
rusty-redis-client = { path = "../rusty-redis/client" }
```

Establishing a connection:
```
if let Ok(mut conn) = rusty_redis_client::connect("localhost:8080").await
```

A more in depth example is located [here](./client/examples/client.rs)

----
Built Live on Twitch @ twitch.tv/soulxburn
