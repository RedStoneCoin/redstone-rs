extern crate clap;
use clap::{Arg, App, SubCommand};
/// Simple program to greet a person


pub fn cli() ->  Vec<String>{
    let matches =  App::new("Redstone Node")
                        .version("0.1.0")
                        .author("Redstone Developers. <redstonecrypto@gmail.com>")
                        
                        .about("Redstone Deamon Software")
                          .arg(Arg::with_name("validator")
                          .long("validator") // allow --name
                          .takes_value(true)
                          .help("validator")
                          .required(false))
                          .arg(Arg::with_name("mode")
                          .long("mode") // allow --name
                          .takes_value(true)
                          .help("mode")
                          .required(false))
                          .arg(Arg::with_name("logging")
                          .long("logging") // allow --name
                          .takes_value(true)
                          .help("logging level")
                          .required(false))
                          .get_matches();
    
    // return vec of args
    let mut args = Vec::new();
    args.push(matches.value_of("validator").unwrap_or("").to_string());
    args.push(matches.value_of("mode").unwrap_or("").to_string());
    args.push(matches.value_of("logging").unwrap_or("").to_string());
    args
    
}