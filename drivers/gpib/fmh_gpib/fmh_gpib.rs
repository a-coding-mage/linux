// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of fmh_gpib.c. Kernel and GPIB symbols
// referenced below are supplied by the surrounding driver environment.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// C headers: fmh_gpib.h, gpibP.h, and Linux kernel headers.
extern "C" {
    fn nec7210_read(board: *mut gpib_board, priv_: *mut nec7210_priv, buffer: *mut u8, length: usize, end: *mut i32, bytes_read: *mut usize) -> i32;
    fn nec7210_write(board: *mut gpib_board, priv_: *mut nec7210_priv, buffer: *mut u8, length: usize, send_eoi: i32, bytes_written: *mut usize) -> i32;
    fn nec7210_command(board: *mut gpib_board, priv_: *mut nec7210_priv, buffer: *mut u8, length: usize, bytes_written: *mut usize) -> i32;
    fn nec7210_take_control(board: *mut gpib_board, priv_: *mut nec7210_priv, synchronous: i32) -> i32;
    fn nec7210_go_to_standby(board: *mut gpib_board, priv_: *mut nec7210_priv) -> i32;
    fn nec7210_request_system_control(board: *mut gpib_board, priv_: *mut nec7210_priv, request_control: i32) -> i32;
    fn nec7210_interface_clear(board: *mut gpib_board, priv_: *mut nec7210_priv, assert_: i32);
    fn nec7210_remote_enable(board: *mut gpib_board, priv_: *mut nec7210_priv, enable: i32);
    fn nec7210_enable_eos(board: *mut gpib_board, priv_: *mut nec7210_priv, eos: u8, compare: i32) -> i32;
    fn nec7210_disable_eos(board: *mut gpib_board, priv_: *mut nec7210_priv);
    fn nec7210_update_status(board: *mut gpib_board, priv_: *mut nec7210_priv, mask: u32) -> u32;
    fn nec7210_primary_address(board: *mut gpib_board, priv_: *mut nec7210_priv, address: u32) -> i32;
    fn nec7210_secondary_address(board: *mut gpib_board, priv_: *mut nec7210_priv, address: u32, enable: i32) -> i32;
    fn nec7210_parallel_poll(board: *mut gpib_board, priv_: *mut nec7210_priv, result: *mut u8) -> i32;
    fn nec7210_parallel_poll_configure(board: *mut gpib_board, priv_: *mut nec7210_priv, configuration: u8);
    fn nec7210_parallel_poll_response(board: *mut gpib_board, priv_: *mut nec7210_priv, ist: i32);
    fn nec7210_serial_poll_status(board: *mut gpib_board, priv_: *mut nec7210_priv) -> u8;
    fn nec7210_t1_delay(board: *mut gpib_board, priv_: *mut nec7210_priv, ns: u32) -> i32;
    fn write_byte(priv_: *mut nec7210_priv, value: u32, reg: u32);
}

#[repr(C)] pub struct nec7210_priv { pub srq_pending: i32, pub state: usize, pub mmiobase: *mut c_void, pub offset: i32, pub read_byte: Option<unsafe extern "C" fn(*mut nec7210_priv, u32) -> u8>, pub write_byte: Option<unsafe extern "C" fn(*mut nec7210_priv, u8, u32)>, pub r#type: i32 }
#[repr(C)] pub struct fmh_priv { pub nec7210_priv: nec7210_priv, pub dma_buffer_size: usize, pub dma_buffer: *mut u8, pub dma_channel: *mut c_void, pub dma_burst_length: u32, pub fifo_base: *mut c_void, pub gpib_iomem_res: *mut c_void, pub dma_port_res: *mut c_void, pub irq: i32, pub supports_fifo_interrupts: i32 }
#[repr(C)] pub struct gpib_board { pub private_data: *mut fmh_priv, pub status: usize, pub spinlock: usize, pub wait: usize, pub dev: *mut c_void, pub gpib_dev: *mut c_void }
#[repr(C)] pub struct gpib_board_config { pub device_path: *const i8, pub serial_number: *const i8 }

pub unsafe fn fmh_gpib_read(b: *mut gpib_board, p: *mut u8, n: usize, e: *mut i32, r: *mut usize) -> i32 { nec7210_read(b, &mut (*(*b).private_data).nec7210_priv, p, n, e, r) }
pub unsafe fn fmh_gpib_write(b: *mut gpib_board, p: *mut u8, n: usize, eoi: i32, w: *mut usize) -> i32 { nec7210_write(b, &mut (*(*b).private_data).nec7210_priv, p, n, eoi, w) }
pub unsafe fn fmh_gpib_command(b: *mut gpib_board, p: *mut u8, n: usize, w: *mut usize) -> i32 { nec7210_command(b, &mut (*(*b).private_data).nec7210_priv, p, n, w) }
pub unsafe fn fmh_gpib_take_control(b: *mut gpib_board, s: i32) -> i32 { nec7210_take_control(b, &mut (*(*b).private_data).nec7210_priv, s) }
pub unsafe fn fmh_gpib_go_to_standby(b: *mut gpib_board) -> i32 { nec7210_go_to_standby(b, &mut (*(*b).private_data).nec7210_priv) }
pub unsafe fn fmh_gpib_request_system_control(b: *mut gpib_board, r: i32) -> i32 { nec7210_request_system_control(b, &mut (*(*b).private_data).nec7210_priv, r) }
pub unsafe fn fmh_gpib_interface_clear(b: *mut gpib_board, a: i32) { nec7210_interface_clear(b, &mut (*(*b).private_data).nec7210_priv, a) }
pub unsafe fn fmh_gpib_remote_enable(b: *mut gpib_board, e: i32) { nec7210_remote_enable(b, &mut (*(*b).private_data).nec7210_priv, e) }
pub unsafe fn fmh_gpib_enable_eos(b: *mut gpib_board, e: u8, c: i32) -> i32 { nec7210_enable_eos(b, &mut (*(*b).private_data).nec7210_priv, e, c) }
pub unsafe fn fmh_gpib_disable_eos(b: *mut gpib_board) { nec7210_disable_eos(b, &mut (*(*b).private_data).nec7210_priv) }
pub unsafe fn fmh_gpib_update_status(b: *mut gpib_board, m: u32) -> u32 { nec7210_update_status(b, &mut (*(*b).private_data).nec7210_priv, m) }
pub unsafe fn fmh_gpib_primary_address(b: *mut gpib_board, a: u32) -> i32 { nec7210_primary_address(b, &mut (*(*b).private_data).nec7210_priv, a) }
pub unsafe fn fmh_gpib_secondary_address(b: *mut gpib_board, a: u32, e: i32) -> i32 { nec7210_secondary_address(b, &mut (*(*b).private_data).nec7210_priv, a, e) }
pub unsafe fn fmh_gpib_parallel_poll(b: *mut gpib_board, r: *mut u8) -> i32 { nec7210_parallel_poll(b, &mut (*(*b).private_data).nec7210_priv, r) }
pub unsafe fn fmh_gpib_parallel_poll_configure(b: *mut gpib_board, c: u8) { nec7210_parallel_poll_configure(b, &mut (*(*b).private_data).nec7210_priv, c) }
pub unsafe fn fmh_gpib_parallel_poll_response(b: *mut gpib_board, i: i32) { nec7210_parallel_poll_response(b, &mut (*(*b).private_data).nec7210_priv, i) }
pub unsafe fn fmh_gpib_serial_poll_status(b: *mut gpib_board) -> u8 { nec7210_serial_poll_status(b, &mut (*(*b).private_data).nec7210_priv) }
pub unsafe fn fmh_gpib_t1_delay(b: *mut gpib_board, n: u32) -> i32 { nec7210_t1_delay(b, &mut (*(*b).private_data).nec7210_priv, n) }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
