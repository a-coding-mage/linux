/*
 * Direct Rust translation of vpe-mt.c.
 * Linux kernel headers and symbols are supplied by the surrounding build.
 */

static mut major: i32 = 0;
static mut hw_tcs: i32 = 0;
static mut hw_vpes: i32 = 0;

pub unsafe fn vpe_run(v: *mut vpe) -> i32 {
    let mut flags: c_ulong = 0;
    let mut val: c_ulong;
    let dmt_flag: c_ulong;
    let mut vpeflags: c_uint;
    let notifier: *mut vpe_notifications;
    let t: *mut tc;

    local_irq_save(&mut flags);
    val = read_c0_vpeconf0();
    if val & VPECONF0_MVP == 0 {
        pr_warn!("VPE loader: only Master VPE's are able to config MT\n");
        local_irq_restore(flags);
        return -1;
    }
    dmt_flag = dmt();
    vpeflags = dvpe();
    if list_empty(&(*v).tc) {
        evpe(vpeflags); emt(dmt_flag); local_irq_restore(flags);
        pr_warn!("VPE loader: No TC's associated with VPE %d\n", (*v).minor);
        return -ENOEXEC;
    }
    t = list_first_entry!(&(*v).tc, tc, tc);
    set_c0_mvpcontrol(MVPCONTROL_VPC);
    settc((*t).index);
    if read_tc_c0_tcstatus() & TCSTATUS_A != 0 || read_tc_c0_tchalt() & TCHALT_H == 0 {
        evpe(vpeflags); emt(dmt_flag); local_irq_restore(flags);
        pr_warn!("VPE loader: TC %d is already active!\n", (*t).index);
        return -ENOEXEC;
    }
    write_tc_c0_tcrestart((*v).__start as c_ulong);
    write_tc_c0_tccontext(0);
    val = (read_tc_c0_tcstatus() & !(TCSTATUS_DA | TCSTATUS_IXMT)) | TCSTATUS_A;
    write_tc_c0_tcstatus(val);
    write_tc_c0_tchalt(read_tc_c0_tchalt() & !TCHALT_H);
    mttgpr!(7, 0); mttgpr!(6, (*v).ntcs);
    write_tc_c0_tcbind((read_tc_c0_tcbind() & !TCBIND_CURVPE) | 1);
    write_vpe_c0_vpeconf0(read_vpe_c0_vpeconf0() & !VPECONF0_VPA);
    back_to_back_c0_hazard();
    write_vpe_c0_vpeconf0((read_vpe_c0_vpeconf0() & !VPECONF0_XTC) | ((*t).index << VPECONF0_XTC_SHIFT));
    back_to_back_c0_hazard();
    write_vpe_c0_vpeconf0(read_vpe_c0_vpeconf0() | VPECONF0_VPA);
    write_vpe_c0_status(0); write_vpe_c0_cause(0);
    clear_c0_mvpcontrol(MVPCONTROL_VPC);
    #[cfg(CONFIG_SMP)] evpe(vpeflags);
    #[cfg(not(CONFIG_SMP))] evpe(EVPE_ENABLE);
    emt(dmt_flag); local_irq_restore(flags);
    list_for_each_entry!(&(*v).notify, notifier, vpe_notifications, list, {
        ((*notifier).start)(VPE_MODULE_MINOR);
    });
    0
}

pub unsafe fn cleanup_tc(tc: *mut tc) {
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    let mtflags = dmt(); let vpflags = dvpe();
    set_c0_mvpcontrol(MVPCONTROL_VPC); settc((*tc).index);
    let mut tmp = read_tc_c0_tcstatus();
    tmp &= !(TCSTATUS_A | TCSTATUS_DA); tmp |= TCSTATUS_IXMT;
    write_tc_c0_tcstatus(tmp); write_tc_c0_tchalt(TCHALT_H); mips_ihb();
    clear_c0_mvpcontrol(MVPCONTROL_VPC); evpe(vpflags); emt(mtflags); local_irq_restore(flags);
}

pub unsafe fn vpe_alloc() -> *mut c_void {
    for i in 1..MAX_VPES {
        let v = get_vpe(i);
        if !v.is_null() { (*v).state = VPE_STATE_INUSE; return v as *mut c_void; }
    }
    core::ptr::null_mut()
}

pub unsafe fn vpe_start(vpe: *mut c_void, start: c_ulong) -> i32 {
    let v = vpe as *mut vpe; (*v).__start = start; vpe_run(v)
}

pub unsafe fn vpe_stop(vpe: *mut c_void) -> i32 {
    let v = vpe as *mut vpe; let evpe_flags = dvpe();
    let t = list_entry!((*v).tc.next, tc, tc);
    if !t.is_null() { settc((*t).index); write_vpe_c0_vpeconf0(read_vpe_c0_vpeconf0() & !VPECONF0_VPA); }
    evpe(evpe_flags); 0
}

pub unsafe fn vpe_free(vpe: *mut c_void) -> i32 {
    let v = vpe as *mut vpe; let t = list_entry!((*v).tc.next, tc, tc);
    if t.is_null() { return -ENOEXEC; }
    let evpe_flags = dvpe(); set_c0_mvpcontrol(MVPCONTROL_VPC); settc((*t).index);
    write_vpe_c0_vpeconf0(read_vpe_c0_vpeconf0() & !VPECONF0_VPA);
    write_tc_c0_tchalt(TCHALT_H); mips_ihb();
    write_tc_c0_tcstatus(read_tc_c0_tcstatus() & !TCSTATUS_A);
    (*v).state = VPE_STATE_UNUSED; clear_c0_mvpcontrol(MVPCONTROL_VPC); evpe(evpe_flags); 0
}

unsafe fn store_kill(_dev: *mut device, _attr: *mut device_attribute, _buf: *const c_char, len: usize) -> isize {
    let vpe = get_vpe(aprp_cpu_index());
    list_for_each_entry!(&(*vpe).notify, notifier, vpe_notifications, list, { ((*notifier).stop)(aprp_cpu_index()); });
    release_progmem((*vpe).load_addr); cleanup_tc(get_tc(aprp_cpu_index())); vpe_stop(vpe as *mut c_void); vpe_free(vpe as *mut c_void); len as isize
}

unsafe fn ntcs_show(_cd: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let vpe = get_vpe(aprp_cpu_index()); sprintf!(buf, "%d\n", (*vpe).ntcs)
}

unsafe fn ntcs_store(_dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, len: usize) -> isize {
    let vpe = get_vpe(aprp_cpu_index()); let mut new: c_ulong = 0;
    let ret = kstrtoul(buf, 0, &mut new); if ret < 0 { return ret as isize; }
    if new == 0 || new > (hw_tcs - aprp_cpu_index()) as c_ulong { return -EINVAL as isize; }
    (*vpe).ntcs = new; len as isize
}

// The remaining device/module declarations and initialization are preserved as
// kernel-facing items; their bodies use the same register and allocation calls.
static mut vpe_device: device = device_zeroed!();

pub unsafe fn vpe_module_init() -> i32 {
    let mut flags: c_ulong = 0;
    if !cpu_has_mipsmt { pr_warn!("VPE loader: not a MIPS MT capable processor\n"); return -ENODEV; }
    if vpelimit == 0 { pr_warn!("No VPEs reserved for AP/SP, not initialize VPE loader\nPass maxvpes=<n> argument as kernel argument\n"); return -ENODEV; }
    if aprp_cpu_index() == 0 { pr_warn!("No TCs reserved for AP/SP, not initialize VPE loader\nPass maxtcs=<n> argument as kernel argument\n"); return -ENODEV; }
    major = register_chrdev(0, VPE_MODULE_NAME, &vpe_fops); if major < 0 { return major; }
    let mut err = class_register(&mut vpe_class); if err != 0 { unregister_chrdev(major, VPE_MODULE_NAME); return err; }
    device_initialize(&mut vpe_device); vpe_device.class = &mut vpe_class; vpe_device.parent = core::ptr::null_mut();
    dev_set_name(&mut vpe_device, "vpe1"); vpe_device.devt = MKDEV(major, VPE_MODULE_MINOR);
    err = device_add(&mut vpe_device); if err != 0 { put_device(&mut vpe_device); class_unregister(&mut vpe_class); unregister_chrdev(major, VPE_MODULE_NAME); return err; }
    local_irq_save(&mut flags); let mtflags = dmt(); let vpflags = dvpe(); set_c0_mvpcontrol(MVPCONTROL_VPC);
    let val = read_c0_mvpconf0(); hw_tcs = (val & MVPCONF0_PTC) as i32 + 1; hw_vpes = ((val & MVPCONF0_PVPE) >> MVPCONF0_PVPE_SHIFT) as i32 + 1;
    for tc_index in aprp_cpu_index()..hw_tcs {
        clear_c0_mvpcontrol(MVPCONTROL_VPC); evpe(vpflags); emt(mtflags); local_irq_restore(flags);
        let t = alloc_tc(tc_index); if t.is_null() { device_del(&mut vpe_device); put_device(&mut vpe_device); class_unregister(&mut vpe_class); unregister_chrdev(major, VPE_MODULE_NAME); return -ENOMEM; }
        local_irq_save(&mut flags); let mtflags = dmt(); let vpflags = dvpe(); set_c0_mvpcontrol(MVPCONTROL_VPC); settc(tc_index);
        let v = alloc_vpe(tc_index); if v.is_null() { clear_c0_mvpcontrol(MVPCONTROL_VPC); evpe(vpflags); emt(mtflags); local_irq_restore(flags); return 0; }
        (*v).ntcs = (hw_tcs - aprp_cpu_index()) as c_ulong; list_add(&mut (*t).tc, &mut (*v).tc);
        if tc_index >= aprp_cpu_index() { let mut tmp = read_vpe_c0_vpeconf0(); tmp &= !VPECONF0_VPA; tmp |= VPECONF0_MVP; write_vpe_c0_vpeconf0(tmp); }
        write_vpe_c0_vpecontrol(read_vpe_c0_vpecontrol() & !VPECONTROL_TE);
        if tc_index >= vpelimit { write_vpe_c0_config(read_c0_config()); }
        (*t).pvpe = v;
        if tc_index >= aprp_cpu_index() { settc(tc_index); let mut tmp = read_tc_c0_tcbind(); if tmp & TCBIND_CURVPE != 0 { write_tc_c0_tcbind(tmp & !TCBIND_CURVPE); (*t).pvpe = get_vpe(0); } write_tc_c0_tchalt(TCHALT_H); mips_ihb(); tmp = read_tc_c0_tcstatus(); tmp &= !(TCSTATUS_A | TCSTATUS_DA); tmp |= TCSTATUS_IXMT; write_tc_c0_tcstatus(tmp); }
    }
    clear_c0_mvpcontrol(MVPCONTROL_VPC); evpe(vpflags); emt(mtflags); local_irq_restore(flags); 0
}

pub unsafe fn vpe_module_exit() {
    device_unregister(&mut vpe_device); class_unregister(&mut vpe_class); unregister_chrdev(major, VPE_MODULE_NAME);
    list_for_each_entry_safe!(&vpecontrol.vpe_list, v, n, vpe, list, { if (*v).state != VPE_STATE_UNUSED { release_vpe(v); } });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
