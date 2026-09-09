/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) International Business Machines Corp., 2000-2004 */

// Translated from jfs_types.h. Linux type and endian definitions are supplied
// by the surrounding translation unit.

pub type tid_t = u16;
pub type lid_t = u16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timestruc_t {
    pub tv_sec: u32,
    pub tv_nsec: u32,
}

pub const LEFTMOSTONE: u32 = 0x80000000;
pub const HIGHORDER: u32 = 0x80000000u32;
pub const ONES: u32 = 0xffffffffu32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxd_t {
    pub len_addr: u32,
    pub addr2: u32,
}

#[inline]
pub unsafe fn PXDlength(pxd: *mut pxd_t, len: u32) {
    (*pxd).len_addr = ((*pxd).len_addr & (!0xffffffu32).to_le())
        | (len & 0xffffffu32).to_le();
}

#[inline]
pub unsafe fn PXDaddress(pxd: *mut pxd_t, addr: u64) {
    (*pxd).len_addr = ((*pxd).len_addr & 0xffffffu32.to_le())
        | (((addr >> 32) << 24) as u32).to_le();
    (*pxd).addr2 = (addr as u32).to_le();
}

#[inline]
pub unsafe fn lengthPXD(pxd: *mut pxd_t) -> u32 {
    u32::from_le((*pxd).len_addr) & 0xffffffu32
}

#[inline]
pub unsafe fn addressPXD(pxd: *mut pxd_t) -> u64 {
    let n = u32::from_le((*pxd).len_addr) & !0xffffffu32;
    ((n as u64) << 8) + u32::from_le((*pxd).addr2) as u64
}

pub const MAXTREEHEIGHT: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxdlist {
    pub maxnpxd: i16,
    pub npxd: i16,
    pub pxd: [pxd_t; MAXTREEHEIGHT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dxd_t {
    pub flag: u8,
    pub rsrvd: [u8; 3],
    pub size: u32,
    pub loc: pxd_t,
}

pub const DXD_INDEX: u8 = 0x80;
pub const DXD_INLINE: u8 = 0x40;
pub const DXD_EXTENT: u8 = 0x20;
pub const DXD_FILE: u8 = 0x10;
pub const DXD_CORRUPT: u8 = 0x08;

#[inline]
pub unsafe fn DXDlength(dxd: *mut dxd_t, len: u32) { PXDlength(&mut (*dxd).loc, len); }
#[inline]
pub unsafe fn DXDaddress(dxd: *mut dxd_t, addr: u64) { PXDaddress(&mut (*dxd).loc, addr); }
#[inline]
pub unsafe fn lengthDXD(dxd: *mut dxd_t) -> u32 { lengthPXD(&mut (*dxd).loc) }
#[inline]
pub unsafe fn addressDXD(dxd: *mut dxd_t) -> u64 { addressPXD(&mut (*dxd).loc) }
#[inline]
pub unsafe fn DXDsize(dxd: *mut dxd_t, size32: u32) { (*dxd).size = size32.to_le(); }
#[inline]
pub unsafe fn sizeDXD(dxd: *mut dxd_t) -> u32 { u32::from_le((*dxd).size) }

#[repr(C)]
pub struct component_name {
    pub namlen: core::ffi::c_int,
    pub name: *mut i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd {
    pub thresh: u8,
    pub delta: u8,
    pub rsrvd1: u8,
    pub limit_hi: u8,
    pub limit_lo: u32,
    pub rsrvd2: [u8; 3],
    pub used_hi: u8,
    pub used_lo: u32,
}

#[inline]
pub unsafe fn DASDLIMIT(dasdp: *const dasd) -> u64 {
    ((*dasdp).limit_hi as u64) << 32 | u32::from_le((*dasdp).limit_lo) as u64
}
#[inline]
pub unsafe fn setDASDLIMIT(dasdp: *mut dasd, limit: u64) {
    (*dasdp).limit_hi = (limit >> 32) as u8;
    (*dasdp).limit_lo = (limit as u32).to_le();
}
#[inline]
pub unsafe fn DASDUSED(dasdp: *const dasd) -> u64 {
    ((*dasdp).used_hi as u64) << 32 | u32::from_le((*dasdp).used_lo) as u64
}
#[inline]
pub unsafe fn setDASDUSED(dasdp: *mut dasd, used: u64) {
    (*dasdp).used_hi = (used >> 32) as u8;
    (*dasdp).used_lo = (used as u32).to_le();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
