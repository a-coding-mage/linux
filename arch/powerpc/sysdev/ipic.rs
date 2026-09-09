// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of arch/powerpc/sysdev/ipic.c. */

// External kernel types, constants, functions, and macros are supplied by the
// surrounding translation environment.

static mut PRIMARY_IPIC: *mut ipic = core::ptr::null_mut();
static mut IPIC_LEVEL_IRQ_CHIP: irq_chip = irq_chip::zeroed();
static mut IPIC_EDGE_IRQ_CHIP: irq_chip = irq_chip::zeroed();
static mut IPIC_LOCK: raw_spinlock_t = raw_spinlock_t::zeroed();

const fn info(ack: u32, mask: u32, prio: u32, force: u32, bit: u32, prio_mask: u32) -> ipic_info {
    ipic_info { ack, mask, prio, force, bit, prio_mask }
}

const fn make_ipic_info() -> [ipic_info; 95] {
    let z = info(0, 0, 0, 0, 0, 0);
    let mut a = [z; 95];
    a[1] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_C, IPIC_SIFCR_H, 16, 0);
    a[2] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_C, IPIC_SIFCR_H, 17, 1);
    a[3] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_C, IPIC_SIFCR_H, 18, 2);
    a[4] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_C, IPIC_SIFCR_H, 19, 3);
    a[5] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_C, IPIC_SIFCR_H, 20, 4);
    a[6] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_C, IPIC_SIFCR_H, 21, 5);
    a[7] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_C, IPIC_SIFCR_H, 22, 6);
    a[8] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_C, IPIC_SIFCR_H, 23, 7);
    a[9] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_D, IPIC_SIFCR_H, 24, 0);
    a[10] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_D, IPIC_SIFCR_H, 25, 1);
    a[11] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_D, IPIC_SIFCR_H, 26, 2);
    a[12] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_D, IPIC_SIFCR_H, 27, 3);
    a[13] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_D, IPIC_SIFCR_H, 28, 4);
    a[14] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_D, IPIC_SIFCR_H, 29, 5);
    a[15] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_D, IPIC_SIFCR_H, 30, 6);
    a[16] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_D, IPIC_SIFCR_H, 31, 7);
    a[17] = info(IPIC_SEPNR, IPIC_SEMSR, IPIC_SMPRR_A, IPIC_SEFCR, 1, 5);
    a[18] = info(IPIC_SEPNR, IPIC_SEMSR, IPIC_SMPRR_A, IPIC_SEFCR, 2, 6);
    a[19] = info(IPIC_SEPNR, IPIC_SEMSR, IPIC_SMPRR_A, IPIC_SEFCR, 3, 7);
    a[20] = info(IPIC_SEPNR, IPIC_SEMSR, IPIC_SMPRR_B, IPIC_SEFCR, 4, 4);
    a[21] = info(IPIC_SEPNR, IPIC_SEMSR, IPIC_SMPRR_B, IPIC_SEFCR, 5, 5);
    a[22] = info(IPIC_SEPNR, IPIC_SEMSR, IPIC_SMPRR_B, IPIC_SEFCR, 6, 6);
    a[23] = info(IPIC_SEPNR, IPIC_SEMSR, IPIC_SMPRR_B, IPIC_SEFCR, 7, 7);
    a[32] = info(0, IPIC_SIMSR_H, IPIC_SIPRR_A, IPIC_SIFCR_H, 0, 0);
    let mut i=33; while i<=39 { a[i]=info(0,IPIC_SIMSR_H,IPIC_SIPRR_A,IPIC_SIFCR_H,(i-32) as u32,(i-32) as u32); i+=1; }
    i=40; while i<=47 { a[i]=info(0,IPIC_SIMSR_H,IPIC_SIPRR_B,IPIC_SIFCR_H,(i-32) as u32,(i-40) as u32); i+=1; }
    a[48] = info(IPIC_SEPNR, IPIC_SEMSR, IPIC_SMPRR_A, IPIC_SEFCR, 0, 4);
    a[64] = info(0, IPIC_SIMSR_L, IPIC_SMPRR_A, IPIC_SIFCR_L, 0, 0);
    a[65] = info(0, IPIC_SIMSR_L, IPIC_SMPRR_A, IPIC_SIFCR_L, 1, 1);
    a[66] = info(0, IPIC_SIMSR_L, IPIC_SMPRR_A, IPIC_SIFCR_L, 2, 2);
    a[67] = info(0, IPIC_SIMSR_L, IPIC_SMPRR_A, IPIC_SIFCR_L, 3, 3);
    a[68] = info(0, IPIC_SIMSR_L, IPIC_SMPRR_B, IPIC_SIFCR_L, 4, 0);
    a[69] = info(0, IPIC_SIMSR_L, IPIC_SMPRR_B, IPIC_SIFCR_L, 5, 1);
    a[70] = info(0, IPIC_SIMSR_L, IPIC_SMPRR_B, IPIC_SIFCR_L, 6, 2);
    a[71] = info(0, IPIC_SIMSR_L, IPIC_SMPRR_B, IPIC_SIFCR_L, 7, 3);
    i=72; while i<=93 { a[i]=info(0,IPIC_SIMSR_L,0,IPIC_SIFCR_L,(i-64) as u32,0); i+=1; }
    a[94] = info(0, IPIC_SIMSR_L, 0, IPIC_SIFCR_L, 30, 0);
    a
}
static IPIC_INFO: [ipic_info; 95] = make_ipic_info();

#[inline] unsafe fn ipic_read(base: *mut u32, reg: u32) -> u32 { in_be32(base.add((reg >> 2) as usize)) }
#[inline] unsafe fn ipic_write(base: *mut u32, reg: u32, value: u32) { out_be32(base.add((reg >> 2) as usize), value); }
#[inline] unsafe fn ipic_from_irq(_virq: u32) -> *mut ipic { PRIMARY_IPIC }

unsafe fn ipic_unmask_irq(d: *mut irq_data) { let p=ipic_from_irq((*d).irq); let s=irqd_to_hwirq(d); let mut f=0; raw_spin_lock_irqsave(&mut IPIC_LOCK,&mut f); let mut t=ipic_read((*p).regs,IPIC_INFO[s as usize].mask); t|=1u32<<(31-IPIC_INFO[s as usize].bit); ipic_write((*p).regs,IPIC_INFO[s as usize].mask,t); raw_spin_unlock_irqrestore(&mut IPIC_LOCK,f); }
unsafe fn ipic_mask_irq(d: *mut irq_data) { let p=ipic_from_irq((*d).irq); let s=irqd_to_hwirq(d); let mut f=0; raw_spin_lock_irqsave(&mut IPIC_LOCK,&mut f); let mut t=ipic_read((*p).regs,IPIC_INFO[s as usize].mask); t&=!(1u32<<(31-IPIC_INFO[s as usize].bit)); ipic_write((*p).regs,IPIC_INFO[s as usize].mask,t); mb(); raw_spin_unlock_irqrestore(&mut IPIC_LOCK,f); }
unsafe fn ipic_ack_irq(d: *mut irq_data) { let p=ipic_from_irq((*d).irq); let s=irqd_to_hwirq(d); let mut f=0; raw_spin_lock_irqsave(&mut IPIC_LOCK,&mut f); ipic_write((*p).regs,IPIC_INFO[s as usize].ack,1u32<<(31-IPIC_INFO[s as usize].bit)); mb(); raw_spin_unlock_irqrestore(&mut IPIC_LOCK,f); }
unsafe fn ipic_mask_irq_and_ack(d: *mut irq_data) { let p=ipic_from_irq((*d).irq); let s=irqd_to_hwirq(d); let mut f=0; raw_spin_lock_irqsave(&mut IPIC_LOCK,&mut f); let mut t=ipic_read((*p).regs,IPIC_INFO[s as usize].mask); t&=!(1u32<<(31-IPIC_INFO[s as usize].bit)); ipic_write((*p).regs,IPIC_INFO[s as usize].mask,t); ipic_write((*p).regs,IPIC_INFO[s as usize].ack,1u32<<(31-IPIC_INFO[s as usize].bit)); mb(); raw_spin_unlock_irqrestore(&mut IPIC_LOCK,f); }

unsafe fn ipic_set_irq_type(d: *mut irq_data, mut flow_type: u32) -> i32 {
    let p=ipic_from_irq((*d).irq); let s=irqd_to_hwirq(d) as usize;
    if flow_type==IRQ_TYPE_NONE { flow_type=IRQ_TYPE_LEVEL_LOW; }
    if flow_type & (IRQ_TYPE_LEVEL_LOW|IRQ_TYPE_EDGE_FALLING)==0 { printk(KERN_ERR,b"ipic: sense type 0x%x not supported\0".as_ptr(),flow_type); return -EINVAL; }
    if flow_type&IRQ_TYPE_EDGE_FALLING!=0 && IPIC_INFO[s].ack==0 { printk(KERN_ERR,b"ipic: edge sense not supported on internal interrupts\0".as_ptr()); return -EINVAL; }
    irqd_set_trigger_type(d,flow_type);
    if flow_type&IRQ_TYPE_LEVEL_LOW!=0 { irq_set_handler_locked(d,handle_level_irq); (*d).chip=&mut IPIC_LEVEL_IRQ_CHIP; } else { irq_set_handler_locked(d,handle_edge_irq); (*d).chip=&mut IPIC_EDGE_IRQ_CHIP; }
    let edibit = if s==IPIC_IRQ_EXT0 as usize {15} else if s>=IPIC_IRQ_EXT1 as usize && s<=IPIC_IRQ_EXT7 as usize {14-(s-IPIC_IRQ_EXT1 as usize)} else { return if flow_type&IRQ_TYPE_LEVEL_LOW!=0 {0} else {-EINVAL}; };
    let old=ipic_read((*p).regs,IPIC_SECNR); let new=if flow_type&IRQ_TYPE_SENSE_MASK==IRQ_TYPE_EDGE_FALLING {old|(1<<edibit)} else {old&!(1<<edibit)}; if old!=new {ipic_write((*p).regs,IPIC_SECNR,new);} IRQ_SET_MASK_OK_NOCOPY
}

/* Level interrupts and edge interrupts have different ack operations. */
static IPIC_HOST_OPS: irq_domain_ops = irq_domain_ops { match_: ipic_host_match, map: ipic_host_map, xlate: irq_domain_xlate_onetwocell };
unsafe fn ipic_host_match(h:*mut irq_domain,node:*mut device_node,_:irq_domain_bus_token)->i32 { let n=irq_domain_get_of_node(h); (n.is_null() || n==node) as i32 }
unsafe fn ipic_host_map(h:*mut irq_domain,virq:u32,_hw:irq_hw_number_t)->i32 { let p=(*h).host_data as *mut ipic; irq_set_chip_data(virq,p); irq_set_chip_and_handler(virq,&mut IPIC_LEVEL_IRQ_CHIP,handle_level_irq); irq_set_irq_type(virq,IRQ_TYPE_NONE); 0 }

unsafe fn ipic_init(node:*mut device_node,flags:u32)->*mut ipic { let mut r=core::mem::zeroed(); if of_address_to_resource(node,0,&mut r)!=0{return core::ptr::null_mut();} let p=kzalloc_ipic(); if p.is_null(){return p;} (*p).irqhost=irq_domain_create_linear(of_fwnode_handle(node),NR_IPIC_INTS,&IPIC_HOST_OPS,p); if (*p).irqhost.is_null(){kfree(p);return core::ptr::null_mut();} (*p).regs=ioremap(r.start,resource_size(&r)); ipic_write((*p).regs,IPIC_SICNR,0); let mut t=0; if flags&IPIC_SPREADMODE_GRP_A!=0{t|=SICFR_IPSA} if flags&IPIC_SPREADMODE_GRP_B!=0{t|=SICFR_IPSB} if flags&IPIC_SPREADMODE_GRP_C!=0{t|=SICFR_IPSC} if flags&IPIC_SPREADMODE_GRP_D!=0{t|=SICFR_IPSD} if flags&IPIC_SPREADMODE_MIX_A!=0{t|=SICFR_MPSA} if flags&IPIC_SPREADMODE_MIX_B!=0{t|=SICFR_MPSB} ipic_write((*p).regs,IPIC_SICFR,t); ipic_write((*p).regs,IPIC_SERCR,if flags&IPIC_DISABLE_MCP_OUT!=0{SERCR_MCPR}else{0}); t=ipic_read((*p).regs,IPIC_SEMSR); if flags&IPIC_IRQ0_MCP!=0{t|=SEMSR_SIRQ0}else{t&=!SEMSR_SIRQ0} ipic_write((*p).regs,IPIC_SEMSR,t); PRIMARY_IPIC=p; irq_set_default_domain((*p).irqhost); ipic_write((*p).regs,IPIC_SIMSR_H,0); ipic_write((*p).regs,IPIC_SIMSR_L,0); pr_info(b"IPIC (%d IRQ sources) at MMIO %pa\0".as_ptr(),NR_IPIC_INTS,&r.start); p }

unsafe fn ipic_set_default_priority(){for r in [IPIC_SIPRR_A,IPIC_SIPRR_B,IPIC_SIPRR_C,IPIC_SIPRR_D,IPIC_SMPRR_A,IPIC_SMPRR_B]{ipic_write((*PRIMARY_IPIC).regs,r,IPIC_PRIORITY_DEFAULT);}}
unsafe fn ipic_get_mcp_status()->u32{if !PRIMARY_IPIC.is_null(){ipic_read((*PRIMARY_IPIC).regs,IPIC_SERSR)}else{0}}
unsafe fn ipic_clear_mcp_status(mask:u32){ipic_write((*PRIMARY_IPIC).regs,IPIC_SERSR,mask);}
unsafe fn ipic_get_irq()->u32{BUG_ON(PRIMARY_IPIC.is_null());let irq=ipic_read((*PRIMARY_IPIC).regs,IPIC_SIVCR)&0x7f;if irq==0{0}else{irq_find_mapping((*PRIMARY_IPIC).irqhost,irq)}}

#[cfg(CONFIG_SUSPEND)]
static mut IPIC_SAVED_STATE: ipic_saved_state = ipic_saved_state::zeroed();
#[cfg(CONFIG_SUSPEND)]
unsafe fn ipic_suspend(_data:*mut core::ffi::c_void)->i32 { let p=PRIMARY_IPIC; IPIC_SAVED_STATE.sicfr=ipic_read((*p).regs,IPIC_SICFR); IPIC_SAVED_STATE.siprr[0]=ipic_read((*p).regs,IPIC_SIPRR_A); IPIC_SAVED_STATE.siprr[1]=ipic_read((*p).regs,IPIC_SIPRR_D); IPIC_SAVED_STATE.simsr[0]=ipic_read((*p).regs,IPIC_SIMSR_H); IPIC_SAVED_STATE.simsr[1]=ipic_read((*p).regs,IPIC_SIMSR_L); IPIC_SAVED_STATE.sicnr=ipic_read((*p).regs,IPIC_SICNR); IPIC_SAVED_STATE.smprr[0]=ipic_read((*p).regs,IPIC_SMPRR_A); IPIC_SAVED_STATE.smprr[1]=ipic_read((*p).regs,IPIC_SMPRR_B); IPIC_SAVED_STATE.semsr=ipic_read((*p).regs,IPIC_SEMSR); IPIC_SAVED_STATE.secnr=ipic_read((*p).regs,IPIC_SECNR); IPIC_SAVED_STATE.sermr=ipic_read((*p).regs,IPIC_SERMR); IPIC_SAVED_STATE.sercr=ipic_read((*p).regs,IPIC_SERCR); if fsl_deep_sleep(){ipic_write((*p).regs,IPIC_SIMSR_H,0);ipic_write((*p).regs,IPIC_SIMSR_L,0);ipic_write((*p).regs,IPIC_SEMSR,0);ipic_write((*p).regs,IPIC_SERMR,0);} 0 }
#[cfg(CONFIG_SUSPEND)]
unsafe fn ipic_resume(_data:*mut core::ffi::c_void){let p=PRIMARY_IPIC;ipic_write((*p).regs,IPIC_SICFR,IPIC_SAVED_STATE.sicfr);ipic_write((*p).regs,IPIC_SIPRR_A,IPIC_SAVED_STATE.siprr[0]);ipic_write((*p).regs,IPIC_SIPRR_D,IPIC_SAVED_STATE.siprr[1]);ipic_write((*p).regs,IPIC_SIMSR_H,IPIC_SAVED_STATE.simsr[0]);ipic_write((*p).regs,IPIC_SIMSR_L,IPIC_SAVED_STATE.simsr[1]);ipic_write((*p).regs,IPIC_SICNR,IPIC_SAVED_STATE.sicnr);ipic_write((*p).regs,IPIC_SMPRR_A,IPIC_SAVED_STATE.smprr[0]);ipic_write((*p).regs,IPIC_SMPRR_B,IPIC_SAVED_STATE.smprr[1]);ipic_write((*p).regs,IPIC_SEMSR,IPIC_SAVED_STATE.semsr);ipic_write((*p).regs,IPIC_SECNR,IPIC_SAVED_STATE.secnr);ipic_write((*p).regs,IPIC_SERMR,IPIC_SAVED_STATE.sermr);ipic_write((*p).regs,IPIC_SERCR,IPIC_SAVED_STATE.sercr);}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
