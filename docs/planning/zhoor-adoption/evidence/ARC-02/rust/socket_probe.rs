// NON-PRODUCTION ARC-02 SPIKE: Linux AF_UNIX/SO_PEERCRED probe.
use std::fs;
use std::io;
use std::mem::size_of;
use std::os::fd::AsRawFd;
use std::os::raw::{c_int, c_void};
use std::os::unix::net::{UnixListener, UnixStream};

#[repr(C)]
struct UCred { pid: i32, uid: u32, gid: u32 }
unsafe extern "C" {
    fn getsockopt(fd: c_int, level: c_int, name: c_int, value: *mut c_void, len: *mut u32) -> c_int;
}
fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join(format!("zhoor-arc02-peer-{}.sock", std::process::id()));
    let _ = fs::remove_file(&path);
    let listener = UnixListener::bind(&path)?;
    let client = UnixStream::connect(&path)?;
    let (server, _) = listener.accept()?;
    let mut cred = UCred { pid: 0, uid: 0, gid: 0 };
    let mut len = size_of::<UCred>() as u32;
    // SAFETY: server is a live AF_UNIX stream; value points to a correctly sized Linux ucred.
    let rc = unsafe { getsockopt(server.as_raw_fd(), 1, 17, (&mut cred as *mut UCred).cast(), &mut len) };
    drop(client);
    drop(server);
    drop(listener);
    fs::remove_file(path)?;
    if rc != 0 { return Err(io::Error::last_os_error()); }
    println!("pid={} uid={} gid={} mechanism=getsockopt(SOL_SOCKET,SO_PEERCRED) unsafe_ffi=true", cred.pid, cred.uid, cred.gid);
    Ok(())
}
