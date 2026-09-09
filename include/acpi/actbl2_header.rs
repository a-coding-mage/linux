/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: actbl2.h - ACPI Table Definitions
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/


/*******************************************************************************
 *
 * Additional ACPI Tables (2)
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
pub const ACPI_SIG_AGDI: &str = "AGDI";	/* Arm Generic Diagnostic Dump and Reset Device Interface */
pub const ACPI_SIG_APMT: &str = "APMT";	/* Arm Performance Monitoring Unit table */
pub const ACPI_SIG_BDAT: &str = "BDAT";	/* BIOS Data ACPI Table */
pub const ACPI_SIG_CCEL: &str = "CCEL";	/* CC Event Log Table */
pub const ACPI_SIG_CDAT: &str = "CDAT";	/* Coherent Device Attribute Table */
pub const ACPI_SIG_ERDT: &str = "ERDT";	/* Enhanced Resource Director Technology */
pub const ACPI_SIG_IORT: &str = "IORT";	/* IO Remapping Table */
pub const ACPI_SIG_IOVT: &str = "IOVT";	/* I/O Virtualization Table */
pub const ACPI_SIG_IVRS: &str = "IVRS";	/* I/O Virtualization Reporting Structure */
pub const ACPI_SIG_KEYP: &str = "KEYP";	/* Key Programming Interface for IDE */
pub const ACPI_SIG_LPIT: &str = "LPIT";	/* Low Power Idle Table */
pub const ACPI_SIG_MADT: &str = "APIC";	/* Multiple APIC Description Table */
pub const ACPI_SIG_MCFG: &str = "MCFG";	/* PCI Memory Mapped Configuration table */
pub const ACPI_SIG_MCHI: &str = "MCHI";	/* Management Controller Host Interface table */
pub const ACPI_SIG_MPAM: &str = "MPAM";	/* Memory System Resource Partitioning and Monitoring Table */
pub const ACPI_SIG_MPST: &str = "MPST";	/* Memory Power State Table */
pub const ACPI_SIG_MRRM: &str = "MRRM";	/* Memory Range and Region Mapping table */
pub const ACPI_SIG_MSDM: &str = "MSDM";	/* Microsoft Data Management Table */
pub const ACPI_SIG_NFIT: &str = "NFIT";	/* NVDIMM Firmware Interface Table */
pub const ACPI_SIG_NHLT: &str = "NHLT";	/* Non HD Audio Link Table */
pub const ACPI_SIG_PCCT: &str = "PCCT";	/* Platform Communications Channel Table */
pub const ACPI_SIG_PDTT: &str = "PDTT";	/* Platform Debug Trigger Table */
pub const ACPI_SIG_PHAT: &str = "PHAT";	/* Platform Health Assessment Table */
pub const ACPI_SIG_PMTT: &str = "PMTT";	/* Platform Memory Topology Table */
pub const ACPI_SIG_PPTT: &str = "PPTT";	/* Processor Properties Topology Table */
pub const ACPI_SIG_PRMT: &str = "PRMT";	/* Platform Runtime Mechanism Table */
pub const ACPI_SIG_RASF: &str = "RASF";	/* RAS Feature table */
pub const ACPI_SIG_RAS2: &str = "RAS2";	/* RAS2 Feature table */
pub const ACPI_SIG_RGRT: &str = "RGRT";	/* Regulatory Graphics Resource Table */
pub const ACPI_SIG_RHCT: &str = "RHCT";	/* RISC-V Hart Capabilities Table */
pub const ACPI_SIG_RIMT: &str = "RIMT";	/* RISC-V IO Mapping Table */
pub const ACPI_SIG_SBST: &str = "SBST";	/* Smart Battery Specification Table */
pub const ACPI_SIG_SDEI: &str = "SDEI";	/* Software Delegated Exception Interface Table */
pub const ACPI_SIG_SDEV: &str = "SDEV";	/* Secure Devices table */
pub const ACPI_SIG_SVKL: &str = "SVKL";	/* Storage Volume Key Location Table */
pub const ACPI_SIG_SWFT: &str = "SWFT";	/* SoundWire File Table */
pub const ACPI_SIG_TDEL: &str = "TDEL";	/* TD Event Log Table */

/*
 * All tables must be byte-packed to match the ACPI specification, since
 * the tables are provided by the system BIOS.
 */
// C: #pragma pack(1) — all declarations below use packed C layout.

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
 * AEST - Arm Error Source Table
 *
 * Conforms to: ACPI for the Armv8 RAS Extensions 1.1(Sep 2020) and
 * 2.0(May 2023) Platform Design Document.
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_aest {
    pub header: acpi_table_header;
};

/* Common Subtable header - one per Node Structure (Subtable) */

#[repr(C, packed)]
pub struct acpi_aest_hdr {
    pub type: u8;
    pub length: u16;
    pub reserved: u8;
    pub node_specific_offset: u32;
    pub node_interface_offset: u32;
    pub node_interrupt_offset: u32;
    pub node_interrupt_count: u32;
    pub timestamp_rate: u64;
    pub reserved1: u64;
    pub error_injection_rate: u64;
};

/* Values for Type above */

pub const ACPI_AEST_PROCESSOR_ERROR_NODE: u64 = 0;
pub const ACPI_AEST_MEMORY_ERROR_NODE: u64 = 1;
pub const ACPI_AEST_SMMU_ERROR_NODE: u64 = 2;
pub const ACPI_AEST_VENDOR_ERROR_NODE: u64 = 3;
pub const ACPI_AEST_GIC_ERROR_NODE: u64 = 4;
pub const ACPI_AEST_PCIE_ERROR_NODE: u64 = 5;
pub const ACPI_AEST_PROXY_ERROR_NODE: u64 = 6;
pub const ACPI_AEST_NODE_TYPE_RESERVED: u64 = 7 /* 7 and above are reserved */;

/*
 * AEST subtables (Error nodes)
 */

/* 0: Processor Error */

#[repr(C, packed)]
pub struct acpi_aest_processor {
    pub processor_id: u32;
    pub resource_type: u8;
    pub reserved: u8;
    pub flags: u8;
    pub revision: u8;
    pub processor_affinity: u64;
}

/* Values for resource_type above, related structs below */

pub const ACPI_AEST_CACHE_RESOURCE: u64 = 0;
pub const ACPI_AEST_TLB_RESOURCE: u64 = 1;
pub const ACPI_AEST_GENERIC_RESOURCE: u64 = 2;
pub const ACPI_AEST_RESOURCE_RESERVED: u64 = 3	/* 3 and above are reserved */;

/* 0R: Processor Cache Resource Substructure */

#[repr(C, packed)]
pub struct acpi_aest_processor_cache {
    pub cache_reference: u32;
    pub reserved: u32;
}

/* Values for cache_type above */

pub const ACPI_AEST_CACHE_DATA: u64 = 0;
pub const ACPI_AEST_CACHE_INSTRUCTION: u64 = 1;
pub const ACPI_AEST_CACHE_UNIFIED: u64 = 2;
pub const ACPI_AEST_CACHE_RESERVED: u64 = 3	/* 3 and above are reserved */;

/* 1R: Processor TLB Resource Substructure */

#[repr(C, packed)]
pub struct acpi_aest_processor_tlb {
    pub tlb_level: u32;
    pub reserved: u32;
}

/* 2R: Processor Generic Resource Substructure */

#[repr(C, packed)]
pub struct acpi_aest_processor_generic {
    pub resource: u32;
}

/* 1: Memory Error */

#[repr(C, packed)]
pub struct acpi_aest_memory {
    pub srat_proximity_domain: u32;
}

/* 2: Smmu Error */

#[repr(C, packed)]
pub struct acpi_aest_smmu {
    pub iort_node_reference: u32;
    pub subcomponent_reference: u32;
}

/* 3: Vendor Defined */

#[repr(C, packed)]
pub struct acpi_aest_vendor {
    pub acpi_hid: u32;
    pub acpi_uid: u32;
    pub vendor_specific_data: [u8; 16];
}

#[repr(C, packed)]
pub struct acpi_aest_vendor_v2 {
    pub acpi_hid: [i8; 8];
    pub acpi_uid: u32;
    pub vendor_specific_data: [u8; 16];
};

/* 4: Gic Error */

#[repr(C, packed)]
pub struct acpi_aest_gic {
    pub interface_type: u32;
    pub instance_id: u32;
}

/* Values for interface_type above */

pub const ACPI_AEST_GIC_CPU: u64 = 0;
pub const ACPI_AEST_GIC_DISTRIBUTOR: u64 = 1;
pub const ACPI_AEST_GIC_REDISTRIBUTOR: u64 = 2;
pub const ACPI_AEST_GIC_ITS: u64 = 3;
pub const ACPI_AEST_GIC_RESERVED: u64 = 4	/* 4 and above are reserved */;

/* 5: PCIe Error */

#[repr(C, packed)]
pub struct acpi_aest_pcie {
    pub iort_node_reference: u32;
};

/* 6: Proxy Error */

#[repr(C, packed)]
pub struct acpi_aest_proxy {
    pub node_address: u64;
};

/* Node Interface Structure */

#[repr(C, packed)]
pub struct acpi_aest_node_interface {
    pub type: u8;
    pub reserved: [u8; 3];
    pub flags: u32;
    pub address: u64;
    pub error_record_index: u32;
    pub error_record_count: u32;
    pub error_record_implemented: u64;
    pub error_status_reporting: u64;
    pub addressing_mode: u64;
}

/* Node Interface Structure V2 */

#[repr(C, packed)]
pub struct acpi_aest_node_interface_header {
    pub type: u8;
    pub group_format: u8;
    pub reserved: [u8; 2];
    pub flags: u32;
    pub address: u64;
    pub error_record_index: u32;
    pub error_record_count: u32;
};

pub const ACPI_AEST_NODE_GROUP_FORMAT_4K: u64 = 0;
pub const ACPI_AEST_NODE_GROUP_FORMAT_16K: u64 = 1;
pub const ACPI_AEST_NODE_GROUP_FORMAT_64K: u64 = 2;

#[repr(C, packed)]
pub struct acpi_aest_node_interface_common {
    pub error_node_device: u32;
    pub processor_affinity: u32;
    pub error_group_register_base: u64;
    pub fault_inject_register_base: u64;
    pub interrupt_config_register_base: u64;
};

#[repr(C, packed)]
pub struct acpi_aest_node_interface_4k {
    pub error_record_implemented: u64;
    pub error_status_reporting: u64;
    pub addressing_mode: u64;
    pub common: acpi_aest_node_interface_common;
};

#[repr(C, packed)]
pub struct acpi_aest_node_interface_16k {
    pub error_record_implemented: [u64; 4];
    pub error_status_reporting: [u64; 4];
    pub addressing_mode: [u64; 4];
    pub common: acpi_aest_node_interface_common;
};

#[repr(C, packed)]
pub struct acpi_aest_node_interface_64k {
    pub error_record_implemented: [u64; 14];
    pub error_status_reporting: [u64; 14];
    pub addressing_mode: [u64; 14];
    pub common: acpi_aest_node_interface_common;
};

/* Values for Type field above */

pub const ACPI_AEST_NODE_SYSTEM_REGISTER: u64 = 0;
pub const ACPI_AEST_NODE_MEMORY_MAPPED: u64 = 1;
pub const ACPI_AEST_NODE_SINGLE_RECORD_MEMORY_MAPPED: u64 = 2;
pub const ACPI_AEST_XFACE_RESERVED: u64 = 3   /* 2 and above are reserved */;

/* Node Interrupt Structure */

#[repr(C, packed)]
pub struct acpi_aest_node_interrupt {
    pub type: u8;
    pub reserved: [u8; 2];
    pub flags: u8;
    pub gsiv: u32;
    pub iort_id: u8;
    pub reserved1: [u8; 3];
}

/* Node Interrupt Structure V2 */

#[repr(C, packed)]
pub struct acpi_aest_node_interrupt_v2 {
    pub type: u8;
    pub reserved: [u8; 2];
    pub flags: u8;
    pub gsiv: u32;
    pub reserved1: [u8; 4];
};

/* Values for Type field above */

pub const ACPI_AEST_NODE_FAULT_HANDLING: u64 = 0;
pub const ACPI_AEST_NODE_ERROR_RECOVERY: u64 = 1;
pub const ACPI_AEST_XRUPT_RESERVED: u64 = 2	/* 2 and above are reserved */;

/*******************************************************************************
 * AGDI - Arm Generic Diagnostic Dump and Reset Device Interface
 *
 * Conforms to "ACPI for Arm Components 1.1, Platform Design Document"
 * ARM DEN0093 v1.1
 *
 ******************************************************************************/
#[repr(C, packed)]
pub struct acpi_table_agdi {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub flags: u8;
    pub reserved: [u8; 3];
    pub sdei_event: u32;
    pub gsiv: u32;
};

/* Mask for Flags field above */

pub const ACPI_AGDI_SIGNALING_MODE: u64 = (1);

/*******************************************************************************
 *
 * APMT - ARM Performance Monitoring Unit Table
 *
 * Conforms to:
 * ARM Performance Monitoring Unit Architecture 1.0 Platform Design Document
 * ARM DEN0117 v1.0 November 25, 2021
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_apmt {
    pub header: acpi_table_header;	/* Common ACPI table header */
};

pub const ACPI_APMT_NODE_ID_LENGTH: u64 = 4;

/*
 * APMT subtables
 */
#[repr(C, packed)]
pub struct acpi_apmt_node {
    pub length: u16;
    pub flags: u8;
    pub type: u8;
    pub id: u32;
    pub inst_primary: u64;
    pub inst_secondary: u32;
    pub base_address0: u64;
    pub base_address1: u64;
    pub ovflw_irq: u32;
    pub reserved: u32;
    pub ovflw_irq_flags: u32;
    pub proc_affinity: u32;
    pub impl_id: u32;
};

/* Masks for Flags field above */

pub const ACPI_APMT_FLAGS_DUAL_PAGE: u64 = (1u64 << 0);
pub const ACPI_APMT_FLAGS_AFFINITY: u64 = (1u64 << 1);
pub const ACPI_APMT_FLAGS_ATOMIC: u64 = (1u64 << 2);

/* Values for Flags dual page field above */

pub const ACPI_APMT_FLAGS_DUAL_PAGE_NSUPP: u64 = (0<<0);
pub const ACPI_APMT_FLAGS_DUAL_PAGE_SUPP: u64 = (1u64 << 0);

/* Values for Flags processor affinity field above */
pub const ACPI_APMT_FLAGS_AFFINITY_PROC: u64 = (0<<1);
pub const ACPI_APMT_FLAGS_AFFINITY_PROC_CONTAINER: u64 = (1u64 << 1);

/* Values for Flags 64-bit atomic field above */
pub const ACPI_APMT_FLAGS_ATOMIC_NSUPP: u64 = (0<<2);
pub const ACPI_APMT_FLAGS_ATOMIC_SUPP: u64 = (1u64 << 2);

/* Values for Type field above */

#[repr(i32)]
pub enum acpi_apmt_node_type {
	ACPI_APMT_NODE_TYPE_MC = 0x00,
	ACPI_APMT_NODE_TYPE_SMMU = 0x01,
	ACPI_APMT_NODE_TYPE_PCIE_ROOT = 0x02,
	ACPI_APMT_NODE_TYPE_ACPI = 0x03,
	ACPI_APMT_NODE_TYPE_CACHE = 0x04,
	ACPI_APMT_NODE_TYPE_COUNT
};

/* Masks for ovflw_irq_flags field above */

pub const ACPI_APMT_OVFLW_IRQ_FLAGS_MODE: u64 = (1u64 << 0);
pub const ACPI_APMT_OVFLW_IRQ_FLAGS_TYPE: u64 = (1u64 << 1);

/* Values for ovflw_irq_flags mode field above */

pub const ACPI_APMT_OVFLW_IRQ_FLAGS_MODE_LEVEL: u64 = (0<<0);
pub const ACPI_APMT_OVFLW_IRQ_FLAGS_MODE_EDGE: u64 = (1u64 << 0);

/* Values for ovflw_irq_flags type field above */

pub const ACPI_APMT_OVFLW_IRQ_FLAGS_TYPE_WIRED: u64 = (0<<1);

/*******************************************************************************
 *
 * BDAT - BIOS Data ACPI Table
 *
 * Conforms to "BIOS Data ACPI Table", Interface Specification v4.0 Draft 5
 * Nov 2020
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_bdat {
    pub header: acpi_table_header;
    pub gas: acpi_generic_address;
};

/*******************************************************************************
 *
 * CCEL - CC-Event Log
 *        From: "Guest-Host-Communication Interface (GHCI) for Intel
 *        Trust Domain Extensions (Intel TDX)". Feb 2022
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_ccel {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub CCtype: u8;
    pub Ccsub_type: u8;
    pub reserved: u16;
    pub log_area_minimum_length: u64;
    pub log_area_start_address: u64;
};

/*******************************************************************************
 *
 * ERDT - Enhanced Resource Director Technology (ERDT) table
 *
 * Conforms to "Intel Resource Director Technology Architecture Specification"
 * Version 1.1, January 2025
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_erdt {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub max_clos: u32;		/* Maximum classes of service */
    pub reserved: [u8; 24];
    pub erdt_substructures: [u8; 0];
};

/* Values for subtable type in struct acpi_subtbl_hdr_16 */

#[repr(i32)]
pub enum acpi_erdt_type {
	ACPI_ERDT_TYPE_RMDD = 0,
	ACPI_ERDT_TYPE_CACD = 1,
	ACPI_ERDT_TYPE_DACD = 2,
	ACPI_ERDT_TYPE_CMRC = 3,
	ACPI_ERDT_TYPE_MMRC = 4,
	ACPI_ERDT_TYPE_MARC = 5,
	ACPI_ERDT_TYPE_CARC = 6,
	ACPI_ERDT_TYPE_CMRD = 7,
	ACPI_ERDT_TYPE_IBRD = 8,
	ACPI_ERDT_TYPE_IBAD = 9,
	ACPI_ERDT_TYPE_CARD = 10,
	ACPI_ERDT_TYPE_RESERVED = 11	/* 11 and above are reserved */
};

/*
 * ERDT Subtables, correspond to Type in struct acpi_subtbl_hdr_16
 */

/* 0: RMDD - Resource Management Domain Description */

#[repr(C, packed)]
pub struct acpi_erdt_rmdd {
    pub header: acpi_subtbl_hdr_16;
    pub flags: u16;
    pub IO_l3_slices: u16;	/* Number of slices in IO cache */
    pub IO_l3_sets: u8;		/* Number of sets in IO cache */
    pub IO_l3_ways: u8;		/* Number of ways in IO cache */
    pub reserved: u64;
    pub domain_id: u16;		/* Unique domain ID */
    pub max_rmid: u32;		/* Maximun RMID supported */
    pub creg_base: u64;		/* Control Register Base Address */
    pub creg_size: u16;		/* Control Register Size (4K pages) */
    pub rmdd_structs: [u8; 0];
};

/* 1: CACD - CPU Agent Collection Description */

#[repr(C, packed)]
pub struct acpi_erdt_cacd {
    pub header: acpi_subtbl_hdr_16;
    pub reserved: u16;
    pub domain_id: u16;		/* Unique domain ID */
    pub X2APICIDS: [u32; 0];
};

/* 2: DACD - Device Agent Collection Description */

#[repr(C, packed)]
pub struct acpi_erdt_dacd {
    pub header: acpi_subtbl_hdr_16;
    pub reserved: u16;
    pub domain_id: u16;		/* Unique domain ID */
    pub dev_paths: [u8; 0];
};

#[repr(C, packed)]
pub struct acpi_erdt_dacd_dev_paths {
    pub header: acpi_subtable_header;
    pub segment: u16;
    pub reserved: u8;
    pub start_bus: u8;
    pub path: [u8; 0];
};

/* 3: CMRC - Cache Monitoring Registers for CPU Agents */

#[repr(C, packed)]
pub struct acpi_erdt_cmrc {
    pub header: acpi_subtbl_hdr_16;
    pub reserved1: u32;
    pub flags: u32;
    pub index_fn: u8;
    pub reserved2: [u8; 11];
    pub cmt_reg_base: u64;
    pub cmt_reg_size: u32;
    pub clump_size: u16;
    pub clump_stride: u16;
    pub up_scale: u64;
};

/* 4: MMRC - Memory-bandwidth Monitoring Registers for CPU Agents */

#[repr(C, packed)]
pub struct acpi_erdt_mmrc {
    pub header: acpi_subtbl_hdr_16;
    pub reserved1: u32;
    pub flags: u32;
    pub index_fn: u8;
    pub reserved2: [u8; 11];
    pub reg_base: u64;
    pub reg_size: u32;
    pub counter_width: u8;
    pub up_scale: u64;
    pub reserved3: [u8; 7];
    pub corr_factor_list_len: u32;
    pub corr_factor_list: [u32; 0];
};

/* 5: MARC - Memory-bandwidth Allocation Registers for CPU Agents */

#[repr(C, packed)]
pub struct acpi_erdt_marc {
    pub header: acpi_subtbl_hdr_16;
    pub reserved1: u16;
    pub flags: u16;
    pub index_fn: u8;
    pub reserved2: [u8; 7];
    pub reg_base_opt: u64;
    pub reg_base_min: u64;
    pub reg_base_max: u64;
    pub mba_reg_size: u32;
    pub mba_ctrl_range: u32;
};

/* 6: CARC - Cache Allocation Registers for CPU Agents */

#[repr(C, packed)]
pub struct acpi_erdt_carc {
    pub header: acpi_subtbl_hdr_16;
};

/* 7: CMRD - Cache Monitoring Registers for Device Agents */

#[repr(C, packed)]
pub struct acpi_erdt_cmrd {
    pub header: acpi_subtbl_hdr_16;
    pub reserved1: u32;
    pub flags: u32;
    pub index_fn: u8;
    pub reserved2: [u8; 11];
    pub reg_base: u64;
    pub reg_size: u32;
    pub cmt_reg_off: u16;
    pub cmt_clump_size: u16;
    pub up_scale: u64;
};

/* 8: IBRD - Cache Monitoring Registers for Device Agents */

#[repr(C, packed)]
pub struct acpi_erdt_ibrd {
    pub header: acpi_subtbl_hdr_16;
    pub reserved1: u32;
    pub flags: u32;
    pub index_fn: u8;
    pub reserved2: [u8; 11];
    pub reg_base: u64;
    pub reg_size: u32;
    pub total_bw_offset: u16;
    pub Iomiss_bw_offset: u16;
    pub total_bw_clump: u16;
    pub Iomiss_bw_clump: u16;
    pub reserved3: [u8; 7];
    pub counter_width: u8;
    pub up_scale: u64;
    pub corr_factor_list_len: u32;
    pub corr_factor_list: [u32; 0];
};

/* 9: IBAD - IO bandwidth Allocation Registers for device agents */

#[repr(C, packed)]
pub struct acpi_erdt_ibad {
    pub header: acpi_subtbl_hdr_16;
};

/* 10: CARD - IO bandwidth Allocation Registers for Device Agents */

#[repr(C, packed)]
pub struct acpi_erdt_card {
    pub header: acpi_subtbl_hdr_16;
    pub reserved1: u32;
    pub flags: u32;
    pub contention_mask: u32;
    pub index_fn: u8;
    pub reserved2: [u8; 7];
    pub reg_base: u64;
    pub reg_size: u32;
    pub cat_reg_offset: u16;
    pub cat_reg_block_size: u16;
};

/*******************************************************************************
 *
 * IORT - IO Remapping Table
 *
 * Conforms to "IO Remapping Table System Software on ARM Platforms",
 * Document number: ARM DEN 0049E.f, Apr 2024
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_iort {
    pub header: acpi_table_header;
    pub node_count: u32;
    pub node_offset: u32;
    pub reserved: u32;
};

/*
 * IORT subtables
 */
#[repr(C, packed)]
pub struct acpi_iort_node {
    pub type: u8;
    pub length: u16;
    pub revision: u8;
    pub identifier: u32;
    pub mapping_count: u32;
    pub mapping_offset: u32;
    pub node_data: [i8; 0];
};

/* Values for subtable Type above */

#[repr(i32)]
pub enum acpi_iort_node_type {
	ACPI_IORT_NODE_ITS_GROUP = 0x00,
	ACPI_IORT_NODE_NAMED_COMPONENT = 0x01,
	ACPI_IORT_NODE_PCI_ROOT_COMPLEX = 0x02,
	ACPI_IORT_NODE_SMMU = 0x03,
	ACPI_IORT_NODE_SMMU_V3 = 0x04,
	ACPI_IORT_NODE_PMCG = 0x05,
	ACPI_IORT_NODE_RMR = 0x06,
	ACPI_IORT_NODE_IWB = 0x07,
};

#[repr(C, packed)]
pub struct acpi_iort_id_mapping {
    pub input_base: u32;		/* Lowest value in input range */
    pub id_count: u32;		/* Number of IDs */
    pub output_base: u32;	/* Lowest value in output range */
    pub output_reference: u32;	/* A reference to the output node */
    pub flags: u32;
};

/* Masks for Flags field above for IORT subtable */

pub const ACPI_IORT_ID_SINGLE_MAPPING: u64 = (1);

#[repr(C, packed)]
pub struct acpi_iort_memory_access {
    pub cache_coherency: u32;
    pub hints: u8;
    pub reserved: u16;
    pub memory_flags: u8;
};

/* Values for cache_coherency field above */

pub const ACPI_IORT_NODE_COHERENT: u64 = 0x00000001	/* The device node is fully coherent */;
pub const ACPI_IORT_NODE_NOT_COHERENT: u64 = 0x00000000	/* The device node is not coherent */;

/* Masks for Hints field above */

pub const ACPI_IORT_HT_TRANSIENT: u64 = (1);
pub const ACPI_IORT_HT_WRITE: u64 = (1u64 << 1);
pub const ACPI_IORT_HT_READ: u64 = (1u64 << 2);
pub const ACPI_IORT_HT_OVERRIDE: u64 = (1u64 << 3);

/* Masks for memory_flags field above */

pub const ACPI_IORT_MF_COHERENCY: u64 = (1);
pub const ACPI_IORT_MF_ATTRIBUTES: u64 = (1u64 << 1);
pub const ACPI_IORT_MF_CANWBS: u64 = (1u64 << 2);

/*
 * IORT node specific subtables
 */
#[repr(C, packed)]
pub struct acpi_iort_its_group {
    pub its_count: u32;
    pub identifiers: [u32; 0];	/* GIC ITS identifier array */
};

#[repr(C, packed)]
pub struct acpi_iort_named_component {
    pub node_flags: u32;
    pub memory_properties: u64;	/* Memory access properties */
    pub memory_address_limit: u8;	/* Memory address size limit */
    pub device_name: [i8; 0];	/* Path of namespace object */
};

/* Masks for Flags field above */

pub const ACPI_IORT_NC_STALL_SUPPORTED: u64 = (1);
pub const ACPI_IORT_NC_PASID_BITS: u64 = (31u64 << 1);

#[repr(C, packed)]
pub struct acpi_iort_root_complex {
    pub memory_properties: u64;	/* Memory access properties */
    pub ats_attribute: u32;
    pub pci_segment_number: u32;
    pub memory_address_limit: u8;	/* Memory address size limit */
    pub pasid_capabilities: u16;	/* PASID Capabilities */
    pub reserved: [u8; 0];		/* Reserved, must be zero */
};

/* Masks for ats_attribute field above */

pub const ACPI_IORT_ATS_SUPPORTED: u64 = (1)	/* The root complex ATS support */;
pub const ACPI_IORT_PRI_SUPPORTED: u64 = (1u64 << 1)	/* The root complex PRI support */;
pub const ACPI_IORT_PASID_FWD_SUPPORTED: u64 = (1u64 << 2)	/* The root complex PASID forward support */;

/* Masks for pasid_capabilities field above */
pub const ACPI_IORT_PASID_MAX_WIDTH: u64 = (0x1F)	/* Bits 0-4 */;

#[repr(C, packed)]
pub struct acpi_iort_smmu {
    pub base_address: u64;	/* SMMU base address */
    pub span: u64;		/* Length of memory range */
    pub model: u32;
    pub flags: u32;
    pub global_interrupt_offset: u32;
    pub context_interrupt_count: u32;
    pub context_interrupt_offset: u32;
    pub pmu_interrupt_count: u32;
    pub pmu_interrupt_offset: u32;
    pub interrupts: [u64; 0];	/* Interrupt array */
};

/* Values for Model field above */

pub const ACPI_IORT_SMMU_V1: u64 = 0x00000000	/* Generic SMMUv1 */;
pub const ACPI_IORT_SMMU_V2: u64 = 0x00000001	/* Generic SMMUv2 */;
pub const ACPI_IORT_SMMU_CORELINK_MMU400: u64 = 0x00000002	/* ARM Corelink MMU-400 */;
pub const ACPI_IORT_SMMU_CORELINK_MMU500: u64 = 0x00000003	/* ARM Corelink MMU-500 */;
pub const ACPI_IORT_SMMU_CORELINK_MMU401: u64 = 0x00000004	/* ARM Corelink MMU-401 */;
pub const ACPI_IORT_SMMU_CAVIUM_THUNDERX: u64 = 0x00000005	/* Cavium thunder_x SMMUv2 */;

/* Masks for Flags field above */

pub const ACPI_IORT_SMMU_DVM_SUPPORTED: u64 = (1);
pub const ACPI_IORT_SMMU_COHERENT_WALK: u64 = (1u64 << 1);

/* Global interrupt format */

#[repr(C, packed)]
pub struct acpi_iort_smmu_gsi {
    pub nsg_irpt: u32;
    pub nsg_irpt_flags: u32;
    pub nsg_cfg_irpt: u32;
    pub nsg_cfg_irpt_flags: u32;
};

#[repr(C, packed)]
pub struct acpi_iort_smmu_v3 {
    pub base_address: u64;	/* SMMUv3 base address */
    pub flags: u32;
    pub reserved: u32;
    pub vatos_address: u64;
    pub model: u32;
    pub event_gsiv: u32;
    pub pri_gsiv: u32;
    pub gerr_gsiv: u32;
    pub sync_gsiv: u32;
    pub pxm: u32;
    pub id_mapping_index: u32;
};

/* Values for Model field above */

pub const ACPI_IORT_SMMU_V3_GENERIC: u64 = 0x00000000	/* Generic SMMUv3 */;
pub const ACPI_IORT_SMMU_V3_HISILICON_HI161X: u64 = 0x00000001	/* hi_silicon Hi161x SMMUv3 */;
pub const ACPI_IORT_SMMU_V3_CAVIUM_CN99XX: u64 = 0x00000002	/* Cavium CN99xx SMMUv3 */;

/* Masks for Flags field above */

pub const ACPI_IORT_SMMU_V3_COHACC_OVERRIDE: u64 = (1);
pub const ACPI_IORT_SMMU_V3_HTTU_OVERRIDE: u64 = (3u64 << 1);
pub const ACPI_IORT_SMMU_V3_PXM_VALID: u64 = (1u64 << 3);
pub const ACPI_IORT_SMMU_V3_DEVICEID_VALID: u64 = (1u64 << 4);

#[repr(C, packed)]
pub struct acpi_iort_pmcg {
    pub page0_base_address: u64;
    pub overflow_gsiv: u32;
    pub node_reference: u32;
    pub page1_base_address: u64;
};

#[repr(C, packed)]
pub struct acpi_iort_rmr {
    pub flags: u32;
    pub rmr_count: u32;
    pub rmr_offset: u32;
};

/* Masks for Flags field above */
pub const ACPI_IORT_RMR_REMAP_PERMITTED: u64 = (1);
pub const ACPI_IORT_RMR_ACCESS_PRIVILEGE: u64 = (1u64 << 1);

/*
 * Macro to access the Access Attributes in flags field above:
 *  Access Attributes is encoded in bits 9:2
 */
#[inline]
pub const fn ACPI_IORT_RMR_ACCESS_ATTRIBUTES(flags: u32) -> u32 { (((((flags) >> 2) & 0xFF)) as u32) }

/* Values for above Access Attributes */

pub const ACPI_IORT_RMR_ATTR_DEVICE_NGNRNE: u64 = 0x00;
pub const ACPI_IORT_RMR_ATTR_DEVICE_NGNRE: u64 = 0x01;
pub const ACPI_IORT_RMR_ATTR_DEVICE_NGRE: u64 = 0x02;
pub const ACPI_IORT_RMR_ATTR_DEVICE_GRE: u64 = 0x03;
pub const ACPI_IORT_RMR_ATTR_NORMAL_NC: u64 = 0x04;
pub const ACPI_IORT_RMR_ATTR_NORMAL_IWB_OWB: u64 = 0x05;

#[repr(C, packed)]
pub struct acpi_iort_rmr_desc {
    pub base_address: u64;
    pub length: u64;
    pub reserved: u32;
};

#[repr(C, packed)]
pub struct acpi_iort_iwb {
    pub base_address: u64;
    pub iwb_index: u16;		/* Unique IWB identifier matching with the IWB GSI namespace. */
    pub device_name: [i8; 0];	/* Path of the IWB namespace object */
};

/*******************************************************************************
 *
 * IOVT - I/O Virtualization Table
 *
 * Conforms to "LoongArch I/O Virtualization Table",
 *        Version 0.1, October 2024
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_iovt {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub iommu_count: u16;
    pub iommu_offset: u16;
    pub reserved: [u8; 8];
};

/* IOVT subtable header */

#[repr(C, packed)]
pub struct acpi_iovt_header {
    pub type: u16;
    pub length: u16;
};

/* Values for Type field above */

#[repr(i32)]
pub enum acpi_iovt_iommu_type {
	ACPI_IOVT_IOMMU_V1 = 0x00,
	ACPI_IOVT_IOMMU_RESERVED = 0x01	/* 1 and greater are reserved */
};

/* IOVT subtables */

#[repr(C, packed)]
pub struct acpi_iovt_iommu {
    pub header: acpi_iovt_header;
    pub flags: u32;
    pub segment: u16;
    pub phy_width: u16;		/* Physical Address Width */
    pub virt_width: u16;		/* Virtual Address Width */
    pub max_page_level: u16;
    pub page_size: u64;
    pub device_id: u32;
    pub base_address: u64;
    pub address_space_size: u32;
    pub interrupt_type: u8;
    pub reserved: [u8; 3];
    pub gsi_number: u32;
    pub proximity_domain: u32;
    pub max_device_num: u32;
    pub device_entry_num: u32;
    pub device_entry_offset: u32;
};

#[repr(C, packed)]
pub struct acpi_iovt_device_entry {
    pub type: u8;
    pub length: u8;
    pub flags: u8;
    pub reserved: [u8; 3];
    pub device_id: u16;
};

#[repr(i32)]
pub enum acpi_iovt_device_entry_type {
	ACPI_IOVT_DEVICE_ENTRY_SINGLE = 0x00,
	ACPI_IOVT_DEVICE_ENTRY_START = 0x01,
	ACPI_IOVT_DEVICE_ENTRY_END = 0x02,
	ACPI_IOVT_DEVICE_ENTRY_RESERVED = 0x03	/* 3 and greater are reserved */
};

/*******************************************************************************
 *
 * IVRS - I/O Virtualization Reporting Structure
 *        Version 1
 *
 * Conforms to "AMD I/O Virtualization Technology (IOMMU) Specification",
 * Revision 1.26, February 2009.
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_ivrs {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub info: u32;		/* Common virtualization info */
    pub reserved: u64;
};

/* Values for Info field above */

pub const ACPI_IVRS_PHYSICAL_SIZE: u64 = 0x00007F00	/* 7 bits, physical address size */;
pub const ACPI_IVRS_VIRTUAL_SIZE: u64 = 0x003F8000	/* 7 bits, virtual address size */;
pub const ACPI_IVRS_ATS_RESERVED: u64 = 0x00400000	/* ATS address translation range reserved */;

/* IVRS subtable header */

#[repr(C, packed)]
pub struct acpi_ivrs_header {
    pub type: u8;		/* Subtable type */
    pub flags: u8;
    pub length: u16;		/* Subtable length */
    pub device_id: u16;		/* ID of IOMMU */
};

/* Values for subtable Type above */

#[repr(i32)]
pub enum acpi_ivrs_type {
	ACPI_IVRS_TYPE_HARDWARE1 = 0x10,
	ACPI_IVRS_TYPE_HARDWARE2 = 0x11,
	ACPI_IVRS_TYPE_HARDWARE3 = 0x40,
	ACPI_IVRS_TYPE_MEMORY1 = 0x20,
	ACPI_IVRS_TYPE_MEMORY2 = 0x21,
	ACPI_IVRS_TYPE_MEMORY3 = 0x22
};

/* Masks for Flags field above for IVHD subtable */

pub const ACPI_IVHD_TT_ENABLE: u64 = (1);
pub const ACPI_IVHD_PASS_PW: u64 = (1u64 << 1);
pub const ACPI_IVHD_RES_PASS_PW: u64 = (1u64 << 2);
pub const ACPI_IVHD_ISOC: u64 = (1u64 << 3);
pub const ACPI_IVHD_IOTLB: u64 = (1u64 << 4);

/* Masks for Flags field above for IVMD subtable */

pub const ACPI_IVMD_UNITY: u64 = (1);
pub const ACPI_IVMD_READ: u64 = (1u64 << 1);
pub const ACPI_IVMD_WRITE: u64 = (1u64 << 2);
pub const ACPI_IVMD_EXCLUSION_RANGE: u64 = (1u64 << 3);

/*
 * IVRS subtables, correspond to Type in struct acpi_ivrs_header
 */

/* 0x10: I/O Virtualization Hardware Definition Block (IVHD) */

#[repr(C, packed)]
pub struct acpi_ivrs_hardware_10 {
    pub header: acpi_ivrs_header;
    pub capability_offset: u16;	/* Offset for IOMMU control fields */
    pub base_address: u64;	/* IOMMU control registers */
    pub pci_segment_group: u16;
    pub info: u16;		/* MSI number and unit ID */
    pub feature_reporting: u32;
};

/* 0x11: I/O Virtualization Hardware Definition Block (IVHD) */

#[repr(C, packed)]
pub struct acpi_ivrs_hardware_11 {
    pub header: acpi_ivrs_header;
    pub capability_offset: u16;	/* Offset for IOMMU control fields */
    pub base_address: u64;	/* IOMMU control registers */
    pub pci_segment_group: u16;
    pub info: u16;		/* MSI number and unit ID */
    pub attributes: u32;
    pub efr_register_image: u64;
    pub reserved: u64;
};

/* Masks for Info field above */

pub const ACPI_IVHD_MSI_NUMBER_MASK: u64 = 0x001F	/* 5 bits, MSI message number */;
pub const ACPI_IVHD_UNIT_ID_MASK: u64 = 0x1F00	/* 5 bits, unit_ID */;

/*
 * Device Entries for IVHD subtable, appear after struct acpi_ivrs_hardware structure.
 * Upper two bits of the Type field are the (encoded) length of the structure.
 * Currently, only 4 and 8 byte entries are defined. 16 and 32 byte entries
 * are reserved for future use but not defined.
 */
#[repr(C, packed)]
pub struct acpi_ivrs_de_header {
    pub type: u8;
    pub id: u16;
    pub data_setting: u8;
};

/* Length of device entry is in the top two bits of Type field above */

pub const ACPI_IVHD_ENTRY_LENGTH: u64 = 0xC0;

/* Values for device entry Type field above */

#[repr(i32)]
pub enum acpi_ivrs_device_entry_type {
	/* 4-byte device entries, all use struct acpi_ivrs_device4 */

	ACPI_IVRS_TYPE_PAD4 = 0,
	ACPI_IVRS_TYPE_ALL = 1,
	ACPI_IVRS_TYPE_SELECT = 2,
	ACPI_IVRS_TYPE_START = 3,
	ACPI_IVRS_TYPE_END = 4,

	/* 8-byte device entries */

	ACPI_IVRS_TYPE_PAD8 = 64,
	ACPI_IVRS_TYPE_NOT_USED = 65,
	ACPI_IVRS_TYPE_ALIAS_SELECT = 66,	/* Uses struct acpi_ivrs_device8a */
	ACPI_IVRS_TYPE_ALIAS_START = 67,	/* Uses struct acpi_ivrs_device8a */
	ACPI_IVRS_TYPE_EXT_SELECT = 70,	/* Uses struct acpi_ivrs_device8b */
	ACPI_IVRS_TYPE_EXT_START = 71,	/* Uses struct acpi_ivrs_device8b */
	ACPI_IVRS_TYPE_SPECIAL = 72,	/* Uses struct acpi_ivrs_device8c */

	/* Variable-length device entries */

	ACPI_IVRS_TYPE_HID = 240	/* Uses ACPI_IVRS_DEVICE_HID */
};

/* Values for Data field above */

pub const ACPI_IVHD_INIT_PASS: u64 = (1);
pub const ACPI_IVHD_EINT_PASS: u64 = (1u64 << 1);
pub const ACPI_IVHD_NMI_PASS: u64 = (1u64 << 2);
pub const ACPI_IVHD_SYSTEM_MGMT: u64 = (3<<4);
pub const ACPI_IVHD_LINT0_PASS: u64 = (1u64 << 6);
pub const ACPI_IVHD_LINT1_PASS: u64 = (1u64 << 7);

/* Types 0-4: 4-byte device entry */

#[repr(C, packed)]
pub struct acpi_ivrs_device4 {
    pub header: acpi_ivrs_de_header;
};

/* Types 66-67: 8-byte device entry */

#[repr(C, packed)]
pub struct acpi_ivrs_device8a {
    pub header: acpi_ivrs_de_header;
    pub reserved1: u8;
    pub used_id: u16;
    pub reserved2: u8;
};

/* Types 70-71: 8-byte device entry */

#[repr(C, packed)]
pub struct acpi_ivrs_device8b {
    pub header: acpi_ivrs_de_header;
    pub extended_data: u32;
};

/* Values for extended_data above */

pub const ACPI_IVHD_ATS_DISABLED: u64 = (1u64 << 31);

/* Type 72: 8-byte device entry */

#[repr(C, packed)]
pub struct acpi_ivrs_device8c {
    pub header: acpi_ivrs_de_header;
    pub handle: u8;
    pub used_id: u16;
    pub variety: u8;
};

/* Values for Variety field above */

pub const ACPI_IVHD_IOAPIC: u64 = 1;
pub const ACPI_IVHD_HPET: u64 = 2;

/* Type 240: variable-length device entry */

#[repr(C, packed)]
pub struct acpi_ivrs_device_hid {
    pub header: acpi_ivrs_de_header;
    pub acpi_hid: u64;
    pub acpi_cid: u64;
    pub uid_type: u8;
    pub uid_length: u8;
};

/* Values for uid_type above */

pub const ACPI_IVRS_UID_NOT_PRESENT: u64 = 0;
pub const ACPI_IVRS_UID_IS_INTEGER: u64 = 1;
pub const ACPI_IVRS_UID_IS_STRING: u64 = 2;

/* 0x20, 0x21, 0x22: I/O Virtualization Memory Definition Block (IVMD) */

#[repr(C, packed)]
pub struct acpi_ivrs_memory {
    pub header: acpi_ivrs_header;
    pub aux_data: u16;
    pub reserved: u64;
    pub start_address: u64;
    pub memory_length: u64;
};

/*******************************************************************************
 *
 * KEYP - Key Programming Interface for Root Complex Integrity and Data
 *        Encryption (IDE)
 *        Version 1
 *
 * Conforms to "Key Programming Interface for Root Complex Integrity and Data
 * Encryption (IDE)" document. See under ACPI-Related Documents.
 *
 ******************************************************************************/
#[repr(C, packed)]
pub struct acpi_table_keyp {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub reserved: u32;
};

/* KEYP common subtable header */

#[repr(C, packed)]
pub struct acpi_keyp_common_header {
    pub type: u8;
    pub reserved: u8;
    pub length: u16;
};

/* Values for Type field above */

#[repr(i32)]
pub enum acpi_keyp_type {
	ACPI_KEYP_TYPE_CONFIG_UNIT = 0,
};

/* Root Port Information Structure */

#[repr(C, packed)]
pub struct acpi_keyp_rp_info {
    pub segment: u16;
    pub bus: u8;
    pub devfn: u8;
};

/* Key Configuration Unit Structure */

#[repr(C, packed)]
pub struct acpi_keyp_config_unit {
    pub header: acpi_keyp_common_header;
    pub protocol_type: u8;
    pub version: u8;
    pub root_port_count: u8;
    pub flags: u8;
    pub register_base_address: u64;
	struct acpi_keyp_rp_info rp_info[];
};

#[repr(i32)]
pub enum acpi_keyp_protocol_type {
	ACPI_KEYP_PROTO_TYPE_INVALID = 0,
	ACPI_KEYP_PROTO_TYPE_PCIE,
	ACPI_KEYP_PROTO_TYPE_CXL,
	ACPI_KEYP_PROTO_TYPE_RESERVED
};

pub const ACPI_KEYP_F_TVM_USABLE: u64 = (1);

/*******************************************************************************
 *
 * LPIT - Low Power Idle Table
 *
 * Conforms to "ACPI Low Power Idle Table (LPIT)" July 2014.
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_lpit {
    pub header: acpi_table_header;	/* Common ACPI table header */
};

/* LPIT subtable header */

#[repr(C, packed)]
pub struct acpi_lpit_header {
    pub type: u32;		/* Subtable type */
    pub length: u32;		/* Subtable length */
    pub unique_id: u16;
    pub reserved: u16;
    pub flags: u32;
};

/* Values for subtable Type above */

#[repr(i32)]
pub enum acpi_lpit_type {
	ACPI_LPIT_TYPE_NATIVE_CSTATE = 0x00,
	ACPI_LPIT_TYPE_RESERVED = 0x01	/* 1 and above are reserved */
};

/* Masks for Flags field above  */

pub const ACPI_LPIT_STATE_DISABLED: u64 = (1);
pub const ACPI_LPIT_NO_COUNTER: u64 = (1u64 << 1);

/*
 * LPIT subtables, correspond to Type in struct acpi_lpit_header
 */

/* 0x00: Native C-state instruction based LPI structure */

#[repr(C, packed)]
pub struct acpi_lpit_native {
    pub header: acpi_lpit_header;
    pub entry_trigger: acpi_generic_address;
    pub residency: u32;
    pub latency: u32;
    pub residency_counter: acpi_generic_address;
    pub counter_frequency: u64;
};

/*******************************************************************************
 *
 * MADT - Multiple APIC Description Table
 *        Version 3
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_madt {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub address: u32;		/* Physical address of local APIC */
    pub flags: u32;
};

/* Masks for Flags field above */

pub const ACPI_MADT_PCAT_COMPAT: u64 = (1)	/* 00: System also has dual 8259s */;

/* Values for PCATCompat flag */

pub const ACPI_MADT_DUAL_PIC: u64 = 1;
pub const ACPI_MADT_MULTIPLE_APIC: u64 = 0;

/* Values for MADT subtable type in struct acpi_subtable_header */

#[repr(i32)]
pub enum acpi_madt_type {
	ACPI_MADT_TYPE_LOCAL_APIC = 0,
	ACPI_MADT_TYPE_IO_APIC = 1,
	ACPI_MADT_TYPE_INTERRUPT_OVERRIDE = 2,
	ACPI_MADT_TYPE_NMI_SOURCE = 3,
	ACPI_MADT_TYPE_LOCAL_APIC_NMI = 4,
	ACPI_MADT_TYPE_LOCAL_APIC_OVERRIDE = 5,
	ACPI_MADT_TYPE_IO_SAPIC = 6,
	ACPI_MADT_TYPE_LOCAL_SAPIC = 7,
	ACPI_MADT_TYPE_INTERRUPT_SOURCE = 8,
	ACPI_MADT_TYPE_LOCAL_X2APIC = 9,
	ACPI_MADT_TYPE_LOCAL_X2APIC_NMI = 10,
	ACPI_MADT_TYPE_GENERIC_INTERRUPT = 11,
	ACPI_MADT_TYPE_GENERIC_DISTRIBUTOR = 12,
	ACPI_MADT_TYPE_GENERIC_MSI_FRAME = 13,
	ACPI_MADT_TYPE_GENERIC_REDISTRIBUTOR = 14,
	ACPI_MADT_TYPE_GENERIC_TRANSLATOR = 15,
	ACPI_MADT_TYPE_MULTIPROC_WAKEUP = 16,
	ACPI_MADT_TYPE_CORE_PIC = 17,
	ACPI_MADT_TYPE_LIO_PIC = 18,
	ACPI_MADT_TYPE_HT_PIC = 19,
	ACPI_MADT_TYPE_EIO_PIC = 20,
	ACPI_MADT_TYPE_MSI_PIC = 21,
	ACPI_MADT_TYPE_BIO_PIC = 22,
	ACPI_MADT_TYPE_LPC_PIC = 23,
	ACPI_MADT_TYPE_RINTC = 24,
	ACPI_MADT_TYPE_IMSIC = 25,
	ACPI_MADT_TYPE_APLIC = 26,
	ACPI_MADT_TYPE_PLIC = 27,
	ACPI_MADT_TYPE_GICV5_IRS = 28,
	ACPI_MADT_TYPE_GICV5_ITS = 29,
	ACPI_MADT_TYPE_GICV5_ITS_TRANSLATE = 30,
	ACPI_MADT_TYPE_RESERVED = 31,	/* 31 to 0x7F are reserved */
	ACPI_MADT_TYPE_OEM_RESERVED = 0x80	/* 0x80 to 0xFF are reserved for OEM use */
};

/*
 * MADT Subtables, correspond to Type in struct acpi_subtable_header
 */

/* 0: Processor Local APIC */

#[repr(C, packed)]
pub struct acpi_madt_local_apic {
    pub header: acpi_subtable_header;
    pub processor_id: u8;	/* ACPI processor id */
    pub id: u8;			/* Processor's local APIC id */
    pub lapic_flags: u32;
};

/* 1: IO APIC */

#[repr(C, packed)]
pub struct acpi_madt_io_apic {
    pub header: acpi_subtable_header;
    pub id: u8;			/* I/O APIC ID */
    pub reserved: u8;		/* reserved - must be zero */
    pub address: u32;		/* APIC physical address */
    pub global_irq_base: u32;	/* Global system interrupt where INTI lines start */
};

/* 2: Interrupt Override */

#[repr(C, packed)]
pub struct acpi_madt_interrupt_override {
    pub header: acpi_subtable_header;
    pub bus: u8;			/* 0 - ISA */
    pub source_irq: u8;		/* Interrupt source (IRQ) */
    pub global_irq: u32;		/* Global system interrupt */
    pub inti_flags: u16;
};

/* 3: NMI Source */

#[repr(C, packed)]
pub struct acpi_madt_nmi_source {
    pub header: acpi_subtable_header;
    pub inti_flags: u16;
    pub global_irq: u32;		/* Global system interrupt */
};

/* 4: Local APIC NMI */

#[repr(C, packed)]
pub struct acpi_madt_local_apic_nmi {
    pub header: acpi_subtable_header;
    pub processor_id: u8;	/* ACPI processor id */
    pub inti_flags: u16;
    pub lint: u8;		/* LINTn to which NMI is connected */
};

/* 5: Address Override */

#[repr(C, packed)]
pub struct acpi_madt_local_apic_override {
    pub header: acpi_subtable_header;
    pub reserved: u16;		/* Reserved, must be zero */
    pub address: u64;		/* APIC physical address */
};

/* 6: I/O Sapic */

#[repr(C, packed)]
pub struct acpi_madt_io_sapic {
    pub header: acpi_subtable_header;
    pub id: u8;			/* I/O SAPIC ID */
    pub reserved: u8;		/* Reserved, must be zero */
    pub global_irq_base: u32;	/* Global interrupt for SAPIC start */
    pub address: u64;		/* SAPIC physical address */
};

/* 7: Local Sapic */

#[repr(C, packed)]
pub struct acpi_madt_local_sapic {
    pub header: acpi_subtable_header;
    pub processor_id: u8;	/* ACPI processor id */
    pub id: u8;			/* SAPIC ID */
    pub eid: u8;			/* SAPIC EID */
    pub reserved: [u8; 3];		/* Reserved, must be zero */
    pub lapic_flags: u32;
    pub uid: u32;		/* Numeric UID - ACPI 3.0 */
    pub uid_string: [i8; 0];	/* String UID  - ACPI 3.0 */
};

/* 8: Platform Interrupt Source */

#[repr(C, packed)]
pub struct acpi_madt_interrupt_source {
    pub header: acpi_subtable_header;
    pub inti_flags: u16;
    pub type: u8;		/* 1=PMI, 2=INIT, 3=corrected */
    pub id: u8;			/* Processor ID */
    pub eid: u8;			/* Processor EID */
    pub io_sapic_vector: u8;	/* Vector value for PMI interrupts */
    pub global_irq: u32;		/* Global system interrupt */
    pub flags: u32;		/* Interrupt Source Flags */
};

/* Masks for Flags field above */

pub const ACPI_MADT_CPEI_OVERRIDE: u64 = (1);

/* 9: Processor Local X2APIC (ACPI 4.0) */

#[repr(C, packed)]
pub struct acpi_madt_local_x2apic {
    pub header: acpi_subtable_header;
    pub reserved: u16;		/* reserved - must be zero */
    pub local_apic_id: u32;	/* Processor x2APIC ID  */
    pub lapic_flags: u32;
    pub uid: u32;		/* ACPI processor UID */
};

/* 10: Local X2APIC NMI (ACPI 4.0) */

#[repr(C, packed)]
pub struct acpi_madt_local_x2apic_nmi {
    pub header: acpi_subtable_header;
    pub inti_flags: u16;
    pub uid: u32;		/* ACPI processor UID */
    pub lint: u8;		/* LINTn to which NMI is connected */
    pub reserved: [u8; 3];		/* reserved - must be zero */
};

/* 11: Generic interrupt - GICC (ACPI 5.0 + ACPI 6.0 + ACPI 6.3 + ACPI 6.5 + ACPI 6.7 changes) */

#[repr(C, packed)]
pub struct acpi_madt_generic_interrupt {
    pub header: acpi_subtable_header;
    pub reserved: u16;		/* reserved - must be zero */
    pub cpu_interface_number: u32;
    pub uid: u32;
    pub flags: u32;
    pub parking_version: u32;
    pub performance_interrupt: u32;
    pub parked_address: u64;
    pub base_address: u64;
    pub gicv_base_address: u64;
    pub gich_base_address: u64;
    pub vgic_interrupt: u32;
    pub gicr_base_address: u64;
    pub arm_mpidr: u64;
    pub efficiency_class: u8;
    pub reserved2: [u8; 1];
    pub spe_interrupt: u16;	/* ACPI 6.3 */
    pub trbe_interrupt: u16;	/* ACPI 6.5 */
    pub iaffid: u16;		/* ACPI 6.7 */
    pub irs_id: u32;
};

/* Masks for Flags field above */

/* ACPI_MADT_ENABLED                    (1)      Processor is usable if set */
pub const ACPI_MADT_PERFORMANCE_IRQ_MODE: u64 = (1u64 << 1)	/* 01: Performance Interrupt Mode */;
pub const ACPI_MADT_VGIC_IRQ_MODE: u64 = (1u64 << 2)	/* 02: VGIC Maintenance Interrupt mode */;
pub const ACPI_MADT_GICC_ONLINE_CAPABLE: u64 = (1u64 << 3)	/* 03: Processor is online capable  */;
pub const ACPI_MADT_GICC_NON_COHERENT: u64 = (1u64 << 4)	/* 04: GIC redistributor is not coherent */;

/* 12: Generic Distributor (ACPI 5.0 + ACPI 6.0 changes) */

#[repr(C, packed)]
pub struct acpi_madt_generic_distributor {
    pub header: acpi_subtable_header;
    pub reserved: u16;		/* reserved - must be zero */
    pub gic_id: u32;
    pub base_address: u64;
    pub global_irq_base: u32;
    pub version: u8;
    pub reserved2: [u8; 3];	/* reserved - must be zero */
};

/* Values for Version field above and Version field in acpi_madt_gicv5_irs */

#[repr(i32)]
pub enum acpi_madt_gic_version {
	ACPI_MADT_GIC_VERSION_NONE = 0,
	ACPI_MADT_GIC_VERSION_V1 = 1,
	ACPI_MADT_GIC_VERSION_V2 = 2,
	ACPI_MADT_GIC_VERSION_V3 = 3,
	ACPI_MADT_GIC_VERSION_V4 = 4,
	ACPI_MADT_GIC_VERSION_V5 = 5,
	ACPI_MADT_GIC_VERSION_RESERVED = 6	/* 6 and greater are reserved */
};

/* 13: Generic MSI Frame (ACPI 5.1) */

#[repr(C, packed)]
pub struct acpi_madt_generic_msi_frame {
    pub header: acpi_subtable_header;
    pub reserved: u16;		/* reserved - must be zero */
    pub msi_frame_id: u32;
    pub base_address: u64;
    pub flags: u32;
    pub spi_count: u16;
    pub spi_base: u16;
};

/* Masks for Flags field above */

pub const ACPI_MADT_OVERRIDE_SPI_VALUES: u64 = (1);

/* 14: Generic Redistributor (ACPI 5.1) */

#[repr(C, packed)]
pub struct acpi_madt_generic_redistributor {
    pub header: acpi_subtable_header;
    pub flags: u8;
    pub reserved: u8;		/* reserved - must be zero */
    pub base_address: u64;
    pub length: u32;
};

pub const ACPI_MADT_GICR_NON_COHERENT: u64 = (1);

/* 15: Generic Translator (ACPI 6.0) */

#[repr(C, packed)]
pub struct acpi_madt_generic_translator {
    pub header: acpi_subtable_header;
    pub flags: u8;
    pub reserved: u8;		/* reserved - must be zero */
    pub translation_id: u32;
    pub base_address: u64;
    pub reserved2: u32;
};

pub const ACPI_MADT_ITS_NON_COHERENT: u64 = (1);

/* 16: Multiprocessor wakeup (ACPI 6.6) */

#[repr(C, packed)]
pub struct acpi_madt_multiproc_wakeup {
    pub header: acpi_subtable_header;
    pub version: u16;
    pub reserved: u32;		/* reserved - must be zero */
    pub mailbox_address: u64;
    pub reset_vector: u64;
};

/* Values for Version field above */

#[repr(i32)]
pub enum acpi_madt_multiproc_wakeup_version {
	ACPI_MADT_MP_WAKEUP_VERSION_NONE = 0,
	ACPI_MADT_MP_WAKEUP_VERSION_V1 = 1,
	ACPI_MADT_MP_WAKEUP_VERSION_RESERVED = 2, /* 2 and greater are reserved */
};

pub const ACPI_MADT_MP_WAKEUP_SIZE_V0: u64 = 16;
pub const ACPI_MADT_MP_WAKEUP_SIZE_V1: u64 = 24;

pub const ACPI_MULTIPROC_WAKEUP_MB_OS_SIZE: u64 = 2032;
pub const ACPI_MULTIPROC_WAKEUP_MB_FIRMWARE_SIZE: u64 = 2048;

#[repr(C, packed)]
pub struct acpi_madt_multiproc_wakeup_mailbox {
    pub command: u16;
    pub reserved: u16;		/* reserved - must be zero */
    pub apic_id: u32;
    pub wakeup_vector: u64;
	u8 reserved_os[ACPI_MULTIPROC_WAKEUP_MB_OS_SIZE];	/* reserved for OS use */
	u8 reserved_firmware[ACPI_MULTIPROC_WAKEUP_MB_FIRMWARE_SIZE];	/* reserved for firmware use */
};

pub const ACPI_MP_WAKE_COMMAND_WAKEUP: u64 = 1;
pub const ACPI_MP_WAKE_COMMAND_TEST: u64 = 2;

/* 17: CPU Core Interrupt Controller (ACPI 6.5) */

#[repr(C, packed)]
pub struct acpi_madt_core_pic {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub processor_id: u32;
    pub core_id: u32;
    pub flags: u32;
};

/* Values for Version field above */

#[repr(i32)]
pub enum acpi_madt_core_pic_version {
	ACPI_MADT_CORE_PIC_VERSION_NONE = 0,
	ACPI_MADT_CORE_PIC_VERSION_V1 = 1,
	ACPI_MADT_CORE_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
};

/* 18: Legacy I/O Interrupt Controller (ACPI 6.5) */

#[repr(C, packed)]
pub struct acpi_madt_lio_pic {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub address: u64;
    pub size: u16;
    pub cascade: [u8; 2];
    pub cascade_map: [u32; 2];
};

/* Values for Version field above */

#[repr(i32)]
pub enum acpi_madt_lio_pic_version {
	ACPI_MADT_LIO_PIC_VERSION_NONE = 0,
	ACPI_MADT_LIO_PIC_VERSION_V1 = 1,
	ACPI_MADT_LIO_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
};

/* 19: HT Interrupt Controller (ACPI 6.5) */

#[repr(C, packed)]
pub struct acpi_madt_ht_pic {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub address: u64;
    pub size: u16;
    pub cascade: [u8; 8];
};

/* Values for Version field above */

#[repr(i32)]
pub enum acpi_madt_ht_pic_version {
	ACPI_MADT_HT_PIC_VERSION_NONE = 0,
	ACPI_MADT_HT_PIC_VERSION_V1 = 1,
	ACPI_MADT_HT_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
};

/* 20: Extend I/O Interrupt Controller (ACPI 6.5) */

#[repr(C, packed)]
pub struct acpi_madt_eio_pic {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub cascade: u8;
    pub node: u8;
    pub node_map: u64;
};

/* Values for Version field above */

#[repr(i32)]
pub enum acpi_madt_eio_pic_version {
	ACPI_MADT_EIO_PIC_VERSION_NONE = 0,
	ACPI_MADT_EIO_PIC_VERSION_V1 = 1,
	ACPI_MADT_EIO_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
};

/* 21: MSI Interrupt Controller (ACPI 6.5) */

#[repr(C, packed)]
pub struct acpi_madt_msi_pic {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub msg_address: u64;
    pub start: u32;
    pub count: u32;
};

/* Values for Version field above */

#[repr(i32)]
pub enum acpi_madt_msi_pic_version {
	ACPI_MADT_MSI_PIC_VERSION_NONE = 0,
	ACPI_MADT_MSI_PIC_VERSION_V1 = 1,
	ACPI_MADT_MSI_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
};

/* 22: Bridge I/O Interrupt Controller (ACPI 6.5) */

#[repr(C, packed)]
pub struct acpi_madt_bio_pic {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub address: u64;
    pub size: u16;
    pub id: u16;
    pub gsi_base: u16;
};

/* Values for Version field above */

#[repr(i32)]
pub enum acpi_madt_bio_pic_version {
	ACPI_MADT_BIO_PIC_VERSION_NONE = 0,
	ACPI_MADT_BIO_PIC_VERSION_V1 = 1,
	ACPI_MADT_BIO_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
};

/* 23: LPC Interrupt Controller (ACPI 6.5) */

#[repr(C, packed)]
pub struct acpi_madt_lpc_pic {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub address: u64;
    pub size: u16;
    pub cascade: u8;
};

/* Values for Version field above */

#[repr(i32)]
pub enum acpi_madt_lpc_pic_version {
	ACPI_MADT_LPC_PIC_VERSION_NONE = 0,
	ACPI_MADT_LPC_PIC_VERSION_V1 = 1,
	ACPI_MADT_LPC_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
};

/* 24: RISC-V INTC */
#[repr(C, packed)]
pub struct acpi_madt_rintc {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub reserved: u8;
    pub flags: u32;
    pub hart_id: u64;
    pub uid: u32;		/* ACPI processor UID */
    pub ext_intc_id: u32;	/* External INTC Id */
    pub imsic_addr: u64;		/* IMSIC base address */
    pub imsic_size: u32;		/* IMSIC size */
};

/* Values for RISC-V INTC Version field above */

#[repr(i32)]
pub enum acpi_madt_rintc_version {
	ACPI_MADT_RINTC_VERSION_NONE = 0,
	ACPI_MADT_RINTC_VERSION_V1 = 1,
	ACPI_MADT_RINTC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
};

/* 25: RISC-V IMSIC */
#[repr(C, packed)]
pub struct acpi_madt_imsic {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub reserved: u8;
    pub flags: u32;
    pub num_ids: u16;
    pub num_guest_ids: u16;
    pub guest_index_bits: u8;
    pub hart_index_bits: u8;
    pub group_index_bits: u8;
    pub group_index_shift: u8;
};

/* 26: RISC-V APLIC */
#[repr(C, packed)]
pub struct acpi_madt_aplic {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub id: u8;
    pub flags: u32;
    pub hw_id: [u8; 8];
    pub num_idcs: u16;
    pub num_sources: u16;
    pub gsi_base: u32;
    pub base_addr: u64;
    pub size: u32;
};

/* 27: RISC-V PLIC */
#[repr(C, packed)]
pub struct acpi_madt_plic {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub id: u8;
    pub hw_id: [u8; 8];
    pub num_irqs: u16;
    pub max_prio: u16;
    pub flags: u32;
    pub size: u32;
    pub base_addr: u64;
    pub gsi_base: u32;
};

/* 28: Arm GICv5 IRS (ACPI 6.7) */
#[repr(C, packed)]
pub struct acpi_madt_gicv5_irs {
    pub header: acpi_subtable_header;
    pub version: u8;
    pub reserved: u8;
    pub irs_id: u32;
    pub flags: u32;
    pub reserved2: u32;
    pub config_base_address: u64;
    pub setlpi_base_address: u64;
};

pub const ACPI_MADT_IRS_NON_COHERENT: u64 = (1);

/* 29: Arm GICv5 ITS Config Frame (ACPI 6.7) */
#[repr(C, packed)]
pub struct acpi_madt_gicv5_translator {
    pub header: acpi_subtable_header;
    pub flags: u8;
    pub reserved: u8;		/* reserved - must be zero */
    pub translator_id: u32;
    pub base_address: u64;
};

pub const ACPI_MADT_GICV5_ITS_NON_COHERENT: u64 = (1);

/* 30: Arm GICv5 ITS Translate Frame (ACPI 6.7) */
#[repr(C, packed)]
pub struct acpi_madt_gicv5_translate_frame {
    pub header: acpi_subtable_header;
    pub reserved: u16;		/* reserved - must be zero */
    pub linked_translator_id: u32;
    pub translate_frame_id: u32;
    pub reserved2: u32;
    pub base_address: u64;
};

/* 80: OEM data */

#[repr(C, packed)]
pub struct acpi_madt_oem_data {
	ACPI_FLEX_ARRAY(u8, oem_data);
};

/*
 * Common flags fields for MADT subtables
 */

/* MADT Local APIC flags */

pub const ACPI_MADT_ENABLED: u64 = (1)	/* 00: Processor is usable if set */;
pub const ACPI_MADT_ONLINE_CAPABLE: u64 = (2)	/* 01: System HW supports enabling processor at runtime */;

/* MADT MPS INTI flags (inti_flags) */

pub const ACPI_MADT_POLARITY_MASK: u64 = (3)	/* 00-01: Polarity of APIC I/O input signals */;
pub const ACPI_MADT_TRIGGER_MASK: u64 = (3<<2)	/* 02-03: Trigger mode of APIC input signals */;

/* Values for MPS INTI flags */

pub const ACPI_MADT_POLARITY_CONFORMS: u64 = 0;
pub const ACPI_MADT_POLARITY_ACTIVE_HIGH: u64 = 1;
pub const ACPI_MADT_POLARITY_RESERVED: u64 = 2;
pub const ACPI_MADT_POLARITY_ACTIVE_LOW: u64 = 3;

pub const ACPI_MADT_TRIGGER_CONFORMS: u64 = (0);
pub const ACPI_MADT_TRIGGER_EDGE: u64 = (1u64 << 2);
pub const ACPI_MADT_TRIGGER_RESERVED: u64 = (2<<2);
pub const ACPI_MADT_TRIGGER_LEVEL: u64 = (3<<2);

/*******************************************************************************
 *
 * MCFG - PCI Memory Mapped Configuration table and subtable
 *        Version 1
 *
 * Conforms to "PCI Firmware Specification", Revision 3.0, June 20, 2005
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_mcfg {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub reserved: [u8; 8];
};

/* Subtable */

#[repr(C, packed)]
pub struct acpi_mcfg_allocation {
    pub address: u64;		/* Base address, processor-relative */
    pub pci_segment: u16;	/* PCI segment group number */
    pub start_bus_number: u8;	/* Starting PCI Bus number */
    pub end_bus_number: u8;	/* Final PCI Bus number */
    pub reserved: u32;
};

/*******************************************************************************
 *
 * MCHI - Management Controller Host Interface Table
 *        Version 1
 *
 * Conforms to "Management Component Transport Protocol (MCTP) Host
 * Interface Specification", Revision 1.0.0a, October 13, 2009
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_mchi {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub interface_type: u8;
    pub protocol: u8;
    pub protocol_data: u64;
    pub interrupt_type: u8;
    pub gpe: u8;
    pub pci_device_flag: u8;
    pub global_interrupt: u32;
    pub control_register: acpi_generic_address;
    pub pci_segment: u8;
    pub pci_bus: u8;
    pub pci_device: u8;
    pub pci_function: u8;
};

/*******************************************************************************
 *
 * MPAM - Memory System Resource Partitioning and Monitoring
 *
 * Conforms to "ACPI for Memory System Resource Partitioning and Monitoring 2.0"
 * Document number: ARM DEN 0065, December, 2022.
 *
 ******************************************************************************/

/* MPAM RIS locator types. Table 11, Location types */
#[repr(i32)]
pub enum acpi_mpam_locator_type {
	ACPI_MPAM_LOCATION_TYPE_PROCESSOR_CACHE = 0,
	ACPI_MPAM_LOCATION_TYPE_MEMORY = 1,
	ACPI_MPAM_LOCATION_TYPE_SMMU = 2,
	ACPI_MPAM_LOCATION_TYPE_MEMORY_CACHE = 3,
	ACPI_MPAM_LOCATION_TYPE_ACPI_DEVICE = 4,
	ACPI_MPAM_LOCATION_TYPE_INTERCONNECT = 5,
	ACPI_MPAM_LOCATION_TYPE_UNKNOWN = 0xFF
};

/* MPAM Functional dependency descriptor. Table 10 */
#[repr(C, packed)]
pub struct acpi_mpam_func_deps {
    pub producer: u32;
    pub reserved: u32;
};

/* MPAM Processor cache locator descriptor. Table 13 */
#[repr(C, packed)]
pub struct acpi_mpam_resource_cache_locator {
    pub cache_reference: u64;
    pub reserved: u32;
};

/* MPAM Memory locator descriptor. Table 14 */
#[repr(C, packed)]
pub struct acpi_mpam_resource_memory_locator {
    pub proximity_domain: u64;
    pub reserved: u32;
};

/* MPAM SMMU locator descriptor. Table 15 */
#[repr(C, packed)]
pub struct acpi_mpam_resource_smmu_locator {
    pub smmu_interface: u64;
    pub reserved: u32;
};

/* MPAM Memory-side cache locator descriptor. Table 16 */
#[repr(C, packed)]
pub struct acpi_mpam_resource_memcache_locator {
    pub reserved: [u8; 7];
    pub level: u8;
    pub reference: u32;
};

/* MPAM ACPI device locator descriptor. Table 17 */
#[repr(C, packed)]
pub struct acpi_mpam_resource_acpi_locator {
    pub acpi_hw_id: u64;
    pub acpi_unique_id: u32;
};

/* MPAM Interconnect locator descriptor. Table 18 */
#[repr(C, packed)]
pub struct acpi_mpam_resource_interconnect_locator {
    pub inter_connect_desc_tbl_off: u64;
    pub reserved: u32;
};

/* MPAM Locator structure. Table 12 */
#[repr(C, packed)]
pub struct acpi_mpam_resource_generic_locator {
    pub descriptor1: u64;
    pub descriptor2: u32;
};

union acpi_mpam_resource_locator {
    pub cache_locator: acpi_mpam_resource_cache_locator;
    pub memory_locator: acpi_mpam_resource_memory_locator;
    pub smmu_locator: acpi_mpam_resource_smmu_locator;
    pub mem_cache_locator: acpi_mpam_resource_memcache_locator;
    pub acpi_locator: acpi_mpam_resource_acpi_locator;
    pub interconnect_ifc_locator: acpi_mpam_resource_interconnect_locator;
    pub generic_locator: acpi_mpam_resource_generic_locator;
};

/* Memory System Component Resource Node Structure Table 9 */
#[repr(C, packed)]
pub struct acpi_mpam_resource_node {
    pub identifier: u32;
    pub ris_index: u8;
    pub reserved1: u16;
    pub locator_type: u8;
	union acpi_mpam_resource_locator locator;
    pub num_functional_deps: u32;
};

/* Memory System Component (MSC) Node Structure. Table 4 */
#[repr(C, packed)]
pub struct acpi_mpam_msc_node {
    pub length: u16;
    pub interface_type: u8;
    pub reserved: u8;
    pub identifier: u32;
    pub base_address: u64;
    pub mmio_size: u32;
    pub overflow_interrupt: u32;
    pub overflow_interrupt_flags: u32;
    pub reserved1: u32;
    pub overflow_interrupt_affinity: u32;
    pub error_interrupt: u32;
    pub error_interrupt_flags: u32;
    pub reserved2: u32;
    pub error_interrupt_affinity: u32;
    pub max_nrdy_usec: u32;
    pub hardware_id_linked_device: u64;
    pub instance_id_linked_device: u32;
    pub num_resource_nodes: u32;
};

#[repr(C, packed)]
pub struct acpi_table_mpam {
    pub header: acpi_table_header;	/* Common ACPI table header */
};

/*******************************************************************************
 *
 * MPST - Memory Power State Table (ACPI 5.0)
 *        Version 1
 *
 ******************************************************************************/

pub const ACPI_MPST_CHANNEL_INFO: u64 = \;
    pub channel_id: u8; \
    pub reserved1: [u8; 3]; \
    pub power_node_count: u16; \
    pub reserved2: u16;

/* Main table */

#[repr(C, packed)]
pub struct acpi_table_mpst {
    pub header: acpi_table_header;	/* Common ACPI table header */
	 ACPI_MPST_CHANNEL_INFO	/* Platform Communication Channel */
};

/* Memory Platform Communication Channel Info */

#[repr(C, packed)]
pub struct acpi_mpst_channel {
	ACPI_MPST_CHANNEL_INFO	/* Platform Communication Channel */
};

/* Memory Power Node Structure */

#[repr(C, packed)]
pub struct acpi_mpst_power_node {
    pub flags: u8;
    pub reserved1: u8;
    pub node_id: u16;
    pub length: u32;
    pub range_address: u64;
    pub range_length: u64;
    pub num_power_states: u32;
    pub num_physical_components: u32;
};

/* Values for Flags field above */

pub const ACPI_MPST_ENABLED: u64 = 1;
pub const ACPI_MPST_POWER_MANAGED: u64 = 2;
pub const ACPI_MPST_HOT_PLUG_CAPABLE: u64 = 4;

/* Memory Power State Structure (follows POWER_NODE above) */

#[repr(C, packed)]
pub struct acpi_mpst_power_state {
    pub power_state: u8;
    pub info_index: u8;
};

/* Physical Component ID Structure (follows POWER_STATE above) */

#[repr(C, packed)]
pub struct acpi_mpst_component {
    pub component_id: u16;
};

/* Memory Power State Characteristics Structure (follows all POWER_NODEs) */

#[repr(C, packed)]
pub struct acpi_mpst_data_hdr {
    pub characteristics_count: u16;
    pub reserved: u16;
};

#[repr(C, packed)]
pub struct acpi_mpst_power_data {
    pub structure_id: u8;
    pub flags: u8;
    pub reserved1: u16;
    pub average_power: u32;
    pub power_saving: u32;
    pub exit_latency: u64;
    pub reserved2: u64;
};

/* Values for Flags field above */

pub const ACPI_MPST_PRESERVE: u64 = 1;
pub const ACPI_MPST_AUTOENTRY: u64 = 2;
pub const ACPI_MPST_AUTOEXIT: u64 = 4;

/* Shared Memory Region (not part of an ACPI table) */

#[repr(C, packed)]
pub struct acpi_mpst_shared {
    pub signature: u32;
    pub pcc_command: u16;
    pub pcc_status: u16;
    pub command_register: u32;
    pub status_register: u32;
    pub power_state_id: u32;
    pub power_node_id: u32;
    pub energy_consumed: u64;
    pub average_power: u64;
};

/*******************************************************************************
 *
 * MSCT - Maximum System Characteristics Table (ACPI 4.0)
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_msct {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub proximity_offset: u32;	/* Location of proximity info struct(s) */
    pub max_proximity_domains: u32;	/* Max number of proximity domains */
    pub max_clock_domains: u32;	/* Max number of clock domains */
    pub max_address: u64;	/* Max physical address in system */
};

/* subtable - Maximum Proximity Domain Information. Version 1 */

#[repr(C, packed)]
pub struct acpi_msct_proximity {
    pub revision: u8;
    pub length: u8;
    pub range_start: u32;	/* Start of domain range */
    pub range_end: u32;		/* End of domain range */
    pub processor_capacity: u32;
    pub memory_capacity: u64;	/* In bytes */
};

/*******************************************************************************
 *
 * MRRM - Memory Range and Region Mapping (MRRM) table
 * Conforms to "Intel Resource Director Technology Architecture Specification"
 * Version 1.1, January 2025
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_mrrm {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub max_mem_region: u8;	/* Max Memory Regions supported */
    pub flags: u8;		/* Region assignment type */
    pub reserved: [u8; 26];
    pub memory_range_entry: [u8; 0];
};

/* Flags */
pub const ACPI_MRRM_FLAGS_REGION_ASSIGNMENT_OS: u64 = (1u64 << 0);

/*******************************************************************************
	*
	* Memory Range entry - Memory Range entry in MRRM table
	*
	******************************************************************************/

#[repr(C, packed)]
pub struct acpi_mrrm_mem_range_entry {
    pub header: acpi_subtbl_hdr_16;
    pub reserved0: u32;		/* Reserved */
    pub addr_base: u64;		/* Base addr of the mem range */
    pub addr_len: u64;		/* Length of the mem range */
    pub region_id_flags: u16;	/* Valid local or remote Region-ID */
    pub local_region_id: u8;	/* Platform-assigned static local Region-ID */
    pub remote_region_id: u8;	/* Platform-assigned static remote Region-ID */
    pub reserved1: u32;		/* Reserved */
	/* Region-ID Programming Registers[] */
};

/* Values for region_id_flags above */
pub const ACPI_MRRM_VALID_REGION_ID_FLAGS_LOCAL: u64 = (1u64 << 0);
pub const ACPI_MRRM_VALID_REGION_ID_FLAGS_REMOTE: u64 = (1u64 << 1);

/*******************************************************************************
 *
 * MSDM - Microsoft Data Management table
 *
 * Conforms to "Microsoft Software Licensing Tables (SLIC and MSDM)",
 * November 29, 2011. Copyright 2011 Microsoft
 *
 ******************************************************************************/

/* Basic MSDM table is only the common ACPI header */

#[repr(C, packed)]
pub struct acpi_table_msdm {
    pub header: acpi_table_header;	/* Common ACPI table header */
};

/*******************************************************************************
 *
 * NFIT - NVDIMM Interface Table (ACPI 6.0+)
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_nfit {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub reserved: u32;		/* Reserved, must be zero */
};

/* Subtable header for NFIT */

#[repr(C, packed)]
pub struct acpi_nfit_header {
    pub type: u16;
    pub length: u16;
};

/* Values for subtable type in struct acpi_nfit_header */

#[repr(i32)]
pub enum acpi_nfit_type {
	ACPI_NFIT_TYPE_SYSTEM_ADDRESS = 0,
	ACPI_NFIT_TYPE_MEMORY_MAP = 1,
	ACPI_NFIT_TYPE_INTERLEAVE = 2,
	ACPI_NFIT_TYPE_SMBIOS = 3,
	ACPI_NFIT_TYPE_CONTROL_REGION = 4,
	ACPI_NFIT_TYPE_DATA_REGION = 5,
	ACPI_NFIT_TYPE_FLUSH_ADDRESS = 6,
	ACPI_NFIT_TYPE_CAPABILITIES = 7,
	ACPI_NFIT_TYPE_RESERVED = 8	/* 8 and greater are reserved */
};

/*
 * NFIT Subtables
 */

/* 0: System Physical Address Range Structure */

#[repr(C, packed)]
pub struct acpi_nfit_system_address {
    pub header: acpi_nfit_header;
    pub range_index: u16;
    pub flags: u16;
    pub reserved: u32;		/* Reserved, must be zero */
    pub proximity_domain: u32;
    pub range_guid: [u8; 16];
    pub address: u64;
    pub length: u64;
    pub memory_mapping: u64;
    pub location_cookie: u64;	/* ACPI 6.4 */
};

/* Flags */

pub const ACPI_NFIT_ADD_ONLINE_ONLY: u64 = (1)	/* 00: Add/Online Operation Only */;
pub const ACPI_NFIT_PROXIMITY_VALID: u64 = (1u64 << 1)	/* 01: Proximity Domain Valid */;
pub const ACPI_NFIT_LOCATION_COOKIE_VALID: u64 = (1u64 << 2)	/* 02: SPA location cookie valid (ACPI 6.4) */;

/* Range Type GUIDs appear in the include/acuuid.h file */

/* 1: Memory Device to System Address Range Map Structure */

#[repr(C, packed)]
pub struct acpi_nfit_memory_map {
    pub header: acpi_nfit_header;
    pub device_handle: u32;
    pub physical_id: u16;
    pub region_id: u16;
    pub range_index: u16;
    pub region_index: u16;
    pub region_size: u64;
    pub region_offset: u64;
    pub address: u64;
    pub interleave_index: u16;
    pub interleave_ways: u16;
    pub flags: u16;
    pub reserved: u16;		/* Reserved, must be zero */
};

/* Flags */

pub const ACPI_NFIT_MEM_SAVE_FAILED: u64 = (1)	/* 00: Last SAVE to Memory Device failed */;
pub const ACPI_NFIT_MEM_RESTORE_FAILED: u64 = (1u64 << 1)	/* 01: Last RESTORE from Memory Device failed */;
pub const ACPI_NFIT_MEM_FLUSH_FAILED: u64 = (1u64 << 2)	/* 02: Platform flush failed */;
pub const ACPI_NFIT_MEM_NOT_ARMED: u64 = (1u64 << 3)	/* 03: Memory Device is not armed */;
pub const ACPI_NFIT_MEM_HEALTH_OBSERVED: u64 = (1u64 << 4)	/* 04: Memory Device observed SMART/health events */;
pub const ACPI_NFIT_MEM_HEALTH_ENABLED: u64 = (1u64 << 5)	/* 05: SMART/health events enabled */;
pub const ACPI_NFIT_MEM_MAP_FAILED: u64 = (1u64 << 6)	/* 06: Mapping to SPA failed */;

/* 2: Interleave Structure */

#[repr(C, packed)]
pub struct acpi_nfit_interleave {
    pub header: acpi_nfit_header;
    pub interleave_index: u16;
    pub reserved: u16;		/* Reserved, must be zero */
    pub line_count: u32;
    pub line_size: u32;
    pub line_offset: [u32; 0];	/* Variable length */
};

/* 3: SMBIOS Management Information Structure */

#[repr(C, packed)]
pub struct acpi_nfit_smbios {
    pub header: acpi_nfit_header;
    pub reserved: u32;		/* Reserved, must be zero */
    pub data: [u8; 0];		/* Variable length */
};

/* 4: NVDIMM Control Region Structure */

#[repr(C, packed)]
pub struct acpi_nfit_control_region {
    pub header: acpi_nfit_header;
    pub region_index: u16;
    pub vendor_id: u16;
    pub device_id: u16;
    pub revision_id: u16;
    pub subsystem_vendor_id: u16;
    pub subsystem_device_id: u16;
    pub subsystem_revision_id: u16;
    pub valid_fields: u8;
    pub manufacturing_location: u8;
    pub manufacturing_date: u16;
    pub reserved: [u8; 2];		/* Reserved, must be zero */
    pub serial_number: u32;
    pub code: u16;
    pub windows: u16;
    pub window_size: u64;
    pub command_offset: u64;
    pub command_size: u64;
    pub status_offset: u64;
    pub status_size: u64;
    pub flags: u16;
    pub reserved1: [u8; 6];	/* Reserved, must be zero */
};

/* Flags */

pub const ACPI_NFIT_CONTROL_BUFFERED: u64 = (1)	/* Block Data Windows implementation is buffered */;

/* valid_fields bits */

pub const ACPI_NFIT_CONTROL_MFG_INFO_VALID: u64 = (1)	/* Manufacturing fields are valid */;

/* 5: NVDIMM Block Data Window Region Structure */

#[repr(C, packed)]
pub struct acpi_nfit_data_region {
    pub header: acpi_nfit_header;
    pub region_index: u16;
    pub windows: u16;
    pub offset: u64;
    pub size: u64;
    pub capacity: u64;
    pub start_address: u64;
};

/* 6: Flush Hint Address Structure */

#[repr(C, packed)]
pub struct acpi_nfit_flush_address {
    pub header: acpi_nfit_header;
    pub device_handle: u32;
    pub hint_count: u16;
    pub reserved: [u8; 6];		/* Reserved, must be zero */
    pub hint_address: [u64; 0];	/* Variable length */
};

/* 7: Platform Capabilities Structure */

#[repr(C, packed)]
pub struct acpi_nfit_capabilities {
    pub header: acpi_nfit_header;
    pub highest_capability: u8;
    pub reserved: [u8; 3];		/* Reserved, must be zero */
    pub capabilities: u32;
    pub reserved2: u32;
};

/* Capabilities Flags */

pub const ACPI_NFIT_CAPABILITY_CACHE_FLUSH: u64 = (1)	/* 00: Cache Flush to NVDIMM capable */;
pub const ACPI_NFIT_CAPABILITY_MEM_FLUSH: u64 = (1u64 << 1)	/* 01: Memory Flush to NVDIMM capable */;
pub const ACPI_NFIT_CAPABILITY_MEM_MIRRORING: u64 = (1u64 << 2)	/* 02: Memory Mirroring capable */;

/*
 * NFIT/DVDIMM device handle support - used as the _ADR for each NVDIMM
 */
#[repr(C, packed)]
pub struct nfit_device_handle {
    pub handle: u32;
};

/* Device handle construction and extraction macros */

pub const ACPI_NFIT_DIMM_NUMBER_MASK: u64 = 0x0000000F;
pub const ACPI_NFIT_CHANNEL_NUMBER_MASK: u64 = 0x000000F0;
pub const ACPI_NFIT_MEMORY_ID_MASK: u64 = 0x00000F00;
pub const ACPI_NFIT_SOCKET_ID_MASK: u64 = 0x0000F000;
pub const ACPI_NFIT_NODE_ID_MASK: u64 = 0x0FFF0000;

pub const ACPI_NFIT_DIMM_NUMBER_OFFSET: u64 = 0;
pub const ACPI_NFIT_CHANNEL_NUMBER_OFFSET: u64 = 4;
pub const ACPI_NFIT_MEMORY_ID_OFFSET: u64 = 8;
pub const ACPI_NFIT_SOCKET_ID_OFFSET: u64 = 12;
pub const ACPI_NFIT_NODE_ID_OFFSET: u64 = 16;

/* Macro to construct a NFIT/NVDIMM device handle */

#define ACPI_NFIT_BUILD_DEVICE_HANDLE(dimm, channel, memory, socket, node) \
	((dimm)                                         | \
	((channel) << ACPI_NFIT_CHANNEL_NUMBER_OFFSET)  | \
	((memory)  << ACPI_NFIT_MEMORY_ID_OFFSET)       | \
	((socket)  << ACPI_NFIT_SOCKET_ID_OFFSET)       | \
	((node)    << ACPI_NFIT_NODE_ID_OFFSET))

/* Macros to extract individual fields from a NFIT/NVDIMM device handle */

#define ACPI_NFIT_GET_DIMM_NUMBER(handle) \
	((handle) & ACPI_NFIT_DIMM_NUMBER_MASK)

#define ACPI_NFIT_GET_CHANNEL_NUMBER(handle) \
	(((handle) & ACPI_NFIT_CHANNEL_NUMBER_MASK) >> ACPI_NFIT_CHANNEL_NUMBER_OFFSET)

#define ACPI_NFIT_GET_MEMORY_ID(handle) \
	(((handle) & ACPI_NFIT_MEMORY_ID_MASK)      >> ACPI_NFIT_MEMORY_ID_OFFSET)

#define ACPI_NFIT_GET_SOCKET_ID(handle) \
	(((handle) & ACPI_NFIT_SOCKET_ID_MASK)      >> ACPI_NFIT_SOCKET_ID_OFFSET)

#define ACPI_NFIT_GET_NODE_ID(handle) \
	(((handle) & ACPI_NFIT_NODE_ID_MASK)        >> ACPI_NFIT_NODE_ID_OFFSET)

/*******************************************************************************
 *
 * NHLT - Non HDAudio Link Table
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_nhlt {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub endpoints_count: u8;
	/*
	 * struct acpi_nhlt_endpoint endpoints[];
	 * struct acpi_nhlt_config oed_config;
	 */
};

#[repr(C, packed)]
pub struct acpi_nhlt_endpoint {
    pub length: u32;
    pub link_type: u8;
    pub instance_id: u8;
    pub vendor_id: u16;
    pub device_id: u16;
    pub revision_id: u16;
    pub subsystem_id: u32;
    pub device_type: u8;
    pub direction: u8;
    pub virtual_bus_id: u8;
	/*
	 * struct acpi_nhlt_config device_config;
	 * struct acpi_nhlt_formats_config formats_config;
	 * struct acpi_nhlt_devices_info devices_info;
	 */
};

/*
 * Values for link_type field above
 *
 * Only types PDM and SSP are used
 */
pub const ACPI_NHLT_LINKTYPE_HDA: u64 = 0;
pub const ACPI_NHLT_LINKTYPE_DSP: u64 = 1;
pub const ACPI_NHLT_LINKTYPE_PDM: u64 = 2;
pub const ACPI_NHLT_LINKTYPE_SSP: u64 = 3;
pub const ACPI_NHLT_LINKTYPE_SLIMBUS: u64 = 4;
pub const ACPI_NHLT_LINKTYPE_SDW: u64 = 5;
pub const ACPI_NHLT_LINKTYPE_UAOL: u64 = 6;

/* Values for device_id field above */

pub const ACPI_NHLT_DEVICEID_DMIC: u64 = 0xAE20;
pub const ACPI_NHLT_DEVICEID_BT: u64 = 0xAE30;
pub const ACPI_NHLT_DEVICEID_I2S: u64 = 0xAE34;

/* Values for device_type field above */

/*
 * Device types unique to endpoint of link_type=PDM
 *
 * Type PDM used for all SKL+ platforms
 */
pub const ACPI_NHLT_DEVICETYPE_PDM: u64 = 0;
pub const ACPI_NHLT_DEVICETYPE_PDM_SKL: u64 = 1;
/* Device types unique to endpoint of link_type=SSP */
pub const ACPI_NHLT_DEVICETYPE_BT: u64 = 0;
pub const ACPI_NHLT_DEVICETYPE_FM: u64 = 1;
pub const ACPI_NHLT_DEVICETYPE_MODEM: u64 = 2;
pub const ACPI_NHLT_DEVICETYPE_CODEC: u64 = 4;

/* Values for Direction field above */

pub const ACPI_NHLT_DIR_RENDER: u64 = 0;
pub const ACPI_NHLT_DIR_CAPTURE: u64 = 1;

#[repr(C, packed)]
pub struct acpi_nhlt_config {
    pub capabilities_size: u32;
    pub capabilities: [u8; 0];
};

#[repr(C, packed)]
pub struct acpi_nhlt_gendevice_config {
    pub virtual_slot: u8;
    pub config_type: u8;
};

/* Values for config_type field above */

pub const ACPI_NHLT_CONFIGTYPE_GENERIC: u64 = 0;
pub const ACPI_NHLT_CONFIGTYPE_MICARRAY: u64 = 1;

#[repr(C, packed)]
pub struct acpi_nhlt_micdevice_config {
    pub virtual_slot: u8;
    pub config_type: u8;
    pub array_type: u8;
};

/* Values for array_type field above */

pub const ACPI_NHLT_ARRAYTYPE_LINEAR2_SMALL: u64 = 0xA;
pub const ACPI_NHLT_ARRAYTYPE_LINEAR2_BIG: u64 = 0xB;
pub const ACPI_NHLT_ARRAYTYPE_LINEAR4_GEO1: u64 = 0xC;
pub const ACPI_NHLT_ARRAYTYPE_PLANAR4_LSHAPED: u64 = 0xD;
pub const ACPI_NHLT_ARRAYTYPE_LINEAR4_GEO2: u64 = 0xE;
pub const ACPI_NHLT_ARRAYTYPE_VENDOR: u64 = 0xF;

#[repr(C, packed)]
pub struct acpi_nhlt_vendor_mic_config {
    pub type: u8;
    pub panel: u8;
    pub speaker_position_distance: u16;		/* mm */
    pub horizontal_offset: u16;			/* mm */
    pub vertical_offset: u16;			/* mm */
    pub frequency_low_band: u8;			/* 5*Hz */
    pub frequency_high_band: u8;			/* 500*Hz */
    pub direction_angle: u16;			/* -180 - +180 */
    pub elevation_angle: u16;			/* -180 - +180 */
    pub work_vertical_angle_begin: u16;		/* -180 - +180 with 2 deg step */
    pub work_vertical_angle_end: u16;		/* -180 - +180 with 2 deg step */
    pub work_horizontal_angle_begin: u16;	/* -180 - +180 with 2 deg step */
    pub work_horizontal_angle_end: u16;		/* -180 - +180 with 2 deg step */
};

/* Values for Type field above */

pub const ACPI_NHLT_MICTYPE_OMNIDIRECTIONAL: u64 = 0;
pub const ACPI_NHLT_MICTYPE_SUBCARDIOID: u64 = 1;
pub const ACPI_NHLT_MICTYPE_CARDIOID: u64 = 2;
pub const ACPI_NHLT_MICTYPE_SUPERCARDIOID: u64 = 3;
pub const ACPI_NHLT_MICTYPE_HYPERCARDIOID: u64 = 4;
pub const ACPI_NHLT_MICTYPE_8SHAPED: u64 = 5;
pub const ACPI_NHLT_MICTYPE_RESERVED: u64 = 6;
pub const ACPI_NHLT_MICTYPE_VENDORDEFINED: u64 = 7;

/* Values for Panel field above */

pub const ACPI_NHLT_MICLOCATION_TOP: u64 = 0;
pub const ACPI_NHLT_MICLOCATION_BOTTOM: u64 = 1;
pub const ACPI_NHLT_MICLOCATION_LEFT: u64 = 2;
pub const ACPI_NHLT_MICLOCATION_RIGHT: u64 = 3;
pub const ACPI_NHLT_MICLOCATION_FRONT: u64 = 4;
pub const ACPI_NHLT_MICLOCATION_REAR: u64 = 5;

#[repr(C, packed)]
pub struct acpi_nhlt_vendor_micdevice_config {
    pub virtual_slot: u8;
    pub config_type: u8;
    pub array_type: u8;
    pub mics_count: u8;
	struct acpi_nhlt_vendor_mic_config mics[];
};

union acpi_nhlt_device_config {
    pub virtual_slot: u8;
    pub gen: acpi_nhlt_gendevice_config;
    pub mic: acpi_nhlt_micdevice_config;
    pub vendor_mic: acpi_nhlt_vendor_micdevice_config;
};

/* Inherited from Microsoft's WAVEFORMATEXTENSIBLE. */
#[repr(C, packed)]
pub struct acpi_nhlt_wave_formatext {
    pub format_tag: u16;
    pub channel_count: u16;
    pub samples_per_sec: u32;
    pub avg_bytes_per_sec: u32;
    pub block_align: u16;
    pub bits_per_sample: u16;
    pub extra_format_size: u16;
    pub valid_bits_per_sample: u16;
    pub channel_mask: u32;
    pub subformat: [u8; 16];
};

#[repr(C, packed)]
pub struct acpi_nhlt_format_config {
    pub format: acpi_nhlt_wave_formatext;
    pub config: acpi_nhlt_config;
};

#[repr(C, packed)]
pub struct acpi_nhlt_formats_config {
    pub formats_count: u8;
	struct acpi_nhlt_format_config formats[];
};

#[repr(C, packed)]
pub struct acpi_nhlt_device_info {
    pub id: [u8; 16];
    pub instance_id: u8;
    pub port_id: u8;
};

#[repr(C, packed)]
pub struct acpi_nhlt_devices_info {
    pub devices_count: u8;
	struct acpi_nhlt_device_info devices[];
};

/*******************************************************************************
 *
 * PCCT - Platform Communications Channel Table (ACPI 5.0)
 *        Version 2 (ACPI 6.2)
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_pcct {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub flags: u32;
    pub reserved: u64;
};

/* Values for Flags field above */

pub const ACPI_PCCT_DOORBELL: u64 = 1;

/* Values for subtable type in struct acpi_subtable_header */

#[repr(i32)]
pub enum acpi_pcct_type {
	ACPI_PCCT_TYPE_GENERIC_SUBSPACE = 0,
	ACPI_PCCT_TYPE_HW_REDUCED_SUBSPACE = 1,
	ACPI_PCCT_TYPE_HW_REDUCED_SUBSPACE_TYPE2 = 2,	/* ACPI 6.1 */
	ACPI_PCCT_TYPE_EXT_PCC_MASTER_SUBSPACE = 3,	/* ACPI 6.2 */
	ACPI_PCCT_TYPE_EXT_PCC_SLAVE_SUBSPACE = 4,	/* ACPI 6.2 */
	ACPI_PCCT_TYPE_HW_REG_COMM_SUBSPACE = 5,	/* ACPI 6.4 */
	ACPI_PCCT_TYPE_RESERVED = 6	/* 6 and greater are reserved */
};

/*
 * PCCT Subtables, correspond to Type in struct acpi_subtable_header
 */

/* 0: Generic Communications Subspace */

#[repr(C, packed)]
pub struct acpi_pcct_subspace {
    pub header: acpi_subtable_header;
    pub reserved: [u8; 6];
    pub base_address: u64;
    pub length: u64;
    pub doorbell_register: acpi_generic_address;
    pub preserve_mask: u64;
    pub write_mask: u64;
    pub latency: u32;
    pub max_access_rate: u32;
    pub min_turnaround_time: u16;
};

/* 1: HW-reduced Communications Subspace (ACPI 5.1) */

#[repr(C, packed)]
pub struct acpi_pcct_hw_reduced {
    pub header: acpi_subtable_header;
    pub platform_interrupt: u32;
    pub flags: u8;
    pub reserved: u8;
    pub base_address: u64;
    pub length: u64;
    pub doorbell_register: acpi_generic_address;
    pub preserve_mask: u64;
    pub write_mask: u64;
    pub latency: u32;
    pub max_access_rate: u32;
    pub min_turnaround_time: u16;
};

/* 2: HW-reduced Communications Subspace Type 2 (ACPI 6.1) */

#[repr(C, packed)]
pub struct acpi_pcct_hw_reduced_type2 {
    pub header: acpi_subtable_header;
    pub platform_interrupt: u32;
    pub flags: u8;
    pub reserved: u8;
    pub base_address: u64;
    pub length: u64;
    pub doorbell_register: acpi_generic_address;
    pub preserve_mask: u64;
    pub write_mask: u64;
    pub latency: u32;
    pub max_access_rate: u32;
    pub min_turnaround_time: u16;
    pub platform_ack_register: acpi_generic_address;
    pub ack_preserve_mask: u64;
    pub ack_write_mask: u64;
};

/* 3: Extended PCC Master Subspace Type 3 (ACPI 6.2) */

#[repr(C, packed)]
pub struct acpi_pcct_ext_pcc_master {
    pub header: acpi_subtable_header;
    pub platform_interrupt: u32;
    pub flags: u8;
    pub reserved1: u8;
    pub base_address: u64;
    pub length: u32;
    pub doorbell_register: acpi_generic_address;
    pub preserve_mask: u64;
    pub write_mask: u64;
    pub latency: u32;
    pub max_access_rate: u32;
    pub min_turnaround_time: u32;
    pub platform_ack_register: acpi_generic_address;
    pub ack_preserve_mask: u64;
    pub ack_set_mask: u64;
    pub reserved2: u64;
    pub cmd_complete_register: acpi_generic_address;
    pub cmd_complete_mask: u64;
    pub cmd_update_register: acpi_generic_address;
    pub cmd_update_preserve_mask: u64;
    pub cmd_update_set_mask: u64;
    pub error_status_register: acpi_generic_address;
    pub error_status_mask: u64;
};

/* 4: Extended PCC Slave Subspace Type 4 (ACPI 6.2) */

#[repr(C, packed)]
pub struct acpi_pcct_ext_pcc_slave {
    pub header: acpi_subtable_header;
    pub platform_interrupt: u32;
    pub flags: u8;
    pub reserved1: u8;
    pub base_address: u64;
    pub length: u32;
    pub doorbell_register: acpi_generic_address;
    pub preserve_mask: u64;
    pub write_mask: u64;
    pub latency: u32;
    pub max_access_rate: u32;
    pub min_turnaround_time: u32;
    pub platform_ack_register: acpi_generic_address;
    pub ack_preserve_mask: u64;
    pub ack_set_mask: u64;
    pub reserved2: u64;
    pub cmd_complete_register: acpi_generic_address;
    pub cmd_complete_mask: u64;
    pub cmd_update_register: acpi_generic_address;
    pub cmd_update_preserve_mask: u64;
    pub cmd_update_set_mask: u64;
    pub error_status_register: acpi_generic_address;
    pub error_status_mask: u64;
};

/* 5: HW Registers based Communications Subspace */

#[repr(C, packed)]
pub struct acpi_pcct_hw_reg {
    pub header: acpi_subtable_header;
    pub version: u16;
    pub base_address: u64;
    pub length: u64;
    pub doorbell_register: acpi_generic_address;
    pub doorbell_preserve: u64;
    pub doorbell_write: u64;
    pub cmd_complete_register: acpi_generic_address;
    pub cmd_complete_mask: u64;
    pub error_status_register: acpi_generic_address;
    pub error_status_mask: u64;
    pub nominal_latency: u32;
    pub min_turnaround_time: u32;
};

/* Values for doorbell flags above */

pub const ACPI_PCCT_INTERRUPT_POLARITY: u64 = (1);
pub const ACPI_PCCT_INTERRUPT_MODE: u64 = (1u64 << 1);

/*
 * PCC memory structures (not part of the ACPI table)
 */

/* Shared Memory Region */

#[repr(C, packed)]
pub struct acpi_pcct_shared_memory {
    pub signature: u32;
    pub command: u16;
    pub status: u16;
};

/* Extended PCC Subspace Shared Memory Region (ACPI 6.2) */

#[repr(C, packed)]
pub struct acpi_pcct_ext_pcc_shared_memory {
    pub signature: u32;
    pub flags: u32;
    pub length: u32;
    pub command: u32;
};

/*******************************************************************************
 *
 * PDTT - Platform Debug Trigger Table (ACPI 6.2)
 *        Version 0
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_pdtt {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub trigger_count: u8;
    pub reserved: [u8; 3];
    pub array_offset: u32;
};

/*
 * PDTT Communication Channel Identifier Structure.
 * The number of these structures is defined by trigger_count above,
 * starting at array_offset.
 */
#[repr(C, packed)]
pub struct acpi_pdtt_channel {
    pub subchannel_id: u8;
    pub flags: u8;
};

/* Flags for above */

pub const ACPI_PDTT_RUNTIME_TRIGGER: u64 = (1);
pub const ACPI_PDTT_WAIT_COMPLETION: u64 = (1u64 << 1);
pub const ACPI_PDTT_TRIGGER_ORDER: u64 = (1u64 << 2);

/*******************************************************************************
 *
 * PHAT - Platform Health Assessment Table (ACPI 6.4)
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_phat {
    pub header: acpi_table_header;	/* Common ACPI table header */
};

/* Common header for PHAT subtables that follow main table */

#[repr(C, packed)]
pub struct acpi_phat_header {
    pub type: u16;
    pub length: u16;
    pub revision: u8;
};

/* Values for Type field above */

pub const ACPI_PHAT_TYPE_FW_VERSION_DATA: u64 = 0;
pub const ACPI_PHAT_TYPE_FW_HEALTH_DATA: u64 = 1;
pub const ACPI_PHAT_TYPE_RESERVED: u64 = 2	/* 0x02-0xFFFF are reserved */;

/*
 * PHAT subtables, correspond to Type in struct acpi_phat_header
 */

/* 0: Firmware Version Data Record */

#[repr(C, packed)]
pub struct acpi_phat_version_data {
    pub header: acpi_phat_header;
    pub reserved: [u8; 3];
    pub element_count: u32;
};

#[repr(C, packed)]
pub struct acpi_phat_version_element {
    pub guid: [u8; 16];
    pub version_value: u64;
    pub producer_id: u32;
};

/* 1: Firmware Health Data Record */

#[repr(C, packed)]
pub struct acpi_phat_health_data {
    pub header: acpi_phat_header;
    pub reserved: [u8; 2];
    pub health: u8;
    pub device_guid: [u8; 16];
    pub device_specific_offset: u32;	/* Zero if no Device-specific data */
};

/* Values for Health field above */

pub const ACPI_PHAT_ERRORS_FOUND: u64 = 0;
pub const ACPI_PHAT_NO_ERRORS: u64 = 1;
pub const ACPI_PHAT_UNKNOWN_ERRORS: u64 = 2;
pub const ACPI_PHAT_ADVISORY: u64 = 3;

/*******************************************************************************
 *
 * PMTT - Platform Memory Topology Table (ACPI 5.0)
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_pmtt {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub memory_device_count: u32;
	/*
	 * Immediately followed by:
	 * MEMORY_DEVICE memory_device_struct[memory_device_count];
	 */
};

/* Common header for PMTT subtables that follow main table */

#[repr(C, packed)]
pub struct acpi_pmtt_header {
    pub type: u8;
    pub reserved1: u8;
    pub length: u16;
    pub flags: u16;
    pub reserved2: u16;
    pub memory_device_count: u32;	/* Zero means no memory device structs follow */
	/*
	 * Immediately followed by:
	 * u8 type_specific_data[]
	 * MEMORY_DEVICE memory_device_struct[memory_device_count];
	 */
};

/* Values for Type field above */

pub const ACPI_PMTT_TYPE_SOCKET: u64 = 0;
pub const ACPI_PMTT_TYPE_CONTROLLER: u64 = 1;
pub const ACPI_PMTT_TYPE_DIMM: u64 = 2;
pub const ACPI_PMTT_TYPE_RESERVED: u64 = 3	/* 0x03-0xFE are reserved */;
pub const ACPI_PMTT_TYPE_VENDOR: u64 = 0xFF;

/* Values for Flags field above */

pub const ACPI_PMTT_TOP_LEVEL: u64 = 0x0001;
pub const ACPI_PMTT_PHYSICAL: u64 = 0x0002;
pub const ACPI_PMTT_MEMORY_TYPE: u64 = 0x000C;

/*
 * PMTT subtables, correspond to Type in struct acpi_pmtt_header
 */

/* 0: Socket Structure */

#[repr(C, packed)]
pub struct acpi_pmtt_socket {
    pub header: acpi_pmtt_header;
    pub socket_id: u16;
    pub reserved: u16;
};
	/*
	 * Immediately followed by:
	 * MEMORY_DEVICE memory_device_struct[memory_device_count];
	 */

/* 1: Memory Controller subtable */

#[repr(C, packed)]
pub struct acpi_pmtt_controller {
    pub header: acpi_pmtt_header;
    pub controller_id: u16;
    pub reserved: u16;
};
	/*
	 * Immediately followed by:
	 * MEMORY_DEVICE memory_device_struct[memory_device_count];
	 */

/* 2: Physical Component Identifier (DIMM) */

#[repr(C, packed)]
pub struct acpi_pmtt_physical_component {
    pub header: acpi_pmtt_header;
    pub bios_handle: u32;
};

/* 0xFF: Vendor Specific Data */

#[repr(C, packed)]
pub struct acpi_pmtt_vendor_specific {
    pub header: acpi_pmtt_header;
    pub type_uuid: [u8; 16];
    pub specific: [u8; 0];
	/*
	 * Immediately followed by:
	 * u8 vendor_specific_data[];
	 * MEMORY_DEVICE memory_device_struct[memory_device_count];
	 */
};

/*******************************************************************************
 *
 * PPTT - Processor Properties Topology Table (ACPI 6.2)
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_pptt {
    pub header: acpi_table_header;	/* Common ACPI table header */
};

/* Values for Type field above */

#[repr(i32)]
pub enum acpi_pptt_type {
	ACPI_PPTT_TYPE_PROCESSOR = 0,
	ACPI_PPTT_TYPE_CACHE = 1,
	ACPI_PPTT_TYPE_ID = 2,
	ACPI_PPTT_TYPE_RESERVED = 3
};

/* 0: Processor Hierarchy Node Structure */

#[repr(C, packed)]
pub struct acpi_pptt_processor {
    pub header: acpi_subtable_header;
    pub reserved: u16;
    pub flags: u32;
    pub parent: u32;
    pub acpi_processor_id: u32;
    pub number_of_priv_resources: u32;
};

/* Flags */

pub const ACPI_PPTT_PHYSICAL_PACKAGE: u64 = (1);
pub const ACPI_PPTT_ACPI_PROCESSOR_ID_VALID: u64 = (1u64 << 1);
pub const ACPI_PPTT_ACPI_PROCESSOR_IS_THREAD: u64 = (1u64 << 2)	/* ACPI 6.3 */;
pub const ACPI_PPTT_ACPI_LEAF_NODE: u64 = (1u64 << 3)	/* ACPI 6.3 */;
pub const ACPI_PPTT_ACPI_IDENTICAL: u64 = (1u64 << 4)	/* ACPI 6.3 */;

/* 1: Cache Type Structure */

#[repr(C, packed)]
pub struct acpi_pptt_cache {
    pub header: acpi_subtable_header;
    pub reserved: u16;
    pub flags: u32;
    pub next_level_of_cache: u32;
    pub size: u32;
    pub number_of_sets: u32;
    pub associativity: u8;
    pub attributes: u8;
    pub line_size: u16;
};

/* 1: Cache Type Structure for PPTT version 3 */

#[repr(C, packed)]
pub struct acpi_pptt_cache_v1 {
    pub header: acpi_subtable_header;
    pub reserved: u16;
    pub flags: u32;
    pub next_level_of_cache: u32;
    pub size: u32;
    pub number_of_sets: u32;
    pub associativity: u8;
    pub attributes: u8;
    pub line_size: u16;
    pub cache_id: u32;
};

/* Flags */

pub const ACPI_PPTT_SIZE_PROPERTY_VALID: u64 = (1)	/* Physical property valid */;
pub const ACPI_PPTT_NUMBER_OF_SETS_VALID: u64 = (1u64 << 1)	/* Number of sets valid */;
pub const ACPI_PPTT_ASSOCIATIVITY_VALID: u64 = (1u64 << 2)	/* Associativity valid */;
pub const ACPI_PPTT_ALLOCATION_TYPE_VALID: u64 = (1u64 << 3)	/* Allocation type valid */;
pub const ACPI_PPTT_CACHE_TYPE_VALID: u64 = (1u64 << 4)	/* Cache type valid */;
pub const ACPI_PPTT_WRITE_POLICY_VALID: u64 = (1u64 << 5)	/* Write policy valid */;
pub const ACPI_PPTT_LINE_SIZE_VALID: u64 = (1u64 << 6)	/* Line size valid */;
pub const ACPI_PPTT_CACHE_ID_VALID: u64 = (1u64 << 7)	/* Cache ID valid */;

/* Masks for Attributes */

pub const ACPI_PPTT_MASK_ALLOCATION_TYPE: u64 = (0x03)	/* Allocation type */;
pub const ACPI_PPTT_MASK_CACHE_TYPE: u64 = (0x0C)	/* Cache type */;
pub const ACPI_PPTT_MASK_WRITE_POLICY: u64 = (0x10)	/* Write policy */;

/* Attributes describing cache */
pub const ACPI_PPTT_CACHE_READ_ALLOCATE: u64 = (0x0)	/* Cache line is allocated on read */;
pub const ACPI_PPTT_CACHE_WRITE_ALLOCATE: u64 = (0x01)	/* Cache line is allocated on write */;
pub const ACPI_PPTT_CACHE_RW_ALLOCATE: u64 = (0x02)	/* Cache line is allocated on read and write */;
pub const ACPI_PPTT_CACHE_RW_ALLOCATE_ALT: u64 = (0x03)	/* Alternate representation of above */;

pub const ACPI_PPTT_CACHE_TYPE_DATA: u64 = (0x0)	/* Data cache */;
pub const ACPI_PPTT_CACHE_TYPE_INSTR: u64 = (1u64 << 2)	/* Instruction cache */;
pub const ACPI_PPTT_CACHE_TYPE_UNIFIED: u64 = (2<<2)	/* Unified I & D cache */;
pub const ACPI_PPTT_CACHE_TYPE_UNIFIED_ALT: u64 = (3<<2)	/* Alternate representation of above */;

pub const ACPI_PPTT_CACHE_POLICY_WB: u64 = (0x0)	/* Cache is write back */;
pub const ACPI_PPTT_CACHE_POLICY_WT: u64 = (1u64 << 4)	/* Cache is write through */;

/* 2: ID Structure */

#[repr(C, packed)]
pub struct acpi_pptt_id {
    pub header: acpi_subtable_header;
    pub reserved: u16;
    pub vendor_id: u32;
    pub level1_id: u64;
    pub level2_id: u64;
    pub major_rev: u16;
    pub minor_rev: u16;
    pub spin_rev: u16;
};

/*******************************************************************************
 *
 * PRMT - Platform Runtime Mechanism Table
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_prmt {
    pub header: acpi_table_header;	/* Common ACPI table header */
};

#[repr(C, packed)]
pub struct acpi_table_prmt_header {
    pub platform_guid: [u8; 16];
    pub module_info_offset: u32;
    pub module_info_count: u32;
};

#[repr(C, packed)]
pub struct acpi_prmt_module_header {
    pub revision: u16;
    pub length: u16;
};

#[repr(C, packed)]
pub struct acpi_prmt_module_info {
    pub revision: u16;
    pub length: u16;
    pub module_guid: [u8; 16];
    pub major_rev: u16;
    pub minor_rev: u16;
    pub handler_info_count: u16;
    pub handler_info_offset: u32;
    pub mmio_list_pointer: u64;
};

#[repr(C, packed)]
pub struct acpi_prmt_handler_info {
    pub revision: u16;
    pub length: u16;
    pub handler_guid: [u8; 16];
    pub handler_address: u64;
    pub static_data_buffer_address: u64;
    pub acpi_param_buffer_address: u64;
};

/*******************************************************************************
 *
 * RASF - RAS Feature Table (ACPI 5.0)
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_rasf {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub channel_id: [u8; 12];
};

/* RASF Platform Communication Channel Shared Memory Region */

#[repr(C, packed)]
pub struct acpi_rasf_shared_memory {
    pub signature: u32;
    pub command: u16;
    pub status: u16;
    pub version: u16;
    pub capabilities: [u8; 16];
    pub set_capabilities: [u8; 16];
    pub num_parameter_blocks: u16;
    pub set_capabilities_status: u32;
};

/* RASF Parameter Block Structure Header */

#[repr(C, packed)]
pub struct acpi_rasf_parameter_block {
    pub type: u16;
    pub version: u16;
    pub length: u16;
};

/* RASF Parameter Block Structure for PATROL_SCRUB */

#[repr(C, packed)]
pub struct acpi_rasf_patrol_scrub_parameter {
    pub header: acpi_rasf_parameter_block;
    pub patrol_scrub_command: u16;
    pub requested_address_range: [u64; 2];
    pub actual_address_range: [u64; 2];
    pub flags: u16;
    pub requested_speed: u8;
};

/* Masks for Flags and Speed fields above */

pub const ACPI_RASF_SCRUBBER_RUNNING: u64 = 1;
pub const ACPI_RASF_SPEED: u64 = (7<<1);
pub const ACPI_RASF_SPEED_SLOW: u64 = (0<<1);
pub const ACPI_RASF_SPEED_MEDIUM: u64 = (4<<1);
pub const ACPI_RASF_SPEED_FAST: u64 = (7<<1);

/* Channel Commands */

#[repr(i32)]
pub enum acpi_rasf_commands {
	ACPI_RASF_EXECUTE_RASF_COMMAND = 1
};

/* Platform RAS Capabilities */

#[repr(i32)]
pub enum acpi_rasf_capabiliities {
	ACPI_HW_PATROL_SCRUB_SUPPORTED = 0,
	ACPI_SW_PATROL_SCRUB_EXPOSED = 1
};

/* Patrol Scrub Commands */

#[repr(i32)]
pub enum acpi_rasf_patrol_scrub_commands {
	ACPI_RASF_GET_PATROL_PARAMETERS = 1,
	ACPI_RASF_START_PATROL_SCRUBBER = 2,
	ACPI_RASF_STOP_PATROL_SCRUBBER = 3
};

/* Channel Command flags */

pub const ACPI_RASF_GENERATE_SCI: u64 = (1u64 << 15);

/* Status values */

#[repr(i32)]
pub enum acpi_rasf_status {
	ACPI_RASF_SUCCESS = 0,
	ACPI_RASF_NOT_VALID = 1,
	ACPI_RASF_NOT_SUPPORTED = 2,
	ACPI_RASF_BUSY = 3,
	ACPI_RASF_FAILED = 4,
	ACPI_RASF_ABORTED = 5,
	ACPI_RASF_INVALID_DATA = 6
};

/* Status flags */

pub const ACPI_RASF_COMMAND_COMPLETE: u64 = (1);
pub const ACPI_RASF_SCI_DOORBELL: u64 = (1u64 << 1);
pub const ACPI_RASF_ERROR: u64 = (1u64 << 2);
pub const ACPI_RASF_STATUS: u64 = (0x1F<<3);

/*******************************************************************************
 *
 * RAS2 - RAS2 Feature Table (ACPI 6.5)
 *        Version 1
 *
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_ras2 {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub reserved: u16;
    pub num_pcc_descs: u16;
};

/* RAS2 Platform Communication Channel Descriptor */

#[repr(C, packed)]
pub struct acpi_ras2_pcc_desc {
    pub channel_id: u8;
    pub reserved: u16;
    pub feature_type: u8;
    pub instance: u32;
};

/* RAS2 Platform Communication Channel Shared Memory Region */

#[repr(C, packed)]
pub struct acpi_ras2_shmem {
    pub signature: u32;
    pub command: u16;
    pub status: u16;
    pub version: u16;
    pub features: [u8; 16];
    pub set_caps: [u8; 16];
    pub num_param_blks: u16;
    pub set_caps_status: u32;
};

/* RAS2 Parameter Block Structure for PATROL_SCRUB */

#[repr(C, packed)]
pub struct acpi_ras2_parameter_block {
    pub type: u16;
    pub version: u16;
    pub length: u16;
};

/* RAS2 Parameter Block Structure for PATROL_SCRUB */

#[repr(C, packed)]
pub struct acpi_ras2_patrol_scrub_param {
    pub header: acpi_ras2_parameter_block;
    pub command: u16;
    pub req_addr_range: [u64; 2];
    pub actl_addr_range: [u64; 2];
    pub flags: u32;
    pub scrub_params_out: u32;
    pub scrub_params_in: u32;
    pub ext_scrub_params: u32;
    pub scrub_rate_desc: [u8; 256];
};

/* Masks for Flags field above */

pub const ACPI_RAS2_SCRUBBER_RUNNING: u64 = 1;

/* RAS2 Parameter Block Structure for LA2PA_TRANSLATION */

#[repr(C, packed)]
pub struct acpi_ras2_la2pa_translation_parameter {
    pub header: acpi_ras2_parameter_block;
    pub addr_translation_command: u16;
    pub sub_inst_id: u64;
    pub logical_address: u64;
    pub physical_address: u64;
    pub status: u32;
};

/* Channel Commands */

#[repr(i32)]
pub enum acpi_ras2_commands {
	ACPI_RAS2_EXECUTE_RAS2_COMMAND = 1
};

/* Platform RAS2 Features */

#[repr(i32)]
pub enum acpi_ras2_features {
	ACPI_RAS2_PATROL_SCRUB_SUPPORTED = 0,
	ACPI_RAS2_LA2PA_TRANSLATION = 1
};

/* RAS2 Patrol Scrub Commands */

#[repr(i32)]
pub enum acpi_ras2_patrol_scrub_commands {
	ACPI_RAS2_GET_PATROL_PARAMETERS = 1,
	ACPI_RAS2_START_PATROL_SCRUBBER = 2,
	ACPI_RAS2_STOP_PATROL_SCRUBBER = 3
};

/* RAS2 LA2PA Translation Commands */

#[repr(i32)]
pub enum acpi_ras2_la2_pa_translation_commands {
	ACPI_RAS2_GET_LA2PA_TRANSLATION = 1,
};

/* RAS2 LA2PA Translation Status values */

#[repr(i32)]
pub enum acpi_ras2_la2_pa_translation_status {
	ACPI_RAS2_LA2PA_TRANSLATION_SUCCESS = 0,
	ACPI_RAS2_LA2PA_TRANSLATION_FAIL = 1,
};

/* Channel Command flags */

pub const ACPI_RAS2_GENERATE_SCI: u64 = (1u64 << 15);

/* Status values */

#[repr(i32)]
pub enum acpi_ras2_status {
	ACPI_RAS2_SUCCESS = 0,
	ACPI_RAS2_NOT_VALID = 1,
	ACPI_RAS2_NOT_SUPPORTED = 2,
	ACPI_RAS2_BUSY = 3,
	ACPI_RAS2_FAILED = 4,
	ACPI_RAS2_ABORTED = 5,
	ACPI_RAS2_INVALID_DATA = 6
};

/* Status flags */

pub const ACPI_RAS2_COMMAND_COMPLETE: u64 = (1);
pub const ACPI_RAS2_SCI_DOORBELL: u64 = (1u64 << 1);
pub const ACPI_RAS2_ERROR: u64 = (1u64 << 2);
pub const ACPI_RAS2_STATUS: u64 = (0x1F<<3);

/*******************************************************************************
 *
 * RGRT - Regulatory Graphics Resource Table
 *        Version 1
 *
 * Conforms to "ACPI RGRT" available at:
 * https://microsoft.github.io/mu/dyn/mu_plus/ms_core_pkg/acpi_RGRT/feature_acpi_rgrt/
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_rgrt {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub version: u16;
    pub image_type: u8;
    pub reserved: u8;
    pub image: [u8; 0];
};

/* image_type values */

#[repr(i32)]
pub enum acpi_rgrt_image_type {
	ACPI_RGRT_TYPE_RESERVED0 = 0,
	ACPI_RGRT_IMAGE_TYPE_PNG = 1,
	ACPI_RGRT_TYPE_RESERVED = 2	/* 2 and greater are reserved */
};

/*******************************************************************************
 *
 * RHCT - RISC-V Hart Capabilities Table
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_rhct {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub flags: u32;		/* RHCT flags */
    pub time_base_freq: u64;
    pub node_count: u32;
    pub node_offset: u32;
};

/* RHCT Flags */

pub const ACPI_RHCT_TIMER_CANNOT_WAKEUP_CPU: u64 = (1);
/*
 * RHCT subtables
 */
#[repr(C, packed)]
pub struct acpi_rhct_node_header {
    pub type: u16;
    pub length: u16;
    pub revision: u16;
};

/* Values for RHCT subtable Type above */

#[repr(i32)]
pub enum acpi_rhct_node_type {
	ACPI_RHCT_NODE_TYPE_ISA_STRING = 0x0000,
	ACPI_RHCT_NODE_TYPE_CMO = 0x0001,
	ACPI_RHCT_NODE_TYPE_MMU = 0x0002,
	ACPI_RHCT_NODE_TYPE_RESERVED = 0x0003,
	ACPI_RHCT_NODE_TYPE_HART_INFO = 0xFFFF,
};

/*
 * RHCT node specific subtables
 */

/* ISA string node structure */
#[repr(C, packed)]
pub struct acpi_rhct_isa_string {
    pub isa_length: u16;
    pub isa: [i8; 0];
};

#[repr(C, packed)]
pub struct acpi_rhct_cmo_node {
    pub reserved: u8;		/* Must be zero */
    pub cbom_size: u8;		/* CBOM size in powerof 2 */
    pub cbop_size: u8;		/* CBOP size in powerof 2 */
    pub cboz_size: u8;		/* CBOZ size in powerof 2 */
};

#[repr(C, packed)]
pub struct acpi_rhct_mmu_node {
    pub reserved: u8;		/* Must be zero */
    pub mmu_type: u8;		/* Virtual Address Scheme */
};

#[repr(i32)]
pub enum acpi_rhct_mmu_type {
	ACPI_RHCT_MMU_TYPE_SV39 = 0,
	ACPI_RHCT_MMU_TYPE_SV48 = 1,
	ACPI_RHCT_MMU_TYPE_SV57 = 2
};

/* Hart Info node structure */
#[repr(C, packed)]
pub struct acpi_rhct_hart_info {
    pub num_offsets: u16;
    pub uid: u32;		/* ACPI processor UID */
};

/*******************************************************************************
 *
 * RIMT - RISC-V IO Remapping Table
 *
 * https://github.com/riscv-non-isa/riscv-acpi-rimt
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_rimt {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub num_nodes: u32;		/* Number of RIMT Nodes */
    pub node_offset: u32;	/* Offset to RIMT Node Array */
    pub reserved: u32;
};

#[repr(C, packed)]
pub struct acpi_rimt_node {
    pub type: u8;
    pub revision: u8;
    pub length: u16;
    pub reserved: u16;
    pub id: u16;
    pub node_data: [i8; 0];
};

#[repr(i32)]
pub enum acpi_rimt_node_type {
	ACPI_RIMT_NODE_TYPE_IOMMU = 0x0,
	ACPI_RIMT_NODE_TYPE_PCIE_ROOT_COMPLEX = 0x1,
	ACPI_RIMT_NODE_TYPE_PLAT_DEVICE = 0x2,
};

#[repr(C, packed)]
pub struct acpi_rimt_iommu {
    pub hardware_id: [u8; 8];	/* Hardware ID */
    pub base_address: u64;	/* Base Address */
    pub flags: u32;		/* Flags */
    pub proximity_domain: u32;	/* Proximity Domain */
    pub pcie_segment_number: u16;	/* PCIe Segment number */
    pub pcie_bdf: u16;		/* PCIe B/D/F */
    pub num_interrupt_wires: u16;	/* Number of interrupt wires */
    pub interrupt_wire_offset: u16;	/* Interrupt wire array offset */
    pub interrupt_wire: [u64; 0];	/* Interrupt wire array */
};

/* IOMMU Node Flags */
pub const ACPI_RIMT_IOMMU_FLAGS_PCIE: u64 = (1);
pub const ACPI_RIMT_IOMMU_FLAGS_PXM_VALID: u64 = (1 << 1);

/* Interrupt Wire Structure */
#[repr(C, packed)]
pub struct acpi_rimt_iommu_wire_gsi {
    pub irq_num: u32;		/* Interrupt Number */
    pub flags: u32;		/* Flags */
};

/* Interrupt Wire Flags */
pub const ACPI_RIMT_GSI_LEVEL_TRIGGERRED: u64 = (1);
pub const ACPI_RIMT_GSI_ACTIVE_HIGH: u64 = (1 << 1);

#[repr(C, packed)]
pub struct acpi_rimt_id_mapping {
    pub source_id_base: u32;	/* Source ID Base */
    pub num_ids: u32;		/* Number of IDs */
    pub dest_id_base: u32;	/* Destination Device ID Base */
    pub dest_offset: u32;	/* Destination IOMMU Offset */
    pub flags: u32;		/* Flags */
};

#[repr(C, packed)]
pub struct acpi_rimt_pcie_rc {
    pub flags: u32;		/* Flags */
    pub reserved: u16;		/* Reserved */
    pub pcie_segment_number: u16;	/* PCIe Segment number */
    pub id_mapping_offset: u16;	/* ID mapping array offset */
    pub num_id_mappings: u16;	/* Number of ID mappings */
};

/* PCIe Root Complex Node Flags */
pub const ACPI_RIMT_PCIE_ATS_SUPPORTED: u64 = (1);
pub const ACPI_RIMT_PCIE_PRI_SUPPORTED: u64 = (1 << 1);

#[repr(C, packed)]
pub struct acpi_rimt_platform_device {
    pub id_mapping_offset: u16;	/* ID Mapping array offset */
    pub num_id_mappings: u16;	/* Number of ID mappings */
    pub device_name: [i8; 0];	/* Device Object Name */
};

/*******************************************************************************
 *
 * SBST - Smart Battery Specification Table
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_sbst {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub warning_level: u32;
    pub low_level: u32;
    pub critical_level: u32;
};

/*******************************************************************************
 *
 * SDEI - Software Delegated Exception Interface Descriptor Table
 *
 * Conforms to "Software Delegated Exception Interface (SDEI)" ARM DEN0054A,
 * May 8th, 2017. Copyright 2017 ARM Ltd.
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_sdei {
    pub header: acpi_table_header;	/* Common ACPI table header */
};

/*******************************************************************************
 *
 * SDEV - Secure Devices Table (ACPI 6.2)
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_sdev {
    pub header: acpi_table_header;	/* Common ACPI table header */
};

#[repr(C, packed)]
pub struct acpi_sdev_header {
    pub type: u8;
    pub flags: u8;
    pub length: u16;
};

/* Values for subtable type above */

#[repr(i32)]
pub enum acpi_sdev_type {
	ACPI_SDEV_TYPE_NAMESPACE_DEVICE = 0,
	ACPI_SDEV_TYPE_PCIE_ENDPOINT_DEVICE = 1,
	ACPI_SDEV_TYPE_RESERVED = 2	/* 2 and greater are reserved */
};

/* Values for flags above */

pub const ACPI_SDEV_HANDOFF_TO_UNSECURE_OS: u64 = (1);
pub const ACPI_SDEV_SECURE_COMPONENTS_PRESENT: u64 = (1u64 << 1);

/*
 * SDEV subtables
 */

/* 0: Namespace Device Based Secure Device Structure */

#[repr(C, packed)]
pub struct acpi_sdev_namespace {
    pub header: acpi_sdev_header;
    pub device_id_offset: u16;
    pub device_id_length: u16;
    pub vendor_data_offset: u16;
    pub vendor_data_length: u16;
};

#[repr(C, packed)]
pub struct acpi_sdev_secure_component {
    pub secure_component_offset: u16;
    pub secure_component_length: u16;
};

/*
 * SDEV sub-subtables ("Components") for above
 */
#[repr(C, packed)]
pub struct acpi_sdev_component {
    pub header: acpi_sdev_header;
};

/* Values for sub-subtable type above */

#[repr(i32)]
pub enum acpi_sac_type {
	ACPI_SDEV_TYPE_ID_COMPONENT = 0,
	ACPI_SDEV_TYPE_MEM_COMPONENT = 1
};

#[repr(C, packed)]
pub struct acpi_sdev_id_component {
    pub header: acpi_sdev_header;
    pub hardware_id_offset: u16;
    pub hardware_id_length: u16;
    pub subsystem_id_offset: u16;
    pub subsystem_id_length: u16;
    pub hardware_revision: u16;
    pub hardware_rev_present: u8;
    pub class_code_present: u8;
    pub pci_base_class: u8;
    pub pci_sub_class: u8;
    pub pci_programming_xface: u8;
};

#[repr(C, packed)]
pub struct acpi_sdev_mem_component {
    pub header: acpi_sdev_header;
    pub reserved: u32;
    pub memory_base_address: u64;
    pub memory_length: u64;
};

/* 1: PCIe Endpoint Device Based Device Structure */

#[repr(C, packed)]
pub struct acpi_sdev_pcie {
    pub header: acpi_sdev_header;
    pub segment: u16;
    pub start_bus: u16;
    pub path_offset: u16;
    pub path_length: u16;
    pub vendor_data_offset: u16;
    pub vendor_data_length: u16;
};

/* 1a: PCIe Endpoint path entry */

#[repr(C, packed)]
pub struct acpi_sdev_pcie_path {
    pub device: u8;
    pub function: u8;
};

/*******************************************************************************
 *
 * SVKL - Storage Volume Key Location Table (ACPI 6.4)
 *        From: "Guest-Host-Communication Interface (GHCI) for Intel
 *        Trust Domain Extensions (Intel TDX)".
 *        Version 1
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_svkl {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub count: u32;
};

#[repr(C, packed)]
pub struct acpi_svkl_key {
    pub type: u16;
    pub format: u16;
    pub size: u32;
    pub address: u64;
};

#[repr(i32)]
pub enum acpi_svkl_type {
	ACPI_SVKL_TYPE_MAIN_STORAGE = 0,
	ACPI_SVKL_TYPE_RESERVED = 1	/* 1 and greater are reserved */
};

#[repr(i32)]
pub enum acpi_svkl_format {
	ACPI_SVKL_FORMAT_RAW_BINARY = 0,
	ACPI_SVKL_FORMAT_RESERVED = 1	/* 1 and greater are reserved */
};

/*******************************************************************************
 * SWFT - SoundWire File Table
 *
 * Conforms to "Discovery and Configuration (DisCo) Specification for SoundWire"
 * Version 2.1, 2 October 2023
 *
 ******************************************************************************/
#[repr(C, packed)]
pub struct acpi_sw_file {
    pub vendor_id: u16;
    pub file_id: u32;
    pub file_version: u16;
    pub file_length: u32;
    pub data: [u8; 0];
};

#[repr(C, packed)]
pub struct acpi_table_swft {
    pub header: acpi_table_header;
	struct acpi_sw_file files[];
};

/*******************************************************************************
 *
 * TDEL - TD-Event Log
 *        From: "Guest-Host-Communication Interface (GHCI) for Intel
 *        Trust Domain Extensions (Intel TDX)".
 *        September 2020
 *
 ******************************************************************************/

#[repr(C, packed)]
pub struct acpi_table_tdel {
    pub header: acpi_table_header;	/* Common ACPI table header */
    pub reserved: u32;
    pub log_area_minimum_length: u64;
    pub log_area_start_address: u64;
};

/* Reset to default packing */

// C: #pragma pack() reset.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
