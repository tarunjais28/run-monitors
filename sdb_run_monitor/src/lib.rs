#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::result_unwrap_used,
        clippy::panicking_unwrap,
        clippy::option_unwrap_used
    )
)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use fs2::FileExt;
use std::{
    fs,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Serialize, Deserialize)]
struct ProgramLog<'a> {
    prog_name: &'a str,
    count: usize,
}

pub enum Programs {
    Foo,
    Bar,
}

///
/// #Example
///
/// //This program fails to compile because `get_prog_name(enum)` is a private member.
/// ```compile_fails
/// extern crate sdb_run_monitor_file as lib;
///
/// use lib::*;
///
/// fn main() {
///     println!("{}",Programs::Foo.get_prog_name());
/// }
/// ```
impl Programs {
    fn get_prog_name(&self) -> &str {
        match self {
            Programs::Foo => "foo",
            Programs::Bar => "bar",
        }
    }
}

pub enum FileLogError {
    FileNotPresent,
    FailedToParseJSONToString,
    FailedToParseJSONFromString,
    FailedToReadFromFile,
    FailedToWriteFile,
    LockFailed,
    ReleaseLockFailed,
}

/// This function calculates number of times the program is executed. It takes program name as parameter and returns an updated
/// count of the program.
///
/// #Example
///
/// ```
/// extern crate sdb_run_monitor_file as lib;
/// use lib::*;
///
/// fn main() {
///    let count = match get_run_number(&Programs::Bar) {
///        Ok(c) => c,
///        Err(error) => panic!(error),
///    };
///    println!("{}", count);
/// }
/// ```
pub fn get_run_number(prog_enum: &Programs) -> Result<usize, FileLogError> {
    let log_file_path = "input.txt";
    let mut updated_count: usize = 0;

    let log_file = match File::open(log_file_path) {
        Ok(log) => log,
        Err(_) => return Err(FileLogError::FileNotPresent),
    };

    match log_file.lock_exclusive() {
        Ok(lock) => lock,
        Err(_) => return Err(FileLogError::LockFailed),
    };

    let mut updated_logs = String::new();

    for contents in BufReader::new(&log_file).lines() {
        let contents = match contents {
            Ok(read) => read,
            Err(_) => return Err(FileLogError::FailedToReadFromFile),
        };
        let mut program: ProgramLog = match serde_json::from_str(contents.as_str()) {
            Ok(read) => read,
            Err(_) => return Err(FileLogError::FailedToParseJSONFromString),
        };
        let prog_name = prog_enum.get_prog_name();
        if prog_name == program.prog_name {
            program.count += 1;
            updated_count = program.count;
        }
        let write_contents = match serde_json::to_string(&program) {
            Ok(write) => write,
            Err(_) => return Err(FileLogError::FailedToParseJSONToString),
        };
        updated_logs.push_str(write_contents.as_str());
        updated_logs.push('\n');
    }

    match fs::write(log_file_path, &updated_logs) {
        Ok(write) => write,
        Err(_) => return Err(FileLogError::FailedToWriteFile),
    };

    match log_file.unlock() {
        Ok(release) => release,
        Err(_) => return Err(FileLogError::ReleaseLockFailed),
    };

    Ok(updated_count)
}

#[cfg(test)]
mod tests {
    use super::Programs;

    #[test]
    fn test_get_prog_name() {
        let mut prog_name = Programs::Foo.get_prog_name();
        assert_eq!("foo", prog_name);
        prog_name = Programs::Bar.get_prog_name();
        assert_eq!("bar", prog_name);
    }

}
