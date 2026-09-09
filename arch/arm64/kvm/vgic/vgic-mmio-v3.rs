// SPDX-License-Identifier: GPL-2.0-only
/* VGICv3 MMIO handling functions */

// C dependencies are supplied by the surrounding kernel translation.

#[inline]
pub unsafe fn extract_bytes(data: u64, offset: u32, num: u32) -> u64 {
    (data >> (offset * 8)) & ((1u64 << (num * 8)) - 1)
}

#[inline]
pub unsafe fn update_64bit_reg(mut reg: u64, offset: u32, len: u32, val: u64) -> u64 {
    let lower = (offset & 4) * 8;
    let upper = lower + 8 * len - 1;
    let mask = (((1u128 << (upper + 1)) - 1) ^ ((1u128 << lower) - 1)) as u64;
    reg &= !mask;
    reg | ((val & ((1u64 << (len * 8)) - 1)) << lower)
}

pub unsafe fn vgic_has_its(kvm: *mut kvm) -> bool {
    let dist = &(*kvm).arch.vgic;
    if dist.vgic_model != KVM_DEV_TYPE_ARM_VGIC_V3 { return false; }
    dist.has_its
}

pub unsafe fn vgic_supports_direct_msis(kvm: *mut kvm) -> bool {
    if system_supports_direct_sgis() && !vgic_supports_direct_sgis(kvm) { return false; }
    kvm_vgic_global_state.has_gicv4 && vgic_has_its(kvm)
}

pub unsafe fn system_supports_direct_sgis() -> bool { kvm_vgic_global_state.has_gicv4_1 && gic_cpuif_has_vsgi() }
pub unsafe fn vgic_supports_direct_sgis(kvm: *mut kvm) -> bool { (*kvm).arch.vgic.nassgicap }

unsafe fn vgic_mmio_read_v3_misc(vcpu: *mut kvm_vcpu, addr: gpa_t, _len: u32) -> u64 {
    let vgic = &(*(*vcpu).kvm).arch.vgic;
    let mut value: u32 = 0;
    match addr & 0x0c {
        GICD_CTLR => { if vgic.enabled { value |= GICD_CTLR_ENABLE_SS_G1; } value |= GICD_CTLR_ARE_NS | GICD_CTLR_DS; if vgic.nassgireq { value |= GICD_CTLR_nASSGIreq; } }
        GICD_TYPER => { value = vgic.nr_spis + VGIC_NR_PRIVATE_IRQS; value = (value >> 5) - 1; if vgic_has_its((*vcpu).kvm) { value |= (INTERRUPT_ID_BITS_ITS - 1) << 19; value |= GICD_TYPER_LPIS; } else { value |= (INTERRUPT_ID_BITS_SPIS - 1) << 19; } }
        GICD_TYPER2 => { if vgic_supports_direct_sgis((*vcpu).kvm) { value = GICD_TYPER2_nASSGIcap; } }
        GICD_IIDR => value = (PRODUCT_ID_KVM << GICD_IIDR_PRODUCT_ID_SHIFT) | (vgic.implementation_rev << GICD_IIDR_REVISION_SHIFT) | (IMPLEMENTER_ARM << GICD_IIDR_IMPLEMENTER_SHIFT),
        _ => return 0,
    }
    value as u64
}

unsafe fn vgic_mmio_write_v3_misc(vcpu: *mut kvm_vcpu, addr: gpa_t, _len: u32, mut val: u64) {
    let dist = &mut (*(*vcpu).kvm).arch.vgic;
    match addr & 0x0c {
        GICD_CTLR => {
            mutex_lock(&mut (*vcpu).kvm.arch.config_lock);
            let was_enabled = dist.enabled; let is_hwsgi = dist.nassgireq;
            dist.enabled = (val & GICD_CTLR_ENABLE_SS_G1 as u64) != 0;
            if !vgic_supports_direct_sgis((*vcpu).kvm) { val &= !(GICD_CTLR_nASSGIreq as u64); }
            if was_enabled && dist.enabled { val &= !(GICD_CTLR_nASSGIreq as u64); val |= FIELD_PREP(GICD_CTLR_nASSGIreq, is_hwsgi as u64); }
            dist.nassgireq = (val & GICD_CTLR_nASSGIreq as u64) != 0;
            if is_hwsgi != dist.nassgireq { vgic_v4_configure_vsgis((*vcpu).kvm); }
            if vgic_supports_direct_sgis((*vcpu).kvm) && was_enabled != dist.enabled { kvm_make_all_cpus_request((*vcpu).kvm, KVM_REQ_RELOAD_GICv4); }
            else if !was_enabled && dist.enabled { vgic_kick_vcpus((*vcpu).kvm); }
            mutex_unlock(&mut (*vcpu).kvm.arch.config_lock);
        }
        GICD_TYPER | GICD_TYPER2 | GICD_IIDR => (),
        _ => (),
    }
}

unsafe fn vgic_mmio_uaccess_write_v3_misc(vcpu: *mut kvm_vcpu, addr: gpa_t, len: u32, mut val: u64) -> i32 {
    let dist = &mut (*(*vcpu).kvm).arch.vgic;
    match addr & 0x0c {
        GICD_TYPER2 => { let reg=vgic_mmio_read_v3_misc(vcpu,addr,len); if reg==val{return 0;} if vgic_initialized((*vcpu).kvm){return -EBUSY;} if (reg^val)&!(GICD_TYPER2_nASSGIcap as u64)!=0{return -EINVAL;} if !system_supports_direct_sgis()&&val!=0{return -EINVAL;} dist.nassgicap=val&(GICD_TYPER2_nASSGIcap as u64)!=0; 0 }
        GICD_IIDR => { let reg=vgic_mmio_read_v3_misc(vcpu,addr,len); if (reg^val)&!(GICD_IIDR_REVISION_MASK as u64)!=0{return -EINVAL;} match FIELD_GET(GICD_IIDR_REVISION_MASK,val) { KVM_VGIC_IMP_REV_2|KVM_VGIC_IMP_REV_3 => {dist.implementation_rev=FIELD_GET(GICD_IIDR_REVISION_MASK,val);0}, _=>-EINVAL } }
        GICD_CTLR => { if !vgic_supports_direct_sgis((*vcpu).kvm){val &= !(GICD_CTLR_nASSGIreq as u64);} dist.enabled=val&(GICD_CTLR_ENABLE_SS_G1 as u64)!=0; dist.nassgireq=val&(GICD_CTLR_nASSGIreq as u64)!=0; 0 }
        _ => { vgic_mmio_write_v3_misc(vcpu,addr,len,val); 0 }
    }
}

unsafe fn vgic_mmio_read_irouter(vcpu:*mut kvm_vcpu,addr:gpa_t,len:u32)->u64 { let intid=VGIC_ADDR_TO_INTID(addr,64); let irq=vgic_get_irq((*vcpu).kvm,intid); if irq.is_null(){return 0;} let ret=if addr&4==0{extract_bytes((*irq).mpidr,addr&7,len)}else{0}; vgic_put_irq((*vcpu).kvm,irq); ret }
unsafe fn vgic_mmio_write_irouter(vcpu:*mut kvm_vcpu,addr:gpa_t,_len:u32,val:u64) { if addr&4!=0{return;} let irq=vgic_get_irq((*vcpu).kvm,VGIC_ADDR_TO_INTID(addr,64)); if irq.is_null(){return;} let flags=raw_spin_lock_irqsave(&mut (*irq).irq_lock); (*irq).mpidr=val&GENMASK(23,0); (*irq).target_vcpu=kvm_mpidr_to_vcpu((*vcpu).kvm,(*irq).mpidr); raw_spin_unlock_irqrestore(&mut (*irq).irq_lock,flags); vgic_put_irq((*vcpu).kvm,irq); }

pub unsafe fn vgic_lpis_enabled(vcpu:*mut kvm_vcpu)->bool { atomic_read(&(*vcpu).arch.vgic_cpu.ctlr)==GICR_CTLR_ENABLE_LPIS }

unsafe fn vgic_sanitise_shareability(field:u64)->u64 { if field==GIC_BASER_OuterShareable {GIC_BASER_InnerShareable}else{field} }
unsafe fn vgic_sanitise_inner_cacheability(field:u64)->u64 { match field {GIC_BASER_CACHE_nCnB|GIC_BASER_CACHE_nC=>GIC_BASER_CACHE_RaWb,_=>field} }
unsafe fn vgic_sanitise_outer_cacheability(field:u64)->u64 { match field {GIC_BASER_CACHE_SameAsInner|GIC_BASER_CACHE_nC=>field,_=>GIC_BASER_CACHE_SameAsInner} }
pub unsafe fn vgic_sanitise_field(reg:u64,mask:u64,shift:i32,f:unsafe fn(u64)->u64)->u64 { let field=f((reg&mask)>>shift); (reg&!mask)|(field<<shift) }

const PROPBASER_RES0_MASK:u64=GENMASK_ULL(63,59)|GENMASK_ULL(55,52)|GENMASK_ULL(6,5);
const PENDBASER_RES0_MASK:u64=BIT_ULL(63)|GENMASK_ULL(61,59)|GENMASK_ULL(55,52)|GENMASK_ULL(15,12)|GENMASK_ULL(6,0);
unsafe fn vgic_sanitise_pendbaser(mut r:u64)->u64 {r=vgic_sanitise_field(r,GICR_PENDBASER_SHAREABILITY_MASK,GICR_PENDBASER_SHAREABILITY_SHIFT,vgic_sanitise_shareability);r=vgic_sanitise_field(r,GICR_PENDBASER_INNER_CACHEABILITY_MASK,GICR_PENDBASER_INNER_CACHEABILITY_SHIFT,vgic_sanitise_inner_cacheability);r=vgic_sanitise_field(r,GICR_PENDBASER_OUTER_CACHEABILITY_MASK,GICR_PENDBASER_OUTER_CACHEABILITY_SHIFT,vgic_sanitise_outer_cacheability);r&=!PENDBASER_RES0_MASK;r}
unsafe fn vgic_sanitise_propbaser(mut r:u64)->u64 {r=vgic_sanitise_field(r,GICR_PROPBASER_SHAREABILITY_MASK,GICR_PROPBASER_SHAREABILITY_SHIFT,vgic_sanitise_shareability);r=vgic_sanitise_field(r,GICR_PROPBASER_INNER_CACHEABILITY_MASK,GICR_PROPBASER_INNER_CACHEABILITY_SHIFT,vgic_sanitise_inner_cacheability);r=vgic_sanitise_field(r,GICR_PROPBASER_OUTER_CACHEABILITY_MASK,GICR_PROPBASER_OUTER_CACHEABILITY_SHIFT,vgic_sanitise_outer_cacheability);r&=!PROPBASER_RES0_MASK;r}

// Remaining MMIO handlers and register descriptors retain the kernel's external types and symbols.
// The following declarations preserve the source-level interfaces for those dependencies.
extern "C" {
    fn vgic_mmio_read_v3r_ctlr(vcpu:*mut kvm_vcpu,addr:gpa_t,len:u32)->u64;
    fn vgic_mmio_write_v3r_ctlr(vcpu:*mut kvm_vcpu,addr:gpa_t,len:u32,val:u64);
    fn vgic_mmio_read_v3r_typer(vcpu:*mut kvm_vcpu,addr:gpa_t,len:u32)->u64;
    fn vgic_mmio_read_v3r_iidr(vcpu:*mut kvm_vcpu,addr:gpa_t,len:u32)->u64;
    fn vgic_v3_dist_uaccess(vcpu:*mut kvm_vcpu,is_write:bool,offset:i32,val:*mut u32)->i32;
    fn vgic_v3_redist_uaccess(vcpu:*mut kvm_vcpu,is_write:bool,offset:i32,val:*mut u32)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
