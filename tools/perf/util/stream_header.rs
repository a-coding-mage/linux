/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/stream.h.

#[repr(C)]
pub struct callchain_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stream {
    pub cnode: *mut callchain_node,
    pub pair_cnode: *mut callchain_node,
}

#[repr(C)]
pub struct evsel_streams {
    pub streams: *mut stream,
    pub evsel: *const evsel,
    pub nr_streams_max: ::std::os::raw::c_int,
    pub nr_streams: ::std::os::raw::c_int,
    pub streams_hits: u64,
}

#[repr(C)]
pub struct evlist_streams {
    pub ev_streams: *mut evsel_streams,
    pub nr_evsel: ::std::os::raw::c_int,
}

extern "C" {
    pub fn evlist_streams__delete(els: *mut evlist_streams);

    pub fn evlist__create_streams(
        evlist: *mut evlist,
        nr_streams_max: ::std::os::raw::c_int,
    ) -> *mut evlist_streams;

    pub fn evsel_streams__entry(
        els: *mut evlist_streams,
        evsel: *const evsel,
    ) -> *mut evsel_streams;

    pub fn evsel_streams__match(es_base: *mut evsel_streams, es_pair: *mut evsel_streams);

    pub fn evsel_streams__report(es_base: *mut evsel_streams, es_pair: *mut evsel_streams);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
