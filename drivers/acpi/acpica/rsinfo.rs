// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: rsinfo - Dispatch and Info tables
 *
 ******************************************************************************/

// Dependencies supplied by the ACPI headers and resource-conversion units.

/*
 * Resource dispatch and information tables. Any new resource types (either
 * Large or Small) must be reflected in each of these tables, so they are here
 * in one place.
 *
 * The tables for Large descriptors are indexed by bits 6:0 of the AML
 * descriptor type byte. The tables for Small descriptors are indexed by
 * bits 6:3 of the descriptor byte. The tables for internal resource
 * descriptors are indexed by the acpi_resource_type field.
 */
/* Dispatch table for resource-to-AML (Set Resource) conversion functions */
pub static mut acpi_gbl_set_resource_dispatch: [*mut acpi_rsconvert_info; 0x1a] = [
    acpi_rs_set_irq,
    acpi_rs_convert_dma,
    acpi_rs_set_start_dpf,
    acpi_rs_convert_end_dpf,
    acpi_rs_convert_io,
    acpi_rs_convert_fixed_io,
    acpi_rs_set_vendor,
    acpi_rs_convert_end_tag,
    acpi_rs_convert_memory24,
    acpi_rs_convert_memory32,
    acpi_rs_convert_fixed_memory32,
    acpi_rs_convert_address16,
    acpi_rs_convert_address32,
    acpi_rs_convert_address64,
    acpi_rs_convert_ext_address64,
    acpi_rs_convert_ext_irq,
    acpi_rs_convert_generic_reg,
    acpi_rs_convert_gpio,
    acpi_rs_convert_fixed_dma,
    core::ptr::null_mut(),
    acpi_rs_convert_pin_function,
    acpi_rs_convert_pin_config,
    acpi_rs_convert_pin_group,
    acpi_rs_convert_pin_group_function,
    acpi_rs_convert_pin_group_config,
    acpi_rs_convert_clock_input,
];

/* Dispatch tables for AML-to-resource (Get Resource) conversion functions */
pub static mut acpi_gbl_get_resource_dispatch: [*mut acpi_rsconvert_info; 0x24] = [
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    acpi_rs_get_irq, acpi_rs_convert_dma, acpi_rs_get_start_dpf, acpi_rs_convert_end_dpf,
    acpi_rs_convert_io, acpi_rs_convert_fixed_io, acpi_rs_convert_fixed_dma,
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    acpi_rs_get_vendor_small, acpi_rs_convert_end_tag,
    core::ptr::null_mut(), acpi_rs_convert_memory24, acpi_rs_convert_generic_reg,
    core::ptr::null_mut(), acpi_rs_get_vendor_large, acpi_rs_convert_memory32,
    acpi_rs_convert_fixed_memory32, acpi_rs_convert_address32, acpi_rs_convert_address16,
    acpi_rs_convert_ext_irq, acpi_rs_convert_address64, acpi_rs_convert_ext_address64,
    acpi_rs_convert_gpio, acpi_rs_convert_pin_function, core::ptr::null_mut(),
    acpi_rs_convert_pin_config, acpi_rs_convert_pin_group, acpi_rs_convert_pin_group_function,
    acpi_rs_convert_pin_group_config, acpi_rs_convert_clock_input,
];

/* Subtype table for serial_bus -- I2C, SPI, UART, and CSI2 */
pub static mut acpi_gbl_convert_resource_serial_bus_dispatch: [*mut acpi_rsconvert_info; 5] = [
    core::ptr::null_mut(),
    acpi_rs_convert_i2c_serial_bus,
    acpi_rs_convert_spi_serial_bus,
    acpi_rs_convert_uart_serial_bus,
    acpi_rs_convert_csi2_serial_bus,
];

// Preserved build-time condition: enable these tables when ACPI_DEBUG_OUTPUT,
// ACPI_DISASSEMBLER, or ACPI_DEBUGGER is defined.
#[cfg(any(feature = "ACPI_DEBUG_OUTPUT", feature = "ACPI_DISASSEMBLER", feature = "ACPI_DEBUGGER"))]
pub static mut acpi_gbl_dump_resource_dispatch: [*mut acpi_rsdump_info; 0x1a] = [
    acpi_rs_dump_irq, acpi_rs_dump_dma, acpi_rs_dump_start_dpf, acpi_rs_dump_end_dpf,
    acpi_rs_dump_io, acpi_rs_dump_fixed_io, acpi_rs_dump_vendor, acpi_rs_dump_end_tag,
    acpi_rs_dump_memory24, acpi_rs_dump_memory32, acpi_rs_dump_fixed_memory32,
    acpi_rs_dump_address16, acpi_rs_dump_address32, acpi_rs_dump_address64,
    acpi_rs_dump_ext_address64, acpi_rs_dump_ext_irq, acpi_rs_dump_generic_reg,
    acpi_rs_dump_gpio, acpi_rs_dump_fixed_dma, core::ptr::null_mut(),
    acpi_rs_dump_pin_function, acpi_rs_dump_pin_config, acpi_rs_dump_pin_group,
    acpi_rs_dump_pin_group_function, acpi_rs_dump_pin_group_config, acpi_rs_dump_clock_input,
];

#[cfg(any(feature = "ACPI_DEBUG_OUTPUT", feature = "ACPI_DISASSEMBLER", feature = "ACPI_DEBUGGER"))]
pub static mut acpi_gbl_dump_serial_bus_dispatch: [*mut acpi_rsdump_info; 5] = [
    core::ptr::null_mut(),
    acpi_rs_dump_i2c_serial_bus,
    acpi_rs_dump_spi_serial_bus,
    acpi_rs_dump_uart_serial_bus,
    acpi_rs_dump_csi2_serial_bus,
];

/*
 * Base sizes for external AML resource descriptors, indexed by internal type.
 * Includes size of the descriptor header (1 byte for small descriptors,
 * 3 bytes for large descriptors)
 */
pub const acpi_gbl_aml_resource_sizes: &[u8] = &[
    core::mem::size_of::<aml_resource_irq>() as u8,
    core::mem::size_of::<aml_resource_dma>() as u8,
    core::mem::size_of::<aml_resource_start_dependent>() as u8,
    core::mem::size_of::<aml_resource_end_dependent>() as u8,
    core::mem::size_of::<aml_resource_io>() as u8,
    core::mem::size_of::<aml_resource_fixed_io>() as u8,
    core::mem::size_of::<aml_resource_vendor_small>() as u8,
    core::mem::size_of::<aml_resource_end_tag>() as u8,
    core::mem::size_of::<aml_resource_memory24>() as u8,
    core::mem::size_of::<aml_resource_memory32>() as u8,
    core::mem::size_of::<aml_resource_fixed_memory32>() as u8,
    core::mem::size_of::<aml_resource_address16>() as u8,
    core::mem::size_of::<aml_resource_address32>() as u8,
    core::mem::size_of::<aml_resource_address64>() as u8,
    core::mem::size_of::<aml_resource_extended_address64>() as u8,
    core::mem::size_of::<aml_resource_extended_irq>() as u8,
    core::mem::size_of::<aml_resource_generic_register>() as u8,
    core::mem::size_of::<aml_resource_gpio>() as u8,
    core::mem::size_of::<aml_resource_fixed_dma>() as u8,
    core::mem::size_of::<aml_resource_common_serialbus>() as u8,
    core::mem::size_of::<aml_resource_pin_function>() as u8,
    core::mem::size_of::<aml_resource_pin_config>() as u8,
    core::mem::size_of::<aml_resource_pin_group>() as u8,
    core::mem::size_of::<aml_resource_pin_group_function>() as u8,
    core::mem::size_of::<aml_resource_pin_group_config>() as u8,
    core::mem::size_of::<aml_resource_clock_input>() as u8,
];

pub const acpi_gbl_resource_struct_sizes: &[u8] = &[
    0, 0, 0, 0, ACPI_RS_SIZE!(acpi_resource_irq), ACPI_RS_SIZE!(acpi_resource_dma),
    ACPI_RS_SIZE!(acpi_resource_start_dependent), ACPI_RS_SIZE_MIN, ACPI_RS_SIZE!(acpi_resource_io),
    ACPI_RS_SIZE!(acpi_resource_fixed_io), ACPI_RS_SIZE!(acpi_resource_fixed_dma), 0, 0, 0,
    ACPI_RS_SIZE!(acpi_resource_vendor), ACPI_RS_SIZE_MIN,
    0, ACPI_RS_SIZE!(acpi_resource_memory24), ACPI_RS_SIZE!(acpi_resource_generic_register), 0,
    ACPI_RS_SIZE!(acpi_resource_vendor), ACPI_RS_SIZE!(acpi_resource_memory32),
    ACPI_RS_SIZE!(acpi_resource_fixed_memory32), ACPI_RS_SIZE!(acpi_resource_address32),
    ACPI_RS_SIZE!(acpi_resource_address16), ACPI_RS_SIZE!(acpi_resource_extended_irq),
    ACPI_RS_SIZE!(acpi_resource_address64), ACPI_RS_SIZE!(acpi_resource_extended_address64),
    ACPI_RS_SIZE!(acpi_resource_gpio), ACPI_RS_SIZE!(acpi_resource_pin_function),
    ACPI_RS_SIZE!(acpi_resource_common_serialbus), ACPI_RS_SIZE!(acpi_resource_pin_config),
    ACPI_RS_SIZE!(acpi_resource_pin_group), ACPI_RS_SIZE!(acpi_resource_pin_group_function),
    ACPI_RS_SIZE!(acpi_resource_pin_group_config), ACPI_RS_SIZE!(acpi_resource_clock_input),
];

pub const acpi_gbl_aml_resource_serial_bus_sizes: &[u8] = &[
    0,
    core::mem::size_of::<aml_resource_i2c_serialbus>() as u8,
    core::mem::size_of::<aml_resource_spi_serialbus>() as u8,
    core::mem::size_of::<aml_resource_uart_serialbus>() as u8,
    core::mem::size_of::<aml_resource_csi2_serialbus>() as u8,
];

pub const acpi_gbl_resource_struct_serial_bus_sizes: &[u8] = &[
    0,
    ACPI_RS_SIZE!(acpi_resource_i2c_serialbus),
    ACPI_RS_SIZE!(acpi_resource_spi_serialbus),
    ACPI_RS_SIZE!(acpi_resource_uart_serialbus),
    ACPI_RS_SIZE!(acpi_resource_csi2_serialbus),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
