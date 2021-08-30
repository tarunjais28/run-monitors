extern crate diesel;
extern crate sdb_run_monitoring;

use self::models::*;
use diesel::prelude::*;
use sdb_run_monitoring::*;

fn main() {
    use self::schema::logs::dsl::*;

    let connection = establish_connection();
    let results = logs.load::<Logs>(&connection).expect("Error loading logs");

    println!("Displaying {} logs", results.len());
    for log in results {
        println!("{}: {}", log.programs, log.count);
    }
}
