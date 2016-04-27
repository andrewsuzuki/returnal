#[macro_use]

extern crate ws;
extern crate clap;
extern crate rustc_serialize;

use clap::App;
use clap::Arg;

use ws::listen;

use rustc_serialize::json;

pub fn version() -> String {
    format!("{}.{}.{}{}",
                     env!("CARGO_PKG_VERSION_MAJOR"),
                     env!("CARGO_PKG_VERSION_MINOR"),
                     env!("CARGO_PKG_VERSION_PATCH"),
                     option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
}

#[derive(Copy, Clone, RustcEncodable)]
struct Cursor {
    x: usize,
    y: usize
}

impl Cursor {
    fn new(x: usize, y: usize) -> Self {
        Cursor { x: x, y: y }
    }
}

#[derive(Copy, Clone, RustcEncodable)]
struct Editor {
    cursor: Cursor
}

impl Editor {
    fn new() -> Self {
        Editor {
            cursor: Cursor::new(0, 0)
        }
    }
}

fn main() {
    let version = version();

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

    let port: u16 = matches.value_of("port")
                      .unwrap_or("3443")
                      .parse::<u16>()
                      .unwrap_or(3443);

    println!("will use port {}", port);
    println!("press ctrl-c to quit");

    let mut editor = Editor::new();

    if let Err(error) = listen(("127.0.0.1", port), |out| {

        move |msg| {
            println!("received message '{}'", msg);

            let response = json::encode(&editor).unwrap();

            out.send(response)
        }

    }) {
        println!("failed to create websocket: {:?}", error);
    }
}
