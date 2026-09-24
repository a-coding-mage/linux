// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021  Maciej W. Rozycki
 */

//! Original 64-bit/32-bit division vectors through the safe kernel math API.

use core::hint::black_box;
use kernel::math::do_div;
use kernel::prelude::*;
use kernel::time::{Instant, Monotonic};

module! {
    type: DivisionTest,
    name: "test_div64",
    authors: ["Maciej W. Rozycki <macro@orcam.me.uk>"],
    description: "64bit/32bit division and modulo test module",
    license: "GPL",
}

const TEST_DIV64_N_ITER: usize = 1024;

static TEST_DIV64_DIVIDENDS: [u64; 12] = [
    0x00000000ab275080,
    0x0000000fe73c1959,
    0x000000e54c0a74b1,
    0x00000d4398ff1ef9,
    0x0000a18c2ee1c097,
    0x00079fb80b072e4a,
    0x0072db27380dd689,
    0x0842f488162e2284,
    0xf66745411d8ab063,
    0xfffffffffffffffb,
    0xfffffffffffffffc,
    0xffffffffffffffff,
];

const TEST_DIV64_DIVISOR_0: u32 = 0x00000009;
const TEST_DIV64_DIVISOR_1: u32 = 0x0000007c;
const TEST_DIV64_DIVISOR_2: u32 = 0x00000204;
const TEST_DIV64_DIVISOR_3: u32 = 0x0000cb5b;
const TEST_DIV64_DIVISOR_4: u32 = 0x00010000;
const TEST_DIV64_DIVISOR_5: u32 = 0x0008a880;
const TEST_DIV64_DIVISOR_6: u32 = 0x003fd3ae;
const TEST_DIV64_DIVISOR_7: u32 = 0x0b658fac;
const TEST_DIV64_DIVISOR_8: u32 = 0x80000001;
const TEST_DIV64_DIVISOR_9: u32 = 0xdc08b349;
const TEST_DIV64_DIVISOR_A: u32 = 0xfffffffe;
const TEST_DIV64_DIVISOR_B: u32 = 0xffffffff;

static TEST_DIV64_DIVISORS: [u32; 12] = [
    TEST_DIV64_DIVISOR_0,
    TEST_DIV64_DIVISOR_1,
    TEST_DIV64_DIVISOR_2,
    TEST_DIV64_DIVISOR_3,
    TEST_DIV64_DIVISOR_4,
    TEST_DIV64_DIVISOR_5,
    TEST_DIV64_DIVISOR_6,
    TEST_DIV64_DIVISOR_7,
    TEST_DIV64_DIVISOR_8,
    TEST_DIV64_DIVISOR_9,
    TEST_DIV64_DIVISOR_A,
    TEST_DIV64_DIVISOR_B,
];

#[derive(Copy, Clone)]
struct TestDiv64Result {
    quotient: u64,
    remainder: u32,
}

// Expected quotient and remainder values from the original test module.
static TEST_DIV64_RESULTS: [[TestDiv64Result; 12]; 12] = [
    [
        TestDiv64Result {
            quotient: 0x13045e47,
            remainder: 1,
        },
        TestDiv64Result {
            quotient: 0x161596c,
            remainder: 0x30,
        },
        TestDiv64Result {
            quotient: 0x54e9d4,
            remainder: 0x130,
        },
        TestDiv64Result {
            quotient: 0xd776,
            remainder: 0x278e,
        },
        TestDiv64Result {
            quotient: 0xab27,
            remainder: 0x5080,
        },
        TestDiv64Result {
            quotient: 0x13c4,
            remainder: 0x4ce80,
        },
        TestDiv64Result {
            quotient: 0x2ae,
            remainder: 0x1e143c,
        },
        TestDiv64Result {
            quotient: 0xf,
            remainder: 0x33e56c,
        },
        TestDiv64Result {
            quotient: 1,
            remainder: 0x2b27507f,
        },
        TestDiv64Result {
            quotient: 0,
            remainder: 0xab275080,
        },
        TestDiv64Result {
            quotient: 0,
            remainder: 0xab275080,
        },
        TestDiv64Result {
            quotient: 0,
            remainder: 0xab275080,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0x1c45c02d1,
            remainder: 0,
        },
        TestDiv64Result {
            quotient: 0x20d5213c,
            remainder: 0x49,
        },
        TestDiv64Result {
            quotient: 0x7e3d65f,
            remainder: 0x1dd,
        },
        TestDiv64Result {
            quotient: 0x140531,
            remainder: 0x65ee,
        },
        TestDiv64Result {
            quotient: 0xfe73c,
            remainder: 0x1959,
        },
        TestDiv64Result {
            quotient: 0x1d637,
            remainder: 0x4e5d9,
        },
        TestDiv64Result {
            quotient: 0x3fc9,
            remainder: 0x713bb,
        },
        TestDiv64Result {
            quotient: 0x165,
            remainder: 0x29abe7d,
        },
        TestDiv64Result {
            quotient: 0x1f,
            remainder: 0x673c193a,
        },
        TestDiv64Result {
            quotient: 0x12,
            remainder: 0x6e9f7e37,
        },
        TestDiv64Result {
            quotient: 0xf,
            remainder: 0xe73c1977,
        },
        TestDiv64Result {
            quotient: 0xf,
            remainder: 0xe73c1968,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0x197a3a0cf7,
            remainder: 2,
        },
        TestDiv64Result {
            quotient: 0x1d9632e5c,
            remainder: 0x21,
        },
        TestDiv64Result {
            quotient: 0x71c28039,
            remainder: 0x1cd,
        },
        TestDiv64Result {
            quotient: 0x120a844,
            remainder: 0xb885,
        },
        TestDiv64Result {
            quotient: 0xe54c0a,
            remainder: 0x74b1,
        },
        TestDiv64Result {
            quotient: 0x1a7bb3,
            remainder: 0x72331,
        },
        TestDiv64Result {
            quotient: 0x397ad,
            remainder: 0x2c61b,
        },
        TestDiv64Result {
            quotient: 0x141e,
            remainder: 0x6ea2e89,
        },
        TestDiv64Result {
            quotient: 0x1ca,
            remainder: 0x4c0a72e7,
        },
        TestDiv64Result {
            quotient: 0x10a,
            remainder: 0xab002ad7,
        },
        TestDiv64Result {
            quotient: 0xe5,
            remainder: 0x4c0a767b,
        },
        TestDiv64Result {
            quotient: 0xe5,
            remainder: 0x4c0a7596,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0x17949e37538,
            remainder: 1,
        },
        TestDiv64Result {
            quotient: 0x1b62441f37,
            remainder: 0x55,
        },
        TestDiv64Result {
            quotient: 0x694a3391d,
            remainder: 0x85,
        },
        TestDiv64Result {
            quotient: 0x10b2a5d2,
            remainder: 0xa753,
        },
        TestDiv64Result {
            quotient: 0xd4398ff,
            remainder: 0x1ef9,
        },
        TestDiv64Result {
            quotient: 0x1882ec6,
            remainder: 0x5cbf9,
        },
        TestDiv64Result {
            quotient: 0x35333b,
            remainder: 0x17abdf,
        },
        TestDiv64Result {
            quotient: 0x129f1,
            remainder: 0xab4520d,
        },
        TestDiv64Result {
            quotient: 0x1a87,
            remainder: 0x18ff0472,
        },
        TestDiv64Result {
            quotient: 0xf6e,
            remainder: 0x8ac0ce9b,
        },
        TestDiv64Result {
            quotient: 0xd43,
            remainder: 0x98ff397f,
        },
        TestDiv64Result {
            quotient: 0xd43,
            remainder: 0x98ff2c3c,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0x11f321a74e49,
            remainder: 6,
        },
        TestDiv64Result {
            quotient: 0x14d8481d211,
            remainder: 0x5b,
        },
        TestDiv64Result {
            quotient: 0x5025cbd92d,
            remainder: 0x1e3,
        },
        TestDiv64Result {
            quotient: 0xcb5e71e3,
            remainder: 0x43e6,
        },
        TestDiv64Result {
            quotient: 0xa18c2ee1,
            remainder: 0xc097,
        },
        TestDiv64Result {
            quotient: 0x12a88828,
            remainder: 0x36c97,
        },
        TestDiv64Result {
            quotient: 0x287f16f,
            remainder: 0x2c2a25,
        },
        TestDiv64Result {
            quotient: 0xe2cc7,
            remainder: 0x2d581e3,
        },
        TestDiv64Result {
            quotient: 0x14318,
            remainder: 0x2ee07d7f,
        },
        TestDiv64Result {
            quotient: 0xbbf4,
            remainder: 0x1ba08c03,
        },
        TestDiv64Result {
            quotient: 0xa18c,
            remainder: 0x2ee303af,
        },
        TestDiv64Result {
            quotient: 0xa18c,
            remainder: 0x2ee26223,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0xd8db8f72935d,
            remainder: 5,
        },
        TestDiv64Result {
            quotient: 0xfbd5aed7a2e,
            remainder: 2,
        },
        TestDiv64Result {
            quotient: 0x3c84b6ea64a,
            remainder: 0x122,
        },
        TestDiv64Result {
            quotient: 0x998fa8829,
            remainder: 0x44b7,
        },
        TestDiv64Result {
            quotient: 0x79fb80b07,
            remainder: 0x2e4a,
        },
        TestDiv64Result {
            quotient: 0xe16b20fa,
            remainder: 0x2a14a,
        },
        TestDiv64Result {
            quotient: 0x1e940d22,
            remainder: 0x353b2e,
        },
        TestDiv64Result {
            quotient: 0xab40ac,
            remainder: 0x6fba6ba,
        },
        TestDiv64Result {
            quotient: 0xf3f70,
            remainder: 0xaf7eeda,
        },
        TestDiv64Result {
            quotient: 0x8debd,
            remainder: 0x72d98365,
        },
        TestDiv64Result {
            quotient: 0x79fb8,
            remainder: 0xb166dba,
        },
        TestDiv64Result {
            quotient: 0x79fb8,
            remainder: 0xb0ece02,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0xcc3045b8fc281,
            remainder: 0,
        },
        TestDiv64Result {
            quotient: 0xed1f48b5c9fc,
            remainder: 0x79,
        },
        TestDiv64Result {
            quotient: 0x38fb9c63406a,
            remainder: 0xe1,
        },
        TestDiv64Result {
            quotient: 0x909705b825,
            remainder: 0xa62,
        },
        TestDiv64Result {
            quotient: 0x72db27380d,
            remainder: 0xd689,
        },
        TestDiv64Result {
            quotient: 0xd43fce827,
            remainder: 0x82b09,
        },
        TestDiv64Result {
            quotient: 0x1ccaba11a,
            remainder: 0x37e8dd,
        },
        TestDiv64Result {
            quotient: 0xa13f729,
            remainder: 0x566dffd,
        },
        TestDiv64Result {
            quotient: 0xe5b64e,
            remainder: 0x3728203b,
        },
        TestDiv64Result {
            quotient: 0x85a14b,
            remainder: 0x23d36726,
        },
        TestDiv64Result {
            quotient: 0x72db27,
            remainder: 0x38f38cd7,
        },
        TestDiv64Result {
            quotient: 0x72db27,
            remainder: 0x3880b1b0,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0xeafeb9c993592b,
            remainder: 1,
        },
        TestDiv64Result {
            quotient: 0x110e5befa9a991,
            remainder: 0x48,
        },
        TestDiv64Result {
            quotient: 0x41947b4a1d36a,
            remainder: 0xdc,
        },
        TestDiv64Result {
            quotient: 0xa6679327311,
            remainder: 0xc079,
        },
        TestDiv64Result {
            quotient: 0x842f488162e,
            remainder: 0x2284,
        },
        TestDiv64Result {
            quotient: 0xf4459740fc,
            remainder: 0x84484,
        },
        TestDiv64Result {
            quotient: 0x2122c47bf9,
            remainder: 0x2ca446,
        },
        TestDiv64Result {
            quotient: 0xb9936290,
            remainder: 0x4979c4,
        },
        TestDiv64Result {
            quotient: 0x1085e910,
            remainder: 0x5a83974,
        },
        TestDiv64Result {
            quotient: 0x99ca89d,
            remainder: 0x9db446bf,
        },
        TestDiv64Result {
            quotient: 0x842f488,
            remainder: 0x26b40b94,
        },
        TestDiv64Result {
            quotient: 0x842f488,
            remainder: 0x1e71170c,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0x1b60cece589da1d2,
            remainder: 1,
        },
        TestDiv64Result {
            quotient: 0x1fcb42be1453f5b,
            remainder: 0x4f,
        },
        TestDiv64Result {
            quotient: 0x7a3f2457df0749,
            remainder: 0x13f,
        },
        TestDiv64Result {
            quotient: 0x1363130e3ec7b,
            remainder: 0x17aa,
        },
        TestDiv64Result {
            quotient: 0xf66745411d8a,
            remainder: 0xb063,
        },
        TestDiv64Result {
            quotient: 0x1c757dfab350,
            remainder: 0x48863,
        },
        TestDiv64Result {
            quotient: 0x3dc4979c652,
            remainder: 0x224ea7,
        },
        TestDiv64Result {
            quotient: 0x159edc3144,
            remainder: 0x6409ab3,
        },
        TestDiv64Result {
            quotient: 0x1ecce8a7e,
            remainder: 0x30bc25e5,
        },
        TestDiv64Result {
            quotient: 0x11eadfee3,
            remainder: 0xa99c48a8,
        },
        TestDiv64Result {
            quotient: 0xf6674543,
            remainder: 0xa593ae9,
        },
        TestDiv64Result {
            quotient: 0xf6674542,
            remainder: 0x13f1f5a5,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0x1c71c71c71c71c71,
            remainder: 2,
        },
        TestDiv64Result {
            quotient: 0x210842108421084,
            remainder: 0xb,
        },
        TestDiv64Result {
            quotient: 0x7f01fc07f01fc0,
            remainder: 0xfb,
        },
        TestDiv64Result {
            quotient: 0x14245eabf1f9a,
            remainder: 0xa63d,
        },
        TestDiv64Result {
            quotient: 0xffffffffffff,
            remainder: 0xfffb,
        },
        TestDiv64Result {
            quotient: 0x1d913cecc509,
            remainder: 0x7937b,
        },
        TestDiv64Result {
            quotient: 0x402c70c678f,
            remainder: 0x5bfc9,
        },
        TestDiv64Result {
            quotient: 0x16766cb70b,
            remainder: 0x45edf97,
        },
        TestDiv64Result {
            quotient: 0x1fffffffb,
            remainder: 0x80000000,
        },
        TestDiv64Result {
            quotient: 0x129d84b3a,
            remainder: 0xa2e8fe71,
        },
        TestDiv64Result {
            quotient: 0x100000001,
            remainder: 0xfffffffd,
        },
        TestDiv64Result {
            quotient: 0x100000000,
            remainder: 0xfffffffb,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0x1c71c71c71c71c71,
            remainder: 3,
        },
        TestDiv64Result {
            quotient: 0x210842108421084,
            remainder: 0xc,
        },
        TestDiv64Result {
            quotient: 0x7f01fc07f01fc0,
            remainder: 0xfc,
        },
        TestDiv64Result {
            quotient: 0x14245eabf1f9a,
            remainder: 0xa63e,
        },
        TestDiv64Result {
            quotient: 0xffffffffffff,
            remainder: 0xfffc,
        },
        TestDiv64Result {
            quotient: 0x1d913cecc509,
            remainder: 0x7937c,
        },
        TestDiv64Result {
            quotient: 0x402c70c678f,
            remainder: 0x5bfca,
        },
        TestDiv64Result {
            quotient: 0x16766cb70b,
            remainder: 0x45edf98,
        },
        TestDiv64Result {
            quotient: 0x1fffffffc,
            remainder: 0,
        },
        TestDiv64Result {
            quotient: 0x129d84b3a,
            remainder: 0xa2e8fe72,
        },
        TestDiv64Result {
            quotient: 0x100000002,
            remainder: 0,
        },
        TestDiv64Result {
            quotient: 0x100000000,
            remainder: 0xfffffffc,
        },
    ],
    [
        TestDiv64Result {
            quotient: 0x1c71c71c71c71c71,
            remainder: 6,
        },
        TestDiv64Result {
            quotient: 0x210842108421084,
            remainder: 0xf,
        },
        TestDiv64Result {
            quotient: 0x7f01fc07f01fc0,
            remainder: 0xff,
        },
        TestDiv64Result {
            quotient: 0x14245eabf1f9a,
            remainder: 0xa641,
        },
        TestDiv64Result {
            quotient: 0xffffffffffff,
            remainder: 0xffff,
        },
        TestDiv64Result {
            quotient: 0x1d913cecc509,
            remainder: 0x7937f,
        },
        TestDiv64Result {
            quotient: 0x402c70c678f,
            remainder: 0x5bfcd,
        },
        TestDiv64Result {
            quotient: 0x16766cb70b,
            remainder: 0x45edf9b,
        },
        TestDiv64Result {
            quotient: 0x1fffffffc,
            remainder: 3,
        },
        TestDiv64Result {
            quotient: 0x129d84b3a,
            remainder: 0xa2e8fe75,
        },
        TestDiv64Result {
            quotient: 0x100000002,
            remainder: 3,
        },
        TestDiv64Result {
            quotient: 0x100000001,
            remainder: 0,
        },
    ],
];

fn test_div64_one(dividend: u64, divisor: u32, expected: TestDiv64Result) -> bool {
    let mut quotient = dividend;
    // Every original vector has a nonzero divisor. An unexpected checked
    // failure is a test failure, never silently replaced with a result.
    let Some(remainder) = do_div(&mut quotient, divisor) else {
        pr_err!("ERROR: checked division rejected {dividend:016x} / {divisor:08x}\n");
        return false;
    };
    if quotient == expected.quotient && remainder == expected.remainder {
        return true;
    }
    pr_err!("ERROR: {dividend:016x} / {divisor:08x} => {quotient:016x},{remainder:08x}\n");
    pr_err!(
        "ERROR: expected value              => {:016x},{:08x}\n",
        expected.quotient,
        expected.remainder
    );
    false
}

// Each instantiation retains the original explicit constant-divisor call.
// The second pass below deliberately obtains its divisor at runtime.
#[inline(always)]
fn test_div64_constant<const DIVISOR: u32>(dividend: u64, expected: TestDiv64Result) -> bool {
    test_div64_one(dividend, DIVISOR, expected)
}

fn test_div64() -> bool {
    for (i, &dividend) in TEST_DIV64_DIVIDENDS.iter().enumerate() {
        let dividend = black_box(dividend);
        macro_rules! constant {
            ($divisor:ident, $column:expr) => {
                if !test_div64_constant::<$divisor>(dividend, TEST_DIV64_RESULTS[i][$column]) {
                    return false;
                }
            };
        }
        constant!(TEST_DIV64_DIVISOR_0, 0);
        constant!(TEST_DIV64_DIVISOR_1, 1);
        constant!(TEST_DIV64_DIVISOR_2, 2);
        constant!(TEST_DIV64_DIVISOR_3, 3);
        constant!(TEST_DIV64_DIVISOR_4, 4);
        constant!(TEST_DIV64_DIVISOR_5, 5);
        constant!(TEST_DIV64_DIVISOR_6, 6);
        constant!(TEST_DIV64_DIVISOR_7, 7);
        constant!(TEST_DIV64_DIVISOR_8, 8);
        constant!(TEST_DIV64_DIVISOR_9, 9);
        constant!(TEST_DIV64_DIVISOR_A, 10);
        constant!(TEST_DIV64_DIVISOR_B, 11);
        for (j, &divisor) in TEST_DIV64_DIVISORS.iter().enumerate() {
            if !test_div64_one(dividend, black_box(divisor), TEST_DIV64_RESULTS[i][j]) {
                return false;
            }
        }
    }
    true
}

struct DivisionTest;

impl kernel::Module for DivisionTest {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Starting 64bit/32bit division and modulo test\n");
        let start = Instant::<Monotonic>::now();
        for _ in 0..TEST_DIV64_N_ITER {
            if !test_div64() {
                break;
            }
        }
        let elapsed = start.elapsed().as_nanos() as u64;
        let (seconds, nanos) = kernel::math::div_u64_rem(elapsed, 1_000_000_000)
            .expect("nanoseconds per second is nonzero");
        pr_info!("Completed 64bit/32bit division and modulo test, {seconds}.{nanos:09}s elapsed\n");
        // Original C reports failures through diagnostics but returns zero.
        Ok(Self)
    }
}

impl Drop for DivisionTest {
    fn drop(&mut self) {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
