/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not defined here.

pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = usize;

#[repr(C)] pub struct loongarch_fpu { pub fcsr: c_uint, _private: [u8; 0] }
#[repr(C)] pub struct task_struct { pub thread: thread_struct }
#[repr(C)] pub struct thread_struct { pub fpu: loongarch_fpu }

#[repr(C)]
pub struct sigcontext {
    _private: [u8; 0],
}

extern "C" {
    pub fn kernel_fpu_begin();
    pub fn kernel_fpu_end();
    pub fn _init_fpu(fcsr: c_uint);
    pub fn _save_fp(fpu: *mut loongarch_fpu);
    pub fn _restore_fp(fpu: *mut loongarch_fpu);
    pub fn _save_fp_context(fpregs: *mut core::ffi::c_void, fcc: *mut core::ffi::c_void, csr: *mut core::ffi::c_void) -> c_int;
    pub fn _restore_fp_context(fpregs: *mut core::ffi::c_void, fcc: *mut core::ffi::c_void, csr: *mut core::ffi::c_void) -> c_int;
    pub fn _save_lsx(fpu: *mut loongarch_fpu);
    pub fn _restore_lsx(fpu: *mut loongarch_fpu);
    pub fn _init_lsx_upper();
    pub fn _restore_lsx_upper(fpu: *mut loongarch_fpu);
    pub fn _save_lsx_context(fpregs: *mut core::ffi::c_void, fcc: *mut core::ffi::c_void, fcsr: *mut core::ffi::c_void) -> c_int;
    pub fn _restore_lsx_context(fpregs: *mut core::ffi::c_void, fcc: *mut core::ffi::c_void, fcsr: *mut core::ffi::c_void) -> c_int;
    pub fn _save_lasx(fpu: *mut loongarch_fpu);
    pub fn _restore_lasx(fpu: *mut loongarch_fpu);
    pub fn _init_lasx_upper();
    pub fn _restore_lasx_upper(fpu: *mut loongarch_fpu);
    pub fn _save_lasx_context(fpregs: *mut core::ffi::c_void, fcc: *mut core::ffi::c_void, fcsr: *mut core::ffi::c_void) -> c_int;
    pub fn _restore_lasx_context(fpregs: *mut core::ffi::c_void, fcc: *mut core::ffi::c_void, fcsr: *mut core::ffi::c_void) -> c_int;
}

// The following inline functions preserve the source header's implementation.
// Kernel-provided types, constants, globals, and helper functions are external.

#[inline]
pub unsafe fn mask_fcsr_x(fcsr: c_ulong) -> c_ulong {
    fcsr & ((fcsr & FPU_CSR_ALL_E as c_ulong) << (ffs(FPU_CSR_ALL_X) - ffs(FPU_CSR_ALL_E)))
}

#[inline]
pub unsafe fn is_fp_enabled() -> c_int {
    if (csr_read32(LOONGARCH_CSR_EUEN) & CSR_EUEN_FPEN) != 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn is_lsx_enabled() -> c_int {
    if !cpu_has_lsx { return 0; }
    if (csr_read32(LOONGARCH_CSR_EUEN) & CSR_EUEN_LSXEN) != 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn is_lasx_enabled() -> c_int {
    if !cpu_has_lasx { return 0; }
    if (csr_read32(LOONGARCH_CSR_EUEN) & CSR_EUEN_LASXEN) != 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn is_simd_enabled() -> c_int { is_lsx_enabled() | is_lasx_enabled() }

#[inline]
pub unsafe fn enable_fpu() { set_csr_euen(CSR_EUEN_FPEN); }
#[inline]
pub unsafe fn disable_fpu() { clear_csr_euen(CSR_EUEN_FPEN); }
#[inline]
pub unsafe fn clear_fpu_owner() { clear_thread_flag(TIF_USEDFPU); }
#[inline]
pub unsafe fn is_fpu_owner() -> c_int { test_thread_flag(TIF_USEDFPU) }

#[inline]
pub unsafe fn __own_fpu() {
    enable_fpu();
    set_thread_flag(TIF_USEDFPU);
    *KSTK_EUEN(current) |= CSR_EUEN_FPEN;
}

#[inline]
pub unsafe fn own_fpu_inatomic(restore: c_int) {
    if cpu_has_fpu && !is_fpu_owner() {
        __own_fpu();
        if restore != 0 { _restore_fp(&mut (*current).thread.fpu); }
    }
}
#[inline]
pub unsafe fn own_fpu(restore: c_int) { preempt_disable(); own_fpu_inatomic(restore); preempt_enable(); }

#[inline]
pub unsafe fn lose_fpu_inatomic(save: c_int, tsk: *mut task_struct) {
    if is_fpu_owner() != 0 {
        if is_simd_enabled() == 0 {
            if save != 0 { _save_fp(&mut (*tsk).thread.fpu); }
            disable_fpu();
        } else {
            if save != 0 { if is_lasx_enabled() == 0 { save_lsx(tsk); } else { save_lasx(tsk); } }
            disable_fpu(); disable_lsx(); disable_lasx();
            clear_tsk_thread_flag(tsk, TIF_USEDSIMD);
        }
        clear_tsk_thread_flag(tsk, TIF_USEDFPU);
    }
    *KSTK_EUEN(tsk) &= !(CSR_EUEN_FPEN | CSR_EUEN_LSXEN | CSR_EUEN_LASXEN);
}
#[inline]
pub unsafe fn lose_fpu(save: c_int) { preempt_disable(); lose_fpu_inatomic(save, current); preempt_enable(); }

#[inline]
pub unsafe fn init_fpu() {
    let fcsr = (*current).thread.fpu.fcsr;
    __own_fpu(); _init_fpu(fcsr); set_used_math();
}
#[inline]
pub unsafe fn save_fp(tsk: *mut task_struct) { if cpu_has_fpu { _save_fp(&mut (*tsk).thread.fpu); } }
#[inline]
pub unsafe fn restore_fp(tsk: *mut task_struct) { if cpu_has_fpu { _restore_fp(&mut (*tsk).thread.fpu); } }

#[inline]
pub unsafe fn save_fpu_regs(tsk: *mut task_struct) {
    if tsk == current {
        preempt_disable();
        let euen = csr_read32(LOONGARCH_CSR_EUEN);
        if (euen & CSR_EUEN_LASXEN) != 0 { _save_lasx(&mut (*current).thread.fpu); }
        else if (euen & CSR_EUEN_LSXEN) != 0 { _save_lsx(&mut (*current).thread.fpu); }
        else if (euen & CSR_EUEN_FPEN) != 0 { _save_fp(&mut (*current).thread.fpu); }
        preempt_enable();
    }
}
#[inline]
pub unsafe fn is_simd_owner() -> c_int { test_thread_flag(TIF_USEDSIMD) }

// Build-time CONFIG_CPU_HAS_LSX/CONFIG_CPU_HAS_LASX branches are preserved
// with cfg attributes below; the dependent kernel definitions are external.
#[cfg(CONFIG_CPU_HAS_LSX)]
#[inline] pub unsafe fn enable_lsx() { if cpu_has_lsx { csr_xchg32(CSR_EUEN_LSXEN, CSR_EUEN_LSXEN, LOONGARCH_CSR_EUEN); } }
#[cfg(CONFIG_CPU_HAS_LSX)]
#[inline] pub unsafe fn disable_lsx() { if cpu_has_lsx { csr_xchg32(0, CSR_EUEN_LSXEN, LOONGARCH_CSR_EUEN); } }
#[cfg(CONFIG_CPU_HAS_LSX)]
#[inline] pub unsafe fn save_lsx(t: *mut task_struct) { if cpu_has_lsx { _save_lsx(&mut (*t).thread.fpu); } }
#[cfg(CONFIG_CPU_HAS_LSX)]
#[inline] pub unsafe fn restore_lsx(t: *mut task_struct) { if cpu_has_lsx { _restore_lsx(&mut (*t).thread.fpu); } }
#[cfg(CONFIG_CPU_HAS_LSX)]
#[inline] pub unsafe fn init_lsx_upper() { if cpu_has_lsx { _init_lsx_upper(); } }
#[cfg(CONFIG_CPU_HAS_LSX)]
#[inline] pub unsafe fn restore_lsx_upper(t: *mut task_struct) { if cpu_has_lsx { _restore_lsx_upper(&mut (*t).thread.fpu); } }
#[cfg(not(CONFIG_CPU_HAS_LSX))]
#[inline] pub unsafe fn enable_lsx() {}
#[cfg(not(CONFIG_CPU_HAS_LSX))]
#[inline] pub unsafe fn disable_lsx() {}
#[cfg(not(CONFIG_CPU_HAS_LSX))]
#[inline] pub unsafe fn save_lsx(_t: *mut task_struct) {}
#[cfg(not(CONFIG_CPU_HAS_LSX))]
#[inline] pub unsafe fn restore_lsx(_t: *mut task_struct) {}
#[cfg(not(CONFIG_CPU_HAS_LSX))]
#[inline] pub unsafe fn init_lsx_upper() {}
#[cfg(not(CONFIG_CPU_HAS_LSX))]
#[inline] pub unsafe fn restore_lsx_upper(_t: *mut task_struct) {}

#[cfg(CONFIG_CPU_HAS_LASX)]
#[inline] pub unsafe fn enable_lasx() { if cpu_has_lasx { csr_xchg32(CSR_EUEN_LASXEN, CSR_EUEN_LASXEN, LOONGARCH_CSR_EUEN); } }
#[cfg(CONFIG_CPU_HAS_LASX)]
#[inline] pub unsafe fn disable_lasx() { if cpu_has_lasx { csr_xchg32(0, CSR_EUEN_LASXEN, LOONGARCH_CSR_EUEN); } }
#[cfg(CONFIG_CPU_HAS_LASX)]
#[inline] pub unsafe fn save_lasx(t: *mut task_struct) { if cpu_has_lasx { _save_lasx(&mut (*t).thread.fpu); } }
#[cfg(CONFIG_CPU_HAS_LASX)]
#[inline] pub unsafe fn restore_lasx(t: *mut task_struct) { if cpu_has_lasx { _restore_lasx(&mut (*t).thread.fpu); } }
#[cfg(CONFIG_CPU_HAS_LASX)]
#[inline] pub unsafe fn init_lasx_upper() { if cpu_has_lasx { _init_lasx_upper(); } }
#[cfg(CONFIG_CPU_HAS_LASX)]
#[inline] pub unsafe fn restore_lasx_upper(t: *mut task_struct) { if cpu_has_lasx { _restore_lasx_upper(&mut (*t).thread.fpu); } }
#[cfg(not(CONFIG_CPU_HAS_LASX))]
#[inline] pub unsafe fn enable_lasx() {}
#[cfg(not(CONFIG_CPU_HAS_LASX))]
#[inline] pub unsafe fn disable_lasx() {}
#[cfg(not(CONFIG_CPU_HAS_LASX))]
#[inline] pub unsafe fn save_lasx(_t: *mut task_struct) {}
#[cfg(not(CONFIG_CPU_HAS_LASX))]
#[inline] pub unsafe fn restore_lasx(_t: *mut task_struct) {}
#[cfg(not(CONFIG_CPU_HAS_LASX))]
#[inline] pub unsafe fn init_lasx_upper() {}
#[cfg(not(CONFIG_CPU_HAS_LASX))]
#[inline] pub unsafe fn restore_lasx_upper(_t: *mut task_struct) {}

#[inline]
pub unsafe fn thread_lsx_context_live() -> c_int { if !cpu_has_lsx { 0 } else { test_thread_flag(TIF_LSX_CTX_LIVE) } }
#[inline]
pub unsafe fn thread_lasx_context_live() -> c_int { if !cpu_has_lasx { 0 } else { test_thread_flag(TIF_LASX_CTX_LIVE) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
