/* SPDX-License-Identifier: GPL-2.0 */
/* sunserialcore.h
 *
 * Generic SUN serial/kbd/ms layer.  Based entirely
 * upon drivers/sbus/char/sunserial.h which is:
 *
 * Copyright (C) 1997  Eddie C. Dost  (ecd@skynet.be)
 *
 * Port to new UART layer is:
 *
 * Copyright (C) 2002 David S. Miller (davem@redhat.com)
 */

/* C header guard: _SERIAL_SUN_H */

/* Dependencies supplied by the surrounding kernel translation. */

/* Serial keyboard defines for L1-A processing... */
pub const SUNKBD_RESET: u32 = 0xff;
pub const SUNKBD_L1: u32 = 0x01;
pub const SUNKBD_UP: u32 = 0x80;
pub const SUNKBD_A: u32 = 0x4d;

extern "C" {
    pub fn suncore_mouse_baud_cflag_next(arg0: u32, arg1: *mut i32) -> u32;
    pub fn suncore_mouse_baud_detection(arg0: u8, arg1: i32) -> i32;

    pub fn sunserial_register_minors(arg0: *mut uart_driver, arg1: i32) -> i32;
    pub fn sunserial_unregister_minors(arg0: *mut uart_driver, arg1: i32);

    pub fn sunserial_console_match(
        arg0: *mut console,
        arg1: *mut device_node,
        arg2: *mut uart_driver,
        arg3: i32,
        arg4: bool,
    ) -> i32;
    pub fn sunserial_console_termios(arg0: *mut console, arg1: *mut device_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
