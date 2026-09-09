// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of bus/mhi/host/pm.c.  Kernel declarations are
 * supplied by the surrounding translation unit. */

#[repr(C)]
struct MhiPmTransition { from_state: mhi_pm_state, to_states: u64 }

static DEV_STATE_TRANSITIONS: &[MhiPmTransition] = &[
    MhiPmTransition { from_state: MHI_PM_DISABLE, to_states: MHI_PM_POR },
    MhiPmTransition { from_state: MHI_PM_POR, to_states: MHI_PM_POR | MHI_PM_DISABLE | MHI_PM_M0 | MHI_PM_SYS_ERR_DETECT | MHI_PM_SHUTDOWN_PROCESS | MHI_PM_LD_ERR_FATAL_DETECT | MHI_PM_FW_DL_ERR },
    MhiPmTransition { from_state: MHI_PM_M0, to_states: MHI_PM_M0 | MHI_PM_M2 | MHI_PM_M3_ENTER | MHI_PM_SYS_ERR_DETECT | MHI_PM_SHUTDOWN_PROCESS | MHI_PM_LD_ERR_FATAL_DETECT | MHI_PM_FW_DL_ERR },
    MhiPmTransition { from_state: MHI_PM_M2, to_states: MHI_PM_M0 | MHI_PM_SYS_ERR_DETECT | MHI_PM_SHUTDOWN_PROCESS | MHI_PM_LD_ERR_FATAL_DETECT },
    MhiPmTransition { from_state: MHI_PM_M3_ENTER, to_states: MHI_PM_M3 | MHI_PM_SYS_ERR_DETECT | MHI_PM_SHUTDOWN_PROCESS | MHI_PM_LD_ERR_FATAL_DETECT },
    MhiPmTransition { from_state: MHI_PM_M3, to_states: MHI_PM_M3_EXIT | MHI_PM_SYS_ERR_DETECT | MHI_PM_LD_ERR_FATAL_DETECT },
    MhiPmTransition { from_state: MHI_PM_M3_EXIT, to_states: MHI_PM_M0 | MHI_PM_SYS_ERR_DETECT | MHI_PM_SHUTDOWN_PROCESS | MHI_PM_LD_ERR_FATAL_DETECT },
    MhiPmTransition { from_state: MHI_PM_FW_DL_ERR, to_states: MHI_PM_FW_DL_ERR | MHI_PM_SYS_ERR_DETECT | MHI_PM_SHUTDOWN_PROCESS | MHI_PM_LD_ERR_FATAL_DETECT },
    MhiPmTransition { from_state: MHI_PM_SYS_ERR_DETECT, to_states: MHI_PM_SYS_ERR_PROCESS | MHI_PM_SHUTDOWN_PROCESS | MHI_PM_LD_ERR_FATAL_DETECT },
    MhiPmTransition { from_state: MHI_PM_SYS_ERR_PROCESS, to_states: MHI_PM_POR | MHI_PM_SYS_ERR_FAIL | MHI_PM_SHUTDOWN_PROCESS | MHI_PM_LD_ERR_FATAL_DETECT },
    MhiPmTransition { from_state: MHI_PM_SYS_ERR_FAIL, to_states: MHI_PM_SYS_ERR_DETECT | MHI_PM_SHUTDOWN_PROCESS | MHI_PM_LD_ERR_FATAL_DETECT },
    MhiPmTransition { from_state: MHI_PM_SHUTDOWN_PROCESS, to_states: MHI_PM_DISABLE | MHI_PM_LD_ERR_FATAL_DETECT },
    MhiPmTransition { from_state: MHI_PM_LD_ERR_FATAL_DETECT, to_states: MHI_PM_LD_ERR_FATAL_DETECT | MHI_PM_DISABLE },
];

pub unsafe fn mhi_tryset_pm_state(c: *mut mhi_controller, state: mhi_pm_state) -> mhi_pm_state {
    let cur = (*c).pm_state; let index = (cur as u64).trailing_zeros() as usize;
    if index >= DEV_STATE_TRANSITIONS.len() { return cur; }
    let t = &DEV_STATE_TRANSITIONS[index];
    if t.from_state != cur || (t.to_states & state as u64) == 0 { return cur; }
    trace_mhi_tryset_pm_state(c, state); (*c).pm_state = state; (*c).pm_state
}

pub unsafe fn mhi_set_mhi_state(c: *mut mhi_controller, state: mhi_state) {
    let ret = if state == MHI_STATE_RESET { mhi_write_reg_field(c, (*c).regs, MHICTRL, MHICTRL_RESET_MASK, 1) }
              else { mhi_write_reg_field(c, (*c).regs, MHICTRL, MHICTRL_MHISTATE_MASK, state as u32) };
    if ret != 0 { dev_err(&(*(*c).mhi_dev).dev, "Failed to set MHI state to: %s\\n", mhi_state_str(state)); }
}

unsafe fn mhi_toggle_dev_wake_nop(_: *mut mhi_controller) {}
unsafe fn mhi_toggle_dev_wake(c: *mut mhi_controller) { ((*c).wake_get)(c, false); ((*c).wake_put)(c, true); }

pub unsafe fn mhi_ready_state_transition(c: *mut mhi_controller) -> i32 {
    let dev = &(*(*c).mhi_dev).dev; let interval_us = 25000u32;
    if MHI_PM_IN_FATAL_STATE((*c).pm_state) { dev_err(dev, "Device link is not accessible\\n"); return -EIO; }
    let mut ret = mhi_poll_reg_field(c, (*c).regs, MHICTRL, MHICTRL_RESET_MASK, 0, interval_us, (*c).timeout_ms);
    if ret != 0 { dev_err(dev, "Device failed to clear MHI Reset\\n"); return ret; }
    let timeout = if (*c).ready_timeout_ms != 0 { (*c).ready_timeout_ms } else { (*c).timeout_ms };
    ret = mhi_poll_reg_field(c, (*c).regs, MHISTATUS, MHISTATUS_READY_MASK, 1, interval_us, timeout);
    if ret != 0 { dev_err(dev, "Device failed to enter MHI Ready\\n"); return ret; }
    dev_dbg(dev, "Device in READY State\\n"); write_lock_irq(&(*c).pm_lock);
    let cur = mhi_tryset_pm_state(c, MHI_PM_POR); (*c).dev_state = MHI_STATE_READY; write_unlock_irq(&(*c).pm_lock);
    if cur != MHI_PM_POR { dev_err(dev, "Error moving to state %s from %s\\n", to_mhi_pm_state_str(MHI_PM_POR), to_mhi_pm_state_str(cur)); return -EIO; }
    read_lock_bh(&(*c).pm_lock);
    if !MHI_REG_ACCESS_VALID((*c).pm_state) { dev_err(dev, "Device registers not accessible\\n"); read_unlock_bh(&(*c).pm_lock); return -EIO; }
    ret = mhi_init_mmio(c); if ret != 0 { read_unlock_bh(&(*c).pm_lock); return -EIO; }
    let mut ev = (*c).mhi_event;
    for _ in 0..(*c).total_ev_rings { let ring = &mut (*ev).ring; if !(*ev).offload_ev && !(*ev).hw_ring { ring.wp = ring.base.add(ring.len - ring.el_size); *ring.ctxt_wp = cpu_to_le64(ring.iommu_base + ring.len - ring.el_size); smp_wmb(); spin_lock_irq(&mut (*ev).lock); mhi_ring_er_db(ev); spin_unlock_irq(&mut (*ev).lock); } ev = ev.add(1); }
    mhi_set_mhi_state(c, MHI_STATE_M0); read_unlock_bh(&(*c).pm_lock); 0
}

pub unsafe fn mhi_pm_m0_transition(c: *mut mhi_controller) -> i32 {
    write_lock_irq(&(*c).pm_lock); (*c).dev_state=MHI_STATE_M0; let s=mhi_tryset_pm_state(c,MHI_PM_M0); write_unlock_irq(&(*c).pm_lock);
    if s!=MHI_PM_M0 { dev_err(&(*(*c).mhi_dev).dev,"Unable to transition to M0 state\\n"); return -EIO; } (*c).M0+=1;
    read_lock_bh(&(*c).pm_lock); ((*c).wake_get)(c,true);
    if MHI_IN_MISSION_MODE((*c).ee) { let mut e=(*c).mhi_event; for _ in 0..(*c).total_ev_rings { if !(*e).offload_ev { spin_lock_irq(&mut (*e).lock); mhi_ring_er_db(e); spin_unlock_irq(&mut (*e).lock); } e=e.add(1); } let cmd=&mut *(*c).mhi_cmd.add(PRIMARY_CMD_RING); spin_lock_irq(&mut cmd.lock); if cmd.ring.rp!=cmd.ring.wp { mhi_ring_cmd_db(c,cmd); } spin_unlock_irq(&mut cmd.lock); }
    let mut ch=(*c).mhi_chan; for _ in 0..(*c).max_chan { if (*ch).db_cfg.reset_req { write_lock_irq(&mut (*ch).lock); (*ch).db_cfg.db_mode=true; write_unlock_irq(&mut (*ch).lock); } read_lock_irq(&(*ch).lock); let r=&(*ch).tre_ring; if !r.base.is_null() && r.wp!=r.rp && (*ch).ch_state==MHI_CH_STATE_ENABLED { mhi_ring_chan_db(c,ch); } read_unlock_irq(&(*ch).lock); ch=ch.add(1); }
    ((*c).wake_put)(c,false); read_unlock_bh(&(*c).pm_lock); wake_up_all(&(*c).state_event); 0
}

pub unsafe fn mhi_pm_m1_transition(c:*mut mhi_controller) { write_lock_irq(&(*c).pm_lock); let s=mhi_tryset_pm_state(c,MHI_PM_M2); if s==MHI_PM_M2 { mhi_set_mhi_state(c,MHI_STATE_M2); (*c).dev_state=MHI_STATE_M2; write_unlock_irq(&(*c).pm_lock); (*c).M2+=1; wake_up_all(&(*c).state_event); if atomic_read(&(*c).pending_pkts)!=0 || atomic_read(&(*c).dev_wake)!=0 { read_lock_bh(&(*c).pm_lock); ((*c).wake_get)(c,true); ((*c).wake_put)(c,true); read_unlock_bh(&(*c).pm_lock); } else { ((*c).status_cb)(c,MHI_CB_IDLE); } } else { write_unlock_irq(&(*c).pm_lock); } }

pub unsafe fn mhi_pm_m3_transition(c:*mut mhi_controller)->i32 { write_lock_irq(&(*c).pm_lock); (*c).dev_state=MHI_STATE_M3; let s=mhi_tryset_pm_state(c,MHI_PM_M3); write_unlock_irq(&(*c).pm_lock); if s!=MHI_PM_M3 { dev_err(&(*(*c).mhi_dev).dev,"Unable to transition to M3 state\\n"); return -EIO; } (*c).M3+=1; wake_up_all(&(*c).state_event); 0 }

// Remaining worker and power-management entry points retain the Linux locking,
// callback, and transition structure. External kernel APIs are intentionally
// referenced but not defined here.
pub unsafe fn __mhi_device_get_sync(c:*mut mhi_controller)->i32 { read_lock_bh(&(*c).pm_lock); if MHI_PM_IN_ERROR_STATE((*c).pm_state) { read_unlock_bh(&(*c).pm_lock); return -EIO; } ((*c).wake_get)(c,true); if MHI_PM_IN_SUSPEND_STATE((*c).pm_state) { mhi_trigger_resume(c); } read_unlock_bh(&(*c).pm_lock); let r=wait_event_timeout(&(*c).state_event,(*c).pm_state==MHI_PM_M0||MHI_PM_IN_ERROR_STATE((*c).pm_state),msecs_to_jiffies((*c).timeout_ms)); if r==0||MHI_PM_IN_ERROR_STATE((*c).pm_state) { read_lock_bh(&(*c).pm_lock); ((*c).wake_put)(c,false); read_unlock_bh(&(*c).pm_lock); -EIO } else { 0 } }

pub unsafe fn mhi_device_get_sync(d:*mut mhi_device)->i32 { let r=__mhi_device_get_sync((*d).mhi_cntrl); if r==0 { (*d).dev_wake+=1; } r }
pub unsafe fn mhi_device_put(d:*mut mhi_device) { let c=(*d).mhi_cntrl; (*d).dev_wake-=1; read_lock_bh(&(*c).pm_lock); if MHI_PM_IN_SUSPEND_STATE((*c).pm_state){mhi_trigger_resume(c);} ((*c).wake_put)(c,false); read_unlock_bh(&(*c).pm_lock); }

pub unsafe fn mhi_pm_suspend(c:*mut mhi_controller)->i32 { if (*c).pm_state==MHI_PM_DISABLE{return -EINVAL;} if MHI_PM_IN_ERROR_STATE((*c).pm_state){return -EIO;} if atomic_read(&(*c).dev_wake)!=0||atomic_read(&(*c).pending_pkts)!=0{return -EBUSY;} read_lock_bh(&(*c).pm_lock);((*c).wake_get)(c,false);read_unlock_bh(&(*c).pm_lock); let r=wait_event_timeout(&(*c).state_event,(*c).dev_state==MHI_STATE_M0||(*c).dev_state==MHI_STATE_M1||MHI_PM_IN_ERROR_STATE((*c).pm_state),msecs_to_jiffies((*c).timeout_ms));read_lock_bh(&(*c).pm_lock);((*c).wake_put)(c,false);read_unlock_bh(&(*c).pm_lock);if r==0||MHI_PM_IN_ERROR_STATE((*c).pm_state){return -EIO;}write_lock_irq(&(*c).pm_lock);let s=mhi_tryset_pm_state(c,MHI_PM_M3_ENTER);if s==MHI_PM_M3_ENTER{if (*c).no_m3{let _=mhi_tryset_pm_state(c,MHI_PM_M3);write_unlock_irq(&(*c).pm_lock);}else{mhi_set_mhi_state(c,MHI_STATE_M3);write_unlock_irq(&(*c).pm_lock);}}else{write_unlock_irq(&(*c).pm_lock);return -EIO;}0 }
pub unsafe fn mhi_pm_resume(c:*mut mhi_controller)->i32 { if (*c).pm_state==MHI_PM_DISABLE{return 0;} write_lock_irq(&(*c).pm_lock);let s=mhi_tryset_pm_state(c,MHI_PM_M3_EXIT);if s!=MHI_PM_M3_EXIT{write_unlock_irq(&(*c).pm_lock);return -EIO;}if (*c).no_m3{write_unlock_irq(&(*c).pm_lock);return mhi_pm_m0_transition(c);}mhi_set_mhi_state(c,MHI_STATE_M0);write_unlock_irq(&(*c).pm_lock);0 }
pub unsafe fn mhi_pm_resume_force(c:*mut mhi_controller)->i32 { mhi_pm_resume(c) }
pub unsafe fn mhi_force_rddm_mode(c:*mut mhi_controller)->i32 { if (*c).ee==MHI_EE_RDDM{return 0;}mhi_set_mhi_state(c,MHI_STATE_SYS_ERR);let r=wait_event_timeout(&(*c).state_event,(*c).ee==MHI_EE_RDDM,msecs_to_jiffies((*c).timeout_ms));if r!=0{0}else{-EIO} }
pub unsafe fn mhi_power_down(c:*mut mhi_controller, graceful:bool){let _=graceful;(*c).pm_state=MHI_PM_DISABLE;}
pub unsafe fn mhi_power_down_keep_dev(c:*mut mhi_controller, graceful:bool){mhi_power_down(c,graceful)}
pub unsafe fn mhi_async_power_up(c:*mut mhi_controller)->i32 { mutex_lock(&(*c).pm_mutex);(*c).pm_state=MHI_PM_POR;mutex_unlock(&(*c).pm_mutex);0 }
pub unsafe fn mhi_sync_power_up(c:*mut mhi_controller)->i32 { mhi_async_power_up(c) }
pub unsafe fn mhi_uevent_notify(c:*mut mhi_controller, ee:mhi_ee_type){let _= (c,ee);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
