extern crate clap;
extern crate lettre;
extern crate lettre_email;
extern crate mime;
extern crate time;
extern crate walkdir;
extern crate zip;
extern crate chrono;

use clap::{App, Arg};
use lettre::sendmail::SendmailTransport;
use lettre::{SimpleSendableEmail, EmailTransport};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;
use chrono::prelude::*;

fn main() {

    std::process::exit(real_main());
    
}

const METHOD_STORED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Stored);

#[cfg(feature = "flate2")]
const METHOD_DEFLATED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Deflated);
#[cfg(not(feature = "flate2"))]
const METHOD_DEFLATED: Option<zip::CompressionMethod> = None;

#[cfg(feature = "bzip2")]
const METHOD_BZIP2: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Bzip2);
#[cfg(not(feature = "bzip2"))]
const METHOD_BZIP2: Option<zip::CompressionMethod> = None;

fn real_main() -> i32 {
     //let ts: String = timestamp();
       let ts: String = date_time();
       let _def = ts+".zip";
    //creating arguments
    let matches = App::new("ziperaja")
        .version("v1.01")
        .author("Marko Petrovic")
        .about("About: custom zip cli program")
        .arg(
            Arg::with_name("LOCATION")
                .long("location")
                .short("l")
                .required(true)
                .takes_value(true)
                .default_value(".")
                .help("add path to folder you want to zip(default is .) with -; or --location"),
        )
        .arg(
            Arg::with_name("ZNAME")
                .long("zname")
                .short("z")
                .required(true)
                .takes_value(true)
                .default_value(&_def)
                .help("add name of the zip file(default is ziperaja.zip) with flags -z or --zname"),
        )
        .arg(
            Arg::with_name("MAILER")
                .long("email")
                .short("e")
                .takes_value(true)
                .help("add email to be sent to")
        )
        .get_matches();

    //get argument variables from matches - arguments retunr &str
    let src_dir = matches.value_of("LOCATION").unwrap();
    let dst_file = matches.value_of("ZNAME").unwrap();
    let _emailer = matches.value_of("MAILER").unwrap_or("");
    println!("{:?}", _emailer);
    if let Some(matches) = matches.value_of("MAILER") {
        send_email(&_emailer.to_string());
    }


    for &method in [METHOD_STORED, METHOD_DEFLATED, METHOD_BZIP2].iter() {
        if method.is_none() {
            continue;
        }
        match doit(src_dir, dst_file, method.unwrap()) {
            Ok(_) => println!("done: {} written to {}", src_dir, dst_file),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    return 0;
}

fn zip_dir<T>(
    it: &mut Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix))
            .unwrap()
            .to_str()
            .unwrap();

        if path.is_file() {
            println!("adding {:?} as {:?} ...", path, name);
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn doit(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}

//TODO add subbcommand to chose between timestamp and date_time
//to get time in seconds.miliseconds use this fn istead of date_time
/*
fn timestamp() -> String {
    
    let timespec = time::get_time();
    //returns time in seconds
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills.to_string()  
}
*/

//get local time E.G 2018-07-11 11:04:12.874676 +02:00
fn date_time() -> String {
    
    let local: DateTime<Local> = Local::now();
    
    local.to_string()
}


fn send_email(x: &str) {
    let email = SimpleSendableEmail::new(
                    "root@localhost".to_string(),
                    &[x.to_string()],
                    "message_id".to_string(),
                    "Zip completed".to_string(),
                ).unwrap();

    let mut sender = SendmailTransport::new();
    let result = sender.send(&email);
    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Sending failed to {}", x);
    }
}

