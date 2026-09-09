// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn perf_reg_value(regs: *mut pt_regs, mut idx: i32) -> u64 {
    let mut fp: freg_t;

    if idx >= PERF_REG_S390_R0 && idx <= PERF_REG_S390_R15 {
        return (*regs).gprs[idx as usize];
    }

    if idx >= PERF_REG_S390_FP0 && idx <= PERF_REG_S390_FP15 {
        if !user_mode(regs) {
            return 0;
        }

        idx -= PERF_REG_S390_FP0;
        fp = *(((*current).thread.ufpu.vxrs.as_ptr().add(idx as usize)) as *const freg_t);
        return fp.ui;
    }

    if idx == PERF_REG_S390_MASK {
        return (*regs).psw.mask;
    }
    if idx == PERF_REG_S390_PC {
        return (*regs).psw.addr;
    }

    WARN_ON_ONCE((idx as u32) >= PERF_REG_S390_MAX);
    0
}

pub const REG_RESERVED: usize = !((1usize << PERF_REG_S390_MAX) - 1);

pub fn perf_reg_validate(mask: u64) -> i32 {
    if mask == 0 || (mask & REG_RESERVED as u64) != 0 {
        return -EINVAL;
    }

    0
}

pub unsafe fn perf_reg_abi(_task: *mut task_struct) -> u64 {
    PERF_SAMPLE_REGS_ABI_64
}

pub unsafe fn perf_get_regs_user(regs_user: *mut perf_regs, regs: *mut pt_regs) {
    /*
     * Use the regs from the first interruption and let
     * perf_sample_regs_intr() handle interrupts (regs == get_irq_regs()).
     *
     * Also save FPU registers for user-space tasks only.
     */
    (*regs_user).regs = task_pt_regs(current);
    if user_mode((*regs_user).regs) {
        save_user_fpu_regs();
    }
    (*regs_user).abi = perf_reg_abi(current);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
