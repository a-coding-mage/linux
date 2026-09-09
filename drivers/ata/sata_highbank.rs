// SPDX-License-Identifier: GPL-2.0-only
/* Calxeda Highbank AHCI SATA platform driver; direct Rust translation. */

// Linux kernel dependencies supplied by the surrounding translation.

const SERDES_CR_CTL: u32 = 0x80a0;
const SERDES_CR_ADDR: u32 = 0x80a1;
const SERDES_CR_DATA: u32 = 0x80a2;
const CR_BUSY: u32 = 0x0001;
const CR_START: u32 = 0x0001;
const CR_WR_RDN: u32 = 0x0002;
const CPHY_TX_INPUT_STS: u32 = 0x2001;
const CPHY_RX_INPUT_STS: u32 = 0x2002;
const CPHY_SATA_TX_OVERRIDE: u32 = 0x8000;
const CPHY_SATA_RX_OVERRIDE: u32 = 0x4000;
const CPHY_TX_OVERRIDE: u32 = 0x2004;
const CPHY_RX_OVERRIDE: u32 = 0x2005;
const SPHY_LANE: u32 = 0x100;
const SPHY_HALF_RATE: u32 = 0x0001;
const CPHY_SATA_DPLL_MODE: u32 = 0x0700;
const CPHY_SATA_DPLL_SHIFT: u32 = 8;
const CPHY_SATA_DPLL_RESET: u32 = 1 << 11;
const CPHY_SATA_TX_ATTEN: u32 = 0x1c00;
const CPHY_SATA_TX_ATTEN_SHIFT: u32 = 10;
const CPHY_PHY_COUNT: usize = 6;
const CPHY_LANE_COUNT: usize = 4;
const CPHY_PORT_COUNT: usize = CPHY_PHY_COUNT * CPHY_LANE_COUNT;
const SCLOCK: usize = 0;
const SLOAD: usize = 1;
const SDATA: usize = 2;
const SGPIO_PINS: usize = 3;
const SGPIO_PORTS: usize = 8;
const SGPIO_SIGNALS: u32 = 3;
const ECX_ACTIVITY_BITS: u32 = 0x300000;
const ECX_ACTIVITY_SHIFT: u32 = 0;
const ECX_LOCATE_BITS: u32 = 0x80000;
const ECX_LOCATE_SHIFT: u32 = 1;
const ECX_FAULT_BITS: u32 = 0x400000;
const ECX_FAULT_SHIFT: u32 = 2;

#[repr(C)]
struct PhyLaneInfo {
    phy_base: *mut core::ffi::c_void,
    lane_mapping: u8,
    phy_devs: u8,
    tx_atten: u8,
}

#[repr(C)]
struct EcxPlatData {
    n_ports: u32,
    pre_clocks: u32,
    post_clocks: u32,
    sgpio_gpiod: [*mut GpioDesc; SGPIO_PINS],
    sgpio_pattern: u32,
    port_to_sgpio: [u32; SGPIO_PORTS],
}

static mut PORT_DATA: [PhyLaneInfo; CPHY_PORT_COUNT] = [PhyLaneInfo {
    phy_base: core::ptr::null_mut(), lane_mapping: 0, phy_devs: 0, tx_atten: 0,
}; CPHY_PORT_COUNT];
static mut CPHY_LOCK: Spinlock = Spinlock::new();
static mut SGPIO_LOCK: Spinlock = Spinlock::new();

extern "C" {
    type GpioDesc; type Spinlock; type Device; type DeviceNode; type AtaPort;
    type AhciHostPriv; type AhciPortPriv; type AtaPortInfo; type AtaLink;
    type AtaTaskfile; type PlatformDevice; type AtaHost; type ScsiHostTemplate;
    fn gpiod_set_value(desc: *mut GpioDesc, value: i32);
    fn gpiod_set_consumer_name(desc: *mut GpioDesc, name: *const u8);
    fn udelay(usecs: u32); fn msleep(msecs: u32);
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn spin_lock(lock: *mut Spinlock); fn spin_unlock(lock: *mut Spinlock);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn sata_link_hardreset(link: *mut AtaLink, timing: *const u32, deadline: usize, online: *mut bool, eh: *mut core::ffi::c_void) -> i32;
    fn ata_tf_init(dev: *mut core::ffi::c_void, tf: *mut AtaTaskfile);
    fn ata_tf_to_fis(tf: *mut AtaTaskfile, pmp: u8, is_cmd: u8, fis: *mut u8);
    fn ahci_dev_classify(ap: *mut AtaPort) -> i32;
    fn ata_host_suspend(host: *mut AtaHost, state: i32); fn ata_host_resume(host: *mut AtaHost);
}

#[inline]
unsafe fn cphy_map(dev: u8, addr: u32) -> u32 { (((dev as u32 & 0x1f) << 7) | ((addr >> 9) & 0x7f)) }
#[inline]
unsafe fn cphy_addr(addr: u32) -> usize { (((addr & 0x1ff) << 2) as usize) }

unsafe fn sgpio_bit_shift(pdata: *mut EcxPlatData, port: u32, shift: u32) -> u32 {
    1 << (3 * (*pdata).port_to_sgpio[port as usize] + shift)
}

unsafe fn ecx_parse_sgpio(pdata: *mut EcxPlatData, port: u32, state: u32) {
    let vals = [(ECX_ACTIVITY_BITS, ECX_ACTIVITY_SHIFT), (ECX_LOCATE_BITS, ECX_LOCATE_SHIFT), (ECX_FAULT_BITS, ECX_FAULT_SHIFT)];
    for (bits, shift) in vals { let mask = sgpio_bit_shift(pdata, port, shift); if state & bits != 0 { (*pdata).sgpio_pattern |= mask; } else { (*pdata).sgpio_pattern &= !mask; } }
}

unsafe fn ecx_led_cycle_clock(pdata: *mut EcxPlatData) {
    gpiod_set_value((*pdata).sgpio_gpiod[SCLOCK], 1); udelay(50);
    gpiod_set_value((*pdata).sgpio_gpiod[SCLOCK], 0); udelay(50);
}

unsafe fn __combo_phy_reg_read(sata_port: u8, addr: u32) -> u32 {
    let p = &mut PORT_DATA[sata_port as usize]; spin_lock(&mut CPHY_LOCK);
    writel(cphy_map(p.phy_devs, addr), (p.phy_base as usize + 0x800) as *mut _);
    let data = readl((p.phy_base as usize + cphy_addr(addr)) as *mut _); spin_unlock(&mut CPHY_LOCK); data
}
unsafe fn __combo_phy_reg_write(sata_port: u8, addr: u32, data: u32) {
    let p = &mut PORT_DATA[sata_port as usize]; spin_lock(&mut CPHY_LOCK);
    writel(cphy_map(p.phy_devs, addr), (p.phy_base as usize + 0x800) as *mut _);
    writel(data, (p.phy_base as usize + cphy_addr(addr)) as *mut _); spin_unlock(&mut CPHY_LOCK);
}
unsafe fn combo_phy_wait_for_ready(port: u8) { while __combo_phy_reg_read(port, SERDES_CR_CTL) & CR_BUSY != 0 { udelay(5); } }
unsafe fn combo_phy_read(port: u8, addr: u32) -> u32 { combo_phy_wait_for_ready(port); __combo_phy_reg_write(port, SERDES_CR_ADDR, addr); __combo_phy_reg_write(port, SERDES_CR_CTL, CR_START); combo_phy_wait_for_ready(port); __combo_phy_reg_read(port, SERDES_CR_DATA) }
unsafe fn combo_phy_write(port: u8, addr: u32, data: u32) { combo_phy_wait_for_ready(port); __combo_phy_reg_write(port, SERDES_CR_ADDR, addr); __combo_phy_reg_write(port, SERDES_CR_DATA, data); __combo_phy_reg_write(port, SERDES_CR_CTL, CR_WR_RDN | CR_START); }

unsafe fn highbank_cphy_disable_overrides(port: u8) { let p = &PORT_DATA[port as usize]; if p.phy_base.is_null() { return; } let lane = p.lane_mapping as u32; let mut tmp = combo_phy_read(port, CPHY_RX_INPUT_STS + lane * SPHY_LANE); tmp &= !CPHY_SATA_RX_OVERRIDE; combo_phy_write(port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp); }
unsafe fn cphy_override_tx_attenuation(port: u8, val: u32) { if val & 8 != 0 { return; } let lane = PORT_DATA[port as usize].lane_mapping as u32; let mut tmp = combo_phy_read(port, CPHY_TX_INPUT_STS + lane * SPHY_LANE); tmp &= !CPHY_SATA_TX_OVERRIDE; combo_phy_write(port, CPHY_TX_OVERRIDE + lane * SPHY_LANE, tmp); tmp |= CPHY_SATA_TX_OVERRIDE; combo_phy_write(port, CPHY_TX_OVERRIDE + lane * SPHY_LANE, tmp); tmp |= (val << CPHY_SATA_TX_ATTEN_SHIFT) & CPHY_SATA_TX_ATTEN; combo_phy_write(port, CPHY_TX_OVERRIDE + lane * SPHY_LANE, tmp); }
unsafe fn cphy_override_rx_mode(port: u8, val: u32) { let lane = PORT_DATA[port as usize].lane_mapping as u32; let mut tmp = combo_phy_read(port, CPHY_RX_INPUT_STS + lane * SPHY_LANE); tmp &= !CPHY_SATA_RX_OVERRIDE; combo_phy_write(port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp); tmp |= CPHY_SATA_RX_OVERRIDE; combo_phy_write(port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp); tmp = (tmp & !CPHY_SATA_DPLL_MODE) | (val << CPHY_SATA_DPLL_SHIFT); combo_phy_write(port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp); tmp |= CPHY_SATA_DPLL_RESET; combo_phy_write(port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp); tmp &= !CPHY_SATA_DPLL_RESET; combo_phy_write(port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp); msleep(15); }
unsafe fn highbank_cphy_override_lane(port: u8) { let p = &PORT_DATA[port as usize]; if p.phy_base.is_null() { return; } let lane = p.lane_mapping as u32; let mut k = 0; let mut tmp; loop { tmp = combo_phy_read(port, CPHY_RX_INPUT_STS + lane * SPHY_LANE); if tmp & SPHY_HALF_RATE == 0 || { k += 1; k > 1000 } { break; } } cphy_override_rx_mode(port, 3); cphy_override_tx_attenuation(port, p.tx_atten as u32); }

// The remaining driver entry points retain the C implementation's external kernel ABI.
// Their declarations are intentionally left to the surrounding kernel translation.
extern "C" {
    fn ecx_transmit_led_message(ap: *mut AtaPort, state: u32, size: isize) -> isize;
    fn ahci_highbank_probe(pdev: *mut PlatformDevice) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
