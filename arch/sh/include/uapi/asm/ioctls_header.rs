/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the SH ioctl header. The ioctl encoding helpers and
// referenced ABI types are supplied by other headers.

pub const FIOCLEX: _ = _IO('f' as u8, 1);
pub const FIONCLEX: _ = _IO('f' as u8, 2);
pub const FIOASYNC: _ = _IOW('f' as u8, 125, i32);
pub const FIONBIO: _ = _IOW('f' as u8, 126, i32);
pub const FIONREAD: _ = _IOR('f' as u8, 127, i32);
pub const TIOCINQ: _ = FIONREAD;
pub const FIOQSIZE: _ = _IOR('f' as u8, 128, loff_t);

pub const TCGETS: u32 = 0x5401;
pub const TCSETS: u32 = 0x5402;
pub const TCSETSW: u32 = 0x5403;
pub const TCSETSF: u32 = 0x5404;

pub const TCGETA: u32 = 0x80127417; // _IOR('t', 23, struct termio)
pub const TCSETA: u32 = 0x40127418; // _IOW('t', 24, struct termio)
pub const TCSETAW: u32 = 0x40127419; // _IOW('t', 25, struct termio)
pub const TCSETAF: u32 = 0x4012741C; // _IOW('t', 28, struct termio)

pub const TCSBRK: _ = _IO('t' as u8, 29);
pub const TCXONC: _ = _IO('t' as u8, 30);
pub const TCFLSH: _ = _IO('t' as u8, 31);

pub const TIOCSWINSZ: u32 = 0x40087467; // _IOW('t', 103, struct winsize)
pub const TIOCGWINSZ: u32 = 0x80087468; // _IOR('t', 104, struct winsize)
pub const TIOCSTART: _ = _IO('t' as u8, 110); // start output, like ^Q
pub const TIOCSTOP: _ = _IO('t' as u8, 111); // stop output, like ^S
pub const TIOCOUTQ: _ = _IOR('t' as u8, 115, i32); // output queue size

pub const TIOCSPGRP: _ = _IOW('t' as u8, 118, i32);
pub const TIOCGPGRP: _ = _IOR('t' as u8, 119, i32);

pub const TIOCEXCL: _ = _IO('T' as u8, 12); // 0x540C
pub const TIOCNXCL: _ = _IO('T' as u8, 13); // 0x540D
pub const TIOCSCTTY: _ = _IO('T' as u8, 14); // 0x540E

pub const TIOCSTI: _ = _IOW('T' as u8, 18, i8); // 0x5412
pub const TIOCMGET: _ = _IOR('T' as u8, 21, u32); // 0x5415
pub const TIOCMBIS: _ = _IOW('T' as u8, 22, u32); // 0x5416
pub const TIOCMBIC: _ = _IOW('T' as u8, 23, u32); // 0x5417
pub const TIOCMSET: _ = _IOW('T' as u8, 24, u32); // 0x5418
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

pub const TIOCGSOFTCAR: _ = _IOR('T' as u8, 25, u32); // 0x5419
pub const TIOCSSOFTCAR: _ = _IOW('T' as u8, 26, u32); // 0x541A
pub const TIOCLINUX: _ = _IOW('T' as u8, 28, i8); // 0x541C
pub const TIOCCONS: _ = _IO('T' as u8, 29); // 0x541D
pub const TIOCGSERIAL: u32 = 0x803C541E; // _IOR('T', 30, struct serial_struct) 0x541E
pub const TIOCSSERIAL: u32 = 0x403C541F; // _IOW('T', 31, struct serial_struct) 0x541F
pub const TIOCPKT: _ = _IOW('T' as u8, 32, i32); // 0x5420
pub const TIOCPKT_DATA: i32 = 0;
pub const TIOCPKT_FLUSHREAD: i32 = 1;
pub const TIOCPKT_FLUSHWRITE: i32 = 2;
pub const TIOCPKT_STOP: i32 = 4;
pub const TIOCPKT_START: i32 = 8;
pub const TIOCPKT_NOSTOP: i32 = 16;
pub const TIOCPKT_DOSTOP: i32 = 32;
pub const TIOCPKT_IOCTL: i32 = 64;

pub const TIOCNOTTY: _ = _IO('T' as u8, 34); // 0x5422
pub const TIOCSETD: _ = _IOW('T' as u8, 35, i32); // 0x5423
pub const TIOCGETD: _ = _IOR('T' as u8, 36, i32); // 0x5424
pub const TCSBRKP: _ = _IOW('T' as u8, 37, i32); // 0x5425; Needed for POSIX tcsendbreak()
pub const TIOCSBRK: _ = _IO('T' as u8, 39); // 0x5427; BSD compatibility
pub const TIOCCBRK: _ = _IO('T' as u8, 40); // 0x5428; BSD compatibility
pub const TIOCGSID: _ = _IOR('T' as u8, 41, pid_t); // 0x5429; Return the session ID of FD
pub const TCGETS2: _ = _IOR('T' as u8, 42, termios2);
pub const TCSETS2: _ = _IOW('T' as u8, 43, termios2);
pub const TCSETSW2: _ = _IOW('T' as u8, 44, termios2);
pub const TCSETSF2: _ = _IOW('T' as u8, 45, termios2);
pub const TIOCGRS485: _ = _IOR('T' as u8, 46, serial_rs485);
pub const TIOCSRS485: _ = _IOWR('T' as u8, 47, serial_rs485);
pub const TIOCGPTN: _ = _IOR('T' as u8, 0x30, u32); // Get Pty Number (of pty-mux device)
pub const TIOCSPTLCK: _ = _IOW('T' as u8, 0x31, i32); // Lock/unlock Pty
pub const TIOCGDEV: _ = _IOR('T' as u8, 0x32, u32); // Get primary device node of /dev/console
pub const TIOCSIG: _ = _IOW('T' as u8, 0x36, i32); // Generate signal on Pty slave
pub const TIOCVHANGUP: _ = _IO('T' as u8, 0x37);
pub const TIOCGPKT: _ = _IOR('T' as u8, 0x38, i32); // Get packet mode state
pub const TIOCGPTLCK: _ = _IOR('T' as u8, 0x39, i32); // Get Pty lock state
pub const TIOCGEXCL: _ = _IOR('T' as u8, 0x40, i32); // Get exclusive mode state
pub const TIOCGPTPEER: _ = _IO('T' as u8, 0x41); // Safely open the slave
pub const TIOCGISO7816: _ = _IOR('T' as u8, 0x42, serial_iso7816);
pub const TIOCSISO7816: _ = _IOWR('T' as u8, 0x43, serial_iso7816);

pub const TIOCSERCONFIG: _ = _IO('T' as u8, 83); // 0x5453
pub const TIOCSERGWILD: _ = _IOR('T' as u8, 84, i32); // 0x5454
pub const TIOCSERSWILD: _ = _IOW('T' as u8, 85, i32); // 0x5455
pub const TIOCGLCKTRMIOS: u32 = 0x5456;
pub const TIOCSLCKTRMIOS: u32 = 0x5457;
pub const TIOCSERGSTRUCT: u32 = 0x80d85458; // _IOR('T', 88, struct async_struct) 0x5458; For debugging only
pub const TIOCSERGETLSR: _ = _IOR('T' as u8, 89, u32); // 0x5459; Get line status register
// ioctl (fd, TIOCSERGETLSR, &result) where result may be as below
pub const TIOCSER_TEMT: u32 = 0x01; // Transmitter physically empty
pub const TIOCSERGETMULTI: u32 = 0x80A8545A; // _IOR('T', 90, struct serial_multiport_struct) 0x545A; Get multiport config
pub const TIOCSERSETMULTI: u32 = 0x40A8545B; // _IOW('T', 91, struct serial_multiport_struct) 0x545B; Set multiport config

pub const TIOCMIWAIT: _ = _IO('T' as u8, 92); // 0x545C; wait for a change on serial input line(s)
pub const TIOCGICOUNT: u32 = 0x545D; // read serial port inline interrupt counts

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
