// SPDX-License-Identifier: GPL-2.0
// C source defined BPF_NO_KFUNC_PROTOTYPES and included vmlinux.h,
// bpf_tracing_net.h, and bpf_helpers.h for __sk_buff, SEC, TC_ACT_*,
// __ksym, and BPF helper/kfunc declarations.

#[repr(C)]
pub struct bpf_xfrm_info___local {
    pub if_id: u32,
    pub link: i32,
}
// C used __attribute__((preserve_access_index)) on bpf_xfrm_info___local.

#[no_mangle]
pub static mut req_if_id: u32 = 0;

#[no_mangle]
pub static mut resp_if_id: u32 = 0;

unsafe extern "C" {
    #[link_name = "bpf_skb_set_xfrm_info"]
    pub fn bpf_skb_set_xfrm_info(
        skb_ctx: *mut __sk_buff,
        from: *const bpf_xfrm_info___local,
    ) -> i32;

    #[link_name = "bpf_skb_get_xfrm_info"]
    pub fn bpf_skb_get_xfrm_info(
        skb_ctx: *mut __sk_buff,
        to: *mut bpf_xfrm_info___local,
    ) -> i32;
}
// C declarations above were marked __ksym.

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn set_xfrm_info(skb: *mut __sk_buff) -> i32 {
    let info = bpf_xfrm_info___local {
        if_id: req_if_id,
        link: 0,
    };

    if bpf_skb_set_xfrm_info(skb, &info as *const bpf_xfrm_info___local) != 0 {
        TC_ACT_SHOT
    } else {
        TC_ACT_UNSPEC
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn get_xfrm_info(skb: *mut __sk_buff) -> i32 {
    let mut info = bpf_xfrm_info___local { if_id: 0, link: 0 };

    if bpf_skb_get_xfrm_info(skb, &mut info as *mut bpf_xfrm_info___local) < 0 {
        return TC_ACT_SHOT;
    }

    resp_if_id = info.if_id;

    TC_ACT_UNSPEC
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
