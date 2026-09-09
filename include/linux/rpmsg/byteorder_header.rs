/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Follows implementation found in linux/virtio_byteorder.h
 */

// Dependencies supplied by the surrounding Linux/Rust translation.

#[inline]
pub fn rpmsg_is_little_endian() -> bool {
    cfg!(target_endian = "little")
}

#[inline]
pub fn __rpmsg16_to_cpu(little_endian: bool, val: __rpmsg16) -> u16 {
    if little_endian {
        le16_to_cpu(val as __le16)
    } else {
        be16_to_cpu(val as __be16)
    }
}

#[inline]
pub fn __cpu_to_rpmsg16(little_endian: bool, val: u16) -> __rpmsg16 {
    if little_endian {
        cpu_to_le16(val) as __rpmsg16
    } else {
        cpu_to_be16(val) as __rpmsg16
    }
}

#[inline]
pub fn __rpmsg32_to_cpu(little_endian: bool, val: __rpmsg32) -> u32 {
    if little_endian {
        le32_to_cpu(val as __le32)
    } else {
        be32_to_cpu(val as __be32)
    }
}

#[inline]
pub fn __cpu_to_rpmsg32(little_endian: bool, val: u32) -> __rpmsg32 {
    if little_endian {
        cpu_to_le32(val) as __rpmsg32
    } else {
        cpu_to_be32(val) as __rpmsg32
    }
}

#[inline]
pub fn __rpmsg64_to_cpu(little_endian: bool, val: __rpmsg64) -> u64 {
    if little_endian {
        le64_to_cpu(val as __le64)
    } else {
        be64_to_cpu(val as __be64)
    }
}

#[inline]
pub fn __cpu_to_rpmsg64(little_endian: bool, val: u64) -> __rpmsg64 {
    if little_endian {
        cpu_to_le64(val) as __rpmsg64
    } else {
        cpu_to_be64(val) as __rpmsg64
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
