/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/

use std::libc::c_char;
use std::ptr;
use std::str;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_void, c_char, size_t, c_float, c_double};
    use rsfml::sfTypes::sfBool;
    
    pub struct sfPacket {
        This : *c_void
    }

    pub extern "C" {
        fn sfPacket_create() -> *sfPacket;
        fn sfPacket_copy(pack : *sfPacket) -> *sfPacket;
        fn sfPacket_destroy(pack : *sfPacket) -> ();
        //fn sfPacket_append(pack : *sfPacket, data : *c_void, sizeInBytes : size_t) -> ();
        fn sfPacket_clear(pack : *sfPacket) -> ();
        //fn sfPacket_getData(pack : *sfPacket) -> *c_void;
        fn sfPacket_getDataSize(pack : *sfPacket) -> size_t;
        fn sfPacket_endOfPacket(pack : *sfPacket) -> sfBool;
        fn sfPacket_canRead(pack : *sfPacket) -> sfBool;
        fn sfPacket_readBool(pack : *sfPacket) -> sfBool;
        fn sfPacket_readInt8(pack : *sfPacket) -> i8;
        fn sfPacket_readUint8(pack : *sfPacket) -> u8;
        fn sfPacket_readInt16(pack : *sfPacket) -> i16;
        fn sfPacket_readUint16(pack : *sfPacket) -> u16;
        fn sfPacket_readInt32(pack : *sfPacket) -> i32;
        fn sfPacket_readUint32(pack : *sfPacket) -> u32;
        fn sfPacket_readFloat(pack : *sfPacket) -> c_float;
        fn sfPacket_readDouble(pack : *sfPacket) -> c_double;
        fn sfPacket_readString(pack : *sfPacket, string : *c_char) -> ();
        //fn sfPacket_readWideString(pack : *sfPacket, string : *wchar_t) -> ();
        fn sfPacket_writeBool(pack : *sfPacket, data : sfBool) -> ();
        fn sfPacket_writeInt8(pack : *sfPacket, data : i8) -> ();
        fn sfPacket_writeUint8(pack : *sfPacket, data : u8) -> ();
        fn sfPacket_writeInt16(pack : *sfPacket, data : i16) -> ();
        fn sfPacket_writeUint16(pack : *sfPacket, data : u16) -> ();
        fn sfPacket_writeInt32(pack : *sfPacket, data : i32) -> ();
        fn sfPacket_writeUint32(pack : *sfPacket, data : u32) -> ();
        fn sfPacket_writeFloat(pack : *sfPacket, data : c_float) -> ();
        fn sfPacket_writeDouble(pack : *sfPacket, data : c_double) -> ();
        fn sfPacket_writeString(pack : *sfPacket, string : *c_char) -> ();
        //fn sfPacket_writeWideString(pack : *sfPacket, string : *wchar_t) -> ();
    }
}

pub struct Packet {
    packet : *csfml::sfPacket
}

impl Packet {
    pub fn new() -> Packet {
        unsafe {
            Packet { packet : csfml::sfPacket_create()}
        }
    }

    pub fn new_copy(packet : &Packet) -> Packet {
        unsafe {
            Packet { packet : csfml::sfPacket_copy(packet.unwrap())}
        }
    }


    pub fn clear(&self) -> () {
        unsafe {
            csfml::sfPacket_clear(self.packet)
        }
    }

    pub fn get_data_size(&self) -> u32 {
        unsafe {
            csfml::sfPacket_getDataSize(self.packet) as u32 
        }
    }

    pub fn end_of_packet(&self) -> bool {
        unsafe {
            match csfml::sfPacket_endOfPacket(self.packet) {
                0 => false,
                _ => true
            }
        }
    }

    pub fn can_read(&self) -> bool {
        unsafe {
            match csfml::sfPacket_canRead(self.packet) {
                0 => false,
                _ => true
            }
        }
    }

    pub fn read_bool(&self) -> bool {
        unsafe {
            match csfml::sfPacket_readBool(self.packet) {
                0 => false,
                _ => true
            }
        }
    }

    pub fn read_i8(&self) -> i8 {
        unsafe {
            csfml::sfPacket_readInt8(self.packet)
        }
    }

    pub fn read_u8(&self) -> u8 {
        unsafe {
            csfml::sfPacket_readUint8(self.packet)
        }
    }

    pub fn read_i16(&self) -> i16 {
        unsafe {
            csfml::sfPacket_readInt16(self.packet)
        }
    }

    pub fn read_u16(&self) -> u16 {
        unsafe {
            csfml::sfPacket_readUint16(self.packet)
        }
    }

    pub fn read_i32(&self) -> i32 {
        unsafe {
            csfml::sfPacket_readInt32(self.packet)
        }
    }

    pub fn read_u32(&self) -> u32 {
        unsafe {
            csfml::sfPacket_readUint32(self.packet)
        }
    }

    pub fn read_f32(&self) -> f32 {
        unsafe {
            csfml::sfPacket_readFloat(self.packet) as f32
        }
    }

    pub fn read_f64(&self) -> f64 {
        unsafe {
            csfml::sfPacket_readDouble(self.packet) as f64
        }
    }

    pub fn read_string(&self) -> ~str {
        unsafe {
            let string : *c_char = ptr::null();
            csfml::sfPacket_readString(self.packet, string);
            str::raw::from_c_str(string)
        }
    }

    pub fn write_bool(&self, data : bool) -> () {
        unsafe {
            match data {
                true    => csfml::sfPacket_writeBool(self.packet, 1),
                false    => csfml::sfPacket_writeBool(self.packet, 0)
            }
        }
    }

    pub fn write_i8(&self, data : i8) -> () {
        unsafe {
            csfml::sfPacket_writeInt8(self.packet, data)
        }
    }
    
    pub fn write_u8(&self, data : u8) -> () {
        unsafe {
            csfml::sfPacket_writeUint8(self.packet, data)
        }
    }

    pub fn write_i16(&self, data : i16) -> () {
        unsafe {
            csfml::sfPacket_writeInt16(self.packet, data)
        }
    }

    pub fn write_u16(&self, data : u16) -> () {
        unsafe {
            csfml::sfPacket_writeUint16(self.packet, data)
        }
    }

    pub fn write_i32(&self, data : i32) -> () {
        unsafe {
            csfml::sfPacket_writeInt32(self.packet, data)
        }
    }

    pub fn write_u32(&self, data : u32) -> () {
        unsafe {
            csfml::sfPacket_writeUint32(self.packet, data)
        }
    }

    pub fn write_f32(&self, data : f32) -> () {
        unsafe {
            csfml::sfPacket_writeFloat(self.packet, data)
        }
    }

    pub fn write_f64(&self, data : f64) -> () {
        unsafe {
            csfml::sfPacket_writeDouble(self.packet, data)
        }
    }

    pub fn write_string(&self, string : ~str) -> () {
        unsafe {
            do str::as_c_str(string) |string_buf| {
                csfml::sfPacket_writeString(self.packet, string_buf)
            }
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfPacket {
        self.packet
    }

    #[doc(hidden)]
    pub fn wrap(packet : *csfml::sfPacket) -> Packet {
        Packet { packet : packet}
    }
}

impl Drop for Packet {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfPacket_destroy(self.packet)
        }
    }
}