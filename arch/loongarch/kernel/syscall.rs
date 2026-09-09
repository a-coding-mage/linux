// SPDX-License-Identifier: GPL-2.0+
/*
 * Author: Hanlu Li <lihanlu@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn ksys_mmap_pgoff(
        addr: libc::c_ulong,
        len: libc::c_ulong,
        prot: libc::c_ulong,
        flags: libc::c_ulong,
        fd: libc::c_ulong,
        pgoff: libc::c_ulong,
    ) -> libc::c_long;
    fn sys_ni_syscall() -> libc::c_long;
    fn syscall_enter_from_user_mode_randomize_stack(
        regs: *mut pt_regs,
        nr: *mut libc::c_ulong,
    ) -> bool;
    fn syscall_exit_to_user_mode(regs: *mut pt_regs);
}

#[repr(C)]
pub struct pt_regs {
    pub regs: [libc::c_ulong; 32],
    pub csr_era: libc::c_ulong,
    pub orig_a0: libc::c_ulong,
}

pub const EINVAL: libc::c_long = 22;
pub const ENOSYS: libc::c_long = 38;

// PAGE_MASK, PAGE_SHIFT, NR_syscalls, and __NR_syscalls are supplied by the target.
extern "C" {
    static mut PAGE_MASK: libc::c_ulong;
    static PAGE_SHIFT: libc::c_uint;
    static NR_syscalls: libc::c_ulong;
    static __NR_syscalls: libc::c_ulong;
}

pub unsafe fn sys_mmap(
    addr: libc::c_ulong,
    len: libc::c_ulong,
    prot: libc::c_ulong,
    flags: libc::c_ulong,
    fd: libc::c_ulong,
    offset: libc::c_ulong,
) -> libc::c_long {
    if offset & !PAGE_MASK != 0 {
        return -EINVAL;
    }

    ksys_mmap_pgoff(addr, len, prot, flags, fd, offset >> PAGE_SHIFT)
}

pub unsafe fn sys_mmap2(
    addr: libc::c_ulong,
    len: libc::c_ulong,
    prot: libc::c_ulong,
    flags: libc::c_ulong,
    fd: libc::c_ulong,
    offset: libc::c_ulong,
) -> libc::c_long {
    if offset & (!PAGE_MASK >> 12) != 0 {
        return -EINVAL;
    }

    ksys_mmap_pgoff(addr, len, prot, flags, fd, offset >> (PAGE_SHIFT - 12))
}

pub type sys_call_fn = unsafe extern "C" fn(
    libc::c_ulong,
    libc::c_ulong,
    libc::c_ulong,
    libc::c_ulong,
    libc::c_ulong,
    libc::c_ulong,
) -> libc::c_long;

// The architecture-specific syscall table entries are supplied by the target.
#[no_mangle]
pub static mut sys_call_table: *mut sys_call_fn = core::ptr::null_mut();

pub unsafe fn do_syscall(regs: *mut pt_regs) {
    let mut syscall_fn: sys_call_fn;
    let mut nr: libc::c_ulong;

    nr = (*regs).regs[11];
    /* Set for syscall restarting */
    if nr < NR_syscalls {
        (*regs).regs[0] = nr + 1;
    }

    (*regs).csr_era += 4;
    (*regs).orig_a0 = (*regs).regs[4];
    (*regs).regs[4] = (-ENOSYS) as libc::c_ulong;

    if syscall_enter_from_user_mode_randomize_stack(regs, &mut nr) {
        if nr < NR_syscalls {
            syscall_fn = *sys_call_table.add(nr as usize);
            (*regs).regs[4] = syscall_fn(
                (*regs).orig_a0,
                (*regs).regs[5],
                (*regs).regs[6],
                (*regs).regs[7],
                (*regs).regs[8],
                (*regs).regs[9],
            ) as libc::c_ulong;
        }
    }

    syscall_exit_to_user_mode(regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
