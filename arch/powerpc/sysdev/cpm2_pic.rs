/*
 * Platform information definitions.
 *
 * Copied from arch/ppc/syslib/cpm2_pic.c with minor subsequent updates
 * to make in work in arch/powerpc/. Original (c) belongs to Dan Malek.
 *
 * Author:  Vitaly Bordug <vbordug@ru.mvista.com>
 *
 * 1999-2001 (c) Dan Malek <dan@embeddedalley.com>
 * 2006 (c) MontaVista Software, Inc.
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

/* The CPM2 internal interrupt controller.  It is usually
 * the only interrupt controller.
 * There are two 32-bit registers (high/low) for up to 64
 * possible interrupts.
 *
 * Now, the fun starts.....Interrupt Numbers DO NOT MAP
 * in a simple arithmetic fashion to mask or pending registers.
 * That is, interrupt 4 does not map to bit position 4.
 * We create two tables, indexed by vector number, to indicate
 * which register to use and which bit in the register to use.
 */

/* C headers: linux/stddef.h, linux/sched.h, linux/signal.h, linux/irq.h,
 * linux/irqdomain.h, asm/immap_cpm2.h, asm/io.h, and cpm2_pic.h. */

/* External IRQS */
const CPM2_IRQ_EXT1: u32 = 19;
const CPM2_IRQ_EXT7: u32 = 25;

/* Port C IRQS */
const CPM2_IRQ_PORTC15: u32 = 48;
const CPM2_IRQ_PORTC0: u32 = 63;

static mut cpm2_intctl: *mut cpm2_intctl_t = core::ptr::null_mut();
static mut cpm2_pic_host: *mut irq_domain = core::ptr::null_mut();
static mut ppc_cached_irq_mask: [c_ulong; 2] = [0; 2]; /* 2 32-bit registers */

static irq_to_siureg: [u8; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/* bit numbers do not match the docs, these are precomputed so the bit for
 * a given irq is (1 << irq_to_siubit[irq]) */
static irq_to_siubit: [u8; 64] = [
    0, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
    2, 1, 0, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 0,
    31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
];

unsafe fn cpm2_mask_irq(d: *mut irq_data) {
    let irq_nr = irqd_to_hwirq(d) as usize;
    let bit = irq_to_siubit[irq_nr];
    let word = irq_to_siureg[irq_nr] as usize;
    ppc_cached_irq_mask[word] &= !(1 as c_ulong).wrapping_shl(bit as u32);
    out_be32((&mut (*cpm2_intctl).ic_simrh as *mut _).add(word), ppc_cached_irq_mask[word] as u32);
}

unsafe fn cpm2_unmask_irq(d: *mut irq_data) {
    let irq_nr = irqd_to_hwirq(d) as usize;
    let bit = irq_to_siubit[irq_nr];
    let word = irq_to_siureg[irq_nr] as usize;
    ppc_cached_irq_mask[word] |= (1 as c_ulong).wrapping_shl(bit as u32);
    out_be32((&mut (*cpm2_intctl).ic_simrh as *mut _).add(word), ppc_cached_irq_mask[word] as u32);
}

unsafe fn cpm2_ack(d: *mut irq_data) {
    let irq_nr = irqd_to_hwirq(d) as usize;
    let bit = irq_to_siubit[irq_nr];
    let word = irq_to_siureg[irq_nr] as usize;
    out_be32((&mut (*cpm2_intctl).ic_sipnrh as *mut _).add(word), 1u32 << bit);
}

unsafe fn cpm2_end_irq(d: *mut irq_data) {
    let irq_nr = irqd_to_hwirq(d) as usize;
    let bit = irq_to_siubit[irq_nr];
    let word = irq_to_siureg[irq_nr] as usize;
    ppc_cached_irq_mask[word] |= (1 as c_ulong).wrapping_shl(bit as u32);
    out_be32((&mut (*cpm2_intctl).ic_simrh as *mut _).add(word), ppc_cached_irq_mask[word] as u32);
    /* Work around large numbers of spurious IRQs on PowerPC 82xx systems. */
    mb();
}

unsafe fn cpm2_set_irq_type(d: *mut irq_data, mut flow_type: u32) -> c_int {
    let src = irqd_to_hwirq(d) as u32;
    let mut vold: u32;
    let mut vnew: u32;
    let edibit: u32;
    if src >= CPM2_IRQ_PORTC15 && src <= CPM2_IRQ_PORTC0 {
        if flow_type == IRQ_TYPE_NONE { flow_type = IRQ_TYPE_EDGE_BOTH; }
        if flow_type != IRQ_TYPE_EDGE_BOTH && flow_type != IRQ_TYPE_EDGE_FALLING { return sense_error(flow_type); }
    } else {
        if flow_type == IRQ_TYPE_NONE { flow_type = IRQ_TYPE_LEVEL_LOW; }
        if flow_type & (IRQ_TYPE_EDGE_RISING | IRQ_TYPE_LEVEL_HIGH) != 0 { return sense_error(flow_type); }
    }
    irqd_set_trigger_type(d, flow_type);
    if flow_type & IRQ_TYPE_LEVEL_LOW != 0 { irq_set_handler_locked(d, handle_level_irq); } else { irq_set_handler_locked(d, handle_edge_irq); }
    if src >= CPM2_IRQ_EXT1 && src <= CPM2_IRQ_EXT7 { edibit = 14 - (src - CPM2_IRQ_EXT1); }
    else if src >= CPM2_IRQ_PORTC15 && src <= CPM2_IRQ_PORTC0 { edibit = 31 - (CPM2_IRQ_PORTC0 - src); }
    else { return if flow_type & IRQ_TYPE_LEVEL_LOW != 0 { IRQ_SET_MASK_OK_NOCOPY } else { -EINVAL }; }
    vold = in_be32(&(*cpm2_intctl).ic_siexr);
    if flow_type & IRQ_TYPE_SENSE_MASK == IRQ_TYPE_EDGE_FALLING { vnew = vold | (1 << edibit); } else { vnew = vold & !(1 << edibit); }
    if vold != vnew { out_be32(&mut (*cpm2_intctl).ic_siexr, vnew); }
    IRQ_SET_MASK_OK_NOCOPY
}

unsafe fn sense_error(flow_type: u32) -> c_int { pr_err!("CPM2 PIC: sense type 0x%x not supported\n", flow_type); -EINVAL }

static mut cpm2_pic: irq_chip = irq_chip { name: "CPM2 SIU", irq_mask: Some(cpm2_mask_irq), irq_unmask: Some(cpm2_unmask_irq), irq_ack: Some(cpm2_ack), irq_eoi: Some(cpm2_end_irq), irq_set_type: Some(cpm2_set_irq_type), flags: IRQCHIP_EOI_IF_HANDLED };

unsafe fn cpm2_get_irq() -> u32 {
    let bits = in_be32(&(*cpm2_intctl).ic_sivec);
    let irq = bits >> 26;
    if irq == 0 { return (-1i32) as u32; }
    irq_find_mapping(cpm2_pic_host, irq)
}

unsafe fn cpm2_pic_host_map(h: *mut irq_domain, virq: u32, hw: irq_hw_number_t) -> c_int {
    pr_debug!("cpm2_pic_host_map(%d, 0x%lx)\n", virq, hw);
    irq_set_status_flags(virq, IRQ_LEVEL);
    irq_set_chip_and_handler(virq, &mut cpm2_pic, handle_level_irq);
    0
}

static cpm2_pic_host_ops: irq_domain_ops = irq_domain_ops { map: Some(cpm2_pic_host_map), xlate: Some(irq_domain_xlate_onetwocell) };

unsafe fn cpm2_pic_init(node: *mut device_node) {
    cpm2_intctl = &mut (*cpm2_immr).im_intctl;
    out_be32(&mut (*cpm2_intctl).ic_simrh, 0);
    out_be32(&mut (*cpm2_intctl).ic_simrl, 0);
    wmb();
    out_be32(&mut (*cpm2_intctl).ic_sipnrh, 0xffff_ffff);
    out_be32(&mut (*cpm2_intctl).ic_sipnrl, 0xffff_ffff);
    wmb();
    let _i = in_be32(&(*cpm2_intctl).ic_sivec);
    rmb();
    out_be16(&mut (*cpm2_intctl).ic_sicr, 0);
    out_be32(&mut (*cpm2_intctl).ic_scprrh, 0x0530_9770);
    out_be32(&mut (*cpm2_intctl).ic_scprrl, 0x0530_9770);
    cpm2_pic_host = irq_domain_create_linear(of_fwnode_handle(node), 64, &cpm2_pic_host_ops, core::ptr::null_mut());
    if cpm2_pic_host.is_null() { printk!(KERN_ERR "CPM2 PIC: failed to allocate irq host!\n"); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
