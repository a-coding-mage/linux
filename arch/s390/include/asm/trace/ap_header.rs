/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Tracepoint definitions for s390 ap bus related trace events
 *
 * There are two AP bus related tracepoint events defined here:
 * There is a tracepoint s390_ap_nqap event immediately after a request
 * has been pushed into the AP firmware queue with the NQAP AP command.
 * The other tracepoint s390_ap_dqap event fires immediately after a
 * reply has been pulled out of the AP firmware queue via DQAP AP command.
 * The idea of these two trace events focuses on performance to measure
 * the runtime of a crypto request/reply as close as possible at the
 * firmware level. In combination with the two zcrypt tracepoints (see
 * the zcrypt.h trace event definition file) this gives measurement data
 * about the runtime of a request/reply within the zcrpyt and AP bus layer.
 */

// TRACE_SYSTEM = s390
// The original header guard and tracepoint include are C preprocessor
// machinery; their dependency intent is retained here as comments.

#[repr(C)]
pub struct S390ApNqapdqapEntry {
    pub card: u16,
    pub dom: u16,
    pub status: u32,
    pub psmid: u64,
}

/// Format used by the original TP_printk tracepoint.
pub const S390_AP_NQAPDQAP_PRINTK: &str =
    "card=%u dom=%u status=0x%08x psmid=0x%016lx";

/**
 * trace_s390_ap_nqap - ap msg nqap tracepoint function
 * @card:   Crypto card number addressed.
 * @dom:    Domain within the crypto card addressed.
 * @status: AP queue status (GR1 on return of nqap).
 * @psmid:  Unique id identifying this request/reply.
 *
 * Called immediately after a request has been enqueued into
 * the AP firmware queue with the NQAP command.
 */
extern "C" {
    pub fn s390_ap_nqap(card: u16, dom: u16, status: u32, psmid: u64);
}

/**
 * trace_s390_ap_dqap - ap msg dqap tracepoint function
 * @card:  Crypto card number addressed.
 * @dom:   Domain within the crypto card addressed.
 * @status: AP queue status (GR1 on return of dqap).
 * @psmid: Unique id identifying this request/reply.
 *
 * Called immediately after a reply has been dequeued from
 * the AP firmware queue with the DQAP command.
 */
extern "C" {
    pub fn s390_ap_dqap(card: u16, dom: u16, status: u32, psmid: u64);
}

// TRACE_INCLUDE_PATH = asm/trace
// TRACE_INCLUDE_FILE = ap
// The original trace/define_trace.h include supplies the tracepoint
// implementation and remains an external dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
