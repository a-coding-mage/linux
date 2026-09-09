// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014, The Linux Foundation. All rights reserved.
 * Debug helper to dump the current kernel pagetables of the system
 * so that we can see what the various memory ranges are set to.
 *
 * Derived from x86 and arm implementation:
 * (C) Copyright 2008 Intel Corporation
 *
 * Author: Arjan van de Ven <arjan@linux.intel.com>
 */

// Linux and architecture headers supplying the following external symbols are
// intentionally omitted; they are provided by the surrounding kernel crate.

#[repr(C)]
pub struct ptdump_prot_bits {
    pub mask: ptval_t,
    pub val: ptval_t,
    pub set: *const core::ffi::c_char,
    pub clear: *const core::ffi::c_char,
}

#[repr(C)]
pub struct ptdump_pg_level {
    pub name: *const core::ffi::c_char,
    pub bits: *const ptdump_prot_bits,
    pub num: usize,
    pub mask: ptval_t,
}

#[repr(C)]
pub struct ptdump_pg_state {
    pub seq: *mut seq_file,
    pub marker: *mut addr_marker,
    pub mm: *mut mm_struct,
    pub pg_level: *mut ptdump_pg_level,
    pub end_address: usize,
    pub level: i32,
    pub current_prot: ptval_t,
    pub start_address: usize,
    pub check_wx: bool,
    pub wx_pages: usize,
    pub uxn_pages: usize,
    pub ptdump: ptdump_state,
}

// External kernel types and constants.
pub type ptval_t = u64;
pub type pteval_t = u64;
pub type pte_t = u64;
pub type pmd_t = u64;
pub type pud_t = u64;
pub type p4d_t = u64;
pub type pgd_t = u64;
pub enum seq_file {}
pub enum mm_struct {}

#[repr(C)]
pub struct addr_marker { pub start_address: usize, pub name: *const core::ffi::c_char }
#[repr(C)]
pub struct ptdump_range { pub start: usize, pub end: usize }
#[repr(C)]
pub struct ptdump_state {
    pub note_page_pte: Option<unsafe extern "C" fn(*mut ptdump_state, usize, pte_t)>,
    pub note_page_pmd: Option<unsafe extern "C" fn(*mut ptdump_state, usize, pmd_t)>,
    pub note_page_pud: Option<unsafe extern "C" fn(*mut ptdump_state, usize, pud_t)>,
    pub note_page_p4d: Option<unsafe extern "C" fn(*mut ptdump_state, usize, p4d_t)>,
    pub note_page_pgd: Option<unsafe extern "C" fn(*mut ptdump_state, usize, pgd_t)>,
    pub note_page_flush: Option<unsafe extern "C" fn(*mut ptdump_state)>,
    pub range: *mut ptdump_range,
}
#[repr(C)]
pub struct ptdump_info { pub mm: *mut mm_struct, pub markers: *mut addr_marker, pub base_addr: usize }

extern "C" {
    static mut init_mm: mm_struct;
    fn seq_printf(m: *mut seq_file, fmt: *const core::ffi::c_char, ...);
    fn ptdump_walk_pgd(st: *mut ptdump_state, mm: *mut mm_struct, vma: *mut core::ffi::c_void);
    fn ptdump_debugfs_register(info: *mut ptdump_info, name: *const core::ffi::c_char) -> i32;
    fn virt_to_page(addr: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn mm_p4d_folded(mm: *mut mm_struct) -> bool;
    fn mm_pud_folded(mm: *mut mm_struct) -> bool;
    fn warn_once(cond: bool, fmt: *const core::ffi::c_char, ...);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

extern "C" {
    static PTE_VALID: ptval_t; static PTE_USER: ptval_t; static PTE_RDONLY: ptval_t;
    static PTE_PXN: ptval_t; static PTE_SHARED: ptval_t; static PTE_AF: ptval_t;
    static PTE_NG: ptval_t; static PTE_CONT: ptval_t; static PMD_TYPE_MASK: ptval_t;
    static PMD_TYPE_SECT: ptval_t; static PTE_UXN: ptval_t; static PTE_GP: ptval_t;
    static PTE_ATTRINDX_MASK: ptval_t; static MT_DEVICE_nGnRnE: u64;
    static MT_DEVICE_nGnRE: u64; static MT_NORMAL_NC: u64; static MT_NORMAL: u64;
    static MT_NORMAL_TAGGED: u64; static PAGE_SIZE: usize; static ULONG_MAX: usize;
    static TASK_SIZE_64: usize; static PAGE_OFFSET: usize; static PAGE_END: usize;
    static MODULES_VADDR: usize; static MODULES_END: usize; static VMALLOC_START: usize;
    static VMALLOC_END: usize; static VMEMMAP_END: usize; static PCI_IO_START: usize;
    static PCI_IO_END: usize; static FIXADDR_TOT_START: usize; static FIXADDR_TOP: usize;
    static vabits_actual: u32;
    fn pte_val(v: pte_t) -> pteval_t; fn pmd_val(v: pmd_t) -> pteval_t;
    fn pud_val(v: pud_t) -> pteval_t; fn p4d_val(v: p4d_t) -> pteval_t;
    fn pgd_val(v: pgd_t) -> pteval_t; fn page_offset(v: u32) -> usize;
    fn pte_attrindx(v: u64) -> ptval_t;
}

static mut pte_bits: [ptdump_prot_bits; 16] = [
    ptdump_prot_bits { mask: 0, val: 0, set: b" \0".as_ptr() as _, clear: b"F\0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"USR\0".as_ptr() as _, clear: b"   \0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"ro\0".as_ptr() as _, clear: b"RW\0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"NX\0".as_ptr() as _, clear: b"x \0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"SHD\0".as_ptr() as _, clear: b"   \0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"AF\0".as_ptr() as _, clear: b"  \0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"NG\0".as_ptr() as _, clear: b"  \0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"CON\0".as_ptr() as _, clear: b"   \0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"BLK\0".as_ptr() as _, clear: b"   \0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"UXN\0".as_ptr() as _, clear: b"   \0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"GP\0".as_ptr() as _, clear: b"  \0".as_ptr() as _ },
    ptdump_prot_bits { mask: 0, val: 0, set: b"DEVICE/nGnRnE\0".as_ptr() as _, clear: core::ptr::null() },
    ptdump_prot_bits { mask: 0, val: 0, set: b"DEVICE/nGnRE\0".as_ptr() as _, clear: core::ptr::null() },
    ptdump_prot_bits { mask: 0, val: 0, set: b"MEM/NORMAL-NC\0".as_ptr() as _, clear: core::ptr::null() },
    ptdump_prot_bits { mask: 0, val: 0, set: b"MEM/NORMAL\0".as_ptr() as _, clear: core::ptr::null() },
    ptdump_prot_bits { mask: 0, val: 0, set: b"MEM/NORMAL-TAGGED\0".as_ptr() as _, clear: core::ptr::null() },
];

static mut kernel_pg_levels: [ptdump_pg_level; 5] = [
    ptdump_pg_level { name: b"PGD\0".as_ptr() as _, bits: core::ptr::null(), num: 16, mask: 0 },
    ptdump_pg_level { name: b"P4D\0".as_ptr() as _, bits: core::ptr::null(), num: 16, mask: 0 },
    ptdump_pg_level { name: b"PUD\0".as_ptr() as _, bits: core::ptr::null(), num: 16, mask: 0 },
    ptdump_pg_level { name: b"PMD\0".as_ptr() as _, bits: core::ptr::null(), num: 16, mask: 0 },
    ptdump_pg_level { name: b"PTE\0".as_ptr() as _, bits: core::ptr::null(), num: 16, mask: 0 },
];

unsafe fn pt_dump_seq_printf(m: *mut seq_file, fmt: *const core::ffi::c_char, args: ...) { if !m.is_null() { seq_printf(m, fmt, args); } }
unsafe fn pt_dump_seq_puts(m: *mut seq_file, fmt: *const core::ffi::c_char) { if !m.is_null() { seq_printf(m, fmt); } }

unsafe fn dump_prot(st: *mut ptdump_pg_state, mut bits: *const ptdump_prot_bits, num: usize) {
    for _ in 0..num { let b = &*bits; let s = if ((*st).current_prot & b.mask) == b.val { b.set } else { b.clear }; if !s.is_null() { pt_dump_seq_printf((*st).seq, b" %s\0".as_ptr() as _, s); } bits = bits.add(1); }
}

unsafe fn note_prot_uxn(st: *mut ptdump_pg_state, addr: usize) { if !(*st).check_wx || ((*st).current_prot & 0) != 0 { return; } warn_once(false, b"arm64/mm: Found non-UXN mapping at address %p/%pS\n\0".as_ptr() as _, (*st).start_address as *mut _, (*st).start_address as *mut _); (*st).uxn_pages += (addr - (*st).start_address) / PAGE_SIZE; }
unsafe fn note_prot_wx(st: *mut ptdump_pg_state, addr: usize) { if !(*st).check_wx { return; } if ((*st).current_prot & 0) != 0 || ((*st).current_prot & 0) != 0 { return; } warn_once(false, b"arm64/mm: Found insecure W+X mapping at address %p/%pS\n\0".as_ptr() as _, (*st).start_address as *mut _, (*st).start_address as *mut _); (*st).wx_pages += (addr - (*st).start_address) / PAGE_SIZE; }

pub unsafe extern "C" fn note_page(pt_st: *mut ptdump_state, addr: usize, mut level: i32, val: pteval_t) {
    let st = pt_st as *mut ptdump_pg_state;
    let pg_level = (*st).pg_level;
    let units = b"KMGTPE\0";
    let mut prot: ptval_t = 0;
    if !(*st).mm.is_null() && ((level == 1 && mm_p4d_folded((*st).mm)) || (level == 2 && mm_pud_folded((*st).mm))) { level = 0; }
    if level >= 0 { prot = val & (*pg_level.add(level as usize)).mask; }
    if (*st).level == -1 {
        (*st).level = level; (*st).current_prot = prot; (*st).start_address = addr;
        pt_dump_seq_printf((*st).seq, b"---[ %s ]---\n\0".as_ptr() as _, (*(*st).marker).name);
    } else if prot != (*st).current_prot || level != (*st).level || addr >= (*st).marker.add(1).start_address {
        if (*st).current_prot { note_prot_uxn(st, addr); note_prot_wx(st, addr); }
        pt_dump_seq_printf((*st).seq, b"0x%016lx-0x%016lx   \0".as_ptr() as _, (*st).start_address, addr);
        let mut delta = (addr - (*st).start_address) >> 10; let mut unit = units.as_ptr();
        while delta & 1023 == 0 && *unit.add(1) != 0 { delta >>= 10; unit = unit.add(1); }
        let cur = &*pg_level.add((*st).level as usize);
        pt_dump_seq_printf((*st).seq, b"%9lu%c %s\0".as_ptr() as _, delta, *unit, cur.name);
        if (*st).current_prot && !cur.bits.is_null() { dump_prot(st, cur.bits, cur.num); }
        pt_dump_seq_puts((*st).seq, b"\n\0".as_ptr() as _);
        if addr >= (*st).marker.add(1).start_address { (*st).marker = (*st).marker.add(1); pt_dump_seq_printf((*st).seq, b"---[ %s ]---\n\0".as_ptr() as _, (*(*st).marker).name); }
        (*st).start_address = addr; (*st).current_prot = prot; (*st).level = level;
    }
    if addr >= (*st).marker.add(1).start_address { (*st).marker = (*st).marker.add(1); pt_dump_seq_printf((*st).seq, b"---[ %s ]---\n\0".as_ptr() as _, (*(*st).marker).name); }
}
pub unsafe extern "C" fn note_page_pte(s: *mut ptdump_state, a: usize, v: pte_t) { note_page(s, a, 4, pte_val(v)); }
pub unsafe extern "C" fn note_page_pmd(s: *mut ptdump_state, a: usize, v: pmd_t) { note_page(s, a, 3, pmd_val(v)); }
pub unsafe extern "C" fn note_page_pud(s: *mut ptdump_state, a: usize, v: pud_t) { note_page(s, a, 2, pud_val(v)); }
pub unsafe extern "C" fn note_page_p4d(s: *mut ptdump_state, a: usize, v: p4d_t) { note_page(s, a, 1, p4d_val(v)); }
pub unsafe extern "C" fn note_page_pgd(s: *mut ptdump_state, a: usize, v: pgd_t) { note_page(s, a, 0, pgd_val(v)); }
pub unsafe extern "C" fn note_page_flush(s: *mut ptdump_state) { let st = s as *mut ptdump_pg_state; let mut end = (*st).end_address; if end == ULONG_MAX { end = 0; } note_page(s, end, -1, 0); }

pub unsafe extern "C" fn ptdump_walk(s: *mut seq_file, info: *mut ptdump_info) {
    let end = if (*info).base_addr < TASK_SIZE_64 { TASK_SIZE_64 } else { !0usize };
    let mut st: ptdump_pg_state = core::mem::zeroed(); st.seq = s; st.marker = (*info).markers; st.mm = (*info).mm; st.pg_level = kernel_pg_levels.as_mut_ptr(); st.end_address = end; st.level = -1;
    st.ptdump = ptdump_state { note_page_pte: Some(note_page_pte), note_page_pmd: Some(note_page_pmd), note_page_pud: Some(note_page_pud), note_page_p4d: Some(note_page_p4d), note_page_pgd: Some(note_page_pgd), note_page_flush: Some(note_page_flush), range: core::ptr::null_mut() };
    ptdump_walk_pgd(&mut st.ptdump, (*info).mm, core::ptr::null_mut());
}

unsafe fn ptdump_initialize() { for i in 0..5 { let l = &mut kernel_pg_levels[i]; l.bits = pte_bits.as_ptr(); l.mask = 0; for j in 0..l.num { l.mask |= (*l.bits.add(j)).mask; } } }
static mut kernel_ptdump_info: ptdump_info = ptdump_info { mm: core::ptr::null_mut(), markers: core::ptr::null_mut(), base_addr: 0 };

pub unsafe extern "C" fn ptdump_check_wx() -> bool { let mut st: ptdump_pg_state = core::mem::zeroed(); st.marker = core::ptr::null_mut(); st.pg_level = kernel_pg_levels.as_mut_ptr(); st.end_address = !0usize; st.level = -1; st.check_wx = true; ptdump_walk_pgd(&mut st.ptdump, &mut init_mm, core::ptr::null_mut()); if st.wx_pages != 0 || st.uxn_pages != 0 { pr_warn(b"Checked W+X mappings: FAILED, %lu W+X pages found, %lu non-UXN pages found\n\0".as_ptr() as _, st.wx_pages, st.uxn_pages); false } else { pr_info(b"Checked W+X mappings: passed, no W+X pages found\n\0".as_ptr() as _); true } }

pub unsafe extern "C" fn ptdump_init() -> i32 { let page_offset = page_offset(vabits_actual); let _vmemmap_start = virt_to_page(page_offset as *const _); kernel_ptdump_info.mm = &mut init_mm; kernel_ptdump_info.base_addr = page_offset; ptdump_initialize(); ptdump_debugfs_register(&mut kernel_ptdump_info, b"kernel_page_tables\0".as_ptr() as _); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
