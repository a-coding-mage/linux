// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *
 * Floating-point emulation code
 *  Copyright (C) 2001 Hewlett-Packard (Paul Bame) <bame@debian.org>
 */
/*
 *  linux/arch/math-emu/driver.c.c
 *
 *  decodes and dispatches unimplemented FPU instructions
 *
 *  Copyright (C) 1999, 2000  Philipp Rumpf <prumpf@tux.org>
 *  Copyright (C) 2001        Hewlett-Packard <bame@debian.org>
 */

const FPTPOS: u32 = 31;
const FPR1POS: u32 = 10;

#[inline]
const fn extru(r: u32, pos: u32, len: u32) -> u32 {
    (r >> (31 - pos)) & ((1u32 << len) - 1)
}

const FPUDEBUG: i32 = 0;

#[inline] const fn fp0ce_uid(i: u32) -> u32 { (i >> 6) & 3 }
#[inline] const fn fp0ce_class(i: u32) -> u32 { (i >> 9) & 3 }
#[inline] const fn fp0ce_subop(i: u32) -> u32 { (i >> 13) & 7 }
#[inline] const fn fp0ce_subop1(i: u32) -> u32 { (i >> 15) & 7 }
#[inline] const fn fp0c_format(i: u32) -> u32 { (i >> 11) & 3 }
#[inline] const fn fp0e_format(i: u32) -> u32 { (i >> 11) & 1 }

#[inline] const fn fppm_subop(i: u32) -> u32 { (i >> 9) & 0x1f }
#[inline] const fn fp2e_subop(i: u32) -> u32 { (i >> 5) & 1 }
#[inline] const fn fp2e_format(i: u32) -> u32 { (i >> 11) & 1 }
#[inline] const fn fpx6_format(i: u32) -> u32 { i & 0x1f }

const FPSW_V: u32 = 1 << 4;
const FPSW_Z: u32 = 1 << 3;
const FPSW_O: u32 = 1 << 2;
const FPSW_U: u32 = 1 << 1;
const FPSW_I: u32 = 1 << 0;

#[inline] const fn fpsw_flags(w: u32) -> u32 { w >> 27 }
#[inline] const fn fpsw_enable(w: u32) -> u32 { w & 0x1f }

extern "C" {
    fn printbinary(x: usize, nbits: i32);
    fn decode_fpu(fregs: *mut u64, code: i32) -> i32;
    fn force_sig_fault(sig: i32, code: i32, addr: *mut core::ffi::c_void);
}

// Supplied by the PA-RISC kernel headers.
#[repr(C)]
pub struct pt_regs {
    pub fr: [u64; 32],
    pub iaoq: [usize; 2],
}

const SIGFPE: i32 = 8;

/// Handle a floating point exception. Return zero if the faulting
/// instruction can be completed successfully.
#[no_mangle]
pub unsafe extern "C" fn handle_fpe(regs: *mut pt_regs) -> i32 {
    // The intermediate copy includes an artificial last entry expected by
    // the FPU emulation code. The final entry is zero.
    let mut frcopy = [0u64; 36];
    core::ptr::copy_nonoverlapping(
        (*regs).fr.as_ptr(),
        frcopy.as_mut_ptr(),
        (*regs).fr.len(),
    );
    frcopy[32] = 0;

    let orig_sw = core::ptr::read_unaligned(frcopy.as_ptr() as *const u32);

    if FPUDEBUG != 0 {
        printbinary(orig_sw as usize, 32);
    }

    let signalcode = decode_fpu(frcopy.as_mut_ptr(), 0x666);
    let sw = core::ptr::read_unaligned(frcopy.as_ptr() as *const u32);
    let _ = sw;

    core::ptr::copy_nonoverlapping(
        frcopy.as_ptr(),
        (*regs).fr.as_mut_ptr(),
        (*regs).fr.len(),
    );

    if signalcode != 0 {
        let sig = signalcode >> 24;

        if sig == SIGFPE {
            // Clear floating point trap bit to avoid trapping again on the
            // first floating-point instruction in the userspace signal handler.
            (*regs).fr[0] &= !(1u64 << 38);
        }
        force_sig_fault(
            sig,
            signalcode & 0xffffff,
            (*regs).iaoq[0] as *mut core::ffi::c_void,
        );
        return -1;
    }

    if signalcode != 0 { -1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
