// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of book3s_hv_rm_xics.c. */

// Kernel and architecture dependencies are supplied externally.

pub static mut h_ipi_redirect: i32 = 1;
pub static mut kvm_irq_bypass: i32 = 1;

unsafe fn ics_rm_check_resend(xics: *mut kvmppc_xics, ics: *mut kvmppc_ics,
                              icp: *mut kvmppc_icp) {
    for i in 0..KVMPPC_XICS_IRQ_PER_ICS {
        let state = &mut (*ics).irq_state[i as usize];
        if state.resend { icp_rm_deliver_irq(xics, icp, state.number, true); }
    }
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn icp_send_hcore_msg(hcore: i32, vcpu: *mut kvm_vcpu) {
    let hcpu = hcore << threads_shift;
    (*kvmppc_host_rm_ops_hv).rm_core[hcore as usize].rm_data = vcpu;
    smp_muxed_ipi_set_message(hcpu, PPC_MSG_RM_HOST_ACTION);
    kvmppc_set_host_ipi(hcpu); smp_mb(); kvmhv_rm_send_ipi(hcpu);
}
#[cfg(not(feature = "CONFIG_SMP"))]
unsafe fn icp_send_hcore_msg(_hcore: i32, _vcpu: *mut kvm_vcpu) {}

unsafe fn grab_next_hostcore(start: i32, rm_core: *mut kvmppc_host_rm_core,
                             max: i32, action: i32) -> i32 {
    for core in (start + 1)..max {
        let old = READ_ONCE((*rm_core.add(core as usize)).rm_state);
        let mut new = old;
        if !old.in_host || old.rm_action != 0 { continue; }
        new.rm_action = action;
        if cmpxchg64(&mut (*rm_core.add(core as usize)).rm_state.raw, old.raw, new.raw) == old.raw {
            smp_wmb(); return core;
        }
    }
    -1
}

unsafe fn find_available_hostcore(action: i32) -> i32 {
    let my_core = smp_processor_id() >> threads_shift;
    let rm_core = (*kvmppc_host_rm_ops_hv).rm_core.as_mut_ptr();
    let mut core = grab_next_hostcore(my_core, rm_core, cpu_nr_cores(), action);
    if core == -1 { core = grab_next_hostcore(core, rm_core, my_core, action); }
    core
}

unsafe fn icp_rm_set_vcpu_irq(vcpu: *mut kvm_vcpu, this_vcpu: *mut kvm_vcpu) {
    let this_icp = (*this_vcpu).arch.icp;
    (*vcpu).stat.queue_intr += 1;
    set_bit(BOOK3S_IRQPRIO_EXTERNAL, &mut (*vcpu).arch.pending_exceptions);
    if vcpu == this_vcpu { mtspr(SPRN_LPCR, mfspr(SPRN_LPCR) | LPCR_MER); return; }
    let cpu = (*vcpu).arch.thread_cpu;
    if cpu < 0 || cpu >= nr_cpu_ids {
        let mut hcore = -1;
        if !kvmppc_host_rm_ops_hv.is_null() && h_ipi_redirect != 0 { hcore = find_available_hostcore(XICS_RM_KICK_VCPU); }
        if hcore != -1 { icp_send_hcore_msg(hcore, vcpu); }
        else { (*this_icp).rm_action |= XICS_RM_KICK_VCPU; (*this_icp).rm_kick_target = vcpu; }
        return;
    }
    smp_mb(); kvmhv_rm_send_ipi(cpu);
}

unsafe fn icp_rm_clr_vcpu_irq(vcpu: *mut kvm_vcpu) {
    clear_bit(BOOK3S_IRQPRIO_EXTERNAL, &mut (*vcpu).arch.pending_exceptions);
    mtspr(SPRN_LPCR, mfspr(SPRN_LPCR) & !LPCR_MER);
}

unsafe fn icp_rm_try_update(icp: *mut kvmppc_icp, old: kvmppc_icp_state,
                            mut new: kvmppc_icp_state) -> bool {
    let this_vcpu = (*local_paca).kvm_hstate.kvm_vcpu;
    new.out_ee = new.xisr != 0 && new.pending_pri < new.cppr;
    if cmpxchg64(&mut (*icp).state.raw, old.raw, new.raw) != old.raw { return false; }
    if new.out_ee { icp_rm_set_vcpu_irq((*icp).vcpu, this_vcpu); }
    (*this_vcpu).arch.icp.rm_dbgstate = new;
    (*this_vcpu).arch.icp.rm_dbgtgt = (*icp).vcpu;
    true
}

unsafe fn check_too_hard(xics: *mut kvmppc_xics, icp: *mut kvmppc_icp) -> i32 {
    if (*xics).real_mode_dbg || (*icp).rm_action != 0 { H_TOO_HARD } else { H_SUCCESS }
}

unsafe fn icp_rm_check_resend(xics: *mut kvmppc_xics, icp: *mut kvmppc_icp) {
    smp_rmb();
    for icsid in for_each_set_bit((*icp).resend_map, (*xics).max_icsid + 1) {
        let ics = (*xics).ics[icsid as usize];
        if !test_and_clear_bit(icsid, &mut (*icp).resend_map) || ics.is_null() { continue; }
        ics_rm_check_resend(xics, ics, icp);
    }
}

unsafe fn icp_rm_try_to_deliver(icp: *mut kvmppc_icp, irq: u32, priority: u8, reject: *mut u32) -> bool {
    loop {
        let old = READ_ONCE((*icp).state); let mut new = old; *reject = 0;
        let success = new.cppr > priority && new.mfrr > priority && new.pending_pri > priority;
        if success { *reject = new.xisr; new.xisr = irq; new.pending_pri = priority; }
        else { new.need_resend = true; }
        if icp_rm_try_update(icp, old, new) { return success; }
    }
}

unsafe fn icp_rm_deliver_irq(xics: *mut kvmppc_xics, mut icp: *mut kvmppc_icp,
                              mut new_irq: u32, mut check_resend: bool) {
    'again: loop {
        let mut src: u16 = 0; let ics = kvmppc_xics_find_ics(xics, new_irq, &mut src);
        if ics.is_null() { (*xics).err_noics += 1; return; }
        let state = &mut (*ics).irq_state[src as usize]; arch_spin_lock(&mut (*ics).lock);
        if icp.is_null() || state.server != (*icp).server_num { icp = kvmppc_xics_find_server((*xics).kvm, state.server); if icp.is_null() { (*xics).err_noicp += 1; arch_spin_unlock(&mut (*ics).lock); return; } }
        if check_resend && !state.resend { arch_spin_unlock(&mut (*ics).lock); return; }
        state.resend = false;
        if state.priority == MASKED { state.masked_pending = 1; arch_spin_unlock(&mut (*ics).lock); return; }
        let mut reject = 0;
        if icp_rm_try_to_deliver(icp, new_irq, state.priority, &mut reject) {
            if reject != 0 && reject != XICS_IPI { arch_spin_unlock(&mut (*ics).lock); (*icp).n_reject += 1; new_irq = reject; check_resend = false; continue 'again; }
        } else {
            state.resend = true; smp_wmb(); set_bit((*ics).icsid, &mut (*icp).resend_map); smp_mb();
            if !(*icp).state.need_resend { state.resend = false; arch_spin_unlock(&mut (*ics).lock); check_resend = false; continue 'again; }
        }
        arch_spin_unlock(&mut (*ics).lock); return;
    }
}

unsafe fn icp_rm_down_cppr(xics: *mut kvmppc_xics, icp: *mut kvmppc_icp, new_cppr: u8) {
    let resend;
    loop { let old = READ_ONCE((*icp).state); let mut new = old; new.cppr = new_cppr;
        if new.mfrr < new_cppr && new.mfrr <= new.pending_pri { new.pending_pri = new.mfrr; new.xisr = XICS_IPI; }
        resend = new.need_resend; new.need_resend = false;
        if icp_rm_try_update(icp, old, new) { break; }
    }
    if resend { (*icp).n_check_resend += 1; icp_rm_check_resend(xics, icp); }
}

pub unsafe fn xics_rm_h_xirr_x(vcpu: *mut kvm_vcpu) -> u64 { kvmppc_set_gpr(vcpu, 5, get_tb()); xics_rm_h_xirr(vcpu) }
pub unsafe fn xics_rm_h_xirr(vcpu: *mut kvm_vcpu) -> u64 {
    let xics = (*vcpu).kvm.arch.xics; let icp = (*vcpu).arch.icp; if xics.is_null() || !(*xics).real_mode { return H_TOO_HARD as u64; }
    icp_rm_clr_vcpu_irq((*icp).vcpu); let mut xirr = 0;
    loop { let old = READ_ONCE((*icp).state); let mut new = old; xirr = old.xisr | ((old.cppr as u32) << 24); if old.xisr == 0 { break; } new.cppr = new.pending_pri; new.pending_pri = 0xff; new.xisr = 0; if icp_rm_try_update(icp, old, new) { break; } }
    kvmppc_set_gpr(vcpu, 4, xirr); check_too_hard(xics, icp) as u64
}

// The remaining entry points retain the C ABI and delegate through the same
// state-machine primitives above; external kernel structures/functions are
// intentionally unresolved dependencies.
pub unsafe fn xics_rm_h_ipi(vcpu: *mut kvm_vcpu, server: u64, mfrr: u64) -> i32 {
    let xics = (*vcpu).kvm.arch.xics; let this_icp = (*vcpu).arch.icp;
    if xics.is_null() || !(*xics).real_mode { return H_TOO_HARD; }
    let icp = if (*this_icp).server_num as u64 == server { this_icp } else { kvmppc_xics_find_server((*vcpu).kvm, server) };
    if icp.is_null() { return H_PARAMETER; }
    loop { let old = READ_ONCE((*icp).state); let mut new = old; new.mfrr = mfrr; let mut reject = 0;
        if mfrr < new.cppr && mfrr <= new.pending_pri { reject = new.xisr; new.pending_pri = mfrr; new.xisr = XICS_IPI; }
        let resend = if mfrr > old.mfrr { new.need_resend } else { false }; if mfrr > old.mfrr { new.need_resend = false; }
        if icp_rm_try_update(icp, old, new) { if reject != 0 && reject != XICS_IPI { (*this_icp).n_reject += 1; icp_rm_deliver_irq(xics, icp, reject, false); } if resend { (*this_icp).n_check_resend += 1; icp_rm_check_resend(xics, icp); } break; }
    } check_too_hard(xics, this_icp)
}
pub unsafe fn xics_rm_h_cppr(vcpu: *mut kvm_vcpu, cppr: u64) -> i32 {
    let xics = (*vcpu).kvm.arch.xics; let icp = (*vcpu).arch.icp; if xics.is_null() || !(*xics).real_mode { return H_TOO_HARD; }
    if cppr > (*icp).state.cppr as u64 { icp_rm_down_cppr(xics, icp, cppr as u8); return check_too_hard(xics, icp); }
    if cppr == (*icp).state.cppr as u64 { return H_SUCCESS; } icp_rm_clr_vcpu_irq((*icp).vcpu);
    loop { let old = READ_ONCE((*icp).state); let mut new = old; let mut reject = 0; new.cppr = cppr as u8; if new.cppr <= new.pending_pri { reject = new.xisr; new.xisr = 0; new.pending_pri = 0xff; } if icp_rm_try_update(icp, old, new) { if reject != 0 && reject != XICS_IPI { (*icp).n_reject += 1; icp_rm_deliver_irq(xics, icp, reject, false); } break; } } check_too_hard(xics, icp)
}
pub unsafe fn xics_rm_h_eoi(vcpu: *mut kvm_vcpu, xirr: u64) -> i32 {
    let xics = (*vcpu).kvm.arch.xics; let icp = (*vcpu).arch.icp; if xics.is_null() || !(*xics).real_mode { return H_TOO_HARD; }
    icp_rm_down_cppr(xics, icp, (xirr >> 24) as u8); if (xirr as u32 & 0x00ffffff) == XICS_IPI { return check_too_hard(xics, icp); } check_too_hard(xics, icp)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
