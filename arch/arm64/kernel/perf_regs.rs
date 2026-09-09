// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the corresponding Linux kernel headers.

unsafe fn perf_ext_regs_value(idx: i32) -> u64 {
    match idx {
        PERF_REG_ARM64_VG => {
            if WARN_ON_ONCE(!system_supports_sve()) {
                return 0;
            }

            /*
             * Vector granule is current length in bits of SVE registers
             * divided by 64.
             */
            return (task_get_sve_vl(current) * 8) / 64;
        }
        _ => {
            WARN_ON_ONCE(true);
            0
        }
    }
}

pub unsafe fn perf_reg_value(regs: *mut pt_regs, idx: i32) -> u64 {
    if WARN_ON_ONCE((idx as u32) >= PERF_REG_ARM64_EXTENDED_MAX) {
        return 0;
    }

    /*
     * Our handling of compat tasks (PERF_SAMPLE_REGS_ABI_32) is weird, but
     * we're stuck with it for ABI compatibility reasons.
     *
     * For a 32-bit consumer inspecting a 32-bit task, then it will look at
     * the first 16 registers (see arch/arm/include/uapi/asm/perf_regs.h).
     * These correspond directly to a prefix of the registers saved in our
     * 'struct pt_regs', with the exception of the PC, so we copy that down
     * (x15 corresponds to SP_hyp in the architecture).
     *
     * So far, so good.
     *
     * The oddity arises when a 64-bit consumer looks at a 32-bit task and
     * asks for registers beyond PERF_REG_ARM_MAX. In this case, we return
     * SP_usr, LR_usr and PC in the positions where the AArch64 SP, LR and
     * PC registers would normally live. The initial idea was to allow a
     * 64-bit unwinder to unwind a 32-bit task and, although it's not clear
     * how well that works in practice, somebody might be relying on it.
     *
     * At the time we make a sample, we don't know whether the consumer is
     * 32-bit or 64-bit, so we have to cater for both possibilities.
     */
    if compat_user_mode(regs) {
        if (idx as u32) == PERF_REG_ARM64_SP {
            return (*regs).compat_sp;
        }
        if (idx as u32) == PERF_REG_ARM64_LR {
            return (*regs).compat_lr;
        }
        if idx == 15 {
            return (*regs).pc;
        }
    }

    if (idx as u32) == PERF_REG_ARM64_SP {
        return (*regs).sp;
    }

    if (idx as u32) == PERF_REG_ARM64_PC {
        return (*regs).pc;
    }

    if (idx as u32) >= PERF_REG_ARM64_MAX {
        return perf_ext_regs_value(idx);
    }

    (*regs).regs[idx as usize]
}

const REG_RESERVED: u64 = !((1u64 << PERF_REG_ARM64_MAX) - 1);

pub fn perf_reg_validate(mask: u64) -> i32 {
    let mut reserved_mask = REG_RESERVED;

    if system_supports_sve() {
        reserved_mask &= !(1u64 << PERF_REG_ARM64_VG);
    }

    if mask == 0 || (mask & reserved_mask) != 0 {
        return -EINVAL;
    }

    0
}

pub unsafe fn perf_reg_abi(task: *mut task_struct) -> u64 {
    if is_compat_thread(task_thread_info(task)) {
        PERF_SAMPLE_REGS_ABI_32
    } else {
        PERF_SAMPLE_REGS_ABI_64
    }
}

pub unsafe fn perf_get_regs_user(regs_user: *mut perf_regs, regs: *mut pt_regs) {
    (*regs_user).regs = task_pt_regs(current);
    (*regs_user).abi = perf_reg_abi(current);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
