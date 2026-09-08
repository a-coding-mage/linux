// SPDX-License-Identifier: GPL-2.0

/*
 * Helper functions for finding the symbol in an ELF which is "nearest"
 * to a given address.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct syminfo {
    pub symbol_index: u32,
    pub section_index: u32,
    pub addr: Elf_Addr,
}

#[repr(C)]
pub struct symsearch {
    pub table_size: u32,
    pub table: [syminfo; 0],
}

extern "C" {
    fn xmalloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fatal(fmt: *const u8, ...);
    fn is_valid_name(elf: *mut elf_info, sym: *mut Elf_Sym) -> bool;
    fn get_secindex(elf: *mut elf_info, sym: *mut Elf_Sym) -> u32;
    fn qsort(base: *mut c_void, nmemb: usize, size: usize,
             compar: unsafe extern "C" fn(*const c_void, *const c_void) -> i32);
}

unsafe extern "C" fn syminfo_compare(s1: *const c_void, s2: *const c_void) -> i32 {
    let sym1 = &*(s1 as *const syminfo);
    let sym2 = &*(s2 as *const syminfo);

    if sym1.section_index > sym2.section_index { return 1; }
    if sym1.section_index < sym2.section_index { return -1; }
    if sym1.addr > sym2.addr { return 1; }
    if sym1.addr < sym2.addr { return -1; }
    if sym1.symbol_index > sym2.symbol_index { return 1; }
    if sym1.symbol_index < sym2.symbol_index { return -1; }
    0
}

unsafe fn symbol_count(elf: *mut elf_info) -> u32 {
    let mut result = 0;
    let mut sym = (*elf).symtab_start;
    while sym < (*elf).symtab_stop {
        if is_valid_name(elf, sym) { result += 1; }
        sym = sym.add(1);
    }
    result
}

unsafe fn symsearch_populate(elf: *mut elf_info, mut table: *mut syminfo,
                             mut table_size: u32) {
    let is_arm = (*(*elf).hdr).e_machine == EM_ARM;
    let mut sym = (*elf).symtab_start;
    while sym < (*elf).symtab_stop {
        if is_valid_name(elf, sym) {
            if table_size == 0 { fatal(b"%s: size mismatch\n\0".as_ptr(), b"symsearch_populate\0".as_ptr()); }
            table_size -= 1;
            (*table).symbol_index = sym.offset_from((*elf).symtab_start) as u32;
            (*table).section_index = get_secindex(elf, sym);
            (*table).addr = (*sym).st_value;
            if is_arm && ELF_ST_TYPE((*sym).st_info) == STT_FUNC {
                (*table).addr &= !1;
            }
            table = table.add(1);
        }
        sym = sym.add(1);
    }
    if table_size != 0 { fatal(b"%s: size mismatch\n\0".as_ptr(), b"symsearch_populate\0".as_ptr()); }
}

unsafe fn symsearch_fixup(table: *mut syminfo, table_size: u32) {
    let mut i = 1;
    while i < table_size {
        if (*table.add(i as usize)).addr == (*table.add((i - 1) as usize)).addr &&
           (*table.add(i as usize)).section_index == (*table.add((i - 1) as usize)).section_index {
            (*table.add(i as usize)).symbol_index = (*table.add((i - 1) as usize)).symbol_index;
        }
        i += 1;
    }
}

pub unsafe fn symsearch_init(elf: *mut elf_info) {
    let table_size = symbol_count(elf);
    let size = core::mem::size_of::<symsearch>() +
               core::mem::size_of::<syminfo>() * table_size as usize;
    (*elf).symsearch = xmalloc(size) as *mut symsearch;
    (*(*elf).symsearch).table_size = table_size;
    let table = (*(*elf).symsearch).table.as_mut_ptr();
    symsearch_populate(elf, table, table_size);
    qsort(table as *mut c_void, table_size as usize, core::mem::size_of::<syminfo>(), syminfo_compare);
    symsearch_fixup(table, table_size);
}

pub unsafe fn symsearch_finish(elf: *mut elf_info) {
    free((*elf).symsearch as *mut c_void);
    (*elf).symsearch = core::ptr::null_mut();
}

pub unsafe fn symsearch_find_nearest(elf: *mut elf_info, addr: Elf_Addr,
    secndx: u32, allow_negative: bool, mut min_distance: Elf_Addr) -> *mut Elf_Sym {
    let mut hi = (*(*elf).symsearch).table_size;
    let mut lo = 0;
    let table = (*(*elf).symsearch).table.as_mut_ptr();
    let target = syminfo { addr, section_index: secndx, symbol_index: !0 };
    while hi > lo {
        let mid = lo + (hi - lo) / 2;
        if syminfo_compare(table.add(mid as usize) as *const c_void,
                           &target as *const _ as *const c_void) > 0 { hi = mid; }
        else { lo = mid + 1; }
    }
    let mut result = core::ptr::null_mut();
    if allow_negative && hi < (*(*elf).symsearch).table_size &&
       (*table.add(hi as usize)).section_index == secndx &&
       (*table.add(hi as usize)).addr - addr <= min_distance {
        min_distance = (*table.add(hi as usize)).addr - addr;
        result = (*elf).symtab_start.add((*table.add(hi as usize)).symbol_index as usize);
    }
    if hi > 0 && (*table.add((hi - 1) as usize)).section_index == secndx &&
       addr - (*table.add((hi - 1) as usize)).addr <= min_distance {
        result = (*elf).symtab_start.add((*table.add((hi - 1) as usize)).symbol_index as usize);
    }
    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
