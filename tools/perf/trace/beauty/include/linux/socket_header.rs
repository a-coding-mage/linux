/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/socket.h. C include dependencies are expected to be
 * supplied by the surrounding Rust translation unit.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct file {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct pid {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct cred {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct socket {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sock {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sk_buff {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct proto_accept_arg {
    _unused: [u8; 0],
}

/* CONFIG_PROC_FS:
 * struct seq_file;
 * extern void socket_seq_show(struct seq_file *seq);
 */
#[repr(C)]
pub struct seq_file {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn socket_seq_show(seq: *mut seq_file);
}

pub type sa_family_t = __kernel_sa_family_t;

/*
 * 1003.1g requires sa_family_t and that sa_data is char.
 */

/* Deprecated for in-kernel use. Use struct sockaddr_unsized instead. */
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t, /* address family, AF_xxx */
    pub sa_data: [c_char; 14],  /* 14 bytes of protocol address */
}

/**
 * struct sockaddr_unsized - Unspecified size sockaddr for callbacks
 * @sa_family: Address family (AF_UNIX, AF_INET, AF_INET6, etc.)
 * @sa_data: Flexible array for address data
 *
 * This structure is designed for callback interfaces where the
 * total size is known via the sockaddr_len parameter. Unlike struct
 * sockaddr which has a fixed 14-byte sa_data limit or struct
 * sockaddr_storage which has a fixed 128-byte sa_data limit, this
 * structure can accommodate addresses of any size, but must be used
 * carefully.
 */
#[repr(C)]
pub struct sockaddr_unsized {
    pub sa_family: __kernel_sa_family_t, /* address family, AF_xxx */
    pub sa_data: [c_char; 0],            /* flexible address data */
}

#[repr(C)]
pub struct linger {
    pub l_onoff: c_int,  /* Linger active */
    pub l_linger: c_int, /* How long to linger for */
}

pub type sockaddr_storage = __kernel_sockaddr_storage;

/*
 * As we do 4.4BSD message passing we use a 4.4BSD message passing
 * system, not 4.3. Thus msg_accrights(len) are now missing. They
 * belong in an obscure libc emulation or the bin.
 */

#[repr(C)]
pub union msghdr_msg_control_union {
    pub msg_control: *mut c_void,
    pub msg_control_user: *mut c_void,
}

#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,        /* ptr to socket address structure */
    pub msg_namelen: c_int,          /* size of socket address structure */
    pub msg_inq: c_int,              /* output, data left in socket */
    pub msg_iter: iov_iter,          /* data */
    pub msg_control: msghdr_msg_control_union,
    /* C bitfields: bool msg_control_is_user : 1; bool msg_get_inq : 1; */
    pub msg_control_is_user_msg_get_inq: c_uint,
    pub msg_flags: c_uint,                 /* flags on received message */
    pub msg_controllen: __kernel_size_t,   /* ancillary data buffer length */
    pub msg_ubuf: *mut ubuf_info,
    pub sg_from_iter:
        Option<unsafe extern "C" fn(skb: *mut sk_buff, from: *mut iov_iter, length: size_t) -> c_int>,
}

#[repr(C)]
pub struct user_msghdr {
    pub msg_name: *mut c_void,            /* ptr to socket address structure */
    pub msg_namelen: c_int,              /* size of socket address structure */
    pub msg_iov: *mut iovec,             /* scatter/gather array */
    pub msg_iovlen: __kernel_size_t,     /* # elements in msg_iov */
    pub msg_control: *mut c_void,        /* ancillary data */
    pub msg_controllen: __kernel_size_t, /* ancillary data buffer length */
    pub msg_flags: c_uint,               /* flags on received message */
}

/* For recvmmsg/sendmmsg */
#[repr(C)]
pub struct mmsghdr {
    pub msg_hdr: user_msghdr,
    pub msg_len: c_uint,
}

/*
 * POSIX 1003.1g - ancillary data object information
 * Ancillary data consists of a sequence of pairs of
 * (cmsghdr, cmsg_data[])
 */

#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: __kernel_size_t, /* data byte count, including hdr */
    pub cmsg_level: c_int,         /* originating protocol */
    pub cmsg_type: c_int,          /* protocol-specific type */
}

/*
 * Ancillary data object information MACROS
 * Table 5-14 of POSIX 1003.1g
 */

#[inline]
pub const fn CMSG_ALIGN(len: usize) -> usize {
    (len + core::mem::size_of::<c_long>() - 1) & !(core::mem::size_of::<c_long>() - 1)
}

#[inline]
pub unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut c_void {
    unsafe { (cmsg as *mut u8).add(core::mem::size_of::<cmsghdr>()) as *mut c_void }
}

#[inline]
pub unsafe fn CMSG_USER_DATA(cmsg: *mut cmsghdr) -> *mut c_void {
    unsafe { (cmsg as *mut u8).add(core::mem::size_of::<cmsghdr>()) as *mut c_void }
}

#[inline]
pub const fn CMSG_SPACE(len: usize) -> usize {
    core::mem::size_of::<cmsghdr>() + CMSG_ALIGN(len)
}

#[inline]
pub const fn CMSG_LEN(len: usize) -> usize {
    core::mem::size_of::<cmsghdr>() + len
}

#[inline]
pub unsafe fn __CMSG_FIRSTHDR(ctl: *mut c_void, len: __kernel_size_t) -> *mut cmsghdr {
    if (len as usize) >= core::mem::size_of::<cmsghdr>() {
        ctl as *mut cmsghdr
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn CMSG_FIRSTHDR(msg: *mut msghdr) -> *mut cmsghdr {
    unsafe {
        __CMSG_FIRSTHDR(
            (*msg).msg_control.msg_control,
            (*msg).msg_controllen,
        )
    }
}

#[inline]
pub unsafe fn CMSG_OK(mhdr: *mut msghdr, cmsg: *mut cmsghdr) -> bool {
    unsafe {
        ((*cmsg).cmsg_len as usize) >= core::mem::size_of::<cmsghdr>()
            && (*cmsg).cmsg_len as c_ulong
                <= ((*mhdr).msg_controllen
                    - ((cmsg as *mut c_char).offset_from((*mhdr).msg_control.msg_control as *mut c_char)
                        as __kernel_size_t)) as c_ulong
    }
}

/*
 * Get the next cmsg header
 *
 * PLEASE, do not touch this function. If you think, that it is
 * incorrect, grep kernel sources and think about consequences
 * before trying to improve it.
 *
 * Now it always returns valid, not truncated ancillary object
 * HEADER. But caller still MUST check, that cmsg->cmsg_len is
 * inside range, given by msg->msg_controllen before using
 * ancillary object DATA. --ANK (980731)
 */

#[inline]
pub unsafe fn __cmsg_nxthdr(
    __ctl: *mut c_void,
    __size: __kernel_size_t,
    __cmsg: *mut cmsghdr,
) -> *mut cmsghdr {
    let __ptr: *mut cmsghdr =
        unsafe { (__cmsg as *mut u8).add(CMSG_ALIGN((*__cmsg).cmsg_len as usize)) as *mut cmsghdr };
    if unsafe { (__ptr.add(1) as *mut c_char).offset_from(__ctl as *mut c_char) as c_ulong }
        > __size as c_ulong
    {
        return core::ptr::null_mut();
    }

    __ptr
}

#[inline]
pub unsafe fn __CMSG_NXTHDR(
    ctl: *mut c_void,
    len: __kernel_size_t,
    cmsg: *mut cmsghdr,
) -> *mut cmsghdr {
    unsafe { __cmsg_nxthdr(ctl, len, cmsg) }
}

#[inline]
pub unsafe fn cmsg_nxthdr(__msg: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr {
    unsafe { __cmsg_nxthdr((*__msg).msg_control.msg_control, (*__msg).msg_controllen, __cmsg) }
}

#[inline]
pub unsafe fn CMSG_NXTHDR(mhdr: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr {
    unsafe { cmsg_nxthdr(mhdr, cmsg) }
}

#[inline]
pub unsafe fn msg_data_left(msg: *const msghdr) -> size_t {
    unsafe { iov_iter_count(&(*msg).msg_iter as *const iov_iter) }
}

/* "Socket"-level control message types: */

pub const SCM_RIGHTS: c_int = 0x01; /* rw: access rights (array of int) */
pub const SCM_CREDENTIALS: c_int = 0x02; /* rw: struct ucred */
pub const SCM_SECURITY: c_int = 0x03; /* rw: security label */
pub const SCM_PIDFD: c_int = 0x04; /* ro: pidfd (int) */

#[repr(C)]
pub struct ucred {
    pub pid: __u32,
    pub uid: __u32,
    pub gid: __u32,
}

/* Supported address families. */
pub const AF_UNSPEC: c_int = 0;
pub const AF_UNIX: c_int = 1; /* Unix domain sockets */
pub const AF_LOCAL: c_int = 1; /* POSIX name for AF_UNIX */
pub const AF_INET: c_int = 2; /* Internet IP Protocol */
pub const AF_AX25: c_int = 3; /* Amateur Radio AX.25 */
pub const AF_IPX: c_int = 4; /* Novell IPX */
pub const AF_APPLETALK: c_int = 5; /* AppleTalk DDP */
pub const AF_NETROM: c_int = 6; /* Amateur Radio NET/ROM */
pub const AF_BRIDGE: c_int = 7; /* Multiprotocol bridge */
pub const AF_ATMPVC: c_int = 8; /* ATM PVCs */
pub const AF_X25: c_int = 9; /* Reserved for X.25 project */
pub const AF_INET6: c_int = 10; /* IP version 6 */
pub const AF_ROSE: c_int = 11; /* Amateur Radio X.25 PLP */
pub const AF_DECnet: c_int = 12; /* Reserved for DECnet project */
pub const AF_NETBEUI: c_int = 13; /* Reserved for 802.2LLC project */
pub const AF_SECURITY: c_int = 14; /* Security callback pseudo AF */
pub const AF_KEY: c_int = 15; /* PF_KEY key management API */
pub const AF_NETLINK: c_int = 16;
pub const AF_ROUTE: c_int = AF_NETLINK; /* Alias to emulate 4.4BSD */
pub const AF_PACKET: c_int = 17; /* Packet family */
pub const AF_ASH: c_int = 18; /* Ash */
pub const AF_ECONET: c_int = 19; /* Acorn Econet */
pub const AF_ATMSVC: c_int = 20; /* ATM SVCs */
pub const AF_RDS: c_int = 21; /* RDS sockets */
pub const AF_SNA: c_int = 22; /* Linux SNA Project (nutters!) */
pub const AF_IRDA: c_int = 23; /* IRDA sockets */
pub const AF_PPPOX: c_int = 24; /* PPPoX sockets */
pub const AF_WANPIPE: c_int = 25; /* Wanpipe API Sockets */
pub const AF_LLC: c_int = 26; /* Linux LLC */
pub const AF_IB: c_int = 27; /* Native InfiniBand address */
pub const AF_MPLS: c_int = 28; /* MPLS */
pub const AF_CAN: c_int = 29; /* Controller Area Network */
pub const AF_TIPC: c_int = 30; /* TIPC sockets */
pub const AF_BLUETOOTH: c_int = 31; /* Bluetooth sockets */
pub const AF_IUCV: c_int = 32; /* IUCV sockets */
pub const AF_RXRPC: c_int = 33; /* RxRPC sockets */
pub const AF_ISDN: c_int = 34; /* mISDN sockets */
pub const AF_PHONET: c_int = 35; /* Phonet sockets */
pub const AF_IEEE802154: c_int = 36; /* IEEE802154 sockets */
pub const AF_CAIF: c_int = 37; /* CAIF sockets */
pub const AF_ALG: c_int = 38; /* Algorithm sockets */
pub const AF_NFC: c_int = 39; /* NFC sockets */
pub const AF_VSOCK: c_int = 40; /* vSockets */
pub const AF_KCM: c_int = 41; /* Kernel Connection Multiplexor */
pub const AF_QIPCRTR: c_int = 42; /* Qualcomm IPC Router */
pub const AF_SMC: c_int = 43; /* smc sockets: reserve number for
                               * PF_SMC protocol family that
                               * reuses AF_INET address family
                               */
pub const AF_XDP: c_int = 44; /* XDP sockets */
pub const AF_MCTP: c_int = 45; /* Management component
                                * transport protocol
                                */

pub const AF_MAX: c_int = 46; /* For now.. */

/* Protocol families, same as address families. */
pub const PF_UNSPEC: c_int = AF_UNSPEC;
pub const PF_UNIX: c_int = AF_UNIX;
pub const PF_LOCAL: c_int = AF_LOCAL;
pub const PF_INET: c_int = AF_INET;
pub const PF_AX25: c_int = AF_AX25;
pub const PF_IPX: c_int = AF_IPX;
pub const PF_APPLETALK: c_int = AF_APPLETALK;
pub const PF_NETROM: c_int = AF_NETROM;
pub const PF_BRIDGE: c_int = AF_BRIDGE;
pub const PF_ATMPVC: c_int = AF_ATMPVC;
pub const PF_X25: c_int = AF_X25;
pub const PF_INET6: c_int = AF_INET6;
pub const PF_ROSE: c_int = AF_ROSE;
pub const PF_DECnet: c_int = AF_DECnet;
pub const PF_NETBEUI: c_int = AF_NETBEUI;
pub const PF_SECURITY: c_int = AF_SECURITY;
pub const PF_KEY: c_int = AF_KEY;
pub const PF_NETLINK: c_int = AF_NETLINK;
pub const PF_ROUTE: c_int = AF_ROUTE;
pub const PF_PACKET: c_int = AF_PACKET;
pub const PF_ASH: c_int = AF_ASH;
pub const PF_ECONET: c_int = AF_ECONET;
pub const PF_ATMSVC: c_int = AF_ATMSVC;
pub const PF_RDS: c_int = AF_RDS;
pub const PF_SNA: c_int = AF_SNA;
pub const PF_IRDA: c_int = AF_IRDA;
pub const PF_PPPOX: c_int = AF_PPPOX;
pub const PF_WANPIPE: c_int = AF_WANPIPE;
pub const PF_LLC: c_int = AF_LLC;
pub const PF_IB: c_int = AF_IB;
pub const PF_MPLS: c_int = AF_MPLS;
pub const PF_CAN: c_int = AF_CAN;
pub const PF_TIPC: c_int = AF_TIPC;
pub const PF_BLUETOOTH: c_int = AF_BLUETOOTH;
pub const PF_IUCV: c_int = AF_IUCV;
pub const PF_RXRPC: c_int = AF_RXRPC;
pub const PF_ISDN: c_int = AF_ISDN;
pub const PF_PHONET: c_int = AF_PHONET;
pub const PF_IEEE802154: c_int = AF_IEEE802154;
pub const PF_CAIF: c_int = AF_CAIF;
pub const PF_ALG: c_int = AF_ALG;
pub const PF_NFC: c_int = AF_NFC;
pub const PF_VSOCK: c_int = AF_VSOCK;
pub const PF_KCM: c_int = AF_KCM;
pub const PF_QIPCRTR: c_int = AF_QIPCRTR;
pub const PF_SMC: c_int = AF_SMC;
pub const PF_XDP: c_int = AF_XDP;
pub const PF_MCTP: c_int = AF_MCTP;
pub const PF_MAX: c_int = AF_MAX;

/* Maximum queue length specifiable by listen. */
pub const SOMAXCONN: c_int = 4096;

/* Flags we can use with send/ and recv.
 * Added those for 1003.1g not all are supported yet
 */

pub const MSG_OOB: c_uint = 1;
pub const MSG_PEEK: c_uint = 2;
pub const MSG_DONTROUTE: c_uint = 4;
pub const MSG_TRYHARD: c_uint = 4; /* Synonym for MSG_DONTROUTE for DECnet */
pub const MSG_CTRUNC: c_uint = 8;
pub const MSG_PROBE: c_uint = 0x10; /* Do not send. Only probe path f.e. for MTU */
pub const MSG_TRUNC: c_uint = 0x20;
pub const MSG_DONTWAIT: c_uint = 0x40; /* Nonblocking io */
pub const MSG_EOR: c_uint = 0x80; /* End of record */
pub const MSG_WAITALL: c_uint = 0x100; /* Wait for a full request */
pub const MSG_FIN: c_uint = 0x200;
pub const MSG_SYN: c_uint = 0x400;
pub const MSG_CONFIRM: c_uint = 0x800; /* Confirm path validity */
pub const MSG_RST: c_uint = 0x1000;
pub const MSG_ERRQUEUE: c_uint = 0x2000; /* Fetch message from error queue */
pub const MSG_NOSIGNAL: c_uint = 0x4000; /* Do not generate SIGPIPE */
pub const MSG_MORE: c_uint = 0x8000; /* Sender will send more */
pub const MSG_WAITFORONE: c_uint = 0x10000; /* recvmmsg(): block until 1+ packets avail */
pub const MSG_SENDPAGE_NOPOLICY: c_uint = 0x10000; /* sendpage() internal : do no apply policy */
pub const MSG_BATCH: c_uint = 0x40000; /* sendmmsg(): more messages coming */
pub const MSG_EOF: c_uint = MSG_FIN;
pub const MSG_NO_SHARED_FRAGS: c_uint = 0x80000; /* sendpage() internal : page frags are not shared */
pub const MSG_SENDPAGE_DECRYPTED: c_uint = 0x100000; /* sendpage() internal : page may carry
                                                      * plain text and require encryption
                                                      */

pub const MSG_SOCK_DEVMEM: c_uint = 0x2000000; /* Receive devmem skbs as cmsg */
pub const MSG_ZEROCOPY: c_uint = 0x4000000; /* Use user data in kernel path */
pub const MSG_SPLICE_PAGES: c_uint = 0x8000000; /* Splice the pages from the iterator in sendmsg() */
pub const MSG_FASTOPEN: c_uint = 0x20000000; /* Send data in TCP SYN */
pub const MSG_CMSG_CLOEXEC: c_uint = 0x40000000; /* Set close_on_exec for file
                                                 * descriptor received through
                                                 * SCM_RIGHTS
                                                 */
/* CONFIG_COMPAT selects 0x80000000; otherwise Linux defines this as 0. */
pub const MSG_CMSG_COMPAT: c_uint = 0;

/* Flags to be cleared on entry by sendmsg and sendmmsg syscalls */
pub const MSG_INTERNAL_SENDMSG_FLAGS: c_uint =
    MSG_SPLICE_PAGES | MSG_SENDPAGE_NOPOLICY | MSG_SENDPAGE_DECRYPTED | MSG_NO_SHARED_FRAGS;

/* Setsockoptions(2) level. Thanks to BSD these must match IPPROTO_xxx */
pub const SOL_IP: c_int = 0;
/* SOL_ICMP intentionally not defined: Due to Linux :-) we cannot use SOL_ICMP=1 */
pub const SOL_TCP: c_int = 6;
pub const SOL_UDP: c_int = 17;
pub const SOL_IPV6: c_int = 41;
pub const SOL_ICMPV6: c_int = 58;
pub const SOL_SCTP: c_int = 132;
pub const SOL_UDPLITE: c_int = 136; /* UDP-Lite (RFC 3828) */
pub const SOL_RAW: c_int = 255;
pub const SOL_IPX: c_int = 256;
pub const SOL_AX25: c_int = 257;
pub const SOL_ATALK: c_int = 258;
pub const SOL_NETROM: c_int = 259;
pub const SOL_ROSE: c_int = 260;
pub const SOL_DECNET: c_int = 261;
pub const SOL_X25: c_int = 262;
pub const SOL_PACKET: c_int = 263;
pub const SOL_ATM: c_int = 264; /* ATM layer (cell level) */
pub const SOL_AAL: c_int = 265; /* ATM Adaption Layer (packet level) */
pub const SOL_IRDA: c_int = 266;
pub const SOL_NETBEUI: c_int = 267;
pub const SOL_LLC: c_int = 268;
pub const SOL_DCCP: c_int = 269;
pub const SOL_NETLINK: c_int = 270;
pub const SOL_TIPC: c_int = 271;
pub const SOL_RXRPC: c_int = 272;
pub const SOL_PPPOL2TP: c_int = 273;
pub const SOL_BLUETOOTH: c_int = 274;
pub const SOL_PNPIPE: c_int = 275;
pub const SOL_RDS: c_int = 276;
pub const SOL_IUCV: c_int = 277;
pub const SOL_CAIF: c_int = 278;
pub const SOL_ALG: c_int = 279;
pub const SOL_NFC: c_int = 280;
pub const SOL_KCM: c_int = 281;
pub const SOL_TLS: c_int = 282;
pub const SOL_XDP: c_int = 283;
pub const SOL_MPTCP: c_int = 284;
pub const SOL_MCTP: c_int = 285;
pub const SOL_SMC: c_int = 286;
pub const SOL_VSOCK: c_int = 287;

/* IPX options */
pub const IPX_TYPE: c_int = 1;

#[repr(C)]
pub struct timespec64 {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct __kernel_timespec {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct old_timespec32 {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct scm_timestamping_internal {
    pub ts: [ktime_t; 3],
}

unsafe extern "C" {
    pub fn move_addr_to_kernel(
        uaddr: *mut c_void,
        ulen: c_int,
        kaddr: *mut sockaddr_storage,
    ) -> c_int;
    pub fn put_cmsg(
        msg: *mut msghdr,
        level: c_int,
        type_: c_int,
        len: c_int,
        data: *mut c_void,
    ) -> c_int;
    pub fn put_cmsg_notrunc(
        msg: *mut msghdr,
        level: c_int,
        type_: c_int,
        len: c_int,
        data: *mut c_void,
    ) -> c_int;
    pub fn put_cmsg_scm_timestamping64(
        msg: *mut msghdr,
        tss: *mut scm_timestamping_internal,
    );
    pub fn put_cmsg_scm_timestamping(
        msg: *mut msghdr,
        tss: *mut scm_timestamping_internal,
    );

    /* The __sys_...msg variants allow MSG_CMSG_COMPAT iff
     * forbid_cmsg_compat==false
     */
    pub fn __sys_recvmsg(
        fd: c_int,
        msg: *mut user_msghdr,
        flags: c_uint,
        forbid_cmsg_compat: bool,
    ) -> c_long;
    pub fn __sys_sendmsg(
        fd: c_int,
        msg: *mut user_msghdr,
        flags: c_uint,
        forbid_cmsg_compat: bool,
    ) -> c_long;
    pub fn __sys_recvmmsg(
        fd: c_int,
        mmsg: *mut mmsghdr,
        vlen: c_uint,
        flags: c_uint,
        timeout: *mut __kernel_timespec,
        timeout32: *mut old_timespec32,
    ) -> c_int;
    pub fn __sys_sendmmsg(
        fd: c_int,
        mmsg: *mut mmsghdr,
        vlen: c_uint,
        flags: c_uint,
        forbid_cmsg_compat: bool,
    ) -> c_int;
    pub fn __sys_sendmsg_sock(sock: *mut socket, msg: *mut msghdr, flags: c_uint) -> c_long;
    pub fn __sys_recvmsg_sock(
        sock: *mut socket,
        msg: *mut msghdr,
        umsg: *mut user_msghdr,
        uaddr: *mut sockaddr,
        flags: c_uint,
    ) -> c_long;
    pub fn __copy_msghdr(
        kmsg: *mut msghdr,
        umsg: *mut user_msghdr,
        save_addr: *mut *mut sockaddr,
    ) -> c_int;

    /* helpers which do the actual work for syscalls */
    pub fn __sys_recvfrom(
        fd: c_int,
        ubuf: *mut c_void,
        size: size_t,
        flags: c_uint,
        addr: *mut sockaddr,
        addr_len: *mut c_int,
    ) -> c_int;
    pub fn __sys_sendto(
        fd: c_int,
        buff: *mut c_void,
        len: size_t,
        flags: c_uint,
        addr: *mut sockaddr,
        addr_len: c_int,
    ) -> c_int;
    pub fn do_accept(
        file: *mut file,
        arg: *mut proto_accept_arg,
        upeer_sockaddr: *mut sockaddr,
        upeer_addrlen: *mut c_int,
        flags: c_int,
    ) -> *mut file;
    pub fn __sys_accept4(
        fd: c_int,
        upeer_sockaddr: *mut sockaddr,
        upeer_addrlen: *mut c_int,
        flags: c_int,
    ) -> c_int;
    pub fn __sys_socket(family: c_int, type_: c_int, protocol: c_int) -> c_int;
    pub fn __sys_socket_file(family: c_int, type_: c_int, protocol: c_int) -> *mut file;
    pub fn __sys_bind(fd: c_int, umyaddr: *mut sockaddr, addrlen: c_int) -> c_int;
    pub fn __sys_bind_socket(
        sock: *mut socket,
        address: *mut sockaddr_storage,
        addrlen: c_int,
    ) -> c_int;
    pub fn __sys_connect_file(
        file: *mut file,
        addr: *mut sockaddr_storage,
        addrlen: c_int,
        file_flags: c_int,
    ) -> c_int;
    pub fn __sys_connect(fd: c_int, uservaddr: *mut sockaddr, addrlen: c_int) -> c_int;
    pub fn __sys_listen(fd: c_int, backlog: c_int) -> c_int;
    pub fn __sys_listen_socket(sock: *mut socket, backlog: c_int) -> c_int;
    pub fn do_getsockname(
        sock: *mut socket,
        peer: c_int,
        usockaddr: *mut sockaddr,
        usockaddr_len: *mut c_int,
    ) -> c_int;
    pub fn __sys_getsockname(
        fd: c_int,
        usockaddr: *mut sockaddr,
        usockaddr_len: *mut c_int,
        peer: c_int,
    ) -> c_int;
    pub fn __sys_socketpair(
        family: c_int,
        type_: c_int,
        protocol: c_int,
        usockvec: *mut c_int,
    ) -> c_int;
    pub fn __sys_shutdown_sock(sock: *mut socket, how: c_int) -> c_int;
    pub fn __sys_shutdown(fd: c_int, how: c_int) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
