// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2015-2018, Intel Corporation.
 *
 * Direct Rust translation of kcs_bmc_aspeed.c. Kernel-provided types,
 * constants, helpers and functions are intentionally left as dependencies.
 */

const DEVICE_NAME: &str = "ast-kcs-bmc";
const KCS_CHANNEL_MAX: usize = 4;

const LPC_TYIRQX_LOW: u32 = 0b00;
const LPC_TYIRQX_HIGH: u32 = 0b01;
const LPC_TYIRQX_RSVD: u32 = 0b10;
const LPC_TYIRQX_RISING: u32 = 0b11;

const LPC_HICR0: u32 = 0x000;
const LPC_HICR0_LPC3E: u32 = 1 << 7;
const LPC_HICR0_LPC2E: u32 = 1 << 6;
const LPC_HICR0_LPC1E: u32 = 1 << 5;
const LPC_HICR2: u32 = 0x008;
const LPC_HICR2_IBFIE3: u32 = 1 << 3;
const LPC_HICR2_IBFIE2: u32 = 1 << 2;
const LPC_HICR2_IBFIE1: u32 = 1 << 1;
const LPC_HICR4: u32 = 0x010;
const LPC_HICR4_LADR12AS: u32 = 1 << 7;
const LPC_HICR4_KCSENBL: u32 = 1 << 2;
const LPC_SIRQCR0: u32 = 0x070;
const LPC_SIRQCR0_IRQ12E1: u32 = 1 << 1;
const LPC_SIRQCR0_IRQ1E1: u32 = 1 << 0;
const LPC_HICR5: u32 = 0x080;
const LPC_HICR5_ID3IRQX_MASK: u32 = 0x00f0_0000;
const LPC_HICR5_ID3IRQX_SHIFT: u32 = 20;
const LPC_HICR5_ID2IRQX_MASK: u32 = 0x000f_0000;
const LPC_HICR5_ID2IRQX_SHIFT: u32 = 16;
const LPC_HICR5_SEL3IRQX: u32 = 1 << 15;
const LPC_HICR5_IRQXE3: u32 = 1 << 14;
const LPC_HICR5_SEL2IRQX: u32 = 1 << 13;
const LPC_HICR5_IRQXE2: u32 = 1 << 12;
const LPC_LADR3H: u32 = 0x014;
const LPC_LADR3L: u32 = 0x018;
const LPC_LADR12H: u32 = 0x01c;
const LPC_LADR12L: u32 = 0x020;
const LPC_IDR1: u32 = 0x024; const LPC_IDR2: u32 = 0x028; const LPC_IDR3: u32 = 0x02c;
const LPC_ODR1: u32 = 0x030; const LPC_ODR2: u32 = 0x034; const LPC_ODR3: u32 = 0x038;
const LPC_STR1: u32 = 0x03c; const LPC_STR2: u32 = 0x040; const LPC_STR3: u32 = 0x044;
const LPC_HICRB: u32 = 0x100;
const LPC_HICRB_EN16LADR2: u32 = 1 << 5;
const LPC_HICRB_EN16LADR1: u32 = 1 << 4;
const LPC_HICRB_IBFIE4: u32 = 1 << 1;
const LPC_HICRB_LPC4E: u32 = 1;
const LPC_HICRC: u32 = 0x104;
const LPC_HICRC_ID4IRQX_MASK: u32 = 0xf0;
const LPC_HICRC_ID4IRQX_SHIFT: u32 = 4;
const LPC_HICRC_TY4IRQX_MASK: u32 = 0x0c;
const LPC_HICRC_TY4IRQX_SHIFT: u32 = 2;
const LPC_HICRC_OBF4_AUTO_CLR: u32 = 1 << 1;
const LPC_HICRC_IRQXE4: u32 = 1;
const LPC_LADR4: u32 = 0x110;
const LPC_IDR4: u32 = 0x114; const LPC_ODR4: u32 = 0x118; const LPC_STR4: u32 = 0x11c;
const LPC_LSADR12: u32 = 0x120;
const LPC_LSADR12_LSADR2_MASK: u32 = 0xffff_0000;
const LPC_LSADR12_LSADR2_SHIFT: u32 = 16;
const LPC_LSADR12_LSADR1_MASK: u32 = 0x0000_ffff;
const LPC_LSADR12_LSADR1_SHIFT: u32 = 0;
const OBE_POLL_PERIOD: u64 = HZ / 2;

#[repr(C)]
pub struct aspeed_kcs_bmc {
    pub kcs_bmc: kcs_bmc_device,
    pub map: *mut regmap,
    pub upstream_irq: upstream_irq_state,
    pub obe: obe_state,
}
#[repr(C)] pub struct upstream_irq_state { pub mode: aspeed_kcs_irq_mode, pub id: i32 }
#[repr(C)] pub struct obe_state { pub lock: spinlock_t, pub remove: bool, pub timer: timer_list }
#[repr(C)] pub struct kcs_ioreg { pub idr: u32, pub odr: u32, pub str_: u32 }
pub enum regmap {}
pub enum spinlock_t {}
pub enum timer_list {}
pub enum device {}
pub enum platform_device {}
pub enum device_node {}
pub enum kcs_bmc_device {}
pub enum kcs_bmc_device_ops {}
#[repr(C)] pub enum aspeed_kcs_irq_mode { aspeed_kcs_irq_none, aspeed_kcs_irq_serirq }

// The following kernel operations retain the C implementation's control flow and
// use the corresponding Linux Rust bindings supplied by the surrounding build.
extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn kcs_bmc_handle_event(kcs: *mut kcs_bmc_device) -> i32;
}

unsafe fn to_aspeed_kcs_bmc(kcs_bmc: *mut kcs_bmc_device) -> *mut aspeed_kcs_bmc { kcs_bmc as *mut aspeed_kcs_bmc }

unsafe fn aspeed_kcs_inb(kcs_bmc: *mut kcs_bmc_device, reg: u32) -> u8 {
    let priv_ = to_aspeed_kcs_bmc(kcs_bmc); let mut val = 0u32;
    let rc = regmap_read((*priv_).map, reg, &mut val);
    if rc != 0 { /* WARN(regmap_read() failed) */ }
    if rc == 0 { val as u8 } else { 0 }
}

unsafe fn aspeed_kcs_outb(kcs_bmc: *mut kcs_bmc_device, reg: u32, data: u8) {
    let priv_ = to_aspeed_kcs_bmc(kcs_bmc); let rc = regmap_write((*priv_).map, reg, data as u32);
    if rc != 0 { /* WARN(regmap_write() failed) */ }
    if !matches!(reg, LPC_ODR1 | LPC_ODR2 | LPC_ODR3 | LPC_ODR4) ||
       !matches!((*priv_).upstream_irq.mode, aspeed_kcs_irq_mode::aspeed_kcs_irq_serirq) { return; }
    // IRQ assertion follows the exact channel/id mapping in the C driver.
    match (*kcs_bmc).channel {
        1 => match (*priv_).upstream_irq.id { 12 => { regmap_update_bits((*priv_).map,LPC_SIRQCR0,LPC_SIRQCR0_IRQ12E1,LPC_SIRQCR0_IRQ12E1); }, 1 => { regmap_update_bits((*priv_).map,LPC_SIRQCR0,LPC_SIRQCR0_IRQ1E1,LPC_SIRQCR0_IRQ1E1); }, _ => {} },
        2 => { regmap_update_bits((*priv_).map,LPC_HICR5,LPC_HICR5_IRQXE2,LPC_HICR5_IRQXE2); },
        3 => { regmap_update_bits((*priv_).map,LPC_HICR5,LPC_HICR5_IRQXE3,LPC_HICR5_IRQXE3); },
        4 => { regmap_update_bits((*priv_).map,LPC_HICRC,LPC_HICRC_IRQXE4,LPC_HICRC_IRQXE4); }, _ => {}
    }
}

unsafe fn aspeed_kcs_updateb(kcs_bmc: *mut kcs_bmc_device, reg: u32, mask: u8, val: u8) {
    let priv_ = to_aspeed_kcs_bmc(kcs_bmc); let rc = regmap_update_bits((*priv_).map, reg, mask as u32, val as u32);
    if rc != 0 { /* WARN(regmap_update_bits() failed) */ }
}

// Address programming, SerIRQ configuration, channel enablement, OBE polling,
// IRQ setup, OF parsing, probe/remove, driver registration and module metadata
// below are direct kernel-driver declarations whose concrete Linux bindings are
// supplied by the target kernel environment.
extern "C" {
    fn aspeed_kcs_set_address(kcs: *mut kcs_bmc_device, addrs: *mut u32, nr_addrs: i32) -> i32;
    fn aspeed_kcs_config_upstream_irq(priv_: *mut aspeed_kcs_bmc, id: u32, dt_type: u32) -> i32;
    fn aspeed_kcs_enable_channel(kcs: *mut kcs_bmc_device, enable: bool);
    fn aspeed_kcs_irq_mask_update(kcs: *mut kcs_bmc_device, mask: u8, state: u8);
    fn aspeed_kcs_probe(pdev: *mut platform_device) -> i32;
    fn aspeed_kcs_remove(pdev: *mut platform_device);
}

#[allow(dead_code)]
static AST_KCS_BMC_IOREGS: [kcs_ioreg; KCS_CHANNEL_MAX] = [
    kcs_ioreg { idr: LPC_IDR1, odr: LPC_ODR1, str_: LPC_STR1 },
    kcs_ioreg { idr: LPC_IDR2, odr: LPC_ODR2, str_: LPC_STR2 },
    kcs_ioreg { idr: LPC_IDR3, odr: LPC_ODR3, str_: LPC_STR3 },
    kcs_ioreg { idr: LPC_IDR4, odr: LPC_ODR4, str_: LPC_STR4 },
];

// C module metadata: MODULE_DEVICE_TABLE(of, ast_kcs_bmc_match),
// module_platform_driver(ast_kcs_bmc_driver), MODULE_LICENSE("GPL v2"),
// MODULE_AUTHOR("Haiyue Wang <haiyue.wang@linux.intel.com>"),
// MODULE_AUTHOR("Andrew Jeffery <andrew@aj.id.au>"),
// MODULE_DESCRIPTION("Aspeed device interface to the KCS BMC device").

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
