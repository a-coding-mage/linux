/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translation of linux/net.h. Included C dependencies are supplied elsewhere. */

#[repr(C)]
pub struct sockopt_t {
    pub iter_in: iov_iter,
    pub iter_out: iov_iter,
    pub optlen: ::core::ffi::c_int,
}

pub unsafe fn sockopt_init_user(
    opt: *mut sockopt_t,
    optval: *mut ::core::ffi::c_char,
    optlen: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    if get_user(&mut len as *mut _, optlen) != 0 {
        return -EFAULT;
    }
    if len < 0 {
        return -EINVAL;
    }
    iov_iter_ubuf(&mut (*opt).iter_out, ITER_DEST, optval as *mut _, len as usize);
    iov_iter_ubuf(&mut (*opt).iter_in, ITER_SOURCE, optval as *mut _, len as usize);
    (*opt).optlen = len;
    0
}

pub enum poll_table_struct {}
pub enum pipe_inode_info {}
pub enum inode {}
pub enum file {}
pub enum net {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum socket_flags {
    SOCKWQ_ASYNC_NOSPACE,
    SOCKWQ_ASYNC_WAITDATA,
    SOCK_NOSPACE,
    SOCK_SUPPORT_ZC,
    SOCK_CUSTOM_SOCKOPT,
}

// C enum sock_type is omitted when ARCH_HAS_SOCKET_TYPES is supplied.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum sock_type {
    SOCK_STREAM = 1,
    SOCK_DGRAM = 2,
    SOCK_RAW = 3,
    SOCK_RDM = 4,
    SOCK_SEQPACKET = 5,
    SOCK_DCCP = 6,
    SOCK_PACKET = 10,
}

pub const SOCK_MAX: usize = 11;
pub const SOCK_TYPE_MASK: usize = 0xf;
pub const SOCK_CLOEXEC: _ = O_CLOEXEC;
pub const SOCK_NONBLOCK: _ = O_NONBLOCK;
pub const SOCK_COREDUMP: _ = O_NOCTTY;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sock_shutdown_cmd { SHUT_RD, SHUT_WR, SHUT_RDWR }

#[repr(C)]
pub struct socket_wq {
    pub wait: wait_queue_head_t,
    pub fasync_list: *mut fasync_struct,
    pub flags: usize,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct socket {
    pub state: socket_state,
    pub type_: ::core::ffi::c_short,
    pub flags: usize,
    pub file: *mut file,
    pub sk: *mut sock,
    pub ops: *const proto_ops,
    pub wq: socket_wq,
}

#[repr(C)]
pub union read_descriptor_arg {
    pub buf: *mut ::core::ffi::c_char,
    pub data: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct read_descriptor_t {
    pub written: usize,
    pub count: usize,
    pub arg: read_descriptor_arg,
    pub error: ::core::ffi::c_int,
}

pub enum vm_area_struct {}
pub enum page {}
pub enum msghdr {}
pub enum module {}
pub enum sk_buff {}
pub enum proto_accept_arg {}

pub type sk_read_actor_t = unsafe extern "C" fn(*mut read_descriptor_t, *mut sk_buff, u32, usize) -> ::core::ffi::c_int;
pub type skb_read_actor_t = unsafe extern "C" fn(*mut sock, *mut sk_buff) -> ::core::ffi::c_int;

#[repr(C)]
pub struct proto_ops {
    pub family: ::core::ffi::c_int,
    pub owner: *mut module,
    pub release: Option<unsafe extern "C" fn(*mut socket) -> ::core::ffi::c_int>,
    pub bind: Option<unsafe extern "C" fn(*mut socket, *mut sockaddr_unsized, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub connect: Option<unsafe extern "C" fn(*mut socket, *mut sockaddr_unsized, ::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub socketpair: Option<unsafe extern "C" fn(*mut socket, *mut socket) -> ::core::ffi::c_int>,
    pub accept: Option<unsafe extern "C" fn(*mut socket, *mut socket, *mut proto_accept_arg) -> ::core::ffi::c_int>,
    pub getname: Option<unsafe extern "C" fn(*mut socket, *mut sockaddr, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut socket, *mut poll_table_struct) -> __poll_t>,
    pub ioctl: Option<unsafe extern "C" fn(*mut socket, u32, usize) -> ::core::ffi::c_int>,
    // CONFIG_COMPAT may add compat_ioctl here.
    pub gettstamp: Option<unsafe extern "C" fn(*mut socket, *mut ::core::ffi::c_void, bool, bool) -> ::core::ffi::c_int>,
    pub listen: Option<unsafe extern "C" fn(*mut socket, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut socket, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub setsockopt: Option<unsafe extern "C" fn(*mut socket, ::core::ffi::c_int, ::core::ffi::c_int, sockptr_t, u32) -> ::core::ffi::c_int>,
    pub getsockopt: Option<unsafe extern "C" fn(*mut socket, ::core::ffi::c_int, ::core::ffi::c_int, *mut ::core::ffi::c_char, *mut ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub getsockopt_iter: Option<unsafe extern "C" fn(*mut socket, ::core::ffi::c_int, ::core::ffi::c_int, *mut sockopt_t) -> ::core::ffi::c_int>,
    pub show_fdinfo: Option<unsafe extern "C" fn(*mut seq_file, *mut socket)>,
    pub sendmsg: Option<unsafe extern "C" fn(*mut socket, *mut msghdr, usize) -> ::core::ffi::c_int>,
    pub recvmsg: Option<unsafe extern "C" fn(*mut socket, *mut msghdr, usize, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut socket, *mut vm_area_struct) -> ::core::ffi::c_int>,
    pub splice_read: Option<unsafe extern "C" fn(*mut socket, *mut loff_t, *mut pipe_inode_info, usize, u32) -> ssize_t>,
    pub splice_eof: Option<unsafe extern "C" fn(*mut socket)>,
    pub set_peek_off: Option<unsafe extern "C" fn(*mut sock, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub peek_len: Option<unsafe extern "C" fn(*mut socket) -> ::core::ffi::c_int>,
    pub read_sock: Option<unsafe extern "C" fn(*mut sock, *mut read_descriptor_t, sk_read_actor_t) -> ::core::ffi::c_int>,
    pub read_skb: Option<unsafe extern "C" fn(*mut sock, skb_read_actor_t) -> ::core::ffi::c_int>,
    pub sendmsg_locked: Option<unsafe extern "C" fn(*mut sock, *mut msghdr, usize) -> ::core::ffi::c_int>,
    pub set_rcvlowat: Option<unsafe extern "C" fn(*mut sock, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub set_rcvbuf: Option<unsafe extern "C" fn(*mut sock, ::core::ffi::c_int)>,
}

#[repr(C)]
pub struct net_proto_family {
    pub family: ::core::ffi::c_int,
    pub create: Option<unsafe extern "C" fn(*mut net, *mut socket, ::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub owner: *mut module,
}

pub enum iovec {}
pub enum kvec {}

pub const SOCK_WAKE_IO: usize = 0;
pub const SOCK_WAKE_WAITD: usize = 1;
pub const SOCK_WAKE_SPACE: usize = 2;
pub const SOCK_WAKE_URG: usize = 3;

extern "C" {
    pub fn sock_wake_async(sk_wq: *mut socket_wq, how: ::core::ffi::c_int, band: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn sock_register(fam: *const net_proto_family) -> ::core::ffi::c_int;
    pub fn sock_unregister(family: ::core::ffi::c_int);
    pub fn sock_is_registered(family: ::core::ffi::c_int) -> bool;
    pub fn __sock_create(net: *mut net, family: ::core::ffi::c_int, type_: ::core::ffi::c_int, proto: ::core::ffi::c_int, res: *mut *mut socket, kern: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn sock_create(family: ::core::ffi::c_int, type_: ::core::ffi::c_int, proto: ::core::ffi::c_int, res: *mut *mut socket) -> ::core::ffi::c_int;
    pub fn sock_create_kern(net: *mut net, family: ::core::ffi::c_int, type_: ::core::ffi::c_int, proto: ::core::ffi::c_int, res: *mut *mut socket) -> ::core::ffi::c_int;
    pub fn sock_create_lite(family: ::core::ffi::c_int, type_: ::core::ffi::c_int, proto: ::core::ffi::c_int, res: *mut *mut socket) -> ::core::ffi::c_int;
    pub fn sock_alloc() -> *mut socket;
    pub fn sock_release(sock: *mut socket);
    pub fn sock_sendmsg(sock: *mut socket, msg: *mut msghdr) -> ::core::ffi::c_int;
    pub fn sock_recvmsg(sock: *mut socket, msg: *mut msghdr, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn sock_alloc_file(sock: *mut socket, flags: ::core::ffi::c_int, dname: *const ::core::ffi::c_char) -> *mut file;
    pub fn sockfd_lookup(fd: ::core::ffi::c_int, err: *mut ::core::ffi::c_int) -> *mut socket;
    pub fn sock_from_file(file: *mut file) -> *mut socket;
    pub fn sock_read_xattr(sock: *mut socket, name: *const ::core::ffi::c_char, value: *mut ::core::ffi::c_void, size: usize) -> ::core::ffi::c_int;
    pub fn net_ratelimit() -> ::core::ffi::c_int;
    pub fn kernel_sendmsg(sock: *mut socket, msg: *mut msghdr, vec: *mut kvec, num: usize, len: usize) -> ::core::ffi::c_int;
    pub fn kernel_recvmsg(sock: *mut socket, msg: *mut msghdr, vec: *mut kvec, num: usize, len: usize, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn kernel_bind(sock: *mut socket, addr: *mut sockaddr_unsized, addrlen: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn kernel_listen(sock: *mut socket, backlog: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn kernel_accept(sock: *mut socket, newsock: *mut *mut socket, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn kernel_connect(sock: *mut socket, addr: *mut sockaddr_unsized, addrlen: ::core::ffi::c_int, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn kernel_getsockname(sock: *mut socket, addr: *mut sockaddr) -> ::core::ffi::c_int;
    pub fn kernel_getpeername(sock: *mut socket, addr: *mut sockaddr) -> ::core::ffi::c_int;
    pub fn kernel_sock_shutdown(sock: *mut socket, how: sock_shutdown_cmd) -> ::core::ffi::c_int;
    pub fn kernel_sock_ip_overhead(sk: *mut sock) -> u32;
}

pub unsafe fn sendpage_ok(page: *mut page) -> bool { !PageSlab(page) && page_count(page) >= 1 }

pub unsafe fn sendpages_ok(page: *mut page, len: usize, offset: usize) -> bool {
    let mut p = page.add(offset >> PAGE_SHIFT);
    let mut count = 0usize;
    while count < len {
        if !sendpage_ok(p) { return false; }
        p = p.add(1);
        count += PAGE_SIZE;
    }
    true
}

// C-only macros (ratelimited logging, random-once wrappers, module aliases,
// DECLARE_SOCKADDR, and sockfd_put) retain their source-level intent here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
