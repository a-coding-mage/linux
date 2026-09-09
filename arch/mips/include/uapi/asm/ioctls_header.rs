/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1996, 2001 Ralf Baechle
 * Copyright (C) 2001 MIPS Technologies, Inc.
 */

// Dependency: ioctl encoding macros and referenced C-compatible types are
// supplied by the surrounding ABI bindings.

pub const TCGETA: u32 = 0x5401;
pub const TCSETA: u32 = 0x5402; // Clashes with SNDCTL_TMR_START sound ioctl
pub const TCSETAW: u32 = 0x5403;
pub const TCSETAF: u32 = 0x5404;

pub const TCSBRK: u32 = 0x5405;
pub const TCXONC: u32 = 0x5406;
pub const TCFLSH: u32 = 0x5407;

pub const TCGETS: u32 = 0x540d;
pub const TCSETS: u32 = 0x540e;
pub const TCSETSW: u32 = 0x540f;
pub const TCSETSF: u32 = 0x5410;

pub const TIOCEXCL: u32 = 0x740d; // set exclusive use of tty
pub const TIOCNXCL: u32 = 0x740e; // reset exclusive use of tty
pub const TIOCOUTQ: u32 = 0x7472; // output queue size
pub const TIOCSTI: u32 = 0x5472; // simulate terminal input
pub const TIOCMGET: u32 = 0x741d; // get all modem bits
pub const TIOCMBIS: u32 = 0x741b; // bis modem bits
pub const TIOCMBIC: u32 = 0x741c; // bic modem bits
pub const TIOCMSET: u32 = 0x741a; // set all modem bits
pub const TIOCPKT: u32 = 0x5470; // pty: set/clear packet mode
pub const TIOCPKT_DATA: u32 = 0x00; // data packet
pub const TIOCPKT_FLUSHREAD: u32 = 0x01; // flush packet
pub const TIOCPKT_FLUSHWRITE: u32 = 0x02; // flush packet
pub const TIOCPKT_STOP: u32 = 0x04; // stop output
pub const TIOCPKT_START: u32 = 0x08; // start output
pub const TIOCPKT_NOSTOP: u32 = 0x10; // no more ^S, ^Q
pub const TIOCPKT_DOSTOP: u32 = 0x20; // now do ^S ^Q
pub const TIOCPKT_IOCTL: u32 = 0x40; // state change of pty driver
pub const TIOCSWINSZ: u32 = _IOW!(b't', 103, winsize); // set window size
pub const TIOCGWINSZ: u32 = _IOR!(b't', 104, winsize); // get window size
pub const TIOCNOTTY: u32 = 0x5471; // void tty association
pub const TIOCSETD: u32 = 0x7401;
pub const TIOCGETD: u32 = 0x7400;

pub const FIOCLEX: u32 = 0x6601;
pub const FIONCLEX: u32 = 0x6602;
pub const FIOASYNC: u32 = 0x667d;
pub const FIONBIO: u32 = 0x667e;
pub const FIOQSIZE: u32 = 0x667f;

pub const TIOCGLTC: u32 = 0x7474; // get special local chars
pub const TIOCSLTC: u32 = 0x7475; // set special local chars
pub const TIOCSPGRP: u32 = _IOW!(b't', 118, i32); // set pgrp of tty
pub const TIOCGPGRP: u32 = _IOR!(b't', 119, i32); // get pgrp of tty
pub const TIOCCONS: u32 = _IOW!(b't', 120, i32); // become virtual console

pub const FIONREAD: u32 = 0x467f;
pub const TIOCINQ: u32 = FIONREAD;

pub const TIOCGETP: u32 = 0x7408;
pub const TIOCSETP: u32 = 0x7409;
pub const TIOCSETN: u32 = 0x740a; // TIOCSETP wo flush

// 127-124 compat

pub const TIOCSBRK: u32 = 0x5427; // BSD compatibility
pub const TIOCCBRK: u32 = 0x5428; // BSD compatibility
pub const TIOCGSID: u32 = 0x7416; // Return the session ID of FD
pub const TCGETS2: u32 = _IOR!(b'T', 0x2A, termios2);
pub const TCSETS2: u32 = _IOW!(b'T', 0x2B, termios2);
pub const TCSETSW2: u32 = _IOW!(b'T', 0x2C, termios2);
pub const TCSETSF2: u32 = _IOW!(b'T', 0x2D, termios2);
pub const TIOCGRS485: u32 = _IOR!(b'T', 0x2E, serial_rs485);
pub const TIOCSRS485: u32 = _IOWR!(b'T', 0x2F, serial_rs485);
pub const TIOCGPTN: u32 = _IOR!(b'T', 0x30, u32); // Get Pty Number (of pty-mux device)
pub const TIOCSPTLCK: u32 = _IOW!(b'T', 0x31, i32); // Lock/unlock Pty
pub const TIOCGDEV: u32 = _IOR!(b'T', 0x32, u32); // Get primary device node of /dev/console
pub const TIOCSIG: u32 = _IOW!(b'T', 0x36, i32); // Generate signal on Pty slave
pub const TIOCVHANGUP: u32 = 0x5437;
pub const TIOCGPKT: u32 = _IOR!(b'T', 0x38, i32); // Get packet mode state
pub const TIOCGPTLCK: u32 = _IOR!(b'T', 0x39, i32); // Get Pty lock state
pub const TIOCGEXCL: u32 = _IOR!(b'T', 0x40, i32); // Get exclusive mode state
pub const TIOCGPTPEER: u32 = _IO!(b'T', 0x41); // Safely open the slave
pub const TIOCGISO7816: u32 = _IOR!(b'T', 0x42, serial_iso7816);
pub const TIOCSISO7816: u32 = _IOWR!(b'T', 0x43, serial_iso7816);

// I hope the range from 0x5480 on is free ...
pub const TIOCSCTTY: u32 = 0x5480; // become controlling tty
pub const TIOCGSOFTCAR: u32 = 0x5481;
pub const TIOCSSOFTCAR: u32 = 0x5482;
pub const TIOCLINUX: u32 = 0x5483;
pub const TIOCGSERIAL: u32 = 0x5484;
pub const TIOCSSERIAL: u32 = 0x5485;
pub const TCSBRKP: u32 = 0x5486; // Needed for POSIX tcsendbreak()
pub const TIOCSERCONFIG: u32 = 0x5488;
pub const TIOCSERGWILD: u32 = 0x5489;
pub const TIOCSERSWILD: u32 = 0x548a;
pub const TIOCGLCKTRMIOS: u32 = 0x548b;
pub const TIOCSLCKTRMIOS: u32 = 0x548c;
pub const TIOCSERGSTRUCT: u32 = 0x548d; // For debugging only
pub const TIOCSERGETLSR: u32 = 0x548e; // Get line status register
pub const TIOCSERGETMULTI: u32 = 0x548f; // Get multiport config
pub const TIOCSERSETMULTI: u32 = 0x5490; // Set multiport config
pub const TIOCMIWAIT: u32 = 0x5491; // wait for a change on serial input line(s)
pub const TIOCGICOUNT: u32 = 0x5492; // read serial port inline interrupt counts

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
