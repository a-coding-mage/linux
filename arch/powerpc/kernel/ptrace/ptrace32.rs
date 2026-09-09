/*
 * ptrace for 32-bit processes running on a 64-bit kernel.
 *
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
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file COPYING in the main directory of this
 * archive for more details.
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * does not yet catch signals sent when the child dies.
 * in exit.c or in signal.c.
 */

/* Macros to workout the correct index for the FPR in the thread struct */
#[inline]
fn fprnumber(i: usize) -> usize { (i.wrapping_sub(PT_FPR0)) >> 1 }
#[inline]
fn fprhalf(i: usize) -> usize { (i.wrapping_sub(PT_FPR0)) & 1 }
#[inline]
fn fprindex(i: usize) -> usize { TS_FPRWIDTH * fprnumber(i) * 2 + fprhalf(i) }

pub unsafe fn compat_arch_ptrace(
    child: *mut task_struct,
    request: compat_long_t,
    caddr: compat_ulong_t,
    cdata: compat_ulong_t,
) -> c_long {
    let addr = caddr as usize;
    let data = cdata as usize;
    let mut ret: c_long;

    match request {
        PPC_PTRACE_PEEKTEXT_3264 | PPC_PTRACE_PEEKDATA_3264 => {
            let mut tmp: u32 = 0;
            let mut addr_others: *mut u32 = core::ptr::null_mut();
            ret = -EIO;
            if get_user(&mut addr_others, addr as *const *const u32) != 0 { return ret; }
            let copied = ptrace_access_vm(child, addr_others as u64, &mut tmp, core::mem::size_of::<u32>(), FOLL_FORCE);
            if copied != core::mem::size_of::<u32>() { return ret; }
            ret = put_user(tmp, data as *mut u32);
        }
        PTRACE_PEEKUSR => {
            ret = -EIO;
            let index = addr >> 2;
            if (addr & 3) != 0 || index > PT_FPSCR32 { return ret; }
            let mut tmp: usize;
            if index < PT_FPR0 {
                tmp = 0;
                ret = ptrace_get_reg(child, index, &mut tmp);
                if ret != 0 { return ret; }
            } else {
                flush_fp_to_thread(child);
                tmp = *(((*child).thread.fp_state.fpr as *mut u32).add(fprindex(index))) as usize;
            }
            ret = put_user(tmp as u32, data as *mut u32);
        }
        PPC_PTRACE_PEEKUSR_3264 => {
            ret = -EIO;
            let index = ((addr as u64) >> 2) as u32;
            let num_reg = index / 2;
            let part = if index % 2 != 0 { 1 } else { 0 };
            if (addr & 3) != 0 || num_reg as usize > PT_FPSCR { return ret; }
            let tmp: u64;
            if num_reg as usize >= PT_FPR0 {
                flush_fp_to_thread(child);
                tmp = (*child).thread.fp_state.fpr[(num_reg as usize) - PT_FPR0][0];
            } else {
                let mut tmp2: usize = 0;
                ret = ptrace_get_reg(child, num_reg as usize, &mut tmp2);
                if ret != 0 { return ret; }
                tmp = tmp2 as u64;
            }
            let reg32bits = *((&tmp as *const u64 as *const u32).add(part as usize));
            ret = put_user(reg32bits, data as *mut u32);
        }
        PPC_PTRACE_POKETEXT_3264 | PPC_PTRACE_POKEDATA_3264 => {
            let mut tmp = data as u32;
            let mut addr_others: *mut u32 = core::ptr::null_mut();
            ret = -EIO;
            if get_user(&mut addr_others, addr as *const *const u32) != 0 { return ret; }
            ret = 0;
            if ptrace_access_vm(child, addr_others as u64, &mut tmp, core::mem::size_of::<u32>(), FOLL_FORCE | FOLL_WRITE) == core::mem::size_of::<u32>() { return ret; }
            ret = -EIO;
        }
        PTRACE_POKEUSR => {
            ret = -EIO;
            let index = addr >> 2;
            if (addr & 3) != 0 || index > PT_FPSCR32 { return ret; }
            if index < PT_FPR0 {
                ret = ptrace_put_reg(child, index, data);
            } else {
                flush_fp_to_thread(child);
                *(((*child).thread.fp_state.fpr as *mut u32).add(fprindex(index))) = data as u32;
                ret = 0;
            }
        }
        PPC_PTRACE_POKEUSR_3264 => {
            ret = -EIO;
            let index = ((addr as u64) >> 2) as u32;
            let num_reg = index / 2;
            if (addr & 3) != 0 || num_reg as usize > PT_FPSCR { return ret; }
            if (num_reg as usize) < PT_FPR0 {
                let mut freg: usize = 0;
                ret = ptrace_get_reg(child, num_reg as usize, &mut freg);
                if ret != 0 { return ret; }
                if index % 2 != 0 { freg = (freg & !0xffff_ffffusize) | (data & 0xffff_ffff); }
                else { freg = (freg & 0xffff_ffff) | (data << 32); }
                ret = ptrace_put_reg(child, num_reg as usize, freg);
            } else {
                flush_fp_to_thread(child);
                let tmp = &mut (*child).thread.fp_state.fpr[(num_reg as usize) - PT_FPR0][0] as *mut u64 as *mut u32;
                *tmp.add((index % 2) as usize) = data as u32;
                ret = 0;
            }
        }
        PTRACE_GET_DEBUGREG => {
            ret = -EINVAL;
            if addr > 0 { return ret; }
            #[cfg(CONFIG_PPC_ADV_DEBUG_REGS)]
            { ret = put_user((*child).thread.debug.dac1, data as *mut u32); }
            #[cfg(not(CONFIG_PPC_ADV_DEBUG_REGS))]
            {
                let dabr_fake = ((*child).thread.hw_brk[0].address & !HW_BRK_TYPE_DABR) | ((*child).thread.hw_brk[0].type_ & HW_BRK_TYPE_DABR);
                ret = put_user(dabr_fake, data as *mut u32);
            }
        }
        PTRACE_GETREGS => return copy_regset_to_user(child, task_user_regset_view(current), 0, 0, PT_REGS_COUNT * core::mem::size_of::<compat_long_t>(), compat_ptr(data)),
        PTRACE_SETREGS => return copy_regset_from_user(child, task_user_regset_view(current), 0, 0, PT_REGS_COUNT * core::mem::size_of::<compat_long_t>(), compat_ptr(data)),
        PTRACE_GETFPREGS | PTRACE_SETFPREGS | PTRACE_GETVRREGS | PTRACE_SETVRREGS |
        PTRACE_GETVSRREGS | PTRACE_SETVSRREGS | PTRACE_GETREGS64 | PTRACE_SETREGS64 |
        PTRACE_KILL | PTRACE_SINGLESTEP | PTRACE_DETACH | PTRACE_SET_DEBUGREG |
        PTRACE_SYSCALL | PTRACE_CONT | PPC_PTRACE_GETHWDBGINFO | PPC_PTRACE_SETHWDEBUG |
        PPC_PTRACE_DELHWDEBUG => { ret = arch_ptrace(child, request, addr, data); }
        _ => { ret = compat_ptrace_request(child, request, addr, data); }
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
