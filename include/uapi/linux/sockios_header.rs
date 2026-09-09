/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Definitions of the socket-level I/O control calls.
 *
 * Version:	@(#)sockios.h	1.0.2	03/09/93
 *
 * Authors:	Ross Biro
 *		Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 */

// <asm/bitsperlong.h> and <asm/sockios.h> are external dependencies.

/* Linux-specific socket ioctls */
pub const SIOCINQ: _ = FIONREAD;
pub const SIOCOUTQ: _ = TIOCOUTQ; /* output queue size (not sent + not acked) */

pub const SOCK_IOC_TYPE: u32 = 0x89;

/*
 * the timeval/timespec data structure layout is defined by libc,
 * so we need to cover both possible versions on 32-bit.
 */
/* Get stamp (timeval) */
pub const SIOCGSTAMP_NEW: _ = _IOR!(SOCK_IOC_TYPE, 0x06, [i64; 2]);
/* Get stamp (timespec) */
pub const SIOCGSTAMPNS_NEW: _ = _IOR!(SOCK_IOC_TYPE, 0x07, [i64; 2]);

/* The source condition depends on target ABI configuration. */
#[cfg(any(target_pointer_width = "64", target_arch = "x86_64"))]
pub const SIOCGSTAMP: _ = SIOCGSTAMP_OLD;
#[cfg(any(target_pointer_width = "64", target_arch = "x86_64"))]
pub const SIOCGSTAMPNS: _ = SIOCGSTAMPNS_OLD;
#[cfg(not(any(target_pointer_width = "64", target_arch = "x86_64")))]
pub const SIOCGSTAMP: _ = if core::mem::size_of::<libc::timeval>() == 8 {
    SIOCGSTAMP_OLD
} else {
    SIOCGSTAMP_NEW
};
#[cfg(not(any(target_pointer_width = "64", target_arch = "x86_64")))]
pub const SIOCGSTAMPNS: _ = if core::mem::size_of::<libc::timespec>() == 8 {
    SIOCGSTAMPNS_OLD
} else {
    SIOCGSTAMPNS_NEW
};

/* Routing table calls. */
pub const SIOCADDRT: u32 = 0x890B;
pub const SIOCDELRT: u32 = 0x890C;
pub const SIOCRTMSG: u32 = 0x890D;

/* Socket configuration controls. */
pub const SIOCGIFNAME: u32 = 0x8910;
pub const SIOCSIFLINK: u32 = 0x8911;
pub const SIOCGIFCONF: u32 = 0x8912;
pub const SIOCGIFFLAGS: u32 = 0x8913;
pub const SIOCSIFFLAGS: u32 = 0x8914;
pub const SIOCGIFADDR: u32 = 0x8915;
pub const SIOCSIFADDR: u32 = 0x8916;
pub const SIOCGIFDSTADDR: u32 = 0x8917;
pub const SIOCSIFDSTADDR: u32 = 0x8918;
pub const SIOCGIFBRDADDR: u32 = 0x8919;
pub const SIOCSIFBRDADDR: u32 = 0x891a;
pub const SIOCGIFNETMASK: u32 = 0x891b;
pub const SIOCSIFNETMASK: u32 = 0x891c;
pub const SIOCGIFMETRIC: u32 = 0x891d;
pub const SIOCSIFMETRIC: u32 = 0x891e;
pub const SIOCGIFMEM: u32 = 0x891f;
pub const SIOCSIFMEM: u32 = 0x8920;
pub const SIOCGIFMTU: u32 = 0x8921;
pub const SIOCSIFMTU: u32 = 0x8922;
pub const SIOCSIFNAME: u32 = 0x8923;
pub const SIOCSIFHWADDR: u32 = 0x8924;
pub const SIOCGIFENCAP: u32 = 0x8925;
pub const SIOCSIFENCAP: u32 = 0x8926;
pub const SIOCGIFHWADDR: u32 = 0x8927;
pub const SIOCGIFSLAVE: u32 = 0x8929;
pub const SIOCSIFSLAVE: u32 = 0x8930;
pub const SIOCADDMULTI: u32 = 0x8931;
pub const SIOCDELMULTI: u32 = 0x8932;
pub const SIOCGIFINDEX: u32 = 0x8933;
pub const SIOGIFINDEX: u32 = SIOCGIFINDEX;
pub const SIOCSIFPFLAGS: u32 = 0x8934;
pub const SIOCGIFPFLAGS: u32 = 0x8935;
pub const SIOCDIFADDR: u32 = 0x8936;
pub const SIOCSIFHWBROADCAST: u32 = 0x8937;
pub const SIOCGIFCOUNT: u32 = 0x8938;
pub const SIOCGIFBR: u32 = 0x8940;
pub const SIOCSIFBR: u32 = 0x8941;
pub const SIOCGIFTXQLEN: u32 = 0x8942;
pub const SIOCSIFTXQLEN: u32 = 0x8943;
pub const SIOCETHTOOL: u32 = 0x8946;
pub const SIOCGMIIPHY: u32 = 0x8947;
pub const SIOCGMIIREG: u32 = 0x8948;
pub const SIOCSMIIREG: u32 = 0x8949;
pub const SIOCWANDEV: u32 = 0x894A;
pub const SIOCOUTQNSD: u32 = 0x894B;
pub const SIOCGSKNS: u32 = 0x894C;

/* ARP cache control calls. */
/* 0x8950 - 0x8952: obsolete calls, don't re-use */
pub const SIOCDARP: u32 = 0x8953;
pub const SIOCGARP: u32 = 0x8954;
pub const SIOCSARP: u32 = 0x8955;

/* RARP cache control calls. */
pub const SIOCDRARP: u32 = 0x8960;
pub const SIOCGRARP: u32 = 0x8961;
pub const SIOCSRARP: u32 = 0x8962;

/* Driver configuration calls */
pub const SIOCGIFMAP: u32 = 0x8970;
pub const SIOCSIFMAP: u32 = 0x8971;

/* DLCI configuration calls */
pub const SIOCADDDLCI: u32 = 0x8980;
pub const SIOCDELDLCI: u32 = 0x8981;
pub const SIOCGIFVLAN: u32 = 0x8982;
pub const SIOCSIFVLAN: u32 = 0x8983;

/* bonding calls */
pub const SIOCBONDENSLAVE: u32 = 0x8990;
pub const SIOCBONDRELEASE: u32 = 0x8991;
pub const SIOCBONDSETHWADDR: u32 = 0x8992;
pub const SIOCBONDSLAVEINFOQUERY: u32 = 0x8993;
pub const SIOCBONDINFOQUERY: u32 = 0x8994;
pub const SIOCBONDCHANGEACTIVE: u32 = 0x8995;

/* bridge calls */
pub const SIOCBRADDBR: u32 = 0x89a0;
pub const SIOCBRDELBR: u32 = 0x89a1;
pub const SIOCBRADDIF: u32 = 0x89a2;
pub const SIOCBRDELIF: u32 = 0x89a3;

/* hardware time stamping: parameters in linux/net_tstamp.h */
pub const SIOCSHWTSTAMP: u32 = 0x89b0;
pub const SIOCGHWTSTAMP: u32 = 0x89b1;

/* Device private ioctl calls */
/* These 16 ioctls are available to devices via the do_ioctl() device vector. */
pub const SIOCDEVPRIVATE: u32 = 0x89F0; /* to 89FF */

/* These 16 ioctl calls are protocol private */
pub const SIOCPROTOPRIVATE: u32 = 0x89E0; /* to 89EF */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
