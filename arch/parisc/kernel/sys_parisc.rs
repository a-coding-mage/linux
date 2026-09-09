// SPDX-License-Identifier: GPL-2.0-or-later

/* PARISC specific syscalls. Direct translation of sys_parisc.c. */

// External kernel types, constants, globals, and functions are supplied by dependencies.

const O_NONBLOCK_OLD: i32 = 0o200004;

unsafe fn shared_align_offset(filp_pgoff: usize, pgoff: usize) -> usize {
    (filp_pgoff.wrapping_add(pgoff)).wrapping_shl(PAGE_SHIFT)
}

#[inline]
unsafe fn color_align(addr: usize, filp_pgoff: usize, pgoff: usize) -> usize {
    let base = addr.wrapping_add(SHM_COLOUR - 1) & !(SHM_COLOUR - 1);
    let off = (SHM_COLOUR - 1) & shared_align_offset(filp_pgoff, pgoff);
    base.wrapping_add(off)
}

#[cfg(feature = "CONFIG_COMPAT")]
const STACK_SIZE_DEFAULT: usize = if USER_WIDE_MODE {
    1usize << 30
} else {
    CONFIG_STACK_MAX_DEFAULT_SIZE_MB * 1024 * 1024
};

#[cfg(not(feature = "CONFIG_COMPAT"))]
const STACK_SIZE_DEFAULT: usize = 1usize << 30;

pub unsafe fn calc_max_stack_size(mut stack_max: usize) -> usize {
    #[cfg(feature = "CONFIG_COMPAT")]
    {
        if !USER_WIDE_MODE && stack_max == COMPAT_RLIM_INFINITY {
            stack_max = STACK_SIZE_DEFAULT;
        }
    }
    if stack_max == RLIM_INFINITY {
        stack_max = STACK_SIZE_DEFAULT;
    }
    stack_max
}

pub unsafe fn mmap_upper_limit(rlim_stack: *const rlimit) -> usize {
    let mut stack_base = if !rlim_stack.is_null() {
        (*rlim_stack).rlim_max as usize
    } else {
        rlimit_max(RLIMIT_STACK) as usize
    };
    stack_base = calc_max_stack_size(stack_base);
    if ((*current).flags & PF_RANDOMIZE) != 0 {
        stack_base = stack_base.wrapping_add((STACK_RND_MASK as usize).wrapping_shl(PAGE_SHIFT));
    }
    PAGE_ALIGN(STACK_TOP.wrapping_sub(stack_base))
}

#[repr(C)]
pub enum mmap_allocation_direction { UP, DOWN }

unsafe fn arch_get_unmapped_area_common(
    filp: *mut file, mut addr: usize, len: usize, pgoff: usize, flags: usize,
    dir: mmap_allocation_direction,
) -> usize {
    let mm = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let mut prev: *mut vm_area_struct = core::ptr::null_mut();
    let do_color_align = if !filp.is_null() || (flags & MAP_SHARED) != 0 { 1 } else { 0 };
    let filp_pgoff = if !filp.is_null() {
        (((*filp).f_mapping as usize) >> 8) & ((SHM_COLOUR - 1) >> PAGE_SHIFT)
    } else { 0 };
    let mut info = vm_unmapped_area_info { length: len, ..core::mem::zeroed() };

    if len > TASK_SIZE { return (-ENOMEM) as usize; }
    if (flags & MAP_FIXED) != 0 {
        if TASK_SIZE.wrapping_sub(len) < addr { return (-EINVAL) as usize; }
        if (flags & MAP_SHARED) != 0 && !filp.is_null()
            && ((addr.wrapping_sub(shared_align_offset(filp_pgoff, pgoff))) & (SHM_COLOUR - 1)) != 0 {
            return (-EINVAL) as usize;
        }
        return addr;
    }
    if addr != 0 {
        addr = if do_color_align != 0 { color_align(addr, filp_pgoff, pgoff) } else { PAGE_ALIGN(addr) };
        vma = find_vma_prev(mm, addr, &mut prev);
        if TASK_SIZE.wrapping_sub(len) >= addr
            && (vma.is_null() || addr.wrapping_add(len) <= vm_start_gap(vma))
            && (prev.is_null() || addr >= vm_end_gap(prev)) { return addr; }
    }
    info.align_mask = if do_color_align != 0 { PAGE_MASK & (SHM_COLOUR - 1) } else { 0 };
    info.align_offset = shared_align_offset(filp_pgoff, pgoff);
    if matches!(dir, mmap_allocation_direction::DOWN) {
        info.flags = VM_UNMAPPED_AREA_TOPDOWN;
        info.low_limit = PAGE_SIZE;
        info.high_limit = (*mm).mmap_base;
        addr = vm_unmapped_area(&info);
        if (addr & !PAGE_MASK) == 0 { return addr; }
        VM_BUG_ON(addr != (-ENOMEM) as usize);
    }
    info.low_limit = (*mm).mmap_base;
    info.high_limit = mmap_upper_limit(core::ptr::null());
    vm_unmapped_area(&info)
}

pub unsafe fn arch_get_unmapped_area(f: *mut file, a: usize, l: usize, p: usize, fl: usize, _vm: vm_flags_t) -> usize {
    arch_get_unmapped_area_common(f, a, l, p, fl, mmap_allocation_direction::UP)
}

pub unsafe fn arch_get_unmapped_area_topdown(f: *mut file, a: usize, l: usize, p: usize, fl: usize, _vm: vm_flags_t) -> usize {
    arch_get_unmapped_area_common(f, a, l, p, fl, mmap_allocation_direction::DOWN)
}

pub unsafe fn sys_mmap2(a: usize, l: usize, prot: usize, fl: usize, fd: usize, p: usize) -> usize {
    ksys_mmap_pgoff(a, l, prot, fl, fd, p >> (PAGE_SHIFT - 12))
}

pub unsafe fn sys_mmap(a: usize, l: usize, prot: usize, fl: usize, fd: usize, offset: usize) -> usize {
    if (offset & !PAGE_MASK) == 0 { ksys_mmap_pgoff(a, l, prot, fl, fd, offset >> PAGE_SHIFT) } else { (-EINVAL) as usize }
}

pub unsafe fn parisc_truncate64(path: *const i8, high: u32, low: u32) -> i64 { ksys_truncate(path, ((high as i64) << 32) | low as i64) }
pub unsafe fn parisc_ftruncate64(fd: u32, high: u32, low: u32) -> i64 { ksys_ftruncate(fd, ((high as i64) << 32) | low as i64, FTRUNCATE_LFS) }
pub unsafe fn sys_truncate64(path: *const i8, length: usize) -> i64 { ksys_truncate(path, length as i64) }
pub unsafe fn sys_ftruncate64(fd: u32, length: usize) -> i64 { ksys_ftruncate(fd, length as i64, FTRUNCATE_LFS) }
pub unsafe fn sys_fcntl64(fd: u32, cmd: u32, arg: usize) -> i64 { sys_fcntl(fd, cmd, arg) }
pub unsafe fn parisc_pread64(fd: u32, buf: *mut i8, count: usize, high: u32, low: u32) -> isize { ksys_pread64(fd, buf, count, ((high as i64) << 32) | low as i64) }
pub unsafe fn parisc_pwrite64(fd: u32, buf: *const i8, count: usize, high: u32, low: u32) -> isize { ksys_pwrite64(fd, buf, count, ((high as i64) << 32) | low as i64) }
pub unsafe fn parisc_readahead(fd: i32, high: u32, low: u32, count: usize) -> isize { ksys_readahead(fd, ((high as i64) << 32) | low as i64, count) }
pub unsafe fn parisc_fadvise64_64(fd: i32, ho: u32, lo: u32, hl: u32, ll: u32, advice: i32) -> i64 { ksys_fadvise64_64(fd, ((ho as i64) << 32) | lo as i64, ((hl as i64) << 32) | ll as i64, advice) }
pub unsafe fn parisc_sync_file_range(fd: i32, ho: u32, lo: u32, hn: u32, ln: u32, flags: u32) -> i64 { ksys_sync_file_range(fd, ((ho as i64) << 32) | lo as i64, ((hn as i64) << 32) | ln as i64, flags) }
pub unsafe fn parisc_fallocate(fd: i32, mode: i32, oh: u32, ol: u32, lh: u32, ll: u32) -> i64 { ksys_fallocate(fd, mode, ((oh as u64) << 32) | ol as u64, ((lh as u64) << 32) | ll as u64) }

pub unsafe fn parisc_personality(mut p: usize) -> i64 {
    let mut err;
    if personality((*current).personality) == PER_LINUX32 && personality(p) == PER_LINUX { p = (p & !PER_MASK) | PER_LINUX32; }
    err = sys_personality(p);
    if personality(err as usize) == PER_LINUX32 { err = (err & !(PER_MASK as i64)) | PER_LINUX as i64; }
    err
}

const O_NONBLOCK_MASK_OUT: i32 = O_NONBLOCK_OLD & !O_NONBLOCK;
unsafe fn FIX_O_NONBLOCK(mut flags: i32) -> i32 {
    if (flags & O_NONBLOCK_MASK_OUT) != 0 && !test_thread_flag(TIF_NONBLOCK_WARNING) {
        set_thread_flag(TIF_NONBLOCK_WARNING);
        pr_warn("%s(%d) uses a deprecated O_NONBLOCK value. Please recompile with newer glibc.\n", (*current).comm, (*current).pid);
    }
    flags & !O_NONBLOCK_MASK_OUT
}
pub unsafe fn parisc_timerfd_create(c: i32, f: i32) -> i64 { sys_timerfd_create(c, FIX_O_NONBLOCK(f)) }
pub unsafe fn parisc_signalfd4(u: i32, m: *mut sigset_t, s: usize, f: i32) -> i64 { sys_signalfd4(u, m, s, FIX_O_NONBLOCK(f)) }
pub unsafe fn parisc_compat_signalfd4(u: i32, m: *mut compat_sigset_t, s: compat_size_t, f: i32) -> i64 { compat_sys_signalfd4(u, m, s, FIX_O_NONBLOCK(f)) }
pub unsafe fn parisc_eventfd2(c: u32, f: i32) -> i64 { sys_eventfd2(c, FIX_O_NONBLOCK(f)) }
pub unsafe fn parisc_userfaultfd(f: i32) -> i64 { sys_userfaultfd(FIX_O_NONBLOCK(f)) }
pub unsafe fn parisc_pipe2(f: *mut i32, fl: i32) -> i64 { sys_pipe2(f, FIX_O_NONBLOCK(fl)) }
pub unsafe fn parisc_inotify_init1(f: i32) -> i64 { sys_inotify_init1(FIX_O_NONBLOCK(f)) }

pub unsafe fn parisc_madvise(start: usize, len_in: usize, mut behavior: i32) -> i64 {
    behavior = match behavior { 65 => MADV_MERGEABLE, 66 => MADV_UNMERGEABLE, 67 => MADV_HUGEPAGE, 68 => MADV_NOHUGEPAGE, 69 => MADV_DONTDUMP, 70 => MADV_DODUMP, 71 => MADV_WIPEONFORK, 72 => MADV_KEEPONFORK, 73 => MADV_COLLAPSE, x => x };
    sys_madvise(start, len_in, behavior)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
