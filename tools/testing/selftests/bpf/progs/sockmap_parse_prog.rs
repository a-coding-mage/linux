// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

#[no_mangle]
#[link_section = "sk_skb1"]
pub unsafe extern "C" fn bpf_prog1(skb: *mut __sk_buff) -> i32 {
    unsafe { (*skb).len as i32 }
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
