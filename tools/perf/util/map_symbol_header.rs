// SPDX-License-Identifier: GPL-2.0

// Translated from perf/util/map_symbol.h.
// Original C dependencies: <linux/types.h>

pub enum thread {}
pub enum maps {}
pub enum map {}
pub enum symbol {}

#[repr(C)]
pub struct map_symbol {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct addr_map_symbol {
    pub ms: map_symbol,
    pub addr: u64,
    pub al_addr: u64,
    pub al_level: ::std::os::raw::c_char,
    pub phys_addr: u64,
    pub data_page_size: u64,
}

unsafe extern "C" {
    pub fn map_symbol__exit(ms: *mut map_symbol);
    pub fn addr_map_symbol__exit(ams: *mut addr_map_symbol);

    pub fn map_symbol__copy(dst: *mut map_symbol, src: *mut map_symbol);
    pub fn addr_map_symbol__copy(dst: *mut addr_map_symbol, src: *mut addr_map_symbol);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
