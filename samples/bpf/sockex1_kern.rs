// Dependencies supplied by the Linux BPF headers and bpf_legacy.h.

use core::ffi::c_void;

pub const ETH_HLEN: usize = 14;
pub const PACKET_OUTGOING: u32 = 4;

#[repr(C)]
pub struct __sk_buff {
    // The complete layout is supplied by the kernel BPF headers.
    pub pkt_type: u32,
    pub len: u32,
}

#[repr(C)]
pub struct iphdr {
    // Only the field used by this translation is represented here.
    pub protocol: u8,
}

#[repr(C)]
pub struct MyMap {
    pub map_type: u32,
    pub max_entries: u32,
}

// Equivalent to the anonymous BPF map declaration and SEC(".maps").
#[no_mangle]
#[link_section = ".maps"]
pub static mut my_map: MyMap = MyMap {
    map_type: 2, // BPF_MAP_TYPE_ARRAY
    max_entries: 256,
};

extern "C" {
    pub fn load_byte(skb: *mut __sk_buff, offset: usize) -> i32;
    pub fn bpf_map_lookup_elem(map: *mut MyMap, key: *const i32) -> *mut i64;
}

#[no_mangle]
#[link_section = "socket1"]
pub unsafe extern "C" fn bpf_prog1(skb: *mut __sk_buff) -> i32 {
    let index: i32 = load_byte(
        skb,
        ETH_HLEN + core::mem::offset_of!(iphdr, protocol),
    );
    let value: *mut i64;

    if (*skb).pkt_type != PACKET_OUTGOING {
        return 0;
    }

    value = bpf_map_lookup_elem(&raw mut my_map, &index);
    if !value.is_null() {
        // Equivalent to __sync_fetch_and_add(value, skb->len).
        core::ptr::write_volatile(
            value,
            core::ptr::read_volatile(value).wrapping_add((*skb).len as i64),
        );
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
