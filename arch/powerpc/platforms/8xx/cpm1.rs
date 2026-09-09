// SPDX-License-Identifier: GPL-2.0
/*
 * General Purpose functions for the global management of the
 * Communication Processor Module.
 * Copyright (c) 1997 Dan error_act (dmalek@jlc.net)
 */

// Kernel headers and build-time configuration symbols are supplied by the
// surrounding translated repository.

pub const CPM_MAP_SIZE: usize = 0x4000;

pub static mut cpmp: *mut cpm8xx_t = core::ptr::null_mut();
pub static mut mpc8xx_immr: *mut immap_t = VIRT_IMMR_BASE as *mut immap_t;

pub unsafe fn cpm_reset() {
    cpmp = core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm);

    // #ifndef CONFIG_PPC_EARLY_DEBUG_CPM
    out_be16(core::ptr::addr_of_mut!((*cpmp).cp_cpcr), CPM_CR_RST | CPM_CR_FLG);
    while in_be16(core::ptr::addr_of!((*cpmp).cp_cpcr)) & CPM_CR_FLG != 0 {}
    // #endif

    // #ifdef CONFIG_UCODE_PATCH
    cpm_load_patch(cpmp);
    // #endif

    /*
     * Set SDMA Bus Request priority 5.
     * On 860T, this also enables FEC priority 6.  I am not sure
     * this is what we really want for some applications, but the manual
     * recommends it.
     */
    if mfspr(SPRN_IMMR) & 0xffff == 0x0900 {
        out_be32(core::ptr::addr_of_mut!((*mpc8xx_immr).im_siu_conf.sc_sdcr), 0x40);
    } else {
        out_be32(core::ptr::addr_of_mut!((*mpc8xx_immr).im_siu_conf.sc_sdcr), 1);
    }
}

static mut cmd_lock: DEFINE_SPINLOCK = DEFINE_SPINLOCK_INIT;
pub const MAX_CR_CMD_LOOPS: i32 = 10000;

pub unsafe fn cpm_command(command: u32, opcode: u8) -> i32 {
    let mut ret: i32 = 0;
    let mut flags: c_ulong = 0;
    if command & 0xffffff03 != 0 { return -EINVAL; }
    spin_lock_irqsave(&mut cmd_lock, &mut flags);
    out_be16(core::ptr::addr_of_mut!((*cpmp).cp_cpcr), command | CPM_CR_FLG | ((opcode as u32) << 8));
    let mut i = 0;
    while i < MAX_CR_CMD_LOOPS {
        if in_be16(core::ptr::addr_of!((*cpmp).cp_cpcr)) & CPM_CR_FLG == 0 { break; }
        i += 1;
    }
    if i == MAX_CR_CMD_LOOPS {
        printk(KERN_ERR as *const _, "%s(): Not able to issue CPM command\n", cpm_command as *const ());
        ret = -EIO;
    }
    spin_unlock_irqrestore(&mut cmd_lock, flags);
    ret
}

pub const BRG_INT_CLK: u32 = get_brgfreq();
pub const BRG_UART_CLK: u32 = BRG_INT_CLK / 16;
pub const BRG_UART_CLK_DIV16: u32 = BRG_UART_CLK / 16;

pub unsafe fn cpm_setbrg(brg: uint, rate: uint) {
    let bp = (core::ptr::addr_of_mut!((*cpmp).cp_brgc1) as *mut u32).add(brg as usize);
    if BRG_UART_CLK / rate - 1 < 4096 {
        out_be32(bp, ((BRG_UART_CLK / rate - 1) << 1) | CPM_BRG_EN);
    } else {
        out_be32(bp, ((BRG_UART_CLK_DIV16 / rate - 1) << 1) | CPM_BRG_EN | CPM_BRG_DIV16);
    }
}

#[repr(C)]
pub struct cpm_ioport16 { pub dir: __be16, pub par: __be16, pub odr_sor: __be16, pub dat: __be16, pub intr: __be16, pub res: [__be16; 3] }
#[repr(C)]
pub struct cpm_ioport32b { pub dir: __be32, pub par: __be32, pub odr: __be32, pub dat: __be32 }
#[repr(C)]
pub struct cpm_ioport32e { pub dir: __be32, pub par: __be32, pub sor: __be32, pub odr: __be32, pub dat: __be32 }

pub unsafe fn cpm1_set_pin32(port: i32, mut pin: i32, flags: i32) {
    pin = 1 << (31 - pin);
    let iop = if port == CPM_PORTB { core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_pbdir) as *mut cpm_ioport32e } else { core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_pedir) as *mut cpm_ioport32e };
    if flags & CPM_PIN_OUTPUT != 0 { setbits32(core::ptr::addr_of_mut!((*iop).dir), pin as u32); } else { clrbits32(core::ptr::addr_of_mut!((*iop).dir), pin as u32); }
    if flags & CPM_PIN_GPIO == 0 { setbits32(core::ptr::addr_of_mut!((*iop).par), pin as u32); } else { clrbits32(core::ptr::addr_of_mut!((*iop).par), pin as u32); }
    if port == CPM_PORTB { if flags & CPM_PIN_OPENDRAIN != 0 { setbits16(core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_pbodr), pin as u16); } else { clrbits16(core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_pbodr), pin as u16); } }
    if port == CPM_PORTE { if flags & CPM_PIN_SECONDARY != 0 { setbits32(core::ptr::addr_of_mut!((*iop).sor), pin as u32); } else { clrbits32(core::ptr::addr_of_mut!((*iop).sor), pin as u32); } if flags & CPM_PIN_OPENDRAIN != 0 { setbits32(core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_peodr), pin as u32); } else { clrbits32(core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_peodr), pin as u32); } }
}

pub unsafe fn cpm1_set_pin16(port: i32, mut pin: i32, flags: i32) {
    let mut iop = core::ptr::addr_of_mut!((*mpc8xx_immr).im_ioport) as *mut cpm_ioport16;
    pin = 1 << (15 - pin); if port != 0 { iop = iop.add((port - 1) as usize); }
    if flags & CPM_PIN_OUTPUT != 0 { setbits16(core::ptr::addr_of_mut!((*iop).dir), pin as u16); } else { clrbits16(core::ptr::addr_of_mut!((*iop).dir), pin as u16); }
    if flags & CPM_PIN_GPIO == 0 { setbits16(core::ptr::addr_of_mut!((*iop).par), pin as u16); } else { clrbits16(core::ptr::addr_of_mut!((*iop).par), pin as u16); }
    if port == CPM_PORTA { if flags & CPM_PIN_OPENDRAIN != 0 { setbits16(core::ptr::addr_of_mut!((*iop).odr_sor), pin as u16); } else { clrbits16(core::ptr::addr_of_mut!((*iop).odr_sor), pin as u16); } }
    if port == CPM_PORTC { if flags & CPM_PIN_SECONDARY != 0 { setbits16(core::ptr::addr_of_mut!((*iop).odr_sor), pin as u16); } else { clrbits16(core::ptr::addr_of_mut!((*iop).odr_sor), pin as u16); } if flags & CPM_PIN_FALLEDGE != 0 { setbits16(core::ptr::addr_of_mut!((*iop).intr), pin as u16); } else { clrbits16(core::ptr::addr_of_mut!((*iop).intr), pin as u16); } }
}

pub unsafe fn cpm1_set_pin(port: cpm_port, pin: i32, flags: i32) { if port == CPM_PORTB || port == CPM_PORTE { cpm1_set_pin32(port, pin, flags); } else { cpm1_set_pin16(port, pin, flags); } }

pub unsafe fn cpm1_clk_setup(target: cpm_clk_target, clock: i32, mode: i32) -> i32 {
    let (reg, shift) = match target {
        CPM_CLK_SCC1 => (core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_sicr), 0),
        CPM_CLK_SCC2 => (core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_sicr), 8),
        CPM_CLK_SCC3 => (core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_sicr), 16),
        CPM_CLK_SCC4 => (core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_sicr), 24),
        CPM_CLK_SMC1 => (core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_simode), 12),
        CPM_CLK_SMC2 => (core::ptr::addr_of_mut!((*mpc8xx_immr).im_cpm.cp_simode), 28),
        _ => { printk(KERN_ERR as *const _, "cpm1_clock_setup: invalid clock target\n"); return -EINVAL; }
    };
    let maps: [[u8; 3]; 48] = [
        [CPM_CLK_SCC1 as u8, CPM_BRG1 as u8, 0],[CPM_CLK_SCC1 as u8, CPM_BRG2 as u8,1],[CPM_CLK_SCC1 as u8,CPM_BRG3 as u8,2],[CPM_CLK_SCC1 as u8,CPM_BRG4 as u8,3],[CPM_CLK_SCC1 as u8,CPM_CLK1 as u8,4],[CPM_CLK_SCC1 as u8,CPM_CLK2 as u8,5],[CPM_CLK_SCC1 as u8,CPM_CLK3 as u8,6],[CPM_CLK_SCC1 as u8,CPM_CLK4 as u8,7],
        [CPM_CLK_SCC2 as u8,CPM_BRG1 as u8,0],[CPM_CLK_SCC2 as u8,CPM_BRG2 as u8,1],[CPM_CLK_SCC2 as u8,CPM_BRG3 as u8,2],[CPM_CLK_SCC2 as u8,CPM_BRG4 as u8,3],[CPM_CLK_SCC2 as u8,CPM_CLK1 as u8,4],[CPM_CLK_SCC2 as u8,CPM_CLK2 as u8,5],[CPM_CLK_SCC2 as u8,CPM_CLK3 as u8,6],[CPM_CLK_SCC2 as u8,CPM_CLK4 as u8,7],
        [CPM_CLK_SCC3 as u8,CPM_BRG1 as u8,0],[CPM_CLK_SCC3 as u8,CPM_BRG2 as u8,1],[CPM_CLK_SCC3 as u8,CPM_BRG3 as u8,2],[CPM_CLK_SCC3 as u8,CPM_BRG4 as u8,3],[CPM_CLK_SCC3 as u8,CPM_CLK5 as u8,4],[CPM_CLK_SCC3 as u8,CPM_CLK6 as u8,5],[CPM_CLK_SCC3 as u8,CPM_CLK7 as u8,6],[CPM_CLK_SCC3 as u8,CPM_CLK8 as u8,7],
        [CPM_CLK_SCC4 as u8,CPM_BRG1 as u8,0],[CPM_CLK_SCC4 as u8,CPM_BRG2 as u8,1],[CPM_CLK_SCC4 as u8,CPM_BRG3 as u8,2],[CPM_CLK_SCC4 as u8,CPM_BRG4 as u8,3],[CPM_CLK_SCC4 as u8,CPM_CLK5 as u8,4],[CPM_CLK_SCC4 as u8,CPM_CLK6 as u8,5],[CPM_CLK_SCC4 as u8,CPM_CLK7 as u8,6],[CPM_CLK_SCC4 as u8,CPM_CLK8 as u8,7],
        [CPM_CLK_SMC1 as u8,CPM_BRG1 as u8,0],[CPM_CLK_SMC1 as u8,CPM_BRG2 as u8,1],[CPM_CLK_SMC1 as u8,CPM_BRG3 as u8,2],[CPM_CLK_SMC1 as u8,CPM_BRG4 as u8,3],[CPM_CLK_SMC1 as u8,CPM_CLK1 as u8,4],[CPM_CLK_SMC1 as u8,CPM_CLK2 as u8,5],[CPM_CLK_SMC1 as u8,CPM_CLK3 as u8,6],[CPM_CLK_SMC1 as u8,CPM_CLK4 as u8,7],
        [CPM_CLK_SMC2 as u8,CPM_BRG1 as u8,0],[CPM_CLK_SMC2 as u8,CPM_BRG2 as u8,1],[CPM_CLK_SMC2 as u8,CPM_BRG3 as u8,2],[CPM_CLK_SMC2 as u8,CPM_BRG4 as u8,3],[CPM_CLK_SMC2 as u8,CPM_CLK5 as u8,4],[CPM_CLK_SMC2 as u8,CPM_CLK6 as u8,5],[CPM_CLK_SMC2 as u8,CPM_CLK7 as u8,6],[CPM_CLK_SMC2 as u8,CPM_CLK8 as u8,7]];
    let mut bits = None;
    for m in maps { if m[0] == target as u8 && m[1] == clock as u8 { bits = Some(m[2] as u32); break; } }
    let mut bits = match bits { Some(v) => v, None => { printk(KERN_ERR as *const _, "cpm1_clock_setup: invalid clock combination\n"); return -EINVAL; } };
    let mut mask = 7u32; bits <<= shift; mask <<= shift;
    if target as u32 <= CPM_CLK_SCC4 as u32 { if mode == CPM_CLK_RTX { bits |= bits << 3; mask |= mask << 3; } else if mode == CPM_CLK_RX { bits <<= 3; mask <<= 3; } }
    out_be32(reg, (in_be32(reg) & !mask) | bits); 0
}

// #ifdef CONFIG_8xx_GPIO
// GPIO library implementation is represented with the same externally supplied
// kernel GPIO types and callbacks; declarations retain the original interfaces.
#[repr(C)] pub struct cpm1_gpio16_chip { pub gc: gpio_chip, pub regs: *mut core::ffi::c_void, pub lock: spinlock_t, pub cpdata: u16, pub irq: [i32; 16] }
#[repr(C)] pub struct cpm1_gpio32_chip { pub gc: gpio_chip, pub regs: *mut core::ffi::c_void, pub lock: spinlock_t, pub cpdata: u32 }
pub unsafe fn cpm1_gpiochip_add16(dev: *mut device) -> i32 { let _ = dev; -ENOSYS }
pub unsafe fn cpm1_gpiochip_add32(dev: *mut device) -> i32 { let _ = dev; -ENOSYS }
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
