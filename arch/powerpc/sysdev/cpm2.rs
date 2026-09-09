/*
 * General Purpose functions for the global management of the
 * 8260 Communication Processor Module.
 * Copyright (c) 1999-2001 Dan Malek <dan@embeddedalley.com>
 * Copyright (c) 2000 MontaVista Software, Inc (source@mvista.com)
 *	2.3.99 Updates
 *
 * 2006 (c) MontaVista Software, Inc.
 * Vitaly Bordug <vbordug@ru.mvista.com>
 * 	Merged to arch/powerpc from arch/ppc/syslib/cpm2_common.c
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// Kernel and architecture headers from the original implementation supply
// the types, constants, register structures, and I/O helpers referenced here.

extern "C" {
    static mut cpmp: *mut cpm_cpm2_t;
    static mut cpm2_immr: *mut cpm2_map_t;
    fn get_immrbase() -> usize;
    fn ioremap(addr: usize, size: usize) -> *mut cpm2_map_t;
    fn out_be32(addr: *mut u32, value: u32);
    fn in_be32(addr: *const u32) -> u32;
    fn out_8(addr: *mut u8, value: u8);
    fn in_8(addr: *const u8) -> u8;
    fn setbits32(addr: *mut u32, value: u32);
    fn clrbits32(addr: *mut u32, value: u32);
    fn printk(fmt: *const u8, ...) -> i32;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
}

#[repr(C)] pub struct cpm_cpm2_t { pub cp_cpcr: u32 }
#[repr(C)] pub struct cpm2_cpmux_t { pub cmx_scr: u32, pub cmx_fcr: u32, pub cmx_smr: u8 }
#[repr(C)] pub struct cpm2_map_t {
    pub im_cpm: cpm_cpm2_t,
    pub im_brgc1: u32,
    pub im_brgc5: u32,
    pub im_cpmux: cpm2_cpmux_t,
    pub im_ioport: [u8; 0],
}
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

pub const CPM_MAP_SIZE: usize = 0x40000;
pub const MAX_CR_CMD_LOOPS: i32 = 10000;

pub unsafe fn cpm2_reset() {
    #[cfg(feature = "CONFIG_PPC_85xx")]
    { cpm2_immr = ioremap(get_immrbase() + 0x80000, CPM_MAP_SIZE); }
    #[cfg(not(feature = "CONFIG_PPC_85xx"))]
    { cpm2_immr = ioremap(get_immrbase(), CPM_MAP_SIZE); }
    cpmp = &mut (*cpm2_immr).im_cpm;
    #[cfg(not(feature = "CONFIG_PPC_EARLY_DEBUG_CPM"))]
    { cpm_command( CPM_CR_RST, 0); }
}

static mut cmd_lock: spinlock_t = spinlock_t { _private: [] };

pub unsafe fn cpm_command(command: u32, opcode: u8) -> i32 {
    let mut flags: usize = 0;
    spin_lock_irqsave(&mut cmd_lock, &mut flags);
    let mut ret: i32 = 0;
    out_be32(&mut (*cpmp).cp_cpcr, command | opcode as u32 | CPM_CR_FLG);
    let mut i = 0;
    while i < MAX_CR_CMD_LOOPS {
        if (in_be32(&(*cpmp).cp_cpcr) & CPM_CR_FLG) == 0 { break; }
        i += 1;
    }
    if i == MAX_CR_CMD_LOOPS {
        ret = -EIO;
    }
    spin_unlock_irqrestore(&mut cmd_lock, flags);
    ret
}

pub unsafe fn __cpm2_setbrg(mut brg: u32, rate: u32, clk: u32, div16: i32, src: i32) {
    let mut bp: *mut u32;
    if brg < 4 { bp = &mut (*cpm2_immr).im_brgc1; }
    else { bp = &mut (*cpm2_immr).im_brgc5; brg -= 4; }
    bp = bp.add(brg as usize);
    let mut val = (((clk.wrapping_mul(2) / rate).wrapping_sub(1)) & !1) | CPM_BRG_EN | src as u32;
    if div16 != 0 { val |= CPM_BRG_DIV16; }
    out_be32(bp, val);
}

pub unsafe fn cpm2_clk_setup(target: cpm_clk_target, clock: i32, mode: i32) -> i32 {
    let mut shift: u32;
    let (reg, s) = match target {
        CPM_CLK_SCC1 => (&mut (*cpm2_immr).im_cpmux.cmx_scr as *mut u32, 24),
        CPM_CLK_SCC2 => (&mut (*cpm2_immr).im_cpmux.cmx_scr as *mut u32, 16),
        CPM_CLK_SCC3 => (&mut (*cpm2_immr).im_cpmux.cmx_scr as *mut u32, 8),
        CPM_CLK_SCC4 => (&mut (*cpm2_immr).im_cpmux.cmx_scr as *mut u32, 0),
        CPM_CLK_FCC1 => (&mut (*cpm2_immr).im_cpmux.cmx_fcr as *mut u32, 24),
        CPM_CLK_FCC2 => (&mut (*cpm2_immr).im_cpmux.cmx_fcr as *mut u32, 16),
        CPM_CLK_FCC3 => (&mut (*cpm2_immr).im_cpmux.cmx_fcr as *mut u32, 8),
        _ => return -EINVAL,
    };
    shift = s;
    let map: &[[u8; 3]] = &[
        [CPM_CLK_FCC1 as u8, CPM_BRG5 as u8,0],[CPM_CLK_FCC1 as u8,CPM_BRG6 as u8,1],[CPM_CLK_FCC1 as u8,CPM_BRG7 as u8,2],[CPM_CLK_FCC1 as u8,CPM_BRG8 as u8,3],[CPM_CLK_FCC1 as u8,CPM_CLK9 as u8,4],[CPM_CLK_FCC1 as u8,CPM_CLK10 as u8,5],[CPM_CLK_FCC1 as u8,CPM_CLK11 as u8,6],[CPM_CLK_FCC1 as u8,CPM_CLK12 as u8,7],
        [CPM_CLK_FCC2 as u8,CPM_BRG5 as u8,0],[CPM_CLK_FCC2 as u8,CPM_BRG6 as u8,1],[CPM_CLK_FCC2 as u8,CPM_BRG7 as u8,2],[CPM_CLK_FCC2 as u8,CPM_BRG8 as u8,3],[CPM_CLK_FCC2 as u8,CPM_CLK13 as u8,4],[CPM_CLK_FCC2 as u8,CPM_CLK14 as u8,5],[CPM_CLK_FCC2 as u8,CPM_CLK15 as u8,6],[CPM_CLK_FCC2 as u8,CPM_CLK16 as u8,7],
        [CPM_CLK_FCC3 as u8,CPM_BRG5 as u8,0],[CPM_CLK_FCC3 as u8,CPM_BRG6 as u8,1],[CPM_CLK_FCC3 as u8,CPM_BRG7 as u8,2],[CPM_CLK_FCC3 as u8,CPM_BRG8 as u8,3],[CPM_CLK_FCC3 as u8,CPM_CLK13 as u8,4],[CPM_CLK_FCC3 as u8,CPM_CLK14 as u8,5],[CPM_CLK_FCC3 as u8,CPM_CLK15 as u8,6],[CPM_CLK_FCC3 as u8,CPM_CLK16 as u8,7],
        [CPM_CLK_SCC1 as u8,CPM_BRG1 as u8,0],[CPM_CLK_SCC1 as u8,CPM_BRG2 as u8,1],[CPM_CLK_SCC1 as u8,CPM_BRG3 as u8,2],[CPM_CLK_SCC1 as u8,CPM_BRG4 as u8,3],[CPM_CLK_SCC1 as u8,CPM_CLK11 as u8,4],[CPM_CLK_SCC1 as u8,CPM_CLK12 as u8,5],[CPM_CLK_SCC1 as u8,CPM_CLK3 as u8,6],[CPM_CLK_SCC1 as u8,CPM_CLK4 as u8,7],
        [CPM_CLK_SCC2 as u8,CPM_BRG1 as u8,0],[CPM_CLK_SCC2 as u8,CPM_BRG2 as u8,1],[CPM_CLK_SCC2 as u8,CPM_BRG3 as u8,2],[CPM_CLK_SCC2 as u8,CPM_BRG4 as u8,3],[CPM_CLK_SCC2 as u8,CPM_CLK11 as u8,4],[CPM_CLK_SCC2 as u8,CPM_CLK12 as u8,5],[CPM_CLK_SCC2 as u8,CPM_CLK3 as u8,6],[CPM_CLK_SCC2 as u8,CPM_CLK4 as u8,7],
        [CPM_CLK_SCC3 as u8,CPM_BRG1 as u8,0],[CPM_CLK_SCC3 as u8,CPM_BRG2 as u8,1],[CPM_CLK_SCC3 as u8,CPM_BRG3 as u8,2],[CPM_CLK_SCC3 as u8,CPM_BRG4 as u8,3],[CPM_CLK_SCC3 as u8,CPM_CLK5 as u8,4],[CPM_CLK_SCC3 as u8,CPM_CLK6 as u8,5],[CPM_CLK_SCC3 as u8,CPM_CLK7 as u8,6],[CPM_CLK_SCC3 as u8,CPM_CLK8 as u8,7],
        [CPM_CLK_SCC4 as u8,CPM_BRG1 as u8,0],[CPM_CLK_SCC4 as u8,CPM_BRG2 as u8,1],[CPM_CLK_SCC4 as u8,CPM_BRG3 as u8,2],[CPM_CLK_SCC4 as u8,CPM_BRG4 as u8,3],[CPM_CLK_SCC4 as u8,CPM_CLK5 as u8,4],[CPM_CLK_SCC4 as u8,CPM_CLK6 as u8,5],[CPM_CLK_SCC4 as u8,CPM_CLK7 as u8,6],[CPM_CLK_SCC4 as u8,CPM_CLK8 as u8,7],
    ];
    let mut bits: u32 = 0;
    let mut found = false;
    for row in map { if row[0] as i32 == target as i32 && row[1] as i32 == clock { bits = row[2] as u32; found = true; break; } }
    let mut ret = if found { 0 } else { -EINVAL };
    bits <<= shift;
    let mut mask: u32 = 7 << shift;
    if mode == CPM_CLK_RTX { bits |= bits << 3; mask |= mask << 3; }
    else if mode == CPM_CLK_RX { bits <<= 3; mask <<= 3; }
    out_be32(reg, (in_be32(reg) & !mask) | bits);
    ret
}

pub unsafe fn cpm2_smc_clk_setup(target: cpm_clk_target, clock: i32) -> i32 {
    let (reg, shift) = match target {
        CPM_CLK_SMC1 => (&mut (*cpm2_immr).im_cpmux.cmx_smr as *mut u8, 4),
        CPM_CLK_SMC2 => (&mut (*cpm2_immr).im_cpmux.cmx_smr as *mut u8, 0),
        _ => return -EINVAL,
    };
    let map = [[CPM_CLK_SMC1 as u8,CPM_BRG1 as u8,0],[CPM_CLK_SMC1 as u8,CPM_BRG7 as u8,1],[CPM_CLK_SMC1 as u8,CPM_CLK7 as u8,2],[CPM_CLK_SMC1 as u8,CPM_CLK9 as u8,3],[CPM_CLK_SMC2 as u8,CPM_BRG2 as u8,0],[CPM_CLK_SMC2 as u8,CPM_BRG8 as u8,1],[CPM_CLK_SMC2 as u8,CPM_CLK4 as u8,2],[CPM_CLK_SMC2 as u8,CPM_CLK15 as u8,3]];
    let mut bits: u8 = 0;
    let mut found = false;
    for row in map { if row[0] as i32 == target as i32 && row[1] as i32 == clock { bits = row[2]; found = true; break; } }
    let mask: u8 = 3 << shift;
    out_8(reg, (in_8(reg) & !mask) | (bits << shift));
    if found { 0 } else { -EINVAL }
}

#[repr(C)] pub struct cpm2_ioports { pub dir:u32, pub par:u32, pub sor:u32, pub odr:u32, pub dat:u32, pub res:[u32;3] }

pub unsafe fn cpm2_set_pin(port: i32, pin: i32, flags: i32) {
    let iop = &mut (*cpm2_immr).im_ioport as *mut _ as *mut cpm2_ioports;
    let pin = 1u32 << (31 - pin);
    let p = iop.add(port as usize);
    if flags & CPM_PIN_OUTPUT != 0 { setbits32(&mut (*p).dir, pin); } else { clrbits32(&mut (*p).dir, pin); }
    if flags & CPM_PIN_GPIO == 0 { setbits32(&mut (*p).par, pin); } else { clrbits32(&mut (*p).par, pin); }
    if flags & CPM_PIN_SECONDARY != 0 { setbits32(&mut (*p).sor, pin); } else { clrbits32(&mut (*p).sor, pin); }
    if flags & CPM_PIN_OPENDRAIN != 0 { setbits32(&mut (*p).odr, pin); } else { clrbits32(&mut (*p).odr, pin); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
