// SPDX-License-Identifier: GPL-2.0
/*
 * Minimal helper for Landlock selftests.  Opens its own working directory
 * before exiting, which may trigger access denials depending on the sandbox
 * configuration.
 */

extern "C" {
    fn open(pathname: *const i8, flags: i32, ...) -> i32;
    fn close(fd: i32) -> i32;
}

const O_RDONLY: i32 = 0;
const O_DIRECTORY: i32 = 0o200000;
const O_CLOEXEC: i32 = 0o2000000;

fn main() {
    unsafe {
        close(open(
            b".\0".as_ptr() as *const i8,
            O_RDONLY | O_DIRECTORY | O_CLOEXEC,
        ));
    }
}
