// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2024 Intel Corporation */

// Kernel and driver headers from the C translation unit are intentionally
// represented by external symbols supplied by the surrounding crate.

const ADF_GEN4_VF_MSTATE_SIZE: usize = 4096;
const ADF_GEN4_PFVF_RSP_TIMEOUT_US: u64 = 5000;

unsafe fn adf_gen4_vfmig_init_device(mdev: *mut qat_mig_dev) -> i32 {
    let state = kmalloc(ADF_GEN4_VF_MSTATE_SIZE, GFP_KERNEL);
    if state.is_null() { return -ENOMEM; }
    (*mdev).state = state as *mut u8;
    (*mdev).state_size = ADF_GEN4_VF_MSTATE_SIZE;
    (*mdev).setup_size = 0;
    (*mdev).remote_setup_size = 0;
    0
}

unsafe fn adf_gen4_vfmig_cleanup_device(mdev: *mut qat_mig_dev) {
    kfree((*mdev).state as *mut core::ffi::c_void);
    (*mdev).state = core::ptr::null_mut();
}

unsafe fn adf_gen4_vfmig_reset_device(mdev: *mut qat_mig_dev) {
    (*mdev).setup_size = 0;
    (*mdev).remote_setup_size = 0;
}

unsafe fn adf_gen4_vfmig_open_device(mdev: *mut qat_mig_dev) -> i32 {
    let accel_dev = (*mdev).parent_accel_dev;
    let vf_info = &mut (*accel_dev).pf.vf_info[(*mdev).vf_id as usize];
    let vfmig = kzalloc_obj::<adf_gen4_vfmig>();
    if vfmig.is_null() { return -ENOMEM; }
    (*vfmig).mstate_mgr = adf_mstate_mgr_new((*mdev).state, (*mdev).state_size);
    if (*vfmig).mstate_mgr.is_null() {
        kfree(vfmig as *mut core::ffi::c_void);
        return -ENOMEM;
    }
    vf_info.mig_priv = vfmig as *mut _;
    (*mdev).setup_size = 0;
    (*mdev).remote_setup_size = 0;
    0
}

unsafe fn adf_gen4_vfmig_close_device(mdev: *mut qat_mig_dev) {
    let accel_dev = (*mdev).parent_accel_dev;
    let vf_info = &mut (*accel_dev).pf.vf_info[(*mdev).vf_id as usize];
    if !vf_info.mig_priv.is_null() {
        let vfmig = vf_info.mig_priv as *mut adf_gen4_vfmig;
        adf_mstate_mgr_destroy((*vfmig).mstate_mgr);
        kfree(vfmig as *mut core::ffi::c_void);
        vf_info.mig_priv = core::ptr::null_mut();
    }
}

unsafe fn adf_gen4_vfmig_suspend_device(mdev: *mut qat_mig_dev) -> i32 {
    let accel_dev = (*mdev).parent_accel_dev;
    let hw_data = (*accel_dev).hw_device;
    let vf_nr = (*mdev).vf_id;
    let vf_mig = (*accel_dev).pf.vf_info[vf_nr as usize].mig_priv as *mut adf_gen4_vfmig;
    for i in 0..(*hw_data).num_banks_per_vf {
        let pf_bank_nr = i + vf_nr * (*hw_data).num_banks_per_vf;
        let ret = adf_gen4_bank_drain_start(accel_dev, pf_bank_nr, ADF_RPRESET_POLL_TIMEOUT_US);
        if ret != 0 {
            dev_err(GET_DEV(accel_dev), "Failed to drain bank %d for vf_nr %d\n", i, vf_nr);
            return ret;
        }
        (*vf_mig).bank_stopped[i as usize] = true;
        adf_gen4_bank_quiesce_coal_timer(accel_dev, pf_bank_nr, ADF_COALESCED_POLL_TIMEOUT_US);
    }
    0
}

unsafe fn adf_gen4_vfmig_resume_device(mdev: *mut qat_mig_dev) -> i32 {
    let accel_dev = (*mdev).parent_accel_dev;
    let hw_data = (*accel_dev).hw_device;
    let vf_nr = (*mdev).vf_id;
    let vf_mig = (*accel_dev).pf.vf_info[vf_nr as usize].mig_priv as *mut adf_gen4_vfmig;
    for i in 0..(*hw_data).num_banks_per_vf {
        let pf_bank_nr = i + vf_nr * (*hw_data).num_banks_per_vf;
        if (*vf_mig).bank_stopped[i as usize] {
            adf_gen4_bank_drain_finish(accel_dev, pf_bank_nr);
            (*vf_mig).bank_stopped[i as usize] = false;
        }
    }
    0
}

#[repr(C)]
struct adf_vf_bank_info { accel_dev: *mut adf_accel_dev, vf_nr: u32, bank_nr: u32 }
#[repr(C)]
struct mig_user_sla { srv: adf_base_services, rp_mask: u64, cir: u32, pir: u32 }

unsafe fn adf_mstate_sla_check(_: *mut adf_mstate_mgr, src_buf: *mut u8, src_size: u32, opaque: *mut core::ffi::c_void) -> i32 {
    let sinfo = adf_mstate_vreginfo { addr: src_buf, size: src_size };
    let dinfo = opaque as *mut adf_mstate_vreginfo;
    let src_cnt = sinfo.size as usize / core::mem::size_of::<mig_user_sla>();
    let dst_cnt = (*dinfo).size as usize / core::mem::size_of::<mig_user_sla>();
    let src = sinfo.addr as *mut mig_user_sla;
    let dst = (*dinfo).addr as *mut mig_user_sla;
    for i in 0..src_cnt {
        let mut found = false;
        for j in 0..dst_cnt {
            if (*src.add(i)).srv != (*dst.add(j)).srv || (*src.add(i)).rp_mask != (*dst.add(j)).rp_mask { continue; }
            found = true;
            if (*src.add(i)).cir > (*dst.add(j)).cir || (*src.add(i)).pir > (*dst.add(j)).pir {
                pr_err!("QAT: DST VF rate limiting mismatch."); return -EINVAL;
            }
            break;
        }
        if !found { pr_err!("QAT: SRC VF rate limiting mismatch"); return -EINVAL; }
    }
    0
}

#[inline] unsafe fn adf_mstate_check_cap_size(src_sz: u32, dst_sz: u32, max_sz: usize) -> i32 {
    if src_sz as usize > max_sz || dst_sz as usize > max_sz { -EINVAL } else { 0 }
}

unsafe fn adf_mstate_compatver_check(_: *mut adf_mstate_mgr, src_buf: *mut u8, src_sz: u32, opaque: *mut core::ffi::c_void) -> i32 {
    let info = opaque as *mut adf_mstate_vreginfo;
    if src_sz != (*info).size { pr_debug!("QAT: State mismatch (compat version size), current {}, expected {}", src_sz, (*info).size); return -EINVAL; }
    memcpy((*info).addr, src_buf, (*info).size as usize);
    let pcompat = (*info).addr as *mut u8;
    if *pcompat == 0 { pr_warn!("QAT: Unable to determine the version of VF"); return 0; }
    let compat = adf_vf_compat_checker(*pcompat);
    if compat == ADF_PF2VF_VF_INCOMPATIBLE { return -EINVAL; }
    0
}

unsafe fn adf_mstate_capmask_compare(sinfo: *mut adf_mstate_vreginfo, dinfo: *mut adf_mstate_vreginfo) -> i32 {
    if adf_mstate_check_cap_size((*sinfo).size, (*dinfo).size, core::mem::size_of::<u64>()) != 0 { return -1; }
    let mut src = 0u64; let mut dst = 0u64;
    memcpy(&mut src as *mut _ as *mut _, (*sinfo).addr, (*sinfo).size as usize);
    memcpy(&mut dst as *mut _ as *mut _, (*dinfo).addr, (*dinfo).size as usize);
    if src == dst { 0 } else if (src | dst) == dst { 1 } else { -1 }
}

unsafe fn adf_mstate_capmask_superset(_: *mut adf_mstate_mgr, buf: *mut u8, size: u32, opa: *mut core::ffi::c_void) -> i32 {
    let sinfo = adf_mstate_vreginfo { addr: buf, size };
    if adf_mstate_capmask_compare(&sinfo as *const _ as *mut _, opa as *mut _) >= 0 { 0 } else { -EINVAL }
}
unsafe fn adf_mstate_capmask_equal(_: *mut adf_mstate_mgr, buf: *mut u8, size: u32, opa: *mut core::ffi::c_void) -> i32 {
    let sinfo = adf_mstate_vreginfo { addr: buf, size };
    if adf_mstate_capmask_compare(&sinfo as *const _ as *mut _, opa as *mut _) == 0 { 0 } else { -EINVAL }
}
unsafe fn adf_mstate_set_vreg(_: *mut adf_mstate_mgr, buf: *mut u8, size: u32, opa: *mut core::ffi::c_void) -> i32 {
    let info = opa as *mut adf_mstate_vreginfo;
    if size != (*info).size { return -EINVAL; }
    memcpy((*info).addr, buf, (*info).size as usize); 0
}

unsafe fn adf_gen4_vfmig_load_etr_regs(_: *mut adf_mstate_mgr, state: *mut u8, _: u32, opa: *mut core::ffi::c_void) -> i32 {
    let bi = opa as *mut adf_vf_bank_info; let hw = (*(*bi).accel_dev).hw_device;
    let bank = (*bi).bank_nr + (*bi).vf_nr * (*hw).num_banks_per_vf;
    (*hw).bank_state_restore((*bi).accel_dev, bank, state as *mut adf_bank_state)
}

unsafe fn adf_gen4_vfmig_save_etr_regs(_: *mut adf_mstate_mgr, state: *mut u8, _: u32, opa: *mut core::ffi::c_void) -> i32 {
    let bi = opa as *mut adf_vf_bank_info; let hw = (*(*bi).accel_dev).hw_device;
    let bank = (*bi).bank_nr + (*bi).vf_nr * (*hw).num_banks_per_vf;
    let ret = (*hw).bank_state_save((*bi).accel_dev, bank, state as *mut adf_bank_state);
    if ret != 0 { ret } else { core::mem::size_of::<adf_bank_state>() as i32 }
}

// The remaining state-tree routines retain the original callback topology and
// call the same external mstate, CSR, locking, and hardware interfaces.
unsafe fn adf_gen4_vfmig_save_setup(mdev: *mut qat_mig_dev) -> i32 {
    let accel = (*mdev).parent_accel_dev; let vf = (*accel).pf.vf_info[(*mdev).vf_id as usize].mig_priv as *mut adf_gen4_vfmig;
    if (*mdev).setup_size != 0 { return 0; }
    adf_mstate_mgr_init((*vf).mstate_mgr, (*mdev).state, (*mdev).state_size);
    if adf_mstate_preamble_add((*vf).mstate_mgr).is_null() { return -EINVAL; }
    let ret = adf_gen4_vfmig_save_config(accel, (*mdev).vf_id); if ret != 0 { return ret; }
    adf_mstate_preamble_update((*vf).mstate_mgr);
    (*mdev).setup_size = adf_mstate_state_size((*vf).mstate_mgr); 0
}

unsafe fn adf_gen4_vfmig_load_setup(mdev: *mut qat_mig_dev, len: i32) -> i32 {
    let accel = (*mdev).parent_accel_dev; let vf = (*accel).pf.vf_info[(*mdev).vf_id as usize].mig_priv as *mut adf_gen4_vfmig;
    if (*mdev).remote_setup_size != 0 { return 0; }
    if len < core::mem::size_of::<adf_mstate_preh>() as i32 { return -EAGAIN; }
    adf_mstate_mgr_init((*vf).mstate_mgr, (*mdev).state, (*mdev).state_size);
    let setup = adf_mstate_state_size_from_remote((*vf).mstate_mgr);
    if setup > (*mdev).state_size || len < setup as i32 { return if setup > (*mdev).state_size { -EINVAL } else { -EAGAIN }; }
    let ret = adf_mstate_mgr_init_from_remote((*vf).mstate_mgr, (*mdev).state, setup, core::ptr::null_mut(), core::ptr::null_mut());
    if ret != 0 { return ret; }
    (*mdev).remote_setup_size = setup; adf_gen4_vfmig_load_config(accel, (*mdev).vf_id)
}

unsafe fn adf_gen4_vfmig_save_state(mdev: *mut qat_mig_dev) -> i32 {
    let accel = (*mdev).parent_accel_dev; let vf = (*accel).pf.vf_info[(*mdev).vf_id as usize].mig_priv as *mut adf_gen4_vfmig;
    let ret = adf_gen4_vfmig_save_setup(mdev); if ret != 0 { return ret; }
    adf_mstate_mgr_init((*vf).mstate_mgr, (*mdev).state.add((*mdev).setup_size), (*mdev).state_size - (*mdev).setup_size);
    if adf_mstate_preamble_add((*vf).mstate_mgr).is_null() { return -EINVAL; }
    let ret = adf_gen4_vfmig_save_generic(accel, (*mdev).vf_id); if ret != 0 { return ret; }
    let ret = adf_gen4_vfmig_save_misc(accel, (*mdev).vf_id); if ret != 0 { return ret; }
    let ret = adf_gen4_vfmig_save_etr(accel, (*mdev).vf_id); if ret != 0 { return ret; }
    adf_mstate_preamble_update((*vf).mstate_mgr); 0
}

unsafe fn adf_gen4_vfmig_load_state(mdev: *mut qat_mig_dev) -> i32 {
    let accel = (*mdev).parent_accel_dev; let vf = (*accel).pf.vf_info[(*mdev).vf_id as usize].mig_priv as *mut adf_gen4_vfmig;
    let ret = adf_gen4_vfmig_load_setup(mdev, (*mdev).state_size as i32); if ret != 0 { return ret; }
    let ret = adf_mstate_mgr_init_from_remote((*vf).mstate_mgr, (*mdev).state.add((*mdev).remote_setup_size), (*mdev).state_size - (*mdev).remote_setup_size, core::ptr::null_mut(), core::ptr::null_mut()); if ret != 0 { return ret; }
    let ret = adf_gen4_vfmig_load_generic(accel, (*mdev).vf_id); if ret != 0 { return ret; }
    let ret = adf_gen4_vfmig_load_misc(accel, (*mdev).vf_id); if ret != 0 { return ret; }
    adf_gen4_vfmig_load_etr(accel, (*mdev).vf_id)
}

pub unsafe fn adf_gen4_init_vf_mig_ops(ops: *mut qat_migdev_ops) {
    (*ops).init = Some(adf_gen4_vfmig_init_device); (*ops).cleanup = Some(adf_gen4_vfmig_cleanup_device);
    (*ops).reset = Some(adf_gen4_vfmig_reset_device); (*ops).open = Some(adf_gen4_vfmig_open_device); (*ops).close = Some(adf_gen4_vfmig_close_device);
    (*ops).suspend = Some(adf_gen4_vfmig_suspend_device); (*ops).resume = Some(adf_gen4_vfmig_resume_device);
    (*ops).save_state = Some(adf_gen4_vfmig_save_state); (*ops).load_state = Some(adf_gen4_vfmig_load_state);
    (*ops).load_setup = Some(adf_gen4_vfmig_load_setup); (*ops).save_setup = Some(adf_gen4_vfmig_save_setup);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
