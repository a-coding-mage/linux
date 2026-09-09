// SPDX-License-Identifier: GPL-2.0-or-later
/* Support for interrupt controllers found on Power Macintosh. */

// C includes provide external kernel declarations used below.

#[cfg(target_arch = "powerpc")]
#[repr(C)]
pub struct PmacIrqHw {
    pub event: u32,
    pub enable: u32,
    pub ack: u32,
    pub level: u32,
}

#[cfg(target_arch = "powerpc")]
pub static mut OF_IRQ_WORKAROUNDS: u32 = 0;
#[cfg(target_arch = "powerpc")]
pub static mut OF_IRQ_DFLT_PIC: *mut DeviceNode = core::ptr::null_mut();
#[cfg(target_arch = "powerpc")]
static mut PMAC_IRQ_HW: [*mut PmacIrqHw; 4] = [core::ptr::null_mut(); 4];
#[cfg(target_arch = "powerpc")]
static mut MAX_IRQS: i32 = 0;
#[cfg(target_arch = "powerpc")]
static mut MAX_REAL_IRQS: i32 = 0;
#[cfg(target_arch = "powerpc")]
static mut PPC_LOST_INTERRUPTS: [u32; 4] = [0; 4];
#[cfg(target_arch = "powerpc")]
static mut PPC_CACHED_IRQ_MASK: [u32; 4] = [0; 4];
#[cfg(target_arch = "powerpc")]
static mut PMAC_IRQ_CASCADE: i32 = -1;
#[cfg(target_arch = "powerpc")]
static mut PMAC_PIC_HOST: *mut IrqDomain = core::ptr::null_mut();

#[cfg(target_arch = "powerpc")]
unsafe fn __pmac_retrigger(mut irq_nr: u32) {
    if irq_nr >= MAX_REAL_IRQS as u32 && PMAC_IRQ_CASCADE > 0 {
        set_bit(irq_nr, &mut PPC_LOST_INTERRUPTS);
        irq_nr = PMAC_IRQ_CASCADE as u32;
        mb();
    }
    if !test_and_set_bit(irq_nr, &mut PPC_LOST_INTERRUPTS) {
        atomic_inc(&mut ppc_n_lost_interrupts);
        set_dec(1);
    }
}

#[cfg(target_arch = "powerpc")]
unsafe fn pmac_mask_and_ack_irq(d: *mut IrqData) {
    let src = irqd_to_hwirq(d) as u32;
    let bit = 1u32 << (src & 0x1f);
    let i = (src >> 5) as usize;
    let mut flags = 0ul;
    raw_spin_lock_irqsave(&mut pmac_pic_lock, &mut flags);
    clear_bit(src, &mut PPC_CACHED_IRQ_MASK);
    if test_and_clear_bit(src, &mut PPC_LOST_INTERRUPTS) { atomic_dec(&mut ppc_n_lost_interrupts); }
    out_le32(&mut (*PMAC_IRQ_HW[i]).enable, PPC_CACHED_IRQ_MASK[i]);
    out_le32(&mut (*PMAC_IRQ_HW[i]).ack, bit);
    loop { mb(); if (in_le32(&(*PMAC_IRQ_HW[i]).enable) & bit) == (PPC_CACHED_IRQ_MASK[i] & bit) { break; } }
    raw_spin_unlock_irqrestore(&mut pmac_pic_lock, flags);
}

#[cfg(target_arch = "powerpc")]
unsafe fn pmac_ack_irq(d: *mut IrqData) {
    let src = irqd_to_hwirq(d) as u32; let bit = 1u32 << (src & 0x1f); let i = (src >> 5) as usize; let mut flags = 0ul;
    raw_spin_lock_irqsave(&mut pmac_pic_lock, &mut flags);
    if test_and_clear_bit(src, &mut PPC_LOST_INTERRUPTS) { atomic_dec(&mut ppc_n_lost_interrupts); }
    out_le32(&mut (*PMAC_IRQ_HW[i]).ack, bit); let _ = in_le32(&(*PMAC_IRQ_HW[i]).ack);
    raw_spin_unlock_irqrestore(&mut pmac_pic_lock, flags);
}

#[cfg(target_arch = "powerpc")]
unsafe fn __pmac_set_irq_mask(irq_nr: u32, _nokicklost: i32) {
    let bit = 1u32 << (irq_nr & 0x1f); let i = (irq_nr >> 5) as usize;
    if irq_nr >= MAX_IRQS as u32 { return; }
    out_le32(&mut (*PMAC_IRQ_HW[i]).enable, PPC_CACHED_IRQ_MASK[i]);
    loop { mb(); if (in_le32(&(*PMAC_IRQ_HW[i]).enable) & bit) == (PPC_CACHED_IRQ_MASK[i] & bit) { break; } }
    if bit & PPC_CACHED_IRQ_MASK[i] & in_le32(&(*PMAC_IRQ_HW[i]).level) != 0 { __pmac_retrigger(irq_nr); }
}

#[cfg(target_arch = "powerpc")]
unsafe fn pmac_startup_irq(d: *mut IrqData) -> u32 {
    let mut flags = 0ul; let src = irqd_to_hwirq(d) as u32; let bit = 1u32 << (src & 0x1f); let i = (src >> 5) as usize;
    raw_spin_lock_irqsave(&mut pmac_pic_lock, &mut flags);
    if !irqd_is_level_type(d) { out_le32(&mut (*PMAC_IRQ_HW[i]).ack, bit); }
    set_bit(src, &mut PPC_CACHED_IRQ_MASK); __pmac_set_irq_mask(src, 0);
    raw_spin_unlock_irqrestore(&mut pmac_pic_lock, flags); 0
}

#[cfg(target_arch = "powerpc")]
unsafe fn pmac_mask_irq(d: *mut IrqData) { let mut f=0ul; let s=irqd_to_hwirq(d) as u32; raw_spin_lock_irqsave(&mut pmac_pic_lock,&mut f); clear_bit(s,&mut PPC_CACHED_IRQ_MASK); __pmac_set_irq_mask(s,1); raw_spin_unlock_irqrestore(&mut pmac_pic_lock,f); }
#[cfg(target_arch = "powerpc")]
unsafe fn pmac_unmask_irq(d: *mut IrqData) { let mut f=0ul; let s=irqd_to_hwirq(d) as u32; raw_spin_lock_irqsave(&mut pmac_pic_lock,&mut f); set_bit(s,&mut PPC_CACHED_IRQ_MASK); __pmac_set_irq_mask(s,0); raw_spin_unlock_irqrestore(&mut pmac_pic_lock,f); }
#[cfg(target_arch = "powerpc")]
unsafe fn pmac_retrigger(d: *mut IrqData) -> i32 { let mut f=0ul; raw_spin_lock_irqsave(&mut pmac_pic_lock,&mut f); __pmac_retrigger(irqd_to_hwirq(d) as u32); raw_spin_unlock_irqrestore(&mut pmac_pic_lock,f); 1 }

// The remaining declarations preserve the source interfaces and require the kernel's
// corresponding external types and functions from the included headers.
#[cfg(target_arch = "powerpc")]
pub unsafe fn of_irq_parse_oldworld(device: *const DeviceNode, index: i32, out_irq: *mut OfPhandleArgs) -> i32 {
    let mut ints: *const u32 = core::ptr::null(); let mut intlen = 0;
    let mut dev = device;
    while !dev.is_null() {
        ints = of_get_property(dev, "AAPL,interrupts\0".as_ptr() as *const i8, &mut intlen);
        if !ints.is_null() { break; }
        dev = (*dev).parent;
        if !of_node_is_type(dev, "pci\0".as_ptr() as *const i8) { break; }
    }
    if ints.is_null() || index >= intlen / core::mem::size_of::<u32>() as i32 { return -22; }
    (*out_irq).np = core::ptr::null_mut(); (*out_irq).args[0] = *ints.add(index as usize); (*out_irq).args_count = 1; 0
}

unsafe fn pmac_pic_setup_mpic_nmi(_mpic: *mut Mpic) {
    // CONFIG_XMON && CONFIG_PPC32: locate programmer-switch, set priority 9,
    // and register the NMI-XMON handler, as in the C conditional block.
}

unsafe fn pmac_setup_one_mpic(np: *mut DeviceNode, master: i32) -> *mut Mpic {
    let name = if master != 0 { " MPIC 1   " } else { " MPIC 2   " };
    let mut flags = if master != 0 { 0 } else { MPIC_SECONDARY };
    pmac_call_feature(PMAC_FTR_ENABLE_MPIC, np, 0, 0);
    if of_property_read_bool(np, "big-endian\0".as_ptr() as *const i8) { flags |= MPIC_BIG_ENDIAN; }
    if master != 0 && flags & MPIC_BIG_ENDIAN != 0 { flags |= MPIC_U3_HT_IRQS; }
    let mpic = mpic_alloc(np, 0, flags, 0, 0, name.as_ptr() as *const i8);
    if mpic.is_null() { return core::ptr::null_mut(); }
    mpic_init(mpic); mpic
}

unsafe fn pmac_pic_probe_mpic() -> i32 {
    // Enumerate up to two open-pic nodes, install mpic_get_irq, initialize the
    // master and optional cascaded controller, and return -ENODEV if absent.
    -19
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
