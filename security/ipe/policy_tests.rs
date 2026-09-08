// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

// Translated from includes:
// <linux/slab.h>, <linux/types.h>, <linux/list.h>, <kunit/test.h>, "policy.h"

use core::ffi::{c_char, c_int, c_long, c_void};

const EINVAL: c_int = 22;
const ERANGE: c_int = 34;
const EBADMSG: c_int = 74;
const KUNIT_PARAM_DESC_SIZE: usize = 128;

#[repr(C)]
pub struct kunit {
    pub param_value: *const c_void,
}

#[repr(C)]
pub struct kunit_case {
    pub _unused: [usize; 0],
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub test_cases: *mut kunit_case,
}

#[repr(C)]
pub struct ipe_policy {
    pub parsed: *mut c_void,
    pub text: *const c_char,
    pub pkcs7: *mut c_void,
    pub pkcs7len: usize,
}

#[repr(C)]
struct policy_case {
    policy: *const c_char,
    errno: c_int,
    desc: *const c_char,
}

unsafe extern "C" {
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn strlen(s: *const c_char) -> usize;
    fn ipe_new_policy(
        text: *const c_char,
        textlen: usize,
        pkcs7: *const c_void,
        pkcs7len: usize,
    ) -> *mut ipe_policy;
    fn ipe_free_policy(pol: *mut ipe_policy);

    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool;

    fn KUNIT_EXPECT_EQ(test: *mut kunit, left: c_long, right: c_long);
    fn KUNIT_ASSERT_NOT_ERR_OR_NULL(test: *mut kunit, ptr: *const c_void);
    fn KUNIT_EXPECT_NOT_ERR_OR_NULL(test: *mut kunit, ptr: *const c_void);
    fn KUNIT_EXPECT_STREQ(test: *mut kunit, left: *const c_char, right: *const c_char);
    fn KUNIT_EXPECT_PTR_EQ(test: *mut kunit, left: *const c_void, right: *const c_void);
    fn KUNIT_EXPECT_TRUE(test: *mut kunit, condition: bool);

    fn ipe_policies_gen_params() -> *const c_void;
    fn kunit_test_suite(suite: *mut kunit_suite);
}

const POLICY_CASES: &[policy_case] = &[
    policy_case {
        policy: c"policy_name=allowall policy_version=0.0.0\nDEFAULT action=ALLOW".as_ptr(),
        errno: 0,
        desc: c"basic".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=trailing_comment policy_version=152.0.0 #This is comment\nDEFAULT action=ALLOW"
            .as_ptr(),
        errno: 0,
        desc: c"trailing comment".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=allowallnewline policy_version=0.2.0\nDEFAULT action=ALLOW\n\n"
            .as_ptr(),
        errno: 0,
        desc: c"trailing newline".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=carriagereturnlinefeed policy_version=0.0.1\nDEFAULT action=ALLOW\n\r\n"
            .as_ptr(),
        errno: 0,
        desc: c"clrf newline".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=whitespace policy_version=0.0.0\nDEFAULT\taction=ALLOW\n     \t     DEFAULT \t    op=EXECUTE      action=DENY\nop=EXECUTE boot_verified=TRUE action=ALLOW\n# this is a\tcomment\t\t\t\t\nDEFAULT \t op=KMODULE\t\t\t  action=DENY\r\nop=KMODULE boot_verified=TRUE action=ALLOW\n"
            .as_ptr(),
        errno: 0,
        desc: c"various whitespaces and nested default".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=boot_verified policy_version=-1236.0.0\nDEFAULT\taction=ALLOW\n"
            .as_ptr(),
        errno: -EINVAL,
        desc: c"negative version".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=$@!*&^%%\\:;{}() policy_version=0.0.0\nDEFAULT action=ALLOW"
            .as_ptr(),
        errno: 0,
        desc: c"special characters".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=999999.0.0\nDEFAULT action=ALLOW".as_ptr(),
        errno: -ERANGE,
        desc: c"overflow version".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=255.0\nDEFAULT action=ALLOW".as_ptr(),
        errno: -EBADMSG,
        desc: c"incomplete version".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=111.0.0.0\nDEFAULT action=ALLOW".as_ptr(),
        errno: -EBADMSG,
        desc: c"extra version".as_ptr(),
    },
    policy_case {
        policy: c"".as_ptr(),
        errno: -EBADMSG,
        desc: c"0-length policy".as_ptr(),
    },
    policy_case {
        policy: b"policy_name=test\0policy_version=0.0.0\nDEFAULT action=ALLOW\0".as_ptr()
            as *const c_char,
        errno: -EBADMSG,
        desc: c"random null in header".as_ptr(),
    },
    policy_case {
        policy: b"policy_name=test policy_version=0.0.0\n\0DEFAULT action=ALLOW\0".as_ptr()
            as *const c_char,
        errno: -EBADMSG,
        desc: c"incomplete policy from NULL".as_ptr(),
    },
    policy_case {
        policy: b"policy_name=test policy_version=0.0.0\nDEFAULT action=DENY\n\0op=EXECUTE dmverity_signature=TRUE action=ALLOW\n\0"
            .as_ptr() as *const c_char,
        errno: 0,
        desc: c"NULL truncates policy".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\nop=EXECUTE dmverity_signature=abc action=ALLOW"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"invalid property type".as_ptr(),
    },
    policy_case {
        policy: c"DEFAULT action=ALLOW".as_ptr(),
        errno: -EBADMSG,
        desc: c"missing policy header".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\n".as_ptr(),
        errno: -EBADMSG,
        desc: c"missing default definition".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\ndmverity_signature=TRUE op=EXECUTE action=ALLOW"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"invalid rule ordering".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\naction=ALLOW op=EXECUTE dmverity_signature=TRUE"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"invalid rule ordering (2)".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0\nDEFAULT action=ALLOW\nop=EXECUTE dmverity_signature=TRUE action=ALLOW"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"invalid version".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\nop=UNKNOWN dmverity_signature=TRUE action=ALLOW"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"unknown operation".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=asdvpolicy_version=0.0.0\nDEFAULT action=ALLOW\n".as_ptr(),
        errno: -EBADMSG,
        desc: c"missing space after policy name".as_ptr(),
    },
    policy_case {
        policy: b"policy_name=test\xFF\xEF policy_version=0.0.0\nDEFAULT action=ALLOW\nop=EXECUTE dmverity_signature=TRUE action=ALLOW\0"
            .as_ptr() as *const c_char,
        errno: 0,
        desc: c"expanded ascii".as_ptr(),
    },
    policy_case {
        policy: b"policy_name=test\xFF\xEF policy_version=0.0.0\nDEFAULT action=ALLOW\nop=EXECUTE dmverity_roothash=GOOD_DOG action=ALLOW\0"
            .as_ptr() as *const c_char,
        errno: -EBADMSG,
        desc: c"invalid property value (2)".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\npolicy_name=test policy_version=0.1.0\nDEFAULT action=ALLOW"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"double header".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\nDEFAULT action=ALLOW\n"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"double default".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\nDEFAULT op=EXECUTE action=DENY\nDEFAULT op=EXECUTE action=ALLOW\n"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"double operation default".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\nDEFAULT op=EXECUTE action=DEN\n"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"invalid action value".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\nDEFAULT op=EXECUTE action\n"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"invalid action value (2)".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\nUNKNOWN value=true\n"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"unrecognized statement".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\nop=EXECUTE dmverity_roothash=1c0d7ee1f8343b7fbe418378e8eb22c061d7dec7 action=DENY\n"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"old-style digest".as_ptr(),
    },
    policy_case {
        policy: c"policy_name=test policy_version=0.0.0\nDEFAULT action=ALLOW\nop=EXECUTE fsverity_digest=1c0d7ee1f8343b7fbe418378e8eb22c061d7dec7 action=DENY\n"
            .as_ptr(),
        errno: -EBADMSG,
        desc: c"old-style digest".as_ptr(),
    },
];

unsafe fn pol_to_desc(c: *const policy_case, desc: *mut c_char) {
    unsafe {
        strscpy(desc, (*c).desc, KUNIT_PARAM_DESC_SIZE);
    }
}

// C macro translation:
// KUNIT_ARRAY_PARAM(ipe_policies, policy_cases, pol_to_desc);
// The generated parameter provider is supplied by the KUnit integration.

/**
 * ipe_parser_unsigned_test - Test the parser by passing unsigned policies.
 * @test: Supplies a pointer to a kunit structure.
 *
 * This is called by the kunit harness. This test does not check the correctness
 * of the policy, but ensures that errors are handled correctly.
 */
unsafe fn ipe_parser_unsigned_test(test: *mut kunit) {
    unsafe {
        let p = (*test).param_value as *const policy_case;
        let pol: *mut ipe_policy;

        pol = ipe_new_policy((*p).policy, strlen((*p).policy), core::ptr::null(), 0);

        if (*p).errno != 0 {
            KUNIT_EXPECT_EQ(test, PTR_ERR(pol as *const c_void), (*p).errno as c_long);
            return;
        }

        KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pol as *const c_void);
        KUNIT_EXPECT_NOT_ERR_OR_NULL(test, (*pol).parsed as *const c_void);
        KUNIT_EXPECT_STREQ(test, (*pol).text, (*p).policy);
        KUNIT_EXPECT_PTR_EQ(test, core::ptr::null(), (*pol).pkcs7 as *const c_void);
        KUNIT_EXPECT_EQ(test, 0, (*pol).pkcs7len as c_long);

        ipe_free_policy(pol);
    }
}

/**
 * ipe_parser_widestring_test - Ensure parser fail on a wide string policy.
 * @test: Supplies a pointer to a kunit structure.
 *
 * This is called by the kunit harness.
 */
unsafe fn ipe_parser_widestring_test(test: *mut kunit) {
    const POLICY: &[u16] = &[
        'p' as u16, 'o' as u16, 'l' as u16, 'i' as u16, 'c' as u16, 'y' as u16, '_' as u16,
        'n' as u16, 'a' as u16, 'm' as u16, 'e' as u16, '=' as u16, 'T' as u16, 'e' as u16,
        's' as u16, 't' as u16, ' ' as u16, 'p' as u16, 'o' as u16, 'l' as u16, 'i' as u16,
        'c' as u16, 'y' as u16, '_' as u16, 'v' as u16, 'e' as u16, 'r' as u16, 's' as u16,
        'i' as u16, 'o' as u16, 'n' as u16, '=' as u16, '0' as u16, '.' as u16, '0' as u16,
        '.' as u16, '0' as u16, '\n' as u16, 'D' as u16, 'E' as u16, 'F' as u16, 'A' as u16,
        'U' as u16, 'L' as u16, 'T' as u16, ' ' as u16, 'a' as u16, 'c' as u16, 't' as u16,
        'i' as u16, 'o' as u16, 'n' as u16, '=' as u16, 'A' as u16, 'L' as u16, 'L' as u16,
        'O' as u16, 'W' as u16, 0,
    ];
    let mut pol: *mut ipe_policy = core::ptr::null_mut();

    unsafe {
        pol = ipe_new_policy(
            POLICY.as_ptr() as *const c_char,
            (POLICY.len() - 1) * 2,
            core::ptr::null(),
            0,
        );
        KUNIT_EXPECT_TRUE(test, IS_ERR_OR_NULL(pol as *const c_void));

        ipe_free_policy(pol);
    }
}

static mut IPE_PARSER_TEST_CASES: [kunit_case; 3] = [
    // KUNIT_CASE_PARAM(ipe_parser_unsigned_test, ipe_policies_gen_params),
    kunit_case { _unused: [] },
    // KUNIT_CASE(ipe_parser_widestring_test),
    kunit_case { _unused: [] },
    kunit_case { _unused: [] },
];

static mut IPE_PARSER_TEST_SUITE: kunit_suite = kunit_suite {
    name: c"ipe-parser".as_ptr(),
    test_cases: core::ptr::addr_of_mut!(IPE_PARSER_TEST_CASES) as *mut kunit_case,
};

unsafe fn register_ipe_parser_test_suite() {
    unsafe {
        kunit_test_suite(core::ptr::addr_of_mut!(IPE_PARSER_TEST_SUITE));
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
