use diesel;
use diesel::result::Error;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::schema::urls;

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "urls"]
pub struct Url {
	pub id: String,
	pub uri: String,
}

impl Url {
	pub fn create(url: Url, connection: &SqliteConnection) -> Result<usize, Error> {
		diesel::insert_into(urls::table)
			.values(&url)
			.execute(connection)
	}

	pub fn read(name: String, connection: &SqliteConnection) -> Result<Url, Error> {
		use crate::url::urls::dsl::*;
		urls.filter(id.eq(&name)).get_result(connection)
	}
}
