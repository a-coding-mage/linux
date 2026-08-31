// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/runtime_jit.c */

use core::arch::asm;
use core::ffi::c_void;

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

unsafe extern "C" {
    fn bpf_tail_call(ctx: *mut c_void, prog_array_map: *mut c_void, index: u32) -> i64;
}

const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct MapProgSocket {
    // __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    pub type_: u32,
    // __uint(max_entries, ...);
    pub max_entries: u32,
    // __uint(key_size, sizeof(int));
    pub key_size: u32,
    // __array(values, void (void));
    pub values: [Option<unsafe extern "C" fn()>; 8],
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_prog1_socket: MapProgSocket = MapProgSocket {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 4,
    key_size: core::mem::size_of::<i32>() as u32,
    values: [
        Some(dummy_prog_42_socket),
        Some(dummy_prog_loop1_socket),
        Some(dummy_prog_24_socket),
        None,
        None,
        None,
        None,
        None,
    ],
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_prog2_socket: MapProgSocket = MapProgSocket {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 8,
    key_size: core::mem::size_of::<i32>() as u32,
    values: [
        None,
        Some(dummy_prog_loop2_socket),
        Some(dummy_prog_24_socket),
        None,
        None,
        None,
        None,
        Some(dummy_prog_42_socket),
    ],
};

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __auxiliary __auxiliary_unpriv
pub unsafe extern "C" fn dummy_prog_42_socket() {
    unsafe {
        asm!("r0 = 42; exit;", options(noreturn));
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __auxiliary __auxiliary_unpriv
pub unsafe extern "C" fn dummy_prog_24_socket() {
    unsafe {
        asm!("r0 = 24; exit;", options(noreturn));
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __auxiliary __auxiliary_unpriv
pub unsafe extern "C" fn dummy_prog_loop1_socket() {
    unsafe {
        asm!(
            "r3 = 1;",
            "r2 = {map_prog1_socket} ll;",
            "call {bpf_tail_call};",
            "r0 = 41;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __auxiliary __auxiliary_unpriv
pub unsafe extern "C" fn dummy_prog_loop2_socket() {
    unsafe {
        asm!(
            "r3 = 1;",
            "r2 = {map_prog2_socket} ll;",
            "call {bpf_tail_call};",
            "r0 = 41;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog2_socket = sym map_prog2_socket,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call within bounds, prog once")
// __success __success_unpriv __retval(42)
pub unsafe extern "C" fn call_within_bounds_prog_once() {
    unsafe {
        asm!(
            "r3 = 0;",
            "r2 = {map_prog1_socket} ll;",
            "call {bpf_tail_call};",
            "r0 = 1;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call within bounds, prog loop")
// __success __success_unpriv __retval(41)
pub unsafe extern "C" fn call_within_bounds_prog_loop() {
    unsafe {
        asm!(
            "r3 = 1;",
            "r2 = {map_prog1_socket} ll;",
            "call {bpf_tail_call};",
            "r0 = 1;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call within bounds, no prog")
// __success __success_unpriv __retval(1)
pub unsafe extern "C" fn call_within_bounds_no_prog() {
    unsafe {
        asm!(
            "r3 = 3;",
            "r2 = {map_prog1_socket} ll;",
            "call {bpf_tail_call};",
            "r0 = 1;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call within bounds, key 2")
// __success __success_unpriv __retval(24)
pub unsafe extern "C" fn call_within_bounds_key_2() {
    unsafe {
        asm!(
            "r3 = 2;",
            "r2 = {map_prog1_socket} ll;",
            "call {bpf_tail_call};",
            "r0 = 1;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call within bounds, key 2 / key 2, first branch")
// __success __success_unpriv __retval(24)
pub unsafe extern "C" fn _2_key_2_first_branch() {
    unsafe {
        asm!(
            "r0 = 13;",
            "*(u8*)(r1 + {__sk_buff_cb_0}) = r0;",
            "r0 = *(u8*)(r1 + {__sk_buff_cb_0});",
            "if r0 == 13 goto 0f;",
            "r3 = 2;",
            "r2 = {map_prog1_socket} ll;",
            "goto 1f;",
            "0:",
            "r3 = 2;",
            "r2 = {map_prog1_socket} ll;",
            "1:",
            "call {bpf_tail_call};",
            "r0 = 1;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            __sk_buff_cb_0 = const 48,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call within bounds, key 2 / key 2, second branch")
// __success __success_unpriv __retval(24)
pub unsafe extern "C" fn _2_key_2_second_branch() {
    unsafe {
        asm!(
            "r0 = 14;",
            "*(u8*)(r1 + {__sk_buff_cb_0}) = r0;",
            "r0 = *(u8*)(r1 + {__sk_buff_cb_0});",
            "if r0 == 13 goto 0f;",
            "r3 = 2;",
            "r2 = {map_prog1_socket} ll;",
            "goto 1f;",
            "0:",
            "r3 = 2;",
            "r2 = {map_prog1_socket} ll;",
            "1:",
            "call {bpf_tail_call};",
            "r0 = 1;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            __sk_buff_cb_0 = const 48,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call within bounds, key 0 / key 2, first branch")
// __success __success_unpriv __retval(24)
pub unsafe extern "C" fn _0_key_2_first_branch() {
    unsafe {
        asm!(
            "r0 = 13;",
            "*(u8*)(r1 + {__sk_buff_cb_0}) = r0;",
            "r0 = *(u8*)(r1 + {__sk_buff_cb_0});",
            "if r0 == 13 goto 0f;",
            "r3 = 0;",
            "r2 = {map_prog1_socket} ll;",
            "goto 1f;",
            "0:",
            "r3 = 2;",
            "r2 = {map_prog1_socket} ll;",
            "1:",
            "call {bpf_tail_call};",
            "r0 = 1;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            __sk_buff_cb_0 = const 48,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call within bounds, key 0 / key 2, second branch")
// __success __success_unpriv __retval(42)
pub unsafe extern "C" fn _0_key_2_second_branch() {
    unsafe {
        asm!(
            "r0 = 14;",
            "*(u8*)(r1 + {__sk_buff_cb_0}) = r0;",
            "r0 = *(u8*)(r1 + {__sk_buff_cb_0});",
            "if r0 == 13 goto 0f;",
            "r3 = 0;",
            "r2 = {map_prog1_socket} ll;",
            "goto 1f;",
            "0:",
            "r3 = 2;",
            "r2 = {map_prog1_socket} ll;",
            "1:",
            "call {bpf_tail_call};",
            "r0 = 1;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            __sk_buff_cb_0 = const 48,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call within bounds, different maps, first branch")
// __success __failure_unpriv __msg_unpriv("tail_call abusing map_ptr")
// __retval(1)
pub unsafe extern "C" fn bounds_different_maps_first_branch() {
    unsafe {
        asm!(
            "r0 = 13;",
            "*(u8*)(r1 + {__sk_buff_cb_0}) = r0;",
            "r0 = *(u8*)(r1 + {__sk_buff_cb_0});",
            "if r0 == 13 goto 0f;",
            "r3 = 0;",
            "r2 = {map_prog1_socket} ll;",
            "goto 1f;",
            "0:",
            "r3 = 0;",
            "r2 = {map_prog2_socket} ll;",
            "1:",
            "call {bpf_tail_call};",
            "r0 = 1;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            map_prog2_socket = sym map_prog2_socket,
            __sk_buff_cb_0 = const 48,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call within bounds, different maps, second branch")
// __success __failure_unpriv __msg_unpriv("tail_call abusing map_ptr")
// __retval(42)
pub unsafe extern "C" fn bounds_different_maps_second_branch() {
    unsafe {
        asm!(
            "r0 = 14;",
            "*(u8*)(r1 + {__sk_buff_cb_0}) = r0;",
            "r0 = *(u8*)(r1 + {__sk_buff_cb_0});",
            "if r0 == 13 goto 0f;",
            "r3 = 0;",
            "r2 = {map_prog1_socket} ll;",
            "goto 1f;",
            "0:",
            "r3 = 0;",
            "r2 = {map_prog2_socket} ll;",
            "1:",
            "call {bpf_tail_call};",
            "r0 = 1;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            map_prog2_socket = sym map_prog2_socket,
            __sk_buff_cb_0 = const 48,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: tail_call out of bounds")
// __success __success_unpriv __retval(2)
pub unsafe extern "C" fn tail_call_out_of_bounds() {
    unsafe {
        asm!(
            "r3 = 256;",
            "r2 = {map_prog1_socket} ll;",
            "call {bpf_tail_call};",
            "r0 = 2;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: pass negative index to tail_call")
// __success __success_unpriv __retval(2)
pub unsafe extern "C" fn negative_index_to_tail_call() {
    unsafe {
        asm!(
            "r3 = -1;",
            "r2 = {map_prog1_socket} ll;",
            "call {bpf_tail_call};",
            "r0 = 2;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("runtime/jit: pass > 32bit index to tail_call")
// __success __success_unpriv __retval(42)
/* Verifier rewrite for unpriv skips tail call here. */
// __retval_unpriv(2)
pub unsafe extern "C" fn _32bit_index_to_tail_call() {
    unsafe {
        asm!(
            "r3 = 0x100000000 ll;",
            "r2 = {map_prog1_socket} ll;",
            "call {bpf_tail_call};",
            "r0 = 2;",
            "exit;",
            bpf_tail_call = sym bpf_tail_call,
            map_prog1_socket = sym map_prog1_socket,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
