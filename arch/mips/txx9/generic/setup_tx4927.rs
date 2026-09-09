/*
 * TX4927 setup routines
 * Based on linux/arch/mips/txx9/rbtx4938/setup.c,
 *          and RBTX49xx patch from CELF patch archive.
 *
 * 2003-2005 (c) MontaVista Software, Inc.
 * (C) Copyright TOSHIBA CORPORATION 2000-2001, 2004-2007
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive.
 */

// Declarations supplied by the kernel's MIPS/TXX9 dependencies are referenced
// here in their translated form.

unsafe fn tx4927_wdr_init() {
    /* report watchdog reset status */
    if (____raw_readq(&(*tx4927_ccfgptr).ccfg) & TX4927_CCFG_WDRST) != 0 {
        pr_warn!("Watchdog reset detected at 0x%lx\n", read_c0_errorepc());
    }
    /* clear WatchDogReset (W1C) */
    tx4927_ccfg_set(TX4927_CCFG_WDRST);
    /* do reset on watchdog */
    tx4927_ccfg_set(TX4927_CCFG_WR);
}

pub unsafe fn tx4927_wdt_init() {
    txx9_wdt_init(TX4927_TMR_REG(2) & 0xfffffffff_u64);
}

unsafe fn tx4927_machine_restart(_command: *mut core::ffi::c_char) {
    local_irq_disable();
    pr_emerg!("Rebooting (with {} watchdog reset)...\n",
        if (____raw_readq(&(*tx4927_ccfgptr).ccfg) & TX4927_CCFG_WDREXEN) != 0 {
            "external"
        } else {
            "internal"
        });
    /* clear watchdog status */
    tx4927_ccfg_set(TX4927_CCFG_WDRST); /* W1C */
    txx9_wdt_now(TX4927_TMR_REG(2) & 0xfffffffff_u64);
    while (____raw_readq(&(*tx4927_ccfgptr).ccfg) & TX4927_CCFG_WDRST == 0) {}
    mdelay(10);
    if (____raw_readq(&(*tx4927_ccfgptr).ccfg) & TX4927_CCFG_WDREXEN) != 0 {
        pr_emerg!("Rebooting (with internal watchdog reset)...\n");
        /* External WDRST failed.  Do internal watchdog reset */
        tx4927_ccfg_clear(TX4927_CCFG_WDREXEN);
    }
    /* fallback */
    (_machine_halt)();
}

unsafe extern "C" {
    fn show_registers(regs: *mut pt_regs);
}

unsafe fn tx4927_be_handler(regs: *mut pt_regs, _is_fixup: i32) -> i32 {
    let data = (*regs).cp0_cause & 4;
    console_verbose();
    pr_err!("{}BE exception at {:#lx}\n", if data != 0 { 'D' } else { 'I' }, (*regs).cp0_epc);
    pr_err!("ccfg:{:x}, toea:{:x}\n",
        ____raw_readq(&(*tx4927_ccfgptr).ccfg),
        ____raw_readq(&(*tx4927_ccfgptr).toea));
    show_registers(regs);
    panic!("BusError!");
}

unsafe fn tx4927_be_init() {
    mips_set_be_handler(tx4927_be_handler);
}

static mut tx4927_sdram_resource: [resource; 4] = [resource::default(); 4];

pub unsafe fn tx4927_setup() {
    let mut i: i32;
    let mut divmode: u32;
    let mut cpuclk: u32 = 0;
    let ccfg: u64;

    txx9_reg_res_init(TX4927_REV_PCODE(), TX4927_REG_BASE, TX4927_REG_SIZE);
    set_c0_config(TX49_CONF_CWFON);

    /* SDRAMC,EBUSC are configured by PROM */
    i = 0;
    while i < 8 {
        if TX4927_EBUSC_CR(i) & 0x8 == 0 { i += 1; continue; } /* disabled */
        txx9_ce_res[i as usize].start = TX4927_EBUSC_BA(i) as usize;
        txx9_ce_res[i as usize].end = txx9_ce_res[i as usize].start + TX4927_EBUSC_SIZE(i) as usize - 1;
        request_resource(&mut iomem_resource, &mut txx9_ce_res[i as usize]);
        i += 1;
    }

    /* clocks */
    ccfg = ____raw_readq(&(*tx4927_ccfgptr).ccfg);
    if txx9_master_clock != 0 {
        divmode = ccfg as u32 & TX4927_CCFG_DIVMODE_MASK;
        match divmode {
            TX4927_CCFG_DIVMODE_8 | TX4927_CCFG_DIVMODE_10 | TX4927_CCFG_DIVMODE_12 | TX4927_CCFG_DIVMODE_16 => txx9_gbus_clock = txx9_master_clock * 4,
            _ => txx9_gbus_clock = txx9_master_clock,
        }
        cpuclk = match divmode {
            TX4927_CCFG_DIVMODE_2 | TX4927_CCFG_DIVMODE_8 => txx9_gbus_clock * 2,
            TX4927_CCFG_DIVMODE_2_5 | TX4927_CCFG_DIVMODE_10 => txx9_gbus_clock * 5 / 2,
            TX4927_CCFG_DIVMODE_3 | TX4927_CCFG_DIVMODE_12 => txx9_gbus_clock * 3,
            TX4927_CCFG_DIVMODE_4 | TX4927_CCFG_DIVMODE_16 => txx9_gbus_clock * 4,
            _ => cpuclk,
        };
        txx9_cpu_clock = cpuclk;
    } else {
        if txx9_cpu_clock == 0 { txx9_cpu_clock = 200000000; } /* 200MHz */
        cpuclk = txx9_cpu_clock;
        divmode = ccfg as u32 & TX4927_CCFG_DIVMODE_MASK;
        txx9_gbus_clock = match divmode {
            TX4927_CCFG_DIVMODE_2 | TX4927_CCFG_DIVMODE_8 => cpuclk / 2,
            TX4927_CCFG_DIVMODE_2_5 | TX4927_CCFG_DIVMODE_10 => cpuclk * 2 / 5,
            TX4927_CCFG_DIVMODE_3 | TX4927_CCFG_DIVMODE_12 => cpuclk / 3,
            TX4927_CCFG_DIVMODE_4 | TX4927_CCFG_DIVMODE_16 => cpuclk / 4,
            _ => txx9_gbus_clock,
        };
        txx9_master_clock = match divmode {
            TX4927_CCFG_DIVMODE_8 | TX4927_CCFG_DIVMODE_10 | TX4927_CCFG_DIVMODE_12 | TX4927_CCFG_DIVMODE_16 => txx9_gbus_clock / 4,
            _ => txx9_gbus_clock,
        };
    }
    loops_per_jiffy = txx9_cpu_clock / HZ / 2;

    /* CCFG */
    tx4927_wdr_init();
    /* clear BusErrorOnWrite flag (W1C) */
    tx4927_ccfg_set(TX4927_CCFG_BEOW);
    /* enable Timeout BusError */
    if txx9_ccfg_toeon { tx4927_ccfg_set(TX4927_CCFG_TOE); }

    /* DMA selection */
    txx9_clear64(&mut (*tx4927_ccfgptr).pcfg, TX4927_PCFG_DMASEL_ALL);
    /* Use external clock for external arbiter */
    if ____raw_readq(&(*tx4927_ccfgptr).ccfg) & TX4927_CCFG_PCIARB == 0 {
        txx9_clear64(&mut (*tx4927_ccfgptr).pcfg, TX4927_PCFG_PCICLKEN_ALL);
    }
    pr_info!("{} -- {}MHz(M{}MHz) CRIR:{:08x} CCFG:{:x} PCFG:{:x}\n", txx9_pcode_str, (cpuclk + 500000) / 1000000, (txx9_master_clock + 500000) / 1000000, ____raw_readq(&(*tx4927_ccfgptr).crir) as u32, ____raw_readq(&(*tx4927_ccfgptr).ccfg), ____raw_readq(&(*tx4927_ccfgptr).pcfg));

    pr_info!("{} SDRAMC --", txx9_pcode_str);
    i = 0;
    while i < 4 {
        let cr = TX4927_SDRAMC_CR(i);
        if cr as u32 & 0x00000400 == 0 { i += 1; continue; } /* disabled */
        let base = ((cr >> 49) as usize) << 21;
        let size = ((((cr >> 33) as usize) & 0x7fff) + 1) << 21;
        pr_cont!(" CR{}:{:016x}", i, cr);
        tx4927_sdram_resource[i as usize].name = "SDRAM";
        tx4927_sdram_resource[i as usize].start = base;
        tx4927_sdram_resource[i as usize].end = base + size - 1;
        tx4927_sdram_resource[i as usize].flags = IORESOURCE_MEM;
        request_resource(&mut iomem_resource, &mut tx4927_sdram_resource[i as usize]);
        i += 1;
    }
    pr_cont!(" TR:{:09x}\n", ____raw_readq(&(*tx4927_sdramcptr).tr));

    /* TMR */
    /* disable all timers */
    i = 0;
    while i < TX4927_NR_TMR { txx9_tmr_init(TX4927_TMR_REG(i) & 0xfffffffff_u64); i += 1; }
    /* PIO */
    __raw_writel(0, &mut (*tx4927_pioptr).maskcpu);
    __raw_writel(0, &mut (*tx4927_pioptr).maskext);
    _machine_restart = Some(tx4927_machine_restart);
    board_be_init = Some(tx4927_be_init);
}

pub unsafe fn tx4927_time_init(tmrnr: u32) {
    if ____raw_readq(&(*tx4927_ccfgptr).ccfg) & TX4927_CCFG_TINTDIS != 0 {
        txx9_clockevent_init(TX4927_TMR_REG(tmrnr) & 0xfffffffff_u64, TXX9_IRQ_BASE + TX4927_IR_TMR(tmrnr), TXX9_IMCLK);
    }
}

pub unsafe fn tx4927_sio_init(sclk: u32, cts_mask: u32) {
    let mut i = 0;
    while i < 2 {
        txx9_sio_init(TX4927_SIO_REG(i) & 0xfffffffff_u64, TXX9_IRQ_BASE + TX4927_IR_SIO(i), i, sclk, (1 << i) & cts_mask);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
