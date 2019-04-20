use clap::{App, Arg, ArgMatches};

pub fn cli_matches<'a>() -> ArgMatches<'a> {
    App::new("My Super Program")
        .version("0.1")
        .author("Mitchell Hynes. <mshynes@mun.ca>")
        .about("A toy for rendering 3D objects in the command line")
        .arg(
            Arg::with_name("OBJ INPUT")
                .help("Sets the input obj file to render")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
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
