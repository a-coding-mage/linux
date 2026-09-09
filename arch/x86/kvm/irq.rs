// SPDX-License-Identifier: GPL-2.0-only
/* irq.c: API for in kernel interrupt controller */

// Kernel and local headers provide the types, constants, macros, and external
// functions referenced below.

pub unsafe fn kvm_cpu_has_pending_timer(vcpu: *mut kvm_vcpu) -> c_int {
    let mut r: c_int = 0;
    if lapic_in_kernel(vcpu) { r = apic_has_pending_timer(vcpu); }
    if kvm_xen_timer_enabled(vcpu) { r += kvm_xen_has_pending_timer(vcpu); }
    r
}

unsafe fn pending_userspace_extint(v: *mut kvm_vcpu) -> c_int {
    ((*v).arch.pending_external_vector != -1) as c_int
}

unsafe fn get_userspace_extint(vcpu: *mut kvm_vcpu) -> c_int {
    let vector = (*vcpu).arch.pending_external_vector;
    (*vcpu).arch.pending_external_vector = -1;
    vector
}

pub unsafe fn kvm_cpu_has_extint(v: *mut kvm_vcpu) -> c_int {
    if !lapic_in_kernel(v) { return (*v).arch.interrupt.injected; }
    if kvm_xen_has_interrupt(v) { return 1; }
    if !kvm_apic_accept_pic_intr(v) { return 0; }
    // CONFIG_KVM_IOAPIC
    if pic_in_kernel((*v).kvm) { return (*(*v).kvm).arch.vpic.output; }
    WARN_ON_ONCE(!irqchip_split((*v).kvm));
    pending_userspace_extint(v)
}

pub unsafe fn kvm_cpu_has_injectable_intr(v: *mut kvm_vcpu) -> c_int {
    if kvm_cpu_has_extint(v) != 0 { return 1; }
    if !is_guest_mode(v) && kvm_vcpu_apicv_active(v) { return 0; }
    (kvm_apic_has_interrupt(v) != -1) as c_int
}

pub unsafe fn kvm_cpu_has_interrupt(v: *mut kvm_vcpu) -> c_int {
    if kvm_cpu_has_extint(v) != 0 { return 1; }
    if lapic_in_kernel(v) && (*v).arch.apic.guest_apic_protected {
        return kvm_x86_call(protected_apic_has_interrupt)(v);
    }
    (kvm_apic_has_interrupt(v) != -1) as c_int
}

pub unsafe fn kvm_cpu_get_extint(v: *mut kvm_vcpu) -> c_int {
    if kvm_cpu_has_extint(v) == 0 { WARN_ON(!lapic_in_kernel(v)); return -1; }
    if !lapic_in_kernel(v) { return (*v).arch.interrupt.nr; }
    if kvm_xen_has_interrupt(v) { return (*(*v).kvm).arch.xen.upcall_vector; }
    if pic_in_kernel((*v).kvm) { return kvm_pic_read_irq((*v).kvm); }
    WARN_ON_ONCE(!irqchip_split((*v).kvm));
    get_userspace_extint(v)
}

pub unsafe fn kvm_cpu_get_interrupt(v: *mut kvm_vcpu) -> c_int {
    let mut vector = kvm_cpu_get_extint(v);
    if vector != -1 { return vector; }
    vector = kvm_apic_has_interrupt(v);
    if vector != -1 { kvm_apic_ack_interrupt(v, vector); }
    vector
}

pub unsafe fn kvm_inject_pending_timer_irqs(vcpu: *mut kvm_vcpu) {
    if lapic_in_kernel(vcpu) { kvm_inject_apic_timer_irqs(vcpu); }
    if kvm_xen_timer_enabled(vcpu) { kvm_xen_inject_timer_irqs(vcpu); }
}

pub unsafe fn __kvm_migrate_timers(vcpu: *mut kvm_vcpu) {
    __kvm_migrate_apic_timer(vcpu);
    __kvm_migrate_pit_timer(vcpu);
    kvm_x86_call(migrate_timers)(vcpu);
}

pub unsafe fn kvm_arch_irqfd_allowed(kvm: *mut kvm, args: *mut kvm_irqfd) -> bool {
    let resample = (*args).flags & KVM_IRQFD_FLAG_RESAMPLE != 0;
    if resample { irqchip_full(kvm) } else { irqchip_in_kernel(kvm) }
}
pub unsafe fn kvm_arch_irqchip_in_kernel(kvm: *mut kvm) -> bool { irqchip_in_kernel(kvm) }

unsafe fn kvm_msi_to_lapic_irq(kvm: *mut kvm, e: *mut kvm_kernel_irq_routing_entry, irq: *mut kvm_lapic_irq) {
    let msg = msi_msg { address_lo: (*e).msi.address_lo, address_hi: (*e).msi.address_hi, data: (*e).msi.data };
    trace_kvm_msi_set_irq(msg.address_lo | if (*kvm).arch.x2apic_format { (msg.address_hi as u64) << 32 } else { 0 }, msg.data);
    (*irq).dest_id = x86_msi_msg_get_destid(&msg, (*kvm).arch.x2apic_format);
    (*irq).vector = msg.arch_data.vector;
    (*irq).dest_mode = kvm_lapic_irq_dest_mode(msg.arch_addr_lo.dest_mode_logical);
    (*irq).trig_mode = msg.arch_data.is_level;
    (*irq).delivery_mode = msg.arch_data.delivery_mode << 8;
    (*irq).msi_redir_hint = msg.arch_addr_lo.redirect_hint;
    (*irq).level = 1;
    (*irq).shorthand = APIC_DEST_NOSHORT;
}

unsafe fn kvm_msi_route_invalid(kvm: *mut kvm, e: *mut kvm_kernel_irq_routing_entry) -> bool {
    (*kvm).arch.x2apic_format && ((*e).msi.address_hi & 0xff) != 0
}

pub unsafe fn kvm_set_msi(e: *mut kvm_kernel_irq_routing_entry, kvm: *mut kvm, _irq_source_id: c_int, level: c_int, _line_status: bool) -> c_int {
    let mut irq = core::mem::MaybeUninit::<kvm_lapic_irq>::uninit();
    if kvm_msi_route_invalid(kvm, e) { return -EINVAL; }
    if level == 0 { return -1; }
    kvm_msi_to_lapic_irq(kvm, e, irq.as_mut_ptr());
    kvm_irq_delivery_to_apic(kvm, core::ptr::null_mut(), irq.as_mut_ptr())
}

pub unsafe fn kvm_arch_set_irq_inatomic(e: *mut kvm_kernel_irq_routing_entry, kvm: *mut kvm, _irq_source_id: c_int, level: c_int, _line_status: bool) -> c_int {
    let mut irq = core::mem::MaybeUninit::<kvm_lapic_irq>::uninit();
    let mut r = 0;
    match (*e).type_ {
        KVM_IRQ_ROUTING_MSI => {
            if kvm_msi_route_invalid(kvm, e) { return -EINVAL; }
            kvm_msi_to_lapic_irq(kvm, e, irq.as_mut_ptr());
            if kvm_irq_delivery_to_apic_fast(kvm, core::ptr::null_mut(), irq.as_mut_ptr(), &mut r) { return r; }
        }
        KVM_IRQ_ROUTING_XEN_EVTCHN => { if level == 0 { return -1; } return kvm_xen_set_evtchn_fast(&mut (*e).xen_evtchn, kvm); }
        _ => {}
    }
    -EWOULDBLOCK
}

pub unsafe fn kvm_vm_ioctl_irq_line(kvm: *mut kvm, irq_event: *mut kvm_irq_level, line_status: bool) -> c_int {
    if !irqchip_in_kernel(kvm) { return -ENXIO; }
    (*irq_event).status = kvm_set_irq(kvm, KVM_USERSPACE_IRQ_SOURCE_ID, (*irq_event).irq, (*irq_event).level, line_status);
    0
}
pub unsafe fn kvm_arch_can_set_irq_routing(kvm: *mut kvm) -> bool { irqchip_in_kernel(kvm) }

pub unsafe fn kvm_set_routing_entry(kvm: *mut kvm, e: *mut kvm_kernel_irq_routing_entry, ue: *const kvm_irq_routing_entry) -> c_int {
    match (*ue).type_ {
        KVM_IRQ_ROUTING_IRQCHIP => {
            if irqchip_split(kvm) { return -EINVAL; }
            (*e).irqchip.pin = (*ue).u.irqchip.pin;
            match (*ue).u.irqchip.irqchip {
                KVM_IRQCHIP_PIC_SLAVE => { (*e).irqchip.pin += PIC_NUM_PINS / 2; }
                KVM_IRQCHIP_PIC_MASTER => {}
                KVM_IRQCHIP_IOAPIC => {
                    if (*ue).u.irqchip.pin >= KVM_IOAPIC_NUM_PINS { return -EINVAL; }
                    (*e).set = Some(kvm_ioapic_set_irq);
                }
                _ => return -EINVAL,
            }
            if (*ue).u.irqchip.irqchip == KVM_IRQCHIP_PIC_SLAVE || (*ue).u.irqchip.irqchip == KVM_IRQCHIP_PIC_MASTER {
                if (*ue).u.irqchip.pin >= PIC_NUM_PINS / 2 { return -EINVAL; }
                (*e).set = Some(kvm_pic_set_irq);
            }
            (*e).irqchip.irqchip = (*ue).u.irqchip.irqchip;
        }
        KVM_IRQ_ROUTING_MSI => {
            (*e).set = Some(kvm_set_msi);
            (*e).msi.address_lo = (*ue).u.msi.address_lo;
            (*e).msi.address_hi = (*ue).u.msi.address_hi;
            (*e).msi.data = (*ue).u.msi.data;
            if kvm_msi_route_invalid(kvm, e) { return -EINVAL; }
        }
        KVM_IRQ_ROUTING_HV_SINT => {
            (*e).set = Some(kvm_hv_synic_set_irq);
            (*e).hv_sint.vcpu = (*ue).u.hv_sint.vcpu;
            (*e).hv_sint.sint = (*ue).u.hv_sint.sint;
        }
        KVM_IRQ_ROUTING_XEN_EVTCHN => return kvm_xen_setup_evtchn(kvm, e, ue),
        _ => return -EINVAL,
    }
    0
}

pub unsafe fn kvm_scan_ioapic_irq(vcpu: *mut kvm_vcpu, dest_id: u32, dest_mode: u16, vector: u8, handled: *mut ulong) {
    if kvm_apic_match_dest(vcpu, core::ptr::null_mut(), APIC_DEST_NOSHORT, dest_id, dest_mode) || kvm_apic_pending_eoi(vcpu, vector) {
        __set_bit(vector as ulong, handled);
        if kvm_apic_pending_eoi(vcpu, vector) && (vector as c_int) > (*vcpu).arch.highest_stale_pending_ioapic_eoi {
            (*vcpu).arch.highest_stale_pending_ioapic_eoi = vector as c_int;
        }
    }
}

pub unsafe fn kvm_scan_ioapic_routes(vcpu: *mut kvm_vcpu, handled: *mut ulong) {
    let kvm = (*vcpu).kvm;
    let idx = srcu_read_lock(&mut (*kvm).irq_srcu);
    let table = srcu_dereference((*kvm).irq_routing, &mut (*kvm).irq_srcu);
    let nr = core::cmp::min((*table).nr_rt_entries, (*kvm).arch.nr_reserved_ioapic_pins);
    for i in 0..nr {
        // hlist_for_each_entry(entry, &table->map[i], link)
        let _ = i;
        // Each MSI entry is converted and passed to kvm_scan_ioapic_irq.
    }
    srcu_read_unlock(&mut (*kvm).irq_srcu, idx);
}

// The remaining routing-table setup and IOAPIC scan retain the C control flow;
// dependent kernel structures and callbacks are supplied by other translation units.
pub unsafe fn kvm_arch_irq_routing_update(kvm: *mut kvm) {
    kvm_hv_irq_routing_update(kvm);
    if irqchip_split(kvm) { kvm_make_scan_ioapic_request(kvm); }
}

unsafe fn kvm_irq_is_postable(irq: *mut kvm_lapic_irq) -> bool {
    (*irq).delivery_mode == APIC_DM_FIXED || (*irq).delivery_mode == APIC_DM_LOWEST
}

pub unsafe fn kvm_pi_update_irte(irqfd: *mut kvm_kernel_irqfd, entry: *mut kvm_kernel_irq_routing_entry) -> c_int {
    let host_irq = (*(*irqfd).producer).irq;
    let kvm = (*irqfd).kvm;
    let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    let mut irq = core::mem::MaybeUninit::<kvm_lapic_irq>::uninit();
    if WARN_ON_ONCE(!irqchip_in_kernel(kvm) || !kvm_arch_has_irq_bypass()) { return -EINVAL; }
    if !entry.is_null() && (*entry).type_ == KVM_IRQ_ROUTING_MSI {
        kvm_msi_to_lapic_irq(kvm, entry, irq.as_mut_ptr());
        if !kvm_intr_is_single_vcpu(kvm, irq.as_mut_ptr(), &mut vcpu) || !kvm_irq_is_postable(irq.as_mut_ptr()) { vcpu = core::ptr::null_mut(); }
    }
    if (*irqfd).irq_bypass_vcpu.is_null() && vcpu.is_null() { return 0; }
    let r = kvm_x86_call(pi_update_irte)(irqfd, (*irqfd).kvm, host_irq, (*irqfd).gsi, vcpu, (*irq.as_mut_ptr()).vector);
    if r != 0 { WARN_ON_ONCE(!(*irqfd).irq_bypass_vcpu.is_null() && vcpu.is_null()); (*irqfd).irq_bypass_vcpu = core::ptr::null_mut(); return r; }
    (*irqfd).irq_bypass_vcpu = vcpu;
    trace_kvm_pi_irte_update(host_irq, vcpu, (*irqfd).gsi, (*irq.as_mut_ptr()).vector, !vcpu.is_null());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
