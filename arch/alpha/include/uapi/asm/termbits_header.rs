/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency equivalent of <asm-generic/termbits-common.h> is supplied elsewhere.

pub type tcflag_t = u32;

/*
 * termios type and macro definitions.  Be careful about adding stuff
 * to this file since it's used in GNU libc and there are strict rules
 * concerning namespace pollution.
 */

pub const NCCS: usize = 19;

#[repr(C)]
pub struct termios {
	pub c_iflag: tcflag_t, /* input mode flags */
	pub c_oflag: tcflag_t, /* output mode flags */
	pub c_cflag: tcflag_t, /* control mode flags */
	pub c_lflag: tcflag_t, /* local mode flags */
	pub c_cc: [cc_t; NCCS], /* control characters */
	pub c_line: cc_t, /* line discipline (== c_cc[19]) */
	pub c_ispeed: speed_t, /* input speed */
	pub c_ospeed: speed_t, /* output speed */
}

/* Alpha has identical termios and termios2 */
#[repr(C)]
pub struct termios2 {
	pub c_iflag: tcflag_t, /* input mode flags */
	pub c_oflag: tcflag_t, /* output mode flags */
	pub c_cflag: tcflag_t, /* control mode flags */
	pub c_lflag: tcflag_t, /* local mode flags */
	pub c_cc: [cc_t; NCCS], /* control characters */
	pub c_line: cc_t, /* line discipline (== c_cc[19]) */
	pub c_ispeed: speed_t, /* input speed */
	pub c_ospeed: speed_t, /* output speed */
}

/* Alpha has matching termios and ktermios */
#[repr(C)]
pub struct ktermios {
	pub c_iflag: tcflag_t, /* input mode flags */
	pub c_oflag: tcflag_t, /* output mode flags */
	pub c_cflag: tcflag_t, /* control mode flags */
	pub c_lflag: tcflag_t, /* local mode flags */
	pub c_cc: [cc_t; NCCS], /* control characters */
	pub c_line: cc_t, /* line discipline (== c_cc[19]) */
	pub c_ispeed: speed_t, /* input speed */
	pub c_ospeed: speed_t, /* output speed */
}

/* c_cc characters */
pub const VEOF: u32 = 0;
pub const VEOL: u32 = 1;
pub const VEOL2: u32 = 2;
pub const VERASE: u32 = 3;
pub const VWERASE: u32 = 4;
pub const VKILL: u32 = 5;
pub const VREPRINT: u32 = 6;
pub const VSWTC: u32 = 7;
pub const VINTR: u32 = 8;
pub const VQUIT: u32 = 9;
pub const VSUSP: u32 = 10;
pub const VSTART: u32 = 12;
pub const VSTOP: u32 = 13;
pub const VLNEXT: u32 = 14;
pub const VDISCARD: u32 = 15;
pub const VMIN: u32 = 16;
pub const VTIME: u32 = 17;

/* c_iflag bits */
pub const IXON: u32 = 0x0200;
pub const IXOFF: u32 = 0x0400;
pub const IUCLC: u32 = 0x1000;
pub const IMAXBEL: u32 = 0x2000;
pub const IUTF8: u32 = 0x4000;

/* c_oflag bits */
pub const ONLCR: u32 = 0x00002;
pub const OLCUC: u32 = 0x00004;
pub const NLDLY: u32 = 0x00300;
pub const NL0: u32 = 0x00000;
pub const NL1: u32 = 0x00100;
pub const NL2: u32 = 0x00200;
pub const NL3: u32 = 0x00300;
pub const TABDLY: u32 = 0x00c00;
pub const TAB0: u32 = 0x00000;
pub const TAB1: u32 = 0x00400;
pub const TAB2: u32 = 0x00800;
pub const TAB3: u32 = 0x00c00;
pub const CRDLY: u32 = 0x03000;
pub const CR0: u32 = 0x00000;
pub const CR1: u32 = 0x01000;
pub const CR2: u32 = 0x02000;
pub const CR3: u32 = 0x03000;
pub const FFDLY: u32 = 0x04000;
pub const FF0: u32 = 0x00000;
pub const FF1: u32 = 0x04000;
pub const BSDLY: u32 = 0x08000;
pub const BS0: u32 = 0x00000;
pub const BS1: u32 = 0x08000;
pub const VTDLY: u32 = 0x10000;
pub const VT0: u32 = 0x00000;
pub const VT1: u32 = 0x10000;
/*
 * Should be equivalent to TAB3, see description of TAB3 in
 * POSIX.1-2008, Ch. 11.2.3 "Output Modes"
 */
pub const XTABS: u32 = TAB3;

/* c_cflag bit meaning */
pub const CBAUD: u32 = 0x0000001f;
pub const CBAUDEX: u32 = 0x00000000;
pub const BOTHER: u32 = 0x0000001f;
pub const B57600: u32 = 0x00000010;
pub const B115200: u32 = 0x00000011;
pub const B230400: u32 = 0x00000012;
pub const B460800: u32 = 0x00000013;
pub const B500000: u32 = 0x00000014;
pub const B576000: u32 = 0x00000015;
pub const B921600: u32 = 0x00000016;
pub const B1000000: u32 = 0x00000017;
pub const B1152000: u32 = 0x00000018;
pub const B1500000: u32 = 0x00000019;
pub const B2000000: u32 = 0x0000001a;
pub const B2500000: u32 = 0x0000001b;
pub const B3000000: u32 = 0x0000001c;
pub const B3500000: u32 = 0x0000001d;
pub const B4000000: u32 = 0x0000001e;
pub const CSIZE: u32 = 0x00000300;
pub const CS5: u32 = 0x00000000;
pub const CS6: u32 = 0x00000100;
pub const CS7: u32 = 0x00000200;
pub const CS8: u32 = 0x00000300;
pub const CSTOPB: u32 = 0x00000400;
pub const CREAD: u32 = 0x00000800;
pub const PARENB: u32 = 0x00001000;
pub const PARODD: u32 = 0x00002000;
pub const HUPCL: u32 = 0x00004000;
pub const CLOCAL: u32 = 0x00008000;
pub const CIBAUD: u32 = 0x001f0000;

/* c_lflag bits */
pub const ISIG: u32 = 0x00000080;
pub const ICANON: u32 = 0x00000100;
pub const XCASE: u32 = 0x00004000;
pub const ECHO: u32 = 0x00000008;
pub const ECHOE: u32 = 0x00000002;
pub const ECHOK: u32 = 0x00000004;
pub const ECHONL: u32 = 0x00000010;
pub const NOFLSH: u32 = 0x80000000;
pub const TOSTOP: u32 = 0x00400000;
pub const ECHOCTL: u32 = 0x00000040;
pub const ECHOPRT: u32 = 0x00000020;
pub const ECHOKE: u32 = 0x00000001;
pub const FLUSHO: u32 = 0x00800000;
pub const PENDIN: u32 = 0x20000000;
pub const IEXTEN: u32 = 0x00000400;
pub const EXTPROC: u32 = 0x10000000;

/* Values for the OPTIONAL_ACTIONS argument to `tcsetattr'.  */
pub const TCSANOW: u32 = 0;
pub const TCSADRAIN: u32 = 1;
pub const TCSAFLUSH: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
