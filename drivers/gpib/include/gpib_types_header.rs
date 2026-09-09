/* SPDX-License-Identifier: GPL-2.0 */
/*
 * copyright           : (C) 2002 by Frank Mori Hess
 */

// The declarations below correspond to the Linux-kernel-only portion of gpib_types.h.

#[repr(C)]
pub struct gpib_board_config {
    pub init_data: *mut core::ffi::c_void,
    pub init_data_length: core::ffi::c_int,
    pub ibbase: u32,
    pub mmibbase: *mut core::ffi::c_void,
    pub ibirq: core::ffi::c_uint,
    pub ibdma: core::ffi::c_uint,
    pub pci_bus: core::ffi::c_int,
    pub pci_slot: core::ffi::c_int,
    pub device_path: *mut core::ffi::c_char,
    pub serial_number: *mut core::ffi::c_char,
}

#[repr(C)]
pub struct gpib_interface {
    pub name: *mut core::ffi::c_char,
    pub attach: Option<unsafe extern "C" fn(*mut gpib_board, *const gpib_board_config) -> core::ffi::c_int>,
    pub detach: Option<unsafe extern "C" fn(*mut gpib_board)>,
    pub read: Option<unsafe extern "C" fn(*mut gpib_board, *mut u8, usize, *mut core::ffi::c_int, *mut usize) -> core::ffi::c_int>,
    pub write: Option<unsafe extern "C" fn(*mut gpib_board, *mut u8, usize, core::ffi::c_int, *mut usize) -> core::ffi::c_int>,
    pub command: Option<unsafe extern "C" fn(*mut gpib_board, *mut u8, usize, *mut usize) -> core::ffi::c_int>,
    pub take_control: Option<unsafe extern "C" fn(*mut gpib_board, core::ffi::c_int) -> core::ffi::c_int>,
    pub go_to_standby: Option<unsafe extern "C" fn(*mut gpib_board) -> core::ffi::c_int>,
    pub request_system_control: Option<unsafe extern "C" fn(*mut gpib_board, core::ffi::c_int) -> core::ffi::c_int>,
    pub interface_clear: Option<unsafe extern "C" fn(*mut gpib_board, core::ffi::c_int)>,
    pub remote_enable: Option<unsafe extern "C" fn(*mut gpib_board, core::ffi::c_int)>,
    pub enable_eos: Option<unsafe extern "C" fn(*mut gpib_board, u8, core::ffi::c_int) -> core::ffi::c_int>,
    pub disable_eos: Option<unsafe extern "C" fn(*mut gpib_board)>,
    pub parallel_poll_configure: Option<unsafe extern "C" fn(*mut gpib_board, u8)>,
    pub parallel_poll: Option<unsafe extern "C" fn(*mut gpib_board, *mut u8) -> core::ffi::c_int>,
    pub parallel_poll_response: Option<unsafe extern "C" fn(*mut gpib_board, core::ffi::c_int)>,
    pub local_parallel_poll_mode: Option<unsafe extern "C" fn(*mut gpib_board, core::ffi::c_int)>,
    pub line_status: Option<unsafe extern "C" fn(*const gpib_board) -> core::ffi::c_int>,
    pub update_status: Option<unsafe extern "C" fn(*mut gpib_board, core::ffi::c_uint) -> core::ffi::c_uint>,
    pub primary_address: Option<unsafe extern "C" fn(*mut gpib_board, core::ffi::c_uint) -> core::ffi::c_int>,
    pub secondary_address: Option<unsafe extern "C" fn(*mut gpib_board, core::ffi::c_uint, core::ffi::c_int) -> core::ffi::c_int>,
    pub serial_poll_response: Option<unsafe extern "C" fn(*mut gpib_board, u8)>,
    pub serial_poll_response2: Option<unsafe extern "C" fn(*mut gpib_board, u8, core::ffi::c_int)>,
    pub serial_poll_status: Option<unsafe extern "C" fn(*mut gpib_board) -> u8>,
    pub t1_delay: Option<unsafe extern "C" fn(*mut gpib_board, core::ffi::c_uint) -> core::ffi::c_int>,
    pub return_to_local: Option<unsafe extern "C" fn(*mut gpib_board)>,
    pub no_7_bit_eos: core::ffi::c_uint,
    pub skip_check_for_command_acceptors: core::ffi::c_uint,
}

#[repr(C)]
pub struct gpib_event_queue {
    pub event_head: list_head,
    pub lock: spinlock_t,
    pub num_events: core::ffi::c_uint,
    pub dropped_event: core::ffi::c_uint,
}

#[inline]
pub unsafe fn init_event_queue(queue: *mut gpib_event_queue) {
    INIT_LIST_HEAD(&mut (*queue).event_head);
    (*queue).num_events = 0;
    (*queue).dropped_event = 0;
    spin_lock_init(&mut (*queue).lock);
}

#[repr(C)]
pub struct gpib_pseudo_irq {
    pub timer: timer_list,
    pub handler: Option<unsafe extern "C" fn(core::ffi::c_int, *mut core::ffi::c_void) -> irqreturn_t>,
    pub board: *mut gpib_board,
    pub active: atomic_t,
}

#[inline]
pub unsafe fn init_gpib_pseudo_irq(pseudo_irq: *mut gpib_pseudo_irq) {
    (*pseudo_irq).handler = None;
    timer_setup(&mut (*pseudo_irq).timer, None, 0);
    atomic_set(&mut (*pseudo_irq).active, 0);
}

#[repr(C)]
pub struct gpib_interface_list {
    pub list: list_head,
    pub interface: *mut gpib_interface,
    pub module: *mut module,
}

#[repr(C)]
pub struct gpib_board {
    pub interface: *mut gpib_interface,
    pub provider_module: *mut module,
    pub buffer: *mut u8,
    pub buffer_length: core::ffi::c_uint,
    pub status: core::ffi::c_ulong,
    pub wait: wait_queue_head_t,
    pub user_mutex: mutex,
    pub big_gpib_mutex: mutex,
    pub locking_pid: pid_t,
    pub locking_pid_spinlock: spinlock_t,
    pub spinlock: spinlock_t,
    pub timer: timer_list,
    pub dev: *mut device,
    pub gpib_dev: *mut device,
    pub private_data: *mut core::ffi::c_void,
    pub use_count: core::ffi::c_uint,
    pub device_list: list_head,
    pub pad: core::ffi::c_uint,
    pub sad: core::ffi::c_int,
    pub usec_timeout: core::ffi::c_uint,
    pub parallel_poll_configuration: u8,
    pub t1_nano_sec: core::ffi::c_uint,
    pub online: core::ffi::c_uint,
    pub autospollers: core::ffi::c_int,
    pub autospoll_task: *mut task_struct,
    pub event_queue: gpib_event_queue,
    pub minor: core::ffi::c_int,
    pub pseudo_irq: gpib_pseudo_irq,
    pub stuck_srq: atomic_t,
    pub config: gpib_board_config,
    pub master: core::ffi::c_uint,
    pub ist: core::ffi::c_uint,
    pub local_ppoll_mode: core::ffi::c_uint,
}

#[repr(C)]
pub struct gpib_event { pub list: list_head, pub event_type: i16 }

#[repr(C)]
pub struct gpib_status_queue {
    pub list: list_head,
    pub pad: core::ffi::c_uint,
    pub sad: core::ffi::c_int,
    pub status_bytes: list_head,
    pub num_status_bytes: core::ffi::c_uint,
    pub reference_count: core::ffi::c_uint,
    pub dropped_byte: core::ffi::c_uint,
}

#[repr(C)]
pub struct gpib_status_byte { pub list: list_head, pub poll_byte: u8 }

unsafe extern "C" { pub fn init_gpib_status_queue(device: *mut gpib_status_queue); }

#[repr(C)]
pub struct gpib_descriptor {
    pub pad: core::ffi::c_uint,
    pub sad: core::ffi::c_int,
    pub io_in_progress: atomic_t,
    pub descriptor_busy: atomic_t,
    pub is_board: core::ffi::c_uint,
    pub autopoll_enabled: core::ffi::c_uint,
}

#[repr(C)]
pub struct gpib_file_private {
    pub holding_mutex: atomic_t,
    pub descriptors: [*mut gpib_descriptor; GPIB_MAX_NUM_DESCRIPTORS],
    pub descriptors_mutex: mutex,
    pub got_module: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
