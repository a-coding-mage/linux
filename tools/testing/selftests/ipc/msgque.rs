// SPDX-License-Identifier: GPL-2.0
// C dependencies: stdlib.h, stdio.h, string.h, errno.h, sys/msg.h, fcntl.h,
// and "kselftest.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type key_t = c_int;
type mode_t = c_uint;
type size_t = c_ulong;
type ssize_t = c_long;

const O_WRONLY: c_int = 1;
const IPC_CREAT: c_int = 0o1000;
const IPC_EXCL: c_int = 0o2000;
const IPC_NOWAIT: c_int = 0o4000;
const IPC_RMID: c_int = 0;
const MSG_STAT: c_int = 11;
const MSG_COPY: c_int = 0o40000;

const EINVAL: c_int = 22;
const ENOMSG: c_int = 42;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const EOPNOTSUPP: c_int = 95;

const MAX_MSG_SIZE: usize = 32;

#[repr(C)]
struct ipc_perm {
    __key: key_t,
    uid: c_uint,
    gid: c_uint,
    cuid: c_uint,
    cgid: c_uint,
    mode: mode_t,
    __seq: c_ushort,
    __pad1: c_ushort,
    __glibc_reserved1: c_ulong,
    __glibc_reserved2: c_ulong,
}

type c_ushort = u16;
type msgqnum_t = c_ulong;
type msglen_t = c_ulong;
type time_t = c_long;

#[repr(C)]
struct msqid_ds {
    msg_perm: ipc_perm,
    msg_stime: time_t,
    msg_rtime: time_t,
    msg_ctime: time_t,
    __msg_cbytes: c_ulong,
    msg_qnum: msgqnum_t,
    msg_qbytes: msglen_t,
    msg_lspid: c_int,
    msg_lrpid: c_int,
    __glibc_reserved4: c_ulong,
    __glibc_reserved5: c_ulong,
}

#[repr(C)]
struct msg1 {
    msize: c_int,
    mtype: c_long,
    mtext: [c_char; MAX_MSG_SIZE],
}

const TEST_STRING: &[u8; 15] = b"Test sysv5 msg\0";
const MSG_TYPE: c_long = 1;

const ANOTHER_TEST_STRING: &[u8; 30] = b"Yet another test sysv5 msg\0";
const ANOTHER_MSG_TYPE: c_long = 26538;

#[repr(C)]
struct msgque_data {
    key: key_t,
    msq_id: c_int,
    qbytes: c_int,
    qnum: c_int,
    mode: c_int,
    messages: *mut msg1,
}

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn strlen(s: *const c_char) -> size_t;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn getuid() -> c_uint;
    fn ftok(pathname: *const c_char, proj_id: c_int) -> key_t;
    fn msgget(key: key_t, msgflg: c_int) -> c_int;
    fn msgsnd(msqid: c_int, msgp: *const c_void, msgsz: size_t, msgflg: c_int) -> c_int;
    fn msgrcv(
        msqid: c_int,
        msgp: *mut c_void,
        msgsz: size_t,
        msgtyp: c_long,
        msgflg: c_int,
    ) -> ssize_t;
    fn msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn ksft_test_result_fail(msg: *const c_char, ...);
    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn restore_queue(msgque: *mut msgque_data) -> c_int {
    let fd: c_int;
    let mut ret: c_int;
    let id: c_int;
    let mut i: c_int;
    let mut buf: [c_char; 32] = [0; 32];

    fd = unsafe { open(c"/proc/sys/kernel/msg_next_id".as_ptr(), O_WRONLY) };
    if fd == -1 {
        unsafe { ksft_test_result_fail(c"Failed to open /proc/sys/kernel/msg_next_id\n".as_ptr()) };
        return -unsafe { errno() };
    }
    unsafe { sprintf(buf.as_mut_ptr(), c"%d".as_ptr(), (*msgque).msq_id) };

    ret = unsafe { write(fd, buf.as_ptr() as *const c_void, strlen(buf.as_ptr())) as c_int };
    if ret != unsafe { strlen(buf.as_ptr()) } as c_int {
        unsafe { ksft_test_result_fail(c"Failed to write to /proc/sys/kernel/msg_next_id\n".as_ptr()) };
        return -unsafe { errno() };
    }

    id = unsafe { msgget((*msgque).key, (*msgque).mode | IPC_CREAT | IPC_EXCL) };
    if id == -1 {
        unsafe { ksft_test_result_fail(c"Failed to create queue\n".as_ptr()) };
        return -unsafe { errno() };
    }

    if id != unsafe { (*msgque).msq_id } {
        unsafe {
            ksft_test_result_fail(
                c"Restored queue has wrong id (%d instead of %d)\n".as_ptr(),
                id,
                (*msgque).msq_id,
            )
        };
        ret = -EFAULT;
        unsafe {
            if msgctl(id, IPC_RMID, core::ptr::null_mut()) != 0 {
                printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
            }
        }
        return ret;
    }

    i = 0;
    while i < unsafe { (*msgque).qnum } {
        if unsafe {
            msgsnd(
                (*msgque).msq_id,
                &mut (*(*msgque).messages.add(i as usize)).mtype as *mut c_long as *const c_void,
                (*(*msgque).messages.add(i as usize)).msize as size_t,
                IPC_NOWAIT,
            )
        } != 0
        {
            unsafe { ksft_test_result_fail(c"msgsnd failed (%m)\n".as_ptr()) };
            ret = -unsafe { errno() };
            unsafe {
                if msgctl(id, IPC_RMID, core::ptr::null_mut()) != 0 {
                    printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
                }
            }
            return ret;
        }
        i += 1;
    }
    0
}

unsafe fn check_and_destroy_queue(msgque: *mut msgque_data) -> c_int {
    let mut message: msg1 = unsafe { core::mem::zeroed() };
    let mut cnt: c_int = 0;
    let mut ret: c_int;

    loop {
        ret = unsafe {
            msgrcv(
                (*msgque).msq_id,
                &mut message.mtype as *mut c_long as *mut c_void,
                MAX_MSG_SIZE,
                0,
                IPC_NOWAIT,
            ) as c_int
        };
        if ret < 0 {
            if unsafe { errno() } == ENOMSG {
                break;
            }
            unsafe { ksft_test_result_fail(c"Failed to read IPC message: %m\n".as_ptr()) };
            ret = -unsafe { errno() };
            unsafe {
                if msgctl((*msgque).msq_id, IPC_RMID, core::ptr::null_mut()) != 0 {
                    printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
                    return -errno();
                }
            }
            return ret;
        }
        if ret != unsafe { (*(*msgque).messages.add(cnt as usize)).msize } {
            unsafe {
                ksft_test_result_fail(
                    c"Wrong message size: %d (expected %d)\n".as_ptr(),
                    ret,
                    (*(*msgque).messages.add(cnt as usize)).msize,
                )
            };
            ret = -EINVAL;
            unsafe {
                if msgctl((*msgque).msq_id, IPC_RMID, core::ptr::null_mut()) != 0 {
                    printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
                    return -errno();
                }
            }
            return ret;
        }
        if message.mtype != unsafe { (*(*msgque).messages.add(cnt as usize)).mtype } {
            unsafe { ksft_test_result_fail(c"Wrong message type\n".as_ptr()) };
            ret = -EINVAL;
            unsafe {
                if msgctl((*msgque).msq_id, IPC_RMID, core::ptr::null_mut()) != 0 {
                    printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
                    return -errno();
                }
            }
            return ret;
        }
        if unsafe {
            memcmp(
                message.mtext.as_ptr() as *const c_void,
                (*(*msgque).messages.add(cnt as usize)).mtext.as_ptr() as *const c_void,
                ret as size_t,
            )
        } != 0
        {
            unsafe { ksft_test_result_fail(c"Wrong message content\n".as_ptr()) };
            ret = -EINVAL;
            unsafe {
                if msgctl((*msgque).msq_id, IPC_RMID, core::ptr::null_mut()) != 0 {
                    printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
                    return -errno();
                }
            }
            return ret;
        }
        cnt += 1;
    }

    if cnt != unsafe { (*msgque).qnum } {
        unsafe { ksft_test_result_fail(c"Wrong message number\n".as_ptr()) };
        ret = -EINVAL;
        unsafe {
            if msgctl((*msgque).msq_id, IPC_RMID, core::ptr::null_mut()) != 0 {
                printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
                return -errno();
            }
        }
        return ret;
    }

    ret = 0;
    unsafe {
        if msgctl((*msgque).msq_id, IPC_RMID, core::ptr::null_mut()) != 0 {
            printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
            return -errno();
        }
    }
    ret
}

unsafe fn dump_queue(msgque: *mut msgque_data) -> c_int {
    let mut ds: msqid_ds = unsafe { core::mem::zeroed() };
    let mut kern_id: c_int;
    let mut i: c_int;
    let mut ret: c_int;

    kern_id = 0;
    while kern_id < 256 {
        ret = unsafe { msgctl(kern_id, MSG_STAT, &mut ds) };
        if ret < 0 {
            if unsafe { errno() } == EINVAL {
                kern_id += 1;
                continue;
            }
            unsafe {
                ksft_test_result_fail(
                    c"Failed to get stats for IPC queue with id %d\n".as_ptr(),
                    kern_id,
                )
            };
            return -unsafe { errno() };
        }

        if ret == unsafe { (*msgque).msq_id } {
            break;
        }
        kern_id += 1;
    }

    unsafe {
        (*msgque).messages =
            malloc(core::mem::size_of::<msg1>() as size_t * ds.msg_qnum as size_t) as *mut msg1;
    }
    if unsafe { (*msgque).messages.is_null() } {
        unsafe { ksft_test_result_fail(c"Failed to get stats for IPC queue\n".as_ptr()) };
        return -ENOMEM;
    }

    unsafe {
        (*msgque).qnum = ds.msg_qnum as c_int;
        (*msgque).mode = ds.msg_perm.mode as c_int;
        (*msgque).qbytes = ds.msg_qbytes as c_int;
    }

    i = 0;
    while i < unsafe { (*msgque).qnum } {
        ret = unsafe {
            msgrcv(
                (*msgque).msq_id,
                &mut (*(*msgque).messages.add(i as usize)).mtype as *mut c_long as *mut c_void,
                MAX_MSG_SIZE,
                i as c_long,
                IPC_NOWAIT | MSG_COPY,
            ) as c_int
        };
        if ret < 0 {
            if unsafe { errno() } == EOPNOTSUPP {
                unsafe { ksft_exit_skip(c"MSG_COPY not supported\n".as_ptr()) };
            }

            unsafe {
                ksft_test_result_fail(
                    c"Failed to copy IPC message: %m (%d)\n".as_ptr(),
                    errno(),
                )
            };
            return -unsafe { errno() };
        }
        unsafe {
            (*(*msgque).messages.add(i as usize)).msize = ret;
        }
        i += 1;
    }
    0
}

unsafe fn fill_msgque(msgque: *mut msgque_data) -> c_int {
    let mut msgbuf: msg1 = unsafe { core::mem::zeroed() };

    msgbuf.mtype = MSG_TYPE;
    unsafe {
        memcpy(
            msgbuf.mtext.as_mut_ptr() as *mut c_void,
            TEST_STRING.as_ptr() as *const c_void,
            TEST_STRING.len() as size_t,
        )
    };
    if unsafe {
        msgsnd(
            (*msgque).msq_id,
            &mut msgbuf.mtype as *mut c_long as *const c_void,
            TEST_STRING.len() as size_t,
            IPC_NOWAIT,
        )
    } != 0
    {
        unsafe { ksft_test_result_fail(c"First message send failed (%m)\n".as_ptr()) };
        return -unsafe { errno() };
    }

    msgbuf.mtype = ANOTHER_MSG_TYPE;
    unsafe {
        memcpy(
            msgbuf.mtext.as_mut_ptr() as *mut c_void,
            ANOTHER_TEST_STRING.as_ptr() as *const c_void,
            ANOTHER_TEST_STRING.len() as size_t,
        )
    };
    if unsafe {
        msgsnd(
            (*msgque).msq_id,
            &mut msgbuf.mtype as *mut c_long as *const c_void,
            ANOTHER_TEST_STRING.len() as size_t,
            IPC_NOWAIT,
        )
    } != 0
    {
        unsafe { ksft_test_result_fail(c"Second message send failed (%m)\n".as_ptr()) };
        return -unsafe { errno() };
    }
    0
}

unsafe fn main_0(_argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut err: c_int;
    let mut msgque: msgque_data = unsafe { core::mem::zeroed() };

    if unsafe { getuid() } != 0 {
        unsafe { ksft_exit_skip(c"Please run the test as root - Exiting.\n".as_ptr()) };
    }

    msgque.key = unsafe { ftok(*argv.add(0), 822155650) };
    if msgque.key == -1 {
        unsafe { ksft_test_result_fail(c"Can't make key: %d\n".as_ptr(), -errno()) };
        unsafe { ksft_exit_fail() };
    }

    msgque.msq_id = unsafe { msgget(msgque.key, IPC_CREAT | IPC_EXCL | 0o666) };
    if msgque.msq_id == -1 {
        err = -unsafe { errno() };
        unsafe { ksft_test_result_fail(c"Can't create queue: %d\n".as_ptr(), err) };
        unsafe { ksft_exit_fail() };
    }

    err = unsafe { fill_msgque(&mut msgque) };
    if err != 0 {
        unsafe { ksft_test_result_fail(c"Failed to fill queue: %d\n".as_ptr(), err) };
        unsafe {
            if msgctl(msgque.msq_id, IPC_RMID, core::ptr::null_mut()) != 0 {
                printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
                ksft_exit_fail();
            }
        }
        unsafe { ksft_exit_fail() };
    }

    err = unsafe { dump_queue(&mut msgque) };
    if err != 0 {
        unsafe { ksft_test_result_fail(c"Failed to dump queue: %d\n".as_ptr(), err) };
        unsafe {
            if msgctl(msgque.msq_id, IPC_RMID, core::ptr::null_mut()) != 0 {
                printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
                ksft_exit_fail();
            }
        }
        unsafe { ksft_exit_fail() };
    }

    err = unsafe { check_and_destroy_queue(&mut msgque) };
    if err != 0 {
        unsafe {
            ksft_test_result_fail(
                c"Failed to check and destroy queue: %d\n".as_ptr(),
                err,
            )
        };
        unsafe { ksft_exit_fail() };
    }

    err = unsafe { restore_queue(&mut msgque) };
    if err != 0 {
        unsafe { ksft_test_result_fail(c"Failed to restore queue: %d\n".as_ptr(), err) };
        unsafe {
            if msgctl(msgque.msq_id, IPC_RMID, core::ptr::null_mut()) != 0 {
                printf(c"Failed to destroy queue: %d\n".as_ptr(), -errno());
                ksft_exit_fail();
            }
        }
        unsafe { ksft_exit_fail() };
    }

    err = unsafe { check_and_destroy_queue(&mut msgque) };
    if err != 0 {
        unsafe { ksft_test_result_fail(c"Failed to test queue: %d\n".as_ptr(), err) };
        unsafe { ksft_exit_fail() };
    }
    unsafe { ksft_exit_pass() };
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    args.push(core::ptr::null_mut());
    unsafe {
        main_0((args.len() - 1) as c_int, args.as_mut_ptr());
    }
}
