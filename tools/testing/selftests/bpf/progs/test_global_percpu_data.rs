// SPDX-License-Identifier: GPL-2.0
// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::ffi::c_void;

type __u32 = u32;
type u64 = u64;

unsafe extern "C" {
    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_strncmp(s1: *const i8, s1_sz: u32, s2: *const i8) -> i32;
    fn bpf_snprintf(
        str_: *mut i8,
        str_size: u32,
        fmt: *const i8,
        data: *const u64,
        data_len: u32,
    ) -> i32;
    fn bpf_printk(fmt: *const i8, ...) -> i32;
}

#[repr(C)]
pub struct bpf_iter__bpf_map_elem {
    pub value: *mut c_void,
}

#[repr(C)]
pub struct StructData {
    pub set: i8,
    pub i: i32,
    pub nums: [i32; 7],
}

/* Used for testing map name. */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".percpu.looooooooong")]
pub static mut loong: i32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".data.percpu")]
pub static mut data3: i32 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".percpu.data")]
pub static mut data2: i32 = 0;

#[unsafe(no_mangle)]
pub static mut run: i32 = 0;

/* cpu_id as array to verify map value resizing. */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".percpu")]
pub static mut cpu_id: [i32; 1] = [0; 1];

#[unsafe(no_mangle)]
#[unsafe(link_section = ".percpu")]
pub static mut data: i32 = -1;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".percpu")]
pub static mut nums: [i32; 7] = [0; 7];

#[unsafe(no_mangle)]
#[unsafe(link_section = ".percpu")]
pub static mut set: bool = false;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".percpu")]
pub static mut struct_data: StructData = StructData {
    set: 0,
    i: -1,
    nums: [0; 7],
};

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/task_rename")]
// __auxiliary
pub unsafe extern "C" fn update_percpu_data(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    unsafe {
        struct_data.nums[6] = 0xc0de;
        struct_data.set = 1;
        struct_data.i = 1;
        nums[6] = 0xc0de;
        data = 1;
        run += 1;
        set = true;
        cpu_id[0] = bpf_get_smp_processor_id() as i32;
    }
    0
}

#[unsafe(link_section = ".percpu.fmt")]
static fmt: [i8; 9] = *b"data %d\n\0";

#[unsafe(no_mangle)]
#[unsafe(link_section = "?kprobe")]
// __failure
// __msg("R{{[0-9]+}} points to percpu_array map which cannot be used as const string")
pub unsafe extern "C" fn verifier_strncmp(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    unsafe { bpf_strncmp(c"test".as_ptr(), 5, fmt.as_ptr()) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?kprobe")]
// __failure
// __msg("R{{[0-9]+}} points to percpu_array map which cannot be used as const string")
pub unsafe extern "C" fn verifier_snprintf(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    unsafe {
        let args: [u64; 1] = [data as u64];
        let mut buf: [i8; 128] = [0; 128];
        let len: i32;

        len = bpf_snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of_val(&buf) as u32,
            fmt.as_ptr(),
            args.as_ptr(),
            core::mem::size_of_val(&args) as u32,
        );
        if len > 0 {
            bpf_printk(c"snprintf: %s\n".as_ptr(), buf.as_ptr());
        }
    }
    0
}

#[unsafe(no_mangle)]
pub static num_cpus: __u32 = 0;

#[unsafe(no_mangle)]
pub static num_off: i32 = 0;

#[unsafe(no_mangle)]
pub static elem_sz: i32 = 0;

#[unsafe(no_mangle)]
pub static mut sum: __u32 = 0;

#[unsafe(no_mangle)]
pub static mut run_iter: bool = false;

#[unsafe(no_mangle)]
#[unsafe(link_section = "iter/bpf_map_elem")]
// __auxiliary
pub unsafe extern "C" fn dump_percpu_data(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    unsafe {
        let mut pptr: *mut u8 = (*ctx).value as *mut u8;
        let mut i: i32;

        if pptr.is_null() {
            return 0;
        }

        run_iter = true;

        i = 0;
        while i < num_cpus as i32 {
            sum += *(pptr.add(num_off as usize) as *mut i32) as __u32;
            pptr = pptr.add(elem_sz as usize);
            i += 1;
        }
    }
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [i8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
