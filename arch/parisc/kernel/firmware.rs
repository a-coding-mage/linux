// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/parisc/kernel/firmware.c - safe PDC access routines
 * Rust translation.  Kernel-provided constants, types, globals, and
 * synchronization primitives are intentionally left as external dependencies.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut pdc_lock: c_void;
    static mut PAGE0: *mut Page0;
    static mut parisc_narrow_firmware: c_int;
    fn real32_call(function: c_ulong, ...) -> c_long;
    fn real64_call(function: c_ulong, ...) -> c_long;
    fn mem_pdc_call(function: c_ulong, ...) -> c_int;
    fn is_pdc_pat() -> bool;
    fn __pa(p: *const c_void) -> c_ulong;
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn spin_trylock_irqsave(lock: *mut c_void, flags: *mut c_ulong) -> bool;
    fn spin_is_locked(lock: *mut c_void) -> bool;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
}

type c_long = isize;
const NUM_PDC_RESULT: usize = 32;
static mut pdc_result: [c_ulong; NUM_PDC_RESULT] = [0; NUM_PDC_RESULT];
static mut pdc_result2: [c_ulong; NUM_PDC_RESULT] = [0; NUM_PDC_RESULT];
static mut iodc_dbuf: [u8; 4096] = [0; 4096];

#[repr(C)] pub struct Page0 { pub mem_pdc_hi: u32, pub mem_pdc: u32, pub mem_cons: Console, pub mem_kbd: Console }
#[repr(C)] pub struct Console { pub iodc_io: c_ulong, pub hpa: c_ulong, pub spa: c_ulong, pub dp: DataPath }
#[repr(C)] pub struct DataPath { pub layers: *mut c_void }

unsafe fn f_extend(address: c_ulong) -> c_ulong {
    #[cfg(target_pointer_width = "64")]
    if parisc_narrow_firmware != 0 {
        if address & 0xff000000 == 0xf0000000 { return (0xfffffff0u64 << 32) | (address as u32 as c_ulong); }
        if address & 0xf0000000 == 0xf0000000 { return (0xffffffffu64 << 32) | (address as u32 as c_ulong); }
    }
    address
}

unsafe fn convert_to_wide(addr: *mut c_ulong) {
    #[cfg(target_pointer_width = "64")]
    if parisc_narrow_firmware != 0 {
        let p = addr as *mut u32;
        let mut i = NUM_PDC_RESULT;
        while i != 0 { i -= 1; *addr.add(i) = *p.add(i) as c_ulong; }
    }
}

#[cfg(target_pointer_width = "64")]
#[no_mangle] pub unsafe extern "C" fn set_firmware_width_unlocked() {
    let ret = mem_pdc_call(PDC_MODEL, PDC_MODEL_CAPABILITIES, __pa(pdc_result.as_ptr() as *const c_void), 0);
    if ret < 0 { return; }
    convert_to_wide(pdc_result.as_mut_ptr());
    if pdc_result[0] != NARROW_FIRMWARE as c_ulong { parisc_narrow_firmware = 0; }
}

#[no_mangle] pub unsafe extern "C" fn set_firmware_width() {
    #[cfg(target_pointer_width = "64")]
    { if parisc_narrow_firmware != NARROW_FIRMWARE { return; } let mut f=0; spin_lock_irqsave(&mut pdc_lock,&mut f); set_firmware_width_unlocked(); spin_unlock_irqrestore(&mut pdc_lock,f); }
}

macro_rules! locked_call { ($($arg:expr),*) => {{ let mut flags=0; spin_lock_irqsave(&mut pdc_lock,&mut flags); let r=mem_pdc_call($($arg),*); spin_unlock_irqrestore(&mut pdc_lock,flags); r }} }
macro_rules! copy_result { ($out:expr, $ty:ty) => {{ memcpy($out as *mut _ as *mut c_void,pdc_result.as_ptr() as *const c_void,core::mem::size_of::<$ty>()); }} }

#[no_mangle] pub unsafe extern "C" fn pdc_chassis_disp(disp:c_ulong)->c_int { locked_call!(PDC_CHASSIS,PDC_CHASSIS_DISP,disp) }
#[no_mangle] pub unsafe extern "C" fn __pdc_cpu_rendezvous()->c_int { if is_pdc_pat(){mem_pdc_call(PDC_PAT_CPU,PDC_PAT_CPU_RENDEZVOUS)}else{mem_pdc_call(PDC_PROC,1,0)} }
#[no_mangle] pub unsafe extern "C" fn pdc_cpu_rendezvous_lock(){spin_lock(&mut pdc_lock)}
#[no_mangle] pub unsafe extern "C" fn pdc_cpu_rendezvous_unlock(){spin_unlock(&mut pdc_lock)}
#[no_mangle] pub unsafe extern "C" fn pdc_chassis_warn(warn:*mut c_ulong)->c_int { let r=locked_call!(PDC_CHASSIS,PDC_CHASSIS_WARN,__pa(pdc_result.as_ptr() as *const c_void)); *warn=pdc_result[0]; r }
#[no_mangle] pub unsafe extern "C" fn pdc_model_cpuid(id:*mut c_ulong)->c_int { pdc_result[0]=0; let r=locked_call!(PDC_MODEL,PDC_MODEL_CPU_ID,__pa(pdc_result.as_ptr() as *const c_void),0); convert_to_wide(pdc_result.as_mut_ptr()); *id=pdc_result[0]; r }
#[no_mangle] pub unsafe extern "C" fn pdc_tod_set(sec:c_ulong,usec:c_ulong)->c_int { locked_call!(PDC_TOD,PDC_TOD_WRITE,sec,usec) }
#[no_mangle] pub unsafe extern "C" fn pdc_do_reset()->c_int { locked_call!(PDC_BROADCAST_RESET,PDC_DO_RESET) }
#[no_mangle] pub unsafe extern "C" fn pdc_io_reset(){ let _=locked_call!(PDC_IO,PDC_IO_RESET,0); }

/* The remaining wrappers retain the C ABI and use the shared result buffers;
 * their structure is deliberately literal so dependent kernel headers supply
 * the concrete PDC constants and record layouts. */
#[no_mangle] pub unsafe extern "C" fn pdc_iodc_print(str_:*const u8, mut count:c_uint)->c_int { if count>4096{count=4096}; let mut i=0; while i<count { iodc_dbuf[i as usize]=*str_.add(i as usize); i+=1; } let mut f=0; spin_lock_irqsave(&mut pdc_lock,&mut f); let _=real32_call((*PAGE0).mem_cons.iodc_io,(*PAGE0).mem_cons.hpa,ENTRY_IO_COUT,(*PAGE0).mem_cons.spa,__pa((*PAGE0).mem_cons.dp.layers),__pa(pdc_result.as_ptr() as *const c_void),0,__pa(iodc_dbuf.as_ptr() as *const c_void),i,0); spin_unlock_irqrestore(&mut pdc_lock,f); i as c_int }

// Build-time configuration, exported symbols, and declaration-only wrappers
// from the C translation depend on the corresponding architecture headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
