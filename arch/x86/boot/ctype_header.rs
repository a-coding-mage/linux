/* SPDX-License-Identifier: GPL-2.0 */

#[inline]
fn isdigit(ch: i32) -> bool {
    (ch >= '0' as i32) && (ch <= '9' as i32)
}

#[inline]
fn isxdigit(ch: i32) -> bool {
    if isdigit(ch) {
        return true;
    }

    if (ch >= 'a' as i32) && (ch <= 'f' as i32) {
        return true;
    }

    (ch >= 'A' as i32) && (ch <= 'F' as i32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
