extern crate quick_xml;
extern crate undrift_gps;
#[macro_use]
extern crate failure;

use std::convert::From;
use std::f64::NAN;

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "XML error: {}", _0)]
    XMLError(quick_xml::Error),
    #[fail(display = "failed to parse latitude or longitude at {} ", _0)]
    ValueError(usize),
}

impl From<quick_xml::Error> for Error {
    fn from(e: quick_xml::Error) -> Self {
        Error::XMLError(e)
    }
}

fn convert_gpx<R, W>(
    ref mut reader: quick_xml::Reader<R>,
    ref mut writer: quick_xml::Writer<W>,
    convert_fn: fn(f64, f64) -> (f64, f64),
) -> std::result::Result<(), Error>
where
    R: std::io::BufRead,
    W: std::io::Write,
{
    use quick_xml::events::BytesStart;
    use quick_xml::events::Event;
    const TRKPT_NAME: &[u8] = b"trkpt";
    const LAT_NAME: &[u8] = b"lat";
    const LON_NAME: &[u8] = b"lon";
    const LEVELS: [&[u8]; 4] = [b"gpx", b"trk", b"trkseg", b"trkpt"];

    let mut level_matched = 0;
    let mut level_current = 0;
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(elem)) => {
                // Maintain current level
                {
                    if level_matched == level_current && level_matched < LEVELS.len()
                        && LEVELS[level_matched] == elem.name()
                    {
                        level_matched += 1;
                    }
                    level_current += 1;
                    if level_current != level_matched || level_matched != LEVELS.len() {
                        writer.write_event(Event::Start(elem))?;
                        continue;
                    }
                }

                // Convert Element
                {
                    let mut new_elem = BytesStart::owned(TRKPT_NAME.to_owned(), TRKPT_NAME.len());
                    let mut lat: f64 = NAN;
                    let mut lon: f64 = NAN;

                    for attr in elem.attributes() {
                        let attr = try!(attr);
                        if attr.key != LAT_NAME && attr.key != LON_NAME {
                            new_elem.push_attribute(attr);
                            continue;
                        }

                        let value = attr.unescape_and_decode_value(reader)?;
                        if let Ok(v) = value.parse::<f64>() {
                            match attr.key {
                                b"lat" => {
                                    lat = v;
                                }
                                b"lon" => {
                                    lon = v;
                                }
                                _ => {
                                    panic!("shouldn't reach here");
                                }
                            }
                        } else {
                            return Err(Error::ValueError(reader.buffer_position()));
                        }
                    }

                    if lat.is_nan() || lon.is_nan() {
                        return Err(Error::ValueError(reader.buffer_position()));
                    }

                    let (lat, lon) = convert_fn(lat, lon);
                    new_elem.push_attribute((LAT_NAME, lat.to_string().as_bytes()));
                    new_elem.push_attribute((LON_NAME, lon.to_string().as_bytes()));
                    writer.write_event(Event::Start(new_elem))?;
                }
            }
            Ok(Event::End(elem)) => {
                if level_matched == level_current {
                    level_matched -= 1;
                }
                level_current -= 1;
                writer.write_event(Event::End(elem))?;
            }
            Ok(Event::Eof) => {
                break;
            }
            Ok(ref evt) => {
                writer.write_event(evt)?;
            }
            Err(evt) => {
                return Err(evt.into());
            }
        }
        buf.clear();
    }

    Ok(())
}

fn main() {
    let reader = quick_xml::Reader::from_file("demo/src.gpx").unwrap();
    let writer =
        quick_xml::Writer::new_with_indent(std::fs::File::create("demo/dst.gpx").unwrap(), b'\t', 1);
    convert_gpx(reader, writer, undrift_gps::gcj_to_wgs).unwrap();
}
