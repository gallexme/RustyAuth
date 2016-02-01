extern crate capnpc;

fn main() {
    ::capnpc::compile(".", &["src/packets/TPacketGCPhase.capnp"]).unwrap();
}