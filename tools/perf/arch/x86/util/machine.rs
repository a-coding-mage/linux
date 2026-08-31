// SPDX-License-Identifier: GPL-2.0
// Dependencies translated from:
// <linux/types.h>, <linux/string.h>, <limits.h>, <stdlib.h>,
// <internal/lib.h>, "../../../util/machine.h", "../../../util/map.h",
// "../../../util/symbol.h", <linux/ctype.h>, <symbol/kallsyms.h>

#[cfg(target_arch = "x86_64")]
use core::ffi::{c_char, c_int, c_void};

#[cfg(target_arch = "x86_64")]
type u64 = u64;

#[cfg(target_arch = "x86_64")]
const STB_GLOBAL: c_int = 1;

#[cfg(target_arch = "x86_64")]
const PATH_MAX: usize = 4096;

#[cfg(target_arch = "x86_64")]
extern "C" {
    static page_size: u64;

    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlcpy(dst: *mut c_char, src: *const c_char, siz: usize) -> usize;

    fn kallsyms2elf_binding(typ: c_char) -> c_int;
    fn is_entry_trampoline(name: *const c_char) -> bool;
    fn machine__get_kallsyms_filename(machine: *mut machine, filename: *mut c_char, size: usize);
    fn symbol__restricted_filename(filename: *const c_char, fallback: *const c_char) -> bool;
    fn kallsyms__parse(
        filename: *const c_char,
        arg: *mut c_void,
        process_symbol: Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_char, u64) -> c_int>,
    ) -> c_int;
    fn machine__create_extra_kernel_map(
        machine: *mut machine,
        kernel: *mut dso,
        xm: *mut extra_kernel_map,
    ) -> c_int;
}

#[cfg(target_arch = "x86_64")]
const KMAP_NAME_LEN: usize = 256; // From external KMAP_NAME_LEN macro in "../../../util/map.h".

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct machine {
    // Other fields are supplied by "../../../util/machine.h".
    pub trampolines_mapped: c_int,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct extra_kernel_map {
    pub start: u64,
    pub end: u64,
    pub pgoff: u64,
    pub name: [c_char; KMAP_NAME_LEN],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct extra_kernel_map_info {
    cnt: c_int,
    max_cnt: c_int,
    maps: *mut extra_kernel_map,
    get_entry_trampolines: bool,
    entry_trampoline: u64,
}

#[cfg(target_arch = "x86_64")]
unsafe fn extra_kernel_map_name(xm: *mut extra_kernel_map) -> *mut c_char {
    (*xm).name.as_mut_ptr()
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn add_extra_kernel_map(
    mi: *mut extra_kernel_map_info,
    start: u64,
    end: u64,
    pgoff: u64,
    name: *const c_char,
) -> c_int {
    if (*mi).cnt >= (*mi).max_cnt {
        let buf: *mut c_void;
        let sz: usize;

        (*mi).max_cnt = if (*mi).max_cnt != 0 {
            (*mi).max_cnt * 2
        } else {
            32
        };
        sz = core::mem::size_of::<extra_kernel_map>() * (*mi).max_cnt as usize;
        buf = realloc((*mi).maps as *mut c_void, sz);
        if buf.is_null() {
            return -1;
        }
        (*mi).maps = buf as *mut extra_kernel_map;
    }

    (*(*mi).maps.add((*mi).cnt as usize)).start = start;
    (*(*mi).maps.add((*mi).cnt as usize)).end = end;
    (*(*mi).maps.add((*mi).cnt as usize)).pgoff = pgoff;
    strlcpy(
        extra_kernel_map_name((*mi).maps.add((*mi).cnt as usize)),
        name,
        KMAP_NAME_LEN,
    );

    (*mi).cnt += 1;

    0
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn find_extra_kernel_maps(
    arg: *mut c_void,
    name: *const c_char,
    typ: c_char,
    start: u64,
) -> c_int {
    let mi: *mut extra_kernel_map_info = arg as *mut extra_kernel_map_info;

    if (*mi).entry_trampoline == 0
        && kallsyms2elf_binding(typ) == STB_GLOBAL
        && strcmp(name, b"_entry_trampoline\0".as_ptr() as *const c_char) == 0
    {
        (*mi).entry_trampoline = start;
        return 0;
    }

    if is_entry_trampoline(name) {
        let end: u64 = start + page_size;

        return add_extra_kernel_map(mi, start, end, 0, name);
    }

    0
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub unsafe extern "C" fn machine__create_extra_kernel_maps(
    machine: *mut machine,
    kernel: *mut dso,
) -> c_int {
    let mut mi: extra_kernel_map_info = extra_kernel_map_info {
        cnt: 0,
        max_cnt: 0,
        maps: core::ptr::null_mut(),
        get_entry_trampolines: false,
        entry_trampoline: 0,
    };
    let mut filename: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut ret: c_int;
    let mut i: c_int;

    machine__get_kallsyms_filename(machine, filename.as_mut_ptr(), PATH_MAX);

    if symbol__restricted_filename(
        filename.as_ptr(),
        b"/proc/kallsyms\0".as_ptr() as *const c_char,
    ) {
        return 0;
    }

    ret = kallsyms__parse(
        filename.as_ptr(),
        &mut mi as *mut extra_kernel_map_info as *mut c_void,
        Some(find_extra_kernel_maps),
    );
    if ret != 0 {
        free(mi.maps as *mut c_void);
        return ret;
    }

    if mi.entry_trampoline == 0 {
        free(mi.maps as *mut c_void);
        return ret;
    }

    i = 0;
    while i < mi.cnt {
        let xm: *mut extra_kernel_map = mi.maps.add(i as usize);

        (*xm).pgoff = mi.entry_trampoline;
        ret = machine__create_extra_kernel_map(machine, kernel, xm);
        if ret != 0 {
            free(mi.maps as *mut c_void);
            return ret;
        }
        i += 1;
    }

    (*machine).trampolines_mapped = mi.cnt;

    free(mi.maps as *mut c_void);
    ret
}
