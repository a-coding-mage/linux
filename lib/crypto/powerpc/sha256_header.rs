/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-256 Secure Hash Algorithm, SPE optimized
 *
 * Based on generic implementation. The assembler module takes care
 * about the SPE registers so it can run from interrupt context.
 *
 * Copyright (c) 2015 Markus Stockhausen <stockhausen@collogia.de>
 */

// C dependencies supplied by other translation units:
// asm/switch_to.h and linux/preempt.h

/*
 * MAX_BYTES defines the number of bytes that are allowed to be processed
 * between preempt_disable() and preempt_enable(). SHA256 takes ~2,000
 * operations per 64 bytes. e500 cores can issue two arithmetic instructions
 * per clock cycle using one 32/64 bit unit (SU1) and one 32 bit unit (SU2).
 * Thus 1KB of input data will need an estimated maximum of 18,000 cycles.
 * Headroom for cache misses included. Even with the low end model clocked
 * at 667 MHz this equals to a critical time window of less than 27us.
 *
 */
pub const MAX_BYTES: usize = 1024;

extern "C" {
    pub fn ppc_spe_sha256_transform(
        state: *mut sha256_block_state,
        src: *const u8,
        blocks: u32,
    );
}

unsafe fn spe_begin() {
    /* We just start SPE operations and will save SPE registers later. */
    preempt_disable();
    enable_kernel_spe();
}

unsafe fn spe_end() {
    disable_kernel_spe();
    /* reenable preemption */
    preempt_enable();
}

unsafe fn sha256_blocks(
    state: *mut sha256_block_state,
    mut data: *const u8,
    mut nblocks: usize,
) {
    loop {
        /* cut input data into smaller blocks */
        let unit: u32 = core::cmp::min(nblocks, MAX_BYTES / SHA256_BLOCK_SIZE) as u32;

        spe_begin();
        ppc_spe_sha256_transform(state, data, unit);
        spe_end();

        data = data.add((unit as usize) * SHA256_BLOCK_SIZE);
        nblocks -= unit as usize;
        if nblocks == 0 {
            break;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
