/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/linux/mpage.h
 *
 * Contains declarations related to preparing and submitting BIOS which contain
 * multiple pagecache pages.
 */

/*
 * This header is conditionally available when CONFIG_BLOCK is enabled.
 * The declarations below preserve that build-time condition's intent.
 */

#[cfg(CONFIG_BLOCK)]
#[repr(C)]
pub struct writeback_control {
    _private: [u8; 0],
}

#[cfg(CONFIG_BLOCK)]
#[repr(C)]
pub struct readahead_control {
    _private: [u8; 0],
}

#[cfg(CONFIG_BLOCK)]
extern "C" {
    pub fn mpage_readahead(
        rac: *mut readahead_control,
        get_block: get_block_t,
    );

    pub fn mpage_read_folio(
        folio: *mut folio,
        get_block: get_block_t,
    ) -> ::core::ffi::c_int;

    pub fn __mpage_writepages(
        mapping: *mut address_space,
        wbc: *mut writeback_control,
        get_block: get_block_t,
        write_folio: Option<
            unsafe extern "C" fn(
                folio: *mut folio,
                wbc: *mut writeback_control,
            ) -> ::core::ffi::c_int,
        >,
    ) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_BLOCK)]
#[inline]
pub unsafe fn mpage_writepages(
    mapping: *mut address_space,
    wbc: *mut writeback_control,
    get_block: get_block_t,
) -> ::core::ffi::c_int {
    __mpage_writepages(mapping, wbc, get_block, None)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
