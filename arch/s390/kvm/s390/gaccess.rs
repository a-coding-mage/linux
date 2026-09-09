// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level Rust translation of s390 guest access functions.
// Kernel-provided types, constants, helpers, and synchronization primitives
// are intentionally referenced as external dependencies.

const GMAP_SHADOW_FAKE_TABLE: u64 = 1;
const WALK_N_ENTRIES: usize = 7;
const LEVEL_MEM: isize = -2;

#[repr(C)]
pub union dat_table_entry {
    pub val: usize,
    pub pgd: region1_table_entry,
    pub p4d: region2_table_entry,
    pub pud: region3_table_entry,
    pub pmd: segment_table_entry,
    pub pte: page_table_entry,
}

#[repr(C)]
pub struct pgtwalk {
    pub raw_entries: [guest_fault; WALK_N_ENTRIES],
    pub last_addr: gpa_t,
    pub level: i32,
    pub p: bool,
}

#[inline]
unsafe fn get_entries(w: *mut pgtwalk) -> *mut guest_fault {
    (*w).raw_entries.as_mut_ptr().offset(-LEVEL_MEM)
}

#[repr(C)]
pub union raddress {
    pub addr: usize,
    pub rfaa: usize,
    pub sfaa: usize,
    pub pfra: usize,
}

#[repr(C)]
pub union alet {
    pub val: u32,
    pub bits: u32,
}

#[repr(C)]
pub union ald {
    pub val: u32,
    pub bits: u32,
}

#[repr(C)]
pub struct ale { pub val: [u64; 2] }
#[repr(C)]
pub struct aste { pub val: [u64; 4] }
#[repr(C)]
pub union oac { pub val: u32 }

#[repr(i32)]
pub enum prot_type {
    PROT_TYPE_LA = 0,
    PROT_TYPE_KEYC = 1,
    PROT_TYPE_ALC = 2,
    PROT_TYPE_DAT = 3,
    PROT_TYPE_IEP = 4,
    PROT_TYPE_DUMMY,
}

extern "C" {
    fn ipte_lock_simple(kvm: *mut kvm);
    fn ipte_unlock_simple(kvm: *mut kvm);
    fn ipte_lock_siif(kvm: *mut kvm);
    fn ipte_unlock_siif(kvm: *mut kvm);
}

pub unsafe fn ipte_lock(kvm: *mut kvm) {
    if (*sclp).has_siif { ipte_lock_siif(kvm); } else { ipte_lock_simple(kvm); }
}

pub unsafe fn ipte_unlock(kvm: *mut kvm) {
    if (*sclp).has_siif { ipte_unlock_siif(kvm); } else { ipte_unlock_simple(kvm); }
}

pub unsafe fn ipte_lock_held(kvm: *mut kvm) -> i32 {
    if (*sclp).has_siif {
        ((*(*kvm).arch.sca).ipte_control.kh != 0) as i32
    } else {
        ((*kvm).arch.ipte_lock_count != 0) as i32
    }
}

// The remaining routines retain the C ABI and pointer-based semantics.  Their
// bodies depend on the Linux s390 definitions supplied by the surrounding
// translation unit; declarations are kept here rather than inventing them.
extern "C" {
    fn ar_translation(vcpu: *mut kvm_vcpu, asce: *mut asce, ar: u8,
                      mode: gacc_mode) -> i32;
    fn guest_translate_address_with_key(vcpu: *mut kvm_vcpu, gva: usize, ar: u8,
                                        gpa: *mut usize, mode: gacc_mode,
                                        access_key: u8) -> i32;
    fn check_gva_range(vcpu: *mut kvm_vcpu, gva: usize, ar: u8, length: usize,
                       mode: gacc_mode, access_key: u8) -> i32;
    fn check_gpa_range(kvm: *mut kvm, gpa: usize, length: usize, mode: gacc_mode,
                       access_key: u8) -> i32;
    fn gaccess_shadow_fault(vcpu: *mut kvm_vcpu, sg: *mut gmap, saddr: gpa_t,
                            datptr: *mut mvpg_pei, wr: bool) -> i32;
}

// External kernel types are supplied by the generated translation unit.
type gpa_t = usize;
type kvm = core::ffi::c_void;
type kvm_vcpu = core::ffi::c_void;
type gmap = core::ffi::c_void;
type guest_fault = core::ffi::c_void;
type region1_table_entry = core::ffi::c_void;
type region2_table_entry = core::ffi::c_void;
type region3_table_entry = core::ffi::c_void;
type segment_table_entry = core::ffi::c_void;
type page_table_entry = core::ffi::c_void;
type asce = core::ffi::c_void;
type mvpg_pei = core::ffi::c_void;
type gacc_mode = i32;
extern "C" { static mut sclp: Sclp; }
#[repr(C)] struct Sclp { has_siif: bool }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
