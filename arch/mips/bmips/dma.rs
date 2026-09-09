// SPDX-License-Identifier: GPL-2.0+

// Dependencies supplied by the Linux kernel and architecture-specific headers
// are intentionally left as external symbols.

pub static mut bmips_rac_flush_disable: bool = false;

pub unsafe fn arch_sync_dma_for_cpu_all() {
    let cbr = bmips_cbr_addr;
    let mut cfg: u32;

    if boot_cpu_type() != CPU_BMIPS3300
        && boot_cpu_type() != CPU_BMIPS4350
        && boot_cpu_type() != CPU_BMIPS4380
    {
        return;
    }

    if bmips_rac_flush_disable {
        return;
    }

    /* Flush stale data out of the readahead cache */
    cfg = core::ptr::read_volatile(
        (cbr as *const u8).add(BMIPS_RAC_CONFIG as usize) as *const u32,
    );
    core::ptr::write_volatile(
        (cbr as *mut u8).add(BMIPS_RAC_CONFIG as usize) as *mut u32,
        cfg | 0x100,
    );
    core::ptr::read_volatile(
        (cbr as *const u8).add(BMIPS_RAC_CONFIG as usize) as *const u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
