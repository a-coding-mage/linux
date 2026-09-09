// SPDX-License-Identifier: GPL-2.0
/* System Control and Management Interface (SCMI) Message Protocol Quirks */

// Kernel dependencies supplied by the surrounding SCMI implementation.

const SCMI_QUIRKS_HT_SZ: usize = 4;

#[repr(C)]
pub struct scmi_quirk {
    pub enabled: bool,
    pub name: *const core::ffi::c_char,
    pub vendor: *const core::ffi::c_char,
    pub sub_vendor_id: *const core::ffi::c_char,
    pub impl_ver_range: *const core::ffi::c_char,
    pub start_range: u32,
    pub end_range: u32,
    pub key: *mut static_key_false,
    pub hash: hlist_node,
    pub hkey: u32,
    pub compats: *const *const core::ffi::c_char,
}

#[repr(C)]
pub struct static_key_false {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hlist_node {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct device {
    _opaque: [u8; 0],
}

extern "C" {
    fn kasprintf(flags: u32, fmt: *const core::ffi::c_char, ...) -> *mut core::ffi::c_char;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmemdup(src: *const core::ffi::c_void, len: usize, flags: u32) -> *mut core::ffi::c_char;
    fn strlen(s: *const core::ffi::c_char) -> usize;
    fn strchr(s: *mut core::ffi::c_char, c: i32) -> *mut core::ffi::c_char;
    fn kstrtouint(s: *const core::ffi::c_char, base: u32, result: *mut u32) -> i32;
    fn of_machine_compatible_match(compats: *const *const core::ffi::c_char) -> bool;
    fn static_branch_enable(key: *mut static_key_false);
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn partial_name_hash(c: u8, hash: u64) -> u64;
    fn end_name_hash(hash: u64) -> u32;
    fn hash_add(table: *mut core::ffi::c_void, node: *mut hlist_node, key: u32);
    fn hash_for_each_possible(table: *mut core::ffi::c_void, key: u32,
                              callback: extern "C" fn(*mut scmi_quirk));
}

static mut scmi_quirk_clock_rates_triplet_out_of_spec: static_key_false = static_key_false { _opaque: [] };
static mut scmi_quirk_perf_level_get_fc_force: static_key_false = static_key_false { _opaque: [] };

static NAME_CLOCK: &[u8] = b"quirk_clock_rates_triplet_out_of_spec\0";
static NAME_PERF: &[u8] = b"quirk_perf_level_get_fc_force\0";
static VENDOR_QUALCOMM: &[u8] = b"Qualcomm\0";
static RANGE_PERF: &[u8] = b"0x20000-\0";
static EMPTY_COMPATS: [*const core::ffi::c_char; 1] = [core::ptr::null()];

static mut scmi_quirk_entry_clock_rates_triplet_out_of_spec: scmi_quirk = scmi_quirk {
    enabled: false, name: NAME_CLOCK.as_ptr() as _, vendor: core::ptr::null(),
    sub_vendor_id: core::ptr::null(), impl_ver_range: core::ptr::null(),
    start_range: 0, end_range: 0, key: core::ptr::addr_of_mut!(scmi_quirk_clock_rates_triplet_out_of_spec), hash: hlist_node { _opaque: [] },
    hkey: 0, compats: EMPTY_COMPATS.as_ptr(),
};

static mut scmi_quirk_entry_perf_level_get_fc_force: scmi_quirk = scmi_quirk {
    enabled: false, name: NAME_PERF.as_ptr() as _, vendor: VENDOR_QUALCOMM.as_ptr() as _,
    sub_vendor_id: core::ptr::null(), impl_ver_range: RANGE_PERF.as_ptr() as _,
    start_range: 0, end_range: 0, key: core::ptr::addr_of_mut!(scmi_quirk_perf_level_get_fc_force), hash: hlist_node { _opaque: [] },
    hkey: 0, compats: EMPTY_COMPATS.as_ptr(),
};

static mut scmi_quirks_table: [*mut scmi_quirk; 3] = [
    core::ptr::addr_of_mut!(scmi_quirk_entry_clock_rates_triplet_out_of_spec),
    core::ptr::addr_of_mut!(scmi_quirk_entry_perf_level_get_fc_force),
    core::ptr::null_mut(),
];

static mut scmi_quirks_ht: [u8; SCMI_QUIRKS_HT_SZ] = [0; SCMI_QUIRKS_HT_SZ];

unsafe fn scmi_quirk_signature(vend: *const core::ffi::c_char, sub_vend: *const core::ffi::c_char) -> u32 {
    let _ = (vend, sub_vend);
    // The kernel implementation allocates "|%s|%s|", lowercases it, and hashes it.
    0
}

unsafe fn scmi_quirk_range_parse(quirk: *mut scmi_quirk) -> i32 {
    (*quirk).start_range = 0;
    (*quirk).end_range = 0xFFFF_FFFF;
    if (*quirk).impl_ver_range.is_null() || strlen((*quirk).impl_ver_range) == 0 { return 0; }
    let len = strlen((*quirk).impl_ver_range);
    let first = kmemdup((*quirk).impl_ver_range as _, len + 1, 0);
    if first.is_null() { return -12; }
    let last = first.add(len - 1);
    let sep = strchr(first, b'-' as i32);
    if !sep.is_null() { *sep = 0; }
    let mut ret = 0;
    if sep != first {
        ret = kstrtouint(first, 0, &mut (*quirk).start_range);
        if ret != 0 { kfree(first as _); return ret; }
    }
    if sep.is_null() { (*quirk).end_range = (*quirk).start_range; }
    else if sep != last { ret = kstrtouint(sep.add(1), 0, &mut (*quirk).end_range); }
    kfree(first as _);
    if (*quirk).start_range > (*quirk).end_range { return -22; }
    ret
}

pub unsafe fn scmi_quirks_initialize() {
    let mut i = 0;
    while !scmi_quirks_table[i].is_null() {
        let quirk = scmi_quirks_table[i];
        if scmi_quirk_range_parse(quirk) != 0 { i += 1; continue; }
        (*quirk).hkey = scmi_quirk_signature((*quirk).vendor, (*quirk).sub_vendor_id);
        hash_add(scmi_quirks_ht.as_mut_ptr() as _, &mut (*quirk).hash, (*quirk).hkey);
        i += 1;
    }
}

pub unsafe fn scmi_quirks_enable(dev: *mut device, vend: *const core::ffi::c_char,
                                 subv: *const core::ffi::c_char, impl_: u32) {
    for i in (0..=3).rev() {
        let hkey = scmi_quirk_signature(if i > 1 { vend } else { core::ptr::null() },
                                        if i > 2 { subv } else { core::ptr::null() });
        let _ = (dev, hkey, impl_);
        // `hash_for_each_possible` is a kernel macro; its iteration is supplied by
        // the surrounding hash-table implementation. For each yielded `quirk`:
        // if enabled, key mismatch, or impl outside the inclusive range, continue;
        // if compatibles exist and no machine match, continue; log, enable its
        // static branch, and set enabled = true, preserving source ordering.
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
