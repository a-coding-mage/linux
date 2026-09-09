/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TLB flush support for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// C dependencies: linux/mm.h and asm/processor.h.

/*
 * TLB flushing -- in "SMP", these routines get defined to be the
 * ones from smp.c, else they are some local flavors.
 */

/*
 * These functions are commonly macros, but in the interests of
 * VM vs. native implementation and code size, we simply declare
 * the function prototypes here.
 */

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

extern "C" {
    pub fn tlb_flush_all();
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    pub fn flush_tlb_page(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong);
    pub fn flush_tlb_range(
        vma: *mut vm_area_struct,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    pub fn flush_tlb_kernel_range(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    pub fn flush_tlb_one(addr: ::core::ffi::c_ulong);
}

/*
 * "This is called in munmap when we have freed up some page-table pages.
 * We don't need to do anything here..."
 *
 * The VM kernel doesn't walk page tables, and they are passed to the VMM
 * by logical address. There doesn't seem to be any possibility that they
 * could be referenced by the VM kernel based on a stale mapping, since
 * they would only be located by consulting the mm structure, and they
 * will have been purged from that structure by the munmap.  Seems like
 * a noop on HVM as well.
 */
#[macro_export]
macro_rules! flush_tlb_pgtables {
    ($mm:expr, $start:expr, $end:expr) => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
