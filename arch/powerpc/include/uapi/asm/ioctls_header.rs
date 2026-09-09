/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the PowerPC UAPI ioctl definitions.
// `_IO`, `_IOW`, `_IOR`, and `_IOWR`, as well as referenced C types, are
// supplied by the corresponding ioctl and UAPI dependencies.

pub const FIOCLEX: u32 = _IO(b'f' as u32, 1);
pub const FIONCLEX: u32 = _IO(b'f' as u32, 2);
pub const FIOASYNC: u32 = _IOW(b'f' as u32, 125, core::mem::size_of::<core::ffi::c_int>());
pub const FIONBIO: u32 = _IOW(b'f' as u32, 126, core::mem::size_of::<core::ffi::c_int>());
pub const FIONREAD: u32 = _IOR(b'f' as u32, 127, core::mem::size_of::<core::ffi::c_int>());
pub const TIOCINQ: u32 = FIONREAD;
pub const FIOQSIZE: u32 = _IOR(b'f' as u32, 128, core::mem::size_of::<loff_t>());

pub const TIOCGETP: u32 = _IOR(b't' as u32, 8, core::mem::size_of::<sgttyb>());
pub const TIOCSETP: u32 = _IOW(b't' as u32, 9, core::mem::size_of::<sgttyb>());
pub const TIOCSETN: u32 = _IOW(b't' as u32, 10, core::mem::size_of::<sgttyb>()); // TIOCSETP wo flush

pub const TIOCSETC: u32 = _IOW(b't' as u32, 17, core::mem::size_of::<tchars>());
pub const TIOCGETC: u32 = _IOR(b't' as u32, 18, core::mem::size_of::<tchars>());
pub const TCGETS: u32 = _IOR(b't' as u32, 19, core::mem::size_of::<termios>());
pub const TCSETS: u32 = _IOW(b't' as u32, 20, core::mem::size_of::<termios>());
pub const TCSETSW: u32 = _IOW(b't' as u32, 21, core::mem::size_of::<termios>());
pub const TCSETSF: u32 = _IOW(b't' as u32, 22, core::mem::size_of::<termios>());

pub const TCGETA: u32 = 0x40147417; // _IOR('t', 23, struct termio)
pub const TCSETA: u32 = 0x80147418; // _IOW('t', 24, struct termio)
pub const TCSETAW: u32 = 0x80147419; // _IOW('t', 25, struct termio)
pub const TCSETAF: u32 = 0x8014741c; // _IOW('t', 28, struct termio)

pub const TCSBRK: u32 = _IO(b't' as u32, 29);
pub const TCXONC: u32 = _IO(b't' as u32, 30);
pub const TCFLSH: u32 = _IO(b't' as u32, 31);

pub const TIOCSWINSZ: u32 = _IOW(b't' as u32, 103, core::mem::size_of::<winsize>());
pub const TIOCGWINSZ: u32 = _IOR(b't' as u32, 104, core::mem::size_of::<winsize>());
pub const TIOCSTART: u32 = _IO(b't' as u32, 110); // start output, like ^Q
pub const TIOCSTOP: u32 = _IO(b't' as u32, 111); // stop output, like ^S
pub const TIOCOUTQ: u32 = _IOR(b't' as u32, 115, core::mem::size_of::<core::ffi::c_int>()); // output queue size

pub const TIOCGLTC: u32 = _IOR(b't' as u32, 116, core::mem::size_of::<ltchars>());
pub const TIOCSLTC: u32 = _IOW(b't' as u32, 117, core::mem::size_of::<ltchars>());
pub const TIOCSPGRP: u32 = _IOW(b't' as u32, 118, core::mem::size_of::<core::ffi::c_int>());
pub const TIOCGPGRP: u32 = _IOR(b't' as u32, 119, core::mem::size_of::<core::ffi::c_int>());

pub const TIOCEXCL: u32 = 0x540C;
pub const TIOCNXCL: u32 = 0x540D;
pub const TIOCSCTTY: u32 = 0x540E;
pub const TIOCSTI: u32 = 0x5412;
pub const TIOCMGET: u32 = 0x5415;
pub const TIOCMBIS: u32 = 0x5416;
pub const TIOCMBIC: u32 = 0x5417;
pub const TIOCMSET: u32 = 0x5418;
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
pub const TIOCM_OUT1: u32 = 0x2000;
pub const TIOCM_OUT2: u32 = 0x4000;
pub const TIOCM_LOOP: u32 = 0x8000;

pub const TIOCGSOFTCAR: u32 = 0x5419;
pub const TIOCSSOFTCAR: u32 = 0x541A;
pub const TIOCLINUX: u32 = 0x541C;
pub const TIOCCONS: u32 = 0x541D;
pub const TIOCGSERIAL: u32 = 0x541E;
pub const TIOCSSERIAL: u32 = 0x541F;
pub const TIOCPKT: u32 = 0x5420;
pub const TIOCPKT_DATA: u32 = 0;
pub const TIOCPKT_FLUSHREAD: u32 = 1;
pub const TIOCPKT_FLUSHWRITE: u32 = 2;
pub const TIOCPKT_STOP: u32 = 4;
pub const TIOCPKT_START: u32 = 8;
pub const TIOCPKT_NOSTOP: u32 = 16;
pub const TIOCPKT_DOSTOP: u32 = 32;
pub const TIOCPKT_IOCTL: u32 = 64;

pub const TIOCNOTTY: u32 = 0x5422;
pub const TIOCSETD: u32 = 0x5423;
pub const TIOCGETD: u32 = 0x5424;
pub const TCSBRKP: u32 = 0x5425; // Needed for POSIX tcsendbreak()
pub const TIOCSBRK: u32 = 0x5427; // BSD compatibility
pub const TIOCCBRK: u32 = 0x5428; // BSD compatibility
pub const TIOCGSID: u32 = 0x5429; // Return the session ID of FD
pub const TIOCGRS485: u32 = 0x542e;
pub const TIOCSRS485: u32 = 0x542f;
pub const TIOCGPTN: u32 = _IOR(b'T' as u32, 0x30, core::mem::size_of::<u32>()); // Get Pty Number (of pty-mux device)
pub const TIOCSPTLCK: u32 = _IOW(b'T' as u32, 0x31, core::mem::size_of::<core::ffi::c_int>()); // Lock/unlock Pty
pub const TIOCGDEV: u32 = _IOR(b'T' as u32, 0x32, core::mem::size_of::<u32>()); // Get primary device node of /dev/console
pub const TIOCSIG: u32 = _IOW(b'T' as u32, 0x36, core::mem::size_of::<core::ffi::c_int>()); // Generate signal on Pty slave
pub const TIOCVHANGUP: u32 = 0x5437;
pub const TIOCGPKT: u32 = _IOR(b'T' as u32, 0x38, core::mem::size_of::<core::ffi::c_int>()); // Get packet mode state
pub const TIOCGPTLCK: u32 = _IOR(b'T' as u32, 0x39, core::mem::size_of::<core::ffi::c_int>()); // Get Pty lock state
pub const TIOCGEXCL: u32 = _IOR(b'T' as u32, 0x40, core::mem::size_of::<core::ffi::c_int>()); // Get exclusive mode state
pub const TIOCGPTPEER: u32 = _IO(b'T' as u32, 0x41); // Safely open the slave
pub const TIOCGISO7816: u32 = _IOR(b'T' as u32, 0x42, core::mem::size_of::<serial_iso7816>());
pub const TIOCSISO7816: u32 = _IOWR(b'T' as u32, 0x43, core::mem::size_of::<serial_iso7816>());

pub const TIOCSERCONFIG: u32 = 0x5453;
pub const TIOCSERGWILD: u32 = 0x5454;
pub const TIOCSERSWILD: u32 = 0x5455;
pub const TIOCGLCKTRMIOS: u32 = 0x5456;
pub const TIOCSLCKTRMIOS: u32 = 0x5457;
pub const TIOCSERGSTRUCT: u32 = 0x5458; // For debugging only
pub const TIOCSERGETLSR: u32 = 0x5459; // Get line status register
// ioctl (fd, TIOCSERGETLSR, &result) where result may be as below
pub const TIOCSER_TEMT: u32 = 0x01; // Transmitter physically empty
pub const TIOCSERGETMULTI: u32 = 0x545A; // Get multiport config
pub const TIOCSERSETMULTI: u32 = 0x545B; // Set multiport config
pub const TIOCMIWAIT: u32 = 0x545C; // wait for a change on serial input line(s)
pub const TIOCGICOUNT: u32 = 0x545D; // read serial port inline interrupt counts

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
