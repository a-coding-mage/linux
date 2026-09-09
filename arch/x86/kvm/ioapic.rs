// SPDX-License-Identifier: LGPL-2.1-or-later
/* Direct Rust translation of the Linux KVM IOAPIC implementation. */

unsafe fn ioapic_read_indirect(ioapic: *mut kvm_ioapic) -> c_ulong {
    let mut result: c_ulong = 0;
    match (*ioapic).ioregsel {
        IOAPIC_REG_VERSION => result = (((IOAPIC_NUM_PINS - 1) & 0xff) << 16) | (IOAPIC_VERSION_ID & 0xff),
        IOAPIC_REG_APIC_ID | IOAPIC_REG_ARB_ID => result = ((*ioapic).id & 0xf) << 24,
        _ => {
            let redir_index = ((*ioapic).ioregsel - 0x10) >> 1;
            let mut redir_content: u64 = !0u64;
            if redir_index < IOAPIC_NUM_PINS {
                let index = array_index_nospec(redir_index, IOAPIC_NUM_PINS);
                redir_content = (*ioapic).redirtbl[index as usize].bits;
            }
            result = if ((*ioapic).ioregsel & 1) != 0 { (redir_content >> 32) & 0xffff_ffff } else { redir_content & 0xffff_ffff };
        }
    }
    result
}

unsafe fn rtc_irq_eoi_tracking_reset(ioapic: *mut kvm_ioapic) {
    (*ioapic).rtc_status.pending_eoi = 0;
    bitmap_zero((*ioapic).rtc_status.map.as_mut_ptr(), KVM_MAX_VCPU_IDS);
}

unsafe fn kvm_rtc_eoi_tracking_restore_all(ioapic: *mut kvm_ioapic);

unsafe fn rtc_status_pending_eoi_check_valid(ioapic: *mut kvm_ioapic) {
    if WARN_ON_ONCE((*ioapic).rtc_status.pending_eoi < 0) { kvm_rtc_eoi_tracking_restore_all(ioapic); }
}

unsafe fn __rtc_irq_eoi_tracking_restore_one(vcpu: *mut kvm_vcpu) {
    let ioapic = (*(*vcpu).kvm).arch.vioapic;
    let status = &mut (*ioapic).rtc_status;
    let e = &mut (*ioapic).redirtbl[RTC_GSI as usize];
    if !kvm_apic_match_dest(vcpu, core::ptr::null_mut(), APIC_DEST_NOSHORT, e.fields.dest_id, kvm_lapic_irq_dest_mode(e.fields.dest_mode != 0)) { return; }
    let new_val = kvm_apic_pending_eoi(vcpu, e.fields.vector) != 0;
    let old_val = test_bit((*vcpu).vcpu_id, status.map.as_ptr()) != 0;
    if new_val == old_val { return; }
    if new_val {
        __set_bit((*vcpu).vcpu_id, status.map.as_mut_ptr());
        status.vectors[(*vcpu).vcpu_id as usize] = e.fields.vector;
        (*ioapic).rtc_status.pending_eoi += 1;
    } else {
        __clear_bit((*vcpu).vcpu_id, status.map.as_mut_ptr());
        (*ioapic).rtc_status.pending_eoi -= 1;
        rtc_status_pending_eoi_check_valid(ioapic);
    }
}

#[no_mangle] pub unsafe extern "C" fn kvm_rtc_eoi_tracking_restore_one(vcpu: *mut kvm_vcpu) {
    let ioapic = (*(*vcpu).kvm).arch.vioapic;
    spin_lock(&mut (*ioapic).lock); __rtc_irq_eoi_tracking_restore_one(vcpu); spin_unlock(&mut (*ioapic).lock);
}

unsafe fn kvm_rtc_eoi_tracking_restore_all(ioapic: *mut kvm_ioapic) {
    if RTC_GSI >= IOAPIC_NUM_PINS { return; }
    rtc_irq_eoi_tracking_reset(ioapic);
    let mut i = 0; let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    kvm_for_each_vcpu(i, vcpu, (*ioapic).kvm) { __rtc_irq_eoi_tracking_restore_one(vcpu); }
}

unsafe fn rtc_irq_eoi(ioapic: *mut kvm_ioapic, vcpu: *mut kvm_vcpu, vector: i32) {
    let status = &mut (*ioapic).rtc_status;
    let id = (*vcpu).vcpu_id;
    if test_bit(id, status.map.as_ptr()) != 0 && vector == status.vectors[id as usize] && test_and_clear_bit(id, status.map.as_mut_ptr()) != 0 {
        (*ioapic).rtc_status.pending_eoi -= 1; rtc_status_pending_eoi_check_valid(ioapic);
    }
}
unsafe fn rtc_irq_check_coalesced(ioapic: *mut kvm_ioapic) -> bool { (*ioapic).rtc_status.pending_eoi > 0 }

unsafe fn ioapic_lazy_update_eoi(ioapic: *mut kvm_ioapic, irq: i32) {
    let entry = &mut (*ioapic).redirtbl[irq as usize];
    let mut i = 0; let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    kvm_for_each_vcpu(i, vcpu, (*ioapic).kvm) {
        if !kvm_apic_match_dest(vcpu, core::ptr::null_mut(), APIC_DEST_NOSHORT, entry.fields.dest_id, entry.fields.dest_mode != 0) || kvm_apic_pending_eoi(vcpu, entry.fields.vector) != 0 { continue; }
        rtc_irq_eoi(ioapic, vcpu, entry.fields.vector); break;
    }
}

unsafe fn ioapic_set_irq(ioapic: *mut kvm_ioapic, irq: u32, irq_level: i32, line_status: bool) -> i32 {
    let entry = (*ioapic).redirtbl[irq as usize]; let mask = 1u32 << irq; let edge = entry.fields.trig_mode == IOAPIC_EDGE_TRIG;
    if irq_level == 0 { (*ioapic).irr &= !mask; return 1; }
    if edge && kvm_apicv_activated((*ioapic).kvm) { ioapic_lazy_update_eoi(ioapic, irq as i32); }
    if irq == RTC_GSI && line_status && rtc_irq_check_coalesced(ioapic) { return 0; }
    let old_irr = (*ioapic).irr; (*ioapic).irr |= mask;
    if edge { (*ioapic).irr_delivered &= !mask; if old_irr == (*ioapic).irr { return 0; } }
    let ret = ioapic_service(ioapic, irq as i32, line_status); trace_kvm_ioapic_set_irq(entry.bits, irq, ret == 0); ret
}

unsafe fn kvm_ioapic_inject_all(ioapic: *mut kvm_ioapic, irr: c_ulong) {
    rtc_irq_eoi_tracking_reset(ioapic); let mut idx = 0; for_each_set_bit(idx, &irr, IOAPIC_NUM_PINS) { ioapic_set_irq(ioapic, idx, 1, true); } kvm_rtc_eoi_tracking_restore_all(ioapic);
}

#[no_mangle] pub unsafe extern "C" fn kvm_ioapic_scan_entry(vcpu: *mut kvm_vcpu, handled: *mut c_ulong) {
    let ioapic = (*(*vcpu).kvm).arch.vioapic; let status = &mut (*ioapic).rtc_status; spin_lock(&mut (*ioapic).lock);
    if test_bit((*vcpu).vcpu_id, status.map.as_ptr()) != 0 { __set_bit(status.vectors[(*vcpu).vcpu_id as usize], handled); }
    for index in 0..IOAPIC_NUM_PINS { let e = &(*ioapic).redirtbl[index as usize]; if e.fields.trig_mode == IOAPIC_LEVEL_TRIG || kvm_irq_has_notifier((*ioapic).kvm, KVM_IRQCHIP_IOAPIC, index) || index == RTC_GSI { kvm_scan_ioapic_irq(vcpu, e.fields.dest_id, kvm_lapic_irq_dest_mode(e.fields.dest_mode != 0), e.fields.vector, handled); } }
    spin_unlock(&mut (*ioapic).lock);
}

unsafe fn ioapic_service(ioapic: *mut kvm_ioapic, irq: i32, _line_status: bool) -> i32 {
    let entry = &mut (*ioapic).redirtbl[irq as usize];
    if entry.fields.mask || (entry.fields.trig_mode == IOAPIC_LEVEL_TRIG && entry.fields.remote_irr) { return -1; }
    let mut irqe: kvm_lapic_irq = core::mem::zeroed();
    irqe.dest_id = entry.fields.dest_id; irqe.vector = entry.fields.vector;
    irqe.dest_mode = kvm_lapic_irq_dest_mode(entry.fields.dest_mode != 0);
    irqe.trig_mode = entry.fields.trig_mode; irqe.delivery_mode = entry.fields.delivery_mode << 8;
    irqe.level = 1; irqe.shorthand = APIC_DEST_NOSHORT; irqe.msi_redir_hint = false;
    if irqe.trig_mode == IOAPIC_EDGE_TRIG { (*ioapic).irr_delivered |= 1u32 << irq; }
    let ret = kvm_irq_delivery_to_apic((*ioapic).kvm, core::ptr::null_mut(), &mut irqe);
    if ret != 0 && irqe.trig_mode == IOAPIC_LEVEL_TRIG { entry.fields.remote_irr = 1; } ret
}

#[no_mangle] pub unsafe extern "C" fn kvm_ioapic_set_irq(e: *mut kvm_kernel_irq_routing_entry, kvm: *mut kvm, source: i32, level: i32, line_status: bool) -> i32 {
    let ioapic = (*kvm).arch.vioapic; let irq = (*e).irqchip.pin;
    if WARN_ON_ONCE(irq < 0 || irq >= IOAPIC_NUM_PINS as i32) { return -1; }
    spin_lock(&mut (*ioapic).lock); let irq_level = __kvm_irq_line_state(&mut (*ioapic).irq_states[irq as usize], source, level);
    let ret = ioapic_set_irq(ioapic, irq as u32, irq_level, line_status); spin_unlock(&mut (*ioapic).lock); ret
}

#[no_mangle] pub unsafe extern "C" fn kvm_ioapic_update_eoi(vcpu: *mut kvm_vcpu, vector: i32, trigger_mode: i32) {
    let ioapic = (*(*vcpu).kvm).arch.vioapic; spin_lock(&mut (*ioapic).lock); rtc_irq_eoi(ioapic, vcpu, vector);
    for i in 0..IOAPIC_NUM_PINS { let ent = &mut (*ioapic).redirtbl[i as usize]; if ent.fields.vector == vector { ent.fields.remote_irr = 0; if !ent.fields.mask && ((*ioapic).irr & (1u32 << i)) != 0 { ioapic_service(ioapic, i, false); } } }
    spin_unlock(&mut (*ioapic).lock);
}

#[no_mangle] pub unsafe extern "C" fn kvm_ioapic_init(kvm: *mut kvm) -> i32 {
    let ioapic = kzalloc_obj::<kvm_ioapic>(GFP_KERNEL_ACCOUNT); if ioapic.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*ioapic).lock); INIT_HLIST_HEAD(&mut (*ioapic).mask_notifier_list); (*kvm).arch.vioapic = ioapic;
    (*ioapic).base_address = IOAPIC_DEFAULT_BASE_ADDRESS; (*ioapic).kvm = kvm; let ret = kvm_io_bus_register_dev(kvm, KVM_MMIO_BUS, (*ioapic).base_address, IOAPIC_MEM_LENGTH, &mut (*ioapic).dev);
    if ret < 0 { (*kvm).arch.vioapic = core::ptr::null_mut(); kfree(ioapic); } ret
}

#[no_mangle] pub unsafe extern "C" fn kvm_ioapic_destroy(kvm: *mut kvm) { let ioapic = (*kvm).arch.vioapic; if ioapic.is_null() { return; } mutex_lock(&mut (*kvm).slots_lock); kvm_io_bus_unregister_dev(kvm, KVM_MMIO_BUS, &mut (*ioapic).dev); mutex_unlock(&mut (*kvm).slots_lock); (*kvm).arch.vioapic = core::ptr::null_mut(); kfree(ioapic); }

#[no_mangle] pub unsafe extern "C" fn kvm_get_ioapic(kvm: *mut kvm, state: *mut kvm_ioapic_state) { let ioapic = (*kvm).arch.vioapic; spin_lock(&mut (*ioapic).lock); memcpy(state, ioapic, core::mem::size_of::<kvm_ioapic_state>()); (*state).irr &= !(*ioapic).irr_delivered; spin_unlock(&mut (*ioapic).lock); }

#[no_mangle] pub unsafe extern "C" fn kvm_set_ioapic(kvm: *mut kvm, state: *mut kvm_ioapic_state) { let ioapic = (*kvm).arch.vioapic; spin_lock(&mut (*ioapic).lock); memcpy(ioapic, state, core::mem::size_of::<kvm_ioapic_state>()); (*ioapic).irr = 0; (*ioapic).irr_delivered = 0; kvm_make_scan_ioapic_request(kvm); kvm_ioapic_inject_all(ioapic, (*state).irr as c_ulong); spin_unlock(&mut (*ioapic).lock); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
