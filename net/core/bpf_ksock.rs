// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Isovalent */

// Kernel declarations supplied by the surrounding tree are intentionally left external.

use core::ffi::c_void;

type U32 = u32;
type CInt = i32;

#[repr(C)]
pub struct Socket {
    pub sk: *mut Sock,
}

#[repr(C)]
pub struct Sock {
    pub sk_rcvbuf: CInt,
    pub sk_userlocks: U32,
}

#[repr(C)]
pub struct RefcountT {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RcuWork {
    _private: [u8; 0],
}

#[repr(C)]
pub struct WorkStruct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BpfProg {
    pub prog_type: CInt,
    pub aux: *mut BpfProgAux,
}

#[repr(C)]
pub struct BpfProgAux {
    pub attach_btf_id: U32,
}

#[repr(C)]
pub struct BpfKsockCreateOpts {
    pub family: CInt,
    pub type_: CInt,
    pub protocol: CInt,
    pub reserved: U32,
}

#[repr(C)]
pub union BpfKsockAddr {
    pub ipv4: SockaddrIn,
    pub ipv6: SockaddrIn6,
}

#[repr(C)]
pub struct SockaddrStorage {
    pub ss_family: CInt,
    pub data: [u8; 126],
}

#[repr(C)]
pub struct SockaddrIn {
    _private: [u8; 16],
}

#[repr(C)]
pub struct SockaddrIn6 {
    _private: [u8; 28],
}

#[repr(C)]
pub struct Msghdr {
    pub msg_flags: CInt,
}

#[repr(C)]
pub struct Kvec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
pub struct BtfKfuncIdSet {
    _private: [u8; 0],
}

extern "C" {
    static system_dfl_wq: *mut c_void;
    static this_module: c_void;

    fn sock_release(sock: *mut Socket);
    fn kfree(ptr: *mut c_void);
    fn sock_create(family: CInt, type_: CInt, protocol: CInt, res: *mut *mut Socket) -> CInt;
    fn connect_socket(sock: *mut Socket, addr: *const SockaddrStorage, addrlen: CInt, flags: CInt) -> CInt;
    fn kernel_sendmsg(sock: *mut Socket, msg: *mut Msghdr, vec: *mut Kvec, nr: usize, size: usize) -> CInt;
    fn refcount_inc_not_zero(refcount: *mut RefcountT) -> bool;
    fn refcount_dec_and_test(refcount: *mut RefcountT) -> bool;
    fn refcount_set(refcount: *mut RefcountT, value: CInt);
    fn init_rcu_work(work: *mut RcuWork, func: unsafe extern "C" fn(*mut WorkStruct));
    fn queue_rcu_work(wq: *mut c_void, work: *mut RcuWork) -> bool;
    fn btf_id_set8_contains(set: *const c_void, id: U32) -> bool;
    fn register_btf_kfunc_id_set(prog_type: CInt, set: *const BtfKfuncIdSet) -> CInt;
    fn register_btf_id_dtor_kfuncs(dtors: *const c_void, count: usize, owner: *const c_void) -> CInt;
}

#[repr(C)]
pub struct BpfKsock {
    pub sock: *mut Socket,
    pub usage: RefcountT,
    pub rwork: RcuWork,
}

unsafe extern "C" fn ksock_release_work_fn(work: *mut WorkStruct) {
    let ks = (work as *mut u8).sub(core::mem::offset_of!(BpfKsock, rwork)) as *mut BpfKsock;
    sock_release((*ks).sock);
    kfree(ks.cast());
}

unsafe fn bpf_ksock_has_user_task_context() -> bool {
    // Task work can run from do_exit() after exit_nsproxy_namespaces() cleared
    // current->nsproxy, while current is still not a kthread.
    extern "C" {
        static current_flags: U32;
        static current_nsproxy: *mut c_void;
    }
    const PF_KTHREAD: U32 = 0x0020_0000;
    (current_flags & PF_KTHREAD) == 0 && !current_nsproxy.is_null()
}

pub unsafe extern "C" fn bpf_ksock_create(
    opts: *const BpfKsockCreateOpts,
    opts__sz: U32,
    err__uninit: *mut CInt,
) -> *mut BpfKsock {
    let mut err: CInt;
    if !bpf_ksock_has_user_task_context() { err = -95; }
    else if opts.is_null() || opts__sz as usize != core::mem::size_of::<BpfKsockCreateOpts>() { err = -22; }
    else {
        let o = &*opts;
        let copy = BpfKsockCreateOpts { family: core::ptr::read_volatile(&o.family), type_: core::ptr::read_volatile(&o.type_), protocol: core::ptr::read_volatile(&o.protocol), reserved: core::ptr::read_volatile(&o.reserved) };
        if copy.reserved != 0 { err = -22; }
        else if copy.family != 2 && copy.family != 10 { err = -97; }
        else if copy.type_ != 2 { err = -93; }
        else if copy.protocol != 17 && copy.protocol != 0 { err = -93; }
        else {
            let ks = alloc_zeroed::<BpfKsock>();
            if ks.is_null() { err = -12; }
            else {
                err = sock_create(copy.family, copy.type_, copy.protocol, &mut (*ks).sock);
                if err == 0 {
                    (*(*ks).sock).sk.as_mut().unwrap().sk_rcvbuf = 256;
                    (*(*ks).sock).sk.as_mut().unwrap().sk_userlocks |= 1;
                    refcount_set(&mut (*ks).usage, 1);
                    core::ptr::write_unaligned(err__uninit, 0);
                    return ks;
                }
                kfree(ks.cast());
            }
        }
    }
    core::ptr::write_unaligned(err__uninit, err);
    core::ptr::null_mut()
}

unsafe fn alloc_zeroed<T>() -> *mut T {
    let layout = core::alloc::Layout::new::<T>();
    std::alloc::alloc_zeroed(layout).cast()
}

pub unsafe extern "C" fn bpf_ksock_connect(ks: *mut BpfKsock, addr: *const BpfKsockAddr, addr__sz: U32) -> CInt {
    if !bpf_ksock_has_user_task_context() { return -95; }
    if addr.is_null() || addr__sz as usize != core::mem::size_of::<BpfKsockAddr>() { return -22; }
    let sa: SockaddrStorage = core::ptr::read_unaligned(addr.cast());
    let addrlen = match sa.ss_family { 2 => 16, 10 => 28, _ => return -97 };
    connect_socket((*ks).sock, &sa, addrlen, 0)
}

pub unsafe extern "C" fn bpf_ksock_acquire(ks: *mut BpfKsock) -> *mut BpfKsock {
    if !refcount_inc_not_zero(&mut (*ks).usage) { core::ptr::null_mut() } else { ks }
}

pub unsafe extern "C" fn bpf_ksock_release(ks: *mut BpfKsock) {
    if refcount_dec_and_test(&mut (*ks).usage) {
        init_rcu_work(&mut (*ks).rwork, ksock_release_work_fn);
        queue_rcu_work(system_dfl_wq, &mut (*ks).rwork);
    }
}

pub unsafe extern "C" fn bpf_ksock_release_dtor(ks: *mut c_void) { bpf_ksock_release(ks.cast()); }

pub unsafe extern "C" fn bpf_ksock_send(ks: *mut BpfKsock, data: *const c_void, data__sz: U32) -> CInt {
    if !bpf_ksock_has_user_task_context() { return -95; }
    let mut msg = Msghdr { msg_flags: 0x40 };
    let mut iov = Kvec { iov_base: data as *mut c_void, iov_len: data__sz as usize };
    kernel_sendmsg((*ks).sock, &mut msg, &mut iov, 1, data__sz as usize)
}

// BTF kfunc registration, filtering, destructor IDs, and late_initcall are
// supplied by the kernel build; their source-level declarations are external.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
