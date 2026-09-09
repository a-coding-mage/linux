/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the corresponding Linux ASN.1 and type headers.

/// Equivalent of the C `asn1_oid_len(oid)` macro.
#[macro_export]
macro_rules! asn1_oid_len {
    ($oid:expr) => {
        ::core::mem::size_of_val(&$oid) / ::core::mem::size_of::<u32>()
    };
}

extern "C" {
    pub fn asn1_encode_integer(
        data: *mut u8,
        end_data: *const u8,
        integer: i64,
    ) -> *mut u8;

    pub fn asn1_encode_oid(
        data: *mut u8,
        end_data: *const u8,
        oid: *mut u32,
        oid_len: i32,
    ) -> *mut u8;

    pub fn asn1_encode_tag(
        data: *mut u8,
        end_data: *const u8,
        tag: u32,
        string: *const u8,
        len: i32,
    ) -> *mut u8;

    pub fn asn1_encode_octet_string(
        data: *mut u8,
        end_data: *const u8,
        string: *const u8,
        len: u32,
    ) -> *mut u8;

    pub fn asn1_encode_sequence(
        data: *mut u8,
        end_data: *const u8,
        seq: *const u8,
        len: i32,
    ) -> *mut u8;

    pub fn asn1_encode_boolean(
        data: *mut u8,
        end_data: *const u8,
        val: bool,
    ) -> *mut u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
