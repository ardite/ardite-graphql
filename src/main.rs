extern crate ansi_term;
#[macro_use(crate_version)]
extern crate clap;
extern crate hyper;
extern crate iron;
extern crate logger;
extern crate time;

mod server;

use ansi_term::Style;
use clap::{App, AppSettings, Arg};
use iron::{Iron, Chain};
use logger::Logger;
use server::Server;

fn main() {
  let matches = {
    App::new("Ardite GraphQL Server")
    .version(crate_version!())
    .author("Caleb Meredith <calebmeredith8@gmail.com>")
    .about("An Ardite service for a GraphQL server.")
    .version_short("v")
    .setting(AppSettings::UnifiedHelpMessage)
    .args(&[
      Arg::with_name("hostname").long("hostname").short("n").takes_value(true).value_name("STRING").help("The host name that the server will listen on"),
      Arg::with_name("port").long("port").short("p").takes_value(true).value_name("PORT").help("The port that the server will listen on"),
      Arg::with_name("enable_graphiql").long("graphiql").short("i").help("Enable the serving of a GraphiQL interface to your GraphQL server")
    ])
    .get_matches()
  };

  let hostname = matches.value_of("hostname").unwrap_or("localhost");
  let port = matches.value_of("port").unwrap_or("3002").parse::<u16>().unwrap();

  let server = Server {
    enable_graphiql: matches.is_present("enable_graphiql")
  };

  let mut chain = Chain::new(server);
  let (logger_before, logger_after) = Logger::new(None);

  chain.link_before(logger_before);
  chain.link_after(logger_after);

  println!("GraphQL server listening on {}", Style::new().underline().paint(format!("http://{}:{}", hostname, port)));
  Iron::new(chain).http((hostname, port)).unwrap();
}
