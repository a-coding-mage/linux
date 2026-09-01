// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/*
 * Translated from C source. External kernel/BPF types, constants, helpers,
 * section attributes, and tcp option helpers are expected from the surrounding
 * selftest environment corresponding to the original includes:
 * <stddef.h>, <errno.h>, <stdbool.h>, <sys/types.h>, <sys/socket.h>,
 * <linux/ipv6.h>, <linux/tcp.h>, <linux/socket.h>, <linux/bpf.h>,
 * <linux/types.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>, and
 * "test_tcp_hdr_options.h" with BPF_PROG_TEST_TCP_HDR_OPTIONS defined.
 */

use core::mem::size_of;

type __u8 = u8;
type __u16 = u16;
type __u64 = u64;

extern "C" {
    fn __bpf_htons(x: __u16) -> __u16;
    fn bpf_load_hdr_opt(
        skops: *mut bpf_sock_ops,
        searchby_res: *mut core::ffi::c_void,
        len: i32,
        flags: __u64,
    ) -> i32;
    fn bpf_getsockopt(
        skops: *mut bpf_sock_ops,
        level: i32,
        optname: i32,
        optval: *mut core::ffi::c_void,
        optlen: i32,
    ) -> i32;
    fn bpf_reserve_hdr_opt(skops: *mut bpf_sock_ops, len: i32, flags: __u64) -> i32;
    fn bpf_store_hdr_opt(
        skops: *mut bpf_sock_ops,
        from: *const core::ffi::c_void,
        len: i32,
        flags: __u64,
    ) -> i32;
    fn bpf_sock_ops_cb_flags_set(skops: *mut bpf_sock_ops, flags: i32) -> i32;
    fn bpf_setsockopt(
        skops: *mut bpf_sock_ops,
        level: i32,
        optname: i32,
        optval: *const core::ffi::c_void,
        optlen: i32,
    ) -> i32;
    fn skops_tcp_flags(skops: *mut bpf_sock_ops) -> __u8;
    fn set_hdr_cb_flags(skops: *mut bpf_sock_ops, flags: i32);
    fn tcp_hdrlen(th: *mut tcphdr) -> i32;
}

macro_rules! RET_CG_ERR {
    ($ret:expr) => {{
        let _ = $ret;
        return CG_ERR;
    }};
}

#[repr(C)]
union hdr_union {
    th: tcphdr,
    ip6: ipv6hdr,
    exprm_opt: tcp_exprm_opt,
    reg_opt: tcp_opt,
    data: [__u8; 100], /* IPv6 (40) + Max TCP hdr (60) */
}

static mut last_addr16_n: __u16 = unsafe { __bpf_htons(1) };
static mut active_lport_n: __u16 = 0;
static mut active_lport_h: __u16 = 0;
static mut passive_lport_n: __u16 = 0;
static mut passive_lport_h: __u16 = 0;

/* options received at passive side */
static mut nr_pure_ack: u32 = 0;
static mut nr_data: u32 = 0;
static mut nr_syn: u32 = 0;
static mut nr_fin: u32 = 0;
static mut nr_hwtstamp: u32 = 0;

static mut nodelay_est_ok: bool = false;
static mut nodelay_hdr_len_reject: bool = false;
static mut nodelay_write_hdr_reject: bool = false;

/* Check the header received from the active side */
unsafe fn __check_active_hdr_in(skops: *mut bpf_sock_ops, check_syn: bool) -> i32 {
    let mut hdr: hdr_union = core::mem::zeroed();
    let load_flags: __u64 = if check_syn {
        BPF_LOAD_HDR_OPT_TCP_SYN
    } else {
        0
    };
    let mut pth: *mut tcphdr;
    let mut ret: i32;

    hdr.reg_opt.kind = 0xB9;

    /* The option is 4 bytes long instead of 2 bytes */
    ret = bpf_load_hdr_opt(
        skops,
        &mut hdr.reg_opt as *mut tcp_opt as *mut core::ffi::c_void,
        2,
        load_flags,
    );
    if ret != -ENOSPC {
        RET_CG_ERR!(ret);
    }

    /* Test searching magic with regular kind */
    hdr.reg_opt.len = 4;
    ret = bpf_load_hdr_opt(
        skops,
        &mut hdr.reg_opt as *mut tcp_opt as *mut core::ffi::c_void,
        size_of::<tcp_opt>() as i32,
        load_flags,
    );
    if ret != -EINVAL {
        RET_CG_ERR!(ret);
    }

    hdr.reg_opt.len = 0;
    ret = bpf_load_hdr_opt(
        skops,
        &mut hdr.reg_opt as *mut tcp_opt as *mut core::ffi::c_void,
        size_of::<tcp_opt>() as i32,
        load_flags,
    );
    if ret != 4
        || hdr.reg_opt.len != 4
        || hdr.reg_opt.kind != 0xB9
        || hdr.reg_opt.data[0] != 0xfa
        || hdr.reg_opt.data[1] != 0xce
    {
        RET_CG_ERR!(ret);
    }

    /* Test searching experimental option with invalid kind length */
    hdr.exprm_opt.kind = TCPOPT_EXP;
    hdr.exprm_opt.len = 5;
    hdr.exprm_opt.magic = 0;
    ret = bpf_load_hdr_opt(
        skops,
        &mut hdr.exprm_opt as *mut tcp_exprm_opt as *mut core::ffi::c_void,
        size_of::<tcp_exprm_opt>() as i32,
        load_flags,
    );
    if ret != -EINVAL {
        RET_CG_ERR!(ret);
    }

    /* Test searching experimental option with 0 magic value */
    hdr.exprm_opt.len = 4;
    ret = bpf_load_hdr_opt(
        skops,
        &mut hdr.exprm_opt as *mut tcp_exprm_opt as *mut core::ffi::c_void,
        size_of::<tcp_exprm_opt>() as i32,
        load_flags,
    );
    if ret != -ENOMSG {
        RET_CG_ERR!(ret);
    }

    hdr.exprm_opt.magic = __bpf_htons(0xeB9F);
    ret = bpf_load_hdr_opt(
        skops,
        &mut hdr.exprm_opt as *mut tcp_exprm_opt as *mut core::ffi::c_void,
        size_of::<tcp_exprm_opt>() as i32,
        load_flags,
    );
    if ret != 4
        || hdr.exprm_opt.len != 4
        || hdr.exprm_opt.kind != TCPOPT_EXP
        || hdr.exprm_opt.magic != __bpf_htons(0xeB9F)
    {
        RET_CG_ERR!(ret);
    }

    if !check_syn {
        return CG_OK;
    }

    /* Test loading from skops->syn_skb if sk_state == TCP_NEW_SYN_RECV
     *
     * Test loading from tp->saved_syn for other sk_state.
     */
    ret = bpf_getsockopt(
        skops,
        SOL_TCP,
        TCP_BPF_SYN_IP,
        &mut hdr.ip6 as *mut ipv6hdr as *mut core::ffi::c_void,
        size_of::<ipv6hdr>() as i32,
    );
    if ret != -ENOSPC {
        RET_CG_ERR!(ret);
    }

    if hdr.ip6.saddr.s6_addr16[7] != last_addr16_n
        || hdr.ip6.daddr.s6_addr16[7] != last_addr16_n
    {
        RET_CG_ERR!(0);
    }

    ret = bpf_getsockopt(
        skops,
        SOL_TCP,
        TCP_BPF_SYN_IP,
        &mut hdr as *mut hdr_union as *mut core::ffi::c_void,
        size_of::<hdr_union>() as i32,
    );
    if ret < 0 {
        RET_CG_ERR!(ret);
    }

    pth = (&mut hdr.ip6 as *mut ipv6hdr).add(1) as *mut tcphdr;
    if (*pth).dest != passive_lport_n || (*pth).source != active_lport_n {
        RET_CG_ERR!(0);
    }

    ret = bpf_getsockopt(
        skops,
        SOL_TCP,
        TCP_BPF_SYN,
        &mut hdr as *mut hdr_union as *mut core::ffi::c_void,
        size_of::<hdr_union>() as i32,
    );
    if ret < 0 {
        RET_CG_ERR!(ret);
    }

    if hdr.th.dest != passive_lport_n || hdr.th.source != active_lport_n {
        RET_CG_ERR!(0);
    }

    CG_OK
}

unsafe fn check_active_syn_in(skops: *mut bpf_sock_ops) -> i32 {
    __check_active_hdr_in(skops, true)
}

unsafe fn check_active_hdr_in(skops: *mut bpf_sock_ops) -> i32 {
    let th: *mut tcphdr;

    if __check_active_hdr_in(skops, false) == CG_ERR {
        return CG_ERR;
    }

    th = (*skops).skb_data as *mut tcphdr;
    if th.add(1) as *mut core::ffi::c_void > (*skops).skb_data_end {
        RET_CG_ERR!(0);
    }

    if tcp_hdrlen(th) < (*skops).skb_len {
        nr_data += 1;
    }

    if (*th).fin != 0 {
        nr_fin += 1;
    }

    if (*th).ack != 0 && (*th).fin == 0 && tcp_hdrlen(th) == (*skops).skb_len {
        nr_pure_ack += 1;
    }

    if (*skops).skb_hwtstamp != 0 {
        nr_hwtstamp += 1;
    }

    CG_OK
}

unsafe fn active_opt_len(skops: *mut bpf_sock_ops) -> i32 {
    let err: i32;

    /* Reserve more than enough to allow the -EEXIST test in
     * the write_active_opt().
     */
    err = bpf_reserve_hdr_opt(skops, 12, 0);
    if err != 0 {
        RET_CG_ERR!(err);
    }

    CG_OK
}

unsafe fn write_active_opt(skops: *mut bpf_sock_ops) -> i32 {
    let mut exprm_opt: tcp_exprm_opt = core::mem::zeroed();
    let mut win_scale_opt: tcp_opt = core::mem::zeroed();
    let mut reg_opt: tcp_opt = core::mem::zeroed();
    let th: *mut tcphdr;
    let mut err: i32;
    let mut ret: i32;

    exprm_opt.kind = TCPOPT_EXP;
    exprm_opt.len = 4;
    exprm_opt.magic = __bpf_htons(0xeB9F);

    reg_opt.kind = 0xB9;
    reg_opt.len = 4;
    reg_opt.data[0] = 0xfa;
    reg_opt.data[1] = 0xce;

    win_scale_opt.kind = TCPOPT_WINDOW;

    err = bpf_store_hdr_opt(
        skops,
        &exprm_opt as *const tcp_exprm_opt as *const core::ffi::c_void,
        size_of::<tcp_exprm_opt>() as i32,
        0,
    );
    if err != 0 {
        RET_CG_ERR!(err);
    }

    /* Store the same exprm option */
    err = bpf_store_hdr_opt(
        skops,
        &exprm_opt as *const tcp_exprm_opt as *const core::ffi::c_void,
        size_of::<tcp_exprm_opt>() as i32,
        0,
    );
    if err != -EEXIST {
        RET_CG_ERR!(err);
    }

    err = bpf_store_hdr_opt(
        skops,
        &reg_opt as *const tcp_opt as *const core::ffi::c_void,
        size_of::<tcp_opt>() as i32,
        0,
    );
    if err != 0 {
        RET_CG_ERR!(err);
    }
    err = bpf_store_hdr_opt(
        skops,
        &reg_opt as *const tcp_opt as *const core::ffi::c_void,
        size_of::<tcp_opt>() as i32,
        0,
    );
    if err != -EEXIST {
        RET_CG_ERR!(err);
    }

    /* Check the option has been written and can be searched */
    ret = bpf_load_hdr_opt(
        skops,
        &mut exprm_opt as *mut tcp_exprm_opt as *mut core::ffi::c_void,
        size_of::<tcp_exprm_opt>() as i32,
        0,
    );
    if ret != 4
        || exprm_opt.len != 4
        || exprm_opt.kind != TCPOPT_EXP
        || exprm_opt.magic != __bpf_htons(0xeB9F)
    {
        RET_CG_ERR!(ret);
    }

    reg_opt.len = 0;
    ret = bpf_load_hdr_opt(
        skops,
        &mut reg_opt as *mut tcp_opt as *mut core::ffi::c_void,
        size_of::<tcp_opt>() as i32,
        0,
    );
    if ret != 4
        || reg_opt.len != 4
        || reg_opt.kind != 0xB9
        || reg_opt.data[0] != 0xfa
        || reg_opt.data[1] != 0xce
    {
        RET_CG_ERR!(ret);
    }

    th = (*skops).skb_data as *mut tcphdr;
    if th.add(1) as *mut core::ffi::c_void > (*skops).skb_data_end {
        RET_CG_ERR!(0);
    }

    if (*th).syn != 0 {
        active_lport_h = (*skops).local_port;
        active_lport_n = (*th).source;

        /* Search the win scale option written by kernel
         * in the SYN packet.
         */
        ret = bpf_load_hdr_opt(
            skops,
            &mut win_scale_opt as *mut tcp_opt as *mut core::ffi::c_void,
            size_of::<tcp_opt>() as i32,
            0,
        );
        if ret != 3 || win_scale_opt.len != 3 || win_scale_opt.kind != TCPOPT_WINDOW {
            RET_CG_ERR!(ret);
        }

        /* Write the win scale option that kernel
         * has already written.
         */
        err = bpf_store_hdr_opt(
            skops,
            &win_scale_opt as *const tcp_opt as *const core::ffi::c_void,
            size_of::<tcp_opt>() as i32,
            0,
        );
        if err != -EEXIST {
            RET_CG_ERR!(err);
        }
    }

    CG_OK
}

unsafe fn handle_hdr_opt_len(skops: *mut bpf_sock_ops) -> i32 {
    let tcp_flags: __u8 = skops_tcp_flags(skops);

    if (tcp_flags & TCPHDR_SYNACK) == TCPHDR_SYNACK {
        /* Check the SYN from bpf_sock_ops_kern->syn_skb */
        return check_active_syn_in(skops);
    }

    /* Passive side should have cleared the write hdr cb by now */
    if (*skops).local_port == passive_lport_h {
        RET_CG_ERR!(0);
    }

    active_opt_len(skops)
}

unsafe fn handle_write_hdr_opt(skops: *mut bpf_sock_ops) -> i32 {
    if (*skops).local_port == passive_lport_h {
        RET_CG_ERR!(0);
    }

    write_active_opt(skops)
}

unsafe fn handle_parse_hdr(skops: *mut bpf_sock_ops) -> i32 {
    /* Passive side is not writing any non-standard/unknown
     * option, so the active side should never be called.
     */
    if (*skops).local_port == active_lport_h {
        RET_CG_ERR!(0);
    }

    check_active_hdr_in(skops)
}

unsafe fn handle_passive_estab(skops: *mut bpf_sock_ops) -> i32 {
    let err: i32;

    /* No more write hdr cb */
    bpf_sock_ops_cb_flags_set(
        skops,
        (*skops).bpf_sock_ops_cb_flags & !BPF_SOCK_OPS_WRITE_HDR_OPT_CB_FLAG,
    );

    /* Recheck the SYN but check the tp->saved_syn this time */
    err = check_active_syn_in(skops);
    if err == CG_ERR {
        return err;
    }

    nr_syn += 1;

    /* The ack has header option written by the active side also */
    check_active_hdr_in(skops)
}

#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn misc_estab(skops: *mut bpf_sock_ops) -> i32 {
    let true_val: i32 = 1;
    let false_val: i32 = 0;
    let mut ret: i32;

    match (*skops).op {
        BPF_SOCK_OPS_TCP_LISTEN_CB => {
            passive_lport_h = (*skops).local_port;
            passive_lport_n = __bpf_htons(passive_lport_h);
            bpf_setsockopt(
                skops,
                SOL_TCP,
                TCP_SAVE_SYN,
                &true_val as *const i32 as *const core::ffi::c_void,
                size_of::<i32>() as i32,
            );
            set_hdr_cb_flags(skops, 0);
        }
        BPF_SOCK_OPS_TCP_CONNECT_CB => {
            set_hdr_cb_flags(skops, 0);
        }
        BPF_SOCK_OPS_PARSE_HDR_OPT_CB => {
            return handle_parse_hdr(skops);
        }
        BPF_SOCK_OPS_HDR_OPT_LEN_CB => {
            ret = bpf_setsockopt(
                skops,
                SOL_TCP,
                TCP_NODELAY,
                &true_val as *const i32 as *const core::ffi::c_void,
                size_of::<i32>() as i32,
            );
            if ret == -EOPNOTSUPP {
                nodelay_hdr_len_reject = true;
            }
            return handle_hdr_opt_len(skops);
        }
        BPF_SOCK_OPS_WRITE_HDR_OPT_CB => {
            ret = bpf_setsockopt(
                skops,
                SOL_TCP,
                TCP_NODELAY,
                &true_val as *const i32 as *const core::ffi::c_void,
                size_of::<i32>() as i32,
            );
            if ret == -EOPNOTSUPP {
                nodelay_write_hdr_reject = true;
            }
            return handle_write_hdr_opt(skops);
        }
        BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => {
            ret = bpf_setsockopt(
                skops,
                SOL_TCP,
                TCP_NODELAY,
                &false_val as *const i32 as *const core::ffi::c_void,
                size_of::<i32>() as i32,
            );
            if ret == 0 {
                nodelay_est_ok = true;
            }
            return handle_passive_estab(skops);
        }
        _ => {}
    }

    CG_OK
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
