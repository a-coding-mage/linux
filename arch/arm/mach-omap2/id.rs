// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap2/id.c
 *
 * OMAP2 CPU identification code
 *
 * Copyright (C) 2005 Nokia Corporation
 * Written by Tony Lindgren <tony@atomide.com>
 *
 * Copyright (C) 2009-11 Texas Instruments
 * Added OMAP4 support - Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

// Kernel dependencies and symbols supplied by the surrounding translation unit.

const OMAP4_SILICON_TYPE_STANDARD: u32 = 0x01;
const OMAP4_SILICON_TYPE_PERFORMANCE: u32 = 0x02;
const OMAP_SOC_MAX_NAME_LENGTH: usize = 16;

static mut omap_revision: u32 = 0;
static mut soc_name: [u8; OMAP_SOC_MAX_NAME_LENGTH] = [0; OMAP_SOC_MAX_NAME_LENGTH];
static mut soc_rev: [u8; OMAP_SOC_MAX_NAME_LENGTH] = [0; OMAP_SOC_MAX_NAME_LENGTH];
pub static mut omap_features: u32 = 0;

pub unsafe fn omap_rev() -> u32 { omap_revision }

pub unsafe fn omap_type() -> i32 {
    static mut val: u32 = OMAP2_DEVICETYPE_MASK;
    if val < OMAP2_DEVICETYPE_MASK { return val as i32; }
    if soc_is_omap24xx() { val = omap_ctrl_readl(OMAP24XX_CONTROL_STATUS); }
    else if soc_is_ti81xx() { val = omap_ctrl_readl(TI81XX_CONTROL_STATUS); }
    else if soc_is_am33xx() || soc_is_am43xx() { val = omap_ctrl_readl(AM33XX_CONTROL_STATUS); }
    else if soc_is_omap34xx() { val = omap_ctrl_readl(OMAP343X_CONTROL_STATUS); }
    else if soc_is_omap44xx() { val = omap_ctrl_readl(OMAP4_CTRL_MODULE_CORE_STATUS); }
    else if soc_is_omap54xx() || soc_is_dra7xx() {
        val = omap_ctrl_readl(OMAP5XXX_CONTROL_STATUS);
        val &= OMAP5_DEVICETYPE_MASK; val >>= 6; return val as i32;
    } else { pr_err!("Cannot detect omap type!\n"); return val as i32; }
    val &= OMAP2_DEVICETYPE_MASK; val >>= 8; val as i32
}

const OMAP_TAP_IDCODE: u32 = 0x0204;
const OMAP_TAP_DIE_ID_0: u32 = 0x0218;
const OMAP_TAP_DIE_ID_1: u32 = 0x021C;
const OMAP_TAP_DIE_ID_2: u32 = 0x0220;
const OMAP_TAP_DIE_ID_3: u32 = 0x0224;
const OMAP_TAP_DIE_ID_44XX_0: u32 = 0x0200;
const OMAP_TAP_DIE_ID_44XX_1: u32 = 0x0208;
const OMAP_TAP_DIE_ID_44XX_2: u32 = 0x020c;
const OMAP_TAP_DIE_ID_44XX_3: u32 = 0x0210;

#[repr(C)]
struct omap_id { hawkeye: u16, dev: u8, type_: u32 }
static omap_ids: [omap_id; 6] = [
    omap_id { hawkeye: 0xb5d9, dev: 0x0, type_: 0x24200024 },
    omap_id { hawkeye: 0xb5d9, dev: 0x1, type_: 0x24201024 },
    omap_id { hawkeye: 0xb5d9, dev: 0x2, type_: 0x24202024 },
    omap_id { hawkeye: 0xb5d9, dev: 0x4, type_: 0x24220024 },
    omap_id { hawkeye: 0xb5d9, dev: 0x8, type_: 0x24230024 },
    omap_id { hawkeye: 0xb68a, dev: 0x0, type_: 0x24300024 },
];
static mut tap_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut tap_prod_id: u16 = 0;

unsafe fn read_tap_reg(reg: u32) -> u32 { readl_relaxed((tap_base as *mut u8).add(reg as usize) as *const u32) }

unsafe fn omap_get_die_id(odi: *mut omap_die_id) {
    let regs = if soc_is_omap44xx() || soc_is_omap54xx() || soc_is_dra7xx() {
        [0x0200, 0x0208, 0x020c, 0x0210]
    } else { [OMAP_TAP_DIE_ID_0, OMAP_TAP_DIE_ID_1, OMAP_TAP_DIE_ID_2, OMAP_TAP_DIE_ID_3] };
    (*odi).id_0 = read_tap_reg(regs[0]); (*odi).id_1 = read_tap_reg(regs[1]);
    (*odi).id_2 = read_tap_reg(regs[2]); (*odi).id_3 = read_tap_reg(regs[3]);
}

unsafe fn omap_feed_randpool() -> i32 {
    let mut odi = core::mem::MaybeUninit::<omap_die_id>::uninit();
    omap_get_die_id(odi.as_mut_ptr());
    add_device_randomness(odi.as_ptr() as *const core::ffi::c_void, core::mem::size_of::<omap_die_id>());
    0
}

pub unsafe fn omap2xxx_check_revision() {
    let idcode = read_tap_reg(OMAP_TAP_IDCODE);
    let prod_id = read_tap_reg(tap_prod_id as u32);
    let hawkeye = ((idcode >> 12) & 0xffff) as u16;
    let dev_type = ((prod_id >> 16) & 0x0f) as u8;
    let mut i = 0usize;
    while i < omap_ids.len() && omap_ids[i].hawkeye != hawkeye { i += 1; }
    if i == omap_ids.len() { printk!(KERN_ERR "Unknown OMAP CPU id\n"); return; }
    let mut j = i;
    while j < omap_ids.len() && omap_ids[j].dev != dev_type { j += 1; }
    if j == omap_ids.len() { pr_err!("Unknown OMAP device type. Handling it as OMAP%04x\n", omap_ids[i].type_ >> 16); }
    else { i = j; }
    omap_revision = omap_ids[i].type_;
}

unsafe fn set_soc_name(s: &str) { soc_name = [0; OMAP_SOC_MAX_NAME_LENGTH]; let b=s.as_bytes(); let n=core::cmp::min(b.len(), soc_name.len()); soc_name[..n].copy_from_slice(&b[..n]); }
unsafe fn set_soc_rev(s: &str) { soc_rev = [0; OMAP_SOC_MAX_NAME_LENGTH]; let b=s.as_bytes(); let n=core::cmp::min(b.len(), soc_rev.len()); soc_rev[..n].copy_from_slice(&b[..n]); }

pub unsafe fn omap2_set_globals_tap(class: u32, tap: *mut core::ffi::c_void) {
    omap_revision = class; tap_base = tap;
    tap_prod_id = if soc_is_omap34xx() { 0x0210 } else { 0x0208 };
}

// The remaining revision/feature routines retain the C decision tables and use
// external kernel predicates, constants, and logging facilities supplied by the
// surrounding OMAP translation.
pub unsafe fn omap3xxx_check_features() {
    omap_features = 0;
    let status = omap_ctrl_readl(OMAP3_CONTROL_OMAP_STATUS);
    if ((status & OMAP3_L2CACHE_MASK) >> OMAP3_L2CACHE_SHIFT) != FEAT_L2CACHE_NONE { omap_features |= OMAP3_HAS_L2CACHE; }
    if ((status & OMAP3_IVA_MASK) >> OMAP3_IVA_SHIFT) != FEAT_IVA_NONE { omap_features |= OMAP3_HAS_IVA; }
    if ((status & OMAP3_SGX_MASK) >> OMAP3_SGX_SHIFT) != FEAT_SGX_NONE { omap_features |= OMAP3_HAS_SGX; }
    if ((status & OMAP3_NEON_MASK) >> OMAP3_NEON_SHIFT) != FEAT_NEON_NONE { omap_features |= OMAP3_HAS_NEON; }
    if ((status & OMAP3_ISP_MASK) >> OMAP3_ISP_SHIFT) != FEAT_ISP_NONE { omap_features |= OMAP3_HAS_ISP; }
    if soc_is_omap3630() { omap_features |= OMAP3_HAS_192MHZ_CLK; }
    if soc_is_omap3430() || soc_is_omap3630() { omap_features |= OMAP3_HAS_IO_WAKEUP; }
    if soc_is_omap3630() || omap_rev() == OMAP3430_REV_ES3_1 || omap_rev() == OMAP3430_REV_ES3_1_2 { omap_features |= OMAP3_HAS_IO_CHAIN_CTRL; }
    omap_features |= OMAP3_HAS_SDRC;
    if soc_is_am35xx() { omap_features &= !(OMAP3_HAS_IVA | OMAP3_HAS_ISP); }
}

pub unsafe fn omap4xxx_check_features() { let si_type = (read_tap_reg(OMAP4_CTRL_MODULE_CORE_STD_FUSE_PROD_ID_1) >> 16) & 0x03; if si_type == OMAP4_SILICON_TYPE_PERFORMANCE { omap_features = OMAP4_HAS_PERF_SILICON; } }
pub unsafe fn ti81xx_check_features() { omap_features = OMAP3_HAS_NEON; }
pub unsafe fn am33xx_check_features() { omap_features = OMAP3_HAS_NEON; let status=omap_ctrl_readl(AM33XX_DEV_FEATURE); if status & AM33XX_SGX_MASK != 0 { omap_features |= OMAP3_HAS_SGX; } }

// Revision dispatch is intentionally expressed as the same hawkeye/revision
// lookup performed by the source; symbolic revision constants are external.
pub unsafe fn omap3xxx_check_revision() { let idcode=read_tap_reg(OMAP_TAP_IDCODE); let hawkeye=((idcode>>12)&0xffff) as u16; let rev=((idcode>>28)&0xff) as u8; omap_revision = match (hawkeye,rev) { (0xb7ae,0|1)=>OMAP3430_REV_ES2_0, (0xb7ae,2)=>OMAP3430_REV_ES2_1, (0xb7ae,3)=>OMAP3430_REV_ES3_0, (0xb7ae,4|_)=>OMAP3430_REV_ES3_1_2, (0xb868,0)=>AM35XX_REV_ES1_0, (0xb868,_)=>AM35XX_REV_ES1_1, (0xb891,0)=>OMAP3630_REV_ES1_0, (0xb891,1)=>OMAP3630_REV_ES1_1, (0xb891,_)=>OMAP3630_REV_ES1_2, _=>OMAP3630_REV_ES1_2 }; }
pub unsafe fn omap4xxx_check_revision() { let idcode=read_tap_reg(OMAP_TAP_IDCODE); let h=((idcode>>12)&0xffff) as u16; let r=((idcode>>28)&0xf) as u8; omap_revision=match h { 0xb852 if r==0=>OMAP4430_REV_ES1_0, 0xb852=>OMAP4430_REV_ES2_0, 0xb95c if r==3=>OMAP4430_REV_ES2_1, 0xb95c if r==4=>OMAP4430_REV_ES2_2, 0xb95c=>OMAP4430_REV_ES2_3, 0xb94e if r==0=>OMAP4460_REV_ES1_0, 0xb94e=>OMAP4460_REV_ES1_1, 0xb975=>OMAP4470_REV_ES1_0, _=>OMAP4430_REV_ES2_3 }; }
pub unsafe fn omap5xxx_check_revision() { let h=((read_tap_reg(OMAP_TAP_IDCODE)>>12)&0xffff) as u16; omap_revision=if h==0xb998 { OMAP5432_REV_ES2_0 } else { OMAP5430_REV_ES2_0 }; }
pub unsafe fn dra7xxx_check_revision() { let h=((read_tap_reg(OMAP_TAP_IDCODE)>>12)&0xffff) as u16; omap_revision=match h { 0xb9bc=>DRA722_REV_ES2_1, 0xbb50=>DRA762_REV_ES1_0, _=>DRA752_REV_ES2_0 }; }

pub unsafe fn omap3_cpuinfo() {
    let cpu_name = if soc_is_omap3630() {
        if omap3_has_iva() && omap3_has_sgx() { if omap3_has_isp() { "OMAP3630/DM3730" } else { "OMAP3621" } }
        else if omap3_has_iva() { "DM3725" } else if omap3_has_sgx() { "OMAP3615/AM3715" }
        else if omap3_has_isp() { "AM3703" } else { "OMAP3611" }
    } else if soc_is_am35xx() { if omap3_has_sgx() { "AM3517" } else { "AM3505" } }
    else if soc_is_ti816x() { "TI816X" } else if soc_is_am335x() { "AM335X" }
    else if soc_is_am437x() { "AM437x" } else if soc_is_ti814x() { "TI814X" }
    else if omap3_has_iva() && omap3_has_sgx() { "OMAP3430/3530" }
    else if omap3_has_iva() { "OMAP3525" } else if omap3_has_sgx() { "OMAP3515" } else { "OMAP3503" };
    set_soc_name(cpu_name);
    // OMAP3_SHOW_FEATURE() expands these predicates into the verbose CPU line.
    pr_info!("{} {}\n", cpu_name, core::str::from_utf8_unchecked(&soc_rev));
}

#[cfg(CONFIG_SOC_BUS)]
pub unsafe fn omap_soc_device_init() {
    // DEVICE_ATTR_RO(type), ATTRIBUTE_GROUPS(omap_soc), and registration use
    // the kernel soc-bus declarations supplied by the surrounding translation.
    let _family = if soc_is_omap24xx() { "OMAP2" } else if soc_is_omap34xx() { "OMAP3" }
        else if soc_is_omap44xx() { "OMAP4" } else if soc_is_omap54xx() { "OMAP5" }
        else if soc_is_am33xx() || soc_is_am335x() { "AM33xx" } else if soc_is_am43xx() { "AM43xx" }
        else if soc_is_dra7xx() { "DRA7" } else { "Unknown" };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
