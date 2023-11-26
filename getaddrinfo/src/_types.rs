use core::ffi::{c_char, c_int, c_uint, c_ushort};

// <sys./socket.h> sa_family_t : uint by POSIX
#[cfg(target_os="macos")]
type SaFamilyT = u8; // but it's u8 in macos
#[cfg(not(target_os="macos"))]
type SaFamilyT = c_ushort; // and ushort in linux

pub const AF_INET: SaFamilyT = 2;
#[cfg(target_os="macos")]
pub const AF_INET6: SaFamilyT = 30;
#[cfg(not(target_os="macos"))]
pub const AF_INET6: SaFamilyT = 10;


#[derive(Debug)]
#[repr(C)] // <netdb.h> addr_info
pub struct AddrInfo {
    pub ai_flags: c_int, // input flags
    pub ai_family: c_int, // protocol family for socket */
    pub ai_socktype: c_int, // socket type */
    pub ai_protocol: c_int, // protocol for socket */
    pub ai_addrlen: c_uint, // length of socket-address */

    #[cfg(target_os="macos")] // on FreeBSD, ai_canonname comes before ai_addr
    pub ai_canonname: *mut c_char, // canonical name for service location */
    pub ai_addr: *mut SockAddr, // socket-address for socket */
    #[cfg(not(target_os="macos"))]
    pub ai_canonname: *mut c_char,

    pub ai_next: *mut AddrInfo, // pointer to next in list */
}

#[derive(Debug)]
#[repr(C)] // <sys/socket.h> Structure describing a generic socket address.
pub struct SockAddr {
    #[cfg(target_os = "macos")]
    pub sa_len: u8, // (macos) total length

	pub sa_family: SaFamilyT,
    pub sa_data: *mut c_char, // usually defined as 14-bytes array, but it isn't used anyway. this data is accessed through the attributes of more specific structs such as sockaddr_in
}

#[derive(Debug)]
#[repr(C)] // <netinet/in.h> sockaddr_in: Structure describing an Internet (IP) socket address
pub struct SockAddrIn {
    #[cfg(target_os = "macos")]
    sa_len: u8, // (macos) total length

    pub sin_family: SaFamilyT,
    pub sin_port: c_ushort, // <netinet/in.h> in_port_t : uint16
    pub sin_addr: InAddr,
    pub sin_zero: *mut c_char, // padding
}

#[derive(Debug)]
#[repr(C)] // <netinet/in.h>
pub struct SockAddrIn6 {
    #[cfg(target_os = "macos")]
    sa_len: u8, // (macos) total length

    pub sin6_family: c_ushort, // <sys/socket.h> sa_family_t: unsigned int (POSIX)
    pub sin6_port: c_ushort, // <netinet/in.h> in_port_t: uint16_t (POSIX)
    pub sin6_flowinfo: c_uint, // <netinet/in.h> uint32_t
    pub sin6_addr: In6Addr, // <netinet/in.h> in_addr_t: uint32_t (POSIX)
    pub sin6_zero: *mut c_char, // padding
}

#[derive(Debug)]
#[repr(C)] // <netinet/in.h>
pub struct InAddr {
    pub s_addr: c_uint, // <netinet/in.h> in_addr_t: uint32_t (POSIX)
}

#[derive(Debug)]
#[repr(C)] // <netinet/in.h>, or <netinet/in6.h>
pub struct In6Addr {
    pub s6_addr: [u8; 16], // TODO: write union
}