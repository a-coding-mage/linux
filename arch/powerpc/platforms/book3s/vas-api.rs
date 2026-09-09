// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * VAS user space API for its accelerators (Only NX-GZIP is supported now)
 * Copyright (C) 2019 Haren Myneni, IBM Corp
 */

// C dependencies supplied by the kernel and architecture headers.

#[repr(C)]
struct CoprocDev {
    cdev: cdev,
    device: *mut device,
    name: *mut i8,
    devt: dev_t,
    class: *mut class,
    cop_type: vas_cop_type,
    vops: *const vas_user_win_ops,
}

#[repr(C)]
struct CoprocInstance {
    coproc: *mut CoprocDev,
    txwin: *mut vas_window,
}

static mut coproc_device: CoprocDev = CoprocDev {
    cdev: unsafe { core::mem::zeroed() },
    device: core::ptr::null_mut(),
    name: core::ptr::null_mut(),
    devt: 0,
    class: core::ptr::null_mut(),
    cop_type: unsafe { core::mem::zeroed() },
    vops: core::ptr::null(),
};

unsafe extern "C" {
    fn kasprintf(flags: gfp_t, fmt: *const i8, ...) -> *mut i8;
    fn dev_name(dev: *const device) -> *const i8;
    fn get_task_pid(task: *mut task_struct, ty: i32) -> *mut pid;
    fn get_task_mm(task: *mut task_struct) -> *mut mm_struct;
    fn put_pid(pid: *mut pid);
    fn mmgrab(mm: *mut mm_struct);
    fn mmput(mm: *mut mm_struct);
    fn find_get_pid(nr: i32) -> *mut pid;
    fn get_pid_task(pid: *mut pid, ty: i32) -> *mut task_struct;
    fn put_task_struct(task: *mut task_struct);
    fn copy_to_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> usize;
    fn copy_from_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> usize;
    fn kthread_use_mm(mm: *mut mm_struct);
    fn kthread_unuse_mm(mm: *mut mm_struct);
    fn clear_siginfo(info: *mut kernel_siginfo);
    fn kill_pid_info(sig: i32, info: *mut kernel_siginfo, pid: *mut pid) -> i32;
    fn pid_vnr(pid: *mut pid) -> i32;
    fn memset(dst: *mut core::ffi::c_void, value: i32, n: usize) -> *mut core::ffi::c_void;
    fn kzalloc(size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn mutex_init(lock: *mut mutex);
    fn get_user(dst: *mut u32, src: *const u32) -> i32;
    fn user_mode(regs: *mut pt_regs) -> bool;
    fn regs_add_return_ip(regs: *mut pt_regs, offset: u64);
    fn vmf_insert_pfn(vma: *mut vm_area_struct, addr: u64, pfn: u64) -> vm_fault_t;
    fn remap_pfn_range(vma: *mut vm_area_struct, addr: u64, pfn: u64, size: u64, prot: pgprot_t) -> i32;
    fn alloc_chrdev_region(dev: *mut dev_t, firstminor: u32, count: u32, name: *const i8) -> i32;
    fn class_create(name: *const i8) -> *mut class;
    fn cdev_init(cdev: *mut cdev, fops: *mut file_operations);
    fn cdev_add(cdev: *mut cdev, dev: dev_t, count: u32) -> i32;
    fn device_create(class: *mut class, parent: *mut device, dev: dev_t, drvdata: *mut core::ffi::c_void, name: *const i8, ...) -> *mut device;
    fn cdev_del(cdev: *mut cdev);
    fn class_destroy(class: *mut class);
    fn unregister_chrdev_region(dev: dev_t, count: u32);
    fn device_destroy(class: *mut class, dev: dev_t);
}

unsafe extern "C" {
    static mut current: *mut task_struct;
}

unsafe fn coproc_devnode(dev: *const device, _mode: *mut umode_t) -> *mut i8 {
    kasprintf(GFP_KERNEL, c"crypto/%s".as_ptr(), dev_name(dev))
}

#[no_mangle]
pub unsafe extern "C" fn get_vas_user_win_ref(task_ref: *mut vas_user_win_ref) -> i32 {
    (*task_ref).pid = get_task_pid(current, PIDTYPE_PID);
    (*task_ref).mm = get_task_mm(current);
    if (*task_ref).mm.is_null() {
        put_pid((*task_ref).pid);
        pr_err!("pid({}): mm_struct is not found\\n", (*current).pid);
        return -EPERM;
    }
    mmgrab((*task_ref).mm);
    mmput((*task_ref).mm);
    (*task_ref).tgid = find_get_pid(task_tgid_vnr(current));
    0
}

unsafe fn ref_get_pid_and_task(task_ref: *mut vas_user_win_ref, tskp: *mut *mut task_struct, pidp: *mut *mut pid) -> bool {
    let mut pid = (*task_ref).pid;
    let mut tsk = get_pid_task(pid, PIDTYPE_PID);
    if tsk.is_null() {
        pid = (*task_ref).tgid;
        tsk = get_pid_task(pid, PIDTYPE_PID);
        if WARN_ON_ONCE!(tsk.is_null()) { return false; }
    }
    if (*tsk).flags & PF_EXITING != 0 {
        put_task_struct(tsk);
        return false;
    }
    *tskp = tsk;
    *pidp = pid;
    true
}

#[no_mangle]
pub unsafe extern "C" fn vas_update_csb(crb: *mut coprocessor_request_block, task_ref: *mut vas_user_win_ref) {
    let mut csb: coprocessor_status_block = core::mem::zeroed();
    let mut info: kernel_siginfo = core::mem::zeroed();
    let mut tsk: *mut task_struct = core::ptr::null_mut();
    let csb_addr: *mut core::ffi::c_void;
    let mut pid: *mut pid = core::ptr::null_mut();
    let mut rc: i32;
    if WARN_ON_ONCE!((*task_ref).mm.is_null()) { return; }
    csb_addr = be64_to_cpu((*crb).csb_addr) as *mut core::ffi::c_void;
    memset(&mut csb as *mut _ as *mut _, 0, core::mem::size_of::<coprocessor_status_block>());
    csb.cc = CSB_CC_FAULT_ADDRESS; csb.ce = CSB_CE_TERMINATION; csb.cs = 0; csb.count = 0;
    csb.address = (*crb).stamp.nx.fault_storage_addr; csb.flags = 0;
    if !ref_get_pid_and_task(task_ref, &mut tsk, &mut pid) { return; }
    kthread_use_mm((*task_ref).mm);
    rc = copy_to_user(csb_addr, &csb as *const _ as *const _, core::mem::size_of::<coprocessor_status_block>()) as i32;
    if rc == 0 { csb.flags = CSB_V; smp_mb!(); rc = copy_to_user(csb_addr, &csb as *const _ as *const _, core::mem::size_of::<u8>()) as i32; }
    kthread_unuse_mm((*task_ref).mm); put_task_struct(tsk);
    if rc == 0 { return; }
    pr_debug!("Invalid CSB address 0x%p signalling pid({})\\n", csb_addr, pid_vnr(pid));
    clear_siginfo(&mut info); info.si_signo = SIGSEGV; info.si_errno = EFAULT; info.si_code = SEGV_MAPERR; info.si_addr = csb_addr;
    rcu_read_lock!(); rc = kill_pid_info(SIGSEGV, &mut info, pid); rcu_read_unlock!();
    pr_devel!("pid {} kill_proc_info() rc {}\\n", pid_vnr(pid), rc);
}

unsafe fn vas_dump_crb(crb: *mut coprocessor_request_block) {
    let mut dde = &mut (*crb).source as *mut data_descriptor_entry;
    pr_devel!("SrcDDE: addr 0x{:x}, len {}, count {}, idx {}, flags {}\\n", be64_to_cpu((*dde).address), be32_to_cpu((*dde).length), (*dde).count, (*dde).index, (*dde).flags);
    dde = &mut (*crb).target; pr_devel!("TgtDDE: addr 0x{:x}, len {}, count {}, idx {}, flags {}\\n", be64_to_cpu((*dde).address), be32_to_cpu((*dde).length), (*dde).count, (*dde).index, (*dde).flags);
    let nx = &(*crb).stamp.nx; pr_devel!("NX Stamp: PSWID 0x{:x}, FSA 0x{:x}, flags 0x{:x}, FS 0x{:x}\\n", be32_to_cpu(nx.pswid), be64_to_cpu((*crb).stamp.nx.fault_storage_addr), nx.flags, nx.fault_status);
}

unsafe fn coproc_open(inode: *mut inode, fp: *mut file) -> i32 {
    let cp = kzalloc(core::mem::size_of::<CoprocInstance>(), GFP_KERNEL) as *mut CoprocInstance;
    if cp.is_null() { return -ENOMEM; }
    (*cp).coproc = container_of!((*inode).i_cdev, CoprocDev, cdev);
    (*fp).private_data = cp as *mut core::ffi::c_void; 0
}

unsafe fn coproc_ioc_tx_win_open(fp: *mut file, arg: usize) -> i32 {
    let cp = (*fp).private_data as *mut CoprocInstance; let mut a: vas_tx_win_open_attr = core::mem::zeroed();
    if !(*cp).txwin.is_null() { return -EEXIST; }
    if copy_from_user(&mut a as *mut _ as *mut _, arg as *const _, core::mem::size_of_val(&a)) != 0 { pr_err!("copy_from_user() returns 1\\n"); return -EFAULT; }
    if a.version != 1 { pr_err!("Invalid window open API version\\n"); return -EINVAL; }
    let ops = (*cp).coproc; if (*ops).vops.is_null() || (*(*ops).vops).open_win.is_none() { pr_err!("VAS API is not registered\\n"); return -EACCES; }
    let win = ((*(*ops).vops).open_win.unwrap())(a.vas_id, a.flags, (*ops).cop_type);
    if IS_ERR!(win) { return PTR_ERR!(win); }
    mutex_init(&mut (*win).task_ref.mmap_mutex); (*cp).txwin = win; 0
}

unsafe fn coproc_release(_inode: *mut inode, fp: *mut file) -> i32 {
    let cp = (*fp).private_data as *mut CoprocInstance;
    if !(*cp).txwin.is_null() && !(*(*cp).coproc).vops.is_null() && (*(*(*cp).coproc).vops).close_win.is_some() {
        let rc = ((*(*(*cp).coproc).vops).close_win.unwrap())((*cp).txwin); if rc != 0 { return rc; }
        (*cp).txwin = core::ptr::null_mut();
    }
    kfree(cp as *mut _); (*fp).private_data = core::ptr::null_mut(); 0
}

unsafe fn do_fail_paste() -> i32 {
    let regs = (*current).thread.regs; if WARN_ON_ONCE!(regs.is_null()) || WARN_ON_ONCE!(!user_mode(regs)) { return -EINVAL; }
    let mut inst: u32 = 0; if get_user(&mut inst, (*regs).nip as *const u32) != 0 { return -EAGAIN; }
    if inst & PPC_INST_PASTE_MASK != PPC_INST_PASTE { return -ENOENT; }
    (*regs).ccr &= !0xe0000000; regs_add_return_ip(regs, 4); 0
}

unsafe fn vas_mmap_close(vma: *mut vm_area_struct) {
    let cp = (*(*vma).vm_file).private_data as *mut CoprocInstance;
    if cp.is_null() || (*cp).txwin.is_null() { pr_err!("No attached VAS window for the paste address mmap\\n"); return; }
    let win = (*cp).txwin; if WARN_ON!((*win).task_ref.vma != vma) { pr_err!("Invalid paste address mmaping\\n"); return; }
    (*win).task_ref.vma = core::ptr::null_mut();
}

unsafe fn vas_mmap_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let vma = (*vmf).vma; let cp = (*(*vma).vm_file).private_data as *mut CoprocInstance;
    if cp.is_null() || (*cp).txwin.is_null() { return VM_FAULT_SIGBUS; }
    let win = (*cp).txwin; if (*win).task_ref.vma != vma { return VM_FAULT_SIGBUS; }
    if (*win).status == VAS_WIN_ACTIVE {
        let ops = (*cp).coproc; if !(*ops).vops.is_null() && (*(*ops).vops).paste_addr.is_some() {
            let addr = ((*(*ops).vops).paste_addr.unwrap())(win); if addr != 0 { return vmf_insert_pfn(vma, (*vma).vm_start, addr >> PAGE_SHIFT); }
        }
    }
    let ret = do_fail_paste(); if ret == 0 || ret == -EAGAIN { VM_FAULT_NOPAGE } else { VM_FAULT_SIGBUS }
}

unsafe fn coproc_mmap(fp: *mut file, vma: *mut vm_area_struct) -> i32 {
    let cp = (*fp).private_data as *mut CoprocInstance; if cp.is_null() || (*cp).txwin.is_null() { return -EINVAL; }
    let win = (*cp).txwin; if (*vma).vm_end - (*vma).vm_start > PAGE_SIZE || (*vma).vm_pgoff != 0 { return -EINVAL; }
    if (*win).status != VAS_WIN_ACTIVE { return -EACCES; }
    let ops = (*cp).coproc; if (*ops).vops.is_null() || (*(*ops).vops).paste_addr.is_none() { return -EACCES; }
    let addr = ((*(*ops).vops).paste_addr.unwrap())(win); if addr == 0 { return -EINVAL; }
    vm_flags_set!((*vma), VM_IO | VM_PFNMAP); (*win).task_ref.vma = vma; remap_pfn_range(vma, (*vma).vm_start, addr >> PAGE_SHIFT, (*vma).vm_end - (*vma).vm_start, (*vma).vm_page_prot)
}

unsafe fn coproc_ioctl(fp: *mut file, cmd: u32, arg: usize) -> i64 { match cmd { VAS_TX_WIN_OPEN => coproc_ioc_tx_win_open(fp, arg) as i64, _ => -EINVAL as i64 } }

static mut coproc_fops: file_operations = file_operations { open: Some(coproc_open), release: Some(coproc_release), mmap: Some(coproc_mmap), unlocked_ioctl: Some(coproc_ioctl), ..unsafe { core::mem::zeroed() } };

#[no_mangle]
pub unsafe extern "C" fn vas_register_coproc_api(mod_: *mut module, cop_type: vas_cop_type, name: *const i8, vops: *const vas_user_win_ops) -> i32 {
    let mut rc = alloc_chrdev_region(&mut coproc_device.devt, 1, 1, name); if rc != 0 { return rc; }
    coproc_device.class = class_create(name); if IS_ERR!(coproc_device.class) { rc = PTR_ERR!(coproc_device.class); goto_err_class!(); }
    coproc_device.cop_type = cop_type; coproc_device.vops = vops; coproc_fops.owner = mod_; cdev_init(&mut coproc_device.cdev, &mut coproc_fops);
    let devno = MKDEV!(MAJOR!(coproc_device.devt), 0); rc = cdev_add(&mut coproc_device.cdev, devno, 1); if rc != 0 { class_destroy(coproc_device.class); unregister_chrdev_region(coproc_device.devt, 1); return rc; }
    coproc_device.device = device_create(coproc_device.class, core::ptr::null_mut(), devno, core::ptr::null_mut(), name, MINOR!(devno));
    if IS_ERR!(coproc_device.device) { rc = PTR_ERR!(coproc_device.device); cdev_del(&mut coproc_device.cdev); class_destroy(coproc_device.class); unregister_chrdev_region(coproc_device.devt, 1); return rc; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn vas_unregister_coproc_api() {
    cdev_del(&mut coproc_device.cdev); let devno = MKDEV!(MAJOR!(coproc_device.devt), 0);
    device_destroy(coproc_device.class, devno); class_destroy(coproc_device.class); unregister_chrdev_region(coproc_device.devt, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
