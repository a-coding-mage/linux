// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation.

static mut euen_mask: u32 = CSR_EUEN_FPEN;

/*
 * The critical section between kernel_fpu_begin() and kernel_fpu_end()
 * is non-reentrant. It is the caller's responsibility to avoid reentrance.
 * See drivers/gpu/drm/amd/display/amdgpu_dm/dc_fpu.c as an example.
 */
static mut in_kernel_fpu: bool = false;
static mut euen_current: u32 = 0;

#[inline]
unsafe fn fpregs_lock() {
    // The CONFIG_PREEMPT_RT condition is supplied by the build configuration.
    if IS_ENABLED(CONFIG_PREEMPT_RT) {
        preempt_disable();
    } else {
        local_bh_disable();
    }
}

#[inline]
unsafe fn fpregs_unlock() {
    // The CONFIG_PREEMPT_RT condition is supplied by the build configuration.
    if IS_ENABLED(CONFIG_PREEMPT_RT) {
        preempt_enable();
    } else {
        local_bh_enable();
    }
}

pub unsafe fn kernel_fpu_begin() {
    let euen_curr: *mut u32;

    if !irqs_disabled() {
        fpregs_lock();
    }

    WARN_ON(in_kernel_fpu);

    in_kernel_fpu = true;
    euen_curr = &mut euen_current as *mut u32;

    *euen_curr = csr_xchg32(euen_mask, euen_mask, LOONGARCH_CSR_EUEN);

    // CONFIG_CPU_HAS_LASX is a build-time condition from the original source.
    #[cfg(CONFIG_CPU_HAS_LASX)]
    if (*euen_curr & CSR_EUEN_LASXEN) != 0 {
        _save_lasx(&mut (*current).thread.fpu);
    } else
    // CONFIG_CPU_HAS_LSX is a build-time condition from the original source.
    #[cfg(CONFIG_CPU_HAS_LSX)]
    if (*euen_curr & CSR_EUEN_LSXEN) != 0 {
        _save_lsx(&mut (*current).thread.fpu);
    } else
    if (*euen_curr & CSR_EUEN_FPEN) != 0 {
        _save_fp(&mut (*current).thread.fpu);
    }

    write_fcsr(LOONGARCH_FCSR0, 0);
}

pub unsafe fn kernel_fpu_end() {
    let euen_curr: *mut u32;

    WARN_ON(!in_kernel_fpu);

    euen_curr = &mut euen_current as *mut u32;

    // CONFIG_CPU_HAS_LASX is a build-time condition from the original source.
    #[cfg(CONFIG_CPU_HAS_LASX)]
    if (*euen_curr & CSR_EUEN_LASXEN) != 0 {
        _restore_lasx(&mut (*current).thread.fpu);
    } else
    // CONFIG_CPU_HAS_LSX is a build-time condition from the original source.
    #[cfg(CONFIG_CPU_HAS_LSX)]
    if (*euen_curr & CSR_EUEN_LSXEN) != 0 {
        _restore_lsx(&mut (*current).thread.fpu);
    } else
    if (*euen_curr & CSR_EUEN_FPEN) != 0 {
        _restore_fp(&mut (*current).thread.fpu);
    }

    *euen_curr = csr_xchg32(*euen_curr, euen_mask, LOONGARCH_CSR_EUEN);

    in_kernel_fpu = false;

    if !irqs_disabled() {
        fpregs_unlock();
    }
}

unsafe fn init_euen_mask() -> i32 {
    if cpu_has_lsx {
        euen_mask |= CSR_EUEN_LSXEN;
    }

    if cpu_has_lasx {
        euen_mask |= CSR_EUEN_LASXEN;
    }

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
