#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

// Direct translation of user-trap.c. System constants, types, and functions
// below are supplied by the surrounding system bindings.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const ARRAY_SIZE_FILTER: usize = 4;

extern "C" {
    fn syscall(number: c_long, ...) -> c_long;
    fn sendmsg(sock: c_int, msg: *const msghdr, flags: c_int) -> c_long;
    fn recvmsg(sock: c_int, msg: *mut msghdr, flags: c_int) -> c_long;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...);
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ... ) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn lseek(fd: c_int, offset: c_ulong, whence: c_int) -> c_long;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn mount(source: *const c_char, target: *const c_char, filesystemtype: *const c_char, mountflags: c_ulong, data: *const c_void) -> c_int;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn socketpair(domain: c_int, ty: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn fork() -> c_int;
    fn setuid(uid: c_uint) -> c_int;
    fn mkdir(path: *const c_char, mode: c_uint) -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn remove(path: *const c_char) -> c_int;
    static mut errno: c_int;
    static mut stderr: *mut c_void;
}

#[repr(C)]
struct iovec { iov_base: *mut c_void, iov_len: usize }
#[repr(C)]
struct msghdr {
    msg_name: *mut c_void, msg_namelen: u32, msg_iov: *mut iovec,
    msg_iovlen: usize, msg_control: *mut c_void, msg_controllen: usize,
    msg_flags: c_int,
}
#[repr(C)] struct cmsghdr { cmsg_len: usize, cmsg_level: c_int, cmsg_type: c_int }
#[repr(C)] struct sock_filter { code: u16, jt: u8, jf: u8, k: u32 }
#[repr(C)] struct sock_fprog { len: u16, filter: *mut sock_filter }
#[repr(C)] struct seccomp_data { nr: c_int, arch: u32, instruction_pointer: u64, args: [u64; 6] }
#[repr(C)] struct seccomp_notif { id: u64, pid: u32, flags: u32, data: seccomp_data }
#[repr(C)] struct seccomp_notif_resp { id: u64, val: i64, error: i32, flags: u32 }
#[repr(C)] struct seccomp_notif_sizes { seccomp_notif: u16, seccomp_notif_resp: u16, seccomp_data: u16 }

const EPERM: c_int = 1;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const O_RDONLY: c_int = 0;
const SEEK_SET: c_int = 0;
const MS_BIND: c_ulong = 4096;
const MNT_DETACH: c_int = 2;
const SIGKILL: c_int = 9;
const PF_LOCAL: c_int = 1;
const SOCK_SEQPACKET: c_int = 5;
const SOL_SOCKET: c_int = 1;
const SCM_RIGHTS: c_int = 1;
const SECCOMP_SET_MODE_FILTER: c_uint = 1;
const SECCOMP_GET_NOTIF_SIZES: c_uint = 3;
const SECCOMP_FILTER_FLAG_NEW_LISTENER: c_uint = 8;
const SECCOMP_RET_USER_NOTIF: u32 = 0x7fc00000;
const SECCOMP_RET_ALLOW: u32 = 0x7fff0000;
const __NR_mount: c_int = 165;
const PATH_MAX: usize = 4096;
const SECCOMP_IOCTL_NOTIF_ID_VALID: c_ulong = 0;
const SECCOMP_IOCTL_NOTIF_RECV: c_ulong = 0;
const SECCOMP_IOCTL_NOTIF_SEND: c_ulong = 0;

unsafe fn seccomp(op: c_uint, flags: c_uint, args: *mut c_void) -> c_int {
    errno = 0;
    syscall(157, op, flags, args) as c_int
}

unsafe fn send_fd(sock: c_int, fd: c_int) -> c_int {
    let mut msg: msghdr = core::mem::zeroed();
    let mut buf = [0u8; 32]; let mut c = b'c';
    let mut io = iovec { iov_base: &mut c as *mut u8 as *mut c_void, iov_len: 1 };
    msg.msg_iov = &mut io; msg.msg_iovlen = 1; msg.msg_control = buf.as_mut_ptr() as *mut c_void; msg.msg_controllen = buf.len();
    let cmsg = msg.msg_control as *mut cmsghdr;
    (*cmsg).cmsg_level = SOL_SOCKET; (*cmsg).cmsg_type = SCM_RIGHTS; (*cmsg).cmsg_len = 20;
    *((cmsg.add(1)) as *mut c_int) = fd; msg.msg_controllen = (*cmsg).cmsg_len;
    if sendmsg(sock, &msg, 0) < 0 { perror(c"sendmsg".as_ptr()); return -1; } 0
}

unsafe fn recv_fd(sock: c_int) -> c_int {
    let mut msg: msghdr = core::mem::zeroed(); let mut buf = [0u8; 32]; let mut c = b'c';
    let mut io = iovec { iov_base: &mut c as *mut u8 as *mut c_void, iov_len: 1 };
    msg.msg_iov = &mut io; msg.msg_iovlen = 1; msg.msg_control = buf.as_mut_ptr() as *mut c_void; msg.msg_controllen = buf.len();
    if recvmsg(sock, &mut msg, 0) < 0 { perror(c"recvmsg".as_ptr()); return -1; }
    *((msg.msg_control as *mut cmsghdr).add(1) as *mut c_int)
}

unsafe fn user_trap_syscall(nr: c_int, flags: c_uint) -> c_int {
    let mut filter = [
        sock_filter { code: 0x20, jt: 0, jf: 0, k: 0 },
        sock_filter { code: 0x15, jt: 0, jf: 1, k: nr as u32 },
        sock_filter { code: 0x06, jt: 0, jf: 0, k: SECCOMP_RET_USER_NOTIF },
        sock_filter { code: 0x06, jt: 0, jf: 0, k: SECCOMP_RET_ALLOW },
    ];
    let mut prog = sock_fprog { len: ARRAY_SIZE_FILTER as u16, filter: filter.as_mut_ptr() };
    seccomp(SECCOMP_SET_MODE_FILTER, flags, &mut prog as *mut _ as *mut c_void)
}

unsafe fn handle_req(req: *mut seccomp_notif, resp: *mut seccomp_notif_resp, listener: c_int) -> c_int {
    let mut path = [0i8; PATH_MAX]; let mut source = [0i8; PATH_MAX]; let mut target = [0i8; PATH_MAX];
    let mut ret = -1; let mem;
    (*resp).id = (*req).id; (*resp).error = -EPERM; (*resp).val = 0;
    if (*req).data.nr != __NR_mount { fprintf(stderr, c"huh? trapped something besides mount? %d\n".as_ptr(), (*req).data.nr); return -1; }
    if (*req).data.args[3] & MS_BIND == 0 { return 0; }
    snprintf(path.as_mut_ptr(), path.len(), c"/proc/%d/mem".as_ptr(), (*req).pid);
    mem = open(path.as_ptr(), O_RDONLY); if mem < 0 { perror(c"open mem".as_ptr()); return -1; }
    if ioctl(listener, SECCOMP_IOCTL_NOTIF_ID_VALID, &mut (*req).id) < 0 { fprintf(stderr, c"task died before we could map its memory\n".as_ptr()); close(mem); return ret; }
    if lseek(mem, (*req).data.args[0] as c_ulong, SEEK_SET) < 0 { perror(c"seek".as_ptr()); close(mem); return ret; }
    ret = read(mem, source.as_mut_ptr() as *mut c_void, source.len()) as c_int; if ret < 0 { perror(c"read".as_ptr()); close(mem); return ret; }
    if lseek(mem, (*req).data.args[1] as c_ulong, SEEK_SET) < 0 { perror(c"seek".as_ptr()); close(mem); return ret; }
    ret = read(mem, target.as_mut_ptr() as *mut c_void, target.len()) as c_int; if ret < 0 { perror(c"read".as_ptr()); close(mem); return ret; }
    if strncmp(source.as_ptr(), c"/tmp/".as_ptr(), 5) == 0 && strncmp(target.as_ptr(), c"/tmp/".as_ptr(), 5) == 0 {
        if mount(source.as_ptr(), target.as_ptr(), core::ptr::null(), (*req).data.args[3], core::ptr::null()) < 0 { perror(c"actual mount".as_ptr()); close(mem); return -1; }
        (*resp).error = 0;
    }
    ret = 0; close(mem); ret
}

pub unsafe fn main() -> c_int {
    let mut sk_pair = [0; 2]; let mut ret = 1; let mut status = 0; let mut listener; let mut worker = 0; let mut tracer = 0;
    if socketpair(PF_LOCAL, SOCK_SEQPACKET, 0, sk_pair.as_mut_ptr()) < 0 { perror(c"socketpair".as_ptr()); return 1; }
    worker = fork(); if worker < 0 { perror(c"fork".as_ptr()); close(sk_pair[0]); close(sk_pair[1]); return ret; }
    if worker == 0 {
        listener = user_trap_syscall(__NR_mount, SECCOMP_FILTER_FLAG_NEW_LISTENER); if listener < 0 { perror(c"seccomp".as_ptr()); return 1; }
        if setuid(1000) < 0 { perror(c"setuid".as_ptr()); return 1; }
        if send_fd(sk_pair[1], listener) < 0 { return 1; } close(listener);
        if mkdir(c"/tmp/foo".as_ptr(), 0o755) < 0 { perror(c"mkdir".as_ptr()); return 1; }
        if mount(c"/dev/sda".as_ptr(), c"/tmp/foo".as_ptr(), core::ptr::null(), 0, core::ptr::null()) != -1 { fprintf(stderr, c"huh? mounted /dev/sda?\n".as_ptr()); return 1; }
        if errno != EPERM { perror(c"bad error from mount".as_ptr()); return 1; }
        if mount(c"/tmp/foo".as_ptr(), c"/tmp/foo".as_ptr(), core::ptr::null(), MS_BIND, core::ptr::null()) < 0 { perror(c"mount".as_ptr()); return 1; }
        return 0;
    }
    listener = recv_fd(sk_pair[0]); if listener < 0 { kill(worker, SIGKILL); return ret; }
    tracer = fork(); if tracer < 0 { perror(c"fork".as_ptr()); kill(worker, SIGKILL); return ret; }
    if tracer == 0 {
        let mut sizes: seccomp_notif_sizes = core::mem::zeroed(); if seccomp(SECCOMP_GET_NOTIF_SIZES, 0, &mut sizes as *mut _ as *mut c_void) < 0 { perror(c"seccomp(GET_NOTIF_SIZES)".as_ptr()); return 1; }
        let req = malloc(sizes.seccomp_notif as usize) as *mut seccomp_notif; if req.is_null() { return 1; }
        let resp = malloc(sizes.seccomp_notif_resp as usize) as *mut seccomp_notif_resp; if resp.is_null() { free(req as *mut c_void); return 1; } memset(resp as *mut c_void, 0, sizes.seccomp_notif_resp as usize);
        loop { memset(req as *mut c_void, 0, sizes.seccomp_notif as usize); if ioctl(listener, SECCOMP_IOCTL_NOTIF_RECV, req) != 0 { perror(c"ioctl recv".as_ptr()); break; } if handle_req(req, resp, listener) < 0 { break; } if ioctl(listener, SECCOMP_IOCTL_NOTIF_SEND, resp) < 0 && errno != ENOENT { perror(c"ioctl send".as_ptr()); break; } }
        free(resp as *mut c_void); free(req as *mut c_void); close(listener); return 1;
    }
    close(listener);
    if waitpid(worker, &mut status, 0) != worker { perror(c"waitpid".as_ptr()); kill(tracer, SIGKILL); kill(worker, SIGKILL); return ret; }
    if umount2(c"/tmp/foo".as_ptr(), MNT_DETACH) < 0 && errno != EINVAL { perror(c"umount2".as_ptr()); kill(tracer, SIGKILL); kill(worker, SIGKILL); return ret; }
    if remove(c"/tmp/foo".as_ptr()) < 0 && errno != ENOENT { perror(c"remove".as_ptr()); return 1; }
    if status != 0 { fprintf(stderr, c"worker exited nonzero\n".as_ptr()); kill(tracer, SIGKILL); kill(worker, SIGKILL); return ret; }
    ret = 0; if tracer > 0 { kill(tracer, SIGKILL); } if worker > 0 { kill(worker, SIGKILL); } close(sk_pair[0]); close(sk_pair[1]); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
