/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *    copyright            : (C) 2002 by Frank Mori Hess
 ***************************************************************************/

// Dependencies supplied by the surrounding translation unit:
// gpib_state_machines.h, linux/types.h, linux/spinlock.h, linux/string.h,
// linux/interrupt.h, gpib_types.h, and nec7210_registers.h.

use core::ffi::{c_ulong, c_void};

/* struct used to provide variables local to a nec7210 chip */
#[repr(C)]
pub struct nec7210_priv {
    #[cfg(CONFIG_HAS_IOPORT)]
    pub iobase: u32,
    pub mmiobase: *mut c_void,
    pub offset: u32, // offset between successive nec7210 io addresses
    pub dma_channel: u32,
    pub dma_buffer: *mut u8,
    pub dma_buffer_length: u32, // length of dma buffer
    pub dma_buffer_addr: dma_addr_t, // bus address of board->buffer for use with dma
    // software copy of bits written to registers
    pub reg_bits: [u8; 8],
    pub auxa_bits: u8, // bits written to auxiliary register A
    pub auxb_bits: u8, // bits written to auxiliary register B
    // used to keep track of board's state, bit definitions given below
    pub state: c_ulong,
    // lock for chips that extend the nec7210 registers by paging in alternate regs
    pub register_page_lock: spinlock_t,
    // wrappers for outb, inb, readb, or writeb
    pub read_byte: Option<unsafe extern "C" fn(*mut nec7210_priv, u32) -> u8>,
    pub write_byte: Option<unsafe extern "C" fn(*mut nec7210_priv, u8, u32)>,
    pub r#type: nec7210_chipset,
    pub talker_state: talker_function_state,
    pub listener_state: listener_function_state,
    pub private: *mut c_void,
    pub srq_pending: u8, // C bit-field: unsigned srq_pending : 1
}

pub unsafe fn init_nec7210_private(priv_: *mut nec7210_priv) {
    core::ptr::write_bytes(priv_ as *mut u8, 0, core::mem::size_of::<nec7210_priv>());
    spin_lock_init(core::ptr::addr_of_mut!((*priv_).register_page_lock));
}

// slightly shorter way to access read_byte and write_byte
pub unsafe fn read_byte(priv_: *mut nec7210_priv, register_number: u32) -> u8 {
    ((*priv_).read_byte.expect("read_byte callback"))(priv_, register_number)
}

pub unsafe fn write_byte(priv_: *mut nec7210_priv, byte: u8, register_number: u32) {
    ((*priv_).write_byte.expect("write_byte callback"))(priv_, byte, register_number);
}

// struct nec7210_priv.state bit numbers
pub const PIO_IN_PROGRESS_BN: u32 = 0;
pub const DMA_READ_IN_PROGRESS_BN: u32 = 1;
pub const DMA_WRITE_IN_PROGRESS_BN: u32 = 2;
pub const READ_READY_BN: u32 = 3;
pub const WRITE_READY_BN: u32 = 4;
pub const COMMAND_READY_BN: u32 = 5;
pub const RECEIVED_END_BN: u32 = 6;
pub const BUS_ERROR_BN: u32 = 7;
pub const RFD_HOLDOFF_BN: u32 = 8;
pub const DEV_CLEAR_BN: u32 = 9;
pub const ADR_CHANGE_BN: u32 = 10;

extern "C" {
    pub fn nec7210_read(board: *mut gpib_board, priv_: *mut nec7210_priv, buffer: *mut u8,
                        length: usize, end: *mut i32, bytes_read: *mut usize) -> i32;
    pub fn nec7210_write(board: *mut gpib_board, priv_: *mut nec7210_priv, buffer: *mut u8,
                         length: usize, send_eoi: i32, bytes_written: *mut usize) -> i32;
    pub fn nec7210_command(board: *mut gpib_board, priv_: *mut nec7210_priv, buffer: *mut u8,
                           length: usize, bytes_written: *mut usize) -> i32;
    pub fn nec7210_take_control(board: *mut gpib_board, priv_: *mut nec7210_priv, syncronous: i32) -> i32;
    pub fn nec7210_go_to_standby(board: *mut gpib_board, priv_: *mut nec7210_priv) -> i32;
    pub fn nec7210_request_system_control(board: *mut gpib_board, priv_: *mut nec7210_priv, request_control: i32) -> i32;
    pub fn nec7210_interface_clear(board: *mut gpib_board, priv_: *mut nec7210_priv, assert_: i32);
    pub fn nec7210_remote_enable(board: *mut gpib_board, priv_: *mut nec7210_priv, enable: i32);
    pub fn nec7210_enable_eos(board: *mut gpib_board, priv_: *mut nec7210_priv, eos_bytes: u8, compare_8_bits: i32) -> i32;
    pub fn nec7210_disable_eos(board: *mut gpib_board, priv_: *mut nec7210_priv);
    pub fn nec7210_update_status(board: *mut gpib_board, priv_: *mut nec7210_priv, clear_mask: u32) -> u32;
    pub fn nec7210_update_status_nolock(board: *mut gpib_board, priv_: *mut nec7210_priv) -> u32;
    pub fn nec7210_primary_address(board: *const gpib_board, priv_: *mut nec7210_priv, address: u32) -> i32;
    pub fn nec7210_secondary_address(board: *const gpib_board, priv_: *mut nec7210_priv, address: u32, enable: i32) -> i32;
    pub fn nec7210_parallel_poll(board: *mut gpib_board, priv_: *mut nec7210_priv, result: *mut u8) -> i32;
    pub fn nec7210_serial_poll_response(board: *mut gpib_board, priv_: *mut nec7210_priv, status: u8);
    pub fn nec7210_parallel_poll_configure(board: *mut gpib_board, priv_: *mut nec7210_priv, configuration: u32);
    pub fn nec7210_parallel_poll_response(board: *mut gpib_board, priv_: *mut nec7210_priv, ist: i32);
    pub fn nec7210_serial_poll_status(board: *mut gpib_board, priv_: *mut nec7210_priv) -> u8;
    pub fn nec7210_t1_delay(board: *mut gpib_board, priv_: *mut nec7210_priv, nano_sec: u32) -> i32;
    pub fn nec7210_return_to_local(board: *const gpib_board, priv_: *mut nec7210_priv);

    pub fn nec7210_board_reset(priv_: *mut nec7210_priv, board: *const gpib_board);
    pub fn nec7210_board_online(priv_: *mut nec7210_priv, board: *const gpib_board);
    pub fn nec7210_set_reg_bits(priv_: *mut nec7210_priv, reg: u32, mask: u32, bits: u32) -> u32;
    pub fn nec7210_set_handshake_mode(board: *mut gpib_board, priv_: *mut nec7210_priv, mode: i32);
    pub fn nec7210_release_rfd_holdoff(board: *mut gpib_board, priv_: *mut nec7210_priv);
    pub fn nec7210_read_data_in(board: *mut gpib_board, priv_: *mut nec7210_priv, end: *mut i32) -> u8;

    pub fn nec7210_ioport_read_byte(priv_: *mut nec7210_priv, register_num: u32) -> u8;
    pub fn nec7210_ioport_write_byte(priv_: *mut nec7210_priv, data: u8, register_num: u32);
    pub fn nec7210_iomem_read_byte(priv_: *mut nec7210_priv, register_num: u32) -> u8;
    pub fn nec7210_iomem_write_byte(priv_: *mut nec7210_priv, data: u8, register_num: u32);
    pub fn nec7210_locking_ioport_read_byte(priv_: *mut nec7210_priv, register_num: u32) -> u8;
    pub fn nec7210_locking_ioport_write_byte(priv_: *mut nec7210_priv, data: u8, register_num: u32);
    pub fn nec7210_locking_iomem_read_byte(priv_: *mut nec7210_priv, register_num: u32) -> u8;
    pub fn nec7210_locking_iomem_write_byte(priv_: *mut nec7210_priv, data: u8, register_num: u32);

    pub fn nec7210_interrupt(board: *mut gpib_board, priv_: *mut nec7210_priv) -> irqreturn_t;
    pub fn nec7210_interrupt_have_status(board: *mut gpib_board, priv_: *mut nec7210_priv,
                                         status1: i32, status2: i32) -> irqreturn_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
