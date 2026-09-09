/* SPDX-License-Identifier: GPL-2.0 */

/* ECC atomic, DMA, SMP and interrupt safe scrub function */

/// Very carefully read and write to memory atomically so the operation is
/// interrupt, DMA and SMP safe.
#[inline]
pub unsafe fn edac_atomic_scrub(va: *mut core::ffi::c_void, size: u32) {
    let mut virt_addr = va as *mut u32;

    for _i in 0..(size / 4) {
        core::arch::asm!(
            "lock addl $0, [{addr}]",
            addr = in(reg) virt_addr,
            options(nostack)
        );
        virt_addr = virt_addr.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
