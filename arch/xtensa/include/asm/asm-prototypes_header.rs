/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <asm/cacheflush.h>
// #include <asm/checksum.h>
// #include <asm/ftrace.h>
// #include <asm/page.h>
// #include <asm/string.h>
// #include <asm/uaccess.h>
// #include <asm-generic/asm-prototypes.h>

/*
 * gcc internal math functions
 */
unsafe extern "C" {
    pub fn __ashrdi3(arg0: i64, arg1: i32) -> i64;
    pub fn __ashldi3(arg0: i64, arg1: i32) -> i64;
    pub fn __bswapdi2(arg0: i64) -> i64;
    pub fn __bswapsi2(arg0: i32) -> i32;
    pub fn __lshrdi3(arg0: i64, arg1: i32) -> i64;
    pub fn __divsi3(arg0: i32, arg1: i32) -> i32;
    pub fn __modsi3(arg0: i32, arg1: i32) -> i32;
    pub fn __mulsi3(arg0: i32, arg1: i32) -> i32;
    pub fn __udivsi3(arg0: u32, arg1: u32) -> u32;
    pub fn __umodsi3(arg0: u32, arg1: u32) -> u32;
    pub fn __umulsidi3(arg0: u32, arg1: u32) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
