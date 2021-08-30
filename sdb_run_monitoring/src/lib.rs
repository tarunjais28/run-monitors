#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection, EstablishConnectionError> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL"){
        Ok(valid) => valid,
        Err(e) => return Err(EstablishConnectionError::DBNotPresent)
    };

    
    match SqliteConnection::establish(&database_url) {
        Ok(c) => Ok(c),
        Err(e) => Err(EstablishConnectionError::DBConnectionError(e))
    }
        
}

pub enum EstablishConnectionError {
    DBNotPresent,
    DBConnectionError(ConnectionError)
}

pub enum Programs {
    Foo,
    Bar,
}

/// This function take program name as enum type and returns string version of program name.
/// 
/// #Example
/// 
/// //This program fails to compile because `get_prog_name(enum)` is a private member.
/// ```compile_fails
/// extern crate sdb_run_monitoring;
/// 
/// use sdb_run_monitoring::*;
/// 
/// fn main() {
///     println!("{}",Programs::Foo.get_prog_name());
/// }
/// ```
impl Programs {
    fn get_prog_name(&self)->&str{
        match self {
            Programs::Foo => "foo",
            Programs::Bar => "bar",
        }
    }
}

/// This function calculates number of times the program is executed. It takes program name as parameter and returns an updated 
/// count of the program.
/// 
/// #Example
/// 
/// ```
/// extern crate diesel;
/// extern crate sdb_run_monitoring as lib;
/// use lib::*;
/// 
/// fn main() {
///    println!("{}", get_run_num(Programs::Foo));
/// }
/// ```
pub fn get_run_num(prog_enum: Programs) -> Result<i32,EstablishConnectionError> {
    let updated_count = schema::logs::dsl::count + 1;
    
    let prog_name = prog_enum.get_prog_name();

    let connection  = match establish_connection(){
        Ok(c) => c,
        Err(e) => return Err(e)
    };

    let _ = diesel::update(schema::logs::dsl::logs.find(&prog_name))
        .set(schema::logs::dsl::count.eq(updated_count))
        .execute(&connection)
        .expect("Unable to find program name!");

    let log: models::Logs = schema::logs::dsl::logs
        .find(&prog_name)
        .first(&connection)
        .unwrap();

    Ok(log.count)
}

pub enum GetValError {
    ValueInDBNotInt,
    ValueNotPresent
}

#[cfg(test)]
mod tests {
    extern crate diesel;
    use super::establish_connection;
    use super::Programs;

    #[test]
    fn test_establish_connection() {
        let _ = establish_connection();
    }

    #[test]
    fn test_get_prog_name() {
        let mut prog_name = Programs::Foo.get_prog_name();
        assert_eq!("foo", prog_name);
        prog_name = Programs::Bar.get_prog_name();
        assert_eq!("bar", prog_name);
    }

}
