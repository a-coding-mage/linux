// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 */

// Kernel headers and symbols are supplied by the surrounding translation unit.

unsafe fn mhi_debugfs_states_show(m: *mut seq_file, _d: *mut c_void) -> c_int {
    let mhi_cntrl = (*m).private as *mut mhi_controller;
    seq_printf(m, "PM state: %s Device: %s MHI state: %s EE: %s wake: %s\n",
        to_mhi_pm_state_str((*mhi_cntrl).pm_state),
        if mhi_is_active(mhi_cntrl) { "Active" } else { "Inactive" },
        mhi_state_str((*mhi_cntrl).dev_state),
        TO_MHI_EXEC_STR((*mhi_cntrl).ee),
        str_true_false((*mhi_cntrl).wake_set));
    seq_printf(m, "M0: %u M2: %u M3: %u", (*mhi_cntrl).M0, (*mhi_cntrl).M2, (*mhi_cntrl).M3);
    seq_printf(m, " device wake: %u pending packets: %u\n",
        atomic_read(&(*mhi_cntrl).dev_wake), atomic_read(&(*mhi_cntrl).pending_pkts));
    0
}

unsafe fn mhi_debugfs_events_show(m: *mut seq_file, _d: *mut c_void) -> c_int {
    let mhi_cntrl = (*m).private as *mut mhi_controller;
    if !mhi_is_active(mhi_cntrl) { seq_puts(m, "Device not ready\n"); return -ENODEV; }
    let mut er_ctxt = (*(*mhi_cntrl).mhi_ctxt).er_ctxt;
    let mut mhi_event = (*mhi_cntrl).mhi_event;
    for i in 0..(*mhi_cntrl).total_ev_rings {
        let ring = &mut (*mhi_event).ring;
        if (*mhi_event).offload_ev {
            seq_printf(m, "Index: %d is an offload event ring\n", i); continue;
        }
        seq_printf(m, "Index: %d intmod count: %lu time: %lu", i,
            (le32_to_cpu((*er_ctxt).intmod) & EV_CTX_INTMODC_MASK) >> __ffs(EV_CTX_INTMODC_MASK),
            (le32_to_cpu((*er_ctxt).intmod) & EV_CTX_INTMODT_MASK) >> __ffs(EV_CTX_INTMODT_MASK));
        seq_printf(m, " base: 0x%0llx len: 0x%llx", le64_to_cpu((*er_ctxt).rbase), le64_to_cpu((*er_ctxt).rlen));
        seq_printf(m, " rp: 0x%llx wp: 0x%llx", le64_to_cpu((*er_ctxt).rp), le64_to_cpu((*er_ctxt).wp));
        seq_printf(m, " local rp: 0x%pK db: 0x%pad\n", ring.rp, &(*mhi_event).db_cfg.db_val);
        er_ctxt = er_ctxt.add(1); mhi_event = mhi_event.add(1);
    }
    0
}

unsafe fn mhi_debugfs_channels_show(m: *mut seq_file, _d: *mut c_void) -> c_int {
    let mhi_cntrl = (*m).private as *mut mhi_controller;
    if !mhi_is_active(mhi_cntrl) { seq_puts(m, "Device not ready\n"); return -ENODEV; }
    let mut mhi_chan = (*mhi_cntrl).mhi_chan;
    let mut chan_ctxt = (*(*mhi_cntrl).mhi_ctxt).chan_ctxt;
    for i in 0..(*mhi_cntrl).max_chan {
        let ring = &mut (*mhi_chan).tre_ring;
        if (*mhi_chan).offload_ch { seq_printf(m, "%s(%u) is an offload channel\n", (*mhi_chan).name, (*mhi_chan).chan); }
        else if !(*mhi_chan).mhi_dev {
            mhi_chan = mhi_chan.add(1); chan_ctxt = chan_ctxt.add(1); continue;
        } else {
            seq_printf(m, "%s(%u) state: 0x%lx brstmode: 0x%lx pollcfg: 0x%lx", (*mhi_chan).name, (*mhi_chan).chan,
                (le32_to_cpu((*chan_ctxt).chcfg) & CHAN_CTX_CHSTATE_MASK) >> __ffs(CHAN_CTX_CHSTATE_MASK),
                (le32_to_cpu((*chan_ctxt).chcfg) & CHAN_CTX_BRSTMODE_MASK) >> __ffs(CHAN_CTX_BRSTMODE_MASK),
                (le32_to_cpu((*chan_ctxt).chcfg) & CHAN_CTX_POLLCFG_MASK) >> __ffs(CHAN_CTX_POLLCFG_MASK));
            seq_printf(m, " type: 0x%x event ring: %u", le32_to_cpu((*chan_ctxt).chtype), le32_to_cpu((*chan_ctxt).erindex));
            seq_printf(m, " base: 0x%llx len: 0x%llx rp: 0x%llx wp: 0x%llx", le64_to_cpu((*chan_ctxt).rbase), le64_to_cpu((*chan_ctxt).rlen), le64_to_cpu((*chan_ctxt).rp), le64_to_cpu((*chan_ctxt).wp));
            seq_printf(m, " local rp: 0x%pK local wp: 0x%pK db: 0x%pad\n", ring.rp, ring.wp, &(*mhi_chan).db_cfg.db_val);
        }
        mhi_chan = mhi_chan.add(1); chan_ctxt = chan_ctxt.add(1);
    }
    0
}

unsafe fn mhi_device_info_show(dev: *mut device, data: *mut c_void) -> c_int {
    if (*dev).bus != &mhi_bus_type { return 0; }
    let mhi_dev = to_mhi_device(dev);
    seq_printf(data as *mut seq_file, "%s: type: %s dev_wake: %u", (*mhi_dev).name, if (*mhi_dev).dev_type { "Controller" } else { "Transfer" }, (*mhi_dev).dev_wake);
    if (*mhi_dev).dev_type == MHI_DEVICE_XFER { seq_printf(data as *mut seq_file, " channels: %u(UL)/%u(DL)", (*mhi_dev).ul_chan_id, (*mhi_dev).dl_chan_id); }
    seq_puts(data as *mut seq_file, "\n"); 0
}

unsafe fn mhi_debugfs_devices_show(m: *mut seq_file, _d: *mut c_void) -> c_int {
    let c = (*m).private as *mut mhi_controller;
    if !mhi_is_active(c) { seq_puts(m, "Device not ready\n"); return -ENODEV; }
    mhi_device_info_show(&mut (*(*c).mhi_dev).dev, m as *mut c_void);
    device_for_each_child(&mut (*(*c).mhi_dev).dev, m as *mut c_void, mhi_device_info_show); 0
}

unsafe fn mhi_debugfs_regdump_show(m: *mut seq_file, _d: *mut c_void) -> c_int {
    let c = (*m).private as *mut mhi_controller; let mut val: u32 = 0;
    if !MHI_REG_ACCESS_VALID((*c).pm_state) { return -EIO; }
    seq_printf(m, "Host PM state: %s Device state: %s EE: %s\n", to_mhi_pm_state_str((*c).pm_state), mhi_state_str((*c).dev_state), TO_MHI_EXEC_STR((*c).ee));
    let state = mhi_get_mhi_state(c); let ee = mhi_get_exec_env(c);
    seq_printf(m, "Device EE: %s state: %s\n", TO_MHI_EXEC_STR(ee), mhi_state_str(state));
    let regs = [
        ("MHI_REGLEN", MHIREGLEN, (*c).regs), ("MHI_VER", MHIVER, (*c).regs), ("MHI_CFG", MHICFG, (*c).regs), ("MHI_CTRL", MHICTRL, (*c).regs), ("MHI_STATUS", MHISTATUS, (*c).regs), ("MHI_WAKE_DB", 0, (*c).wake_db),
        ("BHI_EXECENV", BHI_EXECENV, (*c).bhi), ("BHI_STATUS", BHI_STATUS, (*c).bhi), ("BHI_ERRCODE", BHI_ERRCODE, (*c).bhi), ("BHI_ERRDBG1", BHI_ERRDBG1, (*c).bhi), ("BHI_ERRDBG2", BHI_ERRDBG2, (*c).bhi), ("BHI_ERRDBG3", BHI_ERRDBG3, (*c).bhi),
        ("BHIE_TXVEC_DB", BHIE_TXVECDB_OFFS, (*c).bhie), ("BHIE_TXVEC_STATUS", BHIE_TXVECSTATUS_OFFS, (*c).bhie), ("BHIE_RXVEC_DB", BHIE_RXVECDB_OFFS, (*c).bhie), ("BHIE_RXVEC_STATUS", BHIE_RXVECSTATUS_OFFS, (*c).bhie)];
    for &(name, offset, base) in &regs { if base.is_null() { continue; } if mhi_read_reg(c, base, offset, &mut val) != 0 { continue; } seq_printf(m, "%s: 0x%x\n", name, val); } 0
}

unsafe fn mhi_debugfs_device_wake_show(m: *mut seq_file, _d: *mut c_void) -> c_int {
    let c = (*m).private as *mut mhi_controller; let d = (*c).mhi_dev;
    if !mhi_is_active(c) { seq_puts(m, "Device not ready\n"); return -ENODEV; }
    seq_printf(m, "Wake count: %d\n%s\n", (*d).dev_wake, "Usage: echo get/put > device_wake to vote/unvote for M0"); 0
}

unsafe fn mhi_debugfs_device_wake_write(file: *mut file, ubuf: *const c_char, count: usize, _ppos: *mut loff_t) -> isize {
    let m = (*file).private_data as *mut seq_file; let c = (*m).private as *mut mhi_controller; let d = (*c).mhi_dev; let mut buf = [0i8; 16];
    if copy_from_user(buf.as_mut_ptr(), ubuf, core::cmp::min(15, count)) != 0 { return -EFAULT as isize; }
    let ret = if !strncmp(buf.as_ptr(), b"get\0".as_ptr() as *const c_char, 3) { mhi_device_get_sync(d) } else if !strncmp(buf.as_ptr(), b"put\0".as_ptr() as *const c_char, 3) { mhi_device_put(d); 0 } else { -EINVAL };
    if ret != 0 { ret as isize } else { count as isize }
}

unsafe fn mhi_debugfs_timeout_ms_show(m: *mut seq_file, _d: *mut c_void) -> c_int { seq_printf(m, "%u ms\n", (*((*m).private as *mut mhi_controller)).timeout_ms); 0 }
unsafe fn mhi_debugfs_timeout_ms_write(file: *mut file, ubuf: *const c_char, count: usize, _ppos: *mut loff_t) -> isize { let c = (*((*file).private_data as *mut seq_file)).private as *mut mhi_controller; let mut t=0u32; if kstrtou32_from_user(ubuf,count,0,&mut t)!=0{return -EINVAL as isize;} (*c).timeout_ms=t; count as isize }

unsafe fn mhi_debugfs_states_open(i:*mut inode,f:*mut file)->c_int{single_open(f,mhi_debugfs_states_show,(*i).i_private)}
unsafe fn mhi_debugfs_events_open(i:*mut inode,f:*mut file)->c_int{single_open(f,mhi_debugfs_events_show,(*i).i_private)}
unsafe fn mhi_debugfs_channels_open(i:*mut inode,f:*mut file)->c_int{single_open(f,mhi_debugfs_channels_show,(*i).i_private)}
unsafe fn mhi_debugfs_devices_open(i:*mut inode,f:*mut file)->c_int{single_open(f,mhi_debugfs_devices_show,(*i).i_private)}
unsafe fn mhi_debugfs_regdump_open(i:*mut inode,f:*mut file)->c_int{single_open(f,mhi_debugfs_regdump_show,(*i).i_private)}
unsafe fn mhi_debugfs_device_wake_open(i:*mut inode,f:*mut file)->c_int{single_open(f,mhi_debugfs_device_wake_show,(*i).i_private)}
unsafe fn mhi_debugfs_timeout_ms_open(i:*mut inode,f:*mut file)->c_int{single_open(f,mhi_debugfs_timeout_ms_show,(*i).i_private)}

static debugfs_states_fops: file_operations = file_operations { open: Some(mhi_debugfs_states_open), release: Some(single_release), read: Some(seq_read) };
static debugfs_events_fops: file_operations = file_operations { open: Some(mhi_debugfs_events_open), release: Some(single_release), read: Some(seq_read) };
static debugfs_channels_fops: file_operations = file_operations { open: Some(mhi_debugfs_channels_open), release: Some(single_release), read: Some(seq_read) };
static debugfs_devices_fops: file_operations = file_operations { open: Some(mhi_debugfs_devices_open), release: Some(single_release), read: Some(seq_read) };
static debugfs_regdump_fops: file_operations = file_operations { open: Some(mhi_debugfs_regdump_open), release: Some(single_release), read: Some(seq_read) };
static debugfs_device_wake_fops: file_operations = file_operations { open: Some(mhi_debugfs_device_wake_open), write: Some(mhi_debugfs_device_wake_write), release: Some(single_release), read: Some(seq_read) };
static debugfs_timeout_ms_fops: file_operations = file_operations { open: Some(mhi_debugfs_timeout_ms_open), write: Some(mhi_debugfs_timeout_ms_write), release: Some(single_release), read: Some(seq_read) };

static mut mhi_debugfs_root: *mut dentry = core::ptr::null_mut();

pub unsafe fn mhi_create_debugfs(c:*mut mhi_controller){
    (*c).debugfs_dentry=debugfs_create_dir(dev_name(&mut (*(*c).mhi_dev).dev),mhi_debugfs_root);
    debugfs_create_file("states",0o444,(*c).debugfs_dentry,c,&debugfs_states_fops); debugfs_create_file("events",0o444,(*c).debugfs_dentry,c,&debugfs_events_fops); debugfs_create_file("channels",0o444,(*c).debugfs_dentry,c,&debugfs_channels_fops); debugfs_create_file("devices",0o444,(*c).debugfs_dentry,c,&debugfs_devices_fops); debugfs_create_file("regdump",0o444,(*c).debugfs_dentry,c,&debugfs_regdump_fops); debugfs_create_file("device_wake",0o644,(*c).debugfs_dentry,c,&debugfs_device_wake_fops); debugfs_create_file("timeout_ms",0o644,(*c).debugfs_dentry,c,&debugfs_timeout_ms_fops);
}
pub unsafe fn mhi_destroy_debugfs(c:*mut mhi_controller){debugfs_remove_recursive((*c).debugfs_dentry);(*c).debugfs_dentry=core::ptr::null_mut();}
pub unsafe fn mhi_debugfs_init(){mhi_debugfs_root=debugfs_create_dir(mhi_bus_type.name,core::ptr::null_mut());}
pub unsafe fn mhi_debugfs_exit(){debugfs_remove_recursive(mhi_debugfs_root);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
