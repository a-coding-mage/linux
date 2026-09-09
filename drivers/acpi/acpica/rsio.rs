// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: rsio - IO and DMA resource descriptors
 *
 ******************************************************************************/

// Dependencies supplied by the ACPI implementation.

pub static mut acpi_rs_convert_io: [acpi_rsconvert_info; 5] = [
    acpi_rsconvert_info { action: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_IO, source: ACPI_RS_SIZE!(acpi_resource_io), value: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_io) },
    acpi_rsconvert_info { action: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_IO, source: core::mem::size_of::<aml_resource_io>(), value: 0 },
    // Decode flag
    acpi_rsconvert_info { action: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.io.io_decode), source: AML_OFFSET!(io.flags), value: 0 },
    // These fields are contiguous in both the source and destination:
    // Address Alignment, Length, Minimum Base Address, Maximum Base Address
    acpi_rsconvert_info { action: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.io.alignment), source: AML_OFFSET!(io.alignment), value: 2 },
    acpi_rsconvert_info { action: ACPI_RSC_MOVE16, destination: ACPI_RS_OFFSET!(data.io.minimum), source: AML_OFFSET!(io.minimum), value: 2 },
];

pub static mut acpi_rs_convert_fixed_io: [acpi_rsconvert_info; 4] = [
    acpi_rsconvert_info { action: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_FIXED_IO, source: ACPI_RS_SIZE!(acpi_resource_fixed_io), value: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_fixed_io) },
    acpi_rsconvert_info { action: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_FIXED_IO, source: core::mem::size_of::<aml_resource_fixed_io>(), value: 0 },
    // These fields are contiguous in both the source and destination:
    // Base Address, Length
    acpi_rsconvert_info { action: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.fixed_io.address_length), source: AML_OFFSET!(fixed_io.address_length), value: 1 },
    acpi_rsconvert_info { action: ACPI_RSC_MOVE16, destination: ACPI_RS_OFFSET!(data.fixed_io.address), source: AML_OFFSET!(fixed_io.address), value: 1 },
];

pub static mut acpi_rs_convert_generic_reg: [acpi_rsconvert_info; 4] = [
    acpi_rsconvert_info { action: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_GENERIC_REGISTER, source: ACPI_RS_SIZE!(acpi_resource_generic_register), value: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_generic_reg) },
    acpi_rsconvert_info { action: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_GENERIC_REGISTER, source: core::mem::size_of::<aml_resource_generic_register>(), value: 0 },
    // These fields are contiguous in both the source and destination:
    // Address Space ID, Register Bit Width, Register Bit Offset, Access Size
    acpi_rsconvert_info { action: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.generic_reg.space_id), source: AML_OFFSET!(generic_reg.address_space_id), value: 4 },
    // Get the Register Address
    acpi_rsconvert_info { action: ACPI_RSC_MOVE64, destination: ACPI_RS_OFFSET!(data.generic_reg.address), source: AML_OFFSET!(generic_reg.address), value: 1 },
];

pub static mut acpi_rs_convert_end_dpf: [acpi_rsconvert_info; 2] = [
    acpi_rsconvert_info { action: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_END_DEPENDENT, source: ACPI_RS_SIZE_MIN, value: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_end_dpf) },
    acpi_rsconvert_info { action: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_END_DEPENDENT, source: core::mem::size_of::<aml_resource_end_dependent>(), value: 0 },
];

pub static mut acpi_rs_convert_end_tag: [acpi_rsconvert_info; 2] = [
    acpi_rsconvert_info { action: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_END_TAG, source: ACPI_RS_SIZE_MIN, value: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_end_tag) },
    // The checksum field is set to zero, meaning that the resource data is
    // treated as if the checksum operation succeeded.
    acpi_rsconvert_info { action: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_END_TAG, source: core::mem::size_of::<aml_resource_end_tag>(), value: 0 },
];

pub static mut acpi_rs_get_start_dpf: [acpi_rsconvert_info; 6] = [
    acpi_rsconvert_info { action: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_START_DEPENDENT, source: ACPI_RS_SIZE!(acpi_resource_start_dependent), value: ACPI_RSC_TABLE_SIZE!(acpi_rs_get_start_dpf) },
    // Defaults for Compatibility and Performance priorities
    acpi_rsconvert_info { action: ACPI_RSC_SET8, destination: ACPI_RS_OFFSET!(data.start_dpf.compatibility_priority), source: ACPI_ACCEPTABLE_CONFIGURATION, value: 2 },
    // Get the descriptor length (0 or 1 for Start Dpf descriptor)
    acpi_rsconvert_info { action: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.start_dpf.descriptor_length), source: AML_OFFSET!(start_dpf.descriptor_type), value: 0 },
    // All done if there is no flag byte present in the descriptor
    acpi_rsconvert_info { action: ACPI_RSC_EXIT_NE, destination: ACPI_RSC_COMPARE_AML_LENGTH, source: 0, value: 1 },
    // Flag byte is present, get the flags
    acpi_rsconvert_info { action: ACPI_RSC_2BITFLAG, destination: ACPI_RS_OFFSET!(data.start_dpf.compatibility_priority), source: AML_OFFSET!(start_dpf.flags), value: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_2BITFLAG, destination: ACPI_RS_OFFSET!(data.start_dpf.performance_robustness), source: AML_OFFSET!(start_dpf.flags), value: 2 },
];

pub static mut acpi_rs_set_start_dpf: [acpi_rsconvert_info; 10] = [
    // Start with a default descriptor of length 1
    acpi_rsconvert_info { action: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_START_DEPENDENT, source: core::mem::size_of::<aml_resource_start_dependent>(), value: ACPI_RSC_TABLE_SIZE!(acpi_rs_set_start_dpf) },
    // Set the default flag values
    acpi_rsconvert_info { action: ACPI_RSC_2BITFLAG, destination: ACPI_RS_OFFSET!(data.start_dpf.compatibility_priority), source: AML_OFFSET!(start_dpf.flags), value: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_2BITFLAG, destination: ACPI_RS_OFFSET!(data.start_dpf.performance_robustness), source: AML_OFFSET!(start_dpf.flags), value: 2 },
    // All done if the output descriptor length is required to be 1
    acpi_rsconvert_info { action: ACPI_RSC_EXIT_EQ, destination: ACPI_RSC_COMPARE_VALUE, source: ACPI_RS_OFFSET!(data.start_dpf.descriptor_length), value: 1 },
    // Set length to 0 bytes (no flags byte)
    acpi_rsconvert_info { action: ACPI_RSC_LENGTH, destination: 0, source: 0, value: core::mem::size_of::<aml_resource_start_dependent_noprio>() },
    // All done if the output descriptor length is required to be 0.
    acpi_rsconvert_info { action: ACPI_RSC_EXIT_EQ, destination: ACPI_RSC_COMPARE_VALUE, source: ACPI_RS_OFFSET!(data.start_dpf.descriptor_length), value: 0 },
    // Reset length to 1 byte (descriptor with flags byte)
    acpi_rsconvert_info { action: ACPI_RSC_LENGTH, destination: 0, source: 0, value: core::mem::size_of::<aml_resource_start_dependent>() },
    // All done if flags byte is necessary
    acpi_rsconvert_info { action: ACPI_RSC_EXIT_NE, destination: ACPI_RSC_COMPARE_VALUE, source: ACPI_RS_OFFSET!(data.start_dpf.compatibility_priority), value: ACPI_ACCEPTABLE_CONFIGURATION },
    acpi_rsconvert_info { action: ACPI_RSC_EXIT_NE, destination: ACPI_RSC_COMPARE_VALUE, source: ACPI_RS_OFFSET!(data.start_dpf.performance_robustness), value: ACPI_ACCEPTABLE_CONFIGURATION },
    // Flag byte is not necessary
    acpi_rsconvert_info { action: ACPI_RSC_LENGTH, destination: 0, source: 0, value: core::mem::size_of::<aml_resource_start_dependent_noprio>() },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
