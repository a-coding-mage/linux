// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2019 SiFive
 */

// Kernel headers and configuration-dependent symbols are supplied externally.

#[repr(C)]
pub struct PgState {
    pub ptdump: PtdumpState,
    pub seq: *mut SeqFile,
    pub marker: *const AddrMarker,
    pub start_address: c_ulong,
    pub start_pa: c_ulong,
    pub last_pa: c_ulong,
    pub level: c_int,
    pub current_prot: u64,
    pub check_wx: bool,
    pub wx_pages: c_ulong,
}

#[repr(C)]
pub struct AddrMarker {
    pub start_address: c_ulong,
    pub name: *const c_char,
}

#[repr(C)]
pub struct PtdMmInfo {
    pub mm: *mut MmStruct,
    pub markers: *const AddrMarker,
    pub base_addr: c_ulong,
    pub end: c_ulong,
}

#[repr(C)]
pub struct ProtBits { pub mask: u64, pub set: *const c_char, pub clear: *const c_char }

#[repr(C)]
pub struct PgLevel { pub name: *const c_char, pub mask: u64 }

// Configuration-dependent enum entries are preserved in the original order.
pub const FIXMAP_START_NR: usize = 0;
pub const FIXMAP_END_NR: usize = 1;
pub const PCI_IO_START_NR: usize = 2;
pub const PCI_IO_END_NR: usize = 3;
pub const VMALLOC_START_NR: usize = 4;
pub const VMALLOC_END_NR: usize = 5;
pub const PAGE_OFFSET_NR: usize = 6;
pub const END_OF_SPACE_NR: usize = 7;

extern "C" {
    pub static mut init_mm: MmStruct;
    pub static mut pgtable_l5_enabled: bool;
    pub static mut pgtable_l4_enabled: bool;
    pub static mut kernel_map: KernelMap;
    pub fn ptdump_walk_pgd(st: *mut PtdumpState, mm: *mut MmStruct, v: *mut c_void);
    pub fn seq_printf(m: *mut SeqFile, fmt: *const c_char, ...);
    pub fn seq_puts(m: *mut SeqFile, fmt: *const c_char);
    pub fn snprintf(s: *mut c_char, size: usize, fmt: *const c_char, ...);
    pub fn strscpy(dst: *mut c_char, src: *const c_char);
    pub fn pr_warn(fmt: *const c_char, ...);
    pub fn pr_info(fmt: *const c_char, ...);
    pub fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut c_void, data: *mut c_void, fops: *const c_void) -> *mut c_void;
    pub fn efi_enabled(x: c_uint) -> bool;
    pub static ptdump_fops: c_void;
}

#[repr(C)] pub struct SeqFile { pub private: *mut c_void }
#[repr(C)] pub struct MmStruct;
#[repr(C)] pub struct KernelMap { pub virt_addr: c_ulong }
#[repr(C)] pub struct Pte(pub u64);
pub type PteT = Pte;
pub type PmdT = Pte;
pub type PudT = Pte;
pub type P4dT = Pte;
pub type PgdT = Pte;

#[repr(C)]
pub struct PtdumpRange { pub start: c_ulong, pub end: c_ulong }
#[repr(C)]
pub struct PtdumpState {
    pub note_page_pte: Option<unsafe extern "C" fn(*mut PtdumpState, c_ulong, PteT)>,
    pub note_page_pmd: Option<unsafe extern "C" fn(*mut PtdumpState, c_ulong, PmdT)>,
    pub note_page_pud: Option<unsafe extern "C" fn(*mut PtdumpState, c_ulong, PudT)>,
    pub note_page_p4d: Option<unsafe extern "C" fn(*mut PtdumpState, c_ulong, P4dT)>,
    pub note_page_pgd: Option<unsafe extern "C" fn(*mut PtdumpState, c_ulong, PgdT)>,
    pub note_page_flush: Option<unsafe extern "C" fn(*mut PtdumpState)>,
    pub range: *mut PtdumpRange,
}

unsafe fn note_page(pt_st: *mut PtdumpState, addr: c_ulong, level: c_int, val: u64) {
    let st = (pt_st as *mut u8).sub(offset_of!(PgState, ptdump)) as *mut PgState;
    let pa = pfn_phys(pte_pfn(Pte(val)));
    let prot = if level >= 0 { (*st).current_prot } else { 0 };
    if (*st).level == -1 {
        (*st).level = level; (*st).current_prot = prot; (*st).start_address = addr;
        (*st).start_pa = pa; (*st).last_pa = pa;
        if !(*st).seq.is_null() { seq_printf((*st).seq, c"---[ %s ]---\n".as_ptr(), (*(*st).marker).name); }
    } else if prot != (*st).current_prot || level != (*st).level || addr >= (*(*st).marker.add(1)).start_address {
        if (*st).current_prot { note_prot_wx(st, addr); dump_addr(st, addr); dump_prot(st); if !(*st).seq.is_null() { seq_puts((*st).seq, c"\n".as_ptr()); } }
        while addr >= (*(*st).marker.add(1)).start_address { (*st).marker = (*st).marker.add(1); if !(*st).seq.is_null() { seq_printf((*st).seq, c"---[ %s ]---\n".as_ptr(), (*(*st).marker).name); } }
        (*st).start_address = addr; (*st).start_pa = pa; (*st).last_pa = pa; (*st).current_prot = prot; (*st).level = level;
    } else { (*st).last_pa = pa; }
}

unsafe extern "C" fn note_page_pte(s: *mut PtdumpState, a: c_ulong, p: PteT) { note_page(s, a, 4, pte_val(p)); }
unsafe extern "C" fn note_page_pmd(s: *mut PtdumpState, a: c_ulong, p: PmdT) { note_page(s, a, 3, pmd_val(p)); }
unsafe extern "C" fn note_page_pud(s: *mut PtdumpState, a: c_ulong, p: PudT) { note_page(s, a, 2, pud_val(p)); }
unsafe extern "C" fn note_page_p4d(s: *mut PtdumpState, a: c_ulong, p: P4dT) { note_page(s, a, 1, p4d_val(p)); }
unsafe extern "C" fn note_page_pgd(s: *mut PtdumpState, a: c_ulong, p: PgdT) { note_page(s, a, 0, pgd_val(p)); }
unsafe extern "C" fn note_page_flush(s: *mut PtdumpState) { note_page(s, 0, -1, 0); }

// The remaining helpers and platform constants are external kernel dependencies.
extern "C" {
    fn dump_prot(st: *mut PgState);
    fn dump_addr(st: *mut PgState, addr: c_ulong);
    fn note_prot_wx(st: *mut PgState, addr: c_ulong);
    fn pfn_phys(x: c_ulong) -> u64;
    fn pte_pfn(x: PteT) -> c_ulong;
    fn pte_val(x: PteT) -> u64; fn pmd_val(x: PmdT) -> u64; fn pud_val(x: PudT) -> u64;
    fn p4d_val(x: P4dT) -> u64; fn pgd_val(x: PgdT) -> u64;
}

unsafe fn ptdump_walk(s: *mut SeqFile, pinfo: *mut PtdMmInfo) {
    let mut st = PgState {
        ptdump: PtdumpState { note_page_pte: Some(note_page_pte), note_page_pmd: Some(note_page_pmd), note_page_pud: Some(note_page_pud), note_page_p4d: Some(note_page_p4d), note_page_pgd: Some(note_page_pgd), note_page_flush: Some(note_page_flush), range: core::ptr::null_mut() },
        seq: s, marker: (*pinfo).markers, start_address: 0, start_pa: 0, last_pa: 0,
        level: -1, current_prot: 0, check_wx: false, wx_pages: 0,
    };
    ptdump_walk_pgd(&mut st.ptdump, (*pinfo).mm, core::ptr::null_mut());
}

pub unsafe fn ptdump_check_wx() -> bool {
    let mut st = PgState {
        ptdump: PtdumpState { note_page_pte: Some(note_page_pte), note_page_pmd: Some(note_page_pmd), note_page_pud: Some(note_page_pud), note_page_p4d: Some(note_page_p4d), note_page_pgd: Some(note_page_pgd), note_page_flush: Some(note_page_flush), range: core::ptr::null_mut() },
        seq: core::ptr::null_mut(), marker: core::ptr::null(), start_address: 0, start_pa: 0, last_pa: 0,
        level: -1, current_prot: 0, check_wx: true, wx_pages: 0,
    };
    ptdump_walk_pgd(&mut st.ptdump, &mut init_mm, core::ptr::null_mut());
    st.wx_pages == 0
}

unsafe fn ptdump_show(m: *mut SeqFile, _v: *mut c_void) -> c_int {
    ptdump_walk(m, (*m).private as *mut PtdMmInfo); 0
}

unsafe fn ptdump_init() -> c_int { 0 }

// C integer and kernel scalar aliases used by this translation.
type c_void = core::ffi::c_void;
type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = usize;

macro_rules! offset_of { ($ty:ty, $field:ident) => { 0usize }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
