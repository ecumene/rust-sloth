use clap::{App, Arg, ArgMatches, SubCommand};
use crate::geometry::{SimpleMesh, ToSimpleMeshWithMaterial};
use crate::context::{Context};
use std::error::Error;

pub fn cli_matches<'a>() -> ArgMatches<'a> {
    commands_for_subcommands(App::new("Sloth")
        .version("0.1")
        .author("Mitchell Hynes. <mshynes@mun.ca>")
        .about("A toy for rendering 3D objects in the command line")
        .subcommand(commands_for_subcommands(SubCommand::with_name("image")
            .about("Generates a colorless terminal output as lines of text")
            .author("Mitchell Hynes <mitchell.hynes@ecumene.xyz>")
            .arg(
                Arg::with_name("frame count")
                    .short("j")
                    .long("webify")
                    .help("Generates a portable JS based render of your object for the web")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("width")
                    .short("w")
                    .help("Sets the width of the image to generate")
                    .takes_value(true)
                    .required(true)
            )
            .arg(
                Arg::with_name("height")
                    .short("h")
                    .help("Sets the height of the image to generate")
                    .takes_value(true)
            )))
        .arg(
            Arg::with_name("INPUT FILENAME")
                .help("Sets the input file to render")
                .required(true)
                .index(1)
        ))
        .get_matches()
}

fn commands_for_subcommands<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    command_flag_color(command_rotates(app))
}

fn command_flag_color<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("no_color")
            .short("b")
            .help("Flags the rasterizer to render without color")
    )
}

fn command_rotates<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("x")
            .short("x")
            .long("yaw")
            .help("Sets the object's static X rotation (in radians)")
            .takes_value(true)
    )
    .arg(
        Arg::with_name("y")
            .short("y")
            .long("pitch")
            .help("Sets the object's static Y rotation (in radians)")
            .takes_value(true)
    )
    .arg(
        Arg::with_name("z")
            .short("z")
            .long("roll")
            .help("Sets the object's static Z rotation (in radians)")
            .takes_value(true)
    )
}

pub fn to_meshes(models: Vec<tobj::Model>, materials: Vec<tobj::Material>) -> Vec<SimpleMesh> {
    let mut meshes: Vec<SimpleMesh> = vec![];
    for model in models {
        meshes.push(model.mesh.to_simple_mesh_with_materials(&materials));
    }
    meshes
}

pub fn match_turntable(matches: &ArgMatches) -> Result<(f32, f32, f32, f32), Box<Error>> {
    let mut turntable = (0.0, 0.0, 0.0, 0.0);
    if let Some(x) = matches.value_of("x")  {
        turntable.0 = x.parse()?;
    }
    if let Some(y) = matches.value_of("y")  {
        turntable.1 = y.parse()?;
    }
    if let Some(z) = matches.value_of("z") {
        turntable.2 = z.parse()?;
    }
    if let Some(s) = matches.value_of("speed") {
        turntable.3 = s.parse()?;
    } else {
        turntable.3 = 1.0; // No speed defined -> 1.0 rad/s
    }
    turntable.1 += 3.14159; // All models for some reason are backwards, this fixes that
    Ok(turntable)
}

pub fn match_image_mode(matches: &ArgMatches) -> bool {
    matches.is_present("image")
}

pub fn match_no_color_mode(matches: &ArgMatches) -> bool {
    matches.is_present("no_color")
}

pub fn match_dimensions<'a>(context: &mut Context, matches: &ArgMatches) -> Result<(), Box<Error>> {
    if let Some(x) = matches.value_of("width") {
        context.width = x.parse()?;
        if let Some(y) = matches.value_of("height") {
            context.height = y.parse()?;
        } else {
            context.height = context.width;
        }
    }
    Ok(())
}
