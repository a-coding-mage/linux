// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Implementation of various system calls for Linux/PowerPC.
 *
 * This is a direct Rust translation of the corresponding C implementation.
 * Kernel-provided declarations and macros referenced below are supplied by
 * other translation units.
 */

unsafe fn do_mmap2(
    addr: libc::c_ulong,
    len: usize,
    prot: libc::c_ulong,
    flags: libc::c_ulong,
    fd: libc::c_ulong,
    off: libc::c_ulong,
    shift: libc::c_int,
) -> libc::c_long {
    if !arch_validate_prot(prot, addr) {
        return -(EINVAL as libc::c_long);
    }

    if !IS_ALIGNED(off, 1usize << shift) {
        return -(EINVAL as libc::c_long);
    }

    ksys_mmap_pgoff(addr, len, prot, flags, fd, off >> shift)
}

#[no_mangle]
pub unsafe extern "C" fn mmap2(
    addr: libc::c_ulong,
    len: usize,
    prot: libc::c_ulong,
    flags: libc::c_ulong,
    fd: libc::c_ulong,
    pgoff: libc::c_ulong,
) -> libc::c_long {
    do_mmap2(addr, len, prot, flags, fd, pgoff, PAGE_SHIFT - 12)
}

#[cfg(CONFIG_COMPAT)]
#[no_mangle]
pub unsafe extern "C" fn compat_mmap2(
    addr: libc::c_ulong,
    len: usize,
    prot: libc::c_ulong,
    flags: libc::c_ulong,
    fd: libc::c_ulong,
    off_4k: libc::c_ulong,
) -> libc::c_long {
    do_mmap2(addr, len, prot, flags, fd, off_4k, PAGE_SHIFT - 12)
}

#[no_mangle]
pub unsafe extern "C" fn mmap(
    addr: libc::c_ulong,
    len: usize,
    prot: libc::c_ulong,
    flags: libc::c_ulong,
    fd: libc::c_ulong,
    offset: libc::off_t,
) -> libc::c_long {
    do_mmap2(addr, len, prot, flags, fd, offset as libc::c_ulong, PAGE_SHIFT)
}

#[cfg(CONFIG_PPC64)]
unsafe fn do_ppc64_personality(personality_arg: libc::c_ulong) -> libc::c_long {
    let mut personality_value = personality_arg;
    let mut ret: libc::c_long;

    if personality(current.personality) == PER_LINUX32
        && personality(personality_value) == PER_LINUX
    {
        personality_value = (personality_value & !PER_MASK) | PER_LINUX32;
    }
    ret = ksys_personality(personality_value);
    if personality(ret as libc::c_ulong) == PER_LINUX32 {
        ret = (ret & !(PER_MASK as libc::c_long)) | (PER_LINUX as libc::c_long);
    }
    ret
}

#[cfg(CONFIG_PPC64)]
#[no_mangle]
pub unsafe extern "C" fn ppc64_personality(personality_arg: libc::c_ulong) -> libc::c_long {
    do_ppc64_personality(personality_arg)
}

#[cfg(all(CONFIG_PPC64, CONFIG_COMPAT))]
#[no_mangle]
pub unsafe extern "C" fn compat_ppc64_personality(
    personality_arg: libc::c_ulong,
) -> libc::c_long {
    do_ppc64_personality(personality_arg)
}

#[no_mangle]
pub unsafe extern "C" fn ppc_fadvise64_64(
    fd: libc::c_int,
    advice: libc::c_int,
    offset_high: u32,
    offset_low: u32,
    len_high: u32,
    len_low: u32,
) -> libc::c_long {
    ksys_fadvise64_64(
        fd,
        merge_64(offset_high, offset_low),
        merge_64(len_high, len_low),
        advice,
    )
}

#[no_mangle]
pub unsafe extern "C" fn switch_endian() -> libc::c_long {
    let ti: *mut thread_info;

    regs_set_return_msr((*current).thread.regs, (*(*current).thread.regs).msr ^ MSR_LE);

    /*
     * Set TIF_RESTOREALL so that r3 isn't clobbered on return to
     * userspace. That also has the effect of restoring the non-volatile
     * GPRs, so we saved them on the way in here.
     */
    ti = current_thread_info();
    (*ti).flags |= _TIF_RESTOREALL;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
