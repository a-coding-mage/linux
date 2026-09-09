// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2007 David Gibson, IBM Corporation.
 *
 * Based on earlier code:
 *   Matt Porter <mporter@kernel.crashing.org>
 *   Copyright 2002-2005 MontaVista Software Inc.
 *
 *   Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
 *   Copyright (c) 2003, 2004 Zultys Technologies
 *
 * Copyright (C) 2009 Wind River Systems, Inc.
 *   Updated for supporting PPC405EX on Kilauea.
 *   Tiejun Chen <tiejun.chen@windriver.com>
 */

// External declarations and macros are supplied by the translated dependency files.

static unsafe fn chip_11_errata(mut memsize: usize) -> usize {
    let pvr = mfpvr();
    match pvr & 0xf0000ff0 {
        0x40000850 | 0x400008d0 | 0x200008d0 => memsize -= 4096,
        _ => {}
    }
    memsize
}

/* Read the 4xx SDRAM controller to get size of system memory. */
pub unsafe fn ibm4xx_sdram_fixup_memsize() {
    let mut memsize: usize = 0;
    for i in 0..sdram_bxcr.len() {
        let bank_config = SDRAM0_READ(sdram_bxcr[i]);
        if bank_config & SDRAM_CONFIG_BANK_ENABLE != 0 {
            memsize += SDRAM_CONFIG_BANK_SIZE(bank_config) as usize;
        }
    }
    memsize = chip_11_errata(memsize);
    dt_fixup_memory(0, memsize as u64);
}

/* Read the 440SPe MQ controller to get size of system memory. */
const DCRN_MQ0_B0BAS: u32 = 0x40;
const DCRN_MQ0_B1BAS: u32 = 0x41;
const DCRN_MQ0_B2BAS: u32 = 0x42;
const DCRN_MQ0_B3BAS: u32 = 0x43;

unsafe fn ibm440spe_decode_bas(bas: u32) -> u64 {
    let base = ((bas & 0xffe00000) as u64) << 2;
    match (bas >> 4) & 0xfff {
        0 => 0,
        0xffc => base + 0x0008_0000_00,
        0xff8 => base + 0x0010_0000_00,
        0xff0 => base + 0x0020_0000_00,
        0xfe0 => base + 0x0040_0000_00,
        0xfc0 => base + 0x0080_0000_00,
        0xf80 => base + 0x0100_0000_00,
        0xf00 => base + 0x0200_0000_00,
        0xe00 => base + 0x0400_0000_00,
        0xc00 => base + 0x0800_0000_00,
        0x800 => base + 0x1000_0000_00,
        _ => { printf!("Memory BAS value 0x%08x unsupported !\n", bas); 0 }
    }
}

pub unsafe fn ibm440spe_fixup_memsize() {
    let mut memsize = 0u64;
    for reg in [DCRN_MQ0_B0BAS, DCRN_MQ0_B1BAS, DCRN_MQ0_B2BAS, DCRN_MQ0_B3BAS] {
        let banktop = ibm440spe_decode_bas(mfdcr(reg));
        if banktop > memsize { memsize = banktop; }
    }
    dt_fixup_memory(0, memsize);
}

/* 4xx DDR1/2 Denali memory controller support */
const DDR0_02: u32 = 2; const DDR0_08: u32 = 8; const DDR0_10: u32 = 10;
const DDR0_14: u32 = 14; const DDR0_42: u32 = 42; const DDR0_43: u32 = 43;
const DDR_START: u32 = 1; const DDR_START_SHIFT: u32 = 0;
const DDR_MAX_CS_REG: u32 = 3; const DDR_MAX_CS_REG_SHIFT: u32 = 24;
const DDR_MAX_COL_REG: u32 = 0xf; const DDR_MAX_COL_REG_SHIFT: u32 = 16;
const DDR_MAX_ROW_REG: u32 = 0xf; const DDR_MAX_ROW_REG_SHIFT: u32 = 8;
const DDR_DDR2_MODE: u32 = 1; const DDR_DDR2_MODE_SHIFT: u32 = 0;
const DDR_CS_MAP: u32 = 3; const DDR_CS_MAP_SHIFT: u32 = 8;
const DDR_REDUC: u32 = 1; const DDR_REDUC_SHIFT: u32 = 16;
const DDR_APIN: u32 = 7; const DDR_APIN_SHIFT: u32 = 24;
const DDR_COL_SZ: u32 = 7; const DDR_COL_SZ_SHIFT: u32 = 8;
const DDR_BANK8: u32 = 1; const DDR_BANK8_SHIFT: u32 = 0;
#[inline] fn ddr_get_val(val: u32, mask: u32, shift: u32) -> u32 { (val >> shift) & mask }

unsafe fn ibm4xx_denali_get_cs() -> u32 {
    let devp = finddevice("/");
    if !devp.is_null() {
        let mut model = [0i8; 64];
        if getprop(devp, "model", model.as_mut_ptr(), model.len()) > 0 {
            model[63] = 0;
            if strcmp(model.as_ptr(), "amcc,sequoia") == 0 || strcmp(model.as_ptr(), "amcc,rainier") == 0 { return 1; }
        }
    }
    let mut val = ddr_get_val(SDRAM0_READ(DDR0_10), DDR_CS_MAP, DDR_CS_MAP_SHIFT);
    let mut cs = 0;
    while val != 0 { if val & 1 != 0 { cs += 1; } val >>= 1; }
    cs
}

pub unsafe fn ibm4xx_denali_fixup_memsize() {
    let mut val = SDRAM0_READ(DDR0_02);
    if ddr_get_val(val, DDR_START, DDR_START_SHIFT) == 0 { fatal("DDR controller is not initialized\n"); }
    let max_cs = ddr_get_val(val, DDR_MAX_CS_REG, DDR_MAX_CS_REG_SHIFT);
    let max_col = ddr_get_val(val, DDR_MAX_COL_REG, DDR_MAX_COL_REG_SHIFT);
    let max_row = ddr_get_val(val, DDR_MAX_ROW_REG, DDR_MAX_ROW_REG_SHIFT);
    let cs = ibm4xx_denali_get_cs();
    if cs == 0 { fatal("No memory installed\n"); }
    if cs > max_cs { fatal("DDR wrong CS configuration\n"); }
    val = SDRAM0_READ(DDR0_14);
    let dpath = if ddr_get_val(val, DDR_REDUC, DDR_REDUC_SHIFT) != 0 { 4 } else { 8 };
    val = SDRAM0_READ(DDR0_42);
    let row = ddr_get_val(val, DDR_APIN, DDR_APIN_SHIFT);
    if row > max_row { fatal("DDR wrong APIN configuration\n"); }
    let row = max_row - row;
    val = SDRAM0_READ(DDR0_43);
    let col = ddr_get_val(val, DDR_COL_SZ, DDR_COL_SZ_SHIFT);
    if col > max_col { fatal("DDR wrong COL configuration\n"); }
    let col = max_col - col;
    let bank = if ddr_get_val(val, DDR_BANK8, DDR_BANK8_SHIFT) != 0 { 8 } else { 4 };
    let mut memsize = (cs as usize) * (1usize << (col + row)) * bank * dpath;
    memsize = chip_11_errata(memsize);
    dt_fixup_memory(0, memsize as u64);
}

const SPRN_DBCR0_44X: u32 = 0x134; const DBCR0_RST_SYSTEM: u32 = 0x30000000;
pub unsafe fn ibm44x_dbcr_reset() {
    let mut tmp: u32;
    core::arch::asm!("mfspr {0},{1}; oris {0},{0},{2}@h; mtspr {1},{0}", out(reg) tmp, const SPRN_DBCR0_44X, const DBCR0_RST_SYSTEM);
}

const EMAC_RESET: u32 = 0x20000000;
pub unsafe fn ibm4xx_quiesce_eth(emac0: *mut u32, emac1: *mut u32) {
    if !emac0.is_null() { *emac0 = EMAC_RESET; } if !emac1.is_null() { *emac1 = EMAC_RESET; }
    mtdcr(DCRN_MAL0_CFG, MAL_RESET); while mfdcr(DCRN_MAL0_CFG) & MAL_RESET != 0 {}
}

pub unsafe fn ibm4xx_fixup_ebc_ranges(ebc: *const i8) {
    let mut ranges = [0u32; EBC_NUM_BANKS * 4]; let mut p = 0usize;
    for i in 0..EBC_NUM_BANKS { mtdcr(DCRN_EBC0_CFGADDR, EBC_BXCR(i)); let bxcr = mfdcr(DCRN_EBC0_CFGDATA); if bxcr & EBC_BXCR_BU != EBC_BXCR_BU_OFF { ranges[p]=i as u32; ranges[p+1]=0; ranges[p+2]=bxcr & EBC_BXCR_BAS; ranges[p+3]=EBC_BXCR_BANK_SIZE(bxcr); p += 4; } }
    let devp = finddevice(ebc); if devp.is_null() { fatal("Couldn't locate EBC node %s\n\r", ebc); }
    setprop(devp, "ranges", ranges.as_ptr(), p * core::mem::size_of::<u32>());
}

pub unsafe fn ibm440gp_fixup_clocks(sys_clk: u32, ser_clk: u32) {
    let sys0 = mfdcr(DCRN_CPC0_SYS0); let cr0 = mfdcr(DCRN_CPC0_CR0);
    let opdv = CPC0_SYS0_OPDV(sys0); let epdv = CPC0_SYS0_EPDV(sys0); let (cpu, plb);
    if sys0 & CPC0_SYS0_BYPASS != 0 { cpu=sys_clk; plb=sys_clk; } else {
        let m = if sys0 & CPC0_SYS0_EXTSL != 0 { CPC0_SYS0_FWDVB(sys0)*opdv*epdv } else { CPC0_SYS0_FBDV(sys0)*CPC0_SYS0_FWDVA(sys0) };
        cpu=sys_clk*m/CPC0_SYS0_FWDVA(sys0); plb=sys_clk*m/CPC0_SYS0_FWDVB(sys0);
    }
    let opb=plb/opdv; let ebc=opb/epdv;
    let tb=if mfpvr()&0xf0000fff==0x40000440 {sys_clk} else {cpu};
    let uart0=if cr0&CPC0_CR0_U0EC!=0 {ser_clk} else {plb/CPC0_CR0_UDIV(cr0)};
    let uart1=if cr0&CPC0_CR0_U1EC!=0 {ser_clk} else {plb/CPC0_CR0_UDIV(cr0)};
    printf!("PPC440GP: SysClk = %dMHz (%x)\n\r",(sys_clk+500000)/1000000,sys_clk);
    dt_fixup_cpu_clocks(cpu,tb,0); dt_fixup_clock("/plb",plb); dt_fixup_clock("/plb/opb",opb); dt_fixup_clock("/plb/opb/ebc",ebc); dt_fixup_clock("/plb/opb/serial@40000200",uart0); dt_fixup_clock("/plb/opb/serial@40000300",uart1);
}

const SPRN_CCR1:u32=0x378;
#[inline] unsafe fn __fix_zero(v:u32,def:u32)->u32 { if v!=0 {v} else {def} }
unsafe fn __ibm440eplike_fixup_clocks(sys_clk:u32,tmr_clk:u32,per_clk_from_opb:i32)->u32 {
    let pllc=CPR0_READ(DCRN_CPR0_PLLC); let plld=CPR0_READ(DCRN_CPR0_PLLD);
    let fbdv=__fix_zero((plld>>24)&0x1f,32); let fwdva=__fix_zero((plld>>16)&0xf,16); let fwdvb=__fix_zero((plld>>8)&7,8); let lfbdv=__fix_zero(plld&0x3f,64); let pradv0=__fix_zero((CPR0_READ(DCRN_CPR0_PRIMAD)>>24)&7,8); let prbdv0=__fix_zero((CPR0_READ(DCRN_CPR0_PRIMBD)>>24)&7,8); let opbdv0=__fix_zero((CPR0_READ(DCRN_CPR0_OPBD)>>24)&3,4); let perdv0=__fix_zero((CPR0_READ(DCRN_CPR0_PERD)>>24)&3,4);
    let (clk_a,clk_b)=if pllc&0x40000000!=0 { let m=match (pllc>>24)&7 {0=>if pllc&0x20000000!=0{fwdvb}else{fwdva}*lfbdv,1=>fwdva*pradv0,5=>fwdvb*prbdv0*opbdv0*perdv0,_=>{printf!("WARNING ! Invalid PLL feedback source !\n");0}}*fbdv; let vco=sys_clk*m; (vco/fwdva,vco/fwdvb) } else {(sys_clk,sys_clk)};
    let cpu=clk_a/pradv0; let plb=clk_b/prbdv0; let opb=plb/opbdv0; let ebc=(if per_clk_from_opb!=0{opb}else{plb})/perdv0; let mut tb=tmr_clk; let mut ccr1=mfspr(SPRN_CCR1); if tb==0 {ccr1&=!0x80u32;mtspr(SPRN_CCR1,ccr1);} if ccr1&0x80==0{tb=cpu;} dt_fixup_cpu_clocks(cpu,tb,0); dt_fixup_clock("/plb",plb); dt_fixup_clock("/plb/opb",opb); dt_fixup_clock("/plb/opb/ebc",ebc); plb
}
unsafe fn eplike_fixup_uart_clk(index:i32,path:*const i8,ser_clk:u32,plb_clk:u32){let sdr=match index{0=>SDR0_READ(DCRN_SDR0_UART0),1=>SDR0_READ(DCRN_SDR0_UART1),2=>SDR0_READ(DCRN_SDR0_UART2),3=>SDR0_READ(DCRN_SDR0_UART3),_=>return};let clock=if sdr&0x00800000!=0{ser_clk}else{plb_clk/__fix_zero(sdr&0xff,256)};dt_fixup_clock(path,clock);}
pub unsafe fn ibm440ep_fixup_clocks(s:u32,r:u32,t:u32){let p=__ibm440eplike_fixup_clocks(s,t,0);eplike_fixup_uart_clk(0,"/plb/opb/serial@ef600300",r,p);eplike_fixup_uart_clk(1,"/plb/opb/serial@ef600400",r,p);eplike_fixup_uart_clk(2,"/plb/opb/serial@ef600500",r,p);eplike_fixup_uart_clk(3,"/plb/opb/serial@ef600600",r,p);}
pub unsafe fn ibm440gx_fixup_clocks(s:u32,r:u32,t:u32){let p=__ibm440eplike_fixup_clocks(s,t,1);eplike_fixup_uart_clk(0,"/plb/opb/serial@40000200",r,p);eplike_fixup_uart_clk(1,"/plb/opb/serial@40000300",r,p);}
pub unsafe fn ibm440spe_fixup_clocks(s:u32,r:u32,t:u32){let p=__ibm440eplike_fixup_clocks(s,t,1);eplike_fixup_uart_clk(0,"/plb/opb/serial@f0000200",r,p);eplike_fixup_uart_clk(1,"/plb/opb/serial@f0000300",r,p);eplike_fixup_uart_clk(2,"/plb/opb/serial@f0000600",r,p);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
