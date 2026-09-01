// SPDX-License-Identifier: GPL-2.0
/*
 * Compare and figure out the top N hottest streams
 * Copyright (c) 2020, Intel Corporation.
 * Author: Jin Yao
 */

use core::ffi::{c_char, c_int, c_long, c_void};

type u64 = u64;

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
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
pub struct hists {
    pub entries: rb_root_cached,
}

#[repr(C)]
pub struct hist_entry {
    pub rb_node: rb_node,
    pub sorted_chain: rb_root,
}

#[repr(C)]
pub struct callchain_node {
    pub rb_node: rb_node,
    pub val: list_head,
    pub hit: u64,
}

#[repr(C)]
pub struct callchain_list {
    pub list: list_head,
}

#[repr(C)]
pub struct stream {
    pub cnode: *mut callchain_node,
    pub pair_cnode: *mut callchain_node,
}

#[repr(C)]
pub struct evsel_streams {
    pub streams: *mut stream,
    pub nr_streams: c_int,
    pub nr_streams_max: c_int,
    pub streams_hits: u64,
    pub evsel: *mut evsel,
}

#[repr(C)]
pub struct evlist_streams {
    pub ev_streams: *mut evsel_streams,
    pub nr_evsel: c_int,
}

unsafe extern "C" {
    fn free(ptr: *mut c_void);
    fn zalloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;

    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;

    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn hists__output_resort(hists: *mut hists, cb: *mut c_void);

    fn callchain_total_hits(hists: *mut hists) -> u64;
    fn callchain_avg_cycles(cnode: *mut callchain_node) -> c_long;
    fn callchain_cnode_matched(
        base_cnode: *mut callchain_node,
        pair_cnode: *mut callchain_node,
    ) -> bool;
    fn callchain_list__sym_name(
        chain: *mut callchain_list,
        bf: *mut c_char,
        bfsize: usize,
        show_dso: bool,
    ) -> *mut c_char;

    fn evlist__first_entry(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next_entry(evlist: *mut evlist, pos: *mut evsel) -> *mut evsel;
}

unsafe fn BUG_ON(cond: bool) {
    if cond {
        core::intrinsics::abort();
    }
}

unsafe fn rb_entry_callchain_node(node: *mut rb_node) -> *mut callchain_node {
    node as *mut callchain_node
}

unsafe fn rb_entry_hist_entry(node: *mut rb_node) -> *mut hist_entry {
    node as *mut hist_entry
}

unsafe fn list_first_entry_callchain_list(head: *mut list_head) -> *mut callchain_list {
    (*head).next as *mut callchain_list
}

unsafe fn list_next_entry_callchain_list(pos: *mut callchain_list) -> *mut callchain_list {
    (*pos).list.next as *mut callchain_list
}

unsafe fn evsel_streams__delete(es: *mut evsel_streams, nr_evsel: c_int) {
    for i in 0..nr_evsel {
        zfree(&mut (*es.add(i as usize)).streams as *mut *mut stream as *mut *mut c_void);
    }

    free(es as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn evlist_streams__delete(els: *mut evlist_streams) {
    evsel_streams__delete((*els).ev_streams, (*els).nr_evsel);
    free(els as *mut c_void);
}

unsafe fn evlist_streams__new(
    nr_evsel: c_int,
    nr_streams_max: c_int,
) -> *mut evlist_streams {
    let els: *mut evlist_streams = zalloc(core::mem::size_of::<evlist_streams>()) as *mut _;
    let es: *mut evsel_streams;

    if els.is_null() {
        return core::ptr::null_mut();
    }

    es = calloc(
        nr_evsel as usize,
        core::mem::size_of::<evsel_streams>(),
    ) as *mut _;
    if es.is_null() {
        free(els as *mut c_void);
        return core::ptr::null_mut();
    }

    for i in 0..nr_evsel {
        let s = es.add(i as usize);

        (*s).streams = calloc(
            nr_streams_max as usize,
            core::mem::size_of::<stream>(),
        ) as *mut _;
        if (*s).streams.is_null() {
            evsel_streams__delete(es, nr_evsel);
            return core::ptr::null_mut();
        }

        (*s).nr_streams_max = nr_streams_max;
    }

    (*els).ev_streams = es;
    (*els).nr_evsel = nr_evsel;
    els
}

/*
 * The cnodes with high hit number are hot callchains.
 */
unsafe fn evsel_streams__set_hot_cnode(es: *mut evsel_streams, cnode: *mut callchain_node) {
    let mut idx: c_int = 0;
    let mut hit: u64;

    if (*es).nr_streams < (*es).nr_streams_max {
        let i = (*es).nr_streams;
        (*(*es).streams.add(i as usize)).cnode = cnode;
        (*es).nr_streams += 1;
        return;
    }

    /*
     * Considering a few number of hot streams, only use simple
     * way to find the cnode with smallest hit number and replace.
     */
    hit = (*(*(*es).streams.add(0)).cnode).hit;
    for i in 1..(*es).nr_streams {
        if (*(*(*es).streams.add(i as usize)).cnode).hit < hit {
            hit = (*(*(*es).streams.add(i as usize)).cnode).hit;
            idx = i;
        }
    }

    if (*cnode).hit > hit {
        (*(*es).streams.add(idx as usize)).cnode = cnode;
    }
}

unsafe fn update_hot_callchain(he: *mut hist_entry, es: *mut evsel_streams) {
    let root = &mut (*he).sorted_chain as *mut rb_root;
    let mut rb_node = rb_first(root);
    let mut cnode: *mut callchain_node;

    while !rb_node.is_null() {
        cnode = rb_entry_callchain_node(rb_node);
        evsel_streams__set_hot_cnode(es, cnode);
        rb_node = rb_next(rb_node);
    }
}

unsafe fn init_hot_callchain(hists: *mut hists, es: *mut evsel_streams) {
    let mut next = rb_first_cached(&mut (*hists).entries as *mut rb_root_cached);

    while !next.is_null() {
        let he: *mut hist_entry;

        he = rb_entry_hist_entry(next);
        update_hot_callchain(he, es);
        next = rb_next(&mut (*he).rb_node as *mut rb_node);
    }

    (*es).streams_hits = callchain_total_hits(hists);
}

unsafe fn evlist__init_callchain_streams(
    evlist: *mut evlist,
    els: *mut evlist_streams,
) -> c_int {
    let es = (*els).ev_streams;
    let mut pos: *mut evsel;
    let mut i: c_int = 0;

    BUG_ON((*els).nr_evsel < evlist__nr_entries(evlist));

    pos = evlist__first_entry(evlist);
    while !pos.is_null() {
        let hists = evsel__hists(pos);

        hists__output_resort(hists, core::ptr::null_mut());
        init_hot_callchain(hists, es.add(i as usize));
        (*es.add(i as usize)).evsel = pos;
        i += 1;

        pos = evlist__next_entry(evlist, pos);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn evlist__create_streams(
    evlist: *mut evlist,
    nr_streams_max: c_int,
) -> *mut evlist_streams {
    let nr_evsel = evlist__nr_entries(evlist);
    let mut ret: c_int = -1;
    let els = evlist_streams__new(nr_evsel, nr_streams_max);

    if els.is_null() {
        return core::ptr::null_mut();
    }

    ret = evlist__init_callchain_streams(evlist, els);
    if ret != 0 {
        evlist_streams__delete(els);
        return core::ptr::null_mut();
    }

    els
}

#[no_mangle]
pub unsafe extern "C" fn evsel_streams__entry(
    els: *mut evlist_streams,
    evsel: *const evsel,
) -> *mut evsel_streams {
    let es = (*els).ev_streams;

    for i in 0..(*els).nr_evsel {
        if (*es.add(i as usize)).evsel == evsel as *mut evsel {
            return es.add(i as usize);
        }
    }

    core::ptr::null_mut()
}

unsafe fn stream__callchain_match(
    base_stream: *mut stream,
    es_pair: *mut evsel_streams,
) -> *mut stream {
    for i in 0..(*es_pair).nr_streams {
        let pair_stream = (*es_pair).streams.add(i as usize);

        if callchain_cnode_matched((*base_stream).cnode, (*pair_stream).cnode) {
            return pair_stream;
        }
    }

    core::ptr::null_mut()
}

unsafe fn stream__match(base_stream: *mut stream, es_pair: *mut evsel_streams) -> *mut stream {
    stream__callchain_match(base_stream, es_pair)
}

unsafe fn stream__link(base_stream: *mut stream, pair_stream: *mut stream) {
    (*base_stream).pair_cnode = (*pair_stream).cnode;
    (*pair_stream).pair_cnode = (*base_stream).cnode;
}

#[no_mangle]
pub unsafe extern "C" fn evsel_streams__match(
    es_base: *mut evsel_streams,
    es_pair: *mut evsel_streams,
) {
    for i in 0..(*es_base).nr_streams {
        let base_stream = (*es_base).streams.add(i as usize);
        let pair_stream: *mut stream;

        pair_stream = stream__match(base_stream, es_pair);
        if !pair_stream.is_null() {
            stream__link(base_stream, pair_stream);
        }
    }
}

unsafe fn print_callchain_pair(
    base_stream: *mut stream,
    idx: c_int,
    es_base: *mut evsel_streams,
    es_pair: *mut evsel_streams,
) {
    let base_cnode = (*base_stream).cnode;
    let pair_cnode = (*base_stream).pair_cnode;
    let mut base_chain: *mut callchain_list;
    let mut pair_chain: *mut callchain_list;
    let mut buf1 = [0 as c_char; 512];
    let mut buf2 = [0 as c_char; 512];
    let mut cbuf1 = [0 as c_char; 256];
    let mut cbuf2 = [0 as c_char; 256];
    let mut s1: *mut c_char;
    let mut s2: *mut c_char;
    let mut pct: f64;

    printf(b"\nhot chain pair %d:\n\0".as_ptr() as *const c_char, idx);

    pct = (*base_cnode).hit as f64 / (*es_base).streams_hits as f64;
    scnprintf(
        buf1.as_mut_ptr(),
        buf1.len(),
        b"cycles: %ld, hits: %.2f%%\0".as_ptr() as *const c_char,
        callchain_avg_cycles(base_cnode),
        pct * 100.0,
    );

    pct = (*pair_cnode).hit as f64 / (*es_pair).streams_hits as f64;
    scnprintf(
        buf2.as_mut_ptr(),
        buf2.len(),
        b"cycles: %ld, hits: %.2f%%\0".as_ptr() as *const c_char,
        callchain_avg_cycles(pair_cnode),
        pct * 100.0,
    );

    printf(
        b"%35s\t%35s\n\0".as_ptr() as *const c_char,
        buf1.as_ptr(),
        buf2.as_ptr(),
    );

    printf(
        b"%35s\t%35s\n\0".as_ptr() as *const c_char,
        b"---------------------------\0".as_ptr() as *const c_char,
        b"--------------------------\0".as_ptr() as *const c_char,
    );

    pair_chain = list_first_entry_callchain_list(&mut (*pair_cnode).val as *mut list_head);

    base_chain = list_first_entry_callchain_list(&mut (*base_cnode).val as *mut list_head);
    while &mut (*base_chain).list as *mut list_head != &mut (*base_cnode).val as *mut list_head {
        if &mut (*pair_chain).list as *mut list_head == &mut (*pair_cnode).val as *mut list_head {
            return;
        }

        s1 = callchain_list__sym_name(base_chain, cbuf1.as_mut_ptr(), cbuf1.len(), false);
        s2 = callchain_list__sym_name(pair_chain, cbuf2.as_mut_ptr(), cbuf2.len(), false);

        scnprintf(
            buf1.as_mut_ptr(),
            buf1.len(),
            b"%35s\t%35s\0".as_ptr() as *const c_char,
            s1,
            s2,
        );
        printf(b"%s\n\0".as_ptr() as *const c_char, buf1.as_ptr());
        pair_chain = list_next_entry_callchain_list(pair_chain);
        base_chain = list_next_entry_callchain_list(base_chain);
    }
}

unsafe fn print_stream_callchain(
    stream: *mut stream,
    idx: c_int,
    es: *mut evsel_streams,
    pair: bool,
) {
    let cnode = (*stream).cnode;
    let mut chain: *mut callchain_list;
    let mut buf = [0 as c_char; 512];
    let mut cbuf = [0 as c_char; 256];
    let mut s: *mut c_char;
    let pct: f64;

    printf(b"\nhot chain %d:\n\0".as_ptr() as *const c_char, idx);

    pct = (*cnode).hit as f64 / (*es).streams_hits as f64;
    scnprintf(
        buf.as_mut_ptr(),
        buf.len(),
        b"cycles: %ld, hits: %.2f%%\0".as_ptr() as *const c_char,
        callchain_avg_cycles(cnode),
        pct * 100.0,
    );

    if pair {
        printf(
            b"%35s\t%35s\n\0".as_ptr() as *const c_char,
            b"\0".as_ptr() as *const c_char,
            buf.as_ptr(),
        );
        printf(
            b"%35s\t%35s\n\0".as_ptr() as *const c_char,
            b"\0".as_ptr() as *const c_char,
            b"--------------------------\0".as_ptr() as *const c_char,
        );
    } else {
        printf(b"%35s\n\0".as_ptr() as *const c_char, buf.as_ptr());
        printf(
            b"%35s\n\0".as_ptr() as *const c_char,
            b"--------------------------\0".as_ptr() as *const c_char,
        );
    }

    chain = list_first_entry_callchain_list(&mut (*cnode).val as *mut list_head);
    while &mut (*chain).list as *mut list_head != &mut (*cnode).val as *mut list_head {
        s = callchain_list__sym_name(chain, cbuf.as_mut_ptr(), cbuf.len(), false);

        if pair {
            scnprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"%35s\t%35s\0".as_ptr() as *const c_char,
                b"\0".as_ptr() as *const c_char,
                s,
            );
        } else {
            scnprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"%35s\0".as_ptr() as *const c_char,
                s,
            );
        }

        printf(b"%s\n\0".as_ptr() as *const c_char, buf.as_ptr());
        chain = list_next_entry_callchain_list(chain);
    }
}

unsafe fn callchain_streams_report(es_base: *mut evsel_streams, es_pair: *mut evsel_streams) {
    let mut base_stream: *mut stream;
    let mut idx: c_int = 0;

    printf(b"[ Matched hot streams ]\n\0".as_ptr() as *const c_char);
    for i in 0..(*es_base).nr_streams {
        base_stream = (*es_base).streams.add(i as usize);
        if !(*base_stream).pair_cnode.is_null() {
            idx += 1;
            print_callchain_pair(base_stream, idx, es_base, es_pair);
        }
    }

    idx = 0;
    printf(b"\n[ Hot streams in old perf data only ]\n\0".as_ptr() as *const c_char);
    for i in 0..(*es_base).nr_streams {
        base_stream = (*es_base).streams.add(i as usize);
        if (*base_stream).pair_cnode.is_null() {
            idx += 1;
            print_stream_callchain(base_stream, idx, es_base, false);
        }
    }

    idx = 0;
    printf(b"\n[ Hot streams in new perf data only ]\n\0".as_ptr() as *const c_char);
    for i in 0..(*es_pair).nr_streams {
        base_stream = (*es_pair).streams.add(i as usize);
        if (*base_stream).pair_cnode.is_null() {
            idx += 1;
            print_stream_callchain(base_stream, idx, es_pair, true);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn evsel_streams__report(
    es_base: *mut evsel_streams,
    es_pair: *mut evsel_streams,
) {
    return callchain_streams_report(es_base, es_pair);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
