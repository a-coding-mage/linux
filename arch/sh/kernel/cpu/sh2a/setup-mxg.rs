// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas MX-G (R8A03022BG) Setup
 *
 *  Copyright (C) 2008, 2009  Paul Mundt
 */
// Linux kernel dependencies supplied by other translation units.

#[repr(i32)]
enum InterruptSource {
    UNUSED = 0,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7,
    IRQ8, IRQ9, IRQ10, IRQ11, IRQ12, IRQ13, IRQ14, IRQ15,
    PINT0, PINT1, PINT2, PINT3, PINT4, PINT5, PINT6, PINT7,
    SINT8, SINT7, SINT6, SINT5, SINT4, SINT3, SINT2, SINT1,
    SCIF0, SCIF1,
    MTU2_GROUP1, MTU2_GROUP2, MTU2_GROUP3, MTU2_GROUP4, MTU2_GROUP5,
    MTU2_TGI3B, MTU2_TGI3C,
    PINT,
}

static mut vectors: [intc_vect; 54] = [
    INTC_IRQ!(IRQ0, 64), INTC_IRQ!(IRQ1, 65), INTC_IRQ!(IRQ2, 66), INTC_IRQ!(IRQ3, 67),
    INTC_IRQ!(IRQ4, 68), INTC_IRQ!(IRQ5, 69), INTC_IRQ!(IRQ6, 70), INTC_IRQ!(IRQ7, 71),
    INTC_IRQ!(IRQ8, 72), INTC_IRQ!(IRQ9, 73), INTC_IRQ!(IRQ10, 74), INTC_IRQ!(IRQ11, 75),
    INTC_IRQ!(IRQ12, 76), INTC_IRQ!(IRQ13, 77), INTC_IRQ!(IRQ14, 78), INTC_IRQ!(IRQ15, 79),
    INTC_IRQ!(PINT0, 80), INTC_IRQ!(PINT1, 81), INTC_IRQ!(PINT2, 82), INTC_IRQ!(PINT3, 83),
    INTC_IRQ!(PINT4, 84), INTC_IRQ!(PINT5, 85), INTC_IRQ!(PINT6, 86), INTC_IRQ!(PINT7, 87),
    INTC_IRQ!(SINT8, 94), INTC_IRQ!(SINT7, 95), INTC_IRQ!(SINT6, 96), INTC_IRQ!(SINT5, 97),
    INTC_IRQ!(SINT4, 98), INTC_IRQ!(SINT3, 99), INTC_IRQ!(SINT2, 100), INTC_IRQ!(SINT1, 101),
    INTC_IRQ!(SCIF0, 220), INTC_IRQ!(SCIF0, 221), INTC_IRQ!(SCIF0, 222), INTC_IRQ!(SCIF0, 223),
    INTC_IRQ!(SCIF1, 224), INTC_IRQ!(SCIF1, 225), INTC_IRQ!(SCIF1, 226), INTC_IRQ!(SCIF1, 227),
    INTC_IRQ!(MTU2_GROUP1, 228), INTC_IRQ!(MTU2_GROUP1, 229), INTC_IRQ!(MTU2_GROUP1, 230), INTC_IRQ!(MTU2_GROUP1, 231), INTC_IRQ!(MTU2_GROUP1, 232), INTC_IRQ!(MTU2_GROUP1, 233),
    INTC_IRQ!(MTU2_GROUP2, 234), INTC_IRQ!(MTU2_GROUP2, 235), INTC_IRQ!(MTU2_GROUP2, 236), INTC_IRQ!(MTU2_GROUP2, 237), INTC_IRQ!(MTU2_GROUP2, 238), INTC_IRQ!(MTU2_GROUP2, 239),
    INTC_IRQ!(MTU2_GROUP3, 240), INTC_IRQ!(MTU2_GROUP3, 241), INTC_IRQ!(MTU2_GROUP3, 242), INTC_IRQ!(MTU2_GROUP3, 243),
    INTC_IRQ!(MTU2_TGI3B, 244), INTC_IRQ!(MTU2_TGI3C, 245),
    INTC_IRQ!(MTU2_GROUP4, 246), INTC_IRQ!(MTU2_GROUP4, 247), INTC_IRQ!(MTU2_GROUP4, 248), INTC_IRQ!(MTU2_GROUP4, 249), INTC_IRQ!(MTU2_GROUP4, 250), INTC_IRQ!(MTU2_GROUP4, 251),
    INTC_IRQ!(MTU2_GROUP5, 252), INTC_IRQ!(MTU2_GROUP5, 253), INTC_IRQ!(MTU2_GROUP5, 254), INTC_IRQ!(MTU2_GROUP5, 255),
];

static mut groups: [intc_group; 1] = [INTC_GROUP!(PINT, PINT0, PINT1, PINT2, PINT3, PINT4, PINT5, PINT6, PINT7)];

static mut prio_registers: [intc_prio_reg; 17] = [
    intc_prio_reg { set: 0xfffd9418, ..INTC_PRIO!(IRQ0, IRQ1, IRQ2, IRQ3) },
    intc_prio_reg { set: 0xfffd941a, ..INTC_PRIO!(IRQ4, IRQ5, IRQ6, IRQ7) },
    intc_prio_reg { set: 0xfffd941c, ..INTC_PRIO!(IRQ8, IRQ9, IRQ10, IRQ11) },
    intc_prio_reg { set: 0xfffd941e, ..INTC_PRIO!(IRQ12, IRQ13, IRQ14, IRQ15) },
    intc_prio_reg { set: 0xfffd9420, ..INTC_PRIO!(PINT, 0, 0, 0) },
    INTC_PRIO_REG!(0xfffd9800, 0, 16, 4, []), INTC_PRIO_REG!(0xfffd9802, 0, 16, 4, []),
    INTC_PRIO_REG!(0xfffd9804, 0, 16, 4, []), INTC_PRIO_REG!(0xfffd9806, 0, 16, 4, []),
    INTC_PRIO_REG!(0xfffd9808, 0, 16, 4, []), INTC_PRIO_REG!(0xfffd980a, 0, 16, 4, []),
    INTC_PRIO_REG!(0xfffd980c, 0, 16, 4, []), INTC_PRIO_REG!(0xfffd980e, 0, 16, 4, []),
    INTC_PRIO_REG!(0xfffd9810, 0, 16, 4, [0, 0, 0, SCIF0]),
    INTC_PRIO_REG!(0xfffd9812, 0, 16, 4, [SCIF1, MTU2_GROUP1, MTU2_GROUP2, MTU2_GROUP3]),
    INTC_PRIO_REG!(0xfffd9814, 0, 16, 4, [MTU2_TGI3B, MTU2_TGI3C, MTU2_GROUP4, MTU2_GROUP5]),
];

static mut mask_registers: [intc_mask_reg; 1] = [INTC_MASK_REG!(0xfffd9408, 0, 16, [0, 0, 0, 0, 0, 0, 0, 0, PINT7, PINT6, PINT5, PINT4, PINT3, PINT2, PINT1, PINT0])];

static mut intc_desc: intc_desc = DECLARE_INTC_DESC!("mxg", vectors, groups, mask_registers, prio_registers);

static mut mtu2_resources: [resource; 4] = [
    DEFINE_RES_MEM!(0xff801000, 0x400), DEFINE_RES_IRQ_NAMED!(228, "tgi0a"),
    DEFINE_RES_IRQ_NAMED!(234, "tgi1a"), DEFINE_RES_IRQ_NAMED!(240, "tgi2a"),
];
static mut mtu2_device: platform_device = platform_device { name: "sh-mtu2", id: -1, resource: mtu2_resources.as_ptr(), num_resources: mtu2_resources.len(), ..Default::default() };
static mut scif0_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, r#type: PORT_SCIF };
static mut scif0_resources: [resource; 2] = [DEFINE_RES_MEM!(0xff804000, 0x100), DEFINE_RES_IRQ!(220)];
static mut scif0_device: platform_device = platform_device { name: "sh-sci", id: 0, resource: scif0_resources.as_ptr(), num_resources: scif0_resources.len(), dev: device { platform_data: &mut scif0_platform_data }, ..Default::default() };
static mut mxg_devices: [*mut platform_device; 2] = [&mut scif0_device, &mut mtu2_device];

unsafe fn mxg_devices_setup() -> i32 { platform_add_devices(mxg_devices.as_ptr(), mxg_devices.len()) }
// arch_initcall(mxg_devices_setup)

unsafe fn plat_irq_setup() { register_intc_controller(&mut intc_desc); }

static mut mxg_early_devices: [*mut platform_device; 2] = [&mut scif0_device, &mut mtu2_device];
unsafe fn plat_early_device_setup() { sh_early_platform_add_devices(mxg_early_devices.as_ptr(), mxg_early_devices.len()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
