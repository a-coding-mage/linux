// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Linaro
 * Author: Christoffer Dall <christoffer.dall@linaro.org>
 */

// Linux/KVM dependencies supplied by other translation units.

#[repr(C)]
struct VgicStateIter {
    nr_cpus: i32,
    nr_spis: i32,
    dist_id: i32,
    vcpu_id: i32,
    intid: usize,
}

unsafe fn iter_next(kvm: *mut Kvm, iter: *mut VgicStateIter) {
    let dist = &mut (*kvm).arch.vgic;

    if (*iter).dist_id == 0 {
        (*iter).dist_id += 1;
        return;
    }

    // Let the xarray drive the iterator after the last SPI.
    if (*iter).intid >= ((*iter).nr_spis as usize + VGIC_NR_PRIVATE_IRQS - 1) {
        if (*iter).intid == VGIC_LPI_MAX_INTID + 1 {
            return;
        }

        rcu_read_lock();
        if xa_find_after(
            &mut dist.lpi_xa,
            &mut (*iter).intid,
            VGIC_LPI_MAX_INTID,
            XA_PRESENT,
        ) == 0
        {
            (*iter).intid = VGIC_LPI_MAX_INTID + 1;
        }
        rcu_read_unlock();
        return;
    }

    (*iter).intid += 1;
    if (*iter).intid == VGIC_NR_PRIVATE_IRQS {
        (*iter).vcpu_id += 1;
        if (*iter).vcpu_id < (*iter).nr_cpus {
            (*iter).intid = 0;
        }
    }
}

unsafe fn vgic_count_lpis(kvm: *mut Kvm) -> i32 {
    let dist = &mut (*kvm).arch.vgic;
    let mut irq: *mut VgicIrq = core::ptr::null_mut();
    let mut intid: usize = 0;
    let mut nr_lpis = 0;

    rcu_read_lock();
    xa_for_each(&mut dist.lpi_xa, &mut intid, &mut irq) {
        nr_lpis += 1;
    }
    rcu_read_unlock();
    nr_lpis
}

unsafe fn iter_init(kvm: *mut Kvm, iter: *mut VgicStateIter, mut pos: i64) {
    let nr_cpus = atomic_read(&(*kvm).online_vcpus);
    core::ptr::write_bytes(iter, 0, 1);
    (*iter).nr_cpus = nr_cpus;
    (*iter).nr_spis = (*kvm).arch.vgic.nr_spis;
    while pos != 0 {
        pos -= 1;
        iter_next(kvm, iter);
    }
}

unsafe fn end_of_vgic(iter: *mut VgicStateIter) -> bool {
    (*iter).dist_id > 0
        && (*iter).vcpu_id == (*iter).nr_cpus
        && (*iter).intid >= ((*iter).nr_spis as usize + VGIC_NR_PRIVATE_IRQS)
        && (*iter).intid > VGIC_LPI_MAX_INTID
}

unsafe fn vgic_debug_start(s: *mut SeqFile, pos: *mut i64) -> *mut core::ffi::c_void {
    let kvm = (*s).private as *mut Kvm;
    let iter = kmalloc_obj::<VgicStateIter>();
    if iter.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    iter_init(kvm, iter, *pos);
    if end_of_vgic(iter) {
        kfree(iter);
        return core::ptr::null_mut();
    }
    iter as *mut core::ffi::c_void
}

unsafe fn vgic_debug_next(s: *mut SeqFile, v: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void {
    let kvm = (*s).private as *mut Kvm;
    let iter = v as *mut VgicStateIter;
    *pos += 1;
    iter_next(kvm, iter);
    if end_of_vgic(iter) {
        kfree(iter);
        return core::ptr::null_mut();
    }
    v
}

unsafe fn vgic_debug_stop(_s: *mut SeqFile, v: *mut core::ffi::c_void) {
    if is_err_or_null(v) {
        return;
    }
    kfree(v);
}

unsafe fn print_dist_state(s: *mut SeqFile, dist: *mut VgicDist, _iter: *mut VgicStateIter) {
    let v3 = (*dist).vgic_model == KVM_DEV_TYPE_ARM_VGIC_V3;
    let kvm = (*s).private as *mut Kvm;
    seq_printf(s, "Distributor\n");
    seq_printf(s, "===========\n");
    seq_printf(s, "vgic_model:\t%s\n", if v3 { "GICv3" } else { "GICv2" });
    seq_printf(s, "nr_spis:\t%d\n", (*dist).nr_spis);
    if v3 { seq_printf(s, "nr_lpis:\t%d\n", vgic_count_lpis(kvm)); }
    seq_printf(s, "enabled:\t%d\n", (*dist).enabled);
    seq_printf(s, "\n");
    seq_printf(s, "P=pending_latch, L=line_level, A=active\n");
    seq_printf(s, "E=enabled, H=hw, C=config (level=1, edge=0)\n");
    seq_printf(s, "G=group\n");
}

unsafe fn print_header(s: *mut SeqFile, _irq: *mut VgicIrq, vcpu: *mut KvmVcpu) {
    let mut id = 0;
    let mut hdr = "SPI ";
    if !vcpu.is_null() { hdr = "VCPU"; id = (*vcpu).vcpu_idx; }
    seq_printf(s, "\n");
    seq_printf(s, "%s%2d TYP   ID TGT_ID PLAEHCG     HWID   TARGET SRC PRI VCPU_ID\n", hdr, id);
    seq_printf(s, "----------------------------------------------------------------\n");
}

unsafe fn print_irq_state(s: *mut SeqFile, irq: *mut VgicIrq, vcpu: *mut KvmVcpu) {
    let irq_id = (*irq).intid;
    let irq_type = if irq_id < VGIC_NR_SGIS { "SGI" } else if irq_id < VGIC_NR_PRIVATE_IRQS { "PPI" } else if irq_id < VGIC_MAX_SPI { "SPI" } else { "LPI" };
    if irq_id == 0 || irq_id == VGIC_NR_PRIVATE_IRQS { print_header(s, irq, vcpu); }
    let mut pending = (*irq).pending_latch;
    if (*irq).hw && vgic_irq_is_sgi(irq_id) {
        let err = irq_get_irqchip_state((*irq).host_irq, IRQCHIP_STATE_PENDING, &mut pending);
        WARN_ON_ONCE(err);
    }
    seq_printf(s, "       %s %4d     %2d %d%d%d%d%d%d%d %8d %8x  %2x %3d     %2d\n", irq_type, irq_id,
        if !(*irq).target_vcpu.is_null() { (*(*irq).target_vcpu).vcpu_idx } else { -1 }, pending,
        (*irq).line_level, (*irq).active, (*irq).enabled, (*irq).hw, (*irq).config == VGIC_CONFIG_LEVEL,
        (*irq).group, (*irq).hwintid, (*irq).mpidr, (*irq).source, (*irq).priority,
        if !(*irq).vcpu.is_null() { (*(*irq).vcpu).vcpu_idx } else { -1 });
}

unsafe fn vgic_debug_show(s: *mut SeqFile, v: *mut core::ffi::c_void) -> i32 {
    let kvm = (*s).private as *mut Kvm;
    let iter = v as *mut VgicStateIter;
    if (*iter).dist_id == 0 { print_dist_state(s, &mut (*kvm).arch.vgic, iter); return 0; }
    if !(*kvm).arch.vgic.initialized { return 0; }
    let vcpu = if (*iter).vcpu_id < (*iter).nr_cpus { kvm_get_vcpu(kvm, (*iter).vcpu_id) } else { core::ptr::null_mut() };
    let irq = if (*iter).intid < VGIC_NR_PRIVATE_IRQS { vgic_get_vcpu_irq(vcpu, (*iter).intid) } else { vgic_get_irq(kvm, (*iter).intid) };
    if irq.is_null() { return 0; }
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut (*irq).irq_lock, &mut flags);
    print_irq_state(s, irq, vcpu);
    raw_spin_unlock_irqrestore(&mut (*irq).irq_lock, flags);
    vgic_put_irq(kvm, irq);
    0
}

static mut VGIC_DEBUG_SOPS: SeqOperations = SeqOperations { start: vgic_debug_start, next: vgic_debug_next, stop: vgic_debug_stop, show: vgic_debug_show };

pub unsafe fn vgic_debug_init(kvm: *mut Kvm) {
    debugfs_create_file("vgic-state", 0o444, (*kvm).debugfs_dentry, kvm, &mut VGIC_DEBUG_FOPS);
}

pub unsafe fn vgic_debug_destroy(_kvm: *mut Kvm) {}

#[repr(C)]
struct VgicItsIter { dev: *mut ItsDevice, ite: *mut ItsIte }

unsafe fn end_of_iter(iter: *mut VgicItsIter) -> bool { (*iter).dev.is_null() && (*iter).ite.is_null() }

unsafe fn vgic_its_iter_next(its: *mut VgicIts, iter: *mut VgicItsIter) {
    let mut dev = (*iter).dev;
    let mut ite = (*iter).ite;
    if ite.is_null() || list_is_last(&(*ite).ite_list, &(*dev).itt_head) {
        if list_is_last(&(*dev).dev_list, &(*its).device_list) { dev = core::ptr::null_mut(); ite = core::ptr::null_mut(); }
        else { dev = list_next_entry(dev, dev_list); ite = list_first_entry_or_null(&(*dev).itt_head, core::mem::size_of::<ItsIte>(), ite_list); }
    } else { ite = list_next_entry(ite, ite_list); }
    (*iter).dev = dev; (*iter).ite = ite;
}

unsafe fn vgic_its_debug_start(s: *mut SeqFile, pos: *mut i64) -> *mut core::ffi::c_void {
    let its = (*s).private as *mut VgicIts;
    mutex_lock(&mut (*its).its_lock);
    let dev = list_first_entry_or_null(&(*its).device_list, core::mem::size_of::<ItsDevice>(), dev_list);
    if dev.is_null() { return core::ptr::null_mut(); }
    let iter = kmalloc_obj::<VgicItsIter>();
    if iter.is_null() { return ERR_PTR(-ENOMEM); }
    (*iter).dev = dev; (*iter).ite = list_first_entry_or_null(&(*dev).itt_head, core::mem::size_of::<ItsIte>(), ite_list);
    let mut offset = *pos; while !end_of_iter(iter) && offset != 0 { offset -= 1; vgic_its_iter_next(its, iter); }
    if end_of_iter(iter) { kfree(iter); return core::ptr::null_mut(); } iter as *mut core::ffi::c_void
}

unsafe fn vgic_its_debug_next(s: *mut SeqFile, v: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void {
    let its = (*s).private as *mut VgicIts; let iter = v as *mut VgicItsIter; *pos += 1; vgic_its_iter_next(its, iter);
    if end_of_iter(iter) { kfree(iter); core::ptr::null_mut() } else { v }
}

unsafe fn vgic_its_debug_stop(s: *mut SeqFile, v: *mut core::ffi::c_void) { let its = (*s).private as *mut VgicIts; if !is_err_or_null(v) { kfree(v); } mutex_unlock(&mut (*its).its_lock); }

unsafe fn vgic_its_debug_show(s: *mut SeqFile, v: *mut core::ffi::c_void) -> i32 {
    let iter = v as *mut VgicItsIter; let dev = (*iter).dev; let ite = (*iter).ite; if ite.is_null() { return 0; }
    if list_is_first(&(*ite).ite_list, &(*dev).itt_head) { seq_printf(s, "\n"); seq_printf(s, "Device ID: 0x%x, Event ID Range: [0 - %llu]\n", (*dev).device_id, (1u64 << (*dev).num_eventid_bits) - 1); seq_printf(s, "EVENT_ID    INTID  HWINTID   TARGET   COL_ID HW\n"); seq_printf(s, "-----------------------------------------------\n"); }
    if !(*ite).irq.is_null() && !(*ite).collection.is_null() { seq_printf(s, "%8u %8u %8u %8u %8u %2d\n", (*ite).event_id, (*(*ite).irq).intid, (*(*ite).irq).hwintid, (*(*ite).collection).target_addr, (*(*ite).collection).collection_id, (*(*ite).irq).hw); }
    0
}

static mut VGIC_ITS_DEBUG_SOPS: SeqOperations = SeqOperations { start: vgic_its_debug_start, next: vgic_its_debug_next, stop: vgic_its_debug_stop, show: vgic_its_debug_show };

pub unsafe fn vgic_its_debug_init(dev: *mut KvmDevice) -> i32 {
    let its = (*dev).private as *mut VgicIts;
    let name = kasprintf(GFP_KERNEL, "vgic-its-state@%llx", (*its).vgic_its_base as u64);
    if name.is_null() { return -ENOMEM; }
    debugfs_create_file(name, 0o444, (*(*dev).kvm).debugfs_dentry, its, &mut VGIC_ITS_DEBUG_FOPS);
    kfree(name); 0
}

pub unsafe fn vgic_its_debug_destroy(_dev: *mut KvmDevice) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
