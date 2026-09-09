/* SPDX-License-Identifier: GPL-2.0+ */

/*
 * The magic value that all the functions in the test_kprobes_functions array return. The test
 * installs kprobes into these functions, and verify that the functions still correctly return this
 * value.
 */
pub const KPROBE_TEST_MAGIC: u32 = 0xcafebabe;
pub const KPROBE_TEST_MAGIC_LOWER: u32 = 0x0000babe;
pub const KPROBE_TEST_MAGIC_UPPER: u32 = 0xcafe0000;

/* array of addresses to install kprobes */
unsafe extern "C" {
    pub static mut test_kprobes_addresses: [*mut core::ffi::c_void];

    /* array of functions that return KPROBE_TEST_MAGIC */
    pub static mut test_kprobes_functions: [unsafe extern "C" fn() -> core::ffi::c_long];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
