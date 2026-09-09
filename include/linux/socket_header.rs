/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/socket.h. C includes and configuration-dependent
// declarations are supplied by the surrounding kernel translation.

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
pub type sa_family_t = __kernel_sa_family_t;

extern "C" {
    pub fn socket_seq_show(seq: *mut seq_file);
    pub fn move_addr_to_kernel(uaddr: *mut core::ffi::c_void, ulen: i32, kaddr: *mut sockaddr_storage) -> i32;
    pub fn put_cmsg(msg: *mut msghdr, level: i32, type_: i32, len: i32, data: *mut core::ffi::c_void) -> i32;
    pub fn put_cmsg_notrunc(msg: *mut msghdr, level: i32, type_: i32, len: i32, data: *mut core::ffi::c_void) -> i32;
    pub fn put_cmsg_scm_timestamping64(msg: *mut msghdr, tss: *mut scm_timestamping_internal);
    pub fn put_cmsg_scm_timestamping(msg: *mut msghdr, tss: *mut scm_timestamping_internal);
    pub fn __sys_recvmsg(fd: i32, msg: *mut user_msghdr, flags: u32, forbid_cmsg_compat: bool) -> i64;
    pub fn __sys_sendmsg(fd: i32, msg: *mut user_msghdr, flags: u32, forbid_cmsg_compat: bool) -> i64;
    pub fn __sys_recvmmsg(fd: i32, mmsg: *mut mmsghdr, vlen: u32, flags: u32, timeout: *mut __kernel_timespec, timeout32: *mut old_timespec32) -> i32;
    pub fn __sys_sendmmsg(fd: i32, mmsg: *mut mmsghdr, vlen: u32, flags: u32, forbid_cmsg_compat: bool) -> i32;
    pub fn __sys_sendmsg_sock(sock: *mut socket, msg: *mut msghdr, flags: u32) -> i64;
    pub fn __sys_recvmsg_sock(sock: *mut socket, msg: *mut msghdr, umsg: *mut user_msghdr, uaddr: *mut sockaddr, flags: u32) -> i64;
    pub fn __copy_msghdr(kmsg: *mut msghdr, umsg: *mut user_msghdr, save_addr: *mut *mut sockaddr) -> i32;
    pub fn __sys_recvfrom(fd: i32, ubuf: *mut core::ffi::c_void, size: usize, flags: u32, addr: *mut sockaddr, addr_len: *mut i32) -> i32;
    pub fn __sys_sendto(fd: i32, buff: *mut core::ffi::c_void, len: usize, flags: u32, addr: *mut sockaddr, addr_len: i32) -> i32;
    pub fn do_accept(file: *mut file, arg: *mut proto_accept_arg, upeer_sockaddr: *mut sockaddr, upeer_addrlen: *mut i32, flags: i32) -> *mut file;
    pub fn __sys_accept4(fd: i32, upeer_sockaddr: *mut sockaddr, upeer_addrlen: *mut i32, flags: i32) -> i32;
    pub fn __sys_socket(family: i32, type_: i32, protocol: i32) -> i32;
    pub fn __sys_socket_file(family: i32, type_: i32, protocol: i32) -> *mut file;
    pub fn __sys_bind(fd: i32, umyaddr: *mut sockaddr, addrlen: i32) -> i32;
    pub fn __sys_bind_socket(sock: *mut socket, address: *mut sockaddr_storage, addrlen: i32) -> i32;
    pub fn connect_socket(sock: *mut socket, addr: *mut sockaddr_storage, addrlen: i32, flags: i32) -> i32;
    pub fn __sys_connect_file(file: *mut file, addr: *mut sockaddr_storage, addrlen: i32, file_flags: i32) -> i32;
    pub fn __sys_connect(fd: i32, uservaddr: *mut sockaddr, addrlen: i32) -> i32;
    pub fn __sys_listen(fd: i32, backlog: i32) -> i32;
    pub fn __sys_listen_socket(sock: *mut socket, backlog: i32) -> i32;
    pub fn do_getsockname(sock: *mut socket, peer: i32, usockaddr: *mut sockaddr, usockaddr_len: *mut i32) -> i32;
    pub fn __sys_getsockname(fd: i32, usockaddr: *mut sockaddr, usockaddr_len: *mut i32, peer: i32) -> i32;
    pub fn __sys_socketpair(family: i32, type_: i32, protocol: i32, usockvec: *mut i32) -> i32;
    pub fn __sys_shutdown_sock(sock: *mut socket, how: i32) -> i32;
    pub fn __sys_shutdown(fd: i32, how: i32) -> i32;
}

#[repr(C)] pub struct sockaddr { pub sa_family: sa_family_t, pub sa_data: [i8; 14] }
#[repr(C)] pub struct sockaddr_unsized { pub sa_family: __kernel_sa_family_t, pub sa_data: [i8; 0] }
#[repr(C)] pub struct linger { pub l_onoff: i32, pub l_linger: i32 }
pub type sockaddr_storage = __kernel_sockaddr_storage;

#[repr(C)] pub union msghdr_control { pub msg_control: *mut core::ffi::c_void, pub msg_control_user: *mut core::ffi::c_void }
#[repr(C)] pub struct msghdr {
    pub msg_name: *mut core::ffi::c_void, pub msg_namelen: i32, pub msg_inq: i32,
    pub msg_iter: iov_iter, pub control: msghdr_control, pub msg_control_is_user: bool,
    pub msg_get_inq: bool, pub msg_flags: u32, pub msg_controllen: __kernel_size_t,
    pub msg_ubuf: *mut ubuf_info,
    pub sg_from_iter: Option<unsafe extern "C" fn(*mut sk_buff, *mut iov_iter, usize) -> i32>,
}
#[repr(C)] pub struct user_msghdr { pub msg_name: *mut core::ffi::c_void, pub msg_namelen: i32, pub msg_iov: *mut iovec, pub msg_iovlen: __kernel_size_t, pub msg_control: *mut core::ffi::c_void, pub msg_controllen: __kernel_size_t, pub msg_flags: u32 }
#[repr(C)] pub struct mmsghdr { pub msg_hdr: user_msghdr, pub msg_len: u32 }
#[repr(C)] pub struct cmsghdr { pub cmsg_len: __kernel_size_t, pub cmsg_level: i32, pub cmsg_type: i32 }
#[repr(C)] pub struct ucred { pub pid: u32, pub uid: u32, pub gid: u32 }

#[inline] pub fn CMSG_ALIGN(len: usize) -> usize { (len + core::mem::size_of::<usize>() - 1) & !(core::mem::size_of::<usize>() - 1) }
#[inline] pub unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut core::ffi::c_void { (cmsg as *mut u8).add(core::mem::size_of::<cmsghdr>()) as *mut _ }
#[inline] pub unsafe fn CMSG_USER_DATA(cmsg: *mut cmsghdr) -> *mut core::ffi::c_void { CMSG_DATA(cmsg) }
#[inline] pub const fn CMSG_SPACE(len: usize) -> usize { core::mem::size_of::<cmsghdr>() + len }
#[inline] pub const fn CMSG_LEN(len: usize) -> usize { core::mem::size_of::<cmsghdr>() + len }
#[inline] pub unsafe fn __cmsg_nxthdr(ctl: *mut core::ffi::c_void, size: usize, cmsg: *mut cmsghdr) -> *mut cmsghdr { let ptr = (cmsg as *mut u8).add(CMSG_ALIGN((*cmsg).cmsg_len as usize)) as *mut cmsghdr; if (ptr.add(1) as usize).wrapping_sub(ctl as usize) > size { core::ptr::null_mut() } else { ptr } }
#[inline] pub unsafe fn cmsg_nxthdr(msg: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr { __cmsg_nxthdr((*msg).control.msg_control, (*msg).msg_controllen as usize, cmsg) }
#[inline] pub unsafe fn msg_data_left(msg: *const msghdr) -> usize { iov_iter_count(&(*msg).msg_iter) }

pub const SCM_RIGHTS: i32 = 0x01; pub const SCM_CREDENTIALS: i32 = 0x02; pub const SCM_SECURITY: i32 = 0x03; pub const SCM_PIDFD: i32 = 0x04;
pub const AF_UNSPEC: i32 = 0; pub const AF_UNIX: i32 = 1; pub const AF_LOCAL: i32 = 1; pub const AF_INET: i32 = 2; pub const AF_AX25: i32 = 3; pub const AF_IPX: i32 = 4; pub const AF_APPLETALK: i32 = 5; pub const AF_NETROM: i32 = 6; pub const AF_BRIDGE: i32 = 7; pub const AF_ATMPVC: i32 = 8; pub const AF_X25: i32 = 9; pub const AF_INET6: i32 = 10; pub const AF_ROSE: i32 = 11; pub const AF_DECnet: i32 = 12; pub const AF_NETBEUI: i32 = 13; pub const AF_SECURITY: i32 = 14; pub const AF_KEY: i32 = 15; pub const AF_NETLINK: i32 = 16; pub const AF_ROUTE: i32 = AF_NETLINK; pub const AF_PACKET: i32 = 17; pub const AF_ASH: i32 = 18; pub const AF_ECONET: i32 = 19; pub const AF_ATMSVC: i32 = 20; pub const AF_RDS: i32 = 21; pub const AF_SNA: i32 = 22; pub const AF_IRDA: i32 = 23; pub const AF_PPPOX: i32 = 24; pub const AF_WANPIPE: i32 = 25; pub const AF_LLC: i32 = 26; pub const AF_IB: i32 = 27; pub const AF_MPLS: i32 = 28; pub const AF_CAN: i32 = 29; pub const AF_TIPC: i32 = 30; pub const AF_BLUETOOTH: i32 = 31; pub const AF_IUCV: i32 = 32; pub const AF_RXRPC: i32 = 33; pub const AF_ISDN: i32 = 34; pub const AF_PHONET: i32 = 35; pub const AF_IEEE802154: i32 = 36; pub const AF_CAIF: i32 = 37; pub const AF_ALG: i32 = 38; pub const AF_NFC: i32 = 39; pub const AF_VSOCK: i32 = 40; pub const AF_KCM: i32 = 41; pub const AF_QIPCRTR: i32 = 42; pub const AF_SMC: i32 = 43; pub const AF_XDP: i32 = 44; pub const AF_MCTP: i32 = 45; pub const AF_MAX: i32 = 46;

pub const SOMAXCONN: i32 = 4096;
pub const MSG_OOB: u32 = 1; pub const MSG_PEEK: u32 = 2; pub const MSG_DONTROUTE: u32 = 4; pub const MSG_TRYHARD: u32 = 4; pub const MSG_CTRUNC: u32 = 8; pub const MSG_PROBE: u32 = 0x10; pub const MSG_TRUNC: u32 = 0x20; pub const MSG_DONTWAIT: u32 = 0x40; pub const MSG_EOR: u32 = 0x80; pub const MSG_WAITALL: u32 = 0x100; pub const MSG_FIN: u32 = 0x200; pub const MSG_SYN: u32 = 0x400; pub const MSG_CONFIRM: u32 = 0x800; pub const MSG_RST: u32 = 0x1000; pub const MSG_ERRQUEUE: u32 = 0x2000; pub const MSG_NOSIGNAL: u32 = 0x4000; pub const MSG_MORE: u32 = 0x8000; pub const MSG_WAITFORONE: u32 = 0x10000; pub const MSG_SENDPAGE_NOPOLICY: u32 = 0x10000; pub const MSG_BATCH: u32 = 0x40000; pub const MSG_EOF: u32 = MSG_FIN; pub const MSG_NO_SHARED_FRAGS: u32 = 0x80000; pub const MSG_SENDPAGE_DECRYPTED: u32 = 0x100000; pub const MSG_SOCK_DEVMEM: u32 = 0x2000000; pub const MSG_ZEROCOPY: u32 = 0x4000000; pub const MSG_SPLICE_PAGES: u32 = 0x8000000; pub const MSG_FASTOPEN: u32 = 0x20000000; pub const MSG_CMSG_CLOEXEC: u32 = 0x40000000; pub const MSG_CMSG_COMPAT: u32 = 0; pub const MSG_INTERNAL_SENDMSG_FLAGS: u32 = MSG_SPLICE_PAGES | MSG_SENDPAGE_NOPOLICY | MSG_SENDPAGE_DECRYPTED | MSG_NO_SHARED_FRAGS;

pub const SOL_IP: i32 = 0; pub const SOL_TCP: i32 = 6; pub const SOL_UDP: i32 = 17; pub const SOL_IPV6: i32 = 41; pub const SOL_ICMPV6: i32 = 58; pub const SOL_SCTP: i32 = 132; pub const SOL_UDPLITE: i32 = 136; pub const SOL_RAW: i32 = 255; pub const SOL_IPX: i32 = 256; pub const SOL_AX25: i32 = 257; pub const SOL_ATALK: i32 = 258; pub const SOL_NETROM: i32 = 259; pub const SOL_ROSE: i32 = 260; pub const SOL_DECNET: i32 = 261; pub const SOL_X25: i32 = 262; pub const SOL_PACKET: i32 = 263; pub const SOL_ATM: i32 = 264; pub const SOL_AAL: i32 = 265; pub const SOL_IRDA: i32 = 266; pub const SOL_NETBEUI: i32 = 267; pub const SOL_LLC: i32 = 268; pub const SOL_DCCP: i32 = 269; pub const SOL_NETLINK: i32 = 270; pub const SOL_TIPC: i32 = 271; pub const SOL_RXRPC: i32 = 272; pub const SOL_PPPOL2TP: i32 = 273; pub const SOL_BLUETOOTH: i32 = 274; pub const SOL_PNPIPE: i32 = 275; pub const SOL_RDS: i32 = 276; pub const SOL_IUCV: i32 = 277; pub const SOL_CAIF: i32 = 278; pub const SOL_ALG: i32 = 279; pub const SOL_NFC: i32 = 280; pub const SOL_KCM: i32 = 281; pub const SOL_TLS: i32 = 282; pub const SOL_XDP: i32 = 283; pub const SOL_MPTCP: i32 = 284; pub const SOL_MCTP: i32 = 285; pub const SOL_SMC: i32 = 286; pub const SOL_VSOCK: i32 = 287; pub const IPX_TYPE: i32 = 1;

#[repr(C)] pub struct scm_timestamping_internal { pub ts: [ktime_t; 3] }
pub struct file; pub struct pid; pub struct cred; pub struct socket; pub struct sock; pub struct sk_buff; pub struct proto_accept_arg; pub struct seq_file; pub struct ubuf_info; pub struct iov_iter; pub struct iovec; pub struct __kernel_timespec; pub struct old_timespec32;
extern "C" { pub fn iov_iter_count(iter: *const iov_iter) -> usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
