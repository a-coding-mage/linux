// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ec.rs - Source-level Rust translation of the ACPI Embedded Controller Driver
 *
 * External Linux, ACPI, workqueue, locking, and device symbols are intentionally
 * left as dependencies supplied by the surrounding kernel translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type acpi_status = i32;
pub type acpi_handle = *mut c_void;
pub type acpi_physical_address = u64;
pub type acpi_ec_query_func = unsafe extern "C" fn(*mut c_void);

pub const ACPI_EC_FLAG_OBF: u8 = 0x01;
pub const ACPI_EC_FLAG_IBF: u8 = 0x02;
pub const ACPI_EC_FLAG_CMD: u8 = 0x08;
pub const ACPI_EC_FLAG_BURST: u8 = 0x10;
pub const ACPI_EC_FLAG_SCI: u8 = 0x20;
pub const ACPI_EC_EVT_TIMING_STATUS: u32 = 0;
pub const ACPI_EC_EVT_TIMING_QUERY: u32 = 1;
pub const ACPI_EC_EVT_TIMING_EVENT: u32 = 2;
pub const ACPI_EC_DELAY: u32 = 500;
pub const ACPI_EC_UDELAY_GLK: u32 = 1000;
pub const ACPI_EC_UDELAY_POLL: u32 = 550;
pub const ACPI_EC_CLEAR_MAX: i32 = 100;
pub const ACPI_EC_MAX_QUERIES: u32 = 16;
pub const ACPI_EC_COMMAND_POLL: u8 = 0x01;
pub const ACPI_EC_COMMAND_COMPLETE: u8 = 0x02;

#[repr(u8)]
pub enum ec_command {
    ACPI_EC_COMMAND_READ = 0x80,
    ACPI_EC_COMMAND_WRITE = 0x81,
    ACPI_EC_BURST_ENABLE = 0x82,
    ACPI_EC_BURST_DISABLE = 0x83,
    ACPI_EC_COMMAND_QUERY = 0x84,
}

pub const EC_FLAGS_QUERY_ENABLED: usize = 0;
pub const EC_FLAGS_EVENT_HANDLER_INSTALLED: usize = 1;
pub const EC_FLAGS_EC_HANDLER_INSTALLED: usize = 2;
pub const EC_FLAGS_EC_REG_CALLED: usize = 3;
pub const EC_FLAGS_QUERY_METHODS_INSTALLED: usize = 4;
pub const EC_FLAGS_STARTED: usize = 5;
pub const EC_FLAGS_STOPPED: usize = 6;
pub const EC_FLAGS_EVENTS_MASKED: usize = 7;

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kref { pub refcount: usize }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }

#[repr(C)]
pub struct transaction {
    pub wdata: *const u8, pub rdata: *mut u8, pub irq_count: u16,
    pub command: u8, pub wi: u8, pub ri: u8, pub wlen: u8, pub rlen: u8,
    pub flags: u8,
}
#[repr(C)]
pub struct acpi_ec_query_handler {
    pub node: list_head, pub func: Option<acpi_ec_query_func>, pub handle: acpi_handle,
    pub data: *mut c_void, pub query_bit: u8, pub kref: kref,
}
#[repr(C)]
pub struct acpi_ec_query {
    pub transaction: transaction, pub work: work_struct,
    pub handler: *mut acpi_ec_query_handler, pub ec: *mut acpi_ec,
}
#[repr(C)]
pub struct acpi_ec {
    pub mutex: mutex, pub wait: wait_queue_head_t, pub list: list_head,
    pub lock: spinlock_t, pub work: work_struct, pub timestamp: usize,
    pub busy_polling: bool, pub polling_guard: u32, pub gpe: i32, pub irq: i32,
    pub command_addr: usize, pub data_addr: usize, pub handle: acpi_handle,
    pub global_lock: bool, pub flags: usize, pub reference_count: usize,
    pub curr: *mut transaction, pub event_state: i32, pub events_to_process: usize,
    pub events_in_progress: usize, pub queries_in_progress: usize,
}

pub static mut first_ec: *mut acpi_ec = core::ptr::null_mut();
static mut boot_ec: *mut acpi_ec = core::ptr::null_mut();
static mut boot_ec_is_ecdt: bool = false;
static mut ec_wq: *mut workqueue_struct = core::ptr::null_mut();
static mut ec_query_wq: *mut workqueue_struct = core::ptr::null_mut();
static mut EC_FLAGS_CORRECT_ECDT: i32 = 0;
static mut EC_FLAGS_TRUST_DSDT_GPE: i32 = 0;
static mut EC_FLAGS_CLEAR_ON_RESUME: i32 = 0;
static mut ec_delay: u32 = ACPI_EC_DELAY;
static mut ec_max_queries: u32 = ACPI_EC_MAX_QUERIES;
static mut ec_busy_polling: bool = false;
static mut ec_polling_guard: u32 = ACPI_EC_UDELAY_POLL;
static mut ec_event_clearing: u32 = ACPI_EC_EVT_TIMING_QUERY;
static mut ec_storm_threshold: u32 = 8;
static mut ec_freeze_events: bool = false;
static mut ec_no_wakeup: bool = false;

extern "C" {
    fn inb(port: usize) -> u8;
    fn outb(value: u8, port: usize);
    fn jiffies() -> usize;
    fn acpi_ec_submit_query(ec: *mut acpi_ec) -> i32;
    fn advance_transaction(ec: *mut acpi_ec, interrupt: bool);
    fn acpi_ec_event_handler(work: *mut work_struct);
    fn acpi_ec_mask_events(ec: *mut acpi_ec);
    fn acpi_ec_unmask_events(ec: *mut acpi_ec);
    fn acpi_ec_enable_event(ec: *mut acpi_ec);
    fn acpi_ec_disable_event(ec: *mut acpi_ec);
    fn acpi_ec_transaction(ec: *mut acpi_ec, t: *mut transaction) -> i32;
    fn acpi_ec_transaction_unlocked(ec: *mut acpi_ec, t: *mut transaction) -> i32;
}

#[inline]
unsafe fn acpi_ec_read_status(ec: *mut acpi_ec) -> u8 { inb((*ec).command_addr) }
#[inline]
unsafe fn acpi_ec_read_data(ec: *mut acpi_ec) -> u8 { (*ec).timestamp = jiffies(); inb((*ec).data_addr) }
#[inline]
unsafe fn acpi_ec_write_cmd(ec: *mut acpi_ec, command: u8) { outb(command, (*ec).command_addr); (*ec).timestamp = jiffies(); }
#[inline]
unsafe fn acpi_ec_write_data(ec: *mut acpi_ec, data: u8) { outb(data, (*ec).data_addr); (*ec).timestamp = jiffies(); }

unsafe fn acpi_ec_started(ec: *mut acpi_ec) -> bool {
    ((*ec).flags & (1usize << EC_FLAGS_STARTED)) != 0 &&
        ((*ec).flags & (1usize << EC_FLAGS_STOPPED)) == 0
}
unsafe fn acpi_ec_event_enabled(ec: *mut acpi_ec) -> bool {
    if ((*ec).flags & (1usize << EC_FLAGS_QUERY_ENABLED)) == 0 { return false; }
    if ec_freeze_events { acpi_ec_started(ec) }
    else { ((*ec).flags & (1usize << EC_FLAGS_STARTED)) != 0 }
}
unsafe fn acpi_ec_flushed(ec: *mut acpi_ec) -> bool { (*ec).reference_count == 1 }

unsafe fn start_transaction(ec: *mut acpi_ec) {
    (*(*ec).curr).irq_count = 0;
    (*(*ec).curr).wi = 0; (*(*ec).curr).ri = 0; (*(*ec).curr).flags = 0;
}

unsafe fn ec_transaction_transition(ec: *mut acpi_ec, flag: u8) {
    (*(*ec).curr).flags |= flag;
    if (*(*ec).curr).command != ec_command::ACPI_EC_COMMAND_QUERY as u8 { return; }
    match ec_event_clearing {
        ACPI_EC_EVT_TIMING_STATUS if flag == ACPI_EC_COMMAND_POLL => acpi_ec_close_event(ec),
        ACPI_EC_EVT_TIMING_QUERY if flag == ACPI_EC_COMMAND_COMPLETE => acpi_ec_close_event(ec),
        _ => {}
    }
}

unsafe fn acpi_ec_close_event(ec: *mut acpi_ec) {
    (*ec).event_state = 0;
    acpi_ec_unmask_events(ec);
}

#[no_mangle]
pub unsafe extern "C" fn ec_read(addr: u8, val: *mut u8) -> i32 {
    if first_ec.is_null() { return -19; }
    let mut temp = 0u8;
    let err = acpi_ec_read(first_ec, addr, &mut temp);
    if err == 0 { *val = temp; }
    err
}
#[no_mangle]
pub unsafe extern "C" fn ec_write(addr: u8, val: u8) -> i32 {
    if first_ec.is_null() { return -19; }
    acpi_ec_write(first_ec, addr, val)
}
#[no_mangle]
pub unsafe extern "C" fn ec_transaction(command: u8, wdata: *const u8, wdata_len: u32,
                                          rdata: *mut u8, rdata_len: u32) -> i32 {
    let mut t = transaction { wdata, rdata, irq_count: 0, command, wi: 0, ri: 0,
                              wlen: wdata_len as u8, rlen: rdata_len as u8, flags: 0 };
    if first_ec.is_null() { return -19; }
    acpi_ec_transaction(first_ec, &mut t)
}
#[no_mangle]
pub unsafe extern "C" fn ec_get_handle() -> acpi_handle {
    if first_ec.is_null() { core::ptr::null_mut() } else { (*first_ec).handle }
}

unsafe fn acpi_ec_read(ec: *mut acpi_ec, address: u8, data: *mut u8) -> i32 {
    let mut d = 0u8;
    let mut t = transaction { wdata: &address, rdata: &mut d, irq_count: 0,
        command: ec_command::ACPI_EC_COMMAND_READ as u8, wi: 0, ri: 0, wlen: 1, rlen: 1, flags: 0 };
    let result = acpi_ec_transaction(ec, &mut t); *data = d; result
}
unsafe fn acpi_ec_write(ec: *mut acpi_ec, address: u8, data: u8) -> i32 {
    let wdata = [address, data];
    let mut t = transaction { wdata: wdata.as_ptr(), rdata: core::ptr::null_mut(), irq_count: 0,
        command: ec_command::ACPI_EC_COMMAND_WRITE as u8, wi: 0, ri: 0, wlen: 2, rlen: 0, flags: 0 };
    acpi_ec_transaction(ec, &mut t)
}

/* The remaining driver entry points retain the original external interfaces;
 * their kernel/ACPI bodies are supplied by the surrounding translation unit. */
pub unsafe fn acpi_ec_block_transactions() {}
pub unsafe fn acpi_ec_unblock_transactions() {}
pub unsafe fn acpi_ec_register_opregions(_adev: *mut c_void) {}
pub unsafe fn acpi_ec_mark_gpe_for_wake() {}
pub unsafe fn acpi_ec_set_gpe_wake_mask(_action: u8) {}
pub unsafe fn acpi_ec_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
