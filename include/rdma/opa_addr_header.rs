/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/*
 * Copyright(c) 2017 Intel Corporation.
 */

// Dependency intent: declarations supplied by <rdma/opa_smi.h> are expected
// to be available to this translation.

pub const OPA_SPECIAL_OUI: u64 = 0x00066A_u64;

#[macro_export]
macro_rules! OPA_MAKE_ID {
    ($x:expr) => {{
        cpu_to_be64((($crate::OPA_SPECIAL_OUI << 40) | ($x)))
    }};
}

#[macro_export]
macro_rules! OPA_TO_IB_UCAST_LID {
    ($x:expr) => {{
        if (($x) >= be16_to_cpu(IB_MULTICAST_LID_BASE)) { 0 } else { $x }
    }};
}

pub const OPA_GID_INDEX: u32 = 0x1;

/**
 * 0xF8 - 4 bits of multicast range and 1 bit for collective range
 * Example: For 24 bit LID space,
 * Multicast range: 0xF00000 to 0xF7FFFF
 * Collective range: 0xF80000 to 0xFFFFFE
 */
pub const OPA_MCAST_NR: u32 = 0x4; // Number of top bits set
pub const OPA_COLLECTIVE_NR: u32 = 0x1; // Number of bits after MCAST_NR

/**
 * ib_is_opa_gid: Returns true if the top 24 bits of the gid
 * contains the OPA_STL_OUI identifier. This identifies that the
 * provided gid is a special purpose GID meant to carry
 * extended LID information.
 *
 * @gid: The Global identifier
 */
pub unsafe fn ib_is_opa_gid(gid: *const ib_gid) -> bool {
    (be64_to_cpu((*gid).global.interface_id) >> 40) == OPA_SPECIAL_OUI
}

/**
 * opa_get_lid_from_gid: Returns the last 32 bits of the gid.
 * OPA devices use one of the gids in the gid table to also
 * store the lid.
 *
 * @gid: The Global identifier
 */
pub unsafe fn opa_get_lid_from_gid(gid: *const ib_gid) -> u32 {
    (be64_to_cpu((*gid).global.interface_id) & 0xFFFF_FFFF) as u32
}

/**
 * opa_is_extended_lid: Returns true if dlid or slid are
 * extended.
 *
 * @dlid: The DLID
 * @slid: The SLID
 */
pub fn opa_is_extended_lid(dlid: __be32, slid: __be32) -> bool {
    if (be32_to_cpu(dlid) >= be16_to_cpu(IB_MULTICAST_LID_BASE))
        || (be32_to_cpu(slid) >= be16_to_cpu(IB_MULTICAST_LID_BASE))
    {
        return true;
    }

    false
}

/* Get multicast lid base */
pub fn opa_get_mcast_base(nr_top_bits: u32) -> u32 {
    be32_to_cpu(OPA_LID_PERMISSIVE) << (32 - nr_top_bits)
}

/* Check for a valid unicast LID for non-SM traffic types */
pub unsafe fn rdma_is_valid_unicast_lid(attr: *mut rdma_ah_attr) -> bool {
    if (*attr).type_ == RDMA_AH_ATTR_TYPE_IB {
        if rdma_ah_get_dlid(attr) == 0
            || rdma_ah_get_dlid(attr) >= be16_to_cpu(IB_MULTICAST_LID_BASE)
        {
            return false;
        }
    } else if (*attr).type_ == RDMA_AH_ATTR_TYPE_OPA {
        if rdma_ah_get_dlid(attr) == 0
            || rdma_ah_get_dlid(attr) >= opa_get_mcast_base(OPA_MCAST_NR)
        {
            return false;
        }
    }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
