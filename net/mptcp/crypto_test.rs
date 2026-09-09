// SPDX-License-Identifier: GPL-2.0
// Dependency intent from <kunit/test.h> and "protocol.h" is preserved below.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct test_case {
    pub key: *mut c_char,
    pub msg: *mut c_char,
    pub result: *mut c_char,
}

/* we can't reuse RFC 4231 test vectors, as we have constraint on the
 * input and key size.
 */
static mut TEST_KEY_0: [u8; 17] = *b"0b0b0b0b0b0b0b0b\0";
static mut TEST_MSG_0: [u8; 9] = *b"48692054\0";
static mut TEST_RESULT_0: [u8; 65] = *b"8385e24fb4235ac37556b6b886db106284a1da671699f46db1f235ec622dcafa\0";
static mut TEST_KEY_1: [u8; 17] = *b"aaaaaaaaaaaaaaaa\0";
static mut TEST_MSG_1: [u8; 9] = *b"dddddddd\0";
static mut TEST_RESULT_1: [u8; 65] = *b"2c5e219164ff1dca1c4a92318d847bb6b9d44492984e1eb71aff9022f71046e9\0";
static mut TEST_KEY_2: [u8; 17] = *b"0102030405060708\0";
static mut TEST_MSG_2: [u8; 9] = *b"cdcdcdcd\0";
static mut TEST_RESULT_2: [u8; 65] = *b"e73b9ba9969969cefb04aa0d6df18ec2fcc075b6f23b4d8c4da736a5dbbc6e7d\0";

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

extern "C" {
    fn mptcp_crypto_hmac_sha(key1: u64, key2: u64, msg: *const u8, len: usize, hmac: *mut c_char);
    fn kunit_expect_streq(test: *mut kunit, actual: *const c_char, expected: *const c_char);
}

unsafe fn mptcp_crypto_test_basic(test: *mut kunit) {
    let mut hmac = [0 as c_char; 32];
    let mut hmac_hex = [0 as c_char; 65];
    let mut msg = [0u8; 8];
    let tests: [test_case; 3] = [
        test_case { key: TEST_KEY_0.as_mut_ptr() as *mut c_char, msg: TEST_MSG_0.as_mut_ptr() as *mut c_char, result: TEST_RESULT_0.as_mut_ptr() as *mut c_char },
        test_case { key: TEST_KEY_1.as_mut_ptr() as *mut c_char, msg: TEST_MSG_1.as_mut_ptr() as *mut c_char, result: TEST_RESULT_1.as_mut_ptr() as *mut c_char },
        test_case { key: TEST_KEY_2.as_mut_ptr() as *mut c_char, msg: TEST_MSG_2.as_mut_ptr() as *mut c_char, result: TEST_RESULT_2.as_mut_ptr() as *mut c_char },
    ];

    for i in 0..tests.len() {
        /* mptcp hmap will convert to be before computing the hmac */
        let key1 = u64::from_be_bytes(*(tests[i].key as *const [u8; 8]));
        let key2 = u64::from_be_bytes(*(tests[i].key.add(8) as *const [u8; 8]));
        let nonce1 = u32::from_be_bytes(*(tests[i].msg as *const [u8; 4]));
        let nonce2 = u32::from_be_bytes(*(tests[i].msg.add(4) as *const [u8; 4]));

        msg[0..4].copy_from_slice(&nonce1.to_be_bytes());
        msg[4..8].copy_from_slice(&nonce2.to_be_bytes());

        mptcp_crypto_hmac_sha(key1, key2, msg.as_ptr(), 8, hmac.as_mut_ptr());
        for j in 0..32 {
            let value = (hmac[j] as u8) & 0xff;
            let hex = b"0123456789abcdef";
            hmac_hex[j << 1] = hex[(value >> 4) as usize] as c_char;
            hmac_hex[(j << 1) + 1] = hex[(value & 0xf) as usize] as c_char;
        }
        hmac_hex[64] = 0;

        kunit_expect_streq(test, hmac_hex.as_ptr(), tests[i].result);
    }
}

#[repr(C)]
pub struct kunit_case {
    pub run_case: Option<unsafe extern "C" fn(*mut kunit)>,
}

static mut MPTCP_CRYPTO_TEST_CASES: [kunit_case; 2] = [
    kunit_case { run_case: Some(mptcp_crypto_test_basic) },
    kunit_case { run_case: None },
];

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub test_cases: *mut kunit_case,
}

static mut MPTCP_CRYPTO_SUITE: kunit_suite = kunit_suite {
    name: b"mptcp-crypto\0".as_ptr() as *const c_char,
    test_cases: unsafe { MPTCP_CRYPTO_TEST_CASES.as_mut_ptr() },
};

// Equivalent registration intent for kunit_test_suite(mptcp_crypto_suite).
#[no_mangle]
pub static mut mptcp_crypto_suite: *mut kunit_suite = unsafe { &mut MPTCP_CRYPTO_SUITE };

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("KUnit tests for MPTCP Crypto");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
