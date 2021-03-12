# hello-rocket

## Installing Rust
```
rustup default nightly
```

```
rustup override set nightly
```

## Hello, world!
```
cargo new hello-rocket --bin
cd hello-rocket
```

in `Cargo.toml`:
```
[dependencies]
rocket = "0.4.7"
```

in `src/main.rs`
```
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
```

output:
```
🔧  Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: [logical cores * 2]
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => read timeout: 5s
    => write timeout: 5s
    => tls: disabled
🛰  Mounting '/':
    => GET / (index)
🚀  Rocket has launched from http://localhost:8000
```

visit http://localhost:8000

source: https://rocket.rs/v0.4/guide/requests/