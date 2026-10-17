//! Decode one VTF image (default: mip 0, frame 0, face 0) to an RGBA PNG.
//!
//! Usage: vtf-to-png <input.vtf> <output.png> [--mip N] [--frame N] [--face N]
//!
//! Prints a one-line JSON summary. Used by dribble.tf to convert BSP-embedded
//! (PAK) skybox faces without any external image tooling.

use bsp_to_glb::{LightmapImage, VtfImageSelection, decode_vtf, encode_lightmap_png};
use std::env;
use std::fs;
use std::process::ExitCode;

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        return Err(
            "Usage: vtf-to-png <input.vtf> <output.png> [--mip N] [--frame N] [--face N]"
                .to_owned(),
        );
    }
    let mut selection = VtfImageSelection::default();
    let mut index = 2;
    while index < args.len() {
        let value = args
            .get(index + 1)
            .ok_or_else(|| format!("missing value for {}", args[index]))?;
        match args[index].as_str() {
            "--mip" => selection.mip = value.parse().map_err(|_| "invalid --mip")?,
            "--frame" => selection.frame = value.parse().map_err(|_| "invalid --frame")?,
            "--face" => selection.face = value.parse().map_err(|_| "invalid --face")?,
            other => return Err(format!("unknown argument: {other}")),
        }
        index += 2;
    }
    let data =
        fs::read(&args[0]).map_err(|error| format!("failed to read {}: {error}", args[0]))?;
    let decoded =
        decode_vtf(&data, selection).map_err(|error| format!("{}: {error:?}", args[0]))?;
    let png = encode_lightmap_png(&LightmapImage {
        width: decoded.width,
        height: decoded.height,
        pixels: decoded.pixels,
    })?;
    fs::write(&args[1], &png).map_err(|error| format!("failed to write {}: {error}", args[1]))?;
    println!(
        "{{\"width\":{},\"height\":{},\"format\":\"{}\",\"bytes\":{}}}",
        decoded.width,
        decoded.height,
        decoded.metadata.format.name,
        png.len()
    );
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
