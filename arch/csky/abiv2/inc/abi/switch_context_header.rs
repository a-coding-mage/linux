/* SPDX-License-Identifier: GPL-2.0 */

// The CONFIG_CPU_HAS_HILO build-time condition is represented by the
// corresponding Rust cfg feature.

#[repr(C)]
pub struct switch_stack {
    #[cfg(CONFIG_CPU_HAS_HILO)]
    pub rhi: kernel::ffi::c_ulong,
    #[cfg(CONFIG_CPU_HAS_HILO)]
    pub rlo: kernel::ffi::c_ulong,
    #[cfg(CONFIG_CPU_HAS_HILO)]
    pub cr14: kernel::ffi::c_ulong,
    #[cfg(CONFIG_CPU_HAS_HILO)]
    pub pad: kernel::ffi::c_ulong,

    pub r4: kernel::ffi::c_ulong,
    pub r5: kernel::ffi::c_ulong,
    pub r6: kernel::ffi::c_ulong,
    pub r7: kernel::ffi::c_ulong,
    pub r8: kernel::ffi::c_ulong,
    pub r9: kernel::ffi::c_ulong,
    pub r10: kernel::ffi::c_ulong,
    pub r11: kernel::ffi::c_ulong,

    pub r15: kernel::ffi::c_ulong,
    pub r16: kernel::ffi::c_ulong,
    pub r17: kernel::ffi::c_ulong,
    pub r26: kernel::ffi::c_ulong,
    pub r27: kernel::ffi::c_ulong,
    pub r28: kernel::ffi::c_ulong,
    pub r29: kernel::ffi::c_ulong,
    pub r30: kernel::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
