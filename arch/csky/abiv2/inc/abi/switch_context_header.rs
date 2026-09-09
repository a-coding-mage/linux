/* SPDX-License-Identifier: GPL-2.0 */

// The CONFIG_CPU_HAS_HILO build-time condition is represented by the
// corresponding Rust cfg feature.

#[repr(C)]
pub struct switch_stack {
    #[cfg(feature = "CONFIG_CPU_HAS_HILO")]
    pub rhi: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_CPU_HAS_HILO")]
    pub rlo: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_CPU_HAS_HILO")]
    pub cr14: core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_CPU_HAS_HILO")]
    pub pad: core::ffi::c_ulong,

    pub r4: core::ffi::c_ulong,
    pub r5: core::ffi::c_ulong,
    pub r6: core::ffi::c_ulong,
    pub r7: core::ffi::c_ulong,
    pub r8: core::ffi::c_ulong,
    pub r9: core::ffi::c_ulong,
    pub r10: core::ffi::c_ulong,
    pub r11: core::ffi::c_ulong,

    pub r15: core::ffi::c_ulong,
    pub r16: core::ffi::c_ulong,
    pub r17: core::ffi::c_ulong,
    pub r26: core::ffi::c_ulong,
    pub r27: core::ffi::c_ulong,
    pub r28: core::ffi::c_ulong,
    pub r29: core::ffi::c_ulong,
    pub r30: core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
