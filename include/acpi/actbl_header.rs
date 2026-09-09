/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: actbl.h - Basic ACPI Table Definitions
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* Fundamental ACPI tables. The RSDP and FACS do not use the common ACPI
 * table header. All other ACPI tables use the header. */

pub const ACPI_SIG_DSDT: &str = "DSDT";
pub const ACPI_SIG_FADT: &str = "FACP";
pub const ACPI_SIG_FACS: &str = "FACS";
pub const ACPI_SIG_OSDT: &str = "OSDT";
pub const ACPI_SIG_PSDT: &str = "PSDT";
pub const ACPI_SIG_RSDP: &str = "RSD PTR ";
pub const ACPI_SIG_RSDT: &str = "RSDT";
pub const ACPI_SIG_XSDT: &str = "XSDT";
pub const ACPI_SIG_SSDT: &str = "SSDT";
pub const ACPI_RSDP_NAME: &str = "RSDP";
pub const ACPI_OEM_NAME: &str = "OEM";

/* All table and structure layouts are byte-packed to match ACPI tables. */
#[repr(C, packed)]
pub struct acpi_table_header {
    pub signature: [core::ffi::c_char; ACPI_NAMESEG_SIZE],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [core::ffi::c_char; ACPI_OEM_ID_SIZE],
    pub oem_table_id: [core::ffi::c_char; ACPI_OEM_TABLE_ID_SIZE],
    pub oem_revision: u32,
    pub asl_compiler_id: [core::ffi::c_char; ACPI_NAMESEG_SIZE],
    pub asl_compiler_revision: u32,
}

#[repr(C, packed)]
pub struct acpi_generic_address {
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_width: u8,
    pub address: u64,
}

#[repr(C, packed)]
pub struct acpi_table_rsdp {
    pub signature: [core::ffi::c_char; 8], pub checksum: u8,
    pub oem_id: [core::ffi::c_char; ACPI_OEM_ID_SIZE], pub revision: u8,
    pub rsdt_physical_address: u32, pub length: u32,
    pub xsdt_physical_address: u64, pub extended_checksum: u8, pub reserved: [u8; 3],
}

#[repr(C, packed)]
pub struct acpi_rsdp_common {
    pub signature: [core::ffi::c_char; 8], pub checksum: u8,
    pub oem_id: [core::ffi::c_char; ACPI_OEM_ID_SIZE], pub revision: u8,
    pub rsdt_physical_address: u32,
}

#[repr(C, packed)]
pub struct acpi_rsdp_extension {
    pub length: u32, pub xsdt_physical_address: u64, pub extended_checksum: u8, pub reserved: [u8; 3],
}

#[repr(C, packed)]
pub struct acpi_table_rsdt { pub header: acpi_table_header, pub table_offset_entry: [u32; 1] }
#[repr(C, packed)]
pub struct acpi_table_xsdt { pub header: acpi_table_header, pub table_offset_entry: [u64; 1] }
pub const ACPI_RSDT_ENTRY_SIZE: usize = core::mem::size_of::<u32>();
pub const ACPI_XSDT_ENTRY_SIZE: usize = core::mem::size_of::<u64>();

#[repr(C, packed)]
pub struct acpi_table_facs {
    pub signature: [core::ffi::c_char; 4], pub length: u32, pub hardware_signature: u32,
    pub firmware_waking_vector: u32, pub global_lock: u32, pub flags: u32,
    pub xfirmware_waking_vector: u64, pub version: u8, pub reserved: [u8; 3],
    pub ospm_flags: u32, pub reserved1: [u8; 24],
}

pub const ACPI_GLOCK_PENDING: u32 = 1;
pub const ACPI_GLOCK_OWNED: u32 = 1 << 1;
pub const ACPI_FACS_S4_BIOS_PRESENT: u32 = 1;
pub const ACPI_FACS_64BIT_WAKE: u32 = 1 << 1;
pub const ACPI_FACS_64BIT_ENVIRONMENT: u32 = 1;

#[repr(C, packed)]
pub struct acpi_table_fadt {
    pub header: acpi_table_header, pub facs: u32, pub dsdt: u32, pub model: u8,
    pub preferred_profile: u8, pub sci_interrupt: u16, pub smi_command: u32,
    pub acpi_enable: u8, pub acpi_disable: u8, pub s4_bios_request: u8, pub pstate_control: u8,
    pub pm1a_event_block: u32, pub pm1b_event_block: u32, pub pm1a_control_block: u32,
    pub pm1b_control_block: u32, pub pm2_control_block: u32, pub pm_timer_block: u32,
    pub gpe0_block: u32, pub gpe1_block: u32, pub pm1_event_length: u8, pub pm1_control_length: u8,
    pub pm2_control_length: u8, pub pm_timer_length: u8, pub gpe0_block_length: u8,
    pub gpe1_block_length: u8, pub gpe1_base: u8, pub cst_control: u8, pub c2_latency: u16,
    pub c3_latency: u16, pub flush_size: u16, pub flush_stride: u16, pub duty_offset: u8,
    pub duty_width: u8, pub day_alarm: u8, pub month_alarm: u8, pub century: u8,
    pub boot_flags: u16, pub reserved: u8, pub flags: u32, pub reset_register: acpi_generic_address,
    pub reset_value: u8, pub arm_boot_flags: u16, pub minor_revision: u8, pub Xfacs: u64, pub Xdsdt: u64,
    pub xpm1a_event_block: acpi_generic_address, pub xpm1b_event_block: acpi_generic_address,
    pub xpm1a_control_block: acpi_generic_address, pub xpm1b_control_block: acpi_generic_address,
    pub xpm2_control_block: acpi_generic_address, pub xpm_timer_block: acpi_generic_address,
    pub xgpe0_block: acpi_generic_address, pub xgpe1_block: acpi_generic_address,
    pub sleep_control: acpi_generic_address, pub sleep_status: acpi_generic_address, pub hypervisor_id: u64,
}

pub const ACPI_FADT_LEGACY_DEVICES: u32 = 1; pub const ACPI_FADT_8042: u32 = 1 << 1;
pub const ACPI_FADT_NO_VGA: u32 = 1 << 2; pub const ACPI_FADT_NO_MSI: u32 = 1 << 3;
pub const ACPI_FADT_NO_ASPM: u32 = 1 << 4; pub const ACPI_FADT_NO_CMOS_RTC: u32 = 1 << 5;
pub const FADT2_REVISION_ID: u32 = 3;
pub const ACPI_FADT_PSCI_COMPLIANT: u32 = 1; pub const ACPI_FADT_PSCI_USE_HVC: u32 = 1 << 1;
pub const ACPI_FADT_WBINVD: u32 = 1; pub const ACPI_FADT_WBINVD_FLUSH: u32 = 1 << 1;
pub const ACPI_FADT_C1_SUPPORTED: u32 = 1 << 2; pub const ACPI_FADT_C2_MP_SUPPORTED: u32 = 1 << 3;
pub const ACPI_FADT_POWER_BUTTON: u32 = 1 << 4; pub const ACPI_FADT_SLEEP_BUTTON: u32 = 1 << 5;
pub const ACPI_FADT_FIXED_RTC: u32 = 1 << 6; pub const ACPI_FADT_S4_RTC_WAKE: u32 = 1 << 7;
pub const ACPI_FADT_32BIT_TIMER: u32 = 1 << 8; pub const ACPI_FADT_DOCKING_SUPPORTED: u32 = 1 << 9;
pub const ACPI_FADT_RESET_REGISTER: u32 = 1 << 10; pub const ACPI_FADT_SEALED_CASE: u32 = 1 << 11;
pub const ACPI_FADT_HEADLESS: u32 = 1 << 12; pub const ACPI_FADT_SLEEP_TYPE: u32 = 1 << 13;
pub const ACPI_FADT_PCI_EXPRESS_WAKE: u32 = 1 << 14; pub const ACPI_FADT_PLATFORM_CLOCK: u32 = 1 << 15;
pub const ACPI_FADT_S4_RTC_VALID: u32 = 1 << 16; pub const ACPI_FADT_REMOTE_POWER_ON: u32 = 1 << 17;
pub const ACPI_FADT_APIC_CLUSTER: u32 = 1 << 18; pub const ACPI_FADT_APIC_PHYSICAL: u32 = 1 << 19;
pub const ACPI_FADT_HW_REDUCED: u32 = 1 << 20; pub const ACPI_FADT_LOW_POWER_S0: u32 = 1 << 21;

#[repr(u32)]
pub enum acpi_preferred_pm_profiles { PM_UNSPECIFIED = 0, PM_DESKTOP = 1, PM_MOBILE = 2, PM_WORKSTATION = 3, PM_ENTERPRISE_SERVER = 4, PM_SOHO_SERVER = 5, PM_APPLIANCE_PC = 6, PM_PERFORMANCE_SERVER = 7, PM_TABLET = 8, NR_PM_PROFILES = 9 }
pub const ACPI_X_WAKE_STATUS: u32 = 0x80;
pub const ACPI_X_SLEEP_TYPE_MASK: u32 = 0x1C;
pub const ACPI_X_SLEEP_TYPE_POSITION: u32 = 0x02;
pub const ACPI_X_SLEEP_ENABLE: u32 = 0x20;

#[repr(C)]
pub union acpi_name_union { pub integer: u32, pub ascii: [core::ffi::c_char; 4] }

#[repr(C)]
pub struct acpi_table_desc {
    pub address: acpi_physical_address, pub pointer: *mut acpi_table_header, pub length: u32,
    pub signature: acpi_name_union, pub owner_id: acpi_owner_id, pub flags: u8, pub validation_count: u16,
}

pub const ACPI_MAX_TABLE_VALIDATIONS: u16 = ACPI_UINT16_MAX;
pub const ACPI_TABLE_ORIGIN_EXTERNAL_VIRTUAL: u8 = 0;
pub const ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL: u8 = 1;
pub const ACPI_TABLE_ORIGIN_INTERNAL_VIRTUAL: u8 = 2;
pub const ACPI_TABLE_ORIGIN_MASK: u8 = 3;
pub const ACPI_TABLE_IS_VERIFIED: u8 = 4;
pub const ACPI_TABLE_IS_LOADED: u8 = 8;

/* actbl1.h, actbl2.h, and actbl3.h provide additional table declarations. */

#[macro_export]
macro_rules! ACPI_FADT_OFFSET { ($field:tt) => { core::mem::offset_of!($crate::acpi_table_fadt, $field) as u16 }; }
pub const ACPI_FADT_V1_SIZE: u32 = (core::mem::offset_of!(acpi_table_fadt, flags) + 4) as u32;
pub const ACPI_FADT_V2_SIZE: u32 = (core::mem::offset_of!(acpi_table_fadt, minor_revision) + 1) as u32;
pub const ACPI_FADT_V3_SIZE: u32 = core::mem::offset_of!(acpi_table_fadt, sleep_control) as u32;
pub const ACPI_FADT_V5_SIZE: u32 = core::mem::offset_of!(acpi_table_fadt, hypervisor_id) as u32;
pub const ACPI_FADT_V6_SIZE: u32 = core::mem::size_of::<acpi_table_fadt>() as u32;
pub const ACPI_FADT_CONFORMANCE: &str = "ACPI 6.1 (FADT version 6)";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
