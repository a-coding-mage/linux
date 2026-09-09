// Faithful low-level Rust translation of actbl1.h.


/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: actbl1.h - Additional ACPI table definitions
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/


/*******************************************************************************
 *
 * Additional ACPI Tables
 *
 * These tables are not consumed directly by the ACPICA subsystem, but are
 * included here to support device drivers and the AML disassembler.
 *
 ******************************************************************************/

/*
 * Values for description table header signatures for tables defined in this
 * file. Useful because they make it more difficult to inadvertently type in
 * the wrong signature.
 */
pub const ACPI_SIG_AEST: &[u8] = b"AEST";
pub const ACPI_SIG_ASF: &[u8] = b"ASF!";
pub const ACPI_SIG_ASPT: &[u8] = b"ASPT";
pub const ACPI_SIG_BERT: &[u8] = b"BERT";
pub const ACPI_SIG_BGRT: &[u8] = b"BGRT";
pub const ACPI_SIG_BOOT: &[u8] = b"BOOT";
pub const ACPI_SIG_CEDT: &[u8] = b"CEDT";
pub const ACPI_SIG_CPEP: &[u8] = b"CPEP";
pub const ACPI_SIG_CSRT: &[u8] = b"CSRT";
pub const ACPI_SIG_DBG2: &[u8] = b"DBG2";
pub const ACPI_SIG_DBGP: &[u8] = b"DBGP";
pub const ACPI_SIG_DMAR: &[u8] = b"DMAR";
pub const ACPI_SIG_DRTM: &[u8] = b"DRTM";
pub const ACPI_SIG_DTPR: &[u8] = b"DTPR";
pub const ACPI_SIG_ECDT: &[u8] = b"ECDT";
pub const ACPI_SIG_EINJ: &[u8] = b"EINJ";
pub const ACPI_SIG_ERST: &[u8] = b"ERST";
pub const ACPI_SIG_FPDT: &[u8] = b"FPDT";
pub const ACPI_SIG_GTDT: &[u8] = b"GTDT";
pub const ACPI_SIG_HEST: &[u8] = b"HEST";
pub const ACPI_SIG_HMAT: &[u8] = b"HMAT";
pub const ACPI_SIG_HPET: &[u8] = b"HPET";
pub const ACPI_SIG_IBFT: &[u8] = b"IBFT";
pub const ACPI_SIG_MSCT: &[u8] = b"MSCT";
pub const ACPI_SIG_S3PT: &[u8] = b"S3PT";
pub const ACPI_SIG_PCCS: &[u8] = b"PCC";
pub const ACPI_SIG_NBFT: &[u8] = b"NBFT";

/* Reserved table signatures */
pub const ACPI_SIG_MATR: &[u8] = b"MATR";
pub const ACPI_SIG_MSDM: &[u8] = b"MSDM";

/*
 * These tables have been seen in the field, but no definition has been found
 */
// Conditional declarations retained under the ACPI_UNDEFINED_TABLES build condition.

pub const ACPI_SIG_ATKG: &[u8] = b"ATKG";
pub const ACPI_SIG_GSCI: &[u8] = b"GSCI";
pub const ACPI_SIG_IEIT: &[u8] = b"IEIT";
 * All tables must be byte-packed to match the ACPI specification, since
 * the tables are provided by the system BIOS.
 */
// C source uses #pragma pack(1); all translated structures retain packed layout.

/*
 * Note: C bitfields are not used for this reason:
 *
 * "Bitfields are great and easy to read, but unfortunately the C language
 * does not specify the layout of bitfields in memory, which means they are
 * essentially useless for dealing with packed data in on-disk formats or
 * binary wire protocols." (Or ACPI tables and buffers.) "If you ask me,
 * this decision was a design error in C. Ritchie could have picked an order
 * and stuck with it." Norman Ramsey.
 * See http://stackoverflow.com/a/1053662/41661
 */

/*******************************************************************************
 *
 * Common subtable headers
 *
 ******************************************************************************/

/* Generic subtable header (used in MADT, SRAT, etc.) */


pub acpi_subtable_header {
	u8: type;
	u8: length
};

/* Subtable header for WHEA tables (EINJ, ERST, WDAT) */


pub acpi_whea_header {
	u8: action;
	u8: instruction;
	u8: flags;
	u8: reserved;
	register_region: acpi_generic_address;
	u64: value;		/* Value used with Read/Write register */
	u64: mask;		/* Bitmask required for this register instruction */
};

/* https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/acpitabl/ns-acpitabl-aspt_table */
pub const ASPT_REVISION_ID: u64 = 0x01;

pub acpi_table_aspt {
	header: acpi_table_header;
	u32: num_entries
};


pub acpi_aspt_header {
	u16: type;
	u16: length
};


pub acpi_aspt_type {
    ACPI_ASPT_TYPE_GLOBAL_REGS = 0,
    ACPI_ASPT_TYPE_SEV_MBOX_REGS = 1,
    ACPI_ASPT_TYPE_ACPI_MBOX_REGS = 2,
};

/* 0: ASPT Global Registers */

pub acpi_aspt_global_regs {
	header: acpi_aspt_header;
	u32: reserved;
	u64: feature_reg_addr;
	u64: irq_en_reg_addr;
	u64: irq_st_reg_addr
};

/* 1: ASPT SEV Mailbox Registers */

pub acpi_aspt_sev_mbox_regs {
	header: acpi_aspt_header;
	u8: mbox_irq_id;
	u8: [reserved; 3];
	u64: cmd_resp_reg_addr;
	u64: cmd_buf_lo_reg_addr;
	u64: cmd_buf_hi_reg_addr
};

/* 2: ASPT ACPI Mailbox Registers */

pub acpi_aspt_acpi_mbox_regs {
	header: acpi_aspt_header;
	u32: reserved1;
	u64: cmd_resp_reg_addr;
	u64: [reserved2; 2]
};

/* Larger subtable header (when Length can exceed 255) */


pub acpi_subtbl_hdr_16 {
	u16: type;
	u16: length
};

/*******************************************************************************
 *
 * ASF - Alert Standard Format table (Signature "ASF!")
 *       Revision 0x10
 *
 * Conforms to the Alert Standard Format Specification V2.0, 23 April 2003
 *
 ******************************************************************************/


pub acpi_table_asf {
	header: acpi_table_header;	/* Common ACPI table header */
};

/* ASF subtable header */


pub acpi_asf_header {
	u8: type;
	u8: reserved;
	u16: length
};

/* Values for Type field above */


pub acpi_asf_type {
    ACPI_ASF_TYPE_INFO = 0,
    ACPI_ASF_TYPE_ALERT = 1,
    ACPI_ASF_TYPE_CONTROL = 2,
    ACPI_ASF_TYPE_BOOT = 3,
    ACPI_ASF_TYPE_ADDRESS = 4,
    ACPI_ASF_TYPE_RESERVED = 5
};

/*
 * ASF subtables
 */

/* 0: ASF Information */
,

pub acpi_asf_info {
	header: acpi_asf_header;
	u8: min_reset_value;
	u8: min_poll_interval;
	u16: system_id;
	u32: mfg_id;
	u8: flags;
	u8: [reserved2; 3]
};

/* Masks for Flags field above */
pub const ACPI_ASF_SMBUS_PROTOCOLS: u64 = 1;


pub acpi_asf_alert {
	header: acpi_asf_header;
	u8: assert_mask;
	u8: deassert_mask;
	u8: alerts;
	u8: data_length
};


pub acpi_asf_alert_data {
	u8: address;
	u8: command;
	u8: mask;
	u8: value;
	u8: sensor_type;
	u8: type;
	u8: offset;
	u8: source_type;
	u8: severity;
	u8: sensor_number;
	u8: entity;
	u8: instance
};

/* 2: ASF Remote Control */


pub acpi_asf_remote {
	header: acpi_asf_header;
	u8: controls;
	u8: data_length;
	u16: reserved2
};


pub acpi_asf_control_data {
	u8: function;
	u8: address;
	u8: command;
	u8: value
};

/* 3: ASF RMCP Boot Options */


pub acpi_asf_rmcp {
	header: acpi_asf_header;
	u8: [capabilities; 7];
	u8: completion_code;
	u32: enterprise_id;
	u8: command;
	u16: parameter;
	u16: boot_options;
	u16: oem_parameters
};

/* 4: ASF Address */


pub acpi_asf_address {
	header: acpi_asf_header;
	u8: eprom_address;
	u8: devices
};

/*******************************************************************************
 *
 * BERT - Boot Error Record Table (ACPI 4.0)
 *        Version 1
 *
 ******************************************************************************/


pub acpi_table_bert {
	header: acpi_table_header;	/* Common ACPI table header */
	u32: region_length;	/* Length of the boot error region */
	u64: address;		/* Physical address of the error region */
};

/* Boot Error Region (not a subtable, pointed to by Address field above) */


pub acpi_bert_region {
	u32: block_status;	/* Type of error information */
	u32: raw_data_offset;	/* Offset to raw error data */
	u32: raw_data_length;	/* Length of raw error data */
	u32: data_length;	/* Length of generic error data */
	u32: error_severity;	/* Severity code */
};

/* Values for block_status flags above */
pub const ACPI_BERT_UNCORRECTABLE: u64 = 1;
pub const ACPI_BERT_CORRECTABLE: u64 = 1<<1;
pub const ACPI_BERT_MULTIPLE_UNCORRECTABLE: u64 = 1<<2;
pub const ACPI_BERT_MULTIPLE_CORRECTABLE: u64 = 1<<3;
pub const ACPI_BERT_ERROR_ENTRY_COUNT: u64 = 0xFF<<4;

/* Values for error_severity above */


pub acpi_bert_error_severity {
    ACPI_BERT_ERROR_CORRECTABLE = 0,
    ACPI_BERT_ERROR_FATAL = 1,
    ACPI_BERT_ERROR_CORRECTED = 2,
    ACPI_BERT_ERROR_NONE = 3,
    ACPI_BERT_ERROR_RESERVED = 4	/* 4 and greater are reserved */
};

/*
 * Note: The generic error data that follows the error_severity field above
 * uses the acpi_hest_generic_data defined under the HEST table below
 */

/*******************************************************************************
 *
 * BGRT - Boot Graphics Resource Table (ACPI 5.0)
 *        Version 1
 *
 ******************************************************************************/
,

pub acpi_table_bgrt {
	header: acpi_table_header;	/* Common ACPI table header */
	u16: version;
	u8: status;
	u8: image_type;
	u64: image_address;
	u32: image_offset_x;
	u32: image_offset_y
};

/* Flags for Status field above */
pub const ACPI_BGRT_DISPLAYED: u64 = 1;
pub const ACPI_BGRT_ORIENTATION_OFFSET: u64 = 3 << 1;
 *
 * BOOT - Simple Boot Flag Table
 *        Version 1
 *
 * Conforms to the "Simple Boot Flag Specification", Version 2.1
 *
 ******************************************************************************/


pub acpi_table_boot {
	header: acpi_table_header;	/* Common ACPI table header */
	u8: cmos_index;		/* Index in CMOS RAM for the boot register */
	u8: [reserved; 3]
};

/*******************************************************************************
 *
 * CDAT - Coherent Device Attribute Table
 *        Version 1
 *
 * Conforms to the "Coherent Device Attribute Table (CDAT) Specification
 " (Revision 1.01, October 2020.)
 *
 ******************************************************************************/


pub acpi_table_cdat {
	u32: length;		/* Length of table in bytes, including this header */
	u8: revision;		/* ACPI Specification minor version number */
	u8: checksum;		/* To make sum of entire table == 0 */
	u8: [reserved; 6];
	u32: sequence;		/* Used to detect runtime CDAT table changes */
};

/* CDAT common subtable header */


pub acpi_cdat_header {
	u8: type;
	u8: reserved;
	u16: length
};

/* Values for Type field above */


pub acpi_cdat_type {
    ACPI_CDAT_TYPE_DSMAS = 0,
    ACPI_CDAT_TYPE_DSLBIS = 1,
    ACPI_CDAT_TYPE_DSMSCIS = 2,
    ACPI_CDAT_TYPE_DSIS = 3,
    ACPI_CDAT_TYPE_DSEMTS = 4,
    ACPI_CDAT_TYPE_SSLBIS = 5,
    ACPI_CDAT_TYPE_RESERVED = 6	/* 6 through 0xFF are reserved */
};

/* Subtable 0: Device Scoped Memory Affinity Structure (DSMAS) */
,

pub acpi_cdat_dsmas {
	u8: dsmad_handle;
	u8: flags;
	u16: reserved;
	u64: dpa_base_address;
	u64: dpa_length
};

/* Flags for subtable above */
pub const ACPI_CDAT_DSMAS_NON_VOLATILE: u64 = 1 << 2;
pub const ACPI_CDAT_DSMAS_SHAREABLE: u64 = 1 << 3;
pub const ACPI_CDAT_DSMAS_READ_ONLY: u64 = 1 << 6;


pub acpi_cdat_dslbis {
	u8: handle;
	u8: flags;		/* If Handle matches a DSMAS handle, the definition of this field matches
				 * Flags field in HMAT System Locality Latency */
	u8: data_type;
	u8: reserved;
	u64: entry_base_unit;
	u16: [entry; 3];
	u16: reserved2
};

/* Subtable 2: Device Scoped Memory Side Cache Information Structure (DSMSCIS) */


pub acpi_cdat_dsmscis {
	u8: dsmas_handle;
	u8: [reserved; 3];
	u64: side_cache_size;
	u32: cache_attributes
};

/* Subtable 3: Device Scoped Initiator Structure (DSIS) */


pub acpi_cdat_dsis {
	u8: flags;
	u8: handle;
	u16: reserved
};

/* Flags for above subtable */
pub const ACPI_CDAT_DSIS_MEM_ATTACHED: u64 = 1 << 0;


pub acpi_cdat_dsemts {
	u8: dsmas_handle;
	u8: memory_type;
	u16: reserved;
	u64: dpa_offset;
	u64: range_length
};

/* Subtable 5: Switch Scoped Latency and Bandwidth Information Structure (SSLBIS) */


pub acpi_cdat_sslbis {
	u8: data_type;
	u8: [reserved; 3];
	u64: entry_base_unit
};

/* Sub-subtable for above, sslbe_entries field */


pub acpi_cdat_sslbe {
	u16: portx_id;
	u16: porty_id;
	u16: latency_or_bandwidth;
	u16: reserved
};
pub const ACPI_CDAT_SSLBIS_US_PORT: u64 = 0x0100;
pub const ACPI_CDAT_SSLBIS_ANY_PORT: u64 = 0xffff;
 *
 * CEDT - CXL Early Discovery Table
 *        Version 1
 *
 * Conforms to the "CXL Early Discovery Table" (CXL 2.0, October 2020)
 *
 ******************************************************************************/


pub acpi_table_cedt {
	header: acpi_table_header;	/* Common ACPI table header */
};

/* CEDT subtable header (Performance Record Structure) */


pub acpi_cedt_header {
	u8: type;
	u8: reserved;
	u16: length
};

/* Values for Type field above */


pub acpi_cedt_type {
    ACPI_CEDT_TYPE_CHBS = 0,
    ACPI_CEDT_TYPE_CFMWS = 1,
    ACPI_CEDT_TYPE_CXIMS = 2,
    ACPI_CEDT_TYPE_RDPAS = 3,
    ACPI_CEDT_TYPE_RESERVED = 4,
};

/* Values for version field above */
pub const ACPI_CEDT_CHBS_VERSION_CXL11: u64 = 0;
pub const ACPI_CEDT_CHBS_VERSION_CXL20: u64 = 1;
pub const ACPI_CEDT_CHBS_LENGTH_CXL11: u64 = 0x2000;
pub const ACPI_CEDT_CHBS_LENGTH_CXL20: u64 = 0x10000;
 * CEDT subtables
 */

/* 0: CXL Host Bridge Structure */


pub acpi_cedt_chbs {
	header: acpi_cedt_header;
	u32: uid;
	u32: cxl_version;
	u32: reserved;
	u64: base;
	u64: length
};

/* 1: CXL Fixed Memory Window Structure */


pub acpi_cedt_cfmws {
	header: acpi_cedt_header;
	u32: reserved1;
	u64: base_hpa;
	u64: window_size;
	u8: interleave_ways;
	u8: interleave_arithmetic;
	u16: reserved2;
	u32: granularity;
	u16: restrictions;
	u16: qtg_id;
	u32 interleave_targets: [u8; 0]
};


pub acpi_cedt_cfmws_target_element {
	u32: interleave_target
};

/* Values for Interleave Arithmetic field above */
pub const ACPI_CEDT_CFMWS_ARITHMETIC_MODULO: u64 = 0;
pub const ACPI_CEDT_CFMWS_ARITHMETIC_XOR: u64 = 1;
pub const ACPI_CEDT_CFMWS_RESTRICT_DEVMEM: u64 = 1;
pub const ACPI_CEDT_CFMWS_RESTRICT_HOSTONLYMEM: u64 = 1<<1;
pub const ACPI_CEDT_CFMWS_RESTRICT_VOLATILE: u64 = 1<<2;
pub const ACPI_CEDT_CFMWS_RESTRICT_PMEM: u64 = 1<<3;
pub const ACPI_CEDT_CFMWS_RESTRICT_FIXED: u64 = 1<<4;
pub const ACPI_CEDT_CFMWS_RESTRICT_BI: u64 = 1<<5;


pub acpi_cedt_cxims {
	header: acpi_cedt_header;
	u16: reserved1;
	u8: hbig;
	u8: nr_xormaps;
	u64 xormap_list: [u8; 0]
};


pub acpi_cedt_cxims_target_element {
	u64: xormap
};

/* 3: CXL RCEC Downstream Port Association Structure */


pub acpi_cedt_rdpas {
	header: acpi_cedt_header;
	u16: segment;
	u16: bdf;
	u8: protocol;
	u64: address
};

/* Masks for bdf field above */
pub const ACPI_CEDT_RDPAS_BUS_MASK: u64 = 0xff00;
pub const ACPI_CEDT_RDPAS_DEVICE_MASK: u64 = 0x00f8;
pub const ACPI_CEDT_RDPAS_FUNCTION_MASK: u64 = 0x0007;
pub const ACPI_CEDT_RDPAS_PROTOCOL_IO: u64 = 0;
pub const ACPI_CEDT_RDPAS_PROTOCOL_CACHEMEM: u64 = 1;
 *
 * CPEP - Corrected Platform Error Polling table (ACPI 4.0)
 *        Version 1
 *
 ******************************************************************************/


pub acpi_table_cpep {
	header: acpi_table_header;	/* Common ACPI table header */
	u64: reserved
};

/* Subtable */


pub acpi_cpep_polling {
	header: acpi_subtable_header;
	u8: id;			/* Processor ID */
	u8: eid;			/* Processor EID */
	u32: interval;		/* Polling interval (msec) */
};

/*******************************************************************************
 *
 * CSRT - Core System Resource Table
 *        Version 0
 *
 * Conforms to the "Core System Resource Table (CSRT)", November 14, 2011
 *
 ******************************************************************************/


pub acpi_table_csrt {
	header: acpi_table_header;	/* Common ACPI table header */
};

/* Resource Group subtable */


pub acpi_csrt_group {
	u32: length;
	u32: vendor_id;
	u32: subvendor_id;
	u16: device_id;
	u16: subdevice_id;
	u16: revision;
	u16: reserved;
	u32: shared_info_length;

	/* Shared data immediately follows (Length = shared_info_length) */
};

/* Shared Info subtable */


pub acpi_csrt_shared_info {
	u16: major_version;
	u16: minor_version;
	u32: mmio_base_low;
	u32: mmio_base_high;
	u32: gsi_interrupt;
	u8: interrupt_polarity;
	u8: interrupt_mode;
	u8: num_channels;
	u8: dma_address_width;
	u16: base_request_line;
	u16: num_handshake_signals;
	u32: max_block_size;

	/* Resource descriptors immediately follow (Length = Group length - shared_info_length) */
};

/* Resource Descriptor subtable */


pub acpi_csrt_descriptor {
	u32: length;
	u16: type;
	u16: subtype;
	u32: uid;

	/* Resource-specific information immediately follows */
};

/* Resource Types */
pub const ACPI_CSRT_TYPE_INTERRUPT: u64 = 0x0001;
pub const ACPI_CSRT_TYPE_TIMER: u64 = 0x0002;
pub const ACPI_CSRT_TYPE_DMA: u64 = 0x0003;
pub const ACPI_CSRT_XRUPT_LINE: u64 = 0x0000;
pub const ACPI_CSRT_XRUPT_CONTROLLER: u64 = 0x0001;
pub const ACPI_CSRT_TIMER: u64 = 0x0000;
pub const ACPI_CSRT_DMA_CHANNEL: u64 = 0x0000;
pub const ACPI_CSRT_DMA_CONTROLLER: u64 = 0x0001;
 *
 * DBG2 - Debug Port Table 2
 *        Version 0 (Both main table and subtables)
 *
 * Conforms to "Microsoft Debug Port Table 2 (DBG2)", September 21, 2020
 *
 ******************************************************************************/


pub acpi_table_dbg2 {
	header: acpi_table_header;	/* Common ACPI table header */
	u32: info_offset;
	u32: info_count
};


pub acpi_dbg2_header {
	u32: info_offset;
	u32: info_count
};

/* Debug Device Information Subtable */


pub acpi_dbg2_device {
	u8: revision;
	u16: length;
	u8: register_count;	/* Number of base_address registers */
	u16: namepath_length;
	u16: namepath_offset;
	u16: oem_data_length;
	u16: oem_data_offset;
	u16: port_type;
	u16: port_subtype;
	u16: reserved;
	u16: base_address_offset;
	u16: address_size_offset;
	/*
	 * Data that follows:
	 *    base_address (required) - Each in 12-byte Generic Address Structure format.
	 *    address_size (required) - Array of u32 sizes corresponding to each base_address register.
	 *    Namepath    (required) - Null terminated string. Single dot if not supported.
	 *    oem_data    (optional) - Length is oem_data_length.
	 */
};

/* Types for port_type field above */
pub const ACPI_DBG2_SERIAL_PORT: u64 = 0x8000;
pub const ACPI_DBG2_1394_PORT: u64 = 0x8001;
pub const ACPI_DBG2_USB_PORT: u64 = 0x8002;
pub const ACPI_DBG2_NET_PORT: u64 = 0x8003;
pub const ACPI_DBG2_16550_COMPATIBLE: u64 = 0x0000;
pub const ACPI_DBG2_16550_SUBSET: u64 = 0x0001;
pub const ACPI_DBG2_MAX311XE_SPI: u64 = 0x0002;
pub const ACPI_DBG2_ARM_PL011: u64 = 0x0003;
pub const ACPI_DBG2_MSM8X60: u64 = 0x0004;
pub const ACPI_DBG2_16550_NVIDIA: u64 = 0x0005;
pub const ACPI_DBG2_TI_OMAP: u64 = 0x0006;
pub const ACPI_DBG2_APM88XXXX: u64 = 0x0008;
pub const ACPI_DBG2_MSM8974: u64 = 0x0009;
pub const ACPI_DBG2_SAM5250: u64 = 0x000A;
pub const ACPI_DBG2_INTEL_USIF: u64 = 0x000B;
pub const ACPI_DBG2_IMX6: u64 = 0x000C;
pub const ACPI_DBG2_ARM_SBSA_32BIT: u64 = 0x000D;
pub const ACPI_DBG2_ARM_SBSA_GENERIC: u64 = 0x000E;
pub const ACPI_DBG2_ARM_DCC: u64 = 0x000F;
pub const ACPI_DBG2_BCM2835: u64 = 0x0010;
pub const ACPI_DBG2_SDM845_1_8432MHZ: u64 = 0x0011;
pub const ACPI_DBG2_16550_WITH_GAS: u64 = 0x0012;
pub const ACPI_DBG2_SDM845_7_372MHZ: u64 = 0x0013;
pub const ACPI_DBG2_INTEL_LPSS: u64 = 0x0014;
pub const ACPI_DBG2_RISCV_SBI_CON: u64 = 0x0015;
pub const ACPI_DBG2_1394_STANDARD: u64 = 0x0000;
pub const ACPI_DBG2_USB_XHCI: u64 = 0x0000;
pub const ACPI_DBG2_USB_EHCI: u64 = 0x0001;
 *
 * DBGP - Debug Port table
 *        Version 1
 *
 * Conforms to the "Debug Port Specification", Version 1.00, 2/9/2000
 *
 ******************************************************************************/


pub acpi_table_dbgp {
	header: acpi_table_header;	/* Common ACPI table header */
	u8: type;		/* 0=full 16550, 1=subset of 16550 */
	u8: [reserved; 3];
	debug_port: acpi_generic_address
};

/*******************************************************************************
 *
 * DMAR - DMA Remapping table
 *        Version 1
 *
 * Conforms to "Intel Virtualization Technology for Directed I/O",
 * Version 2.3, October 2014
 *
 ******************************************************************************/


pub acpi_table_dmar {
	header: acpi_table_header;	/* Common ACPI table header */
	u8: width;		/* Host Address Width */
	u8: flags;
	u8: [reserved; 10]
};

/* Masks for Flags field above */
pub const ACPI_DMAR_INTR_REMAP: u64 = 1;
pub const ACPI_DMAR_X2APIC_OPT_OUT: u64 = 1<<1;
pub const ACPI_DMAR_X2APIC_MODE: u64 = 1<<2;


pub acpi_dmar_header {
	u16: type;
	u16: length
};

/* Values for subtable type in acpi_dmar_header */


pub acpi_dmar_type {
    ACPI_DMAR_TYPE_HARDWARE_UNIT = 0,
    ACPI_DMAR_TYPE_RESERVED_MEMORY = 1,
    ACPI_DMAR_TYPE_ROOT_ATS = 2,
    ACPI_DMAR_TYPE_HARDWARE_AFFINITY = 3,
    ACPI_DMAR_TYPE_NAMESPACE = 4,
    ACPI_DMAR_TYPE_SATC = 5,
    ACPI_DMAR_TYPE_SIDP = 6,
    ACPI_DMAR_TYPE_RESERVED = 7	/* 7 and greater are reserved */
};

/* DMAR Device Scope structure */
,

pub acpi_dmar_device_scope {
	u8: entry_type;
	u8: length;
	u8: flags;
	u8: reserved;
	u8: enumeration_id;
	u8: bus
};

/* Values for entry_type in acpi_dmar_device_scope - device types */


pub acpi_dmar_scope_type {
    ACPI_DMAR_SCOPE_TYPE_NOT_USED = 0,
    ACPI_DMAR_SCOPE_TYPE_ENDPOINT = 1,
    ACPI_DMAR_SCOPE_TYPE_BRIDGE = 2,
    ACPI_DMAR_SCOPE_TYPE_IOAPIC = 3,
    ACPI_DMAR_SCOPE_TYPE_HPET = 4,
    ACPI_DMAR_SCOPE_TYPE_NAMESPACE = 5,
    ACPI_DMAR_SCOPE_TYPE_RESERVED = 6	/* 6 and greater are reserved */
};
,

pub acpi_dmar_pci_path {
	u8: device;
	u8: function
};

/*
 * DMAR Subtables, correspond to Type in acpi_dmar_header
 */

/* 0: Hardware Unit Definition */


pub acpi_dmar_hardware_unit {
	header: acpi_dmar_header;
	u8: flags;
	u8: size;		/* Size of the register set */
	u16: segment;
	u64: address;		/* Register Base Address */
};

/* Masks for Flags field above */
pub const ACPI_DMAR_INCLUDE_ALL: u64 = 1;


pub acpi_dmar_reserved_memory {
	header: acpi_dmar_header;
	u16: reserved;
	u16: segment;
	u64: base_address;	/* 4K aligned base address */
	u64: end_address;	/* 4K aligned limit address */
};

/* Masks for Flags field above */
pub const ACPI_DMAR_ALLOW_ALL: u64 = 1;


pub acpi_dmar_atsr {
	header: acpi_dmar_header;
	u8: flags;
	u8: reserved;
	u16: segment
};

/* Masks for Flags field above */
pub const ACPI_DMAR_ALL_PORTS: u64 = 1;


pub acpi_dmar_rhsa {
	header: acpi_dmar_header;
	u32: reserved;
	u64: base_address;
	u32: proximity_domain
};

/* 4: ACPI Namespace Device Declaration Structure */


pub acpi_dmar_andd {
	header: acpi_dmar_header;
	u8: [reserved; 3];
	u8: device_number;

pub union __AcpiAnonymousUnion {
		__pad: core::ffi::c_char;
		 device_name: [core::ffi::c_char; 0]
	}
};

/* 5: SOC Integrated Address Translation Cache Reporting Structure */


pub acpi_dmar_satc {
	header: acpi_dmar_header;
	u8: flags;
	u8: reserved;
	u16: segment
};

/* 6: so_c Integrated Device Property Reporting Structure */


pub acpi_dmar_sidp {
	header: acpi_dmar_header;
	u16: reserved;
	u16: segment
};

/*******************************************************************************
 *
 * DRTM - Dynamic Root of Trust for Measurement table
 * Conforms to "TCG D-RTM Architecture" June 17 2013, Version 1.0.0
 * Table version 1
 *
 ******************************************************************************/


pub acpi_table_drtm {
	header: acpi_table_header;	/* Common ACPI table header */
	u64: entry_base_address;
	u64: entry_length;
	u32: entry_address32;
	u64: entry_address64;
	u64: exit_address;
	u64: log_area_address;
	u32: log_area_length;
	u64: arch_dependent_address;
	u32: flags
};

/* Flag Definitions for above */
pub const ACPI_DRTM_ACCESS_ALLOWED: u64 = 1;
pub const ACPI_DRTM_ENABLE_GAP_CODE: u64 = 1<<1;
pub const ACPI_DRTM_INCOMPLETE_MEASUREMENTS: u64 = 1<<2;
pub const ACPI_DRTM_AUTHORITY_ORDER: u64 = 1<<3;


pub acpi_drtm_vtable_list {
	u32: validated_table_count;
	u64 validated_tables: [u8; 0]
};

/* 2) Resources List (of Resource Descriptors) */

/* Resource Descriptor */


pub acpi_drtm_resource {
	u8: [size; 7];
	u8: type;
	u64: address
};


pub acpi_drtm_resource_list {
	u32: resource_count;
	acpi_drtm_resource resources: [u8; 0]
};

/* 3) Platform-specific Identifiers List */


pub acpi_drtm_dps_id {
	u32: dps_id_length;
	u8: [dps_id; 16]
};

/*******************************************************************************
 *
 * DTPR - DMA TXT Protection Ranges Table
 *        Version 1
 *
 * Conforms to "Intel® Trusted Execution Technology (Intel® TXT) DMA Protection
 *              Ranges",
 * Revision 0.73, August 2021
 *
 ******************************************************************************/


pub acpi_table_dtpr {
	header: acpi_table_header;
	u32: flags;		/* 36 */
	u32: ins_cnt
};


pub acpi_tpr_array {
	u64: base
};


pub acpi_tpr_instance {
	u32: flags;
	u32: tpr_cnt
};


pub acpi_tpr_aux_sr {
	u32: srl_cnt
};

/*
 * TPRn_BASE (ACPI_TPRN_BASE_REG)
 *
 * Specifies the start address of TPRn region. TPR region address and size must
 * be with 1MB resolution. These bits are compared with the result of the
 * TPRn_LIMIT[63:20], which is applied to the incoming address, to
 * determine if an access fall within the TPRn defined region.
 *
 * Minimal TPRn_Base resolution is 1MB. Applied to the incoming address, to
 * determine if an access fall within the TPRn defined region. Width is
 * determined by a bus width which can be obtained via CPUID
 * function 0x80000008.
 */

pub type ACPI_TPRN_BASE_REG = u64;

/* TPRn_BASE Register Bit Masks */

/* Bit 3 - RW: access: 1 == RO, 0 == RW register (for TPR must be RW) */
pub const ACPI_TPRN_BASE_RW_SHIFT: u64 = 3;
pub const ACPI_TPRN_BASE_RW_MASK: u64 = ((u64) 1 << ACPI_TPRN_BASE_RW_SHIFT);

/*
 * Bit 4 - Enable: 0 – TPRn address enabled: range;
 *                 1 – TPRn address range disabled.
 */
pub const ACPI_TPRN_BASE_ENABLE_SHIFT: u64 = 4;
pub const ACPI_TPRN_BASE_ENABLE_MASK: u64 = ((u64) 1 << ACPI_TPRN_BASE_ENABLE_SHIFT);

/* Bits 63:20 - tpr_base_rw */
pub const ACPI_TPRN_BASE_ADDR_SHIFT: u64 = 20;
pub const ACPI_TPRN_BASE_ADDR_MASK: u64 = ((u64) 0xFFFFFFFFFFF << \;
			 ACPI_TPRN_BASE_ADDR_SHIFT)

/* TPRn_BASE Register Bit Handlers*/

/*
 * GET_TPRN_BASE_RW:
 *
 * Read RW bit from TPRn Base register - bit 3.
 *
 * Input:
 * - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
 *
 * Output:
 *
 * Returns RW bit value (u64).
 */

pub unsafe fn GET_TPRN_BASE_RW(reg: u64) -> u64 { match "GET_TPRN_BASE_RW" { "GET_TPRN_BASE_RW" => (reg & ACPI_TPRN_BASE_RW_MASK) >> ACPI_TPRN_BASE_RW_SHIFT, "GET_TPRN_BASE_ENABLE" => (reg & ACPI_TPRN_BASE_ENABLE_MASK) >> ACPI_TPRN_BASE_ENABLE_SHIFT, "GET_TPRN_BASE_ADDR" => (reg & ACPI_TPRN_BASE_ADDR_MASK) >> ACPI_TPRN_BASE_ADDR_SHIFT, "GET_TPRN_LIMIT_RW" => (reg & ACPI_TPRN_LIMIT_RW_MASK) >> ACPI_TPRN_LIMIT_RW_SHIFT, _ => (reg & ACPI_TPRN_LIMIT_ADDR_MASK) >> ACPI_TPRN_LIMIT_ADDR_SHIFT } }
					  ACPI_TPRN_BASE_RW_SHIFT)

/*
 * GET_TPRN_BASE_ENABLE:
 *
 * Read Enable bit from TPRn Base register - bit 4.
 *
 * Input:
 * - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
 *
 * Output:
 *
 * Returns Enable bit value (u64).
 */

pub unsafe fn GET_TPRN_BASE_ENABLE(reg: u64) -> u64 { match "GET_TPRN_BASE_ENABLE" { "GET_TPRN_BASE_RW" => (reg & ACPI_TPRN_BASE_RW_MASK) >> ACPI_TPRN_BASE_RW_SHIFT, "GET_TPRN_BASE_ENABLE" => (reg & ACPI_TPRN_BASE_ENABLE_MASK) >> ACPI_TPRN_BASE_ENABLE_SHIFT, "GET_TPRN_BASE_ADDR" => (reg & ACPI_TPRN_BASE_ADDR_MASK) >> ACPI_TPRN_BASE_ADDR_SHIFT, "GET_TPRN_LIMIT_RW" => (reg & ACPI_TPRN_LIMIT_RW_MASK) >> ACPI_TPRN_LIMIT_RW_SHIFT, _ => (reg & ACPI_TPRN_LIMIT_ADDR_MASK) >> ACPI_TPRN_LIMIT_ADDR_SHIFT } }
							 >> ACPI_TPRN_BASE_ENABLE_SHIFT)

/*
 * GET_TPRN_BASE_ADDR:
 *
 * Read TPRn Base Register address from bits 63:20.
 *
 * Input:
 * - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
 *
 * Output:
 *
 * Returns TPRn Base Register address (u64).
 */

pub unsafe fn GET_TPRN_BASE_ADDR(reg: u64) -> u64 { match "GET_TPRN_BASE_ADDR" { "GET_TPRN_BASE_RW" => (reg & ACPI_TPRN_BASE_RW_MASK) >> ACPI_TPRN_BASE_RW_SHIFT, "GET_TPRN_BASE_ENABLE" => (reg & ACPI_TPRN_BASE_ENABLE_MASK) >> ACPI_TPRN_BASE_ENABLE_SHIFT, "GET_TPRN_BASE_ADDR" => (reg & ACPI_TPRN_BASE_ADDR_MASK) >> ACPI_TPRN_BASE_ADDR_SHIFT, "GET_TPRN_LIMIT_RW" => (reg & ACPI_TPRN_LIMIT_RW_MASK) >> ACPI_TPRN_LIMIT_RW_SHIFT, _ => (reg & ACPI_TPRN_LIMIT_ADDR_MASK) >> ACPI_TPRN_LIMIT_ADDR_SHIFT } }
								   >> ACPI_TPRN_BASE_ADDR_SHIFT)

/*
 * SET_TPRN_BASE_RW:
 *
 * Set RW bit in TPRn Base register - bit 3.
 *
 * Input:
 * - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
 * - val (represents RW value to be set (u64))
 */
// #define SET_TPRN_BASE_RW(reg, val) ACPI_REGISTER_INSERT_VALUE(reg,     \
										ACPI_TPRN_BASE_RW_SHIFT,       \
										ACPI_TPRN_BASE_RW_MASK, val);

/*
 * SET_TPRN_BASE_ENABLE:
 *
 * Set Enable bit in TPRn Base register - bit 4.
 *
 * Input:
 * - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
 * - val (represents Enable value to be set (u64))
 */
// #define SET_TPRN_BASE_ENABLE(reg, val) ACPI_REGISTER_INSERT_VALUE(reg, \
										ACPI_TPRN_BASE_ENABLE_SHIFT,   \
										ACPI_TPRN_BASE_ENABLE_MASK, val);

/*
 * SET_TPRN_BASE_ADDR:
 *
 * Set TPRn Base Register address - bits 63:20
 *
 * Input
 * - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
 * - val (represents address value to be set (u64))
 */
// #define SET_TPRN_BASE_ADDR(reg, val) ACPI_REGISTER_INSERT_VALUE(reg,   \
										ACPI_TPRN_BASE_ADDR_SHIFT,     \
										ACPI_TPRN_BASE_ADDR_MASK, val);

/*
 * TPRn_LIMIT
 *
 * This register defines an isolated region of memory that can be enabled
 * to prohibit certain system agents from accessing memory. When an agent
 * sends a request upstream, whether snooped or not, a TPR prevents that
 * transaction from changing the state of memory.
 *
 * Minimal TPRn_Limit resolution is 1MB. Width is determined by a bus width.
 */

pub type ACPI_TPRN_LIMIT_REG = u64;

/* TPRn_LIMIT Register Bit Masks */

/* Bit 3 - RW: access: 1 == RO, 0 == RW register (for TPR must be RW) */
pub const ACPI_TPRN_LIMIT_RW_SHIFT: u64 = 3;
pub const ACPI_TPRN_LIMIT_RW_MASK: u64 = ((u64) 1 << ACPI_TPRN_LIMIT_RW_SHIFT);

/* Bits 63:20 - tpr_limit_rw */
pub const ACPI_TPRN_LIMIT_ADDR_SHIFT: u64 = 20;
pub const ACPI_TPRN_LIMIT_ADDR_MASK: u64 = ((u64) 0xFFFFFFFFFFF << \;
								   ACPI_TPRN_LIMIT_ADDR_SHIFT)

/* TPRn_LIMIT Register Bit Handlers*/

/*
 * GET_TPRN_LIMIT_RW:
 *
 * Read RW bit from TPRn Limit register - bit 3.
 *
 * Input:
 * - reg (represents TPRn Limit Register (ACPI_TPRN_LIMIT_REG))
 *
 * Output:
 *
 * Returns RW bit value (u64).
 */

pub unsafe fn GET_TPRN_LIMIT_RW(reg: u64) -> u64 { match "GET_TPRN_LIMIT_RW" { "GET_TPRN_BASE_RW" => (reg & ACPI_TPRN_BASE_RW_MASK) >> ACPI_TPRN_BASE_RW_SHIFT, "GET_TPRN_BASE_ENABLE" => (reg & ACPI_TPRN_BASE_ENABLE_MASK) >> ACPI_TPRN_BASE_ENABLE_SHIFT, "GET_TPRN_BASE_ADDR" => (reg & ACPI_TPRN_BASE_ADDR_MASK) >> ACPI_TPRN_BASE_ADDR_SHIFT, "GET_TPRN_LIMIT_RW" => (reg & ACPI_TPRN_LIMIT_RW_MASK) >> ACPI_TPRN_LIMIT_RW_SHIFT, _ => (reg & ACPI_TPRN_LIMIT_ADDR_MASK) >> ACPI_TPRN_LIMIT_ADDR_SHIFT } }
								   >> ACPI_TPRN_LIMIT_RW_SHIFT)

/*
 * GET_TPRN_LIMIT_ADDR:
 *
 * Read TPRn Limit Register address from bits 63:20.
 *
 * Input:
 * - reg (represents TPRn Limit Register (ACPI_TPRN_LIMIT_REG))
 *
 * Output:
 *
 * Returns TPRn Limit Register address (u64).
 */

pub unsafe fn GET_TPRN_LIMIT_ADDR(reg: u64) -> u64 { match "GET_TPRN_LIMIT_ADDR" { "GET_TPRN_BASE_RW" => (reg & ACPI_TPRN_BASE_RW_MASK) >> ACPI_TPRN_BASE_RW_SHIFT, "GET_TPRN_BASE_ENABLE" => (reg & ACPI_TPRN_BASE_ENABLE_MASK) >> ACPI_TPRN_BASE_ENABLE_SHIFT, "GET_TPRN_BASE_ADDR" => (reg & ACPI_TPRN_BASE_ADDR_MASK) >> ACPI_TPRN_BASE_ADDR_SHIFT, "GET_TPRN_LIMIT_RW" => (reg & ACPI_TPRN_LIMIT_RW_MASK) >> ACPI_TPRN_LIMIT_RW_SHIFT, _ => (reg & ACPI_TPRN_LIMIT_ADDR_MASK) >> ACPI_TPRN_LIMIT_ADDR_SHIFT } }
								   >> ACPI_TPRN_LIMIT_ADDR_SHIFT)

/*
 * SET_TPRN_LIMIT_RW:
 *
 * Set RW bit in TPRn Limit register - bit 3.
 *
 * Input:
 * - reg (represents TPRn Limit Register (ACPI_TPRN_LIMIT_REG))
 * - val (represents RW value to be set (u64))
 */
// #define SET_TPRN_LIMIT_RW(reg, val) ACPI_REGISTER_INSERT_VALUE(reg,            \
										ACPI_TPRN_LIMIT_RW_SHIFT,              \
										ACPI_TPRN_LIMIT_RW_MASK, val);

/*
 * SET_TPRN_LIMIT_ADDR:
 *
 * Set TPRn Limit Register address - bits 63:20.
 *
 * Input:
 * - reg (represents TPRn Limit Register (ACPI_TPRN_LIMIT_REG))
 * - val (represents address value to be set (u64))
 */
// #define SET_TPRN_LIMIT_ADDR(reg, val) ACPI_REGISTER_INSERT_VALUE(reg,          \
										ACPI_TPRN_LIMIT_ADDR_SHIFT,            \
										ACPI_TPRN_LIMIT_ADDR_MASK, val);

/*
 * SERIALIZE_REQUEST
 *
 * This register is used to request serialization of non-coherent DMA
 * transactions. OS shall  issue it before changing of TPR settings
 * (base / size).
 */


pub acpi_tpr_serialize_request {
	u64: sr_register;
	/*
	 * BIT 1 - Status of serialization request (RO)
	 *         0 == register idle, 1 == serialization in progress
	 * BIT 2 - Control field to initiate serialization (RW)
	 *         0 == normal, 1 == initialize serialization
	 * (self-clear to allow multiple serialization requests)
	 */
};

/*******************************************************************************
 *
 * ECDT - Embedded Controller Boot Resources Table
 *        Version 1
 *
 ******************************************************************************/


pub acpi_table_ecdt {
	header: acpi_table_header;	/* Common ACPI table header */
	control: acpi_generic_address;	/* Address of EC command/status register */
	data: acpi_generic_address;	/* Address of EC data register */
	u32: uid;		/* Unique ID - must be same as the EC _UID method */
	u8: gpe;			/* The GPE for the EC */
	u8 id: [u8; 0];		/* Full namepath of the EC in the ACPI namespace */
};

/*******************************************************************************
 *
 * EINJ - Error Injection Table (ACPI 4.0)
 *        Version 1
 *
 ******************************************************************************/


pub acpi_table_einj {
	header: acpi_table_header;	/* Common ACPI table header */
	u32: header_length;
	u8: flags;
	u8: [reserved; 3];
	u32: entries
};

/* EINJ Injection Instruction Entries (actions) */


pub acpi_einj_entry {
	whea_header: acpi_whea_header;	/* Common header for WHEA tables */
};

/* Masks for Flags field above */
pub const ACPI_EINJ_PRESERVE: u64 = 1;


pub acpi_einj_actions {
    ACPI_EINJ_BEGIN_OPERATION = 0x0,
    ACPI_EINJ_GET_TRIGGER_TABLE = 0x1,
    ACPI_EINJ_SET_ERROR_TYPE = 0x2,
    ACPI_EINJ_GET_ERROR_TYPE = 0x3,
    ACPI_EINJ_END_OPERATION = 0x4,
    ACPI_EINJ_EXECUTE_OPERATION = 0x5,
    ACPI_EINJ_CHECK_BUSY_STATUS = 0x6,
    ACPI_EINJ_GET_COMMAND_STATUS = 0x7,
    ACPI_EINJ_SET_ERROR_TYPE_WITH_ADDRESS = 0x8,
    ACPI_EINJ_GET_EXECUTE_TIMINGS = 0x9,
    ACPI_EINJV2_GET_ERROR_TYPE = 0x11,
	ACPI_EINJ_ACTION_RESERVED = 0x12,	/* 0x12 and greater are reserved */
    ACPI_EINJ_TRIGGER_ERROR = 0xFF	/* Except for this value */
};

/* Values for Instruction field above */


pub acpi_einj_instructions {
	ACPI_EINJ_READ_REGISTER = 0,
    ACPI_EINJ_READ_REGISTER_VALUE = 1,
    ACPI_EINJ_WRITE_REGISTER = 2,
    ACPI_EINJ_WRITE_REGISTER_VALUE = 3,
    ACPI_EINJ_NOOP = 4,
    ACPI_EINJ_FLUSH_CACHELINE = 5,
    ACPI_EINJ_INSTRUCTION_RESERVED = 6	/* 6 and greater are reserved */
};
,

pub acpi_einj_error_type_with_addr {
	u32: error_type;
	u32: vendor_struct_offset;
	u32: flags;
	u32: apic_id;
	u64: address;
	u64: range;
	u32: pcie_id
};


pub acpi_einj_vendor {
	u32: length;
	u32: pcie_id;
	u16: vendor_id;
	u16: device_id;
	u8: revision_id;
	u8: [reserved; 3]
};

/* EINJ Trigger Error Action Table */


pub acpi_einj_trigger {
	u32: header_size;
	u32: revision;
	u32: table_size;
	u32: entry_count
};

/* Command status return values */


pub acpi_einj_command_status {
    ACPI_EINJ_SUCCESS = 0,
    ACPI_EINJ_FAILURE = 1,
    ACPI_EINJ_INVALID_ACCESS = 2,
    ACPI_EINJ_STATUS_RESERVED = 3	/* 3 and greater are reserved */
};

/* Error types returned from ACPI_EINJ_GET_ERROR_TYPE (bitfield) */
pub const ACPI_EINJ_PROCESSOR_CORRECTABLE: u64 = 1;
pub const ACPI_EINJ_PROCESSOR_UNCORRECTABLE: u64 = 1<<1;
pub const ACPI_EINJ_PROCESSOR_FATAL: u64 = 1<<2;
pub const ACPI_EINJ_MEMORY_CORRECTABLE: u64 = 1<<3;
pub const ACPI_EINJ_MEMORY_UNCORRECTABLE: u64 = 1<<4;
pub const ACPI_EINJ_MEMORY_FATAL: u64 = 1<<5;
pub const ACPI_EINJ_PCIX_CORRECTABLE: u64 = 1<<6;
pub const ACPI_EINJ_PCIX_UNCORRECTABLE: u64 = 1<<7;
pub const ACPI_EINJ_PCIX_FATAL: u64 = 1<<8;
pub const ACPI_EINJ_PLATFORM_CORRECTABLE: u64 = 1<<9;
pub const ACPI_EINJ_PLATFORM_UNCORRECTABLE: u64 = 1<<10;
pub const ACPI_EINJ_PLATFORM_FATAL: u64 = 1<<11;
pub const ACPI_EINJ_CXL_CACHE_CORRECTABLE: u64 = 1<<12;
pub const ACPI_EINJ_CXL_CACHE_UNCORRECTABLE: u64 = 1<<13;
pub const ACPI_EINJ_CXL_CACHE_FATAL: u64 = 1<<14;
pub const ACPI_EINJ_CXL_MEM_CORRECTABLE: u64 = 1<<15;
pub const ACPI_EINJ_CXL_MEM_UNCORRECTABLE: u64 = 1<<16;
pub const ACPI_EINJ_CXL_MEM_FATAL: u64 = 1<<17;
pub const ACPI_EINJ_VENDOR_DEFINED: u64 = 1<<31;
pub const ACPI_EINJV2_PROCESSOR: u64 = 1;
pub const ACPI_EINJV2_MEMORY: u64 = 1<<1;
pub const ACPI_EINJV2_PCIE: u64 = 1<<2;
 *
 * ERST - Error Record Serialization Table (ACPI 4.0)
 *        Version 1
 *
 ******************************************************************************/
,

pub acpi_table_erst {
	header: acpi_table_header;	/* Common ACPI table header */
	u32: header_length;
	u32: reserved;
	u32: entries
};

/* ERST Serialization Entries (actions) */


pub acpi_erst_entry {
	whea_header: acpi_whea_header;	/* Common header for WHEA tables */
};

/* Masks for Flags field above */
pub const ACPI_ERST_PRESERVE: u64 = 1;


pub acpi_erst_actions {
    ACPI_ERST_BEGIN_WRITE = 0,
    ACPI_ERST_BEGIN_READ = 1,
    ACPI_ERST_BEGIN_CLEAR = 2,
    ACPI_ERST_END = 3,
    ACPI_ERST_SET_RECORD_OFFSET = 4,
    ACPI_ERST_EXECUTE_OPERATION = 5,
    ACPI_ERST_CHECK_BUSY_STATUS = 6,
    ACPI_ERST_GET_COMMAND_STATUS = 7,
    ACPI_ERST_GET_RECORD_ID = 8,
    ACPI_ERST_SET_RECORD_ID = 9,
    ACPI_ERST_GET_RECORD_COUNT = 10,
    ACPI_ERST_BEGIN_DUMMY_WRIITE = 11,
    ACPI_ERST_NOT_USED = 12,
    ACPI_ERST_GET_ERROR_RANGE = 13,
    ACPI_ERST_GET_ERROR_LENGTH = 14,
    ACPI_ERST_GET_ERROR_ATTRIBUTES = 15,
    ACPI_ERST_EXECUTE_TIMINGS = 16,
    ACPI_ERST_ACTION_RESERVED = 17	/* 17 and greater are reserved */
};

/* Values for Instruction field above */


pub acpi_erst_instructions {
	ACPI_ERST_READ_REGISTER = 0,
    ACPI_ERST_READ_REGISTER_VALUE = 1,
    ACPI_ERST_WRITE_REGISTER = 2,
    ACPI_ERST_WRITE_REGISTER_VALUE = 3,
    ACPI_ERST_NOOP = 4,
    ACPI_ERST_LOAD_VAR1 = 5,
    ACPI_ERST_LOAD_VAR2 = 6,
    ACPI_ERST_STORE_VAR1 = 7,
    ACPI_ERST_ADD = 8,
    ACPI_ERST_SUBTRACT = 9,
    ACPI_ERST_ADD_VALUE = 10,
    ACPI_ERST_SUBTRACT_VALUE = 11,
    ACPI_ERST_STALL = 12,
    ACPI_ERST_STALL_WHILE_TRUE = 13,
    ACPI_ERST_SKIP_NEXT_IF_TRUE = 14,
    ACPI_ERST_GOTO = 15,
    ACPI_ERST_SET_SRC_ADDRESS_BASE = 16,
    ACPI_ERST_SET_DST_ADDRESS_BASE = 17,
    ACPI_ERST_MOVE_DATA = 18,
    ACPI_ERST_INSTRUCTION_RESERVED = 19	/* 19 and greater are reserved */
};

/* Command status return values */


pub acpi_erst_command_status {
	ACPI_ERST_SUCCESS = 0,
    ACPI_ERST_NO_SPACE = 1,
    ACPI_ERST_NOT_AVAILABLE = 2,
    ACPI_ERST_FAILURE = 3,
    ACPI_ERST_RECORD_EMPTY = 4,
    ACPI_ERST_NOT_FOUND = 5,
    ACPI_ERST_STATUS_RESERVED = 6	/* 6 and greater are reserved */
};

/* Error Record Serialization Information */
,

pub acpi_erst_info {
	u16: signature;		/* Should be "ER" */
	u8: [data; 48]
};

/*******************************************************************************
 *
 * FPDT - Firmware Performance Data Table (ACPI 5.0)
 *        Version 1
 *
 ******************************************************************************/


pub acpi_table_fpdt {
	header: acpi_table_header;	/* Common ACPI table header */
};

/* FPDT subtable header (Performance Record Structure) */


pub acpi_fpdt_header {
	u16: type;
	u8: length;
	u8: revision
};

/* Values for Type field above */


pub acpi_fpdt_type {
    ACPI_FPDT_TYPE_BOOT = 0,
    ACPI_FPDT_TYPE_S3PERF = 1
};

/*
 * FPDT subtables
 */

/* 0: Firmware Basic Boot Performance Record */
,

pub acpi_fpdt_boot_pointer {
	header: acpi_fpdt_header;
	u8: [reserved; 4];
	u64: address
};

/* 1: S3 Performance Table Pointer Record */


pub acpi_fpdt_s3pt_pointer {
	header: acpi_fpdt_header;
	u8: [reserved; 4];
	u64: address
};

/*
 * S3PT - S3 Performance Table. This table is pointed to by the
 * S3 Pointer Record above.
 */

pub acpi_table_s3pt {
	u8: [signature; 4];	/* "S3PT" */
	u32: length
};

/*
 * S3PT Subtables (Not part of the actual FPDT)
 */

/* Values for Type field in S3PT header */


pub acpi_s3pt_type {
    ACPI_S3PT_TYPE_RESUME = 0,
    ACPI_S3PT_TYPE_SUSPEND = 1,
    ACPI_FPDT_BOOT_PERFORMANCE = 2
};
,

pub acpi_s3pt_resume {
	header: acpi_fpdt_header;
	u32: resume_count;
	u64: full_resume;
	u64: average_resume
};


pub acpi_s3pt_suspend {
	header: acpi_fpdt_header;
	u64: suspend_start;
	u64: suspend_end
};

/*
 * FPDT Boot Performance Record (Not part of the actual FPDT)
 */

pub acpi_fpdt_boot {
	header: acpi_fpdt_header;
	u8: [reserved; 4];
	u64: reset_end;
	u64: load_start;
	u64: startup_start;
	u64: exit_services_entry;
	u64: exit_services_exit
};

/*******************************************************************************
 *
 * GTDT - Generic Timer Description Table (ACPI 5.1)
 *        Version 2
 *
 ******************************************************************************/


pub acpi_table_gtdt {
	header: acpi_table_header;	/* Common ACPI table header */
	u64: counter_block_addresss;
	u32: reserved;
	u32: secure_el1_interrupt;
	u32: secure_el1_flags;
	u32: non_secure_el1_interrupt;
	u32: non_secure_el1_flags;
	u32: virtual_timer_interrupt;
	u32: virtual_timer_flags;
	u32: non_secure_el2_interrupt;
	u32: non_secure_el2_flags;
	u64: counter_read_block_address;
	u32: platform_timer_count;
	u32: platform_timer_offset
};

/* Flag Definitions: Timer Block Physical Timers and Virtual timers */
pub const ACPI_GTDT_INTERRUPT_MODE: u64 = 1;
pub const ACPI_GTDT_INTERRUPT_POLARITY: u64 = 1<<1;
pub const ACPI_GTDT_ALWAYS_ON: u64 = 1<<2;

pub acpi_gtdt_el2 {
	u32: virtual_el2_timer_gsiv;
	u32: virtual_el2_timer_flags
};

/* Common GTDT subtable header */


pub acpi_gtdt_header {
	u8: type;
	u16: length
};

/* Values for GTDT subtable type above */


pub acpi_gtdt_type {
    ACPI_GTDT_TYPE_TIMER_BLOCK = 0,
    ACPI_GTDT_TYPE_WATCHDOG = 1,
    ACPI_GTDT_TYPE_RESERVED = 2	/* 2 and greater are reserved */
};
,
/* GTDT Subtables, correspond to Type in acpi_gtdt_header */

/* 0: Generic Timer Block */


pub acpi_gtdt_timer_block {
	header: acpi_gtdt_header;
	u8: reserved;
	u64: block_address;
	u32: timer_count;
	u32: timer_offset
};

/* Timer Sub-Structure, one per timer */


pub acpi_gtdt_timer_entry {
	u8: frame_number;
	u8: [reserved; 3];
	u64: base_address;
	u64: el0_base_address;
	u32: timer_interrupt;
	u32: timer_flags;
	u32: virtual_timer_interrupt;
	u32: virtual_timer_flags;
	u32: common_flags
};

/* Flag Definitions: timer_flags and virtual_timer_flags above */
pub const ACPI_GTDT_GT_IRQ_MODE: u64 = 1;
pub const ACPI_GTDT_GT_IRQ_POLARITY: u64 = 1<<1;
pub const ACPI_GTDT_GT_IS_SECURE_TIMER: u64 = 1;
pub const ACPI_GTDT_GT_ALWAYS_ON: u64 = 1<<1;


pub acpi_gtdt_watchdog {
	header: acpi_gtdt_header;
	u8: reserved;
	u64: refresh_frame_address;
	u64: control_frame_address;
	u32: timer_interrupt;
	u32: timer_flags
};

/* Flag Definitions: timer_flags above */
pub const ACPI_GTDT_WATCHDOG_IRQ_MODE: u64 = 1;
pub const ACPI_GTDT_WATCHDOG_IRQ_POLARITY: u64 = 1<<1;
pub const ACPI_GTDT_WATCHDOG_SECURE: u64 = 1<<2;
 *
 * HEST - Hardware Error Source Table (ACPI 4.0)
 *        Version 1
 *
 ******************************************************************************/


pub acpi_table_hest {
	header: acpi_table_header;	/* Common ACPI table header */
	u32: error_source_count
};

/* HEST subtable header */


pub acpi_hest_header {
	u16: type;
	u16: source_id
};

/* Values for Type field above for subtables */


pub acpi_hest_types {
    ACPI_HEST_TYPE_IA32_CHECK = 0,
    ACPI_HEST_TYPE_IA32_CORRECTED_CHECK = 1,
    ACPI_HEST_TYPE_IA32_NMI = 2,
    ACPI_HEST_TYPE_NOT_USED3 = 3,
    ACPI_HEST_TYPE_NOT_USED4 = 4,
    ACPI_HEST_TYPE_NOT_USED5 = 5,
    ACPI_HEST_TYPE_AER_ROOT_PORT = 6,
    ACPI_HEST_TYPE_AER_ENDPOINT = 7,
    ACPI_HEST_TYPE_AER_BRIDGE = 8,
    ACPI_HEST_TYPE_GENERIC_ERROR = 9,
    ACPI_HEST_TYPE_GENERIC_ERROR_V2 = 10,
    ACPI_HEST_TYPE_IA32_DEFERRED_CHECK = 11,
    ACPI_HEST_TYPE_RESERVED = 12	/* 12 and greater are reserved */
};

/*
 * HEST substructures contained in subtables
 */

/*
 * IA32 Error Bank(s) - Follows the acpi_hest_ia_machine_check and
 * acpi_hest_ia_corrected structures.
 */,

pub acpi_hest_ia_error_bank {
	u8: bank_number;
	u8: clear_status_on_init;
	u8: status_format;
	u8: reserved;
	u32: control_register;
	u64: control_data;
	u32: status_register;
	u32: address_register;
	u32: misc_register
};

/* Common HEST sub-structure for PCI/AER structures below (6,7,8) */


pub acpi_hest_aer_common {
	u16: reserved1;
	u8: flags;
	u8: enabled;
	u32: records_to_preallocate;
	u32: max_sections_per_record;
	u32: bus;		/* Bus and Segment numbers */
	u16: device;
	u16: function;
	u16: device_control;
	u16: reserved2;
	u32: uncorrectable_mask;
	u32: uncorrectable_severity;
	u32: correctable_mask;
	u32: advanced_capabilities
};

/* Masks for HEST Flags fields */
pub const ACPI_HEST_FIRMWARE_FIRST: u64 = 1;
pub const ACPI_HEST_GLOBAL: u64 = 1<<1;
pub const ACPI_HEST_GHES_ASSIST: u64 = 1<<2;
 * Macros to access the bus/segment numbers in Bus field above:
 *  Bus number is encoded in bits 7:0
 *  Segment number is encoded in bits 23:8
 */
// #define ACPI_HEST_BUS(bus)              ((bus) & 0xFF)
// #define ACPI_HEST_SEGMENT(bus)          (((bus) >> 8) & 0xFFFF)

/* Hardware Error Notification */


pub acpi_hest_notify {
	u8: type;
	u8: length;
	u16: config_write_enable;
	u32: poll_interval;
	u32: vector;
	u32: polling_threshold_value;
	u32: polling_threshold_window;
	u32: error_threshold_value;
	u32: error_threshold_window
};

/* Values for Notify Type field above */


pub acpi_hest_notify_types {
    ACPI_HEST_NOTIFY_POLLED = 0,
    ACPI_HEST_NOTIFY_EXTERNAL = 1,
    ACPI_HEST_NOTIFY_LOCAL = 2,
    ACPI_HEST_NOTIFY_SCI = 3,
    ACPI_HEST_NOTIFY_NMI = 4,
	ACPI_HEST_NOTIFY_CMCI = 5,	/* ACPI 5.0 */
	ACPI_HEST_NOTIFY_MCE = 6,	/* ACPI 5.0 */
	ACPI_HEST_NOTIFY_GPIO = 7,	/* ACPI 6.0 */
	ACPI_HEST_NOTIFY_SEA = 8,	/* ACPI 6.1 */
	ACPI_HEST_NOTIFY_SEI = 9,	/* ACPI 6.1 */
	ACPI_HEST_NOTIFY_GSIV = 10,	/* ACPI 6.1 */
	ACPI_HEST_NOTIFY_SOFTWARE_DELEGATED = 11,	/* ACPI 6.2 */
    ACPI_HEST_NOTIFY_RESERVED = 12	/* 12 and greater are reserved */
};

/* Values for config_write_enable bitfield above */
pub const ACPI_HEST_TYPE: u64 = 1;
pub const ACPI_HEST_POLL_INTERVAL: u64 = 1<<1;
pub const ACPI_HEST_POLL_THRESHOLD_VALUE: u64 = 1<<2;
pub const ACPI_HEST_POLL_THRESHOLD_WINDOW: u64 = 1<<3;
pub const ACPI_HEST_ERR_THRESHOLD_VALUE: u64 = 1<<4;
pub const ACPI_HEST_ERR_THRESHOLD_WINDOW: u64 = 1<<5;
 * HEST subtables
 */

/* 0: IA32 Machine Check Exception */
,

pub acpi_hest_ia_machine_check {
	header: acpi_hest_header;
	u16: reserved1;
	u8: flags;		/* See flags ACPI_HEST_GLOBAL, etc. above */
	u8: enabled;
	u32: records_to_preallocate;
	u32: max_sections_per_record;
	u64: global_capability_data;
	u64: global_control_data;
	u8: num_hardware_banks;
	u8: [reserved3; 7]
};

/* 1: IA32 Corrected Machine Check */


pub acpi_hest_ia_corrected {
	header: acpi_hest_header;
	u16: reserved1;
	u8: flags;		/* See flags ACPI_HEST_GLOBAL, etc. above */
	u8: enabled;
	u32: records_to_preallocate;
	u32: max_sections_per_record;
	notify: acpi_hest_notify;
	u8: num_hardware_banks;
	u8: [reserved2; 3]
};

/* 2: IA32 Non-Maskable Interrupt */


pub acpi_hest_ia_nmi {
	header: acpi_hest_header;
	u32: reserved;
	u32: records_to_preallocate;
	u32: max_sections_per_record;
	u32: max_raw_data_length
};

/* 3,4,5: Not used */

/* 6: PCI Express Root Port AER */


pub acpi_hest_aer_root {
	header: acpi_hest_header;
	aer: acpi_hest_aer_common;
	u32: root_error_command
};

/* 7: PCI Express AER (AER Endpoint) */


pub acpi_hest_aer {
	header: acpi_hest_header;
	aer: acpi_hest_aer_common
};

/* 8: PCI Express/PCI-X Bridge AER */


pub acpi_hest_aer_bridge {
	header: acpi_hest_header;
	aer: acpi_hest_aer_common;
	u32: uncorrectable_mask2;
	u32: uncorrectable_severity2;
	u32: advanced_capabilities2
};

/* 9: Generic Hardware Error Source */


pub acpi_hest_generic {
	header: acpi_hest_header;
	u16: related_source_id;
	u8: reserved;
	u8: enabled;
	u32: records_to_preallocate;
	u32: max_sections_per_record;
	u32: max_raw_data_length;
	error_status_address: acpi_generic_address;
	notify: acpi_hest_notify;
	u32: error_block_length
};

/* 10: Generic Hardware Error Source, version 2 */


pub acpi_hest_generic_v2 {
	header: acpi_hest_header;
	u16: related_source_id;
	u8: reserved;
	u8: enabled;
	u32: records_to_preallocate;
	u32: max_sections_per_record;
	u32: max_raw_data_length;
	error_status_address: acpi_generic_address;
	notify: acpi_hest_notify;
	u32: error_block_length;
	read_ack_register: acpi_generic_address;
	u64: read_ack_preserve;
	u64: read_ack_write
};

/* Generic Error Status block */


pub acpi_hest_generic_status {
	u32: block_status;
	u32: raw_data_offset;
	u32: raw_data_length;
	u32: data_length;
	u32: error_severity
};

/* Values for block_status flags above */
pub const ACPI_HEST_UNCORRECTABLE: u64 = 1;
pub const ACPI_HEST_CORRECTABLE: u64 = 1<<1;
pub const ACPI_HEST_MULTIPLE_UNCORRECTABLE: u64 = 1<<2;
pub const ACPI_HEST_MULTIPLE_CORRECTABLE: u64 = 1<<3;
pub const ACPI_HEST_ERROR_ENTRY_COUNT: u64 = 0xFF<<4;

/* Generic Error Data entry */


pub acpi_hest_generic_data {
	u8: [section_type; 16];
	u32: error_severity;
	u16: revision;
	u8: validation_bits;
	u8: flags;
	u32: error_data_length;
	u8: [fru_id; 16];
	u8: [fru_text; 20]
};

/* Extension for revision 0x0300 */


pub acpi_hest_generic_data_v300 {
	u8: [section_type; 16];
	u32: error_severity;
	u16: revision;
	u8: validation_bits;
	u8: flags;
	u32: error_data_length;
	u8: [fru_id; 16];
	u8: [fru_text; 20];
	u64: time_stamp
};

/* Values for error_severity above */
pub const ACPI_HEST_GEN_ERROR_RECOVERABLE: u64 = 0;
pub const ACPI_HEST_GEN_ERROR_FATAL: u64 = 1;
pub const ACPI_HEST_GEN_ERROR_CORRECTED: u64 = 2;
pub const ACPI_HEST_GEN_ERROR_NONE: u64 = 3;
pub const ACPI_HEST_GEN_VALID_FRU_ID: u64 = 1;
pub const ACPI_HEST_GEN_VALID_FRU_STRING: u64 = 1<<1;
pub const ACPI_HEST_GEN_VALID_TIMESTAMP: u64 = 1<<2;


pub acpi_hest_ia_deferred_check {
	header: acpi_hest_header;
	u16: reserved1;
	u8: flags;		/* See flags ACPI_HEST_GLOBAL, etc. above */
	u8: enabled;
	u32: records_to_preallocate;
	u32: max_sections_per_record;
	notify: acpi_hest_notify;
	u8: num_hardware_banks;
	u8: [reserved2; 3]
};

/*******************************************************************************
 *
 * HMAT - Heterogeneous Memory Attributes Table (ACPI 6.2)
 *        Version 1
 *
 ******************************************************************************/


pub acpi_table_hmat {
	header: acpi_table_header;	/* Common ACPI table header */
	u32: reserved
};

/* Values for HMAT structure types */


pub acpi_hmat_type {
	ACPI_HMAT_TYPE_PROXIMITY = 0,	/* Memory proximity domain attributes */
	ACPI_HMAT_TYPE_LOCALITY = 1,	/* System locality latency and bandwidth information */
	ACPI_HMAT_TYPE_CACHE = 2,	/* Memory side cache information */
    ACPI_HMAT_TYPE_RESERVED = 3	/* 3 and greater are reserved */
};
,

pub acpi_hmat_structure {
	u16: type;
	u16: reserved;
	u32: length
};

/*
 * HMAT Structures, correspond to Type in acpi_hmat_structure
 */

/* 0: Memory proximity domain attributes */


pub acpi_hmat_proximity_domain {
	header: acpi_hmat_structure;
	u16: flags;
	u16: reserved1;
	u32: processor_PD;	/* Processor proximity domain */
	u32: memory_PD;		/* Memory proximity domain */
	u32: reserved2;
	u64: reserved3;
	u64: reserved4
};

/* Masks for Flags field above */
pub const ACPI_HMAT_PROCESSOR_PD_VALID: u64 = 1;
pub const ACPI_HMAT_MEMORY_PD_VALID: u64 = 1<<1;
pub const ACPI_HMAT_RESERVATION_HINT: u64 = 1<<2;

/* 1: System locality latency and bandwidth information */


pub acpi_hmat_locality {
	header: acpi_hmat_structure;
	u8: flags;
	u8: data_type;
	u8: min_transfer_size;
	u8: reserved1;
	u32: number_of_initiator_Pds;
	u32: number_of_target_Pds;
	u32: reserved2;
	u64: entry_base_unit
};

/* Masks for Flags field above */
pub const ACPI_HMAT_MEMORY_HIERARCHY: u64 = 0x0F;

/* Values for Memory Hierarchy flags */
pub const ACPI_HMAT_MEMORY: u64 = 0;
pub const ACPI_HMAT_LAST_LEVEL_CACHE: u64 = 1;
pub const ACPI_HMAT_1ST_LEVEL_CACHE: u64 = 2;
pub const ACPI_HMAT_2ND_LEVEL_CACHE: u64 = 3;
pub const ACPI_HMAT_3RD_LEVEL_CACHE: u64 = 4;
pub const ACPI_HMAT_MINIMUM_XFER_SIZE: u64 = 0x10;
pub const ACPI_HMAT_NON_SEQUENTIAL_XFERS: u64 = 0x20;


/* Values for data_type field above */
pub const ACPI_HMAT_ACCESS_LATENCY: u64 = 0;
pub const ACPI_HMAT_READ_LATENCY: u64 = 1;
pub const ACPI_HMAT_WRITE_LATENCY: u64 = 2;
pub const ACPI_HMAT_ACCESS_BANDWIDTH: u64 = 3;
pub const ACPI_HMAT_READ_BANDWIDTH: u64 = 4;
pub const ACPI_HMAT_WRITE_BANDWIDTH: u64 = 5;


pub acpi_hmat_cache {
	header: acpi_hmat_structure;
	u32: memory_PD;
	u32: reserved1;
	u64: cache_size;
	u32: cache_attributes;
	u16: address_mode;
	u16: number_of_SMBIOShandles
};

/* Masks for cache_attributes field above */
pub const ACPI_HMAT_TOTAL_CACHE_LEVEL: u64 = 0x0000000F;
pub const ACPI_HMAT_CACHE_LEVEL: u64 = 0x000000F0;
pub const ACPI_HMAT_CACHE_ASSOCIATIVITY: u64 = 0x00000F00;
pub const ACPI_HMAT_WRITE_POLICY: u64 = 0x0000F000;
pub const ACPI_HMAT_CACHE_LINE_SIZE: u64 = 0xFFFF0000;pub const ACPI_HMAT_CACHE_MODE_UNKNOWN: u64 = 0;
pub const ACPI_HMAT_CACHE_MODE_EXTENDED_LINEAR: u64 = 1;
pub const ACPI_HMAT_CA_NONE: u64 = 0;
pub const ACPI_HMAT_CA_DIRECT_MAPPED: u64 = 1;
pub const ACPI_HMAT_CA_COMPLEX_CACHE_INDEXING: u64 = 2;
pub const ACPI_HMAT_CP_NONE: u64 = 0;
pub const ACPI_HMAT_CP_WB: u64 = 1;
pub const ACPI_HMAT_CP_WT: u64 = 2;
 *
 * HPET - High Precision Event Timer table
 *        Version 1
 *
 * Conforms to "IA-PC HPET (High Precision Event Timers) Specification",
 * Version 1.0a, October 2004
 *
 ******************************************************************************/


pub acpi_table_hpet {
	header: acpi_table_header;	/* Common ACPI table header */
	u32: id;			/* Hardware ID of event timer block */
	address: acpi_generic_address;	/* Address of event timer block */
	u8: sequence;		/* HPET sequence number */
	u16: minimum_tick;	/* Main counter min tick, periodic mode */
	u8: flags
};

/* Masks for Flags field above */
pub const ACPI_HPET_PAGE_PROTECT_MASK: u64 = 3;


pub acpi_hpet_page_protect {
    ACPI_HPET_NO_PAGE_PROTECT = 0,
    ACPI_HPET_PAGE_PROTECT4 = 1,
    ACPI_HPET_PAGE_PROTECT64 = 2
};

/*******************************************************************************
 *
 * IBFT - Boot Firmware Table
 *        Version 1
 *
 * Conforms to "iSCSI Boot Firmware Table (iBFT) as Defined in ACPI 3.0b,
 * Specification", Version 1.01, March 1, 2007
 *
 * Note: It appears that this table is not intended to appear in the RSDT/XSDT.
 * Therefore, it is not currently supported by the disassembler.
 *
 ******************************************************************************/


pub acpi_table_ibft {
	header: acpi_table_header;	/* Common ACPI table header */
	u8: [reserved; 12]
};

/* IBFT common subtable header */


pub acpi_ibft_header {
	u8: type;
	u8: version;
	u16: length;
	u8: index;
	u8: flags
};

/* Values for Type field above */


pub acpi_ibft_type {
    ACPI_IBFT_TYPE_NOT_USED = 0,
    ACPI_IBFT_TYPE_CONTROL = 1,
    ACPI_IBFT_TYPE_INITIATOR = 2,
    ACPI_IBFT_TYPE_NIC = 3,
    ACPI_IBFT_TYPE_TARGET = 4,
    ACPI_IBFT_TYPE_EXTENSIONS = 5,
    ACPI_IBFT_TYPE_RESERVED = 6	/* 6 and greater are reserved */
};

/* IBFT subtables */
,

pub acpi_ibft_control {
	header: acpi_ibft_header;
	u16: extensions;
	u16: initiator_offset;
	u16: nic0_offset;
	u16: target0_offset;
	u16: nic1_offset;
	u16: target1_offset
};


pub acpi_ibft_initiator {
	header: acpi_ibft_header;
	u8: [sns_server; 16];
	u8: [slp_server; 16];
	u8: [primary_server; 16];
	u8: [secondary_server; 16];
	u16: name_length;
	u16: name_offset
};


pub acpi_ibft_nic {
	header: acpi_ibft_header;
	u8: [ip_address; 16];
	u8: subnet_mask_prefix;
	u8: origin;
	u8: [gateway; 16];
	u8: [primary_dns; 16];
	u8: [secondary_dns; 16];
	u8: [dhcp; 16];
	u16: vlan;
	u8: [mac_address; 6];
	u16: pci_address;
	u16: name_length;
	u16: name_offset
};


pub acpi_ibft_target {
	header: acpi_ibft_header;
	u8: [target_ip_address; 16];
	u16: target_ip_socket;
	u8: [target_boot_lun; 8];
	u8: chap_type;
	u8: nic_association;
	u16: target_name_length;
	u16: target_name_offset;
	u16: chap_name_length;
	u16: chap_name_offset;
	u16: chap_secret_length;
	u16: chap_secret_offset;
	u16: reverse_chap_name_length;
	u16: reverse_chap_name_offset;
	u16: reverse_chap_secret_length;
	u16: reverse_chap_secret_offset
};

/* Reset to default packing */





// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
