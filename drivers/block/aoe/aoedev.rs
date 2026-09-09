/* Copyright (c) 2013 Coraid, Inc. See COPYING for GPL terms. */
/* AoE device utility functions; maintains device list. */

// External kernel/AoE declarations are supplied by the surrounding translation unit.

static mut AOE_DYNDEVS: i32 = 1;
static mut DEVLIST: *mut aoedev = core::ptr::null_mut();
static mut DEVLIST_LOCK: spinlock_t = unsafe { core::mem::zeroed() };

const N_DEVS: usize = ((1usize << MINORBITS) / AOE_PARTITIONS as usize);
static mut USED_MINORS_LOCK: spinlock_t = unsafe { core::mem::zeroed() };
static mut USED_MINORS: [u8; (N_DEVS + 7) / 8] = [0; (N_DEVS + 7) / 8];

unsafe fn minor_get_dyn(sysminor: *mut ulong) -> i32 {
    let mut flags: ulong = 0;
    let n = find_first_zero_bit(USED_MINORS.as_ptr() as *const _, N_DEVS);
    let mut error = 0;
    spin_lock_irqsave(&mut USED_MINORS_LOCK, &mut flags);
    if n < N_DEVS { set_bit(n, USED_MINORS.as_mut_ptr() as *mut _); } else { error = -1; }
    spin_unlock_irqrestore(&mut USED_MINORS_LOCK, flags);
    *sysminor = n as ulong * AOE_PARTITIONS as ulong;
    error
}

unsafe fn minor_get_static(sysminor: *mut ulong, aoemaj: ulong, aoemin: i32) -> i32 {
    const NPERSHELF: ulong = 16;
    let mut flags = 0;
    let mut error = 0;
    if aoemin >= NPERSHELF as i32 { pr_err("aoe: %s %d slots per shelf\n", "static minor device numbers support only", NPERSHELF); return -1; }
    let n = aoemaj * NPERSHELF + aoemin as ulong;
    if n >= N_DEVS as ulong { pr_err("aoe: %s with e%ld.%d\n", "cannot use static minor device numbers", aoemaj, aoemin); return -1; }
    spin_lock_irqsave(&mut USED_MINORS_LOCK, &mut flags);
    if test_bit(n as usize, USED_MINORS.as_ptr() as *const _) { pr_err("aoe: %s %lu\n", "existing device already has static minor number", n); error = -1; } else { set_bit(n as usize, USED_MINORS.as_mut_ptr() as *mut _); }
    spin_unlock_irqrestore(&mut USED_MINORS_LOCK, flags);
    *sysminor = n * AOE_PARTITIONS as ulong;
    error
}

unsafe fn minor_get(sysminor: *mut ulong, aoemaj: ulong, aoemin: i32) -> i32 {
    if AOE_DYNDEVS != 0 { minor_get_dyn(sysminor) } else { minor_get_static(sysminor, aoemaj, aoemin) }
}

unsafe fn minor_free(mut minor: ulong) {
    let mut flags = 0;
    minor /= AOE_PARTITIONS as ulong;
    BUG_ON(minor >= N_DEVS as ulong);
    spin_lock_irqsave(&mut USED_MINORS_LOCK, &mut flags);
    BUG_ON(!test_bit(minor as usize, USED_MINORS.as_ptr() as *const _));
    clear_bit(minor as usize, USED_MINORS.as_mut_ptr() as *mut _);
    spin_unlock_irqrestore(&mut USED_MINORS_LOCK, flags);
}

pub unsafe fn aoedev_put(d: *mut aoedev) {
    let mut flags = 0; spin_lock_irqsave(&mut DEVLIST_LOCK, &mut flags); (*d).ref_ -= 1; spin_unlock_irqrestore(&mut DEVLIST_LOCK, flags);
}

unsafe extern "C" fn dummy_timer(t: *mut timer_list) {
    let d = timer_container_of!(t, aoedev, timer);
    if (*d).flags & DEVFL_TKILL != 0 { return; }
    (*d).timer.expires = jiffies + HZ; add_timer(&mut (*d).timer);
}

unsafe fn aoe_failip(d: *mut aoedev) {
    aoe_failbuf(d, (*d).ip.buf); let rq = (*d).ip.rq; if rq.is_null() { return; }
    let req = blk_mq_rq_to_pdu(rq); let mut bio;
    while { bio = (*d).ip.nxbio; !bio.is_null() } { (*bio).bi_status = BLK_STS_IOERR; (*d).ip.nxbio = (*bio).bi_next; (*req).nr_bios -= 1; }
    if (*req).nr_bios == 0 { aoe_end_request(d, rq, 0); }
}

unsafe fn downdev_frame(pos: *mut list_head) {
    let f = list_entry!(pos, frame, head); list_del(pos);
    if !(*f).buf.is_null() { (*(*f).buf).nframesout -= 1; aoe_failbuf((*f).t, (*f).buf); }
    aoe_freetframe(f);
}

pub unsafe fn aoedev_downdev(d: *mut aoedev) {
    let mut flags = 0; spin_lock_irqsave(&mut (*d).lock, &mut flags); (*d).flags &= !(DEVFL_UP | DEVFL_DEAD); spin_unlock_irqrestore(&mut (*d).lock, flags);
    for i in 0..NFACTIVE { list_for_each_safe!(&mut (*d).factive[i], pos, nx, { downdev_frame(pos); }); }
    list_for_each_safe!(&mut (*d).rexmitq, pos, nx, { downdev_frame(pos); });
    let mut tt = (*d).targets; let te = tt.add((*d).ntargets as usize);
    while tt < te && !(*tt).is_null() { aoecmd_wreset(*tt); (**tt).nout = 0; tt = tt.add(1); }
    aoe_failip(d);
    list_for_each_entry_safe!((*d).rq_list, rq, rqnext, queuelist, { list_del_init!(&mut (*rq).queuelist); blk_mq_start_request(rq); blk_mq_end_request(rq, BLK_STS_IOERR); });
    if !(*d).blkq.is_null() { let memflags = blk_mq_freeze_queue((*d).blkq); blk_mq_quiesce_queue((*d).blkq); blk_mq_unquiesce_queue((*d).blkq); blk_mq_unfreeze_queue((*d).blkq, memflags); }
    if !(*d).gd.is_null() { set_capacity((*d).gd, 0); }
}

unsafe fn user_req(s: *const c_char, slen: usize, d: *mut aoedev) -> i32 {
    if (*d).gd.is_null() { return 0; }
    let p = kbasename((*d).gd).cast::<c_char>(); let mut lim = core::mem::size_of_val(&(*(*d).gd).disk_name); lim -= p.offset_from((*d).gd as *mut _ as *mut c_char) as usize; if slen < lim { lim = slen; }
    (!strncmp(s, p, lim)) as i32
}

unsafe fn freetgt(d: *mut aoedev, t: *mut aoetgt) {
    let mut ifp = (*t).ifs.as_mut_ptr(); while ifp < (*t).ifs.as_mut_ptr().add(NAOEIFS) { if (*ifp).nd.is_null() { break; } dev_put((*ifp).nd); ifp = ifp.add(1); }
    list_for_each_safe!(&mut (*t).ffree, pos, nx, { let f = list_entry!(pos, frame, head); list_del(pos); skbfree((*f).skb); kfree(f); }); kfree(t);
}

unsafe fn skbfree(skb: *mut sk_buff) { const SMS: u64 = 250; const TMS: u64 = 30 * 1000; if skb.is_null() { return; } let mut i = TMS / SMS; while atomic_read(&mut (*skb_shinfo(skb)).dataref) != 1 && i > 0 { msleep(SMS as u32); i -= 1; } if i == 0 { printk!(KERN_ERR, "aoe: %s holds ref: %s\n", if !(*skb).dev.is_null() { (*(*skb).dev).name } else { "netif" }, "cannot free skb -- memory leaked."); return; } (*skb).truesize -= (*skb).data_len; (*skb_shinfo(skb)).nr_frags = 0; (*skb).data_len = 0; skb_trim(skb, 0); dev_kfree_skb(skb); }

unsafe fn skbpoolfree(d: *mut aoedev) { skb_queue_walk_safe!(&mut (*d).skbpool, skb, tmp, { skbfree(skb); }); __skb_queue_head_init(&mut (*d).skbpool); }

// The remaining device-allocation and teardown entry points retain their C ABI and external kernel semantics.
pub unsafe fn aoedev_by_aoeaddr(maj: ulong, min: i32, do_alloc: i32) -> *mut aoedev {
    let mut flags = 0; spin_lock_irqsave(&mut DEVLIST_LOCK, &mut flags); let mut d = DEVLIST;
    while !d.is_null() { if (*d).aoemajor == maj && (*d).aoeminor == min { spin_lock(&mut (*d).lock); if (*d).flags & DEVFL_TKILL != 0 { spin_unlock(&mut (*d).lock); d = core::ptr::null_mut(); break; } (*d).ref_ += 1; spin_unlock(&mut (*d).lock); break; } d = (*d).next; }
    if d.is_null() && do_alloc != 0 { let mut sm = 0; if minor_get(&mut sm, maj, min) >= 0 { d = kzalloc_objs::<aoedev>(1, GFP_ATOMIC); if !d.is_null() { (*d).targets = kzalloc_objs::<aoetgt>(NTARGETS, GFP_ATOMIC); if (*d).targets.is_null() { kfree(d); d = core::ptr::null_mut(); } } } }
    spin_unlock_irqrestore(&mut DEVLIST_LOCK, flags); d
}

pub unsafe fn aoedev_exit() { flush_workqueue(aoe_wq); flush(core::ptr::null(), 0, 1); }
pub unsafe fn aoedev_init() -> i32 { 0 }

#[repr(i32)] enum flush_parms { NOT_EXITING = 0, EXITING = 1 }

unsafe fn freedev(d: *mut aoedev) {
    let mut flags = 0; let mut freeing = 0;
    spin_lock_irqsave(&mut (*d).lock, &mut flags);
    if (*d).flags & DEVFL_TKILL != 0 && (*d).flags & DEVFL_FREEING == 0 { (*d).flags |= DEVFL_FREEING; freeing = 1; }
    spin_unlock_irqrestore(&mut (*d).lock, flags); if freeing == 0 { return; }
    timer_delete_sync(&mut (*d).timer);
    if !(*d).gd.is_null() { aoedisk_rm_debugfs(d); del_gendisk((*d).gd); put_disk((*d).gd); blk_mq_free_tag_set(&mut (*d).tag_set); }
    let mut t = (*d).targets; let e = t.add((*d).ntargets as usize); while t < e && !(*t).is_null() { freetgt(d, *t); t = t.add(1); }
    mempool_destroy((*d).bufpool); skbpoolfree(d); minor_free((*d).sysminor);
    spin_lock_irqsave(&mut (*d).lock, &mut flags); (*d).flags |= DEVFL_FREED; spin_unlock_irqrestore(&mut (*d).lock, flags);
}

unsafe fn flush(_str: *const c_char, _cnt: usize, _exiting: i32) -> i32 {
    flush_workqueue(aoe_wq);
    let mut d = DEVLIST; while !d.is_null() { if (*d).flags & DEVFL_TKILL == 0 { aoedev_downdev(d); (*d).flags |= DEVFL_TKILL; } d = (*d).next; }
    d = DEVLIST; while !d.is_null() { freedev(d); d = (*d).next; } 0
}

pub unsafe fn aoedev_flush(s: *const c_char, cnt: usize) -> i32 { flush(s, cnt, NOT_EXITING as i32) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
