// SPDX-License-Identifier: GPL-2.0
/* linux/arch/sparc/kernel/sys_sparc.c
 *
 * This file contains various random system calls that
 * have a non-standard calling sequence on the Linux/sparc
 * platform.
 */

// Kernel declarations and constants supplied by the surrounding Linux/Rust bindings.

/* #define DEBUG_UNIMP_SYSCALL */

/* XXX Make this per-binary type, this way we can detect the type of
 * XXX a binary.  Every Sparc executable calls this very early on.
 */
pub unsafe extern "C" fn getpagesize() -> c_long {
    PAGE_SIZE as c_long /* Possibly older binaries want 8192 on sun4's? */
}

pub unsafe fn arch_get_unmapped_area(
    filp: *mut file,
    mut addr: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
    vm_flags: vm_flags_t,
) -> c_ulong {
    let mut info: vm_unmapped_area_info = core::mem::zeroed();
    let mut file_hugepage = false;

    if !filp.is_null() && is_file_hugepages(filp) {
        file_hugepage = true;
    }

    if flags & MAP_FIXED != 0 {
        /* We do not accept a shared mapping if it would violate
         * cache aliasing constraints.
         */
        if !file_hugepage
            && flags & MAP_SHARED != 0
            && ((addr.wrapping_sub(pgoff << PAGE_SHIFT)) & (SHMLBA - 1)) != 0
        {
            return (-EINVAL) as c_ulong;
        }
        return addr;
    }

    /* See asm-sparc/uaccess.h */
    if len > TASK_SIZE - PAGE_SIZE {
        return (-ENOMEM) as c_ulong;
    }
    if addr == 0 {
        addr = TASK_UNMAPPED_BASE;
    }

    info.length = len;
    info.low_limit = addr;
    info.high_limit = TASK_SIZE;
    if !file_hugepage {
        info.align_mask = if flags & MAP_SHARED != 0 {
            PAGE_MASK & (SHMLBA - 1)
        } else {
            0
        };
        info.align_offset = pgoff << PAGE_SHIFT;
    } else {
        info.align_mask = huge_page_mask_align(filp);
    }
    vm_unmapped_area(&info)
}

/*
 * sys_pipe() is the normal C calling standard for creating
 * a pipe. It's not the way unix traditionally does this, though.
 */
pub unsafe extern "C" fn sparc_pipe() -> c_int {
    let mut fd = [0 as c_int; 2];
    let mut error: c_int;

    error = do_pipe_flags(fd.as_mut_ptr(), 0);
    if error != 0 {
        return error;
    }
    (*current_pt_regs()).u_regs[UREG_I1] = fd[1] as _;
    error = fd[0];
    error
}

pub unsafe fn sparc_mmap_check(addr: c_ulong, len: c_ulong) -> c_int {
    /* See asm-sparc/uaccess.h */
    if len > TASK_SIZE - PAGE_SIZE || addr.wrapping_add(len) > TASK_SIZE - PAGE_SIZE {
        return -EINVAL;
    }

    0
}

/* Linux version of mmap */

pub unsafe extern "C" fn mmap2(
    addr: c_ulong, len: c_ulong, prot: c_ulong, flags: c_ulong,
    fd: c_ulong, pgoff: c_ulong,
) -> c_long {
    /* Make sure the shift for mmap2 is constant (12), no matter what PAGE_SIZE
       we have. */
    ksys_mmap_pgoff(addr, len, prot, flags, fd, pgoff >> (PAGE_SHIFT - 12))
}

pub unsafe extern "C" fn mmap(
    addr: c_ulong, len: c_ulong, prot: c_ulong, flags: c_ulong,
    fd: c_ulong, off: c_ulong,
) -> c_long {
    /* no alignment check? */
    ksys_mmap_pgoff(addr, len, prot, flags, fd, off >> PAGE_SHIFT)
}

pub unsafe extern "C" fn sparc_remap_file_pages(
    start: c_ulong, size: c_ulong, prot: c_ulong, pgoff: c_ulong, flags: c_ulong,
) -> c_long {
    /* This works on an existing mmap so we don't need to validate
     * the range as that was done at the original mmap call.
     */
    sys_remap_file_pages(start, size, prot, pgoff >> (PAGE_SHIFT - 12), flags)
}

pub unsafe extern "C" fn nis_syscall() -> c_long {
    static mut COUNT: c_int = 0;
    let regs = current_pt_regs();

    COUNT += 1;
    if COUNT > 5 {
        return (-ENOSYS) as c_long;
    }
    printk(
        c"%s[%d]: Unimplemented SPARC system call %d\n".as_ptr(),
        (*current).comm.as_ptr(), task_pid_nr(current), (*regs).u_regs[1] as c_int,
    );
    /* #ifdef DEBUG_UNIMP_SYSCALL: show_regs(regs); */
    (-ENOSYS) as c_long
}

/* #define DEBUG_SPARC_BREAKPOINT */

pub unsafe extern "C" fn sparc_breakpoint(regs: *mut pt_regs) {
    /* #ifdef DEBUG_SPARC_BREAKPOINT: printk(...); */
    force_sig_fault(SIGTRAP, TRAP_BRKPT, (*regs).pc as *mut core::ffi::c_void);
    /* #ifdef DEBUG_SPARC_BREAKPOINT: printk(...); */
}

pub unsafe extern "C" fn sparc_sigaction(
    sig: c_int,
    act: *mut old_sigaction,
    oact: *mut old_sigaction,
) -> c_long {
    WARN_ON_ONCE(sig >= 0);
    sys_sigaction(-sig, act, oact)
}

pub unsafe extern "C" fn rt_sigaction(
    sig: c_int,
    act: *const sigaction,
    oact: *mut sigaction,
    restorer: *mut core::ffi::c_void,
    sigsetsize: usize,
) -> c_long {
    let mut new_ka: k_sigaction = core::mem::zeroed();
    let mut old_ka: k_sigaction = core::mem::zeroed();
    let mut ret: c_int;

    /* XXX: Don't preclude handling different sized sigset_t's.  */
    if sigsetsize != core::mem::size_of::<sigset_t>() {
        return (-EINVAL) as c_long;
    }

    if !act.is_null() {
        new_ka.ka_restorer = restorer;
        if copy_from_user(&mut new_ka.sa, act, core::mem::size_of::<sigaction>()) != 0 {
            return (-EFAULT) as c_long;
        }
    }

    ret = do_sigaction(
        sig,
        if !act.is_null() { &mut new_ka } else { core::ptr::null_mut() },
        if !oact.is_null() { &mut old_ka } else { core::ptr::null_mut() },
    );

    if ret == 0 && !oact.is_null() {
        if copy_to_user(oact, &old_ka.sa, core::mem::size_of::<sigaction>()) != 0 {
            return (-EFAULT) as c_long;
        }
    }

    ret as c_long
}

pub unsafe extern "C" fn getdomainname(name: *mut c_char, len: c_int) -> c_long {
    let mut nlen: usize;
    let mut err: c_int;
    let mut tmp = [0 as c_char; __NEW_UTS_LEN + 1];

    if len < 0 {
        return (-EINVAL) as c_long;
    }

    down_read(&uts_sem);

    nlen = strlen(utsname().domainname.as_ptr()) + 1;
    err = -EINVAL;
    if nlen > len as usize {
        up_read(&uts_sem);
        return err as c_long;
    }
    memcpy(tmp.as_mut_ptr(), utsname().domainname.as_ptr(), nlen);

    up_read(&uts_sem);

    if copy_to_user(name, tmp.as_ptr(), nlen) != 0 {
        return (-EFAULT) as c_long;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
