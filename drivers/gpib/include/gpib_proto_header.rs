/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header.  The Linux kernel types and HZ are supplied
// by the surrounding build environment.

use core::ffi::{c_int, c_long, c_ulong};

pub enum inode {}
pub enum file {}
pub enum gpib_board {}
pub enum gpib_descriptor {}
pub enum gpib_status_queue {}

extern "C" {
    pub fn ibopen(inode: *mut inode, filep: *mut file) -> c_int;
    pub fn ibclose(inode: *mut inode, file: *mut file) -> c_int;
    pub fn ibioctl(filep: *mut file, cmd: u32, arg: c_ulong) -> c_long;
    pub fn os_start_timer(board: *mut gpib_board, usec_timeout: u32);
    pub fn os_remove_timer(board: *mut gpib_board);
    pub fn init_gpib_board(board: *mut gpib_board);

    pub fn serial_poll_all(board: *mut gpib_board, usec_timeout: u32) -> c_int;
    pub fn init_gpib_descriptor(desc: *mut gpib_descriptor);
    pub fn dvrsp(
        board: *mut gpib_board,
        pad: u32,
        sad: c_int,
        usec_timeout: u32,
        result: *mut u8,
    ) -> c_int;
    pub fn ibcac(board: *mut gpib_board, sync: c_int, fallback_to_async: c_int) -> c_int;
    pub fn ibcmd(
        board: *mut gpib_board,
        buf: *mut u8,
        length: usize,
        bytes_written: *mut usize,
    ) -> c_int;
    pub fn ibgts(board: *mut gpib_board) -> c_int;
    pub fn ibonline(board: *mut gpib_board) -> c_int;
    pub fn iboffline(board: *mut gpib_board) -> c_int;
    pub fn iblines(board: *const gpib_board, lines: *mut i16) -> c_int;
    pub fn ibrd(
        board: *mut gpib_board,
        buf: *mut u8,
        length: usize,
        end_flag: *mut c_int,
        bytes_read: *mut usize,
    ) -> c_int;
    pub fn ibrpp(board: *mut gpib_board, buf: *mut u8) -> c_int;
    pub fn ibrsv2(board: *mut gpib_board, status_byte: u8, new_reason_for_service: c_int) -> c_int;
    pub fn ibrsc(board: *mut gpib_board, request_control: c_int) -> c_int;
    pub fn ibsic(board: *mut gpib_board, usec_duration: u32) -> c_int;
    pub fn ibsre(board: *mut gpib_board, enable: c_int) -> c_int;
    pub fn ibpad(board: *mut gpib_board, addr: u32) -> c_int;
    pub fn ibsad(board: *mut gpib_board, addr: c_int) -> c_int;
    pub fn ibeos(board: *mut gpib_board, eos: c_int, eosflags: c_int) -> c_int;
    pub fn ibwait(
        board: *mut gpib_board,
        wait_mask: c_int,
        clear_mask: c_int,
        set_mask: c_int,
        status: *mut c_int,
        usec_timeout: c_ulong,
        desc: *mut gpib_descriptor,
    ) -> c_int;
    pub fn ibwrt(
        board: *mut gpib_board,
        buf: *mut u8,
        cnt: usize,
        send_eoi: c_int,
        bytes_written: *mut usize,
    ) -> c_int;
    pub fn ibstatus(board: *mut gpib_board) -> c_int;
    pub fn general_ibstatus(
        board: *mut gpib_board,
        device: *const gpib_status_queue,
        clear_mask: c_int,
        set_mask: c_int,
        desc: *mut gpib_descriptor,
    ) -> c_int;
    pub fn io_timed_out(board: *mut gpib_board) -> c_int;
    pub fn ibppc(board: *mut gpib_board, configuration: u8) -> c_int;
}

#[inline]
pub unsafe fn usec_to_jiffies(usec: u32) -> c_ulong {
    let usec_per_jiffy: c_ulong = 1_000_000 / HZ;

    1 + (usec as c_ulong + usec_per_jiffy - 1) / usec_per_jiffy
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
