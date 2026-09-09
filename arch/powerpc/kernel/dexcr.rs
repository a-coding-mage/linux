// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the surrounding kernel translation unit.

static mut DEXCR_PRCTL_EDITABLE: u32 =
    DEXCR_PR_IBRTPD | DEXCR_PR_SRAPD | DEXCR_PR_NPHIE;

unsafe fn init_task_dexcr() -> i32 {
    if !early_cpu_has_feature(CPU_FTR_ARCH_31) {
        return 0;
    }

    (*current).thread.dexcr_onexec = mfspr(SPRN_DEXCR);

    0
}

// early_initcall(init_task_dexcr)

unsafe fn prctl_to_aspect(which: c_ulong, aspect: *mut c_uint) -> i32 {
    match which {
        PR_PPC_DEXCR_SBHE => {
            *aspect = DEXCR_PR_SBHE;
        }
        PR_PPC_DEXCR_IBRTPD => {
            *aspect = DEXCR_PR_IBRTPD;
        }
        PR_PPC_DEXCR_SRAPD => {
            *aspect = DEXCR_PR_SRAPD;
        }
        PR_PPC_DEXCR_NPHIE => {
            *aspect = DEXCR_PR_NPHIE;
        }
        _ => return -ENODEV,
    }

    0
}

pub unsafe fn get_dexcr_prctl(task: *mut task_struct, which: c_ulong) -> i32 {
    let mut aspect: c_uint = 0;
    let mut ret: i32;

    ret = prctl_to_aspect(which, &mut aspect);
    if ret != 0 {
        return ret;
    }

    if aspect & DEXCR_PRCTL_EDITABLE != 0 {
        ret |= PR_PPC_DEXCR_CTRL_EDITABLE;
    }

    if aspect & mfspr(SPRN_DEXCR) != 0 {
        ret |= PR_PPC_DEXCR_CTRL_SET;
    } else {
        ret |= PR_PPC_DEXCR_CTRL_CLEAR;
    }

    if aspect & (*task).thread.dexcr_onexec != 0 {
        ret |= PR_PPC_DEXCR_CTRL_SET_ONEXEC;
    } else {
        ret |= PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC;
    }

    ret
}

pub unsafe fn set_dexcr_prctl(
    task: *mut task_struct,
    which: c_ulong,
    ctrl: c_ulong,
) -> i32 {
    let mut dexcr: c_ulong;
    let mut aspect: c_uint = 0;
    let mut err: i32 = 0;

    err = prctl_to_aspect(which, &mut aspect);
    if err != 0 {
        return err;
    }

    if aspect & DEXCR_PRCTL_EDITABLE == 0 {
        return -EPERM;
    }

    if ctrl & !PR_PPC_DEXCR_CTRL_MASK != 0 {
        return -EINVAL;
    }

    if ctrl & PR_PPC_DEXCR_CTRL_SET != 0 && ctrl & PR_PPC_DEXCR_CTRL_CLEAR != 0 {
        return -EINVAL;
    }

    if ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC != 0
        && ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC != 0
    {
        return -EINVAL;
    }

    /*
     * We do not want an unprivileged process being able to disable
     * a setuid process's hash check instructions
     */
    if aspect == DEXCR_PR_NPHIE
        && ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC != 0
        && !capable(CAP_SYS_ADMIN)
    {
        return -EPERM;
    }

    dexcr = mfspr(SPRN_DEXCR);

    if ctrl & PR_PPC_DEXCR_CTRL_SET != 0 {
        dexcr |= aspect as c_ulong;
    } else if ctrl & PR_PPC_DEXCR_CTRL_CLEAR != 0 {
        dexcr &= !(aspect as c_ulong);
    }

    if ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC != 0 {
        (*task).thread.dexcr_onexec |= aspect;
    } else if ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC != 0 {
        (*task).thread.dexcr_onexec &= !aspect;
    }

    mtspr(SPRN_DEXCR, dexcr);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
