/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from the C header.  The Linux kernel types and constants used
// here are supplied by other translation units.

pub const ETHNL_MAX_BITSET_SIZE: i32 = S16_MAX;

pub type EthnlStringArrayT = *const [core::ffi::c_char; ETH_GSTRING_LEN];

extern "C" {
    pub fn ethnl_bitset_is_compact(
        bitset: *const nlattr,
        compact: *mut bool,
    ) -> core::ffi::c_int;

    pub fn ethnl_bitset_size(
        val: *const core::ffi::c_ulong,
        mask: *const core::ffi::c_ulong,
        nbits: core::ffi::c_uint,
        names: EthnlStringArrayT,
        compact: bool,
    ) -> core::ffi::c_int;

    pub fn ethnl_bitset32_size(
        val: *const u32,
        mask: *const u32,
        nbits: core::ffi::c_uint,
        names: EthnlStringArrayT,
        compact: bool,
    ) -> core::ffi::c_int;

    pub fn ethnl_put_bitset(
        skb: *mut sk_buff,
        attrtype: core::ffi::c_int,
        val: *const core::ffi::c_ulong,
        mask: *const core::ffi::c_ulong,
        nbits: core::ffi::c_uint,
        names: EthnlStringArrayT,
        compact: bool,
    ) -> core::ffi::c_int;

    pub fn ethnl_put_bitset32(
        skb: *mut sk_buff,
        attrtype: core::ffi::c_int,
        val: *const u32,
        mask: *const u32,
        nbits: core::ffi::c_uint,
        names: EthnlStringArrayT,
        compact: bool,
    ) -> core::ffi::c_int;

    pub fn ethnl_update_bitset(
        bitmap: *mut core::ffi::c_ulong,
        nbits: core::ffi::c_uint,
        attr: *const nlattr,
        names: EthnlStringArrayT,
        extack: *mut netlink_ext_ack,
        mod_: *mut bool,
    ) -> core::ffi::c_int;

    pub fn ethnl_update_bitset32(
        bitmap: *mut u32,
        nbits: core::ffi::c_uint,
        attr: *const nlattr,
        names: EthnlStringArrayT,
        extack: *mut netlink_ext_ack,
        mod_: *mut bool,
    ) -> core::ffi::c_int;

    pub fn ethnl_parse_bitset(
        val: *mut core::ffi::c_ulong,
        mask: *mut core::ffi::c_ulong,
        nbits: core::ffi::c_uint,
        attr: *const nlattr,
        names: EthnlStringArrayT,
        extack: *mut netlink_ext_ack,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
