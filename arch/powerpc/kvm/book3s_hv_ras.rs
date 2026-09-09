// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright 2012 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
 */

// External kernel declarations and architecture-specific types are supplied by
// the surrounding translation unit.

const SRR1_MC_LDSTERR: u64 = 1u64 << (63 - 42);
const SRR1_MC_IFETCH_SH: u32 = 63 - 45;
const SRR1_MC_IFETCH_MASK: u64 = 0x7;
const SRR1_MC_IFETCH_SLBPAR: u64 = 2;
const SRR1_MC_IFETCH_SLBMULTI: u64 = 3;
const SRR1_MC_IFETCH_SLBPARMULTI: u64 = 4;
const SRR1_MC_IFETCH_TLBMULTI: u64 = 5;

const DSISR_MC_DERAT_MULTI: u64 = 0x800;
const DSISR_MC_TLB_MULTI: u64 = 0x400;
const DSISR_MC_SLB_PARITY: u64 = 0x100;
const DSISR_MC_SLB_MULTI: u64 = 0x080;
const DSISR_MC_SLB_PARMULTI: u64 = 0x040;

unsafe fn reload_slb(vcpu: *mut kvm_vcpu) {
    let slb: *mut slb_shadow;
    let mut n: u32;

    core::arch::asm!("slbmte {0},{0}; slbia", in(reg) 0usize);

    slb = (*vcpu).arch.slb_shadow.pinned_addr;
    if slb.is_null() {
        return;
    }

    n = core::cmp::min(be32_to_cpu((*slb).persistent), SLB_MIN_SIZE);
    if ((*slb).save_area.as_ptr().add(n as usize) as *mut u8)
        > (*vcpu).arch.slb_shadow.pinned_end as *mut u8
    {
        return;
    }

    for i in 0..n as usize {
        let mut rb = be64_to_cpu((*slb).save_area[i].esid);
        let rs = be64_to_cpu((*slb).save_area[i].vsid);

        rb = (rb & !0xfffu64) | i as u64;
        core::arch::asm!("slbmte {0},{1}", in(reg) rs, in(reg) rb);
    }
}

unsafe fn kvmppc_realmode_mc_power7(vcpu: *mut kvm_vcpu) -> libc::c_long {
    let srr1 = (*vcpu).arch.shregs.msr;
    let mut handled: libc::c_long = 1;

    if srr1 & SRR1_MC_LDSTERR != 0 {
        let mut dsisr = (*vcpu).arch.shregs.dsisr;

        if dsisr & (DSISR_MC_SLB_PARMULTI | DSISR_MC_SLB_MULTI |
                    DSISR_MC_SLB_PARITY | DSISR_MC_DERAT_MULTI) != 0 {
            reload_slb(vcpu);
            dsisr &= !(DSISR_MC_SLB_PARMULTI | DSISR_MC_SLB_MULTI |
                       DSISR_MC_SLB_PARITY | DSISR_MC_DERAT_MULTI);
        }
        if dsisr & DSISR_MC_TLB_MULTI != 0 {
            tlbiel_all_lpid((*vcpu).kvm.arch.radix);
            dsisr &= !DSISR_MC_TLB_MULTI;
        }
        if dsisr & 0xffff_ffff != 0 {
            handled = 0;
        }
    }

    match (srr1 >> SRR1_MC_IFETCH_SH) & SRR1_MC_IFETCH_MASK {
        0 => {}
        SRR1_MC_IFETCH_SLBPAR | SRR1_MC_IFETCH_SLBMULTI |
        SRR1_MC_IFETCH_SLBPARMULTI => reload_slb(vcpu),
        SRR1_MC_IFETCH_TLBMULTI => tlbiel_all_lpid((*vcpu).kvm.arch.radix),
        _ => handled = 0,
    }
    handled
}

pub unsafe fn kvmppc_realmode_machine_check(vcpu: *mut kvm_vcpu) {
    let mut mce_evt: machine_check_event = core::mem::zeroed();
    let handled: libc::c_long;

    if (*vcpu).kvm.arch.fwnmi_enabled {
        handled = 0;
    } else {
        handled = kvmppc_realmode_mc_power7(vcpu);
    }

    if get_mce_event(&mut mce_evt, MCE_EVENT_RELEASE) != 0 {
        if handled != 0 && mce_evt.version == MCE_V1 {
            mce_evt.disposition = MCE_DISPOSITION_RECOVERED;
        }
    } else {
        core::ptr::write_bytes(&mut mce_evt, 0, 1);
    }
    (*vcpu).arch.mce_evt = mce_evt;
}

pub unsafe fn kvmppc_p9_realmode_hmi_handler(vcpu: *mut kvm_vcpu) -> libc::c_long {
    let vc = (*vcpu).arch.vcore;
    let mut ret: libc::c_long = 0;

    if (*vc).tb_offset_applied != 0 {
        let mut new_tb = mftb() - (*vc).tb_offset_applied;
        mtspr(SPRN_TBU40, new_tb);
        if (mftb() & 0xffffff) < (new_tb & 0xffffff) {
            new_tb += 0x1000000;
            mtspr(SPRN_TBU40, new_tb);
        }
        (*vc).tb_offset_applied = 0;
    }
    (*local_paca).hmi_irqs += 1;
    if hmi_handle_debugtrig(core::ptr::null_mut()) >= 0 {
        ret = 1;
    } else if ppc_md.hmi_exception_early.is_some() {
        ppc_md.hmi_exception_early(core::ptr::null_mut());
    }
    if kvmppc_get_tb_offset(vcpu) != 0 {
        let mut new_tb = mftb() + (*vc).tb_offset;
        mtspr(SPRN_TBU40, new_tb);
        if (mftb() & 0xffffff) < (new_tb & 0xffffff) {
            new_tb += 0x1000000;
            mtspr(SPRN_TBU40, new_tb);
        }
        (*vc).tb_offset_applied = kvmppc_get_tb_offset(vcpu);
    }
    ret
}

#[inline]
unsafe fn kvmppc_cur_subcore_size() -> libc::c_int {
    if !(*local_paca).kvm_hstate.kvm_split_mode.is_null() {
        (*local_paca).kvm_hstate.kvm_split_mode.subcore_size
    } else {
        threads_per_subcore
    }
}

pub unsafe fn kvmppc_subcore_enter_guest() {
    let thread_id = cpu_thread_in_core((*local_paca).paca_index);
    let subcore_id = thread_id / kvmppc_cur_subcore_size();
    (*local_paca).sibling_subcore_state.in_guest[subcore_id as usize] = 1;
}

pub unsafe fn kvmppc_subcore_exit_guest() {
    let thread_id = cpu_thread_in_core((*local_paca).paca_index);
    let subcore_id = thread_id / kvmppc_cur_subcore_size();
    (*local_paca).sibling_subcore_state.in_guest[subcore_id as usize] = 0;
}

unsafe fn kvmppc_tb_resync_required() -> bool {
    !test_and_set_bit(CORE_TB_RESYNC_REQ_BIT,
        &mut (*local_paca).sibling_subcore_state.flags)
}

unsafe fn kvmppc_tb_resync_done() {
    clear_bit(CORE_TB_RESYNC_REQ_BIT,
        &mut (*local_paca).sibling_subcore_state.flags);
}

pub unsafe fn kvmppc_realmode_hmi_handler() -> libc::c_long {
    (*local_paca).hmi_irqs += 1;
    if hmi_handle_debugtrig(core::ptr::null_mut()) >= 0 {
        return 1;
    }

    let resync_req = kvmppc_tb_resync_required();
    kvmppc_subcore_exit_guest();
    wait_for_subcore_guest_exit();
    if ppc_md.hmi_exception_early.is_some() {
        ppc_md.hmi_exception_early(core::ptr::null_mut());
    }
    if resync_req {
        opal_resync_timebase();
        kvmppc_tb_resync_done();
    } else {
        wait_for_tb_resync();
    }
    if !(*local_paca).kvm_hstate.kvm_vcore.is_null() {
        (*local_paca).kvm_hstate.kvm_vcore.tb_offset_applied = 0;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
