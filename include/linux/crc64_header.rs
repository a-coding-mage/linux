/* SPDX-License-Identifier: GPL-2.0 */

//! Declarations translated from the Linux CRC64 header.

/**
 * crc64_be - Calculate bitwise big-endian ECMA-182 CRC64
 * @crc: seed value for computation. 0 or (u64)~0 for a new CRC calculation,
 *       or the previous crc64 value if computing incrementally.
 * @p: pointer to buffer over which CRC64 is run
 * @len: length of buffer @p
 */
pub unsafe extern "C" fn crc64_be(crc: u64, p: *const core::ffi::c_void, len: usize) -> u64;

/**
 * crc64_nvme - Calculate CRC64-NVME
 * @crc: seed value for computation. 0 for a new CRC calculation, or the
 *       previous crc64 value if computing incrementally.
 * @p: pointer to buffer over which CRC64 is run
 * @len: length of buffer @p
 *
 * This computes the CRC64 defined in the NVME NVM Command Set Specification,
 * *including the bitwise inversion at the beginning and end*.
 */
pub unsafe extern "C" fn crc64_nvme(crc: u64, p: *const core::ffi::c_void, len: usize) -> u64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
