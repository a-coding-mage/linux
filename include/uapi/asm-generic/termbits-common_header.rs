/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub type cc_t = u8;
pub type speed_t = u32;

/* c_iflag bits */
pub const IGNBRK: u32 = 0x001; /* Ignore break condition */
pub const BRKINT: u32 = 0x002; /* Signal interrupt on break */
pub const IGNPAR: u32 = 0x004; /* Ignore characters with parity errors */
pub const PARMRK: u32 = 0x008; /* Mark parity and framing errors */
pub const INPCK: u32 = 0x010; /* Enable input parity check */
pub const ISTRIP: u32 = 0x020; /* Strip 8th bit off characters */
pub const INLCR: u32 = 0x040; /* Map NL to CR on input */
pub const IGNCR: u32 = 0x080; /* Ignore CR */
pub const ICRNL: u32 = 0x100; /* Map CR to NL on input */
pub const IXANY: u32 = 0x800; /* Any character will restart after stop */

/* c_oflag bits */
pub const OPOST: u32 = 0x01; /* Perform output processing */
pub const OCRNL: u32 = 0x08;
pub const ONOCR: u32 = 0x10;
pub const ONLRET: u32 = 0x20;
pub const OFILL: u32 = 0x40;
pub const OFDEL: u32 = 0x80;

/* c_cflag bit meaning */
/* Common CBAUD rates */
pub const B0: u32 = 0x00000000; /* hang up */
pub const B50: u32 = 0x00000001;
pub const B75: u32 = 0x00000002;
pub const B110: u32 = 0x00000003;
pub const B134: u32 = 0x00000004;
pub const B150: u32 = 0x00000005;
pub const B200: u32 = 0x00000006;
pub const B300: u32 = 0x00000007;
pub const B600: u32 = 0x00000008;
pub const B1200: u32 = 0x00000009;
pub const B1800: u32 = 0x0000000a;
pub const B2400: u32 = 0x0000000b;
pub const B4800: u32 = 0x0000000c;
pub const B9600: u32 = 0x0000000d;
pub const B19200: u32 = 0x0000000e;
pub const B38400: u32 = 0x0000000f;
pub const EXTA: u32 = B19200;
pub const EXTB: u32 = B38400;

pub const ADDRB: u32 = 0x20000000; /* address bit */
pub const CMSPAR: u32 = 0x40000000; /* mark or space (stick) parity */
pub const CRTSCTS: u32 = 0x80000000; /* flow control */

pub const IBSHIFT: u32 = 16; /* Shift from CBAUD to CIBAUD */

/* tcflow() ACTION argument and TCXONC use these */
pub const TCOOFF: u32 = 0; /* Suspend output */
pub const TCOON: u32 = 1; /* Restart suspended output */
pub const TCIOFF: u32 = 2; /* Send a STOP character */
pub const TCION: u32 = 3; /* Send a START character */

/* tcflush() QUEUE_SELECTOR argument and TCFLSH use these */
pub const TCIFLUSH: u32 = 0; /* Discard data received but not yet read */
pub const TCOFLUSH: u32 = 1; /* Discard data written but not yet sent */
pub const TCIOFLUSH: u32 = 2; /* Discard all pending data */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
