// SPDX-License-Identifier: GPL-2.0-only
// Kernel dependencies supplied by the surrounding netfilter implementation.

use core::ffi::c_void;

pub type S32 = i32;
pub type U32 = u32;
pub type Be32 = u32;

#[repr(C)]
pub struct Spinlock { _private: [u8; 0] }

#[repr(C)]
pub struct NfCtSeqadj {
    pub correction_pos: U32,
    pub offset_before: S32,
    pub offset_after: S32,
}

#[repr(C)]
pub struct NfConnSeqadj {
    pub seq: [NfCtSeqadj; 2],
}

#[repr(C)]
pub struct NfConn {
    pub lock: Spinlock,
    pub status: U32,
}

#[repr(C)]
pub struct SkBuff {
    pub data: *mut u8,
}

#[repr(C)]
pub struct Tcphdr {
    pub source: u16,
    pub dest: u16,
    pub seq: Be32,
    pub ack_seq: Be32,
    pub doff: u8,
    pub ack: u8,
    pub check: u16,
}

#[repr(C)]
pub struct TcpSackBlockWire {
    pub start_seq: Be32,
    pub end_seq: Be32,
}

pub type IpConntrackInfo = i32;
pub type IpConntrackDir = usize;

extern "C" {
    fn ctinfo2dir(ctinfo: IpConntrackInfo) -> IpConntrackDir;
    fn nfct_seqadj(ct: *const NfConn) -> *mut NfConnSeqadj;
    fn spin_lock_bh(lock: *mut Spinlock);
    fn spin_unlock_bh(lock: *mut Spinlock);
    fn set_bit(bit: usize, word: *mut U32);
    fn nf_ct_protonum(ct: *const NfConn) -> u8;
    fn skb_network_header(skb: *const SkBuff) -> *mut u8;
    fn ip_hdrlen(skb: *const SkBuff) -> usize;
    fn skb_ensure_writable(skb: *mut SkBuff, len: usize) -> i32;
    fn inet_proto_csum_replace4(check: *mut u16, skb: *mut SkBuff,
                                from: Be32, to: Be32, pseudohdr: bool);
    fn pr_debug(format: *const u8, ...);
    fn before(a: U32, b: U32) -> bool;
    fn after(a: U32, b: U32) -> bool;
}

pub const IPS_SEQ_ADJUST_BIT: usize = 5;
pub const IPPROTO_TCP: u8 = 6;
pub const TCPOPT_EOL: u8 = 0;
pub const TCPOPT_NOP: u8 = 1;
pub const TCPOPT_SACK: u8 = 5;
pub const TCPOLEN_SACK_PERBLOCK: usize = 8;

#[inline]
unsafe fn ntohl(value: Be32) -> U32 { value.to_be() }
#[inline]
unsafe fn htonl(value: U32) -> Be32 { value.to_be() }

#[no_mangle]
pub unsafe extern "C" fn nf_ct_seqadj_init(ct: *mut NfConn, ctinfo: IpConntrackInfo, off: S32) -> i32 {
    let dir = ctinfo2dir(ctinfo);
    if off == 0 { return 0; }
    spin_lock_bh(&mut (*ct).lock);
    let seqadj = nfct_seqadj(ct);
    if seqadj.is_null() {
        spin_unlock_bh(&mut (*ct).lock);
        return 0;
    }
    set_bit(IPS_SEQ_ADJUST_BIT, &mut (*ct).status);
    let this_way = &mut (*seqadj).seq[dir];
    this_way.offset_before = off;
    this_way.offset_after = off;
    spin_unlock_bh(&mut (*ct).lock);
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_seqadj_set(ct: *mut NfConn, ctinfo: IpConntrackInfo, seq: Be32, off: S32) -> i32 {
    let seqadj = nfct_seqadj(ct);
    let dir = ctinfo2dir(ctinfo);
    if off == 0 || seqadj.is_null() { return 0; }
    set_bit(IPS_SEQ_ADJUST_BIT, &mut (*ct).status);
    spin_lock_bh(&mut (*ct).lock);
    let this_way = &mut (*seqadj).seq[dir];
    if this_way.offset_before == this_way.offset_after ||
       before(this_way.correction_pos, ntohl(seq)) {
        this_way.correction_pos = ntohl(seq);
        this_way.offset_before = this_way.offset_after;
        this_way.offset_after = this_way.offset_after.wrapping_add(off);
    }
    spin_unlock_bh(&mut (*ct).lock);
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_tcp_seqadj_set(skb: *mut SkBuff, ct: *mut NfConn,
                                                ctinfo: IpConntrackInfo, off: S32) {
    if nf_ct_protonum(ct) != IPPROTO_TCP { return; }
    let th = (skb_network_header(skb).add(ip_hdrlen(skb))) as *const Tcphdr;
    nf_ct_seqadj_set(ct, ctinfo, (*th).seq, off);
}

unsafe fn nf_ct_sack_block_adjust(skb: *mut SkBuff, tcph: *mut Tcphdr,
                                  mut sackoff: usize, sackend: usize,
                                  seq: *mut NfCtSeqadj) {
    while sackoff < sackend {
        let sack = (*skb).data.add(sackoff) as *mut TcpSackBlockWire;
        let start = ntohl((*sack).start_seq);
        let end = ntohl((*sack).end_seq);
        let new_start = if after(start.wrapping_sub((*seq).offset_before as u32), (*seq).correction_pos) {
            htonl(start.wrapping_sub((*seq).offset_after as u32))
        } else { htonl(start.wrapping_sub((*seq).offset_before as u32)) };
        let new_end = if after(end.wrapping_sub((*seq).offset_before as u32), (*seq).correction_pos) {
            htonl(end.wrapping_sub((*seq).offset_after as u32))
        } else { htonl(end.wrapping_sub((*seq).offset_before as u32)) };
        inet_proto_csum_replace4(&mut (*tcph).check, skb, (*sack).start_seq, new_start, false);
        inet_proto_csum_replace4(&mut (*tcph).check, skb, (*sack).end_seq, new_end, false);
        (*sack).start_seq = new_start;
        (*sack).end_seq = new_end;
        sackoff += core::mem::size_of::<TcpSackBlockWire>();
    }
}

unsafe fn nf_ct_sack_adjust(skb: *mut SkBuff, protoff: usize, ct: *mut NfConn,
                            ctinfo: IpConntrackInfo) -> u32 {
    let mut tcph = ((*skb).data.add(protoff)) as *mut Tcphdr;
    let seqadj = nfct_seqadj(ct);
    if seqadj.is_null() { return 0; }
    let mut optoff = protoff + core::mem::size_of::<Tcphdr>();
    let optend = protoff + ((*tcph).doff as usize) * 4;
    if skb_ensure_writable(skb, optend) != 0 { return 0; }
    tcph = (*skb).data.add(protoff) as *mut Tcphdr;
    let dir = ctinfo2dir(ctinfo);
    while optoff < optend {
        let op = (*skb).data.add(optoff);
        match *op {
            TCPOPT_EOL => return 1,
            TCPOPT_NOP => { optoff += 1; continue; }
            _ => {
                if optoff + 1 == optend || optoff + *op.add(1) as usize > optend || *op.add(1) < 2 { return 0; }
                if *op == TCPOPT_SACK && *op.add(1) as usize >= 2 + TCPOLEN_SACK_PERBLOCK &&
                   ((*op.add(1) as usize - 2) % TCPOLEN_SACK_PERBLOCK) == 0 {
                    nf_ct_sack_block_adjust(skb, tcph, optoff + 2, optoff + *op.add(1) as usize,
                                            &mut (*seqadj).seq[1 - dir]);
                }
                optoff += *op.add(1) as usize;
            }
        }
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_seq_adjust(skb: *mut SkBuff, ct: *mut NfConn,
                                            ctinfo: IpConntrackInfo, protoff: usize) -> i32 {
    let dir = ctinfo2dir(ctinfo);
    let seqadj = nfct_seqadj(ct);
    if seqadj.is_null() { return 0; }
    let this_way = &mut (*seqadj).seq[dir];
    let other_way = &mut (*seqadj).seq[1 - dir];
    if skb_ensure_writable(skb, protoff + core::mem::size_of::<Tcphdr>()) != 0 { return 0; }
    let tcph = (*skb).data.add(protoff) as *mut Tcphdr;
    spin_lock_bh(&mut (*ct).lock);
    let seqoff = if after(ntohl((*tcph).seq), this_way.correction_pos) { this_way.offset_after } else { this_way.offset_before };
    let newseq = htonl(ntohl((*tcph).seq).wrapping_add(seqoff as u32));
    inet_proto_csum_replace4(&mut (*tcph).check, skb, (*tcph).seq, newseq, false);
    (*tcph).seq = newseq;
    if (*tcph).ack == 0 { spin_unlock_bh(&mut (*ct).lock); return 1; }
    let ackoff = if after(ntohl((*tcph).ack_seq).wrapping_sub(other_way.offset_before as u32), other_way.correction_pos) { other_way.offset_after } else { other_way.offset_before };
    let newack = htonl(ntohl((*tcph).ack_seq).wrapping_sub(ackoff as u32));
    inet_proto_csum_replace4(&mut (*tcph).check, skb, (*tcph).ack_seq, newack, false);
    (*tcph).ack_seq = newack;
    let res = nf_ct_sack_adjust(skb, protoff, ct, ctinfo);
    spin_unlock_bh(&mut (*ct).lock);
    res as i32
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_seq_offset(ct: *const NfConn, dir: IpConntrackDir, seq: U32) -> S32 {
    let seqadj = nfct_seqadj(ct);
    if seqadj.is_null() { return 0; }
    let this_way = &(*seqadj).seq[dir];
    if after(seq, this_way.correction_pos) { this_way.offset_after } else { this_way.offset_before }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
