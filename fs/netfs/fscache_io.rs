// SPDX-License-Identifier: GPL-2.0-or-later
/* Cache data I/O routines
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */
// FSCACHE_DEBUG_LEVEL OPERATION
// External kernel and internal dependencies are supplied by other translation units.

pub unsafe fn fscache_wait_for_operation(
    cres: *mut netfs_cache_resources,
    want_state: fscache_want_state,
) -> bool {
    let cookie = fscache_cres_cookie(cres);
    let mut state: fscache_cookie_state;

    loop {
        if !fscache_cache_is_live((*(*cookie).volume).cache) {
            _leave!(" [broken]");
            return false;
        }

        state = fscache_cookie_state(cookie);
        _enter!("c=%08x{%u},%x", (*cookie).debug_id, state, want_state);

        match state {
            FSCACHE_COOKIE_STATE_CREATING | FSCACHE_COOKIE_STATE_INVALIDATING => {
                if want_state == FSCACHE_WANT_PARAMS {
                    break;
                }
                loop {
                    match fscache_cookie_state(cookie) {
                        s if s != state => break,
                        _ => wait_var_event!(&mut (*cookie).state, fscache_cookie_state(cookie) != state),
                    }
                }
                continue;
            }
            FSCACHE_COOKIE_STATE_LOOKING_UP | FSCACHE_COOKIE_STATE_LRU_DISCARDING => {
                wait_var_event!(&mut (*cookie).state, fscache_cookie_state(cookie) != state);
                continue;
            }
            FSCACHE_COOKIE_STATE_ACTIVE => break,
            FSCACHE_COOKIE_STATE_DROPPED | FSCACHE_COOKIE_STATE_RELINQUISHING => {
                _leave!(" [not live]");
                return false;
            }
            _ => {
                _leave!(" [not live]");
                return false;
            }
        }
    }

    if (*cres).cache_priv2.is_null() {
        (*(*(*cookie).volume).cache).ops.begin_operation(cres, want_state)
    } else {
        true
    }
}

static unsafe fn fscache_begin_operation(
    cres: *mut netfs_cache_resources,
    cookie: *mut fscache_cookie,
    want_state: fscache_want_state,
    why: fscache_access_trace,
) -> i32 {
    let mut state: fscache_cookie_state;
    let mut timeo: c_long;
    let mut once_only = false;

    (*cres).ops = core::ptr::null_mut();
    (*cres).cache_priv = cookie as *mut c_void;
    (*cres).cache_priv2 = core::ptr::null_mut();
    (*cres).debug_id = (*cookie).debug_id;
    (*cres).inval_counter = (*cookie).inval_counter;

    if !fscache_begin_cookie_access(cookie, why) {
        (*cres).cache_priv = core::ptr::null_mut();
        return -ENOBUFS;
    }

    loop {
        spin_lock!(&mut (*cookie).lock);
        state = fscache_cookie_state(cookie);
        _enter!("c=%08x{%u},%x", (*cookie).debug_id, state, want_state);
        match state {
            FSCACHE_COOKIE_STATE_LOOKING_UP | FSCACHE_COOKIE_STATE_LRU_DISCARDING |
            FSCACHE_COOKIE_STATE_INVALIDATING => {}
            FSCACHE_COOKIE_STATE_CREATING => {
                if want_state == FSCACHE_WANT_PARAMS { goto_ready!(); }
            }
            FSCACHE_COOKIE_STATE_ACTIVE => { goto_ready!(); }
            FSCACHE_COOKIE_STATE_DROPPED | FSCACHE_COOKIE_STATE_RELINQUISHING => {
                WARN!(1, "Can't use cookie in state %u\n", (*cookie).state);
                spin_unlock!(&mut (*cookie).lock);
                break;
            }
            _ => { spin_unlock!(&mut (*cookie).lock); break; }
        }
        spin_unlock!(&mut (*cookie).lock);
        trace_fscache_access!((*cookie).debug_id, refcount_read!(&(*cookie).ref),
                              atomic_read!(&(*cookie).n_accesses), fscache_access_io_wait);
        timeo = wait_var_event_timeout!(&mut (*cookie).state,
                                        fscache_cookie_state(cookie) != state, 20 * HZ);
        if timeo <= 1 && !once_only {
            pr_warn!("{}: cookie state change wait timed out: cookie.state={} state={}",
                     core::module_path!(), fscache_cookie_state(cookie), state);
            fscache_print_cookie(cookie, 'O');
            once_only = true;
        }
    }

    (*cres).cache_priv = core::ptr::null_mut();
    (*cres).ops = core::ptr::null_mut();
    fscache_end_cookie_access(cookie, fscache_access_io_not_live);
    _leave!(" = -ENOBUFS");
    -ENOBUFS
}

pub unsafe fn __fscache_begin_read_operation(cres: *mut netfs_cache_resources, cookie: *mut fscache_cookie) -> i32 {
    fscache_begin_operation(cres, cookie, FSCACHE_WANT_PARAMS, fscache_access_io_read)
}

pub unsafe fn __fscache_begin_write_operation(cres: *mut netfs_cache_resources, cookie: *mut fscache_cookie) -> i32 {
    fscache_begin_operation(cres, cookie, FSCACHE_WANT_PARAMS, fscache_access_io_write)
}

#[repr(C)]
pub struct fscache_write_request {
    pub cache_resources: netfs_cache_resources,
    pub mapping: *mut address_space,
    pub start: loff_t,
    pub len: usize,
    pub set_bits: bool,
    pub using_pgpriv2: bool,
    pub term_func: netfs_io_terminated_t,
    pub term_func_priv: *mut c_void,
}

pub unsafe fn __fscache_clear_page_bits(mapping: *mut address_space, start: loff_t, len: usize) {
    let first = start / PAGE_SIZE;
    let last = (start + len as loff_t - 1) / PAGE_SIZE;
    if len != 0 {
        let mut xas = XA_STATE!((*mapping).i_pages, first);
        rcu_read_lock!();
        let mut page: *mut page;
        xas_for_each!(&mut xas, page, last, {
            folio_end_private_2(page_folio(page));
        });
        rcu_read_unlock!();
    }
}

static unsafe fn fscache_wreq_done(priv_: *mut c_void, transferred_or_error: ssize_t) {
    let wreq = priv_ as *mut fscache_write_request;
    if (*wreq).using_pgpriv2 {
        fscache_clear_page_bits((*wreq).mapping, (*wreq).start, (*wreq).len, (*wreq).set_bits);
    }
    if let Some(term) = (*wreq).term_func { term((*wreq).term_func_priv, transferred_or_error); }
    fscache_end_operation(&mut (*wreq).cache_resources);
    kfree!(wreq);
}

pub unsafe fn __fscache_write_to_cache(cookie: *mut fscache_cookie, mapping: *mut address_space,
    start: loff_t, len: usize, i_size: loff_t, term_func: netfs_io_terminated_t,
    term_func_priv: *mut c_void, using_pgpriv2: bool, cond: bool) {
    let mut ret = -ENOBUFS;
    if len == 0 { goto_abandon!(using_pgpriv2, mapping, start, len, cond, term_func, term_func_priv, ret); }
    _enter!("%llx,%zx", start, len);
    let wreq = kzalloc_obj!(fscache_write_request, GFP_NOFS);
    if wreq.is_null() { goto_abandon!(using_pgpriv2, mapping, start, len, cond, term_func, term_func_priv, ret); }
    (*wreq).mapping = mapping; (*wreq).start = start; (*wreq).len = len;
    (*wreq).using_pgpriv2 = using_pgpriv2; (*wreq).set_bits = cond;
    (*wreq).term_func = term_func; (*wreq).term_func_priv = term_func_priv;
    let cres = &mut (*wreq).cache_resources;
    if fscache_begin_operation(cres, cookie, FSCACHE_WANT_WRITE, fscache_access_io_write) < 0 { kfree!(wreq); goto_abandon!(using_pgpriv2, mapping, start, len, cond, term_func, term_func_priv, ret); }
    ret = (*(*cres).ops).prepare_write(cres, &mut (start as loff_t), &mut (len as usize), len, i_size, false);
    if ret < 0 { fscache_wreq_done(wreq as *mut c_void, ret as ssize_t); return; }
    let mut iter: iov_iter = core::mem::zeroed();
    iov_iter_xarray(&mut iter, ITER_SOURCE, &mut (*mapping).i_pages, start, len);
    fscache_write(cres, start, &mut iter, Some(fscache_wreq_done), wreq as *mut c_void);
}

pub unsafe fn __fscache_resize_cookie(cookie: *mut fscache_cookie, new_size: loff_t) {
    let mut cres: netfs_cache_resources = core::mem::zeroed();
    trace_fscache_resize!(cookie, new_size);
    if fscache_begin_operation(&mut cres, cookie, FSCACHE_WANT_WRITE, fscache_access_io_resize) == 0 {
        fscache_stat!(&fscache_n_resizes); set_bit!(FSCACHE_COOKIE_NEEDS_UPDATE, &mut (*cookie).flags);
        (*(*(*cookie).volume).cache).ops.resize_cookie(&mut cres, new_size);
        fscache_end_operation(&mut cres);
    } else { fscache_stat!(&fscache_n_resizes_null); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
