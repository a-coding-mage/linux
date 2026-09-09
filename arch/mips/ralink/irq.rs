// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2009 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

// Linux and architecture dependencies supplied by the surrounding kernel.

const INTC_INT_GLOBAL: u32 = 1u32 << 31;

const RALINK_CPU_IRQ_INTC: i32 = MIPS_CPU_IRQ_BASE + 2;
const RALINK_CPU_IRQ_PCI: i32 = MIPS_CPU_IRQ_BASE + 4;
const RALINK_CPU_IRQ_FE: i32 = MIPS_CPU_IRQ_BASE + 5;
const RALINK_CPU_IRQ_WIFI: i32 = MIPS_CPU_IRQ_BASE + 6;
const RALINK_CPU_IRQ_COUNTER: i32 = MIPS_CPU_IRQ_BASE + 7;

/* we have a cascade of 8 irqs */
const RALINK_INTC_IRQ_BASE: u32 = 8;

/* we have 32 SoC irqs */
const RALINK_INTC_IRQ_COUNT: u32 = 32;

const RALINK_INTC_IRQ_PERFC: u32 = RALINK_INTC_IRQ_BASE + 9;

#[repr(C)]
enum RtIntcRegsEnum {
    INTC_REG_STATUS0 = 0,
    INTC_REG_STATUS1,
    INTC_REG_TYPE,
    INTC_REG_RAW_STATUS,
    INTC_REG_ENABLE,
    INTC_REG_DISABLE,
}

static mut rt_intc_regs: [u32; 6] = [
    0x00, // INTC_REG_STATUS0
    0x04, // INTC_REG_STATUS1
    0x20, // INTC_REG_TYPE
    0x30, // INTC_REG_RAW_STATUS
    0x34, // INTC_REG_ENABLE
    0x38, // INTC_REG_DISABLE
];

static mut rt_intc_membase: *mut core::ffi::c_void = core::ptr::null_mut();

static mut rt_perfcount_irq: i32 = 0;

#[inline]
unsafe fn rt_intc_w32(val: u32, reg: u32) {
    __raw_writel(val, (rt_intc_membase as *mut u8).add(rt_intc_regs[reg as usize] as usize));
}

#[inline]
unsafe fn rt_intc_r32(reg: u32) -> u32 {
    __raw_readl((rt_intc_membase as *mut u8).add(rt_intc_regs[reg as usize] as usize))
}

unsafe fn ralink_intc_irq_unmask(d: *mut irq_data) {
    rt_intc_w32(1u32 << (*d).hwirq, INTC_REG_ENABLE as u32);
}

unsafe fn ralink_intc_irq_mask(d: *mut irq_data) {
    rt_intc_w32(1u32 << (*d).hwirq, INTC_REG_DISABLE as u32);
}

static mut ralink_intc_irq_chip: irq_chip = irq_chip {
    name: b"INTC\0".as_ptr() as *const i8,
    irq_unmask: Some(ralink_intc_irq_unmask),
    irq_mask: Some(ralink_intc_irq_mask),
    irq_mask_ack: Some(ralink_intc_irq_mask),
};

#[no_mangle]
pub unsafe extern "C" fn get_c0_perfcount_int() -> i32 {
    rt_perfcount_irq
}

#[no_mangle]
pub unsafe extern "C" fn get_c0_compare_int() -> u32 {
    CP0_LEGACY_COMPARE_IRQ
}

unsafe fn ralink_intc_irq_handler(desc: *mut irq_desc) {
    let pending: u32 = rt_intc_r32(INTC_REG_STATUS0 as u32);

    if pending != 0 {
        let domain: *mut irq_domain = irq_desc_get_handler_data(desc);
        generic_handle_domain_irq(domain, pending.trailing_zeros());
    } else {
        spurious_interrupt();
    }
}

pub unsafe extern "C" fn plat_irq_dispatch() {
    let pending: usize = (read_c0_status() & read_c0_cause() & ST0_IM) as usize;

    if pending & STATUSF_IP7 as usize != 0 {
        do_IRQ(RALINK_CPU_IRQ_COUNTER);
    } else if pending & STATUSF_IP5 as usize != 0 {
        do_IRQ(RALINK_CPU_IRQ_FE);
    } else if pending & STATUSF_IP6 as usize != 0 {
        do_IRQ(RALINK_CPU_IRQ_WIFI);
    } else if pending & STATUSF_IP4 as usize != 0 {
        do_IRQ(RALINK_CPU_IRQ_PCI);
    } else if pending & STATUSF_IP2 as usize != 0 {
        do_IRQ(RALINK_CPU_IRQ_INTC);
    } else {
        spurious_interrupt();
    }
}

unsafe fn intc_map(d: *mut irq_domain, irq: u32, hw: irq_hw_number_t) -> i32 {
    irq_set_chip_and_handler(irq, &mut ralink_intc_irq_chip, handle_level_irq);
    0
}

static irq_domain_ops: irq_domain_ops = irq_domain_ops {
    .xlate: irq_domain_xlate_onecell,
    .map: intc_map,
};

unsafe fn intc_of_init(node: *mut device_node, parent: *mut device_node) -> i32 {
    let mut res: resource = core::mem::zeroed();
    let mut domain: *mut irq_domain;
    let irq: i32;

    if of_property_read_u32_array(node, b"ralink,intc-registers\0".as_ptr() as *const i8,
                                  rt_intc_regs.as_mut_ptr(), 6) == 0 {
        pr_info(b"intc: using register map from devicetree\n\0".as_ptr() as *const i8);
    }

    irq = irq_of_parse_and_map(node, 0);
    if irq == 0 {
        panic!("Failed to get INTC IRQ");
    }

    if of_address_to_resource(node, 0, &mut res) != 0 {
        panic!("Failed to get intc memory range");
    }

    if request_mem_region(res.start, resource_size(&res), res.name).is_null() {
        pr_err(b"Failed to request intc memory\n\0".as_ptr() as *const i8);
    }

    rt_intc_membase = ioremap(res.start, resource_size(&res));
    if rt_intc_membase.is_null() {
        panic!("Failed to remap intc memory");
    }

    /* disable all interrupts */
    rt_intc_w32(!0u32, INTC_REG_DISABLE as u32);

    /* route all INTC interrupts to MIPS HW0 interrupt */
    rt_intc_w32(0, INTC_REG_TYPE as u32);

    domain = irq_domain_create_legacy(of_fwnode_handle(node), RALINK_INTC_IRQ_COUNT,
                                      RALINK_INTC_IRQ_BASE, 0, &irq_domain_ops, core::ptr::null_mut());
    if domain.is_null() {
        panic!("Failed to add irqdomain");
    }

    rt_intc_w32(INTC_INT_GLOBAL, INTC_REG_ENABLE as u32);
    irq_set_chained_handler_and_data(irq, ralink_intc_irq_handler, domain);

    /* tell the kernel which irq is used for performance monitoring */
    rt_perfcount_irq = irq_create_mapping(domain, 9);
    0
}

static mut of_irq_ids: [of_device_id; 3] = [
    of_device_id { compatible: b"mti,cpu-interrupt-controller\0".as_ptr() as *const i8, data: mips_cpu_irq_of_init },
    of_device_id { compatible: b"ralink,rt2880-intc\0".as_ptr() as *const i8, data: intc_of_init },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

pub unsafe fn arch_init_irq() {
    of_irq_init(of_irq_ids.as_mut_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
