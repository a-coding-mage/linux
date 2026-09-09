/* SPDX-License-Identifier: BSD-3-Clause
 * Copyright 2016-2018 NXP
 * Copyright (c) 2018-2019, Vladimir Oltean <olteanv@gmail.com>
 */

// C dependencies: linux/array_size.h, linux/bitops.h, linux/build_bug.h,
// linux/minmax.h, linux/stddef.h, and linux/types.h.

#[repr(C)]
pub struct packed_field_u8 {
    pub startbit: u8,
    pub endbit: u8,
    pub offset: usize,
    pub size: usize,
}

#[repr(C)]
pub struct packed_field_u16 {
    pub startbit: u16,
    pub endbit: u16,
    pub offset: usize,
    pub size: usize,
}

#[macro_export]
macro_rules! PACKED_FIELD {
    ($start:expr, $end:expr, $struct_name:ty, $struct_field:ident) => {
        $crate::packed_field_u8 {
            startbit: $start,
            endbit: $end,
            offset: core::mem::offset_of!($struct_name, $struct_field),
            size: core::mem::size_of::<$struct_name>(),
        }
    };
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum packing_op {
    PACK,
    UNPACK,
}

pub const QUIRK_MSB_ON_THE_RIGHT: u8 = 1 << 0;
pub const QUIRK_LITTLE_ENDIAN: u8 = 1 << 1;
pub const QUIRK_LSW32_IS_FIRST: u8 = 1 << 2;

extern "C" {
    pub fn packing(
        pbuf: *mut core::ffi::c_void,
        uval: *mut u64,
        startbit: i32,
        endbit: i32,
        pbuflen: usize,
        op: packing_op,
        quirks: u8,
    ) -> i32;
    pub fn pack(
        pbuf: *mut core::ffi::c_void,
        uval: u64,
        startbit: usize,
        endbit: usize,
        pbuflen: usize,
        quirks: u8,
    ) -> i32;
    pub fn unpack(
        pbuf: *const core::ffi::c_void,
        uval: *mut u64,
        startbit: usize,
        endbit: usize,
        pbuflen: usize,
        quirks: u8,
    ) -> i32;
    pub fn pack_fields_u8(
        pbuf: *mut core::ffi::c_void, pbuflen: usize, ustruct: *const core::ffi::c_void,
        fields: *const packed_field_u8, num_fields: usize, quirks: u8,
    );
    pub fn pack_fields_u16(
        pbuf: *mut core::ffi::c_void, pbuflen: usize, ustruct: *const core::ffi::c_void,
        fields: *const packed_field_u16, num_fields: usize, quirks: u8,
    );
    pub fn unpack_fields_u8(
        pbuf: *const core::ffi::c_void, pbuflen: usize, ustruct: *mut core::ffi::c_void,
        fields: *const packed_field_u8, num_fields: usize, quirks: u8,
    );
    pub fn unpack_fields_u16(
        pbuf: *const core::ffi::c_void, pbuflen: usize, ustruct: *mut core::ffi::c_void,
        fields: *const packed_field_u16, num_fields: usize, quirks: u8,
    );
}

// The following checks are compile-time assertions in the C header. Rust
// callers may use the corresponding assertion macros for constant arrays.
#[macro_export]
macro_rules! CHECK_PACKED_FIELD {
    ($fields:expr, $index:expr) => {{
        const _: () = {
            let _ = &$fields[$index];
        };
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS {
    ($fields:expr) => {{
        const _: () = {
            let _ = &$fields;
        };
    }};
}

// Generated field-count checks, corresponding to CHECK_PACKED_FIELDS_1..50.
#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_1 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELD!($fields, 0);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_2 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_1!($fields);
        CHECK_PACKED_FIELD!($fields, 1);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_3 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_2!($fields);
        CHECK_PACKED_FIELD!($fields, 2);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_4 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_3!($fields);
        CHECK_PACKED_FIELD!($fields, 3);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_5 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_4!($fields);
        CHECK_PACKED_FIELD!($fields, 4);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_6 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_5!($fields);
        CHECK_PACKED_FIELD!($fields, 5);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_7 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_6!($fields);
        CHECK_PACKED_FIELD!($fields, 6);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_8 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_7!($fields);
        CHECK_PACKED_FIELD!($fields, 7);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_9 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_8!($fields);
        CHECK_PACKED_FIELD!($fields, 8);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_10 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_9!($fields);
        CHECK_PACKED_FIELD!($fields, 9);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_11 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_10!($fields);
        CHECK_PACKED_FIELD!($fields, 10);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_12 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_11!($fields);
        CHECK_PACKED_FIELD!($fields, 11);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_13 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_12!($fields);
        CHECK_PACKED_FIELD!($fields, 12);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_14 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_13!($fields);
        CHECK_PACKED_FIELD!($fields, 13);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_15 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_14!($fields);
        CHECK_PACKED_FIELD!($fields, 14);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_16 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_15!($fields);
        CHECK_PACKED_FIELD!($fields, 15);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_17 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_16!($fields);
        CHECK_PACKED_FIELD!($fields, 16);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_18 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_17!($fields);
        CHECK_PACKED_FIELD!($fields, 17);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_19 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_18!($fields);
        CHECK_PACKED_FIELD!($fields, 18);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_20 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_19!($fields);
        CHECK_PACKED_FIELD!($fields, 19);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_21 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_20!($fields);
        CHECK_PACKED_FIELD!($fields, 20);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_22 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_21!($fields);
        CHECK_PACKED_FIELD!($fields, 21);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_23 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_22!($fields);
        CHECK_PACKED_FIELD!($fields, 22);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_24 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_23!($fields);
        CHECK_PACKED_FIELD!($fields, 23);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_25 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_24!($fields);
        CHECK_PACKED_FIELD!($fields, 24);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_26 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_25!($fields);
        CHECK_PACKED_FIELD!($fields, 25);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_27 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_26!($fields);
        CHECK_PACKED_FIELD!($fields, 26);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_28 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_27!($fields);
        CHECK_PACKED_FIELD!($fields, 27);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_29 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_28!($fields);
        CHECK_PACKED_FIELD!($fields, 28);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_30 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_29!($fields);
        CHECK_PACKED_FIELD!($fields, 29);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_31 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_30!($fields);
        CHECK_PACKED_FIELD!($fields, 30);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_32 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_31!($fields);
        CHECK_PACKED_FIELD!($fields, 31);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_33 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_32!($fields);
        CHECK_PACKED_FIELD!($fields, 32);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_34 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_33!($fields);
        CHECK_PACKED_FIELD!($fields, 33);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_35 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_34!($fields);
        CHECK_PACKED_FIELD!($fields, 34);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_36 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_35!($fields);
        CHECK_PACKED_FIELD!($fields, 35);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_37 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_36!($fields);
        CHECK_PACKED_FIELD!($fields, 36);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_38 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_37!($fields);
        CHECK_PACKED_FIELD!($fields, 37);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_39 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_38!($fields);
        CHECK_PACKED_FIELD!($fields, 38);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_40 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_39!($fields);
        CHECK_PACKED_FIELD!($fields, 39);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_41 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_40!($fields);
        CHECK_PACKED_FIELD!($fields, 40);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_42 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_41!($fields);
        CHECK_PACKED_FIELD!($fields, 41);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_43 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_42!($fields);
        CHECK_PACKED_FIELD!($fields, 42);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_44 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_43!($fields);
        CHECK_PACKED_FIELD!($fields, 43);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_45 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_44!($fields);
        CHECK_PACKED_FIELD!($fields, 44);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_46 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_45!($fields);
        CHECK_PACKED_FIELD!($fields, 45);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_47 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_46!($fields);
        CHECK_PACKED_FIELD!($fields, 46);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_48 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_47!($fields);
        CHECK_PACKED_FIELD!($fields, 47);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_49 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_48!($fields);
        CHECK_PACKED_FIELD!($fields, 48);
    }};
}

#[macro_export]
macro_rules! CHECK_PACKED_FIELDS_50 {
    ($fields:expr) => {{
        CHECK_PACKED_FIELDS_49!($fields);
        CHECK_PACKED_FIELD!($fields, 49);
    }};
}


#[macro_export]
macro_rules! pack_fields {
    ($pbuf:expr, $pbuflen:expr, $ustruct:expr, $fields:expr, $quirks:expr) => {{
        $crate::pack_fields_u8(
            $pbuf, $pbuflen, $ustruct, ($fields).as_ptr(), ($fields).len(), $quirks,
        )
    }};
}

#[macro_export]
macro_rules! unpack_fields {
    ($pbuf:expr, $pbuflen:expr, $ustruct:expr, $fields:expr, $quirks:expr) => {{
        $crate::unpack_fields_u8(
            $pbuf, $pbuflen, $ustruct, ($fields).as_ptr(), ($fields).len(), $quirks,
        )
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
