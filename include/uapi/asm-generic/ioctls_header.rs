/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: ioctl encoding macros and the referenced C ABI types are supplied externally.

/*
 * These are the most common definitions for tty ioctl numbers.
 * Most of them do not use the recommended _IOC(), but there is
 * probably some source code out there hardcoding the number,
 * so we might as well use them for all new platforms.
 *
 * The architectures that use different values here typically
 * try to be compatible with some Unix variants for the same
 * architecture.
 */

/* 0x54 is just a magic number to make these relatively unique ('T') */

pub const TCGETS: u32 = 0x5401;
pub const TCSETS: u32 = 0x5402;
pub const TCSETSW: u32 = 0x5403;
pub const TCSETSF: u32 = 0x5404;
pub const TCGETA: u32 = 0x5405;
pub const TCSETA: u32 = 0x5406;
pub const TCSETAW: u32 = 0x5407;
pub const TCSETAF: u32 = 0x5408;
pub const TCSBRK: u32 = 0x5409;
pub const TCXONC: u32 = 0x540A;
pub const TCFLSH: u32 = 0x540B;
pub const TIOCEXCL: u32 = 0x540C;
pub const TIOCNXCL: u32 = 0x540D;
pub const TIOCSCTTY: u32 = 0x540E;
pub const TIOCGPGRP: u32 = 0x540F;
pub const TIOCSPGRP: u32 = 0x5410;
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
pub const TCSBRKP: u32 = 0x5425; /* Needed for POSIX tcsendbreak() */
pub const TIOCSBRK: u32 = 0x5427; /* BSD compatibility */
pub const TIOCCBRK: u32 = 0x5428; /* BSD compatibility */
pub const TIOCGSID: u32 = 0x5429; /* Return the session ID of FD */
pub const TCGETS2: u32 = _IOR!('T', 0x2A, termios2);
pub const TCSETS2: u32 = _IOW!('T', 0x2B, termios2);
pub const TCSETSW2: u32 = _IOW!('T', 0x2C, termios2);
pub const TCSETSF2: u32 = _IOW!('T', 0x2D, termios2);
pub const TIOCGRS485: u32 = 0x542E;
// Conditional definition: preserve the C header's existing-definition guard.
pub const TIOCSRS485: u32 = 0x542F;
pub const TIOCGPTN: u32 = _IOR!('T', 0x30, u32); /* Get Pty Number (of pty-mux device) */
pub const TIOCSPTLCK: u32 = _IOW!('T', 0x31, i32); /* Lock/unlock Pty */
pub const TIOCGDEV: u32 = _IOR!('T', 0x32, u32); /* Get primary device node of /dev/console */
pub const TCGETX: u32 = 0x5432; /* SYS5 TCGETX compatibility */
pub const TCSETX: u32 = 0x5433;
pub const TCSETXF: u32 = 0x5434;
pub const TCSETXW: u32 = 0x5435;
pub const TIOCSIG: u32 = _IOW!('T', 0x36, i32); /* pty: generate signal */
pub const TIOCVHANGUP: u32 = 0x5437;
pub const TIOCGPKT: u32 = _IOR!('T', 0x38, i32); /* Get packet mode state */
pub const TIOCGPTLCK: u32 = _IOR!('T', 0x39, i32); /* Get Pty lock state */
pub const TIOCGEXCL: u32 = _IOR!('T', 0x40, i32); /* Get exclusive mode state */
pub const TIOCGPTPEER: u32 = _IO!('T', 0x41); /* Safely open the slave */
pub const TIOCGISO7816: u32 = _IOR!('T', 0x42, serial_iso7816);
pub const TIOCSISO7816: u32 = _IOWR!('T', 0x43, serial_iso7816);

pub const FIONCLEX: u32 = 0x5450;
pub const FIOCLEX: u32 = 0x5451;
pub const FIOASYNC: u32 = 0x5452;
pub const TIOCSERCONFIG: u32 = 0x5453;
pub const TIOCSERGWILD: u32 = 0x5454;
pub const TIOCSERSWILD: u32 = 0x5455;
pub const TIOCGLCKTRMIOS: u32 = 0x5456;
pub const TIOCSLCKTRMIOS: u32 = 0x5457;
pub const TIOCSERGSTRUCT: u32 = 0x5458; /* For debugging only */
pub const TIOCSERGETLSR: u32 = 0x5459; /* Get line status register */
pub const TIOCSERGETMULTI: u32 = 0x545A; /* Get multiport config  */
pub const TIOCSERSETMULTI: u32 = 0x545B; /* Set multiport config */

pub const TIOCMIWAIT: u32 = 0x545C; /* wait for a change on serial input line(s) */
pub const TIOCGICOUNT: u32 = 0x545D; /* read serial port inline interrupt counts */

/* Some arches already define FIOQSIZE due to a historical conflict with a Hayes modem-specific ioctl value. */
pub const FIOQSIZE: u32 = 0x5460;

/* Used for packet mode */
pub const TIOCPKT_DATA: u32 = 0;
pub const TIOCPKT_FLUSHREAD: u32 = 1;
pub const TIOCPKT_FLUSHWRITE: u32 = 2;
pub const TIOCPKT_STOP: u32 = 4;
pub const TIOCPKT_START: u32 = 8;
pub const TIOCPKT_NOSTOP: u32 = 16;
pub const TIOCPKT_DOSTOP: u32 = 32;
pub const TIOCPKT_IOCTL: u32 = 64;

pub const TIOCSER_TEMT: u32 = 0x01; /* Transmitter physically empty */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
