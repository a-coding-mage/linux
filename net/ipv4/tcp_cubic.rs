// SPDX-License-Identifier: GPL-2.0-only
/* TCP CUBIC: Binary Increase Congestion control for TCP v2.3 */

// Linux kernel dependencies supplied by other translation units.
use core::mem::{size_of, zeroed};

const BICTCP_BETA_SCALE: u32 = 1024;
const BICTCP_HZ: u32 = 10;
const HYSTART_ACK_TRAIN: u32 = 0x1;
const HYSTART_DELAY: u32 = 0x2;
const HYSTART_MIN_SAMPLES: u8 = 8;
const HYSTART_DELAY_MIN: u32 = 4000;
const HYSTART_DELAY_MAX: u32 = 16000;

static mut fast_convergence: i32 = 1;
static mut beta: i32 = 717;
static mut initial_ssthresh: i32 = 0;
static mut bic_scale: i32 = 41;
static mut tcp_friendliness: i32 = 1;
static mut hystart: i32 = 1;
static mut hystart_detect: i32 = (HYSTART_ACK_TRAIN | HYSTART_DELAY) as i32;
static mut hystart_low_window: i32 = 16;
static mut hystart_ack_delta_us: i32 = 2000;
static mut cube_rtt_scale: u32 = 0;
static mut beta_scale: u32 = 0;
static mut cube_factor: u64 = 0;

#[repr(C)]
pub struct bictcp {
    cnt: u32, last_max_cwnd: u32, last_cwnd: u32, last_time: u32,
    bic_origin_point: u32, bic_K: u32, delay_min: u32, epoch_start: u32,
    ack_cnt: u32, tcp_cwnd: u32, unused: u16, sample_cnt: u8, found: u8,
    round_start: u32, end_seq: u32, last_ack: u32, curr_rtt: u32,
}

extern "C" {
    static mut tcp_jiffies32: u32;
    static HZ: u32;
    fn tcp_sk(sk: *const sock) -> *mut tcp_sock;
    fn inet_csk_ca(sk: *mut sock) -> *mut bictcp;
    fn tcp_is_cwnd_limited(sk: *mut sock) -> bool;
    fn tcp_in_slow_start(tp: *const tcp_sock) -> bool;
    fn tcp_slow_start(tp: *mut tcp_sock, acked: u32) -> u32;
    fn tcp_snd_cwnd(tp: *const tcp_sock) -> u32;
    fn tcp_cong_avoid_ai(tp: *mut tcp_sock, cnt: u32, acked: u32);
    fn tcp_reno_undo_cwnd(sk: *mut sock) -> u32;
    fn tcp_register_congestion_control(ops: *mut tcp_congestion_ops) -> i32;
    fn tcp_unregister_congestion_control(ops: *mut tcp_congestion_ops);
    fn register_btf_kfunc_id_set(ty: i32, set: *const core::ffi::c_void) -> i32;
    fn usecs_to_jiffies(x: u32) -> u32;
    fn fls64(x: u64) -> u32;
    fn div64_u64(a: u64, b: u64) -> u64;
}

#[repr(C)] pub struct sock { sk_pacing_rate: u64, sk_gso_max_size: u32, sk_pacing_status: u8 }
#[repr(C)] pub struct tcp_sock { tcp_mstamp: u32, snd_nxt: u32, lsndtime: u32, snd_ssthresh: u32, snd_una: u32 }
#[repr(C)] pub struct ack_sample { rtt_us: i32 }
#[repr(C)] pub struct tcp_congestion_ops { init: Option<unsafe extern "C" fn(*mut sock)>, ssthresh: Option<unsafe extern "C" fn(*mut sock)->u32>, cong_avoid: Option<unsafe extern "C" fn(*mut sock,u32,u32)>, set_state: Option<unsafe extern "C" fn(*mut sock,u8)>, undo_cwnd: Option<unsafe extern "C" fn(*mut sock)->u32>, cwnd_event_tx_start: Option<unsafe extern "C" fn(*mut sock)>, pkts_acked: Option<unsafe extern "C" fn(*mut sock,*const ack_sample)>, owner: *mut core::ffi::c_void, name: *const u8 }

#[inline] unsafe fn bictcp_reset(ca: *mut bictcp) { core::ptr::write_bytes(ca, 0, 1); (*ca).found = 0; }
#[inline] unsafe fn bictcp_clock_us(sk: *const sock) -> u32 { (*tcp_sk(sk)).tcp_mstamp }
#[inline] unsafe fn bictcp_hystart_reset(sk: *mut sock) { let tp=tcp_sk(sk); let ca=inet_csk_ca(sk); (*ca).round_start=bictcp_clock_us(sk); (*ca).last_ack=(*ca).round_start; (*ca).end_seq=(*tp).snd_nxt; (*ca).curr_rtt=u32::MAX; (*ca).sample_cnt=0; }

unsafe fn cubictcp_init(sk: *mut sock) { let ca=inet_csk_ca(sk); bictcp_reset(ca); if hystart != 0 { bictcp_hystart_reset(sk); } if hystart == 0 && initial_ssthresh != 0 { (*tcp_sk(sk)).snd_ssthresh=initial_ssthresh as u32; } }
unsafe fn cubictcp_cwnd_event_tx_start(sk: *mut sock) { let ca=inet_csk_ca(sk); let now=tcp_jiffies32; let delta=now.wrapping_sub((*tcp_sk(sk)).lsndtime) as i32; if (*ca).epoch_start != 0 && delta>0 { (*ca).epoch_start=(*ca).epoch_start.wrapping_add(delta as u32); if (*ca).epoch_start.wrapping_sub(now) as i32 > 0 { (*ca).epoch_start=now; } } }

unsafe fn cubic_root(mut a: u64) -> u32 {
    let v: [u8;64]=[0,54,54,54,118,118,118,118,123,129,134,138,143,147,151,156,157,161,164,168,170,173,176,179,181,185,187,190,192,194,197,199,200,202,204,206,209,211,213,215,217,219,221,222,224,225,227,229,231,232,234,236,237,239,240,242,244,245,246,248,250,251,252,254];
    let mut b=fls64(a); if b<7 { return ((v[a as usize] as u32)+35)>>6; } b=((b*84)>>8)-1; let shift=a>>(b*3); let mut x=(((v[shift as usize] as u32+10)<<b))>>6; x=2*x+(div64_u64(a,(x as u64)*(x.wrapping_sub(1) as u64)) as u32); (x*341)>>10
}

unsafe fn bictcp_update(ca:*mut bictcp,cwnd:u32,acked:u32) {
    (*ca).ack_cnt=(*ca).ack_cnt.wrapping_add(acked); if (*ca).last_cwnd==cwnd && (tcp_jiffies32.wrapping_sub((*ca).last_time) as i32)<=HZ as i32/32{return;} if (*ca).epoch_start!=0 && tcp_jiffies32==(*ca).last_time { } else { (*ca).last_cwnd=cwnd; (*ca).last_time=tcp_jiffies32; if (*ca).epoch_start==0 { (*ca).epoch_start=tcp_jiffies32; (*ca).ack_cnt=acked; (*ca).tcp_cwnd=cwnd; if (*ca).last_max_cwnd<=cwnd {(*ca).bic_K=0;(*ca).bic_origin_point=cwnd;} else {(*ca).bic_K=cubic_root(cube_factor*(((*ca).last_max_cwnd-cwnd) as u64));(*ca).bic_origin_point=(*ca).last_max_cwnd;} } let mut t=(tcp_jiffies32.wrapping_sub((*ca).epoch_start) as i32) as u64; t+=usecs_to_jiffies((*ca).delay_min) as u64; t<<=BICTCP_HZ; t/=HZ as u64; let offs=if t<(*ca).bic_K as u64 {(*ca).bic_K as u64-t}else{t-(*ca).bic_K as u64}; let delta=((cube_rtt_scale as u64*offs*offs*offs)>>(10+3*BICTCP_HZ)) as u32; let target=if t<(*ca).bic_K as u64{(*ca).bic_origin_point-delta}else{(*ca).bic_origin_point+delta}; (*ca).cnt=if target>cwnd{cwnd/(target-cwnd)}else{100*cwnd}; if (*ca).last_max_cwnd==0&&(*ca).cnt>20{(*ca).cnt=20;} }
    if tcp_friendliness!=0 { let scale=beta_scale; let delta=(cwnd*scale)>>3; while (*ca).ack_cnt>delta {(*ca).ack_cnt-=delta;(*ca).tcp_cwnd+=1;} if (*ca).tcp_cwnd>cwnd {let d=(*ca).tcp_cwnd-cwnd; let m=cwnd/d; if (*ca).cnt>m{(*ca).cnt=m;}} } if (*ca).cnt<2{(*ca).cnt=2;}
}

unsafe fn cubictcp_cong_avoid(sk:*mut sock,_ack:u32,mut acked:u32){let tp=tcp_sk(sk);let ca=inet_csk_ca(sk);if !tcp_is_cwnd_limited(sk){return;}if tcp_in_slow_start(tp){acked=tcp_slow_start(tp,acked);if acked==0{return;}}bictcp_update(ca,tcp_snd_cwnd(tp),acked);tcp_cong_avoid_ai(tp,(*ca).cnt,acked);}
unsafe fn cubictcp_recalc_ssthresh(sk:*mut sock)->u32{let tp=tcp_sk(sk);let ca=inet_csk_ca(sk);(*ca).epoch_start=0;if tcp_snd_cwnd(tp)<(*ca).last_max_cwnd&&fast_convergence!=0{(*ca).last_max_cwnd=(tcp_snd_cwnd(tp)*(BICTCP_BETA_SCALE+beta as u32))/(2*BICTCP_BETA_SCALE);}else{(*ca).last_max_cwnd=tcp_snd_cwnd(tp);}let x=(tcp_snd_cwnd(tp)*beta as u32)/BICTCP_BETA_SCALE;if x>2{x}else{2}}
unsafe fn cubictcp_state(sk:*mut sock,new_state:u8){if new_state==TCP_CA_Loss{bictcp_reset(inet_csk_ca(sk));bictcp_hystart_reset(sk);}}
const TCP_CA_Loss:u8=4;

unsafe fn hystart_ack_delay(sk:*const sock)->u32 { let rate=(*sk).sk_pacing_rate; if rate==0{return 0;} let x=((*sk).sk_gso_max_size as u64*4*1_000_000)/rate; if x>1000{1000}else{x as u32} }
unsafe fn hystart_update(sk:*mut sock,delay:u32){let tp=tcp_sk(sk);let ca=inet_csk_ca(sk);if ((*tp).snd_una.wrapping_sub((*ca).end_seq) as i32)>0{bictcp_hystart_reset(sk);}if tcp_snd_cwnd(tp)<hystart_low_window as u32{return;}if (hystart_detect as u32)&HYSTART_ACK_TRAIN!=0{let now=bictcp_clock_us(sk);if now.wrapping_sub((*ca).last_ack) as i32<=hystart_ack_delta_us{(*ca).last_ack=now;let mut threshold=(*ca).delay_min+hystart_ack_delay(sk);if (*sk).sk_pacing_status==0{threshold>>=1;}if now.wrapping_sub((*ca).round_start)>threshold{(*ca).found=1;(*tp).snd_ssthresh=tcp_snd_cwnd(tp);}}}if (hystart_detect as u32)&HYSTART_DELAY!=0{if (*ca).curr_rtt>delay{(*ca).curr_rtt=delay;}if (*ca).sample_cnt<HYSTART_MIN_SAMPLES{(*ca).sample_cnt+=1;}else if (*ca).curr_rtt>(*ca).delay_min+((*ca).delay_min>>3).clamp(HYSTART_DELAY_MIN,HYSTART_DELAY_MAX){(*ca).found=1;(*tp).snd_ssthresh=tcp_snd_cwnd(tp);}}}
unsafe fn cubictcp_acked(sk:*mut sock,sample:*const ack_sample){let tp=tcp_sk(sk);let ca=inet_csk_ca(sk);if (*sample).rtt_us<0{return;}if (*ca).epoch_start!=0&&(tcp_jiffies32.wrapping_sub((*ca).epoch_start) as i32)<HZ as i32{return;}let mut delay=(*sample).rtt_us as u32;if delay==0{delay=1;}if (*ca).delay_min==0||(*ca).delay_min>delay{(*ca).delay_min=delay;}if (*ca).found==0&&tcp_in_slow_start(tp)&&hystart!=0{hystart_update(sk,delay);}}

// Precomputation performed by cubictcp_register in the kernel module.
unsafe fn cubictcp_register()->i32{beta_scale=8*(BICTCP_BETA_SCALE+beta as u32)/3/(BICTCP_BETA_SCALE-beta as u32);cube_rtt_scale=(bic_scale*10) as u32;cube_factor=1u64<<(10+3*BICTCP_HZ);cube_factor/= (bic_scale*10) as u64;0}
unsafe fn cubictcp_unregister(){}

// Remaining module registration metadata and kernel statistics are supplied by the Linux build environment.
#[allow(dead_code)] static mut cubictcp: tcp_congestion_ops = tcp_congestion_ops {init:Some(cubictcp_init),ssthresh:Some(cubictcp_recalc_ssthresh),cong_avoid:Some(cubictcp_cong_avoid),set_state:Some(cubictcp_state),undo_cwnd:None,cwnd_event_tx_start:Some(cubictcp_cwnd_event_tx_start),pkts_acked:None,owner:core::ptr::null_mut(),name:b"cubic\0".as_ptr()};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
