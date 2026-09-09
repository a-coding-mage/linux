/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependencies: linux/jump_label.h, linux/percpu.h,
 * linux/resctrl_types.h, linux/sched.h, and asm/msr.h. */

#[cfg(CONFIG_X86_CPU_RESCTRL)]
pub const X86_RESCTRL_EMPTY_CLOSID: u32 = !0u32;

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[repr(C)]
pub struct resctrl_pqr_state {
    pub cur_rmid: u32,
    pub cur_closid: u32,
    pub default_rmid: u32,
    pub default_closid: u32,
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
extern "C" {
    pub static mut rdt_alloc_capable: bool;
    pub static mut rdt_mon_capable: bool;
    pub static mut pqr_state: resctrl_pqr_state;
    pub static mut rdt_enable_key: static_key;
    pub static mut rdt_alloc_enable_key: static_key;
    pub static mut rdt_mon_enable_key: static_key;
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
extern "C" {
    pub fn static_branch_enable_cpuslocked(key: *mut static_key);
    pub fn static_branch_inc_cpuslocked(key: *mut static_key);
    pub fn static_branch_disable_cpuslocked(key: *mut static_key);
    pub fn static_branch_dec_cpuslocked(key: *mut static_key);
    pub fn static_branch_likely(key: *const static_key) -> bool;
    pub fn this_cpu_ptr<T>(ptr: *mut T) -> *mut T;
    pub fn might_sleep();
    pub fn wrmsrq(msr: u32, val: u64);
    pub fn resctrl_cpu_detect(c: *mut cpuinfo_x86);
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_alloc_capable() -> bool {
    rdt_alloc_capable
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_enable_alloc() {
    static_branch_enable_cpuslocked(&mut rdt_alloc_enable_key);
    static_branch_inc_cpuslocked(&mut rdt_enable_key);
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_disable_alloc() {
    static_branch_disable_cpuslocked(&mut rdt_alloc_enable_key);
    static_branch_dec_cpuslocked(&mut rdt_enable_key);
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_mon_capable() -> bool {
    rdt_mon_capable
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_enable_mon() {
    static_branch_enable_cpuslocked(&mut rdt_mon_enable_key);
    static_branch_inc_cpuslocked(&mut rdt_enable_key);
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_disable_mon() {
    static_branch_disable_cpuslocked(&mut rdt_mon_enable_key);
    static_branch_dec_cpuslocked(&mut rdt_enable_key);
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn __resctrl_sched_in(tsk: *mut task_struct) {
    let state = &mut pqr_state;
    let mut closid = core::ptr::read_volatile(&state.default_closid);
    let mut rmid = core::ptr::read_volatile(&state.default_rmid);
    let mut tmp: u32;

    if static_branch_likely(&rdt_alloc_enable_key) {
        tmp = core::ptr::read_volatile(&(*tsk).closid);
        if tmp != 0 { closid = tmp; }
    }
    if static_branch_likely(&rdt_mon_enable_key) {
        tmp = core::ptr::read_volatile(&(*tsk).rmid);
        if tmp != 0 { rmid = tmp; }
    }
    if closid != state.cur_closid || rmid != state.cur_rmid {
        state.cur_closid = closid;
        state.cur_rmid = rmid;
        let val = (closid as u64) << 32 | rmid as u64;
        wrmsrq(MSR_IA32_PQR_ASSOC, val);
    }
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_round_mon_val(mut val: u32) -> u32 {
    let scale = boot_cpu_data.x86_cache_occ_scale;
    val /= scale;
    val * scale
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_set_cpu_default_closid_rmid(_cpu: i32, closid: u32, rmid: u32) {
    pqr_state.default_closid = closid;
    pqr_state.default_rmid = rmid;
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_set_closid_rmid(tsk: *mut task_struct, closid: u32, rmid: u32) {
    (*tsk).closid = closid;
    (*tsk).rmid = rmid;
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_match_closid(tsk: *mut task_struct, closid: u32) -> bool {
    core::ptr::read_volatile(&(*tsk).closid) == closid
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_match_rmid(tsk: *mut task_struct, _ignored: u32, rmid: u32) -> bool {
    core::ptr::read_volatile(&(*tsk).rmid) == rmid
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_sched_in(tsk: *mut task_struct) {
    if static_branch_likely(&rdt_enable_key) { __resctrl_sched_in(tsk); }
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_rmid_idx_decode(idx: u32, closid: *mut u32, rmid: *mut u32) {
    *rmid = idx;
    *closid = X86_RESCTRL_EMPTY_CLOSID;
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_rmid_idx_encode(_ignored: u32, rmid: u32) -> u32 { rmid }

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_mon_ctx_alloc(_r: *mut rdt_resource, _evtid: resctrl_event_id) -> *mut core::ffi::c_void {
    might_sleep();
    core::ptr::null_mut()
}

#[cfg(CONFIG_X86_CPU_RESCTRL)]
#[inline]
pub unsafe fn resctrl_arch_mon_ctx_free(_r: *mut rdt_resource, _evtid: resctrl_event_id, _ctx: *mut core::ffi::c_void) {}

#[cfg(not(CONFIG_X86_CPU_RESCTRL))]
#[inline]
pub unsafe fn resctrl_arch_sched_in(_tsk: *mut task_struct) {}

#[cfg(not(CONFIG_X86_CPU_RESCTRL))]
#[inline]
pub unsafe fn resctrl_cpu_detect(_c: *mut cpuinfo_x86) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
