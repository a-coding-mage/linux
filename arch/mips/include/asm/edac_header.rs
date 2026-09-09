/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <asm/compiler.h> is supplied by the surrounding build.

/// ECC atomic, DMA, SMP and interrupt safe scrub function.
#[inline(always)]
pub unsafe fn edac_atomic_scrub(va: *mut core::ffi::c_void, size: u32) {
    let mut virt_addr = va as *mut c_ulong;
    let mut temp: c_ulong;
    let mut i: u32 = 0;

    while i < size / core::mem::size_of::<c_ulong>() as u32 {
        /*
         * Very carefully read and write to memory atomically
         * so we are interrupt, DMA and SMP safe.
         *
         * Intel: asm!("lock; addl $0, {0}", inout(reg) *virt_addr);
         */
        core::arch::asm!(
            "\t.set\tpush",
            "\t.set\tmips2",
            "1:\tll\t{temp}, 0({addr})\t# edac_atomic_scrub",
            "\taddu\t{temp}, $0",
            "\tsc\t{temp}, 0({addr})",
            "\tbeqz\t{temp}, 1b",
            "\t.set\tpop",
            temp = lateout(reg) temp,
            addr = inout(reg) virt_addr => _,
            options(volatile)
        );

        virt_addr = virt_addr.add(1);
        i = i.wrapping_add(1);
    }
}

// C's unsigned long, used by the source header's target ABI.
type c_ulong = usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
