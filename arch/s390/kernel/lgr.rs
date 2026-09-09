// SPDX-License-Identifier: GPL-2.0
/*
 * Linux Guest Relocation (LGR) detection
 *
 * Copyright IBM Corp. 2012
 * Author(s): Michael Holzheu <holzheu@linux.vnet.ibm.com>
 */

// Kernel and s390 headers from the original translation unit provide these
// declarations and macros.

const LGR_TIMER_INTERVAL_SECS: u64 = 30 * 60;
const VM_LEVEL_MAX: usize = 2;

#[repr(C, packed(8))]
struct LgrInfo {
    stfle_fac_list: [u64; 4],
    level: u32,
    manufacturer: [i8; 16],
    type_: [i8; 4],
    sequence: [i8; 16],
    plant: [i8; 4],
    model: [i8; 16],
    lpar_number: u16,
    name: [i8; 8],
    vm_count: u8,
    vm: [VmInfo; VM_LEVEL_MAX],
}

#[repr(C)]
struct VmInfo {
    name: [i8; 8],
    cpi: [i8; 16],
}

extern "C" {
    static mut lgr_page: [i8; PAGE_SIZE];
    static mut lgr_info_last: LgrInfo;
    static mut lgr_info_cur: LgrInfo;
    static mut lgr_dbf: *mut DebugInfo;
}

#[allow(non_camel_case_types)]
type spinlock_t = core::ffi::c_void;
#[allow(non_camel_case_types)]
type timer_list = core::ffi::c_void;
#[allow(non_camel_case_types)]
type DebugInfo = core::ffi::c_void;

extern "C" {
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize);
    fn memset(dst: *mut core::ffi::c_void, value: i32, size: usize);
    fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, size: usize) -> i32;
    fn EBCASC(dst: *mut i8, size: i32);
    fn stfle(list: *mut u64, n: usize);
    fn stsi(si: *mut core::ffi::c_void, fc: i32, sel1: i32, sel2: i32) -> i32;
    fn debug_register(name: *const i8, pages: i32, areas: i32, len: usize) -> *mut DebugInfo;
    fn debug_register_view(dbf: *mut DebugInfo, view: *const core::ffi::c_void);
    fn debug_event(dbf: *mut DebugInfo, level: i32, data: *const core::ffi::c_void, len: usize);
    fn spin_trylock_irqsave(lock: *mut spinlock_t, flags: *mut usize) -> bool;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn mod_timer(timer: *mut timer_list, expires: usize);
    fn secs_to_jiffies(secs: u64) -> usize;
    fn timer_setup(timer: *mut timer_list, callback: unsafe extern "C" fn(*mut timer_list), flags: u32);
    static debug_hex_ascii_view: core::ffi::c_void;
    static mut jiffies: usize;
}

const PAGE_SIZE: usize = 4096;
const ENOMEM: i32 = 12;
const TIMER_DEFERRABLE: u32 = 1;

// The C DEFINE_SPINLOCK(lgr_info_lock) declaration is supplied by the kernel.
static mut lgr_info_lock: spinlock_t = core::ptr::null_mut::<core::ffi::c_void>();

unsafe fn cpascii(dst: *mut i8, src: *const i8, size: i32) {
    memcpy(dst.cast(), src.cast(), size as usize);
    EBCASC(dst, size);
}

unsafe fn lgr_stsi_1_1_1(info: *mut LgrInfo) {
    let si = lgr_page.as_mut_ptr().cast::<Sysinfo111>();
    if stsi(si.cast(), 1, 1, 1) != 0 { return; }
    cpascii((*info).manufacturer.as_mut_ptr(), (*si).manufacturer.as_ptr(), 16);
    cpascii((*info).type_.as_mut_ptr(), (*si).type_.as_ptr(), 4);
    cpascii((*info).model.as_mut_ptr(), (*si).model.as_ptr(), 16);
    cpascii((*info).sequence.as_mut_ptr(), (*si).sequence.as_ptr(), 16);
    cpascii((*info).plant.as_mut_ptr(), (*si).plant.as_ptr(), 4);
}

unsafe fn lgr_stsi_2_2_2(info: *mut LgrInfo) {
    let si = lgr_page.as_mut_ptr().cast::<Sysinfo222>();
    if stsi(si.cast(), 2, 2, 2) != 0 { return; }
    cpascii((*info).name.as_mut_ptr(), (*si).name.as_ptr(), 8);
    (*info).lpar_number = (*si).lpar_number;
}

unsafe fn lgr_stsi_3_2_2(info: *mut LgrInfo) {
    let si = lgr_page.as_mut_ptr().cast::<Sysinfo322>();
    if stsi(si.cast(), 3, 2, 2) != 0 { return; }
    let count = core::cmp::min((*si).count as usize, VM_LEVEL_MAX);
    for i in 0..count {
        cpascii((*info).vm[i].name.as_mut_ptr(), (*si).vm[i].name.as_ptr(), 8);
        cpascii((*info).vm[i].cpi.as_mut_ptr(), (*si).vm[i].cpi.as_ptr(), 16);
    }
    (*info).vm_count = (*si).count;
}

unsafe fn lgr_info_get(info: *mut LgrInfo) {
    memset(info.cast(), 0, core::mem::size_of::<LgrInfo>());
    stfle((*info).stfle_fac_list.as_mut_ptr(), 4);
    let level = stsi(core::ptr::null_mut(), 0, 0, 0);
    (*info).level = level as u32;
    if level >= 1 { lgr_stsi_1_1_1(info); }
    if level >= 2 { lgr_stsi_2_2_2(info); }
    if level >= 3 { lgr_stsi_3_2_2(info); }
}

#[no_mangle]
pub unsafe extern "C" fn lgr_info_log() {
    let mut flags = 0usize;
    if !spin_trylock_irqsave(&mut lgr_info_lock, &mut flags) { return; }
    lgr_info_get(&mut lgr_info_cur);
    if memcmp((&lgr_info_last as *const LgrInfo).cast(), (&lgr_info_cur as *const LgrInfo).cast(), core::mem::size_of::<LgrInfo>()) != 0 {
        debug_event(lgr_dbf, 1, (&lgr_info_cur as *const LgrInfo).cast(), core::mem::size_of::<LgrInfo>());
        lgr_info_last = core::ptr::read(&lgr_info_cur);
    }
    spin_unlock_irqrestore(&mut lgr_info_lock, flags);
}

unsafe extern "C" fn lgr_timer_fn(_unused: *mut timer_list) {
    lgr_info_log();
    lgr_timer_set();
}

static mut lgr_timer: timer_list = core::ptr::null_mut::<core::ffi::c_void>();

unsafe fn lgr_timer_set() {
    mod_timer(&mut lgr_timer, jiffies.wrapping_add(secs_to_jiffies(LGR_TIMER_INTERVAL_SECS)));
}

unsafe extern "C" fn lgr_init() -> i32 {
    lgr_dbf = debug_register(b"lgr\0".as_ptr() as *const i8, 1, 1, core::mem::size_of::<LgrInfo>());
    if lgr_dbf.is_null() { return -ENOMEM; }
    debug_register_view(lgr_dbf, &debug_hex_ascii_view);
    lgr_info_get(&mut lgr_info_last);
    debug_event(lgr_dbf, 1, (&lgr_info_last as *const LgrInfo).cast(), core::mem::size_of::<LgrInfo>());
    timer_setup(&mut lgr_timer, lgr_timer_fn, TIMER_DEFERRABLE);
    lgr_timer_set();
    0
}

// device_initcall(lgr_init);

#[repr(C)] struct Sysinfo111 { manufacturer: [i8; 16], type_: [i8; 4], model: [i8; 16], sequence: [i8; 16], plant: [i8; 4] }
#[repr(C)] struct Sysinfo222 { name: [i8; 8], lpar_number: u16 }
#[repr(C)] struct Sysinfo322 { count: u8, vm: [VmInfo; VM_LEVEL_MAX] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
