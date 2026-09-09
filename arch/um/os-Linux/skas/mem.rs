// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Benjamin Berg <benjamin@sipsolutions.net>
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use core::mem::{offset_of, size_of};

extern "C" {
    static mut __syscall_stub_start: core::ffi::c_char;
}

pub unsafe fn syscall_stub_dump_error(mm_idp: *mut mm_id) {
    let proc_data = (*mm_idp).stack as *mut stub_data;
    let mut sc: *mut stub_syscall;

    if (*proc_data).syscall_data_len < 0
        || (*proc_data).syscall_data_len as usize >= (*proc_data).syscall_data.len()
    {
        panic!("Syscall data was corrupted by stub (len is: %d, expected maximum: %d)!",
            (*proc_data).syscall_data_len,
            (*mm_idp).syscall_data_len);
    }

    sc = (*proc_data).syscall_data.as_mut_ptr().add((*proc_data).syscall_data_len as usize);

    printk!(UM_KERN_ERR "{} : length = {}, last offset = {}",
        "syscall_stub_dump_error", (*mm_idp).syscall_data_len,
        (*proc_data).syscall_data_len);
    printk!(UM_KERN_ERR "{} : stub syscall type {} failed, return value = 0x{:lx}\n",
        "syscall_stub_dump_error", (*sc).syscall, (*proc_data).err);

    print_hex_dump!(UM_KERN_ERR, "    syscall data: ", 0,
        16, 4, sc as *const _, size_of::<stub_syscall>(), 0);

    if using_seccomp {
        printk!(UM_KERN_ERR "{}: FD map num: {}", "syscall_stub_dump_error",
            (*mm_idp).syscall_fd_num);
        print_hex_dump!(UM_KERN_ERR, "    FD map: ", 0, 16,
            size_of_val(&(*mm_idp).syscall_fd_map[0]),
            (*mm_idp).syscall_fd_map.as_ptr(),
            size_of_val(&(*mm_idp).syscall_fd_map), 0);
    }
}

#[inline]
unsafe fn check_init_stack(mm_idp: *mut mm_id, mut stack: *mut libc::c_ulong) -> *mut libc::c_ulong {
    if stack.is_null() {
        stack = ((*mm_idp).stack as *mut libc::c_ulong).add(2);
        *stack = 0;
    }
    stack
}

static mut syscall_regs: [libc::c_ulong; MAX_REG_NR] = [0; MAX_REG_NR];

unsafe fn init_syscall_regs() -> libc::c_int {
    get_safe_registers(syscall_regs.as_mut_ptr(), core::ptr::null_mut());

    syscall_regs[REGS_IP_INDEX] = STUB_CODE
        + (stub_syscall_handler as usize as libc::c_ulong
            - (&__syscall_stub_start as *const _ as usize as libc::c_ulong));
    syscall_regs[REGS_SP_INDEX] = STUB_DATA
        + offset_of!(stub_data, sigstack) as libc::c_ulong
        + size_of_val(&(*(core::ptr::null::<stub_data>())).sigstack) as libc::c_ulong
        - size_of::<*mut libc::c_void>() as libc::c_ulong;

    0
}

// Equivalent to __initcall(init_syscall_regs).
__initcall!(init_syscall_regs);

#[inline]
unsafe fn do_syscall_stub(mm_idp: *mut mm_id) -> libc::c_long {
    let proc_data = (*mm_idp).stack as *mut stub_data;
    let mut n: libc::c_int;
    let mut i: libc::c_int;
    let mut err: libc::c_int;
    let pid = (*mm_idp).pid;

    (*proc_data).syscall_data_len = (*mm_idp).syscall_data_len;

    if using_seccomp {
        (*proc_data).restart_wait = 1;
        wait_stub_done_seccomp(mm_idp, 0, 1);
    } else {
        n = ptrace_setregs(pid, syscall_regs.as_ptr());
        if n < 0 {
            printk!(UM_KERN_ERR "Registers -\n");
            for i in 0..MAX_REG_NR {
                printk!(UM_KERN_ERR "\t{}\t0x{:lx}\n", i, syscall_regs[i]);
            }
            panic!("{} : PTRACE_SETREGS failed, errno = {}\n", "do_syscall_stub", -n);
        }

        err = ptrace(PTRACE_CONT, pid, 0, 0);
        if err != 0 {
            panic!("Failed to continue stub, pid = {}, errno = {}\n", pid, errno);
        }

        wait_stub_done(pid);
    }

    if (*proc_data).err < 0 {
        syscall_stub_dump_error(mm_idp);
        (*mm_idp).syscall_data_len = (*proc_data).err;
    } else {
        (*mm_idp).syscall_data_len = 0;
    }

    if using_seccomp {
        (*mm_idp).syscall_fd_num = 0;
    }

    (*mm_idp).syscall_data_len as libc::c_long
}

pub unsafe fn syscall_stub_flush(mm_idp: *mut mm_id) -> libc::c_int {
    let mut res: libc::c_int;

    if (*mm_idp).syscall_data_len == 0 { return 0; }

    if (*mm_idp).syscall_data_len < 0 {
        res = (*mm_idp).syscall_data_len;
        (*mm_idp).syscall_data_len = 0;
        return res;
    }

    res = do_syscall_stub(mm_idp) as libc::c_int;
    (*mm_idp).syscall_data_len = 0;
    res
}

pub unsafe fn syscall_stub_alloc(mm_idp: *mut mm_id) -> *mut stub_syscall {
    let proc_data = (*mm_idp).stack as *mut stub_data;
    let sc: *mut stub_syscall;

    if (*mm_idp).syscall_data_len > 0
        && (*mm_idp).syscall_data_len as usize == (*proc_data).syscall_data.len()
    {
        do_syscall_stub(mm_idp);
    }

    if (*mm_idp).syscall_data_len < 0 {
        sc = (*proc_data).syscall_data.as_mut_ptr();
    } else {
        sc = (*proc_data).syscall_data.as_mut_ptr().add((*mm_idp).syscall_data_len as usize);
        (*mm_idp).syscall_data_len += 1;
    }
    core::ptr::write_bytes(sc, 0, 1);
    sc
}

unsafe fn syscall_stub_get_previous(mm_idp: *mut mm_id, syscall_type: libc::c_int,
                                    virt: libc::c_ulong) -> *mut stub_syscall {
    if (*mm_idp).syscall_data_len > 0 {
        let proc_data = (*mm_idp).stack as *mut stub_data;
        let sc = (*proc_data).syscall_data.as_mut_ptr()
            .add(((*mm_idp).syscall_data_len - 1) as usize);
        if (*sc).syscall == syscall_type
            && (*sc).mem.addr + (*sc).mem.length == virt
        { return sc; }
    }
    core::ptr::null_mut()
}

unsafe fn get_stub_fd(mm_idp: *mut mm_id, fd: libc::c_int) -> libc::c_int {
    if !using_seccomp { return fd; }
    if (*mm_idp).syscall_data_len < 0 { return 0; }
    if (*mm_idp).syscall_data_len as usize < (*mm_idp).syscall_data.len() {
        for i in 0..(*mm_idp).syscall_fd_num as usize {
            if (*mm_idp).syscall_fd_map[i] == fd { return i as libc::c_int; }
        }
        if (*mm_idp).syscall_fd_num < STUB_MAX_FDS {
            let i = (*mm_idp).syscall_fd_num;
            (*mm_idp).syscall_fd_map[i as usize] = fd;
            (*mm_idp).syscall_fd_num += 1;
            return i;
        }
    }
    do_syscall_stub(mm_idp);
    (*mm_idp).syscall_fd_map[0] = fd;
    (*mm_idp).syscall_fd_num = 1;
    0
}

pub unsafe fn map(mm_idp: *mut mm_id, virt: libc::c_ulong, len: libc::c_ulong,
                  prot: libc::c_int, mut phys_fd: libc::c_int,
                  offset: libc::c_ulonglong) -> libc::c_int {
    let mut sc = syscall_stub_get_previous(mm_idp, STUB_SYSCALL_MMAP, virt);
    if !sc.is_null() && (*sc).mem.prot == prot
        && (*sc).mem.offset == MMAP_OFFSET(offset - (*sc).mem.length as libc::c_ulonglong)
    {
        let mut prev_fd = (*sc).mem.fd;
        if using_seccomp { prev_fd = (*mm_idp).syscall_fd_map[prev_fd as usize]; }
        if phys_fd == prev_fd { (*sc).mem.length += len; return 0; }
    }
    phys_fd = get_stub_fd(mm_idp, phys_fd);
    sc = syscall_stub_alloc(mm_idp);
    (*sc).syscall = STUB_SYSCALL_MMAP;
    (*sc).mem.addr = virt; (*sc).mem.length = len; (*sc).mem.prot = prot;
    (*sc).mem.fd = phys_fd; (*sc).mem.offset = MMAP_OFFSET(offset);
    0
}

pub unsafe fn unmap(mm_idp: *mut mm_id, addr: libc::c_ulong,
                    len: libc::c_ulong) -> libc::c_int {
    let mut sc = syscall_stub_get_previous(mm_idp, STUB_SYSCALL_MUNMAP, addr);
    if !sc.is_null() { (*sc).mem.length += len; return 0; }
    sc = syscall_stub_alloc(mm_idp);
    (*sc).syscall = STUB_SYSCALL_MUNMAP;
    (*sc).mem.addr = addr; (*sc).mem.length = len;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
