/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: ioctl encoding macros and the referenced C-compatible types are
// supplied by other translated headers.

/* 0x54 is just a magic number to make these relatively unique ('T') */

pub const TCGETS: _ = _IOR('T', 16, core::mem::size_of::<termios>()); // TCGETATTR
pub const TCSETS: _ = _IOW('T', 17, core::mem::size_of::<termios>()); // TCSETATTR
pub const TCSETSW: _ = _IOW('T', 18, core::mem::size_of::<termios>()); // TCSETATTRD
pub const TCSETSF: _ = _IOW('T', 19, core::mem::size_of::<termios>()); // TCSETATTRF
pub const TCGETA: u32 = 0x40125401;
pub const TCSETA: u32 = 0x80125402;
pub const TCSETAW: u32 = 0x80125403;
pub const TCSETAF: u32 = 0x80125404;
pub const TCSBRK: _ = _IO('T', 5);
pub const TCXONC: _ = _IO('T', 6);
pub const TCFLSH: _ = _IO('T', 7);
pub const TIOCEXCL: u32 = 0x540C;
pub const TIOCNXCL: u32 = 0x540D;
pub const TIOCSCTTY: u32 = 0x540E;
pub const TIOCGPGRP: _ = _IOR('T', 30, core::mem::size_of::<i32>());
pub const TIOCSPGRP: _ = _IOW('T', 29, core::mem::size_of::<i32>());
pub const TIOCOUTQ: u32 = 0x5411;
pub const TIOCSTI: u32 = 0x5412;
pub const TIOCGWINSZ: u32 = 0x5413;
pub const TIOCSWINSZ: u32 = 0x5414;
pub const TIOCMGET: u32 = 0x5415;
pub const TIOCMBIS: u32 = 0x5416;
pub const TIOCMBIC: u32 = 0x5417;
pub const TIOCMSET: u32 = 0x5418;
pub const TIOCGSOFTCAR: u32 = 0x5419;
pub const TIOCSSOFTCAR: u32 = 0x541A;
pub const FIONREAD: u32 = 0x541B;
pub const TIOCINQ: u32 = FIONREAD;
pub const TIOCLINUX: u32 = 0x541C;
pub const TIOCCONS: u32 = 0x541D;
pub const TIOCGSERIAL: u32 = 0x541E;
pub const TIOCSSERIAL: u32 = 0x541F;
pub const TIOCPKT: u32 = 0x5420;
pub const FIONBIO: u32 = 0x5421;
pub const TIOCNOTTY: u32 = 0x5422;
pub const TIOCSETD: u32 = 0x5423;
pub const TIOCGETD: u32 = 0x5424;
pub const TCSBRKP: u32 = 0x5425; // Needed for POSIX tcsendbreak()
pub const TIOCSBRK: u32 = 0x5427; // BSD compatibility
pub const TIOCCBRK: u32 = 0x5428; // BSD compatibility
pub const TIOCGSID: _ = _IOR('T', 20, core::mem::size_of::<i32>()); // Return the session ID of FD
pub const TCGETS2: _ = _IOR('T', 0x2A, core::mem::size_of::<termios2>());
pub const TCSETS2: _ = _IOW('T', 0x2B, core::mem::size_of::<termios2>());
pub const TCSETSW2: _ = _IOW('T', 0x2C, core::mem::size_of::<termios2>());
pub const TCSETSF2: _ = _IOW('T', 0x2D, core::mem::size_of::<termios2>());
pub const TIOCGRS485: _ = _IOR('T', 0x2E, core::mem::size_of::<serial_rs485>());
pub const TIOCSRS485: _ = _IOWR('T', 0x2F, core::mem::size_of::<serial_rs485>());
pub const TIOCGPTN: _ = _IOR('T', 0x30, core::mem::size_of::<u32>()); // Get Pty Number (of pty-mux device)
pub const TIOCSPTLCK: _ = _IOW('T', 0x31, core::mem::size_of::<i32>()); // Lock/unlock Pty
pub const TIOCGDEV: _ = _IOR('T', 0x32, core::mem::size_of::<i32>()); // Get primary device node of /dev/console
pub const TIOCSIG: _ = _IOW('T', 0x36, core::mem::size_of::<i32>()); // Generate signal on Pty slave
pub const TIOCVHANGUP: u32 = 0x5437;
pub const TIOCGPKT: _ = _IOR('T', 0x38, core::mem::size_of::<i32>()); // Get packet mode state
pub const TIOCGPTLCK: _ = _IOR('T', 0x39, core::mem::size_of::<i32>()); // Get Pty lock state
pub const TIOCGEXCL: _ = _IOR('T', 0x40, core::mem::size_of::<i32>()); // Get exclusive mode state
pub const TIOCGPTPEER: _ = _IO('T', 0x41); // Safely open the slave
pub const TIOCGISO7816: _ = _IOR('T', 0x42, core::mem::size_of::<serial_iso7816>());
pub const TIOCSISO7816: _ = _IOWR('T', 0x43, core::mem::size_of::<serial_iso7816>());

pub const FIONCLEX: u32 = 0x5450; // these numbers need to be adjusted.
pub const FIOCLEX: u32 = 0x5451;
pub const FIOASYNC: u32 = 0x5452;
pub const TIOCSERCONFIG: u32 = 0x5453;
pub const TIOCSERGWILD: u32 = 0x5454;
pub const TIOCSERSWILD: u32 = 0x5455;
pub const TIOCGLCKTRMIOS: u32 = 0x5456;
pub const TIOCSLCKTRMIOS: u32 = 0x5457;
pub const TIOCSERGSTRUCT: u32 = 0x5458; // For debugging only
pub const TIOCSERGETLSR: u32 = 0x5459; // Get line status register
pub const TIOCSERGETMULTI: u32 = 0x545A; // Get multiport config
pub const TIOCSERSETMULTI: u32 = 0x545B; // Set multiport config

pub const TIOCMIWAIT: u32 = 0x545C; // wait for a change on serial input line(s)
pub const TIOCGICOUNT: u32 = 0x545D; // read serial port inline interrupt counts
pub const FIOQSIZE: u32 = 0x5460; // Get exact space used by quota

pub const TIOCSTART: u32 = 0x5461;
pub const TIOCSTOP: u32 = 0x5462;
pub const TIOCSLTC: u32 = 0x5462;

/* Used for packet mode */
pub const TIOCPKT_DATA: u32 = 0;
pub const TIOCPKT_FLUSHREAD: u32 = 1;
pub const TIOCPKT_FLUSHWRITE: u32 = 2;
pub const TIOCPKT_STOP: u32 = 4;
pub const TIOCPKT_START: u32 = 8;
pub const TIOCPKT_NOSTOP: u32 = 16;
pub const TIOCPKT_DOSTOP: u32 = 32;
pub const TIOCPKT_IOCTL: u32 = 64;

pub const TIOCSER_TEMT: u32 = 0x01; // Transmitter physically empty

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
