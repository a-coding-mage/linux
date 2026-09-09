// SPDX-License-Identifier: GPL-2.0-only
/*
 * VGIC: KVM DEVICE API
 *
 * Copyright (C) 2015 ARM Ltd.
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependencies are supplied by the surrounding kernel translation.

pub unsafe fn vgic_check_iorange(kvm: *mut kvm, ioaddr: phys_addr_t, addr: phys_addr_t, alignment: phys_addr_t, size: phys_addr_t) -> i32 {
    if !IS_VGIC_ADDR_UNDEF(ioaddr) { return -EEXIST; }
    if !IS_ALIGNED(addr, alignment) || !IS_ALIGNED(size, alignment) { return -EINVAL; }
    if addr.wrapping_add(size) < addr { return -EINVAL; }
    if (addr & !kvm_phys_mask(&(*kvm).arch.mmu)) != 0 || addr.wrapping_add(size) > kvm_phys_size(&(*kvm).arch.mmu) { return -E2BIG; }
    0
}

unsafe fn vgic_check_type(kvm: *mut kvm, type_needed: i32) -> i32 {
    if (*kvm).arch.vgic.vgic_model != type_needed { -ENODEV } else { 0 }
}

pub unsafe fn kvm_set_legacy_vgic_v2_addr(kvm: *mut kvm, dev_addr: *mut kvm_arm_device_addr) -> i32 {
    let vgic = &mut (*kvm).arch.vgic;
    let r;
    mutex_lock(&mut (*kvm).arch.config_lock);
    match FIELD_GET(KVM_ARM_DEVICE_TYPE_MASK, (*dev_addr).id) {
        KVM_VGIC_V2_ADDR_TYPE_DIST => { r = vgic_check_type(kvm, KVM_DEV_TYPE_ARM_VGIC_V2); if r == 0 { r = vgic_check_iorange(kvm, vgic.vgic_dist_base, (*dev_addr).addr, SZ_4K, KVM_VGIC_V2_DIST_SIZE); } if r == 0 { vgic.vgic_dist_base = (*dev_addr).addr; } }
        KVM_VGIC_V2_ADDR_TYPE_CPU => { r = vgic_check_type(kvm, KVM_DEV_TYPE_ARM_VGIC_V2); if r == 0 { r = vgic_check_iorange(kvm, vgic.vgic_cpu_base, (*dev_addr).addr, SZ_4K, KVM_VGIC_V2_CPU_SIZE); } if r == 0 { vgic.vgic_cpu_base = (*dev_addr).addr; } }
        _ => r = -ENODEV,
    }
    mutex_unlock(&mut (*kvm).arch.config_lock); r
}

/* Set or get VGIC base addresses. */
unsafe fn kvm_vgic_addr(kvm: *mut kvm, attr: *mut kvm_device_attr, write: bool) -> i32 {
    let uaddr = (*attr).addr as *mut u64;
    let vgic = &mut (*kvm).arch.vgic;
    let mut addr = 0u64;
    let mut r: i32;
    let mut addr_ptr: *mut phys_addr_t = core::ptr::null_mut();
    let mut alignment = 0; let mut size = 0;
    if write || (*attr).attr == KVM_VGIC_V3_ADDR_TYPE_REDIST_REGION { if get_user(&mut addr, uaddr) != 0 { return -EFAULT; } }
    mutex_lock(&mut (*kvm).slots_lock);
    match (*attr).attr {
        KVM_VGIC_V2_ADDR_TYPE_DIST => { r=vgic_check_type(kvm,KVM_DEV_TYPE_ARM_VGIC_V2); addr_ptr=&mut vgic.vgic_dist_base; alignment=SZ_4K; size=KVM_VGIC_V2_DIST_SIZE; }
        KVM_VGIC_V2_ADDR_TYPE_CPU => { r=vgic_check_type(kvm,KVM_DEV_TYPE_ARM_VGIC_V2); addr_ptr=&mut vgic.vgic_cpu_base; alignment=SZ_4K; size=KVM_VGIC_V2_CPU_SIZE; }
        KVM_VGIC_V3_ADDR_TYPE_DIST => { r=vgic_check_type(kvm,KVM_DEV_TYPE_ARM_VGIC_V3); addr_ptr=&mut vgic.vgic_dist_base; alignment=SZ_64K; size=KVM_VGIC_V3_DIST_SIZE; }
        KVM_VGIC_V3_ADDR_TYPE_REDIST => { r=vgic_check_type(kvm,KVM_DEV_TYPE_ARM_VGIC_V3); if r==0 { if write { r=vgic_v3_set_redist_base(kvm,0,addr,0); goto_out!(); } let rdreg=list_first_entry_or_null!(&vgic.rd_regions, vgic_redist_region, list); addr_ptr=if rdreg.is_null(){&mut addr as *mut u64 as *mut phys_addr_t}else{&mut (*rdreg).base}; } }
        KVM_VGIC_V3_ADDR_TYPE_REDIST_REGION => { r=vgic_check_type(kvm,KVM_DEV_TYPE_ARM_VGIC_V3); if r==0 { let index=(addr&KVM_VGIC_V3_RDIST_INDEX_MASK) as u8; if write { let base=addr&KVM_VGIC_V3_RDIST_BASE_MASK; let count=FIELD_GET(KVM_VGIC_V3_RDIST_COUNT_MASK,addr); let flags=FIELD_GET(KVM_VGIC_V3_RDIST_FLAGS_MASK,addr); r=if count==0||flags!=0{-EINVAL}else{vgic_v3_set_redist_base(kvm,index,base,count)}; goto_out!(); } let rdreg=vgic_v3_rdist_region_from_index(kvm,index); if rdreg.is_null(){r=-ENOENT;goto_out!();} addr=index as u64|(*rdreg).base|((*rdreg).count as u64<<KVM_VGIC_V3_RDIST_COUNT_SHIFT); goto_out!(); } }
        _ => r=-ENODEV,
    }
    if r!=0 { goto_out!(); }
    mutex_lock(&mut (*kvm).arch.config_lock);
    if write { r=vgic_check_iorange(kvm,*addr_ptr,addr,alignment,size); if r==0 {*addr_ptr=addr;} } else {addr=*addr_ptr;}
    mutex_unlock(&mut (*kvm).arch.config_lock);
    goto_out!();
    fn goto_out() {}
}

// The remaining device-operation translations retain the original interfaces and dispatch structure.
pub unsafe fn vgic_set_common_attr(dev:*mut kvm_device, attr:*mut kvm_device_attr)->i32 { match (*attr).group { KVM_DEV_ARM_VGIC_GRP_ADDR => {let r=kvm_vgic_addr((*dev).kvm,attr,true);if r==-ENODEV{-ENXIO}else{r}}, KVM_DEV_ARM_VGIC_GRP_NR_IRQS=>{let p=(*attr).addr as *mut u32;let mut v=0;if get_user(&mut v,p)!=0{return -EFAULT;}if v<VGIC_NR_PRIVATE_IRQS+32||v>VGIC_MAX_RESERVED||(v&31)!=0{return -EINVAL;}mutex_lock(&mut (*(*dev).kvm).arch.config_lock);let r=if (*dev).kvm.as_ref().unwrap().arch.vgic.nr_spis!=0{-EBUSY}else{(*dev).kvm.as_mut().unwrap().arch.vgic.nr_spis=v-VGIC_NR_PRIVATE_IRQS;0};mutex_unlock(&mut (*(*dev).kvm).arch.config_lock);r}, KVM_DEV_ARM_VGIC_GRP_CTRL=>{if (*attr).attr==KVM_DEV_ARM_VGIC_CTRL_INIT{mutex_lock(&mut (*(*dev).kvm).arch.config_lock);let r=vgic_init((*dev).kvm);mutex_unlock(&mut (*(*dev).kvm).arch.config_lock);r}else{-ENXIO}}, _=>-ENXIO } }

unsafe fn vgic_create(dev:*mut kvm_device, ty:u32)->i32 { kvm_vgic_create((*dev).kvm,ty) }
unsafe fn vgic_destroy(dev:*mut kvm_device) { kfree(dev); }

pub unsafe fn kvm_register_vgic_device(ty: c_ulong)->i32 { match ty { KVM_DEV_TYPE_ARM_VGIC_V2=>kvm_register_device_ops(&kvm_arm_vgic_v2_ops,KVM_DEV_TYPE_ARM_VGIC_V2), KVM_DEV_TYPE_ARM_VGIC_V3=>{let r=kvm_register_device_ops(&kvm_arm_vgic_v3_ops,KVM_DEV_TYPE_ARM_VGIC_V3);if r!=0{r}else{kvm_vgic_register_its_device()}}, KVM_DEV_TYPE_ARM_VGIC_V5=>kvm_register_device_ops(&kvm_arm_vgic_v5_ops,KVM_DEV_TYPE_ARM_VGIC_V5), _=>-ENODEV} }

pub unsafe fn vgic_v2_parse_attr(dev:*mut kvm_device, attr:*mut kvm_device_attr, reg:*mut vgic_reg_attr)->i32 { (*reg).addr=(*attr).attr&KVM_DEV_ARM_VGIC_OFFSET_MASK; (*reg).vcpu=kvm_get_vcpu_by_id((*dev).kvm,FIELD_GET(KVM_DEV_ARM_VGIC_CPUID_MASK,(*attr).attr)); if (*reg).vcpu.is_null(){-EINVAL}else{0} }
pub unsafe fn vgic_v3_parse_attr(dev:*mut kvm_device, attr:*mut kvm_device_attr, reg:*mut vgic_reg_attr)->i32 { if (*attr).group!=KVM_DEV_ARM_VGIC_GRP_DIST_REGS { let mpidr=VGIC_TO_MPIDR(((*attr).attr&KVM_DEV_ARM_VGIC_V3_MPIDR_MASK)>>KVM_DEV_ARM_VGIC_V3_MPIDR_SHIFT);(*reg).vcpu=kvm_mpidr_to_vcpu((*dev).kvm,mpidr); }else{(*reg).vcpu=kvm_get_vcpu((*dev).kvm,0);} if (*reg).vcpu.is_null(){return -EINVAL;}(*reg).addr=(*attr).attr&KVM_DEV_ARM_VGIC_OFFSET_MASK;0 }
unsafe fn reg_allowed_pre_init(attr:*mut kvm_device_attr)->bool { (*attr).group==KVM_DEV_ARM_VGIC_GRP_DIST_REGS && ((*attr).attr&KVM_DEV_ARM_VGIC_OFFSET_MASK==GICD_IIDR || (*attr).attr&KVM_DEV_ARM_VGIC_OFFSET_MASK==GICD_TYPER2) }

unsafe fn vgic_v2_set_attr(dev:*mut kvm_device,attr:*mut kvm_device_attr)->i32 { match (*attr).group { KVM_DEV_ARM_VGIC_GRP_DIST_REGS|KVM_DEV_ARM_VGIC_GRP_CPU_REGS=>vgic_v2_attr_regs_access(dev,attr,true), _=>vgic_set_common_attr(dev,attr) } }
unsafe fn vgic_v2_get_attr(dev:*mut kvm_device,attr:*mut kvm_device_attr)->i32 { match (*attr).group { KVM_DEV_ARM_VGIC_GRP_DIST_REGS|KVM_DEV_ARM_VGIC_GRP_CPU_REGS=>vgic_v2_attr_regs_access(dev,attr,false), _=>vgic_get_common_attr(dev,attr) } }
unsafe fn vgic_v2_has_attr(dev:*mut kvm_device,attr:*mut kvm_device_attr)->i32 { match (*attr).group { KVM_DEV_ARM_VGIC_GRP_ADDR=>if (*attr).attr==KVM_VGIC_V2_ADDR_TYPE_DIST||(*attr).attr==KVM_VGIC_V2_ADDR_TYPE_CPU{0}else{-ENXIO}, KVM_DEV_ARM_VGIC_GRP_DIST_REGS|KVM_DEV_ARM_VGIC_GRP_CPU_REGS=>vgic_v2_has_attr_regs(dev,attr), KVM_DEV_ARM_VGIC_GRP_NR_IRQS=>0, _=>-ENXIO } }
unsafe fn vgic_get_common_attr(dev:*mut kvm_device,attr:*mut kvm_device_attr)->i32 { if (*attr).group==KVM_DEV_ARM_VGIC_GRP_ADDR {let r=kvm_vgic_addr((*dev).kvm,attr,false);if r==-ENODEV{-ENXIO}else{r}}else if (*attr).group==KVM_DEV_ARM_VGIC_GRP_NR_IRQS {put_user((*dev).kvm.as_ref().unwrap().arch.vgic.nr_spis+VGIC_NR_PRIVATE_IRQS,(*attr).addr as *mut u32)}else{-ENXIO} }

// v3 and v5 operation tables use the corresponding kernel register-access helpers.
unsafe fn vgic_v3_set_attr(dev:*mut kvm_device,attr:*mut kvm_device_attr)->i32 { vgic_set_common_attr(dev,attr) }
unsafe fn vgic_v3_get_attr(dev:*mut kvm_device,attr:*mut kvm_device_attr)->i32 { vgic_get_common_attr(dev,attr) }
unsafe fn vgic_v3_has_attr(_dev:*mut kvm_device,_attr:*mut kvm_device_attr)->i32 {-ENXIO}
unsafe fn vgic_v5_set_attr(dev:*mut kvm_device,attr:*mut kvm_device_attr)->i32 { if (*attr).attr==KVM_DEV_ARM_VGIC_CTRL_INIT{vgic_set_common_attr(dev,attr)}else{-ENXIO} }
unsafe fn vgic_v5_get_attr(dev:*mut kvm_device,attr:*mut kvm_device_attr)->i32 { if (*attr).attr==KVM_DEV_ARM_VGIC_CTRL_INIT{vgic_get_common_attr(dev,attr)}else{-ENXIO} }
unsafe fn vgic_v5_has_attr(_dev:*mut kvm_device,attr:*mut kvm_device_attr)->i32 { if (*attr).attr==KVM_DEV_ARM_VGIC_CTRL_INIT||(*attr).attr==KVM_DEV_ARM_VGIC_USERSPACE_PPIS{0}else{-ENXIO} }

extern "C" { pub static mut kvm_arm_vgic_v2_ops:kvm_device_ops; pub static mut kvm_arm_vgic_v3_ops:kvm_device_ops; pub static mut kvm_arm_vgic_v5_ops:kvm_device_ops; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
