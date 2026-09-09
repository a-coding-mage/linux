// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2024 Intel Corporation */

// Kernel/project declarations supplied by the surrounding translation unit.

const ADF_MSTATE_MAGIC: u32 = 0xADF5CAEA;
const ADF_MSTATE_VERSION: u32 = 0x1;

#[repr(C)]
pub struct adf_mstate_sect_h {
    pub id: [u8; ADF_MSTATE_ID_LEN],
    pub size: u32,
    pub sub_sects: u32,
    pub state: [u8; 0],
}

pub unsafe fn adf_mstate_state_size(mgr: *mut adf_mstate_mgr) -> u32 {
    (*mgr).state.offset_from((*mgr).buf) as u32
}

#[inline]
unsafe fn adf_mstate_avail_room(mgr: *mut adf_mstate_mgr) -> u32 {
    (*mgr).buf.add((*mgr).size as usize).offset_from((*mgr).state) as u32
}

pub unsafe fn adf_mstate_mgr_init(mgr: *mut adf_mstate_mgr, buf: *mut u8, size: u32) {
    (*mgr).buf = buf;
    (*mgr).state = buf;
    (*mgr).size = size;
    (*mgr).n_sects = 0;
}

pub unsafe fn adf_mstate_mgr_new(buf: *mut u8, size: u32) -> *mut adf_mstate_mgr {
    let mgr = kzalloc_obj::<adf_mstate_mgr>();
    if mgr.is_null() {
        return core::ptr::null_mut();
    }
    adf_mstate_mgr_init(mgr, buf, size);
    mgr
}

pub unsafe fn adf_mstate_mgr_destroy(mgr: *mut adf_mstate_mgr) {
    kfree(mgr);
}

pub unsafe fn adf_mstate_mgr_init_from_parent(mgr: *mut adf_mstate_mgr, p_mgr: *mut adf_mstate_mgr) {
    adf_mstate_mgr_init(mgr, (*p_mgr).state,
        (*p_mgr).size - adf_mstate_state_size(p_mgr));
}

pub unsafe fn adf_mstate_mgr_init_from_psect(mgr: *mut adf_mstate_mgr, p_sect: *mut adf_mstate_sect_h) {
    adf_mstate_mgr_init(mgr, (*p_sect).state.as_mut_ptr(), (*p_sect).size);
    (*mgr).n_sects = (*p_sect).sub_sects;
}

unsafe fn adf_mstate_preamble_init(preamble: *mut adf_mstate_preh) {
    (*preamble).magic = ADF_MSTATE_MAGIC;
    (*preamble).version = ADF_MSTATE_VERSION;
    (*preamble).preh_len = core::mem::size_of::<adf_mstate_preh>() as _;
    (*preamble).size = 0;
    (*preamble).n_sects = 0;
}

unsafe fn adf_mstate_preamble_def_checker(preamble: *mut adf_mstate_preh, opaque: *mut core::ffi::c_void) -> i32 {
    let mgr = opaque as *mut adf_mstate_mgr;
    if (*preamble).magic != ADF_MSTATE_MAGIC || (*preamble).version > ADF_MSTATE_VERSION ||
       (*preamble).preh_len > (*mgr).size { return -EINVAL; }
    0
}

pub unsafe fn adf_mstate_preamble_add(mgr: *mut adf_mstate_mgr) -> *mut adf_mstate_preh {
    let pre = (*mgr).buf as *mut adf_mstate_preh;
    if adf_mstate_avail_room(mgr) < core::mem::size_of::<adf_mstate_preh>() as u32 { return core::ptr::null_mut(); }
    adf_mstate_preamble_init(pre);
    (*mgr).state = (*mgr).state.add((*pre).preh_len as usize);
    pre
}

pub unsafe fn adf_mstate_preamble_update(mgr: *mut adf_mstate_mgr) -> i32 {
    let pre = (*mgr).buf as *mut adf_mstate_preh;
    (*pre).size = adf_mstate_state_size(mgr) - (*pre).preh_len;
    (*pre).n_sects = (*mgr).n_sects;
    0
}

unsafe fn adf_mstate_dump_sect(_sect: *mut adf_mstate_sect_h, _prefix: *const i8) {}

#[inline]
unsafe fn __adf_mstate_sect_update(mgr: *mut adf_mstate_mgr, sect: *mut adf_mstate_sect_h, size: u32, n_subsects: u32) {
    (*sect).size += size;
    (*sect).sub_sects += n_subsects;
    (*mgr).n_sects += 1;
    (*mgr).state = (*mgr).state.add((*sect).size as usize);
    adf_mstate_dump_sect(sect, b"Add\0".as_ptr() as *const i8);
}

pub unsafe fn adf_mstate_sect_update(p_mgr: *mut adf_mstate_mgr, curr_mgr: *mut adf_mstate_mgr, sect: *mut adf_mstate_sect_h) {
    __adf_mstate_sect_update(p_mgr, sect, adf_mstate_state_size(curr_mgr), (*curr_mgr).n_sects);
}

unsafe fn adf_mstate_sect_add_header(mgr: *mut adf_mstate_mgr, id: *const i8) -> *mut adf_mstate_sect_h {
    let sect = (*mgr).state as *mut adf_mstate_sect_h;
    if adf_mstate_avail_room(mgr) < core::mem::size_of::<adf_mstate_sect_h>() as u32 { return core::ptr::null_mut(); }
    strscpy((*sect).id.as_mut_ptr(), id);
    (*sect).size = 0; (*sect).sub_sects = 0;
    (*mgr).state = (*mgr).state.add(core::mem::size_of::<adf_mstate_sect_h>());
    sect
}

pub unsafe fn adf_mstate_sect_add_vreg(mgr: *mut adf_mstate_mgr, id: *const i8, info: *mut adf_mstate_vreginfo) -> *mut adf_mstate_sect_h {
    let sect = adf_mstate_sect_add_header(mgr, id); if sect.is_null() { return sect; }
    if adf_mstate_avail_room(mgr) < (*info).size { return core::ptr::null_mut(); }
    core::ptr::copy_nonoverlapping((*info).addr, (*sect).state.as_mut_ptr(), (*info).size as usize);
    __adf_mstate_sect_update(mgr, sect, (*info).size, 0); sect
}

pub unsafe fn adf_mstate_sect_add(mgr: *mut adf_mstate_mgr, id: *const i8, populate: adf_mstate_populate, opaque: *mut core::ffi::c_void) -> *mut adf_mstate_sect_h {
    let sect = adf_mstate_sect_add_header(mgr, id); if sect.is_null() { return sect; }
    if populate.is_none() { return sect; }
    let avail_room = adf_mstate_avail_room(mgr); let mut sub = core::mem::MaybeUninit::<adf_mstate_mgr>::uninit();
    adf_mstate_mgr_init_from_parent(sub.as_mut_ptr(), mgr);
    let sub = sub.as_mut_ptr(); let size = populate.unwrap()(sub, (*sect).state.as_mut_ptr(), avail_room, opaque);
    if size < 0 { return core::ptr::null_mut(); }
    let size = size as u32 + adf_mstate_state_size(sub);
    if avail_room < size { return core::ptr::null_mut(); }
    __adf_mstate_sect_update(mgr, sect, size, (*sub).n_sects); sect
}

unsafe fn adf_mstate_sect_validate(mgr: *mut adf_mstate_mgr) -> i32 {
    let start = (*mgr).state as *mut adf_mstate_sect_h; let mut sect = start;
    let end = (*mgr).buf as u64 + (*mgr).size as u64;
    for _ in 0..(*mgr).n_sects { let s = (*sect).state.as_ptr() as u64; let e = s + (*sect).size as u64; if e < s || e > end { return -EINVAL; } sect = e as *mut adf_mstate_sect_h; }
    0
}

pub unsafe fn adf_mstate_state_size_from_remote(mgr: *mut adf_mstate_mgr) -> u32 { let p = (*mgr).buf as *mut adf_mstate_preh; (*p).preh_len + (*p).size }

pub unsafe fn adf_mstate_mgr_init_from_remote(mgr: *mut adf_mstate_mgr, buf: *mut u8, size: u32, pre_checker: adf_mstate_preamble_checker, opaque: *mut core::ffi::c_void) -> i32 {
    adf_mstate_mgr_init(mgr, buf, size); let pre = (*mgr).buf as *mut adf_mstate_preh;
    let ret = if let Some(f) = pre_checker { f(pre, opaque) } else { adf_mstate_preamble_def_checker(pre, mgr as *mut _ as _) };
    if ret != 0 { return ret; } (*mgr).state = (*mgr).buf.add((*pre).preh_len as usize); (*mgr).n_sects = (*pre).n_sects; adf_mstate_sect_validate(mgr)
}

pub unsafe fn adf_mstate_sect_lookup(mgr: *mut adf_mstate_mgr, id: *const i8, action: adf_mstate_action, opaque: *mut core::ffi::c_void) -> *mut adf_mstate_sect_h {
    let mut sect = (*mgr).state as *mut adf_mstate_sect_h;
    for _ in 0..(*mgr).n_sects { if !strncmp((*sect).id.as_ptr(), id, (*sect).id.len()) { break; } sect = (*sect).state.as_ptr().add((*sect).size as usize) as *mut adf_mstate_sect_h; }
    if sect.is_null() { return core::ptr::null_mut(); }
    let mut sub = core::mem::MaybeUninit::<adf_mstate_mgr>::uninit(); adf_mstate_mgr_init_from_psect(sub.as_mut_ptr(), sect); let sub = sub.as_mut_ptr();
    if (*sect).sub_sects != 0 && adf_mstate_sect_validate(sub) != 0 { return core::ptr::null_mut(); }
    if let Some(f) = action { if f(sub, (*sect).state.as_mut_ptr(), (*sect).size, opaque) != 0 { return core::ptr::null_mut(); } }
    sect
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
