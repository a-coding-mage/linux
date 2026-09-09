/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for NTFS kernel bitmap handling.
 *
 * Copyright (c) 2004 Anton Altaparmakov
 */

// Dependency declarations supplied by the corresponding kernel and volume
// headers are intentionally left as external types here.
#[repr(C)]
pub struct ntfs_volume {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fstrim_range {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

extern "C" {
    pub fn ntfs_trim_fs(vol: *mut ntfs_volume, range: *mut fstrim_range) -> i32;
    pub fn __ntfs_bitmap_set_bits_in_run(
        vi: *mut inode,
        start_bit: i64,
        count: i64,
        value: u8,
        is_rollback: bool,
    ) -> i32;
}

/*
 * ntfs_bitmap_set_bits_in_run - set a run of bits in a bitmap to a value
 * @vi:          vfs inode describing the bitmap
 * @start_bit:   first bit to set
 * @count:       number of bits to set
 * @value:       value to set the bits to (i.e. 0 or 1)
 *
 * Set @count bits starting at bit @start_bit in the bitmap described by the
 * vfs inode @vi to @value, where @value is either 0 or 1.
 *
 * Return 0 on success and -errno on error.
 */
#[inline]
pub unsafe fn ntfs_bitmap_set_bits_in_run(
    vi: *mut inode,
    start_bit: i64,
    count: i64,
    value: u8,
) -> i32 {
    __ntfs_bitmap_set_bits_in_run(vi, start_bit, count, value, false)
}

/*
 * ntfs_bitmap_set_run - set a run of bits in a bitmap
 * @vi:         vfs inode describing the bitmap
 * @start_bit:  first bit to set
 * @count:      number of bits to set
 *
 * Set @count bits starting at bit @start_bit in the bitmap described by the
 * vfs inode @vi.
 *
 * Return 0 on success and -errno on error.
 */
#[inline]
pub unsafe fn ntfs_bitmap_set_run(vi: *mut inode, start_bit: i64, count: i64) -> i32 {
    ntfs_bitmap_set_bits_in_run(vi, start_bit, count, 1)
}

/*
 * ntfs_bitmap_clear_run - clear a run of bits in a bitmap
 * @vi:         vfs inode describing the bitmap
 * @start_bit:  first bit to clear
 * @count:      number of bits to clear
 *
 * Clear @count bits starting at bit @start_bit in the bitmap described by the
 * vfs inode @vi.
 *
 * Return 0 on success and -errno on error.
 */
#[inline]
pub unsafe fn ntfs_bitmap_clear_run(vi: *mut inode, start_bit: i64, count: i64) -> i32 {
    ntfs_bitmap_set_bits_in_run(vi, start_bit, count, 0)
}

/*
 * ntfs_bitmap_set_bit - set a bit in a bitmap
 * @vi:  vfs inode describing the bitmap
 * @bit: bit to set
 *
 * Set bit @bit in the bitmap described by the vfs inode @vi.
 *
 * Return 0 on success and -errno on error.
 */
#[inline]
pub unsafe fn ntfs_bitmap_set_bit(vi: *mut inode, bit: i64) -> i32 {
    ntfs_bitmap_set_run(vi, bit, 1)
}

/*
 * ntfs_bitmap_clear_bit - clear a bit in a bitmap
 * @vi:  vfs inode describing the bitmap
 * @bit: bit to clear
 *
 * Clear bit @bit in the bitmap described by the vfs inode @vi.
 *
 * Return 0 on success and -errno on error.
 */
#[inline]
pub unsafe fn ntfs_bitmap_clear_bit(vi: *mut inode, bit: i64) -> i32 {
    ntfs_bitmap_clear_run(vi, bit, 1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
