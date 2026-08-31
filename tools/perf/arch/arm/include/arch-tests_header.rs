/* SPDX-License-Identifier: GPL-2.0 */

use crate::test_suite;

unsafe extern "C" {
    pub static mut arch_tests: [*mut test_suite; 0usize];
}
