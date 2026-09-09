/*
 * OR1K timer synchronisation
 *
 * Based on work from MIPS implementation.
 *
 * All CPUs will have their count registers synchronised to the CPU0 next time
 * value. This can cause a small timewarp for CPU0. All other CPU's should
 * not have done anything significant (but they may have had interrupts
 * enabled briefly - prom_smp_finish() should not be responsible for enabling
 * interrupts...)
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut initcount: ::core::ffi::c_uint = 0;
static mut count_count_start: atomic_t = ATOMIC_INIT(0);
static mut count_count_stop: atomic_t = ATOMIC_INIT(0);

const COUNTON: ::core::ffi::c_uint = 100;
const NR_LOOPS: ::core::ffi::c_int = 3;

pub unsafe fn synchronise_count_master(cpu: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int;
    let mut flags: ::core::ffi::c_ulong;

    pr_info!("Synchronize counters for CPU %u: ", cpu);

    local_irq_save!(flags);

    /*
     * We loop a few times to get a primed instruction cache,
     * then the last pass is more or less synchronised and
     * the master and slaves each set their cycle counters to a known
     * value all at once. This reduces the chance of having random offsets
     * between the processors, and guarantees that the maximum
     * delay between the cycle counters is never bigger than
     * the latency of information-passing (cachelines) between
     * two CPUs.
     */

    i = 0;
    while i < NR_LOOPS {
        /* slaves loop on '!= 2' */
        while atomic_read(&raw const { count_count_start }) != 1 {
            mb();
        }
        atomic_set(&raw mut { count_count_stop }, 0);
        smp_wmb();

        /* Let the slave writes its count register */
        atomic_inc(&raw mut { count_count_start });

        /* Count will be initialised to current timer */
        if i == 1 {
            initcount = get_cycles();
        }

        /*
         * Everyone initialises count in the last loop:
         */
        if i == NR_LOOPS - 1 {
            openrisc_timer_set(initcount);
        }

        /*
         * Wait for slave to leave the synchronization point:
         */
        while atomic_read(&raw const { count_count_stop }) != 1 {
            mb();
        }
        atomic_set(&raw mut { count_count_start }, 0);
        smp_wmb();
        atomic_inc(&raw mut { count_count_stop });

        i += 1;
    }
    /* Arrange for an interrupt in a short while */
    openrisc_timer_set_next(COUNTON);

    local_irq_restore!(flags);

    /*
     * i386 code reported the skew here, but the
     * count registers were almost certainly out of sync
     * so no point in alarming people
     */
    pr_cont!("done.\n");
}

pub unsafe fn synchronise_count_slave(_cpu: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int;

    /*
     * Not every cpu is online at the time this gets called,
     * so we first wait for the master to say everyone is ready
     */

    i = 0;
    while i < NR_LOOPS {
        atomic_inc(&raw mut { count_count_start });
        while atomic_read(&raw const { count_count_start }) != 2 {
            mb();
        }

        /*
         * Everyone initialises count in the last loop:
         */
        if i == NR_LOOPS - 1 {
            openrisc_timer_set(initcount);
        }

        atomic_inc(&raw mut { count_count_stop });
        while atomic_read(&raw const { count_count_stop }) != 2 {
            mb();
        }

        i += 1;
    }
    /* Arrange for an interrupt in a short while */
    openrisc_timer_set_next(COUNTON);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
