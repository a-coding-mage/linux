/* SPDX-License-Identifier: GPL-2.0-only */

// C conditional: defined(CONFIG_X86_64) && !defined(CONFIG_UML)
// The declarations below are available only for x86_64 builds outside UML.

#[cfg(all(target_arch = "x86_64", not(feature = "CONFIG_UML")))]
pub const NFT_PIPAPO_ALIGN: usize = XSAVE_YMM_SIZE / BITS_PER_BYTE;

#[cfg(all(target_arch = "x86_64", not(feature = "CONFIG_UML")))]
#[repr(C)]
pub struct nft_pipapo_match {
    _private: [u8; 0],
}

#[cfg(all(target_arch = "x86_64", not(feature = "CONFIG_UML")))]
extern "C" {
    pub fn nft_pipapo_avx2_estimate(
        desc: *const nft_set_desc,
        features: u32,
        est: *mut nft_set_estimate,
    ) -> bool;

    pub fn pipapo_get_avx2(
        m: *const nft_pipapo_match,
        data: *const u8,
        genmask: u8,
        tstamp: u64,
    ) -> *mut nft_pipapo_elem;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
