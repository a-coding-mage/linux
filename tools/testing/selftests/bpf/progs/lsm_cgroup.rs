// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// "vmlinux.h", "bpf_tracing_net.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type __u64 = u64;
type gfp_t = u32;

const BPF_MAP_TYPE_CGROUP_STORAGE: u32 = 19;
const AF_PACKET: i32 = 17;
const AF_UNIX: i32 = 1;
const EPERM: i32 = 1;

extern "C" {
    static mut CONFIG_SECURITY_SELINUX: bool;
    static mut CONFIG_SECURITY_SMACK: bool;
    static mut CONFIG_SECURITY_APPARMOR: bool;

    fn bpf_get_local_storage(map: *mut core::ffi::c_void, flags: u64) -> *mut core::ffi::c_void;
    fn bpf_setsockopt(
        sk: *mut sock,
        level: i32,
        optname: i32,
        optval: *const core::ffi::c_void,
        optlen: i32,
    ) -> i32;
    fn bpf_getsockopt(
        sk: *mut sock,
        level: i32,
        optname: i32,
        optval: *mut core::ffi::c_void,
        optlen: i32,
    ) -> i32;
    fn bpf_probe_read_kernel(
        dst: *mut core::ffi::c_void,
        size: u32,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i32;
    fn bpf_get_retval() -> i32;
    fn bpf_set_retval(retval: i32) -> i32;
}

extern "C" {
    static SOL_SOCKET: i32;
    static SO_PRIORITY: i32;
}

#[repr(C)]
pub struct socket {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct sock {
    pub __sk_common: sock_common,
    pub sk_kern_sock: u8,
}

#[repr(C)]
pub struct sock_common {
    pub skc_family: u16,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

#[repr(C)]
pub struct sockaddr_ll {
    pub sll_family: u16,
    pub sll_protocol: u16,
    pub sll_ifindex: i32,
    pub sll_hatype: u16,
    pub sll_pkttype: u8,
    pub sll_halen: u8,
    pub sll_addr: [u8; 8],
}

#[repr(C)]
pub struct request_sock {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct cgroup_storage_map {
    // __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
    // __type(key, __u64);
    // __type(value, __u64);
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SEC(".maps")
#[no_mangle]
pub static mut cgroup_storage: cgroup_storage_map = cgroup_storage_map {
    type_: BPF_MAP_TYPE_CGROUP_STORAGE,
    key_size: core::mem::size_of::<__u64>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

#[no_mangle]
pub static mut called_socket_post_create: i32 = 0;
#[no_mangle]
pub static mut called_socket_post_create2: i32 = 0;
#[no_mangle]
pub static mut called_socket_bind: i32 = 0;
#[no_mangle]
pub static mut called_socket_bind2: i32 = 0;
#[no_mangle]
pub static mut called_socket_alloc: i32 = 0;
#[no_mangle]
pub static mut called_socket_clone: i32 = 0;
#[no_mangle]
pub static mut skipcap_retval: i32 = -4095;
#[no_mangle]
pub static mut socket_retval: i32 = -4095;

#[inline(always)]
unsafe fn test_local_storage() -> i32 {
    let val: *mut __u64;

    val = bpf_get_local_storage(
        core::ptr::addr_of_mut!(cgroup_storage) as *mut core::ffi::c_void,
        0,
    ) as *mut __u64;
    if val.is_null() {
        return 0;
    }
    *val = (*val).wrapping_add(1);

    1
}

#[inline(always)]
unsafe fn real_create(sock: *mut socket, family: i32, protocol: i32) -> i32 {
    let sk: *mut sock;
    let mut prio: i32 = 123;

    /* Reject non-tx-only AF_PACKET. */
    if family == AF_PACKET && protocol != 0 {
        return 0; /* EPERM */
    }

    sk = (*sock).sk;
    if sk.is_null() {
        return 1;
    }

    /* The rest of the sockets get default policy. */
    if bpf_setsockopt(
        sk,
        SOL_SOCKET,
        SO_PRIORITY,
        core::ptr::addr_of!(prio) as *const core::ffi::c_void,
        core::mem::size_of_val(&prio) as i32,
    ) != 0
    {
        return 0; /* EPERM */
    }

    /* Make sure bpf_getsockopt is allowed and works. */
    prio = 0;
    if bpf_getsockopt(
        sk,
        SOL_SOCKET,
        SO_PRIORITY,
        core::ptr::addr_of_mut!(prio) as *mut core::ffi::c_void,
        core::mem::size_of_val(&prio) as i32,
    ) != 0
    {
        return 0; /* EPERM */
    }
    if prio != 123 {
        return 0; /* EPERM */
    }

    /* Can access cgroup local storage. */
    if test_local_storage() == 0 {
        return 0; /* EPERM */
    }

    1
}

/* __cgroup_bpf_run_lsm_socket */
// SEC("lsm_cgroup/socket_post_create")
#[no_mangle]
pub unsafe extern "C" fn socket_post_create(
    sock: *mut socket,
    family: i32,
    _type: i32,
    protocol: i32,
    kern: i32,
) -> i32 {
    let _ = kern;
    called_socket_post_create = called_socket_post_create.wrapping_add(1);
    real_create(sock, family, protocol)
}

/* __cgroup_bpf_run_lsm_socket */
// SEC("lsm_cgroup/socket_post_create")
#[no_mangle]
pub unsafe extern "C" fn socket_post_create2(
    sock: *mut socket,
    family: i32,
    _type: i32,
    protocol: i32,
    kern: i32,
) -> i32 {
    let _ = kern;
    called_socket_post_create2 = called_socket_post_create2.wrapping_add(1);
    real_create(sock, family, protocol)
}

#[inline(always)]
unsafe fn real_bind(sock: *mut socket, address: *mut sockaddr, addrlen: i32) -> i32 {
    let mut sa: sockaddr_ll = core::mem::zeroed();
    let sk: *mut sock = (*sock).sk;

    let _ = addrlen;

    if sk.is_null() {
        return 1;
    }

    if (*sk).__sk_common.skc_family as i32 != AF_PACKET {
        return 1;
    }

    if (*sk).sk_kern_sock != 0 {
        return 1;
    }

    bpf_probe_read_kernel(
        core::ptr::addr_of_mut!(sa) as *mut core::ffi::c_void,
        core::mem::size_of_val(&sa) as u32,
        address as *const core::ffi::c_void,
    );
    if sa.sll_protocol != 0 {
        return 0; /* EPERM */
    }

    /* Can access cgroup local storage. */
    if test_local_storage() == 0 {
        return 0; /* EPERM */
    }

    1
}

/* __cgroup_bpf_run_lsm_socket */
// SEC("lsm_cgroup/socket_bind")
#[no_mangle]
pub unsafe extern "C" fn socket_bind(
    sock: *mut socket,
    address: *mut sockaddr,
    addrlen: i32,
) -> i32 {
    called_socket_bind = called_socket_bind.wrapping_add(1);
    real_bind(sock, address, addrlen)
}

/* __cgroup_bpf_run_lsm_socket */
// SEC("lsm_cgroup/socket_bind")
#[no_mangle]
pub unsafe extern "C" fn socket_bind2(
    sock: *mut socket,
    address: *mut sockaddr,
    addrlen: i32,
) -> i32 {
    called_socket_bind2 = called_socket_bind2.wrapping_add(1);
    real_bind(sock, address, addrlen)
}

/* __cgroup_bpf_run_lsm_current (via bpf_lsm_current_hooks) */
// SEC("lsm_cgroup/sk_alloc_security")
#[no_mangle]
pub unsafe extern "C" fn socket_alloc(sk: *mut sock, family: i32, priority: gfp_t) -> i32 {
    let _ = sk;
    let _ = priority;

    called_socket_alloc = called_socket_alloc.wrapping_add(1);
    /* if already have non-bpf lsms installed, EPERM will cause memory leak of non-bpf lsms */
    if CONFIG_SECURITY_SELINUX || CONFIG_SECURITY_SMACK || CONFIG_SECURITY_APPARMOR {
        return 1;
    }

    if family == AF_UNIX {
        return 0; /* EPERM */
    }

    /* Can access cgroup local storage. */
    if test_local_storage() == 0 {
        return 0; /* EPERM */
    }

    1
}

/* __cgroup_bpf_run_lsm_sock */
// SEC("lsm_cgroup/inet_csk_clone")
#[no_mangle]
pub unsafe extern "C" fn socket_clone(newsk: *mut sock, req: *const request_sock) -> i32 {
    let mut prio: i32 = 234;

    let _ = req;

    if newsk.is_null() {
        return 1;
    }

    /* Accepted request sockets get a different priority. */
    if bpf_setsockopt(
        newsk,
        SOL_SOCKET,
        SO_PRIORITY,
        core::ptr::addr_of!(prio) as *const core::ffi::c_void,
        core::mem::size_of_val(&prio) as i32,
    ) != 0
    {
        return 1;
    }

    /* Make sure bpf_getsockopt is allowed and works. */
    prio = 0;
    if bpf_getsockopt(
        newsk,
        SOL_SOCKET,
        SO_PRIORITY,
        core::ptr::addr_of_mut!(prio) as *mut core::ffi::c_void,
        core::mem::size_of_val(&prio) as i32,
    ) != 0
    {
        return 1;
    }
    if prio != 234 {
        return 1;
    }

    /* Can access cgroup local storage. */
    if test_local_storage() == 0 {
        return 1;
    }

    called_socket_clone = called_socket_clone.wrapping_add(1);

    1
}

// SEC("lsm_cgroup/inode_xattr_skipcap")
#[no_mangle]
pub unsafe extern "C" fn skipcap_first(name: *const i8) -> i32 {
    let _ = name;
    0
}

// SEC("lsm_cgroup/inode_xattr_skipcap")
#[no_mangle]
pub unsafe extern "C" fn skipcap_second(name: *const i8) -> i32 {
    let _ = name;
    skipcap_retval = bpf_get_retval();
    bpf_set_retval(0);
    1
}

// SEC("lsm_cgroup/socket_create")
#[no_mangle]
pub unsafe extern "C" fn socket_first(
    family: i32,
    _type: i32,
    protocol: i32,
    kern: i32,
) -> i32 {
    let _ = family;
    let _ = protocol;
    let _ = kern;
    0
}

// SEC("lsm_cgroup/socket_create")
#[no_mangle]
pub unsafe extern "C" fn socket_second(
    family: i32,
    _type: i32,
    protocol: i32,
    kern: i32,
) -> i32 {
    let _ = family;
    let _ = protocol;
    let _ = kern;
    socket_retval = bpf_get_retval();
    bpf_set_retval(0);
    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
