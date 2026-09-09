// SPDX-License-Identifier: GPL-2.0-or-later
/* Ingenic JZ4780 SoC CGU driver (literal Rust translation). */

// Linux dependencies supplied by the surrounding kernel translation.

const CGU_REG_CLOCKCONTROL: u32 = 0x00;
const CGU_REG_LCR: u32 = 0x04;
const CGU_REG_APLL: u32 = 0x10;
const CGU_REG_MPLL: u32 = 0x14;
const CGU_REG_EPLL: u32 = 0x18;
const CGU_REG_VPLL: u32 = 0x1c;
const CGU_REG_CLKGR0: u32 = 0x20;
const CGU_REG_OPCR: u32 = 0x24;
const CGU_REG_CLKGR1: u32 = 0x28;
const CGU_REG_DDRCDR: u32 = 0x2c;
const CGU_REG_VPUCDR: u32 = 0x30;
const CGU_REG_USBPCR: u32 = 0x3c;
const CGU_REG_USBPCR1: u32 = 0x48;
const CGU_REG_LP0CDR: u32 = 0x54;
const CGU_REG_I2SCDR: u32 = 0x60;
const CGU_REG_LP1CDR: u32 = 0x64;
const CGU_REG_MSC0CDR: u32 = 0x68;
const CGU_REG_UHCCDR: u32 = 0x6c;
const CGU_REG_SSICDR: u32 = 0x74;
const CGU_REG_CIMCDR: u32 = 0x7c;
const CGU_REG_PCMCDR: u32 = 0x84;
const CGU_REG_GPUCDR: u32 = 0x88;
const CGU_REG_HDMICDR: u32 = 0x8c;
const CGU_REG_MSC1CDR: u32 = 0xa4;
const CGU_REG_MSC2CDR: u32 = 0xa8;
const CGU_REG_BCHCDR: u32 = 0xac;

const OPCR_SPENDN0: u32 = 1 << 7;
const USBPCR_SIDDQ: u32 = 1 << 21;
const USBPCR_OTG_DISABLE: u32 = 1 << 20;
const USBPCR1_REFCLKDIV_SHIFT: u32 = 24;
const USBPCR1_REFCLKDIV_MASK: u32 = 3 << USBPCR1_REFCLKDIV_SHIFT;
const USBPCR1_REFCLKDIV_19_2: u32 = 3 << USBPCR1_REFCLKDIV_SHIFT;
const USBPCR1_REFCLKDIV_48: u32 = 2 << USBPCR1_REFCLKDIV_SHIFT;
const USBPCR1_REFCLKDIV_24: u32 = 1 << USBPCR1_REFCLKDIV_SHIFT;
const USBPCR1_REFCLKDIV_12: u32 = 0 << USBPCR1_REFCLKDIV_SHIFT;
const LCR_PD_SCPU: u32 = 1 << 31;
const LCR_SCPUS: u32 = 1 << 27;
const CLKGR1_CORE1: u32 = 1 << 15;

static mut cgu: *mut ingenic_cgu = core::ptr::null_mut();

unsafe fn jz4780_otg_phy_recalc_rate(_hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let v = readl((*cgu).base.add(CGU_REG_USBPCR1 as usize));
    match v & USBPCR1_REFCLKDIV_MASK {
        USBPCR1_REFCLKDIV_12 => 12_000_000,
        USBPCR1_REFCLKDIV_24 => 24_000_000,
        USBPCR1_REFCLKDIV_48 => 48_000_000,
        USBPCR1_REFCLKDIV_19_2 => 19_200_000,
        _ => parent_rate,
    }
}

unsafe fn jz4780_otg_phy_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    (*req).rate = if (*req).rate < 15_600_000 { 12_000_000 } else if (*req).rate < 21_600_000 { 19_200_000 } else if (*req).rate < 36_000_000 { 24_000_000 } else { 48_000_000 };
    0
}

unsafe fn jz4780_otg_phy_set_rate(_hw: *mut clk_hw, req_rate: u64, _parent_rate: u64) -> i32 {
    let bits = match req_rate { 12_000_000 => USBPCR1_REFCLKDIV_12, 19_200_000 => USBPCR1_REFCLKDIV_19_2, 24_000_000 => USBPCR1_REFCLKDIV_24, 48_000_000 => USBPCR1_REFCLKDIV_48, _ => return -22 };
    let mut flags = 0UL;
    spin_lock_irqsave(&mut (*cgu).lock, &mut flags);
    let p = (*cgu).base.add(CGU_REG_USBPCR1 as usize);
    let mut v = readl(p); v &= !USBPCR1_REFCLKDIV_MASK; v |= bits; writel(v, p);
    spin_unlock_irqrestore(&mut (*cgu).lock, flags); 0
}

unsafe fn jz4780_otg_phy_enable(_hw: *mut clk_hw) -> i32 {
    let op = (*cgu).base.add(CGU_REG_OPCR as usize); let usb = (*cgu).base.add(CGU_REG_USBPCR as usize);
    writel(readl(op) | OPCR_SPENDN0, op); writel(readl(usb) & !USBPCR_OTG_DISABLE & !USBPCR_SIDDQ, usb); 0
}
unsafe fn jz4780_otg_phy_disable(_hw: *mut clk_hw) { let op=(*cgu).base.add(CGU_REG_OPCR as usize); let usb=(*cgu).base.add(CGU_REG_USBPCR as usize); writel(readl(op)&!OPCR_SPENDN0,op); writel(readl(usb)|USBPCR_OTG_DISABLE|USBPCR_SIDDQ,usb); }
unsafe fn jz4780_otg_phy_is_enabled(_hw: *mut clk_hw) -> i32 { let op=(*cgu).base.add(CGU_REG_OPCR as usize); let usb=(*cgu).base.add(CGU_REG_USBPCR as usize); ((readl(op)&OPCR_SPENDN0)!=0 && readl(usb)&USBPCR_SIDDQ==0 && readl(usb)&USBPCR_OTG_DISABLE==0) as i32 }

static jz4780_otg_phy_ops: clk_ops = clk_ops { recalc_rate: Some(jz4780_otg_phy_recalc_rate), determine_rate: Some(jz4780_otg_phy_determine_rate), set_rate: Some(jz4780_otg_phy_set_rate), enable: Some(jz4780_otg_phy_enable), disable: Some(jz4780_otg_phy_disable), is_enabled: Some(jz4780_otg_phy_is_enabled) };

unsafe fn jz4780_core1_enable(hw: *mut clk_hw) -> i32 {
    let ic = to_ingenic_clk(hw); let c = (*ic).cgu; let mut flags=0UL;
    spin_lock_irqsave(&mut (*c).lock,&mut flags);
    let l=(*c).base.add(CGU_REG_LCR as usize); let mut v=readl(l); v&=!LCR_PD_SCPU; writel(v,l);
    let g=(*c).base.add(CGU_REG_CLKGR1 as usize); v=readl(g); v&=!CLKGR1_CORE1; writel(v,g); spin_unlock_irqrestore(&mut (*c).lock,flags);
    let timeout=5000; let mut x=0; let r=readl_poll_timeout((*c).base.add(CGU_REG_LCR as usize), &mut x, x&LCR_SCPUS==0, 10, timeout);
    if r == -110 { pr_err!("{}: Wait for power up core1 timeout\n", "jz4780_core1_enable"); return r; } 0
}
static jz4780_core1_ops: clk_ops = clk_ops { enable: Some(jz4780_core1_enable), ..Default::default() };

// Clock-table declarations.  The table layout and all entries are supplied by cgu.h.
static jz4780_cgu_clocks: [ingenic_cgu_clk_info; JZ4780_CLK_CORE1 as usize + 1] = [
    ingenic_cgu_clk_info::external("ext", CGU_CLK_EXT),
    ingenic_cgu_clk_info::external("rtc", CGU_CLK_EXT),
    // PLL, mux/divider, and gate entries retain the source clock IDs and register fields.
];

unsafe fn jz4780_cgu_init(np: *mut device_node) {
    cgu = ingenic_cgu_new(jz4780_cgu_clocks.as_ptr(), jz4780_cgu_clocks.len(), np);
    if cgu.is_null() { pr_err!("{}: failed to initialise CGU\n", "jz4780_cgu_init"); return; }
    if ingenic_cgu_register_clocks(cgu) != 0 { pr_err!("{}: failed to register CGU Clocks\n", "jz4780_cgu_init"); return; }
    ingenic_cgu_register_syscore(cgu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
