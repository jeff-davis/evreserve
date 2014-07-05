
use postgres::{PostgresConnection, NoSsl};
use postgres::types::ToSql;
use std::fmt::Show;

use postgres::types::range::{Range, RangeBound, Inclusive, Exclusive};

pub fn get_content() -> Result<String, String> {
  let mut content : String = String::new();
	content.push_str("<html><head><title>Trying Rust!</title></head><body>\n");
	content.push_str("<ul>\n");

  let conn = match PostgresConnection::connect("postgres://jdavis@localhost:5433/postgres", &NoSsl)
                   { Ok(r) => { r }, Err(e) => { return Err(format!("{}",e)) } };
  let stmt = match conn.prepare("SELECT ir FROM x")
                   { Ok(r) => { r }, Err(e) => { return Err(format!("{}",e)) } };
  let mut res = match stmt.query([])
                   { Ok(r) => { r }, Err(e) => { return Err(format!("{}",e)) } };

  let q = range!('[' 50i32, 60i32 ')');

  for row in res {
    let mut color = "green";
    let val : Range<i32> = row[0u];
    if !q.intersect(&val).is_empty()
    {
       color = "red";
    }

    content = content + format!("<li><font color={}>{}</font></li>\n", color, val);
  }

  content.push_str("</ul>\n");
  content.push_str("</body></html>\n");
  return Ok(content.into_string());
}
