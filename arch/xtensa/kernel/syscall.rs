/*
 * arch/xtensa/kernel/syscall.c
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 * Copyright (C) 2000 Silicon Graphics, Inc.
 * Copyright (C) 1995 - 2000 by Ralf Baechle
 *
 * Joe Taylor <joe@tensilica.com, joetylr@yahoo.com>
 * Marc Gauthier <marc@tensilica.com, marc@alumni.uwaterloo.ca>
 * Chris Zankel <chris@zankel.net>
 * Kevin Chea
 *
 */

// C dependencies supplied by the surrounding kernel translation.

pub static mut sys_call_table: [syscall_t; 0] = [];

#[inline]
const fn colour_align(addr: usize, pgoff: usize) -> usize {
    (((addr + SHMLBA - 1) & !(SHMLBA - 1))
        + (((pgoff << PAGE_SHIFT) & (SHMLBA - 1))))
}

pub unsafe fn xtensa_shmat(
    shmid: ::core::ffi::c_int,
    shmaddr: *mut ::core::ffi::c_char,
    shmflg: ::core::ffi::c_int,
) -> ::core::ffi::c_long {
    let mut ret: ::core::ffi::c_ulong = 0;
    let err: ::core::ffi::c_long = do_shmat(shmid, shmaddr, shmflg, &mut ret, SHMLBA);
    if err != 0 {
        return err;
    }
    ret as ::core::ffi::c_long
}

pub unsafe fn xtensa_fadvise64_64(
    fd: ::core::ffi::c_int,
    advice: ::core::ffi::c_int,
    offset: u64,
    len: u64,
) -> ::core::ffi::c_long {
    ksys_fadvise64_64(fd, offset, len, advice)
}

#[cfg(CONFIG_MMU)]
pub unsafe fn arch_get_unmapped_area(
    filp: *mut file,
    mut addr: usize,
    len: usize,
    pgoff: usize,
    flags: usize,
    vm_flags: vm_flags_t,
) -> usize {
    let mut vmm: *mut vm_area_struct;
    let mut vmi: vma_iterator;

    if flags & MAP_FIXED != 0 {
        /* We do not accept a shared mapping if it would violate
         * cache aliasing constraints.
         */
        if flags & MAP_SHARED != 0
            && ((addr.wrapping_sub(pgoff << PAGE_SHIFT)) & (SHMLBA - 1)) != 0
        {
            return (-EINVAL) as usize;
        }
        return addr;
    }

    if len > TASK_SIZE {
        return (-ENOMEM) as usize;
    }
    if addr == 0 {
        addr = TASK_UNMAPPED_BASE;
    }

    if flags & MAP_SHARED != 0 {
        addr = colour_align(addr, pgoff);
    } else {
        addr = PAGE_ALIGN(addr);
    }

    vma_iter_init(&mut vmi, (*current).mm, addr);
    for_each_vma!(vmi, vmm, {
        /* At this point:  (addr < vmm->vm_end). */
        if addr.wrapping_add(len) <= vm_start_gap(vmm) {
            break;
        }

        addr = (*vmm).vm_end;
        if flags & MAP_SHARED != 0 {
            addr = colour_align(addr, pgoff);
        }
    });

    if TASK_SIZE - len < addr {
        return (-ENOMEM) as usize;
    }

    addr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
