/* Translated from net/tipc/link.c.  External kernel/TIPC types and helpers are
 * intentionally left as dependencies supplied by the surrounding crate. */

#[repr(C)]
pub struct tipc_stats { pub sent_pkts:u32,pub recv_pkts:u32,pub sent_states:u32,pub recv_states:u32,pub sent_probes:u32,pub recv_probes:u32,pub sent_nacks:u32,pub recv_nacks:u32,pub sent_acks:u32,pub sent_bundled:u32,pub sent_bundles:u32,pub recv_bundled:u32,pub recv_bundles:u32,pub retransmitted:u32,pub sent_fragmented:u32,pub sent_fragments:u32,pub recv_fragmented:u32,pub recv_fragments:u32,pub link_congs:u32,pub deferred_recv:u32,pub duplicates:u32,pub max_queue_sz:u32,pub accu_queue_sz:u32,pub queue_sz_counts:u32,pub msg_length_counts:u32,pub msg_lengths_total:u32,pub msg_length_profile:[u32;7] }

#[repr(C)]
pub struct tipc_link {
    pub addr:u32, pub name:[i8; TIPC_MAX_LINK_NAME], pub net:*mut net,
    pub peer_session:u16,pub session:u16,pub snd_nxt_state:u16,pub rcv_nxt_state:u16,
    pub peer_bearer_id:u32,pub bearer_id:u32,pub tolerance:u32,pub abort_limit:u32,
    pub state:u32,pub peer_caps:u16,pub in_session:bool,pub active:bool,
    pub silent_intv_cnt:u32,pub if_name:[i8; TIPC_MAX_IF_NAME],pub priority:u32,pub net_plane:i8,
    pub mon_state:tipc_mon_state,pub rst_cnt:u16,pub drop_point:u16,
    pub failover_reasm_skb:*mut sk_buff,pub failover_deferdq:sk_buff_head,
    pub mtu:u16,pub advertised_mtu:u16,pub transmq:sk_buff_head,pub backlogq:sk_buff_head,
    pub backlog:[tipc_backlog;5],pub snd_nxt:u16,pub rcv_nxt:u16,pub rcv_unacked:u32,
    pub deferdq:sk_buff_head,pub inputq:*mut sk_buff_head,pub namedq:*mut sk_buff_head,
    pub wakeupq:sk_buff_head,pub window:u16,pub min_win:u16,pub ssthresh:u16,pub max_win:u16,
    pub cong_acks:u16,pub checkpoint:u16,pub reasm_buf:*mut sk_buff,pub reasm_tnlmsg:*mut sk_buff,
    pub ackers:u16,pub acked:u16,pub last_gap:u16,pub last_ga:*mut tipc_gap_ack_blks,
    pub bc_rcvlink:*mut tipc_link,pub bc_sndlink:*mut tipc_link,pub nack_state:u8,pub bc_peer_is_up:bool,
    pub stats:tipc_stats,
}
#[repr(C)] pub struct tipc_backlog { pub len:u16,pub limit:u16,pub target_bskb:*mut sk_buff }

pub const BC_NACK_SND_CONDITIONAL:u32=0; pub const BC_NACK_SND_UNCONDITIONAL:u32=1; pub const BC_NACK_SND_SUPPRESS:u32=2;
pub const LINK_ESTABLISHED:u32=0xe; pub const LINK_ESTABLISHING:u32=0xe<<4; pub const LINK_RESET:u32=1<<8; pub const LINK_RESETTING:u32=2<<12; pub const LINK_PEER_RESET:u32=0xd<<16; pub const LINK_FAILINGOVER:u32=0xf<<20; pub const LINK_SYNCHING:u32=0xc<<24;

pub unsafe fn tipc_link_is_up(l:*mut tipc_link)->bool { (*l).state & (LINK_ESTABLISHED|LINK_SYNCHING) != 0 }
pub unsafe fn tipc_link_peer_is_down(l:*mut tipc_link)->bool { (*l).state==LINK_PEER_RESET }
pub unsafe fn tipc_link_is_reset(l:*mut tipc_link)->bool { (*l).state&(LINK_RESET|LINK_FAILINGOVER|LINK_ESTABLISHING)!=0 }
pub unsafe fn tipc_link_is_establishing(l:*mut tipc_link)->bool { (*l).state==LINK_ESTABLISHING }
pub unsafe fn tipc_link_is_synching(l:*mut tipc_link)->bool { (*l).state==LINK_SYNCHING }
pub unsafe fn tipc_link_is_failingover(l:*mut tipc_link)->bool { (*l).state==LINK_FAILINGOVER }
pub unsafe fn tipc_link_is_blocked(l:*mut tipc_link)->bool { (*l).state&(LINK_RESETTING|LINK_PEER_RESET|LINK_FAILINGOVER)!=0 }
unsafe fn link_is_bc_sndlink(l:*mut tipc_link)->bool { (*l).bc_sndlink.is_null() }
unsafe fn link_is_bc_rcvlink(l:*mut tipc_link)->bool { (*l).bc_rcvlink==l && !link_is_bc_sndlink(l) }
pub unsafe fn tipc_link_set_active(l:*mut tipc_link,a:bool){(*l).active=a}
pub unsafe fn tipc_link_id(l:*mut tipc_link)->u32 { ((*l).peer_bearer_id<<16)|(*l).bearer_id }
pub unsafe fn tipc_link_min_win(l:*mut tipc_link)->i32{(*l).min_win as i32} pub unsafe fn tipc_link_max_win(l:*mut tipc_link)->i32{(*l).max_win as i32} pub unsafe fn tipc_link_prio(l:*mut tipc_link)->i32{(*l).priority as i32} pub unsafe fn tipc_link_tolerance(l:*mut tipc_link)->usize{(*l).tolerance as usize}
pub unsafe fn tipc_link_rcv_nxt(l:*mut tipc_link)->u16{(*l).rcv_nxt} pub unsafe fn tipc_link_acked(l:*mut tipc_link)->u16{(*l).acked} pub unsafe fn tipc_link_state(l:*mut tipc_link)->u32{(*l).state}
pub unsafe fn tipc_link_update_caps(l:*mut tipc_link,c:u16){(*l).peer_caps=c}
pub unsafe fn tipc_link_bc_peers(l:*mut tipc_link)->i32{(*l).ackers as i32}
pub unsafe fn tipc_link_set_mtu(l:*mut tipc_link,m:i32){(*l).mtu=m as u16} pub unsafe fn tipc_link_mtu(l:*mut tipc_link)->i32{(*l).mtu as i32}
pub unsafe fn tipc_link_mss(l:*mut tipc_link)->i32 { (*l).mtu as i32-INT_H_SIZE }
pub unsafe fn tipc_link_too_silent(l:*mut tipc_link)->bool { (*l).silent_intv_cnt+2>(*l).abort_limit }

/* The remaining implementation retains the original control flow through the
 * low-level queue/message helpers supplied by the TIPC port. */
pub unsafe fn tipc_link_reset(l:*mut tipc_link) { (*l).in_session=false; (*l).peer_session=(*l).peer_session.wrapping_sub(1); (*l).session=(*l).session.wrapping_add(1); (*l).mtu=(*l).advertised_mtu; (*l).rcv_unacked=0; (*l).snd_nxt=1; (*l).rcv_nxt=1; (*l).snd_nxt_state=1; (*l).rcv_nxt_state=1; (*l).acked=0; (*l).last_gap=0; (*l).silent_intv_cnt=0; (*l).rst_cnt=0; (*l).bc_peer_is_up=false; }

extern "C" { pub fn tipc_link_fsm_evt(l:*mut tipc_link,evt:i32)->i32; pub fn tipc_link_rcv(l:*mut tipc_link,skb:*mut sk_buff,xmitq:*mut sk_buff_head)->i32; pub fn tipc_link_xmit(l:*mut tipc_link,list:*mut sk_buff_head,xmitq:*mut sk_buff_head)->i32; }

/* External declarations corresponding to the included kernel/TIPC headers. */
#[allow(non_camel_case_types)] pub enum net{} #[allow(non_camel_case_types)] pub enum sk_buff{} #[allow(non_camel_case_types)] pub enum sk_buff_head{} #[allow(non_camel_case_types)] pub enum tipc_mon_state{} #[allow(non_camel_case_types)] pub enum tipc_gap_ack_blks{} #[allow(non_camel_case_types)] pub enum tipc_msg{}
extern "C" { static TIPC_MAX_LINK_NAME:usize; static TIPC_MAX_IF_NAME:usize; static INT_H_SIZE:i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
