// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2006 Jiri Benc <jbenc@suse.cz>
 * Copyright 2007 Johannes Berg <johannes@sipsolutions.net>
 * Copyright (C) 2020-2023 Intel Corporation
 */

// Linux/kernel/mac80211 dependencies are supplied by the surrounding translation.

#[repr(C)]
pub struct ieee80211_if_read_sdata_data {
    pub format: Option<unsafe extern "C" fn(*const ieee80211_sub_if_data, *mut c_char, c_int) -> ssize_t>,
    pub sdata: *mut ieee80211_sub_if_data,
}

unsafe extern "C" fn ieee80211_if_read_sdata_handler(_wiphy: *mut wiphy, _file: *mut file,
                                                       buf: *mut c_char, bufsize: usize, data: *mut c_void) -> ssize_t {
    let d = &*(data as *const ieee80211_if_read_sdata_data);
    (d.format.unwrap())(d.sdata, buf, bufsize as c_int)
}

unsafe extern "C" fn ieee80211_if_read_sdata(file: *mut file, userbuf: *mut c_char,
                                               count: usize, ppos: *mut loff_t,
                                               format: Option<unsafe extern "C" fn(*const ieee80211_sub_if_data, *mut c_char, c_int) -> ssize_t>) -> ssize_t {
    let sdata = (*file).private_data as *mut ieee80211_sub_if_data;
    let data = ieee80211_if_read_sdata_data { format, sdata };
    let mut buf = [0 as c_char; 200];
    wiphy_locked_debugfs_read((*(*sdata).local).hw.wiphy, file, buf.as_mut_ptr(), buf.len(), userbuf, count, ppos,
                              Some(ieee80211_if_read_sdata_handler), &data as *const _ as *mut c_void)
}

#[repr(C)]
pub struct ieee80211_if_write_sdata_data {
    pub write: Option<unsafe extern "C" fn(*mut ieee80211_sub_if_data, *const c_char, c_int) -> ssize_t>,
    pub sdata: *mut ieee80211_sub_if_data,
}

unsafe extern "C" fn ieee80211_if_write_sdata_handler(_wiphy: *mut wiphy, _file: *mut file,
                                                        buf: *mut c_char, count: usize, data: *mut c_void) -> ssize_t {
    let d = &*(data as *const ieee80211_if_write_sdata_data);
    (d.write.unwrap())(d.sdata, buf, count as c_int)
}

unsafe extern "C" fn ieee80211_if_write_sdata(file: *mut file, userbuf: *const c_char, count: usize,
                                                ppos: *mut loff_t,
                                                write: Option<unsafe extern "C" fn(*mut ieee80211_sub_if_data, *const c_char, c_int) -> ssize_t>) -> ssize_t {
    let sdata = (*file).private_data as *mut ieee80211_sub_if_data;
    let data = ieee80211_if_write_sdata_data { write, sdata };
    let mut buf = [0 as c_char; 64];
    wiphy_locked_debugfs_write((*(*sdata).local).hw.wiphy, file, buf.as_mut_ptr(), buf.len(), userbuf, count,
                               Some(ieee80211_if_write_sdata_handler), &data as *const _ as *mut c_void)
}

#[repr(C)]
pub struct ieee80211_if_read_link_data {
    pub format: Option<unsafe extern "C" fn(*const ieee80211_link_data, *mut c_char, c_int) -> ssize_t>,
    pub link: *mut ieee80211_link_data,
}

unsafe extern "C" fn ieee80211_if_read_link_handler(_wiphy: *mut wiphy, _file: *mut file,
                                                      buf: *mut c_char, bufsize: usize, data: *mut c_void) -> ssize_t {
    let d = &*(data as *const ieee80211_if_read_link_data);
    (d.format.unwrap())(d.link, buf, bufsize as c_int)
}

unsafe extern "C" fn ieee80211_if_read_link(file: *mut file, userbuf: *mut c_char, count: usize,
                                              ppos: *mut loff_t,
                                              format: Option<unsafe extern "C" fn(*const ieee80211_link_data, *mut c_char, c_int) -> ssize_t>) -> ssize_t {
    let link = (*file).private_data as *mut ieee80211_link_data;
    let data = ieee80211_if_read_link_data { format, link };
    let mut buf = [0 as c_char; 200];
    wiphy_locked_debugfs_read((*(*(*link).sdata).local).hw.wiphy, file, buf.as_mut_ptr(), buf.len(), userbuf, count, ppos,
                              Some(ieee80211_if_read_link_handler), &data as *const _ as *mut c_void)
}

#[repr(C)]
pub struct ieee80211_if_write_link_data {
    pub write: Option<unsafe extern "C" fn(*mut ieee80211_link_data, *const c_char, c_int) -> ssize_t>,
    pub link: *mut ieee80211_link_data,
}

unsafe extern "C" fn ieee80211_if_write_link_handler(_wiphy: *mut wiphy, _file: *mut file,
                                                       buf: *mut c_char, count: usize, data: *mut c_void) -> ssize_t {
    let d = &*(data as *const ieee80211_if_write_link_data);
    (d.write.unwrap())(d.link, buf, count as c_int)
}

unsafe extern "C" fn ieee80211_if_write_link(file: *mut file, userbuf: *const c_char, count: usize,
                                               ppos: *mut loff_t,
                                               write: Option<unsafe extern "C" fn(*mut ieee80211_link_data, *const c_char, c_int) -> ssize_t>) -> ssize_t {
    let link = (*file).private_data as *mut ieee80211_link_data;
    let data = ieee80211_if_write_link_data { write, link };
    let mut buf = [0 as c_char; 64];
    wiphy_locked_debugfs_write((*(*(*link).sdata).local).hw.wiphy, file, buf.as_mut_ptr(), buf.len(), userbuf, count,
                               Some(ieee80211_if_write_link_handler), &data as *const _ as *mut c_void)
}

// The following declarations mirror the C macro-generated debugfs accessors.
// Field-specific types and external operations are supplied by the surrounding tree.
macro_rules! ieee80211_if_fmt_dec { ($name:ident, $ty:ty, $field:expr) => {
    unsafe extern "C" fn $name(data: *const $ty, buf: *mut c_char, buflen: c_int) -> ssize_t {
        scnprintf(buf, buflen, "%d\0".as_ptr() as *const c_char, (*data).$field)
    }
}; }

static mut smps_modes: [*const c_char; IEEE80211_SMPS_NUM_MODES as usize] = [
    b"auto\0".as_ptr() as *const c_char, b"off\0".as_ptr() as *const c_char,
    b"static\0".as_ptr() as *const c_char, b"dynamic\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn ieee80211_set_smps(link: *mut ieee80211_link_data, smps_mode: ieee80211_smps_mode) -> c_int {
    let sdata = (*link).sdata;
    let local = (*sdata).local;
    if (*sdata).vif.driver_flags & IEEE80211_VIF_EML_ACTIVE != 0 { return -EOPNOTSUPP; }
    if (*(*local).hw.wiphy).features & NL80211_FEATURE_STATIC_SMPS == 0 && smps_mode == IEEE80211_SMPS_STATIC { return -EINVAL; }
    if (*(*local).hw.wiphy).features & NL80211_FEATURE_DYNAMIC_SMPS == 0 &&
       (smps_mode == IEEE80211_SMPS_DYNAMIC || smps_mode == IEEE80211_SMPS_AUTOMATIC) { return -EINVAL; }
    if (*sdata).vif.type != NL80211_IFTYPE_STATION { return -EOPNOTSUPP; }
    __ieee80211_request_smps_mgd(sdata, link, smps_mode)
}

unsafe extern "C" fn ieee80211_if_fmt_smps(link: *const ieee80211_link_data, buf: *mut c_char, buflen: c_int) -> ssize_t {
    if (*(*link).sdata).vif.type == NL80211_IFTYPE_STATION {
        return snprintf(buf, buflen, b"request: %s\nused: %s\n\0".as_ptr() as *const c_char,
                        smps_modes[(*link).u.mgd.req_smps as usize], smps_modes[(*link).smps_mode as usize]);
    }
    -EINVAL as ssize_t
}

unsafe extern "C" fn ieee80211_if_parse_smps(link: *mut ieee80211_link_data, buf: *const c_char, buflen: c_int) -> ssize_t {
    for mode in 0..IEEE80211_SMPS_NUM_MODES {
        if strncmp(buf, smps_modes[mode as usize], buflen as usize) == 0 {
            let err = ieee80211_set_smps(link, mode as ieee80211_smps_mode);
            return if err == 0 { buflen as ssize_t } else { err as ssize_t };
        }
    }
    -EINVAL as ssize_t
}

// Remaining file-local debugfs registration and format/parse functions retain the C control flow.
// They are declared here as external-facing Rust symbols so generated accessors can bind to them.
extern "C" {
    fn add_files(sdata: *mut ieee80211_sub_if_data);
    fn add_link_files(link: *mut ieee80211_link_data, dentry: *mut dentry);
}

pub unsafe extern "C" fn ieee80211_debugfs_add_netdev(sdata: *mut ieee80211_sub_if_data, mld_vif: bool) {
    let mut buf = [0 as c_char; 10 + IFNAMSIZ];
    sprintf(buf.as_mut_ptr(), b"netdev:%s\0".as_ptr() as *const c_char, (*sdata).name.as_ptr());
    (*sdata).vif.debugfs_dir = debugfs_create_dir(buf.as_ptr(), (*(*(*sdata).local).hw.wiphy).debugfsdir);
    (*sdata).deflink.debugfs_dir = (*sdata).vif.debugfs_dir;
    (*sdata).debugfs.subdir_stations = debugfs_create_dir(b"stations\0".as_ptr() as *const c_char, (*sdata).vif.debugfs_dir);
    add_files(sdata);
    if !mld_vif { add_link_files(&mut (*sdata).deflink, (*sdata).vif.debugfs_dir); }
}

pub unsafe extern "C" fn ieee80211_debugfs_remove_netdev(sdata: *mut ieee80211_sub_if_data) {
    if (*sdata).vif.debugfs_dir.is_null() { return; }
    debugfs_remove_recursive((*sdata).vif.debugfs_dir);
    (*sdata).vif.debugfs_dir = core::ptr::null_mut();
    (*sdata).debugfs.subdir_stations = core::ptr::null_mut();
}

pub unsafe extern "C" fn ieee80211_debugfs_rename_netdev(sdata: *mut ieee80211_sub_if_data) {
    debugfs_change_name((*sdata).vif.debugfs_dir, b"netdev:%s\0".as_ptr() as *const c_char, (*sdata).name.as_ptr());
}

pub unsafe extern "C" fn ieee80211_debugfs_recreate_netdev(sdata: *mut ieee80211_sub_if_data, mld_vif: bool) {
    ieee80211_debugfs_remove_netdev(sdata);
    ieee80211_debugfs_add_netdev(sdata, mld_vif);
    if (*sdata).flags & IEEE80211_SDATA_IN_DRIVER != 0 {
        drv_vif_add_debugfs((*sdata).local, sdata);
        if !mld_vif { ieee80211_link_debugfs_drv_add(&mut (*sdata).deflink); }
    }
}

pub unsafe extern "C" fn ieee80211_link_debugfs_add(link: *mut ieee80211_link_data) {
    if WARN_ON((*link).sdata.is_null() || (*link).sdata.as_ref().unwrap().vif.debugfs_dir.is_null() || !(*link).debugfs_dir.is_null()) { return; }
    if WARN_ON((*(*(*link).sdata).local).hw.wiphy.as_ref().unwrap().flags & WIPHY_FLAG_SUPPORTS_MLO == 0) { return; }
    let mut name = [0 as c_char; 10];
    snprintf(name.as_mut_ptr(), name.len() as c_int, b"link-%d\0".as_ptr() as *const c_char, (*link).link_id);
    (*link).debugfs_dir = debugfs_create_dir(name.as_ptr(), (*(*link).sdata).vif.debugfs_dir);
    add_link_files(link, (*link).debugfs_dir);
}

pub unsafe extern "C" fn ieee80211_link_debugfs_remove(link: *mut ieee80211_link_data) {
    if link.is_null() || (*link).sdata.is_null() || (*(*link).sdata).vif.debugfs_dir.is_null() || (*link).debugfs_dir.is_null() { if !link.is_null() { (*link).debugfs_dir = core::ptr::null_mut(); } return; }
    if (*link).debugfs_dir == (*(*link).sdata).vif.debugfs_dir { WARN_ON(link != &mut (*(*link).sdata).deflink); (*link).debugfs_dir = core::ptr::null_mut(); return; }
    debugfs_remove_recursive((*link).debugfs_dir); (*link).debugfs_dir = core::ptr::null_mut();
}

pub unsafe extern "C" fn ieee80211_link_debugfs_drv_add(link: *mut ieee80211_link_data) {
    if (*(*link).sdata).vif.type == NL80211_IFTYPE_MONITOR || WARN_ON((*link).debugfs_dir.is_null()) { return; }
    drv_link_add_debugfs((*(*link).sdata).local, (*link).sdata, (*link).conf, (*link).debugfs_dir);
}

pub unsafe extern "C" fn ieee80211_link_debugfs_drv_remove(link: *mut ieee80211_link_data) {
    if link.is_null() || (*link).debugfs_dir.is_null() { return; }
    if WARN_ON((*link).debugfs_dir == (*(*link).sdata).vif.debugfs_dir) { return; }
    debugfs_remove_recursive((*link).debugfs_dir); (*link).debugfs_dir = core::ptr::null_mut();
    ieee80211_link_debugfs_add(link);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
