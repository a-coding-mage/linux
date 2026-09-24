// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//! Allocation-free Linux glob matching with unsigned byte semantics.

#[cfg(not(CONFIG_RUST))]
use core::ffi::c_char;
// Kernel char is unsigned on every architecture, including x86 KCFI.
#[cfg(CONFIG_RUST)]
use kernel::ffi::c_char;

/// Matches the entire input; a NUL in either slice terminates it.
/// Missing terminators are treated as NUL at the slice boundary.
pub fn matches(pattern: &[u8], input: &[u8]) -> bool {
    match_readers(
        |i| pattern.get(i).copied().unwrap_or(0),
        |i| input.get(i).copied().unwrap_or(0),
    )
}

fn match_readers(mut pattern: impl FnMut(usize) -> u8, mut input: impl FnMut(usize) -> u8) -> bool {
    let (mut p, mut s) = (0, 0);
    let mut backtrack = None;
    loop {
        let c = input(s);
        let mut d = pattern(p);
        p += 1;
        s += 1;
        let matched = match d {
            b'?' => {
                if c == 0 {
                    return false;
                }
                true
            }
            b'*' => {
                if pattern(p) == 0 {
                    return true;
                }
                s -= 1;
                backtrack = Some((p, s));
                true
            }
            b'[' => {
                if c == 0 {
                    return false;
                }
                let inverted = pattern(p) == b'!';
                let mut class = p + usize::from(inverted);
                let mut a = pattern(class);
                class += 1;
                let mut hit = false;
                loop {
                    if a == 0 {
                        break c == b'[';
                    }
                    let mut b = a;
                    if pattern(class) == b'-' && pattern(class + 1) != b']' {
                        b = pattern(class + 1);
                        if b == 0 {
                            break c == b'[';
                        }
                        class += 2;
                    }
                    hit |= a <= c && c <= b;
                    a = pattern(class);
                    class += 1;
                    if a == b']' {
                        if hit != inverted {
                            p = class;
                        }
                        break hit != inverted;
                    }
                }
            }
            _ => {
                if d == b'\\' {
                    d = pattern(p);
                    p += 1;
                }
                if c == d && d == 0 {
                    return true;
                }
                c == d
            }
        };
        if !matched {
            if c == 0 {
                return false;
            }
            match backtrack {
                Some((bp, bs)) => {
                    p = bp;
                    s = bs + 1;
                    backtrack = Some((bp, s));
                }
                None => return false,
            }
        }
    }
}

// Reads are lazy: do not inspect suffixes that the matcher never visits.
// In particular, a zero bound returns NUL without doing pointer arithmetic.
unsafe fn read_byte(ptr: *const c_char, index: usize, limit: usize) -> u8 {
    if index >= limit {
        return 0;
    }
    #[cfg(test)]
    READS.with(|count| count.set(count.get() + 1));
    // SAFETY: The caller provides readable, stable bytes through NUL or limit.
    // The matcher never requests a byte beyond a terminator.
    unsafe { *ptr.add(index) as u8 }
}

unsafe fn match_raw(pat: *const c_char, input: *const c_char, len: usize) -> bool {
    // SAFETY: Forward the caller's string contracts to each lazy access.
    unsafe {
        match_readers(
            |i| read_byte(pat, i, usize::MAX),
            |i| read_byte(input, i, len),
        )
    }
}

/// Shell-style matching, like `!fnmatch(pat, str, 0)`.
///
/// # Safety
/// Both pointers must reference readable NUL-terminated strings, stable for this call.
#[no_mangle]
pub unsafe extern "C" fn glob_match(pat: *const c_char, str_: *const c_char) -> bool {
    // SAFETY: Required by this function's contract.
    unsafe { match_raw(pat, str_, usize::MAX) }
}

/// Matches an input that need not be NUL-terminated.
///
/// # Safety
/// `pat` must be a readable NUL-terminated string. `str_` must be readable
/// through its first NUL or `len` bytes, whichever comes first. These bytes
/// must remain stable for this call and belong to one object.
#[no_mangle]
pub unsafe extern "C" fn glob_match_len(
    pat: *const c_char,
    str_: *const c_char,
    len: usize,
) -> bool {
    // SAFETY: Required by this function's contract; a zero limit reads nothing.
    unsafe { match_raw(pat, str_, len) }
}

#[cfg(test)]
std::thread_local! {
    static READS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

#[cfg(test)]
mod access_tests {
    use super::*;

    #[test]
    fn slice_boundaries_and_nul() {
        assert!(matches(b"a*", b"abc"));
        assert!(matches(b"a\0b", b"a\0c"));
        assert!(matches(b"", b""));
        assert!(!matches(b"?", b""));
        assert!(!matches(b"b", b"a"));
    }

    #[test]
    fn early_exit_read_counts_do_not_scale_with_suffixes() {
        for n in [1, 64, 4096, 1 << 20] {
            let mut input = vec![b'a'; n];
            input.push(0);
            let mut long_pattern = vec![b'b'; n];
            long_pattern.push(0);
            for (pattern, expected, reads) in [
                (&b"*\0"[..], true, 3),
                (&b"a*\0"[..], true, 5),
                (&b"b\0"[..], false, 2),
                (&long_pattern[..], false, 2),
            ] {
                for bounded in [false, true] {
                    READS.with(|count| count.set(0));
                    // SAFETY: Both vectors/slices include a NUL and remain live.
                    let actual = unsafe {
                        if bounded {
                            glob_match_len(pattern.as_ptr().cast(), input.as_ptr().cast(), n)
                        } else {
                            glob_match(pattern.as_ptr().cast(), input.as_ptr().cast())
                        }
                    };
                    let count = READS.with(|count| count.get());
                    assert_eq!(actual, expected);
                    let expected_reads = reads - usize::from(bounded && n == 1 && pattern == b"a*\0");
                    assert_eq!(count, expected_reads, "size={n} bounded={bounded} pattern={:?}", &pattern[..pattern.len().min(3)]);
                    println!("size={n} bounded={bounded} reads={count} expected={expected_reads}");
                }
            }
        }
        READS.with(|count| count.set(0));
        // SAFETY: A zero-length input is never read; the pattern is terminated.
        assert!(unsafe { glob_match_len(b"*\0".as_ptr().cast(), core::ptr::null(), 0) });
        assert_eq!(READS.with(|count| count.get()), 2);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
