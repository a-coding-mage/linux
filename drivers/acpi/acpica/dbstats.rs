// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: dbstats - Generation and display of ACPI table statistics

// Dependencies supplied by the ACPI headers and other translation units.

const CMD_STAT_ALLOCATIONS: u32 = 0;
const CMD_STAT_OBJECTS: u32 = 1;
const CMD_STAT_MEMORY: u32 = 2;
const CMD_STAT_MISC: u32 = 3;
const CMD_STAT_TABLES: u32 = 4;
const CMD_STAT_SIZES: u32 = 5;
const CMD_STAT_STACK: u32 = 6;

// Statistics subcommands (the final null entry is represented by the option
// list terminator used by the surrounding ACPI debugger implementation).
static ACPI_DB_STAT_TYPES: [&str; 7] = [
    "ALLOCATIONS", "OBJECTS", "MEMORY", "MISC", "TABLES", "SIZES", "STACK",
];

unsafe fn acpi_db_list_info(list: *mut acpi_memory_list) {
    #[cfg(feature = "ACPI_DBG_TRACK_ALLOCATIONS")]
    let outstanding: u32;

    acpi_os_printf("\n%s\n", (*list).list_name);
    if (*list).max_depth > 0 {
        acpi_os_printf("    Cache: [Depth    MaxD Avail  Size]                %8.2X %8.2X %8.2X %8.2X\n",
            (*list).current_depth, (*list).max_depth,
            (*list).max_depth - (*list).current_depth,
            (*list).current_depth * (*list).object_size);
    }
    #[cfg(feature = "ACPI_DBG_TRACK_ALLOCATIONS")]
    {
        if (*list).max_depth > 0 {
            acpi_os_printf("    Cache: [Requests Hits Misses ObjSize]             %8.2X %8.2X %8.2X %8.2X\n",
                (*list).requests, (*list).hits, (*list).requests - (*list).hits,
                (*list).object_size);
        }
        outstanding = acpi_db_get_cache_info(list);
        if (*list).object_size != 0 {
            acpi_os_printf("    Mem:   [Alloc    Free Max    CurSize Outstanding] %8.2X %8.2X %8.2X %8.2X %8.2X\n",
                (*list).total_allocated, (*list).total_freed, (*list).max_occupied,
                outstanding * (*list).object_size, outstanding);
        } else {
            acpi_os_printf("    Mem:   [Alloc Free Max CurSize Outstanding Total] %8.2X %8.2X %8.2X %8.2X %8.2X %8.2X\n",
                (*list).total_allocated, (*list).total_freed, (*list).max_occupied,
                (*list).current_total_size, outstanding, (*list).total_size);
        }
    }
}

unsafe fn acpi_db_enumerate_object(obj_desc: *mut acpi_operand_object) {
    if obj_desc.is_null() { return; }
    acpi_gbl_num_objects += 1;
    if (*obj_desc).common.type_ > ACPI_TYPE_NS_NODE_MAX {
        acpi_gbl_obj_type_count_misc += 1;
    } else {
        acpi_gbl_obj_type_count[(*obj_desc).common.type_ as usize] += 1;
    }
    match (*obj_desc).common.type_ {
        ACPI_TYPE_PACKAGE => for i in 0..(*obj_desc).package.count {
            acpi_db_enumerate_object((*obj_desc).package.elements[i as usize]);
        },
        ACPI_TYPE_DEVICE => {
            acpi_db_enumerate_object((*obj_desc).device.notify_list[0]);
            acpi_db_enumerate_object((*obj_desc).device.notify_list[1]);
            acpi_db_enumerate_object((*obj_desc).device.handler);
        },
        ACPI_TYPE_BUFFER_FIELD => if !acpi_ns_get_secondary_object(obj_desc).is_null() {
            acpi_gbl_obj_type_count[ACPI_TYPE_BUFFER_FIELD as usize] += 1;
        },
        ACPI_TYPE_REGION => {
            acpi_gbl_obj_type_count[ACPI_TYPE_LOCAL_REGION_FIELD as usize] += 1;
            acpi_db_enumerate_object((*obj_desc).region.handler);
        },
        ACPI_TYPE_POWER => {
            acpi_db_enumerate_object((*obj_desc).power_resource.notify_list[0]);
            acpi_db_enumerate_object((*obj_desc).power_resource.notify_list[1]);
        },
        ACPI_TYPE_PROCESSOR => {
            acpi_db_enumerate_object((*obj_desc).processor.notify_list[0]);
            acpi_db_enumerate_object((*obj_desc).processor.notify_list[1]);
            acpi_db_enumerate_object((*obj_desc).processor.handler);
        },
        ACPI_TYPE_THERMAL => {
            acpi_db_enumerate_object((*obj_desc).thermal_zone.notify_list[0]);
            acpi_db_enumerate_object((*obj_desc).thermal_zone.notify_list[1]);
            acpi_db_enumerate_object((*obj_desc).thermal_zone.handler);
        },
        _ => {}
    }
}

unsafe extern "C" fn acpi_db_classify_one_object(obj_handle: acpi_handle, _nesting_level: u32,
    _context: *mut core::ffi::c_void, _return_value: *mut *mut core::ffi::c_void) -> acpi_status {
    acpi_gbl_num_nodes += 1;
    let node = obj_handle as *mut acpi_namespace_node;
    acpi_db_enumerate_object(acpi_ns_get_attached_object(node));
    let type_ = (*node).type_;
    if type_ > ACPI_TYPE_NS_NODE_MAX { acpi_gbl_node_type_count_misc += 1; }
    else { acpi_gbl_node_type_count[type_ as usize] += 1; }
    AE_OK
}

unsafe fn acpi_db_count_namespace_objects() {
    acpi_gbl_num_nodes = 0;
    acpi_gbl_num_objects = 0;
    acpi_gbl_obj_type_count_misc = 0;
    for i in 0..(ACPI_TYPE_NS_NODE_MAX - 1) as usize {
        acpi_gbl_obj_type_count[i] = 0;
        acpi_gbl_node_type_count[i] = 0;
    }
    acpi_ns_walk_namespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, ACPI_UINT32_MAX, FALSE,
        Some(acpi_db_classify_one_object), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
}

pub unsafe fn acpi_db_display_statistics(type_arg: *mut i8) -> acpi_status {
    let _ = ACPI_DB_STAT_TYPES;
    acpi_ut_strupr(type_arg);
    let temp = acpi_db_match_argument(type_arg, ACPI_DB_STAT_TYPES.as_ptr());
    if temp == ACPI_TYPE_NOT_FOUND { acpi_os_printf("Invalid or unsupported argument\n"); return AE_OK; }
    match temp {
        CMD_STAT_ALLOCATIONS => {
            #[cfg(feature = "ACPI_DBG_TRACK_ALLOCATIONS")] { acpi_ut_dump_allocation_info(); }
        },
        CMD_STAT_TABLES => acpi_os_printf("ACPI Table Information (not implemented):\n\n"),
        CMD_STAT_OBJECTS => {
            acpi_db_count_namespace_objects();
            acpi_os_printf("\nObjects defined in the current namespace:\n\n");
            acpi_os_printf("%16.16s %10.10s %10.10s\n", "ACPI_TYPE", "NODES", "OBJECTS");
            for i in 0..ACPI_TYPE_NS_NODE_MAX as usize { acpi_os_printf("%16.16s %10u %10u\n", acpi_ut_get_type_name(i as u32), acpi_gbl_node_type_count[i], acpi_gbl_obj_type_count[i]); }
            acpi_os_printf("%16.16s %10u %10u\n", "Misc/Unknown", acpi_gbl_node_type_count_misc, acpi_gbl_obj_type_count_misc);
            acpi_os_printf("%16.16s %10u %10u\n", "TOTALS:", acpi_gbl_num_nodes, acpi_gbl_num_objects);
        },
        CMD_STAT_MEMORY => {},
        CMD_STAT_MISC => {
            acpi_os_printf("\nMiscellaneous Statistics:\n\n");
            acpi_os_printf("%-28s:     %7u\n", "Calls to AcpiPsFind", acpi_gbl_ps_find_count);
            acpi_os_printf("%-28s:     %7u\n", "Calls to AcpiNsLookup", acpi_gbl_ns_lookup_count);
            acpi_os_printf("\nMutex usage:\n\n");
            for i in 0..ACPI_NUM_MUTEX as usize { acpi_os_printf("%-28s:     %7u\n", acpi_ut_get_mutex_name(i as u32), acpi_gbl_mutex_info[i].use_count); }
        },
        CMD_STAT_SIZES => {
            acpi_os_printf("\nInternal object sizes:\n\n");
            acpi_os_printf("Common         %3d\n", core::mem::size_of::<acpi_object_common>() as u32);
            acpi_os_printf("Number         %3d\n", core::mem::size_of::<acpi_object_integer>() as u32);
            acpi_os_printf("String         %3d\n", core::mem::size_of::<acpi_object_string>() as u32);
            acpi_os_printf("Buffer         %3d\n", core::mem::size_of::<acpi_object_buffer>() as u32);
            acpi_os_printf("Package        %3d\n", core::mem::size_of::<acpi_object_package>() as u32);
            acpi_os_printf("OperandObject  %3d\n", core::mem::size_of::<acpi_operand_object>() as u32);
            acpi_os_printf("NamespaceNode  %3d\n", core::mem::size_of::<acpi_namespace_node>() as u32);
            acpi_os_printf("AcpiObject     %3d\n", core::mem::size_of::<acpi_object>() as u32);
        },
        CMD_STAT_STACK => {},
        _ => {}
    }
    acpi_os_printf("\n");
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
