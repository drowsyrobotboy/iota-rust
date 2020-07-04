#[get("/hello/<name>")]
pub fn hello(name: String) -> String {
    format!("Hello there, {}!", name)
}