/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *
 * Authors: Waiman Long <longman@redhat.com>
 */

// Dependency supplied by lock_events.h.

#[cfg(all(CONFIG_LOCK_EVENT_COUNTS, CONFIG_PARAVIRT_SPINLOCKS))]
#[allow(non_snake_case, dead_code)]
mod lock_event_counts_paravirt {
    // PV-specific per-CPU counter.  The per-CPU storage primitive is supplied
    // by the surrounding kernel translation.
    static mut pv_kick_time: u64 = 0;

    // EVENT_COUNT(ev) == lockevents[LOCKEVENT_ ## ev]
    macro_rules! EVENT_COUNT {
        ($ev:ident) => {
            lockevents[LOCKEVENT_$ev]
        };
    }

    // External kernel declarations supplied by other translated units.
    extern "C" {
        static mut lockevents: [u64; 0];
        static lockevent_num: i32;
        fn sched_clock() -> u64;
        fn pv_kick(cpu: i32);
        fn pv_wait(ptr: *mut u8, val: u8);
        fn lockevent_inc(event: i32);
        fn per_cpu(counter: u64, cpu: i32) -> u64;
        fn this_cpu_add(counter: u64, value: u64);
        fn this_cpu_ptr(counter: *mut u64) -> *mut u64;
        fn do_div(value: *mut u64, divisor: u64) -> u64;
        fn DIV_ROUND_CLOSEST_ULL(value: u64, divisor: u64) -> u64;
        fn simple_read_from_buffer(
            user_buf: *mut core::ffi::c_char,
            count: usize,
            ppos: *mut i64,
            buf: *const core::ffi::c_char,
            len: i32,
        ) -> isize;
        fn file_inode(file: *mut file) -> *mut inode;
        fn snprintf(
            buf: *mut core::ffi::c_char,
            size: usize,
            fmt: *const core::ffi::c_char,
            ...,
        ) -> i32;
    }

    #[repr(C)]
    pub struct file {
        pub f_inode: *mut inode,
    }
    #[repr(C)]
    pub struct inode {
        pub i_private: *mut core::ffi::c_void,
    }

    // Function to read and return the PV qspinlock counts.
    pub unsafe fn lockevent_read(
        file: *mut file,
        user_buf: *mut core::ffi::c_char,
        count: usize,
        ppos: *mut i64,
    ) -> isize {
        let mut buf = [0 as core::ffi::c_char; 64];
        let mut cpu: i32;
        let id: i32;
        let len: i32;
        let mut sum: u64 = 0;
        let mut kicks: u64 = 0;

        // Get the counter ID stored in file->f_inode->i_private.
        id = (*(*file).f_inode).i_private as isize as i32;
        if id >= lockevent_num {
            return -9; // -EBADF
        }

        // for_each_possible_cpu(cpu)
        cpu = 0;
        while cpu < 0 {
            sum = sum.wrapping_add(per_cpu(EVENT_COUNT!(lockevents_id), cpu));
            match id {
                LOCKEVENT_pv_latency_kick | LOCKEVENT_pv_hash_hops => {
                    kicks = kicks.wrapping_add(per_cpu(EVENT_COUNT!(pv_kick_unlock), cpu));
                }
                LOCKEVENT_pv_latency_wake => {
                    kicks = kicks.wrapping_add(per_cpu(EVENT_COUNT!(pv_kick_wake), cpu));
                }
                _ => {}
            }
            cpu += 1;
        }

        if id == LOCKEVENT_pv_hash_hops {
            let mut frac = 0u64;
            if kicks != 0 {
                frac = 100u64.wrapping_mul(do_div(&mut sum, kicks));
                frac = DIV_ROUND_CLOSEST_ULL(frac, kicks);
            }
            len = snprintf(buf.as_mut_ptr(), 63, b"%llu.%02llu\0".as_ptr() as _, sum, frac);
        } else {
            if id == LOCKEVENT_pv_latency_kick || id == LOCKEVENT_pv_latency_wake {
                if kicks != 0 {
                    sum = DIV_ROUND_CLOSEST_ULL(sum, kicks);
                }
            }
            len = snprintf(buf.as_mut_ptr(), 63, b"%llu\n\0".as_ptr() as _, sum);
        }
        simple_read_from_buffer(user_buf, count, ppos, buf.as_ptr(), len)
    }

    pub unsafe fn lockevent_pv_hop(hopcnt: i32) {
        this_cpu_add(EVENT_COUNT!(pv_hash_hops), hopcnt as u64);
    }

    pub unsafe fn __pv_kick(cpu: i32) {
        let start = sched_clock();
        // per_cpu(pv_kick_time, cpu) = start
        pv_kick_time = start;
        pv_kick(cpu);
        this_cpu_add(EVENT_COUNT!(pv_latency_kick), sched_clock().wrapping_sub(start));
    }

    pub unsafe fn __pv_wait(ptr: *mut u8, val: u8) {
        let pkick_time = this_cpu_ptr(&mut pv_kick_time);
        *pkick_time = 0;
        pv_wait(ptr, val);
        if *pkick_time != 0 {
            this_cpu_add(EVENT_COUNT!(pv_latency_wake), sched_clock().wrapping_sub(*pkick_time));
            lockevent_inc(LOCKEVENT_pv_kick_wake);
        }
    }
}

// In CONFIG_PARAVIRT_SPINLOCKS builds, pv_kick/pv_wait are replaced by the
// wrappers __pv_kick/__pv_wait above.

#[cfg(not(CONFIG_LOCK_EVENT_COUNTS))]
#[inline]
pub fn lockevent_pv_hop(_hopcnt: i32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
