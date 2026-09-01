// SPDX-License-Identifier: GPL-2.0
// C dependencies: vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h,
// bpf/bpf_core_read.h, bpf_misc.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_ulong, c_void};

type __u32 = u32;

extern "C" {
    type sockaddr_in;

    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_probe_read_user(dst: *mut c_void, size: usize, unsafe_ptr: *const c_void) -> c_int;
    fn bpf_probe_write_user(dst: *mut c_void, src: *const c_void, size: usize) -> c_int;
}

#[repr(C)]
pub struct test_pro_bss {
    pub old: sockaddr_in,
    pub test_pid: __u32,
}

#[no_mangle]
pub static mut bss: test_pro_bss = unsafe { core::mem::zeroed() };

unsafe fn handle_sys_connect_common(uservaddr: *mut sockaddr_in) -> c_int {
    let mut new: sockaddr_in = core::mem::zeroed();
    let cur: __u32 = (bpf_get_current_pid_tgid() >> 32) as __u32;

    if bss.test_pid != 0 && cur != bss.test_pid {
        return 0;
    }

    bpf_probe_read_user(
        core::ptr::addr_of_mut!(bss.old).cast::<c_void>(),
        core::mem::size_of_val(&bss.old),
        uservaddr.cast::<c_void>(),
    );
    core::ptr::write_bytes(
        core::ptr::addr_of_mut!(new).cast::<u8>(),
        0xab,
        core::mem::size_of_val(&new),
    );
    bpf_probe_write_user(
        uservaddr.cast::<c_void>(),
        core::ptr::addr_of!(new).cast::<c_void>(),
        core::mem::size_of_val(&new),
    );

    0
}

#[no_mangle]
#[link_section = "ksyscall/connect"]
pub unsafe extern "C" fn handle_sys_connect(
    fd: c_int,
    uservaddr: *mut sockaddr_in,
    addrlen: c_int,
) -> c_int {
    let _ = fd;
    let _ = addrlen;

    handle_sys_connect_common(uservaddr)
}

// Original C condition: #if defined(bpf_target_s390)
#[cfg(bpf_target_s390)]
const SYS_CONNECT: c_int = 3;

// Original C condition: #if defined(bpf_target_s390)
#[cfg(bpf_target_s390)]
#[no_mangle]
#[link_section = "ksyscall/socketcall"]
pub unsafe extern "C" fn handle_sys_socketcall(call: c_int, args: *mut c_ulong) -> c_int {
    if call == SYS_CONNECT {
        let mut uservaddr: *mut sockaddr_in = core::ptr::null_mut();

        bpf_probe_read_user(
            core::ptr::addr_of_mut!(uservaddr).cast::<c_void>(),
            core::mem::size_of_val(&uservaddr),
            args.add(1).cast::<c_void>(),
        );
        return handle_sys_connect_common(uservaddr);
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
