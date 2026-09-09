/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007 MIPS Technologies, Inc.
 * Copyright (C) 2007 Ralf Baechle <ralf@linux-mips.org>
 */

// Dependencies are supplied by the surrounding kernel translation.

static mut MIPS_CLOCKEVENT_DEVICE: /* DEFINE_PER_CPU */ clock_event_device = unsafe { core::mem::zeroed() };
pub static mut cp0_timer_irq_installed: i32 = 0;

unsafe fn mips_next_event(mut delta: c_ulong, evt: *mut clock_event_device) -> i32 {
    let mut cnt: c_uint;
    let res: i32;
    cnt = read_c0_count();
    cnt = cnt.wrapping_add(delta as c_uint);
    write_c0_compare(cnt);
    res = if (read_c0_count().wrapping_sub(cnt) as i32) >= 0 { -ETIME } else { 0 };
    res
}

unsafe fn calculate_min_delta() -> c_uint {
    let mut cnt: c_uint;
    let mut i: c_uint;
    let mut j: c_uint;
    let mut k: c_uint;
    let mut l: c_uint;
    let mut buf1 = [0 as c_uint; 4];
    let mut buf2 = [0 as c_uint; 3];
    let mut min_delta: c_uint;

    for i in 0..5 {
        for j in 0..5 {
            cnt = read_c0_count();
            write_c0_compare(cnt);
            cnt = read_c0_count().wrapping_sub(cnt);

            k = 0;
            while k < j {
                if cnt < buf1[k as usize] {
                    l = core::cmp::min(j, (buf1.len() - 1) as c_uint);
                    while l > k {
                        buf1[l as usize] = buf1[(l - 1) as usize];
                        l -= 1;
                    }
                    break;
                }
                k += 1;
            }
            if k < buf1.len() as c_uint { buf1[k as usize] = cnt; }
        }

        k = 0;
        while k < i && k < buf2.len() as c_uint {
            if buf1[buf1.len() - 1] < buf2[k as usize] {
                l = core::cmp::min(i, (buf2.len() - 1) as c_uint);
                while l > k {
                    buf2[l as usize] = buf2[(l - 1) as usize];
                    l -= 1;
                }
                break;
            }
            k += 1;
        }
        if k < buf2.len() as c_uint { buf2[k as usize] = buf1[buf1.len() - 1]; }
    }

    min_delta = buf2[buf2.len() - 1].wrapping_mul(2);
    if min_delta < 0x300 { min_delta = 0x300; }
    pr_debug!("{}: median 75th percentile={:#x}, min_delta={:#x}\n", "calculate_min_delta", buf2[buf2.len() - 1], min_delta);
    min_delta
}

unsafe fn handle_perf_irq(r2: i32) -> i32 {
    if cp0_perfcount_irq < 0 && perf_irq() == IRQ_HANDLED && r2 == 0 { 1 } else { 0 }
}

pub unsafe extern "C" fn c0_compare_interrupt(_irq: i32, _dev_id: *mut c_void) -> irqreturn_t {
    let r2 = cpu_has_mips_r2_r6;
    let cpu = smp_processor_id();
    if handle_perf_irq(r2) != 0 { return IRQ_HANDLED; }
    if r2 == 0 || (read_c0_cause() & CAUSEF_TI) != 0 {
        write_c0_compare(read_c0_compare());
        let cd = &mut MIPS_CLOCKEVENT_DEVICE;
        (cd.event_handler)(cd as *mut _);
        return IRQ_HANDLED;
    }
    IRQ_NONE
}

pub unsafe extern "C" fn mips_event_handler(_dev: *mut clock_event_device) {}

unsafe fn c0_compare_int_pending() -> i32 {
    ((read_c0_cause() >> cp0_compare_irq_shift) & (1u64 << CAUSEB_IP)) as i32
}

const COMPARE_INT_SEEN_TICKS: c_uint = 50;

pub unsafe extern "C" fn c0_compare_int_usable() -> i32 {
    let mut delta: c_uint;
    let mut cnt: c_uint;
    if c0_compare_int_pending() != 0 {
        cnt = read_c0_count();
        write_c0_compare(cnt.wrapping_sub(1));
        back_to_back_c0_hazard();
        while read_c0_count() < cnt.wrapping_add(COMPARE_INT_SEEN_TICKS) {
            if c0_compare_int_pending() == 0 { break; }
        }
        if c0_compare_int_pending() != 0 { return 0; }
    }
    delta = 0x10;
    loop {
        cnt = read_c0_count();
        cnt = cnt.wrapping_add(delta);
        write_c0_compare(cnt);
        back_to_back_c0_hazard();
        if (read_c0_count().wrapping_sub(cnt) as i32) < 0 { break; }
        delta <<= 1;
        if delta > 0x400000 { break; }
    }
    while (read_c0_count().wrapping_sub(cnt) as i32) <= 0 {}
    while read_c0_count() < cnt.wrapping_add(COMPARE_INT_SEEN_TICKS) {
        if c0_compare_int_pending() != 0 { break; }
    }
    if c0_compare_int_pending() == 0 { return 0; }
    cnt = read_c0_count();
    write_c0_compare(cnt.wrapping_sub(1));
    back_to_back_c0_hazard();
    while read_c0_count() < cnt.wrapping_add(COMPARE_INT_SEEN_TICKS) {
        if c0_compare_int_pending() == 0 { break; }
    }
    if c0_compare_int_pending() != 0 { return 0; }
    1
}

pub unsafe extern "C" fn get_c0_compare_int() -> c_uint { MIPS_CPU_IRQ_BASE + cp0_compare_irq }

pub unsafe extern "C" fn r4k_clockevent_init() -> i32 {
    let flags = IRQF_PERCPU | IRQF_TIMER | IRQF_SHARED;
    let cpu = smp_processor_id();
    if !cpu_has_counter || mips_hpt_frequency == 0 || c0_compare_int_usable() == 0 { return -ENXIO; }
    let cd = &mut MIPS_CLOCKEVENT_DEVICE;
    cd.name = "MIPS";
    cd.features = CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_C3STOP | CLOCK_EVT_FEAT_PERCPU;
    let min_delta = calculate_min_delta();
    cd.rating = 300;
    cd.cpumask = cpumask_of(cpu);
    cd.set_next_event = Some(mips_next_event);
    cd.event_handler = mips_event_handler;
    clockevents_config_and_register(cd, mips_hpt_frequency, min_delta, 0x7fffffff);
    if cp0_timer_irq_installed != 0 { return 0; }
    cp0_timer_irq_installed = 1;
    let irq = get_c0_compare_int();
    if request_irq(irq, c0_compare_interrupt, flags, "timer", c0_compare_interrupt) != 0 {
        pr_err!("Failed to request irq {} (timer)\n", irq);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
