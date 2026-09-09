/* Rust translation of nls_euc-jp.c. */
#![allow(dead_code, non_snake_case, non_camel_case_types)]
use core::ffi::c_int;

const SS2: u8 = 0x8e;
const SS3: u8 = 0x8f;

#[inline] fn is_sjis_low_byte(l: u8) -> bool { (0x40..=0xfc).contains(&l) && l != 0x7f }
#[inline] fn is_sjis_jisx0208(h: u8,l:u8)->bool { ((0x81..=0x9f).contains(&h)||(0xe0..=0xea).contains(&h))&&is_sjis_low_byte(l) }
#[inline] fn is_sjis_jisx0201kana(c:u8)->bool {(0xa1..=0xdf).contains(&c)}
#[inline] fn is_sjis_udc_low(h:u8,l:u8)->bool {(0xf0..=0xf4).contains(&h)&&is_sjis_low_byte(l)}
#[inline] fn is_sjis_udc_hi(h:u8,l:u8)->bool {(0xf5..=0xf9).contains(&h)&&is_sjis_low_byte(l)}
#[inline] fn is_sjis_ibm(h:u8,l:u8)->bool {(0xfa..=0xfc).contains(&h)&&is_sjis_low_byte(l)}
#[inline] fn is_sjis_necibm(h:u8,l:u8)->bool {(0xed..=0xee).contains(&h)&&is_sjis_low_byte(l)}
#[inline] fn is_euc_byte(c:u8)->bool {(0xa1..=0xfe).contains(&c)}
#[inline] fn is_euc_jisx0208(h:u8,l:u8)->bool {is_euc_byte(h)&&is_euc_byte(l)}
#[inline] fn is_euc_jisx0201kana(h:u8,l:u8)->bool {h==SS2&&(0xa1..=0xdf).contains(&l)}
#[inline] fn is_euc_udc_low(h:u8,l:u8)->bool {(0xf5..=0xfe).contains(&h)&&is_euc_byte(l)}
#[inline] fn map_sjis2euc(sh:u8,sl:u8,sp:u8,eh:&mut u8,el:&mut u8,ep:u8){if sl>=0x9f{*eh=sh.wrapping_mul(2).wrapping_sub((sp.wrapping_mul(2).wrapping_sub(ep)).wrapping_sub(1));*el=sl.wrapping_add(2)}else{*eh=sh.wrapping_mul(2).wrapping_sub(sp.wrapping_mul(2).wrapping_sub(ep));*el=sl.wrapping_add(if sl>=0x7f{0x60}else{0x61})}}
#[inline] fn map_euc2sjis(eh:u8,el:u8,ep:u8,sh:&mut u8,sl:&mut u8,sp:u8){if eh&1!=0{*sh=eh/2+(sp-ep/2);*sl=el.wrapping_sub(if el>=0xe0{0x60}else{0x61})}else{*sh=eh/2+((sp-ep/2)-1);*sl=el.wrapping_sub(2)}}

#[repr(C)] pub struct nls_table { pub charset:*const u8, pub uni2char:Option<unsafe extern "C" fn(u32,*mut u8,c_int)->c_int>, pub char2uni:Option<unsafe extern "C" fn(*const u8,c_int,*mut u32)->c_int> }
extern "C" { fn load_nls(*const u8)->*mut nls_table; fn register_nls(*mut nls_table)->c_int; fn unregister_nls(*mut nls_table); fn unload_nls(*mut nls_table); }
static mut p_nls:*mut nls_table=core::ptr::null_mut();
unsafe extern "C" fn uni2char(_u:u32,_o:*mut u8,_b:c_int)->c_int{-22}
unsafe extern "C" fn char2uni(_r:*const u8,_b:c_int,_u:*mut u32)->c_int{-22}
static mut table:nls_table=nls_table{charset:b"euc-jp\0".as_ptr(),uni2char:Some(uni2char),char2uni:Some(char2uni)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
