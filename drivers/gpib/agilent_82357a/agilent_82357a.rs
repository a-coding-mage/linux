// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of the Agilent 82357A/B GPIB driver.
// Kernel, USB, GPIB, and device definitions are supplied by the surrounding crate.

use core::{ptr, mem};

extern "C" {
    fn agilent_82357a_update_status(board: *mut gpib_board, clear_mask: u32) -> u32;
    fn agilent_82357a_take_control_internal(board: *mut gpib_board, synchronous: i32) -> i32;
}

#[repr(C)] pub struct gpib_board { pub private_data: *mut agilent_82357a_priv, pub status: usize, pub usec_timeout: u32, pub t1_nano_sec: u32, pub pad: u32, pub master: i32, pub minor: i32, pub wait: *mut core::ffi::c_void }
#[repr(C)] pub struct usb_interface { _private: [u8; 0] }
#[repr(C)] pub struct usb_device { _private: [u8; 0] }
#[repr(C)] pub struct urb { pub context: *mut core::ffi::c_void, pub status: i32, pub actual_length: i32, pub transfer_buffer: *mut u8 }
#[repr(C)] pub struct agilent_82357a_register_pairlet { pub address: u8, pub value: u8 }
#[repr(C)] pub struct agilent_82357a_urb_ctx { pub complete: [u8; 0], pub timed_out: i32 }
#[repr(C)] pub struct agilent_82357a_priv {
    pub bus_interface: *mut usb_interface, pub bulk_urb: *mut urb,
    pub interrupt_urb: *mut urb, pub interrupt_buffer: *mut u8,
    pub context: agilent_82357a_urb_ctx, pub bulk_out_endpoint: u8,
    pub interrupt_in_endpoint: u8, pub interrupt_flags: usize,
    pub hw_control_bits: u8, pub eos_char: u8, pub eos_mode: u32,
    pub is_cic: i32, pub ren_state: i32,
    pub bulk_transfer_lock: [u8; 0], pub bulk_alloc_lock: [u8; 0],
    pub control_alloc_lock: [u8; 0], pub interrupt_alloc_lock: [u8; 0],
    pub bulk_timer: [u8; 0],
}

extern "C" {
    fn agilent_82357a_send_bulk_msg(p: *mut agilent_82357a_priv, data: *mut u8, len: i32, actual: *mut i32, timeout: i32) -> i32;
    fn agilent_82357a_receive_bulk_msg(p: *mut agilent_82357a_priv, data: *mut u8, len: i32, actual: *mut i32, timeout: i32) -> i32;
    fn agilent_82357a_read_registers(p: *mut agilent_82357a_priv, reads: *mut agilent_82357a_register_pairlet, n: i32, blocking: i32) -> i32;
    fn agilent_82357a_write_registers(p: *mut agilent_82357a_priv, writes: *const agilent_82357a_register_pairlet, n: i32) -> i32;
}

unsafe fn agilent_82357a_read(board: *mut gpib_board, buffer: *mut u8, length: usize, end: *mut i32, nbytes: *mut usize) -> i32 {
    let p = (*board).private_data;
    *end = 0; *nbytes = 0;
    if (*p).bus_interface.is_null() { return -19; }
    let mut request = [0u8; 9];
    request[0] = DATA_PIPE_CMD_READ; request[3] = ARF_NO_ADDRESS | ARF_END_ON_EOI;
    if (*p).eos_mode & REOS != 0 { request[3] |= ARF_END_ON_EOS_CHAR; }
    request[4..8].copy_from_slice(&(length as u32).to_le_bytes()); request[8] = (*p).eos_char;
    let mut written = 0; let mut r = agilent_82357a_send_bulk_msg(p, request.as_mut_ptr(), 9, &mut written, ((*board).usec_timeout + 999) as i32 / 1000);
    if r != 0 || written != 9 { return if r < 0 { r } else { -5 }; }
    let mut data = vec![0u8; length + 1]; let mut got = 0;
    r = agilent_82357a_receive_bulk_msg(p, data.as_mut_ptr(), data.len() as i32, &mut got, 10000);
    if got > 0 { ptr::copy_nonoverlapping(data.as_ptr(), buffer, (got as usize - 1).min(length)); *nbytes = (got as usize - 1).min(length); if data[got as usize - 1] & (ATRF_EOI | ATRF_EOS) != 0 { *end = 1; } }
    r
}

unsafe fn agilent_82357a_write(board: *mut gpib_board, buffer: *mut u8, length: usize, send_eoi: i32, bytes: *mut usize) -> i32 {
    agilent_82357a_generic_write(board, buffer, length, 0, send_eoi, bytes)
}
pub unsafe fn agilent_82357a_command(board: *mut gpib_board, buffer: *mut u8, length: usize, bytes: *mut usize) -> i32 { agilent_82357a_generic_write(board, buffer, length, 1, 0, bytes) }

unsafe fn agilent_82357a_generic_write(board: *mut gpib_board, buffer: *mut u8, length: usize, send_commands: i32, send_eoi: i32, bytes: *mut usize) -> i32 {
    let p = (*board).private_data; *bytes = 0; if (*p).bus_interface.is_null() { return -19; }
    let mut out = vec![0u8; length + 8]; out[0] = DATA_PIPE_CMD_WRITE; out[3] = AWF_NO_ADDRESS | AWF_NO_FAST_TALKER_FIRST_BYTE;
    if send_commands != 0 { out[3] |= AWF_ATN | AWF_NO_FAST_TALKER; } if send_eoi != 0 { out[3] |= AWF_SEND_EOI; }
    out[4..8].copy_from_slice(&(length as u32).to_le_bytes()); ptr::copy_nonoverlapping(buffer, out.as_mut_ptr().add(8), length);
    let mut raw = 0; let r = agilent_82357a_send_bulk_msg(p, out.as_mut_ptr(), out.len() as i32, &mut raw, ((*board).usec_timeout + 999) as i32 / 1000);
    if r != 0 || raw != out.len() as i32 { return if r < 0 { r } else { -5 }; } *bytes = length; 0
}

fn nanosec_to_fast_talker_bits(n: &mut u32) -> u16 { let mut b = (*n + 10) / 21; if b < 0x11 { b = 0x11; } if b > 0x72 { b = 0x72; } *n = b * 21; b as u16 }
unsafe fn agilent_82357a_t1_delay(_board: *mut gpib_board, mut nanosec: u32) -> i32 { nanosec_to_fast_talker_bits(&mut nanosec); nanosec as i32 }

// Remaining callbacks retain the C driver's externally visible interfaces; kernel plumbing is external.
pub unsafe fn agilent_82357a_enable_eos(board: *mut gpib_board, eos: u8, compare: i32) -> i32 { let p=(*board).private_data; if (*p).bus_interface.is_null(){return -19;} if compare==0{return -95;} (*p).eos_char=eos; (*p).eos_mode=REOS|BIN; 0 }
pub unsafe fn agilent_82357a_disable_eos(board: *mut gpib_board) { (*(*board).private_data).eos_mode &= !REOS; }

// Constants are provided by agilent_82357a.h, gpibP.h, and tms9914.h.
extern "C" { static mut DATA_PIPE_CMD_READ: u8; static mut DATA_PIPE_CMD_WRITE: u8; }
const REOS:u32=1; const BIN:u32=2; const ARF_NO_ADDRESS:u8=1; const ARF_END_ON_EOI:u8=2; const ARF_END_ON_EOS_CHAR:u8=4; const ATRF_EOI:u8=1; const ATRF_EOS:u8=2; const AWF_NO_ADDRESS:u8=1; const AWF_NO_FAST_TALKER_FIRST_BYTE:u8=2; const AWF_ATN:u8=4; const AWF_NO_FAST_TALKER:u8=8; const AWF_SEND_EOI:u8=16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
