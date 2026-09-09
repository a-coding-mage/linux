/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2002 MontaVista Software Inc.
 * Author: Jun Sun, jsun@mvista.com or jsun@junsun.net
 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * This enum specifies a mode in which we want the FPU to operate, for cores
 * which implement the Status.FR bit. Note that the bottom bit of the value
 * purposefully matches the desired value of the Status.FR bit.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fpu_mode {
    FPU_32BIT = 0, /* FR = 0 */
    FPU_64BIT,     /* FR = 1, FRE = 0 */
    FPU_AS_IS,
    FPU_HYBRID,    /* FR = 1, FRE = 1 */
}

pub const FPU_FR_MASK: i32 = 0x1;

/* CONFIG_MIPS_FP_SUPPORT */
extern "C" {
    pub fn _save_fp(task: *mut task_struct);
    pub fn _restore_fp(task: *mut task_struct);
}

#[inline(always)]
pub unsafe fn __disable_fpu() {
    clear_c0_status(ST0_CU1);
    disable_fpu_hazard();
}

#[inline]
pub unsafe fn __enable_fpu(mode: fpu_mode) -> i32 {
    let fr: i32;

    match mode {
        fpu_mode::FPU_AS_IS => {
            /* just enable the FPU in its current mode */
            set_c0_status(ST0_CU1);
            enable_fpu_hazard();
            return 0;
        }
        fpu_mode::FPU_HYBRID => {
            if !cpu_has_fre {
                return SIGFPE;
            }

            /* set FRE */
            set_c0_config5(MIPS_CONF5_FRE);
            return fr_common(mode);
        }
        fpu_mode::FPU_64BIT => {
            /* On configurations without a 64-bit FPU this returns SIGFPE. */
            set_c0_config5(MIPS_CONF5_FRE);
            return fr_common(mode);
        }
        fpu_mode::FPU_32BIT => {
            if cpu_has_fre {
                /* clear FRE */
                clear_c0_config5(MIPS_CONF5_FRE);
            }
            return fr_common(mode);
        }
    }

    #[inline(always)]
    unsafe fn fr_common(mode: fpu_mode) -> i32 {
        let fr = (mode as i32) & FPU_FR_MASK;
        /* set CU1 & change FR appropriately */
        change_c0_status(ST0_CU1 | ST0_FR, ST0_CU1 | if fr != 0 { ST0_FR } else { 0 });
        enable_fpu_hazard();

        /* check FR has the desired value */
        if ((read_c0_status() & ST0_FR) != 0) == (fr != 0) {
            return 0;
        }

        /* unsupported FR value */
        __disable_fpu();
        SIGFPE
    }
}

#[inline(always)]
pub unsafe fn clear_fpu_owner() {
    clear_thread_flag(TIF_USEDFPU);
}

#[inline]
pub unsafe fn __is_fpu_owner() -> bool {
    test_thread_flag(TIF_USEDFPU)
}

#[inline]
pub unsafe fn is_fpu_owner() -> bool {
    cpu_has_fpu && __is_fpu_owner()
}

#[inline]
pub unsafe fn __own_fpu() -> i32 {
    let mode: fpu_mode;
    let mut ret: i32;

    if test_thread_flag(TIF_HYBRID_FPREGS) {
        mode = fpu_mode::FPU_HYBRID;
    } else {
        mode = if !test_thread_flag(TIF_32BIT_FPREGS) {
            fpu_mode::FPU_64BIT
        } else {
            fpu_mode::FPU_32BIT
        };
    }

    ret = __enable_fpu(mode);
    if ret != 0 {
        return ret;
    }

    if current->thread.fpu.fcr31 & FPU_CSR_NAN2008 != 0 {
        if !cpu_has_nan_2008 {
            ret = SIGFPE;
            return failed_fpu(ret);
        }
    } else if !cpu_has_nan_legacy {
        ret = SIGFPE;
        return failed_fpu(ret);
    }

    KSTK_STATUS(current) |= ST0_CU1;
    if mode == fpu_mode::FPU_64BIT || mode == fpu_mode::FPU_HYBRID {
        KSTK_STATUS(current) |= ST0_FR;
    } else {
        KSTK_STATUS(current) &= !ST0_FR;
    }

    set_thread_flag(TIF_USEDFPU);
    0
}

#[inline(always)]
unsafe fn failed_fpu(ret: i32) -> i32 {
    __disable_fpu();
    ret
}

#[inline]
pub unsafe fn own_fpu_inatomic(restore: i32) -> i32 {
    let mut ret = 0;
    if cpu_has_fpu && !__is_fpu_owner() {
        ret = __own_fpu();
        if restore != 0 && ret == 0 {
            _restore_fp(current);
        }
    }
    ret
}

#[inline]
pub unsafe fn own_fpu(restore: i32) -> i32 {
    preempt_disable();
    let ret = own_fpu_inatomic(restore);
    preempt_enable();
    ret
}

#[inline]
pub unsafe fn lose_fpu_inatomic(save: i32, tsk: *mut task_struct) {
    if is_msa_enabled() {
        if save != 0 {
            save_msa(tsk);
            (*tsk).thread.fpu.fcr31 = read_32bit_cp1_register(CP1_STATUS);
        }
        disable_msa();
        clear_tsk_thread_flag(tsk, TIF_USEDMSA);
        __disable_fpu();
    } else if is_fpu_owner() {
        if save != 0 {
            _save_fp(tsk);
        }
        __disable_fpu();
    } else {
        /* FPU should not have been left enabled with no owner */
        WARN(read_c0_status() & ST0_CU1, "Orphaned FPU left enabled");
    }
    KSTK_STATUS(tsk) &= !ST0_CU1;
    clear_tsk_thread_flag(tsk, TIF_USEDFPU);
}

#[inline]
pub unsafe fn lose_fpu(save: i32) {
    preempt_disable();
    lose_fpu_inatomic(save, current);
    preempt_enable();
}

/**
 * init_fp_ctx() - Initialize task FP context
 * @target: The task whose FP context should be initialized.
 *
 * Initializes the FP context of the target task to sane default values if that
 * target task does not already have valid FP context. Once the context has
 * been initialized, the task will be marked as having used FP & thus having
 * valid FP context.
 *
 * Returns: true if context is initialized, else false.
 */
#[inline]
pub unsafe fn init_fp_ctx(target: *mut task_struct) -> bool {
    /* If FP has been used then the target already has context */
    if tsk_used_math(target) {
        return false;
    }

    /* Begin with data registers set to all 1s... */
    memset(
        &mut (*target).thread.fpu.fpr as *mut _ as *mut core::ffi::c_void,
        !0,
        core::mem::size_of_val(&(*target).thread.fpu.fpr),
    );

    /* FCSR has been preset by `mips_set_personality_nan'.  */

    /*
     * Record that the target has "used" math, such that the context
     * just initialised, and any modifications made by the caller,
     * aren't discarded.
     */
    set_stopped_child_used_math(target);

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
