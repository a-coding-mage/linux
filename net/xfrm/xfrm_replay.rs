// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of xfrm_replay.c. Kernel types and helpers are supplied by
 * the surrounding translation unit. */

// Includes: linux/export.h, net/xfrm.h

pub unsafe fn xfrm_replay_seqhi(x: *mut xfrm_state, net_seq: __be32) -> u32 {
    if !((*x).props.flags & XFRM_STATE_ESN) != 0 { return 0; }
    let r = (*x).replay_esn;
    let seq = ntohl(net_seq);
    let mut seq_hi = (*r).seq_hi;
    let bottom = (*r).seq.wrapping_sub((*r).replay_window).wrapping_add(1);
    if (*r).seq >= (*r).replay_window.wrapping_sub(1) {
        if seq < bottom { seq_hi = seq_hi.wrapping_add(1); }
    } else if seq >= bottom { seq_hi = seq_hi.wrapping_sub(1); }
    seq_hi
}

unsafe fn xfrm_replay_notify_bmp(x: *mut xfrm_state, event: i32);
unsafe fn xfrm_replay_notify_esn(x: *mut xfrm_state, event: i32);

pub unsafe fn xfrm_replay_notify(x: *mut xfrm_state, mut event: i32) {
    let mut c: km_event = core::mem::zeroed();
    match (*x).repl_mode { XFRM_REPLAY_MODE_LEGACY => {}, XFRM_REPLAY_MODE_BMP => { xfrm_replay_notify_bmp(x,event); return }, XFRM_REPLAY_MODE_ESN => { xfrm_replay_notify_esn(x,event); return }, _ => {} }
    match event {
        XFRM_REPLAY_UPDATE => {
            if (*x).replay_maxdiff == 0 || ((*x).replay.seq - (*x).preplay.seq < (*x).replay_maxdiff && (*x).replay.oseq - (*x).preplay.oseq < (*x).replay_maxdiff) {
                if (*x).xflags & XFRM_TIME_DEFER != 0 { event = XFRM_REPLAY_TIMEOUT; } else { return; }
            }
        }
        XFRM_REPLAY_TIMEOUT => {
            if core::slice::from_raw_parts((&(*x).replay) as *const _ as *const u8, core::mem::size_of::<xfrm_replay_state>()) == core::slice::from_raw_parts((&(*x).preplay) as *const _ as *const u8, core::mem::size_of::<xfrm_replay_state>()) { (*x).xflags |= XFRM_TIME_DEFER; return; }
        }
        _ => {}
    }
    core::ptr::copy_nonoverlapping(&(*x).replay, &mut (*x).preplay, 1);
    c.event = XFRM_MSG_NEWAE; c.data.aevent = event; km_state_notify(x, &mut c);
    if (*x).replay_maxage != 0 && mod_timer(&mut (*x).rtimer, jiffies + (*x).replay_maxage) == 0 { (*x).xflags &= !XFRM_TIME_DEFER; }
}

unsafe fn __xfrm_replay_overflow(x: *mut xfrm_state, skb: *mut sk_buff) -> i32 {
    let mut err=0; let net=xs_net(x);
    if (*(*x).type_).flags & XFRM_TYPE_REPLAY_PROT != 0 { XFRM_SKB_CB(skb).seq.output.low=(*x).replay.oseq.wrapping_add(1); (*x).replay.oseq=XFRM_SKB_CB(skb).seq.output.low; XFRM_SKB_CB(skb).seq.output.hi=0; if (*x).replay.oseq==0 && (*x).props.extra_flags & XFRM_SA_XFLAG_OSEQ_MAY_WRAP == 0 { (*x).replay.oseq=(*x).replay.oseq.wrapping_sub(1); xfrm_audit_state_replay_overflow(x,skb); err=-EOVERFLOW; return err; } if xfrm_aevent_is_on(net) { xfrm_replay_notify(x,XFRM_REPLAY_UPDATE); } }
    err
}

unsafe fn xfrm_replay_check_legacy(x:*mut xfrm_state, skb:*mut sk_buff, net_seq:__be32)->i32 { let seq=ntohl(net_seq); if (*x).props.replay_window==0{return 0} if seq==0 {return xfrm_replay_check_legacy_err(x,skb,net_seq)} if seq>(*x).replay.seq{return 0} let diff=(*x).replay.seq-seq; if diff>=(*x).props.replay_window {(*x).stats.replay_window+=1;return xfrm_replay_check_legacy_err(x,skb,net_seq)} if (*x).replay.bitmap&(1u32<<diff)!=0 {(*x).stats.replay+=1;return xfrm_replay_check_legacy_err(x,skb,net_seq)} 0 }
unsafe fn xfrm_replay_check_legacy_err(x:*mut xfrm_state,skb:*mut sk_buff,s:__be32)->i32{xfrm_audit_state_replay(x,skb,s);-EINVAL}

unsafe fn xfrm_replay_advance_bmp(x:*mut xfrm_state,s:__be32); unsafe fn xfrm_replay_advance_esn(x:*mut xfrm_state,s:__be32);
pub unsafe fn xfrm_replay_advance(x:*mut xfrm_state,s:__be32){match (*x).repl_mode{XFRM_REPLAY_MODE_LEGACY=>{},XFRM_REPLAY_MODE_BMP=>{xfrm_replay_advance_bmp(x,s);return},XFRM_REPLAY_MODE_ESN=>{xfrm_replay_advance_esn(x,s);return},_=>{}} if (*x).props.replay_window==0{return} let seq=ntohl(s);if seq>(*x).replay.seq{let d=seq-(*x).replay.seq;(*x).replay.bitmap=if d<(*x).props.replay_window{((*x).replay.bitmap<<d)|1}else{1};(*x).replay.seq=seq}else{(*x).replay.bitmap|=1u32<<((*x).replay.seq-seq)}if xfrm_aevent_is_on(xs_net(x)){xfrm_replay_notify(x,XFRM_REPLAY_UPDATE)}}

unsafe fn xfrm_replay_overflow_bmp(x:*mut xfrm_state,skb:*mut sk_buff)->i32{let r=(*x).replay_esn;let mut e=0;if (*(*x).type_).flags&XFRM_TYPE_REPLAY_PROT!=0{(*r).oseq=(*r).oseq.wrapping_add(1);XFRM_SKB_CB(skb).seq.output.low=(*r).oseq;XFRM_SKB_CB(skb).seq.output.hi=0;if (*r).oseq==0&&(*x).props.extra_flags&XFRM_SA_XFLAG_OSEQ_MAY_WRAP==0{(*r).oseq-=1;xfrm_audit_state_replay_overflow(x,skb);e=-EOVERFLOW;return e}if xfrm_aevent_is_on(xs_net(x)){xfrm_replay_notify(x,XFRM_REPLAY_UPDATE)}}e}

unsafe fn xfrm_replay_check_bmp(x:*mut xfrm_state,skb:*mut sk_buff,s:__be32)->i32{let r=(*x).replay_esn;let seq=ntohl(s);if (*r).replay_window==0{return 0}if seq==0{return xfrm_replay_check_legacy_err(x,skb,s)}if seq>(*r).seq{return 0}let d=(*r).seq-seq;if d>=(*r).replay_window{(*x).stats.replay_window+=1;return xfrm_replay_check_legacy_err(x,skb,s)}let p=((*r).seq-1)%(*r).replay_window;let b=if p>=d{(p-d)%(*r).replay_window}else{(*r).replay_window-(d-p)};let n=b>>5;let bit=b&31;if (*r).bmp[n as usize]&(1u32<<bit)!=0{(*x).stats.replay+=1;return xfrm_replay_check_legacy_err(x,skb,s)}0}

pub unsafe fn xfrm_replay_check(x:*mut xfrm_state,skb:*mut sk_buff,s:__be32)->i32{match (*x).repl_mode{XFRM_REPLAY_MODE_BMP=>xfrm_replay_check_bmp(x,skb,s),XFRM_REPLAY_MODE_ESN=>xfrm_replay_check_esn(x,skb,s),_=>xfrm_replay_check_legacy(x,skb,s)}}
unsafe fn xfrm_replay_check_esn(x:*mut xfrm_state,skb:*mut sk_buff,s:__be32)->i32{xfrm_replay_check_bmp(x,skb,s)}
pub unsafe fn xfrm_replay_recheck(x:*mut xfrm_state,skb:*mut sk_buff,s:__be32)->i32{if (*x).repl_mode==XFRM_REPLAY_MODE_ESN&&XFRM_SKB_CB(skb).seq.input.hi!=htonl(xfrm_replay_seqhi(x,s)){(*x).stats.replay_window+=1;return -EINVAL}xfrm_replay_check(x,skb,s)}

unsafe fn xfrm_replay_advance_esn(x:*mut xfrm_state,s:__be32){xfrm_replay_advance_bmp(x,s)}
unsafe fn xfrm_replay_notify_bmp(x:*mut xfrm_state,_:i32){xfrm_replay_notify(x,XFRM_REPLAY_UPDATE)}
unsafe fn xfrm_replay_notify_esn(x:*mut xfrm_state,_:i32){xfrm_replay_notify(x,XFRM_REPLAY_UPDATE)}

pub unsafe fn xfrm_replay_overflow(x:*mut xfrm_state,skb:*mut sk_buff)->i32{match (*x).repl_mode{XFRM_REPLAY_MODE_BMP=>xfrm_replay_overflow_bmp(x,skb),XFRM_REPLAY_MODE_ESN=>xfrm_replay_overflow_esn(x,skb),_=>__xfrm_replay_overflow(x,skb)}}
unsafe fn xfrm_replay_overflow_esn(x:*mut xfrm_state,skb:*mut sk_buff)->i32{xfrm_replay_overflow_bmp(x,skb)}

pub unsafe fn xfrm_init_replay(x:*mut xfrm_state,extack:*mut netlink_ext_ack)->i32{let r=(*x).replay_esn;if !r.is_null(){if (*r).replay_window>(*r).bmp_len*core::mem::size_of::<__u32>() as u32*8{return -EINVAL}if (*x).props.flags&XFRM_STATE_ESN!=0{if (*r).replay_window==0&&((*x).dir==0||(*x).dir==XFRM_SA_DIR_IN){return -EINVAL}(*x).repl_mode=XFRM_REPLAY_MODE_ESN}else{(*x).repl_mode=XFRM_REPLAY_MODE_BMP}}else{(*x).repl_mode=XFRM_REPLAY_MODE_LEGACY}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
