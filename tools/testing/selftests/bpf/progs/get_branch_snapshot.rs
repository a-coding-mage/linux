// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies translated as external Rust dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

unsafe extern "C" {
    fn bpf_get_branch_snapshot(entries: *mut perf_branch_entry, size: u32, flags: u64) -> i64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut test1_hits: u64 = 0;
#[unsafe(no_mangle)]
pub static mut address_low: u64 = 0;
#[unsafe(no_mangle)]
pub static mut address_high: u64 = 0;
#[unsafe(no_mangle)]
pub static mut wasted_entries: i32 = 0;
#[unsafe(no_mangle)]
pub static mut total_entries: i64 = 0;

pub const ENTRY_CNT: usize = 32;

#[unsafe(no_mangle)]
pub static mut entries: [perf_branch_entry; ENTRY_CNT] = unsafe { core::mem::zeroed() };

#[inline(always)]
unsafe fn gbs_in_range(val: u64) -> bool {
    unsafe { (val >= address_low) && (val < address_high) }
}

#[unsafe(link_section = "fexit/bpf_testmod_loop_test")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test1(n: i32, ret: i32) -> i32 {
    let mut i: i64;

    unsafe {
        total_entries = bpf_get_branch_snapshot(
            core::ptr::addr_of_mut!(entries) as *mut perf_branch_entry,
            core::mem::size_of::<[perf_branch_entry; ENTRY_CNT]>() as u32,
            0,
        );
        total_entries /= core::mem::size_of::<perf_branch_entry>() as i64;

        i = 0;
        while i < ENTRY_CNT as i64 {
            if i >= total_entries {
                break;
            }
            if gbs_in_range(entries[i as usize].from) && gbs_in_range(entries[i as usize].to) {
                test1_hits = test1_hits.wrapping_add(1);
            } else if test1_hits == 0 {
                wasted_entries = wasted_entries.wrapping_add(1);
            }
            i += 1;
        }
    }
    0
}
