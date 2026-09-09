/*
 * Interrupt handling for GE FPGA based PIC
 *
 * Author: Martyn Welch <martyn.welch@ge.com>
 *
 * 2008 (c) GE Intelligent Platforms Embedded Systems, Inc.
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty
 * of any kind, whether express or implied.
 */

// Linux and architecture headers supplied by the surrounding kernel bindings.

const GEF_PIC_NUM_IRQS: u32 = 32;
const GEF_PIC_INTR_STATUS: usize = 0x0000;
const fn gef_pic_intr_mask(cpu: usize) -> usize { 0x0010 + (0x4 * cpu) }
const GEF_PIC_CPU0_INTR_MASK: usize = gef_pic_intr_mask(0);
const GEF_PIC_CPU1_INTR_MASK: usize = gef_pic_intr_mask(1);
const fn gef_pic_mcp_mask(cpu: usize) -> usize { 0x0018 + (0x4 * cpu) }
const GEF_PIC_CPU0_MCP_MASK: usize = gef_pic_mcp_mask(0);
const GEF_PIC_CPU1_MCP_MASK: usize = gef_pic_mcp_mask(1);

#[repr(C)] pub struct irq_domain { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct irq_desc { pub irq_data: irq_data, pub chip: *mut irq_chip }
#[repr(C)] pub struct irq_data { _private: [u8; 0] }
#[repr(C)] pub struct irq_chip {
    pub name: *const u8,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)>,
}
#[repr(C)] pub struct irq_domain_ops {
    pub map: Option<unsafe extern "C" fn(*mut irq_domain, u32, u64) -> i32>,
    pub xlate: Option<unsafe extern "C" fn(*mut irq_domain, *mut device_node, *const u32, u32, *mut u64, *mut u32) -> i32>,
}

extern "C" {
    fn in_be32(addr: *const u8) -> u32;
    fn out_be32(addr: *mut u8, value: u32);
    fn irqd_to_hwirq(d: *mut irq_data) -> u32;
    fn irq_set_status_flags(virq: u32, flags: u32);
    fn irq_set_chip_and_handler(virq: u32, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn handle_level_irq();
    fn irq_domain_create_linear(fwnode: *mut core::ffi::c_void, size: u32, ops: *const irq_domain_ops, host_data: *mut core::ffi::c_void) -> *mut irq_domain;
    fn of_fwnode_handle(np: *mut device_node) -> *mut core::ffi::c_void;
    fn of_iomap(np: *mut device_node, index: u32) -> *mut u8;
    fn irq_of_parse_and_map(np: *mut device_node, index: u32) -> u32;
    fn irq_set_chained_handler(irq: u32, handler: unsafe extern "C" fn(*mut irq_desc));
    fn irq_find_mapping(domain: *mut irq_domain, hwirq: u64) -> u32;
    fn generic_handle_irq(irq: u32);
    fn printk(fmt: *const u8, ...);
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: usize);
}

#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
static mut gef_pic_lock: raw_spinlock_t = raw_spinlock_t { _private: [] };
static mut gef_pic_irq_reg_base: *mut u8 = core::ptr::null_mut();
static mut gef_pic_irq_host: *mut irq_domain = core::ptr::null_mut();
static mut gef_pic_cascade_irq: i32 = 0;

unsafe extern "C" fn gef_pic_cascade(desc: *mut irq_desc) {
    let chip = (*desc).chip;
    let cascade_irq = gef_pic_get_irq();
    if cascade_irq != 0 { generic_handle_irq(cascade_irq); }
    ((*chip).irq_eoi.expect("irq_eoi"))(&mut (*desc).irq_data);
}

unsafe extern "C" fn gef_pic_mask(d: *mut irq_data) {
    let mut flags = 0usize;
    let hwirq = irqd_to_hwirq(d);
    raw_spin_lock_irqsave(&mut gef_pic_lock, &mut flags);
    let mut mask = in_be32(gef_pic_irq_reg_base.add(GEF_PIC_INTR_MASK(0)));
    mask &= !(1u32 << hwirq);
    out_be32(gef_pic_irq_reg_base.add(GEF_PIC_INTR_MASK(0)), mask);
    raw_spin_unlock_irqrestore(&mut gef_pic_lock, flags);
}

unsafe extern "C" fn gef_pic_mask_ack(d: *mut irq_data) { gef_pic_mask(d); }

unsafe extern "C" fn gef_pic_unmask(d: *mut irq_data) {
    let mut flags = 0usize;
    let hwirq = irqd_to_hwirq(d);
    raw_spin_lock_irqsave(&mut gef_pic_lock, &mut flags);
    let mut mask = in_be32(gef_pic_irq_reg_base.add(GEF_PIC_INTR_MASK(0)));
    mask |= 1u32 << hwirq;
    out_be32(gef_pic_irq_reg_base.add(GEF_PIC_INTR_MASK(0)), mask);
    raw_spin_unlock_irqrestore(&mut gef_pic_lock, flags);
}

static mut gef_pic_chip: irq_chip = irq_chip {
    name: b"gefp\0".as_ptr(), irq_mask: Some(gef_pic_mask),
    irq_mask_ack: Some(gef_pic_mask_ack), irq_unmask: Some(gef_pic_unmask), irq_eoi: None,
};

unsafe extern "C" fn gef_pic_host_map(_h: *mut irq_domain, virq: u32, _hwirq: u64) -> i32 {
    irq_set_status_flags(virq, 1); irq_set_chip_and_handler(virq, &mut gef_pic_chip, handle_level_irq); 0
}
unsafe extern "C" fn gef_pic_host_xlate(_h: *mut irq_domain, _ct: *mut device_node, intspec: *const u32, intsize: u32, out_hwirq: *mut u64, out_flags: *mut u32) -> i32 {
    *out_hwirq = *intspec as u64;
    *out_flags = if intsize > 1 { *intspec.add(1) } else { 4 }; 0
}
static gef_pic_host_ops: irq_domain_ops = irq_domain_ops { map: Some(gef_pic_host_map), xlate: Some(gef_pic_host_xlate) };

pub unsafe extern "C" fn gef_pic_init(np: *mut device_node) {
    let mut flags = 0usize; gef_pic_irq_reg_base = of_iomap(np, 0);
    raw_spin_lock_irqsave(&mut gef_pic_lock, &mut flags);
    out_be32(gef_pic_irq_reg_base.add(GEF_PIC_CPU0_INTR_MASK), 0); out_be32(gef_pic_irq_reg_base.add(GEF_PIC_CPU1_INTR_MASK), 0);
    out_be32(gef_pic_irq_reg_base.add(GEF_PIC_CPU0_MCP_MASK), 0); out_be32(gef_pic_irq_reg_base.add(GEF_PIC_CPU1_MCP_MASK), 0);
    raw_spin_unlock_irqrestore(&mut gef_pic_lock, flags);
    gef_pic_cascade_irq = irq_of_parse_and_map(np, 0) as i32; if gef_pic_cascade_irq == 0 { return; }
    gef_pic_irq_host = irq_domain_create_linear(of_fwnode_handle(np), GEF_PIC_NUM_IRQS, &gef_pic_host_ops, core::ptr::null_mut());
    if gef_pic_irq_host.is_null() { return; }
    irq_set_chained_handler(gef_pic_cascade_irq as u32, gef_pic_cascade);
}

pub unsafe extern "C" fn gef_pic_get_irq() -> u32 {
    let cause = in_be32(gef_pic_irq_reg_base.add(GEF_PIC_INTR_STATUS));
    let mask = in_be32(gef_pic_irq_reg_base.add(GEF_PIC_INTR_MASK(0)));
    let active = cause & mask; if active == 0 { return 0; }
    let mut hwirq: i32 = GEF_PIC_NUM_IRQS as i32 - 1;
    while hwirq > -1 { if active & (1u32 << hwirq) != 0 { break; } hwirq -= 1; }
    irq_find_mapping(gef_pic_irq_host, hwirq as u64)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
