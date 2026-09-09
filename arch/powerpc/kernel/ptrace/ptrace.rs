// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  PowerPC version
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Derived from "arch/m68k/kernel/ptrace.c"
 *  Copyright (C) 1994 by Hamish Macdonald
 *  Taken from linux/kernel/ptrace.c and modified for M680x0.
 *  linux/kernel/ptrace.c is by Ross Biro 1/23/92, edited by Linus Torvalds
 *
 * Modified by Cort Dougan (cort@hq.fsmlabs.com)
 * and Paul Mackerras (paulus@samba.org).
 */

// C dependencies: linux/regset.h, linux/ptrace.h, linux/audit.h,
// linux/context_tracking.h, linux/syscalls.h, asm/switch_to.h, asm/debug.h,
// and ptrace-decl.h.

pub unsafe fn ptrace_disable(child: *mut task_struct) {
    // make sure the single step bit is not set.
    user_disable_single_step(child);
}

pub unsafe fn arch_ptrace(
    child: *mut task_struct,
    request: c_long,
    addr: c_ulong,
    data: c_ulong,
) -> c_long {
    let mut ret: c_long = -EPERM;
    let datavp = data as *mut c_void;
    let datalp = datavp as *mut c_ulong;

    match request {
        PTRACE_PEEKUSR => {
            let mut index: c_ulong;
            let mut tmp: c_ulong = 0;

            ret = -EIO;
            index = addr / core::mem::size_of::<c_long>() as c_ulong;
            if (addr & (core::mem::size_of::<c_long>() as c_ulong - 1)) != 0
                || (*child).thread.regs.is_null()
            {
                return ret;
            }

            if index < PT_FPR0 {
                ret = ptrace_get_reg(child, index as c_int, &mut tmp);
            } else {
                ret = ptrace_get_fpr(child, index, &mut tmp);
            }
            if ret != 0 {
                return ret;
            }
            ret = put_user(tmp, datalp);
        }
        PTRACE_POKEUSR => {
            let index: c_ulong;

            ret = -EIO;
            index = addr / core::mem::size_of::<c_long>() as c_ulong;
            if (addr & (core::mem::size_of::<c_long>() as c_ulong - 1)) != 0
                || (*child).thread.regs.is_null()
            {
                return ret;
            }
            if index < PT_FPR0 {
                ret = ptrace_put_reg(child, index, data);
            } else {
                ret = ptrace_put_fpr(child, index, data);
            }
        }
        PPC_PTRACE_GETHWDBGINFO => {
            let mut dbginfo: ppc_debug_info = core::mem::zeroed();
            ppc_gethwdinfo(&mut dbginfo);
            if copy_to_user(
                datavp,
                &dbginfo as *const _ as *const c_void,
                core::mem::size_of::<ppc_debug_info>(),
            ) != 0 {
                return -EFAULT;
            }
            return 0;
        }
        PPC_PTRACE_SETHWDEBUG => {
            let mut bp_info: ppc_hw_breakpoint = core::mem::zeroed();
            if copy_from_user(
                &mut bp_info as *mut _ as *mut c_void,
                datavp,
                core::mem::size_of::<ppc_hw_breakpoint>(),
            ) != 0 {
                return -EFAULT;
            }
            return ppc_set_hwdebug(child, &mut bp_info);
        }
        PPC_PTRACE_DELHWDEBUG => ret = ppc_del_hwdebug(child, data),
        PTRACE_GET_DEBUGREG => ret = ptrace_get_debugreg(child, addr, datalp),
        PTRACE_SET_DEBUGREG => ret = ptrace_set_debugreg(child, addr, data),
        // CONFIG_PPC64: PTRACE_GETREGS64 aliases this arm.
        PTRACE_GETREGS => return copy_regset_to_user(child, &user_ppc_native_view, REGSET_GPR, 0, core::mem::size_of::<user_pt_regs>(), datavp),
        // CONFIG_PPC64: PTRACE_SETREGS64 aliases this arm.
        PTRACE_SETREGS => return copy_regset_from_user(child, &user_ppc_native_view, REGSET_GPR, 0, core::mem::size_of::<user_pt_regs>(), datavp),
        PTRACE_GETFPREGS => return copy_regset_to_user(child, &user_ppc_native_view, REGSET_FPR, 0, core::mem::size_of::<elf_fpregset_t>(), datavp),
        PTRACE_SETFPREGS => return copy_regset_from_user(child, &user_ppc_native_view, REGSET_FPR, 0, core::mem::size_of::<elf_fpregset_t>(), datavp),
        // CONFIG_ALTIVEC
        #[cfg(CONFIG_ALTIVEC)]
        PTRACE_GETVRREGS => return copy_regset_to_user(child, &user_ppc_native_view, REGSET_VMX, 0, 33 * core::mem::size_of::<vector128>() + core::mem::size_of::<u32>(), datavp),
        #[cfg(CONFIG_ALTIVEC)]
        PTRACE_SETVRREGS => return copy_regset_from_user(child, &user_ppc_native_view, REGSET_VMX, 0, 33 * core::mem::size_of::<vector128>() + core::mem::size_of::<u32>(), datavp),
        // CONFIG_VSX
        #[cfg(CONFIG_VSX)]
        PTRACE_GETVSRREGS => return copy_regset_to_user(child, &user_ppc_native_view, REGSET_VSX, 0, 32 * core::mem::size_of::<f64>(), datavp),
        #[cfg(CONFIG_VSX)]
        PTRACE_SETVSRREGS => return copy_regset_from_user(child, &user_ppc_native_view, REGSET_VSX, 0, 32 * core::mem::size_of::<f64>(), datavp),
        // CONFIG_SPE
        #[cfg(CONFIG_SPE)]
        PTRACE_GETEVRREGS => return copy_regset_to_user(child, &user_ppc_native_view, REGSET_SPE, 0, 35 * core::mem::size_of::<u32>(), datavp),
        #[cfg(CONFIG_SPE)]
        PTRACE_SETEVRREGS => return copy_regset_from_user(child, &user_ppc_native_view, REGSET_SPE, 0, 35 * core::mem::size_of::<u32>(), datavp),
        _ => ret = ptrace_request(child, request, addr, data),
    }
    ret
}

pub unsafe fn pt_regs_check() {
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, gpr) != core::mem::offset_of!(user_pt_regs, gpr));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, nip) != core::mem::offset_of!(user_pt_regs, nip));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, msr) != core::mem::offset_of!(user_pt_regs, msr));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, orig_gpr3) != core::mem::offset_of!(user_pt_regs, orig_gpr3));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, ctr) != core::mem::offset_of!(user_pt_regs, ctr));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, link) != core::mem::offset_of!(user_pt_regs, link));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, xer) != core::mem::offset_of!(user_pt_regs, xer));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, ccr) != core::mem::offset_of!(user_pt_regs, ccr));
    // __powerpc64__: check softe; otherwise check mq.
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, trap) != core::mem::offset_of!(user_pt_regs, trap));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, dar) != core::mem::offset_of!(user_pt_regs, dar));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, dear) != core::mem::offset_of!(user_pt_regs, dar));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, dsisr) != core::mem::offset_of!(user_pt_regs, dsisr));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, esr) != core::mem::offset_of!(user_pt_regs, dsisr));
    BUILD_BUG_ON!(core::mem::offset_of!(pt_regs, result) != core::mem::offset_of!(user_pt_regs, result));
    BUILD_BUG_ON!(core::mem::size_of::<user_pt_regs>() > core::mem::size_of::<pt_regs>());
    macro_rules! check_reg { ($pt:expr, $reg:ident) => { BUILD_BUG_ON!($pt != core::mem::offset_of!(user_pt_regs, $reg) / core::mem::size_of::<c_ulong>()); }; }
    check_reg!(PT_R0, gpr[0]); check_reg!(PT_R1, gpr[1]); check_reg!(PT_R2, gpr[2]); check_reg!(PT_R3, gpr[3]);
    check_reg!(PT_R4, gpr[4]); check_reg!(PT_R5, gpr[5]); check_reg!(PT_R6, gpr[6]); check_reg!(PT_R7, gpr[7]);
    check_reg!(PT_R8, gpr[8]); check_reg!(PT_R9, gpr[9]); check_reg!(PT_R10, gpr[10]); check_reg!(PT_R11, gpr[11]);
    check_reg!(PT_R12, gpr[12]); check_reg!(PT_R13, gpr[13]); check_reg!(PT_R14, gpr[14]); check_reg!(PT_R15, gpr[15]);
    check_reg!(PT_R16, gpr[16]); check_reg!(PT_R17, gpr[17]); check_reg!(PT_R18, gpr[18]); check_reg!(PT_R19, gpr[19]);
    check_reg!(PT_R20, gpr[20]); check_reg!(PT_R21, gpr[21]); check_reg!(PT_R22, gpr[22]); check_reg!(PT_R23, gpr[23]);
    check_reg!(PT_R24, gpr[24]); check_reg!(PT_R25, gpr[25]); check_reg!(PT_R26, gpr[26]); check_reg!(PT_R27, gpr[27]);
    check_reg!(PT_R28, gpr[28]); check_reg!(PT_R29, gpr[29]); check_reg!(PT_R30, gpr[30]); check_reg!(PT_R31, gpr[31]);
    check_reg!(PT_NIP, nip); check_reg!(PT_MSR, msr); check_reg!(PT_ORIG_R3, orig_gpr3); check_reg!(PT_CTR, ctr);
    check_reg!(PT_LNK, link); check_reg!(PT_XER, xer); check_reg!(PT_CCR, ccr);
    // CONFIG_PPC64: check PT_SOFTE, otherwise check PT_MQ.
    check_reg!(PT_TRAP, trap); check_reg!(PT_DAR, dar); check_reg!(PT_DSISR, dsisr); check_reg!(PT_RESULT, result);
    BUILD_BUG_ON!(PT_REGS_COUNT != core::mem::size_of::<user_pt_regs>() / core::mem::size_of::<c_ulong>());
    BUILD_BUG_ON!(PT_DSCR < core::mem::size_of::<user_pt_regs>() / core::mem::size_of::<c_ulong>());
    BUILD_BUG_ON!(IS_ENABLED!(CONFIG_PPC32) && IS_ENABLED!(CONFIG_VSX));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
