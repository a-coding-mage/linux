/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to <asm/barrier.h>.
unsafe extern "C" {
    fn bcr_serialize();
}

#[inline(always)]
unsafe fn sync_core() {
    bcr_serialize();
}

unsafe extern "C" {
    fn text_poke_sync();
    fn text_poke_sync_lock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
