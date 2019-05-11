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
            Arg::with_name("image")
                .short("i")
                .long("image")
                .help("Generates an image instead of realtime raw terminal output with a square resolution"),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .help("Sets the width of the image to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .help("Sets the height of the image to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("speed")
                .short("s")
                .long("turntable")
                .help("Sets the automatic turntable speed (radians / second in the x direction)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("x")
                .short("x")
                .long("x rotation")
                .help("Sets the object's static X rotation (in radians)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("y")
                .short("y")
                .long("y rotation")
                .help("Sets the object's static Y rotation (in radians)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("z")
                .short("z")
                .long("z rotation")
                .help("Sets the object's static Z rotation (in radians)")
                .takes_value(true),
        )
        .get_matches()
}
