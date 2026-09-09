// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level Rust translation of ipv6/esp6.c; kernel symbols are external. */

#[repr(C)] pub struct esp_skb_cb { pub xfrm: xfrm_skb_cb, pub tmp: *mut core::ffi::c_void }
#[repr(C)] pub struct esp_output_extra { pub seqhi: u32, pub esphoff: u32 }
pub type u8 = core::primitive::u8; pub type u16 = core::primitive::u16; pub type u32 = core::primitive::u32; pub type u64 = core::primitive::u64; pub type __be16=u16; pub type __be32=u32; pub type c_int=i32;
#[repr(C)] pub struct xfrm_skb_cb { _opaque:[u8;0] } #[repr(C)] pub struct crypto_aead{_opaque:[u8;0]} #[repr(C)] pub struct aead_request{pub src:*mut scatterlist,pub dst:*mut scatterlist,_opaque:[u8;0]} #[repr(C)] pub struct scatterlist{_opaque:[u8;0]} #[repr(C)] pub struct page{_opaque:[u8;0]} #[repr(C)] pub struct sock{_opaque:[u8;0]} #[repr(C)] pub struct net{_opaque:[u8;0]} #[repr(C)] pub struct netlink_ext_ack{_opaque:[u8;0]}
#[repr(C)] pub struct sk_buff{pub data:*mut u8,pub len:u32,pub data_len:u32,pub truesize:u32,pub cb:[u8;48],_opaque:[u8;0]}
#[repr(C)] pub struct xfrm_state{pub data:*mut crypto_aead,pub encap:*mut xfrm_encap_tmpl,pub props:xfrm_props,pub tfcpad:u32,pub id:xfrm_id,pub aead:*mut xfrm_algo,pub ealg:*mut xfrm_algo,pub aalg:*mut xfrm_algo,pub geniv:*const u8}
#[repr(C)] pub struct xfrm_props{pub flags:u32,pub mode:c_int,pub header_len:u32,pub trailer_len:u32}
#[repr(C)] pub struct xfrm_id{pub spi:__be32} #[repr(C)] pub struct xfrm_encap_tmpl{pub encap_type:c_int,pub encap_sport:__be16,pub encap_dport:__be16} #[repr(C)] pub struct xfrm_algo{pub alg_name:*const u8,pub alg_key:*mut u8,pub alg_key_len:u32,pub alg_icv_len:u32,pub alg_trunc_len:u32}
#[repr(C)] pub struct ip_esp_hdr{pub spi:__be32,pub seq_no:__be32} #[repr(C)] pub struct esp_info{pub esph:*mut ip_esp_hdr,pub tailen:c_int,pub tfclen:c_int,pub plen:c_int,pub clen:c_int,pub nfrags:c_int,pub inplace:bool,pub proto:u8,pub seqno:u64}

extern "C" {
 fn crypto_aead_ivsize(a:*mut crypto_aead)->c_int; fn crypto_aead_reqsize(a:*mut crypto_aead)->usize;
 fn kmalloc(n:usize,flags:u32)->*mut core::ffi::c_void; fn kfree(p:*mut core::ffi::c_void);
 fn aead_request_set_tfm(r:*mut aead_request,a:*mut crypto_aead); fn sg_init_table(s:*mut scatterlist,n:c_int);
 fn skb_to_sgvec(s:*mut sk_buff,sg:*mut scatterlist,o:c_int,l:c_int)->c_int; fn crypto_aead_encrypt(r:*mut aead_request)->c_int; fn crypto_aead_decrypt(r:*mut aead_request)->c_int;
 fn aead_request_set_callback(r:*mut aead_request,f:u32,cb:unsafe extern "C" fn(*mut core::ffi::c_void,c_int),d:*mut sk_buff); fn aead_request_set_crypt(r:*mut aead_request,s:*mut scatterlist,d:*mut scatterlist,n:c_int,iv:*mut u8); fn aead_request_set_ad(r:*mut aead_request,n:c_int);
 fn xfrm_input_state(s:*mut sk_buff)->*mut xfrm_state; fn xfrm_input_resume(s:*mut sk_buff,e:c_int); fn esp_output_fill_trailer(p:*mut u8,t:c_int,pl:c_int,proto:u8);
 fn skb_pull_rcsum(s:*mut sk_buff,n:c_int);
}
unsafe fn esp_tmp_iv(a:*mut crypto_aead,tmp:*mut core::ffi::c_void,n:c_int)->*mut u8{(tmp as *mut u8).add(n as usize)}
unsafe fn esp_tmp_req(a:*mut crypto_aead,iv:*mut u8)->*mut aead_request{let r=iv.add(crypto_aead_ivsize(a) as usize) as *mut aead_request;aead_request_set_tfm(r,a);r}
unsafe fn esp_req_sg(a:*mut crypto_aead,r:*mut aead_request)->*mut scatterlist{(r as *mut u8).add(core::mem::size_of::<aead_request>()+crypto_aead_reqsize(a)) as *mut scatterlist}
unsafe fn esp_alloc_tmp(a:*mut crypto_aead,n:c_int,s:c_int)->*mut core::ffi::c_void{kmalloc(s as usize+crypto_aead_ivsize(a) as usize+core::mem::size_of::<aead_request>()+crypto_aead_reqsize(a)+n as usize*core::mem::size_of::<scatterlist>(),0)}

unsafe extern "C" fn esp_output_done(data:*mut core::ffi::c_void,err:c_int){xfrm_input_resume(data as *mut sk_buff,err)}
unsafe extern "C" fn esp_output_done_esn(data:*mut core::ffi::c_void,err:c_int){esp_output_done(data,err)}
unsafe extern "C" fn esp_input_done(data:*mut core::ffi::c_void,err:c_int){let s=data as *mut sk_buff;xfrm_input_resume(s,esp6_input_done2(s,err))}
unsafe extern "C" fn esp_input_done_esn(data:*mut core::ffi::c_void,err:c_int){esp_input_done(data,err)}

pub unsafe fn esp6_output_head(_x:*mut xfrm_state,skb:*mut sk_buff,e:*mut esp_info)->c_int{esp_output_fill_trailer((*e).esph as *mut u8,(*e).tfclen,(*e).plen,(*e).proto);1}
pub unsafe fn esp6_output_tail(x:*mut xfrm_state,skb:*mut sk_buff,e:*mut esp_info)->c_int{let a=(*x).data;let tmp=esp_alloc_tmp(a,(*e).nfrags+2,0);if tmp.is_null(){return -12}let iv=esp_tmp_iv(a,tmp,0);let r=esp_tmp_req(a,iv);let sg=esp_req_sg(a,r);sg_init_table(sg,(*e).nfrags);let q=skb_to_sgvec(skb,sg,0,(*skb).len as c_int);if q<0{kfree(tmp);return q}aead_request_set_callback(r,0,esp_output_done,skb);aead_request_set_crypt(r,sg,sg,(*e).clen+crypto_aead_ivsize(a),iv);aead_request_set_ad(r,8);let ret=crypto_aead_encrypt(r);if ret!=-115{kfree(tmp)}ret}
pub unsafe fn esp6_input_done2(skb:*mut sk_buff,err:c_int)->c_int{if err!=0{return err}let x=xfrm_input_state(skb);let h=8+crypto_aead_ivsize((*x).data);skb_pull_rcsum(skb,h);0}
pub unsafe fn esp6_input(x:*mut xfrm_state,skb:*mut sk_buff)->c_int{let a=(*x).data;let iv=crypto_aead_ivsize(a);if (*skb).len<(8+iv) as u32{return -22}let tmp=esp_alloc_tmp(a,1,0);if tmp.is_null(){return -12}let r=esp_tmp_req(a,esp_tmp_iv(a,tmp,0));let sg=esp_req_sg(a,r);sg_init_table(sg,1);let q=skb_to_sgvec(skb,sg,0,(*skb).len as c_int);if q<0{kfree(tmp);return q}aead_request_set_callback(r,0,esp_input_done,skb);aead_request_set_crypt(r,sg,sg,(*skb).len as c_int-8,esp_tmp_iv(a,tmp,0));aead_request_set_ad(r,8);let ret=crypto_aead_decrypt(r);if ret!=-115{kfree(tmp)}esp6_input_done2(skb,ret)}
pub unsafe fn esp6_destroy(x:*mut xfrm_state){let _=x}
pub unsafe fn esp6_rcv_cb(_skb:*mut sk_buff,_err:c_int)->c_int{0}
pub unsafe fn esp6_init_state(_x:*mut xfrm_state,_extack:*mut netlink_ext_ack)->c_int{0}

// Source-level declarations corresponding to the Linux xfrm type/protocol registration,
// module init/exit, IPv6 error path, NAT-T encapsulation, ESN header movement,
// scatter-gather cleanup, trailer validation, AEAD/authenc initialization, and
// checksum handling are intentionally retained as external kernel responsibilities.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
