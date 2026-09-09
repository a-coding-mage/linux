/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding GPIB implementation: "gpibP.h".
// Linux kernel and architecture headers are likewise external dependencies.

pub const MAX_GPIB_PRIMARY_ADDRESS: i32 = 30;
pub const MAX_GPIB_SECONDARY_ADDRESS: i32 = 31;

// Opaque declarations corresponding to C struct types supplied by dependencies.
#[repr(C)]
pub struct gpib_board {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpib_status_queue {
    _private: [u8; 0],
}

extern "C" {
    pub fn gpib_allocate_board(board: *mut gpib_board) -> i32;
    pub fn gpib_deallocate_board(board: *mut gpib_board);

    pub fn num_status_bytes(dev: *const gpib_status_queue) -> u32;
    pub fn push_status_byte(
        board: *mut gpib_board,
        device: *mut gpib_status_queue,
        poll_byte: u8,
    ) -> i32;
    pub fn pop_status_byte(
        board: *mut gpib_board,
        device: *mut gpib_status_queue,
        poll_byte: *mut u8,
    ) -> i32;
    pub fn get_gpib_status_queue(
        board: *mut gpib_board,
        pad: u32,
        sad: i32,
    ) -> *mut gpib_status_queue;
    pub fn get_serial_poll_byte(
        board: *mut gpib_board,
        pad: u32,
        sad: i32,
        usec_timeout: u32,
        poll_byte: *mut u8,
    ) -> i32;
    pub fn autopoll_all_devices(board: *mut gpib_board) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
