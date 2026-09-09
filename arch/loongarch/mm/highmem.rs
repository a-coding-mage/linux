// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel headers:
// - linux/init.h
// - linux/export.h
// - linux/highmem.h
// - asm/fixmap.h
// - asm/tlbflush.h

extern "C" {
    fn flush_tlb_one(addr: usize);
}

pub unsafe fn kmap_flush_tlb(addr: usize) {
    unsafe {
        flush_tlb_one(addr);
    }
}

// EXPORT_SYMBOL(kmap_flush_tlb);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
