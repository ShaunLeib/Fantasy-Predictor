use rocket::*;
use rocket::fs::NamedFile;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("../../svelte/src/App.svelte").await.ok()
}

#[get("/pre")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("../../svelte/src/Pre.svelte").await.ok()
}

#[get("/weekly")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("../../svelte/src/Weekly.svelte").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
