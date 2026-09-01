// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut unique_tgid_cnt: __u32 = 0;
#[no_mangle]
pub static mut address: uintptr_t = 0;
#[no_mangle]
pub static mut offset: uintptr_t = 0;
#[no_mangle]
pub static mut last_tgid: __u32 = 0;
#[no_mangle]
pub static mut pid: __u32 = 0;
#[no_mangle]
pub static mut page_shift: __u32 = 0;

#[no_mangle]
#[link_section = "iter/task_vma"]
pub unsafe extern "C" fn get_vma_offset(ctx: *mut bpf_iter__task_vma) -> ::core::ffi::c_int {
    let vma: *mut vm_area_struct = unsafe { (*ctx).vma };
    let seq: *mut seq_file = unsafe { (*(*ctx).meta).seq };
    let task: *mut task_struct = unsafe { (*ctx).task };

    if task == ::core::ptr::null_mut() || vma == ::core::ptr::null_mut() {
        return 0;
    }

    if unsafe { last_tgid != (*task).tgid } {
        unsafe {
            unique_tgid_cnt = unique_tgid_cnt.wrapping_add(1);
        }
    }
    unsafe {
        last_tgid = (*task).tgid;
    }

    if unsafe { (*task).tgid != pid } {
        return 0;
    }

    if unsafe { (*vma).vm_start <= address && (*vma).vm_end > address } {
        unsafe {
            offset = address
                .wrapping_sub((*vma).vm_start)
                .wrapping_add((*vma).vm_pgoff.wrapping_shl(page_shift));
            BPF_SEQ_PRINTF(seq, b"OK\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
