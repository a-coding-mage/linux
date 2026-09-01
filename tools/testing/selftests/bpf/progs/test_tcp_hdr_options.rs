// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// Rust translation of testing/selftests/bpf/progs/test_tcp_hdr_options.c.
// C includes are intentionally not executable Rust; symbols from those headers
// are referenced below as external repository dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;

#[repr(C)]
union write_opt_union {
    exprm: tcp_exprm_opt,
    regular: tcp_opt,
}

#[repr(C)]
union search_opt_union {
    exprm: tcp_exprm_opt,
    regular: tcp_opt,
}

static mut test_kind: __u8 = TCPOPT_EXP;
static mut test_magic: __u16 = 0xeB9F;
static mut inherit_cb_flags: __u32 = 0;

static mut passive_synack_out: bpf_test_option = bpf_test_option {};
static mut passive_fin_out: bpf_test_option = bpf_test_option {};

static mut passive_estab_in: bpf_test_option = bpf_test_option {};
static mut passive_fin_in: bpf_test_option = bpf_test_option {};

static mut active_syn_out: bpf_test_option = bpf_test_option {};
static mut active_fin_out: bpf_test_option = bpf_test_option {};

static mut active_estab_in: bpf_test_option = bpf_test_option {};
static mut active_fin_in: bpf_test_option = bpf_test_option {};

// struct {
//     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, struct hdr_stg);
// } hdr_stg_map SEC(".maps");
#[no_mangle]
#[link_section = ".maps"]
static mut hdr_stg_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<hdr_stg>() as __u32,
    max_entries: 0,
};

unsafe fn skops_want_cookie(skops: *const bpf_sock_ops) -> bool {
    (*skops).args[0] == BPF_WRITE_HDR_TCP_SYNACK_COOKIE
}

unsafe fn skops_current_mss(skops: *const bpf_sock_ops) -> bool {
    (*skops).args[0] == BPF_WRITE_HDR_TCP_CURRENT_MSS
}

unsafe fn option_total_len(flags: __u8) -> __u8 {
    let mut i: __u8;
    let mut len: __u8 = 1; /* +1 for flags */

    if flags == 0 {
        return 0;
    }

    /* RESEND bit does not use a byte */
    i = OPTION_RESEND + 1;
    while i < __NR_OPTION_FLAGS {
        len = len.wrapping_add((TEST_OPTION_FLAGS(flags, i) != 0) as __u8);
        i = i.wrapping_add(1);
    }

    if test_kind == TCPOPT_EXP {
        len.wrapping_add(TCP_BPF_EXPOPT_BASE_LEN)
    } else {
        len.wrapping_add(2) /* +1 kind, +1 kind-len */
    }
}

unsafe fn write_test_option(test_opt: *const bpf_test_option, data: *mut __u8) {
    let mut offset: __u8 = 0;

    *data.add(offset as usize) = (*test_opt).flags;
    offset = offset.wrapping_add(1);
    if TEST_OPTION_FLAGS((*test_opt).flags, OPTION_MAX_DELACK_MS) != 0 {
        *data.add(offset as usize) = (*test_opt).max_delack_ms;
        offset = offset.wrapping_add(1);
    }

    if TEST_OPTION_FLAGS((*test_opt).flags, OPTION_RAND) != 0 {
        *data.add(offset as usize) = (*test_opt).rand;
    }
}

unsafe fn store_option(skops: *mut bpf_sock_ops, test_opt: *const bpf_test_option) -> i32 {
    let mut write_opt: write_opt_union = core::mem::zeroed();
    let err: i32;

    if test_kind == TCPOPT_EXP {
        write_opt.exprm.kind = TCPOPT_EXP;
        write_opt.exprm.len = option_total_len((*test_opt).flags);
        write_opt.exprm.magic = __bpf_htons(test_magic);
        write_opt.exprm.data32 = 0;
        write_test_option(test_opt, write_opt.exprm.data.as_mut_ptr());
        err = bpf_store_hdr_opt(
            skops,
            &mut write_opt.exprm as *mut tcp_exprm_opt as *mut core::ffi::c_void,
            core::mem::size_of::<tcp_exprm_opt>() as i32,
            0,
        );
    } else {
        write_opt.regular.kind = test_kind;
        write_opt.regular.len = option_total_len((*test_opt).flags);
        write_opt.regular.data32 = 0;
        write_test_option(test_opt, write_opt.regular.data.as_mut_ptr());
        err = bpf_store_hdr_opt(
            skops,
            &mut write_opt.regular as *mut tcp_opt as *mut core::ffi::c_void,
            core::mem::size_of::<tcp_opt>() as i32,
            0,
        );
    }

    if err != 0 {
        return RET_CG_ERR(err);
    }

    CG_OK
}

unsafe fn parse_test_option(opt: *mut bpf_test_option, mut start: *const __u8) -> i32 {
    (*opt).flags = *start;
    start = start.add(1);

    if TEST_OPTION_FLAGS((*opt).flags, OPTION_MAX_DELACK_MS) != 0 {
        (*opt).max_delack_ms = *start;
        start = start.add(1);
    }

    if TEST_OPTION_FLAGS((*opt).flags, OPTION_RAND) != 0 {
        (*opt).rand = *start;
    }

    0
}

unsafe fn load_option(
    skops: *mut bpf_sock_ops,
    test_opt: *mut bpf_test_option,
    from_syn: bool,
) -> i32 {
    let mut search_opt: search_opt_union = core::mem::zeroed();
    let ret: i32;
    let load_flags: i32 = if from_syn { BPF_LOAD_HDR_OPT_TCP_SYN } else { 0 };

    if test_kind == TCPOPT_EXP {
        search_opt.exprm.kind = TCPOPT_EXP;
        search_opt.exprm.len = 4;
        search_opt.exprm.magic = __bpf_htons(test_magic);
        search_opt.exprm.data32 = 0;
        ret = bpf_load_hdr_opt(
            skops,
            &mut search_opt.exprm as *mut tcp_exprm_opt as *mut core::ffi::c_void,
            core::mem::size_of::<tcp_exprm_opt>() as i32,
            load_flags,
        );
        if ret < 0 {
            return ret;
        }
        return parse_test_option(test_opt, search_opt.exprm.data.as_ptr());
    } else {
        search_opt.regular.kind = test_kind;
        search_opt.regular.len = 0;
        search_opt.regular.data32 = 0;
        ret = bpf_load_hdr_opt(
            skops,
            &mut search_opt.regular as *mut tcp_opt as *mut core::ffi::c_void,
            core::mem::size_of::<tcp_opt>() as i32,
            load_flags,
        );
        if ret < 0 {
            return ret;
        }
        return parse_test_option(test_opt, search_opt.regular.data.as_ptr());
    }
}

unsafe fn synack_opt_len(skops: *mut bpf_sock_ops) -> i32 {
    let mut test_opt: bpf_test_option = core::mem::zeroed();
    let optlen: __u8;
    let err: i32;

    if passive_synack_out.flags == 0 {
        return CG_OK;
    }

    err = load_option(skops, &mut test_opt, true);

    /* bpf_test_option is not found */
    if err == -ENOMSG {
        return CG_OK;
    }

    if err != 0 {
        return RET_CG_ERR(err);
    }

    optlen = option_total_len(passive_synack_out.flags);
    if optlen != 0 {
        let reserve_err = bpf_reserve_hdr_opt(skops, optlen as i32, 0);
        if reserve_err != 0 {
            return RET_CG_ERR(reserve_err);
        }
    }

    CG_OK
}

unsafe fn write_synack_opt(skops: *mut bpf_sock_ops) -> i32 {
    let mut opt: bpf_test_option;

    if passive_synack_out.flags == 0 {
        /* We should not even be called since no header
         * space has been reserved.
         */
        return RET_CG_ERR(0);
    }

    opt = passive_synack_out;
    if skops_want_cookie(skops) {
        SET_OPTION_FLAGS(&mut opt.flags, OPTION_RESEND);
    }

    store_option(skops, &opt)
}

unsafe fn syn_opt_len(skops: *mut bpf_sock_ops) -> i32 {
    let optlen: __u8;
    let err: i32;

    if active_syn_out.flags == 0 {
        return CG_OK;
    }

    optlen = option_total_len(active_syn_out.flags);
    if optlen != 0 {
        err = bpf_reserve_hdr_opt(skops, optlen as i32, 0);
        if err != 0 {
            return RET_CG_ERR(err);
        }
    }

    CG_OK
}

unsafe fn write_syn_opt(skops: *mut bpf_sock_ops) -> i32 {
    if active_syn_out.flags == 0 {
        return RET_CG_ERR(0);
    }

    store_option(skops, &active_syn_out)
}

unsafe fn fin_opt_len(skops: *mut bpf_sock_ops) -> i32 {
    let opt: *mut bpf_test_option;
    let hdr_stg: *mut hdr_stg;
    let optlen: __u8;
    let err: i32;

    if (*skops).sk.is_null() {
        return RET_CG_ERR(0);
    }

    hdr_stg = bpf_sk_storage_get(&mut hdr_stg_map, (*skops).sk, core::ptr::null_mut(), 0)
        as *mut hdr_stg;
    if hdr_stg.is_null() {
        return RET_CG_ERR(0);
    }

    if (*hdr_stg).active {
        opt = &mut active_fin_out;
    } else {
        opt = &mut passive_fin_out;
    }

    optlen = option_total_len((*opt).flags);
    if optlen != 0 {
        err = bpf_reserve_hdr_opt(skops, optlen as i32, 0);
        if err != 0 {
            return RET_CG_ERR(err);
        }
    }

    CG_OK
}

unsafe fn write_fin_opt(skops: *mut bpf_sock_ops) -> i32 {
    let opt: *mut bpf_test_option;
    let hdr_stg: *mut hdr_stg;

    if (*skops).sk.is_null() {
        return RET_CG_ERR(0);
    }

    hdr_stg = bpf_sk_storage_get(&mut hdr_stg_map, (*skops).sk, core::ptr::null_mut(), 0)
        as *mut hdr_stg;
    if hdr_stg.is_null() {
        return RET_CG_ERR(0);
    }

    if (*hdr_stg).active {
        opt = &mut active_fin_out;
    } else {
        opt = &mut passive_fin_out;
    }

    if (*opt).flags == 0 {
        return RET_CG_ERR(0);
    }

    store_option(skops, opt)
}

unsafe fn resend_in_ack(skops: *mut bpf_sock_ops) -> i32 {
    let hdr_stg: *mut hdr_stg;

    if (*skops).sk.is_null() {
        return -1;
    }

    hdr_stg = bpf_sk_storage_get(&mut hdr_stg_map, (*skops).sk, core::ptr::null_mut(), 0)
        as *mut hdr_stg;
    if hdr_stg.is_null() {
        return -1;
    }

    ((*hdr_stg).resend_syn != 0) as i32
}

unsafe fn nodata_opt_len(skops: *mut bpf_sock_ops) -> i32 {
    let resend: i32;

    resend = resend_in_ack(skops);
    if resend < 0 {
        return RET_CG_ERR(0);
    }

    if resend != 0 {
        return syn_opt_len(skops);
    }

    CG_OK
}

unsafe fn write_nodata_opt(skops: *mut bpf_sock_ops) -> i32 {
    let resend: i32;

    resend = resend_in_ack(skops);
    if resend < 0 {
        return RET_CG_ERR(0);
    }

    if resend != 0 {
        return write_syn_opt(skops);
    }

    CG_OK
}

unsafe fn data_opt_len(skops: *mut bpf_sock_ops) -> i32 {
    /* Same as the nodata version.  Mostly to show
     * an example usage on skops->skb_len.
     */
    nodata_opt_len(skops)
}

unsafe fn write_data_opt(skops: *mut bpf_sock_ops) -> i32 {
    write_nodata_opt(skops)
}

unsafe fn current_mss_opt_len(skops: *mut bpf_sock_ops) -> i32 {
    /* Reserve maximum that may be needed */
    let err: i32;

    err = bpf_reserve_hdr_opt(skops, option_total_len(OPTION_MASK) as i32, 0);
    if err != 0 {
        return RET_CG_ERR(err);
    }

    CG_OK
}

unsafe fn handle_hdr_opt_len(skops: *mut bpf_sock_ops) -> i32 {
    let tcp_flags: __u8 = skops_tcp_flags(skops);

    if (tcp_flags & TCPHDR_SYNACK) == TCPHDR_SYNACK {
        return synack_opt_len(skops);
    }

    if (tcp_flags & TCPHDR_SYN) != 0 {
        return syn_opt_len(skops);
    }

    if (tcp_flags & TCPHDR_FIN) != 0 {
        return fin_opt_len(skops);
    }

    if skops_current_mss(skops) {
        /* The kernel is calculating the MSS */
        return current_mss_opt_len(skops);
    }

    if (*skops).skb_len != 0 {
        return data_opt_len(skops);
    }

    nodata_opt_len(skops)
}

unsafe fn handle_write_hdr_opt(skops: *mut bpf_sock_ops) -> i32 {
    let tcp_flags: __u8 = skops_tcp_flags(skops);
    let th: *mut tcphdr;

    if (tcp_flags & TCPHDR_SYNACK) == TCPHDR_SYNACK {
        return write_synack_opt(skops);
    }

    if (tcp_flags & TCPHDR_SYN) != 0 {
        return write_syn_opt(skops);
    }

    if (tcp_flags & TCPHDR_FIN) != 0 {
        return write_fin_opt(skops);
    }

    th = (*skops).skb_data as *mut tcphdr;
    if th.add(1) > (*skops).skb_data_end as *mut tcphdr {
        return RET_CG_ERR(0);
    }

    if (*skops).skb_len > tcp_hdrlen(th) {
        return write_data_opt(skops);
    }

    write_nodata_opt(skops)
}

unsafe fn set_delack_max(skops: *mut bpf_sock_ops, max_delack_ms: __u8) -> i32 {
    let mut max_delack_us: __u32 = (max_delack_ms as __u32).wrapping_mul(1000);

    bpf_setsockopt(
        skops,
        SOL_TCP,
        TCP_BPF_DELACK_MAX,
        &mut max_delack_us as *mut __u32 as *mut core::ffi::c_void,
        core::mem::size_of::<__u32>() as i32,
    )
}

unsafe fn set_rto_min(skops: *mut bpf_sock_ops, peer_max_delack_ms: __u8) -> i32 {
    let mut min_rto_us: __u32 = (peer_max_delack_ms as __u32).wrapping_mul(1000);

    bpf_setsockopt(
        skops,
        SOL_TCP,
        TCP_BPF_RTO_MIN,
        &mut min_rto_us as *mut __u32 as *mut core::ffi::c_void,
        core::mem::size_of::<__u32>() as i32,
    )
}

unsafe fn handle_active_estab(skops: *mut bpf_sock_ops) -> i32 {
    let mut init_stg: hdr_stg = core::mem::zeroed();
    init_stg.active = true;
    let mut err: i32;

    err = load_option(skops, &mut active_estab_in, false);
    if err != 0 && err != -ENOMSG {
        return RET_CG_ERR(err);
    }

    init_stg.resend_syn = TEST_OPTION_FLAGS(active_estab_in.flags, OPTION_RESEND) != 0;
    if (*skops).sk.is_null()
        || bpf_sk_storage_get(
            &mut hdr_stg_map,
            (*skops).sk,
            &mut init_stg as *mut hdr_stg as *mut core::ffi::c_void,
            BPF_SK_STORAGE_GET_F_CREATE,
        )
        .is_null()
    {
        return RET_CG_ERR(0);
    }

    if init_stg.resend_syn {
        /* Don't clear the write_hdr cb now because
         * the ACK may get lost and retransmit may
         * be needed.
         *
         * PARSE_ALL_HDR cb flag is set to learn if this
         * resend_syn option has received by the peer.
         *
         * The header option will be resent until a valid
         * packet is received at handle_parse_hdr()
         * and all hdr cb flags will be cleared in
         * handle_parse_hdr().
         */
        set_parse_all_hdr_cb_flags(skops);
    } else if active_fin_out.flags == 0 {
        /* No options will be written from now */
        clear_hdr_cb_flags(skops);
    }

    if active_syn_out.max_delack_ms != 0 {
        err = set_delack_max(skops, active_syn_out.max_delack_ms);
        if err != 0 {
            return RET_CG_ERR(err);
        }
    }

    if active_estab_in.max_delack_ms != 0 {
        err = set_rto_min(skops, active_estab_in.max_delack_ms);
        if err != 0 {
            return RET_CG_ERR(err);
        }
    }

    CG_OK
}

unsafe fn handle_passive_estab(skops: *mut bpf_sock_ops) -> i32 {
    let mut init_stg: hdr_stg = core::mem::zeroed();
    let th: *mut tcphdr;
    let mut err: i32;

    inherit_cb_flags = (*skops).bpf_sock_ops_cb_flags;

    err = load_option(skops, &mut passive_estab_in, true);
    if err == -ENOENT {
        /* saved_syn is not found. It was in syncookie mode.
         * We have asked the active side to resend the options
         * in ACK, so try to find the bpf_test_option from ACK now.
         */
        err = load_option(skops, &mut passive_estab_in, false);
        init_stg.syncookie = true;
    }

    /* ENOMSG: The bpf_test_option is not found which is fine.
     * Bail out now for all other errors.
     */
    if err != 0 && err != -ENOMSG {
        return RET_CG_ERR(err);
    }

    th = (*skops).skb_data as *mut tcphdr;
    if th.add(1) > (*skops).skb_data_end as *mut tcphdr {
        return RET_CG_ERR(0);
    }

    if (*th).syn != 0 {
        /* Fastopen */

        /* Cannot clear cb_flags to stop write_hdr cb.
         * synack is not sent yet for fast open.
         * Even it was, the synack may need to be retransmitted.
         *
         * PARSE_ALL_HDR cb flag is set to learn
         * if synack has reached the peer.
         * All cb_flags will be cleared in handle_parse_hdr().
         */
        set_parse_all_hdr_cb_flags(skops);
        init_stg.fastopen = true;
    } else if passive_fin_out.flags == 0 {
        /* No options will be written from now */
        clear_hdr_cb_flags(skops);
    }

    if (*skops).sk.is_null()
        || bpf_sk_storage_get(
            &mut hdr_stg_map,
            (*skops).sk,
            &mut init_stg as *mut hdr_stg as *mut core::ffi::c_void,
            BPF_SK_STORAGE_GET_F_CREATE,
        )
        .is_null()
    {
        return RET_CG_ERR(0);
    }

    if passive_synack_out.max_delack_ms != 0 {
        err = set_delack_max(skops, passive_synack_out.max_delack_ms);
        if err != 0 {
            return RET_CG_ERR(err);
        }
    }

    if passive_estab_in.max_delack_ms != 0 {
        err = set_rto_min(skops, passive_estab_in.max_delack_ms);
        if err != 0 {
            return RET_CG_ERR(err);
        }
    }

    CG_OK
}

unsafe fn handle_parse_hdr(skops: *mut bpf_sock_ops) -> i32 {
    let hdr_stg: *mut hdr_stg;
    let th: *mut tcphdr;

    if (*skops).sk.is_null() {
        return RET_CG_ERR(0);
    }

    th = (*skops).skb_data as *mut tcphdr;
    if th.add(1) > (*skops).skb_data_end as *mut tcphdr {
        return RET_CG_ERR(0);
    }

    hdr_stg = bpf_sk_storage_get(&mut hdr_stg_map, (*skops).sk, core::ptr::null_mut(), 0)
        as *mut hdr_stg;
    if hdr_stg.is_null() {
        return RET_CG_ERR(0);
    }

    if (*hdr_stg).resend_syn || (*hdr_stg).fastopen {
        /* The PARSE_ALL_HDR cb flag was turned on
         * to ensure that the previously written
         * options have reached the peer.
         * Those previously written option includes:
         *     - Active side: resend_syn in ACK during syncookie
         *      or
         *     - Passive side: SYNACK during fastopen
         *
         * A valid packet has been received here after
         * the 3WHS, so the PARSE_ALL_HDR cb flag
         * can be cleared now.
         */
        clear_parse_all_hdr_cb_flags(skops);
    }

    if (*hdr_stg).resend_syn && active_fin_out.flags == 0 {
        /* Active side resent the syn option in ACK
         * because the server was in syncookie mode.
         * A valid packet has been received, so
         * clear header cb flags if there is no
         * more option to send.
         */
        clear_hdr_cb_flags(skops);
    }

    if (*hdr_stg).fastopen && passive_fin_out.flags == 0 {
        /* Passive side was in fastopen.
         * A valid packet has been received, so
         * the SYNACK has reached the peer.
         * Clear header cb flags if there is no more
         * option to send.
         */
        clear_hdr_cb_flags(skops);
    }

    if (*th).fin != 0 {
        let fin_opt: *mut bpf_test_option;
        let err: i32;

        if (*hdr_stg).active {
            fin_opt = &mut active_fin_in;
        } else {
            fin_opt = &mut passive_fin_in;
        }

        err = load_option(skops, fin_opt, false);
        if err != 0 && err != -ENOMSG {
            return RET_CG_ERR(err);
        }
    }

    CG_OK
}

#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn estab(skops: *mut bpf_sock_ops) -> i32 {
    let mut true_val: i32 = 1;

    match (*skops).op {
        BPF_SOCK_OPS_TCP_LISTEN_CB => {
            bpf_setsockopt(
                skops,
                SOL_TCP,
                TCP_SAVE_SYN,
                &mut true_val as *mut i32 as *mut core::ffi::c_void,
                core::mem::size_of::<i32>() as i32,
            );
            set_hdr_cb_flags(skops, BPF_SOCK_OPS_STATE_CB_FLAG);
        }
        BPF_SOCK_OPS_TCP_CONNECT_CB => {
            set_hdr_cb_flags(skops, 0);
        }
        BPF_SOCK_OPS_PARSE_HDR_OPT_CB => {
            return handle_parse_hdr(skops);
        }
        BPF_SOCK_OPS_HDR_OPT_LEN_CB => {
            return handle_hdr_opt_len(skops);
        }
        BPF_SOCK_OPS_WRITE_HDR_OPT_CB => {
            return handle_write_hdr_opt(skops);
        }
        BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => {
            return handle_passive_estab(skops);
        }
        BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB => {
            return handle_active_estab(skops);
        }
        _ => {}
    }

    CG_OK
}

#[no_mangle]
#[link_section = "license"]
static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
