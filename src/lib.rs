pub mod quote;
pub mod trade;
mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::*;
use libc::c_char;
use std::ffi::CStr;

#[doc = "配置信息"]
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Configuration {
    #[doc = "经纪公司代码"]
    pub broker_id: String,
    #[doc = "用户代码"]
    pub user_id: String,
    #[doc = "用户端系统内部信息"]
    pub system_info: String,
    #[doc = "登录成功时间"]
    pub login_time: String,
    #[doc = "认证码"]
    pub auth_code: String,
    #[doc = "App代码"]
    pub appid: String,
    #[doc = "用户公网IP"]
    pub public_ip: String,
    #[doc = "交易日"]
    pub trading_day: String,
    #[doc = "密码"]
    pub passwd: String,
    #[doc = "用户端产品信息"]
    pub user_product_info: String,
    #[doc = "接口端产品信息"]
    pub interface_product_info: String,
    #[doc = "协议信息"]
    pub protocol_info: String,
    #[doc = "Mac地址"]
    pub mac_addr: String,
    #[doc = "动态密码"]
    pub one_time_passwd: String,
    #[doc = "登录备注"]
    pub login_remark: String,
    #[doc = "终端IP端口"]
    pub port: i32,
    #[doc = "终端IP地址"]
    pub ip_addr: String,
    #[doc = "图形验证码的文字内容"]
    pub captcha: String,
    #[doc = "短信验证码文字内容"]
    pub text: String,
    #[doc = "OTP密码"]
    pub otp_passwd: String,
    pub front_addr: String,
    pub ns_addr: String,
    pub login_mode: i8,
}
pub trait FromRaw<T> {
    unsafe fn from_raw(raw: T) -> Self;
}

#[macro_export]
macro_rules! impl_ffi_convert {
    ($rtype:ty, $ctype: ty, $lb: expr, $ub: expr) => {
        // impl FromRaw<$ctype> for $rtype {
        //     unsafe fn from_raw(from: $ctype) -> Self {
        //         assert!($lb <= from as u32 && from as u32 <= $ub);
        //         std::mem::transmute::<_, $rtype>(from)
        //     }
        // }

        impl From<$rtype> for $ctype {
            fn from(r: $rtype) -> Self {
                unsafe { std::mem::transmute::<_, $ctype>(r) }
            }
        }
    };
    ($rtype:ty, $ctype: ty, $ub: expr) => {
        impl_ffi_convert!($rtype, $ctype, 0, $ub);
    };
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

#[doc = "接口应答"]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Response {
    pub code: i32,
    pub message: String,
    pub req_id: Option<i32>,
    pub is_last: Option<bool>,
}

impl Response {
    pub fn with_req_id(mut self, req_id: i32) -> Self {
        self.req_id = Some(req_id);
        self
    }
    pub fn with_is_last(mut self, is_last: bool) -> Self {
        self.is_last = Some(is_last);
        self
    }
}

impl<'a> From<&'a CThostFtdcRspInfoField> for Response {
    fn from(field: &'a CThostFtdcRspInfoField) -> Self {
        Response {
            code: field.ErrorID,
            message: String::from_c_buf(&field.ErrorMsg),
            req_id: None,
            is_last: None,
        }
    }
}
