// SPDX-License-Identifier: GPL-2.0

/*
 * CPU accounting code for task groups.
 *
 * Based on the work by Paul Menage (menage@google.com) and Balbir Singh
 * (balbir@in.ibm.com).
 */
// Continued against native oracle 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
// Native headers own C layouts/constants; this remains a source-only owner.
#[cfg(CONFIG_RUST_SCHED_CPUACCT)]
compile_error!("SOURCE ONLY HOLD: scheduler cpuacct is not admitted");

use core::ptr::{addr_of_mut, null_mut};
use kernel::bindings::sched_cpuacct_native::*;
use kernel::ffi::{c_int, c_void};

type CpuacctStatIndex = cpuacct_stat_index;

unsafe fn css_ca(css: *mut cgroup_subsys_state) -> *mut cpuacct {
    // SAFETY: A non-null css belongs to this controller and is pinned by caller.
    unsafe {
        if !css.is_null() { lupos_cpuacct_from_css(css) } else { null_mut() }
    }
}

unsafe fn task_ca(tsk: *mut task_struct) -> *mut cpuacct {
    // SAFETY: Caller holds the native task/css lifetime and lookup contract.
    unsafe { css_ca(lupos_cpuacct_task_css(tsk)) }
}

unsafe fn parent_ca(ca: *mut cpuacct) -> *mut cpuacct {
    // SAFETY: ca and its css parent remain pinned during this traversal.
    unsafe { css_ca(lupos_cpuacct_parent_css(ca)) }
}

/// Allocate this controller's css, or return its native static root.
///
/// # Safety
/// Called by cgroup allocation with a live parent and sleeping allocation context.
#[export_name = "lupos_cpuacct_css_alloc"]
pub unsafe extern "C" fn cpuacct_css_alloc(parent_css: *mut cgroup_subsys_state) -> *mut cgroup_subsys_state {
    // SAFETY: Until success, only this callback owns the new allocation.
    unsafe {
        if parent_css.is_null() { return lupos_cpuacct_css(lupos_cpuacct_root()); }
        let ca = lupos_cpuacct_alloc();
        if ca.is_null() { return lupos_cpuacct_nomem(); }
        if !lupos_cpuacct_alloc_usage(ca) {
            lupos_cpuacct_free(ca);
            return lupos_cpuacct_nomem();
        }
        if !lupos_cpuacct_alloc_stat(ca) {
            lupos_cpuacct_free_usage(ca);
            lupos_cpuacct_free(ca);
            return lupos_cpuacct_nomem();
        }
        lupos_cpuacct_css(ca)
    }
}

/// Release a non-root controller allocation after cgroup teardown.
///
/// # Safety
/// cgroup core has excluded all readers and invokes this once for the live css.
#[export_name = "lupos_cpuacct_css_free"]
pub unsafe extern "C" fn cpuacct_css_free(css: *mut cgroup_subsys_state) {
    // SAFETY: The teardown callback owns all three allocations exclusively.
    unsafe {
        let ca = css_ca(css);
        lupos_cpuacct_free_stat(ca);
        lupos_cpuacct_free_usage(ca);
        lupos_cpuacct_free(ca);
    }
}

unsafe fn cpuacct_cpuusage_read(ca: *mut cpuacct, cpu: c_int, index: CpuacctStatIndex) -> u64 {
    // SAFETY: ca is pinned and cpu is possible. Native rq locking protects the
    // 64-bit loads on 32-bit platforms, exactly as in the original read path.
    unsafe {
        let cpuusage = lupos_cpuacct_usage_ptr(ca, cpu);
        let cpustat = lupos_cpuacct_stat_ptr(ca, cpu);
        if lupos_cpuacct_warn_index(index) { return 0; }
        #[cfg(not(CONFIG_64BIT))]
        lupos_cpuacct_rq_lock_irq(cpu);
        let data = if index.0 == CPUACCT_STAT_USER.0 {
            lupos_cpuacct_read_counter(cpustat.add(CPUTIME_USER.0 as usize))
                .wrapping_add(lupos_cpuacct_read_counter(cpustat.add(CPUTIME_NICE.0 as usize)))
        } else if index.0 == CPUACCT_STAT_SYSTEM.0 {
            lupos_cpuacct_read_counter(cpustat.add(CPUTIME_SYSTEM.0 as usize))
                .wrapping_add(lupos_cpuacct_read_counter(cpustat.add(CPUTIME_IRQ.0 as usize)))
                .wrapping_add(lupos_cpuacct_read_counter(cpustat.add(CPUTIME_SOFTIRQ.0 as usize)))
        } else {
            lupos_cpuacct_read_counter(cpuusage)
        };
        #[cfg(not(CONFIG_64BIT))]
        lupos_cpuacct_rq_unlock_irq(cpu);
        data
    }
}

unsafe fn cpuacct_cpuusage_write(ca: *mut cpuacct, cpu: c_int) {
    // SAFETY: Root is never reset. A pinned non-root group and possible CPU
    // supply valid slots; 32-bit writes hold the native rq lock with IRQs off.
    unsafe {
        let cpuusage = lupos_cpuacct_usage_ptr(ca, cpu);
        let cpustat = lupos_cpuacct_stat_ptr(ca, cpu);
        if ca == lupos_cpuacct_root() { return; }
        #[cfg(not(CONFIG_64BIT))]
        lupos_cpuacct_rq_lock_irq(cpu);
        lupos_cpuacct_write_counter(cpuusage, 0);
        lupos_cpuacct_write_counter(cpustat.add(CPUTIME_NICE.0 as usize), 0);
        lupos_cpuacct_write_counter(cpustat.add(CPUTIME_USER.0 as usize), 0);
        lupos_cpuacct_write_counter(cpustat.add(CPUTIME_IRQ.0 as usize), 0);
        lupos_cpuacct_write_counter(cpustat.add(CPUTIME_SYSTEM.0 as usize), 0);
        lupos_cpuacct_write_counter(cpustat.add(CPUTIME_SOFTIRQ.0 as usize), 0);
        #[cfg(not(CONFIG_64BIT))]
        lupos_cpuacct_rq_unlock_irq(cpu);
    }
}

unsafe fn __cpuusage_read(css: *mut cgroup_subsys_state, index: CpuacctStatIndex) -> u64 {
    // SAFETY: The cgroup callback pins css throughout the possible-CPU walk.
    unsafe {
        let ca = css_ca(css);
        let mut totalcpuusage = 0u64;
        let mut i = lupos_cpuacct_next_possible_cpu(-1);
        while i >= 0 {
            totalcpuusage = totalcpuusage.wrapping_add(cpuacct_cpuusage_read(ca, i, index));
            i = lupos_cpuacct_next_possible_cpu(i);
        }
        totalcpuusage
    }
}

#[export_name = "lupos_cpuacct_usage_user_read"]
unsafe extern "C" fn cpuusage_user_read(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> u64 {
    // SAFETY: cgroup core supplies a pinned css to the registered callback.
    unsafe { __cpuusage_read(css, CPUACCT_STAT_USER) }
}
#[export_name = "lupos_cpuacct_usage_sys_read"]
unsafe extern "C" fn cpuusage_sys_read(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> u64 {
    // SAFETY: cgroup core supplies a pinned css to the registered callback.
    unsafe { __cpuusage_read(css, CPUACCT_STAT_SYSTEM) }
}
#[export_name = "lupos_cpuacct_usage_read"]
unsafe extern "C" fn cpuusage_read(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> u64 {
    // SAFETY: cgroup core supplies a pinned css to the registered callback.
    unsafe { __cpuusage_read(css, CPUACCT_STAT_NSTATS) }
}

#[export_name = "lupos_cpuacct_usage_write"]
unsafe extern "C" fn cpuusage_write(css: *mut cgroup_subsys_state, _cft: *mut cftype, val: u64) -> c_int {
    // SAFETY: cgroup core pins css during the reset. Nonzero requests fail
    // before any accounting storage is changed.
    unsafe {
        let ca = css_ca(css);
        if val != 0 { return -(LUPOS_CPUACCT_EINVAL as c_int); }
        let mut cpu = lupos_cpuacct_next_possible_cpu(-1);
        while cpu >= 0 {
            cpuacct_cpuusage_write(ca, cpu);
            cpu = lupos_cpuacct_next_possible_cpu(cpu);
        }
        0
    }
}

unsafe fn __cpuacct_percpu_seq_show(m: *mut seq_file, index: CpuacctStatIndex) -> c_int {
    // SAFETY: cgroup seq context pins its css and owns the writable seq_file.
    unsafe {
        let ca = css_ca(lupos_cpuacct_seq_css(m));
        let mut i = lupos_cpuacct_next_possible_cpu(-1);
        while i >= 0 {
            lupos_cpuacct_seq_percpu(m, cpuacct_cpuusage_read(ca, i, index));
            i = lupos_cpuacct_next_possible_cpu(i);
        }
        lupos_cpuacct_seq_percpu_end(m);
        0
    }
}
#[export_name = "lupos_cpuacct_percpu_user_seq_show"]
unsafe extern "C" fn cpuacct_percpu_user_seq_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: Native seq callback owns m and pins the cgroup css.
    unsafe { __cpuacct_percpu_seq_show(m, CPUACCT_STAT_USER) }
}
#[export_name = "lupos_cpuacct_percpu_sys_seq_show"]
unsafe extern "C" fn cpuacct_percpu_sys_seq_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: Native seq callback owns m and pins the cgroup css.
    unsafe { __cpuacct_percpu_seq_show(m, CPUACCT_STAT_SYSTEM) }
}
#[export_name = "lupos_cpuacct_percpu_seq_show"]
unsafe extern "C" fn cpuacct_percpu_seq_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: Native seq callback owns m and pins the cgroup css.
    unsafe { __cpuacct_percpu_seq_show(m, CPUACCT_STAT_NSTATS) }
}

#[export_name = "lupos_cpuacct_all_seq_show"]
unsafe extern "C" fn cpuacct_all_seq_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: The native seq context pins the css. The index remains strictly
    // below the native stat count, including at every descriptor-table lookup.
    unsafe {
        let ca = css_ca(lupos_cpuacct_seq_css(m));
        lupos_cpuacct_seq_cpu_header(m);
        let mut index = CPUACCT_STAT_USER.0;
        while index < CPUACCT_STAT_NSTATS.0 {
            lupos_cpuacct_seq_stat_header(m, cpuacct_stat_index(index));
            index += 1;
        }
        lupos_cpuacct_seq_newline(m);
        let mut cpu = lupos_cpuacct_next_possible_cpu(-1);
        while cpu >= 0 {
            lupos_cpuacct_seq_cpu(m, cpu);
            index = CPUACCT_STAT_USER.0;
            while index < CPUACCT_STAT_NSTATS.0 {
                lupos_cpuacct_seq_usage(m, cpuacct_cpuusage_read(ca, cpu, cpuacct_stat_index(index)));
                index += 1;
            }
            lupos_cpuacct_seq_newline(m);
            cpu = lupos_cpuacct_next_possible_cpu(cpu);
        }
        0
    }
}

#[export_name = "lupos_cpuacct_stats_show"]
unsafe extern "C" fn cpuacct_stats_show(sf: *mut seq_file, _v: *mut c_void) -> c_int {
    // SAFETY: task_cputime is generated from its native all-integer definition;
    // zero initialization matches memset. css/prev remain pinned by seq context.
    // This path intentionally retains the original unlocked stat aggregation.
    unsafe {
        let ca = css_ca(lupos_cpuacct_seq_css(sf));
        let mut cputime: task_cputime = core::mem::zeroed();
        let mut cpu = lupos_cpuacct_next_possible_cpu(-1);
        while cpu >= 0 {
            let cpustat = lupos_cpuacct_stat_ptr(ca, cpu);
            cputime.utime = cputime.utime
                .wrapping_add(lupos_cpuacct_read_counter(cpustat.add(CPUTIME_USER.0 as usize)))
                .wrapping_add(lupos_cpuacct_read_counter(cpustat.add(CPUTIME_NICE.0 as usize)));
            cputime.stime = cputime.stime
                .wrapping_add(lupos_cpuacct_read_counter(cpustat.add(CPUTIME_SYSTEM.0 as usize)))
                .wrapping_add(lupos_cpuacct_read_counter(cpustat.add(CPUTIME_IRQ.0 as usize)))
                .wrapping_add(lupos_cpuacct_read_counter(cpustat.add(CPUTIME_SOFTIRQ.0 as usize)));
            cputime.sum_exec_runtime = cputime.sum_exec_runtime
                .wrapping_add(lupos_cpuacct_read_counter(lupos_cpuacct_usage_ptr(ca, cpu)));
            cpu = lupos_cpuacct_next_possible_cpu(cpu);
        }
        let mut val = [0u64; CPUACCT_STAT_NSTATS.0 as usize];
        let values = val.as_mut_ptr();
        cputime_adjust(
            addr_of_mut!(cputime),
            lupos_cpuacct_seq_prev_cputime(sf),
            values.add(CPUACCT_STAT_USER.0 as usize),
            values.add(CPUACCT_STAT_SYSTEM.0 as usize),
        );
        let mut stat = CPUACCT_STAT_USER.0;
        while stat < CPUACCT_STAT_NSTATS.0 {
            lupos_cpuacct_seq_stat(sf, cpuacct_stat_index(stat), *values.add(stat as usize));
            stat += 1;
        }
        0
    }
}

/// Charge execution time to the task's accounting group and every ancestor.
///
/// # Safety
/// Caller pins tsk and its cgroup ancestry and holds the task's rq lock.
#[no_mangle]
pub unsafe extern "C" fn cpuacct_charge(tsk: *mut task_struct, cputime: u64) {
    // SAFETY: The rq lock serializes task CPU selection and accounting updates.
    unsafe {
        let cpu = lupos_cpuacct_task_cpu(tsk);
        lupos_cpuacct_assert_rq_held(cpu);
        let mut ca = task_ca(tsk);
        while !ca.is_null() {
            let usage = lupos_cpuacct_usage_ptr(ca, cpu as c_int);
            lupos_cpuacct_write_counter(usage, lupos_cpuacct_read_counter(usage).wrapping_add(cputime));
            ca = parent_ca(ca);
        }
    }
}

/// Add a user/system accounting field to non-root ancestors on this CPU.
///
/// # Safety
/// Caller pins tsk and its ancestry, supplies a native valid cpustat index,
/// and satisfies __this_cpu_add's CPU-local serialization rules. Caller alone
/// updates the root kernel_cpustat account.
#[no_mangle]
pub unsafe extern "C" fn cpuacct_account_field(tsk: *mut task_struct, index: c_int, val: u64) {
    // SAFETY: A task css has a root-terminated chain under the caller's pin.
    unsafe {
        let mut ca = task_ca(tsk);
        while ca != lupos_cpuacct_root() {
            lupos_cpuacct_this_cpu_add(ca, index, val);
            ca = parent_ca(ca);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
