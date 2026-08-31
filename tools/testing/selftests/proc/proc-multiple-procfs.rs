/*
 * Copyright © 2020 Alexey Gladkov <gladkov.alexey@gmail.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use std::ffi::{c_char, c_int, c_ulong, c_void};
use std::mem::MaybeUninit;

unsafe extern "C" {
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut libc::stat) -> c_int;
    fn umount(target: *const c_char) -> c_int;
}

fn main() {
    let mut proc_st1 = MaybeUninit::<libc::stat>::uninit();
    let mut proc_st2 = MaybeUninit::<libc::stat>::uninit();
    let mut procbuff = *b"/tmp/proc.XXXXXX/meminfo\0";
    let mut procdir1 = *b"/tmp/proc.XXXXXX\0";
    let mut procdir2 = *b"/tmp/proc.XXXXXX\0";

    unsafe {
        assert!(!mkdtemp(procdir1.as_mut_ptr().cast::<c_char>()).is_null());
        assert!(!mkdtemp(procdir2.as_mut_ptr().cast::<c_char>()).is_null());

        assert!(
            mount(
                c"proc".as_ptr(),
                procdir1.as_ptr().cast::<c_char>(),
                c"proc".as_ptr(),
                0,
                c"hidepid=1".as_ptr().cast::<c_void>(),
            ) == 0
        );
        assert!(
            mount(
                c"proc".as_ptr(),
                procdir2.as_ptr().cast::<c_char>(),
                c"proc".as_ptr(),
                0,
                c"hidepid=2".as_ptr().cast::<c_void>(),
            ) == 0
        );

        snprintf(
            procbuff.as_mut_ptr().cast::<c_char>(),
            procbuff.len(),
            c"%s/meminfo".as_ptr(),
            procdir1.as_ptr().cast::<c_char>(),
        );
        assert!(stat(procbuff.as_ptr().cast::<c_char>(), proc_st1.as_mut_ptr()) == 0);

        snprintf(
            procbuff.as_mut_ptr().cast::<c_char>(),
            procbuff.len(),
            c"%s/meminfo".as_ptr(),
            procdir2.as_ptr().cast::<c_char>(),
        );
        assert!(stat(procbuff.as_ptr().cast::<c_char>(), proc_st2.as_mut_ptr()) == 0);

        umount(procdir1.as_ptr().cast::<c_char>());
        umount(procdir2.as_ptr().cast::<c_char>());

        assert!((*proc_st1.as_ptr()).st_dev != (*proc_st2.as_ptr()).st_dev);
    }
}
