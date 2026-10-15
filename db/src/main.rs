// A function with a database. Deploy this folder (set "Folder" to db), put
// DATABASE_URL in its environment, and every call records a visit and reports
// the count. The table is made on first use.
use barua_fn::{execute, query, Error};
use serde_json::{json, Value};
use std::io::Read;

fn visit(who: &str) -> Result<Value, Error> {
    execute("create table if not exists visits (id bigserial primary key, who text not null, at timestamptz not null default now())", &[])?;
    execute("insert into visits (who) values ($1)", &[json!(who)])?;
    let total = query("select count(*)::int as total, max(at) as last from visits", &[])?;
    let recent = query("select who, at from visits order by id desc limit 5", &[])?;
    Ok(json!({ "visitor": who, "total": total[0]["total"], "last": total[0]["last"], "recent": recent }))
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).ok();
    let who = input.trim();
    let who = if who.is_empty() { "someone" } else { who };
    let out = match visit(who) {
        Ok(v) => v,
        Err(e) => json!({ "error": e.to_string(), "hint": "set DATABASE_URL in the function's environment" }),
    };
    print!("{out}");
}
