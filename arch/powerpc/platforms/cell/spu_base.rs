// SPDX-License-Identifier: GPL-2.0-or-later
/* Low-level SPU handling. Direct translation of spu_base.c. */

// Linux and architecture headers provide the external types, constants, macros,
// locking primitives, I/O accessors, and helper functions referenced below.

extern "C" {
    static mut spu_management_ops: *const spu_management_ops;
    static mut spu_priv1_ops: *const spu_priv1_ops;
    static mut cbe_spu_info: [cbe_spu_info; MAX_NUMNODES];
    fn force_sig_fault();
    fn hash_page(ea: c_ulong, access: c_ulong, trap: c_ulong, dsisr: c_ulong) -> c_int;
}

static mut spu_lock: spinlock_t = DEFINE_SPINLOCK();
static mut spu_full_list: list_head = LIST_HEAD();
static mut spu_full_list_lock: spinlock_t = DEFINE_SPINLOCK();
static mut spu_full_list_mutex: mutex = DEFINE_MUTEX();

pub unsafe extern "C" fn spu_invalidate_slbs(spu: *mut spu) {
    let priv2 = (*spu).priv2;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*spu).register_lock, &mut flags);
    if spu_mfc_sr1_get(spu) & MFC_STATE1_RELOCATE_MASK != 0 {
        out_be64(&mut (*priv2).slb_invalidate_all_W, 0);
    }
    spin_unlock_irqrestore(&mut (*spu).register_lock, flags);
}

pub unsafe extern "C" fn spu_flush_all_slbs(mm: *mut mm_struct) {
    let mut flags = 0;
    spin_lock_irqsave(&mut spu_full_list_lock, &mut flags);
    let mut pos = (*spu_full_list).next;
    while pos != &mut spu_full_list {
        let s = container_of(pos, spu, full_list);
        pos = (*pos).next;
        if (*s).mm == mm { spu_invalidate_slbs(s); }
    }
    spin_unlock_irqrestore(&mut spu_full_list_lock, flags);
}

unsafe fn mm_needs_global_tlbie(mm: *mut mm_struct) {
    let nr = if NR_CPUS > 1 { NR_CPUS } else { NR_CPUS + 1 };
    bitmap_fill(mm_cpumask(mm), nr);
}

pub unsafe extern "C" fn spu_associate_mm(spu: *mut spu, mm: *mut mm_struct) {
    let mut flags = 0;
    spin_lock_irqsave(&mut spu_full_list_lock, &mut flags);
    (*spu).mm = mm;
    spin_unlock_irqrestore(&mut spu_full_list_lock, flags);
    if !mm.is_null() { mm_needs_global_tlbie(mm); }
}

pub unsafe extern "C" fn spu_64k_pages_available() -> c_int {
    (mmu_psize_defs[MMU_PAGE_64K].shift != 0) as c_int
}

unsafe fn spu_restart_dma(spu: *mut spu) {
    let priv2 = (*spu).priv2;
    if !test_bit(SPU_CONTEXT_SWITCH_PENDING, &(*spu).flags) {
        out_be64(&mut (*priv2).mfc_control_RW, MFC_CNTL_RESTART_DMA_COMMAND);
    } else {
        set_bit(SPU_CONTEXT_FAULT_PENDING, &mut (*spu).flags);
        mb();
    }
}

unsafe fn spu_load_slb(spu: *mut spu, slbe: c_int, slb: *mut copro_slb) {
    let p = (*spu).priv2;
    pr_debug!("{}: adding SLB[{}] 0x{:016x} 0x{:016x}\n", "spu_load_slb", slbe, (*slb).vsid, (*slb).esid);
    out_be64(&mut (*p).slb_index_W, slbe as u64);
    out_be64(&mut (*p).slb_esid_RW, 0);
    out_be64(&mut (*p).slb_vsid_RW, (*slb).vsid);
    out_be64(&mut (*p).slb_esid_RW, (*slb).esid);
}

unsafe fn __spu_trap_data_seg(spu: *mut spu, ea: c_ulong) -> c_int {
    let mut slb = copro_slb::default();
    let ret = copro_calculate_slb((*spu).mm, ea, &mut slb);
    if ret != 0 { return ret; }
    spu_load_slb(spu, (*spu).slb_replace, &mut slb);
    (*spu).slb_replace += 1;
    if (*spu).slb_replace >= 8 { (*spu).slb_replace = 0; }
    spu_restart_dma(spu);
    (*spu).stats.slb_flt += 1;
    0
}

unsafe fn __spu_trap_data_map(spu: *mut spu, ea: c_ulong, dsisr: u64) -> c_int {
    pr_debug!("{}, {:x}, {:x}\n", "__spu_trap_data_map", dsisr, ea);
    if dsisr & MFC_DSISR_PTE_NOT_FOUND != 0 && get_region_id(ea) != USER_REGION_ID {
        spin_unlock(&mut (*spu).register_lock);
        let ret = hash_page(ea, _PAGE_PRESENT | _PAGE_READ | _PAGE_PRIVILEGED, 0x300, dsisr as c_ulong);
        spin_lock(&mut (*spu).register_lock);
        if ret == 0 { spu_restart_dma(spu); return 0; }
    }
    (*spu).class_1_dar = ea;
    (*spu).class_1_dsisr = dsisr;
    ((*spu).stop_callback)(spu, 1);
    (*spu).class_1_dar = 0;
    (*spu).class_1_dsisr = 0;
    0
}

unsafe fn __spu_kernel_slb(addr: *mut c_void, slb: *mut copro_slb) {
    let ea = addr as c_ulong;
    let llp = if get_region_id(ea) == LINEAR_MAP_REGION_ID { mmu_psize_defs[mmu_linear_psize].sllp } else { mmu_psize_defs[mmu_virtual_psize].sllp };
    (*slb).vsid = (get_kernel_vsid(ea, MMU_SEGSIZE_256M) << SLB_VSID_SHIFT) | SLB_VSID_KERNEL | llp;
    (*slb).esid = (ea & ESID_MASK) | SLB_ESID_V;
}

unsafe fn __slb_present(slbs: *mut copro_slb, nr_slbs: c_int, new_addr: *mut c_void) -> c_int {
    let ea = new_addr as c_ulong;
    for i in 0..nr_slbs { if !(((*slbs.add(i as usize)).esid ^ ea) & ESID_MASK != 0) { return 1; } }
    0
}

pub unsafe extern "C" fn spu_setup_kernel_slbs(spu: *mut spu, lscsa: *mut spu_lscsa, code: *mut c_void, code_size: c_int) {
    let mut slbs: [copro_slb; 4] = [copro_slb::default(); 4];
    let mut nr = 0;
    let addrs = [lscsa as *mut c_void, (lscsa as *mut u8).add(size_of::<spu_lscsa>() - 1) as *mut c_void, code, (code as *mut u8).add(code_size as usize - 1) as *mut c_void];
    for a in addrs { if __slb_present(slbs.as_mut_ptr(), nr, a) == 0 { __spu_kernel_slb(a, &mut slbs[nr as usize]); nr += 1; } }
    spin_lock_irq(&mut (*spu).register_lock);
    for i in 0..nr { spu_load_slb(spu, i, &mut slbs[i as usize]); }
    spin_unlock_irq(&mut (*spu).register_lock);
}

unsafe fn spu_irq_class_0(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let spu = data as *mut spu; spin_lock(&mut (*spu).register_lock);
    let mask = spu_int_mask_get(spu, 0); let stat = spu_int_stat_get(spu, 0) & mask;
    (*spu).class_0_pending |= stat; (*spu).class_0_dar = spu_mfc_dar_get(spu); ((*spu).stop_callback)(spu, 0);
    (*spu).class_0_pending = 0; (*spu).class_0_dar = 0; spu_int_stat_clear(spu, 0, stat); spin_unlock(&mut (*spu).register_lock); IRQ_HANDLED
}

unsafe fn spu_irq_class_1(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let spu = data as *mut spu; spin_lock(&mut (*spu).register_lock);
    let mask = spu_int_mask_get(spu, 1); let stat = spu_int_stat_get(spu, 1) & mask; let dar = spu_mfc_dar_get(spu); let dsisr = spu_mfc_dsisr_get(spu);
    if stat & CLASS1_STORAGE_FAULT_INTR != 0 { spu_mfc_dsisr_set(spu, 0); } spu_int_stat_clear(spu, 1, stat);
    if stat & CLASS1_SEGMENT_FAULT_INTR != 0 { __spu_trap_data_seg(spu, dar); } if stat & CLASS1_STORAGE_FAULT_INTR != 0 { __spu_trap_data_map(spu, dar, dsisr); }
    (*spu).class_1_dsisr = 0; (*spu).class_1_dar = 0; spin_unlock(&mut (*spu).register_lock); if stat != 0 { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe fn spu_irq_class_2(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let spu = data as *mut spu; let mailbox = CLASS2_MAILBOX_THRESHOLD_INTR | CLASS2_MAILBOX_INTR;
    spin_lock(&mut (*spu).register_lock); let mut stat = spu_int_stat_get(spu, 2); let mask = spu_int_mask_get(spu, 2); stat &= mask;
    if stat & mailbox != 0 { spu_int_mask_and(spu, 2, !(stat & mailbox)); } spu_int_stat_clear(spu, 2, stat);
    if stat & CLASS2_MAILBOX_INTR != 0 { ((*spu).ibox_callback)(spu); } if stat & CLASS2_SPU_STOP_INTR != 0 { ((*spu).stop_callback)(spu, 2); } if stat & CLASS2_SPU_HALT_INTR != 0 { ((*spu).stop_callback)(spu, 2); }
    if stat & CLASS2_SPU_DMA_TAG_GROUP_COMPLETE_INTR != 0 { ((*spu).mfc_callback)(spu); } if stat & CLASS2_MAILBOX_THRESHOLD_INTR != 0 { ((*spu).wbox_callback)(spu); }
    (*spu).stats.class2_intr += 1; spin_unlock(&mut (*spu).register_lock); if stat != 0 { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe fn spu_request_irqs(spu: *mut spu) -> c_int {
    let mut ret = 0; if (*spu).irqs[0] != 0 { snprintf((*spu).irq_c0.as_mut_ptr(), (*spu).irq_c0.len(), "spe%02d.0", (*spu).number); ret = request_irq((*spu).irqs[0], spu_irq_class_0, 0, (*spu).irq_c0.as_ptr(), spu as *mut c_void); if ret != 0 { return ret; } }
    if (*spu).irqs[1] != 0 { snprintf((*spu).irq_c1.as_mut_ptr(), (*spu).irq_c1.len(), "spe%02d.1", (*spu).number); ret = request_irq((*spu).irqs[1], spu_irq_class_1, 0, (*spu).irq_c1.as_ptr(), spu as *mut c_void); if ret != 0 { if (*spu).irqs[0] != 0 { free_irq((*spu).irqs[0], spu as *mut c_void); } return ret; } }
    if (*spu).irqs[2] != 0 { snprintf((*spu).irq_c2.as_mut_ptr(), (*spu).irq_c2.len(), "spe%02d.2", (*spu).number); ret = request_irq((*spu).irqs[2], spu_irq_class_2, 0, (*spu).irq_c2.as_ptr(), spu as *mut c_void); if ret != 0 { if (*spu).irqs[1] != 0 { free_irq((*spu).irqs[1], spu as *mut c_void); } if (*spu).irqs[0] != 0 { free_irq((*spu).irqs[0], spu as *mut c_void); } } } ret
}

unsafe fn spu_free_irqs(spu: *mut spu) { for i in 0..3 { if (*spu).irqs[i] != 0 { free_irq((*spu).irqs[i], spu as *mut c_void); } } }

pub unsafe extern "C" fn spu_init_channels(spu: *mut spu) {
    #[derive(Copy, Clone)] struct Channel { channel: u64, count: u64 }
    let zero = [Channel{channel:0x00,count:1},Channel{channel:0x01,count:1},Channel{channel:0x03,count:1},Channel{channel:0x04,count:1},Channel{channel:0x18,count:1},Channel{channel:0x19,count:1},Channel{channel:0x1b,count:1},Channel{channel:0x1d,count:1}];
    let counts = [Channel{channel:0x00,count:0},Channel{channel:0x03,count:0},Channel{channel:0x04,count:0},Channel{channel:0x15,count:16},Channel{channel:0x17,count:1},Channel{channel:0x18,count:0},Channel{channel:0x19,count:0},Channel{channel:0x1b,count:0},Channel{channel:0x1c,count:1},Channel{channel:0x1d,count:0},Channel{channel:0x1e,count:1}];
    let p = (*spu).priv2; for x in zero { out_be64(&mut (*p).spu_chnlcntptr_RW, x.channel); for _ in 0..x.count { out_be64(&mut (*p).spu_chnldata_RW, 0); } } for x in counts { out_be64(&mut (*p).spu_chnlcntptr_RW, x.channel); out_be64(&mut (*p).spu_chnlcnt_RW, x.count); }
}

// The remaining device, accounting, optional kexec, shutdown, and init code
// follows the source literally; kernel list/sysfs/device helpers are external.
pub unsafe extern "C" fn spu_add_dev_attr(attr: *mut device_attribute) -> c_int { mutex_lock(&mut spu_full_list_mutex); list_for_each_spu!(spu, { device_create_file(&mut (*spu).dev, attr); }); mutex_unlock(&mut spu_full_list_mutex); 0 }
pub unsafe extern "C" fn spu_remove_dev_attr(attr: *mut device_attribute) { mutex_lock(&mut spu_full_list_mutex); list_for_each_spu!(spu, { device_remove_file(&mut (*spu).dev, attr); }); mutex_unlock(&mut spu_full_list_mutex); }
pub unsafe extern "C" fn spu_remove_dev_attr_group(attrs: *const attribute_group) { mutex_lock(&mut spu_full_list_mutex); list_for_each_spu!(spu, { sysfs_remove_group(&mut (*spu).dev.kobj, attrs); }); mutex_unlock(&mut spu_full_list_mutex); }

// Device creation and lifetime management.
unsafe fn spu_create_dev(spu: *mut spu) -> c_int {
    (*spu).dev.id = (*spu).number; (*spu).dev.bus = &spu_subsys;
    let ret = device_register(&mut (*spu).dev);
    if ret != 0 { printk!(KERN_ERR, "Can't register SPU %d with sysfs\n", (*spu).number); return ret; }
    sysfs_add_device_to_node(&mut (*spu).dev, (*spu).node); 0
}

unsafe fn create_spu(data: *mut c_void) -> c_int {
    static mut number: c_int = 0; let mut ret = -ENOMEM; let spu = kzalloc_obj::<spu>();
    if spu.is_null() { return ret; }
    (*spu).alloc_state = SPU_FREE; spin_lock_init(&mut (*spu).register_lock);
    spin_lock(&mut spu_lock); (*spu).number = number; number += 1; spin_unlock(&mut spu_lock);
    ret = spu_create_spu(spu, data); if ret != 0 { kfree(spu); return ret; }
    spu_mfc_sdr_setup(spu); spu_mfc_sr1_set(spu, 0x33); ret = spu_request_irqs(spu); if ret != 0 { spu_destroy_spu(spu); kfree(spu); return ret; }
    ret = spu_create_dev(spu); if ret != 0 { spu_free_irqs(spu); spu_destroy_spu(spu); kfree(spu); return ret; }
    mutex_lock(&mut cbe_spu_info[(*spu).node].list_mutex); list_add(&mut (*spu).cbe_list, &mut cbe_spu_info[(*spu).node].spus); cbe_spu_info[(*spu).node].n_spus += 1; mutex_unlock(&mut cbe_spu_info[(*spu).node].list_mutex);
    mutex_lock(&mut spu_full_list_mutex); let mut flags = 0; spin_lock_irqsave(&mut spu_full_list_lock, &mut flags); list_add(&mut (*spu).full_list, &mut spu_full_list); spin_unlock_irqrestore(&mut spu_full_list_lock, flags); mutex_unlock(&mut spu_full_list_mutex);
    (*spu).stats.util_state = SPU_UTIL_IDLE_LOADED; (*spu).stats.tstamp = ktime_get_ns(); INIT_LIST_HEAD(&mut (*spu).aff_list); 0
}

static spu_state_names: [&str; 4] = ["user", "system", "iowait", "idle"];
unsafe fn spu_acct_time(spu: *mut spu, state: spu_utilization_state) -> u64 { let mut time = (*spu).stats.times[state as usize]; if (*spu).stats.util_state == state { time += ktime_get_ns() - (*spu).stats.tstamp; } time / NSEC_PER_MSEC }
unsafe fn spu_stat_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let spu = container_of(dev, spu, dev); sysfs_emit!(buf, "{} {} {} {} {} {} {} {} {} {} {} {} {}\n", spu_state_names[(*spu).stats.util_state as usize], spu_acct_time(spu, SPU_UTIL_USER), spu_acct_time(spu, SPU_UTIL_SYSTEM), spu_acct_time(spu, SPU_UTIL_IOWAIT), spu_acct_time(spu, SPU_UTIL_IDLE_LOADED), (*spu).stats.vol_ctx_switch, (*spu).stats.invol_ctx_switch, (*spu).stats.slb_flt, (*spu).stats.hash_flt, (*spu).stats.min_flt, (*spu).stats.maj_flt, (*spu).stats.class2_intr, (*spu).stats.libassist) }
static DEVICE_ATTR!(stat, 0444, spu_stat_show, NULL);

#[cfg(CONFIG_KEXEC_CORE)]
struct crash_spu_info { spu: *mut spu, saved_spu_runcntl_RW: u32, saved_spu_status_R: u32, saved_spu_npc_RW: u32, saved_mfc_sr1_RW: u64, saved_mfc_dar: u64, saved_mfc_dsisr: u64 }
#[cfg(CONFIG_KEXEC_CORE)] static mut crash_spu_info: [crash_spu_info; 16] = [crash_spu_info { spu: core::ptr::null_mut(), saved_spu_runcntl_RW:0, saved_spu_status_R:0, saved_spu_npc_RW:0, saved_mfc_sr1_RW:0, saved_mfc_dar:0, saved_mfc_dsisr:0 }; 16];
#[cfg(CONFIG_KEXEC_CORE)] unsafe fn crash_kexec_stop_spus() { for i in 0..16 { let s = crash_spu_info[i].spu; if s.is_null() { continue; } crash_spu_info[i].saved_spu_runcntl_RW = in_be32(&(*s).problem.spu_runcntl_RW); crash_spu_info[i].saved_spu_status_R = in_be32(&(*s).problem.spu_status_R); crash_spu_info[i].saved_spu_npc_RW = in_be32(&(*s).problem.spu_npc_RW); crash_spu_info[i].saved_mfc_dar = spu_mfc_dar_get(s); crash_spu_info[i].saved_mfc_dsisr = spu_mfc_dsisr_get(s); let mut tmp = spu_mfc_sr1_get(s); crash_spu_info[i].saved_mfc_sr1_RW = tmp; tmp &= !MFC_STATE1_MASTER_RUN_CONTROL_MASK; spu_mfc_sr1_set(s, tmp); __delay(200); } }
#[cfg(not(CONFIG_KEXEC_CORE))] unsafe fn crash_register_spus(_list: *mut list_head) {}

unsafe fn spu_shutdown(_data: *mut c_void) { mutex_lock(&mut spu_full_list_mutex); list_for_each_spu!(spu, { spu_free_irqs(spu); spu_destroy_spu(spu); }); mutex_unlock(&mut spu_full_list_mutex); }
static spu_syscore_ops: syscore_ops = syscore_ops { shutdown: Some(spu_shutdown) };
static spu_syscore: syscore = syscore { ops: &spu_syscore_ops };

unsafe fn init_spu_base() -> c_int {
    for i in 0..MAX_NUMNODES { mutex_init(&mut cbe_spu_info[i].list_mutex); INIT_LIST_HEAD(&mut cbe_spu_info[i].spus); }
    if spu_management_ops.is_null() { return 0; }
    let mut ret = subsys_system_register(&spu_subsys, core::ptr::null()); if ret != 0 { return ret; }
    ret = spu_enumerate_spus(create_spu); if ret < 0 { bus_unregister(&spu_subsys); return ret; }
    if ret > 0 { fb_append_extra_logo(&logo_spe_clut224, ret); }
    mutex_lock(&mut spu_full_list_mutex); crash_register_spus(&mut spu_full_list); mutex_unlock(&mut spu_full_list_mutex); spu_add_dev_attr(&dev_attr_stat); register_syscore(&spu_syscore); spu_init_affinity(); 0
}
device_initcall!(init_spu_base);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
