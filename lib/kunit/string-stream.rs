// SPDX-License-Identifier: GPL-2.0
/*
 * C++ stream style string builder used in KUnit for building messages.
 *
 * Copyright (C) 2019, Google LLC.
 * Author: Brendan Higgins <brendanhiggins@google.com>
 */

// External kernel dependencies supplied by the surrounding build.

use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;

#[repr(C)]
pub struct StringStreamFragment {
    pub node: ListHead,
    pub fragment: *mut c_char,
}

#[repr(C)]
pub struct StringStream {
    pub fragments: ListHead,
    pub lock: Spinlock,
    pub length: c_int,
    pub gfp: GfpT,
    pub append_newlines: bool,
}

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

#[repr(C)]
pub struct Spinlock {
    _opaque: [u8; 0],
}

pub type GfpT = u32;
pub type VaList = *mut c_void;

extern "C" {
    fn kzalloc(size: usize, flags: GfpT) -> *mut c_void;
    fn kmalloc(size: usize, flags: GfpT) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: VaList) -> c_int;
    fn va_copy(dst: *mut VaList, src: VaList);
    fn va_end(args: VaList);
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn spin_lock(lock: *mut Spinlock);
    fn spin_unlock(lock: *mut Spinlock);
    fn spin_lock_init(lock: *mut Spinlock);
    fn list_del(entry: *mut ListHead);
    fn list_add_tail(new: *mut ListHead, head: *mut ListHead);
    fn init_list_head(list: *mut ListHead);
    fn list_empty(head: *const ListHead) -> bool;
    fn seq_buf_init(s: *mut SeqBuf, buf: *mut c_char, size: usize);
    fn seq_buf_puts(s: *mut SeqBuf, str: *const c_char);
    fn kunit_add_action_or_reset(
        test: *mut Kunit,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> c_int;
    fn kunit_release_action(
        test: *mut Kunit,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    );
}

#[repr(C)]
pub struct SeqBuf {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Kunit {
    _opaque: [u8; 0],
}

const ENOMEM: isize = 12;

unsafe fn alloc_string_stream_fragment(len: c_int, gfp: GfpT) -> *mut StringStreamFragment {
    let frag = kzalloc(core::mem::size_of::<StringStreamFragment>(), gfp)
        as *mut StringStreamFragment;
    if frag.is_null() {
        return (-ENOMEM) as *mut StringStreamFragment;
    }

    (*frag).fragment = kmalloc(len as usize, gfp) as *mut c_char;
    if (*frag).fragment.is_null() {
        kfree(frag as *mut c_void);
        return (-ENOMEM) as *mut StringStreamFragment;
    }

    frag
}

unsafe fn string_stream_fragment_destroy(frag: *mut StringStreamFragment) {
    list_del(&mut (*frag).node);
    kfree((*frag).fragment as *mut c_void);
    kfree(frag as *mut c_void);
}

pub unsafe fn string_stream_vadd(
    stream: *mut StringStream,
    fmt: *const c_char,
    args: VaList,
) -> c_int {
    let mut args_for_counting: VaList = core::ptr::null_mut();
    va_copy(&mut args_for_counting, args);
    let mut buf_len = vsnprintf(core::ptr::null_mut(), 0, fmt, args_for_counting);
    va_end(args_for_counting);

    if buf_len == 0 {
        return 0;
    }
    if (*stream).append_newlines {
        buf_len += 1;
    }
    buf_len += 1;

    let frag_container = alloc_string_stream_fragment(buf_len, (*stream).gfp);
    if (frag_container as isize) == (-ENOMEM) {
        return -ENOMEM as c_int;
    }

    let result_len;
    if (*stream).append_newlines {
        result_len = vsnprintf((*frag_container).fragment, (buf_len - 1) as usize, fmt, args);
        if *(*frag_container).fragment.add(result_len as usize - 1) as u8 != b'\n' {
            result_len += strscpy(
                (*frag_container).fragment.add(result_len as usize),
                b"\n\0".as_ptr() as *const c_char,
                (buf_len - result_len) as usize,
            ) as c_int;
        }
    } else {
        result_len = vsnprintf((*frag_container).fragment, buf_len as usize, fmt, args);
    }

    spin_lock(&mut (*stream).lock);
    (*stream).length += result_len;
    list_add_tail(&mut (*frag_container).node, &mut (*stream).fragments);
    spin_unlock(&mut (*stream).lock);
    0
}

pub unsafe fn string_stream_add(stream: *mut StringStream, fmt: *const c_char, mut args: ...) -> c_int {
    string_stream_vadd(stream, fmt, (&mut args as *mut _) as VaList)
}

pub unsafe fn string_stream_clear(stream: *mut StringStream) {
    spin_lock(&mut (*stream).lock);
    // list_for_each_entry_safe(frag_container, frag_container_safe, &stream->fragments, node)
    let mut entry = (*stream).fragments.next;
    while entry != &mut (*stream).fragments as *mut ListHead {
        let next = (*entry).next;
        let frag_container = entry as *mut StringStreamFragment;
        string_stream_fragment_destroy(frag_container);
        entry = next;
    }
    (*stream).length = 0;
    spin_unlock(&mut (*stream).lock);
}

pub unsafe fn string_stream_get_string(stream: *mut StringStream) -> *mut c_char {
    let buf_len = ((*stream).length + 1) as usize;
    let buf = kzalloc(buf_len, (*stream).gfp) as *mut c_char;
    if buf.is_null() {
        return core::ptr::null_mut();
    }
    let mut sb = MaybeUninit::<SeqBuf>::uninit();
    seq_buf_init(sb.as_mut_ptr(), buf, buf_len);
    spin_lock(&mut (*stream).lock);
    let mut entry = (*stream).fragments.next;
    while entry != &mut (*stream).fragments as *mut ListHead {
        seq_buf_puts(sb.as_mut_ptr(), (*(entry as *mut StringStreamFragment)).fragment);
        entry = (*entry).next;
    }
    spin_unlock(&mut (*stream).lock);
    buf
}

pub unsafe fn string_stream_append(stream: *mut StringStream, other: *mut StringStream) -> c_int {
    let other_content = string_stream_get_string(other);
    if other_content.is_null() {
        return -ENOMEM as c_int;
    }
    let ret = string_stream_add(stream, other_content);
    kfree(other_content as *mut c_void);
    ret
}

pub unsafe fn string_stream_is_empty(stream: *mut StringStream) -> bool {
    list_empty(&(*stream).fragments)
}

pub unsafe fn alloc_string_stream(gfp: GfpT) -> *mut StringStream {
    let stream = kzalloc(core::mem::size_of::<StringStream>(), gfp) as *mut StringStream;
    if stream.is_null() {
        return (-ENOMEM) as *mut StringStream;
    }
    (*stream).gfp = gfp;
    init_list_head(&mut (*stream).fragments);
    spin_lock_init(&mut (*stream).lock);
    stream
}

pub unsafe fn string_stream_destroy(stream: *mut StringStream) {
    if stream.is_null() || (stream as isize) == (-ENOMEM) {
        return;
    }
    string_stream_clear(stream);
    kfree(stream as *mut c_void);
}

unsafe extern "C" fn resource_free_string_stream(p: *mut c_void) {
    string_stream_destroy(p as *mut StringStream);
}

pub unsafe fn kunit_alloc_string_stream(test: *mut Kunit, gfp: GfpT) -> *mut StringStream {
    let stream = alloc_string_stream(gfp);
    if (stream as isize) == (-ENOMEM) {
        return stream;
    }
    if kunit_add_action_or_reset(test, Some(resource_free_string_stream), stream as *mut c_void) != 0 {
        return (-ENOMEM) as *mut StringStream;
    }
    stream
}

pub unsafe fn kunit_free_string_stream(test: *mut Kunit, stream: *mut StringStream) {
    kunit_release_action(test, Some(resource_free_string_stream), stream as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
