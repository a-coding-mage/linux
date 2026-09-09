// SPDX-License-Identifier: GPL-2.0-only
/* Connection tracking protocol helper module for SCTP. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// translation unit.

static SCTP_CONNTRACK_NAMES: [&str; SCTP_CONNTRACK_MAX as usize] = [
    "NONE", "CLOSED", "COOKIE_WAIT", "COOKIE_ECHOED", "ESTABLISHED",
    "SHUTDOWN_SENT", "SHUTDOWN_RECD", "SHUTDOWN_ACK_SENT", "HEARTBEAT_SENT",
];

static SCTP_TIMEOUTS: [u32; SCTP_CONNTRACK_MAX as usize] = [
    0, secs_to_jiffies(10), secs_to_jiffies(3), secs_to_jiffies(3),
    secs_to_jiffies(210), secs_to_jiffies(3), secs_to_jiffies(3),
    secs_to_jiffies(3), secs_to_jiffies(30),
];

const SCTP_FLAG_HEARTBEAT_VTAG_FAILED: u32 = 1;
const SNO: u8 = SCTP_CONNTRACK_NONE as u8;
const SCL: u8 = SCTP_CONNTRACK_CLOSED as u8;
const SCW: u8 = SCTP_CONNTRACK_COOKIE_WAIT as u8;
const SCE: u8 = SCTP_CONNTRACK_COOKIE_ECHOED as u8;
const SES: u8 = SCTP_CONNTRACK_ESTABLISHED as u8;
const SSS: u8 = SCTP_CONNTRACK_SHUTDOWN_SENT as u8;
const SSR: u8 = SCTP_CONNTRACK_SHUTDOWN_RECD as u8;
const SSA: u8 = SCTP_CONNTRACK_SHUTDOWN_ACK_SENT as u8;
const SHS: u8 = SCTP_CONNTRACK_HEARTBEAT_SENT as u8;
const SIV: u8 = SCTP_CONNTRACK_MAX as u8;

/* SCTP conntrack state transitions. */
static SCTP_CONNTRACKS: [[[u8; SCTP_CONNTRACK_MAX as usize]; 11]; 2] = [
 [
  [SCL,SCL,SCW,SCE,SES,SCL,SCL,SSA,SCW], [SCL,SCL,SCW,SCE,SES,SSS,SSR,SSA,SCL],
  [SCL,SCL,SCL,SCL,SCL,SCL,SCL,SCL,SCL], [SCL,SCL,SCW,SCE,SSS,SSS,SSR,SSA,SCL],
  [SSA,SCL,SCW,SCE,SES,SSA,SSA,SSA,SSA], [SCL,SCL,SCW,SCE,SES,SSS,SSR,SSA,SCL],
  [SCL,SCL,SCE,SCE,SES,SSS,SSR,SSA,SCL], [SCL,SCL,SCW,SES,SES,SSS,SSR,SSA,SCL],
  [SCL,SCL,SCW,SCE,SES,SSS,SSR,SCL,SCL], [SHS,SCL,SCW,SCE,SES,SSS,SSR,SSA,SHS],
  [SCL,SCL,SCW,SCE,SES,SSS,SSR,SSA,SHS],
 ],
 [
  [SIV,SCL,SCW,SCE,SES,SSS,SSR,SSA,SIV], [SIV,SCW,SCW,SCE,SES,SSS,SSR,SSA,SIV],
  [SIV,SCL,SCL,SCL,SCL,SCL,SCL,SCL,SIV], [SIV,SCL,SCW,SCE,SSR,SSS,SSR,SSA,SIV],
  [SIV,SCL,SCW,SCE,SES,SSA,SSA,SSA,SIV], [SIV,SCL,SCW,SCL,SES,SSS,SSR,SSA,SIV],
  [SIV,SCL,SCE,SCE,SES,SSS,SSR,SSA,SIV], [SIV,SCL,SCW,SES,SES,SSS,SSR,SSA,SIV],
  [SIV,SCL,SCW,SCE,SES,SSS,SSR,SCL,SIV], [SIV,SCL,SCW,SCE,SES,SSS,SSR,SSA,SHS],
  [SIV,SCL,SCW,SCE,SES,SSS,SSR,SSA,SES],
 ]
];

unsafe fn sctp_new_state(dir: ip_conntrack_dir, cur: sctp_conntrack, chunk_type: i32) -> i32 {
    let i = match chunk_type {
        SCTP_CID_INIT => 0, SCTP_CID_INIT_ACK => 1, SCTP_CID_ABORT => 2,
        SCTP_CID_SHUTDOWN => 3, SCTP_CID_SHUTDOWN_ACK => 4, SCTP_CID_ERROR => 5,
        SCTP_CID_COOKIE_ECHO => 6, SCTP_CID_COOKIE_ACK => 7,
        SCTP_CID_SHUTDOWN_COMPLETE => 8, SCTP_CID_HEARTBEAT => 9,
        SCTP_CID_HEARTBEAT_ACK => 10,
        _ => { pr_debug!("Unknown chunk type %d, Will stay in %s\n", chunk_type, SCTP_CONNTRACK_NAMES[cur as usize]); return cur as i32; }
    };
    SCTP_CONNTRACKS[dir as usize][i][cur as usize] as i32
}

unsafe fn do_basic_checks(ct: *mut nf_conn, skb: *const sk_buff, dataoff: u32,
                          map: *mut c_ulong, state: *const nf_hook_state) -> i32 {
    let mut offset = dataoff + core::mem::size_of::<sctphdr>() as u32;
    let mut count = 0u32;
    let mut sch = sctp_chunkhdr { type_: 0, flags: 0, length: 0 };
    while offset < (*skb).len && skb_header_pointer(skb, offset, core::mem::size_of_val(&sch), &mut sch as *mut _) {
        let flag = sch.type_ == SCTP_CID_INIT || sch.type_ == SCTP_CID_INIT_ACK || sch.type_ == SCTP_CID_SHUTDOWN_COMPLETE;
        if ((sch.type_ == SCTP_CID_COOKIE_ACK || sch.type_ == SCTP_CID_COOKIE_ECHO || flag) && count != 0) || sch.length == 0 {
            nf_ct_l4proto_log_invalid(skb, ct, state, "%s failed. chunk num %d, type %d, len %d flag %d\n", "do_basic_checks", count, sch.type_, sch.length, flag as i32);
            return 1;
        }
        if !map.is_null() { set_bit(sch.type_ as usize, map); }
        offset += ((ntohs(sch.length) as u32 + 3) & !3); count += 1;
    }
    (count == 0) as i32
}

/* The packet path, netlink conversion, timeout conversion, and protocol
 * registration retain the kernel implementation's interfaces and ordering. */
unsafe fn sctp_can_early_drop(ct: *const nf_conn) -> bool {
    matches!((*ct).proto.sctp.state, SCTP_CONNTRACK_SHUTDOWN_SENT | SCTP_CONNTRACK_SHUTDOWN_RECD | SCTP_CONNTRACK_SHUTDOWN_ACK_SENT)
}

unsafe fn sctp_error(skb: *mut sk_buff, dataoff: u32, state: *const nf_hook_state) -> bool {
    if (*skb).len < dataoff + core::mem::size_of::<sctphdr>() as u32 {
        nf_l4proto_log_invalid(skb, state, IPPROTO_SCTP, "%s", "nf_ct_sctp: short packet ");
        return true;
    }
    if (*state).hook == NF_INET_PRE_ROUTING && (*state).net.ct.sysctl_checksum && (*skb).ip_summed == CHECKSUM_NONE {
        if skb_ensure_writable(skb, dataoff + core::mem::size_of::<sctphdr>() as u32) {
            nf_l4proto_log_invalid(skb, state, IPPROTO_SCTP, "%s", "nf_ct_sctp: failed to read header ");
            return true;
        }
        let sh = &*((*skb).data.add(dataoff as usize) as *const sctphdr);
        if sh.checksum != sctp_compute_cksum(skb, dataoff) {
            nf_l4proto_log_invalid(skb, state, IPPROTO_SCTP, "%s", "nf_ct_sctp: bad CRC ");
            return true;
        }
        (*skb).ip_summed = CHECKSUM_UNNECESSARY;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_sctp_packet(ct: *mut nf_conn, skb: *mut sk_buff,
    dataoff: u32, ctinfo: ip_conntrack_info, state: *const nf_hook_state) -> i32 {
    if sctp_error(skb, dataoff, state) { return -NF_ACCEPT; }
    let mut hdr = sctphdr { source: 0, dest: 0, vtag: 0, checksum: 0 };
    let sh = skb_header_pointer(skb, dataoff, core::mem::size_of::<sctphdr>(), &mut hdr as *mut _);
    if !sh { return -NF_ACCEPT; }
    let mut map = [0 as c_ulong; 256 / core::mem::size_of::<c_ulong>()];
    if do_basic_checks(ct, skb, dataoff, map.as_mut_ptr(), state) != 0 { return -NF_ACCEPT; }
    if !nf_ct_is_confirmed(ct) {
        if test_bit(SCTP_CID_ABORT as usize, map.as_mut_ptr()) || test_bit(SCTP_CID_SHUTDOWN_COMPLETE as usize, map.as_mut_ptr()) || test_bit(SCTP_CID_COOKIE_ACK as usize, map.as_mut_ptr()) { return -NF_ACCEPT; }
        (*ct).proto.sctp.state = SCTP_CONNTRACK_NONE;
    }
    let dir = CTINFO2DIR(ctinfo);
    if !test_bit(SCTP_CID_INIT as usize, map.as_mut_ptr()) && hdr.vtag != (*ct).proto.sctp.vtag[dir as usize] { return -NF_ACCEPT; }
    let mut new_state = (*ct).proto.sctp.state;
    let mut offset = dataoff + core::mem::size_of::<sctphdr>() as u32;
    let mut chunk = sctp_chunkhdr { type_: 0, flags: 0, length: 0 };
    while offset < (*skb).len && skb_header_pointer(skb, offset, core::mem::size_of_val(&chunk), &mut chunk as *mut _) {
        let old = new_state;
        new_state = sctp_new_state(dir, old, chunk.type_) as _;
        if new_state == SCTP_CONNTRACK_MAX { return -NF_ACCEPT; }
        (*ct).proto.sctp.state = new_state;
        offset += (ntohs(chunk.length) as u32 + 3) & !3;
    }
    nf_ct_refresh_acct(ct, ctinfo, skb, SCTP_TIMEOUTS[new_state as usize]);
    NF_ACCEPT
}

pub unsafe fn nf_conntrack_sctp_init_net(net: *mut net) {
    let sn = nf_sctp_pernet(net);
    for i in 0..SCTP_CONNTRACK_MAX as usize { (*sn).timeouts[i] = SCTP_TIMEOUTS[i]; }
    (*sn).timeouts[0] = SCTP_TIMEOUTS[SCTP_CONNTRACK_CLOSED as usize];
}

#[no_mangle]
pub static nf_conntrack_l4proto_sctp: nf_conntrack_l4proto = nf_conntrack_l4proto {
    l4proto: IPPROTO_SCTP,
    can_early_drop: Some(sctp_can_early_drop),
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
