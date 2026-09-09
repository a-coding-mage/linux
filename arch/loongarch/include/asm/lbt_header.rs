/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Author: Qi Hu <huqi@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/cpu.h, asm/current.h, asm/loongarch.h, and asm/processor.h.

extern "C" {
    pub fn _init_lbt();
    pub fn _save_lbt(lbt: *mut loongarch_lbt);
    pub fn _restore_lbt(lbt: *mut loongarch_lbt);
    pub fn _save_lbt_context(regs: *mut core::ffi::c_void, eflags: *mut core::ffi::c_void) -> i32;
    pub fn _restore_lbt_context(regs: *mut core::ffi::c_void, eflags: *mut core::ffi::c_void) -> i32;
    pub fn _save_ftop_context(ftop: *mut core::ffi::c_void) -> i32;
    pub fn _restore_ftop_context(ftop: *mut core::ffi::c_void) -> i32;
}

#[repr(C)]
pub struct loongarch_lbt {
    _private: [u8; 0],
}

pub unsafe fn is_lbt_enabled() -> i32 {
    if !cpu_has_lbt {
        return 0;
    }

    if (csr_read32(LOONGARCH_CSR_EUEN) & CSR_EUEN_LBTEN) != 0 { 1 } else { 0 }
}

pub unsafe fn is_lbt_owner() -> i32 {
    test_thread_flag(TIF_USEDLBT)
}

#[cfg(feature = "CONFIG_CPU_HAS_LBT")]
pub unsafe fn enable_lbt() {
    if cpu_has_lbt {
        csr_xchg32(CSR_EUEN_LBTEN, CSR_EUEN_LBTEN, LOONGARCH_CSR_EUEN);
    }
}

#[cfg(feature = "CONFIG_CPU_HAS_LBT")]
pub unsafe fn disable_lbt() {
    if cpu_has_lbt {
        csr_xchg32(0, CSR_EUEN_LBTEN, LOONGARCH_CSR_EUEN);
    }
}

#[cfg(feature = "CONFIG_CPU_HAS_LBT")]
pub unsafe fn __own_lbt() {
    enable_lbt();
    set_thread_flag(TIF_USEDLBT);
    // C macro lvalue: KSTK_EUEN(current) |= CSR_EUEN_LBTEN;
}

#[cfg(feature = "CONFIG_CPU_HAS_LBT")]
pub unsafe fn own_lbt_inatomic(restore: i32) {
    if cpu_has_lbt && is_lbt_owner() == 0 {
        __own_lbt();
        if restore != 0 {
            _restore_lbt(core::ptr::null_mut()); // current->thread.lbt
        }
    }
}

#[cfg(feature = "CONFIG_CPU_HAS_LBT")]
pub unsafe fn own_lbt(restore: i32) {
    preempt_disable();
    own_lbt_inatomic(restore);
    preempt_enable();
}

#[cfg(feature = "CONFIG_CPU_HAS_LBT")]
pub unsafe fn lose_lbt_inatomic(save: i32, tsk: *mut task_struct) {
    if cpu_has_lbt && is_lbt_owner() != 0 {
        if save != 0 {
            _save_lbt(core::ptr::null_mut()); // tsk->thread.lbt
        }
        disable_lbt();
        clear_tsk_thread_flag(tsk, TIF_USEDLBT);
    }
    // C macro lvalue: KSTK_EUEN(tsk) &= ~(CSR_EUEN_LBTEN);
}

#[cfg(feature = "CONFIG_CPU_HAS_LBT")]
pub unsafe fn lose_lbt(save: i32) {
    preempt_disable();
    lose_lbt_inatomic(save, current);
    preempt_enable();
}

#[cfg(feature = "CONFIG_CPU_HAS_LBT")]
pub unsafe fn init_lbt() {
    __own_lbt();
    _init_lbt();
}

#[cfg(not(feature = "CONFIG_CPU_HAS_LBT"))]
pub unsafe fn own_lbt_inatomic(_restore: i32) {}
#[cfg(not(feature = "CONFIG_CPU_HAS_LBT"))]
pub unsafe fn lose_lbt_inatomic(_save: i32, _tsk: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_CPU_HAS_LBT"))]
pub unsafe fn init_lbt() {}
#[cfg(not(feature = "CONFIG_CPU_HAS_LBT"))]
pub unsafe fn lose_lbt(_save: i32) {}

pub unsafe fn thread_lbt_context_live() -> i32 {
    if !cpu_has_lbt {
        return 0;
    }

    test_thread_flag(TIF_LBT_CTX_LIVE)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
