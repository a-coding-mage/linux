/* SPDX-License-Identifier: GPL-2.0 */

// Translation of Linux TRACE_EVENT declarations from fsi.h.
// The tracepoint registration/printing machinery is supplied externally.

#[repr(C)]
pub struct FsiMasterReadEntry { pub master_idx: core::ffi::c_int, pub link: core::ffi::c_int, pub id: core::ffi::c_int, pub addr: u32, pub size: usize }

pub unsafe fn fsi_master_read(master_idx: core::ffi::c_int, link: core::ffi::c_int, id: core::ffi::c_int, addr: u32, size: usize) -> FsiMasterReadEntry {
    FsiMasterReadEntry { master_idx, link, id, addr, size }
}

#[repr(C)]
pub struct FsiMasterWriteEntry { pub master_idx: core::ffi::c_int, pub link: core::ffi::c_int, pub id: core::ffi::c_int, pub addr: u32, pub size: usize, pub data: u32 }

pub unsafe fn fsi_master_write(master_idx: core::ffi::c_int, link: core::ffi::c_int, id: core::ffi::c_int, addr: u32, size: usize, data: *const core::ffi::c_void) -> FsiMasterWriteEntry {
    let mut entry = FsiMasterWriteEntry { master_idx, link, id, addr, size, data: 0 };
    core::ptr::copy_nonoverlapping(data.cast::<u8>(), core::ptr::addr_of_mut!(entry.data).cast::<u8>(), size);
    entry
}

#[repr(C)]
pub struct FsiMasterRwResultEntry { pub master_idx: core::ffi::c_int, pub link: core::ffi::c_int, pub id: core::ffi::c_int, pub addr: u32, pub size: usize, pub write: bool, pub data: u32, pub ret: core::ffi::c_int }

pub unsafe fn fsi_master_rw_result(master_idx: core::ffi::c_int, link: core::ffi::c_int, id: core::ffi::c_int, addr: u32, size: usize, write: bool, data: *const core::ffi::c_void, ret: core::ffi::c_int) -> FsiMasterRwResultEntry {
    let mut entry = FsiMasterRwResultEntry { master_idx, link, id, addr, size, write, data: 0, ret };
    if entry.write || entry.ret == 0 {
        core::ptr::copy_nonoverlapping(data.cast::<u8>(), core::ptr::addr_of_mut!(entry.data).cast::<u8>(), size);
    }
    entry
}

#[repr(C)]
pub struct FsiMasterBreakEntry { pub master_idx: core::ffi::c_int, pub link: core::ffi::c_int }
pub unsafe fn fsi_master_break(master_idx: core::ffi::c_int, link: core::ffi::c_int) -> FsiMasterBreakEntry { FsiMasterBreakEntry { master_idx, link } }

#[repr(C)]
pub struct FsiMasterScanEntry { pub master_idx: core::ffi::c_int, pub n_links: core::ffi::c_int, pub scan: bool }
pub unsafe fn fsi_master_scan(master_idx: core::ffi::c_int, n_links: core::ffi::c_int, scan: bool) -> FsiMasterScanEntry { FsiMasterScanEntry { master_idx, n_links, scan } }

#[repr(C)]
pub struct FsiMasterUnregisterEntry { pub master_idx: core::ffi::c_int, pub n_links: core::ffi::c_int }
pub unsafe fn fsi_master_unregister(master_idx: core::ffi::c_int, n_links: core::ffi::c_int) -> FsiMasterUnregisterEntry { FsiMasterUnregisterEntry { master_idx, n_links } }

#[repr(C)]
pub struct FsiSlaveInitEntry { pub master_idx: core::ffi::c_int, pub master_n_links: core::ffi::c_int, pub idx: core::ffi::c_int, pub link: core::ffi::c_int, pub chip_id: core::ffi::c_int, pub cfam_id: u32, pub size: u32 }
pub unsafe fn fsi_slave_init(master_idx: core::ffi::c_int, master_n_links: core::ffi::c_int, idx: core::ffi::c_int, link: core::ffi::c_int, chip_id: core::ffi::c_int, cfam_id: u32, size: u32) -> FsiSlaveInitEntry { FsiSlaveInitEntry { master_idx, master_n_links, idx, link, chip_id, cfam_id, size } }

#[repr(C)]
pub struct FsiSlaveInvalidCfamEntry { pub master_idx: core::ffi::c_int, pub master_n_links: core::ffi::c_int, pub link: core::ffi::c_int, pub cfam_id: u32 }
pub unsafe fn fsi_slave_invalid_cfam(master_idx: core::ffi::c_int, master_n_links: core::ffi::c_int, link: core::ffi::c_int, cfam_id: u32) -> FsiSlaveInvalidCfamEntry { FsiSlaveInvalidCfamEntry { master_idx, master_n_links, link, cfam_id } }

#[repr(C)]
pub struct FsiDevInitEntry { pub master_idx: core::ffi::c_int, pub link: core::ffi::c_int, pub type_: core::ffi::c_int, pub unit: core::ffi::c_int, pub version: core::ffi::c_int, pub addr: u32, pub size: u32 }
pub unsafe fn fsi_dev_init(master_idx: core::ffi::c_int, link: core::ffi::c_int, type_: core::ffi::c_int, unit: core::ffi::c_int, version: core::ffi::c_int, addr: u32, size: u32) -> FsiDevInitEntry { FsiDevInitEntry { master_idx, link, type_, unit, version, addr, size } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
