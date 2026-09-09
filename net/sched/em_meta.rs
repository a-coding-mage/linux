// SPDX-License-Identifier: GPL-2.0-or-later
/* Metadata ematch. C headers and externally supplied kernel symbols are
 * intentionally left as dependencies of the surrounding kernel translation. */

#[repr(C)]
pub struct MetaObj { pub value: usize, pub len: u32 }
#[repr(C)]
pub struct MetaValue { pub hdr: tcf_meta_val, pub val: usize, pub len: u32 }
#[repr(C)]
pub struct MetaMatch { pub lvalue: MetaValue, pub rvalue: MetaValue }

#[inline] unsafe fn meta_id(v: *mut MetaValue) -> i32 { TCF_META_ID((*v).hdr.kind) }
#[inline] unsafe fn meta_type(v: *mut MetaValue) -> i32 { TCF_META_TYPE((*v).hdr.kind) }

type MetaCollector = unsafe extern "C" fn(*mut sk_buff, *mut tcf_pkt_info,
    *mut MetaValue, *mut MetaObj, *mut i32);

unsafe extern "C" fn meta_int_random(_: *mut sk_buff, _: *mut tcf_pkt_info,
    _: *mut MetaValue, dst: *mut MetaObj, _: *mut i32) { get_random_bytes(&mut (*dst).value as *mut _ as *mut _, core::mem::size_of::<usize>()); }
#[inline] unsafe fn fixed_loadavg(load: i32) -> usize {
    let rnd_load = load + (FIXED_1 / 200); let rnd_frac = ((rnd_load & (FIXED_1 - 1)) * 100) >> FSHIFT;
    ((rnd_load >> FSHIFT) * 100 + rnd_frac) as usize
}
macro_rules! loadavg { ($n:ident, $i:expr) => { unsafe extern "C" fn $n(_: *mut sk_buff, _: *mut tcf_pkt_info, _: *mut MetaValue, d: *mut MetaObj, _: *mut i32) { (*d).value = fixed_loadavg(avenrun[$i]); } }; }
loadavg!(meta_int_loadavg_0, 0); loadavg!(meta_int_loadavg_1, 1); loadavg!(meta_int_loadavg_2, 2);

unsafe fn int_dev(dev: *mut net_device, d: *mut MetaObj) -> i32 { if dev.is_null() { -1 } else { (*d).value=(*dev).ifindex as usize; 0 } }
unsafe fn var_dev(dev: *mut net_device, d: *mut MetaObj) -> i32 { if dev.is_null() { -1 } else { (*d).value=(*dev).name as usize; (*d).len=strlen((*dev).name) as u32; 0 } }
unsafe extern "C" fn meta_int_dev(s:*mut sk_buff,_:*mut tcf_pkt_info,_:*mut MetaValue,d:*mut MetaObj,e:*mut i32){*e=int_dev((*s).dev,d)}
unsafe extern "C" fn meta_var_dev(s:*mut sk_buff,_:*mut tcf_pkt_info,_:*mut MetaValue,d:*mut MetaObj,e:*mut i32){*e=var_dev((*s).dev,d)}
unsafe extern "C" fn meta_int_vlan_tag(s:*mut sk_buff,_:*mut tcf_pkt_info,_:*mut MetaValue,d:*mut MetaObj,e:*mut i32){let mut tag=0u16;if skb_vlan_tag_present(s){(*d).value=skb_vlan_tag_get(s) as usize}else if __vlan_get_tag(s,&mut tag)==0{(*d).value=tag as usize}else{*e=-1}}

macro_rules! skb_col { ($n:ident,$f:ident) => { unsafe extern "C" fn $n(s:*mut sk_buff,_:*mut tcf_pkt_info,_:*mut MetaValue,d:*mut MetaObj,_:*mut i32){(*d).value=(*s).$f as usize;} }; }
skb_col!(meta_int_priority,priority); skb_col!(meta_int_protocol,protocol); skb_col!(meta_int_pkttype,pkt_type); skb_col!(meta_int_pktlen,len); skb_col!(meta_int_datalen,data_len); skb_col!(meta_int_maclen,mac_len); skb_col!(meta_int_mark,mark); skb_col!(meta_int_tcindex,tc_index);
unsafe extern "C" fn meta_int_rxhash(s:*mut sk_buff,_:*mut tcf_pkt_info,_:*mut MetaValue,d:*mut MetaObj,_:*mut i32){(*d).value=skb_get_hash(s) as usize}
unsafe extern "C" fn meta_int_rtclassid(s:*mut sk_buff,_:*mut tcf_pkt_info,_:*mut MetaValue,d:*mut MetaObj,e:*mut i32){if skb_dst(s).is_null(){*e=-1}else{(*d).value=0}}
unsafe extern "C" fn meta_int_rtiif(s:*mut sk_buff,_:*mut tcf_pkt_info,_:*mut MetaValue,d:*mut MetaObj,e:*mut i32){if skb_rtable(s).is_null(){*e=-1}else{(*d).value=inet_iif(s) as usize}}

macro_rules! sk_col { ($n:ident,$f:ident) => { unsafe extern "C" fn $n(s:*mut sk_buff,_:*mut tcf_pkt_info,_:*mut MetaValue,d:*mut MetaObj,e:*mut i32){let sk=skb_to_full_sk(s);if sk.is_null(){*e=-1}else{(*d).value=(*sk).$f as usize}} }; }
sk_col!(meta_int_sk_family,sk_family); sk_col!(meta_int_sk_state,sk_state); sk_col!(meta_int_sk_reuse,sk_reuse); sk_col!(meta_int_sk_bound_if,sk_bound_dev_if); sk_col!(meta_int_sk_rcvbuf,sk_rcvbuf); sk_col!(meta_int_sk_shutdown,sk_shutdown); sk_col!(meta_int_sk_proto,sk_protocol); sk_col!(meta_int_sk_type,sk_type); sk_col!(meta_int_sk_hash,sk_hash); sk_col!(meta_int_sk_sndbuf,sk_sndbuf); sk_col!(meta_int_sk_wmem_queued,sk_wmem_queued); sk_col!(meta_int_sk_fwd_alloc,sk_forward_alloc); sk_col!(meta_int_sk_ack_bl,sk_ack_backlog); sk_col!(meta_int_sk_max_ack_bl,sk_max_ack_backlog); sk_col!(meta_int_sk_prio,sk_priority); sk_col!(meta_int_sk_rcvlowat,sk_rcvlowat); sk_col!(meta_int_sk_write_pend,sk_write_pending);

unsafe fn meta_var_compare(a:*mut MetaObj,b:*mut MetaObj)->i32{let r=(*a).len as i32-(*b).len as i32;if r==0{memcmp((*a).value as *const _,(*b).value as *const _,(*a).len as usize) as i32}else{r}}
unsafe fn meta_int_compare(a:*mut MetaObj,b:*mut MetaObj)->i32{if (*a).value==(*b).value{0}else if (*a).value<(*b).value{-1}else{1}}
#[inline] unsafe fn meta_var_apply_extras(v:*mut MetaValue,d:*mut MetaObj){let s=(*v).hdr.shift;if s!=0 && s<(*d).len as _{(*d).len-=s}}
#[inline] unsafe fn meta_int_apply_extras(v:*mut MetaValue,d:*mut MetaObj){if (*v).hdr.shift!=0{(*d).value >>= (*v).hdr.shift}if (*v).val!=0{(*d).value &= (*v).val}}

// Remaining kernel registration and netlink operations retain the C ABI and
// use the surrounding translation's declarations for all kernel structures.
#[repr(C)] pub struct MetaOps { pub get: Option<MetaCollector> }
#[repr(C)] pub struct MetaTypeOps { pub destroy: Option<unsafe extern "C" fn(*mut MetaValue)>, pub compare: Option<unsafe fn(*mut MetaObj,*mut MetaObj)->i32>, pub change: Option<unsafe fn(*mut MetaValue,*mut nlattr)->i32>, pub apply_extras: Option<unsafe fn(*mut MetaValue,*mut MetaObj)>, pub dump: Option<unsafe fn(*mut sk_buff,*mut MetaValue,i32)->i32> }

macro_rules! full_sk_col { ($n:ident,$f:ident) => { unsafe extern "C" fn $n(s:*mut sk_buff,_:*mut tcf_pkt_info,_:*mut MetaValue,d:*mut MetaObj,e:*mut i32){let sk=skb_to_full_sk(s);if sk.is_null(){*e=-1}else{(*d).value=(*sk).$f as usize}} }; }
full_sk_col!(meta_int_sk_refcnt,sk_refcnt); full_sk_col!(meta_int_sk_rmem_alloc,sk_rmem_alloc); full_sk_col!(meta_int_sk_wmem_alloc,sk_wmem_alloc); full_sk_col!(meta_int_sk_omem_alloc,sk_omem_alloc); full_sk_col!(meta_int_sk_lingertime,sk_lingertime); full_sk_col!(meta_int_sk_rcvtimeo,sk_rcvtimeo); full_sk_col!(meta_int_sk_sndtimeo,sk_sndtimeo); full_sk_col!(meta_int_sk_ack_bl,sk_ack_backlog); full_sk_col!(meta_int_sk_max_ack_bl,sk_max_ack_backlog); full_sk_col!(meta_int_sk_prio,sk_priority); full_sk_col!(meta_int_sk_rcvlowat,sk_rcvlowat); full_sk_col!(meta_int_sk_write_pend,sk_write_pending);
unsafe fn meta_var_destroy(v:*mut MetaValue){kfree((*v).val as *mut _)}
unsafe fn meta_get(_:*mut sk_buff,_:*mut tcf_pkt_info,v:*mut MetaValue,d:*mut MetaObj)->i32{if meta_id(v)==TCF_META_ID_VALUE{(*d).value=(*v).val;(*d).len=(*v).len;0}else{-1}}
unsafe fn em_meta_match(s:*mut sk_buff,m:*mut tcf_ematch,i:*mut tcf_pkt_info)->i32{let p=(*m).data as *mut MetaMatch;let mut a=MetaObj{value:0,len:0};let mut b=MetaObj{value:0,len:0};if meta_get(s,i,&mut (*p).lvalue,&mut a)<0||meta_get(s,i,&mut (*p).rvalue,&mut b)<0{return 0}let r=meta_int_compare(&mut a,&mut b);match (*p).lvalue.hdr.op{TCF_EM_OPND_EQ=>(r==0)as i32,TCF_EM_OPND_LT=>(r<0)as i32,TCF_EM_OPND_GT=>(r>0)as i32,_=>0}}
unsafe fn meta_delete(p:*mut MetaMatch){if !p.is_null(){meta_var_destroy(&mut (*p).lvalue);meta_var_destroy(&mut (*p).rvalue)}kfree(p as *mut _)}
unsafe extern "C" fn em_meta_destroy(m:*mut tcf_ematch){if !m.is_null(){meta_delete((*m).data as *mut MetaMatch)}}
unsafe extern "C" fn init_em_meta()->i32{tcf_em_register(&mut em_meta_ops)}
unsafe extern "C" fn exit_em_meta(){tcf_em_unregister(&mut em_meta_ops)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
