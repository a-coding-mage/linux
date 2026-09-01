// SPDX-License-Identifier: LGPL-2.1

use std::os::raw::{c_char, c_int};

// Dependencies from the original C includes:
// "trace/beauty/beauty.h", <sys/types.h>, and <sys/socket.h>.

// Original fallback definitions guarded by #ifndef in C.
pub const SOCK_DCCP: c_int = 6;
pub const SOCK_CLOEXEC: c_int = 0o2000000;
pub const SOCK_NONBLOCK: c_int = 0o0004000;
pub const SOCK_TYPE_MASK: c_int = 0xf;

extern "C" {
    static SOCK_STREAM: c_int;
    static SOCK_DGRAM: c_int;
    static SOCK_RAW: c_int;
    static SOCK_RDM: c_int;
    static SOCK_SEQPACKET: c_int;
    static SOCK_PACKET: c_int;

    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
}

#[repr(C)]
pub struct syscall_arg {
    pub show_string_prefix: bool,
    pub val: c_int,
}

pub unsafe extern "C" fn syscall_arg__scnprintf_socket_type(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let show_prefix = (*arg).show_string_prefix;
    let prefix = b"SOCK_\0".as_ptr() as *const c_char;
    let mut printed: usize;
    let mut type_ = (*arg).val;
    let mut flags = type_ & !SOCK_TYPE_MASK;

    type_ &= SOCK_TYPE_MASK;
    /*
     * Can't use a strarray, MIPS may override for ABI reasons.
     */
    if type_ == SOCK_STREAM {
        printed = scnprintf(
            bf,
            size,
            b"%s%s\0".as_ptr() as *const c_char,
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"STREAM\0".as_ptr() as *const c_char,
        );
    } else if type_ == SOCK_DGRAM {
        printed = scnprintf(
            bf,
            size,
            b"%s%s\0".as_ptr() as *const c_char,
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"DGRAM\0".as_ptr() as *const c_char,
        );
    } else if type_ == SOCK_RAW {
        printed = scnprintf(
            bf,
            size,
            b"%s%s\0".as_ptr() as *const c_char,
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"RAW\0".as_ptr() as *const c_char,
        );
    } else if type_ == SOCK_RDM {
        printed = scnprintf(
            bf,
            size,
            b"%s%s\0".as_ptr() as *const c_char,
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"RDM\0".as_ptr() as *const c_char,
        );
    } else if type_ == SOCK_SEQPACKET {
        printed = scnprintf(
            bf,
            size,
            b"%s%s\0".as_ptr() as *const c_char,
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"SEQPACKET\0".as_ptr() as *const c_char,
        );
    } else if type_ == SOCK_DCCP {
        printed = scnprintf(
            bf,
            size,
            b"%s%s\0".as_ptr() as *const c_char,
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"DCCP\0".as_ptr() as *const c_char,
        );
    } else if type_ == SOCK_PACKET {
        printed = scnprintf(
            bf,
            size,
            b"%s%s\0".as_ptr() as *const c_char,
            if show_prefix {
                prefix
            } else {
                b"\0".as_ptr() as *const c_char
            },
            b"PACKET\0".as_ptr() as *const c_char,
        );
    } else {
        printed = scnprintf(
            bf,
            size,
            b"%#x\0".as_ptr() as *const c_char,
            type_,
        );
    }

    if flags & SOCK_CLOEXEC != 0 {
        printed += scnprintf(
            bf.add(printed),
            size.wrapping_sub(printed),
            b"|%s\0".as_ptr() as *const c_char,
            b"CLOEXEC\0".as_ptr() as *const c_char,
        );
        flags &= !SOCK_CLOEXEC;
    }

    if flags & SOCK_NONBLOCK != 0 {
        printed += scnprintf(
            bf.add(printed),
            size.wrapping_sub(printed),
            b"|%s\0".as_ptr() as *const c_char,
            b"NONBLOCK\0".as_ptr() as *const c_char,
        );
        flags &= !SOCK_NONBLOCK;
    }

    if flags != 0 {
        printed += scnprintf(
            bf.add(printed),
            size.wrapping_sub(printed),
            b"|%#x\0".as_ptr() as *const c_char,
            flags,
        );
    }

    printed
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
