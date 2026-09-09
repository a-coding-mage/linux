// SPDX-License-Identifier: GPL-2.0-only
/* Debug helper to dump the current kernel pagetables of the system. */

// External kernel declarations and architecture constants are supplied by the surrounding tree.

#[repr(C)]
pub struct AddrMarker { pub start_address: usize, pub name: *const core::ffi::c_char }
#[repr(C)]
pub struct SeqFile { _private: [u8; 0] }
#[repr(C)]
pub struct MmStruct { _private: [u8; 0] }
#[repr(C)]
pub struct PtdumpInfo { pub mm: *mut MmStruct, pub markers: *mut AddrMarker, pub base_addr: usize }
#[repr(C)] pub struct PgdT { _private: [u8; 0] }
#[repr(C)] pub struct P4dT { _private: [u8; 0] }
#[repr(C)] pub struct PudT { _private: [u8; 0] }
#[repr(C)] pub struct PmdT { _private: [u8; 0] }
#[repr(C)] pub struct PteT { _private: [u8; 0] }

#[repr(C)]
struct ProtBits { mask: u64, val: u64, set: *const u8, clear: *const u8, ro_bit: bool, nx_bit: bool }
#[repr(C)]
struct PgState {
    seq: *mut SeqFile, marker: *mut AddrMarker, start_address: usize, level: u32,
    current_prot: u64, check_wx: bool, wx_pages: usize, current_domain: *const u8,
}
#[repr(C)]
struct PgLevel {
    name: *const u8, bits: *const ProtBits, num: usize, mask: u64,
    ro_bit: *const ProtBits, nx_bit: *const ProtBits,
}

extern "C" {
    static mut init_mm: MmStruct;
    fn seq_printf(m: *mut SeqFile, fmt: *const u8, ...);
    fn pte_offset_kernel(pmd: *mut PmdT, addr: usize) -> *mut PteT;
    fn pmd_offset(pud: *mut PudT, addr: usize) -> *mut PmdT;
    fn pud_offset(p4d: *mut P4dT, addr: usize) -> *mut PudT;
    fn p4d_offset(pgd: *mut PgdT, addr: usize) -> *mut P4dT;
    fn pgd_offset(mm: *mut MmStruct, addr: usize) -> *mut PgdT;
    fn pte_val(x: PteT) -> u64;
    fn pmd_val(x: PmdT) -> u64;
    fn pud_val(x: PudT) -> u64;
    fn p4d_val(x: P4dT) -> u64;
    fn pgd_val(x: PgdT) -> u64;
    fn pmd_none(x: PmdT) -> bool; fn pmd_leaf(x: PmdT) -> bool; fn pmd_present(x: PmdT) -> bool;
    fn pud_none(x: PudT) -> bool; fn p4d_none(x: P4dT) -> bool; fn pgd_none(x: PgdT) -> bool;
    fn ptdump_debugfs_register(info: *mut PtdumpInfo, name: *const u8);
    fn warn_once(cond: bool, fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...); fn pr_info(fmt: *const u8, ...);
}

// Architecture-provided constants. Build-time CONFIG_* branches remain represented by cfg attributes.
extern "C" {
    static mut address_markers: [AddrMarker; 10];
    static mut pg_level: [PgLevel; 6];
}

unsafe fn dump_prot(st: *mut PgState, bits: *const ProtBits, num: usize) {
    for i in 0..num {
        let b = &*bits.add(i);
        let s = if ((*st).current_prot & b.mask) == b.val { b.set } else { b.clear };
        if !s.is_null() { seq_printf((*st).seq, b" %s\0".as_ptr(), s); }
    }
}

unsafe fn note_prot_wx(st: *mut PgState, addr: usize) {
    if !(*st).check_wx { return; }
    let l = &pg_level[(*st).level as usize];
    if ((*st).current_prot & (*l.ro_bit).mask) == (*l.ro_bit).val ||
       ((*st).current_prot & (*l.nx_bit).mask) == (*l.nx_bit).val { return; }
    warn_once(true, b"arm/mm: Found insecure W+X mapping at address %pS\n\0".as_ptr(), (*st).start_address);
    (*st).wx_pages += (addr - (*st).start_address) / PAGE_SIZE;
}

unsafe fn note_page(st: *mut PgState, addr: usize, level: u32, val: u64, domain: *const u8) {
    let units = b"KMGTPE\0";
    let prot = val & pg_level[level as usize].mask;
    if (*st).level == 0 {
        (*st).level = level; (*st).current_prot = prot; (*st).current_domain = domain;
        seq_printf((*st).seq, b"---[ %s ]---\n\0".as_ptr(), (*(*st).marker).name);
    } else if prot != (*st).current_prot || level != (*st).level || domain != (*st).current_domain || addr >= (*st).marker.add(1).read().start_address {
        let mut unit = units.as_ptr(); let mut delta;
        if (*st).current_prot != 0 {
            note_prot_wx(st, addr);
            seq_printf((*st).seq, b"0x%08lx-0x%08lx   \0".as_ptr(), (*st).start_address, addr);
            delta = (addr - (*st).start_address) >> 10;
            while (delta & 1023) == 0 && *unit.add(1) != 0 { delta >>= 10; unit = unit.add(1); }
            let l = &pg_level[(*st).level as usize];
            seq_printf((*st).seq, b"%9lu%c %s\0".as_ptr(), delta, *unit, l.name);
            if !(*st).current_domain.is_null() { seq_printf((*st).seq, b" %s\0".as_ptr(), (*st).current_domain); }
            if !l.bits.is_null() { dump_prot(st, l.bits, l.num); }
            seq_printf((*st).seq, b"\n\0".as_ptr());
        }
        if addr >= (*st).marker.add(1).read().start_address { (*st).marker = (*st).marker.add(1); seq_printf((*st).seq, b"---[ %s ]---\n\0".as_ptr(), (*(*st).marker).name); }
        (*st).start_address = addr; (*st).current_prot = prot; (*st).current_domain = domain; (*st).level = level;
    }
}

unsafe fn walk_pte(st: *mut PgState, pmd: *mut PmdT, start: usize, domain: *const u8) {
    let mut pte = pte_offset_kernel(pmd, 0);
    for i in 0..PTRS_PER_PTE { note_page(st, start + i * PAGE_SIZE, 5, pte_val(pte.add(i).read()), domain); }
}
unsafe fn walk_pmd(st: *mut PgState, pud: *mut PudT, start: usize) {
    let mut pmd = pmd_offset(pud, 0);
    for i in 0..PTRS_PER_PMD { let addr = start + i * PMD_SIZE; let d = get_domain_name(pmd.add(i)); let p = pmd.add(i).read(); if pmd_none(p) || pmd_leaf(p) || !pmd_present(p) { note_page(st, addr, 4, pmd_val(p), d); } else { walk_pte(st, pmd.add(i), addr, d); } }
}
unsafe fn walk_pud(st: *mut PgState, p4d: *mut P4dT, start: usize) { let pud = pud_offset(p4d, 0); for i in 0..PTRS_PER_PUD { let p=pud.add(i).read(); let a=start+i*PUD_SIZE; if !pud_none(p) { walk_pmd(st,pud.add(i),a); } else { note_page(st,a,3,pud_val(p),core::ptr::null()); } } }
unsafe fn walk_p4d(st: *mut PgState, pgd: *mut PgdT, start: usize) { let p4d=p4d_offset(pgd,0); for i in 0..PTRS_PER_P4D { let p=p4d.add(i).read(); let a=start+i*P4D_SIZE; if !p4d_none(p) { walk_pud(st,p4d.add(i),a); } else { note_page(st,a,2,p4d_val(p),core::ptr::null()); } } }
unsafe fn walk_pgd(st: *mut PgState, mm: *mut MmStruct, start: usize) { let pgd=pgd_offset(mm,0); for i in 0..PTRS_PER_PGD { let p=pgd.add(i).read(); let a=start+i*PGDIR_SIZE; if !pgd_none(p) { walk_p4d(st,pgd.add(i),a); } else { note_page(st,a,1,pgd_val(p),core::ptr::null()); } } }

unsafe fn get_domain_name(_pmd: *mut PmdT) -> *const u8 { core::ptr::null() }
pub unsafe fn ptdump_walk_pgd(m: *mut SeqFile, info: *mut PtdumpInfo) { let mut st=PgState{seq:m,marker:(*info).markers,start_address:0,level:0,current_prot:0,check_wx:false,wx_pages:0,current_domain:core::ptr::null()}; walk_pgd(&mut st,(*info).mm,(*info).base_addr); note_page(&mut st,0,0,0,core::ptr::null()); }

unsafe fn ptdump_initialize() {
    for i in 0..pg_level.len() { if !pg_level[i].bits.is_null() { for j in 0..pg_level[i].num { let b=&*pg_level[i].bits.add(j); pg_level[i].mask |= b.mask; if b.ro_bit { pg_level[i].ro_bit= b; } if b.nx_bit { pg_level[i].nx_bit=b; } } } }
    // CONFIG_KASAN selects the corresponding marker slot for VMALLOC_START.
}

pub unsafe fn ptdump_check_wx() {
    let mut markers=[AddrMarker{start_address:0,name:core::ptr::null()},AddrMarker{start_address:usize::MAX,name:core::ptr::null()}];
    let mut st=PgState{seq:core::ptr::null_mut(),marker:markers.as_mut_ptr(),start_address:0,level:0,current_prot:0,check_wx:true,wx_pages:0,current_domain:core::ptr::null()};
    walk_pgd(&mut st,&mut init_mm,0); note_page(&mut st,0,0,0,core::ptr::null());
    if st.wx_pages != 0 { pr_warn(b"Checked W+X mappings: FAILED, %lu W+X pages found\n\0".as_ptr(),st.wx_pages); } else { pr_info(b"Checked W+X mappings: passed, no W+X pages found\n\0".as_ptr()); }
}

pub unsafe fn ptdump_init() -> i32 {
    ptdump_initialize();
    ptdump_debugfs_register(&mut kernel_ptdump_info, b"kernel_page_tables\0".as_ptr());
    0
}

extern "C" { fn ptdump_debugfs_register(info: *mut PtdumpInfo, name: *const u8); }
static mut kernel_ptdump_info: PtdumpInfo = PtdumpInfo { mm: core::ptr::null_mut(), markers: core::ptr::null_mut(), base_addr: 0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
