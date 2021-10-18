use logpeek::{Error, Interface};

fn main() -> Result<(), Error> {
    let mut stdout = std::io::stdout();

    let interface = Interface::from_args()?;
    interface.check(&mut stdout)?;
    interface.print_json(&mut stdout)?;

    Ok(())
}
