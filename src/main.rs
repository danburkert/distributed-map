#![allow(unstable)]

extern crate capnp;

use std::collections::HashMap;
use std::io;
use std::io::{Acceptor, Listener};
use std::io::{TcpListener, TcpStream};
use std::io::IoResult;
use std::os;

use capnp::serialize_packed;
use capnp::{MessageBuilder, MessageReader, ReaderOptions, MallocMessageBuilder};

use messages_capnp::request;
use messages_capnp::response;

pub mod messages_capnp {
  include!(concat!(env!("OUT_DIR"), "/messages_capnp.rs"));
}

const ADDRESS: &'static str = "127.0.0.1";
const PORT: u16 = 4242;

/// Main method.
fn main() {
    let args = std::os::args();
    let ref bin = args[0];

    let usage = format!("Usage:\n\
                         \t{} client\n\
                         \t{} server",
                         bin, bin);

    if args.len() < 2 {
        println!("Not enough arguments provided.");
        println!("{}", usage);
        os::set_exit_status(1);
        return;
    }

    match std::os::args()[1].as_slice() {
        "client" => client().unwrap(),
        "server" => server().unwrap(),
        e => panic!("Unknown argument '{}'.", e)
    }
}

fn client() -> io::IoResult<()> {
    let args = std::os::args();
    let ref bin = args[0];

    let usage = format!("Usage:\n\
                         \t{} client contains <key>\n\
                         \t{} client get <key>\n\
                         \t{} client put <key> <value>",
                         bin, bin, bin);

    if args.len() < 4 {
        println!("Not enough arguments provided.");
        println!("{}", usage);
        os::set_exit_status(1);
        return Ok(());
    }

    let key = args[3].as_slice();
    let mut message = MallocMessageBuilder::new_default();
    {
        let mut request = message.init_root::<request::Builder>();
        request.set_key(key);
        let op = args[2].as_slice();
        match op {
            "contains" => {
                request.get_operation().set_contains(());
            },
            "get" => {
                request.get_operation().set_get(());
            },
            "put" => {
                if args.len() < 5 {
                    println!("No value provided for put.");
                    println!("{}", usage);
                    os::set_exit_status(1);
                    return Ok(());
                }
                request.get_operation().set_put(args[4].as_slice());
            }
            _ => {
                println!("Unkown command '{}'.", op);
                println!("{}", usage);
                os::set_exit_status(1);
                return Ok(());
            }
        }
    }

    let mut stream = TcpStream::connect((ADDRESS, PORT));

    try!(serialize_packed::write_packed_message_unbuffered(&mut stream, &mut message));

    let message_reader = try!(serialize_packed::new_reader_unbuffered(&mut stream, ReaderOptions::new()));
    let response = message_reader.get_root::<response::Reader>();
    let response_key = response.get_key();
    match response.which() {
        Some(response::Contains(contains)) =>
            println!("contains {}: {}", response_key, contains),
        Some(response::Get(get)) => {
            let value = match get.which() {
                Some(response::get::Value(value)) => Some(value),
                Some(response::get::Empty(())) => None,
                None => panic!("No value!")
            };
            println!("get {}: {:?}", response_key, value);
        }
        Some(response::Put(new_key)) => {
            println!("new key {}: {}", response_key, new_key);
        }
        None => panic!("No response!")
    }
    Ok(())
}

fn server() -> io::IoResult<()> {
    let mut map = HashMap::<String, String>::new();

    let listener = TcpListener::bind((ADDRESS, PORT)).unwrap();

    // bind the listener to the specified address
    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
        let mut s = stream.unwrap();
        let message_reader = try!(serialize_packed::new_reader_unbuffered(&mut s, ReaderOptions::new()));
        let request = message_reader.get_root::<request::Reader>();
        let key = request.get_key();

        let mut message = MallocMessageBuilder::new_default();
        {
            let mut response = message.init_root::<response::Builder>();
            response.set_key(key);

            match request.get_operation().which() {
                Some(request::operation::Contains(())) => {
                    response.set_contains(map.contains_key(&key.to_string()));
                }
                Some(request::operation::Get(())) => {
                    match map.get(key) {
                        Some(value) => {
                            response.init_get().set_value(value.as_slice());
                        }
                        None => {
                            response.init_get().set_empty(());
                        }
                    }
                }
                Some(request::operation::Put(value)) => {
                    response.set_put(map.insert(key.to_string(), value.to_string()).is_none());
                }
                None => panic!("No request!")
            }
        }

        try!(serialize_packed::write_packed_message_unbuffered(&mut s, &mut message));
    }

    Ok(())
}
