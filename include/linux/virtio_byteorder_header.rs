/* SPDX-License-Identifier: GPL-2.0 */
// Dependency intent: linux/types.h and uapi/linux/virtio_types.h provide the
// integer and __virtio* types and endian conversion operations used below.

pub unsafe fn virtio_legacy_is_little_endian() -> bool {
    // C build-time condition: preserve the __LITTLE_ENDIAN configuration intent.
    cfg!(target_endian = "little")
}

pub unsafe fn __virtio16_to_cpu(little_endian: bool, val: __virtio16) -> u16 {
    if little_endian {
        le16_to_cpu(val as __le16)
    } else {
        be16_to_cpu(val as __be16)
    }
}

pub unsafe fn __cpu_to_virtio16(little_endian: bool, val: u16) -> __virtio16 {
    if little_endian {
        cpu_to_le16(val) as __virtio16
    } else {
        cpu_to_be16(val) as __virtio16
    }
}

pub unsafe fn __virtio32_to_cpu(little_endian: bool, val: __virtio32) -> u32 {
    if little_endian {
        le32_to_cpu(val as __le32)
    } else {
        be32_to_cpu(val as __be32)
    }
}

pub unsafe fn __cpu_to_virtio32(little_endian: bool, val: u32) -> __virtio32 {
    if little_endian {
        cpu_to_le32(val) as __virtio32
    } else {
        cpu_to_be32(val) as __virtio32
    }
}

pub unsafe fn __virtio64_to_cpu(little_endian: bool, val: __virtio64) -> u64 {
    if little_endian {
        le64_to_cpu(val as __le64)
    } else {
        be64_to_cpu(val as __be64)
    }
}

pub unsafe fn __cpu_to_virtio64(little_endian: bool, val: u64) -> __virtio64 {
    if little_endian {
        cpu_to_le64(val) as __virtio64
    } else {
        cpu_to_be64(val) as __virtio64
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
