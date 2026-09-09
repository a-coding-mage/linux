/* Copyright (c) 2013 Coraid, Inc.  See COPYING for GPL terms. */
/*
 * aoenet.c
 * Ethernet portion of AoE driver
 */

// Linux kernel headers and "aoe.h" supply the referenced types, constants,
// functions, macros, and globals.

const NECODES: usize = 5;

static mut AOE_ERRLIST: [&'static [u8]; 6] = [
    b"no such error\0",
    b"unrecognized command code\0",
    b"bad argument parameter\0",
    b"device unavailable\0",
    b"config string present\0",
    b"unsupported version\0",
];

const IFLISTSZ: usize = 1024;

static mut aoe_iflist: [libc::c_char; IFLISTSZ] = [0; IFLISTSZ];
// module_param_string(aoe_iflist, aoe_iflist, IFLISTSZ, 0600);
// MODULE_PARM_DESC(aoe_iflist, "aoe_iflist=dev1[,dev2...]");

static mut txwq: wait_queue_head_t = unsafe { core::mem::zeroed() };
static mut kts: ktstate = unsafe { core::mem::zeroed() };

// #ifndef MODULE
unsafe extern "C" fn aoe_iflist_setup(str_: *mut libc::c_char) -> libc::c_int {
    strscpy(aoe_iflist.as_mut_ptr(), str_, IFLISTSZ);
    1
}
// __setup("aoe_iflist=", aoe_iflist_setup);
// #endif

static mut txlock: spinlock_t = unsafe { core::mem::zeroed() };
static mut skbtxq: sk_buff_head = unsafe { core::mem::zeroed() };

/* enters with txlock held */
unsafe fn tx(_id: libc::c_int) -> libc::c_int {
    let mut skb: *mut sk_buff;
    let mut ifp: *mut net_device;

    while {
        skb = skb_dequeue(&mut skbtxq);
        !skb.is_null()
    } {
        spin_unlock_irq(&mut txlock);
        ifp = (*skb).dev;
        if dev_queue_xmit(skb) == NET_XMIT_DROP && net_ratelimit() != 0 {
            pr_warn(
                b"aoe: packet could not be sent on %s.  %s\n\0".as_ptr() as *const libc::c_char,
                if !ifp.is_null() { (*ifp).name.as_ptr() } else { b"netif\0".as_ptr() as *const libc::c_char },
                b"consider increasing tx_queue_len\0".as_ptr() as *const libc::c_char,
            );
        }
        dev_put(ifp);
        spin_lock_irq(&mut txlock);
    }
    0
}

unsafe fn is_aoe_netif(ifp: *mut net_device) -> libc::c_int {
    if aoe_iflist[0] == 0 {
        return 1;
    }

    let mut p = aoe_iflist.as_mut_ptr().add(strspn(aoe_iflist.as_ptr(), WHITESPACE));
    while *p != 0 {
        let q = p.add(strcspn(p, WHITESPACE));
        let len = if q != p { q.offset_from(p) as usize } else { strlen(p) };
        if strlen((*ifp).name.as_ptr()) == len && strncmp((*ifp).name.as_ptr(), p, len) == 0 {
            return 1;
        }
        if q == p {
            break;
        }
        p = q.add(strspn(q, WHITESPACE));
    }
    0
}

unsafe fn set_aoe_iflist(user_str: *const libc::c_char, size: usize) -> libc::c_int {
    if size >= IFLISTSZ {
        return -EINVAL;
    }
    if copy_from_user(aoe_iflist.as_mut_ptr(), user_str, size) != 0 {
        printk(KERN_INFO, b"aoe: copy from user failed\n\0".as_ptr() as *const libc::c_char);
        return -EFAULT;
    }
    aoe_iflist[size] = 0;
    0
}

unsafe fn aoenet_xmit(queue: *mut sk_buff_head) {
    let mut skb: *mut sk_buff;
    let mut tmp: *mut sk_buff;
    let mut flags: ulong = 0;

    skb_queue_walk_safe!(queue, skb, tmp, {
        __skb_unlink(skb, queue);
        spin_lock_irqsave(&mut txlock, &mut flags);
        skb_queue_tail(&mut skbtxq, skb);
        spin_unlock_irqrestore(&mut txlock, flags);
        wake_up(&mut txwq);
    });
}

/*
 * (1) len doesn't include the header by default.  I want this.
 */
unsafe fn aoenet_rcv(
    mut skb: *mut sk_buff,
    ifp: *mut net_device,
    _pt: *mut packet_type,
    _orig_dev: *mut net_device,
) -> libc::c_int {
    let h: *mut aoe_hdr;
    let ah: *mut aoe_atahdr;
    let mut n: u32;
    let sn: usize;

    if dev_net(ifp) != &init_net as *const _ {
        goto_exit!(skb, exit);
    }
    skb = skb_share_check(skb, GFP_ATOMIC);
    if skb.is_null() {
        return 0;
    }
    if is_aoe_netif(ifp) == 0 {
        goto_exit!(skb, exit);
    }
    skb_push(skb, ETH_HLEN); // (1)
    sn = core::mem::size_of::<aoe_hdr>() + core::mem::size_of::<aoe_atahdr>();
    if (*skb).len >= sn {
        let needed = sn - skb_headlen(skb);
        if needed > 0 && __pskb_pull_tail(skb, needed).is_null() {
            goto_exit!(skb, exit);
        }
    }
    h = (*skb).data as *mut aoe_hdr;
    n = get_unaligned_be32(&(*h).tag);
    if (*h).verfl & AOEFL_RSP == 0 || n & (1u32 << 31) != 0 {
        goto_exit!(skb, exit);
    }
    if (*h).verfl & AOEFL_ERR != 0 {
        n = (*h).err as u32;
        if n > NECODES as u32 { n = 0; }
        if net_ratelimit() != 0 {
            printk(KERN_ERR, b"%s%d.%d@%s; ecode=%d '%s'\n\0".as_ptr() as *const libc::c_char,
                b"aoe: error packet from \0".as_ptr(), get_unaligned_be16(&(*h).major),
                (*h).minor, (*(*skb).dev).name.as_ptr(), (*h).err,
                AOE_ERRLIST[n as usize].as_ptr());
        }
        goto_exit!(skb, exit);
    }
    match (*h).cmd {
        AOECMD_ATA => { skb = aoecmd_ata_rsp(skb); }
        AOECMD_CFG => { aoecmd_cfg_rsp(skb); }
        cmd if cmd >= AOECMD_VEND_MIN => {}
        cmd => { pr_info(b"aoe: unknown AoE command type 0x%02x\n\0".as_ptr() as *const libc::c_char, cmd); }
    }
    if skb.is_null() { return 0; }
exit:
    dev_kfree_skb(skb);
    0
}

static mut aoe_pt: packet_type = packet_type {
    type_: __constant_htons(ETH_P_AOE),
    func: Some(aoenet_rcv),
};

unsafe fn aoenet_init() -> libc::c_int {
    skb_queue_head_init(&mut skbtxq);
    init_waitqueue_head(&mut txwq);
    spin_lock_init(&mut txlock);
    kts.lock = &mut txlock;
    kts.fn_ = Some(tx);
    kts.waitq = &mut txwq;
    kts.id = 0;
    snprintf(kts.name.as_mut_ptr(), core::mem::size_of_val(&kts.name), b"aoe_tx%d\0".as_ptr() as *const libc::c_char, kts.id);
    if aoe_ktstart(&mut kts) != 0 { return -EAGAIN; }
    dev_add_pack(&mut aoe_pt);
    0
}

unsafe fn aoenet_exit() {
    aoe_ktstop(&mut kts);
    skb_queue_purge(&mut skbtxq);
    dev_remove_pack(&mut aoe_pt);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
