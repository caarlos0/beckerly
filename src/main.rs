#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::databases::diesel as rdiesel;
use rocket::request::Form;
use rocket::response::Redirect;

mod schema;
mod url;
use url::Url;

#[database("urls_db")]
struct UrlsDbConn(rdiesel::SqliteConnection);

#[get("/<name>")]
fn redirect(conn: UrlsDbConn, name: String) -> Result<Redirect, &'static str> {
	match Url::read(name, &conn) {
			Ok(url) => Ok(Redirect::permanent(url.uri)),
			_ => Err("Requested URL was not found")
		}
}

#[derive(FromForm)]
struct FormUrl {
	url: String,
}

#[post("/<name>", data = "<url_form>")]
fn shorten(conn: UrlsDbConn, name: String, url_form: Form<FormUrl>) -> Result<&'static str, &'static str> {
	let ref url = url_form.url;
	match Url::create(Url{
		id: name,
		uri: url.to_string(),
	}, &conn) {
		Ok(_size) => Ok("created"),
		_ => Err("Failed to create URL")
	}
}

fn main() {
	rocket::ignite()
		.attach(UrlsDbConn::fairing())
		.mount("/", routes![redirect, shorten])
		.launch();
}
