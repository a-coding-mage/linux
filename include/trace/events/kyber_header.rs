/* SPDX-License-Identifier: GPL-2.0 */

// Translation of trace/events/kyber.h.
// The Linux tracepoint machinery (TRACE_EVENT, TP_PROTO, TP_ARGS,
// TP_STRUCT__entry, TP_fast_assign, and TP_printk) is supplied externally.

pub const DOMAIN_LEN: usize = 16;
pub const LATENCY_TYPE_LEN: usize = 8;

#[repr(C)]
pub struct KyberLatencyEntry {
    pub dev: libc::dev_t,
    pub domain: [libc::c_char; DOMAIN_LEN],
    pub type_: [libc::c_char; LATENCY_TYPE_LEN],
    pub percentile: u8,
    pub numerator: u8,
    pub denominator: u8,
    pub samples: libc::c_uint,
}

#[repr(C)]
pub struct KyberAdjustEntry {
    pub dev: libc::dev_t,
    pub domain: [libc::c_char; DOMAIN_LEN],
    pub depth: libc::c_uint,
}

#[repr(C)]
pub struct KyberThrottledEntry {
    pub dev: libc::dev_t,
    pub domain: [libc::c_char; DOMAIN_LEN],
}

// Corresponds to TRACE_EVENT(kyber_latency,
//   TP_PROTO(dev_t dev, const char *domain, const char *type,
//            unsigned int percentile, unsigned int numerator,
//            unsigned int denominator, unsigned int samples),
//   TP_ARGS(dev, domain, type, percentile, numerator, denominator, samples)).
// The tracepoint registration, assignment (including strscpy), and printk
// formatting are provided by the external Linux tracepoint implementation:
// "%d,%d %s %s p%u %u/%u samples=%u".

// Corresponds to TRACE_EVENT(kyber_adjust,
//   TP_PROTO(dev_t dev, const char *domain, unsigned int depth),
//   TP_ARGS(dev, domain, depth)).
// Assignment copies domain with strscpy and the print format is
// "%d,%d %s %u".

// Corresponds to TRACE_EVENT(kyber_throttled,
//   TP_PROTO(dev_t dev, const char *domain),
//   TP_ARGS(dev, domain)).
// Assignment copies domain with strscpy and the print format is
// "%d,%d %s".


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
