use logpeek::{Error, Interface};

fn main() -> Result<(), Error> {
    let interface = Interface::from_args()?;
    println!("{}", interface.check()?);

    if let Some(output) = interface.print_json()? {
        println!("{}", output);
    }

    Ok(())
}
