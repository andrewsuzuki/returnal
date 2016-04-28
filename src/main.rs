#[macro_use]

extern crate ws;
extern crate clap;
extern crate rustc_serialize;
extern crate returnal;

use clap::App;
use clap::Arg;
use ws::listen;
use returnal::{Editor};

pub fn version() -> String {
    format!("{}.{}.{}{}",
                     env!("CARGO_PKG_VERSION_MAJOR"),
                     env!("CARGO_PKG_VERSION_MINOR"),
                     env!("CARGO_PKG_VERSION_PATCH"),
                     option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
}

fn main() {
    // get version number
    let version = version();

    // configure clap to parse cli arguments
    let matches = App::new("returnal")
                        .version(version.as_ref())
                        .author("andrew suzuki")
                        .about("editor server")
                        .arg(Arg::with_name("port")
                                    .short("p")
                                    .long("port")
                                    .value_name("PORT")
                                    .help("server port")
                                    .takes_value(true))
                        .get_matches();

    // determine port number based on argument
    let port: u16 = matches.value_of("port")
                      .unwrap_or("3443")
                      .parse::<u16>()
                      .unwrap_or(3443);
    println!("will use port {}", port);

    // listen for websocket connections on specified port
    if let Err(error) = listen(("127.0.0.1", port), |out| {
        // create editor
        let mut editor = Editor::new(out);

        // forward received messages to editor
        move |msg| editor.receive(msg)
    }) {
        println!("failed to create websocket: {:?}", error);
    }
}
