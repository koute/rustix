#![cfg(not(windows))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]

mod fcntl;
mod file;
#[cfg(not(target_os = "wasi"))]
mod flock;
mod invalid_offset;
mod long_paths;
#[cfg(not(any(
    target_os = "ios",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "wasi"
)))]
mod makedev;
mod mkdirat;
mod mknodat;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod openat2;
mod readdir;
mod renameat;
mod statfs;
