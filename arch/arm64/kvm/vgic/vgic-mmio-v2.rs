// SPDX-License-Identifier: GPL-2.0-only
/* VGICv2 MMIO handling functions. */

// External Linux/KVM definitions and register-description macros are supplied by other files.

const GICC_ARCH_VERSION_V2: u32 = 0x2;

unsafe fn vgic_mmio_read_v2_misc(vcpu: *mut kvm_vcpu, addr: gpa_t, _len: u32) -> c_ulong {
    let vgic = &(*(*vcpu).kvm).arch.vgic;
    let value: u32;
    match addr & 0x0c {
        GIC_DIST_CTRL => value = if vgic.enabled { GICD_ENABLE } else { 0 },
        GIC_DIST_CTR => {
            value = vgic.nr_spis + VGIC_NR_PRIVATE_IRQS;
            value = (value >> 5) - 1;
            value |= (atomic_read(&(*vcpu).kvm.online_vcpus) - 1) << 5;
        }
        GIC_DIST_IIDR => {
            value = (PRODUCT_ID_KVM << GICD_IIDR_PRODUCT_ID_SHIFT)
                | (vgic.implementation_rev << GICD_IIDR_REVISION_SHIFT)
                | (IMPLEMENTER_ARM << GICD_IIDR_IMPLEMENTER_SHIFT);
        }
        _ => return 0,
    }
    value as c_ulong
}

unsafe fn vgic_mmio_write_v2_misc(vcpu: *mut kvm_vcpu, addr: gpa_t, _len: u32, val: c_ulong) {
    let dist = &mut (*(*vcpu).kvm).arch.vgic;
    let was_enabled = dist.enabled;
    match addr & 0x0c {
        GIC_DIST_CTRL => {
            dist.enabled = (val & GICD_ENABLE as c_ulong) != 0;
            if !was_enabled && dist.enabled { vgic_kick_vcpus((*vcpu).kvm); }
        }
        GIC_DIST_CTR | GIC_DIST_IIDR => return,
        _ => {}
    }
}

unsafe fn vgic_mmio_uaccess_write_v2_misc(vcpu: *mut kvm_vcpu, addr: gpa_t, len: u32, val: c_ulong) -> c_int {
    let dist = &mut (*(*vcpu).kvm).arch.vgic;
    let mut reg: u32;
    if addr & 0x0c == GIC_DIST_IIDR {
        reg = vgic_mmio_read_v2_misc(vcpu, addr, len) as u32;
        if ((reg ^ val as u32) & !GICD_IIDR_REVISION_MASK) != 0 { return -EINVAL; }
        reg = FIELD_GET(GICD_IIDR_REVISION_MASK, val as u32);
        match reg {
            KVM_VGIC_IMP_REV_2 | KVM_VGIC_IMP_REV_3 => {
                (*vcpu).kvm.arch.vgic.v2_groups_user_writable = true;
                dist.implementation_rev = reg;
                return 0;
            }
            _ => return -EINVAL,
        }
    }
    vgic_mmio_write_v2_misc(vcpu, addr, len, val);
    0
}

unsafe fn vgic_mmio_uaccess_write_v2_group(vcpu: *mut kvm_vcpu, addr: gpa_t, len: u32, val: c_ulong) -> c_int {
    if (*vcpu).kvm.arch.vgic.v2_groups_user_writable { vgic_mmio_write_group(vcpu, addr, len, val); }
    0
}

unsafe fn vgic_mmio_write_sgir(source_vcpu: *mut kvm_vcpu, _addr: gpa_t, _len: u32, val: c_ulong) {
    let nr_vcpus = atomic_read(&(*source_vcpu).kvm.online_vcpus);
    let intid = val & 0xf;
    let mut targets = (val >> 16) & 0xff;
    let mode = (val >> 24) & 0x03;
    match mode {
        0x0 => {}
        0x1 => { targets = (1u32 << nr_vcpus) - 1; targets &= !(1u32 << (*source_vcpu).vcpu_id); }
        0x2 => targets = 1u32 << (*source_vcpu).vcpu_id,
        0x3 => return,
        _ => {}
    }
    let mut c = 0;
    let mut vcpu: *mut kvm_vcpu = core::ptr::null_mut();
    kvm_for_each_vcpu!(c, vcpu, (*source_vcpu).kvm, {
        if targets & (1u32 << c) == 0 { continue; }
        let irq = vgic_get_vcpu_irq(vcpu, intid as u32);
        let mut flags = 0;
        raw_spin_lock_irqsave(&(*irq).irq_lock, &mut flags);
        (*irq).pending_latch = true;
        (*irq).source |= 1u32 << (*source_vcpu).vcpu_id;
        vgic_queue_irq_unlock((*source_vcpu).kvm, irq, flags);
        vgic_put_irq((*source_vcpu).kvm, irq);
    });
}

unsafe fn vgic_mmio_read_target(vcpu: *mut kvm_vcpu, addr: gpa_t, len: u32) -> c_ulong {
    let intid = VGIC_ADDR_TO_INTID(addr, 8); let mut val: u64 = 0;
    for i in 0..len { let irq = vgic_get_vcpu_irq(vcpu, intid + i); val |= ((*irq).targets as u64) << (i * 8); vgic_put_irq((*vcpu).kvm, irq); }
    val as c_ulong
}

unsafe fn vgic_mmio_write_target(vcpu: *mut kvm_vcpu, addr: gpa_t, len: u32, val: c_ulong) {
    let intid = VGIC_ADDR_TO_INTID(addr, 8); let cpu_mask = GENMASK(atomic_read(&(*vcpu).kvm.online_vcpus) - 1, 0);
    if intid < VGIC_NR_PRIVATE_IRQS { return; }
    for i in 0..len { let irq = vgic_get_irq((*vcpu).kvm, intid + i); let mut flags = 0; raw_spin_lock_irqsave(&(*irq).irq_lock, &mut flags); (*irq).targets = ((val >> (i * 8)) & cpu_mask as c_ulong) as u8; let target = if (*irq).targets != 0 { __ffs((*irq).targets as u32) } else { 0 }; (*irq).target_vcpu = kvm_get_vcpu((*vcpu).kvm, target); raw_spin_unlock_irqrestore(&(*irq).irq_lock, flags); vgic_put_irq((*vcpu).kvm, irq); }
}

unsafe fn vgic_mmio_read_sgipend(vcpu: *mut kvm_vcpu, addr: gpa_t, len: u32) -> c_ulong { let intid = addr & 0x0f; let mut val=0u64; for i in 0..len { let irq=vgic_get_vcpu_irq(vcpu,intid+i); val|=((*irq).source as u64)<<(i*8); vgic_put_irq((*vcpu).kvm,irq); } val as c_ulong }
unsafe fn vgic_mmio_write_sgipendc(vcpu:*mut kvm_vcpu,addr:gpa_t,len:u32,val:c_ulong){let intid=addr&0xf;for i in 0..len{let irq=vgic_get_vcpu_irq(vcpu,intid+i);let mut f=0;raw_spin_lock_irqsave(&(*irq).irq_lock,&mut f);(*irq).source&=!((val>>(i*8))&0xff) as u32;if (*irq).source==0{(*irq).pending_latch=false;}raw_spin_unlock_irqrestore(&(*irq).irq_lock,f);vgic_put_irq((*vcpu).kvm,irq);}}
unsafe fn vgic_mmio_write_sgipends(vcpu:*mut kvm_vcpu,addr:gpa_t,len:u32,val:c_ulong){let intid=addr&0xf;for i in 0..len{let irq=vgic_get_vcpu_irq(vcpu,intid+i);let mut f=0;raw_spin_lock_irqsave(&(*irq).irq_lock,&mut f);(*irq).source|=((val>>(i*8))&0xff) as u32;if (*irq).source{(*irq).pending_latch=true;vgic_queue_irq_unlock((*vcpu).kvm,irq,f);}else{raw_spin_unlock_irqrestore(&(*irq).irq_lock,f);}vgic_put_irq((*vcpu).kvm,irq);}}

// Userland-only CPU interface accessors; the remaining register helpers and descriptors are external.
unsafe fn vgic_mmio_read_vcpuif(vcpu:*mut kvm_vcpu,addr:gpa_t,_len:u32)->c_ulong{let mut vmcr=vgic_vmcr::default();vgic_get_vmcr(vcpu,&mut vmcr);let val=match addr&0xff{GIC_CPU_CTRL=>(vmcr.grpen0<<GIC_CPU_CTRL_EnableGrp0_SHIFT)|(vmcr.grpen1<<GIC_CPU_CTRL_EnableGrp1_SHIFT)|(vmcr.ackctl<<GIC_CPU_CTRL_AckCtl_SHIFT)|(vmcr.fiqen<<GIC_CPU_CTRL_FIQEn_SHIFT)|(vmcr.cbpr<<GIC_CPU_CTRL_CBPR_SHIFT)|(vmcr.eoim<<GIC_CPU_CTRL_EOImodeNS_SHIFT),GIC_CPU_PRIMASK=>(vmcr.pmr&GICV_PMR_PRIORITY_MASK)>>GICV_PMR_PRIORITY_SHIFT,GIC_CPU_BINPOINT=>vmcr.bpr,GIC_CPU_ALIAS_BINPOINT=>vmcr.abpr,GIC_CPU_IDENT=>(PRODUCT_ID_KVM<<20)|(GICC_ARCH_VERSION_V2<<16)|IMPLEMENTER_ARM,_=>return 0};val as c_ulong}
unsafe fn vgic_mmio_write_vcpuif(vcpu:*mut kvm_vcpu,addr:gpa_t,_len:u32,val:c_ulong){let mut vmcr=vgic_vmcr::default();vgic_get_vmcr(vcpu,&mut vmcr);match addr&0xff{GIC_CPU_CTRL=>{vmcr.grpen0=!!(val&GIC_CPU_CTRL_EnableGrp0 as c_ulong);vmcr.grpen1=!!(val&GIC_CPU_CTRL_EnableGrp1 as c_ulong);vmcr.ackctl=!!(val&GIC_CPU_CTRL_AckCtl as c_ulong);vmcr.fiqen=!!(val&GIC_CPU_CTRL_FIQEn as c_ulong);vmcr.cbpr=!!(val&GIC_CPU_CTRL_CBPR as c_ulong);vmcr.eoim=!!(val&GIC_CPU_CTRL_EOImodeNS as c_ulong);},GIC_CPU_PRIMASK=>vmcr.pmr=((val<<GICV_PMR_PRIORITY_SHIFT)&GICV_PMR_PRIORITY_MASK) as u32,GIC_CPU_BINPOINT=>vmcr.bpr=val as u32,GIC_CPU_ALIAS_BINPOINT=>vmcr.abpr=val as u32,_=>{}}vgic_set_vmcr(vcpu,&vmcr);}
unsafe fn vgic_mmio_write_dir(vcpu:*mut kvm_vcpu,_addr:gpa_t,_len:u32,val:c_ulong){if kvm_vgic_global_state.type_==VGIC_V2{vgic_v2_deactivate(vcpu,val)}else{vgic_v3_deactivate(vcpu,val)}}
unsafe fn vgic_mmio_read_apr(vcpu:*mut kvm_vcpu,addr:gpa_t,_len:u32)->c_ulong{let mut n=((addr>>2)&3) as u32;if kvm_vgic_global_state.type_==VGIC_V2{if n!=0{return 0}return (*vcpu).arch.vgic_cpu.vgic_v2.vgic_apr as c_ulong}if n>vgic_v3_max_apr_idx(vcpu){return 0}n=array_index_nospec(n,4);(*vcpu).arch.vgic_cpu.vgic_v3.vgic_ap1r[n as usize] as c_ulong}
unsafe fn vgic_mmio_write_apr(vcpu:*mut kvm_vcpu,addr:gpa_t,_len:u32,val:c_ulong){let mut n=((addr>>2)&3) as u32;if kvm_vgic_global_state.type_==VGIC_V2{if n==0{(*vcpu).arch.vgic_cpu.vgic_v2.vgic_apr=val as u32}return}if n>vgic_v3_max_apr_idx(vcpu){return}n=array_index_nospec(n,4);(*vcpu).arch.vgic_cpu.vgic_v3.vgic_ap1r[n as usize]=val as u32;}

// REGISTER_DESC_* entries preserve the source register tables and depend on external definitions.
static vgic_v2_dist_registers: &[vgic_register_region] = &[
    REGISTER_DESC_WITH_LENGTH_UACCESS!(GIC_DIST_CTRL,vgic_mmio_read_v2_misc,vgic_mmio_write_v2_misc,None,vgic_mmio_uaccess_write_v2_misc,12,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_BITS_PER_IRQ!(GIC_DIST_IGROUP,vgic_mmio_read_group,vgic_mmio_write_group,None,vgic_mmio_uaccess_write_v2_group,1,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_BITS_PER_IRQ!(GIC_DIST_ENABLE_SET,vgic_mmio_read_enable,vgic_mmio_write_senable,None,vgic_uaccess_write_senable,1,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_BITS_PER_IRQ!(GIC_DIST_ENABLE_CLEAR,vgic_mmio_read_enable,vgic_mmio_write_cenable,None,vgic_uaccess_write_cenable,1,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_BITS_PER_IRQ!(GIC_DIST_PENDING_SET,vgic_mmio_read_pending,vgic_mmio_write_spending,vgic_uaccess_read_pending,vgic_uaccess_write_spending,1,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_BITS_PER_IRQ!(GIC_DIST_PENDING_CLEAR,vgic_mmio_read_pending,vgic_mmio_write_cpending,vgic_uaccess_read_pending,vgic_uaccess_write_cpending,1,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_BITS_PER_IRQ!(GIC_DIST_ACTIVE_SET,vgic_mmio_read_active,vgic_mmio_write_sactive,vgic_uaccess_read_active,vgic_mmio_uaccess_write_sactive,1,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_BITS_PER_IRQ!(GIC_DIST_ACTIVE_CLEAR,vgic_mmio_read_active,vgic_mmio_write_cactive,vgic_uaccess_read_active,vgic_mmio_uaccess_write_sactive,1,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_BITS_PER_IRQ!(GIC_DIST_PRI,vgic_mmio_read_priority,vgic_mmio_write_priority,None,None,8,VGIC_ACCESS_32bit|VGIC_ACCESS_8bit),
    REGISTER_DESC_WITH_BITS_PER_IRQ!(GIC_DIST_TARGET,vgic_mmio_read_target,vgic_mmio_write_target,None,None,8,VGIC_ACCESS_32bit|VGIC_ACCESS_8bit),
    REGISTER_DESC_WITH_BITS_PER_IRQ!(GIC_DIST_CONFIG,vgic_mmio_read_config,vgic_mmio_write_config,None,None,2,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_LENGTH!(GIC_DIST_SOFTINT,vgic_mmio_read_raz,vgic_mmio_write_sgir,4,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_LENGTH!(GIC_DIST_SGI_PENDING_CLEAR,vgic_mmio_read_sgipend,vgic_mmio_write_sgipendc,16,VGIC_ACCESS_32bit|VGIC_ACCESS_8bit),
    REGISTER_DESC_WITH_LENGTH!(GIC_DIST_SGI_PENDING_SET,vgic_mmio_read_sgipend,vgic_mmio_write_sgipends,16,VGIC_ACCESS_32bit|VGIC_ACCESS_8bit),
];
static vgic_v2_cpu_registers: &[vgic_register_region] = &[
    REGISTER_DESC_WITH_LENGTH!(GIC_CPU_CTRL,vgic_mmio_read_vcpuif,vgic_mmio_write_vcpuif,4,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_LENGTH!(GIC_CPU_PRIMASK,vgic_mmio_read_vcpuif,vgic_mmio_write_vcpuif,4,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_LENGTH!(GIC_CPU_BINPOINT,vgic_mmio_read_vcpuif,vgic_mmio_write_vcpuif,4,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_LENGTH!(GIC_CPU_ALIAS_BINPOINT,vgic_mmio_read_vcpuif,vgic_mmio_write_vcpuif,4,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_LENGTH!(GIC_CPU_ACTIVEPRIO,vgic_mmio_read_apr,vgic_mmio_write_apr,16,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_LENGTH!(GIC_CPU_IDENT,vgic_mmio_read_vcpuif,vgic_mmio_write_vcpuif,4,VGIC_ACCESS_32bit),
    REGISTER_DESC_WITH_LENGTH_UACCESS!(GIC_CPU_DEACTIVATE,vgic_mmio_read_raz,vgic_mmio_write_dir,vgic_mmio_read_raz,vgic_mmio_uaccess_write_wi,4,VGIC_ACCESS_32bit),
];

unsafe fn vgic_v2_init_dist_iodev(dev:*mut vgic_io_device)->u32{(*dev).regions=vgic_v2_dist_registers.as_ptr();(*dev).nr_regions=vgic_v2_dist_registers.len();kvm_iodevice_init(&mut (*dev).dev,&kvm_io_gic_ops);SZ_4K}
unsafe fn vgic_v2_init_cpuif_iodev(dev:*mut vgic_io_device)->u32{(*dev).regions=vgic_v2_cpu_registers.as_ptr();(*dev).nr_regions=vgic_v2_cpu_registers.len();kvm_iodevice_init(&mut (*dev).dev,&kvm_io_gic_ops);KVM_VGIC_V2_CPU_SIZE}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
