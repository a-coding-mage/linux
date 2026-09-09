/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM sctp
// C header guard: _TRACE_SCTP_H / TRACE_HEADER_MULTI_READ
// Dependencies supplied by the surrounding kernel translation:
// <net/sctp/structs.h>, <linux/tracepoint.h>, and <trace/define_trace.h>.

#[repr(C)]
pub struct sctp_probe_path_entry {
    pub asoc: u64,
    pub primary: u32,
    pub ipaddr: [u8; core::mem::size_of::<union_sctp_addr>()],
    pub state: u32,
    pub cwnd: u32,
    pub ssthresh: u32,
    pub flight_size: u32,
    pub partial_bytes_acked: u32,
    pub pathmtu: u32,
}

#[repr(C)]
pub struct sctp_probe_entry {
    pub asoc: u64,
    pub mark: u32,
    pub bind_port: u16,
    pub peer_port: u16,
    pub pathmtu: u32,
    pub rwnd: u32,
    pub unack_data: u16,
}

// Opaque types and fields below are provided by net/sctp/structs.h.
// Their names and member access preserve the corresponding C expressions.
extern "C" {
    pub type sctp_transport;
    pub type sctp_association;
    pub type sctp_endpoint;
    pub type sctp_chunk;
    pub type union_sctp_addr;

    pub fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
}

#[inline]
pub unsafe fn sctp_probe_path_fast_assign(
    entry: *mut sctp_probe_path_entry,
    sp: *mut sctp_transport,
    asoc: *const sctp_association,
) {
    // __entry->asoc = (unsigned long)asoc;
    (*entry).asoc = asoc as usize as u64;
    // __entry->primary = (sp == asoc->peer.primary_path);
    (*entry).primary = (sp == (*asoc).peer.primary_path) as u32;
    // memcpy(__entry->ipaddr, &sp->ipaddr, sizeof(union sctp_addr));
    memcpy(
        (*entry).ipaddr.as_mut_ptr() as *mut core::ffi::c_void,
        &(*sp).ipaddr as *const _ as *const core::ffi::c_void,
        core::mem::size_of::<union_sctp_addr>(),
    );
    (*entry).state = (*sp).state;
    (*entry).cwnd = (*sp).cwnd;
    (*entry).ssthresh = (*sp).ssthresh;
    (*entry).flight_size = (*sp).flight_size;
    (*entry).partial_bytes_acked = (*sp).partial_bytes_acked;
    (*entry).pathmtu = (*sp).pathmtu;
}

pub const SCTP_PROBE_PATH_PRINTK: &str =
    "asoc=%#llx%s ipaddr=%pISpc state=%u cwnd=%u ssthresh=%u flight_size=%u partial_bytes_acked=%u pathmtu=%u";

#[inline]
pub unsafe fn sctp_probe_fast_assign(
    entry: *mut sctp_probe_entry,
    ep: *const sctp_endpoint,
    asoc: *const sctp_association,
    chunk: *mut sctp_chunk,
) {
    // struct sk_buff *skb = chunk->skb;
    let skb = (*chunk).skb;

    (*entry).asoc = asoc as usize as u64;
    (*entry).mark = (*skb).mark;
    (*entry).bind_port = (*ep).base.bind_addr.port;
    (*entry).peer_port = (*asoc).peer.port;
    (*entry).pathmtu = (*asoc).pathmtu;
    (*entry).rwnd = (*asoc).peer.rwnd;
    (*entry).unack_data = (*asoc).unack_data;
}

pub const SCTP_PROBE_PRINTK: &str =
    "asoc=%#llx mark=%#x bind_port=%d peer_port=%d pathmtu=%d rwnd=%u unack_data=%d";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
