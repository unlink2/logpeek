use logpeek::{Error, Interface};

fn main() -> Result<(), Error> {
    let interface = Interface::from_args()?;
    println!("{}", interface.check()?);
    Ok(())
}
