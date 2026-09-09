/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2025 Intel Corporation */

// C dependencies: linux/stddef.h, linux/stringify.h, linux/types.h, and
// icp_qat_fw_init_admin.h are supplied by the surrounding translation unit.

macro_rules! PM_INFO_MEMBER_OFF {
    ($member:ident) => {
        ::core::mem::offset_of!(icp_qat_fw_init_admin_pm_info, $member)
            / ::core::mem::size_of::<u32>()
    };
}

macro_rules! __stringify {
    ($value:ident) => {
        stringify!($value)
    };
}

macro_rules! PM_INFO_REGSET_ENTRY_MASK {
    ($reg:ident, $field:ident, $mask:expr) => {
        pm_status_row {
            reg_offset: PM_INFO_MEMBER_OFF!($reg) as i32,
            key: __stringify!($field),
            field_mask: $mask,
        }
    };
}

macro_rules! PM_INFO_REGSET_ENTRY32 {
    ($reg:ident, $field:ident) => {
        PM_INFO_REGSET_ENTRY_MASK!($reg, $field, u32::MAX)
    };
}

#[repr(C)]
pub struct pm_status_row {
    pub reg_offset: i32,
    pub field_mask: u32,
    pub key: *const ::core::ffi::c_char,
}

unsafe extern "C" {
    pub fn adf_pm_scnprint_table_upper_keys(
        buff: *mut ::core::ffi::c_char,
        table: *const pm_status_row,
        pm_info_regs: *mut u32,
        buff_size: usize,
        table_len: i32,
    ) -> i32;

    pub fn adf_pm_scnprint_table_lower_keys(
        buff: *mut ::core::ffi::c_char,
        table: *const pm_status_row,
        pm_info_regs: *mut u32,
        buff_size: usize,
        table_len: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
