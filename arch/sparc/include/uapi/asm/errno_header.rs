/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// These match the SunOS error numbering scheme.
// Dependency supplied by asm-generic/errno-base.h: EAGAIN.

pub const EWOULDBLOCK: i32 = EAGAIN; // Operation would block
pub const EINPROGRESS: i32 = 36; // Operation now in progress
pub const EALREADY: i32 = 37; // Operation already in progress
pub const ENOTSOCK: i32 = 38; // Socket operation on non-socket
pub const EDESTADDRREQ: i32 = 39; // Destination address required
pub const EMSGSIZE: i32 = 40; // Message too long
pub const EPROTOTYPE: i32 = 41; // Protocol wrong type for socket
pub const ENOPROTOOPT: i32 = 42; // Protocol not available
pub const EPROTONOSUPPORT: i32 = 43; // Protocol not supported
pub const ESOCKTNOSUPPORT: i32 = 44; // Socket type not supported
pub const EOPNOTSUPP: i32 = 45; // Op not supported on transport endpoint
pub const EPFNOSUPPORT: i32 = 46; // Protocol family not supported
pub const EAFNOSUPPORT: i32 = 47; // Address family not supported by protocol
pub const EADDRINUSE: i32 = 48; // Address already in use
pub const EADDRNOTAVAIL: i32 = 49; // Cannot assign requested address
pub const ENETDOWN: i32 = 50; // Network is down
pub const ENETUNREACH: i32 = 51; // Network is unreachable
pub const ENETRESET: i32 = 52; // Net dropped connection because of reset
pub const ECONNABORTED: i32 = 53; // Software caused connection abort
pub const ECONNRESET: i32 = 54; // Connection reset by peer
pub const ENOBUFS: i32 = 55; // No buffer space available
pub const EISCONN: i32 = 56; // Transport endpoint is already connected
pub const ENOTCONN: i32 = 57; // Transport endpoint is not connected
pub const ESHUTDOWN: i32 = 58; // No send after transport endpoint shutdown
pub const ETOOMANYREFS: i32 = 59; // Too many references: cannot splice
pub const ETIMEDOUT: i32 = 60; // Connection timed out
pub const ECONNREFUSED: i32 = 61; // Connection refused
pub const ELOOP: i32 = 62; // Too many symbolic links encountered
pub const ENAMETOOLONG: i32 = 63; // File name too long
pub const EHOSTDOWN: i32 = 64; // Host is down
pub const EHOSTUNREACH: i32 = 65; // No route to host
pub const ENOTEMPTY: i32 = 66; // Directory not empty
pub const EPROCLIM: i32 = 67; // SUNOS: Too many processes
pub const EUSERS: i32 = 68; // Too many users
pub const EDQUOT: i32 = 69; // Quota exceeded
pub const ESTALE: i32 = 70; // Stale file handle
pub const EREMOTE: i32 = 71; // Object is remote
pub const ENOSTR: i32 = 72; // Device not a stream
pub const ETIME: i32 = 73; // Timer expired
pub const ENOSR: i32 = 74; // Out of streams resources
pub const ENOMSG: i32 = 75; // No message of desired type
pub const EBADMSG: i32 = 76; // Not a data message
pub const EFSBADCRC: i32 = EBADMSG; // Bad CRC detected
pub const EIDRM: i32 = 77; // Identifier removed
pub const EDEADLK: i32 = 78; // Resource deadlock would occur
pub const ENOLCK: i32 = 79; // No record locks available
pub const ENONET: i32 = 80; // Machine is not on the network
pub const ERREMOTE: i32 = 81; // SunOS: Too many lvls of remote in path
pub const ENOLINK: i32 = 82; // Link has been severed
pub const EADV: i32 = 83; // Advertise error
pub const ESRMNT: i32 = 84; // Srmount error
pub const ECOMM: i32 = 85; // Communication error on send
pub const EPROTO: i32 = 86; // Protocol error
pub const EMULTIHOP: i32 = 87; // Multihop attempted
pub const EDOTDOT: i32 = 88; // RFS specific error
pub const EREMCHG: i32 = 89; // Remote address changed
pub const ENOSYS: i32 = 90; // Function not implemented

// The rest have no SunOS equivalent.
pub const ESTRPIPE: i32 = 91; // Streams pipe error
pub const EOVERFLOW: i32 = 92; // Value too large for defined data type
pub const EBADFD: i32 = 93; // File descriptor in bad state
pub const ECHRNG: i32 = 94; // Channel number out of range
pub const EL2NSYNC: i32 = 95; // Level 2 not synchronized
pub const EL3HLT: i32 = 96; // Level 3 halted
pub const EL3RST: i32 = 97; // Level 3 reset
pub const ELNRNG: i32 = 98; // Link number out of range
pub const EUNATCH: i32 = 99; // Protocol driver not attached
pub const ENOCSI: i32 = 100; // No CSI structure available
pub const EL2HLT: i32 = 101; // Level 2 halted
pub const EBADE: i32 = 102; // Invalid exchange
pub const EBADR: i32 = 103; // Invalid request descriptor
pub const EXFULL: i32 = 104; // Exchange full
pub const ENOANO: i32 = 105; // No anode
pub const EBADRQC: i32 = 106; // Invalid request code
pub const EBADSLT: i32 = 107; // Invalid slot
pub const EDEADLOCK: i32 = 108; // File locking deadlock error
pub const EBFONT: i32 = 109; // Bad font file format
pub const ELIBEXEC: i32 = 110; // Cannot exec a shared library directly
pub const ENODATA: i32 = 111; // No data available
pub const ELIBBAD: i32 = 112; // Accessing a corrupted shared library
pub const ENOPKG: i32 = 113; // Package not installed
pub const ELIBACC: i32 = 114; // Can not access a needed shared library
pub const ENOTUNIQ: i32 = 115; // Name not unique on network
pub const ERESTART: i32 = 116; // Interrupted syscall should be restarted
pub const EUCLEAN: i32 = 117; // Structure needs cleaning
pub const EFSCORRUPTED: i32 = EUCLEAN; // Filesystem is corrupted
pub const ENOTNAM: i32 = 118; // Not a XENIX named type file
pub const ENAVAIL: i32 = 119; // No XENIX semaphores available
pub const EISNAM: i32 = 120; // Is a named type file
pub const EREMOTEIO: i32 = 121; // Remote I/O error
pub const EILSEQ: i32 = 122; // Illegal byte sequence
pub const ELIBMAX: i32 = 123; // Atmpt to link in too many shared libs
pub const ELIBSCN: i32 = 124; // .lib section in a.out corrupted

pub const ENOMEDIUM: i32 = 125; // No medium found
pub const EMEDIUMTYPE: i32 = 126; // Wrong medium type
pub const ECANCELED: i32 = 127; // Operation Cancelled
pub const ENOKEY: i32 = 128; // Required key not available
pub const EKEYEXPIRED: i32 = 129; // Key has expired
pub const EKEYREVOKED: i32 = 130; // Key has been revoked
pub const EKEYREJECTED: i32 = 131; // Key was rejected by service

// for robust mutexes
pub const EOWNERDEAD: i32 = 132; // Owner died
pub const ENOTRECOVERABLE: i32 = 133; // State not recoverable

pub const ERFKILL: i32 = 134; // Operation not possible due to RF-kill
pub const EHWPOISON: i32 = 135; // Memory page has hardware error
pub const EFTYPE: i32 = 136; // Wrong file type for the intended operation

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
