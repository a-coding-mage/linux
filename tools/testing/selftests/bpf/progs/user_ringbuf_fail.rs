// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C includes:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sample {
    pub pid: i32,
    pub seq: i32,
    pub value: i64,
    pub comm: [i8; 16],
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
}

pub const BPF_MAP_TYPE_USER_RINGBUF: u32 = 27;
pub const BPF_MAP_TYPE_RINGBUF: u32 = 27;

// SEC("license")
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SEC(".maps")
#[unsafe(no_mangle)]
pub static mut user_ringbuf: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_USER_RINGBUF,
    max_entries: 4096,
};

// SEC(".maps")
#[unsafe(no_mangle)]
pub static mut ringbuf: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_RINGBUF,
    max_entries: 2,
};

static mut map_value: i32 = 0;

unsafe extern "C" {
    fn bpf_dynptr_data(dynptr: *mut bpf_dynptr, offset: u32, len: u32) -> *mut core::ffi::c_void;
    fn bpf_printk(fmt: *const i8, ...) -> i64;
    fn bpf_user_ringbuf_drain(
        map: *mut bpf_map_def,
        callback: unsafe extern "C" fn(*mut bpf_dynptr, *mut core::ffi::c_void) -> i64,
        ctx: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_ringbuf_discard_dynptr(dynptr: *mut bpf_dynptr, flags: u64);
    fn bpf_ringbuf_submit_dynptr(dynptr: *mut bpf_dynptr, flags: u64);
    fn bpf_dynptr_from_mem(
        data: *mut core::ffi::c_void,
        size: u32,
        flags: u64,
        dynptr: *mut bpf_dynptr,
    ) -> i64;
    fn bpf_ringbuf_reserve_dynptr(
        map: *mut bpf_map_def,
        size: u32,
        flags: u64,
        dynptr: *mut bpf_dynptr,
    ) -> i64;
}

unsafe extern "C" fn bad_access1(
    dynptr: *mut bpf_dynptr,
    _context: *mut core::ffi::c_void,
) -> i64 {
    let _sample: *const sample;

    _sample = unsafe {
        bpf_dynptr_data(
            dynptr.offset(-1),
            0,
            core::mem::size_of::<sample>() as u32,
        ) as *const sample
    };
    unsafe {
        bpf_printk(
            c"Was able to pass bad pointer %lx\n".as_ptr(),
            (dynptr as u64).wrapping_sub(1),
        );
    }

    0
}

/* A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
 * not be able to read before the pointer.
 */
// SEC("?raw_tp")
// __failure __msg("negative offset dynptr_ptr ptr")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_bad_access1(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            bad_access1,
            core::ptr::null_mut(),
            0,
        );
    }

    0
}

unsafe extern "C" fn bad_access2(
    dynptr: *mut bpf_dynptr,
    _context: *mut core::ffi::c_void,
) -> i64 {
    let _sample: *const sample;

    _sample = unsafe {
        bpf_dynptr_data(
            dynptr.offset(1),
            0,
            core::mem::size_of::<sample>() as u32,
        ) as *const sample
    };
    unsafe {
        bpf_printk(
            c"Was able to pass bad pointer %lx\n".as_ptr(),
            (dynptr as u64).wrapping_add(1),
        );
    }

    0
}

/* A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
 * not be able to read past the end of the pointer.
 */
// SEC("?raw_tp")
// __failure __msg("dereference of modified dynptr_ptr ptr")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_bad_access2(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            bad_access2,
            core::ptr::null_mut(),
            0,
        );
    }

    0
}

unsafe extern "C" fn write_forbidden(
    dynptr: *mut bpf_dynptr,
    _context: *mut core::ffi::c_void,
) -> i64 {
    unsafe {
        *(dynptr as *mut i64) = 0;
    }

    0
}

/* A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
 * not be able to write to that pointer.
 */
// SEC("?raw_tp")
// __failure __msg("invalid mem access 'dynptr_ptr'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_write_forbidden(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            write_forbidden,
            core::ptr::null_mut(),
            0,
        );
    }

    0
}

unsafe extern "C" fn null_context_write(
    _dynptr: *mut bpf_dynptr,
    context: *mut core::ffi::c_void,
) -> i64 {
    unsafe {
        *(context as *mut u64) = 0;
    }

    0
}

/* A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
 * not be able to write to that pointer.
 */
// SEC("?raw_tp")
// __failure __msg("invalid mem access 'scalar'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_null_context_write(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            null_context_write,
            core::ptr::null_mut(),
            0,
        );
    }

    0
}

unsafe extern "C" fn null_context_read(
    _dynptr: *mut bpf_dynptr,
    context: *mut core::ffi::c_void,
) -> i64 {
    let id: u64 = unsafe { *(context as *mut u64) };

    unsafe {
        bpf_printk(c"Read id %lu\n".as_ptr(), id);
    }

    0
}

/* A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
 * not be able to write to that pointer.
 */
// SEC("?raw_tp")
// __failure __msg("invalid mem access 'scalar'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_null_context_read(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            null_context_read,
            core::ptr::null_mut(),
            0,
        );
    }

    0
}

unsafe extern "C" fn try_discard_dynptr(
    dynptr: *mut bpf_dynptr,
    _context: *mut core::ffi::c_void,
) -> i64 {
    unsafe {
        bpf_ringbuf_discard_dynptr(dynptr, 0);
    }

    0
}

/* A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
 * not be able to read past the end of the pointer.
 */
// SEC("?raw_tp")
// __failure __msg("CONST_PTR_TO_DYNPTR cannot be released")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_discard_dynptr(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            try_discard_dynptr,
            core::ptr::null_mut(),
            0,
        );
    }

    0
}

unsafe extern "C" fn try_submit_dynptr(
    dynptr: *mut bpf_dynptr,
    _context: *mut core::ffi::c_void,
) -> i64 {
    unsafe {
        bpf_ringbuf_submit_dynptr(dynptr, 0);
    }

    0
}

/* A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
 * not be able to read past the end of the pointer.
 */
// SEC("?raw_tp")
// __failure __msg("CONST_PTR_TO_DYNPTR cannot be released")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_submit_dynptr(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            try_submit_dynptr,
            core::ptr::null_mut(),
            0,
        );
    }

    0
}

unsafe extern "C" fn invalid_drain_callback_return(
    _dynptr: *mut bpf_dynptr,
    _context: *mut core::ffi::c_void,
) -> i64 {
    2
}

/* A callback that accesses a dynptr in a bpf_user_ringbuf_drain callback should
 * not be able to write to that pointer.
 */
// SEC("?raw_tp")
// __failure __msg("At callback return the register R0 has ")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_invalid_return(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            invalid_drain_callback_return,
            core::ptr::null_mut(),
            0,
        );
    }

    0
}

unsafe extern "C" fn try_reinit_dynptr_mem(
    dynptr: *mut bpf_dynptr,
    _context: *mut core::ffi::c_void,
) -> i64 {
    unsafe {
        bpf_dynptr_from_mem(&raw mut map_value as *mut core::ffi::c_void, 4, 0, dynptr);
    }
    0
}

unsafe extern "C" fn try_reinit_dynptr_ringbuf(
    dynptr: *mut bpf_dynptr,
    _context: *mut core::ffi::c_void,
) -> i64 {
    unsafe {
        bpf_ringbuf_reserve_dynptr(&raw mut ringbuf, 8, 0, dynptr);
    }
    0
}

// SEC("?raw_tp")
// __failure __msg("Dynptr has to be an uninitialized dynptr")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_reinit_dynptr_mem(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            try_reinit_dynptr_mem,
            core::ptr::null_mut(),
            0,
        );
    }
    0
}

// SEC("?raw_tp")
// __failure __msg("Dynptr has to be an uninitialized dynptr")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_reinit_dynptr_ringbuf(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            try_reinit_dynptr_ringbuf,
            core::ptr::null_mut(),
            0,
        );
    }
    0
}

// __noinline
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_call_bpf_dynptr_data(dynptr: *mut bpf_dynptr) -> i64 {
    unsafe {
        bpf_dynptr_data(dynptr, 0xA, 0xA);
    }
    0
}

unsafe extern "C" fn callback_adjust_bpf_dynptr_reg_off(
    mut dynptr: *mut bpf_dynptr,
    _ctx: *mut core::ffi::c_void,
) -> i64 {
    dynptr = unsafe { dynptr.offset(1024) };
    unsafe {
        global_call_bpf_dynptr_data(dynptr);
    }
    0
}

// SEC("?raw_tp")
// __failure __msg("dereference of modified dynptr_ptr ptr R1 off=16384 disallowed")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_ringbuf_callback_const_ptr_to_dynptr_reg_off(
    _ctx: *mut core::ffi::c_void,
) -> i32 {
    unsafe {
        bpf_user_ringbuf_drain(
            &raw mut user_ringbuf,
            callback_adjust_bpf_dynptr_reg_off,
            core::ptr::null_mut(),
            0,
        );
    }
    0
}
