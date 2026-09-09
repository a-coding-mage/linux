/* SPDX-License-Identifier: GPL-2.0 */

pub const IOEND_BATCH_SIZE: usize = 4096;

/*
 * Normally we can build bios as big as the data structure supports.
 *
 * But for integrity protected I/O we need to respect the maximum size of the
 * single contiguous allocation for the integrity buffer.
 */
pub unsafe fn iomap_max_bio_size(iomap: *const iomap) -> usize {
    if (*iomap).flags & IOMAP_F_INTEGRITY != 0 {
        return max_integrity_io_size(bdev_limits((*iomap).bdev));
    }
    BIO_MAX_SIZE
}

pub extern "C" {
    pub fn iomap_finish_ioend_buffered_read(ioend: *mut iomap_ioend) -> u32;
    pub fn iomap_finish_ioend_direct(ioend: *mut iomap_ioend) -> u32;
}

/* CONFIG_BLOCK conditional declaration. */
#[cfg(CONFIG_BLOCK)]
pub extern "C" {
    pub fn iomap_bio_read_folio_range_sync(
        iter: *const iomap_iter,
        folio: *mut folio,
        pos: loff_t,
        len: usize,
    ) -> i32;
}

/* CONFIG_BLOCK conditional fallback definition. */
#[cfg(not(CONFIG_BLOCK))]
pub unsafe fn iomap_bio_read_folio_range_sync(
    _iter: *const iomap_iter,
    _folio: *mut folio,
    _pos: loff_t,
    _len: usize,
) -> i32 {
    WARN_ON_ONCE(1);
    -EIO
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
