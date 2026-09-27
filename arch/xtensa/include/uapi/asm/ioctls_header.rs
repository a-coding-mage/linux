/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from include/asm-xtensa/ioctls.h. */

// The ioctl encoding helpers and referenced C types are supplied by dependencies.

pub const FIOCLEX: u32 = _IO(b'f', 1);
pub const FIONCLEX: u32 = _IO(b'f', 2);
pub const FIOASYNC: u32 = _IOW(b'f', 125, kernel::ffi::c_int);
pub const FIONBIO: u32 = _IOW(b'f', 126, kernel::ffi::c_int);
pub const FIONREAD: u32 = _IOR(b'f', 127, kernel::ffi::c_int);
pub const TIOCINQ: u32 = FIONREAD;
pub const FIOQSIZE: u32 = _IOR(b'f', 128, loff_t);

pub const TCGETS: u32 = 0x5401;
pub const TCSETS: u32 = 0x5402;
pub const TCSETSW: u32 = 0x5403;
pub const TCSETSF: u32 = 0x5404;

pub const TCGETA: u32 = 0x80127417; /* _IOR('t', 23, struct termio) */
pub const TCSETA: u32 = 0x40127418; /* _IOW('t', 24, struct termio) */
pub const TCSETAW: u32 = 0x40127419; /* _IOW('t', 25, struct termio) */
pub const TCSETAF: u32 = 0x4012741C; /* _IOW('t', 28, struct termio) */

pub const TCSBRK: u32 = _IO(b't', 29);
pub const TCXONC: u32 = _IO(b't', 30);
pub const TCFLSH: u32 = _IO(b't', 31);

pub const TIOCSWINSZ: u32 = 0x40087467; /* _IOW('t', 103, struct winsize) */
pub const TIOCGWINSZ: u32 = 0x80087468; /* _IOR('t', 104, struct winsize) */
pub const TIOCSTART: u32 = _IO(b't', 110); /* start output, like ^Q */
pub const TIOCSTOP: u32 = _IO(b't', 111); /* stop output, like ^S */
pub const TIOCOUTQ: u32 = _IOR(b't', 115, kernel::ffi::c_int); /* output queue size */

pub const TIOCSPGRP: u32 = _IOW(b't', 118, kernel::ffi::c_int);
pub const TIOCGPGRP: u32 = _IOR(b't', 119, kernel::ffi::c_int);

pub const TIOCEXCL: u32 = _IO(b'T', 12);
pub const TIOCNXCL: u32 = _IO(b'T', 13);
pub const TIOCSCTTY: u32 = _IO(b'T', 14);
pub const TIOCSTI: u32 = _IOW(b'T', 18, kernel::ffi::c_char);
pub const TIOCMGET: u32 = _IOR(b'T', 21, u32);
pub const TIOCMBIS: u32 = _IOW(b'T', 22, u32);
pub const TIOCMBIC: u32 = _IOW(b'T', 23, u32);
pub const TIOCMSET: u32 = _IOW(b'T', 24, u32);
pub const TIOCM_LE: u32 = 0x001;
pub const TIOCM_DTR: u32 = 0x002;
pub const TIOCM_RTS: u32 = 0x004;
pub const TIOCM_ST: u32 = 0x008;
pub const TIOCM_SR: u32 = 0x010;
pub const TIOCM_CTS: u32 = 0x020;
pub const TIOCM_CAR: u32 = 0x040;
pub const TIOCM_RNG: u32 = 0x080;
pub const TIOCM_DSR: u32 = 0x100;
pub const TIOCM_CD: u32 = TIOCM_CAR;
pub const TIOCM_RI: u32 = TIOCM_RNG;

pub const TIOCGSOFTCAR: u32 = _IOR(b'T', 25, u32);
pub const TIOCSSOFTCAR: u32 = _IOW(b'T', 26, u32);
pub const TIOCLINUX: u32 = _IOW(b'T', 28, kernel::ffi::c_char);
pub const TIOCCONS: u32 = _IO(b'T', 29);
pub const TIOCGSERIAL: u32 = 0x803C541E;
pub const TIOCSSERIAL: u32 = 0x403C541F;
pub const TIOCPKT: u32 = _IOW(b'T', 32, kernel::ffi::c_int);
pub const TIOCPKT_DATA: u32 = 0;
pub const TIOCPKT_FLUSHREAD: u32 = 1;
pub const TIOCPKT_FLUSHWRITE: u32 = 2;
pub const TIOCPKT_STOP: u32 = 4;
pub const TIOCPKT_START: u32 = 8;
pub const TIOCPKT_NOSTOP: u32 = 16;
pub const TIOCPKT_DOSTOP: u32 = 32;
pub const TIOCPKT_IOCTL: u32 = 64;

pub const TIOCNOTTY: u32 = _IO(b'T', 34);
pub const TIOCSETD: u32 = _IOW(b'T', 35, kernel::ffi::c_int);
pub const TIOCGETD: u32 = _IOR(b'T', 36, kernel::ffi::c_int);
pub const TCSBRKP: u32 = _IOW(b'T', 37, kernel::ffi::c_int); /* Needed for POSIX tcsendbreak() */
pub const TIOCSBRK: u32 = _IO(b'T', 39); /* BSD compatibility */
pub const TIOCCBRK: u32 = _IO(b'T', 40); /* BSD compatibility */
pub const TIOCGSID: u32 = _IOR(b'T', 41, pid_t); /* Return the session ID of FD */
pub const TCGETS2: u32 = _IOR(b'T', 42, termios2);
pub const TCSETS2: u32 = _IOW(b'T', 43, termios2);
pub const TCSETSW2: u32 = _IOW(b'T', 44, termios2);
pub const TCSETSF2: u32 = _IOW(b'T', 45, termios2);
pub const TIOCGRS485: u32 = _IOR(b'T', 46, serial_rs485);
pub const TIOCSRS485: u32 = _IOWR(b'T', 47, serial_rs485);
pub const TIOCGPTN: u32 = _IOR(b'T', 0x30, u32);
pub const TIOCSPTLCK: u32 = _IOW(b'T', 0x31, kernel::ffi::c_int);
pub const TIOCGDEV: u32 = _IOR(b'T', 0x32, u32);
pub const TIOCSIG: u32 = _IOW(b'T', 0x36, kernel::ffi::c_int);
pub const TIOCVHANGUP: u32 = _IO(b'T', 0x37);
pub const TIOCGPKT: u32 = _IOR(b'T', 0x38, kernel::ffi::c_int);
pub const TIOCGPTLCK: u32 = _IOR(b'T', 0x39, kernel::ffi::c_int);
pub const TIOCGEXCL: u32 = _IOR(b'T', 0x40, kernel::ffi::c_int);
pub const TIOCGPTPEER: u32 = _IO(b'T', 0x41);
pub const TIOCGISO7816: u32 = _IOR(b'T', 0x42, serial_iso7816);
pub const TIOCSISO7816: u32 = _IOWR(b'T', 0x43, serial_iso7816);

pub const TIOCSERCONFIG: u32 = _IO(b'T', 83);
pub const TIOCSERGWILD: u32 = _IOR(b'T', 84, kernel::ffi::c_int);
pub const TIOCSERSWILD: u32 = _IOW(b'T', 85, kernel::ffi::c_int);
pub const TIOCGLCKTRMIOS: u32 = 0x5456;
pub const TIOCSLCKTRMIOS: u32 = 0x5457;
pub const TIOCSERGSTRUCT: u32 = 0x5458; /* For debugging only */
pub const TIOCSERGETLSR: u32 = _IOR(b'T', 89, u32); /* Get line status reg. */
pub const TIOCSER_TEMT: u32 = 0x01; /* Transmitter physically empty */
pub const TIOCSERGETMULTI: u32 = 0x80a8545a; /* Get multiport config */
pub const TIOCSERSETMULTI: u32 = 0x40a8545b; /* Set multiport config */
pub const TIOCMIWAIT: u32 = _IO(b'T', 92); /* wait for a change on serial input line(s) */
pub const TIOCGICOUNT: u32 = 0x545D; /* read serial port inline interrupt counts */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
