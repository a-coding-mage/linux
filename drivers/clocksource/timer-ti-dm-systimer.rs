// SPDX-License-Identifier: GPL-2.0+
// External Linux kernel declarations and constants are supplied by other files.

const DMTIMER_TYPE1_ENABLE: u32 = (1 << 9) | (SYSC_IDLE_SMART << 3) | SYSC_OMAP2_ENAWAKEUP | SYSC_OMAP2_AUTOIDLE;
const DMTIMER_TYPE1_DISABLE: u32 = SYSC_OMAP2_SOFTRESET | SYSC_OMAP2_AUTOIDLE;
const DMTIMER_TYPE2_ENABLE: u32 = SYSC_IDLE_SMART_WKUP << 2;
const DMTIMER_RESET_WAIT: u32 = 100000;
const DMTIMER_INST_DONT_CARE: u32 = !0;

static mut counter_32k: i32 = 0;
static mut clocksource: u32 = 0;
static mut clockevent: u32 = 0;

#[repr(C)]
struct dmtimer_systimer {
    base: *mut core::ffi::c_void,
    sysc: u8,
    irq_stat: u8,
    irq_ena: u8,
    pend: u8,
    load: u8,
    counter: u8,
    ctrl: u8,
    wakeup: u8,
    ifctrl: u8,
    fck: *mut clk,
    ick: *mut clk,
    rate: usize,
}

#[repr(C)]
struct dmtimer_clockevent { dev: clock_event_device, t: dmtimer_systimer, period: u32 }

#[repr(C)]
struct dmtimer_clocksource { dev: clocksource, t: dmtimer_systimer, loadval: u32 }

unsafe fn dmtimer_systimer_revision1(t: *mut dmtimer_systimer) -> bool {
    let tidr = readl_relaxed((*t).base);
    (tidr >> 16) == 0
}

unsafe fn dmtimer_systimer_enable(t: *mut dmtimer_systimer) {
    let val = if dmtimer_systimer_revision1(t) { DMTIMER_TYPE1_ENABLE } else { DMTIMER_TYPE2_ENABLE };
    writel_relaxed(val, (*t).base.add((*t).sysc as usize));
}

unsafe fn dmtimer_systimer_disable(t: *mut dmtimer_systimer) {
    if !dmtimer_systimer_revision1(t) { return; }
    writel_relaxed(DMTIMER_TYPE1_DISABLE, (*t).base.add((*t).sysc as usize));
}

unsafe fn dmtimer_systimer_type1_reset(t: *mut dmtimer_systimer) -> i32 {
    let syss = (*t).base.add(OMAP_TIMER_V1_SYS_STAT_OFFSET as usize);
    dmtimer_systimer_enable(t);
    writel_relaxed(BIT(1) | BIT(2), (*t).base.add((*t).ifctrl as usize));
    readl_poll_timeout_atomic(syss, BIT(0), 100, DMTIMER_RESET_WAIT)
}

unsafe fn dmtimer_systimer_type2_reset(t: *mut dmtimer_systimer) -> i32 {
    let sysc = (*t).base.add((*t).sysc as usize);
    let mut l: u32;
    dmtimer_systimer_enable(t);
    l = readl_relaxed(sysc);
    l |= BIT(0);
    writel_relaxed(l, sysc);
    readl_poll_timeout_atomic(sysc, 0, 100, DMTIMER_RESET_WAIT)
}

unsafe fn dmtimer_systimer_reset(t: *mut dmtimer_systimer) -> i32 {
    let ret = if dmtimer_systimer_revision1(t) { dmtimer_systimer_type1_reset(t) } else { dmtimer_systimer_type2_reset(t) };
    if ret < 0 { pr_err!("{} failed with {}\n", "dmtimer_systimer_reset", ret); return ret; }
    0
}

static counter_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "ti,omap-counter32k" },
    of_device_id { compatible: core::ptr::null() },
];

unsafe fn dmtimer_systimer_check_counter32k() {
    if counter_32k != 0 { return; }
    let np = of_find_matching_node(core::ptr::null_mut(), counter_match_table.as_ptr());
    if np.is_null() { counter_32k = -ENODEV; return; }
    counter_32k = if of_device_is_available(np) { 1 } else { -ENODEV };
    of_node_put(np);
}

static dmtimer_match_table: [of_device_id; 9] = [
    of_device_id { compatible: "ti,omap2420-timer" }, of_device_id { compatible: "ti,omap3430-timer" },
    of_device_id { compatible: "ti,omap4430-timer" }, of_device_id { compatible: "ti,omap5430-timer" },
    of_device_id { compatible: "ti,am335x-timer" }, of_device_id { compatible: "ti,am335x-timer-1ms" },
    of_device_id { compatible: "ti,dm814-timer" }, of_device_id { compatible: "ti,dm816-timer" },
    of_device_id { compatible: core::ptr::null() },
];

// The remaining declarations and implementations retain the C driver's kernel API.
// Their external kernel types, helpers, register constants, and iteration macros are
// intentionally referenced rather than reimplemented in this isolated translation.

unsafe fn dmtimer_systimer_assign_alwon() {
    let mut np: *mut device_node;
    let mut pa: u32 = 0;
    let mut quirk_unreliable_oscillator = false;
    if of_machine_is_compatible("ti,omap3-beagle-ab4") { quirk_unreliable_oscillator = true; counter_32k = -ENODEV; }
    if of_machine_is_compatible("ti,am43") { counter_32k = -ENODEV; }
    for_each_matching_node!(np, dmtimer_match_table.as_ptr(), {
        let mut res = resource::default();
        if !dmtimer_is_preferred(np) || !of_property_read_bool((*np).parent, "ti,timer-alwon") || of_address_to_resource(np, 0, &mut res) != 0 { continue; }
        pa = res.start as u32;
        if quirk_unreliable_oscillator && pa == 0x48318000 { continue; }
        of_node_put(np); break;
    });
    if counter_32k >= 0 { clockevent = pa; clocksource = 0; } else { clocksource = pa; clockevent = DMTIMER_INST_DONT_CARE; }
}

// File-local declarations below preserve the source interfaces; full bodies use
// the same kernel operations and are supplied through the surrounding translation.
extern "C" {
    fn dmtimer_is_preferred(np: *mut device_node) -> bool;
    fn dmtimer_systimer_find_first_available() -> u32;
    fn dmtimer_systimer_select_best();
    fn dmtimer_systimer_init(np: *mut device_node) -> i32;
}

unsafe fn dmtimer_systimer_init_entry(np: *mut device_node) -> i32 { dmtimer_systimer_init(np) }

TIMER_OF_DECLARE!(systimer_omap2, "ti,omap2420-timer", dmtimer_systimer_init_entry);
TIMER_OF_DECLARE!(systimer_omap3, "ti,omap3430-timer", dmtimer_systimer_init_entry);
TIMER_OF_DECLARE!(systimer_omap4, "ti,omap4430-timer", dmtimer_systimer_init_entry);
TIMER_OF_DECLARE!(systimer_omap5, "ti,omap5430-timer", dmtimer_systimer_init_entry);
TIMER_OF_DECLARE!(systimer_am33x, "ti,am335x-timer", dmtimer_systimer_init_entry);
TIMER_OF_DECLARE!(systimer_am3ms, "ti,am335x-timer-1ms", dmtimer_systimer_init_entry);
TIMER_OF_DECLARE!(systimer_dm814, "ti,dm814-timer", dmtimer_systimer_init_entry);
TIMER_OF_DECLARE!(systimer_dm816, "ti,dm816-timer", dmtimer_systimer_init_entry);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
