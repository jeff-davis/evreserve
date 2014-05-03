
use postgres::{PostgresConnection, NoSsl};
use postgres::types::ToSql;
use std::fmt::Show;

use postgres::types::range::Range;

pub fn get_content() -> Result<~str, ~str> {
  let mut content : StrBuf = StrBuf::new();
	content.push_str("<html><head><title>Trying Rust!</title></head><body>\n");
	content.push_str("<ul>\n");

  let conn = match PostgresConnection::connect("postgres://jdavis@localhost:5433/postgres", &NoSsl)
                   { Ok(r) => { r }, Err(e) => { return Err(format!("{}",e)) } };
  let stmt = match conn.prepare("SELECT ir FROM x")
                   { Ok(r) => { r }, Err(e) => { return Err(format!("{}",e)) } };
  let mut res = match stmt.query([])
                   { Ok(r) => { r }, Err(e) => { return Err(format!("{}",e)) } };
  for row in res {
    let val : Range<i32> = row[1];
    content.push_str(format!("<li>{}</li>\n", val));
  }

  content.push_str("</ul>\n");
  content.push_str("</body></html>\n");
  return Ok(content.into_owned());
}