// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/map_ptr_mixing.c */

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::arch::asm;

const MAX_ENTRIES: usize = 11;

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

// Original C BPF map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, struct test_val);
// } map_array_48b SEC(".maps");
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_array_48b: core::mem::MaybeUninit<()> = core::mem::MaybeUninit::uninit();

// Original C BPF map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 1);
//     __type(key, long long);
//     __type(value, struct test_val);
// } map_hash_48b SEC(".maps");
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_hash_48b: core::mem::MaybeUninit<()> = core::mem::MaybeUninit::uninit();

// Original C BPF map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, int);
//     __array(values, struct {
//         __uint(type, BPF_MAP_TYPE_ARRAY);
//         __uint(max_entries, 1);
//         __type(key, int);
//         __type(value, int);
//     });
// } map_in_map SEC(".maps");
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_in_map: core::mem::MaybeUninit<()> = core::mem::MaybeUninit::uninit();

unsafe extern "C" {
    fn bpf_tail_call();
    fn bpf_map_lookup_elem();
}

// void dummy_prog_42_socket(void);
// void dummy_prog_24_socket(void);
// void dummy_prog_loop1_socket(void);
// void dummy_prog_loop2_socket(void);

type ProgFn = unsafe extern "C" fn();

#[repr(C)]
pub struct map_prog1_socket_def {
    pub values: [Option<ProgFn>; 4],
}

// Original C BPF map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
//     __uint(max_entries, 4);
//     __uint(key_size, sizeof(int));
//     __array(values, void (void));
// } map_prog1_socket SEC(".maps") = {
//     .values = {
//         [0] = (void *)&dummy_prog_42_socket,
//         [1] = (void *)&dummy_prog_loop1_socket,
//         [2] = (void *)&dummy_prog_24_socket,
//     },
// };
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_prog1_socket: map_prog1_socket_def = map_prog1_socket_def {
    values: [
        Some(dummy_prog_42_socket),
        Some(dummy_prog_loop1_socket),
        Some(dummy_prog_24_socket),
        None,
    ],
};

#[repr(C)]
pub struct map_prog2_socket_def {
    pub values: [Option<ProgFn>; 8],
}

// Original C BPF map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
//     __uint(max_entries, 8);
//     __uint(key_size, sizeof(int));
//     __array(values, void (void));
// } map_prog2_socket SEC(".maps") = {
//     .values = {
//         [1] = (void *)&dummy_prog_loop2_socket,
//         [2] = (void *)&dummy_prog_24_socket,
//         [7] = (void *)&dummy_prog_42_socket,
//     },
// };
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_prog2_socket: map_prog2_socket_def = map_prog2_socket_def {
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

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_42_socket() {
    asm!("r0 = 42; exit;", options(noreturn));
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_24_socket() {
    asm!("r0 = 24; exit;", options(noreturn));
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_loop1_socket() {
    asm!(
        "r3 = 1;
         r2 = {map_prog1_socket} ll;
         call {bpf_tail_call};
         r0 = 41;
         exit;",
        bpf_tail_call = sym bpf_tail_call,
        map_prog1_socket = sym map_prog1_socket,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_loop2_socket() {
    asm!(
        "r3 = 1;
         r2 = {map_prog2_socket} ll;
         call {bpf_tail_call};
         r0 = 41;
         exit;",
        bpf_tail_call = sym bpf_tail_call,
        map_prog2_socket = sym map_prog2_socket,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("calls: two calls returning different map pointers for lookup (hash, array)")
// __success __retval(1)
pub unsafe extern "C" fn pointers_for_lookup_hash_array() {
    asm!(
        "/* main prog */
         if r1 != 0 goto l0_0;
         call pointers_for_lookup_hash_array__1;
         goto l1_0;
     l0_0:
         call pointers_for_lookup_hash_array__2;
     l1_0:
         r1 = r0;
         r2 = 0;
         *(u64*)(r10 - 8) = r2;
         r2 = r10;
         r2 += -8;
         call {bpf_map_lookup_elem};
         if r0 == 0 goto l2_0;
         r1 = {test_val_foo};
         *(u64*)(r0 + 0) = r1;
         r0 = 1;
     l2_0:
         exit;",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        test_val_foo = const core::mem::offset_of!(test_val, foo),
        options(noreturn)
    );
}

#[no_mangle]
unsafe extern "C" fn pointers_for_lookup_hash_array__1() {
    asm!(
        "r0 = {map_hash_48b} ll;
         exit;",
        map_hash_48b = sym map_hash_48b,
        options(noreturn)
    );
}

#[no_mangle]
unsafe extern "C" fn pointers_for_lookup_hash_array__2() {
    asm!(
        "r0 = {map_array_48b} ll;
         exit;",
        map_array_48b = sym map_array_48b,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
// __description("calls: two calls returning different map pointers for lookup (hash, map in map)")
// __failure __msg("only read from bpf_array is supported")
pub unsafe extern "C" fn lookup_hash_map_in_map() {
    asm!(
        "/* main prog */
         if r1 != 0 goto l0_1;
         call lookup_hash_map_in_map__1;
         goto l1_1;
     l0_1:
         call lookup_hash_map_in_map__2;
     l1_1:
         r1 = r0;
         r2 = 0;
         *(u64*)(r10 - 8) = r2;
         r2 = r10;
         r2 += -8;
         call {bpf_map_lookup_elem};
         if r0 == 0 goto l2_1;
         r1 = {test_val_foo};
         *(u64*)(r0 + 0) = r1;
         r0 = 1;
     l2_1:
         exit;",
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        test_val_foo = const core::mem::offset_of!(test_val, foo),
        options(noreturn)
    );
}

#[no_mangle]
unsafe extern "C" fn lookup_hash_map_in_map__1() {
    asm!(
        "r0 = {map_array_48b} ll;
         exit;",
        map_array_48b = sym map_array_48b,
        options(noreturn)
    );
}

#[no_mangle]
unsafe extern "C" fn lookup_hash_map_in_map__2() {
    asm!(
        "r0 = {map_in_map} ll;
         exit;",
        map_in_map = sym map_in_map,
        options(noreturn)
    );
}

// offsetof(struct __sk_buff, mark), supplied by <linux/bpf.h> in the original C.
const __sk_buff_mark: usize = 0;

#[link_section = "socket"]
#[no_mangle]
// __description("cond: two branches returning different map pointers for lookup (tail, tail)")
// __success __failure_unpriv __msg_unpriv("tail_call abusing map_ptr")
// __retval(42)
pub unsafe extern "C" fn pointers_for_lookup_tail_tail_1() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_mark});
         if r6 != 0 goto l0_2;
         r2 = {map_prog2_socket} ll;
         goto l1_2;
     l0_2:
         r2 = {map_prog1_socket} ll;
     l1_2:
         r3 = 7;
         call {bpf_tail_call};
         r0 = 1;
         exit;",
        bpf_tail_call = sym bpf_tail_call,
        map_prog1_socket = sym map_prog1_socket,
        map_prog2_socket = sym map_prog2_socket,
        __sk_buff_mark = const __sk_buff_mark,
        options(noreturn)
    );
}

#[link_section = "socket"]
#[no_mangle]
// __description("cond: two branches returning same map pointers for lookup (tail, tail)")
// __success __success_unpriv __retval(42)
pub unsafe extern "C" fn pointers_for_lookup_tail_tail_2() {
    asm!(
        "r6 = *(u32*)(r1 + {__sk_buff_mark});
         if r6 == 0 goto l0_3;
         r2 = {map_prog2_socket} ll;
         goto l1_3;
     l0_3:
         r2 = {map_prog2_socket} ll;
     l1_3:
         r3 = 7;
         call {bpf_tail_call};
         r0 = 1;
         exit;",
        bpf_tail_call = sym bpf_tail_call,
        map_prog2_socket = sym map_prog2_socket,
        __sk_buff_mark = const __sk_buff_mark,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
