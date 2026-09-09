// SPDX-License-Identifier: GPL-2.0-only
// Translated from oct_ilm.c. Kernel and Octeon declarations are supplied by
// the surrounding build environment.

const TIMER_NUM: i32 = 3;

static mut RESET_STATS: bool = false;

#[repr(C)]
struct LatencyInfo {
    io_interval: u64,
    cpu_interval: u64,
    timer_start1: u64,
    timer_start2: u64,
    max_latency: u64,
    min_latency: u64,
    latency_sum: u64,
    average_latency: u64,
    interrupt_cnt: u64,
}

static mut LI: LatencyInfo = LatencyInfo {
    io_interval: 0,
    cpu_interval: 0,
    timer_start1: 0,
    timer_start2: 0,
    max_latency: 0,
    min_latency: 0,
    latency_sum: 0,
    average_latency: 0,
    interrupt_cnt: 0,
};

static mut DIR: *mut dentry = core::ptr::null_mut();

unsafe fn oct_ilm_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let cpuclk: u64;
    let avg: u64;
    let max: u64;
    let min: u64;
    let curr_li = LI;

    cpuclk = octeon_get_clock_rate();
    max = curr_li.max_latency.wrapping_mul(1_000_000_000) / cpuclk;
    min = curr_li.min_latency.wrapping_mul(1_000_000_000) / cpuclk;
    avg = curr_li.latency_sum.wrapping_mul(1_000_000_000)
        / cpuclk.wrapping_mul(curr_li.interrupt_cnt);

    seq_printf(
        m,
        b"cnt: %10lld, avg: %7lld ns, max: %7lld ns, min: %7lld ns\n\0".as_ptr() as *const i8,
        curr_li.interrupt_cnt,
        avg,
        max,
        min,
    );
    0
}

unsafe fn reset_statistics(_data: *mut core::ffi::c_void, _value: u64) -> i32 {
    RESET_STATS = true;
    0
}

unsafe fn init_debugfs() {
    DIR = debugfs_create_dir(b"oct_ilm\0".as_ptr() as *const i8, core::ptr::null_mut());
    debugfs_create_file(
        b"statistics\0".as_ptr() as *const i8,
        0o222,
        DIR,
        core::ptr::null_mut(),
        &oct_ilm_fops,
    );
    debugfs_create_file(
        b"reset\0".as_ptr() as *const i8,
        0o222,
        DIR,
        core::ptr::null_mut(),
        &reset_statistics_ops,
    );
}

unsafe fn init_latency_info(li: *mut LatencyInfo, startup: i32) {
    // Interval in milliseconds after which the interrupt will be triggered.
    let interval: u64 = 1;

    if startup != 0 {
        // Calculate by the amounts the IO and CPU clocks increment in the interval.
        (*li).io_interval = octeon_get_io_clock_rate().wrapping_mul(interval) / 1000;
        (*li).cpu_interval = octeon_get_clock_rate().wrapping_mul(interval) / 1000;
    }
    (*li).timer_start1 = 0;
    (*li).timer_start2 = 0;
    (*li).max_latency = 0;
    (*li).min_latency = u64::MAX;
    (*li).latency_sum = 0;
    (*li).interrupt_cnt = 0;
}

unsafe fn start_timer(timer: i32, interval: u64) {
    let mut timx = cvmx_ciu_timx { u64_: 0 };
    let mut flags: c_ulong = 0;

    (*timx.s_mut()).one_shot = 1;
    (*timx.s_mut()).len = interval;
    raw_local_irq_save(&mut flags);
    LI.timer_start1 = read_c0_cvmcount();
    cvmx_write_csr(cvmx_ciu_timx_addr(timer), timx.u64_);
    // Read it back to force waiting until the register is written.
    timx.u64_ = cvmx_read_csr(cvmx_ciu_timx_addr(timer));
    LI.timer_start2 = read_c0_cvmcount();
    raw_local_irq_restore(flags);
}

unsafe fn cvm_oct_ciu_timer_interrupt(_cpl: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let last_latency: u64;
    let last_int_cnt: u64;

    if RESET_STATS {
        init_latency_info(&mut LI, 0);
        RESET_STATS = false;
    } else {
        last_int_cnt = read_c0_cvmcount();
        last_latency = last_int_cnt.wrapping_sub(LI.timer_start1.wrapping_add(LI.cpu_interval));
        LI.interrupt_cnt = LI.interrupt_cnt.wrapping_add(1);
        LI.latency_sum = LI.latency_sum.wrapping_add(last_latency);
        if last_latency > LI.max_latency {
            LI.max_latency = last_latency;
        }
        if last_latency < LI.min_latency {
            LI.min_latency = last_latency;
        }
    }
    start_timer(TIMER_NUM, LI.io_interval);
    IRQ_HANDLED
}

unsafe fn disable_timer(timer: i32) {
    let mut timx = cvmx_ciu_timx { u64_: 0 };

    (*timx.s_mut()).one_shot = 0;
    (*timx.s_mut()).len = 0;
    cvmx_write_csr(cvmx_ciu_timx_addr(timer), timx.u64_);
    // Read it back to force immediate write of the timer register.
    timx.u64_ = cvmx_read_csr(cvmx_ciu_timx_addr(timer));
}

unsafe fn oct_ilm_module_init() -> i32 {
    let rc: i32;
    let irq = OCTEON_IRQ_TIMER0 + TIMER_NUM;

    init_debugfs();
    rc = request_irq(irq, cvm_oct_ciu_timer_interrupt, IRQF_NO_THREAD, b"oct_ilm\0".as_ptr() as *const i8, 0);
    if rc != 0 {
        WARN(1, b"Could not acquire IRQ %d\0".as_ptr() as *const i8, irq);
        debugfs_remove_recursive(DIR);
        return rc;
    }

    init_latency_info(&mut LI, 1);
    start_timer(TIMER_NUM, LI.io_interval);
    0
}

unsafe fn oct_ilm_module_exit() {
    disable_timer(TIMER_NUM);
    debugfs_remove_recursive(DIR);
    free_irq(OCTEON_IRQ_TIMER0 + TIMER_NUM, 0);
}

// module_exit(oct_ilm_module_exit);
// module_init(oct_ilm_module_init);
// MODULE_AUTHOR("Venkat Subbiah, Cavium");
// MODULE_DESCRIPTION("Measures interrupt latency on Octeon chips.");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
