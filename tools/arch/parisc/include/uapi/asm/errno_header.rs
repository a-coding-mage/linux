/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Depends on constants from <asm-generic/errno-base.h>. */

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
pub const EDEADLOCK: i32 = EDEADLK;
pub const ENOLCK: i32 = 46; /* No record locks available */
pub const EILSEQ: i32 = 47; /* Illegal byte sequence */

pub const ENONET: i32 = 50; /* Machine is not on the network */
pub const ENODATA: i32 = 51; /* No data available */
pub const ETIME: i32 = 52; /* Timer expired */
pub const ENOSR: i32 = 53; /* Out of streams resources */
pub const ENOSTR: i32 = 54; /* Device not a stream */
pub const ENOPKG: i32 = 55; /* Package not installed */

pub const ENOLINK: i32 = 57; /* Link has been severed */
pub const EADV: i32 = 58; /* Advertise error */
pub const ESRMNT: i32 = 59; /* Srmount error */
pub const ECOMM: i32 = 60; /* Communication error on send */
pub const EPROTO: i32 = 61; /* Protocol error */

pub const EMULTIHOP: i32 = 64; /* Multihop attempted */

pub const EDOTDOT: i32 = 66; /* RFS specific error */
pub const EBADMSG: i32 = 67; /* Not a data message */
pub const EFSBADCRC: i32 = EBADMSG; /* Bad CRC detected */
pub const EUSERS: i32 = 68; /* Too many users */
pub const EDQUOT: i32 = 69; /* Quota exceeded */
pub const ESTALE: i32 = 70; /* Stale file handle */
pub const EREMOTE: i32 = 71; /* Object is remote */
pub const EOVERFLOW: i32 = 72; /* Value too large for defined data type */

/* these errnos are defined by Linux but not HPUX. */

pub const EBADE: i32 = 160; /* Invalid exchange */
pub const EBADR: i32 = 161; /* Invalid request descriptor */
pub const EXFULL: i32 = 162; /* Exchange full */
pub const ENOANO: i32 = 163; /* No anode */
pub const EBADRQC: i32 = 164; /* Invalid request code */
pub const EBADSLT: i32 = 165; /* Invalid slot */
pub const EBFONT: i32 = 166; /* Bad font file format */
pub const ENOTUNIQ: i32 = 167; /* Name not unique on network */
pub const EBADFD: i32 = 168; /* File descriptor in bad state */
pub const EREMCHG: i32 = 169; /* Remote address changed */
pub const ELIBACC: i32 = 170; /* Can not access a needed shared library */
pub const ELIBBAD: i32 = 171; /* Accessing a corrupted shared library */
pub const ELIBSCN: i32 = 172; /* .lib section in a.out corrupted */
pub const ELIBMAX: i32 = 173; /* Attempting to link in too many shared libraries */
pub const ELIBEXEC: i32 = 174; /* Cannot exec a shared library directly */
pub const ERESTART: i32 = 175; /* Interrupted system call should be restarted */
pub const ESTRPIPE: i32 = 176; /* Streams pipe error */
pub const EUCLEAN: i32 = 177; /* Structure needs cleaning */
pub const EFSCORRUPTED: i32 = EUCLEAN; /* Filesystem is corrupted */
pub const ENOTNAM: i32 = 178; /* Not a XENIX named type file */
pub const ENAVAIL: i32 = 179; /* No XENIX semaphores available */
pub const EISNAM: i32 = 180; /* Is a named type file */
pub const EREMOTEIO: i32 = 181; /* Remote I/O error */
pub const ENOMEDIUM: i32 = 182; /* No medium found */
pub const EMEDIUMTYPE: i32 = 183; /* Wrong medium type */
pub const ENOKEY: i32 = 184; /* Required key not available */
pub const EKEYEXPIRED: i32 = 185; /* Key has expired */
pub const EKEYREVOKED: i32 = 186; /* Key has been revoked */
pub const EKEYREJECTED: i32 = 187; /* Key was rejected by service */

/* We now return you to your regularly scheduled HPUX. */

pub const ENOTSOCK: i32 = 216; /* Socket operation on non-socket */
pub const EDESTADDRREQ: i32 = 217; /* Destination address required */
pub const EMSGSIZE: i32 = 218; /* Message too long */
pub const EPROTOTYPE: i32 = 219; /* Protocol wrong type for socket */
pub const ENOPROTOOPT: i32 = 220; /* Protocol not available */
pub const EPROTONOSUPPORT: i32 = 221; /* Protocol not supported */
pub const ESOCKTNOSUPPORT: i32 = 222; /* Socket type not supported */
pub const EOPNOTSUPP: i32 = 223; /* Operation not supported on transport endpoint */
pub const EPFNOSUPPORT: i32 = 224; /* Protocol family not supported */
pub const EAFNOSUPPORT: i32 = 225; /* Address family not supported by protocol */
pub const EADDRINUSE: i32 = 226; /* Address already in use */
pub const EADDRNOTAVAIL: i32 = 227; /* Cannot assign requested address */
pub const ENETDOWN: i32 = 228; /* Network is down */
pub const ENETUNREACH: i32 = 229; /* Network is unreachable */
pub const ENETRESET: i32 = 230; /* Network dropped connection because of reset */
pub const ECONNABORTED: i32 = 231; /* Software caused connection abort */
pub const ECONNRESET: i32 = 232; /* Connection reset by peer */
pub const ENOBUFS: i32 = 233; /* No buffer space available */
pub const EISCONN: i32 = 234; /* Transport endpoint is already connected */
pub const ENOTCONN: i32 = 235; /* Transport endpoint is not connected */
pub const ESHUTDOWN: i32 = 236; /* Cannot send after transport endpoint shutdown */
pub const ETOOMANYREFS: i32 = 237; /* Too many references: cannot splice */
pub const ETIMEDOUT: i32 = 238; /* Connection timed out */
pub const ECONNREFUSED: i32 = 239; /* Connection refused */
pub const EREFUSED: i32 = ECONNREFUSED; /* for HP's NFS apparently */
pub const EHOSTDOWN: i32 = 241; /* Host is down */
pub const EHOSTUNREACH: i32 = 242; /* No route to host */

pub const EALREADY: i32 = 244; /* Operation already in progress */
pub const EINPROGRESS: i32 = 245; /* Operation now in progress */
pub const EWOULDBLOCK: i32 = EAGAIN; /* Operation would block (Not HPUX compliant) */
pub const ENOTEMPTY: i32 = 247; /* Directory not empty */
pub const ENAMETOOLONG: i32 = 248; /* File name too long */
pub const ELOOP: i32 = 249; /* Too many symbolic links encountered */
pub const ENOSYS: i32 = 251; /* Function not implemented */

pub const ECANCELLED: i32 = 253; /* aio request was canceled before complete (POSIX.4 / HPUX) */
pub const ECANCELED: i32 = ECANCELLED; /* SuSv3 and Solaris wants one 'L' */

/* for robust mutexes */
pub const EOWNERDEAD: i32 = 254; /* Owner died */
pub const ENOTRECOVERABLE: i32 = 255; /* State not recoverable */

pub const ERFKILL: i32 = 256; /* Operation not possible due to RF-kill */

pub const EHWPOISON: i32 = 257; /* Memory page has hardware error */

pub const EFTYPE: i32 = 258; /* Wrong file type for the intended operation */
