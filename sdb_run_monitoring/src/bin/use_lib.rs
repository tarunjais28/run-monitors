extern crate diesel;
extern crate sdb_run_monitoring;

use sdb_run_monitoring::*;

fn main() {
    let _ = establish_connection();
    println!("{}", get_run_num(Programs::Foo));
    println!("{}", get_run_num(Programs::Bar));
}
