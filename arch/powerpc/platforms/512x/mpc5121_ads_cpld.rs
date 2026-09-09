// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2008 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author: John Rigby, <jrigby@freescale.com>
 *
 * Description:
 * MPC5121ADS CPLD irq handling
 */

// C includes are supplied by the surrounding kernel translation unit.
// #include <linux/kernel.h>
// #include <linux/interrupt.h>
// #include <linux/irq.h>
// #include <linux/io.h>
// #include <linux/of_address.h>
// #include <linux/of_irq.h>
// #include "mpc5121_ads.h"

static mut cpld_pic_node: *mut device_node = core::ptr::null_mut();
static mut cpld_pic_host: *mut irq_domain = core::ptr::null_mut();

/*
 * Bits to ignore in the misc_status register
 * 0x10 touch screen pendown is hard routed to irq1
 * 0x02 pci status is read from pci status register
 */
const MISC_IGNORE: u8 = 0x12;

/*
 * Nothing to ignore in pci status register
 */
const PCI_IGNORE: u8 = 0x00;

#[repr(C)]
struct cpld_pic {
	 pci_mask: u8,
	 pci_status: u8,
	 route: u8,
	 misc_mask: u8,
	 misc_status: u8,
	 misc_control: u8,
}

static mut cpld_regs: *mut cpld_pic = core::ptr::null_mut();

unsafe fn irq_to_pic_mask(irq: u32) -> *mut u8 {
	if irq <= 7 {
		&mut (*cpld_regs).pci_mask
	} else {
		&mut (*cpld_regs).misc_mask
	}
}

fn irq_to_pic_bit(irq: u32) -> u8 {
	(1u32 << (irq & 0x7)) as u8
}

unsafe fn cpld_mask_irq(d: *mut irq_data) {
	let cpld_irq = irqd_to_hwirq(d) as u32;
	let pic_mask = irq_to_pic_mask(cpld_irq);

	out_8(pic_mask, in_8(pic_mask) | irq_to_pic_bit(cpld_irq));
}

unsafe fn cpld_unmask_irq(d: *mut irq_data) {
	let cpld_irq = irqd_to_hwirq(d) as u32;
	let pic_mask = irq_to_pic_mask(cpld_irq);

	out_8(pic_mask, in_8(pic_mask) & !irq_to_pic_bit(cpld_irq));
}

static mut cpld_pic_chip: irq_chip = irq_chip {
	name: "CPLD PIC" as *const _ as *const i8,
	irq_mask: Some(cpld_mask_irq),
	irq_ack: Some(cpld_mask_irq),
	irq_unmask: Some(cpld_unmask_irq),
};

unsafe fn cpld_pic_get_irq(
	offset: u32,
	ignore: u8,
	statusp: *mut u8,
	maskp: *mut u8,
) -> u32 {
	let mut status = in_8(statusp);
	let mask = in_8(maskp);

	/* ignore don't cares and masked irqs */
	status |= ignore | mask;

	if status == 0xff {
		return !0;
	}

	status.trailing_zeros() + offset
}

unsafe fn cpld_pic_cascade(desc: *mut irq_desc) {
	let mut hwirq = cpld_pic_get_irq(
		0,
		PCI_IGNORE,
		&mut (*cpld_regs).pci_status,
		&mut (*cpld_regs).pci_mask,
	);
	if hwirq != !0 {
		generic_handle_domain_irq(cpld_pic_host, hwirq);
		return;
	}

	hwirq = cpld_pic_get_irq(
		8,
		MISC_IGNORE,
		&mut (*cpld_regs).misc_status,
		&mut (*cpld_regs).misc_mask,
	);
	if hwirq != !0 {
		generic_handle_domain_irq(cpld_pic_host, hwirq);
		return;
	}
}

unsafe fn cpld_pic_host_match(
	_h: *mut irq_domain,
	node: *mut device_node,
	_bus_token: irq_domain_bus_token,
) -> i32 {
	(cpld_pic_node == node) as i32
}

unsafe fn cpld_pic_host_map(
	_h: *mut irq_domain,
	virq: u32,
	_hw: irq_hw_number_t,
) -> i32 {
	irq_set_status_flags(virq, IRQ_LEVEL);
	irq_set_chip_and_handler(virq, &mut cpld_pic_chip, handle_level_irq);
	0
}

static cpld_pic_host_ops: irq_domain_ops = irq_domain_ops {
	match_: Some(cpld_pic_host_match),
	map: Some(cpld_pic_host_map),
};

pub unsafe fn mpc5121_ads_cpld_map() {
	let mut np: *mut device_node = core::ptr::null_mut();

	np = of_find_compatible_node(
		core::ptr::null_mut(),
		core::ptr::null_mut(),
		"fsl,mpc5121ads-cpld-pic" as *const _ as *const i8,
	);
	if np.is_null() {
		printk(KERN_ERR, "CPLD PIC init: can not find cpld-pic node\n");
		return;
	}

	cpld_regs = of_iomap(np, 0);
	of_node_put(np);
}

pub unsafe fn mpc5121_ads_cpld_pic_init() {
	let mut cascade_irq: u32;
	let mut np: *mut device_node = core::ptr::null_mut();

	pr_debug!("cpld_ic_init\n");

	np = of_find_compatible_node(
		core::ptr::null_mut(),
		core::ptr::null_mut(),
		"fsl,mpc5121ads-cpld-pic" as *const _ as *const i8,
	);
	if np.is_null() {
		printk(KERN_ERR, "CPLD PIC init: can not find cpld-pic node\n");
		return;
	}

	if cpld_regs.is_null() {
		goto_end(np);
		return;
	}

	cascade_irq = irq_of_parse_and_map(np, 0);
	if cascade_irq == 0 {
		goto_end(np);
		return;
	}

	/*
	 * statically route touch screen pendown through 1
	 * and ignore it here
	 * route all others through our cascade irq
	 */
	out_8(&mut (*cpld_regs).route, 0xfd);
	out_8(&mut (*cpld_regs).pci_mask, 0xff);
	/* unmask pci ints in misc mask */
	out_8(&mut (*cpld_regs).misc_mask, !MISC_IGNORE);

	cpld_pic_node = of_node_get(np);

	cpld_pic_host = irq_domain_create_linear(
		of_fwnode_handle(np),
		16,
		&cpld_pic_host_ops,
		core::ptr::null_mut(),
	);
	if cpld_pic_host.is_null() {
		printk(KERN_ERR, "CPLD PIC: failed to allocate irq host!\n");
		goto_end(np);
		return;
	}

	irq_set_chained_handler(cascade_irq, cpld_pic_cascade);
goto_end(np);
}

unsafe fn goto_end(np: *mut device_node) {
	of_node_put(np);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
