/* SPDX-License-Identifier: GPL-2.0-or-later */

/* defined in blake2s-core.S */
#[repr(C)]
pub struct blake2s_ctx {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn blake2s_compress(
        ctx: *mut blake2s_ctx,
        data: *const u8,
        nblocks: usize,
        inc: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
