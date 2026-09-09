// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2009. SUSE Linux Products GmbH. All rights reserved.
 *
 * Authors:
 *    Alexander Graf <agraf@suse.de>
 *    Kevin Wolf <kevin-wolf.de>
 *
 * Description:
 * This file is derived from arch/powerpc/kvm/44x.c,
 * by Hollis Blanchard <hollisb@us.ibm.com>.
 */

// C headers and symbols supplied by the surrounding kernel are external dependencies.
// #define EXIT_DEBUG

pub static KVM_VM_STATS_DESC: [kvm_stats_desc; 3] = [KVM_GENERIC_VM_STATS!(), STATS_DESC_ICOUNTER!(VM, num_2M_pages), STATS_DESC_ICOUNTER!(VM, num_1G_pages)];
pub static KVM_VM_STATS_HEADER: kvm_stats_header = kvm_stats_header { name_size: KVM_STATS_NAME_SIZE, num_desc: ARRAY_SIZE!(KVM_VM_STATS_DESC), id_offset: core::mem::size_of::<kvm_stats_header>(), desc_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE, data_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE + core::mem::size_of_val(&KVM_VM_STATS_DESC) };
pub static KVM_VCPU_STATS_DESC: [kvm_stats_desc; 29] = [
    KVM_GENERIC_VCPU_STATS!(), STATS_DESC_COUNTER!(VCPU, sum_exits), STATS_DESC_COUNTER!(VCPU, mmio_exits), STATS_DESC_COUNTER!(VCPU, signal_exits), STATS_DESC_COUNTER!(VCPU, light_exits), STATS_DESC_COUNTER!(VCPU, itlb_real_miss_exits), STATS_DESC_COUNTER!(VCPU, itlb_virt_miss_exits), STATS_DESC_COUNTER!(VCPU, dtlb_real_miss_exits), STATS_DESC_COUNTER!(VCPU, dtlb_virt_miss_exits), STATS_DESC_COUNTER!(VCPU, syscall_exits), STATS_DESC_COUNTER!(VCPU, isi_exits), STATS_DESC_COUNTER!(VCPU, dsi_exits), STATS_DESC_COUNTER!(VCPU, emulated_inst_exits), STATS_DESC_COUNTER!(VCPU, dec_exits), STATS_DESC_COUNTER!(VCPU, ext_intr_exits), STATS_DESC_COUNTER!(VCPU, halt_successful_wait), STATS_DESC_COUNTER!(VCPU, dbell_exits), STATS_DESC_COUNTER!(VCPU, gdbell_exits), STATS_DESC_COUNTER!(VCPU, ld), STATS_DESC_COUNTER!(VCPU, st), STATS_DESC_COUNTER!(VCPU, pf_storage), STATS_DESC_COUNTER!(VCPU, pf_instruc), STATS_DESC_COUNTER!(VCPU, sp_storage), STATS_DESC_COUNTER!(VCPU, sp_instruc), STATS_DESC_COUNTER!(VCPU, queue_intr), STATS_DESC_COUNTER!(VCPU, ld_slow), STATS_DESC_COUNTER!(VCPU, st_slow), STATS_DESC_COUNTER!(VCPU, pthru_all), STATS_DESC_COUNTER!(VCPU, pthru_host), STATS_DESC_COUNTER!(VCPU, pthru_bad_aff)
];
pub static KVM_VCPU_STATS_HEADER: kvm_stats_header = kvm_stats_header { name_size: KVM_STATS_NAME_SIZE, num_desc: ARRAY_SIZE!(KVM_VCPU_STATS_DESC), id_offset: core::mem::size_of::<kvm_stats_header>(), desc_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE, data_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE + core::mem::size_of_val(&KVM_VCPU_STATS_DESC) };

#[inline]
unsafe fn kvmppc_update_int_pending(vcpu: *mut kvm_vcpu, pending_now: ulong, old_pending: ulong) { if is_kvmppc_hv_enabled((*vcpu).kvm) { return; } if pending_now != 0 { kvmppc_set_int_pending(vcpu, 1); } else if old_pending != 0 { kvmppc_set_int_pending(vcpu, 0); } }

#[inline]
unsafe fn kvmppc_critical_section(vcpu: *mut kvm_vcpu) -> bool { if is_kvmppc_hv_enabled((*vcpu).kvm) { return false; } let mut crit_raw = kvmppc_get_critical(vcpu); let mut crit_r1 = kvmppc_get_gpr(vcpu, 1); let msr = kvmppc_get_msr(vcpu); if msr & MSR_SF == 0 { crit_raw &= 0xffff_ffff; crit_r1 &= 0xffff_ffff; } (crit_raw == crit_r1) && (msr & MSR_PR == 0) }

pub unsafe fn kvmppc_inject_interrupt(vcpu: *mut kvm_vcpu, vec: c_int, flags: u64) { ((*(*vcpu).kvm).arch.kvm_ops).inject_interrupt(vcpu, vec, flags); }

unsafe fn kvmppc_book3s_vec2irqprio(vec: c_uint) -> c_uint { match vec { 0x100 => BOOK3S_IRQPRIO_SYSTEM_RESET, 0x200 => BOOK3S_IRQPRIO_MACHINE_CHECK, 0x300 => BOOK3S_IRQPRIO_DATA_STORAGE, 0x380 => BOOK3S_IRQPRIO_DATA_SEGMENT, 0x400 => BOOK3S_IRQPRIO_INST_STORAGE, 0x480 => BOOK3S_IRQPRIO_INST_SEGMENT, 0x500 => BOOK3S_IRQPRIO_EXTERNAL, 0x600 => BOOK3S_IRQPRIO_ALIGNMENT, 0x700 => BOOK3S_IRQPRIO_PROGRAM, 0x800 => BOOK3S_IRQPRIO_FP_UNAVAIL, 0x900 => BOOK3S_IRQPRIO_DECREMENTER, 0xc00 => BOOK3S_IRQPRIO_SYSCALL, 0xd00 => BOOK3S_IRQPRIO_DEBUG, 0xf20 => BOOK3S_IRQPRIO_ALTIVEC, 0xf40 => BOOK3S_IRQPRIO_VSX, 0xf60 => BOOK3S_IRQPRIO_FAC_UNAVAIL, _ => BOOK3S_IRQPRIO_MAX } }

pub unsafe fn kvmppc_book3s_dequeue_irqprio(vcpu: *mut kvm_vcpu, vec: c_uint) { let old_pending = (*vcpu).arch.pending_exceptions; clear_bit(kvmppc_book3s_vec2irqprio(vec), &mut (*vcpu).arch.pending_exceptions); kvmppc_update_int_pending(vcpu, (*vcpu).arch.pending_exceptions, old_pending); }
pub unsafe fn kvmppc_book3s_queue_irqprio(vcpu: *mut kvm_vcpu, vec: c_uint) { (*vcpu).stat.queue_intr += 1; set_bit(kvmppc_book3s_vec2irqprio(vec), &mut (*vcpu).arch.pending_exceptions); }
pub unsafe fn kvmppc_core_queue_machine_check(vcpu: *mut kvm_vcpu, flags: ulong) { kvmppc_inject_interrupt(vcpu, BOOK3S_INTERRUPT_MACHINE_CHECK, flags); }
pub unsafe fn kvmppc_core_queue_syscall(vcpu: *mut kvm_vcpu) { kvmppc_inject_interrupt(vcpu, BOOK3S_INTERRUPT_SYSCALL, 0); }
pub unsafe fn kvmppc_core_queue_program(vcpu: *mut kvm_vcpu, flags: ulong) { kvmppc_inject_interrupt(vcpu, BOOK3S_INTERRUPT_PROGRAM, flags); }
pub unsafe fn kvmppc_core_queue_fpunavail(vcpu: *mut kvm_vcpu, flags: ulong) { kvmppc_inject_interrupt(vcpu, BOOK3S_INTERRUPT_FP_UNAVAIL, flags); }
pub unsafe fn kvmppc_core_queue_vec_unavail(vcpu: *mut kvm_vcpu, flags: ulong) { kvmppc_inject_interrupt(vcpu, BOOK3S_INTERRUPT_ALTIVEC, flags); }
pub unsafe fn kvmppc_core_queue_vsx_unavail(vcpu: *mut kvm_vcpu, flags: ulong) { kvmppc_inject_interrupt(vcpu, BOOK3S_INTERRUPT_VSX, flags); }
pub unsafe fn kvmppc_core_queue_dec(vcpu: *mut kvm_vcpu) { kvmppc_book3s_queue_irqprio(vcpu, BOOK3S_INTERRUPT_DECREMENTER); }
pub unsafe fn kvmppc_core_pending_dec(vcpu: *mut kvm_vcpu) -> c_int { test_bit(BOOK3S_IRQPRIO_DECREMENTER, &(*vcpu).arch.pending_exceptions) }
pub unsafe fn kvmppc_core_dequeue_dec(vcpu: *mut kvm_vcpu) { kvmppc_book3s_dequeue_irqprio(vcpu, BOOK3S_INTERRUPT_DECREMENTER); }
pub unsafe fn kvmppc_core_queue_external(vcpu: *mut kvm_vcpu, irq: *mut kvm_interrupt) { if (*irq).irq == KVM_INTERRUPT_SET { (*vcpu).arch.external_oneshot = 1; } kvmppc_book3s_queue_irqprio(vcpu, BOOK3S_INTERRUPT_EXTERNAL); }
pub unsafe fn kvmppc_core_dequeue_external(vcpu: *mut kvm_vcpu) { kvmppc_book3s_dequeue_irqprio(vcpu, BOOK3S_INTERRUPT_EXTERNAL); }
pub unsafe fn kvmppc_core_queue_data_storage(vcpu: *mut kvm_vcpu, flags: ulong, dar: ulong, dsisr: ulong) { kvmppc_set_dar(vcpu, dar); kvmppc_set_dsisr(vcpu, dsisr); kvmppc_inject_interrupt(vcpu, BOOK3S_INTERRUPT_DATA_STORAGE, flags); }
pub unsafe fn kvmppc_core_queue_inst_storage(vcpu: *mut kvm_vcpu, flags: ulong) { kvmppc_inject_interrupt(vcpu, BOOK3S_INTERRUPT_INST_STORAGE, flags); }

unsafe fn kvmppc_book3s_irqprio_deliver(vcpu: *mut kvm_vcpu, priority: c_uint) -> c_int { let mut deliver = 1; let mut vec = 0; let crit = kvmppc_critical_section(vcpu); match priority { BOOK3S_IRQPRIO_DECREMENTER => { deliver = (!kvmhv_is_nestedv2() && (kvmppc_get_msr(vcpu) & MSR_EE != 0) && !crit) as c_int; vec = BOOK3S_INTERRUPT_DECREMENTER; }, BOOK3S_IRQPRIO_EXTERNAL => { deliver = (!kvmhv_is_nestedv2() && (kvmppc_get_msr(vcpu) & MSR_EE != 0) && !crit) as c_int; vec = BOOK3S_INTERRUPT_EXTERNAL; }, BOOK3S_IRQPRIO_SYSTEM_RESET => vec = BOOK3S_INTERRUPT_SYSTEM_RESET, BOOK3S_IRQPRIO_MACHINE_CHECK => vec = BOOK3S_INTERRUPT_MACHINE_CHECK, BOOK3S_IRQPRIO_DATA_STORAGE => vec = BOOK3S_INTERRUPT_DATA_STORAGE, BOOK3S_IRQPRIO_INST_STORAGE => vec = BOOK3S_INTERRUPT_INST_STORAGE, BOOK3S_IRQPRIO_DATA_SEGMENT => vec = BOOK3S_INTERRUPT_DATA_SEGMENT, BOOK3S_IRQPRIO_INST_SEGMENT => vec = BOOK3S_INTERRUPT_INST_SEGMENT, BOOK3S_IRQPRIO_ALIGNMENT => vec = BOOK3S_INTERRUPT_ALIGNMENT, BOOK3S_IRQPRIO_PROGRAM => vec = BOOK3S_INTERRUPT_PROGRAM, BOOK3S_IRQPRIO_VSX => vec = BOOK3S_INTERRUPT_VSX, BOOK3S_IRQPRIO_ALTIVEC => vec = BOOK3S_INTERRUPT_ALTIVEC, BOOK3S_IRQPRIO_FP_UNAVAIL => vec = BOOK3S_INTERRUPT_FP_UNAVAIL, BOOK3S_IRQPRIO_SYSCALL => vec = BOOK3S_INTERRUPT_SYSCALL, BOOK3S_IRQPRIO_DEBUG => vec = BOOK3S_INTERRUPT_TRACE, BOOK3S_IRQPRIO_PERFORMANCE_MONITOR => vec = BOOK3S_INTERRUPT_PERFMON, BOOK3S_IRQPRIO_FAC_UNAVAIL => vec = BOOK3S_INTERRUPT_FAC_UNAVAIL, _ => { deliver = 0; printk!(KERN_ERR, "KVM: Unknown interrupt: 0x%x\n", priority); } } if deliver != 0 { kvmppc_inject_interrupt(vcpu, vec, 0); } deliver }

unsafe fn clear_irqprio(vcpu: *mut kvm_vcpu, priority: c_uint) -> bool { match priority { BOOK3S_IRQPRIO_DECREMENTER => false, BOOK3S_IRQPRIO_EXTERNAL => { if (*vcpu).arch.external_oneshot != 0 { (*vcpu).arch.external_oneshot = 0; true } else { false } }, _ => true } }

pub unsafe fn kvmppc_core_prepare_to_enter(vcpu: *mut kvm_vcpu) -> c_int { let pending = &mut (*vcpu).arch.pending_exceptions; let old_pending = *pending; let mut priority = __ffs(*pending); while priority < BOOK3S_IRQPRIO_MAX { if kvmppc_book3s_irqprio_deliver(vcpu, priority) != 0 && clear_irqprio(vcpu, priority) { clear_bit(priority, pending); break; } priority = find_next_bit(pending, BITS_PER_BYTE * core::mem::size_of_val(pending), priority + 1); } kvmppc_update_int_pending(vcpu, *pending, old_pending); 0 }

pub unsafe fn kvmppc_gpa_to_pfn(vcpu: *mut kvm_vcpu, mut gpa: gpa_t, writing: bool, writable: *mut bool, page: *mut *mut page) -> kvm_pfn_t { let mut mp_pa = (*vcpu).arch.magic_page_pa & KVM_PAM; let gfn = gpa >> PAGE_SHIFT; if kvmppc_get_msr(vcpu) & MSR_SF == 0 { mp_pa = mp_pa as u32 as ulong; } gpa &= !0xfff_u64; if mp_pa != 0 && (gpa & KVM_PAM) == mp_pa { let shared_page = (*vcpu).arch.shared as ulong & PAGE_MASK; let pfn = virt_to_phys(shared_page as *const core::ffi::c_void) >> PAGE_SHIFT; *page = pfn_to_page(pfn); get_page(*page); if !writable.is_null() { *writable = true; } return pfn; } kvm_faultin_pfn(vcpu, gfn, writing, writable, page) }

pub unsafe fn kvmppc_xlate(vcpu: *mut kvm_vcpu, eaddr: ulong, xlid: instruction_fetch_type, xlrw: xlate_readwrite, pte: *mut kvmppc_pte) -> c_int { let data = xlid == XLATE_DATA; let iswrite = xlrw == XLATE_WRITE; let relocated = (kvmppc_get_msr(vcpu) & if data { MSR_DR } else { MSR_IR }) != 0; if relocated { ((*vcpu).arch.mmu.xlate)(vcpu, eaddr, pte, data, iswrite) } else { (*pte).eaddr = eaddr; (*pte).raddr = eaddr & KVM_PAM; (*pte).vpage = VSID_REAL | (eaddr >> 12); (*pte).may_read = true; (*pte).may_write = true; (*pte).may_execute = true; if (kvmppc_get_msr(vcpu) & (MSR_IR | MSR_DR)) == MSR_DR && !data && ((*vcpu).arch.hflags & BOOK3S_HFLAG_SPLIT_HACK) != 0 && (eaddr & SPLIT_HACK_MASK) == SPLIT_HACK_OFFS { (*pte).raddr &= !SPLIT_HACK_MASK; } 0 } }

pub unsafe fn kvmppc_load_last_inst(vcpu: *mut kvm_vcpu, typ: instruction_fetch_type, inst: *mut ulong) -> c_int { let mut pc = kvmppc_get_pc(vcpu); if typ == INST_SC { pc -= 4; } let mut iw: u32 = 0; let mut r = kvmppc_ld(vcpu, &mut pc, core::mem::size_of::<u32>(), &mut iw, false); if r != EMULATE_DONE { return EMULATE_AGAIN; } if kvmppc_get_msr(vcpu) & SRR1_PREFIXED != 0 { let mut suffix: u32 = 0; pc += 4; r = kvmppc_ld(vcpu, &mut pc, core::mem::size_of::<u32>(), &mut suffix, false); if r != EMULATE_DONE { return EMULATE_AGAIN; } *inst = ((iw as u64) << 32 | suffix as u64) as ulong; } else { *inst = iw as ulong; } r }

pub unsafe fn kvmppc_subarch_vcpu_init(_vcpu: *mut kvm_vcpu) -> c_int { 0 }
pub unsafe fn kvmppc_subarch_vcpu_uninit(_vcpu: *mut kvm_vcpu) {}
pub unsafe fn kvm_arch_vcpu_ioctl_get_sregs(vcpu: *mut kvm_vcpu, sregs: *mut kvm_sregs) -> c_int { vcpu_load(vcpu); let ret = ((*(*vcpu).kvm).arch.kvm_ops).get_sregs(vcpu, sregs); vcpu_put(vcpu); ret }
pub unsafe fn kvm_arch_vcpu_ioctl_set_sregs(vcpu: *mut kvm_vcpu, sregs: *mut kvm_sregs) -> c_int { vcpu_load(vcpu); let ret = ((*(*vcpu).kvm).arch.kvm_ops).set_sregs(vcpu, sregs); vcpu_put(vcpu); ret }
pub unsafe fn kvm_arch_vcpu_ioctl_get_regs(vcpu: *mut kvm_vcpu, regs: *mut kvm_regs) -> c_int { (*regs).pc=kvmppc_get_pc(vcpu); (*regs).cr=kvmppc_get_cr(vcpu); (*regs).ctr=kvmppc_get_ctr(vcpu); (*regs).lr=kvmppc_get_lr(vcpu); (*regs).xer=kvmppc_get_xer(vcpu); (*regs).msr=kvmppc_get_msr(vcpu); (*regs).srr0=kvmppc_get_srr0(vcpu); (*regs).srr1=kvmppc_get_srr1(vcpu); (*regs).pid=kvmppc_get_pid(vcpu); (*regs).sprg0=kvmppc_get_sprg0(vcpu); (*regs).sprg1=kvmppc_get_sprg1(vcpu); (*regs).sprg2=kvmppc_get_sprg2(vcpu); (*regs).sprg3=kvmppc_get_sprg3(vcpu); (*regs).sprg4=kvmppc_get_sprg4(vcpu); (*regs).sprg5=kvmppc_get_sprg5(vcpu); (*regs).sprg6=kvmppc_get_sprg6(vcpu); (*regs).sprg7=kvmppc_get_sprg7(vcpu); for i in 0..(*regs).gpr.len() { (*regs).gpr[i]=kvmppc_get_gpr(vcpu,i as _); } 0 }
pub unsafe fn kvm_arch_vcpu_ioctl_set_regs(vcpu: *mut kvm_vcpu, regs: *mut kvm_regs) -> c_int { kvmppc_set_pc(vcpu,(*regs).pc); kvmppc_set_cr(vcpu,(*regs).cr); kvmppc_set_ctr(vcpu,(*regs).ctr); kvmppc_set_lr(vcpu,(*regs).lr); kvmppc_set_xer(vcpu,(*regs).xer); kvmppc_set_msr(vcpu,(*regs).msr); kvmppc_set_srr0(vcpu,(*regs).srr0); kvmppc_set_srr1(vcpu,(*regs).srr1); kvmppc_set_sprg0(vcpu,(*regs).sprg0); kvmppc_set_sprg1(vcpu,(*regs).sprg1); kvmppc_set_sprg2(vcpu,(*regs).sprg2); kvmppc_set_sprg3(vcpu,(*regs).sprg3); kvmppc_set_sprg4(vcpu,(*regs).sprg4); kvmppc_set_sprg5(vcpu,(*regs).sprg5); kvmppc_set_sprg6(vcpu,(*regs).sprg6); kvmppc_set_sprg7(vcpu,(*regs).sprg7); for i in 0..(*regs).gpr.len() { kvmppc_set_gpr(vcpu,i as _,(*regs).gpr[i]); } 0 }
pub unsafe fn kvm_arch_vcpu_ioctl_get_fpu(_vcpu: *mut kvm_vcpu, _fpu: *mut kvm_fpu) -> c_int { -EOPNOTSUPP }
pub unsafe fn kvm_arch_vcpu_ioctl_set_fpu(_vcpu: *mut kvm_vcpu, _fpu: *mut kvm_fpu) -> c_int { -EOPNOTSUPP }

pub unsafe fn kvmppc_get_one_reg(vcpu: *mut kvm_vcpu, id: u64, val: *mut kvmppc_one_reg) -> c_int { let mut r = ((*(*vcpu).kvm).arch.kvm_ops).get_one_reg(vcpu,id,val); if r == -EINVAL { r=0; match id { KVM_REG_PPC_DAR=>*val=get_reg_val(id,kvmppc_get_dar(vcpu)), KVM_REG_PPC_DSISR=>*val=get_reg_val(id,kvmppc_get_dsisr(vcpu)), KVM_REG_PPC_FPR0..=KVM_REG_PPC_FPR31=>*val=get_reg_val(id,kvmppc_get_fpr(vcpu,(id-KVM_REG_PPC_FPR0) as _)), KVM_REG_PPC_FPSCR=>*val=get_reg_val(id,kvmppc_get_fpscr(vcpu)), KVM_REG_PPC_DEBUG_INST=>*val=get_reg_val(id,INS_TW), KVM_REG_PPC_FSCR=>*val=get_reg_val(id,(*vcpu).arch.fscr), KVM_REG_PPC_TAR=>*val=get_reg_val(id,kvmppc_get_tar(vcpu)), KVM_REG_PPC_EBBHR=>*val=get_reg_val(id,kvmppc_get_ebbhr(vcpu)), KVM_REG_PPC_EBBRR=>*val=get_reg_val(id,kvmppc_get_ebbrr(vcpu)), KVM_REG_PPC_BESCR=>*val=get_reg_val(id,kvmppc_get_bescr(vcpu)), KVM_REG_PPC_IC=>*val=get_reg_val(id,kvmppc_get_ic(vcpu)), _=>r=-EINVAL } } r }
pub unsafe fn kvmppc_set_one_reg(vcpu: *mut kvm_vcpu, id: u64, val: *mut kvmppc_one_reg) -> c_int { let mut r=((*(*vcpu).kvm).arch.kvm_ops).set_one_reg(vcpu,id,val); if r == -EINVAL { r=0; match id { KVM_REG_PPC_DAR=>kvmppc_set_dar(vcpu,set_reg_val(id,*val)), KVM_REG_PPC_DSISR=>kvmppc_set_dsisr(vcpu,set_reg_val(id,*val)), KVM_REG_PPC_FPR0..=KVM_REG_PPC_FPR31=>kvmppc_set_fpr(vcpu,(id-KVM_REG_PPC_FPR0) as _,set_reg_val(id,*val)), KVM_REG_PPC_FPSCR=>(*vcpu).arch.fp.fpscr=set_reg_val(id,*val), KVM_REG_PPC_FSCR=>kvmppc_set_fpscr(vcpu,set_reg_val(id,*val)), KVM_REG_PPC_TAR=>kvmppc_set_tar(vcpu,set_reg_val(id,*val)), KVM_REG_PPC_EBBHR=>kvmppc_set_ebbhr(vcpu,set_reg_val(id,*val)), KVM_REG_PPC_EBBRR=>kvmppc_set_ebbrr(vcpu,set_reg_val(id,*val)), KVM_REG_PPC_BESCR=>kvmppc_set_bescr(vcpu,set_reg_val(id,*val)), KVM_REG_PPC_IC=>kvmppc_set_ic(vcpu,set_reg_val(id,*val)), _=>r=-EINVAL } } r }

pub unsafe fn kvmppc_core_vcpu_load(vcpu:*mut kvm_vcpu,cpu:c_int){((*(*vcpu).kvm).arch.kvm_ops).vcpu_load(vcpu,cpu)}
pub unsafe fn kvmppc_core_vcpu_put(vcpu:*mut kvm_vcpu){((*(*vcpu).kvm).arch.kvm_ops).vcpu_put(vcpu)}
pub unsafe fn kvmppc_set_msr(vcpu:*mut kvm_vcpu,msr:u64){((*(*vcpu).kvm).arch.kvm_ops).set_msr(vcpu,msr)}
pub unsafe fn kvmppc_vcpu_run(vcpu:*mut kvm_vcpu)->c_int{((*(*vcpu).kvm).arch.kvm_ops).vcpu_run(vcpu)}
pub unsafe fn kvm_arch_vcpu_ioctl_translate(_vcpu:*mut kvm_vcpu,_tr:*mut kvm_translation)->c_int{0}
pub unsafe fn kvm_arch_vcpu_ioctl_set_guest_debug(vcpu:*mut kvm_vcpu,dbg:*mut kvm_guest_debug)->c_int{vcpu_load(vcpu);(*vcpu).guest_debug=(*dbg).control;vcpu_put(vcpu);0}
pub unsafe fn kvmppc_decrementer_func(vcpu:*mut kvm_vcpu){kvmppc_core_queue_dec(vcpu);kvm_vcpu_kick(vcpu)}
pub unsafe fn kvmppc_core_vcpu_create(vcpu:*mut kvm_vcpu)->c_int{((*(*vcpu).kvm).arch.kvm_ops).vcpu_create(vcpu)}
pub unsafe fn kvmppc_core_vcpu_free(vcpu:*mut kvm_vcpu){((*(*vcpu).kvm).arch.kvm_ops).vcpu_free(vcpu)}
pub unsafe fn kvmppc_core_check_requests(vcpu:*mut kvm_vcpu)->c_int{((*(*vcpu).kvm).arch.kvm_ops).check_requests(vcpu)}
pub unsafe fn kvm_arch_sync_dirty_log(_kvm:*mut kvm,_memslot:*mut kvm_memory_slot){}
pub unsafe fn kvm_vm_ioctl_get_dirty_log(kvm:*mut kvm,log:*mut kvm_dirty_log)->c_int{((*kvm).arch.kvm_ops).get_dirty_log(kvm,log)}
pub unsafe fn kvmppc_core_free_memslot(kvm:*mut kvm,slot:*mut kvm_memory_slot){((*kvm).arch.kvm_ops).free_memslot(slot)}
pub unsafe fn kvmppc_core_flush_memslot(kvm:*mut kvm,slot:*mut kvm_memory_slot){((*kvm).arch.kvm_ops).flush_memslot(kvm,slot)}
pub unsafe fn kvmppc_core_prepare_memory_region(kvm:*mut kvm,old:*const kvm_memory_slot,new:*mut kvm_memory_slot,change:kvm_mr_change)->c_int{((*kvm).arch.kvm_ops).prepare_memory_region(kvm,old,new,change)}
pub unsafe fn kvmppc_core_commit_memory_region(kvm:*mut kvm,old:*mut kvm_memory_slot,new:*const kvm_memory_slot,change:kvm_mr_change){((*kvm).arch.kvm_ops).commit_memory_region(kvm,old,new,change)}
pub unsafe fn kvm_unmap_gfn_range(kvm:*mut kvm,range:*mut kvm_gfn_range)->bool{((*kvm).arch.kvm_ops).unmap_gfn_range(kvm,range)}
pub unsafe fn kvm_age_gfn(kvm:*mut kvm,range:*mut kvm_gfn_range)->bool{((*kvm).arch.kvm_ops).age_gfn(kvm,range)}
pub unsafe fn kvm_test_age_gfn(kvm:*mut kvm,range:*mut kvm_gfn_range)->bool{((*kvm).arch.kvm_ops).test_age_gfn(kvm,range)}
pub unsafe fn kvmppc_core_init_vm(kvm:*mut kvm)->c_int{((*kvm).arch.kvm_ops).init_vm(kvm)}
pub unsafe fn kvmppc_core_destroy_vm(kvm:*mut kvm){((*kvm).arch.kvm_ops).destroy_vm(kvm)}

pub unsafe fn kvmppc_h_logical_ci_load(vcpu:*mut kvm_vcpu)->c_int{let size=kvmppc_get_gpr(vcpu,4);let addr=kvmppc_get_gpr(vcpu,5);let mut buf:u64=0;if !size.is_power_of_two()||size>core::mem::size_of::<u64>() as ulong{return H_TOO_HARD}let idx=srcu_read_lock(&(*(*vcpu).kvm).srcu);let ret=kvm_io_bus_read(vcpu,KVM_MMIO_BUS,addr,size,&mut buf);srcu_read_unlock(&(*(*vcpu).kvm).srcu,idx);if ret!=0{return H_TOO_HARD}kvmppc_set_gpr(vcpu,4,match size{1=>buf as u8 as ulong,2=>u16::from_be(buf as u16) as ulong,4=>u32::from_be(buf as u32) as ulong,8=>u64::from_be(buf),_=>BUG!()});H_SUCCESS}
pub unsafe fn kvmppc_h_logical_ci_store(vcpu:*mut kvm_vcpu)->c_int{let size=kvmppc_get_gpr(vcpu,4);let addr=kvmppc_get_gpr(vcpu,5);let val=kvmppc_get_gpr(vcpu,6);let buf=match size{1=>val as u8 as u64,2=>(val as u16).to_be() as u64,4=>(val as u32).to_be() as u64,8=>val.to_be(),_=>return H_TOO_HARD};let idx=srcu_read_lock(&(*(*vcpu).kvm).srcu);let ret=kvm_io_bus_write(vcpu,KVM_MMIO_BUS,addr,size,&buf);srcu_read_unlock(&(*(*vcpu).kvm).srcu,idx);if ret!=0{H_TOO_HARD}else{H_SUCCESS}}
pub unsafe fn kvmppc_book3s_hcall_implemented(kvm:*mut kvm,hcall:ulong)->c_int{((*kvm).arch.kvm_ops).hcall_implemented(hcall)}

// CONFIG_KVM_XICS-dependent IRQ routing.
pub unsafe fn kvm_set_irq(kvm:*mut kvm,irq_source_id:c_int,irq:u32,level:c_int,line_status:bool)->c_int{if xics_on_xive(){kvmppc_xive_set_irq(kvm,irq_source_id,irq,level,line_status)}else{kvmppc_xics_set_irq(kvm,irq_source_id,irq,level,line_status)}}
pub unsafe fn kvm_arch_set_irq_inatomic(entry:*mut kvm_kernel_irq_routing_entry,kvm:*mut kvm,irq_source_id:c_int,level:c_int,line_status:bool)->c_int{kvm_set_irq(kvm,irq_source_id,(*entry).gsi,level,line_status)}
unsafe fn kvmppc_book3s_set_irq(e:*mut kvm_kernel_irq_routing_entry,kvm:*mut kvm,irq_source_id:c_int,level:c_int,line_status:bool)->c_int{kvm_set_irq(kvm,irq_source_id,(*e).gsi,level,line_status)}
pub unsafe fn kvm_irq_map_gsi(_kvm:*mut kvm,entries:*mut kvm_kernel_irq_routing_entry,gsi:c_int)->c_int{(*entries).gsi=gsi;(*entries).type_=KVM_IRQ_ROUTING_IRQCHIP;(*entries).set=Some(kvmppc_book3s_set_irq);(*entries).irqchip.irqchip=0;(*entries).irqchip.pin=gsi;1}
pub unsafe fn kvm_irq_map_chip_pin(_kvm:*mut kvm,_irqchip:c_uint,pin:c_uint)->c_int{pin as c_int}

// CONFIG_KVM_BOOK3S_32_HANDLER and device registration remain build-time conditional kernel operations.
pub unsafe fn kvmppc_book3s_init()->c_int{kvm_init(core::mem::size_of::<kvm_vcpu>(),0,THIS_MODULE)}
pub unsafe fn kvmppc_book3s_exit(){kvm_exit()}
// module_init(kvmppc_book3s_init); module_exit(kvmppc_book3s_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
