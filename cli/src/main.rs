use logpeek::{Error, Interface};

fn handle_error<T>(res: Result<T, Error>) -> Result<T, (exitcode::ExitCode, String)> {
    match res {
        Ok(res) => Ok(res),
        Err(err) => match err {
            Error::RegexError(err) => Err((exitcode::DATAERR, err.to_string())),
            Error::IOError(err) => Err((exitcode::IOERR, err.to_string())),
            Error::SerdeJsonError(err) => Err((exitcode::DATAERR, err.to_string())),
        },
    }
}

fn main() {
    let mut stdout = std::io::stdout();

    let interface = match handle_error(Interface::from_args()) {
        Ok(interface) => interface,
        Err(tup) => {
            println!("{}", tup.1);
            std::process::exit(tup.0);
        }
    };
    match handle_error(interface.check(&mut stdout)) {
        Ok(_) => (),
        Err(tup) => {
            println!("{}", tup.1);
            std::process::exit(tup.0);
        }
    }
    match handle_error(interface.print_json(&mut stdout)) {
        Ok(_) => (),
        Err(tup) => {
            println!("{}", tup.1);
            std::process::exit(tup.0);
        }
    }
}
