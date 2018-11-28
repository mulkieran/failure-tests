extern crate failure;

extern crate devicemapper;

extern crate rust_project;

use std::io::Error;

use devicemapper as dm;
use devicemapper::DmError;

use rust_project::library::errors;

fn error_return() -> errors::StratisResult<()> {
    Err(Error::last_os_error())?
}

// fn dm_error_return() -> errors::StratisResult<()> {
//     Err(DmError::Core(dm::errors::IoctlResultTooLargeError))?
// }

fn main() {
    match error_return() {
        Err(err) => {
            println!("{:?}", err);
            match err.downcast::<Error>() {
                Ok(io_error) => {
                    println!("It's an IO error!");
                    println!("Its kind is: {:?}", io_error.kind());
                }
                Err(_) => println!("All that downcasting for nuthin!"),
            }
        },
        Ok(_) => {},
    }
}
