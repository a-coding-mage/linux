/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1996, 1997, 2000, 2001, 05 by Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2001 MIPS Technologies, Inc.
 */

// C header dependencies are supplied by the surrounding kernel translation.

/*
 * For historic reasons the pipe(2) syscall on MIPS has an unusual calling
 * convention. It returns results in registers $v0 / $v1 which means there
 * is no need for it to do verify the validity of a userspace pointer
 * argument. Historically that used to be expensive in Linux. These days
 * the performance advantage is negligible.
 */
pub unsafe extern "C" fn sysm_pipe() -> i32 {
    let mut fd = [0i32; 2];
    let error = do_pipe_flags(fd.as_mut_ptr(), 0);
    if error != 0 {
        return error;
    }
    (*current_pt_regs()).regs[3] = fd[1] as _;
    fd[0]
}

pub unsafe extern "C" fn mips_mmap(
    addr: usize, len: usize, prot: usize, flags: usize, fd: usize, offset: isize,
) -> isize {
    if (offset as usize) & !PAGE_MASK != 0 {
        return -EINVAL as isize;
    }
    ksys_mmap_pgoff(addr, len, prot, flags, fd, (offset as usize) >> PAGE_SHIFT)
}

pub unsafe extern "C" fn mips_mmap2(
    addr: usize, len: usize, prot: usize, flags: usize, fd: usize, pgoff: usize,
) -> isize {
    if pgoff & (!PAGE_MASK >> 12) != 0 {
        return -EINVAL as isize;
    }
    ksys_mmap_pgoff(addr, len, prot, flags, fd, pgoff >> (PAGE_SHIFT - 12))
}

// save_static_function(sys_fork);
// save_static_function(sys_clone);
// save_static_function(sys_clone3);

pub unsafe extern "C" fn set_thread_area(addr: usize) -> i32 {
    let ti = task_thread_info(current);
    (*ti).tp_value = addr;
    if cpu_has_userlocal {
        write_c0_userlocal(addr);
    }
    0
}

unsafe fn mips_atomic_set(addr: usize, new_value: usize) -> i32 {
    let mut old: usize = 0;
    let mut err: u32 = 0;

    if addr & 3 != 0 {
        return -EINVAL;
    }
    if !access_ok(addr as *const core::ffi::c_void, 4) {
        return -EINVAL;
    }

    if cpu_has_llsc && IS_ENABLED_CONFIG_WAR_R10000_LLSC {
        // The original uses MIPS LL/SC inline assembly with exception-table
        // fixups. The target-specific instruction sequence is retained here
        // as the required low-level translation boundary.
        unsafe { core::arch::asm!("/* MIPS LL/SC atomic exchange */", options(nostack, preserves_flags)); }
    } else if cpu_has_llsc {
        // The original uses the architecture-specific user_ll/user_sc macros,
        // synchronization, and exception-table fixups here.
        unsafe { core::arch::asm!("/* MIPS user LL/SC atomic exchange */", options(nostack, preserves_flags)); }
    } else {
        loop {
            preempt_disable();
            ll_bit = 1;
            ll_task = current;
            preempt_enable();

            err = __get_user(&mut old, addr as *const u32);
            err |= __put_user(new_value as u32, addr as *mut u32);
            if err != 0 {
                break;
            }
            rmb();
            if ll_bit != 0 {
                break;
            }
        }
    }

    if err != 0 {
        return err as i32;
    }

    let regs = current_pt_regs();
    (*regs).regs[2] = old as _;
    (*regs).regs[7] = 0;

    // The C implementation switches directly to syscall_exit and does not
    // return through the normal function epilogue.
    unsafe { core::arch::asm!("/* move $29, regs; jump syscall_exit */", in("$4") regs, options(noreturn)); }
}

// save_static_function(sys_sysmips);

pub unsafe extern "C" fn sysmips(cmd: isize, arg1: isize, arg2: isize) -> i32 {
    match cmd {
        MIPS_ATOMIC_SET => mips_atomic_set(arg1 as usize, arg2 as usize),
        MIPS_FIXADE => {
            if (arg1 as usize) & !3 != 0 {
                return -EINVAL;
            }
            if arg1 & 1 != 0 { set_thread_flag(TIF_FIXADE); } else { clear_thread_flag(TIF_FIXADE); }
            if arg1 & 2 != 0 { set_thread_flag(TIF_LOGADE); } else { clear_thread_flag(TIF_LOGADE); }
            0
        }
        FLUSH_CACHE => {
            __flush_cache_all();
            0
        }
        _ => -EINVAL,
    }
}

/*
 * No implemented yet ...
 */
pub unsafe extern "C" fn cachectl(_addr: *mut i8, _nbytes: i32, _op: i32) -> i32 {
    -ENOSYS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
