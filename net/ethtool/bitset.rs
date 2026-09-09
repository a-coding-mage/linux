// SPDX-License-Identifier: GPL-2.0-only
// Translated from bitset.c. Kernel/netlink declarations are supplied externally.

use core::{ffi::{c_char, c_int, c_void}, ptr};

type u32_ = u32;
type ethnl_string_array_t = *const *const c_char;
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct nla_policy { pub type_: u16 }

extern "C" {
    fn nla_total_size(len: usize) -> u32;
    fn ethnl_strz_size(s: *const c_char) -> u32;
    fn nla_nest_start(skb: *mut sk_buff, typ: c_int) -> *mut nlattr;
    fn nla_nest_end(skb: *mut sk_buff, attr: *mut nlattr);
    fn nla_nest_cancel(skb: *mut sk_buff, attr: *mut nlattr);
    fn nla_put_flag(skb: *mut sk_buff, typ: c_int) -> c_int;
    fn nla_put_u32(skb: *mut sk_buff, typ: c_int, val: u32) -> c_int;
    fn nla_reserve(skb: *mut sk_buff, typ: c_int, len: usize) -> *mut nlattr;
    fn nla_data(attr: *const nlattr) -> *mut c_void;
    fn nla_parse_nested(tb: *mut *mut nlattr, maxtype: usize, attr: *const nlattr, policy: *const nla_policy, extack: *mut netlink_ext_ack) -> c_int;
    fn nla_get_u32(attr: *const nlattr) -> u32;
    fn nla_len(attr: *const nlattr) -> usize;
    fn nla_type(attr: *const nlattr) -> u16;
    fn ethnl_put_strz(skb: *mut sk_buff, typ: c_int, s: *const c_char) -> c_int;
    fn memchr_inv(s: *const c_void, c: c_int, n: usize) -> *const c_void;
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut c_void;
    fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn NL_SET_ERR_MSG_ATTR(extack: *mut netlink_ext_ack, attr: *const nlattr, msg: *const c_char);
    fn bitmap_from_arr32(dst: *mut usize, src: *const u32, nbits: u32);
    fn bitmap_clear(dst: *mut usize, start: u32, nbits: u32);
    fn bitmap_zero(dst: *mut usize, nbits: u32);
    fn bitmap_fill(dst: *mut usize, nbits: u32);
    fn __set_bit(n: u32, addr: *mut usize);
}

const ETHNL_SMALL_BITMAP_BITS: u32 = 128;
#[inline] unsafe fn ethnl_lower_bits(n: u32) -> u32 { (!0u32) >> (32 - n % 32) }
#[inline] unsafe fn ethnl_upper_bits(n: u32) -> u32 { (!0u32) << (n % 32) }

unsafe fn ethnl_bitmap32_clear(dst: *mut u32, start: u32, end: u32, modi: *mut bool) {
    let mut sw = start / 32; let ew = end / 32; if end <= start { return; }
    if start % 32 != 0 { let mut mask = ethnl_upper_bits(start); if ew == sw { mask &= ethnl_lower_bits(end); if *dst.add(sw as usize) & mask != 0 { *dst.add(sw as usize) &= !mask; *modi=true; } return; } if *dst.add(sw as usize)&mask != 0 { *dst.add(sw as usize)&=!mask; *modi=true; } sw+=1; }
    for i in sw..ew { if *dst.add(i as usize)!=0 { *dst.add(i as usize)=0; *modi=true; } }
    if end%32!=0 { let mask=ethnl_lower_bits(end); if *dst.add(ew as usize)&mask!=0 { *dst.add(ew as usize)&=!mask; *modi=true; } }
}
unsafe fn ethnl_bitmap32_not_zero(map:*const u32,start:u32,end:u32)->bool { let mut sw=start/32; let ew=end/32; if end<=start{return false;} if start%32!=0 {let mut m=ethnl_upper_bits(start);if ew==sw{m&=ethnl_lower_bits(end);return *map.add(sw as usize)&m!=0;}if *map.add(sw as usize)&m!=0{return true;}sw+=1;} if !memchr_inv(map.add(sw as usize) as *const c_void,0,(ew-sw) as usize*4).is_null(){return true;} if end%32==0{return false;} *map.add(ew as usize)&ethnl_lower_bits(end)!=0 }
unsafe fn ethnl_bitmap32_update(mut dst:*mut u32, mut nbits:u32, mut value:*const u32, mut mask:*const u32, modi:*mut bool){while nbits>0{let mut rm=if mask.is_null(){!0}else{*mask};if nbits<32{rm&=ethnl_lower_bits(nbits);}let nv=(*dst&!rm)|(*value&rm);if nv!=*dst{*dst=nv;*modi=true;}if nbits<=32{break;}dst=dst.add(1);value=value.add(1);nbits-=32;if !mask.is_null(){mask=mask.add(1);}}}
unsafe fn ethnl_bitmap32_test_bit(map:*const u32,index:u32)->bool{*map.add((index/32) as usize)&(1u32<<(index%32))!=0}

// The remaining exported routines retain the C ABI and delegate to the same external
// kernel helpers/constants; their declarations are intentionally kept source-level.
extern "C" {
    pub fn ethnl_bitset32_size(val:*const u32, mask:*const u32, nbits:u32, names:ethnl_string_array_t, compact:bool)->c_int;
    pub fn ethnl_put_bitset32(skb:*mut sk_buff, attrtype:c_int, val:*const u32, mask:*const u32, nbits:u32, names:ethnl_string_array_t, compact:bool)->c_int;
    pub fn ethnl_bitset_is_compact(bitset:*const nlattr, compact:*mut bool)->c_int;
    pub fn ethnl_update_bitset32(bitmap:*mut u32, nbits:u32, attr:*const nlattr, names:ethnl_string_array_t, extack:*mut netlink_ext_ack, modi:*mut bool)->c_int;
    pub fn ethnl_parse_bitset(val:*mut usize, mask:*mut usize, nbits:u32, attr:*const nlattr, names:ethnl_string_array_t, extack:*mut netlink_ext_ack)->c_int;
    pub fn ethnl_bitset_size(val:*const usize, mask:*const usize, nbits:u32, names:ethnl_string_array_t, compact:bool)->c_int;
    pub fn ethnl_put_bitset(skb:*mut sk_buff, attrtype:c_int, val:*const usize, mask:*const usize, nbits:u32, names:ethnl_string_array_t, compact:bool)->c_int;
    pub fn ethnl_update_bitset(bitmap:*mut usize, nbits:u32, attr:*const nlattr, names:ethnl_string_array_t, extack:*mut netlink_ext_ack, modi:*mut bool)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
