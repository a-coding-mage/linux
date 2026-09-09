/* Rust translation of tipc/msg.h. External kernel types and functions are dependencies. */

pub const TIPC_VERSION: u32 = 2;
pub const TIPC_SYSTEM_IMPORTANCE: u32 = 4;
pub const TIPC_CONN_MSG: u32 = 0;
pub const TIPC_MCAST_MSG: u32 = 1;
pub const TIPC_NAMED_MSG: u32 = 2;
pub const TIPC_DIRECT_MSG: u32 = 3;
pub const TIPC_GRP_MEMBER_EVT: u32 = 4;
pub const TIPC_GRP_BCAST_MSG: u32 = 5;
pub const TIPC_GRP_MCAST_MSG: u32 = 6;
pub const TIPC_GRP_UCAST_MSG: u32 = 7;
pub const BCAST_PROTOCOL: u32 = 5;
pub const MSG_BUNDLER: u32 = 6;
pub const LINK_PROTOCOL: u32 = 7;
pub const CONN_MANAGER: u32 = 8;
pub const GROUP_PROTOCOL: u32 = 9;
pub const TUNNEL_PROTOCOL: u32 = 10;
pub const NAME_DISTRIBUTOR: u32 = 11;
pub const MSG_FRAGMENTER: u32 = 12;
pub const LINK_CONFIG: u32 = 13;
pub const MSG_CRYPTO: u32 = 14;
pub const SOCK_WAKEUP: u32 = 14;
pub const TOP_SRV: u32 = 15;
pub const SHORT_H_SIZE: u32 = 24;
pub const BASIC_H_SIZE: u32 = 32;
pub const NAMED_H_SIZE: u32 = 40;
pub const MCAST_H_SIZE: u32 = 44;
pub const GROUP_H_SIZE: u32 = 44;
pub const INT_H_SIZE: u32 = 40;
pub const MIN_H_SIZE: u32 = 24;
pub const MAX_H_SIZE: u32 = 60;
pub const TIPC_MEDIA_INFO_OFFSET: usize = 5;
pub const MAX_GAP_ACK_BLKS: usize = 128;
pub const CONN_PROBE: u32 = 0; pub const CONN_PROBE_REPLY: u32 = 1; pub const CONN_ACK: u32 = 2;
pub const PUBLICATION: u32 = 0; pub const WITHDRAWAL: u32 = 1;
pub const FIRST_FRAGMENT: u32 = 0; pub const FRAGMENT: u32 = 1; pub const LAST_FRAGMENT: u32 = 2;
pub const STATE_MSG: u32 = 0; pub const RESET_MSG: u32 = 1; pub const ACTIVATE_MSG: u32 = 2;
pub const SYNCH_MSG: u32 = 0; pub const FAILOVER_MSG: u32 = 1;
pub const DSC_REQ_MSG: u32 = 0; pub const DSC_RESP_MSG: u32 = 1; pub const DSC_TRIAL_MSG: u32 = 2; pub const DSC_TRIAL_FAIL_MSG: u32 = 3;
pub const GRP_JOIN_MSG: u32 = 0; pub const GRP_LEAVE_MSG: u32 = 1; pub const GRP_ADV_MSG: u32 = 2; pub const GRP_ACK_MSG: u32 = 3; pub const GRP_RECLAIM_MSG: u32 = 4; pub const GRP_REMIT_MSG: u32 = 5;
pub const KEY_DISTR_MSG: u32 = 0;

#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: usize, pub cb: [u8; 48] }
#[repr(C)] pub struct sk_buff_head { pub lock: spinlock_t }
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct net; #[repr(C)] pub struct msghdr; #[repr(C)] pub struct tipc_crypto; #[repr(C)] pub struct tipc_aead;
pub type gfp_t = u32; pub type __be16 = u16; pub type __be32 = u32; pub type unchar = u8;
extern "C" { pub static one_page_mtu: i32; }

#[repr(C, packed)] pub struct tipc_skb_cb { pub tail: *mut sk_buff, pub nxt_retr: usize, pub retr_stamp: usize, pub bytes_read: u32, pub orig_member: u32, pub chain_imp: u16, pub ackers: u16, pub retr_cnt: u16, pub flags: u8, pub reserved: u8, pub crypto_ctx: *mut core::ffi::c_void }
#[repr(C)] pub struct tipc_msg { pub hdr: [__be32; 15] }
#[repr(C)] pub struct tipc_gap_ack { pub ack: __be16, pub gap: __be16 }
#[repr(C)] pub struct tipc_gap_ack_blks { pub len: __be16, pub ugack_cnt: u8, pub bgack_cnt: u8, pub gacks: [tipc_gap_ack; 0] }
pub const MAX_MSG_SIZE: usize = (MAX_H_SIZE as usize) + 65535;
pub const MAX_GAP_ACK_BLKS_SZ: usize = core::mem::size_of::<tipc_gap_ack_blks>() + core::mem::size_of::<tipc_gap_ack>() * MAX_GAP_ACK_BLKS;

extern "C" { fn ntohl(x: u32) -> u32; fn htonl(x: u32) -> u32; fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8; fn pr_warn(s: *const u8, ...); }
#[inline] pub unsafe fn buf_msg(skb: *mut sk_buff) -> *mut tipc_msg { (*skb).data as *mut tipc_msg }
#[inline] pub unsafe fn msg_word(m: *mut tipc_msg, p: u32) -> u32 { ntohl((*m).hdr[p as usize]) }
#[inline] pub unsafe fn msg_set_word(m:*mut tipc_msg,w:u32,v:u32){(*m).hdr[w as usize]=htonl(v)}
#[inline] pub unsafe fn msg_bits(m:*mut tipc_msg,w:u32,p:u32,mask:u32)->u32{(msg_word(m,w)>>p)&mask}
#[inline] pub unsafe fn msg_set_bits(m:*mut tipc_msg,w:u32,p:u32,mask:u32,val:u32){let v=(val&mask)<<p;let x=mask<<p;(*m).hdr[w as usize]&=!htonl(x);(*m).hdr[w as usize]|=htonl(v)}
#[inline] pub unsafe fn msg_version(m:*mut tipc_msg)->u32{msg_bits(m,0,29,7)}
#[inline] pub unsafe fn msg_set_version(m:*mut tipc_msg){msg_set_bits(m,0,29,7,TIPC_VERSION)}
#[inline] pub unsafe fn msg_user(m:*mut tipc_msg)->u32{msg_bits(m,0,25,0xf)}
#[inline] pub unsafe fn msg_isdata(m:*mut tipc_msg)->u32{(msg_user(m)<=4) as u32}
#[inline] pub unsafe fn msg_set_user(m:*mut tipc_msg,n:u32){msg_set_bits(m,0,25,0xf,n)}
#[inline] pub unsafe fn msg_hdr_sz(m:*mut tipc_msg)->u32{msg_bits(m,0,21,0xf)<<2}
#[inline] pub unsafe fn msg_set_hdr_sz(m:*mut tipc_msg,n:u32){msg_set_bits(m,0,21,0xf,n>>2)}
#[inline] pub unsafe fn msg_size(m:*mut tipc_msg)->u32{msg_bits(m,0,0,0x1ffff)}
#[inline] pub unsafe fn msg_blocks(m:*mut tipc_msg)->u32{msg_size(m)/1024+1}
#[inline] pub unsafe fn msg_data_sz(m:*mut tipc_msg)->u32{msg_size(m)-msg_hdr_sz(m)}
#[inline] pub unsafe fn msg_non_seq(m:*mut tipc_msg)->i32{msg_bits(m,0,20,1) as i32}
#[inline] pub unsafe fn msg_set_non_seq(m:*mut tipc_msg,n:u32){msg_set_bits(m,0,20,1,n)}
#[inline] pub unsafe fn msg_is_syn(m:*mut tipc_msg)->i32{msg_bits(m,0,17,1) as i32}
#[inline] pub unsafe fn msg_set_syn(m:*mut tipc_msg,n:u32){msg_set_bits(m,0,17,1,n)}
#[inline] pub unsafe fn msg_dest_droppable(m:*mut tipc_msg)->i32{msg_bits(m,0,19,1) as i32}
#[inline] pub unsafe fn msg_set_dest_droppable(m:*mut tipc_msg,n:u32){msg_set_bits(m,0,19,1,n)}
#[inline] pub unsafe fn msg_is_keepalive(m:*mut tipc_msg)->i32{msg_dest_droppable(m)}
#[inline] pub unsafe fn msg_set_is_keepalive(m:*mut tipc_msg,n:u32){msg_set_dest_droppable(m,n)}
#[inline] pub unsafe fn msg_src_droppable(m:*mut tipc_msg)->i32{msg_bits(m,0,18,1) as i32}
#[inline] pub unsafe fn msg_set_src_droppable(m:*mut tipc_msg,n:u32){msg_set_bits(m,0,18,1,n)}
#[inline] pub unsafe fn msg_ack_required(m:*mut tipc_msg)->i32{msg_src_droppable(m)}
#[inline] pub unsafe fn msg_set_ack_required(m:*mut tipc_msg){msg_set_src_droppable(m,1)}
#[inline] pub unsafe fn msg_nagle_ack(m:*mut tipc_msg)->i32{msg_src_droppable(m)}
#[inline] pub unsafe fn msg_set_nagle_ack(m:*mut tipc_msg){msg_set_src_droppable(m,1)}
#[inline] pub unsafe fn msg_is_rcast(m:*mut tipc_msg)->bool{msg_bits(m,0,18,1)!=0}
#[inline] pub unsafe fn msg_set_is_rcast(m:*mut tipc_msg,d:bool){msg_set_bits(m,0,18,1,d as u32)}
#[inline] pub unsafe fn msg_set_size(m:*mut tipc_msg,sz:u32){(*m).hdr[0]=htonl((msg_word(m,0)&!0x1ffff)|sz)}
#[inline] pub unsafe fn msg_data(m:*mut tipc_msg)->*mut u8{(m as *mut u8).add(msg_hdr_sz(m) as usize)}
#[inline] pub unsafe fn msg_inner_hdr(m:*mut tipc_msg)->*mut tipc_msg{msg_data(m) as *mut tipc_msg}

macro_rules! word_accessors { ($($get:ident,$set:ident,$w:expr);* $(;)?)=>{$(#[inline] pub unsafe fn $get(m:*mut tipc_msg)->u32{msg_word(m,$w)} #[inline] pub unsafe fn $set(m:*mut tipc_msg,v:u32){msg_set_word(m,$w,v)})*}; }
word_accessors!(msg_prevnode,msg_set_prevnode,3; msg_destport,msg_set_destport,5; msg_mc_netid,msg_set_mc_netid,5; msg_destnode,msg_set_destnode,7; msg_nametype,msg_set_nametype,8; msg_nameupper,msg_set_nameupper,10; msg_peer_net_hash,msg_set_peer_net_hash,13; msg_sugg_node_addr,msg_set_sugg_node_addr,14);
#[inline] pub unsafe fn msg_type(m:*mut tipc_msg)->u32{msg_bits(m,1,29,7)}
#[inline] pub unsafe fn msg_set_type(m:*mut tipc_msg,n:u32){msg_set_bits(m,1,29,7,n)}
#[inline] pub unsafe fn msg_errcode(m:*mut tipc_msg)->u32{msg_bits(m,1,25,15)}
#[inline] pub unsafe fn msg_set_errcode(m:*mut tipc_msg,n:u32){msg_set_bits(m,1,25,15,n)}
#[inline] pub unsafe fn msg_in_group(m:*mut tipc_msg)->bool{let t=msg_type(m);t>=4&&t<=7}
#[inline] pub unsafe fn msg_is_grp_evt(m:*mut tipc_msg)->bool{msg_type(m)==4}
#[inline] pub unsafe fn msg_named(m:*mut tipc_msg)->u32{(msg_type(m)==2) as u32}
#[inline] pub unsafe fn msg_mcast(m:*mut tipc_msg)->u32{matches!(msg_type(m),1|5|6) as u32}
#[inline] pub unsafe fn msg_connected(m:*mut tipc_msg)->u32{(msg_type(m)==0) as u32}
#[inline] pub unsafe fn msg_direct(m:*mut tipc_msg)->u32{(msg_type(m)==3) as u32}
#[inline] pub unsafe fn msg_set_bulk(m:*mut tipc_msg){msg_set_bits(m,1,28,1,1)} #[inline] pub unsafe fn msg_is_bulk(m:*mut tipc_msg)->u32{msg_bits(m,1,28,1)}
#[inline] pub unsafe fn msg_set_last_bulk(m:*mut tipc_msg){msg_set_bits(m,1,27,1,1)} #[inline] pub unsafe fn msg_is_last_bulk(m:*mut tipc_msg)->u32{msg_bits(m,1,27,1)}
#[inline] pub unsafe fn msg_set_non_legacy(m:*mut tipc_msg){msg_set_bits(m,1,26,1,1)} #[inline] pub unsafe fn msg_is_legacy(m:*mut tipc_msg)->u32{(msg_bits(m,1,26,1)==0) as u32}
#[inline] pub unsafe fn msg_reroute_cnt(m:*mut tipc_msg)->u32{msg_bits(m,1,21,15)} #[inline] pub unsafe fn msg_incr_reroute_cnt(m:*mut tipc_msg){msg_set_bits(m,1,21,15,msg_reroute_cnt(m)+1)}
#[inline] pub unsafe fn msg_lookup_scope(m:*mut tipc_msg)->u32{msg_bits(m,1,19,3)} #[inline] pub unsafe fn msg_set_lookup_scope(m:*mut tipc_msg,n:u32){msg_set_bits(m,1,19,3,n)}
#[inline] pub unsafe fn msg_bcast_ack(m:*mut tipc_msg)->u16{msg_bits(m,1,0,0xffff) as u16} #[inline] pub unsafe fn msg_set_bcast_ack(m:*mut tipc_msg,n:u16){msg_set_bits(m,1,0,0xffff,n as u32)}
#[inline] pub unsafe fn msg_ack(m:*mut tipc_msg)->u16{msg_bits(m,2,16,0xffff) as u16} #[inline] pub unsafe fn msg_set_ack(m:*mut tipc_msg,n:u16){msg_set_bits(m,2,16,0xffff,n as u32)}
#[inline] pub unsafe fn msg_seqno(m:*mut tipc_msg)->u16{msg_bits(m,2,0,0xffff) as u16} #[inline] pub unsafe fn msg_set_seqno(m:*mut tipc_msg,n:u16){msg_set_bits(m,2,0,0xffff,n as u32)}
#[inline] pub unsafe fn msg_short(m:*mut tipc_msg)->bool{msg_hdr_sz(m)==24}
#[inline] pub unsafe fn msg_orignode(m:*mut tipc_msg)->u32{if msg_short(m){msg_prevnode(m)}else{msg_word(m,6)}}
#[inline] pub unsafe fn msg_origport(m:*mut tipc_msg)->u32{msg_word(if msg_user(m)==MSG_FRAGMENTER{msg_inner_hdr(m)}else{m},4)}
#[inline] pub unsafe fn msg_nameinst(m:*mut tipc_msg)->u32{msg_word(m,9)} #[inline] pub unsafe fn msg_namelower(m:*mut tipc_msg)->u32{msg_nameinst(m)} #[inline] pub unsafe fn msg_set_namelower(m:*mut tipc_msg,n:u32){msg_set_word(m,9,n)} #[inline] pub unsafe fn msg_set_nameinst(m:*mut tipc_msg,n:u32){msg_set_namelower(m,n)}
// Remaining inline accessors preserve the header's word/bit operations; external SKB helpers and kernel APIs remain declarations.
#[inline] pub unsafe fn msg_set_origport(m:*mut tipc_msg,n:u32){msg_set_word(m,4,n)}
#[inline] pub unsafe fn msg_named_seqno(m:*mut tipc_msg)->u16{msg_bits(m,4,0,0xffff) as u16} #[inline] pub unsafe fn msg_set_named_seqno(m:*mut tipc_msg,n:u16){msg_set_bits(m,4,0,0xffff,n as u32)}
#[inline] pub unsafe fn msg_set_orignode(m:*mut tipc_msg,n:u32){msg_set_word(m,6,n)}
#[inline] pub unsafe fn msg_importance(m:*mut tipc_msg)->u32{let u=msg_user(m);if u<=4&&msg_errcode(m)==0{u}else if u==12||u==6{msg_bits(m,9,0,7)}else{4}}
#[inline] pub unsafe fn msg_set_importance(m:*mut tipc_msg,i:u32){let u=msg_user(m);if u==12||u==6{msg_set_bits(m,9,0,7,i)}else if i<4{msg_set_user(m,i)}}
#[inline] pub unsafe fn msg_set_dest_session_valid(m:*mut tipc_msg,v:bool){msg_set_bits(m,1,16,1,v as u32)} #[inline] pub unsafe fn msg_dest_session_valid(m:*mut tipc_msg)->bool{msg_bits(m,1,16,1)!=0}
#[inline] pub unsafe fn msg_dest_session(m:*mut tipc_msg)->u16{msg_bits(m,1,0,0xffff) as u16} #[inline] pub unsafe fn msg_set_dest_session(m:*mut tipc_msg,n:u16){msg_set_bits(m,1,0,0xffff,n as u32)}
#[inline] pub unsafe fn msg_seq_gap(m:*mut tipc_msg)->u32{msg_bits(m,1,16,0x1fff)} #[inline] pub unsafe fn msg_set_seq_gap(m:*mut tipc_msg,n:u32){msg_set_bits(m,1,16,0x1fff,n)}
#[inline] pub unsafe fn msg_node_sig(m:*mut tipc_msg)->u32{msg_bits(m,1,0,0xffff)} #[inline] pub unsafe fn msg_set_node_sig(m:*mut tipc_msg,n:u32){msg_set_bits(m,1,0,0xffff,n)}
#[inline] pub unsafe fn msg_node_capabilities(m:*mut tipc_msg)->u32{msg_bits(m,1,15,0x1fff)} #[inline] pub unsafe fn msg_set_node_capabilities(m:*mut tipc_msg,n:u32){msg_set_bits(m,1,15,0x1fff,n)}
#[inline] pub unsafe fn msg_dest_domain(m:*mut tipc_msg)->u32{msg_word(m,2)} #[inline] pub unsafe fn msg_set_dest_domain(m:*mut tipc_msg,n:u32){msg_set_word(m,2,n)}
#[inline] pub unsafe fn msg_set_bcgap_after(m:*mut tipc_msg,n:u32){msg_set_bits(m,2,16,0xffff,n)} #[inline] pub unsafe fn msg_bcgap_to(m:*mut tipc_msg)->u32{msg_bits(m,2,0,0xffff)} #[inline] pub unsafe fn msg_set_bcgap_to(m:*mut tipc_msg,n:u32){msg_set_bits(m,2,0,0xffff,n)}
#[inline] pub unsafe fn msg_last_bcast(m:*mut tipc_msg)->u32{msg_bits(m,4,16,0xffff)} #[inline] pub unsafe fn msg_bc_snd_nxt(m:*mut tipc_msg)->u32{msg_last_bcast(m)+1} #[inline] pub unsafe fn msg_set_last_bcast(m:*mut tipc_msg,n:u32){msg_set_bits(m,4,16,0xffff,n)}
#[inline] pub unsafe fn msg_nof_fragms(m:*mut tipc_msg)->u32{msg_bits(m,4,0,0xffff)} #[inline] pub unsafe fn msg_set_nof_fragms(m:*mut tipc_msg,n:u32){msg_set_bits(m,4,0,0xffff,n)} #[inline] pub unsafe fn msg_fragm_no(m:*mut tipc_msg)->u32{msg_last_bcast(m)} #[inline] pub unsafe fn msg_set_fragm_no(m:*mut tipc_msg,n:u32){msg_set_last_bcast(m,n)}
#[inline] pub unsafe fn msg_next_sent(m:*mut tipc_msg)->u16{msg_bits(m,4,0,0xffff) as u16} #[inline] pub unsafe fn msg_set_next_sent(m:*mut tipc_msg,n:u16){msg_set_bits(m,4,0,0xffff,n as u32)} #[inline] pub unsafe fn msg_bc_netid(m:*mut tipc_msg)->u32{msg_word(m,4)} #[inline] pub unsafe fn msg_set_bc_netid(m:*mut tipc_msg,n:u32){msg_set_word(m,4,n)}
#[inline] pub unsafe fn msg_session(m:*mut tipc_msg)->u16{msg_bits(m,5,16,0xffff) as u16} #[inline] pub unsafe fn msg_set_session(m:*mut tipc_msg,n:u16){msg_set_bits(m,5,16,0xffff,n as u32)} #[inline] pub unsafe fn msg_probe(m:*mut tipc_msg)->u32{msg_bits(m,5,0,1)} #[inline] pub unsafe fn msg_set_probe(m:*mut tipc_msg,n:u32){msg_set_bits(m,5,0,1,n)}
#[inline] pub unsafe fn msg_net_plane(m:*mut tipc_msg)->u8{(msg_bits(m,5,1,7)+b'A' as u32) as u8} #[inline] pub unsafe fn msg_set_net_plane(m:*mut tipc_msg,n:u8){msg_set_bits(m,5,1,7,(n-b'A') as u32)}
#[inline] pub unsafe fn msg_linkprio(m:*mut tipc_msg)->u32{msg_bits(m,5,4,0x1f)} #[inline] pub unsafe fn msg_set_linkprio(m:*mut tipc_msg,n:u32){msg_set_bits(m,5,4,0x1f,n)} #[inline] pub unsafe fn msg_bearer_id(m:*mut tipc_msg)->u32{msg_bits(m,5,9,7)} #[inline] pub unsafe fn msg_set_bearer_id(m:*mut tipc_msg,n:u32){msg_set_bits(m,5,9,7,n)}
#[inline] pub unsafe fn msg_redundant_link(m:*mut tipc_msg)->u32{msg_bits(m,5,12,1)} #[inline] pub unsafe fn msg_set_redundant_link(m:*mut tipc_msg,n:u32){msg_set_bits(m,5,12,1,n)} #[inline] pub unsafe fn msg_peer_stopping(m:*mut tipc_msg)->u32{msg_bits(m,5,13,1)} #[inline] pub unsafe fn msg_set_peer_stopping(m:*mut tipc_msg,n:u32){msg_set_bits(m,5,13,1,n)}
#[inline] pub unsafe fn msg_bc_ack_invalid(m:*mut tipc_msg)->bool{matches!(msg_user(m),5|7|11)&&msg_bits(m,5,14,1)!=0} #[inline] pub unsafe fn msg_set_bc_ack_invalid(m:*mut tipc_msg,n:bool){msg_set_bits(m,5,14,1,n as u32)}
#[inline] pub unsafe fn msg_media_addr(m:*mut tipc_msg)->*mut u8{(*m).hdr.as_mut_ptr().add(5) as *mut u8}
#[inline] pub unsafe fn msg_bc_gap(m:*mut tipc_msg)->u32{msg_bits(m,8,0,0x3ff)} #[inline] pub unsafe fn msg_set_bc_gap(m:*mut tipc_msg,n:u32){msg_set_bits(m,8,0,0x3ff,n)}
#[inline] pub unsafe fn msg_msgcnt(m:*mut tipc_msg)->u16{msg_bits(m,9,16,0xffff) as u16} #[inline] pub unsafe fn msg_set_msgcnt(m:*mut tipc_msg,n:u16){msg_set_bits(m,9,16,0xffff,n as u32)} #[inline] pub unsafe fn msg_syncpt(m:*mut tipc_msg)->u16{msg_msgcnt(m)} #[inline] pub unsafe fn msg_set_syncpt(m:*mut tipc_msg,n:u16){msg_set_msgcnt(m,n)} #[inline] pub unsafe fn msg_conn_ack(m:*mut tipc_msg)->u32{msg_bits(m,9,16,0xffff)} #[inline] pub unsafe fn msg_set_conn_ack(m:*mut tipc_msg,n:u32){msg_set_bits(m,9,16,0xffff,n)} #[inline] pub unsafe fn msg_adv_win(m:*mut tipc_msg)->u16{msg_bits(m,9,0,0xffff) as u16} #[inline] pub unsafe fn msg_set_adv_win(m:*mut tipc_msg,n:u16){msg_set_bits(m,9,0,0xffff,n as u32)}
#[inline] pub unsafe fn msg_max_pkt(m:*mut tipc_msg)->u32{msg_conn_ack(m)*4} #[inline] pub unsafe fn msg_set_max_pkt(m:*mut tipc_msg,n:u32){msg_set_conn_ack(m,n/4)} #[inline] pub unsafe fn msg_link_tolerance(m:*mut tipc_msg)->u32{msg_adv_win(m) as u32} #[inline] pub unsafe fn msg_set_link_tolerance(m:*mut tipc_msg,n:u32){msg_set_adv_win(m,n as u16)}
#[inline] pub unsafe fn msg_grp_evt(m:*mut tipc_msg)->u16{msg_bits(m,10,0,3) as u16} #[inline] pub unsafe fn msg_set_grp_evt(m:*mut tipc_msg,n:i32){msg_set_bits(m,10,0,3,n as u32)} #[inline] pub unsafe fn msg_grp_bc_ack_req(m:*mut tipc_msg)->u16{msg_bits(m,10,0,1) as u16} #[inline] pub unsafe fn msg_set_grp_bc_ack_req(m:*mut tipc_msg,n:bool){msg_set_bits(m,10,0,1,n as u32)} #[inline] pub unsafe fn msg_grp_bc_seqno(m:*mut tipc_msg)->u16{msg_bits(m,10,16,0xffff) as u16} #[inline] pub unsafe fn msg_set_grp_bc_seqno(m:*mut tipc_msg,n:u32){msg_set_bits(m,10,16,0xffff,n)}
#[inline] pub unsafe fn msg_peer_link_is_up(m:*mut tipc_msg)->bool{msg_user(m)!=7||msg_type(m)==0} #[inline] pub unsafe fn msg_peer_node_is_up(m:*mut tipc_msg)->bool{msg_peer_link_is_up(m)||msg_redundant_link(m)!=0} #[inline] pub unsafe fn msg_is_reset(m:*mut tipc_msg)->bool{msg_user(m)==7&&msg_type(m)==1}
#[inline] pub unsafe fn msg_set_node_id(h:*mut tipc_msg,id:*const u8){memcpy(msg_data(h),id,16)} #[inline] pub unsafe fn msg_node_id(h:*mut tipc_msg)->*mut u8{msg_data(h)}
extern "C" { pub fn tipc_buf_acquire(size:u32,gfp:gfp_t)->*mut sk_buff; pub fn tipc_msg_validate(skb:*mut *mut sk_buff)->bool; pub fn tipc_msg_reverse(addr:u32,skb:*mut *mut sk_buff,err:i32)->bool; pub fn tipc_msg_init(addr:u32,m:*mut tipc_msg,user:u32,typ:u32,hsize:u32,dest:u32); pub fn tipc_msg_create(user:u32,typ:u32,hdr:u32,data:u32,dnode:u32,onode:u32,dport:u32,oport:u32,err:i32)->*mut sk_buff; pub fn tipc_buf_append(head:*mut *mut sk_buff,buf:*mut *mut sk_buff)->i32; pub fn tipc_msg_assemble(list:*mut sk_buff_head)->bool; pub fn tipc_msg_reassemble(list:*mut sk_buff_head,rcvq:*mut sk_buff_head)->bool; }
#[inline] pub unsafe fn buf_seqno(skb:*mut sk_buff)->u16{msg_seqno(buf_msg(skb))} #[inline] pub unsafe fn buf_roundup_len(skb:*mut sk_buff)->usize{((*skb).len/1024+1)*1024}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
