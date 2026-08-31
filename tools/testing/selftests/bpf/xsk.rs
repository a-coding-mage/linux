// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * AF_XDP user-space access library.
 *
 * Copyright(c) 2018 - 2019 Intel Corporation.
 *
 * Author(s): Magnus Karlsson <magnus.karlsson@intel.com>
 */

// C dependencies translated as external symbols/types expected from surrounding bindings:
// errno, stdlib, string, unistd, arpa/inet, asm/barrier, linux/compiler,
// linux/ethtool, linux/filter, linux/if_ether, linux/if_link,
// linux/if_packet, linux/if_xdp, linux/kernel, linux/list, linux/netlink,
// linux/rtnetlink, linux/sockios, net/if, sys/ioctl, sys/mman, sys/socket,
// sys/types, bpf/bpf, bpf/libbpf, xsk.h, bpf_util.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

// #ifndef SOL_XDP
pub const SOL_XDP: c_int = 283;
// #endif

// #ifndef AF_XDP
pub const AF_XDP: c_int = 44;
// #endif

// #ifndef PF_XDP
pub const PF_XDP: c_int = AF_XDP;
// #endif

pub const XSKMAP_SIZE: c_int = 1;

pub type u32 = __u32;
pub type __u32 = u32;
pub type __u64 = u64;
pub type uintptr_t = usize;
pub type size_t = usize;
pub type socklen_t = u32;
pub type ssize_t = isize;

extern "C" {
    static mut errno: c_int;

    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    fn getpagesize() -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;

    fn bpf_xdp_query(ifindex: c_int, flags: c_int, opts: *mut bpf_xdp_query_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_xdp_attach(
        ifindex: c_int,
        prog_fd: c_int,
        xdp_flags: __u32,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, xdp_flags: __u32, opts: *const c_void) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
}

extern "C" {
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct xsk_ring_prod {
    pub cached_prod: __u32,
    pub cached_cons: __u32,
    pub mask: __u32,
    pub size: __u32,
    pub producer: *mut __u32,
    pub consumer: *mut __u32,
    pub flags: *mut __u32,
    pub ring: *mut c_void,
}

#[repr(C)]
pub struct xsk_ring_cons {
    pub cached_prod: __u32,
    pub cached_cons: __u32,
    pub mask: __u32,
    pub size: __u32,
    pub producer: *mut __u32,
    pub consumer: *mut __u32,
    pub flags: *mut __u32,
    pub ring: *mut c_void,
}

#[repr(C)]
pub struct xsk_umem_config {
    pub fill_size: __u32,
    pub comp_size: __u32,
    pub frame_size: __u32,
    pub frame_headroom: __u32,
    pub flags: __u32,
    pub tx_metadata_len: __u32,
}

#[repr(C)]
pub struct xsk_socket_config {
    pub rx_size: __u32,
    pub tx_size: __u32,
    pub bind_flags: __u32,
}

#[repr(C)]
pub struct xsk_umem {
    pub fill_save: *mut xsk_ring_prod,
    pub comp_save: *mut xsk_ring_cons,
    pub umem_area: *mut c_char,
    pub config: xsk_umem_config,
    pub fd: c_int,
    pub refcount: c_int,
    pub ctx_list: list_head,
    pub rx_ring_setup_done: bool,
    pub tx_ring_setup_done: bool,
}

#[repr(C)]
pub struct xsk_ctx {
    pub fill: *mut xsk_ring_prod,
    pub comp: *mut xsk_ring_cons,
    pub queue_id: __u32,
    pub umem: *mut xsk_umem,
    pub refcount: c_int,
    pub ifindex: c_int,
    pub list: list_head,
}

#[repr(C)]
pub struct xsk_socket {
    pub rx: *mut xsk_ring_cons,
    pub tx: *mut xsk_ring_prod,
    pub ctx: *mut xsk_ctx,
    pub config: xsk_socket_config,
    pub fd: c_int,
}

#[repr(C)]
pub struct nl_mtu_req {
    pub nh: nlmsghdr,
    pub msg: ifinfomsg,
    pub buf: [c_char; 512],
}

#[repr(C)]
pub struct xdp_ring_offset {
    pub producer: __u64,
    pub consumer: __u64,
    pub desc: __u64,
    pub flags: __u64,
}

#[repr(C)]
pub struct xdp_mmap_offsets {
    pub rx: xdp_ring_offset,
    pub tx: xdp_ring_offset,
    pub fr: xdp_ring_offset,
    pub cr: xdp_ring_offset,
}

#[repr(C)]
pub struct xdp_umem_reg {
    pub addr: __u64,
    pub len: __u64,
    pub chunk_size: __u32,
    pub headroom: __u32,
    pub flags: __u32,
    pub tx_metadata_len: __u32,
}

#[repr(C)]
pub struct xdp_desc {
    pub addr: __u64,
    pub len: __u32,
    pub options: __u32,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_xdp {
    pub sxdp_family: u16,
    pub sxdp_flags: __u16,
    pub sxdp_ifindex: __u32,
    pub sxdp_queue_id: __u32,
    pub sxdp_shared_umem_fd: __u32,
}

pub type __u16 = u16;

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut c_void,
    pub msg_controllen: size_t,
    pub msg_flags: c_int,
}

#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: __u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: __u32,
    pub nlmsg_pid: __u32,
}

#[repr(C)]
pub struct nlmsgerr {
    pub error: c_int,
    pub msg: nlmsghdr,
}

#[repr(C)]
pub struct ifinfomsg {
    pub ifi_family: u8,
    pub __ifi_pad: u8,
    pub ifi_type: u16,
    pub ifi_index: c_int,
    pub ifi_flags: c_uint,
    pub ifi_change: c_uint,
}

#[repr(C)]
pub struct rtattr {
    pub rta_len: u16,
    pub rta_type: u16,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_xdp_query_opts {
    pub sz: size_t,
    pub prog_id: __u32,
    pub drv_prog_id: __u32,
    pub hw_prog_id: __u32,
    pub skb_prog_id: __u32,
    pub attach_mode: c_int,
}

pub const EFAULT: c_int = 14;
pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const EINTR: c_int = 4;
pub const EAGAIN: c_int = 11;
pub const EBUSY: c_int = 16;

pub const SOCK_RAW: c_int = 3;
pub const SOCK_CLOEXEC: c_int = 0o2000000;
pub const SOCK_DGRAM: c_int = 2;
pub const AF_NETLINK: c_int = 16;
pub const NETLINK_ROUTE: c_int = 0;
pub const AF_UNSPEC: u8 = 0;

pub const PROT_READ: c_int = 0x1;
pub const PROT_WRITE: c_int = 0x2;
pub const MAP_SHARED: c_int = 0x01;
pub const MAP_POPULATE: c_int = 0x8000;
pub const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

pub const XDP_MMAP_OFFSETS: c_int = 1;
pub const XDP_UMEM_REG: c_int = 4;
pub const XDP_UMEM_FILL_RING: c_int = 5;
pub const XDP_UMEM_COMPLETION_RING: c_int = 6;
pub const XDP_RX_RING: c_int = 2;
pub const XDP_TX_RING: c_int = 3;
pub const XDP_UMEM_PGOFF_FILL_RING: isize = 0x100000000;
pub const XDP_UMEM_PGOFF_COMPLETION_RING: isize = 0x180000000;
pub const XDP_PGOFF_RX_RING: isize = 0;
pub const XDP_PGOFF_TX_RING: isize = 0x80000000;
pub const XDP_SHARED_UMEM: __u16 = 1 << 0;

pub const XDP_FLAGS_DRV_MODE: c_int = 1 << 2;
pub const XDP_FLAGS_SKB_MODE: c_int = 1 << 1;
pub const XDP_ATTACHED_DRV: c_int = 2;
pub const XDP_ATTACHED_SKB: c_int = 3;

pub const XSK_RING_PROD__DEFAULT_NUM_DESCS: __u32 = 2048;
pub const XSK_RING_CONS__DEFAULT_NUM_DESCS: __u32 = 2048;
pub const XSK_UMEM__DEFAULT_FRAME_SIZE: __u32 = 4096;
pub const XSK_UMEM__DEFAULT_FRAME_HEADROOM: __u32 = 0;
pub const XSK_UMEM__DEFAULT_FLAGS: __u32 = 0;

pub const MSG_PEEK: c_int = 0x02;
pub const MSG_TRUNC: c_int = 0x20;
pub const NLM_F_REQUEST: u16 = 0x01;
pub const NLM_F_MULTI: u16 = 0x02;
pub const NLM_F_ACK: u16 = 0x04;
pub const NLMSG_ERROR: u16 = 0x2;
pub const NLMSG_DONE: u16 = 0x3;
pub const RTM_NEWLINK: u16 = 16;
pub const IFLA_MTU: u16 = 4;

unsafe fn NLMSG_ALIGN(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

unsafe fn NLMSG_LENGTH(len: usize) -> __u32 {
    (len + NLMSG_ALIGN(size_of::<nlmsghdr>())) as __u32
}

unsafe fn NLMSG_DATA(nlh: *mut nlmsghdr) -> *mut c_void {
    (nlh as *mut u8).add(NLMSG_LENGTH(0) as usize) as *mut c_void
}

unsafe fn NLMSG_OK(nlh: *mut nlmsghdr, len: c_int) -> bool {
    len >= size_of::<nlmsghdr>() as c_int
        && (*nlh).nlmsg_len >= size_of::<nlmsghdr>() as __u32
        && ((*nlh).nlmsg_len as c_int) <= len
}

unsafe fn NLMSG_NEXT(nlh: *mut nlmsghdr, len: &mut c_int) -> *mut nlmsghdr {
    let aligned = NLMSG_ALIGN((*nlh).nlmsg_len as usize) as c_int;
    *len -= aligned;
    (nlh as *mut u8).add(aligned as usize) as *mut nlmsghdr
}

unsafe fn RTA_ALIGN(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

unsafe fn RTA_LENGTH(len: usize) -> u16 {
    (RTA_ALIGN(size_of::<rtattr>()) + len) as u16
}

unsafe fn RTA_DATA(rta: *mut rtattr) -> *mut c_void {
    (rta as *mut u8).add(RTA_LENGTH(0) as usize) as *mut c_void
}

unsafe fn pr_warn(fmt: *const c_char, args: ...) -> c_int {
    fprintf(stderr, fmt, args)
}

#[no_mangle]
pub unsafe extern "C" fn xsk_umem__fd(umem: *const xsk_umem) -> c_int {
    if !umem.is_null() {
        (*umem).fd
    } else {
        -EINVAL
    }
}

#[no_mangle]
pub unsafe extern "C" fn xsk_socket__fd(xsk: *const xsk_socket) -> c_int {
    if !xsk.is_null() {
        (*xsk).fd
    } else {
        -EINVAL
    }
}

unsafe fn xsk_page_aligned(buffer: *mut c_void) -> bool {
    let addr = buffer as usize;

    !(addr & (getpagesize() as usize - 1)) != 0
}

unsafe fn xsk_set_umem_config(cfg: *mut xsk_umem_config, usr_cfg: *const xsk_umem_config) {
    if usr_cfg.is_null() {
        (*cfg).fill_size = XSK_RING_PROD__DEFAULT_NUM_DESCS;
        (*cfg).comp_size = XSK_RING_CONS__DEFAULT_NUM_DESCS;
        (*cfg).frame_size = XSK_UMEM__DEFAULT_FRAME_SIZE;
        (*cfg).frame_headroom = XSK_UMEM__DEFAULT_FRAME_HEADROOM;
        (*cfg).flags = XSK_UMEM__DEFAULT_FLAGS;
        (*cfg).tx_metadata_len = 0;
        return;
    }

    (*cfg).fill_size = (*usr_cfg).fill_size;
    (*cfg).comp_size = (*usr_cfg).comp_size;
    (*cfg).frame_size = (*usr_cfg).frame_size;
    (*cfg).frame_headroom = (*usr_cfg).frame_headroom;
    (*cfg).flags = (*usr_cfg).flags;
    (*cfg).tx_metadata_len = (*usr_cfg).tx_metadata_len;
}

unsafe fn xsk_set_xdp_socket_config(
    cfg: *mut xsk_socket_config,
    usr_cfg: *const xsk_socket_config,
) -> c_int {
    if usr_cfg.is_null() {
        (*cfg).rx_size = XSK_RING_CONS__DEFAULT_NUM_DESCS;
        (*cfg).tx_size = XSK_RING_PROD__DEFAULT_NUM_DESCS;
        (*cfg).bind_flags = 0;
        return 0;
    }

    (*cfg).rx_size = (*usr_cfg).rx_size;
    (*cfg).tx_size = (*usr_cfg).tx_size;
    (*cfg).bind_flags = (*usr_cfg).bind_flags;

    0
}

unsafe fn xsk_get_mmap_offsets(fd: c_int, off: *mut xdp_mmap_offsets) -> c_int {
    let mut optlen: socklen_t;
    let err: c_int;

    optlen = size_of::<xdp_mmap_offsets>() as socklen_t;
    err = getsockopt(
        fd,
        SOL_XDP,
        XDP_MMAP_OFFSETS,
        off as *mut c_void,
        &mut optlen,
    );
    if err != 0 {
        return err;
    }

    if optlen as usize == size_of::<xdp_mmap_offsets>() {
        return 0;
    }

    -EINVAL
}

unsafe fn xsk_create_umem_rings(
    umem: *mut xsk_umem,
    fd: c_int,
    fill: *mut xsk_ring_prod,
    comp: *mut xsk_ring_cons,
) -> c_int {
    let mut off: xdp_mmap_offsets = core::mem::zeroed();
    let mut map: *mut c_void;
    let mut err: c_int;

    err = setsockopt(
        fd,
        SOL_XDP,
        XDP_UMEM_FILL_RING,
        &(*umem).config.fill_size as *const _ as *const c_void,
        size_of::<__u32>() as socklen_t,
    );
    if err != 0 {
        return -errno;
    }

    err = setsockopt(
        fd,
        SOL_XDP,
        XDP_UMEM_COMPLETION_RING,
        &(*umem).config.comp_size as *const _ as *const c_void,
        size_of::<__u32>() as socklen_t,
    );
    if err != 0 {
        return -errno;
    }

    err = xsk_get_mmap_offsets(fd, &mut off);
    if err != 0 {
        return -errno;
    }

    map = mmap(
        ptr::null_mut(),
        off.fr.desc as usize + (*umem).config.fill_size as usize * size_of::<__u64>(),
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_POPULATE,
        fd,
        XDP_UMEM_PGOFF_FILL_RING,
    );
    if map == MAP_FAILED {
        return -errno;
    }

    (*fill).mask = (*umem).config.fill_size - 1;
    (*fill).size = (*umem).config.fill_size;
    (*fill).producer = (map as *mut u8).add(off.fr.producer as usize) as *mut __u32;
    (*fill).consumer = (map as *mut u8).add(off.fr.consumer as usize) as *mut __u32;
    (*fill).flags = (map as *mut u8).add(off.fr.flags as usize) as *mut __u32;
    (*fill).ring = (map as *mut u8).add(off.fr.desc as usize) as *mut c_void;
    (*fill).cached_cons = (*umem).config.fill_size;

    map = mmap(
        ptr::null_mut(),
        off.cr.desc as usize + (*umem).config.comp_size as usize * size_of::<__u64>(),
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_POPULATE,
        fd,
        XDP_UMEM_PGOFF_COMPLETION_RING,
    );
    if map == MAP_FAILED {
        err = -errno;
        munmap(
            ((*fill).ring as *mut u8).sub(off.fr.desc as usize) as *mut c_void,
            off.fr.desc as usize + (*umem).config.fill_size as usize * size_of::<__u64>(),
        );
        return err;
    }

    (*comp).mask = (*umem).config.comp_size - 1;
    (*comp).size = (*umem).config.comp_size;
    (*comp).producer = (map as *mut u8).add(off.cr.producer as usize) as *mut __u32;
    (*comp).consumer = (map as *mut u8).add(off.cr.consumer as usize) as *mut __u32;
    (*comp).flags = (map as *mut u8).add(off.cr.flags as usize) as *mut __u32;
    (*comp).ring = (map as *mut u8).add(off.cr.desc as usize) as *mut c_void;

    0
}

#[no_mangle]
pub unsafe extern "C" fn xsk_umem__create(
    umem_ptr: *mut *mut xsk_umem,
    umem_area: *mut c_void,
    size: __u64,
    fill: *mut xsk_ring_prod,
    comp: *mut xsk_ring_cons,
    usr_config: *const xsk_umem_config,
) -> c_int {
    let mut mr: xdp_umem_reg = core::mem::zeroed();
    let mut umem: *mut xsk_umem;
    let mut err: c_int;

    if umem_area.is_null() || umem_ptr.is_null() || fill.is_null() || comp.is_null() {
        return -EFAULT;
    }
    if size == 0 && !xsk_page_aligned(umem_area) {
        return -EINVAL;
    }

    umem = calloc(1, size_of::<xsk_umem>()) as *mut xsk_umem;
    if umem.is_null() {
        return -ENOMEM;
    }

    (*umem).fd = socket(AF_XDP, SOCK_RAW | SOCK_CLOEXEC, 0);
    if (*umem).fd < 0 {
        err = -errno;
        free(umem as *mut c_void);
        return err;
    }

    (*umem).umem_area = umem_area as *mut c_char;
    INIT_LIST_HEAD(&mut (*umem).ctx_list);
    xsk_set_umem_config(&mut (*umem).config, usr_config);

    memset(&mut mr as *mut _ as *mut c_void, 0, size_of::<xdp_umem_reg>());
    mr.addr = umem_area as uintptr_t as __u64;
    mr.len = size;
    mr.chunk_size = (*umem).config.frame_size;
    mr.headroom = (*umem).config.frame_headroom;
    mr.flags = (*umem).config.flags;
    mr.tx_metadata_len = (*umem).config.tx_metadata_len;

    err = setsockopt(
        (*umem).fd,
        SOL_XDP,
        XDP_UMEM_REG,
        &mr as *const _ as *const c_void,
        size_of::<xdp_umem_reg>() as socklen_t,
    );
    if err != 0 {
        err = -errno;
        close((*umem).fd);
        free(umem as *mut c_void);
        return err;
    }

    err = xsk_create_umem_rings(umem, (*umem).fd, fill, comp);
    if err != 0 {
        close((*umem).fd);
        free(umem as *mut c_void);
        return err;
    }

    (*umem).fill_save = fill;
    (*umem).comp_save = comp;
    *umem_ptr = umem;
    0
}

#[no_mangle]
pub unsafe extern "C" fn xsk_is_in_mode(ifindex: u32, mode: c_int) -> bool {
    let mut opts: bpf_xdp_query_opts = core::mem::zeroed();
    let ret: c_int;

    opts.sz = size_of::<bpf_xdp_query_opts>();
    ret = bpf_xdp_query(ifindex as c_int, mode, &mut opts);
    if ret != 0 {
        printf(
            b"XDP mode query returned error %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return false;
    }

    if mode == XDP_FLAGS_DRV_MODE {
        return opts.attach_mode == XDP_ATTACHED_DRV;
    } else if mode == XDP_FLAGS_SKB_MODE {
        return opts.attach_mode == XDP_ATTACHED_SKB;
    }

    false
}

/* Lifted from netlink.c in tools/lib/bpf */
unsafe fn netlink_recvmsg(sock: c_int, mhdr: *mut msghdr, flags: c_int) -> c_int {
    let mut len: ssize_t;

    loop {
        len = recvmsg(sock, mhdr, flags);
        if !(len < 0 && (errno == EINTR || errno == EAGAIN)) {
            break;
        }
    }

    if len < 0 {
        return -errno;
    }
    len as c_int
}

/* Lifted from netlink.c in tools/lib/bpf */
unsafe fn alloc_iov(iov: *mut iovec, len: c_int) -> c_int {
    let nbuf: *mut c_void;

    nbuf = realloc((*iov).iov_base, len as usize);
    if nbuf.is_null() {
        return -ENOMEM;
    }

    (*iov).iov_base = nbuf;
    (*iov).iov_len = len as usize;
    0
}

/* Original version lifted from netlink.c in tools/lib/bpf */
unsafe fn netlink_recv(sock: c_int) -> c_int {
    let mut iov: iovec = core::mem::zeroed();
    let mut mhdr: msghdr = core::mem::zeroed();
    let mut multipart: bool = true;
    let mut errp: *mut nlmsgerr;
    let mut nh: *mut nlmsghdr;
    let mut len: c_int;
    let mut ret: c_int;

    mhdr.msg_iov = &mut iov;
    mhdr.msg_iovlen = 1;

    ret = alloc_iov(&mut iov, 4096);
    if ret != 0 {
        free(iov.iov_base);
        return ret;
    }

    while multipart {
        multipart = false;
        len = netlink_recvmsg(sock, &mut mhdr, MSG_PEEK | MSG_TRUNC);
        if len < 0 {
            ret = len;
            free(iov.iov_base);
            return ret;
        }

        if len as usize > iov.iov_len {
            ret = alloc_iov(&mut iov, len);
            if ret != 0 {
                free(iov.iov_base);
                return ret;
            }
        }

        len = netlink_recvmsg(sock, &mut mhdr, 0);
        if len < 0 {
            ret = len;
            free(iov.iov_base);
            return ret;
        }

        if len == 0 {
            break;
        }

        nh = iov.iov_base as *mut nlmsghdr;
        while NLMSG_OK(nh, len) {
            if (*nh).nlmsg_flags & NLM_F_MULTI != 0 {
                multipart = true;
            }
            match (*nh).nlmsg_type {
                NLMSG_ERROR => {
                    errp = NLMSG_DATA(nh) as *mut nlmsgerr;
                    if (*errp).error == 0 {
                        nh = NLMSG_NEXT(nh, &mut len);
                        continue;
                    }
                    ret = (*errp).error;
                    free(iov.iov_base);
                    return ret;
                }
                NLMSG_DONE => {
                    ret = 0;
                    free(iov.iov_base);
                    return ret;
                }
                _ => {}
            }
            nh = NLMSG_NEXT(nh, &mut len);
        }
    }
    ret = 0;
    free(iov.iov_base);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn xsk_set_mtu(ifindex: c_int, mtu: c_int) -> c_int {
    let mut req: nl_mtu_req = core::mem::zeroed();
    let mut rta: *mut rtattr;
    let fd: c_int;
    let mut ret: c_int;

    fd = socket(AF_NETLINK, SOCK_DGRAM, NETLINK_ROUTE);
    if fd < 0 {
        return fd;
    }

    memset(&mut req as *mut _ as *mut c_void, 0, size_of::<nl_mtu_req>());
    req.nh.nlmsg_len = NLMSG_LENGTH(size_of::<ifinfomsg>());
    req.nh.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
    req.nh.nlmsg_type = RTM_NEWLINK;
    req.msg.ifi_family = AF_UNSPEC;
    req.msg.ifi_index = ifindex;
    rta = (&mut req as *mut _ as *mut u8).add(NLMSG_ALIGN(req.nh.nlmsg_len as usize)) as *mut rtattr;
    (*rta).rta_type = IFLA_MTU;
    (*rta).rta_len = RTA_LENGTH(size_of::<c_uint>());
    req.nh.nlmsg_len =
        (NLMSG_ALIGN(req.nh.nlmsg_len as usize) + RTA_LENGTH(size_of::<c_int>()) as usize) as __u32;
    memcpy(
        RTA_DATA(rta),
        &mtu as *const _ as *const c_void,
        size_of::<c_int>(),
    );

    ret = send(
        fd,
        &req as *const _ as *const c_void,
        req.nh.nlmsg_len as usize,
        0,
    ) as c_int;
    if ret < 0 {
        close(fd);
        return errno;
    }

    ret = netlink_recv(fd);
    close(fd);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn xsk_attach_xdp_program(
    prog: *mut bpf_program,
    ifindex: c_int,
    xdp_flags: u32,
) -> c_int {
    let prog_fd: c_int;

    prog_fd = bpf_program__fd(prog);
    bpf_xdp_attach(ifindex, prog_fd, xdp_flags, ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn xsk_detach_xdp_program(ifindex: c_int, xdp_flags: u32) {
    bpf_xdp_detach(ifindex, xdp_flags, ptr::null());
}

#[no_mangle]
pub unsafe extern "C" fn xsk_clear_xskmap(map: *mut bpf_map) {
    let mut index: u32 = 0;
    let map_fd: c_int;

    map_fd = bpf_map__fd(map);
    bpf_map_delete_elem(map_fd, &mut index as *mut _ as *const c_void);
}

#[no_mangle]
pub unsafe extern "C" fn xsk_update_xskmap(
    map: *mut bpf_map,
    xsk: *mut xsk_socket,
    index: u32,
) -> c_int {
    let map_fd: c_int;
    let sock_fd: c_int;

    map_fd = bpf_map__fd(map);
    sock_fd = xsk_socket__fd(xsk);

    bpf_map_update_elem(
        map_fd,
        &index as *const _ as *const c_void,
        &sock_fd as *const _ as *const c_void,
        0,
    )
}

unsafe fn xsk_get_ctx(umem: *mut xsk_umem, ifindex: c_int, queue_id: __u32) -> *mut xsk_ctx {
    let mut pos: *mut list_head;
    let mut ctx: *mut xsk_ctx;
    let list_offset = core::mem::offset_of!(xsk_ctx, list);

    if list_empty(&(*umem).ctx_list) != 0 {
        return ptr::null_mut();
    }

    pos = (*umem).ctx_list.next;
    while pos != &mut (*umem).ctx_list {
        ctx = (pos as *mut u8).sub(list_offset) as *mut xsk_ctx;
        if (*ctx).ifindex == ifindex && (*ctx).queue_id == queue_id {
            (*ctx).refcount += 1;
            return ctx;
        }
        pos = (*pos).next;
    }

    ptr::null_mut()
}

unsafe fn xsk_put_ctx(ctx: *mut xsk_ctx, unmap: bool) {
    let umem: *mut xsk_umem = (*ctx).umem;
    let mut off: xdp_mmap_offsets = core::mem::zeroed();
    let err: c_int;

    (*ctx).refcount -= 1;
    if (*ctx).refcount != 0 {
        return;
    }

    if !unmap {
        list_del(&mut (*ctx).list);
        free(ctx as *mut c_void);
        return;
    }

    err = xsk_get_mmap_offsets((*umem).fd, &mut off);
    if err == 0 {
        munmap(
            ((*(*ctx).fill).ring as *mut u8).sub(off.fr.desc as usize) as *mut c_void,
            off.fr.desc as usize + (*umem).config.fill_size as usize * size_of::<__u64>(),
        );
        munmap(
            ((*(*ctx).comp).ring as *mut u8).sub(off.cr.desc as usize) as *mut c_void,
            off.cr.desc as usize + (*umem).config.comp_size as usize * size_of::<__u64>(),
        );
    }

    list_del(&mut (*ctx).list);
    free(ctx as *mut c_void);
}

unsafe fn xsk_create_ctx(
    xsk: *mut xsk_socket,
    umem: *mut xsk_umem,
    ifindex: c_int,
    queue_id: __u32,
    fill: *mut xsk_ring_prod,
    comp: *mut xsk_ring_cons,
) -> *mut xsk_ctx {
    let ctx: *mut xsk_ctx;
    let err: c_int;

    ctx = calloc(1, size_of::<xsk_ctx>()) as *mut xsk_ctx;
    if ctx.is_null() {
        return ptr::null_mut();
    }

    if (*umem).fill_save.is_null() {
        err = xsk_create_umem_rings(umem, (*xsk).fd, fill, comp);
        if err != 0 {
            free(ctx as *mut c_void);
            return ptr::null_mut();
        }
    } else if (*umem).fill_save != fill || (*umem).comp_save != comp {
        /* Copy over rings to new structs. */
        memcpy(
            fill as *mut c_void,
            (*umem).fill_save as *const c_void,
            size_of::<xsk_ring_prod>(),
        );
        memcpy(
            comp as *mut c_void,
            (*umem).comp_save as *const c_void,
            size_of::<xsk_ring_cons>(),
        );
    }

    (*ctx).ifindex = ifindex;
    (*ctx).refcount = 1;
    (*ctx).umem = umem;
    (*ctx).queue_id = queue_id;

    (*ctx).fill = fill;
    (*ctx).comp = comp;
    list_add(&mut (*ctx).list, &mut (*umem).ctx_list);
    ctx
}

#[no_mangle]
pub unsafe extern "C" fn xsk_socket__create_shared(
    xsk_ptr: *mut *mut xsk_socket,
    ifindex: c_int,
    queue_id: __u32,
    umem: *mut xsk_umem,
    rx: *mut xsk_ring_cons,
    tx: *mut xsk_ring_prod,
    fill: *mut xsk_ring_prod,
    comp: *mut xsk_ring_cons,
    usr_config: *const xsk_socket_config,
) -> c_int {
    let unmap: bool;
    let mut rx_setup_done: bool = false;
    let mut tx_setup_done: bool = false;
    let mut rx_map: *mut c_void = ptr::null_mut();
    let mut tx_map: *mut c_void = ptr::null_mut();
    let mut sxdp: sockaddr_xdp = core::mem::zeroed();
    let mut off: xdp_mmap_offsets = core::mem::zeroed();
    let xsk: *mut xsk_socket;
    let mut ctx: *mut xsk_ctx;
    let mut err: c_int;

    if umem.is_null() || xsk_ptr.is_null() || (rx.is_null() && tx.is_null()) {
        return -EFAULT;
    }

    unmap = (*umem).fill_save != fill;

    xsk = calloc(1, size_of::<xsk_socket>()) as *mut xsk_socket;
    if xsk.is_null() {
        return -ENOMEM;
    }

    err = xsk_set_xdp_socket_config(&mut (*xsk).config, usr_config);
    if err != 0 {
        free(xsk as *mut c_void);
        return err;
    }

    (*umem).refcount += 1;
    if (*umem).refcount - 1 > 0 {
        (*xsk).fd = socket(AF_XDP, SOCK_RAW | SOCK_CLOEXEC, 0);
        if (*xsk).fd < 0 {
            err = -errno;
            free(xsk as *mut c_void);
            return err;
        }
    } else {
        (*xsk).fd = (*umem).fd;
        rx_setup_done = (*umem).rx_ring_setup_done;
        tx_setup_done = (*umem).tx_ring_setup_done;
    }

    ctx = xsk_get_ctx(umem, ifindex, queue_id);
    if ctx.is_null() {
        if fill.is_null() || comp.is_null() {
            err = -EFAULT;
            if {
                (*umem).refcount -= 1;
                (*umem).refcount != 0
            } {
                close((*xsk).fd);
            }
            free(xsk as *mut c_void);
            return err;
        }

        ctx = xsk_create_ctx(xsk, umem, ifindex, queue_id, fill, comp);
        if ctx.is_null() {
            err = -ENOMEM;
            if {
                (*umem).refcount -= 1;
                (*umem).refcount != 0
            } {
                close((*xsk).fd);
            }
            free(xsk as *mut c_void);
            return err;
        }
    }
    (*xsk).ctx = ctx;

    if !rx.is_null() && !rx_setup_done {
        err = setsockopt(
            (*xsk).fd,
            SOL_XDP,
            XDP_RX_RING,
            &(*xsk).config.rx_size as *const _ as *const c_void,
            size_of::<__u32>() as socklen_t,
        );
        if err != 0 {
            err = -errno;
            xsk_put_ctx(ctx, unmap);
            if {
                (*umem).refcount -= 1;
                (*umem).refcount != 0
            } {
                close((*xsk).fd);
            }
            free(xsk as *mut c_void);
            return err;
        }
        if (*xsk).fd == (*umem).fd {
            (*umem).rx_ring_setup_done = true;
        }
    }
    if !tx.is_null() && !tx_setup_done {
        err = setsockopt(
            (*xsk).fd,
            SOL_XDP,
            XDP_TX_RING,
            &(*xsk).config.tx_size as *const _ as *const c_void,
            size_of::<__u32>() as socklen_t,
        );
        if err != 0 {
            err = -errno;
            xsk_put_ctx(ctx, unmap);
            if {
                (*umem).refcount -= 1;
                (*umem).refcount != 0
            } {
                close((*xsk).fd);
            }
            free(xsk as *mut c_void);
            return err;
        }
        if (*xsk).fd == (*umem).fd {
            (*umem).tx_ring_setup_done = true;
        }
    }

    err = xsk_get_mmap_offsets((*xsk).fd, &mut off);
    if err != 0 {
        err = -errno;
        xsk_put_ctx(ctx, unmap);
        if {
            (*umem).refcount -= 1;
            (*umem).refcount != 0
        } {
            close((*xsk).fd);
        }
        free(xsk as *mut c_void);
        return err;
    }

    if !rx.is_null() {
        rx_map = mmap(
            ptr::null_mut(),
            off.rx.desc as usize + (*xsk).config.rx_size as usize * size_of::<xdp_desc>(),
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_POPULATE,
            (*xsk).fd,
            XDP_PGOFF_RX_RING,
        );
        if rx_map == MAP_FAILED {
            err = -errno;
            xsk_put_ctx(ctx, unmap);
            if {
                (*umem).refcount -= 1;
                (*umem).refcount != 0
            } {
                close((*xsk).fd);
            }
            free(xsk as *mut c_void);
            return err;
        }

        (*rx).mask = (*xsk).config.rx_size - 1;
        (*rx).size = (*xsk).config.rx_size;
        (*rx).producer = (rx_map as *mut u8).add(off.rx.producer as usize) as *mut __u32;
        (*rx).consumer = (rx_map as *mut u8).add(off.rx.consumer as usize) as *mut __u32;
        (*rx).flags = (rx_map as *mut u8).add(off.rx.flags as usize) as *mut __u32;
        (*rx).ring = (rx_map as *mut u8).add(off.rx.desc as usize) as *mut c_void;
        (*rx).cached_prod = *(*rx).producer;
        (*rx).cached_cons = *(*rx).consumer;
    }
    (*xsk).rx = rx;

    if !tx.is_null() {
        tx_map = mmap(
            ptr::null_mut(),
            off.tx.desc as usize + (*xsk).config.tx_size as usize * size_of::<xdp_desc>(),
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_POPULATE,
            (*xsk).fd,
            XDP_PGOFF_TX_RING,
        );
        if tx_map == MAP_FAILED {
            err = -errno;
            if !rx.is_null() {
                munmap(
                    rx_map,
                    off.rx.desc as usize + (*xsk).config.rx_size as usize * size_of::<xdp_desc>(),
                );
            }
            xsk_put_ctx(ctx, unmap);
            if {
                (*umem).refcount -= 1;
                (*umem).refcount != 0
            } {
                close((*xsk).fd);
            }
            free(xsk as *mut c_void);
            return err;
        }

        (*tx).mask = (*xsk).config.tx_size - 1;
        (*tx).size = (*xsk).config.tx_size;
        (*tx).producer = (tx_map as *mut u8).add(off.tx.producer as usize) as *mut __u32;
        (*tx).consumer = (tx_map as *mut u8).add(off.tx.consumer as usize) as *mut __u32;
        (*tx).flags = (tx_map as *mut u8).add(off.tx.flags as usize) as *mut __u32;
        (*tx).ring = (tx_map as *mut u8).add(off.tx.desc as usize) as *mut c_void;
        (*tx).cached_prod = *(*tx).producer;
        /* cached_cons is r->size bigger than the real consumer pointer
         * See xsk_prod_nb_free
         */
        (*tx).cached_cons = (*(*tx).consumer).wrapping_add((*xsk).config.tx_size);
    }
    (*xsk).tx = tx;

    sxdp.sxdp_family = PF_XDP as u16;
    sxdp.sxdp_ifindex = (*ctx).ifindex as __u32;
    sxdp.sxdp_queue_id = (*ctx).queue_id;
    if (*umem).refcount > 1 {
        sxdp.sxdp_flags |= XDP_SHARED_UMEM;
        sxdp.sxdp_shared_umem_fd = (*umem).fd as __u32;
    } else {
        sxdp.sxdp_flags = (*xsk).config.bind_flags as __u16;
    }

    err = bind(
        (*xsk).fd,
        &sxdp as *const _ as *const sockaddr,
        size_of::<sockaddr_xdp>() as socklen_t,
    );
    if err != 0 {
        err = -errno;
        if !tx.is_null() {
            munmap(
                tx_map,
                off.tx.desc as usize + (*xsk).config.tx_size as usize * size_of::<xdp_desc>(),
            );
        }
        if !rx.is_null() {
            munmap(
                rx_map,
                off.rx.desc as usize + (*xsk).config.rx_size as usize * size_of::<xdp_desc>(),
            );
        }
        xsk_put_ctx(ctx, unmap);
        if {
            (*umem).refcount -= 1;
            (*umem).refcount != 0
        } {
            close((*xsk).fd);
        }
        free(xsk as *mut c_void);
        return err;
    }

    *xsk_ptr = xsk;
    (*umem).fill_save = ptr::null_mut();
    (*umem).comp_save = ptr::null_mut();
    0
}

#[no_mangle]
pub unsafe extern "C" fn xsk_socket__create(
    xsk_ptr: *mut *mut xsk_socket,
    ifindex: c_int,
    queue_id: __u32,
    umem: *mut xsk_umem,
    rx: *mut xsk_ring_cons,
    tx: *mut xsk_ring_prod,
    usr_config: *const xsk_socket_config,
) -> c_int {
    if umem.is_null() {
        return -EFAULT;
    }

    xsk_socket__create_shared(
        xsk_ptr,
        ifindex,
        queue_id,
        umem,
        rx,
        tx,
        (*umem).fill_save,
        (*umem).comp_save,
        usr_config,
    )
}

#[no_mangle]
pub unsafe extern "C" fn xsk_umem__delete(umem: *mut xsk_umem) -> c_int {
    let mut off: xdp_mmap_offsets = core::mem::zeroed();
    let err: c_int;

    if umem.is_null() {
        return 0;
    }

    if (*umem).refcount != 0 {
        return -EBUSY;
    }

    err = xsk_get_mmap_offsets((*umem).fd, &mut off);
    if err == 0 && !(*umem).fill_save.is_null() && !(*umem).comp_save.is_null() {
        munmap(
            ((*(*umem).fill_save).ring as *mut u8).sub(off.fr.desc as usize) as *mut c_void,
            off.fr.desc as usize + (*umem).config.fill_size as usize * size_of::<__u64>(),
        );
        munmap(
            ((*(*umem).comp_save).ring as *mut u8).sub(off.cr.desc as usize) as *mut c_void,
            off.cr.desc as usize + (*umem).config.comp_size as usize * size_of::<__u64>(),
        );
    }

    close((*umem).fd);
    free(umem as *mut c_void);

    0
}

#[no_mangle]
pub unsafe extern "C" fn xsk_socket__delete(xsk: *mut xsk_socket) {
    let desc_sz: size_t = size_of::<xdp_desc>();
    let mut off: xdp_mmap_offsets = core::mem::zeroed();
    let umem: *mut xsk_umem;
    let ctx: *mut xsk_ctx;
    let err: c_int;

    if xsk.is_null() {
        return;
    }

    ctx = (*xsk).ctx;
    umem = (*ctx).umem;

    xsk_put_ctx(ctx, true);

    err = xsk_get_mmap_offsets((*xsk).fd, &mut off);
    if err == 0 {
        if !(*xsk).rx.is_null() {
            munmap(
                ((*(*xsk).rx).ring as *mut u8).sub(off.rx.desc as usize) as *mut c_void,
                off.rx.desc as usize + (*xsk).config.rx_size as usize * desc_sz,
            );
        }
        if !(*xsk).tx.is_null() {
            munmap(
                ((*(*xsk).tx).ring as *mut u8).sub(off.tx.desc as usize) as *mut c_void,
                off.tx.desc as usize + (*xsk).config.tx_size as usize * desc_sz,
            );
        }
    }

    (*umem).refcount -= 1;
    /* Do not close an fd that also has an associated umem connected
     * to it.
     */
    if (*xsk).fd != (*umem).fd {
        close((*xsk).fd);
    }
    free(xsk as *mut c_void);
}
