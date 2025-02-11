//! `getsockopt` and `setsockopt` functions.

#![doc(alias = "getsockopt")]
#![doc(alias = "setsockopt")]

use crate::net::{Ipv4Addr, Ipv6Addr, SocketType};
use crate::{imp, io};
use core::time::Duration;
use imp::fd::AsFd;
#[cfg(windows)]
use imp::fd::AsSocketAsFd;

pub use imp::net::Timeout;

/// `getsockopt(fd, SOL_SOCKET, SO_TYPE)`—Returns the type of a socket.
///
/// # References
///  - [POSIX `getsockopt`]
///  - [POSIX `sys/socket.h`]
///  - [Linux `getsockopt`]
///  - [Linux `socket`]
///  - [Winsock2 `getsockopt`]
///  - [Winsock2 `SOL_SOCKET` options]
///
/// [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [POSIX `sys/socket.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_socket.h.html
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
/// [Winsock2 `getsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockopt
/// [Winsock2 `SOL_SOCKET` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/sol-socket-socket-options
#[inline]
#[doc(alias = "SO_TYPE")]
pub fn get_socket_type<Fd: AsFd>(fd: &Fd) -> io::Result<SocketType> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_socket_type(fd)
}

/// `setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, value)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `sys/socket.h`]
///  - [Linux `setsockopt`]
///  - [Linux `socket`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `SOL_SOCKET` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `sys/socket.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_socket.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `SOL_SOCKET` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/sol-socket-socket-options
#[inline]
#[doc(alias = "SO_REUSEADDR")]
pub fn set_socket_reuseaddr<Fd: AsFd>(fd: &Fd, value: bool) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_socket_reuseaddr(fd, value)
}

/// `setsockopt(fd, SOL_SOCKET, SO_BROADCAST, broadcast)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `sys/socket.h`]
///  - [Linux `setsockopt`]
///  - [Linux `socket`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `SOL_SOCKET` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `sys/socket.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_socket.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `SOL_SOCKET` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/sol-socket-socket-options
#[inline]
#[doc(alias = "SO_BROADCAST")]
pub fn set_socket_broadcast<Fd: AsFd>(fd: &Fd, broadcast: bool) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_socket_broadcast(fd, broadcast)
}

/// `getsockopt(fd, SOL_SOCKET, SO_BROADCAST)`
///
/// # References
///  - [POSIX `getsockopt`]
///  - [POSIX `sys/socket.h`]
///  - [Linux `getsockopt`]
///  - [Linux `socket`]
///  - [Winsock2 `getsockopt`]
///  - [Winsock2 `SOL_SOCKET` options]
///
/// [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [POSIX `sys/socket.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_socket.h.html
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
/// [Winsock2 `getsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockopt
/// [Winsock2 `SOL_SOCKET` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/sol-socket-socket-options
#[inline]
#[doc(alias = "SO_BROADCAST")]
pub fn get_socket_broadcast<Fd: AsFd>(fd: &Fd) -> io::Result<bool> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_socket_broadcast(fd)
}

/// `setsockopt(fd, SOL_SOCKET, SO_LINGER, linger)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `sys/socket.h`]
///  - [Linux `setsockopt`]
///  - [Linux `socket`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `SOL_SOCKET` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `sys/socket.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_socket.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `SOL_SOCKET` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/sol-socket-socket-options
#[inline]
#[doc(alias = "SO_LINGER")]
pub fn set_socket_linger<Fd: AsFd>(fd: &Fd, linger: Option<Duration>) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_socket_linger(fd, linger)
}

/// `getsockopt(fd, SOL_SOCKET, SO_LINGER)`
///
/// # References
///  - [POSIX `getsockopt`]
///  - [POSIX `sys/socket.h`]
///  - [Linux `getsockopt`]
///  - [Linux `socket`]
///  - [Winsock2 `getsockopt`]
///  - [Winsock2 `SOL_SOCKET` options]
///
/// [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [POSIX `sys/socket.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_socket.h.html
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
/// [Winsock2 `getsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockopt
/// [Winsock2 `SOL_SOCKET` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/sol-socket-socket-options
#[inline]
#[doc(alias = "SO_LINGER")]
pub fn get_socket_linger<Fd: AsFd>(fd: &Fd) -> io::Result<Option<Duration>> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_socket_linger(fd)
}

/// `setsockopt(fd, SOL_SOCKET, SO_PASSCRED, passcred)`
///
/// # References
///  - [Linux `setsockopt`]
///  - [Linux `socket`]
///
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
#[cfg(any(target_os = "android", target_os = "linux",))]
#[inline]
#[doc(alias = "SO_PASSCRED")]
pub fn set_socket_passcred<Fd: AsFd>(fd: &Fd, passcred: bool) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_socket_passcred(fd, passcred)
}

/// `getsockopt(fd, SOL_SOCKET, SO_PASSCRED)`
///
/// # References
///  - [Linux `getsockopt`]
///  - [Linux `socket`]
///
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
#[cfg(any(target_os = "android", target_os = "linux",))]
#[inline]
#[doc(alias = "SO_PASSCRED")]
pub fn get_socket_passcred<Fd: AsFd>(fd: &Fd) -> io::Result<bool> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_socket_passcred(fd)
}

/// `setsockopt(fd, SOL_SOCKET, id, timeout)`—Set the sending
/// or receiving timeout.
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `sys/socket.h`]
///  - [Linux `setsockopt`]
///  - [Linux `socket`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `SOL_SOCKET` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `sys/socket.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_socket.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `SOL_SOCKET` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/sol-socket-socket-options
#[inline]
#[doc(alias = "SO_RCVTIMEO")]
#[doc(alias = "SO_SNDTIMEO")]
pub fn set_socket_timeout<Fd: AsFd>(
    fd: &Fd,
    id: Timeout,
    timeout: Option<Duration>,
) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_socket_timeout(fd, id, timeout)
}

/// `getsockopt(fd, SOL_SOCKET, id)`—Get the sending or receiving timeout.
///
/// # References
///  - [POSIX `getsockopt`]
///  - [POSIX `sys/socket.h`]
///  - [Linux `getsockopt`]
///  - [Linux `socket`]
///  - [Winsock2 `getsockopt`]
///  - [Winsock2 `SOL_SOCKET` options]
///
/// [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [POSIX `sys/socket.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_socket.h.html
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `socket`]: https://man7.org/linux/man-pages/man7/socket.7.html
/// [Winsock2 `getsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockopt
/// [Winsock2 `SOL_SOCKET` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/sol-socket-socket-options
#[inline]
#[doc(alias = "SO_RCVTIMEO")]
#[doc(alias = "SO_SNDTIMEO")]
pub fn get_socket_timeout<Fd: AsFd>(fd: &Fd, id: Timeout) -> io::Result<Option<Duration>> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_socket_timeout(fd, id)
}

/// `setsockopt(fd, IPPROTO_IP, IP_TTL, ttl)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `setsockopt`]
///  - [Linux `ip`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `IPPROTO_IP` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `ip`]: https://man7.org/linux/man-pages/man7/ip.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_IP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options
#[inline]
#[doc(alias = "IP_TTL")]
pub fn set_ip_ttl<Fd: AsFd>(fd: &Fd, ttl: u32) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_ip_ttl(fd, ttl)
}

/// `getsockopt(fd, IPPROTO_IP, IP_TTL)`
///
/// # References
///  - [POSIX `getsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `getsockopt`]
///  - [Linux `ip`]
///  - [Winsock2 `getsockopt`]
///  - [Winsock2 `IPPROTO_IPV6` options]
///
/// [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `ip`]: https://man7.org/linux/man-pages/man7/ip.7.html
/// [Winsock2 `getsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockopt
/// [Winsock2 `IPPROTO_IP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options
#[inline]
#[doc(alias = "IP_TTL")]
pub fn get_ip_ttl<Fd: AsFd>(fd: &Fd) -> io::Result<u32> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_ip_ttl(fd)
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY, only_v6)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `setsockopt`]
///  - [Linux `ipv6`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `IPPROTO_IPV6` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `ipv6`]: https://man7.org/linux/man-pages/man7/ipv6.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_IPV6` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ipv6-socket-options
#[inline]
#[doc(alias = "IPV6_V6ONLY")]
pub fn set_ipv6_v6only<Fd: AsFd>(fd: &Fd, only_v6: bool) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_ipv6_v6only(fd, only_v6)
}

/// `getsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY)`
///
/// # References
///  - [POSIX `getsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `getsockopt`]
///  - [Linux `ipv6`]
///  - [Winsock2 `getsockopt`]
///  - [Winsock2 `IPPROTO_IPV6` options]
///
/// [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `ipv6`]: https://man7.org/linux/man-pages/man7/ipv6.7.html
/// [Winsock2 `getsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockopt
/// [Winsock2 `IPPROTO_IPV6` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ipv6-socket-options
#[inline]
#[doc(alias = "IPV6_V6ONLY")]
pub fn get_ipv6_v6only<Fd: AsFd>(fd: &Fd) -> io::Result<bool> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_ipv6_v6only(fd)
}

/// `setsockopt(fd, IPPROTO_IP, IP_MULTICAST_LOOP, multicast_loop)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `setsockopt`]
///  - [Linux `ip`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `IPPROTO_IP` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `ip`]: https://man7.org/linux/man-pages/man7/ip.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_IP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options
#[inline]
#[doc(alias = "IP_MULTICAST_LOOP")]
pub fn set_ip_multicast_loop<Fd: AsFd>(fd: &Fd, multicast_loop: bool) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_ip_multicast_loop(fd, multicast_loop)
}

/// `getsockopt(fd, IPPROTO_IP, IP_MULTICAST_LOOP)`
///
/// # References
///  - [POSIX `getsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `getsockopt`]
///  - [Linux `ip`]
///  - [Winsock2 `getsockopt`]
///  - [Winsock2 `IPPROTO_IP` options]
///
/// [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `ip`]: https://man7.org/linux/man-pages/man7/ip.7.html
/// [Winsock2 `getsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockopt
/// [Winsock2 `IPPROTO_IP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options
#[inline]
#[doc(alias = "IP_MULTICAST_LOOP")]
pub fn get_ip_multicast_loop<Fd: AsFd>(fd: &Fd) -> io::Result<bool> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_ip_multicast_loop(fd)
}

/// `setsockopt(fd, IPPROTO_IP, IP_MULTICAST_TTL, multicast_ttl)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `setsockopt`]
///  - [Linux `ip`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `IPPROTO_IP` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `ip`]: https://man7.org/linux/man-pages/man7/ip.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_IP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options
#[inline]
#[doc(alias = "IP_MULTICAST_TTL")]
pub fn set_ip_multicast_ttl<Fd: AsFd>(fd: &Fd, multicast_ttl: u32) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_ip_multicast_ttl(fd, multicast_ttl)
}

/// `getsockopt(fd, IPPROTO_IP, IP_MULTICAST_TTL)`
///
/// # References
///  - [POSIX `getsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `getsockopt`]
///  - [Linux `ip`]
///  - [Winsock2 `getsockopt`]
///  - [Winsock2 `IPPROTO_IP` options]
///
/// [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `ip`]: https://man7.org/linux/man-pages/man7/ip.7.html
/// [Winsock2 `getsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockopt
/// [Winsock2 `IPPROTO_IP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options
#[inline]
#[doc(alias = "IP_MULTICAST_TTL")]
pub fn get_ip_multicast_ttl<Fd: AsFd>(fd: &Fd) -> io::Result<u32> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_ip_multicast_ttl(fd)
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_LOOP, multicast_loop)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `setsockopt`]
///  - [Linux `ipv6`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `IPPROTO_IPV6` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `ipv6`]: https://man7.org/linux/man-pages/man7/ipv6.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_IPV6` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ipv6-socket-options
#[inline]
#[doc(alias = "IPV6_MULTICAST_LOOP")]
pub fn set_ipv6_multicast_loop<Fd: AsFd>(fd: &Fd, multicast_loop: bool) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_ipv6_multicast_loop(fd, multicast_loop)
}

/// `getsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_LOOP)`
///
/// # References
///  - [POSIX `getsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `getsockopt`]
///  - [Linux `ipv6`]
///  - [Winsock2 `getsockopt`]
///  - [Winsock2 `IPPROTO_IPV6` options]
///
/// [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `ipv6`]: https://man7.org/linux/man-pages/man7/ipv6.7.html
/// [Winsock2 `getsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-getsockopt
/// [Winsock2 `IPPROTO_IPV6` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ipv6-socket-options
#[inline]
#[doc(alias = "IPV6_MULTICAST_LOOP")]
pub fn get_ipv6_multicast_loop<Fd: AsFd>(fd: &Fd) -> io::Result<bool> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_ipv6_multicast_loop(fd)
}

/// `setsockopt(fd, IPPROTO_IP, IP_ADD_MEMBERSHIP, multiaddr, interface)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `setsockopt`]
///  - [Linux `ip`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `IPPROTO_IP` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `ip`]: https://man7.org/linux/man-pages/man7/ip.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_IP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options
#[inline]
#[doc(alias = "IP_ADD_MEMBERSHIP")]
pub fn set_ip_add_membership<Fd: AsFd>(
    fd: &Fd,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_ip_add_membership(fd, multiaddr, interface)
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_ADD_MEMBERSHIP, multiaddr, interface)`
///
/// `IPV6_ADD_MEMBERSHIP` is the same as `IPV6_JOIN_GROUP` in POSIX.
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `setsockopt`]
///  - [Linux `ipv6]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `IPPROTO_IPV6` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `ipv6`]: https://man7.org/linux/man-pages/man7/ipv6.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_IPV6` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ipv6-socket-options
#[inline]
#[doc(alias = "IPV6_JOIN_GROUP")]
#[doc(alias = "IPV6_ADD_MEMBERSHIP")]
pub fn set_ipv6_add_membership<Fd: AsFd>(
    fd: &Fd,
    multiaddr: &Ipv6Addr,
    interface: u32,
) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_ipv6_add_membership(fd, multiaddr, interface)
}

/// `setsockopt(fd, IPPROTO_IP, IP_DROP_MEMBERSHIP, multiaddr, interface)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `setsockopt`]
///  - [Linux `ip`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `IPPROTO_IP` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `ip`]: https://man7.org/linux/man-pages/man7/ip.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_IP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options
#[inline]
#[doc(alias = "IP_DROP_MEMBERSHIP")]
pub fn set_ip_drop_membership<Fd: AsFd>(
    fd: &Fd,
    multiaddr: &Ipv4Addr,
    interface: &Ipv4Addr,
) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_ip_drop_membership(fd, multiaddr, interface)
}

/// `setsockopt(fd, IPPROTO_IPV6, IPV6_DROP_MEMBERSHIP, multiaddr, interface)`
///
/// `IPV6_DROP_MEMBERSHIP` is the same as `IPV6_LEAVE_GROUP` in POSIX.
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `netinet/in.h`]
///  - [Linux `setsockopt`]
///  - [Linux `ipv6`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `IPPROTO_IPV6` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `netinet/in.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_in.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `ipv6`]: https://man7.org/linux/man-pages/man7/ipv6.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_IPV6` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ipv6-socket-options
#[inline]
#[doc(alias = "IPV6_LEAVE_GROUP")]
#[doc(alias = "IPV6_DROP_MEMBERSHIP")]
pub fn set_ipv6_drop_membership<Fd: AsFd>(
    fd: &Fd,
    multiaddr: &Ipv6Addr,
    interface: u32,
) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_ipv6_drop_membership(fd, multiaddr, interface)
}

/// `setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, nodelay)`
///
/// # References
///  - [POSIX `setsockopt`]
///  - [POSIX `netinet/tcp.h`]
///  - [Linux `setsockopt`]
///  - [Linux `tcp`]
///  - [Winsock2 `setsockopt`]
///  - [Winsock2 `IPPROTO_TCP` options]
///
/// [POSIX `setsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setsockopt.html
/// [POSIX `netinet/tcp.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_tcp.h.html
/// [Linux `setsockopt`]: https://man7.org/linux/man-pages/man2/setsockopt.2.html
/// [Linux `tcp`]: https://man7.org/linux/man-pages/man7/tcp.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_TCP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-tcp-socket-options
#[inline]
#[doc(alias = "TCP_NODELAY")]
pub fn set_tcp_nodelay<Fd: AsFd>(fd: &Fd, nodelay: bool) -> io::Result<()> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::set_tcp_nodelay(fd, nodelay)
}

/// `getsockopt(fd, IPPROTO_TCP, TCP_NODELAY)`
///
/// # References
///  - [POSIX `getsockopt`]
///  - [POSIX `netinet/tcp.h`]
///  - [Linux `getsockopt`]
///  - [Linux `tcp`]
///  - [Winsock2 `getsockopt`]
///  - [Winsock2 `IPPROTO_TCP` options]
///
/// [POSIX `getsockopt`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockopt.html
/// [POSIX `netinet/tcp.h`]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/netinet_tcp.h.html
/// [Linux `getsockopt`]: https://man7.org/linux/man-pages/man2/getsockopt.2.html
/// [Linux `tcp`]: https://man7.org/linux/man-pages/man7/tcp.7.html
/// [Winsock2 `setsockopt`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-setsockopt
/// [Winsock2 `IPPROTO_TCP` options]: https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-tcp-socket-options
#[inline]
#[doc(alias = "TCP_NODELAY")]
pub fn get_tcp_nodelay<Fd: AsFd>(fd: &Fd) -> io::Result<bool> {
    let fd = fd.as_fd();
    imp::syscalls::sockopt::get_tcp_nodelay(fd)
}
