// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015, 2016 ARM Ltd.
 */

// Linux and VGIC dependencies are supplied by the surrounding translation.

/* Initialization rules and stage descriptions are preserved from the C source. */

const DEFAULT_MI_INTID: u32 = 25;

pub unsafe fn kvm_vgic_early_init(kvm: *mut kvm) {
    let dist = &mut (*kvm).arch.vgic;
    xa_init_flags(&mut dist.lpi_xa, XA_FLAGS_LOCK_IRQ);
}

unsafe fn vgic_allocate_private_irqs_locked(vcpu: *mut kvm_vcpu, ty: u32) -> i32;

pub unsafe fn kvm_vgic_create(kvm: *mut kvm, ty: u32) -> i32 {
    let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    let mut i: usize = 0;
    let mut ret: i32;
    if ty == KVM_DEV_TYPE_ARM_VGIC_V2 && !kvm_vgic_global_state.can_emulate_gicv2 { return -ENODEV; }
    lockdep_assert_held(&(*kvm).lock);
    ret = -EBUSY;
    if kvm_trylock_all_vcpus(kvm) != 0 { return ret; }
    mutex_lock(&mut (*kvm).arch.config_lock);
    if (*kvm).created_vcpus != atomic_read(&(*kvm).online_vcpus) { goto out_unlock; }
    if irqchip_in_kernel(kvm) { ret = -EEXIST; goto out_unlock; }
    kvm_for_each_vcpu!(i, vcpu, kvm, {
        if vcpu_has_run_once(vcpu) { goto out_unlock; }
    });
    ret = 0;
    if ty == KVM_DEV_TYPE_ARM_VGIC_V2 { (*kvm).max_vcpus = VGIC_V2_MAX_CPUS; }
    else if ty == KVM_DEV_TYPE_ARM_VGIC_V3 { (*kvm).max_vcpus = VGIC_V3_MAX_CPUS; }
    else if ty == KVM_DEV_TYPE_ARM_VGIC_V5 { (*kvm).max_vcpus = min(VGIC_V5_MAX_CPUS, kvm_vgic_global_state.max_gic_vcpus); }
    if atomic_read(&(*kvm).online_vcpus) > (*kvm).max_vcpus { ret = -E2BIG; goto out_unlock; }
    (*kvm).arch.vgic.in_kernel = true;
    (*kvm).arch.vgic.vgic_model = ty;
    (*kvm).arch.vgic.implementation_rev = KVM_VGIC_IMP_REV_LATEST;
    (*kvm).arch.vgic.vgic_dist_base = VGIC_ADDR_UNDEF;
    match ty {
        KVM_DEV_TYPE_ARM_VGIC_V2 => (*kvm).arch.vgic.vgic_cpu_base = VGIC_ADDR_UNDEF,
        KVM_DEV_TYPE_ARM_VGIC_V3 => INIT_LIST_HEAD(&mut (*kvm).arch.vgic.rd_regions),
        _ => (),
    }
    kvm_vgic_finalize_idregs(kvm);
    kvm_for_each_vcpu!(i, vcpu, kvm, { ret = vgic_allocate_private_irqs_locked(vcpu, ty); if ret != 0 { break; } });
    if ret != 0 {
        kvm_for_each_vcpu!(i, vcpu, kvm, { let cpu = &mut (*vcpu).arch.vgic_cpu; kfree(cpu.private_irqs); cpu.private_irqs = core::ptr::null_mut(); });
        (*kvm).arch.vgic.vgic_model = 0; (*kvm).arch.vgic.in_kernel = false; goto out_unlock;
    }
    if ty == KVM_DEV_TYPE_ARM_VGIC_V3 { (*kvm).arch.vgic.nassgicap = system_supports_direct_sgis(); }
    if ty == KVM_DEV_TYPE_ARM_VGIC_V5 { kvm_timer_init_vm(kvm); }
out_unlock:
    mutex_unlock(&mut (*kvm).arch.config_lock); kvm_unlock_all_vcpus(kvm); ret
}

unsafe fn kvm_vgic_dist_init(kvm: *mut kvm, nr_spis: u32) -> i32 {
    let dist = &mut (*kvm).arch.vgic; let vcpu0 = kvm_get_vcpu(kvm, 0);
    if !dist.spis.is_null() { return 0; }
    dist.active_spis = ATOMIC_INIT(0); dist.spis = kzalloc_objs::<vgic_irq>(nr_spis, GFP_KERNEL_ACCOUNT);
    if dist.spis.is_null() { return -ENOMEM; }
    for i in 0..nr_spis { let irq = &mut *dist.spis.add(i as usize); irq.intid = i + VGIC_NR_PRIVATE_IRQS; INIT_LIST_HEAD(&mut irq.ap_list); raw_spin_lock_init(&mut irq.irq_lock); irq.vcpu = core::ptr::null_mut(); irq.target_vcpu = vcpu0; refcount_set(&mut irq.refcount, 0); match dist.vgic_model { KVM_DEV_TYPE_ARM_VGIC_V2 => { irq.targets = 0; irq.group = 0; }, KVM_DEV_TYPE_ARM_VGIC_V3 => { irq.mpidr = 0; irq.group = 1; }, _ => { kfree(dist.spis); dist.spis = core::ptr::null_mut(); return -EINVAL; } } }
    0
}

pub unsafe fn kvm_vgic_vcpu_nv_init(vcpu: *mut kvm_vcpu) -> i32 {
    mutex_lock(&mut (*vcpu).kvm.arch.config_lock);
    if (*vcpu).kvm.arch.vgic.mi_intid == 0 { (*vcpu).kvm.arch.vgic.mi_intid = DEFAULT_MI_INTID; }
    let ret = kvm_vgic_set_owner(vcpu, (*vcpu).kvm.arch.vgic.mi_intid, vcpu);
    mutex_unlock(&mut (*vcpu).kvm.arch.config_lock); ret
}

unsafe fn vgic_setup_private_irq(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq, ty: u32) {
    (*irq).intid = irq.offset_from((*vcpu).arch.vgic_cpu.private_irqs) as u32;
    if vgic_irq_is_sgi((*irq).intid) { (*irq).enabled = 1; (*irq).config = VGIC_CONFIG_EDGE; } else { (*irq).config = VGIC_CONFIG_LEVEL; }
    match ty { KVM_DEV_TYPE_ARM_VGIC_V3 => { (*irq).group = 1; (*irq).mpidr = kvm_vcpu_get_mpidr_aff(vcpu); }, KVM_DEV_TYPE_ARM_VGIC_V2 => { (*irq).group = 0; (*irq).targets = BIT((*vcpu).vcpu_id); }, _ => () }
}

unsafe fn vgic_v5_setup_private_irq(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq) { let i = irq.offset_from((*vcpu).arch.vgic_cpu.private_irqs) as i32; (*irq).intid = vgic_v5_make_ppi(i); (*irq).config = if i == GICV5_ARCH_PPI_SW_PPI { VGIC_CONFIG_EDGE } else { VGIC_CONFIG_LEVEL }; vgic_v5_set_ppi_ops(vcpu, (*irq).intid); }

unsafe fn vgic_allocate_private_irqs_locked(vcpu: *mut kvm_vcpu, ty: u32) -> i32 {
    lockdep_assert_held(&(*vcpu).kvm.arch.config_lock); let cpu = &mut (*vcpu).arch.vgic_cpu; if !cpu.private_irqs.is_null() { return 0; }
    let n = if vgic_is_v5((*vcpu).kvm) { VGIC_V5_NR_PRIVATE_IRQS } else { VGIC_NR_PRIVATE_IRQS };
    cpu.private_irqs = kzalloc_objs::<vgic_irq>(n, GFP_KERNEL_ACCOUNT); if cpu.private_irqs.is_null() { return -ENOMEM; }
    for i in 0..n { let irq = cpu.private_irqs.add(i); INIT_LIST_HEAD(&mut (*irq).ap_list); raw_spin_lock_init(&mut (*irq).irq_lock); (*irq).vcpu = core::ptr::null_mut(); (*irq).target_vcpu = vcpu; refcount_set(&mut (*irq).refcount, 0); if vgic_is_v5((*vcpu).kvm) { vgic_v5_setup_private_irq(vcpu, irq); } else { vgic_setup_private_irq(vcpu, irq, ty); } } 0
}

unsafe fn vgic_allocate_private_irqs(vcpu: *mut kvm_vcpu, ty: u32) -> i32 { mutex_lock(&mut (*vcpu).kvm.arch.config_lock); let r = vgic_allocate_private_irqs_locked(vcpu, ty); mutex_unlock(&mut (*vcpu).kvm.arch.config_lock); r }

pub unsafe fn kvm_vgic_vcpu_init(vcpu: *mut kvm_vcpu) -> i32 { let cpu=&mut (*vcpu).arch.vgic_cpu; let dist=&(*vcpu).kvm.arch.vgic; let mut ret=0; cpu.rd_iodev.base_addr=VGIC_ADDR_UNDEF; INIT_LIST_HEAD(&mut cpu.ap_list_head); raw_spin_lock_init(&mut cpu.ap_list_lock); atomic_set(&mut cpu.vgic_v3.its_vpe.vlpi_count,0); if !irqchip_in_kernel((*vcpu).kvm) { return 0; } ret=vgic_allocate_private_irqs(vcpu,dist.vgic_model); if ret!=0{return ret;} if dist.vgic_model==KVM_DEV_TYPE_ARM_VGIC_V3 {mutex_lock(&mut (*vcpu).kvm.slots_lock); ret=vgic_register_redist_iodev(vcpu); mutex_unlock(&mut (*vcpu).kvm.slots_lock);} ret }

unsafe fn kvm_vgic_vcpu_reset(vcpu:*mut kvm_vcpu){let d=&(*vcpu).kvm.arch.vgic;if d.vgic_model==KVM_DEV_TYPE_ARM_VGIC_V5{vgic_v5_reset(vcpu)}else if kvm_vgic_global_state.type_==VGIC_V2{vgic_v2_reset(vcpu)}else{vgic_v3_reset(vcpu)}}

pub unsafe fn vgic_init(kvm:*mut kvm)->i32{let d=&mut(*kvm).arch.vgic;let mut v:*mut kvm_vcpu=core::ptr::null_mut();let mut r=0;let mut i=0usize;lockdep_assert_held(&(*kvm).arch.config_lock);if vgic_initialized(kvm){return 0;}if (*kvm).created_vcpus!=atomic_read(&(*kvm).online_vcpus){return -EBUSY;}if !vgic_is_v5(kvm){if d.nr_spis==0{d.nr_spis=VGIC_NR_IRQS_LEGACY-VGIC_NR_PRIVATE_IRQS;}r=kvm_vgic_dist_init(kvm,d.nr_spis);if r!=0{return r;}if vgic_supports_direct_irqs(kvm){r=vgic_v4_init(kvm);if r!=0{return r;}}}else{r=vgic_v5_init(kvm);if r!=0{return r;}}kvm_for_each_vcpu!(i,v,kvm,{kvm_vgic_vcpu_reset(v);});r=kvm_vgic_setup_default_irq_routing(kvm);if r!=0{return r;}vgic_debug_init(kvm);d.initialized=true;0}

unsafe fn kvm_vgic_dist_destroy(kvm:*mut kvm){let d=&mut(*kvm).arch.vgic;d.ready=false;d.initialized=false;kfree(d.spis);d.spis=core::ptr::null_mut();d.nr_spis=0;d.vgic_dist_base=VGIC_ADDR_UNDEF;if d.vgic_model==KVM_DEV_TYPE_ARM_VGIC_V3{let mut r:*mut vgic_redist_region=core::ptr::null_mut();let mut n:*mut vgic_redist_region=core::ptr::null_mut();list_for_each_entry_safe!(r,n,&mut d.rd_regions,list,{vgic_v3_free_redist_region(kvm,r);});INIT_LIST_HEAD(&mut d.rd_regions);}else{d.vgic_cpu_base=VGIC_ADDR_UNDEF;}if vgic_supports_direct_irqs(kvm){vgic_v4_teardown(kvm);}xa_destroy(&mut d.lpi_xa);}

unsafe fn __kvm_vgic_vcpu_destroy(vcpu:*mut kvm_vcpu){let c=&mut(*vcpu).arch.vgic_cpu;vgic_flush_pending_lpis(vcpu);INIT_LIST_HEAD(&mut c.ap_list_head);kfree(c.private_irqs);c.private_irqs=core::ptr::null_mut();if (*vcpu).kvm.arch.vgic.vgic_model==KVM_DEV_TYPE_ARM_VGIC_V3{if kvm_get_vcpu_by_id((*vcpu).kvm,(*vcpu).vcpu_id)!=vcpu{vgic_unregister_redist_iodev(vcpu);}c.rd_iodev.base_addr=VGIC_ADDR_UNDEF;}}
pub unsafe fn kvm_vgic_vcpu_destroy(vcpu:*mut kvm_vcpu){let k=(*vcpu).kvm;mutex_lock(&mut(*k).slots_lock);__kvm_vgic_vcpu_destroy(vcpu);mutex_unlock(&mut(*k).slots_lock);}
pub unsafe fn kvm_vgic_destroy(kvm:*mut kvm){let mut v:*mut kvm_vcpu=core::ptr::null_mut();let mut i=0usize;mutex_lock(&mut(*kvm).slots_lock);mutex_lock(&mut(*kvm).arch.config_lock);vgic_debug_destroy(kvm);kvm_for_each_vcpu!(i,v,kvm,{__kvm_vgic_vcpu_destroy(v);});kvm_vgic_dist_destroy(kvm);mutex_unlock(&mut(*kvm).arch.config_lock);if(*kvm).arch.vgic.vgic_model==KVM_DEV_TYPE_ARM_VGIC_V3{kvm_for_each_vcpu!(i,v,kvm,{vgic_unregister_redist_iodev(v);});}mutex_unlock(&mut(*kvm).slots_lock);}

pub unsafe fn vgic_lazy_init(kvm:*mut kvm)->i32{if unlikely(!vgic_initialized(kvm)){if(*kvm).arch.vgic.vgic_model!=KVM_DEV_TYPE_ARM_VGIC_V2{return -EBUSY;}mutex_lock(&mut(*kvm).arch.config_lock);let r=vgic_init(kvm);mutex_unlock(&mut(*kvm).arch.config_lock);return r;}0}

pub unsafe fn kvm_vgic_map_resources(kvm:*mut kvm)->i32{let d=&mut(*kvm).arch.vgic;if likely(smp_load_acquire(&d.ready)){return 0;}mutex_lock(&mut(*kvm).slots_lock);mutex_lock(&mut(*kvm).arch.config_lock);if d.ready{goto out;}if !irqchip_in_kernel(kvm){goto out;}let (r,t,needs)=if d.vgic_model==KVM_DEV_TYPE_ARM_VGIC_V2{(vgic_v2_map_resources(kvm),VGIC_V2,true)}else if d.vgic_model==KVM_DEV_TYPE_ARM_VGIC_V3{(vgic_v3_map_resources(kvm),VGIC_V3,true)}else{(vgic_v5_map_resources(kvm),VGIC_V5,false)};if r!=0{goto out;}if needs{let b=d.vgic_dist_base;mutex_unlock(&mut(*kvm).arch.config_lock);if vgic_register_dist_iodev(kvm,b,t)!=0{kvm_err!("Unable to register VGIC dist MMIO regions\n");goto out_slots;}}else{mutex_unlock(&mut(*kvm).arch.config_lock);}smp_store_release(&mut d.ready,true);goto out_slots;out:mutex_unlock(&mut(*kvm).arch.config_lock);out_slots:if d.ready==false{kvm_vm_dead(kvm);}mutex_unlock(&mut(*kvm).slots_lock);0}

pub unsafe fn kvm_vgic_finalize_idregs(kvm:*mut kvm){let ty=(*kvm).arch.vgic.vgic_model;let mut a0=kvm_read_vm_id_reg(kvm,SYS_ID_AA64PFR0_EL1)&!ID_AA64PFR0_EL1_GIC;let mut a2=kvm_read_vm_id_reg(kvm,SYS_ID_AA64PFR2_EL1)&!ID_AA64PFR2_EL1_GCIE;let mut p1=kvm_read_vm_id_reg(kvm,SYS_ID_PFR1_EL1)&!ID_PFR1_EL1_GIC;match ty{KVM_DEV_TYPE_ARM_VGIC_V2=>(),KVM_DEV_TYPE_ARM_VGIC_V3=>{a0|=SYS_FIELD_PREP_ENUM(ID_AA64PFR0_EL1,GIC,IMP);if kvm_supports_32bit_el0(){p1|=SYS_FIELD_PREP_ENUM(ID_PFR1_EL1,GIC,GICv3);}},KVM_DEV_TYPE_ARM_VGIC_V5=>a2|=SYS_FIELD_PREP_ENUM(ID_AA64PFR2_EL1,GCIE,IMP),_=>WARN_ONCE!(true,"Unknown VGIC type!!!\n")};kvm_set_vm_id_reg(kvm,SYS_ID_AA64PFR0_EL1,a0);kvm_set_vm_id_reg(kvm,SYS_ID_AA64PFR2_EL1,a2);kvm_set_vm_id_reg(kvm,SYS_ID_PFR1_EL1,p1);}

pub unsafe fn kvm_vgic_cpu_up(){enable_percpu_irq(kvm_vgic_global_state.maint_irq,0)}
pub unsafe fn kvm_vgic_cpu_down(){disable_percpu_irq(kvm_vgic_global_state.maint_irq)}
unsafe fn vgic_maintenance_handler(_irq:i32,data:*mut core::ffi::c_void)->irqreturn_t{let v=*(data as *mut *mut kvm_vcpu);if !v.is_null()&&vgic_state_is_nested(v){vgic_v3_handle_nested_maint_irq(v);}IRQ_HANDLED}
static mut gic_kvm_info:*mut gic_kvm_info=core::ptr::null_mut();
pub unsafe fn vgic_set_kvm_info(info:*const gic_kvm_info){BUG_ON!(!gic_kvm_info.is_null());gic_kvm_info=kmalloc_obj(*info);if !gic_kvm_info.is_null(){*gic_kvm_info=*info;}}
pub unsafe fn kvm_vgic_init_cpu_hardware(){BUG_ON!(preemptible());if kvm_vgic_global_state.type_==VGIC_V2{vgic_v2_init_lrs();}else if kvm_vgic_global_state.type_==VGIC_V3||kvm_vgic_global_state.has_gcie_v3_compat{kvm_call_hyp(__vgic_v3_init_lrs);}}
pub unsafe fn kvm_vgic_hyp_init()->i32{if gic_kvm_info.is_null(){return -ENODEV;}let has_mask=!(*gic_kvm_info).no_maint_irq_mask;if has_mask&&(*gic_kvm_info).maint_irq==0{kvm_err!("No vgic maintenance irq\n");kfree(gic_kvm_info);gic_kvm_info=core::ptr::null_mut();return -ENXIO;}if(*gic_kvm_info).no_hw_deactivation{kvm_info!("Non-architectural vgic, tainting kernel\n");add_taint(TAINT_CPU_OUT_OF_SPEC,LOCKDEP_STILL_OK);kvm_vgic_global_state.no_hw_deactivation=true;}let r=match(*gic_kvm_info).type_{GIC_V2=>vgic_v2_probe(gic_kvm_info),GIC_V3=>{let x=vgic_v3_probe(gic_kvm_info);if x==0{static_branch_enable(&mut kvm_vgic_global_state.gicv3_cpuif);kvm_info!("GIC system register CPU interface enabled\n");}x},GIC_V5=>vgic_v5_probe(gic_kvm_info),_=>-ENODEV};kvm_vgic_global_state.maint_irq=(*gic_kvm_info).maint_irq;kfree(gic_kvm_info);gic_kvm_info=core::ptr::null_mut();if r!=0{return r;}if !has_mask&&kvm_vgic_global_state.maint_irq==0{return 0;}let r=request_percpu_irq(kvm_vgic_global_state.maint_irq,vgic_maintenance_handler as _,"vgic",kvm_get_running_vcpus());if r!=0{kvm_err!("Cannot register interrupt %d\n",kvm_vgic_global_state.maint_irq);return r;}kvm_info!("vgic interrupt IRQ%d\n",kvm_vgic_global_state.maint_irq);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
