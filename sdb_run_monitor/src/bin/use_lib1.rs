use sdb_run_monitor_file::*;

fn main() {
    let s = match get_run_number(&Programs::Bar) {
        Ok(s) => s,
        Err(error) => panic!(error),
    };
    println!("{}", s);
}
