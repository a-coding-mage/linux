/*
 * Copyright (C) 2002 Jeff Dike (jdike@karaya.com)
 * Licensed under the GPL
 */

// Dependencies supplied by the surrounding translation unit:
// linux/string.h, asm/segment.h, and asm/ldt.h

extern "C" {
    pub static mut host_has_cmov: ::core::ffi::c_int;
}

#[repr(C)]
pub struct uml_tls_struct {
    pub tls: user_desc,
    pub flushed: ::core::ffi::c_uint,
    pub present: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct arch_thread {
    pub tls_array: [uml_tls_struct; GDT_ENTRY_TLS_ENTRIES as usize],
    pub debugregs: [::core::ffi::c_ulong; 8],
    pub debugregs_seq: ::core::ffi::c_int,
    pub faultinfo: faultinfo,
}

// C bit-fields are represented as integer fields; callers must preserve their
// one-bit intent when assigning values.

#[macro_export]
macro_rules! INIT_ARCH_THREAD {
    () => {{
        arch_thread {
            tls_array: [uml_tls_struct {
                tls: unsafe { ::core::mem::zeroed() },
                flushed: 0,
                present: 0,
            }; GDT_ENTRY_TLS_ENTRIES as usize],
            debugregs: [0; 8],
            debugregs_seq: 0,
            faultinfo: unsafe { ::core::mem::zeroed() },
        }
    }};
}

pub const STACKSLOTS_PER_LINE: usize = 8;

pub unsafe fn arch_flush_thread(thread: *mut arch_thread) {
    /* Clear any TLS still hanging */
    ::core::ptr::write_bytes(
        ::core::ptr::addr_of_mut!((*thread).tls_array).cast::<u8>(),
        0,
        ::core::mem::size_of_val(&(*thread).tls_array),
    );
}

pub unsafe fn arch_copy_thread(from: *const arch_thread, to: *mut arch_thread) {
    ::core::ptr::copy_nonoverlapping(
        ::core::ptr::addr_of!((*from).tls_array).cast::<u8>(),
        ::core::ptr::addr_of_mut!((*to).tls_array).cast::<u8>(),
        ::core::mem::size_of_val(&(*from).tls_array),
    );
}

pub unsafe fn current_sp() -> *mut ::core::ffi::c_void {
    let mut sp: *mut ::core::ffi::c_void;
    ::core::arch::asm!("movl %esp, {0}", out(reg) sp);
    sp
}

pub unsafe fn current_bp() -> ::core::ffi::c_ulong {
    let mut bp: ::core::ffi::c_ulong;
    ::core::arch::asm!("movl %ebp, {0}", out(reg) bp);
    bp
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
