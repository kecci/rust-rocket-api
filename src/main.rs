#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello!"
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "World!"
}

// Namespacing
mod other {
    #[get("/welcome")]
    pub fn welcome() -> &'static str {
        "Welcome!"
    }
}
// use other::welcome;

// Dynamic Paths
#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name.as_str())
}

// Query Strings
#[get("/hi?<name>")]
fn hi(name: String) -> String {
    format!("Hi, {}!", name.as_str())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, world, other::welcome, hello, hi])
        .launch();
}