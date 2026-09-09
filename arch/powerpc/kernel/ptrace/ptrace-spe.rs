// SPDX-License-Identifier: GPL-2.0-or-later

// For get_evrregs/set_evrregs functions `data` has the following layout:
//
// struct {
//   u32 evr[32];
//   u64 acc;
//   u32 spefscr;
// }

use core::mem::{offset_of, size_of};

use crate::ptrace_decl::{membuf, task_struct, user_regset};

extern "C" {
    fn flush_spe_to_thread(target: *mut task_struct);
    fn membuf_write(to: *mut membuf, from: *const core::ffi::c_void, len: usize) -> i32;
    fn user_regset_copyin(
        pos: *mut u32,
        count: *mut u32,
        kbuf: *mut *const core::ffi::c_void,
        ubuf: *mut *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        start: usize,
        end: isize,
    ) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn evr_active(
    target: *mut task_struct,
    regset: *const user_regset,
) -> i32 {
    flush_spe_to_thread(target);
    if (*target).thread.used_spe {
        (*regset).n as i32
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn evr_get(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut to: membuf,
) -> i32 {
    flush_spe_to_thread(target);

    membuf_write(
        &mut to,
        &(*target).thread.evr as *const _ as *const core::ffi::c_void,
        size_of_val(&(*target).thread.evr),
    );

    assert!(offset_of!(crate::ptrace_decl::thread_struct, acc) + size_of::<u64>()
        == offset_of!(crate::ptrace_decl::thread_struct, spefscr));

    membuf_write(
        &mut to,
        &(*target).thread.acc as *const _ as *const core::ffi::c_void,
        size_of::<u64>() + size_of::<u32>(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn evr_set(
    target: *mut task_struct,
    _regset: *const user_regset,
    pos: *mut u32,
    count: *mut u32,
    kbuf: *mut *const core::ffi::c_void,
    ubuf: *mut *const core::ffi::c_void,
) -> i32 {
    flush_spe_to_thread(target);

    let mut ret = user_regset_copyin(
        pos,
        count,
        kbuf,
        ubuf,
        &mut (*target).thread.evr as *mut _ as *mut core::ffi::c_void,
        0,
        size_of_val(&(*target).thread.evr) as isize,
    );

    assert!(offset_of!(crate::ptrace_decl::thread_struct, acc) + size_of::<u64>()
        == offset_of!(crate::ptrace_decl::thread_struct, spefscr));

    if ret == 0 {
        ret = user_regset_copyin(
            pos,
            count,
            kbuf,
            ubuf,
            &mut (*target).thread.acc as *mut _ as *mut core::ffi::c_void,
            size_of_val(&(*target).thread.evr),
            -1,
        );
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
