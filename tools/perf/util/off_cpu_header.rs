// Translated from perf/util/off_cpu.h.
// Original dependencies: <linux/perf_event.h>

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    _private: [u8; 0],
}

pub const OFFCPU_EVENT: &str = "offcpu-time";

pub const OFFCPU_SAMPLE_TYPES: u64 = PERF_SAMPLE_IDENTIFIER
    | PERF_SAMPLE_IP
    | PERF_SAMPLE_TID
    | PERF_SAMPLE_TIME
    | PERF_SAMPLE_ID
    | PERF_SAMPLE_CPU
    | PERF_SAMPLE_PERIOD
    | PERF_SAMPLE_RAW
    | PERF_SAMPLE_CGROUP;

pub const OFFCPU_THRESH: u64 = 500000000u64;

// C conditional:
// #ifdef HAVE_BPF_SKEL
// int off_cpu_prepare(struct evlist *evlist, struct target *target,
//                    struct record_opts *opts);
// int off_cpu_write(struct perf_session *session);
// #else
// static inline fallback definitions returning -1.

#[cfg(HAVE_BPF_SKEL)]
extern "C" {
    pub fn off_cpu_prepare(
        evlist: *mut evlist,
        target: *mut target,
        opts: *mut record_opts,
    ) -> ::std::os::raw::c_int;
    pub fn off_cpu_write(session: *mut perf_session) -> ::std::os::raw::c_int;
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn off_cpu_prepare(
    _evlist: *mut evlist,
    _target: *mut target,
    _opts: *mut record_opts,
) -> ::std::os::raw::c_int {
    -1
}

#[cfg(not(HAVE_BPF_SKEL))]
#[inline]
pub unsafe fn off_cpu_write(_session: *mut perf_session) -> ::std::os::raw::c_int {
    -1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
