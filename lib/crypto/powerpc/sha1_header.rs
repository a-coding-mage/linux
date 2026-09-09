/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-1 optimized for PowerPC
 *
 * Copyright (c) 2015 Markus Stockhausen <stockhausen@collogia.de>
 */

/* Dependencies supplied by the surrounding kernel translation. */

/* CONFIG_SPE conditional from the original header. */
#[cfg(feature = "CONFIG_SPE")]
pub const MAX_BYTES: usize = 2048;

#[cfg(feature = "CONFIG_SPE")]
extern "C" {
    pub fn ppc_spe_sha1_transform(
        state: *mut sha1_block_state,
        data: *const u8,
        nblocks: u32,
    );
}

#[cfg(feature = "CONFIG_SPE")]
unsafe fn spe_begin() {
    /* We just start SPE operations and will save SPE registers later. */
    preempt_disable();
    enable_kernel_spe();
}

#[cfg(feature = "CONFIG_SPE")]
unsafe fn spe_end() {
    disable_kernel_spe();
    /* reenable preemption */
    preempt_enable();
}

#[cfg(feature = "CONFIG_SPE")]
unsafe fn sha1_blocks(
    state: *mut sha1_block_state,
    mut data: *const u8,
    mut nblocks: usize,
) {
    loop {
        let limit = MAX_BYTES / SHA1_BLOCK_SIZE;
        let unit = if nblocks < limit { nblocks } else { limit };

        spe_begin();
        ppc_spe_sha1_transform(state, data, unit as u32);
        spe_end();

        data = data.add(unit * SHA1_BLOCK_SIZE);
        nblocks -= unit;
        if nblocks == 0 {
            break;
        }
    }
}

/* CONFIG_SPE disabled branch from the original header. */
#[cfg(not(feature = "CONFIG_SPE"))]
extern "C" {
    pub fn powerpc_sha_transform(
        state: *mut sha1_block_state,
        data: *const u8,
    );
}

#[cfg(not(feature = "CONFIG_SPE"))]
unsafe fn sha1_blocks(
    state: *mut sha1_block_state,
    mut data: *const u8,
    mut nblocks: usize,
) {
    loop {
        powerpc_sha_transform(state, data);
        data = data.add(SHA1_BLOCK_SIZE);
        nblocks -= 1;
        if nblocks == 0 {
            break;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
