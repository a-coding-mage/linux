// SPDX-License-Identifier: GPL-2.0

// C dependency: #include <unistd.h>

/* As a regular read(2), but allows to specify a timeout in micro-seconds.
 * Returns -EAGAIN on timeout.
 */
unsafe extern "C" {
    pub fn read_with_timeout(fd: ::std::os::raw::c_int, buf: *mut ::std::os::raw::c_char, count: usize, usec: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
