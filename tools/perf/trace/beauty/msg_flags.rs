// SPDX-License-Identifier: LGPL-2.1
// C dependencies: "trace/beauty/beauty.h", <sys/types.h>, <sys/socket.h>

use core::ffi::{c_char, c_int};

const MSG_PROBE: c_int = 0x10;
const MSG_WAITFORONE: c_int = 0x10000;
const MSG_BATCH: c_int = 0x40000;
const MSG_SOCK_DEVMEM: c_int = 0x2000000;
const MSG_ZEROCOPY: c_int = 0x4000000;
const MSG_SPLICE_PAGES: c_int = 0x8000000;
const MSG_FASTOPEN: c_int = 0x20000000;
const MSG_CMSG_CLOEXEC: c_int = 0x40000000;

unsafe extern "C" {
    fn scnprintf(s: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_msg_flags(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let show_prefix = unsafe { (*arg).show_string_prefix };
    let prefix = b"MSG_\0".as_ptr() as *const c_char;
    let mut printed: c_int = 0;
    let mut flags: c_int = unsafe { (*arg).val as c_int };

    if flags == 0 {
        return unsafe { scnprintf(bf, size, b"NONE\0".as_ptr() as *const c_char) as usize };
    }

    macro_rules! p_msg_flag {
        ($n:ident, $msg:expr) => {
            if flags & $msg != 0 {
                printed += unsafe {
                    scnprintf(
                        bf.add(printed as usize),
                        size.wrapping_sub(printed as usize),
                        b"%s%s%s\0".as_ptr() as *const c_char,
                        if printed != 0 {
                            b"|\0".as_ptr() as *const c_char
                        } else {
                            b"\0".as_ptr() as *const c_char
                        },
                        if show_prefix {
                            prefix
                        } else {
                            b"\0".as_ptr() as *const c_char
                        },
                        concat!(stringify!($n), "\0").as_ptr() as *const c_char,
                    )
                };
                flags &= !$msg;
            }
        };
    }

    p_msg_flag!(OOB, MSG_OOB);
    p_msg_flag!(PEEK, MSG_PEEK);
    p_msg_flag!(DONTROUTE, MSG_DONTROUTE);
    p_msg_flag!(CTRUNC, MSG_CTRUNC);
    p_msg_flag!(PROBE, MSG_PROBE);
    p_msg_flag!(TRUNC, MSG_TRUNC);
    p_msg_flag!(DONTWAIT, MSG_DONTWAIT);
    p_msg_flag!(EOR, MSG_EOR);
    p_msg_flag!(WAITALL, MSG_WAITALL);
    p_msg_flag!(FIN, MSG_FIN);
    p_msg_flag!(SYN, MSG_SYN);
    p_msg_flag!(CONFIRM, MSG_CONFIRM);
    p_msg_flag!(RST, MSG_RST);
    p_msg_flag!(ERRQUEUE, MSG_ERRQUEUE);
    p_msg_flag!(NOSIGNAL, MSG_NOSIGNAL);
    p_msg_flag!(MORE, MSG_MORE);
    p_msg_flag!(WAITFORONE, MSG_WAITFORONE);
    p_msg_flag!(BATCH, MSG_BATCH);
    p_msg_flag!(SOCK_DEVMEM, MSG_SOCK_DEVMEM);
    p_msg_flag!(ZEROCOPY, MSG_ZEROCOPY);
    p_msg_flag!(SPLICE_PAGES, MSG_SPLICE_PAGES);
    p_msg_flag!(FASTOPEN, MSG_FASTOPEN);
    p_msg_flag!(CMSG_CLOEXEC, MSG_CMSG_CLOEXEC);

    if flags != 0 {
        printed += unsafe {
            scnprintf(
                bf.add(printed as usize),
                size.wrapping_sub(printed as usize),
                b"%s%#x\0".as_ptr() as *const c_char,
                if printed != 0 {
                    b"|\0".as_ptr() as *const c_char
                } else {
                    b"\0".as_ptr() as *const c_char
                },
                flags,
            )
        };
    }

    printed as usize
}
