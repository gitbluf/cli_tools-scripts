extern crate clap;
extern crate ssh2;

use clap::{App, Arg};
use ssh2::Session;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let _matches = App::new("deploy-agent")
        .version("v1.0.0")
        .author("Marko Petrovic")
        .about("About: deployer cli tool")
        .arg(
            Arg::with_name("HOST")
                .long("hostname")
                .short("r")
                .required(true)
                .multiple(true)
                .takes_value(true)
                .help("add hostname or ip adress and port"),
        )
        .arg(
            Arg::with_name("USERNAME")
                .long("username")
                .short("u")
                .required(true)
                .takes_value(true)
                .default_value("virtuser")
                .help("add a username/ default is virtuser"),
        )
        .arg(
            Arg::with_name("PASSWORD")
                .long("password")
                .short("p")
                .takes_value(true)
                .default_value("passw0rd")
                .help("add password to your user AD, default pass is of virtuser"),
        )
        .arg(
            Arg::with_name("FILE")
                .long("file")
                .short("f")
                .help("add file to push to host(s)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("GFILE")
                .long("gfile")
                .short("g")
                .help("enter a remote file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("COMMAND")
                .long("command")
                .short("c")
                .help("enter shell command remotly")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("REMOTE")
                .long("remote")
                .short("r")
                .help("where you want to copy the file on remote")
                .takes_value(true)
        )
        .get_matches();

    // type &str
    let _user = _matches.value_of("USERNAME").unwrap();
    let _password = _matches.value_of("PASSWORD").unwrap();
    if let Some(_hosts) = _matches.values_of("HOST") {
        for i in _hosts {
            if let Some(_file) = _matches.value_of("FILE") {
                let mut path = PathBuf::new(); //.file_name().unwrap();
                path.push(_file);
                let mut aps_path = fs::canonicalize(&path);
                println!("hello");
                println!("{:?}", aps_path);
                deploy_scripts(i, _user, _password, path);
            } else if let Some(_cmd) = _matches.value_of("COMMAND") {
                remote_cmd(i, _user, _password, _cmd);
            }
        }
    }
}

//this fn is called to deploy pushed files
fn deploy_scripts(_host: &str, _user: &str, _password: &str, _path: PathBuf, _rem_file: PathBuf) {
    let tcp = TcpStream::connect(_host).unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    //authenthification
    sess.userauth_password(_user, _password).unwrap();

    //TODO
    //let path = Path::new(&_file).file_name().unwrap();
    let mut data = File::open(&_path).expect("File not found");

    let mut contents = String::new();

    data.read_to_string(&mut contents)
        .expect("Could not read frow a file");

    let data_len = contents.len() as u64;

    let mut remote_file = sess.scp_send(&_rem_file, 0o644, data_len, None).unwrap();

    remote_file.write_fmt(format_args!("{}", contents)).unwrap();
}

fn remote_cmd(_host: &str, _user: &str, _password: &str, _cmd: &str) {
    let tcp = TcpStream::connect(_host).unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();

    //authenthification
    sess.userauth_password(_user, _password).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec(_cmd).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    //print exit status, expected 0
    println!("{}", channel.exit_status().unwrap());
    assert!(sess.authenticated());
}
