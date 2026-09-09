/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* AML resource descriptors; translated from amlresrc.h. */

/* Resource descriptor tags. */
pub const ACPI_RESTAG_ADDRESS: &str = "_ADR";
pub const ACPI_RESTAG_ALIGNMENT: &str = "_ALN";
pub const ACPI_RESTAG_ADDRESSSPACE: &str = "_ASI";
pub const ACPI_RESTAG_ACCESSSIZE: &str = "_ASZ";
pub const ACPI_RESTAG_TYPESPECIFICATTRIBUTES: &str = "_ATT";
pub const ACPI_RESTAG_BASEADDRESS: &str = "_BAS";
pub const ACPI_RESTAG_BUSMASTER: &str = "_BM_";
pub const ACPI_RESTAG_DEBOUNCETIME: &str = "_DBT";
pub const ACPI_RESTAG_DECODE: &str = "_DEC";
pub const ACPI_RESTAG_DEVICEPOLARITY: &str = "_DPL";
pub const ACPI_RESTAG_DMA: &str = "_DMA";
pub const ACPI_RESTAG_DMATYPE: &str = "_TYP";
pub const ACPI_RESTAG_DRIVESTRENGTH: &str = "_DRS";
pub const ACPI_RESTAG_ENDIANNESS: &str = "_END";
pub const ACPI_RESTAG_FLOWCONTROL: &str = "_FLC";
pub const ACPI_RESTAG_FUNCTION: &str = "_FUN";
pub const ACPI_RESTAG_GRANULARITY: &str = "_GRA";
pub const ACPI_RESTAG_INTERRUPT: &str = "_INT";
pub const ACPI_RESTAG_INTERRUPTLEVEL: &str = "_LL_";
pub const ACPI_RESTAG_INTERRUPTSHARE: &str = "_SHR";
pub const ACPI_RESTAG_INTERRUPTTYPE: &str = "_HE_";
pub const ACPI_RESTAG_IORESTRICTION: &str = "_IOR";
pub const ACPI_RESTAG_LENGTH: &str = "_LEN";
pub const ACPI_RESTAG_LINE: &str = "_LIN";
pub const ACPI_RESTAG_LOCALPORT: &str = "_PRT";
pub const ACPI_RESTAG_MEMATTRIBUTES: &str = "_MTP";
pub const ACPI_RESTAG_MEMTYPE: &str = "_MEM";
pub const ACPI_RESTAG_MAXADDR: &str = "_MAX";
pub const ACPI_RESTAG_MINADDR: &str = "_MIN";
pub const ACPI_RESTAG_MAXTYPE: &str = "_MAF";
pub const ACPI_RESTAG_MINTYPE: &str = "_MIF";
pub const ACPI_RESTAG_MODE: &str = "_MOD";
pub const ACPI_RESTAG_PARITY: &str = "_PAR";
pub const ACPI_RESTAG_PHASE: &str = "_PHA";
pub const ACPI_RESTAG_PHYTYPE: &str = "_PHY";
pub const ACPI_RESTAG_PIN: &str = "_PIN";
pub const ACPI_RESTAG_PINCONFIG: &str = "_PPI";
pub const ACPI_RESTAG_PINCONFIG_TYPE: &str = "_TYP";
pub const ACPI_RESTAG_PINCONFIG_VALUE: &str = "_VAL";
pub const ACPI_RESTAG_POLARITY: &str = "_POL";
pub const ACPI_RESTAG_REGISTERBITOFFSET: &str = "_RBO";
pub const ACPI_RESTAG_REGISTERBITWIDTH: &str = "_RBW";
pub const ACPI_RESTAG_RANGETYPE: &str = "_RNG";
pub const ACPI_RESTAG_READWRITETYPE: &str = "_RW_";
pub const ACPI_RESTAG_LENGTH_RX: &str = "_RXL";
pub const ACPI_RESTAG_LENGTH_TX: &str = "_TXL";
pub const ACPI_RESTAG_SLAVEMODE: &str = "_SLV";
pub const ACPI_RESTAG_SPEED: &str = "_SPE";
pub const ACPI_RESTAG_STOPBITS: &str = "_STB";
pub const ACPI_RESTAG_TRANSLATION: &str = "_TRA";
pub const ACPI_RESTAG_TRANSTYPE: &str = "_TRS";
pub const ACPI_RESTAG_TYPE: &str = "_TTP";
pub const ACPI_RESTAG_XFERTYPE: &str = "_SIZ";
pub const ACPI_RESTAG_VENDORDATA: &str = "_VEN";
pub const ACPI_RESTAG_FQN: &str = "_FQN";
pub const ACPI_RESTAG_FQD: &str = "_FQD";

pub const ASL_RDESC_IRQ_SIZE: u32 = 0x02;
pub const ASL_RDESC_DMA_SIZE: u32 = 0x02;
pub const ASL_RDESC_ST_DEPEND_SIZE: u32 = 0x00;
pub const ASL_RDESC_END_DEPEND_SIZE: u32 = 0x00;
pub const ASL_RDESC_IO_SIZE: u32 = 0x07;
pub const ASL_RDESC_FIXED_IO_SIZE: u32 = 0x03;
pub const ASL_RDESC_FIXED_DMA_SIZE: u32 = 0x05;
pub const ASL_RDESC_END_TAG_SIZE: u32 = 0x01;

#[repr(C)]
pub struct asl_resource_node { pub buffer_length: u32, pub buffer: *mut core::ffi::c_void, pub next: *mut asl_resource_node }
#[repr(C)]
pub struct asl_resource_info { pub descriptor_type_op: *mut acpi_parse_object, pub mapping_op: *mut acpi_parse_object, pub current_byte_offset: u32 }

#[macro_export]
macro_rules! ACPI_AML_SIZE_LARGE { ($r:ty) => { core::mem::size_of::<$r>() - core::mem::size_of::<aml_resource_large_header>() }; }
#[macro_export]
macro_rules! ACPI_AML_SIZE_SMALL { ($r:ty) => { core::mem::size_of::<$r>() - core::mem::size_of::<aml_resource_small_header>() }; }

#[repr(C, packed)] pub struct aml_resource_small_header { pub descriptor_type: u8 }
#[repr(C, packed)] pub struct aml_resource_irq { pub descriptor_type: u8, pub irq_mask: u16, pub flags: u8 }
#[repr(C, packed)] pub struct aml_resource_irq_noflags { pub descriptor_type: u8, pub irq_mask: u16 }
#[repr(C, packed)] pub struct aml_resource_dma { pub descriptor_type: u8, pub dma_channel_mask: u8, pub flags: u8 }
#[repr(C, packed)] pub struct aml_resource_start_dependent { pub descriptor_type: u8, pub flags: u8 }
#[repr(C, packed)] pub struct aml_resource_start_dependent_noprio { pub descriptor_type: u8 }
#[repr(C, packed)] pub struct aml_resource_end_dependent { pub descriptor_type: u8 }
#[repr(C, packed)] pub struct aml_resource_io { pub descriptor_type: u8, pub flags: u8, pub minimum: u16, pub maximum: u16, pub alignment: u8, pub address_length: u8 }
#[repr(C, packed)] pub struct aml_resource_fixed_io { pub descriptor_type: u8, pub address: u16, pub address_length: u8 }
#[repr(C, packed)] pub struct aml_resource_vendor_small { pub descriptor_type: u8 }
#[repr(C, packed)] pub struct aml_resource_end_tag { pub descriptor_type: u8, pub checksum: u8 }
#[repr(C, packed)] pub struct aml_resource_fixed_dma { pub descriptor_type: u8, pub request_lines: u16, pub channels: u16, pub width: u8 }

#[repr(C, packed)] pub struct aml_resource_large_header { pub descriptor_type: u8, pub resource_length: u16 }
pub const ACPI_RESOURCE_FLAG_DEC: u32 = 2;
pub const ACPI_RESOURCE_FLAG_MIF: u32 = 4;
pub const ACPI_RESOURCE_FLAG_MAF: u32 = 8;
#[repr(C, packed)] pub struct aml_resource_memory24 { pub descriptor_type:u8,pub resource_length:u16,pub flags:u8,pub minimum:u16,pub maximum:u16,pub alignment:u16,pub address_length:u16 }
#[repr(C, packed)] pub struct aml_resource_vendor_large { pub descriptor_type:u8,pub resource_length:u16 }
#[repr(C, packed)] pub struct aml_resource_memory32 { pub descriptor_type:u8,pub resource_length:u16,pub flags:u8,pub minimum:u32,pub maximum:u32,pub alignment:u32,pub address_length:u32 }
#[repr(C, packed)] pub struct aml_resource_fixed_memory32 { pub descriptor_type:u8,pub resource_length:u16,pub flags:u8,pub address:u32,pub address_length:u32 }
#[repr(C, packed)] pub struct aml_resource_address { pub descriptor_type:u8,pub resource_length:u16,pub resource_type:u8,pub flags:u8,pub specific_flags:u8 }
#[repr(C, packed)] pub struct aml_resource_extended_address64 { pub descriptor_type:u8,pub resource_length:u16,pub resource_type:u8,pub flags:u8,pub specific_flags:u8,pub revision_id:u8,pub reserved:u8,pub granularity:u64,pub minimum:u64,pub maximum:u64,pub translation_offset:u64,pub address_length:u64,pub type_specific:u64 }
#[repr(C, packed)] pub struct aml_resource_address64 { pub descriptor_type:u8,pub resource_length:u16,pub resource_type:u8,pub flags:u8,pub specific_flags:u8,pub granularity:u64,pub minimum:u64,pub maximum:u64,pub translation_offset:u64,pub address_length:u64 }
#[repr(C, packed)] pub struct aml_resource_address32 { pub descriptor_type:u8,pub resource_length:u16,pub resource_type:u8,pub flags:u8,pub specific_flags:u8,pub granularity:u32,pub minimum:u32,pub maximum:u32,pub translation_offset:u32,pub address_length:u32 }
#[repr(C, packed)] pub struct aml_resource_address16 { pub descriptor_type:u8,pub resource_length:u16,pub resource_type:u8,pub flags:u8,pub specific_flags:u8,pub granularity:u16,pub minimum:u16,pub maximum:u16,pub translation_offset:u16,pub address_length:u16 }
#[repr(C, packed)] pub struct aml_resource_extended_irq { pub descriptor_type:u8,pub resource_length:u16,pub flags:u8,pub interrupt_count:u8,pub interrupt:u32 }
#[repr(C, packed)] pub struct aml_resource_generic_register { pub descriptor_type:u8,pub resource_length:u16,pub address_space_id:u8,pub bit_width:u8,pub bit_offset:u8,pub access_size:u8,pub address:u64 }
#[repr(C, packed)] pub struct aml_resource_gpio { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub connection_type:u8,pub flags:u16,pub int_flags:u16,pub pin_config:u8,pub drive_strength:u16,pub debounce_timeout:u16,pub pin_table_offset:u16,pub res_source_index:u8,pub res_source_offset:u16,pub vendor_offset:u16,pub vendor_length:u16 }

pub const AML_RESOURCE_EXTENDED_ADDRESS_REVISION:u8=1;
pub const AML_RESOURCE_GPIO_REVISION:u8=1;
pub const AML_RESOURCE_GPIO_TYPE_INT:u8=0;
pub const AML_RESOURCE_GPIO_TYPE_IO:u8=1;
pub const AML_RESOURCE_MAX_GPIOTYPE:u8=1;
pub const AML_RESOURCE_I2C_SERIALBUSTYPE:u8=1;
pub const AML_RESOURCE_SPI_SERIALBUSTYPE:u8=2;
pub const AML_RESOURCE_UART_SERIALBUSTYPE:u8=3;
pub const AML_RESOURCE_CSI2_SERIALBUSTYPE:u8=4;
pub const AML_RESOURCE_MAX_SERIALBUSTYPE:u8=4;
pub const AML_RESOURCE_VENDOR_SERIALBUSTYPE:u8=192;

#[repr(C, packed)] pub struct aml_resource_common_serialbus { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub res_source_index:u8,pub type_:u8,pub flags:u8,pub type_specific_flags:u16,pub type_revision_id:u8,pub type_data_length:u16 }
#[repr(C, packed)] pub struct aml_resource_csi2_serialbus { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub res_source_index:u8,pub type_:u8,pub flags:u8,pub type_specific_flags:u16,pub type_revision_id:u8,pub type_data_length:u16 }
#[repr(C, packed)] pub struct aml_resource_i2c_serialbus { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub res_source_index:u8,pub type_:u8,pub flags:u8,pub type_specific_flags:u16,pub type_revision_id:u8,pub type_data_length:u16,pub connection_speed:u32,pub slave_address:u16 }
#[repr(C, packed)] pub struct aml_resource_spi_serialbus { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub res_source_index:u8,pub type_:u8,pub flags:u8,pub type_specific_flags:u16,pub type_revision_id:u8,pub type_data_length:u16,pub connection_speed:u32,pub data_bit_length:u8,pub clock_phase:u8,pub clock_polarity:u8,pub device_selection:u16 }
#[repr(C, packed)] pub struct aml_resource_uart_serialbus { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub res_source_index:u8,pub type_:u8,pub flags:u8,pub type_specific_flags:u16,pub type_revision_id:u8,pub type_data_length:u16,pub default_baud_rate:u32,pub rx_fifo_size:u16,pub tx_fifo_size:u16,pub parity:u8,pub lines_enabled:u8 }
#[repr(C, packed)] pub struct aml_resource_pin_function { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub flags:u16,pub pin_config:u8,pub function_number:u16,pub pin_table_offset:u16,pub res_source_index:u8,pub res_source_offset:u16,pub vendor_offset:u16,pub vendor_length:u16 }
#[repr(C, packed)] pub struct aml_resource_pin_config { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub flags:u16,pub pin_config_type:u8,pub pin_config_value:u32,pub pin_table_offset:u16,pub res_source_index:u8,pub res_source_offset:u16,pub vendor_offset:u16,pub vendor_length:u16 }
#[repr(C, packed)] pub struct aml_resource_clock_input { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub flags:u16,pub frequency_divisor:u16,pub frequency_numerator:u32 }
#[repr(C, packed)] pub struct aml_resource_pin_group { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub flags:u16,pub pin_table_offset:u16,pub label_offset:u16,pub vendor_offset:u16,pub vendor_length:u16 }
#[repr(C, packed)] pub struct aml_resource_pin_group_function { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub flags:u16,pub function_number:u16,pub res_source_index:u8,pub res_source_offset:u16,pub res_source_label_offset:u16,pub vendor_offset:u16,pub vendor_length:u16 }
#[repr(C, packed)] pub struct aml_resource_pin_group_config { pub descriptor_type:u8,pub resource_length:u16,pub revision_id:u8,pub flags:u16,pub pin_config_type:u8,pub pin_config_value:u32,pub res_source_index:u8,pub res_source_offset:u16,pub res_source_label_offset:u16,pub vendor_offset:u16,pub vendor_length:u16 }

pub const AML_RESOURCE_CSI2_REVISION:u8=1; pub const AML_RESOURCE_CSI2_TYPE_REVISION:u8=1; pub const AML_RESOURCE_CSI2_MIN_DATA_LEN:u8=0;
pub const AML_RESOURCE_I2C_REVISION:u8=1; pub const AML_RESOURCE_I2C_TYPE_REVISION:u8=1; pub const AML_RESOURCE_I2C_MIN_DATA_LEN:u8=6;
pub const AML_RESOURCE_SPI_REVISION:u8=1; pub const AML_RESOURCE_SPI_TYPE_REVISION:u8=1; pub const AML_RESOURCE_SPI_MIN_DATA_LEN:u8=9;
pub const AML_RESOURCE_UART_REVISION:u8=1; pub const AML_RESOURCE_UART_TYPE_REVISION:u8=1; pub const AML_RESOURCE_UART_MIN_DATA_LEN:u8=10;
pub const AML_RESOURCE_PIN_FUNCTION_REVISION:u8=1; pub const AML_RESOURCE_PIN_CONFIG_REVISION:u8=1; pub const AML_RESOURCE_CLOCK_INPUT_REVISION:u8=1; pub const AML_RESOURCE_PIN_GROUP_REVISION:u8=1; pub const AML_RESOURCE_PIN_GROUP_FUNCTION_REVISION:u8=1; pub const AML_RESOURCE_PIN_GROUP_CONFIG_REVISION:u8=1;

#[repr(C)] pub union aml_resource { pub descriptor_type:u8, pub small_header: core::mem::ManuallyDrop<aml_resource_small_header>, pub large_header: core::mem::ManuallyDrop<aml_resource_large_header>, pub irq:core::mem::ManuallyDrop<aml_resource_irq>, pub dma:core::mem::ManuallyDrop<aml_resource_dma>, pub start_dpf:core::mem::ManuallyDrop<aml_resource_start_dependent>, pub end_dpf:core::mem::ManuallyDrop<aml_resource_end_dependent>, pub io:core::mem::ManuallyDrop<aml_resource_io>, pub fixed_io:core::mem::ManuallyDrop<aml_resource_fixed_io>, pub fixed_dma:core::mem::ManuallyDrop<aml_resource_fixed_dma>, pub vendor_small:core::mem::ManuallyDrop<aml_resource_vendor_small>, pub end_tag:core::mem::ManuallyDrop<aml_resource_end_tag>, pub memory24:core::mem::ManuallyDrop<aml_resource_memory24>, pub generic_reg:core::mem::ManuallyDrop<aml_resource_generic_register>, pub vendor_large:core::mem::ManuallyDrop<aml_resource_vendor_large>, pub memory32:core::mem::ManuallyDrop<aml_resource_memory32>, pub fixed_memory32:core::mem::ManuallyDrop<aml_resource_fixed_memory32>, pub address16:core::mem::ManuallyDrop<aml_resource_address16>, pub address32:core::mem::ManuallyDrop<aml_resource_address32>, pub address64:core::mem::ManuallyDrop<aml_resource_address64>, pub ext_address64:core::mem::ManuallyDrop<aml_resource_extended_address64>, pub extended_irq:core::mem::ManuallyDrop<aml_resource_extended_irq>, pub gpio:core::mem::ManuallyDrop<aml_resource_gpio>, pub i2c_serial_bus:core::mem::ManuallyDrop<aml_resource_i2c_serialbus>, pub spi_serial_bus:core::mem::ManuallyDrop<aml_resource_spi_serialbus>, pub uart_serial_bus:core::mem::ManuallyDrop<aml_resource_uart_serialbus>, pub csi2_serial_bus:core::mem::ManuallyDrop<aml_resource_csi2_serialbus>, pub common_serial_bus:core::mem::ManuallyDrop<aml_resource_common_serialbus>, pub pin_function:core::mem::ManuallyDrop<aml_resource_pin_function>, pub pin_config:core::mem::ManuallyDrop<aml_resource_pin_config>, pub pin_group:core::mem::ManuallyDrop<aml_resource_pin_group>, pub pin_group_function:core::mem::ManuallyDrop<aml_resource_pin_group_function>, pub pin_group_config:core::mem::ManuallyDrop<aml_resource_pin_group_config>, pub clock_input:core::mem::ManuallyDrop<aml_resource_clock_input>, pub address:core::mem::ManuallyDrop<aml_resource_address>, pub dword_item:u32, pub word_item:u16, pub byte_item:u8 }

extern "C" {
    pub fn mp_save_gpio_info(op:*mut acpi_parse_object, resource:*mut aml_resource, pin_count:u32, pin_list:*mut u16, device_name:*mut core::ffi::c_char);
    pub fn mp_save_serial_info(op:*mut acpi_parse_object, resource:*mut aml_resource, device_name:*mut core::ffi::c_char);
    pub fn mp_get_hid_from_parse_tree(hid_node:*mut acpi_namespace_node) -> *mut core::ffi::c_char;
    pub fn mp_get_hid_via_namestring(device_name:*mut core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn mp_get_connection_info(op:*mut acpi_parse_object, pin_index:u32, target_node:*mut *mut acpi_namespace_node, target_name:*mut *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn mp_get_parent_device_hid(op:*mut acpi_parse_object, target_node:*mut *mut acpi_namespace_node, parent_device_name:*mut *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn mp_get_ddn_value(device_name:*mut core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn mp_get_hid_value(device_node:*mut acpi_namespace_node) -> *mut core::ffi::c_char;
}

/* External project types supplied by the surrounding ACPICA translation. */
extern "C" { type acpi_parse_object; type acpi_namespace_node; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
