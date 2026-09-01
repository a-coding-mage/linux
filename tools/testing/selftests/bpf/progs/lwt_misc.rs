// SPDX-License-Identifier: GPL-2.0

// C source dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

extern "C" {
    fn bpf_lwt_push_encap(
        skb: *mut __sk_buff,
        typ: u32,
        hdr: *mut core::ffi::c_void,
        len: u32,
    ) -> i64;
}

const BPF_LWT_ENCAP_IP: u32 = 2;

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iphdr {
    pub ihl_version: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

impl iphdr {
    #[inline]
    unsafe fn set_ihl(&mut self, value: u8) {
        self.ihl_version = (self.ihl_version & 0xf0) | (value & 0x0f);
    }

    #[inline]
    unsafe fn set_version(&mut self, value: u8) {
        self.ihl_version = (self.ihl_version & 0x0f) | ((value & 0x0f) << 4);
    }
}

// SEC("lwt_xmit")
// __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_missing_dst(skb: *mut __sk_buff) -> i32 {
    let mut iph: iphdr = core::mem::zeroed();

    iph.set_ihl(5);
    iph.set_version(4);

    bpf_lwt_push_encap(
        skb,
        BPF_LWT_ENCAP_IP,
        &mut iph as *mut iphdr as *mut core::ffi::c_void,
        core::mem::size_of::<iphdr>() as u32,
    );

    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
