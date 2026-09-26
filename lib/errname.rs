// SPDX-License-Identifier: GPL-2.0
//! Architecture-correct errno names with stable, terminated C string storage.
//!
//! Numeric values come from compiler-evaluated errno enum bindings, so an
//! architecture's final macro redefinitions are respected. Compile the
//! implementation exactly once through errname_rust.rs; callers use bindings.

use kernel::bindings::*;
use kernel::ffi::{c_char, c_int};

type Entry = (u32, &'static [u8], bool);

macro_rules! entry {
    ($name:ident, $value:ident) => {
        entry!($name, $value, true)
    };
    ($name:ident, $value:ident, $enabled:expr) => {
        ($value, concat!("-", stringify!($name), "\0").as_bytes(), $enabled)
    };
}

// These optional names follow the actual architecture errno headers; no absent
// macro gets a fabricated numeric value. EDEADLOCK is a separate name only when
// the generated constants differ, as in the original C preprocessor condition.
const LOW_ENTRIES: &[Entry] = &[
    entry!(E2BIG, RUST_ERRNAME_E2BIG),
    entry!(EACCES, RUST_ERRNAME_EACCES),
    entry!(EADDRINUSE, RUST_ERRNAME_EADDRINUSE),
    entry!(EADDRNOTAVAIL, RUST_ERRNAME_EADDRNOTAVAIL),
    entry!(EADV, RUST_ERRNAME_EADV),
    entry!(EAFNOSUPPORT, RUST_ERRNAME_EAFNOSUPPORT),
    entry!(EAGAIN, RUST_ERRNAME_EAGAIN),
    entry!(EALREADY, RUST_ERRNAME_EALREADY),
    entry!(EBADE, RUST_ERRNAME_EBADE),
    entry!(EBADF, RUST_ERRNAME_EBADF),
    entry!(EBADFD, RUST_ERRNAME_EBADFD),
    entry!(EBADMSG, RUST_ERRNAME_EBADMSG),
    entry!(EBADR, RUST_ERRNAME_EBADR),
    entry!(EBADRQC, RUST_ERRNAME_EBADRQC),
    entry!(EBADSLT, RUST_ERRNAME_EBADSLT),
    entry!(EBFONT, RUST_ERRNAME_EBFONT),
    entry!(EBUSY, RUST_ERRNAME_EBUSY),
    entry!(ECANCELED, RUST_ERRNAME_ECANCELED),
    entry!(ECHILD, RUST_ERRNAME_ECHILD),
    entry!(ECHRNG, RUST_ERRNAME_ECHRNG),
    entry!(ECOMM, RUST_ERRNAME_ECOMM),
    entry!(ECONNABORTED, RUST_ERRNAME_ECONNABORTED),
    entry!(ECONNREFUSED, RUST_ERRNAME_ECONNREFUSED),
    entry!(ECONNRESET, RUST_ERRNAME_ECONNRESET),
    entry!(EDEADLK, RUST_ERRNAME_EDEADLK),
    entry!(EDEADLOCK, RUST_ERRNAME_EDEADLOCK, RUST_ERRNAME_EDEADLK != RUST_ERRNAME_EDEADLOCK),
    entry!(EDESTADDRREQ, RUST_ERRNAME_EDESTADDRREQ),
    entry!(EDOM, RUST_ERRNAME_EDOM),
    entry!(EDOTDOT, RUST_ERRNAME_EDOTDOT),
    #[cfg(not(CONFIG_MIPS))]
    entry!(EDQUOT, RUST_ERRNAME_EDQUOT),
    entry!(EEXIST, RUST_ERRNAME_EEXIST),
    entry!(EFAULT, RUST_ERRNAME_EFAULT),
    entry!(EFBIG, RUST_ERRNAME_EFBIG),
    entry!(EHOSTDOWN, RUST_ERRNAME_EHOSTDOWN),
    entry!(EHOSTUNREACH, RUST_ERRNAME_EHOSTUNREACH),
    entry!(EHWPOISON, RUST_ERRNAME_EHWPOISON),
    entry!(EIDRM, RUST_ERRNAME_EIDRM),
    entry!(EILSEQ, RUST_ERRNAME_EILSEQ),
    #[cfg(CONFIG_MIPS)]
    entry!(EINIT, RUST_ERRNAME_EINIT),
    entry!(EINPROGRESS, RUST_ERRNAME_EINPROGRESS),
    entry!(EINTR, RUST_ERRNAME_EINTR),
    entry!(EINVAL, RUST_ERRNAME_EINVAL),
    entry!(EIO, RUST_ERRNAME_EIO),
    entry!(EISCONN, RUST_ERRNAME_EISCONN),
    entry!(EISDIR, RUST_ERRNAME_EISDIR),
    entry!(EISNAM, RUST_ERRNAME_EISNAM),
    entry!(EKEYEXPIRED, RUST_ERRNAME_EKEYEXPIRED),
    entry!(EKEYREJECTED, RUST_ERRNAME_EKEYREJECTED),
    entry!(EKEYREVOKED, RUST_ERRNAME_EKEYREVOKED),
    entry!(EL2HLT, RUST_ERRNAME_EL2HLT),
    entry!(EL2NSYNC, RUST_ERRNAME_EL2NSYNC),
    entry!(EL3HLT, RUST_ERRNAME_EL3HLT),
    entry!(EL3RST, RUST_ERRNAME_EL3RST),
    entry!(ELIBACC, RUST_ERRNAME_ELIBACC),
    entry!(ELIBBAD, RUST_ERRNAME_ELIBBAD),
    entry!(ELIBEXEC, RUST_ERRNAME_ELIBEXEC),
    entry!(ELIBMAX, RUST_ERRNAME_ELIBMAX),
    entry!(ELIBSCN, RUST_ERRNAME_ELIBSCN),
    entry!(ELNRNG, RUST_ERRNAME_ELNRNG),
    entry!(ELOOP, RUST_ERRNAME_ELOOP),
    entry!(EMEDIUMTYPE, RUST_ERRNAME_EMEDIUMTYPE),
    entry!(EMFILE, RUST_ERRNAME_EMFILE),
    entry!(EMLINK, RUST_ERRNAME_EMLINK),
    entry!(EMSGSIZE, RUST_ERRNAME_EMSGSIZE),
    entry!(EMULTIHOP, RUST_ERRNAME_EMULTIHOP),
    entry!(ENAMETOOLONG, RUST_ERRNAME_ENAMETOOLONG),
    entry!(ENAVAIL, RUST_ERRNAME_ENAVAIL),
    entry!(ENETDOWN, RUST_ERRNAME_ENETDOWN),
    entry!(ENETRESET, RUST_ERRNAME_ENETRESET),
    entry!(ENETUNREACH, RUST_ERRNAME_ENETUNREACH),
    entry!(ENFILE, RUST_ERRNAME_ENFILE),
    entry!(ENOANO, RUST_ERRNAME_ENOANO),
    entry!(ENOBUFS, RUST_ERRNAME_ENOBUFS),
    entry!(ENOCSI, RUST_ERRNAME_ENOCSI),
    entry!(ENODATA, RUST_ERRNAME_ENODATA),
    entry!(ENODEV, RUST_ERRNAME_ENODEV),
    entry!(ENOENT, RUST_ERRNAME_ENOENT),
    entry!(ENOEXEC, RUST_ERRNAME_ENOEXEC),
    entry!(ENOKEY, RUST_ERRNAME_ENOKEY),
    entry!(ENOLCK, RUST_ERRNAME_ENOLCK),
    entry!(ENOLINK, RUST_ERRNAME_ENOLINK),
    entry!(ENOMEDIUM, RUST_ERRNAME_ENOMEDIUM),
    entry!(ENOMEM, RUST_ERRNAME_ENOMEM),
    entry!(ENOMSG, RUST_ERRNAME_ENOMSG),
    entry!(ENONET, RUST_ERRNAME_ENONET),
    entry!(ENOPKG, RUST_ERRNAME_ENOPKG),
    entry!(ENOPROTOOPT, RUST_ERRNAME_ENOPROTOOPT),
    entry!(ENOSPC, RUST_ERRNAME_ENOSPC),
    entry!(ENOSR, RUST_ERRNAME_ENOSR),
    entry!(ENOSTR, RUST_ERRNAME_ENOSTR),
    entry!(ENOSYS, RUST_ERRNAME_ENOSYS),
    entry!(ENOTBLK, RUST_ERRNAME_ENOTBLK),
    entry!(ENOTCONN, RUST_ERRNAME_ENOTCONN),
    entry!(ENOTDIR, RUST_ERRNAME_ENOTDIR),
    entry!(ENOTEMPTY, RUST_ERRNAME_ENOTEMPTY),
    entry!(ENOTNAM, RUST_ERRNAME_ENOTNAM),
    entry!(ENOTRECOVERABLE, RUST_ERRNAME_ENOTRECOVERABLE),
    entry!(ENOTSOCK, RUST_ERRNAME_ENOTSOCK),
    entry!(ENOTTY, RUST_ERRNAME_ENOTTY),
    entry!(ENOTUNIQ, RUST_ERRNAME_ENOTUNIQ),
    entry!(ENXIO, RUST_ERRNAME_ENXIO),
    entry!(EOPNOTSUPP, RUST_ERRNAME_EOPNOTSUPP),
    entry!(EOVERFLOW, RUST_ERRNAME_EOVERFLOW),
    entry!(EOWNERDEAD, RUST_ERRNAME_EOWNERDEAD),
    entry!(EPERM, RUST_ERRNAME_EPERM),
    entry!(EPFNOSUPPORT, RUST_ERRNAME_EPFNOSUPPORT),
    entry!(EPIPE, RUST_ERRNAME_EPIPE),
    #[cfg(CONFIG_SPARC)]
    entry!(EPROCLIM, RUST_ERRNAME_EPROCLIM),
    entry!(EPROTO, RUST_ERRNAME_EPROTO),
    entry!(EPROTONOSUPPORT, RUST_ERRNAME_EPROTONOSUPPORT),
    entry!(EPROTOTYPE, RUST_ERRNAME_EPROTOTYPE),
    entry!(ERANGE, RUST_ERRNAME_ERANGE),
    entry!(EREMCHG, RUST_ERRNAME_EREMCHG),
    #[cfg(CONFIG_MIPS)]
    entry!(EREMDEV, RUST_ERRNAME_EREMDEV),
    entry!(EREMOTE, RUST_ERRNAME_EREMOTE),
    entry!(EREMOTEIO, RUST_ERRNAME_EREMOTEIO),
    entry!(ERESTART, RUST_ERRNAME_ERESTART),
    entry!(ERFKILL, RUST_ERRNAME_ERFKILL),
    entry!(EROFS, RUST_ERRNAME_EROFS),
    #[cfg(CONFIG_SPARC)]
    entry!(ERREMOTE, RUST_ERRNAME_ERREMOTE),
    entry!(ESHUTDOWN, RUST_ERRNAME_ESHUTDOWN),
    entry!(ESOCKTNOSUPPORT, RUST_ERRNAME_ESOCKTNOSUPPORT),
    entry!(ESPIPE, RUST_ERRNAME_ESPIPE),
    entry!(ESRCH, RUST_ERRNAME_ESRCH),
    entry!(ESRMNT, RUST_ERRNAME_ESRMNT),
    entry!(ESTALE, RUST_ERRNAME_ESTALE),
    entry!(ESTRPIPE, RUST_ERRNAME_ESTRPIPE),
    entry!(ETIME, RUST_ERRNAME_ETIME),
    entry!(ETIMEDOUT, RUST_ERRNAME_ETIMEDOUT),
    entry!(ETOOMANYREFS, RUST_ERRNAME_ETOOMANYREFS),
    entry!(ETXTBSY, RUST_ERRNAME_ETXTBSY),
    entry!(EUCLEAN, RUST_ERRNAME_EUCLEAN),
    entry!(EUNATCH, RUST_ERRNAME_EUNATCH),
    entry!(EUSERS, RUST_ERRNAME_EUSERS),
    entry!(EXDEV, RUST_ERRNAME_EXDEV),
    entry!(EXFULL, RUST_ERRNAME_EXFULL),
];

const HIGH_ENTRIES: &[Entry] = &[
    entry!(ERESTARTSYS, RUST_ERRNAME_ERESTARTSYS),
    entry!(ERESTARTNOINTR, RUST_ERRNAME_ERESTARTNOINTR),
    entry!(ERESTARTNOHAND, RUST_ERRNAME_ERESTARTNOHAND),
    entry!(ENOIOCTLCMD, RUST_ERRNAME_ENOIOCTLCMD),
    entry!(ERESTART_RESTARTBLOCK, RUST_ERRNAME_ERESTART_RESTARTBLOCK),
    entry!(EPROBE_DEFER, RUST_ERRNAME_EPROBE_DEFER),
    entry!(EOPENSTALE, RUST_ERRNAME_EOPENSTALE),
    entry!(ENOPARAM, RUST_ERRNAME_ENOPARAM),
    entry!(EBADHANDLE, RUST_ERRNAME_EBADHANDLE),
    entry!(ENOTSYNC, RUST_ERRNAME_ENOTSYNC),
    entry!(EBADCOOKIE, RUST_ERRNAME_EBADCOOKIE),
    entry!(ENOTSUPP, RUST_ERRNAME_ENOTSUPP),
    entry!(ETOOSMALL, RUST_ERRNAME_ETOOSMALL),
    entry!(ESERVERFAULT, RUST_ERRNAME_ESERVERFAULT),
    entry!(EBADTYPE, RUST_ERRNAME_EBADTYPE),
    entry!(EJUKEBOX, RUST_ERRNAME_EJUKEBOX),
    entry!(EIOCBQUEUED, RUST_ERRNAME_EIOCBQUEUED),
    entry!(ERECALLCONFLICT, RUST_ERRNAME_ERECALLCONFLICT),
];

const _: () = assert!(RUST_ERRNAME_EAGAIN == RUST_ERRNAME_EWOULDBLOCK);
#[cfg(CONFIG_PARISC)]
const _: () = {
    assert!(RUST_ERRNAME_EREFUSED == RUST_ERRNAME_ECONNREFUSED);
    assert!(RUST_ERRNAME_ECANCELLED == RUST_ERRNAME_ECANCELED);
};

const fn table_length(entries: &[Entry], base: u32, limit: u32) -> usize {
    let mut length = 0;
    let mut index = 0;
    while index < entries.len() {
        let (number, _, enabled) = entries[index];
        if enabled {
            assert!(number > 0 && number >= base && number <= limit);
            let end = (number - base) as usize + 1;
            if end > length {
                length = end;
            }
        }
        index += 1;
    }
    length
}

const fn table<const N: usize>(entries: &[Entry], base: u32) -> [Option<&'static u8>; N] {
    let mut names = [None; N];
    let mut index = 0;
    while index < entries.len() {
        let (number, name, enabled) = entries[index];
        if enabled {
            // A reference to the first byte retains the full static string's
            // provenance while keeping each sparse table slot pointer-sized.
            names[(number - base) as usize] = Some(&name[0]);
        }
        index += 1;
    }
    names
}

static NAMES_0: [Option<&u8>; table_length(LOW_ENTRIES, 0, 300)] = table(LOW_ENTRIES, 0);
static NAMES_512: [Option<&u8>; table_length(HIGH_ENTRIES, 512, 550)] = table(HIGH_ENTRIES, 512);

fn negative_name(number: u32) -> *const c_char {
    let name = if number < NAMES_0.len() as u32 {
        NAMES_0[number as usize]
    } else if number >= 512 && number - 512 < NAMES_512.len() as u32 {
        NAMES_512[(number - 512) as usize]
    } else {
        #[cfg(CONFIG_MIPS)]
        if number == RUST_ERRNAME_EDQUOT {
            return b"-EDQUOT\0".as_ptr().cast();
        }
        None
    };
    name.map_or(core::ptr::null(), |byte| core::ptr::from_ref(byte).cast())
}

/// Return a permanent errno name, or null for zero and unrecognized values.
///
/// Positive names point one byte past the same negative name's leading '-'.
/// INT_MIN has an unsigned magnitude of 2^31 and therefore returns null.
#[no_mangle]
pub extern "C" fn errname(error: c_int) -> *const c_char {
    let name = negative_name(error.unsigned_abs());
    if error > 0 && !name.is_null() {
        name.wrapping_add(1)
    } else {
        name
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
