// SPDX-License-Identifier: GPL-2.0-or-later
// Original C header depended on <objtool/elf.h>.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[cfg(BUILD_KLP)]
pub unsafe fn checksum_init(sym: *mut symbol) {
    if !sym.is_null() && (*sym).csum.state.is_null() {
        (*sym).csum.state = XXH3_createState();
        XXH3_64bits_reset((*sym).csum.state);
    }
}

#[cfg(BUILD_KLP)]
pub unsafe fn __checksum_update(sym: *mut symbol, data: *const c_void, size: usize) {
    XXH3_64bits_update((*sym).csum.state, data, size);
}

#[cfg(BUILD_KLP)]
pub unsafe fn __checksum_update_insn(
    sym: *mut symbol,
    insn: *mut instruction,
    data: *const c_void,
    size: usize,
) {
    __checksum_update(sym, data, size);
    dbg_checksum_insn(sym, insn, XXH3_64bits_digest((*sym).csum.state));
}

#[cfg(BUILD_KLP)]
pub unsafe fn __checksum_update_object(
    sym: *mut symbol,
    mut offset: c_ulong,
    what: *const c_char,
    data: *const c_void,
    size: usize,
) {
    __checksum_update(
        sym,
        (&mut offset as *mut c_ulong).cast::<c_void>(),
        size_of::<c_ulong>(),
    );
    __checksum_update(sym, data, size);
    dbg_checksum_object(sym, offset, what, XXH3_64bits_digest((*sym).csum.state));
}

#[cfg(BUILD_KLP)]
pub unsafe fn checksum_finish(sym: *mut symbol) {
    if !sym.is_null() && !(*sym).csum.state.is_null() {
        (*sym).csum.checksum = XXH3_64bits_digest((*sym).csum.state);
        XXH3_freeState((*sym).csum.state);
        (*sym).csum.state = ptr::null_mut();
    }
}

#[cfg(BUILD_KLP)]
extern "C" {
    pub fn calculate_checksums(file: *mut objtool_file) -> c_int;
    pub fn create_sym_checksum_section(file: *mut objtool_file) -> c_int;
}

#[cfg(not(BUILD_KLP))]
pub unsafe fn calculate_checksums(file: *mut objtool_file) -> c_int {
    let _ = file;
    -(ENOSYS as c_int)
}

#[cfg(not(BUILD_KLP))]
pub unsafe fn create_sym_checksum_section(file: *mut objtool_file) -> c_int {
    let _ = file;
    -(EINVAL as c_int)
}
