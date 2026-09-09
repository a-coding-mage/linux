/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Dependency: ioctl encoding macros and referenced C-compatible types are
// supplied by the corresponding translated headers.

pub const FIOCLEX: u32 = _IO!('f', 1);
pub const FIONCLEX: u32 = _IO!('f', 2);
pub const FIOASYNC: u32 = _IOW!('f', 125, ::core::ffi::c_int);
pub const FIONBIO: u32 = _IOW!('f', 126, ::core::ffi::c_int);
pub const FIONREAD: u32 = _IOR!('f', 127, ::core::ffi::c_int);
pub const TIOCINQ: u32 = FIONREAD;
pub const FIOQSIZE: u32 = _IOR!('f', 128, loff_t);

pub const TIOCGETP: u32 = _IOR!('t', 8, sgttyb);
pub const TIOCSETP: u32 = _IOW!('t', 9, sgttyb);
pub const TIOCSETN: u32 = _IOW!('t', 10, sgttyb); // TIOCSETP wo flush

pub const TIOCSETC: u32 = _IOW!('t', 17, tchars);
pub const TIOCGETC: u32 = _IOR!('t', 18, tchars);
pub const TCGETS: u32 = _IOR!('t', 19, termios);
pub const TCSETS: u32 = _IOW!('t', 20, termios);
pub const TCSETSW: u32 = _IOW!('t', 21, termios);
pub const TCSETSF: u32 = _IOW!('t', 22, termios);

pub const TCGETA: u32 = 0x40127417;
pub const TCSETA: u32 = 0x80127418;
pub const TCSETAW: u32 = 0x80127419;
pub const TCSETAF: u32 = 0x8012741c;

pub const TCSBRK: u32 = _IO!('t', 29);
pub const TCXONC: u32 = _IO!('t', 30);
pub const TCFLSH: u32 = _IO!('t', 31);

pub const TCGETS2: u32 = _IOR!('T', 42, termios2);
pub const TCSETS2: u32 = _IOW!('T', 43, termios2);
pub const TCSETSW2: u32 = _IOW!('T', 44, termios2);
pub const TCSETSF2: u32 = _IOW!('T', 45, termios2);

pub const TIOCSWINSZ: u32 = _IOW!('t', 103, winsize);
pub const TIOCGWINSZ: u32 = _IOR!('t', 104, winsize);
pub const TIOCSTART: u32 = _IO!('t', 110); // start output, like ^Q
pub const TIOCSTOP: u32 = _IO!('t', 111); // stop output, like ^S
pub const TIOCOUTQ: u32 = _IOR!('t', 115, ::core::ffi::c_int); // output queue size

pub const TIOCGLTC: u32 = _IOR!('t', 116, ltchars);
pub const TIOCSLTC: u32 = _IOW!('t', 117, ltchars);
pub const TIOCSPGRP: u32 = _IOW!('t', 118, ::core::ffi::c_int);
pub const TIOCGPGRP: u32 = _IOR!('t', 119, ::core::ffi::c_int);

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
pub const TIOCGRS485: u32 = _IOR!('T', 0x2E, serial_rs485);
pub const TIOCSRS485: u32 = _IOWR!('T', 0x2F, serial_rs485);
pub const TIOCGPTN: u32 = _IOR!('T', 0x30, u32); // Get Pty Number (of pty-mux device)
pub const TIOCSPTLCK: u32 = _IOW!('T', 0x31, ::core::ffi::c_int); // Lock/unlock Pty
pub const TIOCGDEV: u32 = _IOR!('T', 0x32, u32); // Get primary device node of /dev/console
pub const TIOCSIG: u32 = _IOW!('T', 0x36, ::core::ffi::c_int); // Generate signal on Pty slave
pub const TIOCVHANGUP: u32 = 0x5437;
pub const TIOCGPKT: u32 = _IOR!('T', 0x38, ::core::ffi::c_int); // Get packet mode state
pub const TIOCGPTLCK: u32 = _IOR!('T', 0x39, ::core::ffi::c_int); // Get Pty lock state
pub const TIOCGEXCL: u32 = _IOR!('T', 0x40, ::core::ffi::c_int); // Get exclusive mode state
pub const TIOCGPTPEER: u32 = _IO!('T', 0x41); // Safely open the slave
pub const TIOCGISO7816: u32 = _IOR!('T', 0x42, serial_iso7816);
pub const TIOCSISO7816: u32 = _IOWR!('T', 0x43, serial_iso7816);

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
