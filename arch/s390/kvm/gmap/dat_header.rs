/* SPDX-License-Identifier: GPL-2.0 */
/* KVM guest address space mapping code. */

// Dependencies supplied by the surrounding kernel translation.

pub const KVM_S390_MAX_BIT_DISTANCE: usize = 2 * core::mem::size_of::<*const core::ffi::c_void>();
pub const KVM_S390_CMMA_SIZE_MAX: u32 = KVM_S390_SKEYS_MAX as u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub union pte {
    pub val: c_ulong,
    pub h: page_table_entry,
    pub s: pte_software,
    pub bytes: pte_bytes,
    pub tok: pte_token,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_software { pub raw: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_bytes { pub hwbytes: [u8; 7], pub swbyte: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_token { pub raw: u64 }

pub const _DAT_TOKEN_NONE: i32 = 0;
pub const _DAT_TOKEN_PIC: i32 = 1;
pub const TABLE_TYPE_PAGE_TABLE: i32 = -1;

pub const DAT_WALK_USES_SKEYS: i32 = 0x40;
pub const DAT_WALK_CONTINUE: i32 = 0x20;
pub const DAT_WALK_IGN_HOLES: i32 = 0x10;
pub const DAT_WALK_SPLIT: i32 = 0x08;
pub const DAT_WALK_ALLOC: i32 = 0x04;
pub const DAT_WALK_ANY: i32 = 0x02;
pub const DAT_WALK_LEAF: i32 = 0x01;
pub const DAT_WALK_DEFAULT: i32 = 0;
pub const DAT_WALK_SPLIT_ALLOC: i32 = DAT_WALK_SPLIT | DAT_WALK_ALLOC;
pub const DAT_WALK_ALLOC_CONTINUE: i32 = DAT_WALK_CONTINUE | DAT_WALK_ALLOC;
pub const DAT_WALK_LEAF_ALLOC: i32 = DAT_WALK_LEAF | DAT_WALK_ALLOC;

pub const _PAGE_SD: u64 = 0x002;
pub const PGSTE_PCL_BIT: c_ulong = 0x0080000000000000;
pub const PGSTE_CMMA_D_BIT: c_ulong = 0x0000000000008000;

#[repr(C)]
#[derive(Copy, Clone)]
pub union pgste { pub val: c_ulong, pub raw: pgste_bytes }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct pgste_bytes { pub hwbytes0: u16, pub val16: u16, pub hwbytes4: u16, pub flags: u8, pub hwbyte7: u8 }

#[repr(C)]
#[derive(Copy, Clone)]
pub union pmd { pub val: c_ulong, pub h: segment_table_entry, pub s: pmd_software }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmd_software { pub raw: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub union pud { pub val: c_ulong, pub h: region3_table_entry, pub s: pud_software }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pud_software { pub raw: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub union p4d { pub val: c_ulong, pub h: region2_table_entry }
#[repr(C)]
#[derive(Copy, Clone)]
pub union pgd { pub val: c_ulong, pub h: region1_table_entry }

#[repr(C)]
#[derive(Copy, Clone)]
pub union crste {
    pub val: c_ulong,
    pub h: crste_h,
    pub s: crste_software,
    pub tok: crste_token,
    pub pmd: pmd,
    pub pud: pud,
    pub p4d: p4d,
    pub pgd: pgd,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crste_h { pub raw: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crste_software { pub raw: c_ulong }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crste_token { pub raw: u64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub union skey { pub skey: u8, pub raw: u8 }

#[repr(C)]
pub struct segment_table { pub pmds: [pmd; _CRST_ENTRIES as usize] }
#[repr(C)]
pub struct region3_table { pub puds: [pud; _CRST_ENTRIES as usize] }
#[repr(C)]
pub struct region2_table { pub p4ds: [p4d; _CRST_ENTRIES as usize] }
#[repr(C)]
pub struct region1_table { pub pgds: [pgd; _CRST_ENTRIES as usize] }

#[repr(C)]
pub union crst_table_union {
    pub crstes: [crste; _CRST_ENTRIES as usize],
    pub segment: segment_table,
    pub region3: region3_table,
    pub region2: region2_table,
    pub region1: region1_table,
}
#[repr(C)]
pub struct crst_table { pub u: crst_table_union }
#[repr(C)]
pub struct page_table { pub ptes: [pte; _PAGE_ENTRIES as usize], pub pgstes: [pgste; _PAGE_ENTRIES as usize] }

pub type dat_walk_op = unsafe extern "C" fn(*mut crste, gfn_t, gfn_t, *mut dat_walk) -> c_long;
#[repr(C)]
pub union dat_walk_ops_union { pub crste_ops: [Option<dat_walk_op>; 4], pub entries: dat_walk_entries }
#[repr(C)]
pub struct dat_walk_entries { pub pmd_entry: Option<dat_walk_op>, pub pud_entry: Option<dat_walk_op>, pub p4d_entry: Option<dat_walk_op>, pub pgd_entry: Option<dat_walk_op> }
#[repr(C)]
pub struct dat_walk_ops { pub u: dat_walk_ops_union, pub pte_entry: Option<unsafe extern "C" fn(*mut pte, gfn_t, gfn_t, *mut dat_walk) -> c_long> }
#[repr(C)]
pub struct dat_walk { pub ops: *const dat_walk_ops, pub last: *mut crste, pub last_pte: *mut pte, pub asce: asce, pub start: gfn_t, pub end: gfn_t, pub flags: i32, pub priv_: *mut core::ffi::c_void }
#[repr(C)]
pub struct ptval_param { pub raw: u8 }

#[repr(C)]
#[derive(Copy, Clone)]
pub union essa_state { pub val: u8, pub raw: u8 }

#[repr(C)]
pub struct vsie_rmap { pub next: *mut vsie_rmap, pub u: vsie_rmap_union }
#[repr(C)]
pub union vsie_rmap_union { pub val: c_ulong, pub raw: c_ulong }

pub unsafe fn _pte(pfn: kvm_pfn_t, writable: bool, dirty: bool, special: bool) -> pte {
    let mut val = PFN_PHYS(pfn);
    val |= 1u64 << 61 | 1u64 << 63;
    if writable { val |= 1u64 << 59; }
    if dirty { val |= 1u64 << 60 | 1u64 << 62; } else { val &= !(1u64 << 56); }
    if special { val |= 1u64 << 58; }
    pte { val }
}
pub unsafe fn _crste_fc0(pfn: kvm_pfn_t, tt: i32) -> crste {
    let mut val = PFN_PHYS(pfn);
    val = (val & !(0x3u64 << 2)) | ((tt as u64) & 0x3) << 2;
    val = (val & !(0x3u64 << 8)) | ((_REGION_ENTRY_LENGTH as u64) & 0x3) << 8;
    crste { val }
}
pub unsafe fn _crste_fc1(pfn: kvm_pfn_t, tt: i32, writable: bool, dirty: bool) -> crste {
    let mut val = PFN_PHYS(pfn) & _SEGMENT_MASK;
    val = (val & !(0x3u64 << 2)) | ((tt as u64) & 0x3) << 2;
    if !dirty { val |= 1u64 << 3; }
    val |= 1u64 << 8;
    if writable { val |= 1u64 << 57; }
    if dirty { val |= 1u64 << 60 | 1u64 << 62; }
    val |= 1u64 << 63;
    crste { val }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
