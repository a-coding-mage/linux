/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 2014 Imagination Technologies Ltd.
 * Author: Markos Chandras <markos.chandras@imgtec.com>
 */

#[repr(C)]
pub struct mips_r2_emulator_stats {
    pub movs: u64,
    pub hilo: u64,
    pub muls: u64,
    pub divs: u64,
    pub dsps: u64,
    pub bops: u64,
    pub traps: u64,
    pub fpus: u64,
    pub loads: u64,
    pub stores: u64,
    pub llsc: u64,
    pub dsemul: u64,
}

#[repr(C)]
pub struct mips_r2br_emulator_stats {
    pub jrs: u64,
    pub bltzl: u64,
    pub bgezl: u64,
    pub bltzll: u64,
    pub bgezll: u64,
    pub bltzall: u64,
    pub bgezall: u64,
    pub bltzal: u64,
    pub bgezal: u64,
    pub beql: u64,
    pub bnel: u64,
    pub blezl: u64,
    pub bgtzl: u64,
}

/* CONFIG_DEBUG_FS conditionally enables the statistics operations. */
#[cfg(feature = "CONFIG_DEBUG_FS")]
macro_rules! MIPS_R2_STATS {
    ($M:ident) => {{
        preempt_disable();
        __this_cpu_inc(mipsr2emustats.$M);
        let mut nir: u32 = 0;
        let err = __get_user(&mut nir, regs.cp0_epc as *mut u32);
        if err == 0 {
            if nir == BREAK_MATH(0) {
                __this_cpu_inc(mipsr2bdemustats.$M);
            }
        }
        preempt_enable();
    }};
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
macro_rules! MIPS_R2BR_STATS {
    ($M:ident) => {{
        preempt_disable();
        __this_cpu_inc(mipsr2bremustats.$M);
        preempt_enable();
    }};
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
macro_rules! MIPS_R2_STATS {
    ($M:ident) => {{}};
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
macro_rules! MIPS_R2BR_STATS {
    ($M:ident) => {{}};
}

#[repr(C)]
pub struct r2_decoder_table {
    pub mask: u32,
    pub code: u32,
    pub func: Option<unsafe extern "C" fn(regs: *mut pt_regs, inst: u32) -> i32>,
}

extern "C" {
    pub fn do_trap_or_bp(
        regs: *mut pt_regs,
        code: u32,
        si_code: i32,
        str_: *const core::ffi::c_char,
    );
}

#[cfg(not(feature = "CONFIG_MIPSR2_TO_R6_EMULATOR"))]
pub static mut mipsr2_emulation: i32 = 0;

#[cfg(not(feature = "CONFIG_MIPSR2_TO_R6_EMULATOR"))]
#[inline]
pub unsafe fn mipsr2_decoder(
    _regs: *mut pt_regs,
    _inst: u32,
    _fcr31: *mut u64,
) -> i32 {
    0
}

#[cfg(feature = "CONFIG_MIPSR2_TO_R6_EMULATOR")]
extern "C" {
    pub static mut mipsr2_emulation: i32;
    pub fn mipsr2_decoder(regs: *mut pt_regs, inst: u32, fcr31: *mut u64) -> i32;
}

/* NO_R6EMU = (cpu_has_mips_r6 && !mipsr2_emulation). */
macro_rules! NO_R6EMU {
    () => {
        cpu_has_mips_r6 && unsafe { !mipsr2_emulation }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
