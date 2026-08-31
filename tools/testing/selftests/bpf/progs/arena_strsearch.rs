// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C source:
// #include <vmlinux.h>
// #include "bpf_experimental.h"
// #include "bpf_arena_strsearch.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type bool_ = bool;

const BPF_MAP_TYPE_ARENA: u32 = 33;
const BPF_F_MMAPABLE: u32 = 1024;

#[repr(C)]
pub struct arena_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static arena: arena_map_def = arena_map_def {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: 100, /* number of pages */
};

#[repr(C)]
pub struct glob_test {
    pub pat: *const i8,
    pub str_: *const i8,
    pub expected: bool_,
}

extern "C" {
    fn glob_match(pat: *const i8, str_: *const i8) -> bool_;
    fn bpf_arena_strlen(s: *const i8) -> usize;
}

unsafe fn test(pat: *const i8, str_: *const i8, expected: bool_) -> bool_ {
    let match_ = glob_match(pat, str_);
    let success = match_ == expected;

    /* bpf_printk("glob_match %s %s res %d ok %d", pat, str, match, success); */
    success
}

/*
 * The tests are all jammed together in one array to make it simpler
 * to place that array in the .init.rodata section.  The obvious
 * "array of structures containing char *" has no way to force the
 * pointed-to strings to be in a particular section.
 *
 * Anyway, a test consists of:
 * 1. Expected glob_match result: '1' or '0'.
 * 2. Pattern to match: null-terminated string
 * 3. String to match against: null-terminated string
 *
 * The list of tests is terminated with a final '\0' instead of
 * a glob_match result character.
 */
static glob_tests: &[u8] = concat!(
    /* Some basic tests */
    "1", "a\0", "a\0",
    "0", "a\0", "b\0",
    "0", "a\0", "aa\0",
    "0", "a\0", "\0",
    "1", "\0", "\0",
    "0", "\0", "a\0",
    /* Simple character class tests */
    "1", "[a]\0", "a\0",
    "0", "[a]\0", "b\0",
    "0", "[!a]\0", "a\0",
    "1", "[!a]\0", "b\0",
    "1", "[ab]\0", "a\0",
    "1", "[ab]\0", "b\0",
    "0", "[ab]\0", "c\0",
    "1", "[!ab]\0", "c\0",
    "1", "[a-c]\0", "b\0",
    "0", "[a-c]\0", "d\0",
    /* Corner cases in character class parsing */
    "1", "[a-c-e-g]\0", "-\0",
    "0", "[a-c-e-g]\0", "d\0",
    "1", "[a-c-e-g]\0", "f\0",
    "1", "[]a-ceg-ik[]\0", "a\0",
    "1", "[]a-ceg-ik[]\0", "]\0",
    "1", "[]a-ceg-ik[]\0", "[\0",
    "1", "[]a-ceg-ik[]\0", "h\0",
    "0", "[]a-ceg-ik[]\0", "f\0",
    "0", "[!]a-ceg-ik[]\0", "h\0",
    "0", "[!]a-ceg-ik[]\0", "]\0",
    "1", "[!]a-ceg-ik[]\0", "f\0",
    /* Simple wild cards */
    "1", "?\0", "a\0",
    "0", "?\0", "aa\0",
    "0", "??\0", "a\0",
    "1", "?x?\0", "axb\0",
    "0", "?x?\0", "abx\0",
    "0", "?x?\0", "xab\0",
    /* Asterisk wild cards (backtracking) */
    "0", "*??\0", "a\0",
    "1", "*??\0", "ab\0",
    "1", "*??\0", "abc\0",
    "1", "*??\0", "abcd\0",
    "0", "??*\0", "a\0",
    "1", "??*\0", "ab\0",
    "1", "??*\0", "abc\0",
    "1", "??*\0", "abcd\0",
    "0", "?*?\0", "a\0",
    "1", "?*?\0", "ab\0",
    "1", "?*?\0", "abc\0",
    "1", "?*?\0", "abcd\0",
    "1", "*b\0", "b\0",
    "1", "*b\0", "ab\0",
    "0", "*b\0", "ba\0",
    "1", "*b\0", "bb\0",
    "1", "*b\0", "abb\0",
    "1", "*b\0", "bab\0",
    "1", "*bc\0", "abbc\0",
    "1", "*bc\0", "bc\0",
    "1", "*bc\0", "bbc\0",
    "1", "*bc\0", "bcbc\0",
    /* Multiple asterisks (complex backtracking) */
    "1", "*ac*\0", "abacadaeafag\0",
    "1", "*ac*ae*ag*\0", "abacadaeafag\0",
    "1", "*a*b*[bc]*[ef]*g*\0", "abacadaeafag\0",
    "0", "*a*b*[ef]*[cd]*g*\0", "abacadaeafag\0",
    "1", "*abcd*\0", "abcabcabcabcdefg\0",
    "1", "*ab*cd*\0", "abcabcabcabcdefg\0",
    "1", "*abcd*abcdef*\0", "abcabcdabcdeabcdefg\0",
    "0", "*abcd*\0", "abcabcabcabcefg\0",
    "0", "*ab*cd*\0", "abcabcabcabcefg\0",
    "\0",
).as_bytes();

#[no_mangle]
pub static mut skip: bool_ = false;

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn arena_strsearch(ctx: *mut core::ffi::c_void) -> i32 {
    let mut successes: u32 = 0;
    let mut n: u32 = 0;
    let mut p: *const i8 = glob_tests.as_ptr() as *const i8;

    /*
     * Tests are jammed together in a string.  The first byte is '1'
     * or '0' to indicate the expected outcome, or '\0' to indicate the
     * end of the tests.  Then come two null-terminated strings: the
     * pattern and the string to match it against.
     */
    while *p != 0 {
        let expected: bool_ = (*p & 1) != 0;
        p = p.add(1);
        let pat: *const i8 = p;

        // cond_break;
        p = p.add(bpf_arena_strlen(p) + 1);
        successes = successes.wrapping_add(test(pat, p, expected) as u32);
        p = p.add(bpf_arena_strlen(p) + 1);
        n = n.wrapping_add(1);
    }

    n = n.wrapping_sub(successes);
    /* bpf_printk("glob: %u self-tests passed, %u failed\n", successes, n); */

    if n != 0 { -1 } else { 0 }
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
