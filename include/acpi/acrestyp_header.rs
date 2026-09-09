/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Rust translation of acrestyp.h. External ACPI types/macros are supplied by dependencies. */

pub type acpi_rs_length = u16;
pub type acpi_rsdesc_size = u32;

pub const ACPI_READ_ONLY_MEMORY: u8 = 0;
pub const ACPI_READ_WRITE_MEMORY: u8 = 1;
pub const ACPI_NON_CACHEABLE_MEMORY: u8 = 0;
pub const ACPI_CACHABLE_MEMORY: u8 = 1;
pub const ACPI_WRITE_COMBINING_MEMORY: u8 = 2;
pub const ACPI_PREFETCHABLE_MEMORY: u8 = 3;
pub const ACPI_NON_ISA_ONLY_RANGES: u8 = 1;
pub const ACPI_ISA_ONLY_RANGES: u8 = 2;
pub const ACPI_ENTIRE_RANGE: u8 = ACPI_NON_ISA_ONLY_RANGES | ACPI_ISA_ONLY_RANGES;
pub const ACPI_SPARSE_TRANSLATION: u8 = 1;
pub const ACPI_DECODE_10: u8 = 0;
pub const ACPI_DECODE_16: u8 = 1;
pub const ACPI_LEVEL_SENSITIVE: u8 = 0;
pub const ACPI_EDGE_SENSITIVE: u8 = 1;
pub const ACPI_ACTIVE_HIGH: u8 = 0;
pub const ACPI_ACTIVE_LOW: u8 = 1;
pub const ACPI_ACTIVE_BOTH: u8 = 2;
pub const ACPI_EXCLUSIVE: u8 = 0;
pub const ACPI_SHARED: u8 = 1;
pub const ACPI_NOT_WAKE_CAPABLE: u8 = 0;
pub const ACPI_WAKE_CAPABLE: u8 = 1;
pub const ACPI_COMPATIBILITY: u8 = 0;
pub const ACPI_TYPE_A: u8 = 1;
pub const ACPI_TYPE_B: u8 = 2;
pub const ACPI_TYPE_F: u8 = 3;
pub const ACPI_NOT_BUS_MASTER: u8 = 0;
pub const ACPI_BUS_MASTER: u8 = 1;
pub const ACPI_TRANSFER_8: u8 = 0;
pub const ACPI_TRANSFER_8_16: u8 = 1;
pub const ACPI_TRANSFER_16: u8 = 2;
pub const ACPI_GOOD_CONFIGURATION: u8 = 0;
pub const ACPI_ACCEPTABLE_CONFIGURATION: u8 = 1;
pub const ACPI_SUB_OPTIMAL_CONFIGURATION: u8 = 2;
pub const ACPI_MEMORY_RANGE: u8 = 0;
pub const ACPI_IO_RANGE: u8 = 1;
pub const ACPI_BUS_NUMBER_RANGE: u8 = 2;
pub const ACPI_ADDRESS_NOT_FIXED: u8 = 0;
pub const ACPI_ADDRESS_FIXED: u8 = 1;
pub const ACPI_POS_DECODE: u8 = 0;
pub const ACPI_SUB_DECODE: u8 = 1;
pub const ACPI_PRODUCER: u8 = 0;
pub const ACPI_CONSUMER: u8 = 1;

#[repr(C, packed)]
pub struct acpi_uuid { pub data: [u8; ACPI_UUID_LENGTH] }
#[repr(C, packed)]
pub struct acpi_vendor_uuid { pub subtype: u8, pub data: [u8; ACPI_UUID_LENGTH] }

#[repr(C, packed)] pub struct acpi_resource_irq { pub descriptor_length:u8,pub triggering:u8,pub polarity:u8,pub shareable:u8,pub wake_capable:u8,pub interrupt_count:u8,pub data: acpi_resource_irq_data }
#[repr(C, packed)] pub union acpi_resource_irq_data { pub interrupt:u8, pub interrupts:[u8;0] }
#[repr(C, packed)] pub struct acpi_resource_dma { pub type_:u8,pub bus_master:u8,pub transfer:u8,pub channel_count:u8,pub data:acpi_resource_dma_data }
#[repr(C, packed)] pub union acpi_resource_dma_data { pub channel:u8,pub channels:[u8;0] }
#[repr(C, packed)] pub struct acpi_resource_start_dependent { pub descriptor_length:u8,pub compatibility_priority:u8,pub performance_robustness:u8 }
#[repr(C, packed)] pub struct acpi_resource_io { pub io_decode:u8,pub alignment:u8,pub address_length:u8,pub minimum:u16,pub maximum:u16 }
#[repr(C, packed)] pub struct acpi_resource_fixed_io { pub address:u16,pub address_length:u8 }
#[repr(C, packed)] pub struct acpi_resource_fixed_dma { pub request_lines:u16,pub channels:u16,pub width:u8 }
pub const ACPI_DMA_WIDTH8:u32=0; pub const ACPI_DMA_WIDTH16:u32=1; pub const ACPI_DMA_WIDTH32:u32=2; pub const ACPI_DMA_WIDTH64:u32=3; pub const ACPI_DMA_WIDTH128:u32=4; pub const ACPI_DMA_WIDTH256:u32=5;
#[repr(C, packed)] pub struct acpi_resource_vendor { pub byte_length:u16,pub byte_data:[u8;0] }
#[repr(C, packed)] pub struct acpi_resource_vendor_typed { pub byte_length:u16,pub uuid_subtype:u8,pub uuid:[u8;ACPI_UUID_LENGTH],pub byte_data:[u8;0] }
#[repr(C, packed)] pub struct acpi_resource_end_tag { pub checksum:u8 }
#[repr(C, packed)] pub struct acpi_resource_memory24 { pub write_protect:u8,pub minimum:u16,pub maximum:u16,pub alignment:u16,pub address_length:u16 }
#[repr(C, packed)] pub struct acpi_resource_memory32 { pub write_protect:u8,pub minimum:u32,pub maximum:u32,pub alignment:u32,pub address_length:u32 }
#[repr(C, packed)] pub struct acpi_resource_fixed_memory32 { pub write_protect:u8,pub address:u32,pub address_length:u32 }
#[repr(C, packed)] pub struct acpi_memory_attribute { pub write_protect:u8,pub caching:u8,pub range_type:u8,pub translation:u8 }
#[repr(C, packed)] pub struct acpi_io_attribute { pub range_type:u8,pub translation:u8,pub translation_type:u8,pub reserved1:u8 }
#[repr(C, packed)] pub union acpi_resource_attribute { pub mem:acpi_memory_attribute,pub io:acpi_io_attribute,pub type_specific:u8 }
#[repr(C, packed)] pub struct acpi_resource_label { pub string_length:u16,pub string_ptr:*mut i8 }
#[repr(C, packed)] pub struct acpi_resource_source { pub index:u8,pub string_length:u16,pub string_ptr:*mut i8 }

#[repr(C, packed)] pub struct acpi_address16_attribute { pub granularity:u16,pub minimum:u16,pub maximum:u16,pub translation_offset:u16,pub address_length:u16 }
#[repr(C, packed)] pub struct acpi_address32_attribute { pub granularity:u32,pub minimum:u32,pub maximum:u32,pub translation_offset:u32,pub address_length:u32 }
#[repr(C, packed)] pub struct acpi_address64_attribute { pub granularity:u64,pub minimum:u64,pub maximum:u64,pub translation_offset:u64,pub address_length:u64 }
#[repr(C, packed)] pub struct acpi_resource_address { pub resource_type:u8,pub producer_consumer:u8,pub decode:u8,pub min_address_fixed:u8,pub max_address_fixed:u8,pub info:acpi_resource_attribute }
#[repr(C, packed)] pub struct acpi_resource_address16 { pub common:acpi_resource_address,pub address:acpi_address16_attribute,pub resource_source:acpi_resource_source }
#[repr(C, packed)] pub struct acpi_resource_address32 { pub common:acpi_resource_address,pub address:acpi_address32_attribute,pub resource_source:acpi_resource_source }
#[repr(C, packed)] pub struct acpi_resource_address64 { pub common:acpi_resource_address,pub address:acpi_address64_attribute,pub resource_source:acpi_resource_source }
#[repr(C, packed)] pub struct acpi_resource_extended_address64 { pub common:acpi_resource_address,pub revision_ID:u8,pub address:acpi_address64_attribute,pub type_specific:u64 }
#[repr(C, packed)] pub struct acpi_resource_extended_irq { pub producer_consumer:u8,pub triggering:u8,pub polarity:u8,pub shareable:u8,pub wake_capable:u8,pub interrupt_count:u8,pub resource_source:acpi_resource_source,pub data:acpi_resource_extended_irq_data }
#[repr(C, packed)] pub union acpi_resource_extended_irq_data { pub interrupt:u32,pub interrupts:[u32;0] }
#[repr(C, packed)] pub struct acpi_resource_generic_register { pub space_id:u8,pub bit_width:u8,pub bit_offset:u8,pub access_size:u8,pub address:u64 }
#[repr(C, packed)] pub struct acpi_resource_gpio { pub revision_id:u8,pub connection_type:u8,pub producer_consumer:u8,pub pin_config:u8,pub shareable:u8,pub wake_capable:u8,pub io_restriction:u8,pub triggering:u8,pub polarity:u8,pub drive_strength:u16,pub debounce_timeout:u16,pub pin_table_length:u16,pub vendor_length:u16,pub resource_source:acpi_resource_source,pub pin_table:*mut u16,pub vendor_data:*mut u8 }
pub const ACPI_RESOURCE_GPIO_TYPE_INT:u32=0; pub const ACPI_RESOURCE_GPIO_TYPE_IO:u32=1; pub const ACPI_PIN_CONFIG_DEFAULT:u32=0; pub const ACPI_PIN_CONFIG_PULLUP:u32=1; pub const ACPI_PIN_CONFIG_PULLDOWN:u32=2; pub const ACPI_PIN_CONFIG_NOPULL:u32=3; pub const ACPI_IO_RESTRICT_NONE:u32=0; pub const ACPI_IO_RESTRICT_INPUT:u32=1; pub const ACPI_IO_RESTRICT_OUTPUT:u32=2; pub const ACPI_IO_RESTRICT_NONE_PRESERVE:u32=3;
pub const ACPI_RESOURCE_SERIAL_TYPE_I2C:u32=1; pub const ACPI_RESOURCE_SERIAL_TYPE_SPI:u32=2; pub const ACPI_RESOURCE_SERIAL_TYPE_UART:u32=3; pub const ACPI_RESOURCE_SERIAL_TYPE_CSI2:u32=4; pub const ACPI_CONTROLLER_INITIATED:u32=0; pub const ACPI_DEVICE_INITIATED:u32=1; pub const ACPI_I2C_7BIT_MODE:u32=0; pub const ACPI_I2C_10BIT_MODE:u32=1; pub const ACPI_SPI_4WIRE_MODE:u32=0; pub const ACPI_SPI_3WIRE_MODE:u32=1; pub const ACPI_SPI_ACTIVE_LOW:u32=0; pub const ACPI_SPI_ACTIVE_HIGH:u32=1; pub const ACPI_SPI_FIRST_PHASE:u32=0; pub const ACPI_SPI_SECOND_PHASE:u32=1; pub const ACPI_SPI_START_LOW:u32=0; pub const ACPI_SPI_START_HIGH:u32=1; pub const ACPI_UART_LITTLE_ENDIAN:u32=0; pub const ACPI_UART_BIG_ENDIAN:u32=1; pub const ACPI_UART_5_DATA_BITS:u32=0; pub const ACPI_UART_6_DATA_BITS:u32=1; pub const ACPI_UART_7_DATA_BITS:u32=2; pub const ACPI_UART_8_DATA_BITS:u32=3; pub const ACPI_UART_9_DATA_BITS:u32=4; pub const ACPI_UART_NO_STOP_BITS:u32=0; pub const ACPI_UART_1_STOP_BIT:u32=1; pub const ACPI_UART_1P5_STOP_BITS:u32=2; pub const ACPI_UART_2_STOP_BITS:u32=3; pub const ACPI_UART_FLOW_CONTROL_NONE:u32=0; pub const ACPI_UART_FLOW_CONTROL_HW:u32=1; pub const ACPI_UART_FLOW_CONTROL_XON_XOFF:u32=2; pub const ACPI_UART_PARITY_NONE:u32=0; pub const ACPI_UART_PARITY_EVEN:u32=1; pub const ACPI_UART_PARITY_ODD:u32=2; pub const ACPI_UART_PARITY_MARK:u32=3; pub const ACPI_UART_PARITY_SPACE:u32=4; pub const ACPI_UART_CARRIER_DETECT:u32=1<<2; pub const ACPI_UART_RING_INDICATOR:u32=1<<3; pub const ACPI_UART_DATA_SET_READY:u32=1<<4; pub const ACPI_UART_DATA_TERMINAL_READY:u32=1<<5; pub const ACPI_UART_CLEAR_TO_SEND:u32=1<<6; pub const ACPI_UART_REQUEST_TO_SEND:u32=1<<7;

#[repr(C, packed)] pub struct acpi_resource_common_serialbus { pub revision_id:u8,pub type_:u8,pub producer_consumer:u8,pub slave_mode:u8,pub connection_sharing:u8,pub type_revision_id:u8,pub type_data_length:u16,pub vendor_length:u16,pub resource_source:acpi_resource_source,pub vendor_data:*mut u8 }
#[repr(C, packed)] pub struct acpi_resource_i2c_serialbus { pub common:acpi_resource_common_serialbus,pub access_mode:u8,pub slave_address:u16,pub connection_speed:u32,pub lvr:u8 }
#[repr(C, packed)] pub struct acpi_resource_spi_serialbus { pub common:acpi_resource_common_serialbus,pub wire_mode:u8,pub device_polarity:u8,pub data_bit_length:u8,pub clock_phase:u8,pub clock_polarity:u8,pub device_selection:u16,pub connection_speed:u32 }
#[repr(C, packed)] pub struct acpi_resource_uart_serialbus { pub common:acpi_resource_common_serialbus,pub endian:u8,pub data_bits:u8,pub stop_bits:u8,pub flow_control:u8,pub parity:u8,pub lines_enabled:u8,pub rx_fifo_size:u16,pub tx_fifo_size:u16,pub default_baud_rate:u32 }
#[repr(C, packed)] pub struct acpi_resource_csi2_serialbus { pub common:acpi_resource_common_serialbus,pub local_port_instance:u8,pub phy_type:u8 }
#[repr(C, packed)] pub struct acpi_resource_pin_function { pub revision_id:u8,pub pin_config:u8,pub shareable:u8,pub function_number:u16,pub pin_table_length:u16,pub vendor_length:u16,pub resource_source:acpi_resource_source,pub pin_table:*mut u16,pub vendor_data:*mut u8 }
#[repr(C, packed)] pub struct acpi_resource_pin_config { pub revision_id:u8,pub producer_consumer:u8,pub shareable:u8,pub pin_config_type:u8,pub pin_config_value:u32,pub pin_table_length:u16,pub vendor_length:u16,pub resource_source:acpi_resource_source,pub pin_table:*mut u16,pub vendor_data:*mut u8 }
#[repr(C, packed)] pub struct acpi_resource_clock_input { pub revision_id:u8,pub mode:u8,pub scale:u8,pub frequency_divisor:u16,pub frequency_numerator:u32,pub resource_source:acpi_resource_source }
#[repr(C, packed)] pub struct acpi_resource_pin_group { pub revision_id:u8,pub producer_consumer:u8,pub pin_table_length:u16,pub vendor_length:u16,pub pin_table:*mut u16,pub resource_label:acpi_resource_label,pub vendor_data:*mut u8 }
#[repr(C, packed)] pub struct acpi_resource_pin_group_function { pub revision_id:u8,pub producer_consumer:u8,pub shareable:u8,pub function_number:u16,pub vendor_length:u16,pub resource_source:acpi_resource_source,pub resource_source_label:acpi_resource_label,pub vendor_data:*mut u8 }
#[repr(C, packed)] pub struct acpi_resource_pin_group_config { pub revision_id:u8,pub producer_consumer:u8,pub shareable:u8,pub pin_config_type:u8,pub pin_config_value:u32,pub vendor_length:u16,pub resource_source:acpi_resource_source,pub resource_source_label:acpi_resource_label,pub vendor_data:*mut u8 }
pub const ACPI_RESOURCE_TYPE_IRQ:u32=0; pub const ACPI_RESOURCE_TYPE_DMA:u32=1; pub const ACPI_RESOURCE_TYPE_START_DEPENDENT:u32=2; pub const ACPI_RESOURCE_TYPE_END_DEPENDENT:u32=3; pub const ACPI_RESOURCE_TYPE_IO:u32=4; pub const ACPI_RESOURCE_TYPE_FIXED_IO:u32=5; pub const ACPI_RESOURCE_TYPE_VENDOR:u32=6; pub const ACPI_RESOURCE_TYPE_END_TAG:u32=7; pub const ACPI_RESOURCE_TYPE_MEMORY24:u32=8; pub const ACPI_RESOURCE_TYPE_MEMORY32:u32=9; pub const ACPI_RESOURCE_TYPE_FIXED_MEMORY32:u32=10; pub const ACPI_RESOURCE_TYPE_ADDRESS16:u32=11; pub const ACPI_RESOURCE_TYPE_ADDRESS32:u32=12; pub const ACPI_RESOURCE_TYPE_ADDRESS64:u32=13; pub const ACPI_RESOURCE_TYPE_EXTENDED_ADDRESS64:u32=14; pub const ACPI_RESOURCE_TYPE_EXTENDED_IRQ:u32=15; pub const ACPI_RESOURCE_TYPE_GENERIC_REGISTER:u32=16; pub const ACPI_RESOURCE_TYPE_GPIO:u32=17; pub const ACPI_RESOURCE_TYPE_FIXED_DMA:u32=18; pub const ACPI_RESOURCE_TYPE_SERIAL_BUS:u32=19; pub const ACPI_RESOURCE_TYPE_PIN_FUNCTION:u32=20; pub const ACPI_RESOURCE_TYPE_PIN_CONFIG:u32=21; pub const ACPI_RESOURCE_TYPE_PIN_GROUP:u32=22; pub const ACPI_RESOURCE_TYPE_PIN_GROUP_FUNCTION:u32=23; pub const ACPI_RESOURCE_TYPE_PIN_GROUP_CONFIG:u32=24; pub const ACPI_RESOURCE_TYPE_CLOCK_INPUT:u32=25; pub const ACPI_RESOURCE_TYPE_MAX:u32=25;
#[repr(C, packed)] pub union acpi_resource_data { pub irq:acpi_resource_irq,pub dma:acpi_resource_dma,pub start_dpf:acpi_resource_start_dependent,pub io:acpi_resource_io,pub fixed_io:acpi_resource_fixed_io,pub fixed_dma:acpi_resource_fixed_dma,pub vendor:acpi_resource_vendor,pub vendor_typed:acpi_resource_vendor_typed,pub end_tag:acpi_resource_end_tag,pub memory24:acpi_resource_memory24,pub memory32:acpi_resource_memory32,pub fixed_memory32:acpi_resource_fixed_memory32,pub address16:acpi_resource_address16,pub address32:acpi_resource_address32,pub address64:acpi_resource_address64,pub ext_address64:acpi_resource_extended_address64,pub extended_irq:acpi_resource_extended_irq,pub generic_reg:acpi_resource_generic_register,pub gpio:acpi_resource_gpio,pub i2c_serial_bus:acpi_resource_i2c_serialbus,pub spi_serial_bus:acpi_resource_spi_serialbus,pub uart_serial_bus:acpi_resource_uart_serialbus,pub csi2_serial_bus:acpi_resource_csi2_serialbus,pub common_serial_bus:acpi_resource_common_serialbus,pub pin_function:acpi_resource_pin_function,pub pin_config:acpi_resource_pin_config,pub pin_group:acpi_resource_pin_group,pub pin_group_function:acpi_resource_pin_group_function,pub pin_group_config:acpi_resource_pin_group_config,pub clock_input:acpi_resource_clock_input,pub address:acpi_resource_address }
#[repr(C, packed)] pub struct acpi_resource { pub type_:u32,pub length:u32,pub data:acpi_resource_data }
pub const ACPI_RS_SIZE_NO_DATA:u32=8; pub const ACPI_RS_SIZE_MIN:u32=12;
#[repr(C)] pub union acpi_pci_routing_table_data { pub pad:[i8;4], pub source:[i8;0] }
#[repr(C)] pub struct acpi_pci_routing_table { pub length:u32,pub pin:u32,pub address:u64,pub source_index:u32,pub data:acpi_pci_routing_table_data }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
