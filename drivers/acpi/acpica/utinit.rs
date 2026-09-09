// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utinit - Common ACPI subsystem initialization
//
// Copyright (C) 2000 - 2026, Intel Corp.

// Dependencies supplied by the surrounding ACPICA translation.

// static void acpi_ut_terminate(void);

#[cfg(not(feature = "acpi_reduced_hardware"))]
unsafe fn acpi_ut_free_gpe_lists() {
    let mut gpe_xrupt_info: *mut acpi_gpe_xrupt_info = acpi_gbl_gpe_xrupt_list_head;
    while !gpe_xrupt_info.is_null() {
        let mut gpe_block: *mut acpi_gpe_block_info = (*gpe_xrupt_info).gpe_block_list_head;
        while !gpe_block.is_null() {
            let next_gpe_block = (*gpe_block).next;
            ACPI_FREE((*gpe_block).event_info);
            ACPI_FREE((*gpe_block).register_info);
            ACPI_FREE(gpe_block);
            gpe_block = next_gpe_block;
        }
        let next_gpe_xrupt_info = (*gpe_xrupt_info).next;
        ACPI_FREE(gpe_xrupt_info);
        gpe_xrupt_info = next_gpe_xrupt_info;
    }
}

#[cfg(feature = "acpi_reduced_hardware")]
unsafe fn acpi_ut_free_gpe_lists() {}

pub unsafe fn acpi_ut_init_globals() -> acpi_status {
    let mut status: acpi_status;
    let mut i: u32;

    status = acpi_ut_create_caches();
    if ACPI_FAILURE(status) {
        return status;
    }

    for i in 0..ACPI_ADDRESS_RANGE_MAX {
        acpi_gbl_address_range_list[i as usize] = core::ptr::null_mut();
    }
    for i in 0..ACPI_NUM_MUTEX {
        acpi_gbl_mutex_info[i as usize].mutex = core::ptr::null_mut();
        acpi_gbl_mutex_info[i as usize].thread_id = ACPI_MUTEX_NOT_ACQUIRED;
        acpi_gbl_mutex_info[i as usize].use_count = 0;
    }
    for i in 0..ACPI_NUM_OWNERID_MASKS {
        acpi_gbl_owner_id_mask[i as usize] = 0;
    }
    acpi_gbl_owner_id_mask[(ACPI_NUM_OWNERID_MASKS - 1) as usize] = 0x80000000;

    acpi_method_count = 0;
    acpi_sci_count = 0;
    acpi_gpe_count = 0;
    for i in 0..ACPI_NUM_FIXED_EVENTS {
        acpi_fixed_event_count[i as usize] = 0;
    }

    #[cfg(not(feature = "acpi_reduced_hardware"))]
    {
        acpi_gbl_all_gpes_initialized = FALSE;
        acpi_gbl_gpe_xrupt_list_head = core::ptr::null_mut();
        acpi_gbl_gpe_fadt_blocks[0] = core::ptr::null_mut();
        acpi_gbl_gpe_fadt_blocks[1] = core::ptr::null_mut();
        acpi_current_gpe_count = 0;
        acpi_gbl_global_event_handler = None;
        acpi_gbl_sci_handler_list = core::ptr::null_mut();
    }

    acpi_gbl_global_notify[0].handler = None;
    acpi_gbl_global_notify[1].handler = None;
    acpi_gbl_exception_handler = None;
    acpi_gbl_init_handler = None;
    acpi_gbl_table_handler = None;
    acpi_gbl_interface_handler = None;

    acpi_gbl_global_lock_semaphore = ACPI_SEMAPHORE_NULL;
    acpi_gbl_global_lock_mutex = core::ptr::null_mut();
    acpi_gbl_global_lock_acquired = FALSE;
    acpi_gbl_global_lock_handle = 0;
    acpi_gbl_global_lock_present = FALSE;

    acpi_gbl_DSDT = core::ptr::null_mut();
    acpi_gbl_cm_single_step = FALSE;
    acpi_gbl_shutdown = FALSE;
    acpi_gbl_ns_lookup_count = 0;
    acpi_gbl_ps_find_count = 0;
    acpi_gbl_acpi_hardware_present = TRUE;
    acpi_gbl_last_owner_id_index = 0;
    acpi_gbl_next_owner_id_offset = 0;
    acpi_gbl_debugger_configuration = DEBUGGER_THREADING;
    acpi_gbl_osi_mutex = core::ptr::null_mut();

    acpi_gbl_events_initialized = FALSE;
    acpi_gbl_system_awake_and_running = TRUE;

    acpi_gbl_root_node = core::ptr::null_mut();
    acpi_gbl_root_node_struct.name.integer = ACPI_ROOT_NAME;
    acpi_gbl_root_node_struct.descriptor_type = ACPI_DESC_TYPE_NAMED;
    acpi_gbl_root_node_struct.type_ = ACPI_TYPE_DEVICE;
    acpi_gbl_root_node_struct.parent = core::ptr::null_mut();
    acpi_gbl_root_node_struct.child = core::ptr::null_mut();
    acpi_gbl_root_node_struct.peer = core::ptr::null_mut();
    acpi_gbl_root_node_struct.object = core::ptr::null_mut();

    #[cfg(feature = "acpi_disassembler")]
    {
        acpi_gbl_external_list = core::ptr::null_mut();
        acpi_gbl_num_external_methods = 0;
        acpi_gbl_resolved_external_methods = 0;
    }
    #[cfg(feature = "acpi_debug_output")]
    { acpi_gbl_lowest_stack_pointer = ACPI_SIZE_MAX as acpi_size; }
    #[cfg(feature = "acpi_dbg_track_allocations")]
    {
        acpi_gbl_display_final_mem_stats = FALSE;
        acpi_gbl_disable_mem_tracking = FALSE;
    }
    AE_OK
}

unsafe fn acpi_ut_terminate() {
    acpi_ut_free_gpe_lists();
    acpi_ut_delete_address_lists();
}

pub unsafe fn acpi_ut_subsystem_shutdown() {
    if acpi_gbl_shutdown {
        ACPI_ERROR((AE_INFO, "ACPI Subsystem is already terminated"));
        return;
    }
    acpi_gbl_shutdown = TRUE;
    acpi_gbl_startup_flags = 0;
    ACPI_DEBUG_PRINT((ACPI_DB_INFO, "Shutting down ACPI Subsystem\n"));

    #[cfg(not(feature = "acpi_asl_compiler"))]
    {
        acpi_ev_terminate();
        acpi_ut_interface_terminate();
    }
    acpi_ns_terminate();
    acpi_tb_terminate();
    acpi_ut_terminate();
    let _ = acpi_ut_delete_caches();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
