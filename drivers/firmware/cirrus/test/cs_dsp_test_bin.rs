// SPDX-License-Identifier: GPL-2.0-only
//
// KUnit tests for cs_dsp. Rust translation of cs_dsp_test_bin.c.

use core::ffi::{c_char, c_int, c_void};

/* The kernel/KUnit and cs_dsp interfaces below are supplied by the surrounding
 * kernel translation unit.  Their declarations intentionally remain external. */
#[repr(C)]
pub struct cs_dsp_test_local {
    pub bin_builder: *mut cs_dsp_mock_bin_builder,
    pub wmfw_builder: *mut cs_dsp_mock_wmfw_builder,
    pub wmfw: *mut firmware,
}

#[repr(C)]
pub struct bin_test_param {
    pub name: *const c_char,
    pub mem_type: c_int,
    pub offset_words: u32,
    pub alg_idx: c_int,
    pub add_patch: Option<unsafe extern "C" fn(
        *mut cs_dsp_mock_bin_builder, u32, u32, c_int, u32, *const c_void, usize,
    )>,
}

#[repr(C)]
pub struct cs_dsp_mock_alg_def {
    pub id: u32,
    pub ver: u32,
    pub xm_size_words: u32,
    pub ym_size_words: u32,
    pub zm_size_words: u32,
}

#[repr(C)] pub struct cs_dsp_mock_bin_builder { _private: [u8; 0] }
#[repr(C)] pub struct cs_dsp_mock_wmfw_builder { _private: [u8; 0] }
#[repr(C)] pub struct firmware { _private: [u8; 0] }
#[repr(C)] pub struct kunit { pub priv_: *mut c_void, pub param_value: *const c_void }

pub static BIN_TEST_MOCK_ALGS: [cs_dsp_mock_alg_def; 4] = [
    cs_dsp_mock_alg_def { id: 0xfafa, ver: 0x100000, xm_size_words: 164, ym_size_words: 164, zm_size_words: 164 },
    cs_dsp_mock_alg_def { id: 0xfbfb, ver: 0x100000, xm_size_words: 99, ym_size_words: 99, zm_size_words: 99 },
    cs_dsp_mock_alg_def { id: 0xc321, ver: 0x100000, xm_size_words: 120, ym_size_words: 120, zm_size_words: 120 },
    cs_dsp_mock_alg_def { id: 0xb123, ver: 0x100000, xm_size_words: 96, ym_size_words: 96, zm_size_words: 96 },
];

#[inline]
pub const fn num_words_to_num_packed_regs(num_dsp_words: u32) -> u32 {
    (num_dsp_words * 3) / 4
}

/* External test entry points retained with the same externally visible names.
 * Their implementations are provided by the translated cs_dsp test support. */
extern "C" {
    pub fn bin_patch_one_word(test: *mut kunit);
    pub fn bin_patch_one_multiword(test: *mut kunit);
    pub fn bin_patch_multi_oneword(test: *mut kunit);
    pub fn bin_patch_multi_oneword_unordered(test: *mut kunit);
    pub fn bin_patch_multi_oneword_sparse_unordered(test: *mut kunit);
    pub fn bin_patch_one_word_multiple_mems(test: *mut kunit);
    pub fn bin_patch_one_word_multiple_algs(test: *mut kunit);
    pub fn bin_patch_one_word_multiple_algs_unordered(test: *mut kunit);
    pub fn bin_patch_1_packed(test: *mut kunit);
    pub fn bin_patch_1_packed_1_single_trailing(test: *mut kunit);
    pub fn bin_patch_1_packed_2_single_trailing(test: *mut kunit);
    pub fn bin_patch_1_packed_3_single_trailing(test: *mut kunit);
    pub fn bin_patch_1_packed_2_trailing(test: *mut kunit);
    pub fn bin_patch_1_packed_3_trailing(test: *mut kunit);
    pub fn bin_patch_1_single_leading_1_packed(test: *mut kunit);
    pub fn bin_patch_2_single_leading_1_packed(test: *mut kunit);
    pub fn bin_patch_2_leading_1_packed(test: *mut kunit);
    pub fn bin_patch_3_single_leading_1_packed(test: *mut kunit);
    pub fn bin_patch_3_leading_1_packed(test: *mut kunit);
    pub fn bin_patch_multi_onepacked(test: *mut kunit);
    pub fn bin_patch_multi_onepacked_unordered(test: *mut kunit);
    pub fn bin_patch_multi_onepacked_sparse_unordered(test: *mut kunit);
    pub fn bin_patch_1_packed_multiple_mems(test: *mut kunit);
    pub fn bin_patch_1_packed_multiple_algs(test: *mut kunit);
    pub fn bin_patch_1_packed_multiple_algs_unordered(test: *mut kunit);
    pub fn bin_patch_mixed_packed_unpacked_random(test: *mut kunit);
    pub fn bin_patch_name_and_info(test: *mut kunit);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
