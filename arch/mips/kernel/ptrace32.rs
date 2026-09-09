/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1992 Ross Biro
 * Copyright (C) Linus Torvalds
 * Copyright (C) 1994, 95, 96, 97, 98, 2000 Ralf Baechle
 * Copyright (C) 1996 David S. Miller
 * Kevin D. Kissell, kevink@mips.com and Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 1999 MIPS Technologies, Inc.
 * Copyright (C) 2000 Ulf Carlsson
 *
 * At this time Linux/MIPS64 only supports syscall tracing, even for 32-bit
 * binaries.
 */

/* Tracing a 32-bit process with a 64-bit strace and vice versa will not work. */

pub unsafe fn compat_arch_ptrace(
    child: *mut task_struct,
    request: compat_long_t,
    caddr: compat_ulong_t,
    cdata: compat_ulong_t,
) -> c_long {
    let addr: c_int = caddr as c_int;
    let data: c_int = cdata as c_int;
    let mut ret: c_int;

    match request {
        PTRACE_PEEKTEXT_3264 | PTRACE_PEEKDATA_3264 => {
            let mut tmp: u32 = 0;
            let mut addr_others: *mut u32 = core::ptr::null_mut();
            ret = -EIO;
            if get_user(&mut addr_others, (addr as usize) as *mut *mut u32) != 0 {
                return ret as c_long;
            }
            let copied = ptrace_access_vm(child, addr_others as u64, &mut tmp, core::mem::size_of::<u32>(), FOLL_FORCE);
            if copied != core::mem::size_of::<u32>() as c_int {
                return ret as c_long;
            }
            ret = put_user(tmp, (data as usize) as *mut u32);
        }
        PTRACE_PEEKUSR => {
            let regs = task_pt_regs(child);
            let mut tmp: u32 = 0;
            ret = 0;
            match addr {
                0..=31 => tmp = (*regs).regs[addr as usize] as u32,
                #[cfg(CONFIG_MIPS_FP_SUPPORT)]
                FPR_BASE..=FPR_BASE + 31 => {
                    if !tsk_used_math(child) { tmp = u32::MAX; }
                    else {
                        let fregs = get_fpu_regs(child);
                        if test_tsk_thread_flag(child, TIF_32BIT_FPREGS) {
                            tmp = get_fpr32(fregs.add(((addr & !1) - FPR_BASE) as usize), (addr & 1) as c_int);
                        } else { tmp = get_fpr64(fregs.add((addr - FPR_BASE) as usize), 0); }
                    }
                }
                #[cfg(CONFIG_MIPS_FP_SUPPORT)]
                FPC_CSR => tmp = (*child).thread.fpu.fcr31,
                #[cfg(CONFIG_MIPS_FP_SUPPORT)]
                FPC_EIR => tmp = boot_cpu_data.fpu_id,
                PC => tmp = (*regs).cp0_epc as u32,
                CAUSE => tmp = (*regs).cp0_cause as u32,
                BADVADDR => tmp = (*regs).cp0_badvaddr as u32,
                MMHI => tmp = (*regs).hi as u32,
                MMLO => tmp = (*regs).lo as u32,
                DSP_BASE..=DSP_BASE + 5 => {
                    if !cpu_has_dsp { return (-EIO) as c_long; }
                    tmp = *(__get_dsp_regs(child)).add((addr - DSP_BASE) as usize);
                }
                DSP_CONTROL => {
                    if !cpu_has_dsp { return (-EIO) as c_long; }
                    tmp = (*child).thread.dsp.dspcontrol;
                }
                _ => return (-EIO) as c_long,
            }
            ret = put_user(tmp, (data as usize) as *mut u32);
        }
        PTRACE_POKETEXT_3264 | PTRACE_POKEDATA_3264 => {
            let mut addr_others: *mut u32 = core::ptr::null_mut();
            ret = -EIO;
            if get_user(&mut addr_others, (addr as usize) as *mut *mut u32) != 0 { return ret as c_long; }
            ret = 0;
            if ptrace_access_vm(child, addr_others as u64, (&data as *const c_int) as *mut _, core::mem::size_of::<c_int>(), FOLL_FORCE | FOLL_WRITE) != core::mem::size_of::<c_int>() as c_int { ret = -EIO; }
        }
        PTRACE_POKEUSR => {
            let regs = task_pt_regs(child);
            ret = 0;
            match addr {
                0..=31 => { (*regs).regs[addr as usize] = data as _; if addr == 2 || (addr == 4 && mips_syscall_is_indirect(child, regs)) { mips_syscall_update_nr(child, regs); } }
                #[cfg(CONFIG_MIPS_FP_SUPPORT)]
                FPR_BASE..=FPR_BASE + 31 => { let fregs = get_fpu_regs(child); if !tsk_used_math(child) { core::ptr::write_bytes(&mut (*child).thread.fpu as *mut _, u8::MAX, core::mem::size_of_val(&(*child).thread.fpu)); (*child).thread.fpu.fcr31 = 0; } if test_tsk_thread_flag(child, TIF_32BIT_FPREGS) { set_fpr32(fregs.add(((addr & !1) - FPR_BASE) as usize), (addr & 1) as c_int, data as _); } else { set_fpr64(fregs.add((addr - FPR_BASE) as usize), 0, data as _); } }
                #[cfg(CONFIG_MIPS_FP_SUPPORT)] FPC_CSR => (*child).thread.fpu.fcr31 = data as _,
                PC => (*regs).cp0_epc = data as _, MMHI => (*regs).hi = data as _, MMLO => (*regs).lo = data as _,
                DSP_BASE..=DSP_BASE + 5 => { if !cpu_has_dsp { ret = -EIO; } else { *(__get_dsp_regs(child)).add((addr - DSP_BASE) as usize) = data as _; } }
                DSP_CONTROL => { if !cpu_has_dsp { ret = -EIO; } else { (*child).thread.dsp.dspcontrol = data as _; } }
                _ => ret = -EIO,
            }
        }
        PTRACE_GETREGS => ret = ptrace_getregs(child, data as u64 as *mut user_pt_regs),
        PTRACE_SETREGS => ret = ptrace_setregs(child, data as u64 as *mut user_pt_regs),
        #[cfg(CONFIG_MIPS_FP_SUPPORT)] PTRACE_GETFPREGS => ret = ptrace_getfpregs(child, data as u64 as *mut u32),
        #[cfg(CONFIG_MIPS_FP_SUPPORT)] PTRACE_SETFPREGS => ret = ptrace_setfpregs(child, data as u64 as *mut u32),
        PTRACE_GET_THREAD_AREA => ret = put_user((*task_thread_info(child)).tp_value, (data as usize) as *mut u32),
        PTRACE_GET_THREAD_AREA_3264 => ret = put_user((*task_thread_info(child)).tp_value, (data as usize) as *mut c_ulong),
        PTRACE_GET_WATCH_REGS => ret = ptrace_get_watch_regs(child, (addr as usize) as *mut pt_watch_regs),
        PTRACE_SET_WATCH_REGS => ret = ptrace_set_watch_regs(child, (addr as usize) as *mut pt_watch_regs),
        _ => ret = compat_ptrace_request(child, request, addr, data),
    }
    ret as c_long
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
