use crate::context::Context;
use crate::geometry::{SimpleMesh, ToSimpleMesh, ToSimpleMeshWithMaterial};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;

pub fn cli_matches<'a>() -> ArgMatches<'a> {
    commands_for_subcommands(
        App::new("Sloth")
            .version("0.1")
            .author("Mitchell Hynes. <mshynes@mun.ca>")
            .about("A toy for rendering 3D objects in the command line")
            .subcommand(commands_for_subcommands(
                SubCommand::with_name("image")
                    .about("Generates a colorless terminal output as lines of text")
                    .author("Mitchell Hynes <mitchell.hynes@ecumene.xyz>")
                    .arg(
                        Arg::with_name("frame count")
                            .short("j")
                            .long("webify")
                            .help("Generates a portable JS based render of your object for the web")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("width")
                            .short("w")
                            .help("Sets the width of the image to generate")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("height")
                            .short("h")
                            .help("Sets the height of the image to generate")
                            .takes_value(true),
                    ),
            ))
            .arg(
                Arg::with_name("input filename(s)")
                    .help("Sets the input file to render")
                    .required(true)
                    .multiple(true)
                    .index(1),
            ),
    )
    .get_matches()
}

fn commands_for_subcommands<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    command_flag_color(command_rotates(app))
}

fn command_flag_color<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("no color")
            .short("b")
            .help("Flags the rasterizer to render without color"),
    )
}

fn command_rotates<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("x")
            .short("x")
            .long("yaw")
            .help("Sets the object's static X rotation (in radians)")
            .takes_value(true),
    )
    .arg(
        Arg::with_name("y")
            .short("y")
            .long("pitch")
            .help("Sets the object's static Y rotation (in radians)")
            .takes_value(true),
    )
    .arg(
        Arg::with_name("z")
            .short("z")
            .long("roll")
            .help("Sets the object's static Z rotation (in radians)")
            .takes_value(true),
    )
}

pub fn to_meshes(models: Vec<tobj::Model>, materials: Vec<tobj::Material>) -> Vec<SimpleMesh> {
    let mut meshes: Vec<SimpleMesh> = vec![];
    for model in models {
        meshes.push(model.mesh.to_simple_mesh_with_materials(&materials));
    }
    meshes
}

pub fn match_meshes(matches: &ArgMatches) -> Result<Vec<SimpleMesh>, Box<dyn Error>> {
    let mut mesh_queue: Vec<SimpleMesh> = vec![];
    for slice in matches.value_of("input filename(s)").unwrap().split(' ') {
        let error = |s: &str, e: &str| -> Result<Vec<SimpleMesh>, Box<dyn Error>> {
            Err(format!("filename: [{}] couldn't load, {}. {}", slice, s, e).into())
        };
        // Fill list with file inputs (Splits for spaces -> multiple files)
        let path = Path::new(slice);
        let meshes = match path.extension() {
            None => error("couldn't determine filename extension", ""),
            Some(ext) => match ext.to_str() {
                None => error("couldn't parse filename extension", ""),
                Some(extstr) => match &*extstr.to_lowercase() {
                    "obj" => match tobj::load_obj(&path, true) {
                        Err(e) => error("tobj couldnt load/parse OBJ", &e.to_string()),
                        Ok(present) => Ok(to_meshes(present.0, present.1)),
                    },
                    "stl" => match OpenOptions::new().read(true).open(&path) {
                        Err(e) => error("STL load failed", &e.to_string()),
                        Ok(mut file) => match stl_io::read_stl(&mut file) {
                            Err(e) => error("stl_io couldnt parse STL", &e.to_string()),
                            Ok(stlio_mesh) => Ok(vec![stlio_mesh.to_simple_mesh()]),
                        },
                    },
                    _ => error("unknown filename extension", ""),
                },
            },
        };
        mesh_queue.append(&mut meshes.unwrap());
    }
    Ok(mesh_queue)
}

pub fn match_turntable(matches: &ArgMatches) -> Result<(f32, f32, f32, f32), Box<dyn Error>> {
    let mut turntable = (0.0, 0.0, 0.0, 0.0);
    if let Some(x) = matches.value_of("x") {
        turntable.0 = x.parse()?;
    }
    if let Some(y) = matches.value_of("y") {
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
    turntable.1 += std::f32::consts::PI; // All models for some reason are backwards, this fixes that
    Ok(turntable)
}

pub fn match_image_mode(matches: &ArgMatches) -> bool {
    matches.is_present("image")
}

pub fn match_no_color_mode(matches: &ArgMatches) -> bool {
    matches.is_present("no color")
}

pub fn match_dimensions(context: &mut Context, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
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
