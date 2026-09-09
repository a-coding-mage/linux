// SPDX-License-Identifier: GPL-2.0
/*
 *  S390 version
 *    Copyright IBM Corp. 1999, 2000
 *    Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com),
 *               Thomas Spatzier (tspat@de.ibm.com)
 *
 *  Derived from "arch/i386/kernel/sys_i386.c"
 *
 *  This file contains various random system calls that
 *  have a non-standard calling sequence on the Linux/s390
 *  platform.
 */

// Linux and architecture header dependencies are supplied by the surrounding
// translation unit.

// The C __SYSCALL expansions declare the architecture syscall entry points
// and initialize the syscall table from asm/syscall_table.h.
extern "C" {
    pub fn __s390x_ni_syscall(regs: *mut pt_regs) -> c_long;
}

#[cfg(CONFIG_SYSVIPC)]
pub unsafe extern "C" fn __s390x_s390_ipc(regs: *mut pt_regs) -> c_long {
    s390_ipc(
        (*regs).gprs[2] as u32,
        (*regs).gprs[3] as i32,
        (*regs).gprs[4] as c_ulong,
        (*regs).gprs[5] as c_ulong,
        (*regs).gprs[6] as *mut c_void,
    )
}

// Equivalent to the C-generated syscall table; the complete architecture
// table is provided by the external syscall-table definition.
extern "C" {
    pub static sys_call_table: [sys_call_ptr_t; __NR_syscalls];
}

#[cfg(CONFIG_SYSVIPC)]
/*
 * sys_ipc() is the de-multiplexer for the SysV IPC calls.
 */
pub unsafe extern "C" fn s390_ipc(
    call: u32,
    first: i32,
    second: c_ulong,
    third: c_ulong,
    ptr: *mut c_void,
) -> c_long {
    if call >> 16 != 0 {
        return -EINVAL;
    }
    /* The s390 sys_ipc variant has only five parameters instead of six
     * like the generic variant. The only difference is the handling of
     * the SEMTIMEDOP subcall where on s390 the third parameter is used
     * as a pointer to a struct timespec where the generic variant uses
     * the fifth parameter.
     * Therefore we can call the generic variant by simply passing the
     * third parameter also as fifth parameter.
     */
    ksys_ipc(call, first, second, third, ptr, third)
}

pub unsafe extern "C" fn s390_personality(personality_arg: c_uint) -> c_long {
    let mut ret: c_uint = (*current).personality;
    let mut personality = personality_arg;

    if personality((*current).personality) == PER_LINUX32
        && personality(personality) == PER_LINUX
    {
        personality |= PER_LINUX32;
    }

    if personality != 0xffff_ffff {
        set_personality(personality);
    }

    if personality(ret) == PER_LINUX32 {
        ret &= !PER_LINUX32;
    }

    ret as c_long
}

pub unsafe extern "C" fn ni_syscall() -> c_long {
    -ENOSYS
}

pub unsafe extern "C" fn __do_syscall(regs: *mut pt_regs, flags: c_ulong) {
    let mut nr: c_ulong;
    let mut permit: bool;

    enter_from_user_mode_randomize_stack(regs);

    (*regs).psw = (*get_lowcore()).svc_old_psw;
    (*regs).int_code = (*get_lowcore()).svc_int_code;
    update_timer_sys();
    if cpu_has_bear() {
        (*current).thread.last_break = (*regs).last_break;
    }
    local_irq_enable();
    (*regs).orig_gpr2 = (*regs).gprs[2];
    if unlikely(flags & SYSCALL_FLAG_PER_TRAP != 0) {
        set_thread_flag(TIF_PER_TRAP);
    }
    (*regs).flags = 0;
    set_pt_regs_flag(regs, PIF_SYSCALL);
    nr = (*regs).int_code & 0xffff;
    if likely(nr == 0) {
        nr = (*regs).gprs[1] & 0xffff;
        (*regs).int_code &= !0xffff;
        (*regs).int_code |= nr;
    }
    (*regs).gprs[2] = nr;
    if nr == __NR_restart_syscall && ((*current).restart_block.arch_data & 1) == 0 {
        (*regs).psw.addr = (*current).restart_block.arch_data;
        (*current).restart_block.arch_data = 1;
    }

    permit = syscall_enter_from_user_mode_work(regs, &mut nr);

    /*
     * In the s390 ptrace ABI, both the syscall number and the return value
     * use gpr2. However, userspace puts the syscall number either in the
     * svc instruction itself, or uses gpr1. To make at least skipping syscalls
     * work, the ptrace code sets PIF_SYSCALL_RET_SET, which is checked here
     * and if set, the syscall will be skipped.
     */
    if unlikely(test_and_clear_pt_regs_flag(regs, PIF_SYSCALL_RET_SET) || !permit) {
        syscall_exit_to_user_mode(regs);
        return;
    }
    (*regs).gprs[2] = (-ENOSYS) as c_ulong;
    if likely(nr < NR_syscalls) {
        nr = array_index_nospec(nr, NR_syscalls);
        (*regs).gprs[2] = sys_call_table[nr as usize](regs);
    }
    syscall_exit_to_user_mode(regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
