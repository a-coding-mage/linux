/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/* C header guard: _ASM_IO_H */
/* Dependencies: linux/kernel.h, linux/types.h, asm/addrspace.h,
 * asm/cpu.h, asm/page.h, asm/pgtable-bits.h, asm/string.h,
 * and asm-generic/io.h. */

extern "C" {
    pub fn early_ioremap(phys_addr: phys_addr_t, size: c_ulong) -> *mut core::ffi::c_void;
    pub fn early_iounmap(addr: *mut core::ffi::c_void, size: c_ulong);

    pub fn valid_phys_addr_range(addr: phys_addr_t, size: usize) -> c_int;
    pub fn valid_mmap_phys_addr_range(pfn: c_ulong, size: usize) -> c_int;
}

macro_rules! early_memremap {
    ($phys_addr:expr, $size:expr) => { early_ioremap($phys_addr, $size) };
}

macro_rules! early_memunmap {
    ($addr:expr, $size:expr) => { early_iounmap($addr, $size) };
}

/* CONFIG_ARCH_IOREMAP is a build-time condition from the C header. */
#[cfg(CONFIG_ARCH_IOREMAP)]
pub unsafe fn ioremap_prot(
    offset: phys_addr_t,
    _size: c_ulong,
    prot: pgprot_t,
) -> *mut core::ffi::c_void {
    if offset > TO_PHYS_MASK {
        return core::ptr::null_mut();
    }

    match pgprot_val(prot) & _CACHE_MASK {
        _CACHE_CC => (CACHE_BASE + offset) as c_ulong as *mut core::ffi::c_void,
        _CACHE_SUC => (UNCACHE_BASE + offset) as c_ulong as *mut core::ffi::c_void,
        _CACHE_WUC => (WRITECOMBINE_BASE + offset) as c_ulong as *mut core::ffi::c_void,
        _ => core::ptr::null_mut(),
    }
}

#[cfg(CONFIG_ARCH_IOREMAP)]
macro_rules! ioremap {
    ($offset:expr, $size:expr) => {
        unsafe { ioremap_prot($offset, $size, PAGE_KERNEL_SUC) }
    };
}

#[cfg(CONFIG_ARCH_IOREMAP)]
macro_rules! iounmap {
    ($addr:expr) => {{
        let _ = $addr;
    }};
}

/*
 * On LoongArch, ioremap() has two variants, ioremap_wc() and ioremap_cache().
 * They map bus memory into CPU space, the mapped memory is marked uncachable
 * (_CACHE_SUC), uncachable but accelerated by write-combine (_CACHE_WUC) and
 * cachable (_CACHE_CC) respectively for CPU access.
 *
 * @offset:    bus address of the memory
 * @size:      size of the resource to map
 */
macro_rules! ioremap_wc {
    ($offset:expr, $size:expr) => {
        unsafe { ioremap_prot($offset, $size, if wc_enabled { PAGE_KERNEL_WUC } else { PAGE_KERNEL_SUC }) }
    };
}

macro_rules! ioremap_cache {
    ($offset:expr, $size:expr) => {
        unsafe { ioremap_prot($offset, $size, PAGE_KERNEL) }
    };
}

macro_rules! mmiowb {
    () => { wmb() };
}

macro_rules! __io_aw {
    () => { mmiowb!() };
}

/* CONFIG_KFENCE is a build-time condition from the C header. */
#[cfg(CONFIG_KFENCE)]
extern "C" {
    static mut __kfence_pool: *mut core::ffi::c_char;
}

#[cfg(CONFIG_KFENCE)]
macro_rules! virt_to_phys {
    ($kaddr:expr) => {{
        if likely(($kaddr as c_ulong) < vm_map_base) {
            __pa($kaddr as c_ulong)
        } else {
            page_to_phys(tlb_virt_to_page($kaddr as c_ulong))
                + offset_in_page($kaddr as c_ulong)
        }
    }};
}

#[cfg(CONFIG_KFENCE)]
macro_rules! phys_to_virt {
    ($paddr:expr) => {{
        if unlikely(__kfence_pool.is_null()) {
            __va($paddr as c_ulong)
        } else {
            page_address(phys_to_page($paddr as c_ulong))
                .add(offset_in_page($paddr as c_ulong) as usize)
        }
    }};
}

/* #define ARCH_HAS_VALID_PHYS_ADDR_RANGE */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
