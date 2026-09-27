/* SPDX-License-Identifier: GPL-2.0 */
// Original header guard: _ARM_KEXEC_INTERNAL_H

#[repr(C)]
pub struct kexec_relocate_data {
    pub kexec_start_address: kernel::ffi::c_ulong,
    pub kexec_indirection_page: kernel::ffi::c_ulong,
    pub kexec_mach_type: kernel::ffi::c_ulong,
    pub kexec_r2: kernel::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
