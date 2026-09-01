// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * BTF-to-C dumper tests for implicit and explicit padding between fields and
 * at the end of a struct.
 *
 * Copyright (c) 2019 Facebook
 */
/* ----- START-EXPECTED-OUTPUT ----- */
/*
struct padded_implicitly {
	int a;
	long b;
	char c;
};
*/

/* ------ END-EXPECTED-OUTPUT ------ */

/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *struct padded_explicitly {
 *	int a;
 *	long: 0;
 *	int b;
 *};
 *
 */
/* ------ END-EXPECTED-OUTPUT ------ */

type c_char = i8;
type c_short = i16;
type c_int = i32;
type c_long = i64;

#[repr(C)]
pub struct padded_implicitly {
    pub a: c_int,
    pub b: c_long,
    pub c: c_char,
}

#[repr(C)]
pub struct padded_explicitly {
    pub a: c_int,
    /* int: 1; algo will emit aligning `long: 0;` here */
    pub _bitfield_padding_0: c_int,
    pub b: c_int,
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
struct padded_a_lot {
	int a;
	long: 64;
	long: 64;
	int b;
};
*/

/* ------ END-EXPECTED-OUTPUT ------ */

#[repr(C)]
pub struct padded_a_lot {
    pub a: c_int,
    /* long: 64; */
    pub _bitfield_padding_0: c_long,
    /* long: 64; */
    pub _bitfield_padding_1: c_long,
    pub b: c_int,
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *struct padded_cache_line {
 *	int a;
 *	long: 64;
 *	long: 64;
 *	long: 64;
 *	int b;
 *	long: 64;
 *	long: 64;
 *	long: 64;
 *};
 *
 */
/* ------ END-EXPECTED-OUTPUT ------ */

#[repr(C)]
pub struct padded_cache_line {
    pub a: c_int,
    #[repr(align(32))]
    pub b: c_int,
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *struct zone_padding {
 *	char x[0];
 *};
 *
 *struct zone {
 *	int a;
 *	short b;
 *	long: 0;
 *	struct zone_padding __pad__;
 *};
 *
 */
/* ------ END-EXPECTED-OUTPUT ------ */

#[repr(C, align(8))]
pub struct zone_padding {
    pub x: [c_char; 0],
}

#[repr(C)]
pub struct zone {
    pub a: c_int,
    pub b: c_short,
    pub __pad__: zone_padding,
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
struct padding_wo_named_members {
	long: 64;
	long: 64;
};

struct padding_weird_1 {
	int a;
	long: 64;
	short: 16;
	short b;
};
*/

/* ------ END-EXPECTED-OUTPUT ------ */

#[repr(C)]
pub struct padding_wo_named_members {
    /* long: 64; */
    pub _bitfield_padding_0: c_long,
    /* long: 64; */
    pub _bitfield_padding_1: c_long,
}

#[repr(C)]
pub struct padding_weird_1 {
    pub a: c_int,
    /* long: 64; */
    pub _bitfield_padding_0: c_long,
    /* short: 16; */
    pub _bitfield_padding_1: c_short,
    pub b: c_short,
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *struct padding_weird_2 {
 *	long: 56;
 *	char a;
 *	long: 56;
 *	char b;
 *	char: 8;
 *};
 *
 */
/* ------ END-EXPECTED-OUTPUT ------ */
#[repr(C)]
pub struct padding_weird_2 {
    /* int: 32; these paddings will be collapsed into `long: 56;` */
    pub _bitfield_padding_0: c_int,
    /* short: 16; */
    pub _bitfield_padding_1: c_short,
    /* char: 8; */
    pub _bitfield_padding_2: c_char,
    pub a: c_char,
    /* int: 32; these paddings will be collapsed into `long: 56;` */
    pub _bitfield_padding_3: c_int,
    /* short: 16; */
    pub _bitfield_padding_4: c_short,
    /* char: 8; */
    pub _bitfield_padding_5: c_char,
    pub b: c_char,
    /* char: 8; */
    pub _bitfield_padding_6: c_char,
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
struct exact_1byte {
	char x;
};

struct padded_1byte {
	char: 8;
};

struct exact_2bytes {
	short x;
};

struct padded_2bytes {
	short: 16;
};

struct exact_4bytes {
	int x;
};

struct padded_4bytes {
	int: 32;
};

struct exact_8bytes {
	long x;
};

struct padded_8bytes {
	long: 64;
};

struct ff_periodic_effect {
	int: 32;
	short magnitude;
	long: 0;
	short phase;
	long: 0;
	int: 32;
	int custom_len;
	short *custom_data;
};

struct ib_wc {
	long: 64;
	long: 64;
	int: 32;
	int byte_len;
	void *qp;
	union {} ex;
	long: 64;
	int slid;
	int wc_flags;
	long: 64;
	char smac[6];
	long: 0;
	char network_hdr_type;
};

struct acpi_object_method {
	long: 64;
	char: 8;
	char type;
	short reference_count;
	char flags;
	short: 0;
	char: 8;
	char sync_level;
	long: 64;
	void *node;
	void *aml_start;
	union {} dispatch;
	long: 64;
	int aml_length;
};

struct nested_unpacked {
	int x;
};

struct nested_packed {
	struct nested_unpacked a;
	char c;
} __attribute__((packed));

struct outer_mixed_but_unpacked {
	struct nested_packed b1;
	short a1;
	struct nested_packed b2;
};
*/

/* ------ END-EXPECTED-OUTPUT ------ */

#[repr(C)]
pub struct exact_1byte {
    pub x: c_char,
}

#[repr(C)]
pub struct padded_1byte {
    /* char: 8; */
    pub _bitfield_padding_0: c_char,
}

#[repr(C)]
pub struct exact_2bytes {
    pub x: c_short,
}

#[repr(C)]
pub struct padded_2bytes {
    /* short: 16; */
    pub _bitfield_padding_0: c_short,
}

#[repr(C)]
pub struct exact_4bytes {
    pub x: c_int,
}

#[repr(C)]
pub struct padded_4bytes {
    /* int: 32; */
    pub _bitfield_padding_0: c_int,
}

#[repr(C)]
pub struct exact_8bytes {
    pub x: c_long,
}

#[repr(C)]
pub struct padded_8bytes {
    /* long: 64; */
    pub _bitfield_padding_0: c_long,
}

#[repr(C)]
pub struct ff_periodic_effect {
    /* int: 32; */
    pub _bitfield_padding_0: c_int,
    pub magnitude: c_short,
    /* long: 0; */
    pub phase: c_short,
    /* long: 0; */
    /* int: 32; */
    pub _bitfield_padding_1: c_int,
    pub custom_len: c_int,
    pub custom_data: *mut c_short,
}

#[repr(C)]
pub union ib_wc_ex {
    pub _unused: [u8; 0],
}

#[repr(C)]
pub struct ib_wc {
    /* long: 64; */
    pub _bitfield_padding_0: c_long,
    /* long: 64; */
    pub _bitfield_padding_1: c_long,
    /* int: 32; */
    pub _bitfield_padding_2: c_int,
    pub byte_len: c_int,
    pub qp: *mut core::ffi::c_void,
    pub ex: ib_wc_ex,
    /* long: 64; */
    pub _bitfield_padding_3: c_long,
    pub slid: c_int,
    pub wc_flags: c_int,
    /* long: 64; */
    pub _bitfield_padding_4: c_long,
    pub smac: [c_char; 6],
    /* long: 0; */
    pub network_hdr_type: c_char,
}

#[repr(C)]
pub union acpi_object_method_dispatch {
    pub _unused: [u8; 0],
}

#[repr(C)]
pub struct acpi_object_method {
    /* long: 64; */
    pub _bitfield_padding_0: c_long,
    /* char: 8; */
    pub _bitfield_padding_1: c_char,
    pub type_: c_char,
    pub reference_count: c_short,
    pub flags: c_char,
    /* short: 0; */
    /* char: 8; */
    pub _bitfield_padding_2: c_char,
    pub sync_level: c_char,
    /* long: 64; */
    pub _bitfield_padding_3: c_long,
    pub node: *mut core::ffi::c_void,
    pub aml_start: *mut core::ffi::c_void,
    pub dispatch: acpi_object_method_dispatch,
    /* long: 64; */
    pub _bitfield_padding_4: c_long,
    pub aml_length: c_int,
}

#[repr(C)]
pub struct nested_unpacked {
    pub x: c_int,
}

#[repr(C, packed)]
pub struct nested_packed {
    pub a: nested_unpacked,
    pub c: c_char,
}

#[repr(C)]
pub struct outer_mixed_but_unpacked {
    pub b1: nested_packed,
    pub a1: c_short,
    pub b2: nested_packed,
}

#[repr(C)]
pub struct f_arg {
    pub _1: padded_implicitly,
    pub _2: padded_explicitly,
    pub _3: padded_a_lot,
    pub _4: padded_cache_line,
    pub _5: zone,
    pub _6: padding_wo_named_members,
    pub _7: padding_weird_1,
    pub _8: padding_weird_2,
    pub _100: exact_1byte,
    pub _101: padded_1byte,
    pub _102: exact_2bytes,
    pub _103: padded_2bytes,
    pub _104: exact_4bytes,
    pub _105: padded_4bytes,
    pub _106: exact_8bytes,
    pub _107: padded_8bytes,
    pub _200: ff_periodic_effect,
    pub _201: ib_wc,
    pub _202: acpi_object_method,
    pub _203: outer_mixed_but_unpacked,
}

#[no_mangle]
pub unsafe extern "C" fn f(_: *mut f_arg) -> c_int {
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
