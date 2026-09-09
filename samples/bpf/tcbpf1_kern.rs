// Translated from tcbpf1_kern.c.
// The original source includes Linux UAPI/BPF headers and bpf_legacy.h;
// their declarations are supplied by the surrounding build environment.

pub const KBUILD_MODNAME: &str = "foo";

#[repr(C)]
pub struct __sk_buff {
    pub ifindex: u32,
}

extern "C" {
    pub fn bpf_skb_store_bytes(
        skb: *mut __sk_buff,
        offset: u32,
        from: *const core::ffi::c_void,
        len: u32,
        flags: u64,
    ) -> i64;
    pub fn bpf_l3_csum_replace(
        skb: *mut __sk_buff,
        offset: u32,
        from: u64,
        to: u64,
        size: u64,
    ) -> i64;
    pub fn bpf_l4_csum_replace(
        skb: *mut __sk_buff,
        offset: u32,
        from: u64,
        to: u64,
        flags: u64,
    ) -> i64;
    pub fn bpf_redirect(ifindex: u32, flags: u64) -> i32;
    pub fn bpf_clone_redirect(skb: *mut __sk_buff, ifindex: u32, flags: u64) -> i64;
    pub fn load_byte(skb: *mut __sk_buff, offset: u32) -> u8;
    pub fn load_half(skb: *mut __sk_buff, offset: u32) -> u16;
    pub fn load_word(skb: *mut __sk_buff, offset: u32) -> u32;
    pub fn htons(value: u16) -> u16;
}

pub const ETH_ALEN: u32 = 6;
pub const ETH_HLEN: u32 = 14;
pub const IPPROTO_TCP: u8 = 6;
pub const TC_ACT_SHOT: i32 = 2;

// offsetof(struct iphdr, check), offsetof(struct iphdr, tos),
// offsetof(struct tcphdr, check), offsetof(struct iphdr, saddr), and
// offsetof(struct tcphdr, dest), respectively, from the included UAPI types.
pub const IP_CSUM_OFF: u32 = ETH_HLEN + 10;
pub const TOS_OFF: u32 = ETH_HLEN + 1;
pub const TCP_CSUM_OFF: u32 = ETH_HLEN + 20 + 16;
pub const IP_SRC_OFF: u32 = ETH_HLEN + 12;
pub const TCP_DPORT_OFF: u32 = ETH_HLEN + 20 + 2;
pub const IS_PSEUDO: u64 = 0x10;

#[inline]
pub unsafe fn set_dst_mac(skb: *mut __sk_buff, mac: *mut i8) {
    bpf_skb_store_bytes(
        skb,
        0,
        mac as *const core::ffi::c_void,
        ETH_ALEN,
        1,
    );
}

#[inline]
pub unsafe fn set_ip_tos(skb: *mut __sk_buff, new_tos: u8) {
    let old_tos: u8 = load_byte(skb, TOS_OFF);

    bpf_l3_csum_replace(skb, IP_CSUM_OFF, htons(old_tos as u16) as u64, htons(new_tos as u16) as u64, 2);
    bpf_skb_store_bytes(
        skb,
        TOS_OFF,
        (&new_tos as *const u8).cast::<core::ffi::c_void>(),
        core::mem::size_of::<u8>() as u32,
        0,
    );
}

#[inline]
pub unsafe fn set_tcp_ip_src(skb: *mut __sk_buff, new_ip: u32) {
    let old_ip: u32 = load_word(skb, IP_SRC_OFF).swap_bytes();

    bpf_l4_csum_replace(skb, TCP_CSUM_OFF, old_ip as u64, new_ip as u64, IS_PSEUDO | core::mem::size_of::<u32>() as u64);
    bpf_l3_csum_replace(skb, IP_CSUM_OFF, old_ip as u64, new_ip as u64, core::mem::size_of::<u32>() as u64);
    bpf_skb_store_bytes(
        skb,
        IP_SRC_OFF,
        (&new_ip as *const u32).cast::<core::ffi::c_void>(),
        core::mem::size_of::<u32>() as u32,
        0,
    );
}

#[inline]
pub unsafe fn set_tcp_dest_port(skb: *mut __sk_buff, new_port: u16) {
    let old_port: u16 = htons(load_half(skb, TCP_DPORT_OFF));

    bpf_l4_csum_replace(skb, TCP_CSUM_OFF, old_port as u64, new_port as u64, core::mem::size_of::<u16>() as u64);
    bpf_skb_store_bytes(
        skb,
        TCP_DPORT_OFF,
        (&new_port as *const u16).cast::<core::ffi::c_void>(),
        core::mem::size_of::<u16>() as u32,
        0,
    );
}

#[no_mangle]
#[link_section = "classifier"]
pub unsafe extern "C" fn bpf_prog1(skb: *mut __sk_buff) -> i32 {
    let proto: u8 = load_byte(skb, ETH_HLEN + 9);

    if proto == IPPROTO_TCP {
        set_ip_tos(skb, 8);
        set_tcp_ip_src(skb, 0x0A010101);
        set_tcp_dest_port(skb, 5001);
    }

    0
}

#[no_mangle]
#[link_section = "redirect_xmit"]
pub unsafe extern "C" fn _redirect_xmit(skb: *mut __sk_buff) -> i32 {
    bpf_redirect((*skb).ifindex.wrapping_add(1), 0)
}

#[no_mangle]
#[link_section = "redirect_recv"]
pub unsafe extern "C" fn _redirect_recv(skb: *mut __sk_buff) -> i32 {
    bpf_redirect((*skb).ifindex.wrapping_add(1), 1)
}

#[no_mangle]
#[link_section = "clone_redirect_xmit"]
pub unsafe extern "C" fn _clone_redirect_xmit(skb: *mut __sk_buff) -> i32 {
    bpf_clone_redirect(skb, (*skb).ifindex.wrapping_add(1), 0);
    TC_ACT_SHOT
}

#[no_mangle]
#[link_section = "clone_redirect_recv"]
pub unsafe extern "C" fn _clone_redirect_recv(skb: *mut __sk_buff) -> i32 {
    bpf_clone_redirect(skb, (*skb).ifindex.wrapping_add(1), 1);
    TC_ACT_SHOT
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
