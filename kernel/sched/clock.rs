// SPDX-License-Identifier: GPL-2.0-only
// Rust owner of kernel/sched/clock.c. Native helpers are ABI/architecture leaves.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod b {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/sched_clock_generated.rs"
    ));
}
use b::*;
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};
use kernel::ffi::{c_int, c_ulong};

// The weak exported C symbols only provide the architecture override ABI.
#[no_mangle]
pub unsafe extern "C" fn lupos_clock_default_sched_clock() -> u64 {
    (lupos_clock_jiffies().wrapping_sub(LUPOS_CLOCK_INITIAL_JIFFIES as c_ulong) as u64)
        .wrapping_mul(LUPOS_CLOCK_NSEC_PER_SEC as u64 / LUPOS_CLOCK_HZ as u64)
}

#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
static mut __sched_clock_stable_early: c_int = 1;
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut __sched_clock_offset: u64 = 0;
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[link_section = ".data..read_mostly"]
static mut __gtod_offset: u64 = 0;

#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_stable() -> c_int {
    lupos_clock_stable_branch() as c_int
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
unsafe fn __scd_stamp(scd: *mut sched_clock_data) {
    write_volatile(addr_of_mut!((*scd).tick_gtod), lupos_clock_ktime_get_ns());
    write_volatile(addr_of_mut!((*scd).tick_raw), sched_clock());
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
unsafe fn __set_sched_clock_stable() {
    lupos_clock_irq_disable();
    let scd = lupos_clock_this_scd();
    let gtod = read_volatile(addr_of!((*scd).tick_gtod));
    let raw = read_volatile(addr_of!((*scd).tick_raw));
    write_volatile(
        addr_of_mut!(__sched_clock_offset),
        gtod.wrapping_add(read_volatile(addr_of!(__gtod_offset)))
            .wrapping_sub(raw),
    );
    lupos_clock_irq_enable();
    lupos_clock_print_stable(
        read_volatile(addr_of!((*scd).tick_gtod)),
        read_volatile(addr_of!(__gtod_offset)),
        read_volatile(addr_of!((*scd).tick_raw)),
        read_volatile(addr_of!(__sched_clock_offset)),
    );
    lupos_clock_stable_enable();
    lupos_clock_tick_dep_clear();
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
pub unsafe extern "C" fn lupos_clock_work(_work: *mut work_struct) {
    lupos_clock_preempt_disable();
    let scd = lupos_clock_this_scd();
    __scd_stamp(scd);
    write_volatile(
        addr_of_mut!((*scd).clock),
        read_volatile(addr_of!((*scd).tick_gtod))
            .wrapping_add(read_volatile(addr_of!(__gtod_offset))),
    );
    lupos_clock_preempt_enable();
    // Preserve native for_each_possible_cpu(), including sparse CPU masks.
    let mut cpu = lupos_clock_next_possible(-1);
    while cpu < lupos_clock_nr_cpu_ids() as c_int {
        let dst = lupos_clock_cpu_scd(cpu);
        // Assignment of the complete three-field native record. The source can
        // be the destination; field loads therefore precede their own stores.
        write_volatile(
            addr_of_mut!((*dst).tick_raw),
            read_volatile(addr_of!((*scd).tick_raw)),
        );
        write_volatile(
            addr_of_mut!((*dst).tick_gtod),
            read_volatile(addr_of!((*scd).tick_gtod)),
        );
        write_volatile(
            addr_of_mut!((*dst).clock),
            read_volatile(addr_of!((*scd).clock)),
        );
        cpu = lupos_clock_next_possible(cpu);
    }
    lupos_clock_print_unstable(
        read_volatile(addr_of!((*scd).tick_gtod)),
        read_volatile(addr_of!(__gtod_offset)),
        read_volatile(addr_of!((*scd).tick_raw)),
        read_volatile(addr_of!(__sched_clock_offset)),
    );
    lupos_clock_disable_irqtime();
    lupos_clock_stable_disable();
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
unsafe fn __clear_sched_clock_stable() {
    if sched_clock_stable() == 0 {
        return;
    }
    lupos_clock_tick_dep_set();
    lupos_clock_schedule_work();
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
pub unsafe extern "C" fn clear_sched_clock_stable() {
    write_volatile(addr_of_mut!(__sched_clock_stable_early), 0);
    lupos_clock_smp_mb();
    if lupos_clock_running_count() == 2 {
        __clear_sched_clock_stable();
    }
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
unsafe fn __sched_clock_gtod_offset() {
    let scd = lupos_clock_this_scd();
    __scd_stamp(scd);
    write_volatile(
        addr_of_mut!(__gtod_offset),
        read_volatile(addr_of!((*scd).tick_raw))
            .wrapping_add(read_volatile(addr_of!(__sched_clock_offset)))
            .wrapping_sub(read_volatile(addr_of!((*scd).tick_gtod))),
    );
}
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn sched_clock_init() {
    #[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
    {
        lupos_clock_irq_disable();
        __sched_clock_gtod_offset();
        lupos_clock_irq_enable();
        lupos_clock_running_inc();
    }
    #[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
    {
        lupos_clock_running_inc();
        lupos_clock_irq_disable();
        lupos_clock_generic_init();
        lupos_clock_irq_enable();
    }
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn lupos_clock_init_late() -> c_int {
    lupos_clock_running_inc();
    lupos_clock_smp_mb();
    if read_volatile(addr_of!(__sched_clock_stable_early)) != 0 {
        __set_sched_clock_stable();
    } else {
        lupos_clock_disable_irqtime();
    }
    0
}
#[inline(always)]
fn wrap_min(x: u64, y: u64) -> u64 {
    if (x.wrapping_sub(y) as i64) < 0 {
        x
    } else {
        y
    }
}
#[inline(always)]
fn wrap_max(x: u64, y: u64) -> u64 {
    if (x.wrapping_sub(y) as i64) > 0 {
        x
    } else {
        y
    }
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[inline(always)]
unsafe fn sched_clock_local(scd: *mut sched_clock_data) -> u64 {
    loop {
        let now = lupos_clock_sched_clock_noinstr();
        let delta = now.wrapping_sub(read_volatile(addr_of!((*scd).tick_raw))) as i64;
        let delta = if delta < 0 { 0 } else { delta as u64 };
        let mut old_clock = read_volatile(addr_of!((*scd).clock));
        let gtod = read_volatile(addr_of!((*scd).tick_gtod))
            .wrapping_add(read_volatile(addr_of!(__gtod_offset)));
        let min_clock = wrap_max(gtod, old_clock);
        let max_clock = wrap_max(old_clock, gtod.wrapping_add(LUPOS_CLOCK_TICK_NSEC as u64));
        let clock = wrap_min(wrap_max(gtod.wrapping_add(delta), min_clock), max_clock);
        if lupos_clock_raw_try_cmpxchg64(addr_of_mut!((*scd).clock), &mut old_clock, clock) {
            return clock;
        }
    }
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
#[inline(never)]
#[link_section = ".noinstr.text"]
#[no_sanitize(address, hwaddress, memory, thread)]
pub unsafe extern "C" fn local_clock_noinstr() -> u64 {
    if lupos_clock_stable_branch() {
        return lupos_clock_sched_clock_noinstr()
            .wrapping_add(read_volatile(addr_of!(__sched_clock_offset)));
    }
    if !lupos_clock_running_branch() {
        return lupos_clock_sched_clock_noinstr();
    }
    sched_clock_local(lupos_clock_this_scd())
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
pub unsafe extern "C" fn local_clock() -> u64 {
    lupos_clock_preempt_disable_notrace();
    let now = local_clock_noinstr();
    lupos_clock_preempt_enable_notrace();
    now
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
unsafe fn sched_clock_remote(scd: *mut sched_clock_data) -> u64 {
    let my_scd = lupos_clock_this_scd();
    #[cfg(target_pointer_width = "64")]
    sched_clock_local(my_scd);
    loop {
        #[cfg(not(target_pointer_width = "64"))]
        let (this_clock, remote_clock) = (
            sched_clock_local(my_scd),
            lupos_clock_cmpxchg64(addr_of_mut!((*scd).clock), 0, 0),
        );
        #[cfg(target_pointer_width = "64")]
        let (this_clock, remote_clock) = (
            read_volatile(addr_of!((*my_scd).clock)),
            read_volatile(addr_of!((*scd).clock)),
        );
        let (ptr, mut old, val) = if (remote_clock.wrapping_sub(this_clock) as i64) < 0 {
            (addr_of_mut!((*scd).clock), remote_clock, this_clock)
        } else {
            (addr_of_mut!((*my_scd).clock), this_clock, remote_clock)
        };
        if lupos_clock_try_cmpxchg64(ptr, &mut old, val) {
            return val;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_clock_cpu(cpu: c_int) -> u64 {
    #[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
    {
        if sched_clock_stable() != 0 {
            return sched_clock().wrapping_add(read_volatile(addr_of!(__sched_clock_offset)));
        }
        if !lupos_clock_running_branch() {
            return sched_clock();
        }
        lupos_clock_preempt_disable_notrace();
        let scd = lupos_clock_cpu_scd(cpu);
        let now = if cpu != lupos_clock_cpu() {
            sched_clock_remote(scd)
        } else {
            sched_clock_local(scd)
        };
        lupos_clock_preempt_enable_notrace();
        now
    }
    #[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
    {
        let _ = cpu;
        if !lupos_clock_running_branch() {
            return 0;
        }
        sched_clock()
    }
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_tick() {
    if sched_clock_stable() != 0 || !lupos_clock_running_branch() {
        return;
    }
    lupos_clock_assert_irqs_disabled();
    let scd = lupos_clock_this_scd();
    __scd_stamp(scd);
    sched_clock_local(scd);
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_tick_stable() {
    if sched_clock_stable() == 0 {
        return;
    }
    lupos_clock_irq_disable();
    __sched_clock_gtod_offset();
    lupos_clock_irq_enable();
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_idle_sleep_event() {
    sched_clock_cpu(lupos_clock_cpu());
}
#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
#[no_mangle]
pub unsafe extern "C" fn sched_clock_idle_wakeup_event() {
    if sched_clock_stable() != 0 || read_volatile(addr_of!(timekeeping_suspended)) != 0 {
        return;
    }
    let flags = lupos_clock_irq_save();
    sched_clock_tick();
    lupos_clock_irq_restore(flags);
}
#[no_mangle]
pub unsafe extern "C" fn lupos_clock_default_running_clock() -> u64 {
    #[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
    {
        local_clock()
    }
    #[cfg(not(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK))]
    {
        sched_clock()
    }
}

#[cfg(CONFIG_HAVE_UNSTABLE_SCHED_CLOCK)]
const _: () = {
    assert!(core::mem::size_of::<sched_clock_data>() == LUPOS_CLOCK_SCD_SIZE as usize);
    assert!(core::mem::align_of::<sched_clock_data>() == LUPOS_CLOCK_SCD_ALIGN as usize);
    assert!(core::mem::offset_of!(sched_clock_data, tick_raw) == LUPOS_CLOCK_SCD_RAW as usize);
    assert!(core::mem::offset_of!(sched_clock_data, tick_gtod) == LUPOS_CLOCK_SCD_GTOD as usize);
    assert!(core::mem::offset_of!(sched_clock_data, clock) == LUPOS_CLOCK_SCD_CLOCK as usize);
};
