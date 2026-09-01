// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

// C dependencies: <stdbool.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>

pub const BPF_ANY: u64 = 0;
pub const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct s {
    pub a: i32,
    pub b: i64,
}

/* .data section */
#[unsafe(no_mangle)]
pub static mut in1: i32 = -1;
#[unsafe(no_mangle)]
pub static mut in2: i64 = -1;

/* .bss section */
#[unsafe(no_mangle)]
pub static mut in3: i8 = b'\0' as i8;
#[unsafe(no_mangle)]
#[repr(align(64))]
pub static mut in4: i64 = 0;
#[unsafe(no_mangle)]
pub static mut in5: s = s { a: 0, b: 0 };

/* .rodata section */
#[repr(C)]
pub struct In {
    pub in6: i32,
}

#[unsafe(no_mangle)]
pub static in_: In = In { in6: 0 };

/* .data section */
#[unsafe(no_mangle)]
pub static mut out1: i32 = -1;
#[unsafe(no_mangle)]
pub static mut out2: i64 = -1;

/* .bss section */
#[unsafe(no_mangle)]
pub static mut out3: i8 = 0;
#[unsafe(no_mangle)]
pub static mut out4: i64 = 0;
#[unsafe(no_mangle)]
pub static mut out6: i32 = 0;

unsafe extern "C" {
    #[link_name = "CONFIG_BPF_SYSCALL"]
    pub static CONFIG_BPF_SYSCALL: bool;
    #[link_name = "LINUX_KERNEL_VERSION"]
    pub static LINUX_KERNEL_VERSION: i32;
}

#[unsafe(no_mangle)]
pub static mut bpf_syscall: bool = false;
#[unsafe(no_mangle)]
pub static mut kern_ver: i32 = 0;

#[unsafe(no_mangle)]
pub static mut out5: s = s { a: 0, b: 0 };

#[unsafe(link_section = ".rodata.dyn")]
#[unsafe(no_mangle)]
pub static in_dynarr_sz: i32 = 0;
#[unsafe(link_section = ".rodata.dyn")]
#[unsafe(no_mangle)]
pub static in_dynarr: [i32; 4] = [-1, -2, -3, -4];

#[unsafe(link_section = ".data.dyn")]
#[unsafe(no_mangle)]
pub static mut out_dynarr: [i32; 4] = [1, 2, 3, 4];

#[unsafe(link_section = ".data.read_mostly")]
#[unsafe(no_mangle)]
pub static mut read_mostly_var: i32 = 0;
#[unsafe(no_mangle)]
pub static mut out_mostly_var: i32 = 0;

#[unsafe(no_mangle)]
pub static mut huge_arr: [i8; 16 * 1024 * 1024] = [0; 16 * 1024 * 1024];

/* non-mmapable custom .data section */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_value {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[unsafe(link_section = ".data.non_mmapable")]
#[unsafe(no_mangle)]
pub static mut zero_key: i32 = 0;
#[unsafe(link_section = ".data.non_mmapable")]
static mut zero_value: my_value = my_value { x: 0, y: 0, z: 0 };

// C BPF map definition used __uint/__type macros in SEC(".maps").
#[repr(C)]
pub struct my_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static my_map: my_map_def = my_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<my_value>() as u32,
    max_entries: 1,
};

unsafe extern "C" {
    pub fn bpf_map_update_elem(
        map: *const my_map_def,
        key: *const i32,
        value: *const my_value,
        flags: u64,
    ) -> i64;
}

#[unsafe(link_section = "raw_tp/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handler(ctx: *const core::ffi::c_void) -> i32 {
    let mut i: i32;

    unsafe {
        out1 = in1;
        out2 = in2;
        out3 = in3;
        out4 = in4;
        out5 = in5;
        out6 = in_.in6;

        bpf_syscall = CONFIG_BPF_SYSCALL;
        kern_ver = LINUX_KERNEL_VERSION;

        i = 0;
        while i < in_dynarr_sz {
            out_dynarr[i as usize] = in_dynarr[i as usize];
            i += 1;
        }

        out_mostly_var = read_mostly_var;

        huge_arr[core::mem::size_of_val(&huge_arr) - 1] = 123;

        /* make sure zero_key and zero_value are not optimized out */
        bpf_map_update_elem(
            &raw const my_map,
            &raw const zero_key,
            &raw const zero_value,
            BPF_ANY,
        );
    }

    let _ = ctx;
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
