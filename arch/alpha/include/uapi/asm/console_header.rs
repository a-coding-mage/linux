/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Console callback routine numbers
 */
pub const CCB_GETC: u32 = 0x01;
pub const CCB_PUTS: u32 = 0x02;
pub const CCB_RESET_TERM: u32 = 0x03;
pub const CCB_SET_TERM_INT: u32 = 0x04;
pub const CCB_SET_TERM_CTL: u32 = 0x05;
pub const CCB_PROCESS_KEYCODE: u32 = 0x06;
pub const CCB_OPEN_CONSOLE: u32 = 0x07;
pub const CCB_CLOSE_CONSOLE: u32 = 0x08;

pub const CCB_OPEN: u32 = 0x10;
pub const CCB_CLOSE: u32 = 0x11;
pub const CCB_IOCTL: u32 = 0x12;
pub const CCB_READ: u32 = 0x13;
pub const CCB_WRITE: u32 = 0x14;

pub const CCB_SET_ENV: u32 = 0x20;
pub const CCB_RESET_ENV: u32 = 0x21;
pub const CCB_GET_ENV: u32 = 0x22;
pub const CCB_SAVE_ENV: u32 = 0x23;

pub const CCB_PSWITCH: u32 = 0x30;
pub const CCB_BIOS_EMUL: u32 = 0x32;

/*
 * Environment variable numbers
 */
pub const ENV_AUTO_ACTION: u32 = 0x01;
pub const ENV_BOOT_DEV: u32 = 0x02;
pub const ENV_BOOTDEF_DEV: u32 = 0x03;
pub const ENV_BOOTED_DEV: u32 = 0x04;
pub const ENV_BOOT_FILE: u32 = 0x05;
pub const ENV_BOOTED_FILE: u32 = 0x06;
pub const ENV_BOOT_OSFLAGS: u32 = 0x07;
pub const ENV_BOOTED_OSFLAGS: u32 = 0x08;
pub const ENV_BOOT_RESET: u32 = 0x09;
pub const ENV_DUMP_DEV: u32 = 0x0A;
pub const ENV_ENABLE_AUDIT: u32 = 0x0B;
pub const ENV_LICENSE: u32 = 0x0C;
pub const ENV_CHAR_SET: u32 = 0x0D;
pub const ENV_LANGUAGE: u32 = 0x0E;
pub const ENV_TTY_DEV: u32 = 0x0F;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
