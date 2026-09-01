/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
/* Translated from C header bpf_arena_strsearch.h. */
/* Depends on declarations/macros from bpf_arena_common.h, including __noinline,
 * __arena address-space intent, and cond_break.
 */

#[inline(never)]
pub unsafe extern "C" fn bpf_arena_strlen(s: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut sc: *const ::core::ffi::c_char;

    sc = s;
    while *sc != 0 {
        sc = sc.add(1);
        /* cond_break; */
    }
    sc.offset_from(s) as ::core::ffi::c_int
}

/**
 * glob_match - Shell-style pattern matching, like !fnmatch(pat, str, 0)
 * @pat: Shell-style pattern to match, e.g. "*.[ch]".
 * @str: String to match.  The pattern must match the entire string.
 *
 * Perform shell-style glob matching, returning true (1) if the match
 * succeeds, or false (0) if it fails.  Equivalent to !fnmatch(@pat, @str, 0).
 *
 * Pattern metacharacters are ?, *, [ and \.
 * (And, inside character classes, !, - and ].)
 *
 * This is small and simple implementation intended for device blacklists
 * where a string is matched against a number of patterns.  Thus, it
 * does not preprocess the patterns.  It is non-recursive, and run-time
 * is at most quadratic: strlen(@str)*strlen(@pat).
 *
 * An example of the worst case is glob_match("*aaaaa", "aaaaaaaaaa");
 * it takes 6 passes over the pattern before matching the string.
 *
 * Like !fnmatch(@pat, @str, 0) and unlike the shell, this does NOT
 * treat / or leading . specially; it isn't actually used for pathnames.
 *
 * Note that according to glob(7) (and unlike bash), character classes
 * are complemented by a leading !; this does not support the regex-style
 * [^a-z] syntax.
 *
 * An opening bracket without a matching close is matched literally.
 */
#[inline(never)]
pub unsafe extern "C" fn glob_match(
    mut pat: *const ::core::ffi::c_char,
    mut str_: *const ::core::ffi::c_char,
) -> bool {
    /*
     * Backtrack to previous * on mismatch and retry starting one
     * character later in the string.  Because * matches all characters
     * (no exception for /), it can be easily proved that there's
     * never a need to backtrack multiple levels.
     */
    let mut back_pat: *const ::core::ffi::c_char = ::core::ptr::null();
    let mut back_str: *const ::core::ffi::c_char;

    /*
     * Loop over each token (character or class) in pat, matching
     * it against the remaining unmatched tail of str.  Return false
     * on mismatch, or true after matching the trailing nul bytes.
     */
    loop {
        let c: u8 = *str_ as u8;
        str_ = str_.add(1);
        let mut d: u8 = *pat as u8;
        pat = pat.add(1);

        match d {
            b'?' => {
                /* Wildcard: anything but nul */
                if c == b'\0' {
                    return false;
                }
            }
            b'*' => {
                /* Any-length wildcard */
                if *pat == 0 {
                    /* Optimize trailing * case */
                    return true;
                }
                back_pat = pat;
                str_ = str_.sub(1);
                back_str = str_; /* Allow zero-length match */
            }
            b'[' => {
                /* Character class */
                let mut match_: bool = false;
                let inverted: bool = *pat == b'!' as ::core::ffi::c_char;
                let mut class: *const ::core::ffi::c_char = pat.add(inverted as usize);
                let mut a: u8 = *class as u8;
                class = class.add(1);

                /*
                 * Iterate over each span in the character class.
                 * A span is either a single character a, or a
                 * range a-b.  The first span may begin with ']'.
                 */
                loop {
                    let mut b: u8 = a;

                    if a == b'\0' {
                        /* Malformed */
                        if c == d {
                            if d == b'\0' {
                                return true;
                            }
                            break;
                        }
                        if c == b'\0' || back_pat.is_null() {
                            return false; /* No point continuing */
                        }
                        /* Try again from last *, one character later in str. */
                        pat = back_pat;
                        back_str = back_str.add(1);
                        str_ = back_str;
                        break;
                    }

                    if *class.add(0) == b'-' as ::core::ffi::c_char
                        && *class.add(1) != b']' as ::core::ffi::c_char
                    {
                        b = *class.add(1) as u8;

                        if b == b'\0' {
                            if c == d {
                                if d == b'\0' {
                                    return true;
                                }
                                break;
                            }
                            if c == b'\0' || back_pat.is_null() {
                                return false; /* No point continuing */
                            }
                            /* Try again from last *, one character later in str. */
                            pat = back_pat;
                            back_str = back_str.add(1);
                            str_ = back_str;
                            break;
                        }

                        class = class.add(2);
                        /* Any special action if a > b? */
                    }
                    match_ |= a <= c && c <= b;
                    /* cond_break; */

                    a = *class as u8;
                    class = class.add(1);
                    if a == b']' {
                        if match_ == inverted {
                            if c == b'\0' || back_pat.is_null() {
                                return false; /* No point continuing */
                            }
                            /* Try again from last *, one character later in str. */
                            pat = back_pat;
                            back_str = back_str.add(1);
                            str_ = back_str;
                        } else {
                            pat = class;
                        }
                        break;
                    }
                }
            }
            b'\\' => {
                d = *pat as u8;
                pat = pat.add(1);
                if c == d {
                    if d == b'\0' {
                        return true;
                    }
                } else {
                    if c == b'\0' || back_pat.is_null() {
                        return false; /* No point continuing */
                    }
                    /* Try again from last *, one character later in str. */
                    pat = back_pat;
                    back_str = back_str.add(1);
                    str_ = back_str;
                }
            }
            _ => {
                /* Literal character */
                if c == d {
                    if d == b'\0' {
                        return true;
                    }
                } else {
                    if c == b'\0' || back_pat.is_null() {
                        return false; /* No point continuing */
                    }
                    /* Try again from last *, one character later in str. */
                    pat = back_pat;
                    back_str = back_str.add(1);
                    str_ = back_str;
                }
            }
        }
        /* cond_break; */
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
