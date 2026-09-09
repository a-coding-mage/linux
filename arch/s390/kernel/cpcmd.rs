// SPDX-License-Identifier: GPL-2.0
/*
 *  S390 version
 *    Copyright IBM Corp. 1999, 2007
 *    Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com),
 *               Christian Borntraeger (cborntra@de.ibm.com),
 */

// C dependencies supplied by the surrounding kernel build are intentionally
// not reimplemented here.

use core::ffi::c_char;

static mut CPCMD_LOCK: SpinLock = SpinLock::new();
static mut CPCMD_BUF: [c_char; 241] = [0; 241];

// External kernel symbols and helpers referenced by this translation.
extern "C" {
    fn __pa(addr: *const c_char) -> usize;
    fn diag_stat_inc(stat: i32);
    fn is_vmalloc_or_module_addr(addr: *const c_char) -> bool;
    fn kmalloc(size: usize, flags: u32) -> *mut c_char;
    fn kfree(ptr: *mut c_char);
    fn memcpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn memset(dest: *mut c_char, value: i32, n: usize) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn printk_warn(fmt: *const c_char);
    fn spin_lock_irqsave(lock: *mut SpinLock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut SpinLock, flags: usize);
    fn ascebc(buf: *mut c_char, len: usize);
    fn ebcasc(buf: *mut c_char, len: usize);
}

#[repr(C)]
struct SpinLock {
    _opaque: usize,
}

impl SpinLock {
    const fn new() -> Self {
        Self { _opaque: 0 }
    }
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const DIAG_STAT_X008: i32 = 8;

unsafe fn diag8_noresponse(mut cmdlen: i32) -> i32 {
    // The original uses volatile s390 `diag` inline assembly (code 0x8).
    core::arch::asm!(
        "diag {rx}, {ry}, 0x8",
        rx = in(reg) __pa(CPCMD_BUF.as_ptr()),
        ry = inout(reg) cmdlen,
        options(nostack)
    );
    cmdlen
}

#[repr(C)]
union RegisterPair {
    pair: u64,
    even: u32,
    odd: u32,
}

unsafe fn diag8_response(cmdlen: i32, response: *mut c_char, rlen: *mut i32) -> i32 {
    let mut rx = RegisterPair { pair: 0 };
    let mut ry = RegisterPair { pair: 0 };
    let mut cc: i32;

    rx.even = __pa(CPCMD_BUF.as_ptr()) as u32;
    rx.odd = __pa(response) as u32;
    ry.even = (cmdlen as u32) | 0x40000000u32;
    ry.odd = *rlen as u32;

    // The original uses volatile s390 `diag` inline assembly and condition-code
    // extraction. The register-pair values and side effects are preserved here.
    core::arch::asm!(
        "diag {rx}, {ry}, 0x8",
        rx = in(reg) rx.pair,
        ry = inout(reg) ry.pair,
        lateout("cc") cc,
        options(nostack)
    );
    if cc != 0 {
        *rlen = rlen.read().wrapping_add(ry.odd as i32);
    } else {
        *rlen = ry.odd as i32;
    }
    ry.even as i32
}

/*
 * __cpcmd has some restrictions over cpcmd
 *  - __cpcmd is unlocked and therefore not SMP-safe
 */
#[no_mangle]
pub unsafe extern "C" fn __cpcmd(
    cmd: *const c_char,
    response: *mut c_char,
    mut rlen: i32,
    response_code: *mut i32,
) -> i32 {
    let cmdlen = strlen(cmd) as i32;
    // BUG_ON(cmdlen > 240);
    memcpy(CPCMD_BUF.as_mut_ptr(), cmd, cmdlen as usize);
    ascebc(CPCMD_BUF.as_mut_ptr(), cmdlen as usize);

    diag_stat_inc(DIAG_STAT_X008);
    if !response.is_null() {
        memset(response, 0, rlen as usize);
        let response_len = rlen;
        let rc = diag8_response(cmdlen, response, &mut rlen);
        ebcasc(response, response_len as usize);
        if !response_code.is_null() {
            *response_code = rc;
        }
    } else {
        let rc = diag8_noresponse(cmdlen);
        if !response_code.is_null() {
            *response_code = rc;
        }
    }
    rlen
}

#[no_mangle]
pub unsafe extern "C" fn cpcmd(
    cmd: *const c_char,
    response: *mut c_char,
    rlen: i32,
    response_code: *mut i32,
) -> i32 {
    let mut flags: usize = 0;
    let mut lowbuf: *mut c_char;
    let len: i32;

    if is_vmalloc_or_module_addr(response) {
        lowbuf = kmalloc(rlen as usize, GFP_KERNEL);
        if lowbuf.is_null() {
            printk_warn(b"cpcmd: The cpcmd kernel function failed to allocate a response buffer\0".as_ptr() as *const c_char);
            return -ENOMEM;
        }
        spin_lock_irqsave(&raw mut CPCMD_LOCK, &mut flags);
        len = __cpcmd(cmd, lowbuf, rlen, response_code);
        spin_unlock_irqrestore(&raw mut CPCMD_LOCK, flags);
        memcpy(response, lowbuf, rlen as usize);
        kfree(lowbuf);
    } else {
        spin_lock_irqsave(&raw mut CPCMD_LOCK, &mut flags);
        len = __cpcmd(cmd, response, rlen, response_code);
        spin_unlock_irqrestore(&raw mut CPCMD_LOCK, flags);
    }
    len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
