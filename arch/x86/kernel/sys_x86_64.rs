// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the corresponding Linux kernel headers are referenced
// below but are not defined in this translation unit.

/*
 * Align a virtual address to avoid aliasing in the I$ on AMD F15h.
 */
unsafe fn get_align_mask(filp: *mut file) -> c_ulong {
    if !filp.is_null() && is_file_hugepages(filp) {
        return huge_page_mask_align(filp);
    }
    // handle 32- and 64-bit case with a single conditional
    if va_align.flags < 0 || (va_align.flags & (2 - mmap_is_ia32())) == 0 {
        return 0;
    }

    if ((*current).flags & PF_RANDOMIZE) == 0 {
        return 0;
    }

    va_align.mask
}

/*
 * To avoid aliasing in the I$ on AMD F15h, the bits defined by the
 * va_align.bits, [12:upper_bit), are set to a random value instead of
 * zeroing them. This random value is computed once per boot. This form of
 * ASLR is known as "per-boot ASLR".
 *
 * To achieve this, the random value is added to the info.align_offset
 * value before calling vm_unmapped_area() or ORed directly to the address.
 */
unsafe fn get_align_bits() -> c_ulong {
    va_align.bits & get_align_mask(core::ptr::null_mut())
}

unsafe extern "C" fn control_va_addr_alignment(str_: *mut c_char) -> c_int {
    // guard against enabling this on other CPU families
    if va_align.flags < 0 {
        return 1;
    }

    if *str_ == 0 {
        return 1;
    }

    if strcmp(str_, c"32".as_ptr()) == 0 {
        va_align.flags = ALIGN_VA_32;
    } else if strcmp(str_, c"64".as_ptr()) == 0 {
        va_align.flags = ALIGN_VA_64;
    } else if strcmp(str_, c"off".as_ptr()) == 0 {
        va_align.flags = 0;
    } else if strcmp(str_, c"on".as_ptr()) == 0 {
        va_align.flags = ALIGN_VA_32 | ALIGN_VA_64;
    } else {
        pr_warn!("invalid option value: 'align_va_addr=%s'\\n", str_);
    }

    1
}

// __setup("align_va_addr=", control_va_addr_alignment);

pub unsafe extern "C" fn mmap(
    addr: c_ulong,
    len: c_ulong,
    prot: c_ulong,
    flags: c_ulong,
    fd: c_ulong,
    off: c_ulong,
) -> c_long {
    if (off & !PAGE_MASK) != 0 {
        return -(EINVAL as c_long);
    }

    ksys_mmap_pgoff(addr, len, prot, flags, fd, off >> PAGE_SHIFT)
}

unsafe fn find_start_end(
    addr: c_ulong,
    flags: c_ulong,
    begin: *mut c_ulong,
    end: *mut c_ulong,
) {
    if !in_32bit_syscall() && (flags & MAP_32BIT) != 0 {
        // This is usually used needed to map code in small model, so it needs
        // to be in the first 31bit. Limit it to that. This means we need to
        // move the unmapped base down for this case. This can give conflicts
        // with the heap, but we assume that glibc malloc knows how to fall
        // back to mmap. Give it 1GB of playground for now. -AK
        *begin = 0x40000000;
        *end = 0x80000000;
        if ((*current).flags & PF_RANDOMIZE) != 0 {
            *begin = randomize_page(*begin, 0x02000000);
        }
        return;
    }

    *begin = get_mmap_base(1);
    if in_32bit_syscall() {
        *end = task_size_32bit();
    } else {
        *end = task_size_64bit(addr > DEFAULT_MAP_WINDOW);
    }
}

#[inline]
unsafe fn stack_guard_placement(vm_flags: vm_flags_t) -> c_ulong {
    if (vm_flags & VM_SHADOW_STACK) != 0 {
        return PAGE_SIZE;
    }

    0
}

pub unsafe extern "C" fn arch_get_unmapped_area(
    filp: *mut file,
    mut addr: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
    vm_flags: vm_flags_t,
) -> c_ulong {
    let mm = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let mut info: vm_unmapped_area_info = core::mem::zeroed();
    let (mut begin, mut end): (c_ulong, c_ulong) = (0, 0);

    if (flags & MAP_FIXED) != 0 {
        return addr;
    }

    find_start_end(addr, flags, &mut begin, &mut end);

    if len > end {
        return (-ENOMEM) as c_ulong;
    }

    if addr != 0 {
        addr = PAGE_ALIGN(addr);
        vma = find_vma(mm, addr);
        if end - len >= addr && (vma.is_null() || addr + len <= vm_start_gap(vma)) {
            return addr;
        }
    }

    info.length = len;
    info.low_limit = begin;
    info.high_limit = end;
    if filp.is_null() || !is_file_hugepages(filp) {
        info.align_offset = pgoff << PAGE_SHIFT;
        info.start_gap = stack_guard_placement(vm_flags);
    }
    if !filp.is_null() {
        info.align_mask = get_align_mask(filp);
        info.align_offset += get_align_bits();
    }

    vm_unmapped_area(&info)
}

pub unsafe extern "C" fn arch_get_unmapped_area_topdown(
    filp: *mut file,
    addr0: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
    vm_flags: vm_flags_t,
) -> c_ulong {
    let mut vma: *mut vm_area_struct;
    let mm = (*current).mm;
    let mut addr = addr0;
    let mut info: vm_unmapped_area_info = core::mem::zeroed();

    // requested length too big for entire address space
    if len > TASK_SIZE {
        return (-ENOMEM) as c_ulong;
    }

    // No address checking. See comment at mmap_address_hint_valid()
    if (flags & MAP_FIXED) != 0 {
        return addr;
    }

    // for MAP_32BIT mappings we force the legacy mmap base
    if !in_32bit_syscall() && (flags & MAP_32BIT) != 0 {
        return arch_get_unmapped_area(filp, addr0, len, pgoff, flags, 0);
    }

    // requesting a specific address
    if addr != 0 {
        addr &= PAGE_MASK;
        if !mmap_address_hint_valid(addr, len) {
            addr = 0;
        } else {
            vma = find_vma(mm, addr);
            if vma.is_null() || addr + len <= vm_start_gap(vma) {
                return addr;
            }
        }
    }

    info.flags = VM_UNMAPPED_AREA_TOPDOWN;
    info.length = len;
    if !in_32bit_syscall() && (flags & MAP_ABOVE4G) != 0 {
        info.low_limit = SZ_4G;
    } else {
        info.low_limit = PAGE_SIZE;
    }
    info.high_limit = get_mmap_base(0);
    if filp.is_null() || !is_file_hugepages(filp) {
        info.start_gap = stack_guard_placement(vm_flags);
        info.align_offset = pgoff << PAGE_SHIFT;
    }

    // If hint address is above DEFAULT_MAP_WINDOW, look for unmapped area in
    // the full address space. !in_32bit_syscall() avoids high addresses for x32.
    if addr > DEFAULT_MAP_WINDOW && !in_32bit_syscall() {
        info.high_limit += TASK_SIZE_MAX - DEFAULT_MAP_WINDOW;
    }
    if !filp.is_null() {
        info.align_mask = get_align_mask(filp);
        info.align_offset += get_align_bits();
    }
    addr = vm_unmapped_area(&info);
    if (addr & !PAGE_MASK) == 0 {
        return addr;
    }
    VM_BUG_ON(addr != (-ENOMEM as c_ulong));

    // A failed mmap() very likely causes application failure, so fall back to
    // the bottom-up function here. This can happen with large stack limits and
    // large mmap() allocations.
    arch_get_unmapped_area(filp, addr0, len, pgoff, flags, 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
