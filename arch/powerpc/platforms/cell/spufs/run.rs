// SPDX-License-Identifier: GPL-2.0
// DEBUG

// Kernel and architecture dependencies are supplied by the surrounding crate.

pub unsafe fn spufs_stop_callback(spu: *mut spu, irq: i32) {
    let ctx = (*spu).ctx;
    if !ctx.is_null() {
        match irq {
            0 => {
                (*ctx).csa.class_0_pending = (*spu).class_0_pending;
                (*ctx).csa.class_0_dar = (*spu).class_0_dar;
            }
            1 => {
                (*ctx).csa.class_1_dsisr = (*spu).class_1_dsisr;
                (*ctx).csa.class_1_dar = (*spu).class_1_dar;
            }
            2 => {}
            _ => {}
        }
        smp_wmb();
        wake_up_all(&mut (*ctx).stop_wq);
    }
}

pub unsafe fn spu_stopped(ctx: *mut spu_context, stat: *mut u32) -> i32 {
    let stopped = SPU_STATUS_INVALID_INSTR | SPU_STATUS_SINGLE_STEP |
        SPU_STATUS_STOPPED_BY_HALT | SPU_STATUS_STOPPED_BY_STOP;
    loop {
        *stat = ((*ctx).ops).status_read(ctx);
        if (*stat & stopped) != 0 {
            if (*stat & SPU_STATUS_RUNNING) != 0 { continue; }
            return 1;
        }
        if test_bit(SPU_SCHED_NOTIFY_ACTIVE, &(*ctx).sched_flags) != 0 { return 1; }
        let dsisr = (*ctx).csa.class_1_dsisr;
        if (dsisr & (MFC_DSISR_PTE_NOT_FOUND | MFC_DSISR_ACCESS_DENIED)) != 0 { return 1; }
        if (*ctx).csa.class_0_pending != 0 { return 1; }
        return 0;
    }
}

unsafe fn spu_setup_isolated(ctx: *mut spu_context) -> i32 {
    let mut ret = -ENODEV;
    if isolated_loader.is_null() { return ret; }
    spu_unmap_mappings(ctx);
    let mfc_cntl = &mut (*(*ctx).spu).priv2.mfc_control_RW as *mut _;
    let mut timeout;
    out_be64(mfc_cntl, MFC_CNTL_PURGE_DMA_REQUEST);
    timeout = jiffies + HZ;
    while (in_be64(mfc_cntl) & MFC_CNTL_PURGE_DMA_STATUS_MASK) != MFC_CNTL_PURGE_DMA_COMPLETE {
        if time_after(jiffies, timeout) { printk(KERN_ERR, "%s: timeout flushing MFC DMA queue\n", "spu_setup_isolated"); return -EIO; }
        cond_resched();
    }
    out_be64(mfc_cntl, 0);
    let mut sr1 = spu_mfc_sr1_get((*ctx).spu);
    sr1 &= !MFC_STATE1_PROBLEM_STATE_MASK;
    spu_mfc_sr1_set((*ctx).spu, sr1);
    ((*ctx).ops).signal1_write(ctx, (isolated_loader as usize >> 32) as _);
    ((*ctx).ops).signal2_write(ctx, (isolated_loader as usize & 0xffff_ffff) as _);
    ((*ctx).ops).runcntl_write(ctx, SPU_RUNCNTL_RUNNABLE | SPU_RUNCNTL_ISOLATE);
    ret = 0;
    timeout = jiffies + HZ;
    let mut status;
    let status_loading = SPU_STATUS_RUNNING | SPU_STATUS_ISOLATED_STATE | SPU_STATUS_ISOLATED_LOAD_STATUS;
    loop {
        status = ((*ctx).ops).status_read(ctx);
        if (status & status_loading) != status_loading { break; }
        if time_after(jiffies, timeout) { printk(KERN_ERR, "%s: timeout waiting for loader\n", "spu_setup_isolated"); ret = -EIO; break; }
        cond_resched();
    }
    if ret == 0 && (status & SPU_STATUS_RUNNING) == 0 { pr_debug!("isolated LOAD failed"); ((*ctx).ops).runcntl_write(ctx, SPU_RUNCNTL_RUNNABLE); ret = -EACCES; }
    else if ret == 0 && (status & SPU_STATUS_ISOLATED_STATE) == 0 { pr_debug!("SPU fell out of isolated mode?"); ((*ctx).ops).runcntl_write(ctx, SPU_RUNCNTL_STOP); ret = -EINVAL; }
    sr1 |= MFC_STATE1_PROBLEM_STATE_MASK;
    spu_mfc_sr1_set((*ctx).spu, sr1);
    ret
}

unsafe fn spu_run_init(ctx: *mut spu_context, npc: *mut u32) -> i32 {
    let mut runcntl = SPU_RUNCNTL_RUNNABLE;
    spuctx_switch_state(ctx, SPU_UTIL_SYSTEM);
    if ((*ctx).flags & SPU_CREATE_NOSCHED) != 0 && (*ctx).state == SPU_STATE_SAVED { let ret = spu_activate(ctx, 0); if ret != 0 { return ret; } }
    if ((*ctx).flags & SPU_CREATE_ISOLATE) != 0 {
        if (((*ctx).ops).status_read(ctx) & SPU_STATUS_ISOLATED_STATE) == 0 { let ret = spu_setup_isolated(ctx); if ret != 0 { return ret; } }
        runcntl = ((*ctx).ops).runcntl_read(ctx) & (SPU_RUNCNTL_RUNNABLE | SPU_RUNCNTL_ISOLATE);
        if runcntl == 0 { runcntl = SPU_RUNCNTL_RUNNABLE; }
    } else {
        let privcntl = if test_thread_flag(TIF_SINGLESTEP) != 0 { SPU_PRIVCNTL_MODE_SINGLE_STEP } else { SPU_PRIVCNTL_MODE_NORMAL };
        ((*ctx).ops).privcntl_write(ctx, privcntl); ((*ctx).ops).npc_write(ctx, *npc);
    }
    ((*ctx).ops).runcntl_write(ctx, runcntl);
    if ((*ctx).flags & SPU_CREATE_NOSCHED) != 0 { spuctx_switch_state(ctx, SPU_UTIL_USER); } else if (*ctx).state == SPU_STATE_SAVED { let ret = spu_activate(ctx, 0); if ret != 0 { return ret; } } else { spuctx_switch_state(ctx, SPU_UTIL_USER); }
    set_bit(SPU_SCHED_SPU_RUN, &mut (*ctx).sched_flags); 0
}

unsafe fn spu_run_fini(ctx: *mut spu_context, npc: *mut u32, status: *mut u32) -> i32 {
    spu_del_from_rq(ctx); *status = ((*ctx).ops).status_read(ctx); *npc = ((*ctx).ops).npc_read(ctx); spuctx_switch_state(ctx, SPU_UTIL_IDLE_LOADED); clear_bit(SPU_SCHED_SPU_RUN, &mut (*ctx).sched_flags); spu_switch_log_notify(core::ptr::null_mut(), ctx, SWITCH_LOG_EXIT, *status); spu_release(ctx); if signal_pending(current) != 0 { -ERESTARTSYS } else { 0 }
}

unsafe fn spu_handle_restartsys(_ctx: *mut spu_context, spu_ret: *mut i64, npc: *mut u32) -> i32 {
    match *spu_ret { -ERESTARTSYS | -ERESTARTNOINTR => { *npc = (*npc).wrapping_sub(8); -ERESTARTSYS }, -ERESTARTNOHAND | -ERESTART_RESTARTBLOCK => { *spu_ret = -EINTR; -ERESTARTSYS }, _ => { printk(KERN_WARNING, "%s: unexpected return code %ld\n", "spu_handle_restartsys", *spu_ret); 0 } }
}

unsafe fn spu_process_callback(ctx: *mut spu_context) -> i32 {
    let mut s: spu_syscall_block = core::mem::zeroed();
    let mut npc = ((*ctx).ops).npc_read(ctx) & !3;
    let mut ls = ((*ctx).ops).get_ls(ctx) as *mut u8;
    let ls_pointer = in_be32(ls.add(npc as usize));
    if ls_pointer > (LS_SIZE - core::mem::size_of::<spu_syscall_block>()) as u32 { return -EFAULT; }
    memcpy_fromio(&mut s as *mut _ as *mut u8, ls.add(ls_pointer as usize), core::mem::size_of::<spu_syscall_block>());
    let mut ret = 0;
    let mut spu_ret: i64 = -ENOSYS as i64;
    npc = npc.wrapping_add(4);
    if s.nr_ret < NR_syscalls {
        spu_release(ctx);
        spu_ret = spu_sys_callback(&mut s);
        if spu_ret <= -ERESTARTSYS as i64 { ret = spu_handle_restartsys(ctx, &mut spu_ret, &mut npc); }
        mutex_lock(&mut (*ctx).state_mutex);
        if ret == -ERESTARTSYS { return ret; }
    }
    ls = ((*ctx).ops).get_ls(ctx) as *mut u8;
    memcpy_toio(ls.add(ls_pointer as usize), &spu_ret as *const _ as *const u8, core::mem::size_of::<i64>());
    ((*ctx).ops).npc_write(ctx, npc);
    ((*ctx).ops).runcntl_write(ctx, SPU_RUNCNTL_RUNNABLE);
    ret
}

pub unsafe fn spufs_run_spu(ctx: *mut spu_context, npc: *mut u32, event: *mut u32) -> i64 {
    if mutex_lock_interruptible(&mut (*ctx).run_mutex) != 0 { return -ERESTARTSYS as i64; }
    (*ctx).event_return = 0; let mut ret = spu_acquire(ctx); if ret != 0 { mutex_unlock(&mut (*ctx).run_mutex); return ret as i64; }
    spu_enable_spu(ctx); spu_update_sched_info(ctx); ret = spu_run_init(ctx, npc);
    if ret == 0 { let mut status = 0; loop { ret = spufs_wait(&mut (*ctx).stop_wq, spu_stopped(ctx, &mut status) != 0); if ret != 0 { mutex_lock(&mut (*ctx).state_mutex); break; } if test_and_clear_bit(SPU_SCHED_NOTIFY_ACTIVE, &mut (*ctx).sched_flags) != 0 && (status & SPU_STATUS_STOPPED_BY_STOP) == 0 { continue; } spuctx_switch_state(ctx, SPU_UTIL_SYSTEM); if (status & SPU_STATUS_STOPPED_BY_STOP) != 0 && status >> SPU_STOP_STATUS_SHIFT == 0x2104 { ret = spu_process_callback(ctx); if ret != 0 { break; } } ret = spufs_handle_class1(ctx); if ret != 0 { break; } ret = spufs_handle_class0(ctx); if ret != 0 { break; } if signal_pending(current) != 0 { ret = -ERESTARTSYS; } if ret != 0 || (status & (SPU_STATUS_STOPPED_BY_STOP | SPU_STATUS_STOPPED_BY_HALT | SPU_STATUS_SINGLE_STEP)) != 0 { break; } } spu_disable_spu(ctx); ret = spu_run_fini(ctx, npc, &mut status); spu_yield(ctx); if (status & SPU_STATUS_STOPPED_BY_STOP) != 0 && ((status >> SPU_STOP_STATUS_SHIFT) & 0x3f00) == 0x2100 { (*ctx).stats.libassist += 1; } if ret == 0 || (ret == -ERESTARTSYS && (status & (SPU_STATUS_STOPPED_BY_HALT | SPU_STATUS_SINGLE_STEP)) != 0) { ret = status as i32; } if (status & SPU_STATUS_SINGLE_STEP) != 0 || ((status & SPU_STATUS_STOPPED_BY_STOP) != 0 && status >> SPU_STOP_STATUS_SHIFT == 0x3fff) { ret = -ERESTARTSYS; } } else { spu_release(ctx); }
    *event = (*ctx).event_return; mutex_unlock(&mut (*ctx).run_mutex); ret as i64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
