/* SPDX-License-Identifier: GPL-2.0 */

/* C dependency: <asm/io.h> and the kernel types/constants referenced below. */

/* --- register definitions ------------------------------- */

#[inline]
pub unsafe fn econtrol(p: *mut parport) -> u16 { (*p).base_hi.wrapping_add(0x2) }
#[inline]
pub unsafe fn configb(p: *mut parport) -> u16 { (*p).base_hi.wrapping_add(0x1) }
#[inline]
pub unsafe fn configa(p: *mut parport) -> u16 { (*p).base_hi.wrapping_add(0x0) }
#[inline]
pub unsafe fn fifo(p: *mut parport) -> u16 { (*p).base_hi.wrapping_add(0x0) }
#[inline]
pub unsafe fn eppdata(p: *mut parport) -> u16 { (*p).base.wrapping_add(0x4) }
#[inline]
pub unsafe fn eppaddr(p: *mut parport) -> u16 { (*p).base.wrapping_add(0x3) }
#[inline]
pub unsafe fn control(p: *mut parport) -> u16 { (*p).base.wrapping_add(0x2) }
#[inline]
pub unsafe fn status(p: *mut parport) -> u16 { (*p).base.wrapping_add(0x1) }
#[inline]
pub unsafe fn data(p: *mut parport) -> u16 { (*p).base.wrapping_add(0x0) }

#[repr(C)]
pub struct parport_pc_private {
    /* Contents of CTR. */
    pub ctr: u8,
    /* Bitmask of writable CTR bits. */
    pub ctr_writable: u8,
    /* Whether or not there's an ECR. */
    pub ecr: libc::c_int,
    /* Bitmask of writable ECR bits. */
    pub ecr_writable: u8,
    /* Number of PWords that FIFO will hold. */
    pub fifo_depth: libc::c_int,
    /* Number of bytes per portword. */
    pub pword: libc::c_int,
    /* Not used yet. */
    pub readIntrThreshold: libc::c_int,
    pub writeIntrThreshold: libc::c_int,
    /* buffer suitable for DMA, if DMA enabled */
    pub dma_buf: *mut libc::c_char,
    pub dma_handle: dma_addr_t,
    pub list: list_head,
    pub port: *mut parport,
}

#[repr(C)]
pub struct parport_pc_via_data {
    /* ISA PnP IRQ routing register 1 */
    pub via_pci_parport_irq_reg: u8,
    /* ISA PnP DMA request routing register */
    pub via_pci_parport_dma_reg: u8,
    /* Register and value to enable SuperIO configuration access */
    pub via_pci_superio_config_reg: u8,
    pub via_pci_superio_config_data: u8,
    /* SuperIO function register number */
    pub viacfg_function: u8,
    /* parallel port control register number */
    pub viacfg_parport_control: u8,
    /* Parallel port base address register */
    pub viacfg_parport_base: u8,
}

#[inline]
pub unsafe fn parport_pc_write_data(p: *mut parport, d: u8) {
    #[cfg(feature = "DEBUG_PARPORT")]
    printk(KERN_DEBUG "parport_pc_write_data(%p,0x%02x)\n", p, d);
    outb(d, data(p));
}

#[inline]
pub unsafe fn parport_pc_read_data(p: *mut parport) -> u8 {
    let val = inb(data(p));
    #[cfg(feature = "DEBUG_PARPORT")]
    printk(KERN_DEBUG "parport_pc_read_data(%p) = 0x%02x\n", p, val);
    val
}

/* DEBUG_PARPORT controls whether this diagnostic helper is present. */
#[cfg(feature = "DEBUG_PARPORT")]
pub unsafe fn dump_parport_state(str_: *mut libc::c_char, p: *mut parport) {
    /* here's hoping that reading these ports won't side-effect anything underneath */
    let ecr = inb(econtrol(p));
    let mut dcr = inb(control(p));
    let dsr = inb(status(p));
    static ECR_MODES: [&[u8]; 8] = [b"SPP", b"PS2", b"PPFIFO", b"ECP", b"xXx", b"yYy", b"TST", b"CFG"];
    let priv_ = (*(*p).physport).private_data as *const parport_pc_private;

    printk(KERN_DEBUG "*** parport state (%s): ecr=[%s", str_, ECR_MODES[((ecr & 0xe0) >> 5) as usize].as_ptr());
    if ecr & 0x10 != 0 { printk(",nErrIntrEn"); }
    if ecr & 0x08 != 0 { printk(",dmaEn"); }
    if ecr & 0x04 != 0 { printk(",serviceIntr"); }
    if ecr & 0x02 != 0 { printk(",f_full"); }
    if ecr & 0x01 != 0 { printk(",f_empty"); }
    for i in 0..2 {
        printk("]  dcr(%s)=[", if i != 0 { b"soft\0".as_ptr() } else { b"hard\0".as_ptr() });
        dcr = if i != 0 { (*priv_).ctr } else { inb(control(p)) };
        if dcr & 0x20 != 0 { printk!("rev"); } else { printk!("fwd"); }
        if dcr & 0x10 != 0 { printk!(",ackIntEn"); }
        if dcr & 0x08 == 0 { printk!(",N-SELECT-IN"); }
        if dcr & 0x04 != 0 { printk!(",N-INIT"); }
        if dcr & 0x02 == 0 { printk!(",N-AUTOFD"); }
        if dcr & 0x01 == 0 { printk!(",N-STROBE"); }
    }
    printk!("]  dsr=[");
    if dsr & 0x80 == 0 { printk!("BUSY"); }
    if dsr & 0x40 != 0 { printk!(",N-ACK"); }
    if dsr & 0x20 != 0 { printk!(",PERROR"); }
    if dsr & 0x10 != 0 { printk!(",SELECT"); }
    if dsr & 0x08 != 0 { printk!(",N-FAULT"); }
    printk!("]\n");
}

/* __parport_pc_frob_control differs from parport_pc_frob_control in that it
 * doesn't do any extra masking. */
#[inline]
pub unsafe fn __parport_pc_frob_control(p: *mut parport, mask: u8, val: u8) -> u8 {
    let priv_ = (*(*p).physport).private_data as *mut parport_pc_private;
    let mut ctr = (*priv_).ctr;
    ctr = (ctr & !mask) ^ val;
    ctr &= (*priv_).ctr_writable;
    outb(ctr, control(p));
    (*priv_).ctr = ctr;
    ctr
}

#[inline]
pub unsafe fn parport_pc_data_reverse(p: *mut parport) { __parport_pc_frob_control(p, 0x20, 0x20); }
#[inline]
pub unsafe fn parport_pc_data_forward(p: *mut parport) { __parport_pc_frob_control(p, 0x20, 0x00); }

#[inline]
pub unsafe fn parport_pc_write_control(p: *mut parport, d: u8) {
    let wm = PARPORT_CONTROL_STROBE | PARPORT_CONTROL_AUTOFD | PARPORT_CONTROL_INIT | PARPORT_CONTROL_SELECT;
    if d & 0x20 != 0 { printk(KERN_DEBUG "%s (%s): use data_reverse for this!\n", (*p).name, (*p).cad.name); parport_pc_data_reverse(p); }
    __parport_pc_frob_control(p, wm, d & wm);
}

#[inline]
pub unsafe fn parport_pc_read_control(p: *mut parport) -> u8 {
    let rm = PARPORT_CONTROL_STROBE | PARPORT_CONTROL_AUTOFD | PARPORT_CONTROL_INIT | PARPORT_CONTROL_SELECT;
    let priv_ = (*(*p).physport).private_data as *const parport_pc_private;
    (*priv_).ctr & rm
}

#[inline]
pub unsafe fn parport_pc_frob_control(p: *mut parport, mut mask: u8, mut val: u8) -> u8 {
    let wm = PARPORT_CONTROL_STROBE | PARPORT_CONTROL_AUTOFD | PARPORT_CONTROL_INIT | PARPORT_CONTROL_SELECT;
    if mask & 0x20 != 0 { if val & 0x20 != 0 { parport_pc_data_reverse(p); } else { parport_pc_data_forward(p); } }
    mask &= wm; val &= wm;
    __parport_pc_frob_control(p, mask, val)
}

#[inline]
pub unsafe fn parport_pc_read_status(p: *mut parport) -> u8 { inb(status(p)) }
#[inline]
pub unsafe fn parport_pc_disable_irq(p: *mut parport) { __parport_pc_frob_control(p, 0x10, 0x00); }
#[inline]
pub unsafe fn parport_pc_enable_irq(p: *mut parport) { __parport_pc_frob_control(p, 0x10, 0x10); }

extern "C" {
    pub fn parport_pc_release_resources(p: *mut parport);
    pub fn parport_pc_claim_resources(p: *mut parport) -> libc::c_int;
    pub fn parport_pc_probe_port(base: libc::c_ulong, base_hi: libc::c_ulong, irq: libc::c_int, dma: libc::c_int, dev: *mut device, irqflags: libc::c_int) -> *mut parport;
    pub fn parport_pc_unregister_port(p: *mut parport);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
