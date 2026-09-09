// SPDX-License-Identifier: GPL-2.0
/* Standard user space access functions based on mvcp/mvcs. */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn get_lowcore() -> *mut lowcore;
    fn local_ctl_store(number: i32, value: *mut ctlreg);
    fn panic(format: *const u8, ...);
    fn skey_regions_initialize();
}

#[repr(C)]
pub struct ctlreg { pub val: usize }
#[repr(C)]
pub struct lowcore { pub user_asce: ctlreg, pub kernel_asce: ctlreg }

const CMPXCHG_USER_KEY_MAX_LOOPS: usize = 128;
// PAGE_DEFAULT_KEY, EAGAIN, and the assembly exception-table/key-region
// helpers are provided by the architecture headers.
extern "C" {
    static PAGE_DEFAULT_KEY: usize;
}

#[cfg(CONFIG_DEBUG_ENTRY)]
pub unsafe extern "C" fn debug_user_asce(exit: i32) {
    let lc = &*get_lowcore();
    let mut cr1 = ctlreg { val: 0 };
    let mut cr7 = ctlreg { val: 0 };
    local_ctl_store(1, &mut cr1);
    local_ctl_store(7, &mut cr7);
    if cr1.val == lc.user_asce.val && cr7.val == lc.user_asce.val { return; }
    panic(b"incorrect ASCE on kernel %s\n\0".as_ptr(),
          if exit != 0 { b"exit\0".as_ptr() } else { b"entry\0".as_ptr() });
}

#[inline(always)]
unsafe fn __cmpxchg_key_small(address: *mut c_void, uval: *mut u32,
                              old: u32, new: u32, mask: u32, key: usize) -> i32 {
    let mut prev: u32 = 0;
    let mut count: usize = CMPXCHG_USER_KEY_MAX_LOOPS;
    let mut rc: i32 = 0;
    skey_regions_initialize();
    // The following volatile instruction sequence is the direct s390
    // translation of the C inline assembly, including its exception paths.
    core::arch::asm!(
        "spka 0({key})", "0: l {prev}, 0({address})", "nr {prev}, {mask}",
        "xilf {mask}, 0xffffffff", "or {new}, {prev}", "or {prev}, {old}",
        "2: lr {old}, {prev}", "3: cs {prev}, {new}, 0({address})",
        "4: jnl 5f", "xr {old}, {prev}", "xr {new}, {old}",
        "nr {old}, {mask}", "jnz 5f", "brct {count}, 2b", "5: spka {default_key}",
        key = in(reg) key << 4, address = in(reg) address, prev = lateout(reg) prev,
        old = inout(reg) old, new = inout(reg) new, mask = inout(reg) mask,
        count = inout(reg) count, default_key = in(reg) PAGE_DEFAULT_KEY,
        options(nostack, preserves_flags));
    *uval = prev;
    if count == 0 { rc = -11; }
    rc
}

pub unsafe extern "C" fn __cmpxchg_key1(addr: *mut c_void, uval: *mut u8,
    old: u8, new: u8, key: usize) -> i32 {
    let mut address = addr as usize;
    let shift = ((3 ^ (address & 3)) << 3) as u32;
    address ^= address & 3;
    let _old = (old as u32) << shift;
    let _new = (new as u32) << shift;
    let mask = !(0xffu32 << shift);
    let mut prev = 0; let rc = __cmpxchg_key_small(address as *mut c_void, &mut prev, _old, _new, mask, key);
    *uval = (prev >> shift) as u8; rc
}

pub unsafe extern "C" fn __cmpxchg_key2(addr: *mut c_void, uval: *mut u16,
    old: u16, new: u16, key: usize) -> i32 {
    let mut address = addr as usize;
    let shift = ((2 ^ (address & 2)) << 3) as u32;
    address ^= address & 2;
    let _old = (old as u32) << shift; let _new = (new as u32) << shift;
    let mask = !(0xffffu32 << shift); let mut prev = 0;
    let rc = __cmpxchg_key_small(address as *mut c_void, &mut prev, _old, _new, mask, key);
    *uval = (prev >> shift) as u16; rc
}

pub unsafe extern "C" fn __cmpxchg_key4(address: *mut c_void, uval: *mut u32,
    old: u32, new: u32, key: usize) -> i32 {
    let mut prev = old; let mut rc = 0i32; skey_regions_initialize();
    core::arch::asm!("spka 0({key})", "cs {prev}, {new}, 0({address})", "spka {default_key}",
        key = in(reg) key << 4, address = in(reg) address, prev = inout(reg) prev,
        new = in(reg) new, default_key = in(reg) PAGE_DEFAULT_KEY,
        inout("r0") rc, options(nostack, preserves_flags));
    *uval = prev; rc
}

pub unsafe extern "C" fn __cmpxchg_key8(address: *mut c_void, uval: *mut usize,
    old: usize, new: usize, key: usize) -> i32 {
    let mut prev = old; let mut rc = 0i32; skey_regions_initialize();
    core::arch::asm!("spka 0({key})", "csg {prev}, {new}, 0({address})", "spka {default_key}",
        key = in(reg) key << 4, address = in(reg) address, prev = inout(reg) prev,
        new = in(reg) new, default_key = in(reg) PAGE_DEFAULT_KEY,
        inout("r0") rc, options(nostack, preserves_flags));
    *uval = prev; rc
}

pub unsafe extern "C" fn __cmpxchg_key16(address: *mut c_void, uval: *mut u128,
    old: u128, new: u128, key: usize) -> i32 {
    let mut prev = old; let mut rc = 0i32; skey_regions_initialize();
    core::arch::asm!("spka 0({key})", "cdsg {prev}, {new}, 0({address})", "spka {default_key}",
        key = in(reg) key << 4, address = in(reg) address, prev = inout(reg) prev,
        new = in(reg) new, default_key = in(reg) PAGE_DEFAULT_KEY,
        inout("r0") rc, options(nostack, preserves_flags));
    *uval = prev; rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
