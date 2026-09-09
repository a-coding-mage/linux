// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013, Michael (Ellerman|Neuling), IBM Corporation.
 */

// C kernel dependencies and build-time configuration are supplied externally.

static mut subcores_per_core: i32 = 0;
static mut new_split_mode: i32 = 0;
static mut cpu_offline_mask: cpumask_var_t = core::ptr::null_mut();

#[repr(C)]
struct split_state {
    step: u8,
    master: u8,
}

// DEFINE_PER_CPU(struct split_state, split_state);
extern "C" {
    static mut split_state: split_state;
}

unsafe fn wait_for_sync_step(step: i32) {
    let cpu: i32 = smp_processor_id();
    let mut i = cpu + 1;
    while i < cpu + threads_per_core {
        while per_cpu_split_state(i).step as i32 < step {
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        i += 1;
    }
    mb();
}

unsafe fn update_hid_in_slw(hid0: u64) {
    let idle_states = pnv_get_supported_cpuidle_states();
    if idle_states & OPAL_PM_WINKLE_ENABLED != 0 {
        let cpu_pir = hard_smp_processor_id();
        opal_slw_set_reg(cpu_pir, SPRN_HID0, hid0);
    }
}

#[inline]
unsafe fn update_power8_hid0(hid0: usize) {
    // sync; mtspr SPRN_HID0, hid0; isync
    asm!("sync; mtspr {spr}, {val}; isync", spr = const SPRN_HID0, val = in(reg) hid0);
}

unsafe fn unsplit_core() {
    let mask = HID0_POWER8_2LPARMODE | HID0_POWER8_4LPARMODE;
    let cpu = smp_processor_id();
    if cpu_thread_in_core(cpu) != 0 {
        while mfspr(SPRN_HID0) & mask != 0 {
            power7_idle_type(PNV_THREAD_NAP);
        }
        per_cpu_split_state(cpu).step = SYNC_STEP_UNSPLIT as u8;
        return;
    }
    let mut hid0 = mfspr(SPRN_HID0);
    hid0 &= !HID0_POWER8_DYNLPARDIS;
    update_power8_hid0(hid0 as usize);
    update_hid_in_slw(hid0);
    while mfspr(SPRN_HID0) & mask != 0 { cpu_relax(); }
    let mut i = cpu + 1;
    while i < cpu + threads_per_core {
        smp_send_reschedule(i);
        i += 1;
    }
    wait_for_sync_step(SYNC_STEP_UNSPLIT);
}

unsafe fn split_core(new_mode: i32) {
    let split_parms = [
        (HID0_POWER8_1TO2LPAR, HID0_POWER8_2LPARMODE),
        (HID0_POWER8_1TO4LPAR, HID0_POWER8_4LPARMODE),
    ];
    let i = (new_mode >> 1) - 1;
    BUG_ON(i < 0 || i > 1);
    let cpu = smp_processor_id();
    if cpu_thread_in_core(cpu) != 0 {
        split_core_secondary_loop(&mut per_cpu_split_state(cpu).step);
        return;
    }
    wait_for_sync_step(SYNC_STEP_REAL_MODE);
    let mut hid0 = mfspr(SPRN_HID0);
    hid0 |= HID0_POWER8_DYNLPARDIS | split_parms[i as usize].0;
    update_power8_hid0(hid0 as usize);
    update_hid_in_slw(hid0);
    while mfspr(SPRN_HID0) & split_parms[i as usize].1 == 0 { cpu_relax(); }
}

unsafe fn cpu_do_split(new_mode: i32) {
    if subcores_per_core != 1 { unsplit_core(); }
    if new_mode != 1 { split_core(new_mode); }
    mb();
    per_cpu_split_state(smp_processor_id()).step = SYNC_STEP_FINISHED as u8;
}

#[no_mangle]
pub unsafe extern "C" fn cpu_core_split_required() -> bool {
    smp_rmb();
    if new_split_mode == 0 { return false; }
    cpu_do_split(new_split_mode);
    true
}

pub unsafe fn update_subcore_sibling_mask() {
    let sibling_mask_first_cpu = (1 << threads_per_subcore) - 1;
    for_each_possible_cpu!(cpu, {
        let tid = cpu_thread_in_core(cpu);
        let offset = (tid / threads_per_subcore) * threads_per_subcore;
        let mask = sibling_mask_first_cpu << offset;
        (*paca_ptrs[cpu as usize]).subcore_sibling_mask = mask;
    });
}

unsafe fn cpu_update_split_mode(data: *mut core::ffi::c_void) -> i32 {
    let new_mode = *(data as *mut i32);
    if per_cpu_split_state(smp_processor_id()).master != 0 {
        new_split_mode = new_mode;
        smp_wmb();
        cpumask_andnot(cpu_offline_mask, cpu_present_mask, cpu_online_mask);
        for_each_cpu!(cpu, cpu_offline_mask, { smp_send_reschedule(cpu); });
    }
    cpu_do_split(new_mode);
    if per_cpu_split_state(smp_processor_id()).master != 0 {
        for_each_present_cpu!(cpu, {
            if cpu >= setup_max_cpus { break; }
            while per_cpu_split_state(cpu).step < SYNC_STEP_FINISHED as u8 { compiler_barrier(); }
        });
        new_split_mode = 0;
        subcores_per_core = new_mode;
        threads_per_subcore = threads_per_core / subcores_per_core;
        update_subcore_sibling_mask();
        mb();
    }
    0
}

unsafe fn set_subcores_per_core(new_mode: i32) -> i32 {
    if kvm_hv_mode_active() { pr_err!("Unable to change split core mode while KVM active.\n"); return -EBUSY; }
    BUG_ON(new_mode < 1 || new_mode > 4 || new_mode == 3);
    for_each_present_cpu!(cpu, { per_cpu_split_state(cpu).step = SYNC_STEP_INITIAL as u8; per_cpu_split_state(cpu).master = 0; });
    cpus_read_lock();
    per_cpu_split_state(smp_processor_id()).master = 1;
    mb();
    stop_machine_cpuslocked(cpu_update_split_mode, &new_mode as *const i32 as *mut _, cpu_online_mask);
    cpus_read_unlock();
    0
}

unsafe fn store_subcores_per_core(_dev: *mut device, _attr: *mut device_attribute, buf: *const i8, count: usize) -> isize {
    let mut val: usize = 0;
    if sscanf(buf, "%lx", &mut val) != 1 { return -EINVAL as isize; }
    match val {
        1 | 2 | 4 => if subcores_per_core == val as i32 { return count as isize; },
        _ => return -EINVAL as isize,
    }
    let rc = set_subcores_per_core(val as i32);
    if rc != 0 { return rc as isize; }
    count as isize
}

unsafe fn show_subcores_per_core(_dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    sysfs_emit(buf, "%x\n", subcores_per_core) as isize
}

// DEVICE_ATTR(subcores_per_core, 0644, show_subcores_per_core, store_subcores_per_core);

unsafe fn subcore_init() -> i32 {
    let mut rc = 0;
    let pvr_ver = PVR_VER(mfspr(SPRN_PVR));
    if pvr_ver != PVR_POWER8 && pvr_ver != PVR_POWER8E &&
       pvr_ver != PVR_POWER8NVL && pvr_ver != PVR_HX_C2000 { return 0; }
    if setup_max_cpus % threads_per_core != 0 { return 0; }
    BUG_ON(!alloc_cpumask_var(&mut cpu_offline_mask, GFP_KERNEL));
    set_subcores_per_core(1);
    let dev_root = bus_get_dev_root(&cpu_subsys);
    if !dev_root.is_null() {
        rc = device_create_file(dev_root, &dev_attr_subcores_per_core);
        put_device(dev_root);
    }
    rc
}

// machine_device_initcall(powernv, subcore_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
