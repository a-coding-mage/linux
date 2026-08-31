/* Trivial program to check that compilation with certain flags is working. */

// C dependency: #include <stdio.h>
unsafe extern "C" {
    fn puts(s: *const i8) -> i32;
}

fn main() -> i32 {
    unsafe {
        puts(c"".as_ptr());
    }
    0
}
