/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency supplied externally by the surrounding translation unit:
// typedef __be32 rtas_arg_t;
pub type RtasArgT = __be32;

#[repr(C, align(8))]
pub struct rtas_args {
	pub token: __be32,
	pub nargs: __be32,
	pub nret: __be32,
	pub args: [RtasArgT; 16],
	pub rets: *mut RtasArgT, /* Pointer to return values in args[]. */
}

#[repr(C)]
pub struct rtas_t {
	pub entry: ::core::primitive::usize, /* physical address pointer */
	pub base: ::core::primitive::usize,  /* physical address pointer */
	pub size: ::core::primitive::usize,
	pub dev: *mut device_node, /* virtual address pointer */
}

#[repr(C)]
pub struct rtas_error_log {
	/* Byte 0 */
	pub byte0: u8, /* Architectural version */

	/* Byte 1 */
	pub byte1: u8,
	/* XXXXXXXX
	 * XXX	3: Severity level of error
	 *    XX	2: Degree of recovery
	 *      X	1: Extended log present?
	 *       XX	2: Reserved
	 */

	/* Byte 2 */
	pub byte2: u8,
	/* XXXXXXXX
	 * XXXX		4: Initiator of event
	 *     XXXX	4: Target of failed operation
	 */
	pub byte3: u8, /* General event or error*/
	pub extended_log_length: __be32, /* length in bytes */

	/* Start of extended log, variable length */
	pub buffer: [u8; 0], // __counted_by_be(extended_log_length)
}

/* RTAS general extended event log, Version 6. The extended log starts
 * from "buffer" field of struct rtas_error_log defined above.
 */
#[repr(C)]
pub struct rtas_ext_event_log_v6 {
	/* Byte 0 */
	pub byte0: u8,
	/* XXXXXXXX
	 * X	1: Log valid
	 *  X	1: Unrecoverable error
	 *   X	1: Recoverable (correctable or successfully retried)
	 *    X	1: Bypassed unrecoverable error (degraded operation)
	 *     X	1: Predictive error
	 *      X	1: "New" log (always 1 for data returned from RTAS)
	 *       X	1: Big Endian
	 *        X	1: Reserved
	 */

	/* Byte 1 */
	pub byte1: u8, /* reserved */

	/* Byte 2 */
	pub byte2: u8,
	/* XXXXXXXX
	 * X	1: Set to 1 (indicating log is in PowerPC format)
	 *  XXX	3: Reserved
	 *     XXXX	4: Log format used for bytes 12-2047
	 */

	/* Byte 3 */
	pub byte3: u8, /* reserved */
	/* Byte 4-11 */
	pub reserved: [u8; 8], /* reserved */
	/* Byte 12-15 */
	pub company_id: __be32, /* Company ID of the company */
	/* that defines the format for */
	/* the vendor specific log type */
	/* Byte 16-end of log */
	pub vendor_log: [u8; 1], /* Start of vendor specific log */
	/* Variable length. */
}

/* Vendor specific Platform Event Log Format, Version 6, section header */
#[repr(C)]
pub struct pseries_errorlog {
	pub id: __be16, /* 0x00 2-byte ASCII section ID */
	pub length: __be16, /* 0x02 Section length in bytes */
	pub version: u8, /* 0x04 Section version */
	pub subtype: u8, /* 0x05 Section subtype */
	pub creator_component: __be16, /* 0x06 Creator component ID */
	pub data: [u8; 0], /* 0x08 Start of section data */
}

#[repr(C)]
pub struct pseries_hp_errorlog_ic {
	pub count: __be32,
	pub index: __be32,
}

#[repr(C)]
pub union pseries_hp_errorlog_drc_u {
	pub drc_index: __be32,
	pub drc_count: __be32,
	pub ic: pseries_hp_errorlog_ic,
	pub drc_name: [::core::ffi::c_char; 1],
}

/* RTAS pseries hotplug errorlog section */
#[repr(C)]
pub struct pseries_hp_errorlog {
	pub resource: u8,
	pub action: u8,
	pub id_type: u8,
	pub reserved: u8,
	pub _drc_u: pseries_hp_errorlog_drc_u,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
