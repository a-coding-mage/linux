/* SPDX-License-Identifier: 0BSD */

/*
 * Definitions for handling the .xz file format
 *
 * Author: Lasse Collin <lasse.collin@tukaani.org>
 */

/*
 * The kernel-only crc32 compatibility block depends on the kernel headers and
 * build configuration; its intent is preserved here for the future bindings.
 */

/*
 * See the .xz file format specification at
 * https://tukaani.org/xz/xz-file-format.txt
 * to understand the container format.
 */

pub const STREAM_HEADER_SIZE: usize = 12;

pub const HEADER_MAGIC: &[u8; 6] = b"\xfd7zXZ\x00";
pub const HEADER_MAGIC_SIZE: usize = 6;

pub const FOOTER_MAGIC: &[u8; 2] = b"YZ";
pub const FOOTER_MAGIC_SIZE: usize = 2;

/*
 * Variable-length integer can hold a 63-bit unsigned integer or a special
 * value indicating that the value is unknown.
 *
 * Experimental: vli_type can be defined to uint32_t to save a few bytes
 * in code size (no effect on speed). Doing so limits the uncompressed and
 * compressed size of the file to less than 256 MiB and may also weaken
 * error detection slightly.
 */
pub type VliType = u64;

pub const VLI_MAX: VliType = (VliType::MAX) / 2;
pub const VLI_UNKNOWN: VliType = VliType::MAX;

/* Maximum encoded size of a VLI */
pub const VLI_BYTES_MAX: usize = core::mem::size_of::<VliType>() * 8 / 7;

/* Integrity Check types */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum XzCheck {
	XZ_CHECK_NONE = 0,
	XZ_CHECK_CRC32 = 1,
	XZ_CHECK_CRC64 = 4,
	XZ_CHECK_SHA256 = 10,
}

/* Maximum possible Check ID */
pub const XZ_CHECK_MAX: i32 = 15;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
