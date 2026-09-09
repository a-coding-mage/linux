// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 ARM Ltd.
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Linux headers and "vgic.h" provide the types, constants, and functions
// referenced below.

const DB_IRQ_FLAGS: c_ulong = IRQ_NOAUTOEN | IRQ_DISABLE_UNLAZY | IRQ_NO_BALANCING;

unsafe fn vgic_v4_doorbell_handler(irq: c_int, info: *mut c_void) -> irqreturn_t {
    let vcpu = info as *mut kvm_vcpu;

    /* We got the message, no need to fire again */
    if !kvm_vgic_global_state.has_gicv4_1
        && !irqd_irq_disabled(&(*irq_to_desc(irq)).irq_data)
    {
        disable_irq_nosync(irq);
    }

    /*
     * The v4.1 doorbell can fire concurrently with the vPE being
     * made non-resident. Ensure we only update pending_last
     * *after* the non-residency sequence has completed.
     */
    raw_spin_lock(&mut (*vcpu).arch.vgic_cpu.vgic_v3.its_vpe.vpe_lock);
    (*vcpu).arch.vgic_cpu.vgic_v3.its_vpe.pending_last = true;
    raw_spin_unlock(&mut (*vcpu).arch.vgic_cpu.vgic_v3.its_vpe.vpe_lock);

    kvm_make_request(KVM_REQ_IRQ_PENDING, vcpu);
    kvm_vcpu_kick(vcpu);

    IRQ_HANDLED
}

unsafe fn vgic_v4_sync_sgi_config(vpe: *mut its_vpe, irq: *mut vgic_irq) {
    (*vpe).sgi_config[(*irq).intid].enabled = (*irq).enabled;
    (*vpe).sgi_config[(*irq).intid].group = (*irq).group;
    (*vpe).sgi_config[(*irq).intid].priority = (*irq).priority;
}

unsafe fn vgic_v4_enable_vsgis(vcpu: *mut kvm_vcpu) {
    let vpe = &mut (*vcpu).arch.vgic_cpu.vgic_v3.its_vpe as *mut its_vpe;

    /*
     * With GICv4.1, every virtual SGI can be directly injected. So
     * let's pretend that they are HW interrupts, tied to a host
     * IRQ. The SGI code will do its magic.
     */
    for i in 0..VGIC_NR_SGIS {
        let irq = vgic_get_vcpu_irq(vcpu, i);
        let mut flags: c_ulong = 0;

        raw_spin_lock_irqsave(&mut (*irq).irq_lock, &mut flags);

        if (*irq).hw {
            raw_spin_unlock_irqrestore(&mut (*irq).irq_lock, flags);
            vgic_put_irq((*vcpu).kvm, irq);
            continue;
        }

        (*irq).hw = true;
        (*irq).host_irq = irq_find_mapping((*vpe).sgi_domain, i);
        vgic_v4_sync_sgi_config(vpe, irq);
        let desc = irq_to_desc((*irq).host_irq);
        let ret = irq_domain_activate_irq(irq_desc_get_irq_data(desc), false);
        if !WARN_ON(ret != 0) {
            let ret = irq_set_irqchip_state(
                (*irq).host_irq,
                IRQCHIP_STATE_PENDING,
                (*irq).pending_latch,
            );
            WARN_ON(ret != 0);
            (*irq).pending_latch = false;
        }

        raw_spin_unlock_irqrestore(&mut (*irq).irq_lock, flags);
        vgic_put_irq((*vcpu).kvm, irq);
    }
}

unsafe fn vgic_v4_disable_vsgis(vcpu: *mut kvm_vcpu) {
    for i in 0..VGIC_NR_SGIS {
        let irq = vgic_get_vcpu_irq(vcpu, i);
        let mut flags: c_ulong = 0;

        raw_spin_lock_irqsave(&mut (*irq).irq_lock, &mut flags);
        if !(*irq).hw {
            raw_spin_unlock_irqrestore(&mut (*irq).irq_lock, flags);
            vgic_put_irq((*vcpu).kvm, irq);
            continue;
        }

        (*irq).hw = false;
        let mut pending = false;
        let ret = irq_get_irqchip_state(
            (*irq).host_irq,
            IRQCHIP_STATE_PENDING,
            &mut pending,
        );
        WARN_ON(ret != 0);
        (*irq).pending_latch = pending;
        let desc = irq_to_desc((*irq).host_irq);
        irq_domain_deactivate_irq(irq_desc_get_irq_data(desc));

        raw_spin_unlock_irqrestore(&mut (*irq).irq_lock, flags);
        vgic_put_irq((*vcpu).kvm, irq);
    }
}

unsafe fn vgic_v4_configure_vsgis(kvm: *mut kvm) {
    let dist = &mut (*kvm).arch.vgic as *mut vgic_dist;
    lockdep_assert_held(&(*kvm).arch.config_lock);
    kvm_arm_halt_guest(kvm);
    kvm_for_each_vcpu!(i, vcpu, kvm, {
        if (*dist).nassgireq { vgic_v4_enable_vsgis(vcpu); }
        else { vgic_v4_disable_vsgis(vcpu); }
    });
    kvm_arm_resume_guest(kvm);
}

unsafe fn vgic_v4_get_vlpi_state(irq: *mut vgic_irq, val: *mut bool) {
    let vpe = &mut (*(*irq).target_vcpu).arch.vgic_cpu.vgic_v3.its_vpe as *mut its_vpe;
    let mask = BIT((*irq).intid % BITS_PER_BYTE);
    let va = page_address((*vpe).vpt_page) as *mut u8;
    let ptr = va.add((*irq).intid / BITS_PER_BYTE);
    *val = (*ptr & mask as u8) != 0;
}

unsafe fn vgic_v4_request_vpe_irq(vcpu: *mut kvm_vcpu, irq: c_int) -> c_int {
    request_irq(irq, vgic_v4_doorbell_handler, 0, "vcpu" as *const _, vcpu as *mut c_void)
}

unsafe fn vgic_v4_init(kvm: *mut kvm) -> c_int {
    let dist = &mut (*kvm).arch.vgic as *mut vgic_dist;
    lockdep_assert_held(&(*kvm).arch.config_lock);
    if !kvm_vgic_global_state.has_gicv4 { return 0; }
    if !(*dist).its_vm.vpes.is_null() { return 0; }
    let nr_vcpus = atomic_read(&(*kvm).online_vcpus);
    (*dist).its_vm.vpes = kzalloc_objs(nr_vcpus, GFP_KERNEL_ACCOUNT);
    if (*dist).its_vm.vpes.is_null() { return -ENOMEM; }
    (*dist).its_vm.nr_vpes = nr_vcpus;
    kvm_for_each_vcpu!(i, vcpu, kvm, {
        *(*dist).its_vm.vpes.add(i as usize) = &mut (*vcpu).arch.vgic_cpu.vgic_v3.its_vpe;
    });
    let mut ret = its_alloc_vcpu_irqs(&mut (*dist).its_vm);
    if ret < 0 {
        kvm_err!("VPE IRQ allocation failure\n");
        kfree((*dist).its_vm.vpes as *mut c_void);
        (*dist).its_vm.nr_vpes = 0;
        (*dist).its_vm.vpes = core::ptr::null_mut();
        return ret;
    }
    kvm_for_each_vcpu!(i, vcpu, kvm, {
        let irq = (*(*dist).its_vm.vpes.add(i as usize)).irq;
        let mut irq_flags = DB_IRQ_FLAGS;
        if kvm_vgic_global_state.has_gicv4_1 { irq_flags &= !IRQ_NOAUTOEN; }
        irq_set_status_flags(irq, irq_flags);
        ret = vgic_v4_request_vpe_irq(vcpu, irq);
        if ret != 0 { (*dist).its_vm.nr_vpes = i; }
    });
    if ret != 0 { vgic_v4_teardown(kvm); }
    ret
}

unsafe fn vgic_v4_teardown(kvm: *mut kvm) {
    let its_vm = &mut (*kvm).arch.vgic.its_vm as *mut its_vm;
    lockdep_assert_held(&(*kvm).arch.config_lock);
    if (*its_vm).vpes.is_null() { return; }
    for i in 0..(*its_vm).nr_vpes {
        let vcpu = kvm_get_vcpu(kvm, i);
        let irq = (*(*its_vm).vpes.add(i as usize)).irq;
        irq_clear_status_flags(irq, DB_IRQ_FLAGS);
        free_irq(irq, vcpu);
    }
    its_free_vcpu_irqs(its_vm);
    kfree((*its_vm).vpes as *mut c_void);
    (*its_vm).nr_vpes = 0;
    (*its_vm).vpes = core::ptr::null_mut();
}

unsafe fn vgic_v4_want_doorbell(vcpu: *mut kvm_vcpu) -> bool {
    if vcpu_get_flag(vcpu, IN_WFI) { return true; }
    if likely(!vcpu_has_nv(vcpu)) { return false; }
    vcpu_get_flag(vcpu, IN_NESTED_ERET)
}

unsafe fn vgic_v4_put(vcpu: *mut kvm_vcpu) -> c_int {
    let vpe = &mut (*vcpu).arch.vgic_cpu.vgic_v3.its_vpe as *mut its_vpe;
    if !vgic_supports_direct_irqs((*vcpu).kvm) || !(*vpe).resident { return 0; }
    its_make_vpe_non_resident(vpe, vgic_v4_want_doorbell(vcpu))
}

unsafe fn vgic_v4_load(vcpu: *mut kvm_vcpu) -> c_int {
    let vpe = &mut (*vcpu).arch.vgic_cpu.vgic_v3.its_vpe as *mut its_vpe;
    if !vgic_supports_direct_irqs((*vcpu).kvm) || (*vpe).resident || vcpu_get_flag(vcpu, IN_WFI) { return 0; }
    let mut err = irq_set_affinity((*vpe).irq, cpumask_of(smp_processor_id()));
    if err != 0 { return err; }
    err = its_make_vpe_resident(vpe, false, (*vcpu).kvm.arch.vgic.enabled);
    if err != 0 { return err; }
    if !kvm_vgic_global_state.has_gicv4_1 {
        err = irq_set_irqchip_state((*vpe).irq, IRQCHIP_STATE_PENDING, false);
    }
    err
}

unsafe fn vgic_v4_commit(vcpu: *mut kvm_vcpu) {
    let vpe = &mut (*vcpu).arch.vgic_cpu.vgic_v3.its_vpe as *mut its_vpe;
    if !(*vpe).ready { its_commit_vpe(vpe); }
}

unsafe fn vgic_get_its(kvm: *mut kvm, irq_entry: *mut kvm_kernel_irq_routing_entry) -> *mut vgic_its {
    let msi = kvm_msi {
        address_lo: (*irq_entry).msi.address_lo,
        address_hi: (*irq_entry).msi.address_hi,
        data: (*irq_entry).msi.data,
        flags: (*irq_entry).msi.flags,
        devid: (*irq_entry).msi.devid,
    };
    vgic_msi_to_its(kvm, &msi)
}

unsafe fn kvm_vgic_v4_set_forwarding(kvm: *mut kvm, virq: c_int, irq_entry: *mut kvm_kernel_irq_routing_entry) -> c_int {
    if !vgic_supports_direct_msis(kvm) { return 0; }
    let its = vgic_get_its(kvm, irq_entry);
    if IS_ERR(its) { return 0; }
    let irq_lock = &mut (*its).its_lock;
    mutex_lock(irq_lock);
    let mut irq: *mut vgic_irq = core::ptr::null_mut();
    if vgic_its_resolve_lpi(kvm, its, (*irq_entry).msi.devid, (*irq_entry).msi.data, &mut irq) != 0 {
        mutex_unlock(irq_lock); return 0;
    }
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*irq).irq_lock, &mut flags);
    if (*irq).hw { raw_spin_unlock_irqrestore(&mut (*irq).irq_lock, flags); mutex_unlock(irq_lock); return 0; }
    let map = its_vlpi_map {
        vm: &mut (*kvm).arch.vgic.its_vm,
        vpe: &mut (*(*irq).target_vcpu).arch.vgic_cpu.vgic_v3.its_vpe,
        vintid: (*irq).intid,
        properties: ((*irq).priority & 0xfc) | if (*irq).enabled { LPI_PROP_ENABLED } else { 0 } | LPI_PROP_GROUP1,
        db_enabled: true,
    };
    let mut ret = its_map_vlpi(virq, &map);
    if ret != 0 { raw_spin_unlock_irqrestore(&mut (*irq).irq_lock, flags); mutex_unlock(irq_lock); return ret; }
    (*irq).hw = true; (*irq).host_irq = virq; atomic_inc(&mut (*map.vpe).vlpi_count);
    if (*irq).pending_latch {
        ret = irq_set_irqchip_state((*irq).host_irq, IRQCHIP_STATE_PENDING, (*irq).pending_latch);
        WARN_RATELIMIT(ret, "IRQ %d", (*irq).host_irq);
        (*irq).pending_latch = false;
        vgic_queue_irq_unlock(kvm, irq, flags);
    } else { raw_spin_unlock_irqrestore(&mut (*irq).irq_lock, flags); }
    mutex_unlock(irq_lock); ret
}

unsafe fn __vgic_host_irq_get_vlpi(kvm: *mut kvm, host_irq: c_int) -> *mut vgic_irq {
    let mut irq: *mut vgic_irq = core::ptr::null_mut();
    xa_for_each!(&(*kvm).arch.vgic.lpi_xa, _idx, irq, {
        if (*irq).hw && (*irq).host_irq == host_irq && vgic_try_get_irq_ref(irq) { return irq; }
    });
    core::ptr::null_mut()
}

unsafe fn kvm_vgic_v4_unset_forwarding(kvm: *mut kvm, host_irq: c_int) {
    if !vgic_supports_direct_msis(kvm) { return; }
    let irq = __vgic_host_irq_get_vlpi(kvm, host_irq);
    if irq.is_null() { return; }
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*irq).irq_lock, &mut flags);
    WARN_ON((*irq).hw && (*irq).host_irq != host_irq);
    if (*irq).hw {
        atomic_dec(&mut (*(*irq).target_vcpu).arch.vgic_cpu.vgic_v3.its_vpe.vlpi_count);
        (*irq).hw = false;
        its_unmap_vlpi(host_irq);
    }
    raw_spin_unlock_irqrestore(&mut (*irq).irq_lock, flags);
    vgic_put_irq(kvm, irq);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
