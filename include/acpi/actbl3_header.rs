/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Translation of actbl3.h. External ACPI types are supplied by other headers. */

pub const ACPI_SIG_SLIC: &str = "SLIC";
pub const ACPI_SIG_SLIT: &str = "SLIT";
pub const ACPI_SIG_SPCR: &str = "SPCR";
pub const ACPI_SIG_SPMI: &str = "SPMI";
pub const ACPI_SIG_SRAT: &str = "SRAT";
pub const ACPI_SIG_STAO: &str = "STAO";
pub const ACPI_SIG_TCPA: &str = "TCPA";
pub const ACPI_SIG_TPM2: &str = "TPM2";
pub const ACPI_SIG_UEFI: &str = "UEFI";
pub const ACPI_SIG_VIOT: &str = "VIOT";
pub const ACPI_SIG_WAET: &str = "WAET";
pub const ACPI_SIG_WDAT: &str = "WDAT";
pub const ACPI_SIG_WDDT: &str = "WDDT";
pub const ACPI_SIG_WDRT: &str = "WDRT";
pub const ACPI_SIG_WPBT: &str = "WPBT";
pub const ACPI_SIG_WSMT: &str = "WSMT";
pub const ACPI_SIG_XENV: &str = "XENV";
pub const ACPI_SIG_XXXX: &str = "XXXX";

#[repr(C, packed)] pub struct acpi_table_slic { pub header: acpi_table_header }
#[repr(C, packed)] pub struct acpi_table_slit { pub header: acpi_table_header, pub locality_count: u64, pub entry: [u8; 0] }
#[repr(C, packed)] pub struct acpi_table_spcr {
    pub header: acpi_table_header, pub interface_type: u8, pub reserved: [u8; 3],
    pub serial_port: acpi_generic_address, pub interrupt_type: u8, pub pc_interrupt: u8,
    pub interrupt: u32, pub baud_rate: u8, pub parity: u8, pub stop_bits: u8,
    pub flow_control: u8, pub terminal_type: u8, pub language: u8, pub pci_device_id: u16,
    pub pci_vendor_id: u16, pub pci_bus: u8, pub pci_device: u8, pub pci_function: u8,
    pub pci_flags: u32, pub pci_segment: u8, pub uart_clk_freq: u32, pub precise_baudrate: u32,
    pub name_space_string_length: u16, pub name_space_string_offset: u16,
    pub name_space_string: [i8; 0],
}
pub const ACPI_SPCR_DO_NOT_DISABLE: u32 = 1;

#[repr(C, packed)] pub struct acpi_table_spmi {
    pub header: acpi_table_header, pub interface_type: u8, pub reserved: u8, pub spec_revision: u16,
    pub interrupt_type: u8, pub gpe_number: u8, pub reserved1: u8, pub pci_device_flag: u8,
    pub interrupt: u32, pub ipmi_register: acpi_generic_address, pub pci_segment: u8,
    pub pci_bus: u8, pub pci_device: u8, pub pci_function: u8, pub reserved2: u8,
}
#[repr(i32)] pub enum acpi_spmi_interface_types { ACPI_SPMI_NOT_USED=0, ACPI_SPMI_KEYBOARD=1, ACPI_SPMI_SMI=2, ACPI_SPMI_BLOCK_TRANSFER=3, ACPI_SPMI_SMBUS=4, ACPI_SPMI_RESERVED=5 }

#[repr(C, packed)] pub struct acpi_table_srat { pub header: acpi_table_header, pub table_revision: u32, pub reserved: u64 }
#[repr(i32)] pub enum acpi_srat_type { ACPI_SRAT_TYPE_CPU_AFFINITY=0, ACPI_SRAT_TYPE_MEMORY_AFFINITY=1, ACPI_SRAT_TYPE_X2APIC_CPU_AFFINITY=2, ACPI_SRAT_TYPE_GICC_AFFINITY=3, ACPI_SRAT_TYPE_GIC_ITS_AFFINITY=4, ACPI_SRAT_TYPE_GENERIC_AFFINITY=5, ACPI_SRAT_TYPE_GENERIC_PORT_AFFINITY=6, ACPI_SRAT_TYPE_RINTC_AFFINITY=7, ACPI_SRAT_TYPE_RESERVED=8 }
#[repr(C, packed)] pub struct acpi_srat_cpu_affinity { pub header: acpi_subtable_header, pub proximity_domain_lo:u8, pub apic_id:u8, pub flags:u32, pub local_sapic_eid:u8, pub proximity_domain_hi:[u8;3], pub clock_domain:u32 }
pub const ACPI_SRAT_CPU_USE_AFFINITY:u32=1;
#[repr(C, packed)] pub struct acpi_srat_mem_affinity { pub header:acpi_subtable_header, pub proximity_domain:u32, pub reserved:u16, pub base_address:u64, pub length:u64, pub reserved1:u32, pub flags:u32, pub reserved2:u64 }
pub const ACPI_SRAT_MEM_ENABLED:u32=1; pub const ACPI_SRAT_MEM_HOT_PLUGGABLE:u32=1<<1; pub const ACPI_SRAT_MEM_NON_VOLATILE:u32=1<<2; pub const ACPI_SRAT_MEM_SPEC_PURPOSE:u32=1<<3;
#[repr(C, packed)] pub struct acpi_srat_x2apic_cpu_affinity { pub header:acpi_subtable_header, pub reserved:u16, pub proximity_domain:u32, pub apic_id:u32, pub flags:u32, pub clock_domain:u32, pub reserved2:u32 }
pub const ACPI_SRAT_CPU_ENABLED:u32=1;
#[repr(C, packed)] pub struct acpi_srat_gicc_affinity { pub header:acpi_subtable_header, pub proximity_domain:u32, pub acpi_processor_uid:u32, pub flags:u32, pub clock_domain:u32 }
pub const ACPI_SRAT_GICC_ENABLED:u32=1;
#[repr(C, packed)] pub struct acpi_srat_gic_its_affinity { pub header:acpi_subtable_header, pub proximity_domain:u32, pub reserved:u16, pub its_id:u32 }
pub const ACPI_SRAT_DEVICE_HANDLE_SIZE:usize=16;
#[repr(C, packed)] pub struct acpi_srat_generic_affinity { pub header:acpi_subtable_header, pub reserved:u8, pub device_handle_type:u8, pub proximity_domain:u32, pub device_handle:[u8;ACPI_SRAT_DEVICE_HANDLE_SIZE], pub flags:u32, pub reserved1:u32 }
pub const ACPI_SRAT_GENERIC_AFFINITY_ENABLED:u32=1; pub const ACPI_SRAT_ARCHITECTURAL_TRANSACTIONS:u32=1<<1;
#[repr(C, packed)] pub struct acpi_srat_rintc_affinity { pub header:acpi_subtable_header, pub reserved:u16, pub proximity_domain:u32, pub acpi_processor_uid:u32, pub flags:u32, pub clock_domain:u32 }
pub const ACPI_SRAT_RINTC_ENABLED:u32=1;

#[repr(C, packed)] pub struct acpi_table_stao { pub header:acpi_table_header, pub ignore_uart:u8 }
#[repr(C, packed)] pub struct acpi_table_tcpa_hdr { pub header:acpi_table_header, pub platform_class:u16 }
pub const ACPI_TCPA_CLIENT_TABLE:u32=0; pub const ACPI_TCPA_SERVER_TABLE:u32=1;
#[repr(C, packed)] pub struct acpi_table_tcpa_client { pub minimum_log_length:u32, pub log_address:u64 }
#[repr(C, packed)] pub struct acpi_table_tcpa_server { pub reserved:u16, pub minimum_log_length:u64, pub log_address:u64, pub spec_revision:u16, pub device_flags:u8, pub interrupt_flags:u8, pub gpe_number:u8, pub reserved2:[u8;3], pub global_interrupt:u32, pub address:acpi_generic_address, pub reserved3:u32, pub config_address:acpi_generic_address, pub group:u8, pub bus:u8, pub device:u8, pub function:u8 }
pub const ACPI_TCPA_PCI_DEVICE:u32=1; pub const ACPI_TCPA_BUS_PNP:u32=1<<1; pub const ACPI_TCPA_ADDRESS_VALID:u32=1<<2;
pub const ACPI_TCPA_INTERRUPT_MODE:u32=1; pub const ACPI_TCPA_INTERRUPT_POLARITY:u32=1<<1; pub const ACPI_TCPA_SCI_VIA_GPE:u32=1<<2; pub const ACPI_TCPA_GLOBAL_INTERRUPT:u32=1<<3;

#[repr(C, packed)] pub struct acpi_table_tpm23 { pub header:acpi_table_header, pub reserved:u32, pub control_address:u64, pub start_method:u32 }
pub const ACPI_TPM23_ACPI_START_METHOD:u32=2;
#[repr(C, packed)] pub struct acpi_tmp23_trailer { pub reserved:u32 }
#[repr(C, packed)] pub struct acpi_table_tpm2 { pub header:acpi_table_header, pub platform_class:u16, pub reserved:u16, pub control_address:u64, pub start_method:u32 }
#[repr(C, packed)] pub struct acpi_tpm2_phy { pub start_method_specific:[u8;12], pub log_area_minimum_length:u32, pub log_area_start_address:u64 }
pub const ACPI_TPM2_NOT_ALLOWED:u32=0; pub const ACPI_TPM2_RESERVED1:u32=1; pub const ACPI_TPM2_START_METHOD:u32=2; pub const ACPI_TPM2_RESERVED3:u32=3; pub const ACPI_TPM2_RESERVED4:u32=4; pub const ACPI_TPM2_RESERVED5:u32=5; pub const ACPI_TPM2_MEMORY_MAPPED:u32=6; pub const ACPI_TPM2_COMMAND_BUFFER:u32=7; pub const ACPI_TPM2_COMMAND_BUFFER_WITH_START_METHOD:u32=8; pub const ACPI_TPM2_RESERVED9:u32=9; pub const ACPI_TPM2_RESERVED10:u32=10; pub const ACPI_TPM2_COMMAND_BUFFER_WITH_ARM_SMC:u32=11; pub const ACPI_TPM2_RESERVED:u32=12; pub const ACPI_TPM2_COMMAND_BUFFER_WITH_PLUTON:u32=13; pub const ACPI_TPM2_CRB_WITH_ARM_FFA:u32=15;
#[repr(C, packed)] pub struct acpi_tpm2_trailer { pub method_parameters:[u8;12], pub minimum_log_length:u32, pub log_address:u64 }
#[repr(C, packed)] pub struct acpi_tpm2_arm_smc { pub global_interrupt:u32, pub interrupt_flags:u8, pub operation_flags:u8, pub reserved:u16, pub function_id:u32 }
pub const ACPI_TPM2_INTERRUPT_SUPPORT:u32=1; pub const ACPI_TPM2_IDLE_SUPPORT:u32=1;

#[repr(C, packed)] pub struct acpi_table_uefi { pub header:acpi_table_header, pub identifier:[u8;16], pub data_offset:u16 }
#[repr(C, packed)] pub struct acpi_table_viot { pub header:acpi_table_header, pub node_count:u16, pub node_offset:u16, pub reserved:[u8;8] }
#[repr(C, packed)] pub struct acpi_viot_header { pub type_:u8, pub reserved:u8, pub length:u16 }
#[repr(i32)] pub enum acpi_viot_node_type { ACPI_VIOT_NODE_PCI_RANGE=1, ACPI_VIOT_NODE_MMIO=2, ACPI_VIOT_NODE_VIRTIO_IOMMU_PCI=3, ACPI_VIOT_NODE_VIRTIO_IOMMU_MMIO=4, ACPI_VIOT_RESERVED=5 }
#[repr(C, packed)] pub struct acpi_viot_pci_range { pub header:acpi_viot_header, pub endpoint_start:u32, pub segment_start:u16, pub segment_end:u16, pub bdf_start:u16, pub bdf_end:u16, pub output_node:u16, pub reserved:[u8;6] }
#[repr(C, packed)] pub struct acpi_viot_mmio { pub header:acpi_viot_header, pub endpoint:u32, pub base_address:u64, pub output_node:u16, pub reserved:[u8;6] }
#[repr(C, packed)] pub struct acpi_viot_virtio_iommu_pci { pub header:acpi_viot_header, pub segment:u16, pub bdf:u16, pub reserved:[u8;8] }
#[repr(C, packed)] pub struct acpi_viot_virtio_iommu_mmio { pub header:acpi_viot_header, pub reserved:[u8;4], pub base_address:u64 }

#[repr(C, packed)] pub struct acpi_table_waet { pub header:acpi_table_header, pub flags:u32 }
pub const ACPI_WAET_RTC_NO_ACK:u32=1; pub const ACPI_WAET_TIMER_ONE_READ:u32=1<<1;
#[repr(C, packed)] pub struct acpi_table_wdat { pub header:acpi_table_header, pub header_length:u32, pub pci_segment:u16, pub pci_bus:u8, pub pci_device:u8, pub pci_function:u8, pub reserved:[u8;3], pub timer_period:u32, pub max_count:u32, pub min_count:u32, pub flags:u8, pub reserved2:[u8;3], pub entries:u32 }
pub const ACPI_WDAT_ENABLED:u32=1; pub const ACPI_WDAT_STOPPED:u32=0x80;
#[repr(C, packed)] pub struct acpi_wdat_entry { pub action:u8, pub instruction:u8, pub reserved:u16, pub register_region:acpi_generic_address, pub value:u32, pub mask:u32 }
#[repr(i32)] pub enum acpi_wdat_actions { ACPI_WDAT_RESET=1, ACPI_WDAT_GET_CURRENT_COUNTDOWN=4, ACPI_WDAT_GET_COUNTDOWN=5, ACPI_WDAT_SET_COUNTDOWN=6, ACPI_WDAT_GET_RUNNING_STATE=8, ACPI_WDAT_SET_RUNNING_STATE=9, ACPI_WDAT_GET_STOPPED_STATE=10, ACPI_WDAT_SET_STOPPED_STATE=11, ACPI_WDAT_GET_REBOOT=16, ACPI_WDAT_SET_REBOOT=17, ACPI_WDAT_GET_SHUTDOWN=18, ACPI_WDAT_SET_SHUTDOWN=19, ACPI_WDAT_GET_STATUS=32, ACPI_WDAT_SET_STATUS=33, ACPI_WDAT_ACTION_RESERVED=34 }
#[repr(i32)] pub enum acpi_wdat_instructions { ACPI_WDAT_READ_VALUE=0, ACPI_WDAT_READ_COUNTDOWN=1, ACPI_WDAT_WRITE_VALUE=2, ACPI_WDAT_WRITE_COUNTDOWN=3, ACPI_WDAT_INSTRUCTION_RESERVED=4, ACPI_WDAT_PRESERVE_REGISTER=0x80 }

#[repr(C, packed)] pub struct acpi_table_wddt { pub header:acpi_table_header, pub spec_version:u16, pub table_version:u16, pub pci_vendor_id:u16, pub address:acpi_generic_address, pub max_count:u16, pub min_count:u16, pub period:u16, pub status:u16, pub capability:u16 }
pub const ACPI_WDDT_AVAILABLE:u32=1; pub const ACPI_WDDT_ACTIVE:u32=1<<1; pub const ACPI_WDDT_TCO_OS_OWNED:u32=1<<2; pub const ACPI_WDDT_USER_RESET:u32=1<<11; pub const ACPI_WDDT_WDT_RESET:u32=1<<12; pub const ACPI_WDDT_POWER_FAIL:u32=1<<13; pub const ACPI_WDDT_UNKNOWN_RESET:u32=1<<14; pub const ACPI_WDDT_AUTO_RESET:u32=1; pub const ACPI_WDDT_ALERT_SUPPORT:u32=1<<1;
#[repr(C, packed)] pub struct acpi_table_wdrt { pub header:acpi_table_header, pub control_register:acpi_generic_address, pub count_register:acpi_generic_address, pub pci_device_id:u16, pub pci_vendor_id:u16, pub pci_bus:u8, pub pci_device:u8, pub pci_function:u8, pub pci_segment:u8, pub max_count:u16, pub units:u8 }
#[repr(C, packed)] pub struct acpi_table_wpbt { pub header:acpi_table_header, pub handoff_size:u32, pub handoff_address:u64, pub layout:u8, pub type_:u8, pub arguments_length:u16 }
#[repr(C, packed)] pub struct acpi_wpbt_unicode { pub unicode_string:*mut u16 }
#[repr(C, packed)] pub struct acpi_table_wsmt { pub header:acpi_table_header, pub protection_flags:u32 }
pub const ACPI_WSMT_FIXED_COMM_BUFFERS:u32=1; pub const ACPI_WSMT_COMM_BUFFER_NESTED_PTR_PROTECTION:u32=2; pub const ACPI_WSMT_SYSTEM_RESOURCE_PROTECTION:u32=4;
#[repr(C, packed)] pub struct acpi_table_xenv { pub header:acpi_table_header, pub grant_table_address:u64, pub grant_table_size:u64, pub event_interrupt:u32, pub event_flags:u8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
