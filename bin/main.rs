extern crate quick_xml;
extern crate converter;
extern crate undrift_gps;

use converter::*;
use undrift_gps::*;
use quick_xml::{Reader, Writer};
use std::env::args;
use std::fs::File;
use std::process::exit;

fn convert_file(input: &str, output: &str, func: ConvertFn) -> Result<()> {
    const INDENT_CHAR: u8 = b'\t';
    const INDENT_LEVEL: usize = 1;
    let reader = Reader::from_file(input)?;
    let output_file =
        File::create(output).or_else(|e| Err(Error::XMLError(quick_xml::Error::Io(e))))?;
    let mut writer = Writer::new_with_indent(output_file, INDENT_CHAR, INDENT_LEVEL);
    gpx_transform(reader, &mut writer, func)
}

const FUNCS: &[(&str, ConvertFn)] = &[
    ("bd-gcj", bd_to_gcj),
    ("bd-wgs", bd_to_wgs),
    ("gcj-bd", gcj_to_bd),
    ("gcj-wgs", gcj_to_wgs),
    ("wgs-bd", wgs_to_bd),
    ("wgs-gcj", wgs_to_gcj),
];

fn usage(binary_name: &str) {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("Undrift GPX {}", VERSION);
    println!(
        "Usage: {name} CONVERSION GPX_FILE_IN GPX_FILE_OUT",
        name = binary_name
    );
    println!(
        "Conversions: {}",
        FUNCS
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<&str>>()
            .join(", ")
    );
    println!(
        "Example: fix drift on devices caused by Baidu-based route planner on Garmin Connect:\n\
         {name} gcj-wgs FILE_IN FILE_OUT",
        name = binary_name
    );
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 4 {
        usage(&args[0]);
        exit(1);
    }
    let (func_name, input, output) = (&args[1], &args[2], &args[3]);

    let func = match FUNCS.iter().find(|(name, _)| *name == func_name) {
        Some(func) => func.1,
        None => {
            eprintln!("Command line error: unknown conversion of {}", func_name);
            exit(2);
        }
    };

    if let Err(err) = convert_file(input.as_str(), output.as_str(), func) {
        eprintln!("{}", err);
        exit(3);
    }
}
