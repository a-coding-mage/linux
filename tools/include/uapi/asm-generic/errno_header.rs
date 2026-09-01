/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Depends on <asm-generic/errno-base.h> for base errno constants such as EAGAIN. */

pub const EDEADLK: i32 = 35; /* Resource deadlock would occur */
pub const ENAMETOOLONG: i32 = 36; /* File name too long */
pub const ENOLCK: i32 = 37; /* No record locks available */

/*
 * This error code is special: arch syscall entry code will return
 * -ENOSYS if users try to call a syscall that doesn't exist.  To keep
 * failures of syscalls that really do exist distinguishable from
 * failures due to attempts to use a nonexistent syscall, syscall
 * implementations should refrain from returning -ENOSYS.
 */
pub const ENOSYS: i32 = 38; /* Invalid system call number */

pub const ENOTEMPTY: i32 = 39; /* Directory not empty */
pub const ELOOP: i32 = 40; /* Too many symbolic links encountered */
pub const EWOULDBLOCK: i32 = EAGAIN; /* Operation would block */
pub const ENOMSG: i32 = 42; /* No message of desired type */
pub const EIDRM: i32 = 43; /* Identifier removed */
pub const ECHRNG: i32 = 44; /* Channel number out of range */
pub const EL2NSYNC: i32 = 45; /* Level 2 not synchronized */
pub const EL3HLT: i32 = 46; /* Level 3 halted */
pub const EL3RST: i32 = 47; /* Level 3 reset */
pub const ELNRNG: i32 = 48; /* Link number out of range */
pub const EUNATCH: i32 = 49; /* Protocol driver not attached */
pub const ENOCSI: i32 = 50; /* No CSI structure available */
pub const EL2HLT: i32 = 51; /* Level 2 halted */
pub const EBADE: i32 = 52; /* Invalid exchange */
pub const EBADR: i32 = 53; /* Invalid request descriptor */
pub const EXFULL: i32 = 54; /* Exchange full */
pub const ENOANO: i32 = 55; /* No anode */
pub const EBADRQC: i32 = 56; /* Invalid request code */
pub const EBADSLT: i32 = 57; /* Invalid slot */

pub const EDEADLOCK: i32 = EDEADLK;

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
pub const EMULTIHOP: i32 = 72; /* Multihop attempted */
pub const EDOTDOT: i32 = 73; /* RFS specific error */
pub const EBADMSG: i32 = 74; /* Not a data message */
pub const EFSBADCRC: i32 = EBADMSG; /* Bad CRC detected */
pub const EOVERFLOW: i32 = 75; /* Value too large for defined data type */
pub const ENOTUNIQ: i32 = 76; /* Name not unique on network */
pub const EBADFD: i32 = 77; /* File descriptor in bad state */
pub const EREMCHG: i32 = 78; /* Remote address changed */
pub const ELIBACC: i32 = 79; /* Can not access a needed shared library */
pub const ELIBBAD: i32 = 80; /* Accessing a corrupted shared library */
pub const ELIBSCN: i32 = 81; /* .lib section in a.out corrupted */
pub const ELIBMAX: i32 = 82; /* Attempting to link in too many shared libraries */
pub const ELIBEXEC: i32 = 83; /* Cannot exec a shared library directly */
pub const EILSEQ: i32 = 84; /* Illegal byte sequence */
pub const ERESTART: i32 = 85; /* Interrupted system call should be restarted */
pub const ESTRPIPE: i32 = 86; /* Streams pipe error */
pub const EUSERS: i32 = 87; /* Too many users */
pub const ENOTSOCK: i32 = 88; /* Socket operation on non-socket */
pub const EDESTADDRREQ: i32 = 89; /* Destination address required */
pub const EMSGSIZE: i32 = 90; /* Message too long */
pub const EPROTOTYPE: i32 = 91; /* Protocol wrong type for socket */
pub const ENOPROTOOPT: i32 = 92; /* Protocol not available */
pub const EPROTONOSUPPORT: i32 = 93; /* Protocol not supported */
pub const ESOCKTNOSUPPORT: i32 = 94; /* Socket type not supported */
pub const EOPNOTSUPP: i32 = 95; /* Operation not supported on transport endpoint */
pub const EPFNOSUPPORT: i32 = 96; /* Protocol family not supported */
pub const EAFNOSUPPORT: i32 = 97; /* Address family not supported by protocol */
pub const EADDRINUSE: i32 = 98; /* Address already in use */
pub const EADDRNOTAVAIL: i32 = 99; /* Cannot assign requested address */
pub const ENETDOWN: i32 = 100; /* Network is down */
pub const ENETUNREACH: i32 = 101; /* Network is unreachable */
pub const ENETRESET: i32 = 102; /* Network dropped connection because of reset */
pub const ECONNABORTED: i32 = 103; /* Software caused connection abort */
pub const ECONNRESET: i32 = 104; /* Connection reset by peer */
pub const ENOBUFS: i32 = 105; /* No buffer space available */
pub const EISCONN: i32 = 106; /* Transport endpoint is already connected */
pub const ENOTCONN: i32 = 107; /* Transport endpoint is not connected */
pub const ESHUTDOWN: i32 = 108; /* Cannot send after transport endpoint shutdown */
pub const ETOOMANYREFS: i32 = 109; /* Too many references: cannot splice */
pub const ETIMEDOUT: i32 = 110; /* Connection timed out */
pub const ECONNREFUSED: i32 = 111; /* Connection refused */
pub const EHOSTDOWN: i32 = 112; /* Host is down */
pub const EHOSTUNREACH: i32 = 113; /* No route to host */
pub const EALREADY: i32 = 114; /* Operation already in progress */
pub const EINPROGRESS: i32 = 115; /* Operation now in progress */
pub const ESTALE: i32 = 116; /* Stale file handle */
pub const EUCLEAN: i32 = 117; /* Structure needs cleaning */
pub const EFSCORRUPTED: i32 = EUCLEAN; /* Filesystem is corrupted */
pub const ENOTNAM: i32 = 118; /* Not a XENIX named type file */
pub const ENAVAIL: i32 = 119; /* No XENIX semaphores available */
pub const EISNAM: i32 = 120; /* Is a named type file */
pub const EREMOTEIO: i32 = 121; /* Remote I/O error */
pub const EDQUOT: i32 = 122; /* Quota exceeded */

pub const ENOMEDIUM: i32 = 123; /* No medium found */
pub const EMEDIUMTYPE: i32 = 124; /* Wrong medium type */
pub const ECANCELED: i32 = 125; /* Operation Canceled */
pub const ENOKEY: i32 = 126; /* Required key not available */
pub const EKEYEXPIRED: i32 = 127; /* Key has expired */
pub const EKEYREVOKED: i32 = 128; /* Key has been revoked */
pub const EKEYREJECTED: i32 = 129; /* Key was rejected by service */

/* for robust mutexes */
pub const EOWNERDEAD: i32 = 130; /* Owner died */
pub const ENOTRECOVERABLE: i32 = 131; /* State not recoverable */

pub const ERFKILL: i32 = 132; /* Operation not possible due to RF-kill */

pub const EHWPOISON: i32 = 133; /* Memory page has hardware error */

pub const EFTYPE: i32 = 134; /* Wrong file type for the intended operation */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
