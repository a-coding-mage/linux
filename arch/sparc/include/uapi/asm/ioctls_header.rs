/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the SPARC UAPI ioctl header. The _IO* helpers and referenced
// types are supplied by the corresponding ioctl/type definitions.

/* Big T */
pub const TCGETA: u32 = _IOR(b'T', 1, core::mem::size_of::<termio>());
pub const TCSETA: u32 = _IOW(b'T', 2, core::mem::size_of::<termio>());
pub const TCSETAW: u32 = _IOW(b'T', 3, core::mem::size_of::<termio>());
pub const TCSETAF: u32 = _IOW(b'T', 4, core::mem::size_of::<termio>());
pub const TCSBRK: u32 = _IO(b'T', 5);
pub const TCXONC: u32 = _IO(b'T', 6);
pub const TCFLSH: u32 = _IO(b'T', 7);
pub const TCGETS: u32 = _IOR(b'T', 8, core::mem::size_of::<termios>());
pub const TCSETS: u32 = _IOW(b'T', 9, core::mem::size_of::<termios>());
pub const TCSETSW: u32 = _IOW(b'T', 10, core::mem::size_of::<termios>());
pub const TCSETSF: u32 = _IOW(b'T', 11, core::mem::size_of::<termios>());
pub const TCGETS2: u32 = _IOR(b'T', 12, core::mem::size_of::<termios2>());
pub const TCSETS2: u32 = _IOW(b'T', 13, core::mem::size_of::<termios2>());
pub const TCSETSW2: u32 = _IOW(b'T', 14, core::mem::size_of::<termios2>());
pub const TCSETSF2: u32 = _IOW(b'T', 15, core::mem::size_of::<termios2>());
pub const TIOCGDEV: u32 = _IOR(b'T', 0x32, core::mem::size_of::<u32>());
pub const TIOCVHANGUP: u32 = _IO(b'T', 0x37);
pub const TIOCGPKT: u32 = _IOR(b'T', 0x38, core::mem::size_of::<i32>());
pub const TIOCGPTLCK: u32 = _IOR(b'T', 0x39, core::mem::size_of::<i32>());
pub const TIOCGEXCL: u32 = _IOR(b'T', 0x40, core::mem::size_of::<i32>());
pub const TIOCGRS485: u32 = _IOR(b'T', 0x41, core::mem::size_of::<serial_rs485>());
pub const TIOCSRS485: u32 = _IOWR(b'T', 0x42, core::mem::size_of::<serial_rs485>());
pub const TIOCGISO7816: u32 = _IOR(b'T', 0x43, core::mem::size_of::<serial_iso7816>());
pub const TIOCSISO7816: u32 = _IOWR(b'T', 0x44, core::mem::size_of::<serial_iso7816>());

/* Note: ioctl names unavailable in Linux have a double underscore prefix. */
/* Little t */
pub const TIOCGETD: u32 = _IOR(b't', 0, core::mem::size_of::<i32>());
pub const TIOCSETD: u32 = _IOW(b't', 1, core::mem::size_of::<i32>());
pub const __TIOCHPCL: u32 = _IO(b't', 2);
pub const __TIOCMODG: u32 = _IOR(b't', 3, core::mem::size_of::<i32>());
pub const __TIOCMODS: u32 = _IOW(b't', 4, core::mem::size_of::<i32>());
pub const __TIOCGETP: u32 = _IOR(b't', 8, core::mem::size_of::<sgttyb>());
pub const __TIOCSETP: u32 = _IOW(b't', 9, core::mem::size_of::<sgttyb>());
pub const __TIOCSETN: u32 = _IOW(b't', 10, core::mem::size_of::<sgttyb>());
pub const TIOCEXCL: u32 = _IO(b't', 13);
pub const TIOCNXCL: u32 = _IO(b't', 14);
pub const __TIOCFLUSH: u32 = _IOW(b't', 16, core::mem::size_of::<i32>());
pub const __TIOCSETC: u32 = _IOW(b't', 17, core::mem::size_of::<tchars>());
pub const __TIOCGETC: u32 = _IOR(b't', 18, core::mem::size_of::<tchars>());
pub const __TIOCTCNTL: u32 = _IOW(b't', 32, core::mem::size_of::<i32>());
pub const __TIOCSIGNAL: u32 = _IOW(b't', 33, core::mem::size_of::<i32>());
pub const __TIOCSETX: u32 = _IOW(b't', 34, core::mem::size_of::<i32>());
pub const __TIOCGETX: u32 = _IOR(b't', 35, core::mem::size_of::<i32>());
pub const TIOCCONS: u32 = _IO(b't', 36);
pub const TIOCGSOFTCAR: u32 = _IOR(b't', 100, core::mem::size_of::<i32>());
pub const TIOCSSOFTCAR: u32 = _IOW(b't', 101, core::mem::size_of::<i32>());
pub const __TIOCUCNTL: u32 = _IOW(b't', 102, core::mem::size_of::<i32>());
pub const TIOCSWINSZ: u32 = _IOW(b't', 103, core::mem::size_of::<winsize>());
pub const TIOCGWINSZ: u32 = _IOR(b't', 104, core::mem::size_of::<winsize>());
pub const __TIOCREMOTE: u32 = _IOW(b't', 105, core::mem::size_of::<i32>());
pub const TIOCMGET: u32 = _IOR(b't', 106, core::mem::size_of::<i32>());
pub const TIOCMBIC: u32 = _IOW(b't', 107, core::mem::size_of::<i32>());
pub const TIOCMBIS: u32 = _IOW(b't', 108, core::mem::size_of::<i32>());
pub const TIOCMSET: u32 = _IOW(b't', 109, core::mem::size_of::<i32>());
pub const TIOCSTART: u32 = _IO(b't', 110);
pub const TIOCSTOP: u32 = _IO(b't', 111);
pub const TIOCPKT: u32 = _IOW(b't', 112, core::mem::size_of::<i32>());
pub const TIOCNOTTY: u32 = _IO(b't', 113);
pub const TIOCSTI: u32 = _IOW(b't', 114, core::mem::size_of::<i8>());
pub const TIOCOUTQ: u32 = _IOR(b't', 115, core::mem::size_of::<i32>());
pub const __TIOCGLTC: u32 = _IOR(b't', 116, core::mem::size_of::<ltchars>());
pub const __TIOCSLTC: u32 = _IOW(b't', 117, core::mem::size_of::<ltchars>());
/* 118 and 119 are non-posix setpgrp/getpgrp tty ioctls. */
pub const __TIOCCDTR: u32 = _IO(b't', 120);
pub const __TIOCSDTR: u32 = _IO(b't', 121);
pub const TIOCCBRK: u32 = _IO(b't', 122);
pub const TIOCSBRK: u32 = _IO(b't', 123);
pub const __TIOCLGET: u32 = _IOW(b't', 124, core::mem::size_of::<i32>());
pub const __TIOCLSET: u32 = _IOW(b't', 125, core::mem::size_of::<i32>());
pub const __TIOCLBIC: u32 = _IOW(b't', 126, core::mem::size_of::<i32>());
pub const __TIOCLBIS: u32 = _IOW(b't', 127, core::mem::size_of::<i32>());
pub const __TIOCISPACE: u32 = _IOR(b't', 128, core::mem::size_of::<i32>());
pub const __TIOCISIZE: u32 = _IOR(b't', 129, core::mem::size_of::<i32>());
pub const TIOCSPGRP: u32 = _IOW(b't', 130, core::mem::size_of::<i32>());
pub const TIOCGPGRP: u32 = _IOR(b't', 131, core::mem::size_of::<i32>());
pub const TIOCSCTTY: u32 = _IO(b't', 132);
pub const TIOCGSID: u32 = _IOR(b't', 133, core::mem::size_of::<i32>());
pub const TIOCGPTN: u32 = _IOR(b't', 134, core::mem::size_of::<u32>());
pub const TIOCSPTLCK: u32 = _IOW(b't', 135, core::mem::size_of::<i32>());
pub const TIOCSIG: u32 = _IOW(b't', 136, core::mem::size_of::<i32>());
pub const TIOCGPTPEER: u32 = _IO(b't', 137);

/* Little f */
pub const FIOCLEX: u32 = _IO(b'f', 1);
pub const FIONCLEX: u32 = _IO(b'f', 2);
pub const FIOASYNC: u32 = _IOW(b'f', 125, core::mem::size_of::<i32>());
pub const FIONBIO: u32 = _IOW(b'f', 126, core::mem::size_of::<i32>());
pub const FIONREAD: u32 = _IOR(b'f', 127, core::mem::size_of::<i32>());
pub const TIOCINQ: u32 = FIONREAD;
pub const FIOQSIZE: u32 = _IOR(b'f', 128, core::mem::size_of::<loff_t>());

/* SCARY Rutgers local SunOS kernel hackery, perhaps I will support it someday. */
pub const __TCGETSTAT: u32 = _IO(b'T', 200);
pub const __TCSETSTAT: u32 = _IO(b'T', 201);

/* Linux specific, no SunOS equivalent. */
pub const TIOCLINUX: u32 = 0x541C;
pub const TIOCGSERIAL: u32 = 0x541E;
pub const TIOCSSERIAL: u32 = 0x541F;
pub const TCSBRKP: u32 = 0x5425;
pub const TIOCSERCONFIG: u32 = 0x5453;
pub const TIOCSERGWILD: u32 = 0x5454;
pub const TIOCSERSWILD: u32 = 0x5455;
pub const TIOCGLCKTRMIOS: u32 = 0x5456;
pub const TIOCSLCKTRMIOS: u32 = 0x5457;
pub const TIOCSERGSTRUCT: u32 = 0x5458;
pub const TIOCSERGETLSR: u32 = 0x5459;
pub const TIOCSERGETMULTI: u32 = 0x545A;
pub const TIOCSERSETMULTI: u32 = 0x545B;
pub const TIOCMIWAIT: u32 = 0x545C;
pub const TIOCGICOUNT: u32 = 0x545D;

/* Kernel definitions: packet mode. */
pub const TIOCPKT_DATA: u32 = 0;
pub const TIOCPKT_FLUSHREAD: u32 = 1;
pub const TIOCPKT_FLUSHWRITE: u32 = 2;
pub const TIOCPKT_STOP: u32 = 4;
pub const TIOCPKT_START: u32 = 8;
pub const TIOCPKT_NOSTOP: u32 = 16;
pub const TIOCPKT_DOSTOP: u32 = 32;
pub const TIOCPKT_IOCTL: u32 = 64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
