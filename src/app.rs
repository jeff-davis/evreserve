
extern crate postgres;

use postgres::{PostgresConnection, NoSsl};
use postgres::types::ToSql;

pub fn get_content() -> ~str {
  let mut content : ~str = ~"";
	content.push_str("<html><head><title>Trying Rust!</title></head><body>\n");
	content.push_str("<ul>\n");

  {
    let conn = PostgresConnection::connect("postgres://jdavis@localhost:5432/postgres", &NoSsl).unwrap();
    let stmt = conn.prepare("SELECT x FROM t WHERE $1").unwrap();
    for row in stmt.query([&true as &ToSql]).unwrap() {
      let val : i32 = row[1];
      content.push_str(format!("<li>{}</li>\n", val));
    }
  }

  content.push_str("</ul>\n");
  content.push_str("</body></html>\n");
  return content;
}