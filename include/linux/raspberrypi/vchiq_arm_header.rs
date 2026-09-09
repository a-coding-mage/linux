/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/*
 * Copyright (c) 2014 Raspberry Pi (Trading) Ltd. All rights reserved.
 * Copyright (c) 2010-2012 Broadcom. All rights reserved.
 */

/* Translated from vchiq_arm.h. C header dependencies are supplied elsewhere. */

/* Some per-instance constants */
pub const MAX_COMPLETIONS: usize = 128;
pub const MAX_SERVICES: usize = 64;
pub const MAX_ELEMENTS: usize = 8;
pub const MSG_QUEUE_SIZE: usize = 128;

pub const VCHIQ_DRV_MAX_CALLBACKS: usize = 10;

pub enum rpi_firmware {}
pub enum vchiq_device {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum USE_TYPE_E {
    USE_TYPE_SERVICE,
    USE_TYPE_VCHIQ,
}

#[repr(C)]
pub struct vchiq_platform_info {
    pub cache_line_size: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct vchiq_drv_mgmt {
    pub fw: *mut rpi_firmware,
    pub info: *const vchiq_platform_info,

    pub connected: bool,
    pub num_deferred_callbacks: ::core::ffi::c_int,
    /* Protects connected and num_deferred_callbacks */
    pub connected_mutex: mutex,

    pub deferred_callback: [Option<unsafe extern "C" fn()>; VCHIQ_DRV_MAX_CALLBACKS],

    pub free_fragments_sema: semaphore,
    pub free_fragments_mutex: semaphore,
    pub fragments_base: *mut ::core::ffi::c_char,
    pub free_fragments: *mut ::core::ffi::c_char,
    pub fragments_size: ::core::ffi::c_uint,

    pub regs: *mut ::core::ffi::c_void,

    pub state: vchiq_state,
}

#[repr(C)]
pub struct user_service {
    pub service: *mut vchiq_service,
    pub userdata: *mut ::core::ffi::c_void,
    pub instance: *mut vchiq_instance,
    pub is_vchi: ::core::ffi::c_char,
    pub dequeue_pending: ::core::ffi::c_char,
    pub close_pending: ::core::ffi::c_char,
    pub message_available_pos: ::core::ffi::c_int,
    pub msg_insert: ::core::ffi::c_int,
    pub msg_remove: ::core::ffi::c_int,
    pub insert_event: completion,
    pub remove_event: completion,
    pub close_event: completion,
    pub msg_queue: [*mut vchiq_header; MSG_QUEUE_SIZE],
}

#[repr(C)]
pub struct bulk_waiter_node {
    pub bulk_waiter: bulk_waiter,
    pub pid: ::core::ffi::c_int,
    pub list: list_head,
}

#[repr(C)]
pub struct vchiq_instance {
    pub state: *mut vchiq_state,
    pub completions: [vchiq_completion_data_kernel; MAX_COMPLETIONS],
    pub completion_insert: ::core::ffi::c_int,
    pub completion_remove: ::core::ffi::c_int,
    pub insert_event: completion,
    pub remove_event: completion,
    pub completion_mutex: mutex,

    pub connected: ::core::ffi::c_int,
    pub closing: ::core::ffi::c_int,
    pub pid: ::core::ffi::c_int,
    pub mark: ::core::ffi::c_int,
    pub use_close_delivered: ::core::ffi::c_int,
    pub trace: ::core::ffi::c_int,

    pub bulk_waiter_list: list_head,
    pub bulk_waiter_list_mutex: mutex,

    pub debugfs_node: vchiq_debugfs_node,
}

extern "C" {
    pub fn vchiq_use_service(instance: *mut vchiq_instance, handle: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn vchiq_release_service(instance: *mut vchiq_instance, handle: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn vchiq_check_service(service: *mut vchiq_service) -> ::core::ffi::c_int;
    pub fn vchiq_dump_service_use_state(state: *mut vchiq_state);
    pub fn vchiq_use_internal(
        state: *mut vchiq_state,
        service: *mut vchiq_service,
        use_type: USE_TYPE_E,
    ) -> ::core::ffi::c_int;
    pub fn vchiq_release_internal(state: *mut vchiq_state, service: *mut vchiq_service) -> ::core::ffi::c_int;
    pub fn vchiq_instance_get_debugfs_node(instance: *mut vchiq_instance) -> *mut vchiq_debugfs_node;
    pub fn vchiq_instance_get_use_count(instance: *mut vchiq_instance) -> ::core::ffi::c_int;
    pub fn vchiq_instance_get_pid(instance: *mut vchiq_instance) -> ::core::ffi::c_int;
    pub fn vchiq_instance_get_trace(instance: *mut vchiq_instance) -> ::core::ffi::c_int;
    pub fn vchiq_instance_set_trace(instance: *mut vchiq_instance, trace: ::core::ffi::c_int);
    pub fn vchiq_add_connected_callback(device: *mut vchiq_device, callback: Option<unsafe extern "C" fn()>);

    /* CONFIG_VCHIQ_CDEV: declarations are available when this build condition is enabled. */
    pub fn vchiq_deregister_chrdev();
    pub fn vchiq_register_chrdev(parent: *mut device) -> ::core::ffi::c_int;

    pub fn service_callback(
        vchiq_instance: *mut vchiq_instance,
        reason: vchiq_reason,
        header: *mut vchiq_header,
        handle: ::core::ffi::c_uint,
        cb_data: *mut ::core::ffi::c_void,
        cb_userdata: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn free_bulk_waiter(instance: *mut vchiq_instance);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
