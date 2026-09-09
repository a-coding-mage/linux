// SPDX-License-Identifier: GPL-2.0-or-later
/* net/atm/pppoatm.c - RFC2364 PPP over ATM/AAL5 */

/* Copyright 1999-2000 by Mitchell Blank Jr */
/* Based on clip.c; 1995-1999 by Werner Almesberger, EPFL LRC/ICA */
/* And on ppp_async.c; Copyright 1999 Paul Mackerras */
/* And help from Jens Axboe */

/*
 * This driver provides the encapsulation and framing for sending and
 * receiving PPP frames in ATM AAL5 PDUs.
 */

// C includes and build-time kernel configuration are supplied by the kernel
// environment and are intentionally not expanded here.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum pppoatm_encaps {
    e_autodetect = PPPOATM_ENCAPS_AUTODETECT,
    e_vc = PPPOATM_ENCAPS_VC,
    e_llc = PPPOATM_ENCAPS_LLC,
}

#[repr(C)]
struct pppoatm_vcc {
    atmvcc: *mut atm_vcc,
    old_push: Option<unsafe extern "C" fn(*mut atm_vcc, *mut sk_buff)>,
    old_pop: Option<unsafe extern "C" fn(*mut atm_vcc, *mut sk_buff)>,
    old_release_cb: Option<unsafe extern "C" fn(*mut atm_vcc)>,
    old_owner: *mut module,
    encaps: pppoatm_encaps,
    inflight: atomic_t,
    blocked: ::core::ffi::c_ulong,
    flags: ::core::ffi::c_int,
    chan: ppp_channel,
    wakeup_tasklet: tasklet_struct,
}

const NONE_INFLIGHT: i32 = -2;
const BLOCKED: usize = 0;
static PPPLLC: [u8; 6] = [0xfe, 0xfe, 0x03, 0xcf, 0xc0, 0x21];
const LLC_LEN: usize = 4;

#[inline]
unsafe fn atmvcc_to_pvcc(atmvcc: *const atm_vcc) -> *mut pppoatm_vcc {
    (*atmvcc).user_back as *mut pppoatm_vcc
}

#[inline]
unsafe fn chan_to_pvcc(chan: *const ppp_channel) -> *mut pppoatm_vcc {
    (*chan).private as *mut pppoatm_vcc
}

unsafe extern "C" fn pppoatm_wakeup_sender(t: *mut tasklet_struct) {
    let pvcc = container_of!(t, pppoatm_vcc, wakeup_tasklet);
    ppp_output_wakeup(&mut (*pvcc).chan);
}

unsafe extern "C" fn pppoatm_release_cb(atmvcc: *mut atm_vcc) {
    let pvcc = atmvcc_to_pvcc(atmvcc);
    if test_and_clear_bit(BLOCKED, &mut (*pvcc).blocked) != 0 {
        tasklet_schedule(&mut (*pvcc).wakeup_tasklet);
    }
    if let Some(old_release_cb) = (*pvcc).old_release_cb {
        old_release_cb(atmvcc);
    }
}

unsafe extern "C" fn pppoatm_pop(atmvcc: *mut atm_vcc, skb: *mut sk_buff) {
    let pvcc = atmvcc_to_pvcc(atmvcc);
    if let Some(old_pop) = (*pvcc).old_pop {
        old_pop(atmvcc, skb);
    }
    atomic_dec(&mut (*pvcc).inflight);
    if test_and_clear_bit(BLOCKED, &mut (*pvcc).blocked) != 0 {
        tasklet_schedule(&mut (*pvcc).wakeup_tasklet);
    }
}

unsafe fn pppoatm_unassign_vcc(atmvcc: *mut atm_vcc) {
    let pvcc = atmvcc_to_pvcc(atmvcc);
    (*atmvcc).push = (*pvcc).old_push;
    (*atmvcc).pop = (*pvcc).old_pop;
    (*atmvcc).release_cb = (*pvcc).old_release_cb;
    tasklet_kill(&mut (*pvcc).wakeup_tasklet);
    ppp_unregister_channel(&mut (*pvcc).chan);
    (*atmvcc).user_back = core::ptr::null_mut();
    kfree(pvcc as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn pppoatm_push(atmvcc: *mut atm_vcc, skb: *mut sk_buff) {
    let pvcc = atmvcc_to_pvcc(atmvcc);
    pr_debug!("\\n");
    if skb.is_null() {
        pr_debug!("removing ATMPPP VCC %p\\n", pvcc);
        let module = (*pvcc).old_owner;
        pppoatm_unassign_vcc(atmvcc);
        if let Some(push) = (*atmvcc).push { push(atmvcc, core::ptr::null_mut()); }
        module_put(module);
        return;
    }
    atm_return(atmvcc, (*skb).truesize);
    match (*pvcc).encaps {
        pppoatm_encaps::e_llc => {
            if (*skb).len < LLC_LEN || memcmp((*skb).data, PPPLLC.as_ptr(), LLC_LEN) != 0 { goto_error(pvcc, skb); return; }
            skb_pull(skb, LLC_LEN);
        }
        pppoatm_encaps::e_autodetect => {
            if (*pvcc).chan.ppp.is_null() { kfree_skb(skb); return; }
            if (*skb).len >= PPPLLC.len() && memcmp((*skb).data, PPPLLC.as_ptr(), PPPLLC.len()) == 0 {
                (*pvcc).encaps = pppoatm_encaps::e_llc; skb_pull(skb, LLC_LEN);
            } else if (*skb).len >= PPPLLC.len() - LLC_LEN && memcmp((*skb).data, PPPLLC.as_ptr().add(LLC_LEN), PPPLLC.len() - LLC_LEN) == 0 {
                (*pvcc).encaps = pppoatm_encaps::e_vc; (*pvcc).chan.mtu += LLC_LEN as i32;
            } else { pr_debug!("Couldn't autodetect yet (skb: %6ph)\\n", (*skb).data); goto_error(pvcc, skb); return; }
        }
        pppoatm_encaps::e_vc => {}
    }
    ppp_input(&mut (*pvcc).chan, skb);
}

unsafe fn goto_error(pvcc: *mut pppoatm_vcc, skb: *mut sk_buff) {
    kfree_skb(skb);
    ppp_input_error(&mut (*pvcc).chan);
}

unsafe fn pppoatm_may_send(pvcc: *mut pppoatm_vcc, size: i32) -> i32 {
    if atm_may_send((*pvcc).atmvcc, size) != 0 && atomic_inc_not_zero(&mut (*pvcc).inflight) != 0 { return 1; }
    test_and_set_bit(BLOCKED, &mut (*pvcc).blocked);
    if atm_may_send((*pvcc).atmvcc, size) != 0 && atomic_inc_not_zero(&mut (*pvcc).inflight) != 0 { return 1; }
    0
}

const DROP_PACKET: i32 = 1;

unsafe extern "C" fn pppoatm_send(chan: *mut ppp_channel, skb: *mut sk_buff) -> i32 {
    let pvcc = chan_to_pvcc(chan);
    (*skb).atm.vcc = (*pvcc).atmvcc;
    if (*skb).data[0] == 0 && ((*pvcc).flags & SC_COMP_PROT) != 0 { skb_pull(skb, 1); }
    let vcc = (*skb).atm.vcc;
    bh_lock_sock(sk_atm(vcc));
    if sock_owned_by_user(sk_atm(vcc)) != 0 { test_and_set_bit(BLOCKED, &mut (*pvcc).blocked); bh_unlock_sock(sk_atm(vcc)); return 0; }
    if test_bit(ATM_VF_CLOSE, &(*vcc).flags) != 0 || test_bit(ATM_VF_READY, &(*vcc).flags) == 0 { bh_unlock_sock(sk_atm(vcc)); kfree_skb(skb); return DROP_PACKET; }
    match (*pvcc).encaps {
        pppoatm_encaps::e_llc => { if skb_headroom(skb) < LLC_LEN { let n = skb_realloc_headroom(skb, LLC_LEN); if !n.is_null() && pppoatm_may_send(pvcc, (*n).truesize) == 0 { kfree_skb(n); bh_unlock_sock(sk_atm(vcc)); return 0; } consume_skb(skb); skb = n; if skb.is_null() { bh_unlock_sock(sk_atm(vcc)); return DROP_PACKET; } } else if pppoatm_may_send(pvcc, (*skb).truesize) == 0 { bh_unlock_sock(sk_atm(vcc)); return 0; } memcpy(skb_push(skb, LLC_LEN), PPPLLC.as_ptr(), LLC_LEN); }
        pppoatm_encaps::e_vc => { if pppoatm_may_send(pvcc, (*skb).truesize) == 0 { bh_unlock_sock(sk_atm(vcc)); return 0; } }
        pppoatm_encaps::e_autodetect => { bh_unlock_sock(sk_atm(vcc)); pr_debug!("Trying to send without setting encaps!\\n"); kfree_skb(skb); return 1; }
    }
    atm_account_tx(vcc, skb);
    let ret = if ((*vcc).send.unwrap()) (vcc, skb) != 0 { DROP_PACKET } else { 1 };
    bh_unlock_sock(sk_atm(vcc)); ret
}

unsafe extern "C" fn pppoatm_devppp_ioctl(chan: *mut ppp_channel, cmd: u32, arg: ::core::ffi::c_ulong) -> i32 {
    match cmd { PPPIOCGFLAGS => if put_user((*chan_to_pvcc(chan)).flags, arg as *mut i32) != 0 { -EFAULT } else { 0 }, PPPIOCSFLAGS => if get_user(&mut (*chan_to_pvcc(chan)).flags, arg as *const i32) != 0 { -EFAULT } else { 0 }, _ => -ENOTTY }
}

static pppoatm_ops: ppp_channel_ops = ppp_channel_ops { start_xmit: Some(pppoatm_send), ioctl: Some(pppoatm_devppp_ioctl) };

unsafe extern "C" fn pppoatm_assign_vcc(atmvcc: *mut atm_vcc, arg: *mut ::core::ffi::c_void) -> i32 {
    let mut be: atm_backend_ppp = core::mem::zeroed();
    if copy_from_user(&mut be, arg, core::mem::size_of::<atm_backend_ppp>()) != 0 { return -EFAULT; }
    if be.encaps != PPPOATM_ENCAPS_AUTODETECT && be.encaps != PPPOATM_ENCAPS_VC && be.encaps != PPPOATM_ENCAPS_LLC { return -EINVAL; }
    let pvcc = kzalloc::<pppoatm_vcc>();
    if pvcc.is_null() { return -ENOMEM; }
    (*pvcc).atmvcc = atmvcc;
    atomic_set(&mut (*pvcc).inflight, NONE_INFLIGHT);
    (*pvcc).old_push = (*atmvcc).push;
    (*pvcc).old_pop = (*atmvcc).pop;
    (*pvcc).old_owner = (*atmvcc).owner;
    (*pvcc).old_release_cb = (*atmvcc).release_cb;
    (*pvcc).encaps = core::mem::transmute(be.encaps);
    (*pvcc).chan.private = pvcc as *mut _;
    (*pvcc).chan.ops = &pppoatm_ops;
    (*pvcc).chan.mtu = (*atmvcc).qos.txtp.max_sdu - PPP_HDRLEN - if be.encaps == PPPOATM_ENCAPS_VC { 0 } else { LLC_LEN as i32 };
    tasklet_setup(&mut (*pvcc).wakeup_tasklet, Some(pppoatm_wakeup_sender));
    let err = ppp_register_channel(&mut (*pvcc).chan);
    if err != 0 { kfree(pvcc as *mut _); return err; }
    (*atmvcc).user_back = pvcc as *mut _;
    (*atmvcc).push = Some(pppoatm_push);
    (*atmvcc).pop = Some(pppoatm_pop);
    (*atmvcc).release_cb = Some(pppoatm_release_cb);
    __module_get(THIS_MODULE);
    (*atmvcc).owner = THIS_MODULE;
    vcc_process_recv_queue(atmvcc);
    0
}

unsafe extern "C" fn pppoatm_ioctl(sock: *mut socket, cmd: u32, arg: ::core::ffi::c_ulong) -> i32 {
    let atmvcc = ATM_SD(sock);
    let argp = arg as *mut ::core::ffi::c_void;
    if cmd != ATM_SETBACKEND && (*atmvcc).push != Some(pppoatm_push) { return -ENOIOCTLCMD; }
    match cmd {
        ATM_SETBACKEND => {
            let mut b: atm_backend_t = 0;
            if get_user(&mut b, argp as *const atm_backend_t) != 0 { return -EFAULT; }
            if b != ATM_BACKEND_PPP { return -ENOIOCTLCMD; }
            if capable(CAP_NET_ADMIN) == 0 { return -EPERM; }
            if (*sock).state != SS_CONNECTED { return -EINVAL; }
            pppoatm_assign_vcc(atmvcc, argp)
        }
        PPPIOCGCHAN => { let n = ppp_channel_index(&(*atmvcc_to_pvcc(atmvcc)).chan); if put_user(n, argp as *mut i32) != 0 { -EFAULT } else { 0 } }
        PPPIOCGUNIT => { let n = ppp_unit_number(&(*atmvcc_to_pvcc(atmvcc)).chan); if put_user(n, argp as *mut i32) != 0 { -EFAULT } else { 0 } }
        _ => -ENOIOCTLCMD,
    }
}
static mut pppoatm_ioctl_ops: atm_ioctl = atm_ioctl { owner: THIS_MODULE, ioctl: Some(pppoatm_ioctl) };
unsafe extern "C" fn pppoatm_init() -> i32 { register_atm_ioctl(&mut pppoatm_ioctl_ops); 0 }
unsafe extern "C" fn pppoatm_exit() { deregister_atm_ioctl(&mut pppoatm_ioctl_ops); }

module_init!(pppoatm_init);
module_exit!(pppoatm_exit);
module_author!("Mitchell Blank Jr <mitch@sfgoth.com>");
module_description!("RFC2364 PPP over ATM/AAL5");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
