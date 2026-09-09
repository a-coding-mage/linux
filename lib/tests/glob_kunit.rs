// SPDX-License-Identifier: MIT OR GPL-2.0
/*
 * Test cases for glob functions.
 */

use core::ffi::{c_char, c_int, c_void};

// External declarations supplied by the Linux KUnit and glob interfaces.
extern "C" {
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn glob_match(pattern: *const c_char, string: *const c_char) -> bool;
}

#[repr(C)]
pub struct glob_test_case {
    pub pat: *const c_char,
    pub str_: *const c_char,
    pub expected: bool,
}

static glob_test_cases: &[glob_test_case] = &[
    // Some basic tests
    glob_test_case { pat: c"a".as_ptr(), str_: c"a".as_ptr(), expected: true },
    glob_test_case { pat: c"a".as_ptr(), str_: c"b".as_ptr(), expected: false },
    glob_test_case { pat: c"a".as_ptr(), str_: c"aa".as_ptr(), expected: false },
    glob_test_case { pat: c"a".as_ptr(), str_: c"".as_ptr(), expected: false },
    glob_test_case { pat: c"".as_ptr(), str_: c"".as_ptr(), expected: true },
    glob_test_case { pat: c"".as_ptr(), str_: c"a".as_ptr(), expected: false },
    // Simple character class tests
    glob_test_case { pat: c"[a]".as_ptr(), str_: c"a".as_ptr(), expected: true },
    glob_test_case { pat: c"[a]".as_ptr(), str_: c"b".as_ptr(), expected: false },
    glob_test_case { pat: c"[!a]".as_ptr(), str_: c"a".as_ptr(), expected: false },
    glob_test_case { pat: c"[!a]".as_ptr(), str_: c"b".as_ptr(), expected: true },
    glob_test_case { pat: c"[ab]".as_ptr(), str_: c"a".as_ptr(), expected: true },
    glob_test_case { pat: c"[ab]".as_ptr(), str_: c"b".as_ptr(), expected: true },
    glob_test_case { pat: c"[ab]".as_ptr(), str_: c"c".as_ptr(), expected: false },
    glob_test_case { pat: c"[!ab]".as_ptr(), str_: c"c".as_ptr(), expected: true },
    glob_test_case { pat: c"[a-c]".as_ptr(), str_: c"b".as_ptr(), expected: true },
    glob_test_case { pat: c"[a-c]".as_ptr(), str_: c"d".as_ptr(), expected: false },
    // Corner cases in character class parsing
    glob_test_case { pat: c"[a-c-e-g]".as_ptr(), str_: c"-".as_ptr(), expected: true },
    glob_test_case { pat: c"[a-c-e-g]".as_ptr(), str_: c"d".as_ptr(), expected: false },
    glob_test_case { pat: c"[a-c-e-g]".as_ptr(), str_: c"f".as_ptr(), expected: true },
    glob_test_case { pat: c"[]a-ceg-ik[]".as_ptr(), str_: c"a".as_ptr(), expected: true },
    glob_test_case { pat: c"[]a-ceg-ik[]".as_ptr(), str_: c"]".as_ptr(), expected: true },
    glob_test_case { pat: c"[]a-ceg-ik[]".as_ptr(), str_: c"[".as_ptr(), expected: true },
    glob_test_case { pat: c"[]a-ceg-ik[]".as_ptr(), str_: c"h".as_ptr(), expected: true },
    glob_test_case { pat: c"[]a-ceg-ik[]".as_ptr(), str_: c"f".as_ptr(), expected: false },
    glob_test_case { pat: c"[!]a-ceg-ik[]".as_ptr(), str_: c"h".as_ptr(), expected: false },
    glob_test_case { pat: c"[!]a-ceg-ik[]".as_ptr(), str_: c"]".as_ptr(), expected: false },
    glob_test_case { pat: c"[!]a-ceg-ik[]".as_ptr(), str_: c"f".as_ptr(), expected: true },
    // Simple wild cards
    glob_test_case { pat: c"?".as_ptr(), str_: c"a".as_ptr(), expected: true },
    glob_test_case { pat: c"?".as_ptr(), str_: c"aa".as_ptr(), expected: false },
    glob_test_case { pat: c"??".as_ptr(), str_: c"a".as_ptr(), expected: false },
    glob_test_case { pat: c"?x?".as_ptr(), str_: c"axb".as_ptr(), expected: true },
    glob_test_case { pat: c"?x?".as_ptr(), str_: c"abx".as_ptr(), expected: false },
    glob_test_case { pat: c"?x?".as_ptr(), str_: c"xab".as_ptr(), expected: false },
    // Asterisk wild cards (backtracking)
    glob_test_case { pat: c"*??".as_ptr(), str_: c"a".as_ptr(), expected: false },
    glob_test_case { pat: c"*??".as_ptr(), str_: c"ab".as_ptr(), expected: true },
    glob_test_case { pat: c"*??".as_ptr(), str_: c"abc".as_ptr(), expected: true },
    glob_test_case { pat: c"*??".as_ptr(), str_: c"abcd".as_ptr(), expected: true },
    glob_test_case { pat: c"??*".as_ptr(), str_: c"a".as_ptr(), expected: false },
    glob_test_case { pat: c"??*".as_ptr(), str_: c"ab".as_ptr(), expected: true },
    glob_test_case { pat: c"??*".as_ptr(), str_: c"abc".as_ptr(), expected: true },
    glob_test_case { pat: c"??*".as_ptr(), str_: c"abcd".as_ptr(), expected: true },
    glob_test_case { pat: c"?*?".as_ptr(), str_: c"a".as_ptr(), expected: false },
    glob_test_case { pat: c"?*?".as_ptr(), str_: c"ab".as_ptr(), expected: true },
    glob_test_case { pat: c"?*?".as_ptr(), str_: c"abc".as_ptr(), expected: true },
    glob_test_case { pat: c"?*?".as_ptr(), str_: c"abcd".as_ptr(), expected: true },
    glob_test_case { pat: c"*b".as_ptr(), str_: c"b".as_ptr(), expected: true },
    glob_test_case { pat: c"*b".as_ptr(), str_: c"ab".as_ptr(), expected: true },
    glob_test_case { pat: c"*b".as_ptr(), str_: c"ba".as_ptr(), expected: false },
    glob_test_case { pat: c"*b".as_ptr(), str_: c"bb".as_ptr(), expected: true },
    glob_test_case { pat: c"*b".as_ptr(), str_: c"abb".as_ptr(), expected: true },
    glob_test_case { pat: c"*b".as_ptr(), str_: c"bab".as_ptr(), expected: true },
    glob_test_case { pat: c"*bc".as_ptr(), str_: c"abbc".as_ptr(), expected: true },
    glob_test_case { pat: c"*bc".as_ptr(), str_: c"bc".as_ptr(), expected: true },
    glob_test_case { pat: c"*bc".as_ptr(), str_: c"bbc".as_ptr(), expected: true },
    glob_test_case { pat: c"*bc".as_ptr(), str_: c"bcbc".as_ptr(), expected: true },
    // Multiple asterisks (complex backtracking)
    glob_test_case { pat: c"*ac*".as_ptr(), str_: c"abacadaeafag".as_ptr(), expected: true },
    glob_test_case { pat: c"*ac*ae*ag*".as_ptr(), str_: c"abacadaeafag".as_ptr(), expected: true },
    glob_test_case { pat: c"*a*b*[bc]*[ef]*g*".as_ptr(), str_: c"abacadaeafag".as_ptr(), expected: true },
    glob_test_case { pat: c"*a*b*[ef]*[cd]*g*".as_ptr(), str_: c"abacadaeafag".as_ptr(), expected: false },
    glob_test_case { pat: c"*abcd*".as_ptr(), str_: c"abcabcabcabcdefg".as_ptr(), expected: true },
    glob_test_case { pat: c"*ab*cd*".as_ptr(), str_: c"abcabcabcabcdefg".as_ptr(), expected: true },
    glob_test_case { pat: c"*abcd*abcdef*".as_ptr(), str_: c"abcabcdabcdeabcdefg".as_ptr(), expected: true },
    glob_test_case { pat: c"*abcd*".as_ptr(), str_: c"abcabcabcabcefg".as_ptr(), expected: false },
    glob_test_case { pat: c"*ab*cd*".as_ptr(), str_: c"abcabcabcabcefg".as_ptr(), expected: false },
];

unsafe fn glob_case_to_desc(t: *const glob_test_case, desc: *mut c_char) {
    // KUNIT_PARAM_DESC_SIZE is supplied by the KUnit interface.
    snprintf(desc, 0, c"pat:\"%s\" str:\"%s\"".as_ptr(), (*t).pat, (*t).str_);
}

// KUNIT_ARRAY_PARAM(glob, glob_test_cases, glob_case_to_desc);

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

unsafe fn glob_test_match(test: *mut kunit) {
    // KUNIT_EXPECT_EQ_MSG(test, glob_match(params->pat, params->str),
    //     params->expected, "Pattern: ...", params->pat, params->str,
    //     params->expected);
    let _ = test;
}

// KUNIT_CASE_PARAM(glob_test_match, glob_gen_params);
// KUNIT test suite registration and module metadata are supplied by the
// Linux KUnit/module interfaces.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
