/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1999, 2001, 2002 by Ralf Baechle
 */

/* These error numbers are intended to be MIPS ABI compatible. */
/* C dependency: <asm-generic/errno-base.h> supplies EAGAIN and other bases. */

pub const ENOMSG: i32 = 35; /* No message of desired type */
pub const EIDRM: i32 = 36; /* Identifier removed */
pub const ECHRNG: i32 = 37; /* Channel number out of range */
pub const EL2NSYNC: i32 = 38; /* Level 2 not synchronized */
pub const EL3HLT: i32 = 39; /* Level 3 halted */
pub const EL3RST: i32 = 40; /* Level 3 reset */
pub const ELNRNG: i32 = 41; /* Link number out of range */
pub const EUNATCH: i32 = 42; /* Protocol driver not attached */
pub const ENOCSI: i32 = 43; /* No CSI structure available */
pub const EL2HLT: i32 = 44; /* Level 2 halted */
pub const EDEADLK: i32 = 45; /* Resource deadlock would occur */
pub const ENOLCK: i32 = 46; /* No record locks available */
pub const EBADE: i32 = 50; /* Invalid exchange */
pub const EBADR: i32 = 51; /* Invalid request descriptor */
pub const EXFULL: i32 = 52; /* Exchange full */
pub const ENOANO: i32 = 53; /* No anode */
pub const EBADRQC: i32 = 54; /* Invalid request code */
pub const EBADSLT: i32 = 55; /* Invalid slot */
pub const EDEADLOCK: i32 = 56; /* File locking deadlock error */
pub const EBFONT: i32 = 59; /* Bad font file format */
pub const ENOSTR: i32 = 60; /* Device not a stream */
pub const ENODATA: i32 = 61; /* No data available */
pub const ETIME: i32 = 62; /* Timer expired */
pub const ENOSR: i32 = 63; /* Out of streams resources */
pub const ENONET: i32 = 64; /* Machine is not on the network */
pub const ENOPKG: i32 = 65; /* Package not installed */
pub const EREMOTE: i32 = 66; /* Object is remote */
pub const ENOLINK: i32 = 67; /* Link has been severed */
pub const EADV: i32 = 68; /* Advertise error */
pub const ESRMNT: i32 = 69; /* Srmount error */
pub const ECOMM: i32 = 70; /* Communication error on send */
pub const EPROTO: i32 = 71; /* Protocol error */
pub const EDOTDOT: i32 = 73; /* RFS specific error */
pub const EMULTIHOP: i32 = 74; /* Multihop attempted */
pub const EBADMSG: i32 = 77; /* Not a data message */
pub const EFSBADCRC: i32 = EBADMSG; /* Bad CRC detected */
pub const ENAMETOOLONG: i32 = 78; /* File name too long */
pub const EOVERFLOW: i32 = 79; /* Value too large for defined data type */
pub const ENOTUNIQ: i32 = 80; /* Name not unique on network */
pub const EBADFD: i32 = 81; /* File descriptor in bad state */
pub const EREMCHG: i32 = 82; /* Remote address changed */
pub const ELIBACC: i32 = 83; /* Can not access a needed shared library */
pub const ELIBBAD: i32 = 84; /* Accessing a corrupted shared library */
pub const ELIBSCN: i32 = 85; /* .lib section in a.out corrupted */
pub const ELIBMAX: i32 = 86; /* Attempting to link in too many shared libraries */
pub const ELIBEXEC: i32 = 87; /* Cannot exec a shared library directly */
pub const EILSEQ: i32 = 88; /* Illegal byte sequence */
pub const ENOSYS: i32 = 89; /* Function not implemented */
pub const ELOOP: i32 = 90; /* Too many symbolic links encountered */
pub const ERESTART: i32 = 91; /* Interrupted system call should be restarted */
pub const ESTRPIPE: i32 = 92; /* Streams pipe error */
pub const ENOTEMPTY: i32 = 93; /* Directory not empty */
pub const EUSERS: i32 = 94; /* Too many users */
pub const ENOTSOCK: i32 = 95; /* Socket operation on non-socket */
pub const EDESTADDRREQ: i32 = 96; /* Destination address required */
pub const EMSGSIZE: i32 = 97; /* Message too long */
pub const EPROTOTYPE: i32 = 98; /* Protocol wrong type for socket */
pub const ENOPROTOOPT: i32 = 99; /* Protocol not available */
pub const EPROTONOSUPPORT: i32 = 120; /* Protocol not supported */
pub const ESOCKTNOSUPPORT: i32 = 121; /* Socket type not supported */
pub const EOPNOTSUPP: i32 = 122; /* Operation not supported on transport endpoint */
pub const EPFNOSUPPORT: i32 = 123; /* Protocol family not supported */
pub const EAFNOSUPPORT: i32 = 124; /* Address family not supported by protocol */
pub const EADDRINUSE: i32 = 125; /* Address already in use */
pub const EADDRNOTAVAIL: i32 = 126; /* Cannot assign requested address */
pub const ENETDOWN: i32 = 127; /* Network is down */
pub const ENETUNREACH: i32 = 128; /* Network is unreachable */
pub const ENETRESET: i32 = 129; /* Network dropped connection because of reset */
pub const ECONNABORTED: i32 = 130; /* Software caused connection abort */
pub const ECONNRESET: i32 = 131; /* Connection reset by peer */
pub const ENOBUFS: i32 = 132; /* No buffer space available */
pub const EISCONN: i32 = 133; /* Transport endpoint is already connected */
pub const ENOTCONN: i32 = 134; /* Transport endpoint is not connected */
pub const EUCLEAN: i32 = 135; /* Structure needs cleaning */
pub const EFSCORRUPTED: i32 = EUCLEAN; /* Filesystem is corrupted */
pub const ENOTNAM: i32 = 137; /* Not a XENIX named type file */
pub const ENAVAIL: i32 = 138; /* No XENIX semaphores available */
pub const EISNAM: i32 = 139; /* Is a named type file */
pub const EREMOTEIO: i32 = 140; /* Remote I/O error */
pub const EINIT: i32 = 141; /* Reserved */
pub const EREMDEV: i32 = 142; /* Error 142 */
pub const ESHUTDOWN: i32 = 143; /* Cannot send after transport endpoint shutdown */
pub const ETOOMANYREFS: i32 = 144; /* Too many references: cannot splice */
pub const ETIMEDOUT: i32 = 145; /* Connection timed out */
pub const ECONNREFUSED: i32 = 146; /* Connection refused */
pub const EHOSTDOWN: i32 = 147; /* Host is down */
pub const EHOSTUNREACH: i32 = 148; /* No route to host */
pub const EWOULDBLOCK: i32 = EAGAIN; /* Operation would block */
pub const EALREADY: i32 = 149; /* Operation already in progress */
pub const EINPROGRESS: i32 = 150; /* Operation now in progress */
pub const ESTALE: i32 = 151; /* Stale file handle */
pub const ECANCELED: i32 = 158; /* AIO operation canceled */

/* These error are Linux extensions. */
pub const ENOMEDIUM: i32 = 159; /* No medium found */
pub const EMEDIUMTYPE: i32 = 160; /* Wrong medium type */
pub const ENOKEY: i32 = 161; /* Required key not available */
pub const EKEYEXPIRED: i32 = 162; /* Key has expired */
pub const EKEYREVOKED: i32 = 163; /* Key has been revoked */
pub const EKEYREJECTED: i32 = 164; /* Key was rejected by service */

/* for robust mutexes */
pub const EOWNERDEAD: i32 = 165; /* Owner died */
pub const ENOTRECOVERABLE: i32 = 166; /* State not recoverable */

pub const ERFKILL: i32 = 167; /* Operation not possible due to RF-kill */
pub const EHWPOISON: i32 = 168; /* Memory page has hardware error */
pub const EFTYPE: i32 = 169; /* Wrong file type for the intended operation */
pub const EDQUOT: i32 = 1133; /* Quota exceeded */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
