/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 2006
 *
 * Author(s): Melissa Howland <melissah@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const APPLDATA_START_INTERVAL_REC: u8 = 0x80;
pub const APPLDATA_STOP_REC: u8 = 0x81;
pub const APPLDATA_GEN_EVENT_REC: u8 = 0x82;
pub const APPLDATA_START_CONFIG_REC: u8 = 0x83;

/*
 * Parameter list for DIAGNOSE X'DC'
 */
#[repr(C, packed)]
pub struct appldata_parameter_list {
    pub diag: u16,
    pub function: u8,
    pub parlist_length: u8,
    pub unused01: u32,
    pub reserved: u16,
    pub buffer_length: u16,
    pub unused02: u32,
    pub product_id_addr: u64,
    pub buffer_addr: u64,
}

#[repr(C, packed)]
pub struct appldata_product_id {
    pub prod_nr: [u8; 7], /* product number */
    pub prod_fn: u16,     /* product function */
    pub record_nr: u8,   /* record number */
    pub version_nr: u16, /* version */
    pub release_nr: u16, /* release */
    pub mod_lvl: u16,    /* modification level */
}

// External symbols supplied by the surrounding kernel translation.
extern "C" {
    fn machine_is_vm() -> bool;
    fn virt_to_phys(address: *const core::ffi::c_void) -> u64;
    fn diag_stat_inc(stat: u32);
}

// The value is supplied by the kernel I/O definitions.
pub const EOPNOTSUPP: i32 = 95;
pub const DIAG_STAT_X0DC: u32 = 0;

#[inline]
pub unsafe fn appldata_asm(
    parm_list: *mut appldata_parameter_list,
    id: *mut appldata_product_id,
    fn_: u16,
    buffer: *mut core::ffi::c_void,
    length: u16,
) -> i32 {
    let mut ry: i32;

    if !machine_is_vm() {
        return -EOPNOTSUPP;
    }
    (*parm_list).diag = 0xdc;
    (*parm_list).function = fn_ as u8;
    (*parm_list).parlist_length = core::mem::size_of::<appldata_parameter_list>() as u8;
    (*parm_list).buffer_length = length;
    (*parm_list).product_id_addr = virt_to_phys(id.cast());
    (*parm_list).buffer_addr = virt_to_phys(buffer);
    diag_stat_inc(DIAG_STAT_X0DC);
    core::arch::asm!(
        "diag {1}, {0}, 0xdc",
        inout("r0") virt_to_phys(parm_list.cast()) => ry,
        in("r1") virt_to_phys(parm_list.cast()),
        inlateout("memory") 0usize => _,
        options(nostack)
    );
    ry
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
