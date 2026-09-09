// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: rsaddr - Address resource descriptors (16/32/64)
 *
 ******************************************************************************/

// Dependencies supplied by the surrounding ACPICA translation unit:
// acpi/acpi.h, accommon.h, acresrc.h

// #define _COMPONENT ACPI_RESOURCES
// ACPI_MODULE_NAME("rsaddr")

/*******************************************************************************
 *
 * acpi_rs_convert_address16 - All WORD (16-bit) address resources
 *
 ******************************************************************************/
static mut acpi_rs_convert_address16: [acpi_rsconvert_info; 5] = [
    acpi_rsconvert_info { action: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_ADDRESS16, source: ACPI_RS_SIZE::<acpi_resource_address16>(), count: ACPI_RSC_TABLE_SIZE(acpi_rs_convert_address16) },
    acpi_rsconvert_info { action: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_ADDRESS16, source: core::mem::size_of::<aml_resource_address16>(), count: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_ADDRESS, destination: 0, source: 0, count: 0 },
    // These fields are contiguous in both the source and destination:
    // Address Granularity, Address Range Minimum, Address Range Maximum,
    // Address Translation Offset, Address Length
    acpi_rsconvert_info { action: ACPI_RSC_MOVE16, destination: ACPI_RS_OFFSET!(data.address16.address.granularity), source: AML_OFFSET!(address16.granularity), count: 5 },
    // Optional resource_source (Index and String)
    acpi_rsconvert_info { action: ACPI_RSC_SOURCE, destination: ACPI_RS_OFFSET!(data.address16.resource_source), source: 0, count: core::mem::size_of::<aml_resource_address16>() },
];

/*******************************************************************************
 * acpi_rs_convert_address32 - All DWORD (32-bit) address resources
 ******************************************************************************/
static mut acpi_rs_convert_address32: [acpi_rsconvert_info; 5] = [
    acpi_rsconvert_info { action: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_ADDRESS32, source: ACPI_RS_SIZE::<acpi_resource_address32>(), count: ACPI_RSC_TABLE_SIZE(acpi_rs_convert_address32) },
    acpi_rsconvert_info { action: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_ADDRESS32, source: core::mem::size_of::<aml_resource_address32>(), count: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_ADDRESS, destination: 0, source: 0, count: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_MOVE32, destination: ACPI_RS_OFFSET!(data.address32.address.granularity), source: AML_OFFSET!(address32.granularity), count: 5 },
    acpi_rsconvert_info { action: ACPI_RSC_SOURCE, destination: ACPI_RS_OFFSET!(data.address32.resource_source), source: 0, count: core::mem::size_of::<aml_resource_address32>() },
];

/*******************************************************************************
 * acpi_rs_convert_address64 - All QWORD (64-bit) address resources
 ******************************************************************************/
static mut acpi_rs_convert_address64: [acpi_rsconvert_info; 5] = [
    acpi_rsconvert_info { action: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_ADDRESS64, source: ACPI_RS_SIZE::<acpi_resource_address64>(), count: ACPI_RSC_TABLE_SIZE(acpi_rs_convert_address64) },
    acpi_rsconvert_info { action: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_ADDRESS64, source: core::mem::size_of::<aml_resource_address64>(), count: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_ADDRESS, destination: 0, source: 0, count: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_MOVE64, destination: ACPI_RS_OFFSET!(data.address64.address.granularity), source: AML_OFFSET!(address64.granularity), count: 5 },
    acpi_rsconvert_info { action: ACPI_RSC_SOURCE, destination: ACPI_RS_OFFSET!(data.address64.resource_source), source: 0, count: core::mem::size_of::<aml_resource_address64>() },
];

/*******************************************************************************
 * acpi_rs_convert_ext_address64 - All Extended (64-bit) address resources
 ******************************************************************************/
static mut acpi_rs_convert_ext_address64: [acpi_rsconvert_info; 5] = [
    acpi_rsconvert_info { action: ACPI_RSC_INITGET, destination: ACPI_RESOURCE_TYPE_EXTENDED_ADDRESS64, source: ACPI_RS_SIZE::<acpi_resource_extended_address64>(), count: ACPI_RSC_TABLE_SIZE(acpi_rs_convert_ext_address64) },
    acpi_rsconvert_info { action: ACPI_RSC_INITSET, destination: ACPI_RESOURCE_NAME_EXTENDED_ADDRESS64, source: core::mem::size_of::<aml_resource_extended_address64>(), count: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_ADDRESS, destination: 0, source: 0, count: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.ext_address64.revision_ID), source: AML_OFFSET!(ext_address64.revision_ID), count: 1 },
    // These fields are contiguous in both the source and destination:
    // Address Granularity, Address Range Minimum, Address Range Maximum,
    // Address Translation Offset, Address Length, Type-Specific Attribute
    acpi_rsconvert_info { action: ACPI_RSC_MOVE64, destination: ACPI_RS_OFFSET!(data.ext_address64.address.granularity), source: AML_OFFSET!(ext_address64.granularity), count: 6 },
];

static mut acpi_rs_convert_general_flags: [acpi_rsconvert_info; 6] = [
    acpi_rsconvert_info { action: ACPI_RSC_FLAGINIT, destination: 0, source: AML_OFFSET!(address.flags), count: ACPI_RSC_TABLE_SIZE(acpi_rs_convert_general_flags) },
    acpi_rsconvert_info { action: ACPI_RSC_MOVE8, destination: ACPI_RS_OFFSET!(data.address.resource_type), source: AML_OFFSET!(address.resource_type), count: 1 },
    acpi_rsconvert_info { action: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.address.producer_consumer), source: AML_OFFSET!(address.flags), count: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.address.decode), source: AML_OFFSET!(address.flags), count: 1 },
    acpi_rsconvert_info { action: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.address.min_address_fixed), source: AML_OFFSET!(address.flags), count: 2 },
    acpi_rsconvert_info { action: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.address.max_address_fixed), source: AML_OFFSET!(address.flags), count: 3 },
];

static mut acpi_rs_convert_mem_flags: [acpi_rsconvert_info; 5] = [
    acpi_rsconvert_info { action: ACPI_RSC_FLAGINIT, destination: 0, source: AML_OFFSET!(address.specific_flags), count: ACPI_RSC_TABLE_SIZE(acpi_rs_convert_mem_flags) },
    acpi_rsconvert_info { action: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.address.info.mem.write_protect), source: AML_OFFSET!(address.specific_flags), count: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_2BITFLAG, destination: ACPI_RS_OFFSET!(data.address.info.mem.caching), source: AML_OFFSET!(address.specific_flags), count: 1 },
    acpi_rsconvert_info { action: ACPI_RSC_2BITFLAG, destination: ACPI_RS_OFFSET!(data.address.info.mem.range_type), source: AML_OFFSET!(address.specific_flags), count: 3 },
    acpi_rsconvert_info { action: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.address.info.mem.translation), source: AML_OFFSET!(address.specific_flags), count: 5 },
];

static mut acpi_rs_convert_io_flags: [acpi_rsconvert_info; 4] = [
    acpi_rsconvert_info { action: ACPI_RSC_FLAGINIT, destination: 0, source: AML_OFFSET!(address.specific_flags), count: ACPI_RSC_TABLE_SIZE(acpi_rs_convert_io_flags) },
    acpi_rsconvert_info { action: ACPI_RSC_2BITFLAG, destination: ACPI_RS_OFFSET!(data.address.info.io.range_type), source: AML_OFFSET!(address.specific_flags), count: 0 },
    acpi_rsconvert_info { action: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.address.info.io.translation), source: AML_OFFSET!(address.specific_flags), count: 4 },
    acpi_rsconvert_info { action: ACPI_RSC_1BITFLAG, destination: ACPI_RS_OFFSET!(data.address.info.io.translation_type), source: AML_OFFSET!(address.specific_flags), count: 5 },
];

pub unsafe fn acpi_rs_get_address_common(resource: *mut acpi_resource, aml: *mut aml_resource) -> u8 {
    ACPI_FUNCTION_ENTRY!();
    if ((*aml).address.resource_type > 2) && ((*aml).address.resource_type < 0xC0) && ((*aml).address.resource_type != 0x0A) { return FALSE; }
    let _ = acpi_rs_convert_aml_to_resource(resource, aml, acpi_rs_convert_general_flags.as_mut_ptr());
    if (*resource).data.address.resource_type == ACPI_MEMORY_RANGE {
        let _ = acpi_rs_convert_aml_to_resource(resource, aml, acpi_rs_convert_mem_flags.as_mut_ptr());
    } else if (*resource).data.address.resource_type == ACPI_IO_RANGE {
        let _ = acpi_rs_convert_aml_to_resource(resource, aml, acpi_rs_convert_io_flags.as_mut_ptr());
    } else {
        (*resource).data.address.info.type_specific = (*aml).address.specific_flags;
    }
    TRUE
}

pub unsafe fn acpi_rs_set_address_common(aml: *mut aml_resource, resource: *mut acpi_resource) {
    ACPI_FUNCTION_ENTRY!();
    let _ = acpi_rs_convert_resource_to_aml(resource, aml, acpi_rs_convert_general_flags.as_mut_ptr());
    if (*resource).data.address.resource_type == ACPI_MEMORY_RANGE {
        let _ = acpi_rs_convert_resource_to_aml(resource, aml, acpi_rs_convert_mem_flags.as_mut_ptr());
    } else if (*resource).data.address.resource_type == ACPI_IO_RANGE {
        let _ = acpi_rs_convert_resource_to_aml(resource, aml, acpi_rs_convert_io_flags.as_mut_ptr());
    } else {
        (*aml).address.specific_flags = (*resource).data.address.info.type_specific;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
