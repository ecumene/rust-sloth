use clap::{App, Arg, ArgMatches};

pub fn cli_matches<'a>() -> ArgMatches<'a> {
    App::new("My Super Program")
        .version("0.1")
        .author("Mitchell Hynes. <mshynes@mun.ca>")
        .about("A toy for rendering 3D objects in the command line")
        .arg(
            Arg::with_name("INPUT FILENAME")
                .help("Sets the input file to render")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("resolution")
                .short("i")
                .long("image")
                .help("Generates an image instead of realtime raw terminal output with a square resolution")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("turntable")
                .short("s")
                .long("turntable")
                .takes_value(true),
        )
        .help("Sets the automatic turntable speed (radians / second in the x direction)")
        .arg(
            Arg::with_name("rotation")
                .short("r")
                .long("rotation")
                .takes_value(true),
        )
        .help("Sets the object's static rotation (in degrees)")
        .get_matches()
}
