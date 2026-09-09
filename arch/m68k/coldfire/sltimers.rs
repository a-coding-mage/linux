// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	sltimers.rs -- generic ColdFire slice timer support.
 *
 *	Copyright (C) 2009-2010, Philippe De Muyter <phdm@macqel.be>
 *	based on
 *	timers.c -- generic ColdFire hardware timer support.
 *	Copyright (C) 1999-2008, Greg Ungerer <gerg@snapgear.com>
 */

/***************************************************************************/

// C dependencies supplied by the surrounding kernel translation unit.

#[cfg(feature = "CONFIG_HIGHPROFILE")]
const PROFILEHZ: i32 = 1013;

#[cfg(feature = "CONFIG_HIGHPROFILE")]
#[inline]
unsafe fn pa(a: u32) -> u32 {
	MCFSLT_TIMER1.wrapping_add(a)
}

#[cfg(feature = "CONFIG_HIGHPROFILE")]
pub unsafe extern "C" fn mcfslt_profile_tick(irq: i32, dummy: *mut core::ffi::c_void) -> irqreturn_t {
	let _ = (irq, dummy);
	/* Reset Slice Timer 1 */
	mcf_write32(MCFSLT_SSR_BE | MCFSLT_SSR_TE, pa(MCFSLT_SSR));
	if (*current).pid != 0 {
		profile_tick(CPU_PROFILING);
	}
	IRQ_HANDLED
}

#[cfg(feature = "CONFIG_HIGHPROFILE")]
pub unsafe fn mcfslt_profile_init() {
	let mut ret: i32;

	printk(KERN_INFO, "PROFILE: lodging TIMER 1 @ %dHz as profile timer\n", PROFILEHZ);

	ret = request_irq(MCF_IRQ_PROFILER, Some(mcfslt_profile_tick), IRQF_TIMER,
		"profile timer", core::ptr::null_mut());
	if ret != 0 {
		pr_err("Failed to request irq %d (profile timer): %pe\n",
			MCF_IRQ_PROFILER, ERR_PTR(ret));
	}

	/* Set up TIMER 2 as high speed profile clock */
	mcf_write32(MCF_BUSCLK / PROFILEHZ as u32 - 1, pa(MCFSLT_STCNT));
	mcf_write32(MCFSLT_SCR_RUN | MCFSLT_SCR_IEN | MCFSLT_SCR_TEN,
		pa(MCFSLT_SCR));
}

/***************************************************************************/

/*
 *	By default use Slice Timer 0 as the system clock timer.
 */
#[inline]
unsafe fn ta(a: u32) -> u32 {
	MCFSLT_TIMER0.wrapping_add(a)
}

static mut mcfslt_cycles_per_jiffy: u32 = 0;
static mut mcfslt_cnt: u32 = 0;

unsafe extern "C" fn mcfslt_tick(irq: i32, dummy: *mut core::ffi::c_void) -> irqreturn_t {
	let _ = (irq, dummy);
	/* Reset Slice Timer 0 */
	mcf_write32(MCFSLT_SSR_BE | MCFSLT_SSR_TE, ta(MCFSLT_SSR));
	mcfslt_cnt = mcfslt_cnt.wrapping_add(mcfslt_cycles_per_jiffy);
	legacy_timer_tick(1);
	IRQ_HANDLED
}

unsafe extern "C" fn mcfslt_read_clk(cs: *mut clocksource) -> u64 {
	let _ = cs;
	let mut flags: ulong = 0;
	let mut cycles: u32;
	let mut scnt: u32;

	local_irq_save(&mut flags);
	scnt = mcf_read32(ta(MCFSLT_SCNT));
	cycles = mcfslt_cnt;
	if (mcf_read32(ta(MCFSLT_SSR)) & MCFSLT_SSR_TE) != 0 {
		cycles = cycles.wrapping_add(mcfslt_cycles_per_jiffy);
		scnt = mcf_read32(ta(MCFSLT_SCNT));
	}
	local_irq_restore(flags);

	/* subtract because slice timers count down */
	(cycles as u64).wrapping_add(
		(mcfslt_cycles_per_jiffy.wrapping_sub(1).wrapping_sub(scnt)) as u64,
	)
}

static mut mcfslt_clk: clocksource = clocksource {
	name: b"slt\0".as_ptr() as *const i8,
	rating: 250,
	read: Some(mcfslt_read_clk),
	mask: CLOCKSOURCE_MASK(32),
	flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

pub unsafe extern "C" fn hw_timer_init() {
	let mut r: i32;

	mcfslt_cycles_per_jiffy = MCF_BUSCLK / HZ;
	/*
	 *	The coldfire slice timer (SLT) runs from STCNT to 0 included,
	 *	then STCNT again and so on.  It counts thus actually
	 *	STCNT + 1 steps for 1 tick, not STCNT.  So if you want
	 *	n cycles, initialize STCNT with n - 1.
	 */
	mcf_write32(mcfslt_cycles_per_jiffy.wrapping_sub(1), ta(MCFSLT_STCNT));
	mcf_write32(MCFSLT_SCR_RUN | MCFSLT_SCR_IEN | MCFSLT_SCR_TEN,
		ta(MCFSLT_SCR));
	/* initialize mcfslt_cnt knowing that slice timers count down */
	mcfslt_cnt = mcfslt_cycles_per_jiffy;

	r = request_irq(MCF_IRQ_TIMER, Some(mcfslt_tick), IRQF_TIMER, "timer",
		core::ptr::null_mut());
	if r != 0 {
		pr_err("Failed to request irq %d (timer): %pe\n", MCF_IRQ_TIMER,
			ERR_PTR(r));
	}

	clocksource_register_hz(&raw mut mcfslt_clk, MCF_BUSCLK);

	#[cfg(feature = "CONFIG_HIGHPROFILE")]
	mcfslt_profile_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
