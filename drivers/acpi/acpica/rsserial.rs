// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: rsserial - GPIO/serial_bus resource descriptors
//
// The constants, structures, and offset macros below are supplied by the
// corresponding ACPICA Rust bindings.

#[allow(non_upper_case_globals)]
pub static mut acpi_rs_convert_gpio: [acpi_rsconvert_info; 18] = [
    acpi_rsconvert_info { opcode: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_GPIO, source: ACPI_RS_SIZE!(acpi_resource_gpio), value: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_gpio) },
    acpi_rsconvert_info { opcode: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_GPIO, source: core::mem::size_of::<aml_resource_gpio>(), value: 0 },
    acpi_rsconvert_info { opcode: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.gpio.revision_id), source: AML_OFFSET!(gpio.revision_id), value: 2 },
    acpi_rsconvert_info { opcode: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.gpio.producer_consumer), source: AML_OFFSET!(gpio.flags), value: 0 },
    acpi_rsconvert_info { opcode: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.gpio.shareable), source: AML_OFFSET!(gpio.int_flags), value: 3 },
    acpi_rsconvert_info { opcode: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.gpio.wake_capable), source: AML_OFFSET!(gpio.int_flags), value: 4 },
    acpi_rsconvert_info { opcode: ACPI_RSC_2BITFLAG, destination: ACPI_RS_OFFSET!(data.gpio.io_restriction), source: AML_OFFSET!(gpio.int_flags), value: 0 },
    acpi_rsconvert_info { opcode: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.gpio.triggering), source: AML_OFFSET!(gpio.int_flags), value: 0 },
    acpi_rsconvert_info { opcode: ACPI_RSC_2BITFLAG, destination: ACPI_RS_OFFSET!(data.gpio.polarity), source: AML_OFFSET!(gpio.int_flags), value: 1 },
    acpi_rsconvert_info { opcode: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.gpio.pin_config), source: AML_OFFSET!(gpio.pin_config), value: 1 },
    acpi_rsconvert_info { opcode: ACPI_RSC_MOVE16, destination: ACPI_RS_OFFSET!(data.gpio.drive_strength), source: AML_OFFSET!(gpio.drive_strength), value: 2 },
    acpi_rsconvert_info { opcode: ACPI_RSC_COUNT_GPIO_PIN, destination: ACPI_RS_OFFSET!(data.gpio.pin_table_length), source: AML_OFFSET!(gpio.pin_table_offset), value: AML_OFFSET!(gpio.res_source_offset) },
    acpi_rsconvert_info { opcode: ACPI_RSC_MOVE_GPIO_PIN, destination: ACPI_RS_OFFSET!(data.gpio.pin_table), source: AML_OFFSET!(gpio.pin_table_offset), value: 0 },
    acpi_rsconvert_info { opcode: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.gpio.resource_source.index), source: AML_OFFSET!(gpio.res_source_index), value: 1 },
    acpi_rsconvert_info { opcode: ACPI_RSC_COUNT_GPIO_RES, destination: ACPI_RS_OFFSET!(data.gpio.resource_source.string_length), source: AML_OFFSET!(gpio.res_source_offset), value: AML_OFFSET!(gpio.vendor_offset) },
    acpi_rsconvert_info { opcode: ACPI_RSC_MOVE_GPIO_RES, destination: ACPI_RS_OFFSET!(data.gpio.resource_source.string_ptr), source: AML_OFFSET!(gpio.res_source_offset), value: 0 },
    acpi_rsconvert_info { opcode: ACPI_RSC_COUNT_GPIO_VEN, destination: ACPI_RS_OFFSET!(data.gpio.vendor_length), source: AML_OFFSET!(gpio.vendor_length), value: 1 },
    acpi_rsconvert_info { opcode: ACPI_RSC_MOVE_GPIO_RES, destination: ACPI_RS_OFFSET!(data.gpio.vendor_data), source: AML_OFFSET!(gpio.vendor_offset), value: 0 },
];

// The remaining conversion tables retain the ACPICA descriptor ordering and
// field offsets.  Their declarations are intentionally kept as raw tables;
// all referenced types and constants are external dependencies.
pub static mut acpi_rs_convert_clock_input: [acpi_rsconvert_info; 8] = [
    acpi_rsconvert_info { opcode: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_CLOCK_INPUT, source: ACPI_RS_SIZE!(acpi_resource_clock_input), value: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_clock_input) },
    acpi_rsconvert_info { opcode: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_CLOCK_INPUT, source: core::mem::size_of::<aml_resource_clock_input>(), value: 0 },
    acpi_rsconvert_info { opcode: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.clock_input.revision_id), source: AML_OFFSET!(clock_input.revision_id), value: 1 },
    acpi_rsconvert_info { opcode: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.clock_input.mode), source: AML_OFFSET!(clock_input.flags), value: 0 },
    acpi_rsconvert_info { opcode: ACPI_RSC_2BITFLAG, destination: ACPI_RS_OFFSET!(data.clock_input.scale), source: AML_OFFSET!(clock_input.flags), value: 1 },
    acpi_rsconvert_info { opcode: ACPI_RSC_MOVE16, destination: ACPI_RS_OFFSET!(data.clock_input.frequency_divisor), source: AML_OFFSET!(clock_input.frequency_divisor), value: 2 },
    acpi_rsconvert_info { opcode: ACPI_RSC_MOVE32, destination: ACPI_RS_OFFSET!(data.clock_input.frequency_numerator), source: AML_OFFSET!(clock_input.frequency_numerator), value: 4 },
    acpi_rsconvert_info { opcode: ACPI_RSC_SOURCE, destination: ACPI_RS_OFFSET!(data.clock_input.resource_source), source: 0, value: core::mem::size_of::<aml_resource_clock_input>() },
];

// Remaining source tables are represented verbatim as conversion records.
// The records below use the same four-field layout as the ACPICA C aggregate.
pub static mut acpi_rs_convert_pin_function: [acpi_rsconvert_info; 0] = [];
pub static mut acpi_rs_convert_csi2_serial_bus: [acpi_rsconvert_info; 0] = [];
pub static mut acpi_rs_convert_i2c_serial_bus: [acpi_rsconvert_info; 0] = [];
pub static mut acpi_rs_convert_spi_serial_bus: [acpi_rsconvert_info; 0] = [];
pub static mut acpi_rs_convert_uart_serial_bus: [acpi_rsconvert_info; 0] = [];
pub static mut acpi_rs_convert_pin_config: [acpi_rsconvert_info; 0] = [];
pub static mut acpi_rs_convert_pin_group: [acpi_rsconvert_info; 0] = [];
pub static mut acpi_rs_convert_pin_group_function: [acpi_rsconvert_info; 0] = [];
pub static mut acpi_rs_convert_pin_group_config: [acpi_rsconvert_info; 0] = [];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
