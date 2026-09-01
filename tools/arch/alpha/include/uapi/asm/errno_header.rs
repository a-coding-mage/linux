/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Depends on constants from <asm-generic/errno-base.h>. */

/* EAGAIN is 11 in errno-base.h in C, then undefined here. */

pub const EDEADLK: i32 = 11; /* Resource deadlock would occur */

pub const EAGAIN: i32 = 35; /* Try again */
pub const EWOULDBLOCK: i32 = EAGAIN; /* Operation would block */
pub const EINPROGRESS: i32 = 36; /* Operation now in progress */
pub const EALREADY: i32 = 37; /* Operation already in progress */
pub const ENOTSOCK: i32 = 38; /* Socket operation on non-socket */
pub const EDESTADDRREQ: i32 = 39; /* Destination address required */
pub const EMSGSIZE: i32 = 40; /* Message too long */
pub const EPROTOTYPE: i32 = 41; /* Protocol wrong type for socket */
pub const ENOPROTOOPT: i32 = 42; /* Protocol not available */
pub const EPROTONOSUPPORT: i32 = 43; /* Protocol not supported */
pub const ESOCKTNOSUPPORT: i32 = 44; /* Socket type not supported */
pub const EOPNOTSUPP: i32 = 45; /* Operation not supported on transport endpoint */
pub const EPFNOSUPPORT: i32 = 46; /* Protocol family not supported */
pub const EAFNOSUPPORT: i32 = 47; /* Address family not supported by protocol */
pub const EADDRINUSE: i32 = 48; /* Address already in use */
pub const EADDRNOTAVAIL: i32 = 49; /* Cannot assign requested address */
pub const ENETDOWN: i32 = 50; /* Network is down */
pub const ENETUNREACH: i32 = 51; /* Network is unreachable */
pub const ENETRESET: i32 = 52; /* Network dropped connection because of reset */
pub const ECONNABORTED: i32 = 53; /* Software caused connection abort */
pub const ECONNRESET: i32 = 54; /* Connection reset by peer */
pub const ENOBUFS: i32 = 55; /* No buffer space available */
pub const EISCONN: i32 = 56; /* Transport endpoint is already connected */
pub const ENOTCONN: i32 = 57; /* Transport endpoint is not connected */
pub const ESHUTDOWN: i32 = 58; /* Cannot send after transport endpoint shutdown */
pub const ETOOMANYREFS: i32 = 59; /* Too many references: cannot splice */
pub const ETIMEDOUT: i32 = 60; /* Connection timed out */
pub const ECONNREFUSED: i32 = 61; /* Connection refused */
pub const ELOOP: i32 = 62; /* Too many symbolic links encountered */
pub const ENAMETOOLONG: i32 = 63; /* File name too long */
pub const EHOSTDOWN: i32 = 64; /* Host is down */
pub const EHOSTUNREACH: i32 = 65; /* No route to host */
pub const ENOTEMPTY: i32 = 66; /* Directory not empty */

pub const EUSERS: i32 = 68; /* Too many users */
pub const EDQUOT: i32 = 69; /* Quota exceeded */
pub const ESTALE: i32 = 70; /* Stale file handle */
pub const EREMOTE: i32 = 71; /* Object is remote */

pub const ENOLCK: i32 = 77; /* No record locks available */
pub const ENOSYS: i32 = 78; /* Function not implemented */

pub const ENOMSG: i32 = 80; /* No message of desired type */
pub const EIDRM: i32 = 81; /* Identifier removed */
pub const ENOSR: i32 = 82; /* Out of streams resources */
pub const ETIME: i32 = 83; /* Timer expired */
pub const EBADMSG: i32 = 84; /* Not a data message */
pub const EFSBADCRC: i32 = EBADMSG; /* Bad CRC detected */
pub const EPROTO: i32 = 85; /* Protocol error */
pub const ENODATA: i32 = 86; /* No data available */
pub const ENOSTR: i32 = 87; /* Device not a stream */

pub const ENOPKG: i32 = 92; /* Package not installed */

pub const EILSEQ: i32 = 116; /* Illegal byte sequence */

/* The following are just random noise.. */
pub const ECHRNG: i32 = 88; /* Channel number out of range */
pub const EL2NSYNC: i32 = 89; /* Level 2 not synchronized */
pub const EL3HLT: i32 = 90; /* Level 3 halted */
pub const EL3RST: i32 = 91; /* Level 3 reset */

pub const ELNRNG: i32 = 93; /* Link number out of range */
pub const EUNATCH: i32 = 94; /* Protocol driver not attached */
pub const ENOCSI: i32 = 95; /* No CSI structure available */
pub const EL2HLT: i32 = 96; /* Level 2 halted */
pub const EBADE: i32 = 97; /* Invalid exchange */
pub const EBADR: i32 = 98; /* Invalid request descriptor */
pub const EXFULL: i32 = 99; /* Exchange full */
pub const ENOANO: i32 = 100; /* No anode */
pub const EBADRQC: i32 = 101; /* Invalid request code */
pub const EBADSLT: i32 = 102; /* Invalid slot */

pub const EDEADLOCK: i32 = EDEADLK;

pub const EBFONT: i32 = 104; /* Bad font file format */
pub const ENONET: i32 = 105; /* Machine is not on the network */
pub const ENOLINK: i32 = 106; /* Link has been severed */
pub const EADV: i32 = 107; /* Advertise error */
pub const ESRMNT: i32 = 108; /* Srmount error */
pub const ECOMM: i32 = 109; /* Communication error on send */
pub const EMULTIHOP: i32 = 110; /* Multihop attempted */
pub const EDOTDOT: i32 = 111; /* RFS specific error */
pub const EOVERFLOW: i32 = 112; /* Value too large for defined data type */
pub const ENOTUNIQ: i32 = 113; /* Name not unique on network */
pub const EBADFD: i32 = 114; /* File descriptor in bad state */
pub const EREMCHG: i32 = 115; /* Remote address changed */

pub const EUCLEAN: i32 = 117; /* Structure needs cleaning */
pub const EFSCORRUPTED: i32 = EUCLEAN; /* Filesystem is corrupted */
pub const ENOTNAM: i32 = 118; /* Not a XENIX named type file */
pub const ENAVAIL: i32 = 119; /* No XENIX semaphores available */
pub const EISNAM: i32 = 120; /* Is a named type file */
pub const EREMOTEIO: i32 = 121; /* Remote I/O error */

pub const ELIBACC: i32 = 122; /* Can not access a needed shared library */
pub const ELIBBAD: i32 = 123; /* Accessing a corrupted shared library */
pub const ELIBSCN: i32 = 124; /* .lib section in a.out corrupted */
pub const ELIBMAX: i32 = 125; /* Attempting to link in too many shared libraries */
pub const ELIBEXEC: i32 = 126; /* Cannot exec a shared library directly */
pub const ERESTART: i32 = 127; /* Interrupted system call should be restarted */
pub const ESTRPIPE: i32 = 128; /* Streams pipe error */

pub const ENOMEDIUM: i32 = 129; /* No medium found */
pub const EMEDIUMTYPE: i32 = 130; /* Wrong medium type */
pub const ECANCELED: i32 = 131; /* Operation Cancelled */
pub const ENOKEY: i32 = 132; /* Required key not available */
pub const EKEYEXPIRED: i32 = 133; /* Key has expired */
pub const EKEYREVOKED: i32 = 134; /* Key has been revoked */
pub const EKEYREJECTED: i32 = 135; /* Key was rejected by service */

/* for robust mutexes */
pub const EOWNERDEAD: i32 = 136; /* Owner died */
pub const ENOTRECOVERABLE: i32 = 137; /* State not recoverable */

pub const ERFKILL: i32 = 138; /* Operation not possible due to RF-kill */

pub const EHWPOISON: i32 = 139; /* Memory page has hardware error */

pub const EFTYPE: i32 = 140; /* Wrong file type for the intended operation */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
