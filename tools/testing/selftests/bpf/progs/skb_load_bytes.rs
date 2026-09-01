// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// <linux/bpf.h>
// <bpf/bpf_helpers.h>

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_skb_load_bytes(
        skb: *mut __sk_buff,
        offset: u32,
        to: *mut core::ffi::c_void,
        len: u32,
    ) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut load_offset: u32 = 0;

#[unsafe(no_mangle)]
pub static mut test_result: i32 = 0;

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn skb_process(skb: *mut __sk_buff) -> i32 {
    let mut buf: [u8; 16] = [0; 16];

    unsafe {
        test_result = bpf_skb_load_bytes(
            skb,
            load_offset,
            buf.as_mut_ptr() as *mut core::ffi::c_void,
            10,
        );
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
