// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the kernel and KUnit environment.

const TESTRL_INTERVAL: u64 = 5 * HZ;
static mut testrl: ratelimit_state = DEFINE_RATELIMIT_STATE!(TESTRL_INTERVAL, 3);

unsafe fn test_ratelimited(test: *mut kunit, expected: bool) {
    KUNIT_ASSERT_EQ!(test, ___ratelimit(&raw mut testrl, "test_ratelimit_smoke"), expected);
}

unsafe fn test_ratelimit_smoke(test: *mut kunit) {
    // Check settings.
    KUNIT_ASSERT_GE!(test, TESTRL_INTERVAL, 100);

    // Test normal operation.
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, false);

    schedule_timeout_idle(TESTRL_INTERVAL / 2);
    test_ratelimited(test, false);

    schedule_timeout_idle(TESTRL_INTERVAL * 3 / 4);
    test_ratelimited(test, true);

    schedule_timeout_idle(2 * TESTRL_INTERVAL);
    test_ratelimited(test, true);
    test_ratelimited(test, true);

    schedule_timeout_idle(TESTRL_INTERVAL / 2);
    test_ratelimited(test, true);
    schedule_timeout_idle(TESTRL_INTERVAL * 3 / 4);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, false);

    // Test disabling.
    (*(&raw mut testrl)).burst = 0;
    test_ratelimited(test, false);
    (*(&raw mut testrl)).burst = 2;
    (*(&raw mut testrl)).interval = 0;
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, true);

    // Testing re-enabling.
    (*(&raw mut testrl)).interval = TESTRL_INTERVAL;
    test_ratelimited(test, true);
    test_ratelimited(test, true);
    test_ratelimited(test, false);
    test_ratelimited(test, false);
}

static mut stressrl: ratelimit_state =
    RATELIMIT_STATE_INIT_FLAGS!("stressrl", HZ / 10, 3, RATELIMIT_MSG_ON_RELEASE);

static stress_duration: i32 = 2 * HZ;

#[repr(C)]
struct stress_kthread {
    nattempts: c_ulong,
    nunlimited: c_ulong,
    nlimited: c_ulong,
    nmissed: c_ulong,
    tp: *mut task_struct,
}

unsafe fn test_ratelimit_stress_child(arg: *mut c_void) -> i32 {
    let sktp = arg as *mut stress_kthread;

    set_user_nice(current, MAX_NICE);

    while !kthread_should_stop() {
        (*sktp).nattempts += 1;
        if ___ratelimit(&raw mut stressrl, "test_ratelimit_stress_child") {
            (*sktp).nunlimited += 1;
        } else {
            (*sktp).nlimited += 1;
        }
        cond_resched();
    }

    (*sktp).nmissed = ratelimit_state_reset_miss(&raw mut stressrl);
    0
}

unsafe fn test_ratelimit_stress(test: *mut kunit) {
    let mut i: i32;
    let n_stress_kthread = cpumask_weight(cpu_online_mask);
    let mut skt: stress_kthread = core::mem::zeroed();
    let sktp = kzalloc_objs!(n_stress_kthread);
    let mut n_started = 0;

    KUNIT_ASSERT_NOT_NULL_MSG!(test, sktp, "Memory allocation failure");
    for i in 0..n_stress_kthread {
        (*sktp.add(i as usize)).tp = kthread_run(
            test_ratelimit_stress_child,
            sktp.add(i as usize) as *mut c_void,
            "%s/%i",
            "test_ratelimit_stress_child",
            i,
        );
        if IS_ERR((*sktp.add(i as usize)).tp) {
            KUNIT_FAIL!(test, "kthread_run failed: %ld", PTR_ERR((*sktp.add(i as usize)).tp));
            break;
        }
        n_started += 1;
        pr_alert!("Spawned test_ratelimit_stress_child %d\n", i);
    }
    schedule_timeout_idle(stress_duration);

    for i in 0..n_started {
        kthread_stop((*sktp.add(i as usize)).tp);
        skt.nattempts += (*sktp.add(i as usize)).nattempts;
        skt.nunlimited += (*sktp.add(i as usize)).nunlimited;
        skt.nlimited += (*sktp.add(i as usize)).nlimited;
        skt.nmissed += (*sktp.add(i as usize)).nmissed;
    }
    if n_started == n_stress_kthread {
        KUNIT_ASSERT_EQ_MSG!(test, skt.nunlimited + skt.nlimited, skt.nattempts,
                             "Outcomes not equal to attempts");
        KUNIT_ASSERT_EQ_MSG!(test, skt.nlimited, skt.nmissed,
                             "Misses not equal to limits");
    }

    kfree(sktp);
}

static mut ratelimit_test_cases: [kunit_case; 3] = [
    KUNIT_CASE_SLOW!(test_ratelimit_smoke),
    KUNIT_CASE_SLOW!(test_ratelimit_stress),
    KUNIT_CASE_EMPTY!(),
];

static mut ratelimit_test_suite: kunit_suite = kunit_suite {
    name: "lib_ratelimit",
    test_cases: &raw mut ratelimit_test_cases,
};

kunit_test_suites!(&raw mut ratelimit_test_suite);

MODULE_DESCRIPTION!("___ratelimit() KUnit test suite");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
