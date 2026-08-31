// SPDX-License-Identifier: GPL-2.0
// C dependency: <sys/sdt.h>

extern "C" {
    fn DTRACE_PROBE(provider: *const ::std::os::raw::c_char, name: *const ::std::os::raw::c_char);
}

fn main() {
    unsafe {
        DTRACE_PROBE(b"provider\0".as_ptr() as *const ::std::os::raw::c_char, b"name\0".as_ptr() as *const ::std::os::raw::c_char);
    }
}
