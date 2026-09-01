// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u64 = u64;

unsafe extern "C" {
    fn bpf_core_read(dst: *mut core::ffi::c_void, sz: u32, src: *const core::ffi::c_void) -> i64;
}

// CO-RE helper supplied by bpf_core_read.h in the original C source.
macro_rules! bpf_core_field_size {
    ($field:expr) => {
        core::mem::size_of_val(&$field) as u32
    };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* fields of exactly the same size */
#[repr(C)]
pub struct test_struct___samesize {
    pub ptr: *mut core::ffi::c_void,
    pub val1: u64,
    pub val2: u32,
    pub val3: u16,
    pub val4: u8,
}
// original C type has __attribute((preserve_access_index))

/* unsigned fields that have to be downsized by libbpf */
#[repr(C)]
pub struct test_struct___downsize {
    pub ptr: *mut core::ffi::c_void,
    pub val1: core::ffi::c_ulong,
    pub val2: core::ffi::c_ulong,
    pub val3: core::ffi::c_ulong,
    pub val4: core::ffi::c_ulong,
    /* total sz: 40 */
}
// original C type has __attribute__((preserve_access_index))

/* fields with signed integers of wrong size, should be rejected */
#[repr(C)]
pub struct test_struct___signed {
    pub ptr: *mut core::ffi::c_void,
    pub val1: core::ffi::c_long,
    pub val2: core::ffi::c_long,
    pub val3: core::ffi::c_long,
    pub val4: core::ffi::c_long,
}
// original C type has __attribute((preserve_access_index))

/* real layout and sizes according to test's (32-bit) BTF */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_struct___real {
    pub ptr: u32, /* can't use `void *`, it is always 8 byte in BPF target */
    pub val2: u32,
    pub val1: u64,
    pub val3: u16,
    pub val4: u8,
    pub _pad: u8,
    /* total sz: 20 */
}

#[unsafe(no_mangle)]
pub static mut input: test_struct___real = test_struct___real {
    ptr: 0x01020304,
    val1: 0x1020304050607080,
    val2: 0x0a0b0c0d,
    val3: 0xfeed,
    val4: 0xb9,
    _pad: 0xff, /* make sure no accidental zeros are present */
};

#[unsafe(no_mangle)]
pub static mut ptr_samesized: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val1_samesized: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val2_samesized: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val3_samesized: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val4_samesized: u64 = 0;
#[unsafe(no_mangle)]
pub static mut output_samesized: test_struct___real = test_struct___real {
    ptr: 0,
    val2: 0,
    val1: 0,
    val3: 0,
    val4: 0,
    _pad: 0,
};

#[unsafe(no_mangle)]
pub static mut ptr_downsized: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val1_downsized: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val2_downsized: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val3_downsized: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val4_downsized: u64 = 0;
#[unsafe(no_mangle)]
pub static mut output_downsized: test_struct___real = test_struct___real {
    ptr: 0,
    val2: 0,
    val1: 0,
    val3: 0,
    val4: 0,
    _pad: 0,
};

#[unsafe(no_mangle)]
pub static mut ptr_probed: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val1_probed: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val2_probed: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val3_probed: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val4_probed: u64 = 0;

#[unsafe(no_mangle)]
pub static mut ptr_signed: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val1_signed: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val2_signed: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val3_signed: u64 = 0;
#[unsafe(no_mangle)]
pub static mut val4_signed: u64 = 0;
#[unsafe(no_mangle)]
pub static mut output_signed: test_struct___real = test_struct___real {
    ptr: 0,
    val2: 0,
    val1: 0,
    val3: 0,
    val4: 0,
    _pad: 0,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_exit")]
pub unsafe extern "C" fn handle_samesize(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let in_: *mut test_struct___samesize =
        core::ptr::addr_of_mut!(input).cast::<test_struct___samesize>();
    let out: *mut test_struct___samesize =
        core::ptr::addr_of_mut!(output_samesized).cast::<test_struct___samesize>();

    ptr_samesized = (*in_).ptr as u64;
    val1_samesized = (*in_).val1;
    val2_samesized = (*in_).val2 as u64;
    val3_samesized = (*in_).val3 as u64;
    val4_samesized = (*in_).val4 as u64;

    (*out).ptr = (*in_).ptr;
    (*out).val1 = (*in_).val1;
    (*out).val2 = (*in_).val2;
    (*out).val3 = (*in_).val3;
    (*out).val4 = (*in_).val4;

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_exit")]
pub unsafe extern "C" fn handle_downsize(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let in_: *mut test_struct___downsize =
        core::ptr::addr_of_mut!(input).cast::<test_struct___downsize>();
    let out: *mut test_struct___downsize =
        core::ptr::addr_of_mut!(output_downsized).cast::<test_struct___downsize>();

    ptr_downsized = (*in_).ptr as u64;
    val1_downsized = (*in_).val1 as u64;
    val2_downsized = (*in_).val2 as u64;
    val3_downsized = (*in_).val3 as u64;
    val4_downsized = (*in_).val4 as u64;

    (*out).ptr = (*in_).ptr;
    (*out).val1 = (*in_).val1;
    (*out).val2 = (*in_).val2;
    (*out).val3 = (*in_).val3;
    (*out).val4 = (*in_).val4;

    0
}

// Original C:
// #if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
// #define bpf_core_read_int bpf_core_read
// #else
// #define bpf_core_read_int(dst, sz, src) ({ \
//     /* Prevent "subtraction from stack pointer prohibited" */ \
//     volatile long __off = sizeof(*dst) - (sz); \
//     bpf_core_read((char *)(dst) + __off, sz, src); \
// })
// #endif
#[cfg(target_endian = "little")]
unsafe fn bpf_core_read_int(
    dst: *mut core::ffi::c_void,
    sz: u32,
    src: *const core::ffi::c_void,
) -> i64 {
    bpf_core_read(dst, sz, src)
}

#[cfg(target_endian = "big")]
unsafe fn bpf_core_read_int(
    dst: *mut __u64,
    sz: u32,
    src: *const core::ffi::c_void,
) -> i64 {
    /* Prevent "subtraction from stack pointer prohibited" */
    let __off: volatile_long = volatile_long {
        v: (core::mem::size_of_val(&*dst) as core::ffi::c_long) - (sz as core::ffi::c_long),
    };
    bpf_core_read(
        (dst as *mut u8).offset(core::ptr::read_volatile(core::ptr::addr_of!(__off.v)) as isize)
            as *mut core::ffi::c_void,
        sz,
        src,
    )
}

#[cfg(target_endian = "big")]
#[repr(C)]
struct volatile_long {
    v: core::ffi::c_long,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_enter")]
pub unsafe extern "C" fn handle_probed(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let in_: *mut test_struct___downsize =
        core::ptr::addr_of_mut!(input).cast::<test_struct___downsize>();
    let mut tmp: __u64;

    tmp = 0;
    bpf_core_read_int(
        (&mut tmp as *mut __u64).cast::<core::ffi::c_void>(),
        bpf_core_field_size!((*in_).ptr),
        core::ptr::addr_of!((*in_).ptr).cast::<core::ffi::c_void>(),
    );
    ptr_probed = tmp;

    tmp = 0;
    bpf_core_read_int(
        (&mut tmp as *mut __u64).cast::<core::ffi::c_void>(),
        bpf_core_field_size!((*in_).val1),
        core::ptr::addr_of!((*in_).val1).cast::<core::ffi::c_void>(),
    );
    val1_probed = tmp;

    tmp = 0;
    bpf_core_read_int(
        (&mut tmp as *mut __u64).cast::<core::ffi::c_void>(),
        bpf_core_field_size!((*in_).val2),
        core::ptr::addr_of!((*in_).val2).cast::<core::ffi::c_void>(),
    );
    val2_probed = tmp;

    tmp = 0;
    bpf_core_read_int(
        (&mut tmp as *mut __u64).cast::<core::ffi::c_void>(),
        bpf_core_field_size!((*in_).val3),
        core::ptr::addr_of!((*in_).val3).cast::<core::ffi::c_void>(),
    );
    val3_probed = tmp;

    tmp = 0;
    bpf_core_read_int(
        (&mut tmp as *mut __u64).cast::<core::ffi::c_void>(),
        bpf_core_field_size!((*in_).val4),
        core::ptr::addr_of!((*in_).val4).cast::<core::ffi::c_void>(),
    );
    val4_probed = tmp;

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_enter")]
pub unsafe extern "C" fn handle_signed(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let in_: *mut test_struct___signed =
        core::ptr::addr_of_mut!(input).cast::<test_struct___signed>();
    let out: *mut test_struct___signed =
        core::ptr::addr_of_mut!(output_signed).cast::<test_struct___signed>();

    val2_signed = (*in_).val2 as u64;
    val3_signed = (*in_).val3 as u64;
    val4_signed = (*in_).val4 as u64;

    (*out).val2 = (*in_).val2;
    (*out).val3 = (*in_).val3;
    (*out).val4 = (*in_).val4;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
