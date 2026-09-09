// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 * Copyright (C) 2022 Ventana Micro Systems Inc.
 *
 * Authors:
 *\tAnup Patel <apatel@ventanamicro.com>
 */

#[repr(C)]
struct AiaHgeiControl {
    lock: raw_spinlock_t,
    free_bitmap_initialized: bool,
    free_bitmap: c_ulong,
    owners: [*mut kvm_vcpu; BITS_PER_LONG],
    nr_hgei: c_uint,
    saved_hgeie: c_ulong,
}

static mut AIA_HGEI: PerCpu<AiaHgeiControl> = DEFINE_PER_CPU!();
static mut HGEI_PARENT_IRQ: c_int = 0;

static mut KVM_RISCV_AIA_NR_HGEI: atomic_t = atomic_t::new(0);
static mut KVM_RISCV_AIA_MAX_IDS: c_uint = 0;
static KVM_RISCV_AIA_AVAILABLE: StaticKeyFalse = DEFINE_STATIC_KEY_FALSE!();

#[inline]
unsafe fn aia_hvictl_value(ext_irq_pending: bool) -> c_ulong {
    let mut hvictl = ((IRQ_S_EXT << HVICTL_IID_SHIFT) & HVICTL_IID) as c_ulong;
    hvictl |= ext_irq_pending as c_ulong;
    hvictl
}

#[cfg(CONFIG_32BIT)]
unsafe fn kvm_riscv_vcpu_aia_flush_interrupts(vcpu: *mut kvm_vcpu) {
    let csr = &mut (*vcpu).arch.aia_context.guest_csr;
    lockdep_assert_held(&(*vcpu).arch.irqs_pending_lock);
    if !kvm_riscv_aia_available() { return; }
    let mask = (*vcpu).arch.irqs_pending_mask[1];
    if mask != 0 {
        (*vcpu).arch.irqs_pending_mask[1] = 0;
        let val = (*vcpu).arch.irqs_pending[1] & mask;
        csr.hviph &= !mask;
        csr.hviph |= val;
    }
}

#[cfg(CONFIG_32BIT)]
unsafe fn kvm_riscv_vcpu_aia_sync_interrupts(vcpu: *mut kvm_vcpu) {
    let csr = &mut (*vcpu).arch.aia_context.guest_csr;
    lockdep_assert_held(&(*vcpu).arch.irqs_pending_lock);
    if kvm_riscv_aia_available() { csr.vsieh = ncsr_read(CSR_VSIEH); }
}

unsafe fn kvm_riscv_vcpu_aia_has_interrupts(vcpu: *mut kvm_vcpu, mask: u64) -> bool {
    if !kvm_riscv_aia_available() { return false; }
    #[cfg(CONFIG_32BIT)] {
        let mut flags = 0;
        raw_spin_lock_irqsave(&(*vcpu).arch.irqs_pending_lock, &mut flags);
        let pending = ((*vcpu).arch.irqs_pending[1]
            & ((*vcpu).arch.aia_context.guest_csr.vsieh & upper_32_bits(mask))) != 0;
        raw_spin_unlock_irqrestore(&(*vcpu).arch.irqs_pending_lock, flags);
        if pending { return true; }
    }
    let mut seip = (*vcpu).arch.guest_csr.vsie;
    seip &= mask as c_ulong;
    seip &= BIT(IRQ_S_EXT);
    if !kvm_riscv_aia_initialized((*vcpu).kvm) || seip == 0 { return false; }
    kvm_riscv_vcpu_aia_imsic_has_interrupt(vcpu)
}

unsafe fn kvm_riscv_vcpu_aia_update_hvip(vcpu: *mut kvm_vcpu) {
    let csr = &(*vcpu).arch.guest_csr;
    if !kvm_riscv_aia_available() { return; }
    #[cfg(CONFIG_32BIT)] ncsr_write(CSR_HVIPH, (*vcpu).arch.aia_context.guest_csr.hviph);
    ncsr_write(CSR_HVICTL, aia_hvictl_value((csr.hvip & BIT(IRQ_VS_EXT)) != 0));
}

unsafe fn kvm_riscv_vcpu_aia_load(vcpu: *mut kvm_vcpu, cpu: c_int) {
    let csr = &(*vcpu).arch.aia_context.guest_csr;
    if !kvm_riscv_aia_available() { return; }
    if kvm_riscv_nacl_sync_csr_available() {
        let nsh = nacl_shmem();
        nacl_csr_write(nsh, CSR_VSISELECT, csr.vsiselect);
        nacl_csr_write(nsh, CSR_HVIPRIO1, csr.hviprio1);
        nacl_csr_write(nsh, CSR_HVIPRIO2, csr.hviprio2);
        #[cfg(CONFIG_32BIT)] {
            nacl_csr_write(nsh, CSR_VSIEH, csr.vsieh); nacl_csr_write(nsh, CSR_HVIPH, csr.hviph);
            nacl_csr_write(nsh, CSR_HVIPRIO1H, csr.hviprio1h); nacl_csr_write(nsh, CSR_HVIPRIO2H, csr.hviprio2h);
        }
    } else {
        csr_write(CSR_VSISELECT, csr.vsiselect); csr_write(CSR_HVIPRIO1, csr.hviprio1); csr_write(CSR_HVIPRIO2, csr.hviprio2);
        #[cfg(CONFIG_32BIT)] {
            csr_write(CSR_VSIEH, csr.vsieh); csr_write(CSR_HVIPH, csr.hviph);
            csr_write(CSR_HVIPRIO1H, csr.hviprio1h); csr_write(CSR_HVIPRIO2H, csr.hviprio2h);
        }
    }
    if kvm_riscv_aia_initialized((*vcpu).kvm) { kvm_riscv_vcpu_aia_imsic_load(vcpu, cpu); }
}

unsafe fn kvm_riscv_vcpu_aia_put(vcpu: *mut kvm_vcpu) {
    let csr = &mut (*vcpu).arch.aia_context.guest_csr;
    if !kvm_riscv_aia_available() { return; }
    if kvm_riscv_aia_initialized((*vcpu).kvm) { kvm_riscv_vcpu_aia_imsic_put(vcpu); }
    if kvm_riscv_nacl_available() {
        let nsh = nacl_shmem();
        csr.vsiselect = nacl_csr_read(nsh, CSR_VSISELECT); csr.hviprio1 = nacl_csr_read(nsh, CSR_HVIPRIO1); csr.hviprio2 = nacl_csr_read(nsh, CSR_HVIPRIO2);
        #[cfg(CONFIG_32BIT)] {
            csr.vsieh = nacl_csr_read(nsh, CSR_VSIEH); csr.hviph = nacl_csr_read(nsh, CSR_HVIPH);
            csr.hviprio1h = nacl_csr_read(nsh, CSR_HVIPRIO1H); csr.hviprio2h = nacl_csr_read(nsh, CSR_HVIPRIO2H);
        }
    } else {
        csr.vsiselect = csr_read(CSR_VSISELECT); csr.hviprio1 = csr_read(CSR_HVIPRIO1); csr.hviprio2 = csr_read(CSR_HVIPRIO2);
        #[cfg(CONFIG_32BIT)] {
            csr.vsieh = csr_read(CSR_VSIEH); csr.hviph = csr_read(CSR_HVIPH);
            csr.hviprio1h = csr_read(CSR_HVIPRIO1H); csr.hviprio2h = csr_read(CSR_HVIPRIO2H);
        }
    }
}

unsafe fn kvm_riscv_vcpu_aia_get_csr(vcpu: *mut kvm_vcpu, mut reg_num: c_ulong, out_val: *mut c_ulong) -> c_int {
    let csr = &(*vcpu).arch.aia_context.guest_csr;
    let regs_max = size_of::<kvm_vcpu_aia_csr>() / size_of::<c_ulong>();
    if !riscv_isa_extension_available((*vcpu).arch.isa, SSAIA) || reg_num as usize >= regs_max { return -ENOENT; }
    reg_num = array_index_nospec(reg_num, regs_max as c_ulong);
    *out_val = if kvm_riscv_aia_available() { *((csr as *const _ as *const c_ulong).add(reg_num as usize)) } else { 0 };
    0
}

unsafe fn kvm_riscv_vcpu_aia_set_csr(vcpu: *mut kvm_vcpu, mut reg_num: c_ulong, val: c_ulong) -> c_int {
    let csr = &mut (*vcpu).arch.aia_context.guest_csr;
    let regs_max = size_of::<kvm_vcpu_aia_csr>() / size_of::<c_ulong>();
    if !riscv_isa_extension_available((*vcpu).arch.isa, SSAIA) || reg_num as usize >= regs_max { return -ENOENT; }
    reg_num = array_index_nospec(reg_num, regs_max as c_ulong);
    if kvm_riscv_aia_available() {
        *((csr as *mut _ as *mut c_ulong).add(reg_num as usize)) = val;
        #[cfg(CONFIG_32BIT)] if reg_num == KVM_REG_RISCV_CSR_AIA_REG(siph) {
            let mut flags = 0; raw_spin_lock_irqsave(&(*vcpu).arch.irqs_pending_lock, &mut flags);
            (*vcpu).arch.irqs_pending_mask[1] = 0;
            raw_spin_unlock_irqrestore(&(*vcpu).arch.irqs_pending_lock, flags);
        }
    }
    0
}

unsafe fn kvm_riscv_vcpu_aia_rmw_topei(vcpu: *mut kvm_vcpu, _csr_num: c_uint, val: *mut c_ulong, new_val: c_ulong, wr_mask: c_ulong) -> c_int {
    if !kvm_riscv_aia_available() { return KVM_INSN_ILLEGAL_TRAP; }
    if !kvm_riscv_aia_initialized((*vcpu).kvm) { return KVM_INSN_EXIT_TO_USER_SPACE; }
    kvm_riscv_vcpu_aia_imsic_rmw(vcpu, KVM_RISCV_AIA_IMSIC_TOPEI, val, new_val, wr_mask)
}

// External IRQ priority is read-only zero; the default priority order is preferred.
static AIA_IRQ2BITPOS: [c_int; 64] = [
    0,8,-1,-1,16,24,-1,-1,32,-1,-1,-1,-1,40,48,56,
    64,72,80,88,96,104,112,120,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
];

unsafe fn aia_get_iprio8(_vcpu: *mut kvm_vcpu, irq: c_uint) -> u8 {
    let bitpos = AIA_IRQ2BITPOS[irq as usize]; if bitpos < 0 { return 0; }
    let hviprio = match bitpos as usize / BITS_PER_LONG { 0 => ncsr_read(CSR_HVIPRIO1), 1 => { #[cfg(not(CONFIG_32BIT))] { return ((ncsr_read(CSR_HVIPRIO2) >> (bitpos as usize % BITS_PER_LONG)) & TOPI_IPRIO_MASK) as u8; } #[cfg(CONFIG_32BIT)] { ncsr_read(CSR_HVIPRIO1H) } }, 2 => ncsr_read(CSR_HVIPRIO2), 3 => ncsr_read(CSR_HVIPRIO2H), _ => return 0 };
    ((hviprio >> (bitpos as usize % BITS_PER_LONG)) & TOPI_IPRIO_MASK) as u8
}

unsafe fn aia_set_iprio8(_vcpu: *mut kvm_vcpu, irq: c_uint, prio: u8) {
    let bitpos = AIA_IRQ2BITPOS[irq as usize]; if bitpos < 0 { return; }
    let shift = bitpos as usize % BITS_PER_LONG;
    let mut hviprio = match bitpos as usize / BITS_PER_LONG { 0 => ncsr_read(CSR_HVIPRIO1), 1 => { #[cfg(not(CONFIG_32BIT))] { ncsr_read(CSR_HVIPRIO2) } #[cfg(CONFIG_32BIT)] { ncsr_read(CSR_HVIPRIO1H) } }, 2 => ncsr_read(CSR_HVIPRIO2), 3 => ncsr_read(CSR_HVIPRIO2H), _ => return };
    hviprio = (hviprio & !(TOPI_IPRIO_MASK << shift)) | ((prio as c_ulong) << shift);
    match bitpos as usize / BITS_PER_LONG { 0 => ncsr_write(CSR_HVIPRIO1, hviprio), 1 => { #[cfg(not(CONFIG_32BIT))] ncsr_write(CSR_HVIPRIO2, hviprio); #[cfg(CONFIG_32BIT)] ncsr_write(CSR_HVIPRIO1H, hviprio); }, 2 => ncsr_write(CSR_HVIPRIO2, hviprio), 3 => ncsr_write(CSR_HVIPRIO2H, hviprio), _ => {} }
}

unsafe fn aia_rmw_iprio(vcpu: *mut kvm_vcpu, isel: c_uint, val: *mut c_ulong, mut new_val: c_ulong, wr_mask: c_ulong) -> c_int {
    #[cfg(not(CONFIG_32BIT))] if isel & 0x1 != 0 { return KVM_INSN_ILLEGAL_TRAP; }
    let nirqs = 4 * (BITS_PER_LONG / 32); let first_irq = (isel - ISELECT_IPRIO0) * 4; let mut old_val = 0;
    for i in 0..nirqs { old_val |= (aia_get_iprio8(vcpu, first_irq + i) as c_ulong) << (TOPI_IPRIO_BITS * i); }
    if !val.is_null() { *val = old_val; }
    if wr_mask != 0 { new_val = (old_val & !wr_mask) | (new_val & wr_mask); for i in 0..nirqs { aia_set_iprio8(vcpu, first_irq + i, ((new_val >> (TOPI_IPRIO_BITS * i)) & TOPI_IPRIO_MASK) as u8); } }
    KVM_INSN_CONTINUE_NEXT_SEPC
}

unsafe fn kvm_riscv_aia_rmw_ireg(vcpu: *mut kvm_vcpu, _csr_num: c_uint, val: *mut c_ulong, new_val: c_ulong, wr_mask: c_ulong) -> c_int {
    if !kvm_riscv_aia_available() { return KVM_INSN_ILLEGAL_TRAP; }
    let isel = ncsr_read(CSR_VSISELECT) & ISELECT_MASK;
    if isel >= ISELECT_IPRIO0 && isel <= ISELECT_IPRIO15 { return aia_rmw_iprio(vcpu, isel, val, new_val, wr_mask); }
    if isel >= IMSIC_FIRST && isel <= IMSIC_LAST && kvm_riscv_aia_initialized((*vcpu).kvm) { return kvm_riscv_vcpu_aia_imsic_rmw(vcpu, isel, val, new_val, wr_mask); }
    KVM_INSN_EXIT_TO_USER_SPACE
}

// The remaining HGEI lifecycle routines retain the kernel-facing interfaces and operations.
unsafe fn kvm_riscv_aia_alloc_hgei(cpu: c_int, owner: *mut kvm_vcpu, hgei_va: *mut *mut c_void, hgei_pa: *mut phys_addr_t) -> c_int {
    let hgctrl = per_cpu_ptr(&mut AIA_HGEI, cpu); if !kvm_riscv_aia_available() || hgctrl.is_null() { return -ENODEV; }
    let mut flags = 0; raw_spin_lock_irqsave(&(*hgctrl).lock, &mut flags); let mut ret = -ENOENT;
    if (*hgctrl).free_bitmap != 0 { ret = __ffs((*hgctrl).free_bitmap); (*hgctrl).free_bitmap &= !BIT(ret); (*hgctrl).owners[ret as usize] = owner; }
    raw_spin_unlock_irqrestore(&(*hgctrl).lock, flags);
    let gc = imsic_get_global_config(); let lc = if !gc.is_null() { per_cpu_ptr((*gc).local, cpu) } else { core::ptr::null_mut() };
    if !lc.is_null() && ret > 0 { if !hgei_va.is_null() { *hgei_va = (*lc).msi_va.add((ret as usize) * IMSIC_MMIO_PAGE_SZ); } if !hgei_pa.is_null() { *hgei_pa = (*lc).msi_pa + (ret as usize * IMSIC_MMIO_PAGE_SZ) as phys_addr_t; } }
    ret
}

unsafe fn kvm_riscv_aia_free_hgei(cpu: c_int, hgei: c_int) { let hgctrl = per_cpu_ptr(&mut AIA_HGEI, cpu); if !kvm_riscv_aia_available() || hgctrl.is_null() { return; } let mut flags = 0; raw_spin_lock_irqsave(&(*hgctrl).lock, &mut flags); if hgei > 0 && hgei <= (*hgctrl).nr_hgei as c_int && (*hgctrl).free_bitmap & BIT(hgei) == 0 { (*hgctrl).free_bitmap |= BIT(hgei); (*hgctrl).owners[hgei as usize] = core::ptr::null_mut(); } raw_spin_unlock_irqrestore(&(*hgctrl).lock, flags); }

unsafe fn hgei_interrupt(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let hgctrl = get_cpu_ptr(&mut AIA_HGEI);
    let hgei_mask = csr_read(CSR_HGEIP) & csr_read(CSR_HGEIE);
    csr_clear(CSR_HGEIE, hgei_mask);
    let mut flags = 0; raw_spin_lock_irqsave(&(*hgctrl).lock, &mut flags);
    for i in 0..BITS_PER_LONG { if hgei_mask & BIT(i) != 0 && !(*hgctrl).owners[i].is_null() { kvm_vcpu_kick((*hgctrl).owners[i]); } }
    raw_spin_unlock_irqrestore(&(*hgctrl).lock, flags); put_cpu_ptr(&mut AIA_HGEI); IRQ_HANDLED
}

unsafe fn aia_hgei_init() -> c_int {
    for_each_possible_cpu!(cpu, { let hgctrl = per_cpu_ptr(&mut AIA_HGEI, cpu); raw_spin_lock_init(&mut (*hgctrl).lock); (*hgctrl).free_bitmap_initialized = false; (*hgctrl).free_bitmap = 0; });
    let domain = irq_find_matching_fwnode(riscv_get_intc_hwnode(), DOMAIN_BUS_ANY); if domain.is_null() { kvm_err!("unable to find INTC domain\n"); return -ENOENT; }
    HGEI_PARENT_IRQ = irq_create_mapping(domain, IRQ_S_GEXT); if HGEI_PARENT_IRQ == 0 { kvm_err!("unable to map SGEI IRQ\n"); return -ENOMEM; }
    let rc = request_percpu_irq(HGEI_PARENT_IRQ, hgei_interrupt, "riscv-kvm", &mut AIA_HGEI); if rc != 0 { kvm_err!("failed to request SGEI IRQ\n"); return rc; } 0
}
unsafe fn aia_hgei_exit() { free_percpu_irq(HGEI_PARENT_IRQ, &mut AIA_HGEI); }

unsafe fn kvm_riscv_aia_pm_exit() { if !kvm_riscv_aia_available() { return; } let hgctrl = this_cpu_ptr(&mut AIA_HGEI); csr_write(CSR_HGEIE, (*hgctrl).saved_hgeie); csr_write(CSR_HVICTL, aia_hvictl_value(false)); csr_write(CSR_HVIPRIO1, 0); csr_write(CSR_HVIPRIO2, 0); #[cfg(CONFIG_32BIT)] { csr_write(CSR_HVIPH,0); csr_write(CSR_HIDELEGH,0); csr_write(CSR_HVIPRIO1H,0); csr_write(CSR_HVIPRIO2H,0); } csr_set(CSR_HIE, BIT(IRQ_S_GEXT)); if __riscv_isa_extension_available(core::ptr::null_mut(), RISCV_ISA_EXT_SSCOFPMF) { csr_set(CSR_HVIEN, BIT(IRQ_PMU_OVF)); } }
unsafe fn kvm_riscv_aia_pm_enter() { if !kvm_riscv_aia_available() { return; } if __riscv_isa_extension_available(core::ptr::null_mut(), RISCV_ISA_EXT_SSCOFPMF) { csr_clear(CSR_HVIEN, BIT(IRQ_PMU_OVF)); } csr_write(CSR_HVICTL, aia_hvictl_value(false)); let hgctrl = this_cpu_ptr(&mut AIA_HGEI); (*hgctrl).saved_hgeie = csr_read(CSR_HGEIE); }
unsafe fn kvm_riscv_aia_enable() { if !kvm_riscv_aia_available() { return; } let gc = imsic_get_global_config(); let lc = if !gc.is_null() { this_cpu_ptr((*gc).local) } else { core::ptr::null_mut() }; let hgctrl = this_cpu_ptr(&mut AIA_HGEI); csr_write(CSR_HGEIE, !0); (*hgctrl).nr_hgei = fls_long(csr_read(CSR_HGEIE)); csr_write(CSR_HGEIE, 0); if (*hgctrl).nr_hgei != 0 { (*hgctrl).nr_hgei -= 1; } (*hgctrl).nr_hgei = if !lc.is_null() { min((*hgctrl).nr_hgei as ulong, (*lc).nr_guest_files) as c_uint } else { 0 }; let mut flags=0; raw_spin_lock_irqsave(&(*hgctrl).lock,&mut flags); if !(*hgctrl).free_bitmap_initialized { (*hgctrl).free_bitmap = if (*hgctrl).nr_hgei != 0 { GENMASK_ULL((*hgctrl).nr_hgei,1) } else { 0 }; (*hgctrl).free_bitmap_initialized=true; } raw_spin_unlock_irqrestore(&(*hgctrl).lock,flags); csr_write(CSR_HVICTL,aia_hvictl_value(false)); csr_write(CSR_HVIPRIO1,0); csr_write(CSR_HVIPRIO2,0); #[cfg(CONFIG_32BIT)] { csr_write(CSR_HVIPH,0); csr_write(CSR_HIDELEGH,0); csr_write(CSR_HVIPRIO1H,0); csr_write(CSR_HVIPRIO2H,0); } enable_percpu_irq(HGEI_PARENT_IRQ,irq_get_trigger_type(HGEI_PARENT_IRQ)); csr_set(CSR_HIE,BIT(IRQ_S_GEXT)); if __riscv_isa_extension_available(core::ptr::null_mut(),RISCV_ISA_EXT_SSCOFPMF) { csr_set(CSR_HVIEN,BIT(IRQ_PMU_OVF)); } }
unsafe fn kvm_riscv_aia_disable() { if !kvm_riscv_aia_available() { return; } let hgctrl=get_cpu_ptr(&mut AIA_HGEI); if __riscv_isa_extension_available(core::ptr::null_mut(),RISCV_ISA_EXT_SSCOFPMF) { csr_clear(CSR_HVIEN,BIT(IRQ_PMU_OVF)); } csr_clear(CSR_HIE,BIT(IRQ_S_GEXT)); disable_percpu_irq(HGEI_PARENT_IRQ); csr_write(CSR_HVICTL,aia_hvictl_value(false)); let mut flags=0; raw_spin_lock_irqsave(&(*hgctrl).lock,&mut flags); for i in 0..=(*hgctrl).nr_hgei as usize { let vcpu=(*hgctrl).owners[i]; if vcpu.is_null() { continue; } raw_spin_unlock_irqrestore(&(*hgctrl).lock,flags); kvm_riscv_vcpu_aia_imsic_release(vcpu); if csr_read(CSR_HGEIE)&BIT(i)!=0 { csr_clear(CSR_HGEIE,BIT(i)); kvm_vcpu_kick(vcpu); } raw_spin_lock_irqsave(&(*hgctrl).lock,&mut flags); } raw_spin_unlock_irqrestore(&(*hgctrl).lock,flags); put_cpu_ptr(&mut AIA_HGEI); }
unsafe fn kvm_riscv_aia_init() -> c_int { if !riscv_isa_extension_available(core::ptr::null_mut(),SxAIA) { return -ENODEV; } let gc=imsic_get_global_config(); atomic_set(&mut KVM_RISCV_AIA_NR_HGEI,if !gc.is_null(){(*gc).nr_guest_files}else{0}); KVM_RISCV_AIA_MAX_IDS=if !gc.is_null(){(*gc).nr_guest_ids+1}else{IMSIC_MAX_ID}; let rc=aia_hgei_init(); if rc!=0{return rc;} let rc=kvm_register_device_ops(&kvm_riscv_aia_device_ops,KVM_DEV_TYPE_RISCV_AIA); if rc!=0 { aia_hgei_exit(); return rc; } static_branch_enable(&KVM_RISCV_AIA_AVAILABLE); 0 }
unsafe fn kvm_riscv_aia_exit() { if !kvm_riscv_aia_available() { return; } kvm_unregister_device_ops(KVM_DEV_TYPE_RISCV_AIA); aia_hgei_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
