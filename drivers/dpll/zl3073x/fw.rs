// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_void, CStr};
use core::ptr;

const ZL3073X_FW_ERR_PFX: &str = "FW load failed: ";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Zl3073xFlashType {
    None = 0,
    Sectors,
    Page,
    PageAndCopy,
}

#[repr(C)]
struct Zl3073xFwComponentInfo {
    name: *const c_char,
    max_size: usize,
    flash_type: Zl3073xFlashType,
    load_addr: u32,
    dest_page: u32,
    copy_page: u32,
}

// These types, constants, and functions are provided by the surrounding translation.
#[allow(non_camel_case_types)]
type zl3073x_fw_component_id = i32;
#[allow(non_camel_case_types)]
type zl3073x_dev = c_void;
#[allow(non_camel_case_types)]
type netlink_ext_ack = c_void;

#[repr(C)]
struct zl3073x_fw_component {
    id: zl3073x_fw_component_id,
    size: usize,
    data: *mut u32,
}

#[repr(C)]
struct zl3073x_fw {
    component: [*mut zl3073x_fw_component; ZL_FW_NUM_COMPONENTS as usize],
}

extern "C" {
    static ZL_FW_NUM_COMPONENTS: i32;
    static ZL_FW_COMPONENT_INVALID: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_UTIL: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_FW1: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_FW2: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_FW3: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_CFG0: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_CFG1: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_CFG2: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_CFG3: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_CFG4: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_CFG5: zl3073x_fw_component_id;
    static ZL_FW_COMPONENT_CFG6: zl3073x_fw_component_id;

    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn zl3073x_flash_sectors(dev: *mut zl3073x_dev, name: *const c_char, dest_page: u32, load_addr: u32, data: *mut u32, size: usize, extack: *mut netlink_ext_ack) -> c_int;
    fn zl3073x_flash_page(dev: *mut zl3073x_dev, name: *const c_char, dest_page: u32, load_addr: u32, data: *mut u32, size: usize, extack: *mut netlink_ext_ack) -> c_int;
    fn zl3073x_flash_page_copy(dev: *mut zl3073x_dev, name: *const c_char, dest_page: u32, copy_page: u32, extack: *mut netlink_ext_ack) -> c_int;
}

const GFP_KERNEL: u32 = 0;
const EINVAL: isize = 22;
const ENOMEM: isize = 12;
const ENODATA: isize = 61;

static COMPONENT_INFO: &[Zl3073xFwComponentInfo] = &[
    Zl3073xFwComponentInfo { name: b"utility\0".as_ptr() as *const c_char, max_size: 0x4000, flash_type: Zl3073xFlashType::None, load_addr: 0x20000000, dest_page: 0, copy_page: 0 },
    Zl3073xFwComponentInfo { name: b"firmware1\0".as_ptr() as *const c_char, max_size: 0x35000, flash_type: Zl3073xFlashType::Sectors, load_addr: 0x20002000, dest_page: 0x020, copy_page: 0 },
    Zl3073xFwComponentInfo { name: b"firmware2\0".as_ptr() as *const c_char, max_size: 0x0040, flash_type: Zl3073xFlashType::PageAndCopy, load_addr: 0x20000000, dest_page: 0x3e0, copy_page: 0x000 },
    Zl3073xFwComponentInfo { name: b"firmware3\0".as_ptr() as *const c_char, max_size: 0x0248, flash_type: Zl3073xFlashType::PageAndCopy, load_addr: 0x20000400, dest_page: 0x3e4, copy_page: 0x004 },
    Zl3073xFwComponentInfo { name: b"config0\0".as_ptr() as *const c_char, max_size: 0x1000, flash_type: Zl3073xFlashType::Page, load_addr: 0x20000000, dest_page: 0x3d0, copy_page: 0 },
    Zl3073xFwComponentInfo { name: b"config1\0".as_ptr() as *const c_char, max_size: 0x1000, flash_type: Zl3073xFlashType::Page, load_addr: 0x20000000, dest_page: 0x3c0, copy_page: 0 },
    Zl3073xFwComponentInfo { name: b"config2\0".as_ptr() as *const c_char, max_size: 0x1000, flash_type: Zl3073xFlashType::Page, load_addr: 0x20000000, dest_page: 0x3b0, copy_page: 0 },
    Zl3073xFwComponentInfo { name: b"config3\0".as_ptr() as *const c_char, max_size: 0x1000, flash_type: Zl3073xFlashType::Page, load_addr: 0x20000000, dest_page: 0x3a0, copy_page: 0 },
    Zl3073xFwComponentInfo { name: b"config4\0".as_ptr() as *const c_char, max_size: 0x1000, flash_type: Zl3073xFlashType::Page, load_addr: 0x20000000, dest_page: 0x390, copy_page: 0 },
    Zl3073xFwComponentInfo { name: b"config5\0".as_ptr() as *const c_char, max_size: 0x1000, flash_type: Zl3073xFlashType::Page, load_addr: 0x20000000, dest_page: 0x380, copy_page: 0 },
    Zl3073xFwComponentInfo { name: b"config6\0".as_ptr() as *const c_char, max_size: 0x1000, flash_type: Zl3073xFlashType::Page, load_addr: 0x20000000, dest_page: 0x370, copy_page: 0 },
];

unsafe fn zl3073x_fw_component_alloc(size: usize) -> *mut zl3073x_fw_component {
    let comp = kzalloc(core::mem::size_of::<zl3073x_fw_component>(), GFP_KERNEL) as *mut zl3073x_fw_component;
    if comp.is_null() { return ptr::null_mut(); }
    (*comp).size = size;
    (*comp).data = kzalloc(size, GFP_KERNEL) as *mut u32;
    if (*comp).data.is_null() { kfree(comp as *mut c_void); return ptr::null_mut(); }
    comp
}

unsafe fn zl3073x_fw_component_free(comp: *mut zl3073x_fw_component) {
    if !comp.is_null() { kfree((*comp).data as *mut c_void); }
    kfree(comp as *mut c_void);
}

unsafe fn zl3073x_fw_component_id_get(_name: *const c_char) -> zl3073x_fw_component_id {
    if _name.is_null() { return ZL_FW_COMPONENT_INVALID; }
    let input = CStr::from_ptr(_name).to_bytes().to_ascii_lowercase();
    for (i, info) in COMPONENT_INFO.iter().enumerate() {
        if input == CStr::from_ptr(info.name).to_bytes() { return i as zl3073x_fw_component_id; }
    }
    ZL_FW_COMPONENT_INVALID
}

pub unsafe fn zl3073x_fw_free(fw: *mut zl3073x_fw) {
    if fw.is_null() { return; }
    for i in 0..ZL_FW_NUM_COMPONENTS as usize { zl3073x_fw_component_free((*fw).component[i]); }
    kfree(fw as *mut c_void);
}

pub unsafe fn zl3073x_fw_load(_zldev: *mut zl3073x_dev, mut data: *const c_char, mut size: usize, _extack: *mut netlink_ext_ack) -> *mut zl3073x_fw {
    let fw = kzalloc(core::mem::size_of::<zl3073x_fw>(), GFP_KERNEL) as *mut zl3073x_fw;
    if fw.is_null() { return ptr::null_mut(); }
    while size != 0 {
        let bytes = core::slice::from_raw_parts(data as *const u8, size);
        let end = bytes.iter().position(|&b| b == 0 || b == b'\n').unwrap_or(size);
        let line = &bytes[..end];
        let mut fields = line.split(|&b| b == b' ' || b == b'\t');
        let name = match fields.next() { Some(v) if !v.is_empty() => v, _ => break };
        let count = match fields.next().and_then(|v| core::str::from_utf8(v).ok()).and_then(|v| v.parse::<usize>().ok()) { Some(v) => v, None => { zl3073x_fw_free(fw); return ptr::null_mut(); } };
        let name_buf = [name, &[0]].concat();
        let id = zl3073x_fw_component_id_get(name_buf.as_ptr() as *const c_char);
        if id == ZL_FW_COMPONENT_INVALID || id < 0 || id as usize >= COMPONENT_INFO.len() || fw.as_ref().unwrap().component[id as usize] != ptr::null_mut() { zl3073x_fw_free(fw); return ptr::null_mut(); }
        let comp = zl3073x_fw_component_alloc(count.wrapping_mul(core::mem::size_of::<u32>()));
        if comp.is_null() || (*comp).size > COMPONENT_INFO[id as usize].max_size { zl3073x_fw_component_free(comp); zl3073x_fw_free(fw); return ptr::null_mut(); }
        (*comp).id = id;
        let mut rest = &line[name.len()..];
        while rest.first().is_some_and(|b| b.is_ascii_whitespace()) { rest = &rest[1..]; }
        while rest.first().is_some_and(|b| b.is_ascii_digit()) { rest = &rest[1..]; }
        for n in 0..count {
            while rest.first().is_some_and(|b| b.is_ascii_whitespace()) { rest = &rest[1..]; }
            let take = rest.iter().position(|b| b.is_ascii_whitespace()).unwrap_or(rest.len());
            let word = &rest[..take];
            let text = match core::str::from_utf8(word).ok().and_then(|v| u32::from_str_radix(v, 16).ok()) { Some(v) => v, None => { zl3073x_fw_component_free(comp); zl3073x_fw_free(fw); return ptr::null_mut(); } };
            *(*comp).data.add(n) = text; rest = &rest[take..];
        }
        (*fw).component[id as usize] = comp;
        let consumed = end.saturating_add(if end < size && bytes[end] == b'\n' { 1 } else { 0 });
        data = data.add(consumed); size -= consumed;
    }
    fw
}

unsafe fn zl3073x_fw_component_flash(zldev: *mut zl3073x_dev, comp: *mut zl3073x_fw_component, extack: *mut netlink_ext_ack) -> c_int {
    let info = &COMPONENT_INFO[(*comp).id as usize];
    let mut rc = match info.flash_type {
        Zl3073xFlashType::None => return 0,
        Zl3073xFlashType::Sectors => zl3073x_flash_sectors(zldev, info.name, info.dest_page, info.load_addr, (*comp).data, (*comp).size, extack),
        Zl3073xFlashType::Page | Zl3073xFlashType::PageAndCopy => zl3073x_flash_page(zldev, info.name, info.dest_page, info.load_addr, (*comp).data, (*comp).size, extack),
    };
    if rc == 0 && info.flash_type == Zl3073xFlashType::PageAndCopy { rc = zl3073x_flash_page_copy(zldev, info.name, info.dest_page, info.copy_page, extack); }
    rc
}

pub unsafe fn zl3073x_fw_flash(zldev: *mut zl3073x_dev, zlfw: *mut zl3073x_fw, extack: *mut netlink_ext_ack) -> c_int {
    let mut rc = 0;
    for i in 0..ZL_FW_NUM_COMPONENTS as usize {
        if (*zlfw).component[i].is_null() { continue; }
        rc = zl3073x_fw_component_flash(zldev, (*zlfw).component[i], extack);
        if rc != 0 { break; }
    }
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
