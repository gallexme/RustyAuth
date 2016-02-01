
pub enum State {
    StateHandshake,
    StateAuth,
}
pub trait HasByteable {
    fn get_bytes(&self) -> Vec<u8>;
    fn read_packet(Vec<u8>) -> Self;
}
pub fn get_packet<T: HasByteable>(packet: T) -> Vec<u8> {
    packet.get_bytes()
}

pub struct TPacketGCPhase {
    pub header: u8,
    pub phase: Phase,
}
#[repr(u8)]
#[derive(Clone)]
pub enum Phase {
    Close = 0,
    HandShake = 1,
    Login = 2,
    Select = 3,
    Loading = 4,
    Game = 5,
    Dead = 6,
    DBClientConnecting = 7,
    DBClient = 8,
    P2P = 9,
    Auth = 10,
}
trait from_u8{
    fn from_u8(u8) -> Phase;
	}
impl from_u8 for Phase {
    fn from_u8(byte: u8) -> Phase {
        match byte {
            0 => Phase::Close,
            1 => Phase::HandShake,
            2 => Phase::Login,
            3 => Phase::Loading,
            4 => Phase::Select,
            5 => Phase::Game,
            6 => Phase::Dead,
            7 => Phase::DBClientConnecting,
            8 => Phase::DBClient,
            9 => Phase::P2P,
            10 => Phase::Auth,
            _ => Phase::Close,
        }
    }
}
impl HasByteable for TPacketGCPhase {
    fn get_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.push(self.header);
        v.push(self.phase.clone() as u8);
        v
    }
    fn read_packet(bytes: Vec<u8>) -> TPacketGCPhase {
        TPacketGCPhase {
            header: bytes[0],
            phase: Phase::from_u8(bytes[1]),
        }
    }
}
pub struct TPacketGCHandshake {
    pub header: u8,
    pub handshake: u32,
    pub time: u32,
    pub delta: i32,
}
static CGLOGIN3_HEADER: u8 = 111;
pub struct TPacketCGLogin3 {
    pub name: String,
    pub pwd: String,
    pub client_key: [u32; 4],
    pub key: [u8; 100],
}
use std::mem;
impl HasByteable for TPacketCGLogin3 {
    fn get_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.push(CGLOGIN3_HEADER as u8); //Header
        unsafe {
            let converted = mem::transmute::<[u32; 4], [u8; 16]>(self.client_key);
            for value in converted.iter() {
                v.push(*value);
            }
        }
        for keyelement in self.key.iter() {
            v.push(*keyelement);
        }
        v
    }
    fn read_packet(mut bytes: Vec<u8>) -> TPacketCGLogin3 {

        // let mut tmp_client_key;
        // unsafe {
        // tmp_client_key = mem::transmute::<[u8; 16], [u32; 4]>(bytes.drain(1..16));
        //
        // }
        // TODO: XTEA
        bytes.drain(1..1);
        let name: String = String::from_utf8(bytes.drain(1..32).collect()).unwrap();
        let pwd: String = String::from_utf8(bytes.drain(1..16).collect()).unwrap();
        let client_key: [u32; 4] = {
            unsafe {
                const size: usize = 4 * 4;
                let mut array = [0u8; size];
                for (x, p) in bytes.drain(1..(size)).zip(array.iter_mut()) {
                    *p = x;
                }
                mem::transmute::<[u8; size], [u32; 4]>(array)
            }
        };
        let key: [u8; 100] = {
            let mut array = [0u8; 100];
            for (x, p) in bytes.drain(1..100).zip(array.iter_mut()) {
                *p = x
            }
            array
        };
        TPacketCGLogin3 {
            // client_key: tmp_client_key,
            name: {
                name
            },
            pwd: {
                pwd
            },
            client_key: {
                client_key
            },
            key: {
                key
            },
        }
        // TPacketCGLogin3{
        // 	header:bytes[0],
        // 	phase:bytes[1] as Phase
        // }
    }
}