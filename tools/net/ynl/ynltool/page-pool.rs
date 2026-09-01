// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)

use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem;
use std::ptr;

const IF_NAMESIZE: usize = 16;

type SizeT = usize;
type U64 = u64;

#[repr(C)]
struct pp_stat_live {
    cnt: c_uint,
    refs: SizeT,
    bytes: SizeT,
}

#[repr(C)]
struct pp_stat {
    ifc: c_uint,

    live: [pp_stat_live; 2],

    alloc_slow: SizeT,
    alloc_fast: SizeT,
    recycle_ring: SizeT,
    recycle_cache: SizeT,
}

#[repr(C)]
struct pp_stats_array {
    i: c_uint,
    max: c_uint,
    s: *mut pp_stat,
}

#[repr(C)]
struct ynl_error {
    msg: *const c_char,
}

#[repr(C)]
struct ynl_sock_err {
    msg: *const c_char,
}

#[repr(C)]
struct ynl_sock {
    err: ynl_sock_err,
}

#[repr(C)]
struct ynl_family;

#[repr(C)]
struct json_writer;

#[repr(C)]
struct cmd {
    name: *const c_char,
    func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

#[repr(C)]
struct netdev_page_pool_get_req_dump {
    _unused: [u8; 0],
}

#[repr(C)]
struct netdev_page_pool_stats_get_req_dump {
    _unused: [u8; 0],
}

#[repr(C)]
struct netdev_page_pool_get_present {
    ifindex: bool,
    napi_id: bool,
    inflight: bool,
    inflight_mem: bool,
    detach_time: bool,
    dmabuf: bool,
}

#[repr(C)]
struct netdev_page_pool_get_rsp {
    _present: netdev_page_pool_get_present,
    id: U64,
    ifindex: c_uint,
    napi_id: U64,
    inflight: U64,
    inflight_mem: U64,
    detach_time: U64,
    dmabuf: c_uint,
}

#[repr(C)]
struct netdev_page_pool_info_present {
    id: bool,
}

#[repr(C)]
struct netdev_page_pool_info {
    _present: netdev_page_pool_info_present,
    id: U64,
    ifindex: c_uint,
}

#[repr(C)]
struct netdev_page_pool_stats_get_present {
    info: bool,
    alloc_fast: bool,
    alloc_refill: bool,
    alloc_slow: bool,
    recycle_ring: bool,
    recycle_cached: bool,
}

#[repr(C)]
struct netdev_page_pool_stats_get_rsp {
    _present: netdev_page_pool_stats_get_present,
    info: netdev_page_pool_info,
    alloc_fast: SizeT,
    alloc_refill: SizeT,
    alloc_slow: SizeT,
    recycle_ring: SizeT,
    recycle_cached: SizeT,
}

/* ynl_dump_foreach() is a C macro supplied by generated YNL support.
 * This translation models the list storage it iterates over as a pointer plus
 * count so the macro's loops can be represented directly.
 */
#[repr(C)]
struct netdev_page_pool_get_list {
    n: SizeT,
    objs: *mut netdev_page_pool_get_rsp,
}

#[repr(C)]
struct netdev_page_pool_stats_get_list {
    n: SizeT,
    objs: *mut netdev_page_pool_stats_get_rsp,
}

unsafe extern "C" {
    static mut json_wtr: *mut json_writer;
    static mut json_output: bool;
    static mut bin_name: *const c_char;
    static ynl_netdev_family: ynl_family;

    fn reallocarray(ptr: *mut c_void, nmemb: SizeT, size: SizeT) -> *mut c_void;
    fn calloc(nmemb: SizeT, size: SizeT) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;

    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;

    fn jsonw_float_field(wr: *mut json_writer, prop: *const c_char, num: f64);
    fn jsonw_name(wr: *mut json_writer, prop: *const c_char);
    fn jsonw_start_object(wr: *mut json_writer);
    fn jsonw_end_object(wr: *mut json_writer);
    fn jsonw_start_array(wr: *mut json_writer);
    fn jsonw_end_array(wr: *mut json_writer);
    fn jsonw_uint_field(wr: *mut json_writer, prop: *const c_char, num: SizeT);
    fn jsonw_string_field(wr: *mut json_writer, prop: *const c_char, val: *const c_char);
    fn jsonw_null(wr: *mut json_writer);

    fn is_prefix(prefix: *const c_char, string: *const c_char) -> bool;
    fn p_err(format: *const c_char, ...);
    fn cmd_select(
        cmds: *const cmd,
        argc: c_int,
        argv: *mut *mut c_char,
        help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int,
    ) -> c_int;

    fn ynl_sock_create(family: *const ynl_family, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn netdev_page_pool_get_dump(
        ys: *mut ynl_sock,
        req: *mut netdev_page_pool_get_req_dump,
    ) -> *mut netdev_page_pool_get_list;
    fn netdev_page_pool_stats_get_dump(
        ys: *mut ynl_sock,
        req: *mut netdev_page_pool_stats_get_req_dump,
    ) -> *mut netdev_page_pool_stats_get_list;
    fn netdev_page_pool_get_list_free(list: *mut netdev_page_pool_get_list);
    fn netdev_page_pool_stats_get_list_free(list: *mut netdev_page_pool_stats_get_list);
}

unsafe fn find_ifc(a: *mut pp_stats_array, ifindex: c_uint) -> *mut pp_stat {
    let mut i: c_uint = 0;

    while i < (*a).i {
        if (*(*a).s.add(i as usize)).ifc == ifindex {
            return (*a).s.add(i as usize);
        }
        i += 1;
    }

    (*a).i += 1;
    if (*a).i == (*a).max {
        (*a).max *= 2;
        (*a).s = reallocarray(
            (*a).s as *mut c_void,
            (*a).max as SizeT,
            mem::size_of::<pp_stat>(),
        ) as *mut pp_stat;
    }
    (*(*a).s.add(i as usize)).ifc = ifindex;
    (*a).s.add(i as usize)
}

unsafe fn count_pool(s: *mut pp_stat, l: c_uint, pp: *mut netdev_page_pool_get_rsp) {
    (*s).live[l as usize].cnt += 1;
    if (*pp)._present.inflight {
        (*s).live[l as usize].refs += (*pp).inflight as SizeT;
    }
    if (*pp)._present.inflight_mem {
        (*s).live[l as usize].bytes += (*pp).inflight_mem as SizeT;
    }
}

/* We don't know how many pages are sitting in cache and ring
 * so we will under-count the recycling rate a bit.
 */
unsafe fn print_json_recycling_stats(s: *mut pp_stat) {
    let recycle: f64;

    if (*s).alloc_fast + (*s).alloc_slow != 0 {
        recycle = ((*s).recycle_ring + (*s).recycle_cache) as f64
            / ((*s).alloc_fast + (*s).alloc_slow) as f64
            * 100.0;
        jsonw_float_field(json_wtr, c"recycling_pct".as_ptr(), recycle);
    }

    jsonw_name(json_wtr, c"alloc".as_ptr());
    jsonw_start_object(json_wtr);
    jsonw_uint_field(json_wtr, c"slow".as_ptr(), (*s).alloc_slow);
    jsonw_uint_field(json_wtr, c"fast".as_ptr(), (*s).alloc_fast);
    jsonw_end_object(json_wtr);

    jsonw_name(json_wtr, c"recycle".as_ptr());
    jsonw_start_object(json_wtr);
    jsonw_uint_field(json_wtr, c"ring".as_ptr(), (*s).recycle_ring);
    jsonw_uint_field(json_wtr, c"cache".as_ptr(), (*s).recycle_cache);
    jsonw_end_object(json_wtr);
}

unsafe fn print_plain_recycling_stats(s: *mut pp_stat) {
    let recycle: f64;

    if (*s).alloc_fast + (*s).alloc_slow != 0 {
        recycle = ((*s).recycle_ring + (*s).recycle_cache) as f64
            / ((*s).alloc_fast + (*s).alloc_slow) as f64
            * 100.0;
        printf(
            c"recycling: %.1lf%% (alloc: %zu:%zu recycle: %zu:%zu)".as_ptr(),
            recycle,
            (*s).alloc_slow,
            (*s).alloc_fast,
            (*s).recycle_ring,
            (*s).recycle_cache,
        );
    }
}

unsafe fn print_json_stats(a: *mut pp_stats_array) {
    jsonw_start_array(json_wtr);

    let mut i: c_uint = 0;
    while i < (*a).i {
        let mut ifname = [0 as c_char; IF_NAMESIZE];
        let s = (*a).s.add(i as usize);
        let name: *const c_char;

        jsonw_start_object(json_wtr);

        if (*s).ifc == 0 {
            jsonw_string_field(json_wtr, c"ifname".as_ptr(), c"<orphan>".as_ptr());
            jsonw_uint_field(json_wtr, c"ifindex".as_ptr(), 0);
        } else {
            name = if_indextoname((*s).ifc, ifname.as_mut_ptr());
            if !name.is_null() {
                jsonw_string_field(json_wtr, c"ifname".as_ptr(), name);
            }
            jsonw_uint_field(json_wtr, c"ifindex".as_ptr(), (*s).ifc as SizeT);
        }

        jsonw_uint_field(json_wtr, c"page_pools".as_ptr(), (*s).live[1].cnt as SizeT);
        jsonw_uint_field(json_wtr, c"zombies".as_ptr(), (*s).live[0].cnt as SizeT);

        jsonw_name(json_wtr, c"live".as_ptr());
        jsonw_start_object(json_wtr);
        jsonw_uint_field(json_wtr, c"refs".as_ptr(), (*s).live[1].refs);
        jsonw_uint_field(json_wtr, c"bytes".as_ptr(), (*s).live[1].bytes);
        jsonw_end_object(json_wtr);

        jsonw_name(json_wtr, c"zombie".as_ptr());
        jsonw_start_object(json_wtr);
        jsonw_uint_field(json_wtr, c"refs".as_ptr(), (*s).live[0].refs);
        jsonw_uint_field(json_wtr, c"bytes".as_ptr(), (*s).live[0].bytes);
        jsonw_end_object(json_wtr);

        if (*s).alloc_fast != 0 || (*s).alloc_slow != 0 {
            print_json_recycling_stats(s);
        }

        jsonw_end_object(json_wtr);
        i += 1;
    }

    jsonw_end_array(json_wtr);
}

unsafe fn print_plain_stats(a: *mut pp_stats_array) {
    let mut i: c_uint = 0;
    while i < (*a).i {
        let mut ifname = [0 as c_char; IF_NAMESIZE];
        let s = (*a).s.add(i as usize);
        let name: *const c_char;

        if (*s).ifc == 0 {
            printf(c"<orphan>\t".as_ptr());
        } else {
            name = if_indextoname((*s).ifc, ifname.as_mut_ptr());
            if !name.is_null() {
                printf(c"%8s".as_ptr(), name);
            }
            printf(c"[%u]\t".as_ptr(), (*s).ifc);
        }

        printf(
            c"page pools: %u (zombies: %u)\n".as_ptr(),
            (*s).live[1].cnt,
            (*s).live[0].cnt,
        );
        printf(
            c"\t\trefs: %zu bytes: %zu (refs: %zu bytes: %zu)\n".as_ptr(),
            (*s).live[1].refs,
            (*s).live[1].bytes,
            (*s).live[0].refs,
            (*s).live[0].bytes,
        );

        if (*s).alloc_fast != 0 || (*s).alloc_slow != 0 {
            printf(c"\t\t".as_ptr());
            print_plain_recycling_stats(s);
            printf(c"\n".as_ptr());
        }
        i += 1;
    }
}

unsafe fn find_pool_stat_in_list(
    pp_stats: *mut netdev_page_pool_stats_get_list,
    pool_id: U64,
    pstat: *mut pp_stat,
) -> bool {
    let mut i: SizeT = 0;
    while i < (*pp_stats).n {
        let pp = (*pp_stats).objs.add(i);
        if !(*pp)._present.info || !(*pp).info._present.id {
            i += 1;
            continue;
        }
        if (*pp).info.id != pool_id {
            i += 1;
            continue;
        }

        memset(pstat as *mut c_void, 0, mem::size_of::<pp_stat>());
        if (*pp)._present.alloc_fast {
            (*pstat).alloc_fast = (*pp).alloc_fast;
        }
        if (*pp)._present.alloc_refill {
            (*pstat).alloc_fast += (*pp).alloc_refill;
        }
        if (*pp)._present.alloc_slow {
            (*pstat).alloc_slow = (*pp).alloc_slow;
        }
        if (*pp)._present.recycle_ring {
            (*pstat).recycle_ring = (*pp).recycle_ring;
        }
        if (*pp)._present.recycle_cached {
            (*pstat).recycle_cache = (*pp).recycle_cached;
        }
        return true;
    }
    false
}

unsafe fn print_json_pool_list(
    pools: *mut netdev_page_pool_get_list,
    pp_stats: *mut netdev_page_pool_stats_get_list,
    zombies_only: bool,
) {
    jsonw_start_array(json_wtr);

    let mut i: SizeT = 0;
    while i < (*pools).n {
        let pp = (*pools).objs.add(i);
        let mut ifname = [0 as c_char; IF_NAMESIZE];
        let mut pstat: pp_stat = mem::zeroed();
        let name: *const c_char;

        if zombies_only && !(*pp)._present.detach_time {
            i += 1;
            continue;
        }

        jsonw_start_object(json_wtr);

        jsonw_uint_field(json_wtr, c"id".as_ptr(), (*pp).id as SizeT);

        if (*pp)._present.ifindex {
            name = if_indextoname((*pp).ifindex, ifname.as_mut_ptr());
            if !name.is_null() {
                jsonw_string_field(json_wtr, c"ifname".as_ptr(), name);
            }
            jsonw_uint_field(json_wtr, c"ifindex".as_ptr(), (*pp).ifindex as SizeT);
        }

        if (*pp)._present.napi_id {
            jsonw_uint_field(json_wtr, c"napi_id".as_ptr(), (*pp).napi_id as SizeT);
        }

        if (*pp)._present.inflight {
            jsonw_uint_field(json_wtr, c"refs".as_ptr(), (*pp).inflight as SizeT);
        }

        if (*pp)._present.inflight_mem {
            jsonw_uint_field(json_wtr, c"bytes".as_ptr(), (*pp).inflight_mem as SizeT);
        }

        if (*pp)._present.detach_time {
            jsonw_uint_field(json_wtr, c"detach_time".as_ptr(), (*pp).detach_time as SizeT);
        }

        if (*pp)._present.dmabuf {
            jsonw_uint_field(json_wtr, c"dmabuf".as_ptr(), (*pp).dmabuf as SizeT);
        }

        if find_pool_stat_in_list(pp_stats, (*pp).id, &mut pstat)
            && (pstat.alloc_fast != 0 || pstat.alloc_slow != 0)
        {
            print_json_recycling_stats(&mut pstat);
        }

        jsonw_end_object(json_wtr);
        i += 1;
    }

    jsonw_end_array(json_wtr);
}

unsafe fn print_plain_pool_list(
    pools: *mut netdev_page_pool_get_list,
    pp_stats: *mut netdev_page_pool_stats_get_list,
    zombies_only: bool,
) {
    let mut i: SizeT = 0;
    while i < (*pools).n {
        let pp = (*pools).objs.add(i);
        let mut ifname = [0 as c_char; IF_NAMESIZE];
        let mut pstat: pp_stat = mem::zeroed();
        let name: *const c_char;

        if zombies_only && !(*pp)._present.detach_time {
            i += 1;
            continue;
        }

        printf(c"pool id: %llu".as_ptr(), (*pp).id);

        if (*pp)._present.ifindex {
            name = if_indextoname((*pp).ifindex, ifname.as_mut_ptr());
            if !name.is_null() {
                printf(c"  dev: %s".as_ptr(), name);
            }
            printf(c"[%u]".as_ptr(), (*pp).ifindex);
        }

        if (*pp)._present.napi_id {
            printf(c"  napi: %llu".as_ptr(), (*pp).napi_id);
        }

        printf(c"\n".as_ptr());

        if (*pp)._present.inflight || (*pp)._present.inflight_mem {
            printf(c"  inflight:".as_ptr());
            if (*pp)._present.inflight {
                printf(c" %llu pages".as_ptr(), (*pp).inflight);
            }
            if (*pp)._present.inflight_mem {
                printf(c" %llu bytes".as_ptr(), (*pp).inflight_mem);
            }
            printf(c"\n".as_ptr());
        }

        if (*pp)._present.detach_time {
            printf(c"  detached: %llu\n".as_ptr(), (*pp).detach_time);
        }

        if (*pp)._present.dmabuf {
            printf(c"  dmabuf: %u\n".as_ptr(), (*pp).dmabuf);
        }

        if find_pool_stat_in_list(pp_stats, (*pp).id, &mut pstat)
            && (pstat.alloc_fast != 0 || pstat.alloc_slow != 0)
        {
            printf(c"  ".as_ptr());
            print_plain_recycling_stats(&mut pstat);
            printf(c"\n".as_ptr());
        }

        i += 1;
    }
}

unsafe fn aggregate_device_stats(
    a: *mut pp_stats_array,
    pools: *mut netdev_page_pool_get_list,
    pp_stats: *mut netdev_page_pool_stats_get_list,
) {
    let mut i: SizeT = 0;
    while i < (*pools).n {
        let pp = (*pools).objs.add(i);
        let s = find_ifc(a, (*pp).ifindex);

        count_pool(s, 1, pp);
        if (*pp)._present.detach_time {
            count_pool(s, 0, pp);
        }
        i += 1;
    }

    let mut j: SizeT = 0;
    while j < (*pp_stats).n {
        let pp = (*pp_stats).objs.add(j);
        let s = find_ifc(a, (*pp).info.ifindex);

        if (*pp)._present.alloc_fast {
            (*s).alloc_fast += (*pp).alloc_fast;
        }
        if (*pp)._present.alloc_refill {
            (*s).alloc_fast += (*pp).alloc_refill;
        }
        if (*pp)._present.alloc_slow {
            (*s).alloc_slow += (*pp).alloc_slow;
        }
        if (*pp)._present.recycle_ring {
            (*s).recycle_ring += (*pp).recycle_ring;
        }
        if (*pp)._present.recycle_cached {
            (*s).recycle_cache += (*pp).recycle_cached;
        }
        j += 1;
    }
}

unsafe fn do_stats(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut pp_stat_req: netdev_page_pool_stats_get_req_dump = mem::zeroed();
    let pp_stats: *mut netdev_page_pool_stats_get_list;
    let mut pp_req: netdev_page_pool_get_req_dump = mem::zeroed();
    let pools: *mut netdev_page_pool_get_list;
    const GROUP_BY_DEVICE: c_int = 0;
    const GROUP_BY_POOL: c_int = 1;
    let mut group_by: c_int = GROUP_BY_DEVICE;
    let mut zombies_only = false;
    let mut a: pp_stats_array = mem::zeroed();
    let mut yerr: ynl_error = mem::zeroed();
    let ys: *mut ynl_sock;
    let mut ret: c_int = 0;

    /* Parse options */
    while argc > 0 {
        if is_prefix(*argv, c"group-by".as_ptr()) {
            argv = argv.add(1);
            argc -= 1;

            if argc < 1 {
                return -1;
            }

            if is_prefix(*argv, c"device".as_ptr()) {
                group_by = GROUP_BY_DEVICE;
            } else if is_prefix(*argv, c"pp".as_ptr())
                || is_prefix(*argv, c"page-pool".as_ptr())
                || is_prefix(*argv, c"none".as_ptr())
            {
                group_by = GROUP_BY_POOL;
            } else {
                p_err(c"invalid group-by value '%s'".as_ptr(), *argv);
                return -1;
            }
            argv = argv.add(1);
            argc -= 1;
        } else if is_prefix(*argv, c"zombies".as_ptr()) {
            zombies_only = true;
            group_by = GROUP_BY_POOL;
            argv = argv.add(1);
            argc -= 1;
        } else {
            p_err(c"unknown option '%s'".as_ptr(), *argv);
            return -1;
        }
    }

    ys = ynl_sock_create(&ynl_netdev_family, &mut yerr);
    if ys.is_null() {
        p_err(c"YNL: %s".as_ptr(), yerr.msg);
        return -1;
    }

    pools = netdev_page_pool_get_dump(ys, &mut pp_req);
    if pools.is_null() {
        p_err(c"failed to get page pools: %s".as_ptr(), (*ys).err.msg);
        ret = -1;
        ynl_sock_destroy(ys);
        return ret;
    }

    pp_stats = netdev_page_pool_stats_get_dump(ys, &mut pp_stat_req);
    if pp_stats.is_null() {
        p_err(c"failed to get page pool stats: %s".as_ptr(), (*ys).err.msg);
        ret = -1;
        netdev_page_pool_get_list_free(pools);
        ynl_sock_destroy(ys);
        return ret;
    }

    /* If grouping by pool, print individual pools */
    if group_by == GROUP_BY_POOL {
        if json_output {
            print_json_pool_list(pools, pp_stats, zombies_only);
        } else {
            print_plain_pool_list(pools, pp_stats, zombies_only);
        }
    } else {
        /* Aggregated stats mode (group-by device) */
        a.max = 64;
        a.s = calloc(a.max as SizeT, mem::size_of::<pp_stat>()) as *mut pp_stat;
        if a.s.is_null() {
            p_err(c"failed to allocate stats array".as_ptr());
            ret = -1;
            netdev_page_pool_stats_get_list_free(pp_stats);
            netdev_page_pool_get_list_free(pools);
            ynl_sock_destroy(ys);
            return ret;
        }

        aggregate_device_stats(&mut a, pools, pp_stats);

        if json_output {
            print_json_stats(&mut a);
        } else {
            print_plain_stats(&mut a);
        }

        free(a.s as *mut c_void);
    }

    netdev_page_pool_stats_get_list_free(pp_stats);
    netdev_page_pool_get_list_free(pools);
    ynl_sock_destroy(ys);
    ret
}

unsafe extern "C" fn do_help(
    _argc: c_int,
    _argv: *mut *mut c_char,
) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }

    fprintf(
        stderr,
        c"Usage: %s page-pool { COMMAND | help }\n       %s page-pool stats [ OPTIONS ]\n\n       OPTIONS := { group-by { device | page-pool | none } | zombies }\n\n       stats                   - Display page pool statistics\n       stats group-by device   - Group statistics by network device (default)\n       stats group-by page-pool | pp | none\n                               - Show individual page pool details (no grouping)\n       stats zombies           - Show only zombie page pools (detached but with\n                                 pages in flight). Implies group-by page-pool.\n".as_ptr(),
        bin_name,
        bin_name,
    );

    0
}

unsafe extern "C" fn do_stats_cmd(argc: c_int, argv: *mut *mut c_char) -> c_int {
    do_stats(argc, argv)
}

static page_pool_cmds: [cmd; 3] = [
    cmd {
        name: c"help".as_ptr(),
        func: Some(do_help),
    },
    cmd {
        name: c"stats".as_ptr(),
        func: Some(do_stats_cmd),
    },
    cmd {
        name: ptr::null(),
        func: None,
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_page_pool(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(page_pool_cmds.as_ptr(), argc, argv, do_help)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
