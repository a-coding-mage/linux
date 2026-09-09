// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

#[cfg(CONFIG_MITIGATION_RETPOLINE)]
static mut NF_TABLES_SKIP_DIRECT_CALLS: static_key_false = static_key_false {};

#[cfg(CONFIG_MITIGATION_RETPOLINE)]
#[inline]
unsafe fn nf_skip_indirect_calls() -> bool {
    static_branch_likely(&raw const NF_TABLES_SKIP_DIRECT_CALLS)
}

#[cfg(CONFIG_MITIGATION_RETPOLINE)]
#[inline]
unsafe fn nf_skip_indirect_calls_enable() {
    if !cpu_feature_enabled(X86_FEATURE_RETPOLINE) {
        static_branch_enable(&raw mut NF_TABLES_SKIP_DIRECT_CALLS);
    }
}

#[cfg(not(CONFIG_MITIGATION_RETPOLINE))]
#[inline]
unsafe fn nf_skip_indirect_calls_enable() {}

#[inline(never)]
unsafe fn __nft_trace_packet(
    pkt: *const nft_pktinfo,
    verdict: *const nft_verdict,
    rule: *const nft_rule_dp,
    info: *mut nft_traceinfo,
    ty: nft_trace_types,
) {
    if !(*info).trace || !(*info).nf_trace {
        return;
    }
    (*info).type_ = ty;
    nft_trace_notify(pkt, verdict, rule, info);
}

#[inline]
unsafe fn nft_trace_packet(
    pkt: *const nft_pktinfo,
    verdict: *mut nft_verdict,
    info: *mut nft_traceinfo,
    rule: *const nft_rule_dp,
    ty: nft_trace_types,
) {
    if static_branch_unlikely(&raw const nft_trace_enabled) {
        (*info).nf_trace = (*(*pkt).skb).nf_trace;
        __nft_trace_packet(pkt, verdict, rule, info, ty);
    }
}

#[inline]
unsafe fn nft_trace_copy_nftrace(pkt: *const nft_pktinfo, info: *mut nft_traceinfo) {
    if static_branch_unlikely(&raw const nft_trace_enabled) {
        (*info).nf_trace = (*(*pkt).skb).nf_trace;
    }
}

unsafe fn nft_bitwise_fast_eval(expr: *const nft_expr, regs: *mut nft_regs) {
    let priv_ = nft_expr_priv(expr) as *const nft_bitwise_fast_expr;
    let src = (*regs).data.as_ptr().add((*priv_).sreg as usize);
    let dst = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    *dst = (*src & (*priv_).mask) ^ (*priv_).xor_;
}

unsafe fn nft_cmp_fast_eval(expr: *const nft_expr, regs: *mut nft_regs) {
    let priv_ = nft_expr_priv(expr) as *const nft_cmp_fast_expr;
    if (((*regs).data[(*priv_).sreg as usize] & (*priv_).mask) == (*priv_).data) ^ (*priv_).inv {
        return;
    }
    (*regs).verdict.code = NFT_BREAK;
}

unsafe fn nft_cmp16_fast_eval(expr: *const nft_expr, regs: *mut nft_regs) {
    let priv_ = nft_expr_priv(expr) as *const nft_cmp16_fast_expr;
    let reg_data = (*regs).data.as_ptr().add((*priv_).sreg as usize) as *const u64;
    let mask = &(*priv_).mask as *const _ as *const u64;
    let data = &(*priv_).data as *const _ as *const u64;
    if (((*reg_data & *mask) == *data) &&
        ((*reg_data.add(1) & *mask.add(1)) == *data.add(1))) ^ (*priv_).inv {
        return;
    }
    (*regs).verdict.code = NFT_BREAK;
}

#[inline(never)]
unsafe fn __nft_trace_verdict(pkt: *const nft_pktinfo, info: *mut nft_traceinfo,
                               rule: *const nft_rule_dp, regs: *const nft_regs) {
    let ty;
    match (*regs).verdict.code & NF_VERDICT_MASK {
        NFT_CONTINUE | NFT_RETURN => ty = NFT_TRACETYPE_RETURN,
        NF_STOLEN => ty = NFT_TRACETYPE_RULE,
        _ => {
            ty = NFT_TRACETYPE_RULE;
            if (*info).trace { (*info).nf_trace = (*(*pkt).skb).nf_trace; }
        }
    }
    __nft_trace_packet(pkt, &(*regs).verdict, rule, info, ty);
}

#[inline]
unsafe fn nft_trace_verdict(pkt: *const nft_pktinfo, info: *mut nft_traceinfo,
                             rule: *const nft_rule_dp, regs: *const nft_regs) {
    if static_branch_unlikely(&raw const nft_trace_enabled) { __nft_trace_verdict(pkt, info, rule, regs); }
}

unsafe fn nft_payload_fast_eval(expr: *const nft_expr, regs: *mut nft_regs,
                                pkt: *const nft_pktinfo) -> bool {
    let priv_ = nft_expr_priv(expr) as *const nft_payload;
    let skb = (*pkt).skb;
    let dest = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    let mut ptr: *mut u8;
    if (*priv_).base == NFT_PAYLOAD_NETWORK_HEADER {
        ptr = skb_network_header(skb).add((*pkt).nhoff as usize);
    } else {
        if ((*pkt).flags & NFT_PKTINFO_L4PROTO) == 0 || (*pkt).fragoff != 0 { return false; }
        ptr = (*skb).data.add(nft_thoff(pkt) as usize);
    }
    ptr = ptr.add((*priv_).offset as usize);
    if ptr.add((*priv_).len as usize) > skb_tail_pointer(skb) { return false; }
    *dest = 0;
    if (*priv_).len == 2 { *(dest as *mut u16) = *(ptr as *const u16); }
    else if (*priv_).len == 4 { *(dest as *mut u32) = *(ptr as *const u32); }
    else { *(dest as *mut u8) = *ptr; }
    true
}

static mut NFT_COUNTERS_ENABLED: static_key_false = static_key_false {};

#[inline(never)]
unsafe fn nft_update_chain_stats(chain: *const nft_chain, pkt: *const nft_pktinfo) {
    let base_chain = nft_base_chain(chain);
    let pstats = READ_ONCE((*base_chain).stats);
    if !pstats.is_null() {
        local_bh_disable();
        let stats = this_cpu_ptr(pstats);
        u64_stats_update_begin(&mut (*stats).syncp);
        (*stats).pkts += 1;
        (*stats).bytes += (*(*pkt).skb).len;
        u64_stats_update_end(&mut (*stats).syncp);
        local_bh_enable();
    }
}

#[repr(C)]
struct nft_jumpstack { rule: *const nft_rule_dp }

unsafe fn expr_call_ops_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *mut nft_pktinfo) {
    #[cfg(CONFIG_MITIGATION_RETPOLINE)] {
        if !nf_skip_indirect_calls() {
            let e = (*(*expr).ops).eval as usize;
            macro_rules! x { ($fun:ident) => { if e == $fun as usize { return $fun(expr, regs, pkt); } }; }
            x!(nft_payload_eval); x!(nft_cmp_eval); x!(nft_counter_eval); x!(nft_meta_get_eval);
            x!(nft_lookup_eval); x!(nft_ct_get_fast_eval); x!(nft_range_eval); x!(nft_immediate_eval);
            x!(nft_byteorder_eval); x!(nft_dynset_eval); x!(nft_rt_get_eval); x!(nft_bitwise_eval);
            x!(nft_objref_eval); x!(nft_objref_map_eval);
        }
    }
    ((*(*expr).ops).eval)(expr, regs, pkt);
}

#[inline]
unsafe fn nft_rule_expr_first(rule: *const nft_rule_dp) -> *mut nft_expr { (*rule).data.as_mut_ptr() as *mut nft_expr }
#[inline]
unsafe fn nft_rule_expr_next(expr: *mut nft_expr) -> *mut nft_expr { (expr as *mut u8).add((*(*expr).ops).size as usize) as *mut nft_expr }
#[inline]
unsafe fn nft_rule_expr_last(rule: *const nft_rule_dp) -> *mut nft_expr { (*rule).data.as_ptr().add((*rule).dlen as usize) as *mut nft_expr }

unsafe fn nft_do_chain(pkt: *mut nft_pktinfo, priv_: *mut core::ffi::c_void) -> u32 {
    let mut chain = priv_ as *mut nft_chain;
    let basechain = chain;
    let net = nft_net(pkt);
    let mut regs: nft_regs = core::mem::zeroed();
    let mut stackptr = 0usize;
    let mut jumpstack: [nft_jumpstack; NFT_JUMP_STACK_SIZE as usize] = core::mem::zeroed();
    let genbit = READ_ONCE((*net).nft.gencursor);
    let mut info: nft_traceinfo = core::mem::zeroed();
    info.trace = false;
    if static_branch_unlikely(&raw const nft_trace_enabled) { nft_trace_init(&mut info, pkt, basechain); }
    'do_chain: loop {
        let blob = if genbit { rcu_dereference((*chain).blob_gen_1) } else { rcu_dereference((*chain).blob_gen_0) };
        let mut rule = (*blob).data.as_ptr() as *mut nft_rule_dp;
        loop {
            regs.verdict.code = NFT_CONTINUE;
            while !(*rule).is_last {
                let mut expr = nft_rule_expr_first(rule);
                let last = nft_rule_expr_last(rule);
                while expr != last {
                    if (*(*expr).ops == nft_cmp_fast_ops) { nft_cmp_fast_eval(expr, &mut regs); }
                    else if (*(*expr).ops == nft_cmp16_fast_ops) { nft_cmp16_fast_eval(expr, &mut regs); }
                    else if (*(*expr).ops == nft_bitwise_fast_ops) { nft_bitwise_fast_eval(expr, &mut regs); }
                    else if (*(*expr).ops != nft_payload_fast_ops) || !nft_payload_fast_eval(expr, &mut regs, pkt) { expr_call_ops_eval(expr, &mut regs, pkt); }
                    if regs.verdict.code != NFT_CONTINUE { break; }
                    expr = nft_rule_expr_next(expr);
                }
                match regs.verdict.code {
                    NFT_BREAK => { regs.verdict.code = NFT_CONTINUE; nft_trace_copy_nftrace(pkt, &mut info); }
                    NFT_CONTINUE => { nft_trace_packet(pkt, &mut regs.verdict, &mut info, rule, NFT_TRACETYPE_RULE); }
                    _ => break,
                }
                rule = nft_rule_next(rule);
            }
            nft_trace_verdict(pkt, &mut info, rule, &regs);
            match regs.verdict.code & NF_VERDICT_MASK {
                NF_ACCEPT | NF_QUEUE | NF_STOLEN => return regs.verdict.code,
                NF_DROP => return NF_DROP_REASON((*pkt).skb, SKB_DROP_REASON_NETFILTER_DROP, EPERM),
                _ => {}
            }
            match regs.verdict.code {
                NFT_JUMP => { if stackptr >= NFT_JUMP_STACK_SIZE as usize { DEBUG_NET_WARN_ON_ONCE(1); return NF_DROP_REASON((*pkt).skb, SKB_DROP_REASON_NETFILTER_DROP, ELOOP); } jumpstack[stackptr].rule = nft_rule_next(rule); stackptr += 1; }
                NFT_JUMP | NFT_GOTO => { chain = regs.verdict.chain; continue 'do_chain; }
                NFT_CONTINUE | NFT_RETURN => {}
                _ => { DEBUG_NET_WARN_ON_ONCE(1); }
            }
            if stackptr > 0 { stackptr -= 1; rule = jumpstack[stackptr].rule; continue; }
            nft_trace_packet(pkt, &mut regs.verdict, &mut info, core::ptr::null(), NFT_TRACETYPE_POLICY);
            if static_branch_unlikely(&raw const NFT_COUNTERS_ENABLED) { nft_update_chain_stats(basechain, pkt); }
            if (*nft_base_chain(basechain)).policy == NF_DROP { return NF_DROP_REASON((*pkt).skb, SKB_DROP_REASON_NETFILTER_DROP, EPERM); }
            return (*nft_base_chain(basechain)).policy;
        }
    }
}

static mut NFT_BASIC_TYPES: [*mut nft_expr_type; 15] = [
    &raw mut nft_imm_type, &raw mut nft_cmp_type, &raw mut nft_lookup_type,
    &raw mut nft_bitwise_type, &raw mut nft_byteorder_type, &raw mut nft_payload_type,
    &raw mut nft_dynset_type, &raw mut nft_range_type, &raw mut nft_meta_type,
    &raw mut nft_rt_type, &raw mut nft_exthdr_type, &raw mut nft_last_type,
    &raw mut nft_counter_type, &raw mut nft_objref_type, &raw mut nft_inner_type,
];

static mut NFT_BASIC_OBJECTS: [*mut nft_object_type; 1] = [
    &raw mut nft_counter_obj_type,
];

#[no_mangle]
pub unsafe extern "C" fn nf_tables_core_module_init() -> i32 {
    let mut err: i32;
    let mut i: isize;
    let mut j: isize = 0;
    nft_counter_init_seqcount();
    i = 0;
    while i < NFT_BASIC_OBJECTS.len() as isize { err = nft_register_obj(NFT_BASIC_OBJECTS[i as usize]); if err != 0 { while j > 0 { j -= 1; nft_unregister_expr(NFT_BASIC_TYPES[j as usize]); } while i > 0 { i -= 1; nft_unregister_obj(NFT_BASIC_OBJECTS[i as usize]); } return err; } i += 1; }
    while j < NFT_BASIC_TYPES.len() as isize { err = nft_register_expr(NFT_BASIC_TYPES[j as usize]); if err != 0 { while j > 0 { j -= 1; nft_unregister_expr(NFT_BASIC_TYPES[j as usize]); } while i > 0 { i -= 1; nft_unregister_obj(NFT_BASIC_OBJECTS[i as usize]); } return err; } j += 1; }
    nf_skip_indirect_calls_enable();
    0
}

pub unsafe extern "C" fn nf_tables_core_module_exit() {
    let mut i = NFT_BASIC_TYPES.len();
    while i > 0 { i -= 1; nft_unregister_expr(NFT_BASIC_TYPES[i]); }
    i = NFT_BASIC_OBJECTS.len();
    while i > 0 { i -= 1; nft_unregister_obj(NFT_BASIC_OBJECTS[i]); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
