// SPDX-License-Identifier: GPL-2.0
// BPF_NO_PRESERVE_ACCESS_INDEX is defined for the C build before including
// vmlinux.h and BPF helper headers.

const IPPROTO_TCP: u8 = 6;
const BPF_F_RECOMPUTE_CSUM: u64 = 1;
const ETH_IPV4_TCP_SIZE: usize =
    14 + core::mem::size_of::<iphdr>() + core::mem::size_of::<tcphdr>();

#[repr(C)]
pub struct __sk_buff {
    pub data: u32,
    pub data_end: u32,
}

#[repr(C)]
pub struct ethhdr {
    _unused: [u8; 14],
}

#[repr(C)]
pub struct iphdr {
    pub _bitfield_1: u8,
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

#[repr(C)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
    pub seq: u32,
    pub ack_seq: u32,
    pub _bitfield_1: u16,
    pub window: u16,
    pub check: u16,
    pub urg_ptr: u16,
}

unsafe extern "C" {
    fn bpf_skb_store_bytes(
        skb: *mut __sk_buff,
        offset: u32,
        from: *const core::ffi::c_void,
        len: u32,
        flags: u64,
    ) -> i64;
}

#[inline(always)]
unsafe fn skb_shorter(skb: *mut __sk_buff, len: usize) -> bool {
    unsafe {
        ((*skb).data as usize).wrapping_add(len) as *mut core::ffi::c_void
            > (*skb).data_end as usize as *mut core::ffi::c_void
    }
}

#[inline(always)]
unsafe fn get_iphdr(skb: *mut __sk_buff) -> *mut iphdr {
    let mut ip: *mut iphdr = core::ptr::null_mut();
    let eth: *mut ethhdr;

    unsafe {
        if skb_shorter(skb, ETH_IPV4_TCP_SIZE) {
            return ip;
        }

        eth = (*skb).data as usize as *mut ethhdr;
        ip = eth.add(1) as *mut iphdr;
    }

    ip
}

// SEC("tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_prog(skb: *mut __sk_buff) -> i32 {
    let mut ip: *mut iphdr = core::ptr::null_mut();
    let tcp: *mut tcphdr;
    let mut proto: u8 = 0;
    let urg_ptr: i32;
    let offset: u32;

    unsafe {
        ip = get_iphdr(skb);
        if ip.is_null() {
            return -1;
        }

        proto = (*ip).protocol;

        if proto != IPPROTO_TCP {
            return -1;
        }

        tcp = ip.add(1) as *mut tcphdr;
        if (*tcp).dest != 0 {
            return -1;
        }
        if tcp.is_null() {
            return -1;
        }

        urg_ptr = (*tcp).urg_ptr as i32;

        /* Checksum validation part */
        proto = proto.wrapping_add(1);
        offset = (core::mem::size_of::<ethhdr>() + core::mem::offset_of!(iphdr, protocol)) as u32;
        bpf_skb_store_bytes(
            skb,
            offset,
            &proto as *const u8 as *const core::ffi::c_void,
            core::mem::size_of_val(&proto) as u32,
            BPF_F_RECOMPUTE_CSUM,
        );

        return urg_ptr;
    }
}

// SEC("license")
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
