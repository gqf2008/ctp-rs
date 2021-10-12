mod quote;
mod trade;
mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use ffi::{QuoteSpiStub, TradeSpiStub};
use libc::c_char;
use std::ffi::CStr;


impl Drop for QuoteSpiStub {
    fn drop(&mut self) {
        // unsafe { self.destruct() }
        unsafe { ffi::QuoteSpiStub_Destructor(self) }
        //unreachable!("QuoteSpiStub should be manually dropped!")
    }
}

impl Drop for TradeSpiStub {
    fn drop(&mut self) {
        unsafe { ffi::TradeSpiStub_Destructor(self) }
        // unreachable!("TraderSpiStub should be manually dropped!")
    }
}

trait FromCBuf<'a> {
    fn from_c_buf(b: &'a [c_char]) -> Self;
}

impl<'a> FromCBuf<'a> for &'a CStr {
    fn from_c_buf(b: &'a [c_char]) -> Self {
        // convert from &[i8] to &[u8]
        let b = unsafe { &*(b as *const _ as *const [u8]) };
        match b.iter().position(|&c| c == 0u8) {
            Some(pos) => unsafe { CStr::from_bytes_with_nul_unchecked(&b[..=pos]) },
            None => {
                let s = String::from_utf8(b.to_vec());
                println!("{:?}", s);
                unreachable!("String without null end"); // TODO: not sure if XTP follows this
            }
        }
    }
}

impl<'a> FromCBuf<'a> for String {
    fn from_c_buf(b: &'a [c_char]) -> Self {
        // convert from &[i8] to &[u8]
        let b = unsafe { &*(b as *const _ as *const [u8]) };
        let slice = match b.iter().position(|&c| c == 0u8) {
            Some(pos) => &b[..pos],
            None => b,
        };
        unsafe { String::from_utf8_unchecked(slice.to_vec()) }
    }
}

trait ToCBuf {
    fn to_c_buf16(&self) -> [c_char; 16usize];
    fn to_c_buf64(&self) -> [c_char; 64usize];
}

impl ToCBuf for &CStr {
    fn to_c_buf16(&self) -> [c_char; 16usize] {
        let mut sarr = [0i8; 16];
        let len = self.to_bytes().len().min(16);

        for (i, &byte) in self.to_bytes()[..len].iter().enumerate() {
            sarr[i] = byte as i8;
        }

        sarr
    }
    fn to_c_buf64(&self) -> [c_char; 64usize] {
        let mut sarr = [0i8; 64];
        let len = self.to_bytes().len().min(64);

        for (i, &byte) in self.to_bytes()[..len].iter().enumerate() {
            sarr[i] = byte as i8;
        }

        sarr
    }
}

impl ToCBuf for String {
    fn to_c_buf16(&self) -> [c_char; 16usize] {
        let mut sarr = [0i8; 16];
        let len = self.as_bytes().len().min(16);

        for (i, &byte) in self.as_bytes()[..len].iter().enumerate() {
            sarr[i] = byte as i8;
        }

        sarr
    }
    fn to_c_buf64(&self) -> [c_char; 64usize] {
        let mut sarr = [0i8; 64];
        let len = self.as_bytes().len().min(64);

        for (i, &byte) in self.as_bytes()[..len].iter().enumerate() {
            sarr[i] = byte as i8;
        }

        sarr
    }
}
