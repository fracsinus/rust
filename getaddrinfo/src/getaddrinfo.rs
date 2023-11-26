use core::ffi::{c_char, c_int};
use std::ptr;
use std::ffi::CString;
use std::io;
use std::net::{IpAddr, Ipv4Addr};
use crate::_types::*;

extern "C" {
    // <netdb.h>
    #[link_name="getaddrinfo"]
    fn _getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const AddrInfo,
        res: *mut *mut AddrInfo,
    ) -> c_int;

    // fn printf(format: *const c_char, ...) -> c_int;
}

// TODO: maybe io::Result is not appropriate
pub fn getaddrinfo(domain: &str) -> io::Result<IpAddr> {
    let node = CString::new(domain)?;
    let service = CString::new("80")?;
    let hints = AddrInfo {
        ai_flags: 0,
        ai_family: 0, // AF_UNSPEC
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: ptr::null_mut(),
        ai_canonname: ptr::null_mut(),
        ai_next: ptr::null_mut(),
    };
    let mut res: *mut AddrInfo = ptr::null_mut();
    unsafe {
        let code = _getaddrinfo(node.as_ptr(), service.as_ptr(), &hints, &mut res);
        if code != 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "")); // TODO: ???
        }
        // println!("status code: {:?}", code);
        // println!("res: {:?}", *res);

        let ai_addr = (*res).ai_addr;
        if ai_addr.is_null() {
            // println!("ai_addr is null: {:?}", ai_addr.is_null());
            return Err(io::Error::new(io::ErrorKind::NotFound, "")); // TODO: ???
        }
        // println!("ai_addr: {:?}", *ai_addr);

        let sa_family = (*ai_addr).sa_family;
        // println!("sa_family: {:?}", sa_family);

        if sa_family == AF_INET {
            let sockaddr_in = (ai_addr as *const SockAddrIn).read();
            let s_addr = Ipv4Addr::from(u32::from_be(sockaddr_in.sin_addr.s_addr));
            // println!("ip: {:?}", s_addr);
            // println!("port: {:?}", sockaddr_in.sin_port.to_be());

            return Ok(IpAddr::V4(s_addr));
        }
        else if sa_family == AF_INET6 {
            // println!("Not implemented");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "")); // TODO:
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "")); // TODO:
        }
    }
}