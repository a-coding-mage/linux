/* SPDX-License-Identifier: GPL-2.0 */
/*
 * x86 KMSAN support.
 *
 * Copyright (C) 2022, Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 */

// C header dependencies: <asm/cpu_entry_area.h>, <asm/processor.h>,
// <linux/mmzone.h>. This translation applies only when !MODULE.

use core::ffi::c_char;

extern "C" {
    pub static mut cpu_entry_area_shadow: [c_char; CPU_ENTRY_AREA_SIZE];
    pub static mut cpu_entry_area_origin: [c_char; CPU_ENTRY_AREA_SIZE];
}

/*
 * Functions below are declared in the header to make sure they are inlined.
 * They all are called from kmsan_get_metadata() for every memory access in
 * the kernel, so speed is important here.
 */

/*
 * Compute metadata addresses for the CPU entry area on x86.
 */
#[inline]
pub unsafe fn arch_kmsan_get_meta_or_null(
    addr: *mut core::ffi::c_void,
    is_origin: bool,
) -> *mut core::ffi::c_void {
    let addr64 = addr as usize;
    let metadata_array: *mut c_char;
    let off: usize;
    let cpu: i32;

    if addr64 < CPU_ENTRY_AREA_BASE
        || addr64 >= CPU_ENTRY_AREA_BASE.wrapping_add(CPU_ENTRY_AREA_MAP_SIZE)
    {
        return core::ptr::null_mut();
    }
    cpu = ((addr64 - CPU_ENTRY_AREA_BASE) / CPU_ENTRY_AREA_SIZE) as i32;
    off = addr64.wrapping_sub(get_cpu_entry_area(cpu) as usize);
    if off >= CPU_ENTRY_AREA_SIZE {
        return core::ptr::null_mut();
    }
    metadata_array = if is_origin {
        cpu_entry_area_origin.as_mut_ptr()
    } else {
        cpu_entry_area_shadow.as_mut_ptr()
    };
    per_cpu(metadata_array.add(off), cpu) as *mut core::ffi::c_void
}

/*
 * Taken from arch/x86/mm/physaddr.h to avoid using an instrumented version.
 */
#[inline]
pub unsafe fn kmsan_phys_addr_valid(addr: usize) -> bool {
    if IS_ENABLED_CONFIG_PHYS_ADDR_T_64BIT {
        !(addr >> boot_cpu_data.x86_phys_bits)
    } else {
        true
    }
}

/*
 * Taken from arch/x86/mm/physaddr.c to avoid using an instrumented version.
 */
#[inline]
pub unsafe fn kmsan_virt_addr_valid(addr: *mut core::ffi::c_void) -> bool {
    let mut x = addr as usize;
    let mut y = x.wrapping_sub(__START_KERNEL_map);
    let ret: bool;

    /* use the carry flag to determine if x was < __START_KERNEL_map */
    if unlikely(x > y) {
        x = y.wrapping_add(phys_base);

        if y >= KERNEL_IMAGE_SIZE {
            return false;
        }
    } else {
        x = y.wrapping_add(__START_KERNEL_map.wrapping_sub(PAGE_OFFSET));

        /* carry flag will be set if starting x was >= PAGE_OFFSET */
        if (x > y) || !kmsan_phys_addr_valid(x) {
            return false;
        }
    }

    /*
     * pfn_valid() relies on RCU, and may call into the scheduler on exiting
     * the critical section. However, this would result in recursion with
     * KMSAN. Therefore, disable preemption here, and re-enable preemption
     * below while suppressing reschedules to avoid recursion.
     *
     * Note, this sacrifices occasionally breaking scheduling guarantees.
     * Although, a kernel compiled with KMSAN has already given up on any
     * performance guarantees due to being heavily instrumented.
     */
    preempt_disable();
    ret = pfn_valid(x >> PAGE_SHIFT);
    preempt_enable_no_resched();

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
