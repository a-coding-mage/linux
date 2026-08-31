// SPDX-License-Identifier: GPL-2.0
/*  Copyright(c) 2016-20 Intel Corporation. */

use core::arch::asm;
use core::ffi::c_void;

/*
 * Dependency declarations originally supplied by "defines.h".
 */
const ENCL_OP_MAX: usize = 8;

#[repr(C, align(64))]
struct sgx_secinfo {
    flags: u64,
    reserved: [u64; 7],
}

#[repr(C)]
struct encl_op_header {
    r#type: u64,
}

#[repr(C)]
struct encl_op_emodpe {
    header: encl_op_header,
    flags: u64,
    epc_addr: u64,
}

#[repr(C)]
struct encl_op_eaccept {
    header: encl_op_header,
    flags: u64,
    epc_addr: u64,
    ret: i32,
}

#[repr(C)]
struct encl_op_init_tcs_page {
    header: encl_op_header,
    tcs_page: u64,
    ssa: u64,
    entry: u64,
}

#[repr(C)]
struct encl_op_put_to_buf {
    header: encl_op_header,
    value: u64,
}

#[repr(C)]
struct encl_op_get_from_buf {
    header: encl_op_header,
    value: u64,
}

#[repr(C)]
struct encl_op_put_to_addr {
    header: encl_op_header,
    addr: u64,
    value: u64,
}

#[repr(C)]
struct encl_op_get_from_addr {
    header: encl_op_header,
    addr: u64,
    value: u64,
}

/*
 * Data buffer spanning two pages that will be placed first in the .data
 * segment via the linker script. Even if not used internally the second page
 * is needed by external test manipulating page permissions, so mark
 * encl_buffer as "used" to make sure it is entirely preserved by the compiler.
 */
#[used]
#[link_section = ".data.encl_buffer"]
static mut encl_buffer: [u8; 8192] = {
    let mut buf = [0u8; 8192];
    buf[0] = 1;
    buf
};

#[repr(u32)]
enum sgx_enclu_function {
    EACCEPT = 0x5,
    EMODPE = 0x6,
}

unsafe extern "C" fn do_encl_emodpe(_op: *mut c_void) {
    let mut secinfo = sgx_secinfo {
        flags: 0,
        reserved: [0; 7],
    };
    let op = _op as *mut encl_op_emodpe;

    secinfo.flags = (*op).flags;

    asm!(
        ".byte 0x0f, 0x01, 0xd7",
        in("rax") sgx_enclu_function::EMODPE as u64,
        in("rbx") &secinfo,
        in("rcx") (*op).epc_addr,
        options(nostack, preserves_flags),
    );
}

unsafe extern "C" fn do_encl_eaccept(_op: *mut c_void) {
    let mut secinfo = sgx_secinfo {
        flags: 0,
        reserved: [0; 7],
    };
    let op = _op as *mut encl_op_eaccept;
    let rax: i32;

    secinfo.flags = (*op).flags;

    asm!(
        ".byte 0x0f, 0x01, 0xd7",
        inlateout("rax") sgx_enclu_function::EACCEPT as u64 => rax,
        in("rbx") &secinfo,
        in("rcx") (*op).epc_addr,
        options(nostack, preserves_flags),
    );

    (*op).ret = rax;
}

unsafe fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let mut i: usize = 0;

    while i < n {
        *(dest as *mut i8).add(i) = *(src as *const i8).add(i);
        i += 1;
    }

    dest
}

unsafe fn memset(dest: *mut c_void, c: i32, n: usize) -> *mut c_void {
    let mut i: usize = 0;

    while i < n {
        *(dest as *mut i8).add(i) = c as i8;
        i += 1;
    }

    dest
}

unsafe extern "C" fn do_encl_init_tcs_page(_op: *mut c_void) {
    let op = _op as *mut encl_op_init_tcs_page;
    let tcs = (*op).tcs_page as *mut u8;
    let mut val_32: u32;

    memset(tcs as *mut c_void, 0, 16); /* STATE and FLAGS */
    memcpy(tcs.add(16) as *mut c_void, &(*op).ssa as *const _ as *const c_void, 8); /* OSSA */
    memset(tcs.add(24) as *mut c_void, 0, 4); /* CSSA */
    val_32 = 1;
    memcpy(tcs.add(28) as *mut c_void, &val_32 as *const _ as *const c_void, 4); /* NSSA */
    memcpy(tcs.add(32) as *mut c_void, &(*op).entry as *const _ as *const c_void, 8); /* OENTRY */
    memset(tcs.add(40) as *mut c_void, 0, 24); /* AEP, OFSBASE, OGSBASE */
    val_32 = 0xFFFFFFFF;
    memcpy(tcs.add(64) as *mut c_void, &val_32 as *const _ as *const c_void, 4); /* FSLIMIT */
    memcpy(tcs.add(68) as *mut c_void, &val_32 as *const _ as *const c_void, 4); /* GSLIMIT */
    memset(tcs.add(72) as *mut c_void, 0, 4024); /* Reserved */
}

unsafe extern "C" fn do_encl_op_put_to_buf(op: *mut c_void) {
    let op2 = op as *mut encl_op_put_to_buf;

    memcpy(encl_buffer.as_mut_ptr() as *mut c_void, &(*op2).value as *const _ as *const c_void, 8);
}

unsafe extern "C" fn do_encl_op_get_from_buf(op: *mut c_void) {
    let op2 = op as *mut encl_op_get_from_buf;

    memcpy(&mut (*op2).value as *mut _ as *mut c_void, encl_buffer.as_ptr() as *const c_void, 8);
}

unsafe extern "C" fn do_encl_op_put_to_addr(_op: *mut c_void) {
    let op = _op as *mut encl_op_put_to_addr;

    memcpy((*op).addr as *mut c_void, &(*op).value as *const _ as *const c_void, 8);
}

unsafe extern "C" fn do_encl_op_get_from_addr(_op: *mut c_void) {
    let op = _op as *mut encl_op_get_from_addr;

    memcpy(&mut (*op).value as *mut _ as *mut c_void, (*op).addr as *const c_void, 8);
}

unsafe extern "C" fn do_encl_op_nop(_op: *mut c_void) {}

/*
 * Symbol placed at the start of the enclave image by the linker script.
 * Declare this extern symbol with visibility "hidden" to ensure the compiler
 * does not access it through the GOT and generates position-independent
 * addressing as __encl_base(%rip), so we can get the actual enclave base
 * during runtime.
 */
unsafe extern "C" {
    static __encl_base: u8;
}

type encl_op_t = unsafe extern "C" fn(*mut c_void);

static encl_op_array: [encl_op_t; ENCL_OP_MAX] = [
    do_encl_op_put_to_buf,
    do_encl_op_get_from_buf,
    do_encl_op_put_to_addr,
    do_encl_op_get_from_addr,
    do_encl_op_nop,
    do_encl_eaccept,
    do_encl_emodpe,
    do_encl_init_tcs_page,
];

#[no_mangle]
pub unsafe extern "C" fn encl_body(rdi: *mut c_void, _rsi: *mut c_void) {
    let header = rdi as *mut encl_op_header;
    let op: encl_op_t;

    if (*header).r#type >= ENCL_OP_MAX as u64 {
        return;
    }

    /*
     * The enclave base address needs to be added, as this call site
     * *cannot be* made rip-relative by the compiler, or fixed up by
     * any other possible means.
     */
    op = core::mem::transmute(
        (&__encl_base as *const u8 as u64)
            .wrapping_add(encl_op_array[(*header).r#type as usize] as usize as u64)
            as usize,
    );

    op(header as *mut c_void);
}
