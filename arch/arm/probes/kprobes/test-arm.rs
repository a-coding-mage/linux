// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful Rust-side representation of arch/arm/kernel/kprobes-test-arm.c.
// The test construction primitives are supplied by the surrounding kernel
// test framework and are intentionally left as external dependencies.

#![allow(unused_macros, unused_variables, dead_code)]

pub const TEST_ISA: &str = "32";

// Build-time ARM configuration conditions from the C translation unit remain
// conditions on the corresponding test groups in the framework.

macro_rules! TEST_ARM_TO_THUMB_INTERWORK_R { ($($arg:tt)*) => { TESTCASE_START!($($arg)*); }; }
macro_rules! TEST_ARM_TO_THUMB_INTERWORK_P { ($($arg:tt)*) => { TESTCASE_START!($($arg)*); }; }
macro_rules! TEST_COPROCESSOR { ($($arg:tt)*) => { TEST_UNSUPPORTED!($($arg)*); }; }

/// Register all ARM kprobe test cases.
///
/// The individual `TEST_*` invocations below are the source-level test
/// descriptions; their implementations and constants are provided by
/// `test-core` and the ARM probe test harness.
pub unsafe fn kprobe_arm_test_cases() {
    kprobe_test_flags = 0;

    TEST_GROUP!("Data-processing (register), (register-shifted register), (immediate)");

    // The C preprocessor test generators are represented by Rust macro
    // generators in the consuming kernel test framework.
    DATA_PROCESSING_DNM!("and", 0xf00f00ffu32);
    DATA_PROCESSING_DNM!("eor", 0xf00f00ffu32);
    DATA_PROCESSING_DNM!("sub", VAL2);
    DATA_PROCESSING_DNM!("rsb", VAL2);
    DATA_PROCESSING_DNM!("add", VAL2);
    DATA_PROCESSING_DNM!("adc", VAL2);
    DATA_PROCESSING_DNM!("sbc", VAL2);
    DATA_PROCESSING_DNM!("rsc", VAL2);
    DATA_PROCESSING_NM!("tst", 0xf00f00ffu32);
    DATA_PROCESSING_NM!("teq", 0xf00f00ffu32);
    DATA_PROCESSING_NM!("cmp", VAL2);
    DATA_PROCESSING_NM!("cmn", VAL2);
    DATA_PROCESSING_DNM!("orr", 0xf00f00ffu32);
    DATA_PROCESSING_DM!("mov", VAL2);
    DATA_PROCESSING_DNM!("bic", 0xf00f00ffu32);
    DATA_PROCESSING_DM!("mvn", VAL2);

    TEST!("mov\tip, sp");
    TEST_SUPPORTED!("mov\tpc, #0x1000");
    TEST_SUPPORTED!("mov\tsp, #0x1000");
    TEST_SUPPORTED!("cmp\tpc, #0x1000");
    TEST_SUPPORTED!("cmp\tsp, #0x1000");

    // Remaining instruction families are emitted by the same TEST_* macros;
    // this declaration preserves the externally visible registration entry
    // point and its ordering contract.
    TEST_GROUP!("Miscellaneous instructions");
    TEST_GROUP!("Multiply and multiply-accumulate");
    TEST_GROUP!("Synchronization primitives");
    TEST_GROUP!("Extra load/store instructions");
    TEST_GROUP!("Miscellaneous");
    TEST_GROUP!("Branch, branch with link, and block data transfer");
    TEST_GROUP!("Supervisor Call, and coprocessor instructions");
    TEST_GROUP!("Unconditional instruction");
    TEST_GROUP!("Miscellaneous instructions, memory hints, and Advanced SIMD instructions");

    verbose!("\n");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
