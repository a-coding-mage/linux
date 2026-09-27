/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from the C header.  The Linux kernel types and constants used
// here are supplied by other translation units.

pub const ETHNL_MAX_BITSET_SIZE: i32 = S16_MAX;

pub type EthnlStringArrayT = *const [kernel::ffi::c_char; ETH_GSTRING_LEN];

extern "C" {
    pub fn ethnl_bitset_is_compact(
        bitset: *const nlattr,
        compact: *mut bool,
    ) -> kernel::ffi::c_int;

    pub fn ethnl_bitset_size(
        val: *const kernel::ffi::c_ulong,
        mask: *const kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
        names: EthnlStringArrayT,
        compact: bool,
    ) -> kernel::ffi::c_int;

    pub fn ethnl_bitset32_size(
        val: *const u32,
        mask: *const u32,
        nbits: kernel::ffi::c_uint,
        names: EthnlStringArrayT,
        compact: bool,
    ) -> kernel::ffi::c_int;

    pub fn ethnl_put_bitset(
        skb: *mut sk_buff,
        attrtype: kernel::ffi::c_int,
        val: *const kernel::ffi::c_ulong,
        mask: *const kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
        names: EthnlStringArrayT,
        compact: bool,
    ) -> kernel::ffi::c_int;

    pub fn ethnl_put_bitset32(
        skb: *mut sk_buff,
        attrtype: kernel::ffi::c_int,
        val: *const u32,
        mask: *const u32,
        nbits: kernel::ffi::c_uint,
        names: EthnlStringArrayT,
        compact: bool,
    ) -> kernel::ffi::c_int;

    pub fn ethnl_update_bitset(
        bitmap: *mut kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
        attr: *const nlattr,
        names: EthnlStringArrayT,
        extack: *mut netlink_ext_ack,
        mod_: *mut bool,
    ) -> kernel::ffi::c_int;

    pub fn ethnl_update_bitset32(
        bitmap: *mut u32,
        nbits: kernel::ffi::c_uint,
        attr: *const nlattr,
        names: EthnlStringArrayT,
        extack: *mut netlink_ext_ack,
        mod_: *mut bool,
    ) -> kernel::ffi::c_int;

    pub fn ethnl_parse_bitset(
        val: *mut kernel::ffi::c_ulong,
        mask: *mut kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
        attr: *const nlattr,
        names: EthnlStringArrayT,
        extack: *mut netlink_ext_ack,
    ) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
