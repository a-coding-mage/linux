/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated spinlock header.
extern "C" {
    pub static mut rtc_lock: spinlock_t;
}

// When building for SPARC 64-bit, use the 64-bit MC146818 RTC definitions;
// otherwise use the 32-bit definitions.
// C condition: defined(__sparc__) && defined(__arch64__)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
