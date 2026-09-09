// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct nft_exthdr { pub type_: u8, pub offset: u8, pub len: u8, pub op: u8,
    pub dreg: u8, pub sreg: u8, pub flags: u8 }

unsafe fn optlen(opt: *const u8, offset: u32) -> u32 {
    // Beware zero-length options: make finite progress
    if *opt.add(offset as usize) <= TCPOPT_NOP || *opt.add(offset as usize + 1) == 0 { 1 }
    else { *opt.add(offset as usize + 1) as u32 }
}

unsafe fn nft_skb_copy_to_reg(skb: *const sk_buff, offset: i32, dest: *mut u32, len: u32) -> i32 {
    if len % NFT_REG32_SIZE != 0 { *dest.add((len / NFT_REG32_SIZE) as usize) = 0; }
    skb_copy_bits(skb, offset, dest as *mut _, len)
}

unsafe fn nft_exthdr_ipv6_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *mut nft_exthdr;
    let dest = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    let mut offset = 0u32;
    if (*(*pkt).skb).protocol != htons(ETH_P_IPV6) { (*regs).verdict.code = NFT_BREAK; return; }
    let err = ipv6_find_hdr((*pkt).skb, &mut offset, (*priv_).type_, core::ptr::null_mut(), core::ptr::null_mut());
    if (*priv_).flags & NFT_EXTHDR_F_PRESENT != 0 { nft_reg_store8(dest, err >= 0); return; }
    if err < 0 { (*regs).verdict.code = NFT_BREAK; return; }
    offset += (*priv_).offset as u32;
    if nft_skb_copy_to_reg((*pkt).skb, offset as i32, dest, (*priv_).len as u32) < 0 { (*regs).verdict.code = NFT_BREAK; }
}

unsafe fn ipv4_find_option(net: *mut net, skb: *mut sk_buff, offset: *mut u32, target: i32) -> i32 {
    let mut optbuf = [0u8; core::mem::size_of::<ip_options>() + 40];
    let opt = optbuf.as_mut_ptr() as *mut ip_options;
    let mut iph = core::mem::MaybeUninit::<iphdr>::uninit();
    let iph = skb_header_pointer(skb, 0, core::mem::size_of::<iphdr>() as u32, iph.as_mut_ptr() as *mut _);
    if iph.is_null() { return -EBADMSG; }
    let optlen = (*iph).ihl as i32 * 4 - core::mem::size_of::<iphdr>() as i32;
    if optlen <= 0 { return -ENOENT; }
    memset(opt as *mut _, 0, core::mem::size_of::<ip_options>());
    if skb_copy_bits(skb, core::mem::size_of::<iphdr>() as i32, (*opt).__data.as_mut_ptr() as *mut _, optlen as u32) != 0 { return -EBADMSG; }
    (*opt).optlen = optlen as u8;
    let mut info = 0u32;
    if __ip_options_compile(net, opt, core::ptr::null_mut(), &mut info) != 0 { return -EBADMSG; }
    let mut found = false;
    match target {
        IPOPT_SSRR | IPOPT_LSRR => { if (*opt).srr != 0 { found = if target == IPOPT_SSRR { (*opt).is_strictroute } else { !(*opt).is_strictroute }; if found { *offset = (*opt).srr as u32; } } }
        IPOPT_RR => { if (*opt).rr != 0 { *offset = (*opt).rr as u32; found = true; } }
        IPOPT_RA => { if (*opt).router_alert != 0 { *offset = (*opt).router_alert as u32; found = true; } }
        _ => return -EOPNOTSUPP,
    }
    if found { target } else { -ENOENT }
}

unsafe fn nft_exthdr_ipv4_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let p = nft_expr_priv(expr) as *mut nft_exthdr; let dest = (*regs).data.as_mut_ptr().add((*p).dreg as usize); let mut offset = 0;
    if (*(*pkt).skb).protocol != htons(ETH_P_IP) { (*regs).verdict.code = NFT_BREAK; return; }
    let err = ipv4_find_option(nft_net(pkt), (*pkt).skb, &mut offset, (*p).type_);
    if (*p).flags & NFT_EXTHDR_F_PRESENT != 0 { nft_reg_store8(dest, err >= 0); return; }
    if err < 0 { (*regs).verdict.code = NFT_BREAK; return; }
    offset += (*p).offset as u32; if nft_skb_copy_to_reg((*pkt).skb, offset as i32, dest, (*p).len as u32) < 0 { (*regs).verdict.code = NFT_BREAK; }
}

unsafe fn nft_tcp_header_pointer(pkt: *const nft_pktinfo, len: u32, buffer: *mut _, tcphdr_len: *mut u32) -> *mut _ {
    if (*pkt).tprot != IPPROTO_TCP || (*pkt).fragoff != 0 { return core::ptr::null_mut(); }
    let tcph = skb_header_pointer((*pkt).skb, nft_thoff(pkt), core::mem::size_of::<tcphdr>() as u32, buffer);
    if tcph.is_null() { return core::ptr::null_mut(); }
    *tcphdr_len = __tcp_hdrlen(tcph as *mut tcphdr); if *tcphdr_len < core::mem::size_of::<tcphdr>() as u32 || *tcphdr_len > len { return core::ptr::null_mut(); }
    skb_header_pointer((*pkt).skb, nft_thoff(pkt), *tcphdr_len, buffer)
}

// The remaining operation descriptors and protocol evaluators retain the kernel ABI layout.
// Their external kernel symbols are intentionally referenced rather than reimplemented here.
extern "C" {
    static mut nft_exthdr_type: nft_expr_type;
}

#[repr(C)] pub struct nft_expr_ops { pub type_: *mut nft_expr_type, pub size: usize, pub eval: Option<unsafe fn(*const nft_expr,*mut nft_regs,*const nft_pktinfo)>, pub init: Option<unsafe fn(*const nft_ctx,*const nft_expr,*const *const nlattr)->i32>, pub dump: Option<unsafe fn(*mut sk_buff,*const nft_expr,bool)->i32> }

// File-local declarations below mirror the C registration interface.
extern "C" {
    fn nft_exthdr_init(ctx:*const nft_ctx, expr:*const nft_expr, tb:*const *const nlattr)->i32;
    fn nft_exthdr_select_ops(ctx:*const nft_ctx, tb:*const *const nlattr)->*const nft_expr_ops;
}

#[repr(C)] pub struct nft_expr_type { pub name:*const u8, pub select_ops:Option<unsafe fn(*const nft_ctx,*const *const nlattr)->*const nft_expr_ops>, pub policy:*const nla_policy, pub maxattr:u32, pub owner:*mut core::ffi::c_void }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
