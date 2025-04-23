use nix::sys::socket::{ sockaddr_un, UnixAddr };

#[allow(unused)]
pub struct Socket {
    fd: i32,
    fd_lock: i32,
    addr: sockaddr_un,
    lock_addr: UnixAddr,
    // link: not implementing due to inherent unsafety and less potential for optimization and speculative execution
}