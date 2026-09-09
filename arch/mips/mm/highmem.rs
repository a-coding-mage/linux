// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// linux/compiler.h, linux/init.h, linux/export.h, linux/highmem.h,
// linux/sched.h, linux/smp.h, asm/fixmap.h, asm/tlbflush.h

pub static mut highstart_pfn: ::core::ffi::c_ulong = 0;
pub static mut highend_pfn: ::core::ffi::c_ulong = 0;

unsafe extern "C" {
    pub fn flush_tlb_one(addr: ::core::ffi::c_ulong);
}

pub unsafe fn kmap_flush_tlb(addr: ::core::ffi::c_ulong) {
    unsafe {
        flush_tlb_one(addr);
    }
}

// EXPORT_SYMBOL(kmap_flush_tlb);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
