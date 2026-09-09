/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <sysdep/archsetjmp.h>.
// Dependency supplied by <os.h>.

unsafe extern "C" {
    pub fn setjmp(buf: jmp_buf) -> i32;
    pub fn longjmp(buf: jmp_buf, val: i32);
    pub fn um_get_signals() -> i32;
    pub fn um_set_signals_trace(enable: i32);
}

#[macro_export]
macro_rules! UML_LONGJMP {
    ($buf:expr, $val:expr) => {{
        unsafe {
            longjmp(*$buf, $val);
        }
    }};
}

#[macro_export]
macro_rules! UML_SETJMP {
    ($buf:expr) => {{
        let mut n: i32;
        let enable: i32;
        unsafe {
            enable = um_get_signals();
            n = setjmp(*$buf);
            if n != 0 {
                um_set_signals_trace(enable);
            }
        }
        n
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
