// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2015 Naveen N. Rao, IBM Corporation
 */

// C dependencies removed from executable Rust:
// "dso.h", "symbol.h", "map.h", "probe-event.h", "probe-file.h"

use core::ffi::{c_char, c_int, c_uint, c_uchar};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn map__dso(map: *mut map) -> *mut dso;
    fn map__load(map: *mut map) -> c_int;
    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn get_target_map(target: *const c_char, nsi: *mut nsinfo, uprobes: bool) -> *mut map;

    // External macro/function dependency from PowerPC ELF support.
    fn PPC64_LOCAL_ENTRY_OFFSET(other: c_uchar) -> c_int;

    // Present in the original C only under HAVE_LIBELF_SUPPORT.
    fn kretprobe_offset_is_supported() -> bool;
}

unsafe extern "C" {
    static SYMBOL_A: c_int;
    static SYMBOL_B: c_int;
    static DSO_BINARY_TYPE__KALLSYMS: dso_binary_type;
}

#[repr(C)]
pub struct symbol {
    pub name: *mut c_char,
    pub start: u64,
    pub arch_sym: c_uchar,
}

#[repr(C)]
pub struct perf_probe_event {
    pub point: perf_probe_point,
    pub target: *const c_char,
    pub nsi: *mut nsinfo,
    pub uprobes: bool,
    pub tevs: *mut probe_trace_event,
}

#[repr(C)]
pub struct perf_probe_point {
    pub offset: u64,
    pub retprobe: bool,
}

#[repr(C)]
pub struct probe_trace_event {
    pub point: probe_trace_point,
}

#[repr(C)]
pub struct probe_trace_point {
    pub offset: u64,
    pub address: u64,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    pub symtab_type: dso_binary_type,
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GElf_Sym {
    pub st_other: c_uchar,
}

pub type dso_binary_type = c_int;

#[no_mangle]
pub unsafe extern "C" fn arch__choose_best_symbol(
    syma: *mut symbol,
    _symb: *mut symbol,
) -> c_int {
    let mut sym = unsafe { (*syma).name };

    // Original C condition: #if !defined(_CALL_ELF) || _CALL_ELF != 2
    #[cfg(not(call_elf_2))]
    unsafe {
        /* Skip over any initial dot */
        if *sym == b'.' as c_char {
            sym = sym.add(1);
        }
    }

    /* Avoid "SyS" kernel syscall aliases */
    if unsafe { strlen(sym) } >= 3
        && unsafe { strncmp(sym, c"SyS".as_ptr(), 3) } == 0
    {
        return unsafe { SYMBOL_B };
    }
    if unsafe { strlen(sym) } >= 10
        && unsafe { strncmp(sym, c"compat_SyS".as_ptr(), 10) } == 0
    {
        return unsafe { SYMBOL_B };
    }

    unsafe { SYMBOL_A }
}

// Original C condition: #if !defined(_CALL_ELF) || _CALL_ELF != 2
/* Allow matching against dot variants */
#[cfg(not(call_elf_2))]
#[no_mangle]
pub unsafe extern "C" fn arch__compare_symbol_names(
    mut namea: *const c_char,
    mut nameb: *const c_char,
) -> c_int {
    /* Skip over initial dot */
    unsafe {
        if *namea == b'.' as c_char {
            namea = namea.add(1);
        }
        if *nameb == b'.' as c_char {
            nameb = nameb.add(1);
        }

        strcmp(namea, nameb)
    }
}

#[cfg(not(call_elf_2))]
#[no_mangle]
pub unsafe extern "C" fn arch__compare_symbol_names_n(
    mut namea: *const c_char,
    mut nameb: *const c_char,
    n: c_uint,
) -> c_int {
    /* Skip over initial dot */
    unsafe {
        if *namea == b'.' as c_char {
            namea = namea.add(1);
        }
        if *nameb == b'.' as c_char {
            nameb = nameb.add(1);
        }

        strncmp(namea, nameb, n as usize)
    }
}

#[cfg(not(call_elf_2))]
#[no_mangle]
pub unsafe extern "C" fn arch__normalize_symbol_name(
    mut name: *const c_char,
) -> *const c_char {
    /* Skip over initial dot */
    unsafe {
        if !name.is_null() && *name == b'.' as c_char {
            name = name.add(1);
        }
    }
    name
}

// Original C condition: #if defined(_CALL_ELF) && _CALL_ELF == 2

// Original C condition: #ifdef HAVE_LIBELF_SUPPORT
#[cfg(all(call_elf_2, have_libelf_support))]
#[no_mangle]
pub unsafe extern "C" fn arch__sym_update(s: *mut symbol, sym: *mut GElf_Sym) {
    unsafe {
        (*s).arch_sym = (*sym).st_other;
    }
}

const PPC64LE_LEP_OFFSET: c_int = 8;

#[cfg(call_elf_2)]
#[no_mangle]
pub unsafe extern "C" fn arch__fix_tev_from_maps(
    pev: *mut perf_probe_event,
    tev: *mut probe_trace_event,
    map: *mut map,
    sym: *mut symbol,
) {
    let lep_offset: c_int;

    /*
     * When probing at a function entry point, we normally always want the
     * LEP since that catches calls to the function through both the GEP and
     * the LEP. Hence, we would like to probe at an offset of 8 bytes if
     * the user only specified the function entry.
     *
     * However, if the user specifies an offset, we fall back to using the
     * GEP since all userspace applications (objdump/readelf) show function
     * disassembly with offsets from the GEP.
     */
    unsafe {
        if (*pev).point.offset != 0 || map.is_null() || sym.is_null() {
            return;
        }

        /* For kretprobes, add an offset only if the kernel supports it */
        if !(*pev).uprobes && (*pev).point.retprobe {
            // Original C condition: #ifdef HAVE_LIBELF_SUPPORT
            #[cfg(have_libelf_support)]
            {
                if !kretprobe_offset_is_supported() {
                    return;
                }
            }
            #[cfg(not(have_libelf_support))]
            {
                return;
            }
        }

        lep_offset = PPC64_LOCAL_ENTRY_OFFSET((*sym).arch_sym);

        if (*map__dso(map)).symtab_type == DSO_BINARY_TYPE__KALLSYMS {
            (*tev).point.offset = (*tev)
                .point
                .offset
                .wrapping_add(PPC64LE_LEP_OFFSET as u64);
        } else if lep_offset != 0 {
            if (*pev).uprobes {
                (*tev).point.address = (*tev).point.address.wrapping_add(lep_offset as u64);
            } else {
                (*tev).point.offset = (*tev).point.offset.wrapping_add(lep_offset as u64);
            }
        }
    }
}

// Original C condition: #ifdef HAVE_LIBELF_SUPPORT
#[cfg(all(call_elf_2, have_libelf_support))]
#[no_mangle]
pub unsafe extern "C" fn arch__post_process_probe_trace_events(
    pev: *mut perf_probe_event,
    ntevs: c_int,
) {
    let mut tev: *mut probe_trace_event;
    let mut map: *mut map;
    let mut sym: *mut symbol = core::ptr::null_mut();
    let mut tmp: *mut rb_node;
    let mut i: c_int = 0;

    unsafe {
        map = get_target_map((*pev).target, (*pev).nsi, (*pev).uprobes);
        if map.is_null() || map__load(map) < 0 {
            return;
        }

        while i < ntevs {
            tev = (*pev).tevs.offset(i as isize);

            // Original C used map__for_each_symbol(map, sym, tmp).
            // The iterator macro is supplied by external dependencies.
            while map__for_each_symbol(map, &mut sym, &mut tmp) {
                if map__unmap_ip(map, (*sym).start) == (*tev).point.address {
                    arch__fix_tev_from_maps(pev, tev, map, sym);
                    break;
                }
            }
            i += 1;
        }
    }
}

#[cfg(all(call_elf_2, have_libelf_support))]
unsafe extern "C" {
    fn map__for_each_symbol(
        map: *mut map,
        sym: *mut *mut symbol,
        tmp: *mut *mut rb_node,
    ) -> bool;
}
