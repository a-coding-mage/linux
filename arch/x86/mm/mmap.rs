// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Flexible mmap layout support
 *
 * Based on code by Ingo Molnar and Andi Kleen, copyrighted
 * as follows:
 *
 * Copyright 2003-2009 Red Hat Inc.
 * All Rights Reserved.
 * Copyright 2005 Andi Kleen, SUSE Labs.
 * Copyright 2007 Jiri Kosina, SUSE Labs.
 */

// Linux and architecture dependencies are supplied by other translation units.

#[repr(C)]
pub struct VaAlignment {
    pub flags: i32,
}

#[no_mangle]
pub static mut va_align: VaAlignment = VaAlignment { flags: -1 };

#[no_mangle]
pub unsafe extern "C" fn task_size_32bit() -> ::core::ffi::c_ulong {
    IA32_PAGE_OFFSET
}

#[no_mangle]
pub unsafe extern "C" fn task_size_64bit(full_addr_space: ::core::ffi::c_int) -> ::core::ffi::c_ulong {
    if full_addr_space != 0 { TASK_SIZE_MAX } else { DEFAULT_MAP_WINDOW }
}

unsafe fn stack_maxrandom_size(task_size: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let mut max: ::core::ffi::c_ulong = 0;
    if (*current).flags & PF_RANDOMIZE != 0 {
        max = (!0 as ::core::ffi::c_ulong) & __STACK_RND_MASK(task_size == task_size_32bit());
        max = max << PAGE_SHIFT;
    }
    max
}

#[cfg(feature = "CONFIG_COMPAT")]
const mmap32_rnd_bits: ::core::ffi::c_uint = mmap_rnd_compat_bits;
#[cfg(feature = "CONFIG_COMPAT")]
const mmap64_rnd_bits: ::core::ffi::c_uint = mmap_rnd_bits;
#[cfg(not(feature = "CONFIG_COMPAT"))]
const mmap32_rnd_bits: ::core::ffi::c_uint = mmap_rnd_bits;
#[cfg(not(feature = "CONFIG_COMPAT"))]
const mmap64_rnd_bits: ::core::ffi::c_uint = mmap_rnd_bits;

const SIZE_128M: ::core::ffi::c_ulong = 128 * 1024 * 1024;

unsafe fn mmap_is_legacy() -> ::core::ffi::c_int {
    if (*current).personality & ADDR_COMPAT_LAYOUT != 0 { 1 } else { sysctl_legacy_va_layout }
}

unsafe fn arch_rnd(rndbits: ::core::ffi::c_uint) -> ::core::ffi::c_ulong {
    if (*current).flags & PF_RANDOMIZE == 0 { return 0; }
    (get_random_long() & ((1 as ::core::ffi::c_ulong << rndbits) - 1)) << PAGE_SHIFT
}

#[no_mangle]
pub unsafe extern "C" fn arch_mmap_rnd() -> ::core::ffi::c_ulong {
    arch_rnd(if mmap_is_ia32() { mmap32_rnd_bits } else { mmap64_rnd_bits })
}

unsafe fn mmap_base(rnd: ::core::ffi::c_ulong, task_size: ::core::ffi::c_ulong,
                    rlim_stack: *const Rlimit) -> ::core::ffi::c_ulong {
    let mut gap = (*rlim_stack).rlim_cur;
    let pad = stack_maxrandom_size(task_size) + stack_guard_gap;
    if gap + pad > gap { gap += pad; }
    gap = clamp(gap, SIZE_128M, (task_size / 6) * 5);
    PAGE_ALIGN(task_size - gap - rnd)
}

unsafe fn mmap_legacy_base(rnd: ::core::ffi::c_ulong, task_size: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    __TASK_UNMAPPED_BASE(task_size) + rnd
}

unsafe fn arch_pick_mmap_base(base: *mut ::core::ffi::c_ulong, legacy_base: *mut ::core::ffi::c_ulong,
                              random_factor: ::core::ffi::c_ulong, task_size: ::core::ffi::c_ulong,
                              rlim_stack: *const Rlimit) {
    *legacy_base = mmap_legacy_base(random_factor, task_size);
    if mmap_is_legacy() != 0 { *base = *legacy_base; }
    else { *base = mmap_base(random_factor, task_size, rlim_stack); }
}

#[no_mangle]
pub unsafe extern "C" fn arch_pick_mmap_layout(mm: *mut MmStruct, rlim_stack: *const Rlimit) {
    if mmap_is_legacy() != 0 { mm_flags_clear(MMF_TOPDOWN, mm); }
    else { mm_flags_set(MMF_TOPDOWN, mm); }
    arch_pick_mmap_base(&mut (*mm).mmap_base, &mut (*mm).mmap_legacy_base,
                        arch_rnd(mmap64_rnd_bits), task_size_64bit(0), rlim_stack);
    #[cfg(feature = "CONFIG_HAVE_ARCH_COMPAT_MMAP_BASES")]
    arch_pick_mmap_base(&mut (*mm).mmap_compat_base, &mut (*mm).mmap_compat_legacy_base,
                        arch_rnd(mmap32_rnd_bits), task_size_32bit(), rlim_stack);
}

#[no_mangle]
pub unsafe extern "C" fn get_mmap_base(is_legacy: ::core::ffi::c_int) -> ::core::ffi::c_ulong {
    let mm = (*current).mm;
    #[cfg(feature = "CONFIG_HAVE_ARCH_COMPAT_MMAP_BASES")]
    if in_32bit_syscall() {
        return if is_legacy != 0 { (*mm).mmap_compat_legacy_base } else { (*mm).mmap_compat_base };
    }
    if is_legacy != 0 { (*mm).mmap_legacy_base } else { (*mm).mmap_base }
}

#[no_mangle]
pub unsafe extern "C" fn mmap_address_hint_valid(addr: ::core::ffi::c_ulong, len: ::core::ffi::c_ulong) -> bool {
    if TASK_SIZE - len < addr { return false; }
    (addr > DEFAULT_MAP_WINDOW) == (addr + len > DEFAULT_MAP_WINDOW)
}

#[no_mangle]
pub unsafe extern "C" fn valid_phys_addr_range(addr: PhysAddr, count: usize) -> ::core::ffi::c_int {
    if addr + count as PhysAddr - 1 <= __pa(high_memory - 1) { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn valid_mmap_phys_addr_range(pfn: ::core::ffi::c_ulong, count: usize) -> ::core::ffi::c_int {
    let addr: PhysAddr = (pfn as PhysAddr) << PAGE_SHIFT;
    if phys_addr_valid(addr + count as PhysAddr - 1) { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn pfn_modify_allowed(pfn: ::core::ffi::c_ulong, prot: Pgprot) -> bool {
    if !boot_cpu_has_bug(X86_BUG_L1TF) { return true; }
    if !__pte_needs_invert(pgprot_val(prot)) { return true; }
    if pfn_valid(pfn) { return true; }
    if pfn >= l1tf_pfn_limit() && !capable(CAP_SYS_ADMIN) { return false; }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
