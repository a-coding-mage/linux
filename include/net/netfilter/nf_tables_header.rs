/* SPDX-License-Identifier: GPL-2.0 */
#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Translation of net/netfilter/nf_tables.h.  Kernel dependencies are external. */
use core::{mem, ptr};
pub type u8 = core::primitive::u8; pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32; pub type u64 = core::primitive::u64;
pub type __be16 = u16; pub type __be32 = u32; pub type gfp_t = usize;
pub type bool_ = bool;

pub const NFT_MAX_HOOKS: usize = 5;
pub const NFT_JUMP_STACK_SIZE: usize = 16;
pub const NFT_PKTINFO_L4PROTO: u8 = 1; pub const NFT_PKTINFO_INNER: u8 = 2;
pub const NFT_PKTINFO_INNER_FULL: u8 = 4; pub const NFT_REG32_NUM: usize = 20;
pub const NFT_EXPR_MAXATTR: usize = 16; pub const NFT_SET_EXPR_MAX: usize = 2;
pub const NFT_NETDEVICE_MAX: usize = 256; pub const NFT_TRANS_GC_BATCHCOUNT: usize = 256;

#[repr(C)] pub struct module; #[repr(C)] pub struct sk_buff; #[repr(C)] pub struct sock;
#[repr(C)] pub struct net { pub nft: nft_net_state }
#[repr(C)] pub struct nft_net_state { pub gencursor: u32 }
#[repr(C)] pub struct net_device; #[repr(C)] pub struct nf_hook_state { pub sk:*mut sock, pub net:*mut net, pub hook:u32, pub pf:u8, pub in_:*const net_device, pub out:*const net_device }
#[repr(C)] pub struct nft_table; #[repr(C)] pub struct nft_chain; #[repr(C)] pub struct nft_set_ext;
#[repr(C)] pub struct nft_object; #[repr(C)] pub struct nft_flow_rule; #[repr(C)] pub struct nft_offload_ctx;
#[repr(C)] pub struct flow_stats; #[repr(C)] pub struct nf_flowtable; #[repr(C)] pub struct nf_flowtable_type;
#[repr(C)] pub struct nlaattr; #[repr(C)] pub struct nft_expr_info; #[repr(C)] pub struct nft_set;
#[repr(C)] pub struct nft_expr_ops; #[repr(C)] pub struct nft_object_ops; #[repr(C)] pub struct nft_rule_blob;
#[repr(C)] pub struct rcu_head; #[repr(C)] pub struct nf_hook_ops; #[repr(C)] pub struct nf_hookfn;
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct rhlist_head; #[repr(C)] pub struct rhltable; #[repr(C)] pub struct flow_block;
#[repr(C)] pub struct u64_stats_sync; #[repr(C)] pub struct nft_set_ext_type;
pub type refcount_t = u32; pub type atomic_t = u32; pub type possible_net_t = *mut net;
pub type nla_policy = u8; pub type nft_data_types = u32; pub type nft_registers = u32;
pub const NFT_REG_VERDICT:u32=0; pub const NFT_REG_1:u32=1; pub const NFT_REG_SIZE:usize=16;
pub const NFT_REG32_SIZE:usize=4; pub const NFT_DATA_VERDICT:u32=1; pub const NFT_DATA_VALUE:u32=0;
pub const NFT_DATA_VALUE_MAXLEN:usize=64; pub const NFT_BREAK:u32=0xffff_ffff;

#[repr(C)] pub struct nft_pktinfo { pub skb:*mut sk_buff, pub state:*const nf_hook_state, pub flags:u8, pub tprot:u8, pub ethertype:__be16, pub fragoff:u16, pub nhoff:u16, pub thoff:u16, pub inneroff:u16 }
#[inline] pub unsafe fn nft_sk(p:*const nft_pktinfo)->*mut sock { (*(*p).state).sk }
#[inline] pub unsafe fn nft_thoff(p:*const nft_pktinfo)->u32 { (*p).thoff as u32 }
#[inline] pub unsafe fn nft_net(p:*const nft_pktinfo)->*mut net { (*(*p).state).net }
#[inline] pub unsafe fn nft_hook(p:*const nft_pktinfo)->u32 { (*(*p).state).hook }
#[inline] pub unsafe fn nft_pf(p:*const nft_pktinfo)->u8 { (*(*p).state).pf }
#[inline] pub unsafe fn nft_in(p:*const nft_pktinfo)->*const net_device { (*(*p).state).in_ }
#[inline] pub unsafe fn nft_out(p:*const nft_pktinfo)->*const net_device { (*(*p).state).out }
#[inline] pub unsafe fn nft_set_pktinfo(p:*mut nft_pktinfo,s:*mut sk_buff,st:*const nf_hook_state){(*p).skb=s;(*p).state=st;}
#[inline] pub unsafe fn nft_set_pktinfo_unspec(p:*mut nft_pktinfo){(*p).flags=0;(*p).tprot=0;(*p).nhoff=0;(*p).thoff=0;(*p).fragoff=0;}

#[repr(C)] pub union nft_verdict_union { pub data:[u32;4], pub verdict:nft_verdict }
#[repr(C)] pub struct nft_verdict { pub code:u32, pub chain:*mut nft_chain }
#[repr(C,align(8))] pub struct nft_data { pub u:nft_verdict_union }
#[repr(C)] pub struct nft_regs { pub u:nft_verdict_union }
#[inline] pub unsafe fn nft_reg_store8(d:*mut u32,v:u8){ptr::write(d,0);ptr::write(d as *mut u8,v)}
#[inline] pub unsafe fn nft_reg_load8(s:*const u32)->u8{ptr::read(s as *const u8)}
#[inline] pub unsafe fn nft_reg_store16(d:*mut u32,v:u16){ptr::write(d,0);ptr::write(d as *mut u16,v)}
#[inline] pub unsafe fn nft_reg_store_be16(d:*mut u32,v:__be16){nft_reg_store16(d,v)}
#[inline] pub unsafe fn nft_reg_load16(s:*const u32)->u16{ptr::read(s as *const u16)}
#[inline] pub unsafe fn nft_reg_load_be16(s:*const u32)->__be16{nft_reg_load16(s)}
#[inline] pub unsafe fn nft_reg_load_be32(s:*const u32)->__be32{ptr::read(s)}
#[inline] pub unsafe fn nft_reg_store64(d:*mut u64,v:u64){ptr::write_unaligned(d,v)}
#[inline] pub unsafe fn nft_reg_load64(s:*const u32)->u64{ptr::read_unaligned(s as *const u64)}
#[inline] pub fn nft_reg_overlap(src:u8,dst:u8,len:u32)->bool{let n=((len as usize+3)/4) as u8;src!=dst&&src<dst.wrapping_add(n)&&dst<src.wrapping_add(n)}
#[inline] pub unsafe fn nft_data_copy(dst:*mut u32,src:*const nft_data,len:usize){ptr::copy_nonoverlapping(src as *const u8,dst as *mut u8,len)}

#[repr(C)] pub struct nft_ctx { pub net:*mut net,pub table:*mut nft_table,pub chain:*mut nft_chain,pub nla:*const *const nlaattr,pub portid:u32,pub seq:u32,pub flags:u16,pub family:u8,pub level:u8,pub report:bool,pub reg_inited:[u64;1] }
#[repr(C)] pub struct nft_data_desc { pub r#type:nft_data_types,pub size:usize,pub len:usize,pub flags:usize }
#[repr(C)] pub struct nft_userdata { pub len:u8,pub data:[u8;0] }
#[repr(C)] pub struct nft_elem_priv;
#[repr(C)] pub union nft_elem_union { pub buf:[u32;16],pub val:nft_data }
#[repr(C)] pub struct nft_set_elem { pub key:nft_elem_union,pub key_end:nft_elem_union,pub data:nft_elem_union,pub priv_:*mut nft_elem_priv }
#[repr(C)] pub enum nft_iter_type { NFT_ITER_UNSPEC,NFT_ITER_READ,NFT_ITER_UPDATE,NFT_ITER_UPDATE_CLONE }
#[repr(C)] pub struct nft_set_iter { pub genmask:u8,pub r#type:u8,pub count:u32,pub skip:u32,pub err:i32,pub fn_:Option<unsafe extern "C" fn(*const nft_ctx,*mut nft_set,*const nft_set_iter,*mut nft_elem_priv)->i32> }
#[repr(C)] pub struct nft_set_desc { pub ktype:u32,pub klen:u32,pub dtype:u32,pub dlen:u32,pub objtype:u32,pub size:u32,pub policy:u32,pub gc_int:u32,pub timeout:u64,pub field_len:[u8;20],pub field_count:u8,pub expr:bool }
#[repr(C)] pub enum nft_set_class { NFT_SET_CLASS_O_1,NFT_SET_CLASS_O_LOG_N,NFT_SET_CLASS_O_N }
#[repr(C)] pub struct nft_set_estimate { pub size:u64,pub lookup:nft_set_class,pub space:nft_set_class }

#[repr(C)] pub struct nft_expr { pub ops:*const nft_expr_ops,pub data:[u8;0] }
#[repr(C)] pub struct nft_set_elem_expr { pub size:u8,pub data:[u8;0] }
#[repr(C)] pub struct nft_set_ops { pub lookup:Option<unsafe extern "C" fn(*const net,*const nft_set,*const u32)->*const nft_set_ext>,pub update:Option<unsafe extern "C" fn(*mut nft_set,*const u32,*const nft_expr,*mut nft_regs)->*const nft_set_ext>,pub delete:Option<unsafe extern "C" fn(*const nft_set,*const u32)->bool>,pub elemsize:u32,pub abort_skip_removal:bool }
#[repr(C)] pub struct nft_set_type { pub ops:nft_set_ops,pub features:u32 }
#[repr(C)] pub struct nft_set { pub list:list_head,pub bindings:list_head,pub refs:refcount_t,pub table:*mut nft_table,pub net:possible_net_t,pub name:*mut i8,pub handle:u64,pub ktype:u32,pub dtype:u32,pub objtype:u32,pub size:u32,pub field_len:[u8;20],pub field_count:u8,pub in_update_walk:bool,pub use_:u32,pub nelems:atomic_t,pub ndeact:u32,pub timeout:u64,pub gc_int:u32,pub policy:u16,pub udlen:u16,pub udata:*mut u8,pub pending_update:list_head,pub ops:*const nft_set_ops,pub flags:u16,pub klen:u8,pub dlen:u8,pub num_exprs:u8,pub exprs:[*mut nft_expr;2],pub catchall_list:list_head,pub data:[u8;0] }
#[repr(C)] pub struct nft_set_binding { pub list:list_head,pub chain:*const nft_chain,pub flags:u32 }
#[repr(C)] pub struct nft_set_ext_tmpl { pub len:u16,pub offset:[u8;8],pub ext_len:[u8;8] }
#[repr(C,align(8))] pub struct nft_set_ext { pub genmask:u8,pub offset:[u8;8],pub data:[u8;0] }
#[repr(C)] pub struct nft_timeout { pub timeout:u64,pub expiration:u64 }
#[inline] pub unsafe fn nft_set_ext_exists(e:*const nft_set_ext,id:usize)->bool{!e.is_null()&&(*e).offset[id]!=0}
#[inline] pub unsafe fn nft_set_ext(e:*const nft_set_ext,id:usize)->*mut u8{(e as *mut u8).add((*e).offset[id] as usize)}
#[inline] pub unsafe fn nft_set_ext_key(e:*const nft_set_ext)->*mut nft_data{nft_set_ext(e,0) as *mut nft_data}
#[inline] pub unsafe fn nft_set_ext_data(e:*const nft_set_ext)->*mut nft_data{nft_set_ext(e,2) as *mut nft_data}
#[inline] pub unsafe fn nft_set_ext_timeout(e:*const nft_set_ext)->*mut nft_timeout{nft_set_ext(e,4) as *mut nft_timeout}
#[inline] pub unsafe fn nft_set_ext_userdata(e:*const nft_set_ext)->*mut nft_userdata{nft_set_ext(e,5) as *mut nft_userdata}
#[inline] pub unsafe fn nft_set_ext_expr(e:*const nft_set_ext)->*mut nft_set_elem_expr{nft_set_ext(e,6) as *mut nft_set_elem_expr}

#[repr(C)] pub struct nft_expr_type { pub select_ops:Option<unsafe extern "C" fn(*const nft_ctx,*const *const nlaattr)->*const nft_expr_ops>,pub release_ops:Option<unsafe extern "C" fn(*const nft_expr_ops)>,pub ops:*const nft_expr_ops,pub inner_ops:*const nft_expr_ops,pub list:list_head,pub name:*const i8,pub owner:*mut module,pub policy:*const nla_policy,pub maxattr:u32,pub family:u8,pub flags:u8 }
#[repr(C)] pub struct nft_expr_ops { pub eval:Option<unsafe extern "C" fn(*const nft_expr,*mut nft_regs,*const nft_pktinfo)>,pub clone_:Option<unsafe extern "C" fn(*mut nft_expr,*const nft_expr,gfp_t)->i32>,pub size:u32,pub r#type:*const nft_expr_type,pub data:*mut core::ffi::c_void }
#[repr(C)] pub struct nft_rule { pub list:list_head,pub handle:u64,pub data:[u8;0] }
#[repr(C)] pub enum nft_chain_types { NFT_CHAIN_T_DEFAULT=0,NFT_CHAIN_T_ROUTE,NFT_CHAIN_T_NAT,NFT_CHAIN_T_MAX }
#[repr(C)] pub struct nft_chain { pub blob_gen_0:*mut nft_rule_blob,pub blob_gen_1:*mut nft_rule_blob,pub rules:list_head,pub list:list_head,pub rhlhead:rhlist_head,pub table:*mut nft_table,pub handle:u64,pub use_:u32,pub flags:u8,pub name:*mut i8,pub udlen:u16,pub udata:*mut u8,pub blob_next:*mut nft_rule_blob }
#[repr(C)] pub struct nft_stats { pub bytes:u64,pub pkts:u64,pub syncp:u64_stats_sync }
#[repr(C)] pub struct nft_hook { pub list:list_head,pub ops_list:list_head,pub rcu:rcu_head,pub ifname:[i8;16],pub ifnamelen:u8,pub flags:u8 }
#[repr(C)] pub struct nft_base_chain { pub ops:nf_hook_ops,pub hook_list:list_head,pub r#type:*const nft_chain_type,pub policy:u8,pub flags:u8,pub stats:*mut nft_stats,pub chain:nft_chain,pub flow_block:flow_block }
#[repr(C)] pub struct nft_chain_type { pub name:*const i8,pub r#type:nft_chain_types,pub family:i32,pub owner:*mut module,pub hook_mask:u32,pub hooks:[*mut nf_hookfn;5] }
#[repr(C)] pub struct nft_table { pub list:list_head,pub chains_ht:rhltable,pub chains:list_head,pub sets:list_head,pub objects:list_head,pub flowtables:list_head,pub objname_ht:rhltable,pub hgenerator:u64,pub handle:u64,pub use_:u32,pub family:u16,pub nlpid:u32,pub name:*mut i8,pub udlen:u16,pub udata:*mut u8,pub validate_state:u8 }
#[repr(C)] pub struct nft_object_hash_key { pub name:*const i8,pub table:*const nft_table }
#[repr(C)] pub struct nft_object { pub list:list_head,pub rhlhead:rhlist_head,pub key:nft_object_hash_key,pub genmask:u32,pub use_:u32,pub handle:u64,pub udlen:u16,pub udata:*mut u8,pub ops:*const nft_object_ops,pub data:[u8;0] }
#[repr(C)] pub struct nft_object_type { pub select_ops:Option<unsafe extern "C" fn(*const nft_ctx,*const *const nlaattr)->*const nft_object_ops>,pub ops:*const nft_object_ops,pub list:list_head,pub r#type:u32,pub maxattr:u32,pub family:u8,pub owner:*mut module,pub policy:*const nla_policy }
#[repr(C)] pub struct nft_object_ops { pub eval:Option<unsafe extern "C" fn(*mut nft_object,*mut nft_regs,*const nft_pktinfo)>,pub size:u32,pub r#type:*const nft_object_type }
#[repr(C)] pub struct nft_flowtable { pub list:list_head,pub table:*mut nft_table,pub name:*mut i8,pub hooknum:i32,pub ops_len:i32,pub genmask:u32,pub use_:u32,pub handle:u64,pub hook_list:list_head,pub data:nf_flowtable }
#[repr(C)] pub struct nft_traceinfo { pub trace:bool,pub nf_trace:bool,pub packet_dumped:bool,pub r#type:u8,pub skbid:u32,pub basechain:*const nft_base_chain }

extern "C" {
    pub fn nft_data_init(ctx:*const nft_ctx,data:*mut nft_data,desc:*mut nft_data_desc,nla:*const nlaattr)->i32;
    pub fn nft_data_hold(data:*const nft_data,ty:nft_data_types); pub fn nft_data_release(data:*const nft_data,ty:nft_data_types);
    pub fn nft_expr_inner_parse(ctx:*const nft_ctx,nla:*const nlaattr,info:*mut nft_expr_info)->i32;
    pub fn nft_expr_destroy(ctx:*const nft_ctx,expr:*mut nft_expr); pub fn nft_do_chain(pkt:*mut nft_pktinfo,priv_:*mut core::ffi::c_void)->u32;
    pub fn nft_register_chain_type(ty:*const nft_chain_type); pub fn nft_unregister_chain_type(ty:*const nft_chain_type);
    pub fn nft_register_expr(ty:*mut nft_expr_type)->i32; pub fn nft_unregister_expr(ty:*mut nft_expr_type);
    pub fn nft_register_obj(ty:*mut nft_object_type)->i32; pub fn nft_unregister_obj(ty:*mut nft_object_type);
    pub fn nft_trace_init(info:*mut nft_traceinfo,pkt:*const nft_pktinfo,chain:*const nft_chain);
    pub fn nft_trace_notify(pkt:*const nft_pktinfo,verdict:*const nft_verdict,rule:*const core::ffi::c_void,info:*mut nft_traceinfo);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
