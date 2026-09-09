// SPDX-License-Identifier: GPL-2.0

/*
 * CPU accounting code for task groups.
 *
 * Based on the work by Paul Menage (menage@google.com) and Balbir Singh
 * (balbir@in.ibm.com).
 */
// C dependencies: <linux/sched/cputime.h> and "sched.h".

#[repr(C)]
#[derive(Copy, Clone)]
pub enum CpuacctStatIndex {
    CPUACCT_STAT_USER,
    CPUACCT_STAT_SYSTEM,
    CPUACCT_STAT_NSTATS,
}

static CPUACCT_STAT_DESC: [&'static [u8]; 2] = [b"user\0", b"system\0"];

#[repr(C)]
pub struct cpuacct {
    pub css: cgroup_subsys_state,
    pub cpuusage: *mut u64,
    pub cpustat: *mut kernel_cpustat,
}

unsafe fn css_ca(css: *mut cgroup_subsys_state) -> *mut cpuacct {
    if !css.is_null() { container_of!(css, cpuacct, css) } else { core::ptr::null_mut() }
}

unsafe fn task_ca(tsk: *mut task_struct) -> *mut cpuacct {
    css_ca(task_css!(tsk, cpuacct_cgrp_id))
}

unsafe fn parent_ca(ca: *mut cpuacct) -> *mut cpuacct {
    css_ca((*ca).css.parent)
}

static mut ROOT_CPUACCT_CPUUSAGE: u64 = 0;
static mut root_cpuacct: cpuacct = cpuacct {
    css: cgroup_subsys_state::default(),
    cpustat: core::ptr::addr_of_mut!(kernel_cpustat),
    cpuusage: core::ptr::addr_of_mut!(ROOT_CPUACCT_CPUUSAGE),
};

unsafe fn cpuacct_css_alloc(parent_css: *mut cgroup_subsys_state) -> *mut cgroup_subsys_state {
    if parent_css.is_null() { return core::ptr::addr_of_mut!(root_cpuacct.css); }
    let ca = kzalloc_obj::<cpuacct>();
    if ca.is_null() { return err_ptr!(-12); }
    (*ca).cpuusage = alloc_percpu::<u64>();
    if (*ca).cpuusage.is_null() { kfree(ca); return err_ptr!(-12); }
    (*ca).cpustat = alloc_percpu::<kernel_cpustat>();
    if (*ca).cpustat.is_null() {
        free_percpu((*ca).cpuusage);
        kfree(ca);
        return err_ptr!(-12);
    }
    core::ptr::addr_of_mut!((*ca).css)
}

unsafe fn cpuacct_css_free(css: *mut cgroup_subsys_state) {
    let ca = css_ca(css);
    free_percpu((*ca).cpustat);
    free_percpu((*ca).cpuusage);
    kfree(ca);
}

unsafe fn cpuacct_cpuusage_read(ca: *mut cpuacct, cpu: i32, index: CpuacctStatIndex) -> u64 {
    let cpuusage = per_cpu_ptr((*ca).cpuusage, cpu);
    let cpustat = (*per_cpu_ptr((*ca).cpustat, cpu)).cpustat.as_mut_ptr();
    if matches!(index, CpuacctStatIndex::CPUACCT_STAT_NSTATS) || (index as u32) <= 2 {
    } else { return 0; }
    let data = match index {
        CpuacctStatIndex::CPUACCT_STAT_USER => *cpustat.add(CPUTIME_USER) + *cpustat.add(CPUTIME_NICE),
        CpuacctStatIndex::CPUACCT_STAT_SYSTEM => *cpustat.add(CPUTIME_SYSTEM) + *cpustat.add(CPUTIME_IRQ) + *cpustat.add(CPUTIME_SOFTIRQ),
        CpuacctStatIndex::CPUACCT_STAT_NSTATS => *cpuusage,
    };
    data
}

unsafe fn cpuacct_cpuusage_write(ca: *mut cpuacct, cpu: i32) {
    if ca == core::ptr::addr_of_mut!(root_cpuacct) { return; }
    let cpuusage = per_cpu_ptr((*ca).cpuusage, cpu);
    let cpustat = (*per_cpu_ptr((*ca).cpustat, cpu)).cpustat.as_mut_ptr();
    *cpuusage = 0;
    *cpustat.add(CPUTIME_USER) = 0; *cpustat.add(CPUTIME_NICE) = 0;
    *cpustat.add(CPUTIME_SYSTEM) = 0; *cpustat.add(CPUTIME_IRQ) = 0;
    *cpustat.add(CPUTIME_SOFTIRQ) = 0;
}

unsafe fn __cpuusage_read(css: *mut cgroup_subsys_state, index: CpuacctStatIndex) -> u64 {
    let ca = css_ca(css); let mut totalcpuusage = 0; let mut i = 0;
    for_each_possible_cpu!(i) { totalcpuusage += cpuacct_cpuusage_read(ca, i, index); }
    totalcpuusage
}

unsafe fn cpuusage_user_read(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> u64 { __cpuusage_read(css, CpuacctStatIndex::CPUACCT_STAT_USER) }
unsafe fn cpuusage_sys_read(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> u64 { __cpuusage_read(css, CpuacctStatIndex::CPUACCT_STAT_SYSTEM) }
unsafe fn cpuusage_read(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> u64 { __cpuusage_read(css, CpuacctStatIndex::CPUACCT_STAT_NSTATS) }

unsafe fn cpuusage_write(css: *mut cgroup_subsys_state, _cft: *mut cftype, val: u64) -> i32 {
    if val != 0 { return -22; }
    let ca = css_ca(css); let mut cpu = 0;
    for_each_possible_cpu!(cpu) { cpuacct_cpuusage_write(ca, cpu); }
    0
}

unsafe fn __cpuacct_percpu_seq_show(m: *mut seq_file, index: CpuacctStatIndex) -> i32 {
    let ca = css_ca(seq_css!(m)); let mut i = 0;
    for_each_possible_cpu!(i) { seq_printf!(m, "%llu ", cpuacct_cpuusage_read(ca, i, index)); }
    seq_printf!(m, "\n"); 0
}
unsafe fn cpuacct_percpu_user_seq_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { __cpuacct_percpu_seq_show(m, CpuacctStatIndex::CPUACCT_STAT_USER) }
unsafe fn cpuacct_percpu_sys_seq_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { __cpuacct_percpu_seq_show(m, CpuacctStatIndex::CPUACCT_STAT_SYSTEM) }
unsafe fn cpuacct_percpu_seq_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { __cpuacct_percpu_seq_show(m, CpuacctStatIndex::CPUACCT_STAT_NSTATS) }

unsafe fn cpuacct_all_seq_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let ca = css_ca(seq_css!(m));
    seq_puts!(m, "cpu");
    let mut index = 0;
    while index < 2 { seq_printf!(m, " %s", CPUACCT_STAT_DESC[index]); index += 1; }
    seq_puts!(m, "\n");
    let mut cpu = 0;
    for_each_possible_cpu!(cpu) {
        seq_printf!(m, "%d", cpu);
        index = 0;
        while index < 2 { seq_printf!(m, " %llu", cpuacct_cpuusage_read(ca, cpu, core::mem::transmute(index))); index += 1; }
        seq_puts!(m, "\n");
    }
    0
}

unsafe fn cpuacct_stats_show(sf: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let ca = css_ca(seq_css!(sf));
    let mut cputime = task_cputime::default();
    let mut cpu = 0;
    for_each_possible_cpu!(cpu) {
        let cpustat = (*per_cpu_ptr((*ca).cpustat, cpu)).cpustat.as_mut_ptr();
        cputime.utime += *cpustat.add(CPUTIME_USER) + *cpustat.add(CPUTIME_NICE);
        cputime.stime += *cpustat.add(CPUTIME_SYSTEM) + *cpustat.add(CPUTIME_IRQ) + *cpustat.add(CPUTIME_SOFTIRQ);
        cputime.sum_exec_runtime += *per_cpu_ptr((*ca).cpuusage, cpu);
    }
    let mut val = [0u64; 2];
    cputime_adjust!(&mut cputime, &mut (*seq_css!(sf)).cgroup.prev_cputime, &mut val[0], &mut val[1]);
    let mut stat = 0;
    while stat < 2 { seq_printf!(sf, "%s %llu\n", CPUACCT_STAT_DESC[stat], nsec_to_clock_t(val[stat])); stat += 1; }
    0
}

#[repr(C)]
static mut files: [cftype; 9] = [
    cftype { name: b"usage\0".as_ptr(), read_u64: Some(cpuusage_read), write_u64: Some(cpuusage_write), ..cftype::default() },
    cftype { name: b"usage_user\0".as_ptr(), read_u64: Some(cpuusage_user_read), ..cftype::default() },
    cftype { name: b"usage_sys\0".as_ptr(), read_u64: Some(cpuusage_sys_read), ..cftype::default() },
    cftype { name: b"usage_percpu\0".as_ptr(), seq_show: Some(cpuacct_percpu_seq_show), ..cftype::default() },
    cftype { name: b"usage_percpu_user\0".as_ptr(), seq_show: Some(cpuacct_percpu_user_seq_show), ..cftype::default() },
    cftype { name: b"usage_percpu_sys\0".as_ptr(), seq_show: Some(cpuacct_percpu_sys_seq_show), ..cftype::default() },
    cftype { name: b"usage_all\0".as_ptr(), seq_show: Some(cpuacct_all_seq_show), ..cftype::default() },
    cftype { name: b"stat\0".as_ptr(), seq_show: Some(cpuacct_stats_show), ..cftype::default() },
    cftype::default(),
];

pub unsafe fn cpuacct_charge(tsk: *mut task_struct, cputime: u64) {
    let cpu = task_cpu(tsk); let mut ca = task_ca(tsk);
    lockdep_assert_rq_held!(cpu_rq(cpu));
    while !ca.is_null() { *per_cpu_ptr((*ca).cpuusage, cpu) += cputime; ca = parent_ca(ca); }
}

pub unsafe fn cpuacct_account_field(tsk: *mut task_struct, index: i32, val: u64) {
    let mut ca = task_ca(tsk);
    while ca != core::ptr::addr_of_mut!(root_cpuacct) { __this_cpu_add!((*ca).cpustat, index, val); ca = parent_ca(ca); }
}

#[no_mangle]
pub static mut cpuacct_cgrp_subsys: cgroup_subsys = cgroup_subsys {
    css_alloc: Some(cpuacct_css_alloc), css_free: Some(cpuacct_css_free), legacy_cftypes: files.as_ptr(), early_init: true,
    ..cgroup_subsys::default()
};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
