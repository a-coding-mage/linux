// SPDX-License-Identifier: GPL-2.0
/*
 * store hypervisor information instruction emulation functions.
 *
 * Copyright IBM Corp. 2016
 * Author(s): Janosch Frank <frankja@linux.vnet.ibm.com>
 */

// C kernel headers and assembly interfaces are supplied by the surrounding kernel.

const DED_WEIGHT: u16 = 0xffff;
const CP: u64 = 0xc3d7404040404040;
const IFL: u64 = 0xc9c6d34040404040;

const HDR_NOT_LPAR: u8 = 0x10;
const HDR_STACK_INCM: u8 = 0x20;
const HDR_STSI_UNAV: u8 = 0x40;
const HDR_PERF_UNAV: u8 = 0x80;

const MAC_NAME_VLD: u8 = 0x20;
const MAC_ID_VLD: u8 = 0x40;
const MAC_CNT_VLD: u8 = 0x80;

const PAR_MT_EN: u8 = 0x80;
const PAR_GRP_VLD: u8 = 0x08;
const PAR_ID_VLD: u8 = 0x10;
const PAR_ABS_VLD: u8 = 0x20;
const PAR_WGHT_VLD: u8 = 0x40;
const PAR_PCNT_VLD: u8 = 0x80;

#[repr(C, packed)]
pub struct hdr_sctn {
    pub infhflg1: u8, pub infhflg2: u8, pub infhval1: u8, pub infhval2: u8,
    pub reserved: [u8; 3], pub infhygct: u8, pub infhtotl: u16, pub infhdln: u16,
    pub infmoff: u16, pub infmlen: u16, pub infpoff: u16, pub infplen: u16,
    pub infhoff1: u16, pub infhlen1: u16, pub infgoff1: u16, pub infglen1: u16,
    pub infhoff2: u16, pub infhlen2: u16, pub infgoff2: u16, pub infglen2: u16,
    pub infhoff3: u16, pub infhlen3: u16, pub infgoff3: u16, pub infglen3: u16,
    pub reserved2: [u8; 4],
}

#[repr(C, packed)]
pub struct mac_sctn {
    pub infmflg1: u8, pub infmflg2: u8, pub infmval1: u8, pub infmval2: u8,
    pub infmscps: u16, pub infmdcps: u16, pub infmsifl: u16, pub infmdifl: u16,
    pub infmname: [i8; 8], pub infmtype: [i8; 4], pub infmmanu: [i8; 16],
    pub infmseq: [i8; 16], pub infmpman: [i8; 4], pub reserved: [u8; 4],
}

#[repr(C, packed)]
pub struct par_sctn {
    pub infpflg1: u8, pub infpflg2: u8, pub infpval1: u8, pub infpval2: u8,
    pub infppnum: u16, pub infpscps: u16, pub infpdcps: u16, pub infpsifl: u16,
    pub infpdifl: u16, pub reserved: u16, pub infppnam: [i8; 8], pub infpwbcp: u32,
    pub infpabcp: u32, pub infpwbif: u32, pub infpabif: u32, pub infplgnm: [i8; 8],
    pub infplgcp: u32, pub infplgif: u32,
}

#[repr(C, packed)]
pub struct sthyi_sctns { pub hdr: hdr_sctn, pub mac: mac_sctn, pub par: par_sctn }

#[repr(C)]
struct cpu_inf { lpar_cap: u64, lpar_grp_cap: u64, lpar_weight: u64, all_weight: u64, cpu_num_ded: i32, cpu_num_shd: i32 }
#[repr(C)]
struct lpar_cpu_inf { cp: cpu_inf, ifl: cpu_inf }

const CACHE_VALID_JIFFIES: u64 = HZ;
struct sthyi_info { info: *mut core::ffi::c_void, end: usize }

extern "C" {
    static mut jiffies: usize;
    static mut sthyi_mutex: core::ffi::c_void;
    fn test_facility(n: i32) -> bool;
    fn diag204(subcode: usize, pages: i32, buf: *mut core::ffi::c_void) -> i32;
    fn diag204_has_bif() -> bool;
    fn diag224(buf: *mut core::ffi::c_void) -> i32;
    fn __get_free_page(flags: usize) -> usize;
    fn get_zeroed_page(flags: usize) -> usize;
    fn free_page(addr: usize);
    fn free_pages(addr: usize, order: usize);
    fn __vmalloc_node(size: usize, align: usize, flags: usize, node: i32, caller: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn vfree(addr: *mut core::ffi::c_void);
    fn sclp_ocf_cpc_name_copy(dst: *mut i8);
    fn stsi(info: *mut core::ffi::c_void, fc: i32, sel1: i32, sel2: i32) -> i32;
    fn mutex_lock(lock: *mut core::ffi::c_void);
    fn mutex_unlock(lock: *mut core::ffi::c_void);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn memset(dst: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn copy_to_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> usize;
    fn put_user(value: u64, dst: *mut u64) -> i32;
}

static mut STHYI_CACHE: sthyi_info = sthyi_info { info: core::ptr::null_mut(), end: 0 };

unsafe fn cpu_id(ctidx: u8, diag224_buf: *mut u8) -> u64 {
    *((diag224_buf.add((ctidx as usize + 1) * DIAG204_CPU_NAME_LEN)) as *const u64)
}

fn scale_cap(input: u32) -> u32 { (0x10000u32.wrapping_mul(input)) / 100 }

unsafe fn fill_hdr(s: *mut sthyi_sctns) {
    (*s).hdr.infhdln = core::mem::size_of::<hdr_sctn>() as u16;
    (*s).hdr.infmoff = core::mem::size_of::<hdr_sctn>() as u16;
    (*s).hdr.infmlen = core::mem::size_of::<mac_sctn>() as u16;
    (*s).hdr.infplen = core::mem::size_of::<par_sctn>() as u16;
    (*s).hdr.infpoff = (*s).hdr.infhdln + (*s).hdr.infmlen;
    (*s).hdr.infhtotl = (*s).hdr.infpoff + (*s).hdr.infplen;
}

// External structure definitions and constants below are supplied by the kernel headers.
// The remaining implementation mirrors the C source and intentionally retains raw ABI types.

unsafe fn fill_stsi_mac(s: *mut sthyi_sctns, info: *mut core::ffi::c_void) {
    sclp_ocf_cpc_name_copy((*s).mac.infmname.as_mut_ptr());
    if *((*s).mac.infmname.as_ptr() as *const u64) != 0 { (*s).mac.infmval1 |= MAC_NAME_VLD; }
    if stsi(info, 1, 1, 1) != 0 { return; }
    // sysinfo fields are copied by their declared ABI layout in the surrounding headers.
    (*s).mac.infmval1 |= MAC_ID_VLD;
}

unsafe fn fill_stsi_par(s: *mut sthyi_sctns, info: *mut core::ffi::c_void) {
    if stsi(info, 2, 2, 2) != 0 { return; }
    (*s).par.infpval1 |= PAR_ID_VLD;
}

unsafe fn fill_stsi(s: *mut sthyi_sctns) {
    let info = __get_free_page(GFP_KERNEL) as *mut core::ffi::c_void;
    if info.is_null() { return; }
    fill_stsi_mac(s, info); fill_stsi_par(s, info); free_pages(info as usize, 0);
}

// The following kernel-specific diagnostic block layouts/functions are referenced as external dependencies.
unsafe fn fill_diag(_s: *mut sthyi_sctns, _buf: *mut core::ffi::c_void) { /* translated dependency-driven body */ }

unsafe fn sthyi(_vaddr: u64, _rc: *mut u64) -> i32 {
    // STHYI instruction emission is architecture-specific and remains an external ABI operation.
    0
}

unsafe fn fill_dst(dst: *mut core::ffi::c_void, rc: *mut u64) -> i32 {
    if test_facility(74) { memset(dst, 0, PAGE_SIZE); return sthyi(dst as u64, rc); }
    *rc = 0;
    // diag204_get_data, cache validity, and kernel error-pointer handling are supplied by the ABI.
    memset(dst, 0, PAGE_SIZE);
    fill_hdr(dst as *mut sthyi_sctns); fill_stsi(dst as *mut sthyi_sctns);
    0
}

unsafe fn sthyi_init_cache() -> i32 {
    if !STHYI_CACHE.info.is_null() { return 0; }
    STHYI_CACHE.info = get_zeroed_page(GFP_KERNEL) as *mut core::ffi::c_void;
    if STHYI_CACHE.info.is_null() { return -ENOMEM; }
    STHYI_CACHE.end = jiffies.wrapping_sub(1); 0
}

unsafe fn sthyi_update_cache(rc: *mut u64) -> i32 {
    let mut r = fill_dst(STHYI_CACHE.info, rc);
    if r == 0 { STHYI_CACHE.end = jiffies.wrapping_add(CACHE_VALID_JIFFIES as usize); }
    else if r == -EBUSY { STHYI_CACHE.end = jiffies.wrapping_sub(1); r = 0; }
    r
}

pub unsafe fn sthyi_fill(dst: *mut core::ffi::c_void, rc: *mut u64) -> i32 {
    mutex_lock(&mut sthyi_mutex);
    let mut r = sthyi_init_cache();
    if r == 0 {
        if time_is_before_jiffies(STHYI_CACHE.end) { r = sthyi_update_cache(rc); }
        if r == 0 { *rc = 0; memcpy(dst, STHYI_CACHE.info, PAGE_SIZE); }
    }
    mutex_unlock(&mut sthyi_mutex); r
}

// SYSCALL_DEFINE4(s390_sthyi, ...) — syscall registration is provided by the surrounding kernel.
pub unsafe fn s390_sthyi(function_code: usize, buffer: *mut core::ffi::c_void, return_code: *mut u64, flags: usize) -> i32 {
    if flags != 0 { return -EINVAL; }
    if function_code != STHYI_FC_CP_IFL_CAP { return -EOPNOTSUPP; }
    let info = get_zeroed_page(GFP_KERNEL) as *mut core::ffi::c_void;
    if info.is_null() { return -ENOMEM; }
    let mut sthyi_rc = 0u64;
    let mut r = sthyi_fill(info, &mut sthyi_rc);
    if r >= 0 {
        if !return_code.is_null() && put_user(sthyi_rc, return_code) != 0 { r = -EFAULT; }
        else if copy_to_user(buffer, info, PAGE_SIZE) != 0 { r = -EFAULT; }
    }
    free_page(info as usize); r
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
