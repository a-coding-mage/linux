/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_CSKY_TRAPS_H
// Dependency supplied by the surrounding kernel translation: linux/linkage.h

pub const VEC_RESET: i32 = 0;
pub const VEC_ALIGN: i32 = 1;
pub const VEC_ACCESS: i32 = 2;
pub const VEC_ZERODIV: i32 = 3;
pub const VEC_ILLEGAL: i32 = 4;
pub const VEC_PRIV: i32 = 5;
pub const VEC_TRACE: i32 = 6;
pub const VEC_BREAKPOINT: i32 = 7;
pub const VEC_UNRECOVER: i32 = 8;
pub const VEC_SOFTRESET: i32 = 9;
pub const VEC_AUTOVEC: i32 = 10;
pub const VEC_FAUTOVEC: i32 = 11;
pub const VEC_HWACCEL: i32 = 12;

pub const VEC_TLBMISS: i32 = 14;
pub const VEC_TLBMODIFIED: i32 = 15;

pub const VEC_TRAP0: i32 = 16;
pub const VEC_TRAP1: i32 = 17;
pub const VEC_TRAP2: i32 = 18;
pub const VEC_TRAP3: i32 = 19;

pub const VEC_TLBINVALIDL: i32 = 20;
pub const VEC_TLBINVALIDS: i32 = 21;

pub const VEC_PRFL: i32 = 29;
pub const VEC_FPE: i32 = 30;

// Declaration supplied by the surrounding translation unit.
unsafe extern "C" {
    pub static mut vec_base: *mut *mut core::ffi::c_void;
}

#[macro_export]
macro_rules! VEC_INIT {
    ($i:expr, $func:expr) => {{
        unsafe {
            *$crate::vec_base.add($i) = $func as *mut core::ffi::c_void;
        }
    }};
}

pub unsafe extern "C" fn csky_alignment(regs: *mut pt_regs);

pub unsafe extern "C" fn do_trap_unknown(regs: *mut pt_regs);
pub unsafe extern "C" fn do_trap_zdiv(regs: *mut pt_regs);
pub unsafe extern "C" fn do_trap_buserr(regs: *mut pt_regs);
pub unsafe extern "C" fn do_trap_misaligned(regs: *mut pt_regs);
pub unsafe extern "C" fn do_trap_bkpt(regs: *mut pt_regs);
pub unsafe extern "C" fn do_trap_illinsn(regs: *mut pt_regs);
pub unsafe extern "C" fn do_trap_fpe(regs: *mut pt_regs);
pub unsafe extern "C" fn do_trap_priv(regs: *mut pt_regs);
pub unsafe extern "C" fn trap_c(regs: *mut pt_regs);

pub unsafe extern "C" fn do_notify_resume(
    regs: *mut pt_regs,
    thread_info_flags: c_ulong,
);

pub unsafe extern "C" fn do_page_fault(regs: *mut pt_regs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
