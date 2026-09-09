// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: rsmem24 - Memory resource descriptors
 *
 ******************************************************************************/

// Dependencies supplied by the surrounding ACPI translation.

pub static mut acpi_rs_convert_memory24: [acpi_rsconvert_info; 4] = [
    acpi_rsconvert_info {
        source: ACPI_RSC_INITGET,
        destination: ACPI_RESOURCE_TYPE_MEMORY24,
        length: ACPI_RS_SIZE!(acpi_resource_memory24),
        flags: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_memory24),
    },
    acpi_rsconvert_info {
        source: ACPI_RSC_INITSET,
        destination: ACPI_RESOURCE_NAME_MEMORY24,
        length: core::mem::size_of::<aml_resource_memory24>(),
        flags: 0,
    },
    // Read/Write bit
    acpi_rsconvert_info {
        source: ACPI_RSC_1BITFLAG,
        destination: ACPI_RS_OFFSET!(data.memory24.write_protect),
        length: AML_OFFSET!(memory24.flags),
        flags: 0,
    },
    /*
     * These fields are contiguous in both the source and destination:
     * Minimum Base Address
     * Maximum Base Address
     * Address Base Alignment
     * Range Length
     */
    acpi_rsconvert_info {
        source: ACPI_RSC_MOVE16,
        destination: ACPI_RS_OFFSET!(data.memory24.minimum),
        length: AML_OFFSET!(memory24.minimum),
        flags: 4,
    },
];

pub static mut acpi_rs_convert_memory32: [acpi_rsconvert_info; 4] = [
    acpi_rsconvert_info { source: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_MEMORY32, length: ACPI_RS_SIZE!(acpi_resource_memory32), flags: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_memory32) },
    acpi_rsconvert_info { source: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_MEMORY32, length: core::mem::size_of::<aml_resource_memory32>(), flags: 0 },
    // Read/Write bit
    acpi_rsconvert_info { source: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.memory32.write_protect), length: AML_OFFSET!(memory32.flags), flags: 0 },
    /* These fields are contiguous in both the source and destination:
     * Minimum Base Address, Maximum Base Address, Address Base Alignment,
     * Range Length */
    acpi_rsconvert_info { source: ACPI_RSC_MOVE32, destination: ACPI_RS_OFFSET!(data.memory32.minimum), length: AML_OFFSET!(memory32.minimum), flags: 4 },
];

pub static mut acpi_rs_convert_fixed_memory32: [acpi_rsconvert_info; 4] = [
    acpi_rsconvert_info { source: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_FIXED_MEMORY32, length: ACPI_RS_SIZE!(acpi_resource_fixed_memory32), flags: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_fixed_memory32) },
    acpi_rsconvert_info { source: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_FIXED_MEMORY32, length: core::mem::size_of::<aml_resource_fixed_memory32>(), flags: 0 },
    // Read/Write bit
    acpi_rsconvert_info { source: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.fixed_memory32.write_protect), length: AML_OFFSET!(fixed_memory32.flags), flags: 0 },
    /* These fields are contiguous in both the source and destination:
     * Base Address, Range Length */
    acpi_rsconvert_info { source: ACPI_RSC_MOVE32, destination: ACPI_RS_OFFSET!(data.fixed_memory32.address), length: AML_OFFSET!(fixed_memory32.address), flags: 2 },
];

pub static mut acpi_rs_get_vendor_small: [acpi_rsconvert_info; 3] = [
    acpi_rsconvert_info { source: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_VENDOR, length: ACPI_RS_SIZE!(acpi_resource_vendor), flags: ACPI_RSC_TABLE_SIZE!(acpi_rs_get_vendor_small) },
    // Length of the vendor data (byte count)
    acpi_rsconvert_info { source: ACPI_RSC_COUNT16, destination: ACPI_RS_OFFSET!(data.vendor.byte_length), length: 0, flags: core::mem::size_of::<u8>() },
    // Vendor data
    acpi_rsconvert_info { source: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.vendor.byte_data[0]), length: core::mem::size_of::<aml_resource_small_header>(), flags: 0 },
];

pub static mut acpi_rs_get_vendor_large: [acpi_rsconvert_info; 3] = [
    acpi_rsconvert_info { source: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_VENDOR, length: ACPI_RS_SIZE!(acpi_resource_vendor), flags: ACPI_RSC_TABLE_SIZE!(acpi_rs_get_vendor_large) },
    // Length of the vendor data (byte count)
    acpi_rsconvert_info { source: ACPI_RSC_COUNT16, destination: ACPI_RS_OFFSET!(data.vendor.byte_length), length: 0, flags: core::mem::size_of::<u8>() },
    // Vendor data
    acpi_rsconvert_info { source: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.vendor.byte_data[0]), length: core::mem::size_of::<aml_resource_large_header>(), flags: 0 },
];

pub static mut acpi_rs_set_vendor: [acpi_rsconvert_info; 7] = [
    // Default is a small vendor descriptor
    acpi_rsconvert_info { source: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_VENDOR_SMALL, length: core::mem::size_of::<aml_resource_small_header>(), flags: ACPI_RSC_TABLE_SIZE!(acpi_rs_set_vendor) },
    // Get the length and copy the data
    acpi_rsconvert_info { source: ACPI_RSC_COUNT16, destination: ACPI_RS_OFFSET!(data.vendor.byte_length), length: 0, flags: 0 },
    acpi_rsconvert_info { source: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.vendor.byte_data[0]), length: core::mem::size_of::<aml_resource_small_header>(), flags: 0 },
    /* All done if the Vendor byte length is 7 or less, meaning that it will
     * fit within a small descriptor */
    acpi_rsconvert_info { source: ACPI_RSC_EXIT_LE, destination: 0, length: 0, flags: 7 },
    // Must create a large vendor descriptor
    acpi_rsconvert_info { source: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_VENDOR_LARGE, length: core::mem::size_of::<aml_resource_large_header>(), flags: 0 },
    acpi_rsconvert_info { source: ACPI_RSC_COUNT16, destination: ACPI_RS_OFFSET!(data.vendor.byte_length), length: 0, flags: 0 },
    acpi_rsconvert_info { source: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.vendor.byte_data[0]), length: core::mem::size_of::<aml_resource_large_header>(), flags: 0 },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
