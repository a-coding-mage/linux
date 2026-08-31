// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Dependencies from linux/bpf.h, bpf/bpf_helpers.h, bpf_misc.h, and test_user_ringbuf.h. */

use core::ffi::c_void;

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_USER_RINGBUF: u32 = 30;
const BPF_MAP_TYPE_RINGBUF: u32 = 27;

extern "C" {
    static mut TEST_MSG_OP_INC64: u32;
    static mut TEST_MSG_OP_INC32: u32;
    static mut TEST_MSG_OP_MUL64: u32;
    static mut TEST_MSG_OP_MUL32: u32;
    static mut TEST_MSG_OP_NUM_OPS: u32;
    static mut TEST_OP_64: i32;
    static mut TEST_OP_32: i32;

    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_dynptr_read(
        dst: *mut c_void,
        len: u32,
        src: *mut bpf_dynptr,
        offset: u32,
        flags: u64,
    ) -> i32;
    fn bpf_dynptr_data(dynptr: *mut bpf_dynptr, offset: u32, len: u32) -> *mut c_void;
    fn bpf_printk(fmt: *const u8, ...) -> i32;
    fn bpf_ringbuf_reserve(ringbuf: *mut c_void, size: u64, flags: u64) -> *mut c_void;
    fn bpf_ringbuf_discard(data: *mut c_void, flags: u64);
    fn bpf_ringbuf_submit(data: *mut c_void, flags: u64);
    fn bpf_loop(
        nr_loops: u32,
        callback_fn: extern "C" fn(__u32, *mut c_void) -> i32,
        callback_ctx: *mut c_void,
        flags: u64,
    ) -> i64;
    fn bpf_user_ringbuf_drain(
        ringbuf: *mut c_void,
        callback_fn: extern "C" fn(*mut bpf_dynptr, *mut c_void) -> i64,
        callback_ctx: *mut c_void,
        flags: u64,
    ) -> i64;
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sample {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_msg {
    pub msg_op: u32,
    pub operand_32: u32,
    pub operand_64: u64,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = ".maps"]
pub static mut user_ringbuf: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_USER_RINGBUF,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut kernel_ringbuf: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_RINGBUF,
};

/* inputs */
#[no_mangle]
pub static mut pid: i32 = 0;
#[no_mangle]
pub static mut err: i32 = 0;
#[no_mangle]
pub static mut val: i32 = 0;

#[no_mangle]
pub static mut read: i32 = 0;

/* Counter used for end-to-end protocol test */
#[no_mangle]
pub static mut kern_mutated: __u64 = 0;
#[no_mangle]
pub static mut user_mutated: __u64 = 0;
#[no_mangle]
pub static mut expected_user_mutated: __u64 = 0;

unsafe fn is_test_process() -> i32 {
    let cur_pid: i32 = (bpf_get_current_pid_tgid() >> 32) as i32;

    (cur_pid == pid) as i32
}

extern "C" fn record_sample(dynptr: *mut bpf_dynptr, _context: *mut c_void) -> i64 {
    unsafe {
        static mut NUM_CALLS: i32 = 0;

        let mut sample_ptr: *const sample = core::ptr::null();
        let mut stack_sample: sample = core::mem::zeroed();
        let status: i32;

        if NUM_CALLS % 2 == 0 {
            NUM_CALLS += 1;
            status = bpf_dynptr_read(
                &mut stack_sample as *mut sample as *mut c_void,
                core::mem::size_of_val(&stack_sample) as u32,
                dynptr,
                0,
                0,
            );
            if status != 0 {
                bpf_printk(c"bpf_dynptr_read() failed: %d\n".as_ptr() as *const u8, status);
                err = 1;
                return 1;
            }
        } else {
            NUM_CALLS += 1;
            sample_ptr = bpf_dynptr_data(
                dynptr,
                0,
                core::mem::size_of::<sample>() as u32,
            ) as *const sample;
            if sample_ptr.is_null() {
                bpf_printk(c"Unexpectedly failed to get sample\n".as_ptr() as *const u8);
                err = 2;
                return 1;
            }
            stack_sample = *sample_ptr;
        }

        core::sync::atomic::AtomicI32::from_ptr(&mut read).fetch_add(
            1,
            core::sync::atomic::Ordering::SeqCst,
        );
        let _ = sample_ptr;
        let _ = stack_sample;
        0
    }
}

unsafe fn handle_sample_msg(msg: *const test_msg) {
    if (*msg).msg_op == TEST_MSG_OP_INC64 {
        kern_mutated = kern_mutated.wrapping_add((*msg).operand_64);
    } else if (*msg).msg_op == TEST_MSG_OP_INC32 {
        kern_mutated = kern_mutated.wrapping_add((*msg).operand_32 as __u64);
    } else if (*msg).msg_op == TEST_MSG_OP_MUL64 {
        kern_mutated = kern_mutated.wrapping_mul((*msg).operand_64);
    } else if (*msg).msg_op == TEST_MSG_OP_MUL32 {
        kern_mutated = kern_mutated.wrapping_mul((*msg).operand_32 as __u64);
    } else {
        bpf_printk(c"Unrecognized op %d\n".as_ptr() as *const u8, (*msg).msg_op);
        err = 2;
    }
}

extern "C" fn read_protocol_msg(dynptr: *mut bpf_dynptr, _context: *mut c_void) -> i64 {
    unsafe {
        let mut msg: *const test_msg = core::ptr::null();

        msg = bpf_dynptr_data(dynptr, 0, core::mem::size_of::<test_msg>() as u32) as *const test_msg;
        if msg.is_null() {
            err = 1;
            bpf_printk(c"Unexpectedly failed to get msg\n".as_ptr() as *const u8);
            return 0;
        }

        handle_sample_msg(msg);

        0
    }
}

extern "C" fn publish_next_kern_msg(index: __u32, _context: *mut c_void) -> i32 {
    unsafe {
        let mut msg: *mut test_msg = core::ptr::null_mut();
        let operand_64: i32 = TEST_OP_64;
        let operand_32: i32 = TEST_OP_32;

        msg = bpf_ringbuf_reserve(
            &mut kernel_ringbuf as *mut bpf_map_def as *mut c_void,
            core::mem::size_of::<test_msg>() as u64,
            0,
        ) as *mut test_msg;
        if msg.is_null() {
            err = 4;
            return 1;
        }

        let op = index % TEST_MSG_OP_NUM_OPS;
        if op == TEST_MSG_OP_INC64 {
            (*msg).operand_64 = operand_64 as u64;
            (*msg).msg_op = TEST_MSG_OP_INC64;
            expected_user_mutated = expected_user_mutated.wrapping_add(operand_64 as u64);
        } else if op == TEST_MSG_OP_INC32 {
            (*msg).operand_32 = operand_32 as u32;
            (*msg).msg_op = TEST_MSG_OP_INC32;
            expected_user_mutated = expected_user_mutated.wrapping_add(operand_32 as u64);
        } else if op == TEST_MSG_OP_MUL64 {
            (*msg).operand_64 = operand_64 as u64;
            (*msg).msg_op = TEST_MSG_OP_MUL64;
            expected_user_mutated = expected_user_mutated.wrapping_mul(operand_64 as u64);
        } else if op == TEST_MSG_OP_MUL32 {
            (*msg).operand_32 = operand_32 as u32;
            (*msg).msg_op = TEST_MSG_OP_MUL32;
            expected_user_mutated = expected_user_mutated.wrapping_mul(operand_32 as u64);
        } else {
            bpf_ringbuf_discard(msg as *mut c_void, 0);
            err = 5;
            return 1;
        }

        bpf_ringbuf_submit(msg as *mut c_void, 0);

        0
    }
}

unsafe fn publish_kern_messages() {
    if expected_user_mutated != user_mutated {
        bpf_printk(
            c"%lu != %lu\n".as_ptr() as *const u8,
            expected_user_mutated,
            user_mutated,
        );
        err = 3;
        return;
    }

    bpf_loop(8, publish_next_kern_msg, core::ptr::null_mut(), 0);
}

/* SEC("fentry/" SYS_PREFIX "sys_prctl") */
#[no_mangle]
pub extern "C" fn test_user_ringbuf_protocol(_ctx: *mut c_void) -> i32 {
    unsafe {
        let mut status: i64 = 0;

        if is_test_process() == 0 {
            return 0;
        }

        status = bpf_user_ringbuf_drain(
            &mut user_ringbuf as *mut bpf_map_def as *mut c_void,
            read_protocol_msg,
            core::ptr::null_mut(),
            0,
        );
        if status < 0 {
            bpf_printk(c"Drain returned: %ld\n".as_ptr() as *const u8, status);
            err = 1;
            return 0;
        }

        publish_kern_messages();

        0
    }
}

/* SEC("fentry/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub extern "C" fn test_user_ringbuf(_ctx: *mut c_void) -> i32 {
    unsafe {
        if is_test_process() == 0 {
            return 0;
        }

        err = bpf_user_ringbuf_drain(
            &mut user_ringbuf as *mut bpf_map_def as *mut c_void,
            record_sample,
            core::ptr::null_mut(),
            0,
        ) as i32;

        0
    }
}

extern "C" fn do_nothing_cb(_dynptr: *mut bpf_dynptr, _context: *mut c_void) -> i64 {
    unsafe {
        core::sync::atomic::AtomicI32::from_ptr(&mut read).fetch_add(
            1,
            core::sync::atomic::Ordering::SeqCst,
        );
        0
    }
}

/* SEC("fentry/" SYS_PREFIX "sys_prlimit64") */
#[no_mangle]
pub extern "C" fn test_user_ringbuf_epoll(_ctx: *mut c_void) -> i32 {
    unsafe {
        let num_samples: i64;

        if is_test_process() == 0 {
            return 0;
        }

        num_samples = bpf_user_ringbuf_drain(
            &mut user_ringbuf as *mut bpf_map_def as *mut c_void,
            do_nothing_cb,
            core::ptr::null_mut(),
            0,
        );
        if num_samples <= 0 {
            err = 1;
        }

        0
    }
}
