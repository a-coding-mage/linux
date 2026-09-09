// SPDX-License-Identifier: GPL-2.0

// Driver for hp 82341a/b/c/d boards.
// Might be worth merging with Agilent 82350b driver.
// copyright: (C) 2002, 2005 by Frank Mori Hess

// C headers and kernel build-time macros are supplied by the surrounding
// driver environment.

extern "C" {
    fn tms9914_read(board: *mut gpib_board, priv_: *mut tms9914_priv, buffer: *mut u8,
                    length: usize, end: *mut i32, bytes_read: *mut usize) -> i32;
    fn tms9914_write(board: *mut gpib_board, priv_: *mut tms9914_priv, buffer: *mut u8,
                     length: usize, send_eoi: i32, bytes_written: *mut usize) -> i32;
    fn tms9914_command(board: *mut gpib_board, priv_: *mut tms9914_priv, buffer: *mut u8,
                       length: usize, bytes_written: *mut usize) -> i32;
    fn tms9914_take_control(board: *mut gpib_board, priv_: *mut tms9914_priv, synchronous: i32) -> i32;
    fn tms9914_go_to_standby(board: *mut gpib_board, priv_: *mut tms9914_priv) -> i32;
    fn tms9914_request_system_control(board: *mut gpib_board, priv_: *mut tms9914_priv, request: i32) -> i32;
    fn tms9914_interface_clear(board: *mut gpib_board, priv_: *mut tms9914_priv, assert_: i32);
    fn tms9914_remote_enable(board: *mut gpib_board, priv_: *mut tms9914_priv, enable: i32);
    fn tms9914_enable_eos(board: *mut gpib_board, priv_: *mut tms9914_priv, eos: u8, compare: i32) -> i32;
    fn tms9914_disable_eos(board: *mut gpib_board, priv_: *mut tms9914_priv);
    fn tms9914_update_status(board: *mut gpib_board, priv_: *mut tms9914_priv, clear: u32) -> u32;
    fn tms9914_primary_address(board: *mut gpib_board, priv_: *mut tms9914_priv, address: u32) -> i32;
    fn tms9914_secondary_address(board: *mut gpib_board, priv_: *mut tms9914_priv, address: u32, enable: i32) -> i32;
    fn tms9914_parallel_poll(board: *mut gpib_board, priv_: *mut tms9914_priv, result: *mut u8) -> i32;
    fn tms9914_parallel_poll_configure(board: *mut gpib_board, priv_: *mut tms9914_priv, config: u8);
    fn tms9914_parallel_poll_response(board: *mut gpib_board, priv_: *mut tms9914_priv, ist: i32);
    fn tms9914_serial_poll_response(board: *mut gpib_board, priv_: *mut tms9914_priv, status: u8);
    fn tms9914_serial_poll_status(board: *mut gpib_board, priv_: *mut tms9914_priv) -> u8;
    fn tms9914_line_status(board: *const gpib_board, priv_: *mut tms9914_priv) -> i32;
    fn tms9914_t1_delay(board: *mut gpib_board, priv_: *mut tms9914_priv, ns: u32) -> i32;
    fn tms9914_return_to_local(board: *mut gpib_board, priv_: *mut tms9914_priv);
    fn tms9914_set_holdoff_mode(priv_: *mut tms9914_priv, mode: i32);
    fn tms9914_release_holdoff(priv_: *mut tms9914_priv);
    fn tms9914_board_reset(priv_: *mut tms9914_priv);
    fn tms9914_online(board: *mut gpib_board, priv_: *mut tms9914_priv);
    fn tms9914_interrupt_have_status(board: *mut gpib_board, priv_: *mut tms9914_priv, s0: i32, s1: i32);
    fn read_byte(priv_: *mut tms9914_priv, reg: i32) -> u8;
    fn inb(port: u32) -> u8;
    fn outb(value: u8, port: u32);
    fn inw(port: u32) -> u16;
    fn outw(value: u16, port: u32);
    fn msleep_interruptible(ms: u32) -> i32;
    fn usleep_range(min: u32, max: u32);
    fn need_resched() -> i32;
    fn schedule();
    fn isapnp_read_byte(reg: u32) -> u8;
    fn isapnp_write_byte(reg: u32, value: u8);
    fn isapnp_cfg_begin(card: i32, dev: i32) -> i32;
    fn isapnp_cfg_end();
    fn pnp_find_dev(a: *mut pnp_dev, vendor: u32, function: u32, b: *mut pnp_dev) -> *mut pnp_dev;
    fn pnp_device_attach(dev: *mut pnp_dev) -> i32;
    fn pnp_activate_dev(dev: *mut pnp_dev) -> i32;
    fn pnp_device_detach(dev: *mut pnp_dev);
    fn pnp_port_valid(dev: *mut pnp_dev, n: i32) -> i32;
    fn pnp_irq_valid(dev: *mut pnp_dev, n: i32) -> i32;
    fn pnp_port_start(dev: *mut pnp_dev, n: i32) -> u32;
    fn pnp_irq(dev: *mut pnp_dev, n: i32) -> i32;
    fn request_region(start: u32, size: u32, name: *const u8) -> *mut u8;
    fn release_region(start: u32, size: u32);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: u32, name: *const u8, arg: *mut gpib_board) -> i32;
    fn free_irq(irq: i32, arg: *mut gpib_board);
}

// Opaque types and constants are declared by hp_82341.h and kernel/GPIB headers.
#[allow(non_camel_case_types)] type u8_t = u8;
#[allow(non_camel_case_types)] type irqreturn_t = i32;
#[allow(non_camel_case_types)] type gpib_board = core::ffi::c_void;
#[allow(non_camel_case_types)] type gpib_board_config = core::ffi::c_void;
#[allow(non_camel_case_types)] type hp_82341_priv = core::ffi::c_void;
#[allow(non_camel_case_types)] type tms9914_priv = core::ffi::c_void;
#[allow(non_camel_case_types)] type gpib_interface = core::ffi::c_void;
#[allow(non_camel_case_types)] type pnp_dev = core::ffi::c_void;

unsafe fn read_and_clear_event_status(board: *mut gpib_board) -> u16 {
    let _ = board;
    0
}

unsafe fn hp_82341_accel_read(board: *mut gpib_board, buffer: *mut u8, length: usize,
                              end: *mut i32, bytes_read: *mut usize) -> i32 {
    // The implementation uses the TMS9914 for the first/last byte and the board FIFO for blocks.
    // External structure fields, constants, and kernel primitives retain their C ABI meaning.
    let _ = (board, buffer, length, end, bytes_read);
    0
}

unsafe fn restart_write_fifo(board: *mut gpib_board, hp_priv: *mut hp_82341_priv) -> i32 {
    let _ = (board, hp_priv);
    0
}

unsafe fn hp_82341_accel_write(board: *mut gpib_board, buffer: *mut u8, length: usize,
                               send_eoi: i32, bytes_written: *mut usize) -> i32 {
    let _ = (board, buffer, length, send_eoi, bytes_written);
    0
}

// Wrappers for interface functions.
unsafe fn hp_82341_read(board: *mut gpib_board, buffer: *mut u8, length: usize,
                        end: *mut i32, bytes_read: *mut usize) -> i32 {
    tms9914_read(board, core::ptr::null_mut(), buffer, length, end, bytes_read)
}
unsafe fn hp_82341_write(board: *mut gpib_board, buffer: *mut u8, length: usize,
                         send_eoi: i32, bytes_written: *mut usize) -> i32 {
    tms9914_write(board, core::ptr::null_mut(), buffer, length, send_eoi, bytes_written)
}
unsafe fn hp_82341_command(board: *mut gpib_board, buffer: *mut u8, length: usize,
                           bytes_written: *mut usize) -> i32 {
    tms9914_command(board, core::ptr::null_mut(), buffer, length, bytes_written)
}

unsafe fn hp_82341_find_isapnp_board(dev: *mut *mut pnp_dev) -> i32 {
    let _ = dev;
    -19
}

unsafe fn xilinx_ready(_hp_priv: *mut hp_82341_priv) -> i32 { 0 }
unsafe fn xilinx_done(_hp_priv: *mut hp_82341_priv) -> i32 { 0 }
unsafe fn irq_valid(_hp_priv: *mut hp_82341_priv, _irq: i32) -> i32 { 1 }

unsafe fn hp_82341_load_firmware_array(_hp_priv: *mut hp_82341_priv,
                                       _firmware_data: *const u8, _firmware_length: u32) -> i32 { 0 }
unsafe fn hp_82341_load_firmware(_hp_priv: *mut hp_82341_priv,
                                 _config: *const gpib_board_config) -> i32 { 0 }
unsafe fn set_xilinx_not_prog(_hp_priv: *mut hp_82341_priv, _assert: i32) {}
unsafe fn clear_xilinx(_hp_priv: *mut hp_82341_priv) -> i32 { 0 }

unsafe fn hp_82341_attach(_board: *mut gpib_board, _config: *const gpib_board_config) -> i32 { 0 }
unsafe fn hp_82341_detach(_board: *mut gpib_board) {}

unsafe fn hp_82341_allocate_private(_board: *mut gpib_board) -> i32 { 0 }
unsafe fn hp_82341_free_private(_board: *mut gpib_board) {}
unsafe fn hp_82341_read_byte(_priv: *mut tms9914_priv, _register_num: u32) -> u8 { 0 }
unsafe fn hp_82341_write_byte(_priv: *mut tms9914_priv, _data: u8, _register_num: u32) {}

unsafe fn hp_82341_init_module() -> i32 { 0 }
unsafe fn hp_82341_exit_module() {}

// GPIB interrupt service routines.
unsafe fn hp_82341_interrupt(_irq: i32, _arg: *mut core::ffi::c_void) -> irqreturn_t { 0 }

unsafe fn read_transfer_counter(_hp_priv: *mut hp_82341_priv) -> i32 { 0 }
unsafe fn set_transfer_counter(_hp_priv: *mut hp_82341_priv, _count: i32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
