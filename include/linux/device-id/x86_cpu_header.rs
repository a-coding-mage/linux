/* SPDX-License-Identifier: GPL-2.0 */

// Under __KERNEL__, the C header includes linux/types.h and defines:
// typedef unsigned long kernel_ulong_t;
pub type kernel_ulong_t = usize;

/* Wild cards for x86_cpu_id::vendor, family, model and feature */
pub const X86_VENDOR_ANY: u16 = 0xffff;
pub const X86_FAMILY_ANY: u16 = 0;
pub const X86_MODEL_ANY: u16 = 0;
pub const X86_STEPPING_ANY: u16 = 0;
pub const X86_STEP_MIN: u16 = 0;
pub const X86_STEP_MAX: u16 = 0xf;
pub const X86_PLATFORM_ANY: u8 = 0x0;
pub const X86_FEATURE_ANY: u16 = 0; /* Same as FPU, you can't test for that */
pub const X86_CPU_TYPE_ANY: u8 = 0;

/*
 * Match x86 CPUs for CPU specific drivers.
 * See documentation of "x86_match_cpu" for details.
 */

/*
 * MODULE_DEVICE_TABLE expects this struct to be called x86cpu_device_id.
 * Although gcc seems to ignore this error, clang fails without this define.
 */
#[repr(C)]
pub struct x86_cpu_id {
    pub vendor: __u16,
    pub family: __u16,
    pub model: __u16,
    pub steppings: __u16,
    pub feature: __u16, /* bit index */
    /* Solely for kernel-internal use: DO NOT EXPORT to userspace! */
    pub flags: __u16,
    pub platform_mask: __u8,
    pub type_: __u8,
    pub driver_data: kernel_ulong_t,
}

pub type x86cpu_device_id = x86_cpu_id;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
