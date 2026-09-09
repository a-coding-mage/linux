// SPDX-License-Identifier: GPL-2.0
/* linux/arch/sparc64/kernel/sys_sparc.c
 *
 * This file contains various random system calls that
 * have a non-standard calling sequence on the Linux/sparc
 * platform.
 */

// Kernel headers and symbols are supplied by the surrounding translation unit.

/* #define DEBUG_UNIMP_SYSCALL */

pub unsafe fn getpagesize() -> usize { PAGE_SIZE }

/* Does addr --> addr+len fall within 4GB of the VA-space hole or
 * overflow past the end of the 64-bit address space?
 */
unsafe fn invalid_64bit_range(addr: c_ulong, len: c_ulong) -> c_int {
    let va_exclude_start = VA_EXCLUDE_START;
    let va_exclude_end = VA_EXCLUDE_END;
    if len >= va_exclude_start { return 1; }
    if addr.wrapping_add(len) < addr { return 1; }
    if (addr >= va_exclude_start && addr < va_exclude_end) ||
       (addr.wrapping_add(len) >= va_exclude_start && addr.wrapping_add(len) < va_exclude_end) { return 1; }
    0
}

unsafe fn COLOR_ALIGN(addr: c_ulong, pgoff: c_ulong) -> c_ulong {
    let base = addr.wrapping_add(SHMLBA - 1) & !(SHMLBA - 1);
    let off = (pgoff << PAGE_SHIFT) & (SHMLBA - 1);
    base.wrapping_add(off)
}

unsafe fn get_align_mask(filp: *mut file, flags: c_ulong) -> c_ulong {
    if !filp.is_null() && is_file_hugepages(filp) { return huge_page_mask_align(filp); }
    if !filp.is_null() || (flags & MAP_SHARED) != 0 { return PAGE_MASK & (SHMLBA - 1); }
    0
}

pub unsafe fn arch_get_unmapped_area(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong, _vm_flags: vm_flags_t) -> c_ulong {
    let mm = (*current).mm;
    let mut task_size = TASK_SIZE;
    let mut addr = addr;
    let file_hugepage = !filp.is_null() && is_file_hugepages(filp);
    if (flags & MAP_FIXED) != 0 {
        if !file_hugepage && (flags & MAP_SHARED) != 0 && ((addr.wrapping_sub(pgoff << PAGE_SHIFT)) & (SHMLBA - 1)) != 0 { return -EINVAL as c_ulong; }
        return addr;
    }
    if test_thread_flag(TIF_32BIT) { task_size = STACK_TOP32; }
    if len > task_size || len >= VA_EXCLUDE_START { return -ENOMEM as c_ulong; }
    let do_color_align = (!filp.is_null() || (flags & MAP_SHARED) != 0) && !file_hugepage;
    if addr != 0 {
        addr = if do_color_align { COLOR_ALIGN(addr, pgoff) } else { PAGE_ALIGN(addr) };
        let vma = find_vma(mm, addr);
        if task_size - len >= addr && (vma.is_null() || addr + len <= vm_start_gap(vma)) { return addr; }
    }
    let mut info: vm_unmapped_area_info = core::mem::zeroed();
    info.length = len; info.low_limit = TASK_UNMAPPED_BASE; info.high_limit = min(task_size, VA_EXCLUDE_START);
    info.align_mask = get_align_mask(filp, flags);
    if !file_hugepage { info.align_offset = pgoff << PAGE_SHIFT; }
    addr = vm_unmapped_area(&info);
    if (addr & !PAGE_MASK) != 0 && task_size > VA_EXCLUDE_END {
        VM_BUG_ON(addr != -ENOMEM as c_ulong);
        info.low_limit = VA_EXCLUDE_END; info.high_limit = task_size; addr = vm_unmapped_area(&info);
    }
    addr
}

pub unsafe fn arch_get_unmapped_area_topdown(filp: *mut file, addr0: c_ulong, len: c_ulong, pgoff: c_ulong, flags: c_ulong, _vm_flags: vm_flags_t) -> c_ulong {
    let mm = (*current).mm; let task_size = STACK_TOP32; let mut addr = addr0;
    BUG_ON(!test_thread_flag(TIF_32BIT));
    let file_hugepage = !filp.is_null() && is_file_hugepages(filp);
    if (flags & MAP_FIXED) != 0 {
        if !file_hugepage && (flags & MAP_SHARED) != 0 && ((addr.wrapping_sub(pgoff << PAGE_SHIFT)) & (SHMLBA - 1)) != 0 { return -EINVAL as c_ulong; }
        return addr;
    }
    if len > task_size { return -ENOMEM as c_ulong; }
    let do_color_align = (!filp.is_null() || (flags & MAP_SHARED) != 0) && !file_hugepage;
    if addr != 0 {
        addr = if do_color_align { COLOR_ALIGN(addr, pgoff) } else { PAGE_ALIGN(addr) };
        let vma = find_vma(mm, addr);
        if task_size - len >= addr && (vma.is_null() || addr + len <= vm_start_gap(vma)) { return addr; }
    }
    let mut info: vm_unmapped_area_info = core::mem::zeroed(); info.flags = VM_UNMAPPED_AREA_TOPDOWN; info.length = len;
    info.low_limit = PAGE_SIZE; info.high_limit = (*mm).mmap_base; info.align_mask = get_align_mask(filp, flags);
    if !file_hugepage { info.align_offset = pgoff << PAGE_SHIFT; }
    addr = vm_unmapped_area(&info);
    if (addr & !PAGE_MASK) != 0 { VM_BUG_ON(addr != -ENOMEM as c_ulong); info.flags = 0; info.low_limit = TASK_UNMAPPED_BASE; info.high_limit = STACK_TOP32; addr = vm_unmapped_area(&info); }
    addr
}

/* Try to align mapping such that we align it as much as possible. */
pub unsafe fn get_fb_unmapped_area(_filp: *mut file, orig_addr: c_ulong, len: c_ulong, pgoff: c_ulong, mut flags: c_ulong) -> c_ulong {
    if (flags & MAP_FIXED) != 0 { return mm_get_unmapped_area(core::ptr::null_mut(), orig_addr, len, pgoff, flags); }
    flags &= !MAP_SHARED;
    let mut align_goal = PAGE_SIZE;
    if len >= 4 * 1024 * 1024 { align_goal = 4 * 1024 * 1024; } else if len >= 512 * 1024 { align_goal = 512 * 1024; } else if len >= 64 * 1024 { align_goal = 64 * 1024; }
    let mut addr;
    loop {
        addr = mm_get_unmapped_area(core::ptr::null_mut(), orig_addr, len + align_goal - PAGE_SIZE, pgoff, flags);
        if (addr & !PAGE_MASK) == 0 { addr = (addr + align_goal - 1) & !(align_goal - 1); break; }
        if align_goal == 4 * 1024 * 1024 { align_goal = 512 * 1024; } else if align_goal == 512 * 1024 { align_goal = 64 * 1024; } else { align_goal = PAGE_SIZE; }
        if !((addr & !PAGE_MASK) != 0 && align_goal > PAGE_SIZE) { break; }
    }
    if (addr & !PAGE_MASK) != 0 { addr = mm_get_unmapped_area(core::ptr::null_mut(), orig_addr, len, pgoff, flags); }
    addr
}

/* Essentially the same as PowerPC.  */
unsafe fn mmap_rnd() -> c_ulong {
    let mut rnd = 0;
    if ((*current).flags & PF_RANDOMIZE) != 0 { let val = get_random_long(); rnd = if test_thread_flag(TIF_32BIT) { val % (1 << (23 - PAGE_SHIFT)) } else { val % (1 << (30 - PAGE_SHIFT)) }; }
    rnd << PAGE_SHIFT
}

pub unsafe fn arch_pick_mmap_layout(mm: *mut mm_struct, rlim_stack: *const rlimit) {
    let random_factor = mmap_rnd(); let mut gap = (*rlim_stack).rlim_cur;
    if !test_thread_flag(TIF_32BIT) || ((*current).personality & ADDR_COMPAT_LAYOUT) != 0 || gap == RLIM_INFINITY || sysctl_legacy_va_layout != 0 {
        (*mm).mmap_base = TASK_UNMAPPED_BASE + random_factor; mm_flags_clear(MMF_TOPDOWN, mm);
    } else { let task_size = STACK_TOP32; if gap < 128 * 1024 * 1024 { gap = 128 * 1024 * 1024; } if gap > task_size / 6 * 5 { gap = task_size / 6 * 5; } (*mm).mmap_base = PAGE_ALIGN(task_size - gap - random_factor); mm_flags_set(MMF_TOPDOWN, mm); }
}

/* sys_pipe() is the normal C calling standard for creating a pipe. It's not the way unix traditionally does this, though. */
pub unsafe fn sparc_pipe() -> c_int { let mut fd = [0; 2]; let mut error = do_pipe_flags(fd.as_mut_ptr(), 0); if error == 0 { (*current_pt_regs()).u_regs[UREG_I1] = fd[1] as _; error = fd[0]; } error }

pub unsafe fn sparc_ipc(call: c_uint, first: c_int, second: c_ulong, third: c_ulong, ptr: *mut c_void, fifth: c_long) -> c_long {
    if !IS_ENABLED_CONFIG_SYSVIPC { return -ENOSYS; }
    let mut err;
    if call <= SEMTIMEDOP { err = match call { SEMOP => ksys_semtimedop(first, ptr, second as c_uint, core::ptr::null()), SEMTIMEDOP => ksys_semtimedop(first, ptr, second as c_uint, fifth as c_ulong as *const __kernel_timespec), SEMGET => ksys_semget(first, second as c_int, third as c_int), SEMCTL => ksys_old_semctl(first, second, third as c_int | IPC_64, ptr as c_ulong), _ => -ENOSYS }; return err; }
    if call <= MSGCTL { err = match call { MSGSND => ksys_msgsnd(first, ptr, second as usize, third as c_int), MSGRCV => ksys_msgrcv(first, ptr, second as usize, fifth, third as c_int), MSGGET => ksys_msgget(first as key_t, second as c_int), MSGCTL => ksys_old_msgctl(first, second as c_int | IPC_64, ptr), _ => -ENOSYS }; return err; }
    if call <= SHMCTL { err = match call { SHMAT => { let mut raddr: ulong = 0; let mut e = do_shmat(first, ptr, second as c_int, &mut raddr, SHMLBA); if e == 0 && put_user(raddr, third as *mut ulong) != 0 { e = -EFAULT; } e }, SHMDT => ksys_shmdt(ptr), SHMGET => ksys_shmget(first, second as usize, third as c_int), SHMCTL => ksys_old_shmctl(first, second as c_int | IPC_64, ptr), _ => -ENOSYS }; return err; }
    -ENOSYS
}

pub unsafe fn sparc64_personality(mut personality_arg: c_ulong) -> c_long { let ret; if personality((*current).personality) == PER_LINUX32 && personality(personality_arg) == PER_LINUX { personality_arg |= PER_LINUX32; } ret = sys_personality(personality_arg); if personality(ret as c_ulong) == PER_LINUX32 { ret & !(PER_LINUX32 as c_long) } else { ret } }

pub unsafe fn sparc_mmap_check(addr: c_ulong, len: c_ulong) -> c_int { if test_thread_flag(TIF_32BIT) { if len >= STACK_TOP32 || addr > STACK_TOP32 - len { return -EINVAL; } } else if len >= VA_EXCLUDE_START || invalid_64bit_range(addr, len) != 0 { return -EINVAL; } 0 }

pub unsafe fn mmap(addr: c_ulong, len: c_ulong, prot: c_ulong, flags: c_ulong, fd: c_ulong, off: c_ulong) -> c_ulong { if off.wrapping_add(PAGE_ALIGN(len)) < off || (off & !PAGE_MASK) != 0 { return -EINVAL as c_ulong; } ksys_mmap_pgoff(addr, len, prot, flags, fd, off >> PAGE_SHIFT) }
pub unsafe fn munmap_64(addr: c_ulong, len: usize) -> c_long { if invalid_64bit_range(addr, len as c_ulong) != 0 { return -EINVAL; } vm_munmap(addr, len) }
pub unsafe fn mremap_64(addr: c_ulong, old_len: c_ulong, new_len: c_ulong, flags: c_ulong, new_addr: c_ulong) -> c_long { if test_thread_flag(TIF_32BIT) { return -EINVAL; } sys_mremap(addr, old_len, new_len, flags, new_addr) }

pub unsafe fn nis_syscall() -> c_long { static mut COUNT: c_int = 0; let regs = current_pt_regs(); if COUNT > 5 { return -ENOSYS; } COUNT += 1; printk(b"Unimplemented SPARC system call %ld\0" as *const _ as *const c_char, (*regs).u_regs[1]); -ENOSYS }

/* #define DEBUG_SPARC_BREAKPOINT */
pub unsafe fn sparc_breakpoint(regs: *mut pt_regs) { let prev_state = exception_enter(); if test_thread_flag(TIF_32BIT) { (*regs).tpc &= 0xffffffff; (*regs).tnpc &= 0xffffffff; } force_sig_fault(SIGTRAP, TRAP_BRKPT, (*regs).tpc as *mut c_void); exception_exit(prev_state); }

pub unsafe fn getdomainname(name: *mut c_char, len: c_int) -> c_long { if len < 0 { return -EINVAL; } down_read(&mut uts_sem); let nlen = strlen(utsname().domainname) + 1; if nlen > len as usize { up_read(&mut uts_sem); return -EINVAL; } let mut tmp = [0i8; __NEW_UTS_LEN + 1]; memcpy(tmp.as_mut_ptr() as *mut c_void, utsname().domainname as *const _ as *const c_void, nlen); up_read(&mut uts_sem); if copy_to_user(name as *mut c_void, tmp.as_ptr() as *const c_void, nlen) != 0 { -EFAULT } else { 0 } }

pub unsafe fn sparc_adjtimex(txc_p: *mut __kernel_timex) -> c_long { let mut txc: __kernel_timex = core::mem::zeroed(); if copy_from_user(&mut txc as *mut _ as *mut c_void, txc_p as *const c_void, core::mem::size_of::<__kernel_timex>()) != 0 { return -EFAULT; } let tv = &mut txc.time as *mut _ as *mut __kernel_old_timeval; (*tv).tv_usec = (*(&txc.time as *const _ as *const __kernel_old_timeval)).tv_usec; let ret = do_adjtimex(&mut txc); (*tv).tv_usec = txc.time.tv_usec; if copy_to_user(txc_p as *mut c_void, &txc as *const _ as *const c_void, core::mem::size_of::<__kernel_timex>()) != 0 { -EFAULT } else { ret } }

pub unsafe fn sparc_clock_adjtime(which_clock: clockid_t, txc_p: *mut __kernel_timex) -> c_long { if !IS_ENABLED_CONFIG_POSIX_TIMERS { return -ENOSYS; } let mut txc: __kernel_timex = core::mem::zeroed(); if copy_from_user(&mut txc as *mut _ as *mut c_void, txc_p as *const c_void, core::mem::size_of::<__kernel_timex>()) != 0 { return -EFAULT; } let tv = &mut txc.time as *mut _ as *mut __kernel_old_timeval; (*tv).tv_usec = (*tv).tv_usec; let ret = do_clock_adjtime(which_clock, &mut txc); (*tv).tv_usec = txc.time.tv_usec; if copy_to_user(txc_p as *mut c_void, &txc as *const _ as *const c_void, core::mem::size_of::<__kernel_timex>()) != 0 { -EFAULT } else { ret } }

pub unsafe fn utrap_install(type_: utrap_entry_t, new_p: utrap_handler_t, _new_d: utrap_handler_t, old_p: *mut utrap_handler_t, old_d: *mut utrap_handler_t) -> c_long { if type_ < UT_INSTRUCTION_EXCEPTION || type_ > UT_TRAP_INSTRUCTION_31 { return -EINVAL; } let ti = current_thread_info(); if new_p == UTH_NOCHANGE as utrap_handler_t { if !old_p.is_null() { if (*ti).utraps.is_null() { if put_user(core::ptr::null_mut(), old_p) != 0 { return -EFAULT; } } else if put_user((*ti).utraps.add(type_) as utrap_handler_t, old_p) != 0 { return -EFAULT; } } if !old_d.is_null() && put_user(core::ptr::null_mut(), old_d) != 0 { return -EFAULT; } return 0; } if (*ti).utraps.is_null() { (*ti).utraps = kzalloc_objs_long(UT_TRAP_INSTRUCTION_31 + 1); if (*ti).utraps.is_null() { return -ENOMEM; } *(*ti).utraps = 1; } if !old_p.is_null() && put_user(*(*ti).utraps.add(type_) as utrap_handler_t, old_p) != 0 { return -EFAULT; } if !old_d.is_null() && put_user(core::ptr::null_mut(), old_d) != 0 { return -EFAULT; } *(*ti).utraps.add(type_) = new_p as c_long; 0 }

pub unsafe fn memory_ordering(model: c_ulong) -> c_long { let regs = current_pt_regs(); if model >= 3 { return -EINVAL; } (*regs).tstate = ((*regs).tstate & !TSTATE_MM) | (model << 14); 0 }

pub unsafe fn rt_sigaction(sig: c_int, act: *const sigaction, oact: *mut sigaction, restorer: *mut c_void, sigsetsize: usize) -> c_long { if sigsetsize != core::mem::size_of::<sigset_t>() { return -EINVAL; } let mut new_ka: k_sigaction = core::mem::zeroed(); let mut old_ka: k_sigaction = core::mem::zeroed(); if !act.is_null() { new_ka.ka_restorer = restorer; if copy_from_user(&mut new_ka.sa as *mut _ as *mut c_void, act as *const c_void, core::mem::size_of::<sigaction>()) != 0 { return -EFAULT; } } let ret = do_sigaction(sig, if act.is_null() { core::ptr::null() } else { &new_ka }, if oact.is_null() { core::ptr::null_mut() } else { &mut old_ka }); if ret == 0 && !oact.is_null() && copy_to_user(oact as *mut c_void, &old_ka.sa as *const _ as *const c_void, core::mem::size_of::<sigaction>()) != 0 { return -EFAULT; } ret }

pub unsafe fn kern_features() -> c_ulong { KERN_FEATURE_MIXED_MODE_STACK }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
