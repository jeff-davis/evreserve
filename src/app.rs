
extern crate postgres;

use postgres::{PostgresConnection, NoSsl};
use postgres::types::ToSql;

pub fn get_content() -> ~str {
  let mut content : ~str = ~"";
	content.push_str("<html><head><title>Trying Rust!</title></head><body>\n");
	content.push_str("<ul>\n");

  let conn = PostgresConnection::connect("postgres://jdavis@localhost:5432/postgres", &NoSsl);
	{
	  let stmt = conn.prepare("SELECT x FROM t");
	  for row in stmt.query([]) {
      let val : i32 = row[1];
	  	content.push_str(format!("<li>{}</li>\n", val));
    }
  }
	conn.finish();

	content.push_str("</ul>\n");
	content.push_str("</body></html>\n");
	return content;
}