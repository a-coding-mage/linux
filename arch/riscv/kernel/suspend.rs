// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 * Copyright (c) 2022 Ventana Micro Systems Inc.
 */

// #define pr_fmt(fmt) "suspend: " fmt
// C header dependencies are supplied by the surrounding kernel translation.

pub unsafe fn suspend_save_csrs(context: *mut suspend_context) {
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_XLINUXENVCFG) {
        (*context).envcfg = csr_read(CSR_ENVCFG);
    }
    (*context).tvec = csr_read(CSR_TVEC);
    (*context).ie = csr_read(CSR_IE);

    /*
     * No need to save/restore IP CSR (i.e. MIP or SIP) because:
     *
     * 1. For no-MMU (M-mode) kernel, the bits in MIP are set by
     *    external devices (such as interrupt controller, timer, etc).
     * 2. For MMU (S-mode) kernel, the bits in SIP are set by
     *    M-mode firmware and external devices (such as interrupt
     *    controller, etc).
     */

    #[cfg(CONFIG_MMU)]
    {
        if riscv_has_extension_unlikely(RISCV_ISA_EXT_SSTC) {
            (*context).stimecmp = csr_read(CSR_STIMECMP);
            #[cfg(target_pointer_width = "32")]
            {
                (*context).stimecmph = csr_read(CSR_STIMECMPH);
            }
        }

        (*context).satp = csr_read(CSR_SATP);
    }
}

pub unsafe fn suspend_restore_csrs(context: *mut suspend_context) {
    csr_write(CSR_SCRATCH, 0);
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_XLINUXENVCFG) {
        csr_write(CSR_ENVCFG, (*context).envcfg);
    }
    csr_write(CSR_TVEC, (*context).tvec);
    csr_write(CSR_IE, (*context).ie);

    #[cfg(CONFIG_MMU)]
    {
        if riscv_has_extension_unlikely(RISCV_ISA_EXT_SSTC) {
            #[cfg(target_pointer_width = "32")]
            {
                csr_write(CSR_STIMECMP, ULONG_MAX);
                csr_write(CSR_STIMECMPH, (*context).stimecmph);
            }
            csr_write(CSR_STIMECMP, (*context).stimecmp);
        }

        csr_write(CSR_SATP, (*context).satp);
    }
}

pub unsafe fn cpu_suspend(
    arg: c_ulong,
    finish: Option<unsafe extern "C" fn(c_ulong, c_ulong, c_ulong) -> c_int>,
) -> c_int {
    let mut rc: c_int = 0;
    let mut context: suspend_context = core::mem::zeroed();

    /* Finisher should be non-NULL */
    let finish = match finish {
        Some(finish) => finish,
        None => return -EINVAL,
    };

    /* Save additional CSRs*/
    suspend_save_csrs(&mut context);

    /*
     * Function graph tracer state gets inconsistent when the kernel
     * calls functions that never return (aka finishers) hence disable
     * graph tracing during their execution.
     */
    pause_graph_tracing();

    /* Save context on stack */
    if __cpu_suspend_enter(&mut context) {
        /* Call the finisher */
        rc = finish(arg, __pa_symbol(__cpu_resume_enter), &context as *const _ as c_ulong);

        /*
         * Should never reach here, unless the suspend finisher
         * fails. Successful cpu_suspend() should return from
         * __cpu_resume_entry()
         */
        if rc == 0 {
            rc = -EOPNOTSUPP;
        }
    }

    /* Enable function graph tracer */
    unpause_graph_tracing();

    /* Restore additional CSRs */
    suspend_restore_csrs(&mut context);

    rc
}

#[cfg(CONFIG_RISCV_SBI)]
unsafe fn sbi_system_suspend(
    sleep_type: c_ulong,
    resume_addr: c_ulong,
    opaque: c_ulong,
) -> c_int {
    let ret: sbiret = sbi_ecall(
        SBI_EXT_SUSP,
        SBI_EXT_SUSP_SYSTEM_SUSPEND,
        sleep_type,
        resume_addr,
        opaque,
        0,
        0,
        0,
    );
    if ret.error != 0 {
        return sbi_err_map_linux_errno(ret.error);
    }
    ret.value
}

#[cfg(CONFIG_RISCV_SBI)]
unsafe fn sbi_system_suspend_enter(state: suspend_state_t) -> c_int {
    cpu_suspend(SBI_SUSP_SLEEP_TYPE_SUSPEND_TO_RAM, Some(sbi_system_suspend))
}

#[cfg(CONFIG_RISCV_SBI)]
static sbi_system_suspend_ops: platform_suspend_ops = platform_suspend_ops {
    valid: Some(suspend_valid_only_mem),
    enter: Some(sbi_system_suspend_enter),
};

#[cfg(CONFIG_RISCV_SBI)]
unsafe extern "C" fn sbi_system_suspend_init() -> c_int {
    if sbi_spec_version >= sbi_mk_version(2, 0) && sbi_probe_extension(SBI_EXT_SUSP) > 0 {
        pr_info!("SBI SUSP extension detected\n");
        if IS_ENABLED(CONFIG_SUSPEND) {
            suspend_set_ops(&sbi_system_suspend_ops);
        }
    }
    0
}

// arch_initcall(sbi_system_suspend_init);

#[cfg(CONFIG_RISCV_SBI)]
unsafe fn sbi_suspend_finisher(
    suspend_type: c_ulong,
    resume_addr: c_ulong,
    opaque: c_ulong,
) -> c_int {
    let ret: sbiret = sbi_ecall(
        SBI_EXT_HSM,
        SBI_EXT_HSM_HART_SUSPEND,
        suspend_type,
        resume_addr,
        opaque,
        0,
        0,
        0,
    );
    if ret.error != 0 {
        sbi_err_map_linux_errno(ret.error)
    } else {
        0
    }
}

#[cfg(CONFIG_RISCV_SBI)]
pub unsafe fn riscv_sbi_hart_suspend(state: u32) -> c_int {
    if state & SBI_HSM_SUSP_NON_RET_BIT != 0 {
        cpu_suspend(state as c_ulong, Some(sbi_suspend_finisher))
    } else {
        sbi_suspend_finisher(state as c_ulong, 0, 0)
    }
}

#[cfg(CONFIG_RISCV_SBI)]
pub fn riscv_sbi_suspend_state_is_valid(state: u32) -> bool {
    if state > SBI_HSM_SUSPEND_RET_DEFAULT && state < SBI_HSM_SUSPEND_RET_PLATFORM {
        return false;
    }
    if state > SBI_HSM_SUSPEND_NON_RET_DEFAULT && state < SBI_HSM_SUSPEND_NON_RET_PLATFORM {
        return false;
    }
    true
}

#[cfg(CONFIG_RISCV_SBI)]
pub unsafe fn riscv_sbi_hsm_is_supported() -> bool {
    /*
     * The SBI HSM suspend function is only available when:
     * 1) SBI version is 0.3 or higher
     * 2) SBI HSM extension is available
     */
    if sbi_spec_version < sbi_mk_version(0, 3) || sbi_probe_extension(SBI_EXT_HSM) == 0 {
        pr_info!("HSM suspend not available\n");
        return false;
    }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
