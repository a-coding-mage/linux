// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1995  Linus Torvalds
 *  Adapted from 'alpha' version by Gary Thomas
 *  Modified by Cort Dougan (cort@cs.nmt.edu)
 *  Modified for MBX using prep/chrp/pmac functions by Dan (dmalek@jlc.net)
 *  Further modified for generic 8xx by Dan.
 */

/* bootup setup stuff.. */

// C headers and local headers provide the external kernel, architecture, and
// MPC8xx symbols referenced below.

/* A place holder for time base interrupts, if they are ever enabled. */
unsafe fn timebase_interrupt(_irq: i32, _dev: *mut core::ffi::c_void) -> irqreturn_t {
    printk!("timebase_interrupt()\n");

    IRQ_HANDLED
}

unsafe fn get_freq(name: *mut i8, val: *mut libc::c_ulong) -> i32 {
    let mut cpu: *mut device_node;
    let fp: *const u32;
    let mut found: i32 = 0;

    /* The cpu node should have timebase and clock frequency properties */
    cpu = of_get_cpu_node(0, core::ptr::null_mut());

    if !cpu.is_null() {
        fp = of_get_property(cpu, name, core::ptr::null_mut());
        if !fp.is_null() {
            found = 1;
            *val = *fp as libc::c_ulong;
        }

        of_node_put(cpu);
    }

    found
}

/* The decrementer counts at the system (internal) clock frequency divided by
 * sixteen, or external oscillator divided by four.  We force the processor
 * to use system clock divided by sixteen.
 */
pub unsafe fn mpc8xx_calibrate_decr() {
    let mut cpu: *mut device_node;
    let irq: i32;
    let virq: i32;

    /* Unlock the SCCR. */
    out_be32(&mut (*mpc8xx_immr).im_clkrstk.cark_sccrk, !KAPWR_KEY);
    out_be32(&mut (*mpc8xx_immr).im_clkrstk.cark_sccrk, KAPWR_KEY);

    /* Force all 8xx processors to use divide by 16 processor clock. */
    setbits32(&mut (*mpc8xx_immr).im_clkrst.car_sccr, 0x02000000);

    /* Processor frequency is MHz.
     */
    ppc_proc_freq = 50000000;
    if get_freq(b"clock-frequency\0".as_ptr() as *mut i8, &mut ppc_proc_freq) == 0 {
        printk!(KERN_ERR "WARNING: Estimating processor frequency "
                "(not found)\n");
    }

    ppc_tb_freq = ppc_proc_freq / 16;
    printk!("Decrementer Frequency = 0x%lx\n", ppc_tb_freq);

    /* Perform some more timer/timebase initialization.  This used
     * to be done elsewhere, but other changes caused it to get
     * called more than once....that is a bad thing.
     *
     * First, unlock all of the registers we are going to modify.
     * To protect them from corruption during power down, registers
     * that are maintained by keep alive power are "locked".  To
     * modify these registers we have to write the key value to
     * the key location associated with the register.
     * Some boards power up with these unlocked, while others
     * are locked.  Writing anything (including the unlock code?)
     * to the unlocked registers will lock them again.  So, here
     * we guarantee the registers are locked, then we unlock them
     * for our use.
     */
    out_be32(&mut (*mpc8xx_immr).im_sitk.sitk_tbscrk, !KAPWR_KEY);
    out_be32(&mut (*mpc8xx_immr).im_sitk.sitk_rtcsck, !KAPWR_KEY);
    out_be32(&mut (*mpc8xx_immr).im_sitk.sitk_tbk, !KAPWR_KEY);
    out_be32(&mut (*mpc8xx_immr).im_sitk.sitk_tbscrk, KAPWR_KEY);
    out_be32(&mut (*mpc8xx_immr).im_sitk.sitk_rtcsck, KAPWR_KEY);
    out_be32(&mut (*mpc8xx_immr).im_sitk.sitk_tbk, KAPWR_KEY);

    /* Disable the RTC one second and alarm interrupts. */
    clrbits16(&mut (*mpc8xx_immr).im_sit.sit_rtcsc, RTCSC_SIE | RTCSC_ALE);

    /* Enable the RTC */
    setbits16(&mut (*mpc8xx_immr).im_sit.sit_rtcsc, RTCSC_RTF | RTCSC_RTE);

    /* Enabling the decrementer also enables the timebase interrupts
     * (or from the other point of view, to get decrementer interrupts
     * we have to enable the timebase).  The decrementer interrupt
     * is wired into the vector table, nothing to do here for that.
     */
    cpu = of_get_cpu_node(0, core::ptr::null_mut());
    virq = irq_of_parse_and_map(cpu, 0);
    of_node_put(cpu);
    irq = virq_to_hw(virq);

    out_be16(&mut (*mpc8xx_immr).im_sit.sit_tbscr,
             (((1u16 << (7 - (irq / 2))) << 8) as u16) | (TBSCR_TBF | TBSCR_TBE));

    if request_irq(virq, timebase_interrupt, IRQF_NO_THREAD, b"tbint\0".as_ptr(),
                   core::ptr::null_mut()) != 0 {
        panic!("Could not allocate timer IRQ!");
    }
}

/* The RTC on the MPC8xx is an internal register.
 * We want to protect this during power down, so we need to unlock,
 * modify, and re-lock.
 */

pub unsafe fn mpc8xx_set_rtc_time(tm: *mut rtc_time) -> i32 {
    let time: time64_t;

    time = rtc_tm_to_time64(tm);

    out_be32(&mut (*mpc8xx_immr).im_sitk.sitk_rtck, KAPWR_KEY);
    out_be32(&mut (*mpc8xx_immr).im_sit.sit_rtc, time as u32);
    out_be32(&mut (*mpc8xx_immr).im_sitk.sitk_rtck, !KAPWR_KEY);

    0
}

pub unsafe fn mpc8xx_get_rtc_time(tm: *mut rtc_time) {
    let data: libc::c_ulong;

    /* Get time from the RTC. */
    data = in_be32(&(*mpc8xx_immr).im_sit.sit_rtc) as libc::c_ulong;
    rtc_time64_to_tm(data as time64_t, tm);
    return;
}

pub unsafe fn mpc8xx_restart(_cmd: *mut i8) -> ! {
    local_irq_disable();

    setbits32(&mut (*mpc8xx_immr).im_clkrst.car_plprcr, 0x00000080);
    /* Clear the ME bit in MSR to cause checkstop on machine check
    */
    mtmsr(mfmsr() & !0x1000);

    in_8(&(*mpc8xx_immr).im_clkrst.res[0]);
    panic!("Restart failed\n");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
