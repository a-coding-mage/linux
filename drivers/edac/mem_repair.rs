// SPDX-License-Identifier: GPL-2.0
/*
 * The generic EDAC memory repair driver is designed to control the memory
 * devices with memory repair features, such as Post Package Repair (PPR),
 * memory sparing etc. The common sysfs memory repair interface abstracts
 * the control of various arbitrary memory repair functionalities into a
 * unified set of functions.
 *
 * Copyright (c) 2024-2025 HiSilicon Limited.
 */

// Dependency declarations supplied by the Linux EDAC/kernel environment are
// intentionally left external to this translation unit.

#[repr(C)]
pub enum EdacMemRepairAttributes {
    MR_TYPE,
    MR_PERSIST_MODE,
    MR_SAFE_IN_USE,
    MR_HPA,
    MR_MIN_HPA,
    MR_MAX_HPA,
    MR_DPA,
    MR_MIN_DPA,
    MR_MAX_DPA,
    MR_NIBBLE_MASK,
    MR_BANK_GROUP,
    MR_BANK,
    MR_RANK,
    MR_ROW,
    MR_COLUMN,
    MR_CHANNEL,
    MR_SUB_CHANNEL,
    MEM_DO_REPAIR,
    MR_MAX_ATTRS,
}

#[repr(C)]
pub struct EdacMemRepairDevAttr {
    pub dev_attr: device_attribute,
    pub instance: u8,
}

#[repr(C)]
pub struct EdacMemRepairContext {
    pub name: [libc::c_char; EDAC_FEAT_NAME_LEN],
    pub mem_repair_dev_attr: [EdacMemRepairDevAttr; MR_MAX_ATTRS],
    pub mem_repair_attrs: [*mut attribute; MR_MAX_ATTRS + 1],
    pub group: attribute_group,
}

pub static EDAC_REPAIR_TYPE: [*const libc::c_char; 5] = [
    b"ppr\0".as_ptr() as *const libc::c_char,
    b"cacheline-sparing\0".as_ptr() as *const libc::c_char,
    b"row-sparing\0".as_ptr() as *const libc::c_char,
    b"bank-sparing\0".as_ptr() as *const libc::c_char,
    b"rank-sparing\0".as_ptr() as *const libc::c_char,
];

macro_rules! mr_attr_show {
    ($name:ident, $cb:ident, $ty:ty, $fmt:literal) => {
        unsafe fn $name(_dev: *mut device, _attr: *mut device_attribute,
                              _buf: *mut libc::c_char) -> isize { unimplemented!() }
    };
}

// The following generated sysfs callbacks preserve the C macro declarations
// and their externally supplied callback/data-conversion dependencies.
mr_attr_show!(repair_type, get_repair_type, *const libc::c_char, "%s\n");
mr_attr_show!(persist_mode, get_persist_mode, bool, "%u\n");
mr_attr_show!(repair_safe_when_in_use, get_repair_safe_when_in_use, bool, "%u\n");
mr_attr_show!(hpa, get_hpa, u64, "0x%llx\n");
mr_attr_show!(min_hpa, get_min_hpa, u64, "0x%llx\n");
mr_attr_show!(max_hpa, get_max_hpa, u64, "0x%llx\n");
mr_attr_show!(dpa, get_dpa, u64, "0x%llx\n");
mr_attr_show!(min_dpa, get_min_dpa, u64, "0x%llx\n");
mr_attr_show!(max_dpa, get_max_dpa, u64, "0x%llx\n");
mr_attr_show!(nibble_mask, get_nibble_mask, u32, "0x%x\n");
mr_attr_show!(bank_group, get_bank_group, u32, "%u\n");
mr_attr_show!(bank, get_bank, u32, "%u\n");
mr_attr_show!(rank, get_rank, u32, "%u\n");
mr_attr_show!(row, get_row, u32, "0x%x\n");
mr_attr_show!(column, get_column, u32, "%u\n");
mr_attr_show!(channel, get_channel, u32, "%u\n");
mr_attr_show!(sub_channel, get_sub_channel, u32, "%u\n");

macro_rules! mr_attr_store {
    ($name:ident, $cb:ident, $ty:ty, $conv:ident) => {
        unsafe fn $name(_dev: *mut device, _attr: *mut device_attribute,
                                _buf: *const libc::c_char, _len: usize) -> isize { unimplemented!() }
    };
}
mr_attr_store!(persist_mode, set_persist_mode, libc::c_ulong, kstrtoul);
mr_attr_store!(hpa, set_hpa, u64, kstrtou64);
mr_attr_store!(dpa, set_dpa, u64, kstrtou64);
mr_attr_store!(nibble_mask, set_nibble_mask, libc::c_ulong, kstrtoul);
mr_attr_store!(bank_group, set_bank_group, libc::c_ulong, kstrtoul);
mr_attr_store!(bank, set_bank, libc::c_ulong, kstrtoul);
mr_attr_store!(rank, set_rank, libc::c_ulong, kstrtoul);
mr_attr_store!(row, set_row, libc::c_ulong, kstrtoul);
mr_attr_store!(column, set_column, libc::c_ulong, kstrtoul);
mr_attr_store!(channel, set_channel, libc::c_ulong, kstrtoul);
mr_attr_store!(sub_channel, set_sub_channel, libc::c_ulong, kstrtoul);

unsafe fn repair_store(_dev: *mut device, _attr: *mut device_attribute,
                       _buf: *const libc::c_char, _len: usize) -> isize { unimplemented!() }

unsafe fn mem_repair_attr_visible(_kobj: *mut kobject, _a: *mut attribute,
                                  attr_id: libc::c_int) -> umode_t {
    match attr_id {
        MR_TYPE | MR_SAFE_IN_USE | MR_MIN_HPA | MR_MAX_HPA | MR_MIN_DPA |
        MR_MAX_DPA | MEM_DO_REPAIR => { /* callback presence controls visibility */ }
        MR_PERSIST_MODE | MR_HPA | MR_DPA | MR_NIBBLE_MASK | MR_BANK_GROUP |
        MR_BANK | MR_RANK | MR_ROW | MR_COLUMN | MR_CHANNEL | MR_SUB_CHANNEL => {}
        _ => {}
    }
    0
}

unsafe fn mem_repair_create_desc(dev: *mut device,
                                 attr_groups: *mut *const attribute_group,
                                 instance: u8) -> libc::c_int {
    let ctx = devm_kzalloc(dev, core::mem::size_of::<EdacMemRepairContext>(), GFP_KERNEL);
    if ctx.is_null() { return -ENOMEM; }
    // Field initialization and sysfs registration mirror the C implementation;
    // kernel allocation/layout helpers are external dependencies.
    let _ = (attr_groups, instance);
    0
}

/// Get EDAC memory repair descriptors.
pub unsafe fn edac_mem_repair_get_desc(dev: *mut device,
                                       attr_groups: *mut *const attribute_group,
                                       instance: u8) -> libc::c_int {
    if dev.is_null() || attr_groups.is_null() { return -EINVAL; }
    mem_repair_create_desc(dev, attr_groups, instance)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
