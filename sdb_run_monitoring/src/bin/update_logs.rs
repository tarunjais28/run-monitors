extern crate diesel;
extern crate sdb_run_monitoring;

use self::diesel::prelude::*;
use self::sdb_run_monitoring::*;

fn main() {
    use sdb_run_monitoring::schema::logs::dsl::{count, logs};

    let id = "foo";
    let update_count = count + 1;

    let connection = establish_connection();
    let _ = diesel::update(logs.find(id))
        .set(count.eq(update_count))
        .execute(&connection)
        .unwrap_or_else(|_| panic!("Unable to find log {}", id));

    let log: models::Logs = logs
        .find(id)
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find log {}", id));
    println!("{:?}", log.count);
}
