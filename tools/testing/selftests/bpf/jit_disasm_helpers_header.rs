/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

// C dependency intent: <stddef.h> for size_t.

unsafe extern "C" {
    pub fn get_jited_program_text(
        fd: ::std::os::raw::c_int,
        text: *mut ::std::os::raw::c_char,
        text_sz: usize,
    ) -> ::std::os::raw::c_int;
}
