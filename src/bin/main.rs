extern crate clap;

use fireplace_rs::server::server;
use fireplace_rs::fireplace;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = clap::App::new("fireplace")
        .arg(clap::Arg::with_name("flame")
             .long("flame")
             .takes_value(false)
             .help("Turn on the flame"))
        .arg(clap::Arg::with_name("fan")
             .long("fan")
             .takes_value(false)
             .help("Turn on the blower fan"))
        .subcommand(clap::SubCommand::with_name("server")
            .about("starts the fireplace http server")
            .arg(clap::Arg::with_name("address")
                 .help("The hostname and port where the server will bind, eg `127.0.0.1:8000`")
                 .default_value("127.0.0.1:8000")))
        .get_matches();

    // initialize the fireplace
    let flame = matches.is_present("flame");
    let fan = matches.is_present("fan");
    let mut fp_state = fireplace::State::new().expect("unable to communicate with raspberry pi");
    fp_state.set(fan, flame).expect("unable to set initial fireplace state");

    match matches.subcommand() {
        ("server", Some(server_matches)) => {
            println!("run server with initial state flame: {}, fan: {}", flame, fan);
            server::run(
                server_matches.value_of("address").expect("should always be populated"),
                fp_state,
            ).await?;
        }
        _ => {
            println!("fireplace state set to flame: {}, fan: {}", flame, fan);
        }
    }
    Ok(())
}
