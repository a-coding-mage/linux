/* SPDX-License-Identifier: GPL-2.0 */

// External dependencies supplied by the surrounding translation unit:
// `uint32_t`, `size_t`, `__le32`, `__be32`, `__u32`, `crc32c`, and
// `cpu_to_le32`.

pub const XFS_CRC_SEED: u32 = !0u32;

/*
 * Calculate the intermediate checksum for a buffer that has the CRC field
 * inside it.  The offset of the 32bit crc fields is passed as the
 * cksum_offset parameter. We do not modify the buffer during verification,
 * hence we have to split the CRC calculation across the cksum_offset.
 */
pub unsafe fn xfs_start_cksum_safe(
    buffer: *mut core::ffi::c_char,
    length: usize,
    cksum_offset: usize,
) -> u32 {
    let zero: u32 = 0;
    let mut crc: u32;

    /* Calculate CRC up to the checksum. */
    crc = crc32c(XFS_CRC_SEED, buffer, cksum_offset);

    /* Skip checksum field */
    crc = crc32c(
        crc,
        (&zero as *const u32).cast::<core::ffi::c_void>(),
        core::mem::size_of::<u32>(),
    );

    /* Calculate the rest of the CRC. */
    crc32c(
        crc,
        buffer.add(cksum_offset + core::mem::size_of::<__be32>()),
        length - (cksum_offset + core::mem::size_of::<__be32>()),
    )
}

/*
 * Fast CRC method where the buffer is modified. Callers must have exclusive
 * access to the buffer while the calculation takes place.
 */
pub unsafe fn xfs_start_cksum_update(
    buffer: *mut core::ffi::c_char,
    length: usize,
    cksum_offset: usize,
) -> u32 {
    /* zero the CRC field */
    (buffer.add(cksum_offset) as *mut __le32).write(0 as __le32);

    /* single pass CRC calculation for the entire buffer */
    crc32c(XFS_CRC_SEED, buffer, length)
}

/*
 * Convert the intermediate checksum to the final ondisk format.
 *
 * The CRC32c calculation uses LE format even on BE machines, but returns the
 * result in host endian format. Hence we need to byte swap it back to LE
 * format so that it is consistent on disk.
 */
pub unsafe fn xfs_end_cksum(crc: u32) -> __le32 {
    !cpu_to_le32(crc)
}

/*
 * Helper to generate the checksum for a buffer.
 *
 * This modifies the buffer temporarily - callers must have exclusive
 * access to the buffer while the calculation takes place.
 */
pub unsafe fn xfs_update_cksum(
    buffer: *mut core::ffi::c_char,
    length: usize,
    cksum_offset: usize,
) {
    let crc = xfs_start_cksum_update(buffer, length, cksum_offset);

    (buffer.add(cksum_offset) as *mut __le32).write(xfs_end_cksum(crc));
}

/*
 * Helper to verify the checksum for a buffer.
 */
pub unsafe fn xfs_verify_cksum(
    buffer: *mut core::ffi::c_char,
    length: usize,
    cksum_offset: usize,
) -> i32 {
    let crc = xfs_start_cksum_safe(buffer, length, cksum_offset);

    ((buffer.add(cksum_offset) as *const __le32).read() == xfs_end_cksum(crc)) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
