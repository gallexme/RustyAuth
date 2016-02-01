#[macro_use]
extern crate log;
use std::io::prelude::*;
use std::net::*;
use std::thread;
mod lib;


use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter, SetLoggerError};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(SimpleLogger)
    })
}
use lib::HasByteable;
fn handle_client(mut stream: TcpStream) {
    // info!("Phase {},Header{} test",packet.get_phase().unwrap() as u16,packet.get_header());
    let mut buf;
    let mut state;
    info!("New Client Connected {}", stream.peer_addr().unwrap());

    state = lib::State::StateHandshake;
    loop {
        // clear out the buffer so we don't send garbage
        buf = [0; 65536];
        info!("try to write");
        match state {
            lib::State::StateHandshake => {
                stream.write(&lib::get_packet(lib::TPacketGCPhase {
                    header: 253,
                    phase: lib::Phase::Auth,
                }));
                state = lib::State::StateAuth;


            }
            lib::State::StateAuth => {}
        };
        // serialize::write_message(&mut stream, &message);
        stream.flush();
        // stream.write(&mut buf);
        info!("try to read");
        let length = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    info!("EOF");
                    break;
                }
                m
            }
        };
        info!("Bytes Received:{}", length);
        if length > 0 {
            match buf[0] {
                44 => {
                    info!("Got Pinged");
                    stream.write(&[44]);
                }
                254 => {
                    info!("Got Pong");
                }
                103 => {
                    let mut vec = Vec::new();
                    for i in 0..length {
                        vec.push(buf[i]);
                    }
                    let packet: lib::TPacketCGLogin3 = lib::TPacketCGLogin3::read_packet(vec);
                    info!("{}", packet.name);
                }
                _ => {
                    info!("Unknown Packet Header {} ", buf[0]);

                }
            }
        }
    }
    info!("Client Disconnected");
}


use std::time::Duration;
fn main() {
    let _ = init();
    let listener_thread = thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        // connection succeeded
                        handle_client(stream)
                    });
                }
                Err(e) => {
                    info!("Connection Failed");
                    break; /* connection failed */
                }
            }
        }
        // close the socket server
        drop(listener);
    });
    thread::sleep(Duration::from_millis(4000));
    info!("Shutdown");
    listener_thread.join();
    std::process::exit(0);
    // panic!("Client Disconnected");
}
// accept connections and process them, spawning a new thread for each one
