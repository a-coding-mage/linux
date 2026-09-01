// SPDX-License-Identifier: GPL-2.0

// C dependency: #include <babeltrace2-ctf-writer/writer.h>

use std::ffi::c_void;

unsafe extern "C" {
    fn bt_ctf_stream_class_get_packet_context_type(stream_class: *mut c_void);
}

fn main() {
    unsafe {
        bt_ctf_stream_class_get_packet_context_type(0 as *mut c_void);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
