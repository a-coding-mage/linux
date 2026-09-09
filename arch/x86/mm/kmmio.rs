// SPDX-License-Identifier: GPL-2.0
/* Support for MMIO probes.
 * Benefit many code from kprobes
 * (C) 2002 Louis Zhuang <louis.zhuang@intel.com>.
 *     2007 Alexander Eichner
 *     2008 Pekka Paalanen <pq@iki.fi>
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Dependencies supplied by the surrounding kernel translation are intentionally external.

pub const KMMIO_PAGE_HASH_BITS: usize = 4;
pub const KMMIO_PAGE_TABLE_SIZE: usize = 1 << KMMIO_PAGE_HASH_BITS;

#[repr(C)]
pub struct kmmio_fault_page {
    pub list: list_head,
    pub release_next: *mut kmmio_fault_page,
    pub addr: c_ulong, // the requested address
    pub old_presence: pteval_t, // page presence prior to arming
    pub armed: bool,
    /*
     * Number of times this page has been registered as a part
     * of a probe. If zero, page is disarmed and this may be freed.
     * Used only by writers (RCU) and post_kmmio_handler().
     * Protected by kmmio_lock, when linked into kmmio_page_table.
     */
    pub count: c_int,
    pub scheduled_for_release: bool,
}

#[repr(C)]
pub struct kmmio_delayed_release {
    pub rcu: rcu_head,
    pub release_list: *mut kmmio_fault_page,
}

#[repr(C)]
pub struct kmmio_context {
    pub fpage: *mut kmmio_fault_page,
    pub probe: *mut kmmio_probe,
    pub saved_flags: c_ulong,
    pub addr: c_ulong,
    pub active: c_int,
}

/*
 * The kmmio_lock is taken in int3 context, which is treated as NMI context.
 * This causes lockdep to complain about it bein in both NMI and normal
 * context. Hide it from lockdep, as it should not have any other locks
 * taken under it, and this is only enabled for debugging mmio anyway.
 */
static mut kmmio_lock: arch_spinlock_t = unsafe { __ARCH_SPIN_LOCK_UNLOCKED };

/* Protected by kmmio_lock */
pub static mut kmmio_count: c_uint = 0;

/* Read-protected by RCU, write-protected by kmmio_lock. */
static mut kmmio_page_table: [list_head; KMMIO_PAGE_TABLE_SIZE] = [unsafe { core::mem::zeroed() }; KMMIO_PAGE_TABLE_SIZE];
static mut kmmio_probes: list_head = unsafe { core::mem::zeroed() };

extern "C" {
    static mut kmmio_ctx: percpu__kmmio_context;
}

unsafe fn kmmio_page_list(mut addr: c_ulong) -> *mut list_head {
    let mut l: c_uint = 0;
    let pte = lookup_address(addr, &mut l);
    if pte.is_null() { return core::ptr::null_mut(); }
    addr &= page_level_mask(l);
    &mut kmmio_page_table[hash_long(addr, KMMIO_PAGE_HASH_BITS as c_uint) as usize]
}

/* Accessed per-cpu */
// DEFINE_PER_CPU(struct kmmio_context, kmmio_ctx)

/*
 * this is basically a dynamic stabbing problem:
 * Could use the existing prio tree code or
 * Possible better implementations:
 * The Interval Skip List: A Data Structure for Finding All Intervals That
 * Overlap a Point (might be simple)
 * Space Efficient Dynamic Stabbing with Fast Queries - Mikkel Thorup
 */
/* Get the kmmio at this addr (if any). You must be holding RCU read lock. */
unsafe fn get_kmmio_probe(addr: c_ulong) -> *mut kmmio_probe {
    let mut p: *mut kmmio_probe = core::ptr::null_mut();
    list_for_each_entry_rcu!(p, &mut kmmio_probes, list) {
        if addr >= (*p).addr && addr < (*p).addr.wrapping_add((*p).len) { return p; }
    }
    core::ptr::null_mut()
}

/* You must be holding RCU read lock. */
unsafe fn get_kmmio_fault_page(mut addr: c_ulong) -> *mut kmmio_fault_page {
    let head: *mut list_head;
    let mut f: *mut kmmio_fault_page = core::ptr::null_mut();
    let mut l: c_uint = 0;
    let pte = lookup_address(addr, &mut l);
    if pte.is_null() { return core::ptr::null_mut(); }
    addr &= page_level_mask(l);
    head = kmmio_page_list(addr);
    list_for_each_entry_rcu!(f, head, list) {
        if (*f).addr == addr { return f; }
    }
    core::ptr::null_mut()
}

unsafe fn clear_pmd_presence(pmd: *mut pmd_t, clear: bool, old: *mut pmdval_t) {
    let mut new_pmd: pmd_t;
    let v = pmd_val(*pmd);
    if clear { *old = v; new_pmd = pmd_mkinvalid(*pmd); }
    else { /* Presume this has been called with clear==true previously */ new_pmd = __pmd(*old); }
    set_pmd(pmd, new_pmd);
}

unsafe fn clear_pte_presence(pte: *mut pte_t, clear: bool, old: *mut pteval_t) {
    let v = pte_val(*pte);
    if clear { *old = v; pte_clear(&mut init_mm, 0, pte); }
    else { /* Presume this has been called with clear==true previously */ set_pte_atomic(pte, __pte(*old)); }
}

unsafe fn clear_page_presence(f: *mut kmmio_fault_page, clear: bool) -> c_int {
    let mut level: c_uint = 0;
    let pte = lookup_address((*f).addr, &mut level);
    if pte.is_null() { pr_err!("no pte for addr 0x%08lx\n", (*f).addr); return -1; }
    match level {
        PG_LEVEL_2M => clear_pmd_presence(pte as *mut pmd_t, clear, &mut (*f).old_presence as *mut _ as *mut pmdval_t),
        PG_LEVEL_4K => clear_pte_presence(pte, clear, &mut (*f).old_presence),
        _ => { pr_err!("unexpected page level 0x%x.\n", level); return -1; }
    }
    flush_tlb_one_kernel((*f).addr); 0
}

unsafe fn arm_kmmio_fault_page(f: *mut kmmio_fault_page) -> c_int {
    let ret: c_int;
    WARN_ONCE!((*f).armed, KERN_ERR, pr_fmt!("kmmio page already armed.\n"));
    if (*f).armed { pr_warn!("double-arm: addr 0x%08lx, ref %d, old %d\n", (*f).addr, (*f).count, !!(*f).old_presence); }
    ret = clear_page_presence(f, true);
    WARN_ONCE!(ret < 0, KERN_ERR, pr_fmt!("arming at 0x%08lx failed.\n"), (*f).addr);
    (*f).armed = true; ret
}

/** Restore the given page to saved presence state. */
unsafe fn disarm_kmmio_fault_page(f: *mut kmmio_fault_page) {
    let ret = clear_page_presence(f, false);
    WARN_ONCE!(ret < 0, KERN_ERR, "kmmio disarming at 0x%08lx failed.\n", (*f).addr);
    (*f).armed = false;
}

/* This is being called from do_page_fault(). */
pub unsafe fn kmmio_handler(regs: *mut pt_regs, addr: c_ulong) -> c_int {
    let mut page_base = addr;
    let mut l: c_uint = 0;
    let pte = lookup_address(addr, &mut l);
    if pte.is_null() { return -EINVAL; }
    page_base &= page_level_mask(l);
    rcu_read_lock_sched_notrace();
    let faultpage = get_kmmio_fault_page(page_base);
    if faultpage.is_null() { rcu_read_unlock_sched_notrace(); return 0; }
    let ctx = this_cpu_ptr(&mut kmmio_ctx);
    if (*ctx).active != 0 {
        if page_base == (*ctx).addr {
            pr_debug!("secondary hit for 0x%08lx CPU %d.\n", addr, smp_processor_id());
            if !(*faultpage).old_presence { pr_info!("unexpected secondary hit for address 0x%08lx on CPU %d.\n", addr, smp_processor_id()); }
        } else {
            pr_emerg!("recursive probe hit on CPU %d, for address 0x%08lx. Ignoring.\n", smp_processor_id(), addr);
            pr_emerg!("previous hit was at 0x%08lx.\n", (*ctx).addr);
            disarm_kmmio_fault_page(faultpage);
        }
        rcu_read_unlock_sched_notrace(); return 0;
    }
    (*ctx).active += 1;
    (*ctx).fpage = faultpage;
    (*ctx).probe = get_kmmio_probe(page_base);
    (*ctx).saved_flags = (*regs).flags & (X86_EFLAGS_TF | X86_EFLAGS_IF);
    (*ctx).addr = page_base;
    if !(*ctx).probe.is_null() && (*(*ctx).probe).pre_handler.is_some() { ((*(*ctx).probe).pre_handler.unwrap())((*ctx).probe, regs, addr); }
    (*regs).flags |= X86_EFLAGS_TF;
    (*regs).flags &= !X86_EFLAGS_IF;
    disarm_kmmio_fault_page((*ctx).fpage);
    1
}

unsafe fn post_kmmio_handler(condition: c_ulong, regs: *mut pt_regs) -> c_int {
    let ctx = this_cpu_ptr(&mut kmmio_ctx);
    if (*ctx).active == 0 { pr_warn!("unexpected debug trap on CPU %d.\n", smp_processor_id()); return 0; }
    if !(*ctx).probe.is_null() && (*(*ctx).probe).post_handler.is_some() { ((*(*ctx).probe).post_handler.unwrap())((*ctx).probe, condition, regs); }
    arch_spin_lock(&mut kmmio_lock);
    if (*(*ctx).fpage).count != 0 { arm_kmmio_fault_page((*ctx).fpage); }
    arch_spin_unlock(&mut kmmio_lock);
    (*regs).flags &= !X86_EFLAGS_TF;
    (*regs).flags |= (*ctx).saved_flags;
    (*ctx).active -= 1;
    BUG_ON!((*ctx).active != 0);
    rcu_read_unlock_sched_notrace();
    if (*regs).flags & X86_EFLAGS_TF == 0 { 1 } else { 0 }
}

unsafe fn add_kmmio_fault_page(addr: c_ulong) -> c_int {
    let f = get_kmmio_fault_page(addr);
    if !f.is_null() { if (*f).count == 0 { arm_kmmio_fault_page(f); } (*f).count += 1; return 0; }
    let f = kzalloc::<kmmio_fault_page>(GFP_ATOMIC);
    if f.is_null() { return -1; }
    (*f).count = 1; (*f).addr = addr;
    if arm_kmmio_fault_page(f) != 0 { kfree(f); return -1; }
    list_add_rcu!(&mut (*f).list, kmmio_page_list((*f).addr)); 0
}

unsafe fn release_kmmio_fault_page(addr: c_ulong, release_list: *mut *mut kmmio_fault_page) {
    let f = get_kmmio_fault_page(addr); if f.is_null() { return; }
    (*f).count -= 1; BUG_ON!((*f).count < 0);
    if (*f).count == 0 { disarm_kmmio_fault_page(f); if !(*f).scheduled_for_release { (*f).release_next = *release_list; *release_list = f; (*f).scheduled_for_release = true; } }
}

unsafe fn rcu_free_kmmio_fault_pages(head: *mut rcu_head) {
    let dr = container_of!(head, kmmio_delayed_release, rcu); let mut f = (*dr).release_list;
    while !f.is_null() { let next = (*f).release_next; BUG_ON!((*f).count != 0); kfree(f); f = next; } kfree(dr);
}

unsafe fn remove_kmmio_fault_pages(head: *mut rcu_head) {
    let dr = container_of!(head, kmmio_delayed_release, rcu); let mut f = (*dr).release_list;
    while !f.is_null() { if (*f).count == 0 { list_del_rcu!(&mut (*f).list); } f = (*f).release_next; }
    call_rcu!(&mut (*dr).rcu, rcu_free_kmmio_fault_pages);
}

unsafe fn kmmio_die_notifier(_nb: *mut notifier_block, val: c_ulong, args: *mut c_void) -> c_int {
    let arg = args as *mut die_args; let dr6_p = ERR_PTR!((*arg).err) as *mut c_ulong;
    if val == DIE_DEBUG && (*dr6_p & DR_STEP) != 0 && post_kmmio_handler(*dr6_p, (*arg).regs) == 1 { *dr6_p &= !DR_STEP; return NOTIFY_STOP; }
    NOTIFY_DONE
}

static mut nb_die: notifier_block = notifier_block { notifier_call: Some(kmmio_die_notifier) };

pub unsafe fn register_kmmio_probe(p: *mut kmmio_probe) -> c_int {
    let mut flags = 0; let mut size = 0; let addr = (*p).addr & PAGE_MASK;
    let size_lim = (*p).len + ((*p).addr & !PAGE_MASK); let mut l = 0; let mut ret = 0;
    local_irq_save(&mut flags); arch_spin_lock(&mut kmmio_lock);
    if !get_kmmio_probe(addr).is_null() { ret = -EEXIST; } else if lookup_address(addr, &mut l).is_null() { ret = -EINVAL; } else {
        kmmio_count += 1; list_add_rcu!(&mut (*p).list, &mut kmmio_probes);
        while size < size_lim { add_kmmio_fault_page(addr + size); size += page_level_size(l); }
    }
    arch_spin_unlock(&mut kmmio_lock); local_irq_restore(flags); ret
}

pub unsafe fn unregister_kmmio_probe(p: *mut kmmio_probe) {
    let mut l = 0; let addr = (*p).addr & PAGE_MASK; if lookup_address(addr, &mut l).is_null() { return; }
    let mut size = 0; let size_lim = (*p).len + ((*p).addr & !PAGE_MASK);
    local_irq_save(&mut 0); arch_spin_lock(&mut kmmio_lock);
    while size < size_lim { release_kmmio_fault_page(addr + size, core::ptr::null_mut()); size += page_level_size(l); }
    list_del_rcu!(&mut (*p).list); kmmio_count -= 1; arch_spin_unlock(&mut kmmio_lock);
}

pub unsafe fn kmmio_init() -> c_int { for i in 0..KMMIO_PAGE_TABLE_SIZE { INIT_LIST_HEAD!(&mut kmmio_page_table[i]); } register_die_notifier(&mut nb_die) }
pub unsafe fn kmmio_cleanup() { unregister_die_notifier(&mut nb_die); for i in 0..KMMIO_PAGE_TABLE_SIZE { WARN_ONCE!(!list_empty(&kmmio_page_table[i]), KERN_ERR, "kmmio_page_table not empty at cleanup, any further tracing will leak memory.\n"); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
