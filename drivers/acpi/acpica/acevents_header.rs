/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acevents.h - Event subcomponent prototypes and defines
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/*
 * Conditions to trigger post enabling GPE polling:
 * It is not sufficient to trigger edge-triggered GPE with specific GPE
 * chips, software need to poll once after enabling.
 *
 * ACPI_USE_GPE_POLLING is a build-time condition from the C header.
 */
#[cfg(ACPI_USE_GPE_POLLING)]
macro_rules! ACPI_GPE_IS_POLLING_NEEDED {
    ($gpe:expr) => {
        (($gpe).runtime_count == 1
            && ($gpe).flags & ACPI_GPE_INITIALIZED != 0
            && (($gpe).flags & ACPI_GPE_XRUPT_TYPE_MASK) == ACPI_GPE_EDGE_TRIGGERED)
    };
}

#[cfg(not(ACPI_USE_GPE_POLLING))]
macro_rules! ACPI_GPE_IS_POLLING_NEEDED {
    ($gpe:expr) => {
        false
    };
}

extern "C" {
    /* evevent */
    pub fn acpi_ev_initialize_events() -> acpi_status;
    pub fn acpi_ev_install_xrupt_handlers() -> acpi_status;
    pub fn acpi_ev_fixed_event_detect() -> u32;

    /* evmisc */
    pub fn acpi_ev_is_notify_object(node: *mut acpi_namespace_node) -> u8;
    pub fn acpi_ev_get_gpe_number_index(gpe_number: u32) -> u32;
    pub fn acpi_ev_queue_notify_request(
        node: *mut acpi_namespace_node,
        notify_value: u32,
    ) -> acpi_status;

    /* evglock - Global Lock support */
    pub fn acpi_ev_init_global_lock_handler() -> acpi_status;
    pub fn acpi_ev_acquire_global_lock(timeout: u16) -> acpi_status;
    pub fn acpi_ev_release_global_lock() -> acpi_status;
    pub fn acpi_ev_remove_global_lock_handler() -> acpi_status;

    /* evgpe - Low-level GPE support */
    pub fn acpi_ev_gpe_detect(gpe_xrupt_list: *mut acpi_gpe_xrupt_info) -> u32;
    pub fn acpi_ev_update_gpe_enable_mask(
        gpe_event_info: *mut acpi_gpe_event_info,
    ) -> acpi_status;
    pub fn acpi_ev_enable_gpe(gpe_event_info: *mut acpi_gpe_event_info) -> acpi_status;
    pub fn acpi_ev_mask_gpe(
        gpe_event_info: *mut acpi_gpe_event_info,
        is_masked: u8,
    ) -> acpi_status;
    pub fn acpi_ev_add_gpe_reference(
        gpe_event_info: *mut acpi_gpe_event_info,
        clear_on_enable: u8,
    ) -> acpi_status;
    pub fn acpi_ev_remove_gpe_reference(
        gpe_event_info: *mut acpi_gpe_event_info,
    ) -> acpi_status;
    pub fn acpi_ev_get_gpe_event_info(
        gpe_device: acpi_handle,
        gpe_number: u32,
    ) -> *mut acpi_gpe_event_info;
    pub fn acpi_ev_low_get_gpe_info(
        gpe_number: u32,
        gpe_block: *mut acpi_gpe_block_info,
    ) -> *mut acpi_gpe_event_info;
    pub fn acpi_ev_finish_gpe(gpe_event_info: *mut acpi_gpe_event_info) -> acpi_status;
    pub fn acpi_ev_detect_gpe(
        gpe_device: *mut acpi_namespace_node,
        gpe_event_info: *mut acpi_gpe_event_info,
        gpe_number: u32,
    ) -> u32;

    /* evgpeblk - Upper-level GPE block support */
    pub fn acpi_ev_create_gpe_block(
        gpe_device: *mut acpi_namespace_node,
        address: u64,
        space_id: u8,
        register_count: u32,
        gpe_block_base_number: u16,
        interrupt_number: u32,
        return_gpe_block: *mut *mut acpi_gpe_block_info,
    ) -> acpi_status;
    pub fn acpi_ev_initialize_gpe_block(
        gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
        gpe_block: *mut acpi_gpe_block_info,
        context: *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_ev_delete_gpe_block(gpe_block: *mut acpi_gpe_block_info) -> acpi_status;
    pub fn acpi_ev_gpe_dispatch(
        gpe_device: *mut acpi_namespace_node,
        gpe_event_info: *mut acpi_gpe_event_info,
        gpe_number: u32,
    ) -> u32;

    /* evgpeinit - GPE initialization and update */
    pub fn acpi_ev_gpe_initialize() -> acpi_status;
    pub fn acpi_ev_update_gpes(table_owner_id: acpi_owner_id);
    pub fn acpi_ev_match_gpe_method(
        obj_handle: acpi_handle,
        level: u32,
        context: *mut core::ffi::c_void,
        return_value: *mut *mut core::ffi::c_void,
    ) -> acpi_status;

    /* evgpeutil - GPE utilities */
    pub fn acpi_ev_walk_gpe_list(
        gpe_walk_callback: acpi_gpe_callback,
        context: *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_ev_get_gpe_device(
        gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
        gpe_block: *mut acpi_gpe_block_info,
        context: *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_ev_get_gpe_xrupt_block(
        interrupt_number: u32,
        gpe_xrupt_block: *mut *mut acpi_gpe_xrupt_info,
    ) -> acpi_status;
    pub fn acpi_ev_delete_gpe_xrupt(gpe_xrupt: *mut acpi_gpe_xrupt_info) -> acpi_status;
    pub fn acpi_ev_delete_gpe_handlers(
        gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
        gpe_block: *mut acpi_gpe_block_info,
        context: *mut core::ffi::c_void,
    ) -> acpi_status;

    /* evhandler - Address space handling */
    pub fn acpi_ev_find_region_handler(
        space_id: acpi_adr_space_type,
        handler_obj: *mut acpi_operand_object,
    ) -> *mut acpi_operand_object;
    pub fn acpi_ev_has_default_handler(
        node: *mut acpi_namespace_node,
        space_id: acpi_adr_space_type,
    ) -> u8;
    pub fn acpi_ev_install_region_handlers() -> acpi_status;
    pub fn acpi_ev_install_space_handler(
        node: *mut acpi_namespace_node,
        space_id: acpi_adr_space_type,
        handler: acpi_adr_space_handler,
        setup: acpi_adr_space_setup,
        context: *mut core::ffi::c_void,
    ) -> acpi_status;

    /* evregion - Operation region support */
    pub fn acpi_ev_initialize_op_regions() -> acpi_status;
    pub fn acpi_ev_address_space_dispatch(
        region_obj: *mut acpi_operand_object,
        field_obj: *mut acpi_operand_object,
        function: u32,
        region_offset: u32,
        bit_width: u32,
        value: *mut u64,
    ) -> acpi_status;
    pub fn acpi_ev_attach_region(
        handler_obj: *mut acpi_operand_object,
        region_obj: *mut acpi_operand_object,
        acpi_ns_is_locked: u8,
    ) -> acpi_status;
    pub fn acpi_ev_detach_region(region_obj: *mut acpi_operand_object, acpi_ns_is_locked: u8);
    pub fn acpi_ev_execute_reg_methods(
        node: *mut acpi_namespace_node,
        max_depth: u32,
        space_id: acpi_adr_space_type,
        function: u32,
    );
    pub fn acpi_ev_execute_reg_method(
        region_obj: *mut acpi_operand_object,
        function: u32,
    ) -> acpi_status;

    /* evregini - Region initialization and setup */
    pub fn acpi_ev_system_memory_region_setup(
        handle: acpi_handle,
        function: u32,
        handler_context: *mut core::ffi::c_void,
        region_context: *mut *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_ev_io_space_region_setup(
        handle: acpi_handle,
        function: u32,
        handler_context: *mut core::ffi::c_void,
        region_context: *mut *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_ev_pci_config_region_setup(
        handle: acpi_handle,
        function: u32,
        handler_context: *mut core::ffi::c_void,
        region_context: *mut *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_ev_cmos_region_setup(
        handle: acpi_handle,
        function: u32,
        handler_context: *mut core::ffi::c_void,
        region_context: *mut *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_ev_pci_bar_region_setup(
        handle: acpi_handle,
        function: u32,
        handler_context: *mut core::ffi::c_void,
        region_context: *mut *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_ev_data_table_region_setup(
        handle: acpi_handle,
        function: u32,
        handler_context: *mut core::ffi::c_void,
        region_context: *mut *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_ev_default_region_setup(
        handle: acpi_handle,
        function: u32,
        handler_context: *mut core::ffi::c_void,
        region_context: *mut *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_ev_initialize_region(region_obj: *mut acpi_operand_object) -> acpi_status;
    pub fn acpi_ev_is_pci_root_bridge(node: *mut acpi_namespace_node) -> u8;

    /* evsci - SCI (System Control Interrupt) handling/dispatch */
    pub fn acpi_ev_gpe_xrupt_handler(context: *mut core::ffi::c_void) -> u32;
    pub fn acpi_ev_sci_dispatch() -> u32;
    pub fn acpi_ev_install_sci_handler() -> u32;
    pub fn acpi_ev_remove_all_sci_handlers() -> acpi_status;
    pub fn acpi_ev_terminate();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
