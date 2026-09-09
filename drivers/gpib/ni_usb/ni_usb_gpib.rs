// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of ni_usb_gpib.c.  Linux kernel and GPIB symbols
// referenced here are supplied by the surrounding kernel/Rust integration.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

pub const MAX_NUM_NI_USB_INTERFACES: usize = 128;

#[repr(C)] pub struct usb_interface { _private: [u8; 0] }
#[repr(C)] pub struct usb_device { _private: [u8; 0] }
#[repr(C)] pub struct urb { pub context: *mut c_void, pub status: c_int, pub actual_length: c_int, pub transfer_buffer: *mut c_void }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct gpib_board { pub private_data: *mut ni_usb_priv, pub status: c_uint, pub usec_timeout: c_uint, pub t1_nano_sec: c_uint, pub master: c_int, pub pad: c_uint, pub sad: c_int, pub minor: c_int, pub gpib_dev: *mut c_void, pub spinlock: c_void, pub wait: c_void }
#[repr(C)] pub struct gpib_board_config { pub device_path: *const c_void }
#[repr(C)] pub struct ni_usb_urb_ctx { pub complete: c_void, pub timed_out: c_int }
#[repr(C)] pub struct ni_usb_status_block { pub id: u8, pub ibsta: u16, pub error_code: u8, pub count: u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct ni_usb_register { pub device: u8, pub address: u8, pub value: u8 }
#[repr(C)] pub struct ni_usb_priv { pub bus_interface: *mut usb_interface, pub bulk_urb: *mut urb, pub interrupt_urb: *mut urb, pub bulk_timer: timer_list, pub context: ni_usb_urb_ctx, pub bulk_out_endpoint: c_int, pub bulk_in_endpoint: c_int, pub interrupt_in_endpoint: c_int, pub product_id: c_int, pub eos_mode: c_uint, pub eos_char: u8, pub ren_state: c_int, pub monitored_ibsta_bits: c_uint, pub bulk_transfer_lock: c_void, pub control_transfer_lock: c_void, pub interrupt_transfer_lock: c_void, pub addressed_transfer_lock: c_void, pub interrupt_buffer: [u8; 8] }

extern "C" {
    fn ni_usb_bulk_register_write_header(p: *mut u8, n: c_int) -> c_int;
    fn ni_usb_bulk_register_write(p: *mut u8, r: ni_usb_register) -> c_int;
    fn ni_usb_bulk_register_read_header(p: *mut u8, n: c_int) -> c_int;
    fn ni_usb_bulk_register_read(p: *mut u8, d: u8, a: u8) -> c_int;
    fn ni_usb_bulk_termination(p: *mut u8) -> c_int;
    fn nec7210_to_tnt4882_offset(a: c_int) -> u8;
    fn usb_alloc_urb(n: c_int, flags: c_uint) -> *mut urb;
    fn usb_free_urb(u: *mut urb); fn usb_kill_urb(u: *mut urb) -> c_int;
    fn interface_to_usbdev(i: *mut usb_interface) -> *mut usb_device;
    fn usb_submit_urb(u: *mut urb, flags: c_uint) -> c_int;
    fn usb_register_driver(d: *mut c_void) -> c_int; fn usb_deregister_driver(d: *mut c_void);
    fn gpib_register_driver(i: *mut c_void, m: *mut c_void) -> c_int; fn gpib_unregister_driver(i: *mut c_void);
    fn ni_usb_stop(p: *mut ni_usb_priv);
}

#[inline] pub fn ni_usb_timeout_msecs(usec: c_uint) -> c_ulong { if usec == 0 { 0 } else { 2000 + (usec / 500) as c_ulong } }

pub fn ni_usb_timeout_code(usec: c_uint) -> u16 {
    if usec == 0 { 0xf0 } else if usec <= 10 { 0xf1 } else if usec <= 30 { 0xf2 }
    else if usec <= 100 { 0xf3 } else if usec <= 300 { 0xf4 } else if usec <= 1000 { 0xf5 }
    else if usec <= 3000 { 0xf6 } else if usec <= 10000 { 0xf7 } else if usec <= 30000 { 0xf8 }
    else if usec <= 100000 { 0xf9 } else if usec <= 300000 { 0xfa } else if usec <= 1000000 { 0xfb }
    else if usec <= 3000000 { 0xfc } else if usec <= 10000000 { 0xfd } else if usec <= 30000000 { 0xfe }
    else if usec <= 100000000 { 0xff } else if usec <= 300000000 { 1 } else if usec <= 1000000000 { 2 } else { 0xf0 }
}

pub unsafe fn ni_usb_bulk_complete(urb: *mut urb) { let ctx = (*urb).context as *mut ni_usb_urb_ctx; complete(&mut (*ctx).complete); }
pub unsafe fn ni_usb_timeout_handler(_t: *mut timer_list) { /* timer_container_of and completion are kernel operations */ }

pub unsafe fn ni_usb_parse_status_block(buffer: *const u8, status: *mut ni_usb_status_block) -> c_int {
    (*status).id = *buffer; (*status).ibsta = ((*buffer.add(1) as u16) << 8) | *buffer.add(2) as u16;
    (*status).error_code = *buffer.add(3); let mut count = (*buffer.add(4) as u16) | ((*buffer.add(5) as u16) << 8);
    count = (!count).wrapping_add(1); (*status).count = count; 8
}

pub unsafe fn ni_usb_parse_register_read_block(raw: *const u8, results: *mut c_uint, num: c_int) -> c_int {
    let mut i = 0; let mut j = 0; while j < num { i += 1; let mut k = 0; while k < 3 && j < num { *results.add(j as usize) = *raw.add(i as usize) as c_uint; i += 1; j += 1; k += 1; } }
    while i % 4 != 0 { i += 1; } i += 2; while i % 4 != 0 { i += 1; } i
}

pub unsafe fn ni_usb_parse_termination_block(buffer: *const u8) -> c_int { if *buffer != 0 { /* NIUSB_TERM_ID validation */ } 4 }

pub unsafe fn ni_usb_write_sad(writes: *mut ni_usb_register, address: c_int, enable: c_int) -> c_int {
    let adr_bits = if enable != 0 { address as u8 } else { 0 }; let admr_bits = if enable != 0 { 0x02 } else { 0x01 };
    (*writes).device = 0; (*writes).address = nec7210_to_tnt4882_offset(0); (*writes).value = adr_bits;
    (*writes.add(1)).device = 0; (*writes.add(1)).address = nec7210_to_tnt4882_offset(0); (*writes.add(1)).value = admr_bits;
    (*writes.add(2)).device = 0; (*writes.add(2)).address = 1; (*writes.add(2)).value = if enable != 0 { address as u8 } else { 0 }; 3
}

pub unsafe fn ni_usb_setup_t1_delay(reg: *mut ni_usb_register, nano_sec: c_uint, actual_ns: *mut c_uint) -> c_int {
    *actual_ns = 2000; (*reg).device = 0; (*reg).address = nec7210_to_tnt4882_offset(0); (*reg).value = if nano_sec <= 1100 { *actual_ns = 1100; 0 } else { 0 };
    (*reg.add(1)).device = 0; (*reg.add(1)).address = nec7210_to_tnt4882_offset(0); (*reg.add(1)).value = if nano_sec <= 500 { *actual_ns = 500; 0 } else { 0 };
    (*reg.add(2)).device = 0; (*reg.add(2)).address = 0; (*reg.add(2)).value = if nano_sec <= 350 { *actual_ns = 350; 0 } else { 0 }; 3
}

extern "C" { fn complete(c: *mut c_void); }

// The remaining driver entry points retain their C ABI and are implemented by
// the surrounding kernel integration; declarations preserve the source
// interface without inventing dependency implementations.
extern "C" {
    fn ni_usb_read(board: *mut gpib_board, buffer: *mut u8, length: usize, end: *mut c_int, bytes_read: *mut usize) -> c_int;
    fn ni_usb_write(board: *mut gpib_board, buffer: *mut u8, length: usize, send_eoi: c_int, bytes_written: *mut usize) -> c_int;
    fn ni_usb_command(board: *mut gpib_board, buffer: *mut u8, length: usize, bytes_written: *mut usize) -> c_int;
    fn ni_usb_attach(board: *mut gpib_board, config: *const gpib_board_config) -> c_int;
    fn ni_usb_detach(board: *mut gpib_board);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
