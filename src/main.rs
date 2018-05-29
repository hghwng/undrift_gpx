extern crate quick_xml;
extern crate undrift_gps;
#[macro_use]
extern crate failure;

use quick_xml::events::{BytesStart, Event, attributes::Attribute};
use quick_xml::{Reader, Writer};
use std::convert::From;
use std::f64::NAN;
use std::io::{BufRead, Write};
use std::str::FromStr;

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "XML error: {}", _0)]
    XMLError(quick_xml::Error),
    #[fail(display = "failed to parse latitude or longitude at {} ", _0)]
    ValueError(usize),
}

type Result<T> = std::result::Result<T, Error>;

impl From<quick_xml::Error> for Error {
    fn from(e: quick_xml::Error) -> Self {
        Error::XMLError(e)
    }
}

fn xml_transform<R: BufRead, W: Write, F>(
    mut reader: Reader<R>,
    mut writer: Writer<W>,
    levels: &[&[u8]],
    mut transformer: F,
) -> Result<()>
where
    F: for<'a> FnMut(BytesStart<'a>, &Reader<R>) -> Result<BytesStart<'a>>,
{
    let mut matched = 0;
    let mut current = 0;

    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Eof) => {
                break;
            }
            Ok(Event::Start(elem)) => {
                if matched == current && matched < levels.len() && levels[matched] == elem.name() {
                    matched += 1;
                }
                current += 1;

                let elem = if matched == current && matched == levels.len() {
                    transformer(elem, &reader)?
                } else {
                    elem
                };
                writer.write_event(Event::Start(elem))?;
            }
            Ok(Event::End(elem)) => {
                if matched == current {
                    matched -= 1;
                }
                current -= 1;
                writer.write_event(Event::End(elem))?;
            }
            Ok(event) => {
                writer.write_event(event)?;
            }
            Err(err) => {
                return Err(err.into());
            }
        }
        buf.clear();
    }

    Ok(())
}

fn gpx_transform<R: BufRead, W: Write>(
    reader: Reader<R>,
    writer: Writer<W>,
    convert_fn: fn(f64, f64) -> (f64, f64),
) -> Result<()> {
    const LAT_NAME: &[u8] = b"lat";
    const LON_NAME: &[u8] = b"lon";
    const LEVELS: [&[u8]; 4] = [b"gpx", b"trk", b"trkseg", b"trkpt"];

    xml_transform(reader, writer, &LEVELS, |elem, reader| {
        let mut new_elem = BytesStart::owned(elem.name().to_vec(), elem.name().len());
        let mut lat: f64 = NAN;
        let mut lon: f64 = NAN;

        for attr in elem.attributes() {
            fn parse<T: FromStr, R: BufRead>(attr: Attribute, reader: &Reader<R>) -> Result<T> {
                match attr.unescape_and_decode_value(reader)?.parse::<T>() {
                    Ok(v) => Ok(v),
                    Err(_) => Err(Error::ValueError(reader.buffer_position())),
                }
            }

            let attr = try!(attr);
            match attr.key {
                LAT_NAME => lat = parse(attr, reader)?,
                LON_NAME => lon = parse(attr, reader)?,
                _ => new_elem.push_attribute(attr),
            }
        }

        if !lat.is_nan() && !lon.is_nan() {
            let (lat, lon) = convert_fn(lat, lon);
            new_elem.push_attribute((LAT_NAME, lat.to_string().as_bytes()));
            new_elem.push_attribute((LON_NAME, lon.to_string().as_bytes()));
        }
        Ok(new_elem)
    })
}

fn main() -> Result<()> {
    let reader = quick_xml::Reader::from_file("demo/src.gpx").unwrap();
    let writer = quick_xml::Writer::new_with_indent(
        std::fs::File::create("demo/dst.gpx").unwrap(),
        b'\t',
        1,
    );

    gpx_transform(reader, writer, undrift_gps::gcj_to_wgs)
}
