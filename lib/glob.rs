// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Dependency intent: Linux module metadata and symbol-export declarations.

/*
 * The only reason this code can be compiled as a module is because the
 * ATA code that depends on it can be as well.  In practice, they're
 * both usually compiled in and the module overhead goes away.
 */
// MODULE_DESCRIPTION("glob(7) matching");
// MODULE_LICENSE("Dual MIT/GPL");

unsafe fn glob_match_str(
    mut pat: *const u8,
    mut str_: *const u8,
    str_end: *const u8,
) -> bool {
    /*
     * Backtrack to previous * on mismatch and retry starting one
     * character later in the string.  Because * matches all characters
     * (no exception for /), it can be easily proved that there's
     * never a need to backtrack multiple levels.
     */
    let mut back_pat: *const u8 = core::ptr::null();
    let mut back_str: *const u8 = core::ptr::null();

    /*
     * Loop over each token (character or class) in pat, matching
     * it against the remaining unmatched tail of str.  Return false
     * on mismatch, or true after matching the trailing nul bytes.
     */
    loop {
        let c = if !str_end.is_null() && str_ >= str_end {
            0
        } else {
            *str_
        };
        let mut d = *pat;
        pat = pat.add(1);
        str_ = str_.add(1);

        match d {
            b'?' => {
                if c == 0 {
                    return false;
                }
            }
            b'*' => {
                if *pat == 0 {
                    return true;
                }
                back_pat = pat;
                back_str = str_.sub(1);
            }
            b'[' => {
                if c == 0 {
                    return false;
                }
                let inverted = *pat == b'!';
                let mut class = if inverted { pat.add(1) } else { pat };
                let mut a = *class;
                class = class.add(1);
                let mut matched = false;

                /* Iterate over each span in the character class. */
                loop {
                    let mut b = a;
                    if a == 0 {
                        d = b'[';
                        break;
                    }
                    if *class == b'-' && *class.add(1) != b']' {
                        b = *class.add(1);
                        if b == 0 {
                            d = b'[';
                            break;
                        }
                        class = class.add(2);
                    }
                    if a <= c && c <= b {
                        matched = true;
                    }
                    a = *class;
                    class = class.add(1);
                    if a == b']' {
                        if matched == inverted {
                            if c == 0 || back_pat.is_null() {
                                return false;
                            }
                            pat = back_pat;
                            back_str = back_str.add(1);
                            str_ = back_str;
                        } else {
                            pat = class;
                        }
                        d = b'\0';
                        break;
                    }
                }
                if d == 0 {
                    continue;
                }
                if c == d {
                    continue;
                }
                if c == 0 || back_pat.is_null() {
                    return false;
                }
                pat = back_pat;
                back_str = back_str.add(1);
                str_ = back_str;
            }
            b'\\' => {
                d = *pat;
                pat = pat.add(1);
                if c == d {
                    if d == 0 {
                        return true;
                    }
                    continue;
                }
                if c == 0 || back_pat.is_null() {
                    return false;
                }
                pat = back_pat;
                back_str = back_str.add(1);
                str_ = back_str;
            }
            _ => {
                if c == d {
                    if d == 0 {
                        return true;
                    }
                    continue;
                }
                if c == 0 || back_pat.is_null() {
                    return false;
                }
                pat = back_pat;
                back_str = back_str.add(1);
                str_ = back_str;
            }
        }
    }
}

/// Shell-style pattern matching, like !fnmatch(pat, str, 0).
pub unsafe extern "C" fn glob_match(pat: *const core::ffi::c_char, str_: *const core::ffi::c_char) -> bool {
    glob_match_str(pat.cast(), str_.cast(), core::ptr::null())
}

// EXPORT_SYMBOL(glob_match);

/// Glob match against a length-bounded string.
pub unsafe extern "C" fn glob_match_len(
    pat: *const core::ffi::c_char,
    str_: *const core::ffi::c_char,
    len: usize,
) -> bool {
    glob_match_str(pat.cast(), str_.cast(), str_.cast::<u8>().add(len))
}

// EXPORT_SYMBOL(glob_match_len);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
