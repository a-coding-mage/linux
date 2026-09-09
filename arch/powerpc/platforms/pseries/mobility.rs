// SPDX-License-Identifier: GPL-2.0-only
/* Support for Partition Mobility/Migration */

// Kernel dependencies and configuration conditions are supplied by the surrounding tree.

#[repr(C, packed)]
struct UpdatePropsWorkarea { phandle: u32, state: u32, reserved: u64, nprops: u32 }

const NODE_ACTION_MASK: u32 = 0xff000000;
const NODE_COUNT_MASK: u32 = 0x00ffffff;
const DELETE_DT_NODE: u32 = 0x01000000;
const UPDATE_DT_NODE: u32 = 0x02000000;
const ADD_DT_NODE: u32 = 0x03000000;
const MIGRATION_SCOPE: i32 = 1;
const PRRN_SCOPE: i32 = -2;
const MIGRATION_API_VERSION: u32 = 1;

static mut MOBILITY_KOBJ: *mut Kobject = core::ptr::null_mut();

#[cfg(feature = "CONFIG_PPC_WATCHDOG")]
static mut NMI_WD_LPM_FACTOR: u32 = 200;

unsafe fn mobility_rtas_call(token: i32, buf: *mut i8, scope: i32) -> i32 {
    spin_lock(&raw mut RTAS_DATA_BUF_LOCK);
    memcpy(RTAS_DATA_BUF, buf, RTAS_DATA_BUF_SIZE);
    let rc = rtas_call(token, 2, 1, core::ptr::null_mut(), RTAS_DATA_BUF, scope);
    memcpy(buf, RTAS_DATA_BUF, RTAS_DATA_BUF_SIZE);
    spin_unlock(&raw mut RTAS_DATA_BUF_LOCK);
    rc
}

unsafe fn delete_dt_node(dn: *mut DeviceNode) -> i32 {
    let pdn = of_get_parent(dn);
    let is_platfac = of_node_is_type(dn, c"ibm,platform-facilities".as_ptr()) ||
                     of_node_is_type(pdn, c"ibm,platform-facilities".as_ptr());
    of_node_put(pdn);
    if is_platfac { pr_notice!("ignoring remove operation for %pOFfp\n", dn); return 0; }
    pr_debug!("removing node %pOFfp\n", dn);
    dlpar_detach_node(dn); 0
}

unsafe fn update_dt_property(dn: *mut DeviceNode, prop: *mut *mut Property,
                             name: *const i8, mut vd: u32, value: *mut i8) -> i32 {
    let mut new_prop = *prop;
    let mut more = false;
    if vd & 0x80000000 != 0 { vd = (!vd).wrapping_add(1); more = true; }
    if !new_prop.is_null() {
        let new_data = kzalloc((*(*new_prop).length as usize + vd as usize), GFP_KERNEL);
        if new_data.is_null() { return -ENOMEM; }
        memcpy(new_data, (*new_prop).value, (*new_prop).length as usize);
        memcpy(new_data.add((*new_prop).length as usize), value, vd as usize);
        kfree((*new_prop).value); (*new_prop).value = new_data;
        (*new_prop).length += vd;
    } else {
        new_prop = kzalloc_obj::<Property>();
        if new_prop.is_null() { return -ENOMEM; }
        (*new_prop).name = kstrdup(name, GFP_KERNEL);
        if (*new_prop).name.is_null() { kfree(new_prop); return -ENOMEM; }
        (*new_prop).length = vd;
        (*new_prop).value = kzalloc(vd as usize, GFP_KERNEL);
        if (*new_prop).value.is_null() { kfree((*new_prop).name); kfree(new_prop); return -ENOMEM; }
        memcpy((*new_prop).value, value, vd as usize); *prop = new_prop;
    }
    if !more { pr_debug!("updating node %pOF property %s\n", dn, name); of_update_property(dn, new_prop); *prop = core::ptr::null_mut(); }
    0
}

unsafe fn update_dt_node(dn: *mut DeviceNode, scope: i32) -> i32 {
    let token = rtas_function_token(RTAS_FN_IBM_UPDATE_PROPERTIES);
    if token == RTAS_UNKNOWN_SERVICE { return -EINVAL; }
    let buf = kzalloc(RTAS_DATA_BUF_SIZE, GFP_KERNEL);
    if buf.is_null() { return -ENOMEM; }
    let upwa = buf as *mut UpdatePropsWorkarea; (*upwa).phandle = cpu_to_be32((*dn).phandle);
    let mut prop: *mut Property = core::ptr::null_mut();
    let mut rtas_rc;
    loop {
        rtas_rc = mobility_rtas_call(token, buf, scope); if rtas_rc < 0 { break; }
        let mut p = buf.add(core::mem::size_of::<UpdatePropsWorkarea>());
        let mut nprops = be32_to_cpu((*upwa).nprops);
        if *p == 0 { p = p.add(1); let vd = be32_to_cpu(*(p as *mut u32)); p = p.add(vd as usize + 4); nprops -= 1; }
        for _ in 0..nprops {
            let prop_name = p; p = p.add(strlen(prop_name) + 1); let vd = be32_to_cpu(*(p as *mut u32)); p = p.add(4);
            match vd { 0 => (), 0x80000000 => { of_remove_property(dn, of_find_property(dn, prop_name, core::ptr::null_mut())); prop = core::ptr::null_mut(); }, _ => { let rc = update_dt_property(dn, &mut prop, prop_name, vd, p); if rc != 0 { pr_err!("updating %s property failed: %d\n", prop_name, rc); } p = p.add(vd as usize); } }
            cond_resched();
        }
        cond_resched(); if rtas_rc != 1 { break; }
    }
    kfree(buf); 0
}

unsafe fn add_dt_node(parent_dn: *mut DeviceNode, drc_index: u32) -> i32 {
    let dn = dlpar_configure_connector(drc_index, parent_dn); if dn.is_null() { return -ENOENT; }
    if of_node_is_type(dn, c"ibm,platform-facilities".as_ptr()) { pr_notice!("ignoring add operation for %pOF\n", dn); dlpar_free_cc_nodes(dn); return 0; }
    let rc = dlpar_attach_node(dn, parent_dn); if rc != 0 { dlpar_free_cc_nodes(dn); }
    pr_debug!("added node %pOFfp\n", dn); rc
}

unsafe fn pseries_devicetree_update(scope: i32) -> i32 {
    let token = rtas_function_token(RTAS_FN_IBM_UPDATE_NODES); if token == RTAS_UNKNOWN_SERVICE { return 0; }
    let buf = kzalloc(RTAS_DATA_BUF_SIZE, GFP_KERNEL); if buf.is_null() { return -ENOMEM; }
    let mut rc;
    loop {
        rc = mobility_rtas_call(token, buf, scope); if rc != 0 && rc != 1 { break; }
        let mut data = (buf as *mut u32).add(4);
        while be32_to_cpu(*data) & NODE_ACTION_MASK != 0 {
            let action = be32_to_cpu(*data) & NODE_ACTION_MASK; let count = be32_to_cpu(*data) & NODE_COUNT_MASK; data = data.add(1);
            for _ in 0..count { let np = of_find_node_by_phandle(be32_to_cpu(*data)); data = data.add(1); if np.is_null() { pr_warn!("Failed lookup: phandle for action\n"); continue; }
                match action { DELETE_DT_NODE => { delete_dt_node(np); }, UPDATE_DT_NODE => { update_dt_node(np, scope); }, ADD_DT_NODE => { let idx = *data; data = data.add(1); add_dt_node(np, idx); }, _ => () }
                of_node_put(np); cond_resched();
            }
        }
        cond_resched(); if rc != 1 { break; }
    }
    kfree(buf); rc
}

pub unsafe fn post_mobility_fixup() { rtas_activate_firmware(); cpus_read_lock(); cacheinfo_teardown(); let rc = pseries_devicetree_update(MIGRATION_SCOPE); if rc != 0 { pr_err!("device tree update failed: %d\n", rc); } cacheinfo_rebuild(); cpus_read_unlock(); pseries_setup_security_mitigations(); read_24x7_sys_info(); }

unsafe fn poll_vasi_state(handle: u64, res: *mut usize) -> i32 { let mut b = [0usize; PLPAR_HCALL_BUFSIZE]; match plpar_hcall(H_VASI_STATE, b.as_mut_ptr(), handle) { H_SUCCESS => { *res = b[0]; 0 }, H_PARAMETER => -EINVAL, H_FUNCTION => -EOPNOTSUPP, x => { pr_err!("unexpected H_VASI_STATE result %ld\n", x); -EIO } } }

unsafe fn wait_for_vasi_session_suspending(handle: u64) -> i32 { loop { let mut s=0; let r=poll_vasi_state(handle,&mut s); if r!=0 || s==H_VASI_SUSPENDING { return if r==-EOPNOTSUPP {0}else{r}; } if s==H_VASI_ENABLED { ssleep(1); } else { pr_err!("unexpected H_VASI_STATE result %lu\n",s); return -EIO; } } }
unsafe fn wait_for_vasi_session_completed(handle: u64) { loop { let mut s=0; let r=poll_vasi_state(handle,&mut s); if r==-EINVAL || (r==0 && s==H_VASI_COMPLETED) { pr_info!("memory transfer completed.\n"); break; } if r!=0 || s!=H_VASI_RESUMED { break; } msleep(500); } }
unsafe fn prod_single(cpu: u32) { let hwid=get_hard_smp_processor_id(cpu); let r=plpar_hcall_norets(H_PROD,hwid); if r!=H_SUCCESS { pr_err_ratelimited!("H_PROD of CPU %u (hwid %d) error: %ld\n",cpu,hwid,r); } }
unsafe fn prod_others() { for_each_online_cpu!(cpu => { if cpu != smp_processor_id() { prod_single(cpu); } }); }

#[repr(C)] struct PseriesSuspendInfo { counter: AtomicT, done: bool }
unsafe fn do_suspend() -> i32 { let saved=clamp_slb_size(); let mut status=0; let r=rtas_ibm_suspend_me(&mut status); if r!=0 { pr_err!("ibm,suspend-me error: %d\n",status); slb_set_size(saved); } r }
unsafe fn clamp_slb_size() -> u16 { #[cfg(feature="CONFIG_PPC_64S_HASH_MMU")] { let p=MMU_SLB_SIZE; slb_set_size(SLB_MIN_SIZE); p } #[cfg(not(feature="CONFIG_PPC_64S_HASH_MMU"))] { 0 } }
unsafe fn do_join(arg: *mut core::ffi::c_void) -> i32 { let info=arg as *mut PseriesSuspendInfo; loop { hard_irq_disable(); match plpar_hcall_norets(H_JOIN) { H_CONTINUE => break, H_SUCCESS => { smp_mb(); if !READ_ONCE!((*info).done) { continue; } return 0; }, _ => return -EIO } } let r=do_suspend(); if atomic_inc_return(&mut (*info).counter)==1 { WRITE_ONCE!((*info).done,true); smp_mb(); prod_others(); } r }

#[repr(u32)] enum VasiAbortingEntity { ORCHESTRATOR=1, VSP_SOURCE, PARTITION_FIRMWARE, PLATFORM_FIRMWARE, VSP_TARGET, MIGRATING_PARTITION }
unsafe fn pseries_cancel_migration(handle:u64, err:i32) { let reason=((MIGRATING_PARTITION as u32)<<24)|(err.unsigned_abs()&0xffffff); let r=plpar_hcall_norets(H_VASI_SIGNAL,handle,H_VASI_SIGNAL_CANCEL,reason); if r!=0 { pr_err!("H_VASI_SIGNAL error: %ld\n",r); } }
unsafe fn pseries_suspend(handle:u64)->i32 { let mut interval=1; let mut attempt=1; let mut ret; loop { let mut info=PseriesSuspendInfo{counter:ATOMIC_INIT(0),done:false}; ret=stop_machine(do_join,&mut info as *mut _ as *mut _,cpu_online_mask); if ret==0 || attempt==5 { break; } let mut state=0; let e=poll_vasi_state(handle,&mut state); if (e==0 && state!=H_VASI_SUSPENDING)||(e!=0&&e!=-EOPNOTSUPP){ break; } msleep(interval); interval*=10; attempt+=1; } ret }
unsafe fn pseries_migrate_partition(handle:u64)->i32 { let mut factor=0; #[cfg(feature="CONFIG_PPC_WATCHDOG")] { factor=NMI_WD_LPM_FACTOR; } vas_migration_handler(VAS_SUSPEND); hvpipe_migration_handler(HVPIPE_SUSPEND); let mut ret=wait_for_vasi_session_suspending(handle); if ret==0 { if factor!=0 { watchdog_hardlockup_set_timeout_pct(factor); } ret=pseries_suspend(handle); if ret==0 { post_mobility_fixup(); wait_for_vasi_session_completed(handle); } else { pseries_cancel_migration(handle,ret); } if factor!=0 { watchdog_hardlockup_set_timeout_pct(0); } } vas_migration_handler(VAS_RESUME); hvpipe_migration_handler(HVPIPE_RESUME); ret }
pub unsafe fn rtas_syscall_dispatch_ibm_suspend_me(handle:u64)->i32 { pseries_migrate_partition(handle) }

unsafe fn migration_store(_class: *const Class, _attr: *const ClassAttribute, buf: *const i8, count: usize) -> isize {
    let mut streamid=0u64; let rc=kstrtou64(buf,0,&mut streamid); if rc!=0 { return rc as isize; }
    let rc=pseries_migrate_partition(streamid); if rc!=0 { return rc as isize; } count as isize
}

// CLASS_ATTR_WO(migration) and CLASS_ATTR_STRING(api_version, 0444, "1")
unsafe fn mobility_sysfs_init() -> i32 {
    MOBILITY_KOBJ=kobject_create_and_add(c"mobility".as_ptr(),kernel_kobj); if MOBILITY_KOBJ.is_null(){return -ENOMEM;}
    let mut rc=sysfs_create_file(MOBILITY_KOBJ,&class_attr_migration.attr); if rc!=0 {pr_err!("unable to create migration sysfs file (%d)\n",rc);}
    rc=sysfs_create_file(MOBILITY_KOBJ,&class_attr_api_version.attr.attr); if rc!=0 {pr_err!("unable to create api_version sysfs file (%d)\n",rc);} 0
}

// machine_device_initcall(pseries, mobility_sysfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
