// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <trace-seq.h>

#[repr(C)]
pub struct trace_seq {
    pub state: ::std::os::raw::c_int,
}

pub const TRACE_SEQ__GOOD: ::std::os::raw::c_int = 0;

unsafe extern "C" {
    pub fn trace_seq_init(s: *mut trace_seq);
    pub fn trace_seq_destroy(s: *mut trace_seq);
}

fn main() -> ::std::os::raw::c_int {
    let mut rv: ::std::os::raw::c_int = 0;
    let mut s: trace_seq = unsafe { ::std::mem::zeroed() };

    unsafe {
        trace_seq_init(&mut s);
    }
    rv += (!(s.state == TRACE_SEQ__GOOD)) as ::std::os::raw::c_int;
    unsafe {
        trace_seq_destroy(&mut s);
    }

    rv
}
