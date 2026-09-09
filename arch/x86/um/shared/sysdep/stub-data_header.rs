/* SPDX-License-Identifier: GPL-2.0 */

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct stub_data_arch {
    pub sync: i32,
    // Dependency: UM_KERN_GDT_ENTRY_TLS_ENTRIES and user_desc are supplied by
    // the corresponding generated/asm-offsets.h and asm/ldt.h translations.
    pub tls: [user_desc; UM_KERN_GDT_ENTRY_TLS_ENTRIES],
}

#[cfg(not(target_arch = "x86"))]
pub const STUB_SYNC_FS_BASE: i32 = 1 << 0;
#[cfg(not(target_arch = "x86"))]
pub const STUB_SYNC_GS_BASE: i32 = 1 << 1;

#[cfg(not(target_arch = "x86"))]
#[repr(C)]
pub struct stub_data_arch {
    pub sync: i32,
    pub fs_base: c_ulong,
    pub gs_base: c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
