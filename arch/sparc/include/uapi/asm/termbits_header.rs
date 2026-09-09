/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by asm-generic/termbits-common.h in the source header.
// The aliases below remain external to this translation.

#[cfg(all(target_arch = "sparc64"))]
pub type tcflag_t = u32;
#[cfg(not(all(target_arch = "sparc64")))]
pub type tcflag_t = usize;

pub const NCCS: usize = 17;

#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    // The kernel build uses [NCCS + 2] to hold vmin/vtime; user builds use [NCCS].
    #[cfg(not(feature = "kernel"))]
    pub c_cc: [cc_t; NCCS],
    #[cfg(feature = "kernel")]
    pub c_cc: [cc_t; NCCS + 2],
}

#[cfg(feature = "kernel")]
pub const SIZEOF_USER_TERMIOS: usize = core::mem::size_of::<termios>() - (2 * core::mem::size_of::<cc_t>());

#[repr(C)]
pub struct termios2 {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; NCCS + 2],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

#[repr(C)]
pub struct ktermios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; NCCS + 2],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

pub const VINTR: usize = 0;
pub const VQUIT: usize = 1;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VEOF: usize = 4;
pub const VEOL: usize = 5;
pub const VEOL2: usize = 6;
pub const VSWTC: usize = 7;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VSUSP: usize = 10;
pub const VDSUSP: usize = 11; // SunOS POSIX nicety I do believe...
pub const VREPRINT: usize = 12;
pub const VDISCARD: usize = 13;
pub const VWERASE: usize = 14;
pub const VLNEXT: usize = 15;

#[cfg(not(feature = "kernel"))]
pub const VMIN: usize = VEOF;
#[cfg(not(feature = "kernel"))]
pub const VTIME: usize = VEOL;

pub const IUCLC: tcflag_t = 0x0200;
pub const IXON: tcflag_t = 0x0400;
pub const IXOFF: tcflag_t = 0x1000;
pub const IMAXBEL: tcflag_t = 0x2000;
pub const IUTF8: tcflag_t = 0x4000;

pub const OLCUC: tcflag_t = 0x00002;
pub const ONLCR: tcflag_t = 0x00004;
pub const NLDLY: tcflag_t = 0x00100;
pub const NL0: tcflag_t = 0x00000;
pub const NL1: tcflag_t = 0x00100;
pub const CRDLY: tcflag_t = 0x00600;
pub const CR0: tcflag_t = 0x00000;
pub const CR1: tcflag_t = 0x00200;
pub const CR2: tcflag_t = 0x00400;
pub const CR3: tcflag_t = 0x00600;
pub const TABDLY: tcflag_t = 0x01800;
pub const TAB0: tcflag_t = 0x00000;
pub const TAB1: tcflag_t = 0x00800;
pub const TAB2: tcflag_t = 0x01000;
pub const TAB3: tcflag_t = 0x01800;
pub const XTABS: tcflag_t = 0x01800;
pub const BSDLY: tcflag_t = 0x02000;
pub const BS0: tcflag_t = 0x00000;
pub const BS1: tcflag_t = 0x02000;
pub const VTDLY: tcflag_t = 0x04000;
pub const VT0: tcflag_t = 0x00000;
pub const VT1: tcflag_t = 0x04000;
pub const FFDLY: tcflag_t = 0x08000;
pub const FF0: tcflag_t = 0x00000;
pub const FF1: tcflag_t = 0x08000;
pub const PAGEOUT: tcflag_t = 0x10000; // SUNOS specific
pub const WRAP: tcflag_t = 0x20000; // SUNOS specific

pub const CBAUD: tcflag_t = 0x0000100f;
pub const CSIZE: tcflag_t = 0x00000030;
pub const CS5: tcflag_t = 0x00000000;
pub const CS6: tcflag_t = 0x00000010;
pub const CS7: tcflag_t = 0x00000020;
pub const CS8: tcflag_t = 0x00000030;
pub const CSTOPB: tcflag_t = 0x00000040;
pub const CREAD: tcflag_t = 0x00000080;
pub const PARENB: tcflag_t = 0x00000100;
pub const PARODD: tcflag_t = 0x00000200;
pub const HUPCL: tcflag_t = 0x00000400;
pub const CLOCAL: tcflag_t = 0x00000800;
pub const CBAUDEX: tcflag_t = 0x00001000;
pub const BOTHER: tcflag_t = 0x00001000;
pub const B57600: tcflag_t = 0x00001001;
pub const B115200: tcflag_t = 0x00001002;
pub const B230400: tcflag_t = 0x00001003;
pub const B460800: tcflag_t = 0x00001004;
pub const B76800: tcflag_t = 0x00001005;
pub const B153600: tcflag_t = 0x00001006;
pub const B307200: tcflag_t = 0x00001007;
pub const B614400: tcflag_t = 0x00001008;
pub const B921600: tcflag_t = 0x00001009;
pub const B500000: tcflag_t = 0x0000100a;
pub const B576000: tcflag_t = 0x0000100b;
pub const B1000000: tcflag_t = 0x0000100c;
pub const B1152000: tcflag_t = 0x0000100d;
pub const B1500000: tcflag_t = 0x0000100e;
pub const B2000000: tcflag_t = 0x0000100f;
// B2500000 through B4000000 are intentionally absent, matching the commented C definitions.
pub const CIBAUD: tcflag_t = 0x100f0000; // input baud rate (not used)

pub const ISIG: tcflag_t = 0x00000001;
pub const ICANON: tcflag_t = 0x00000002;
pub const XCASE: tcflag_t = 0x00000004;
pub const ECHO: tcflag_t = 0x00000008;
pub const ECHOE: tcflag_t = 0x00000010;
pub const ECHOK: tcflag_t = 0x00000020;
pub const ECHONL: tcflag_t = 0x00000040;
pub const NOFLSH: tcflag_t = 0x00000080;
pub const TOSTOP: tcflag_t = 0x00000100;
pub const ECHOCTL: tcflag_t = 0x00000200;
pub const ECHOPRT: tcflag_t = 0x00000400;
pub const ECHOKE: tcflag_t = 0x00000800;
pub const DEFECHO: tcflag_t = 0x00001000; // SUNOS thing, what is it?
pub const FLUSHO: tcflag_t = 0x00002000;
pub const PENDIN: tcflag_t = 0x00004000;
pub const IEXTEN: tcflag_t = 0x00008000;
pub const EXTPROC: tcflag_t = 0x00010000;

pub const TIOCM_LE: tcflag_t = 0x001;
pub const TIOCM_DTR: tcflag_t = 0x002;
pub const TIOCM_RTS: tcflag_t = 0x004;
pub const TIOCM_ST: tcflag_t = 0x008;
pub const TIOCM_SR: tcflag_t = 0x010;
pub const TIOCM_CTS: tcflag_t = 0x020;
pub const TIOCM_CAR: tcflag_t = 0x040;
pub const TIOCM_RNG: tcflag_t = 0x080;
pub const TIOCM_DSR: tcflag_t = 0x100;
pub const TIOCM_CD: tcflag_t = TIOCM_CAR;
pub const TIOCM_RI: tcflag_t = TIOCM_RNG;
pub const TIOCM_OUT1: tcflag_t = 0x2000;
pub const TIOCM_OUT2: tcflag_t = 0x4000;
pub const TIOCM_LOOP: tcflag_t = 0x8000;

pub const TIOCSER_TEMT: tcflag_t = 0x01; // Transmitter physically empty

pub const TCSANOW: tcflag_t = 0;
pub const TCSADRAIN: tcflag_t = 1;
pub const TCSAFLUSH: tcflag_t = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
