// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Kernel and driver dependencies supplied by other translation units.

const ADF_VINTSOU_OFFSET: u32 = 0x204;
const ADF_VINTMSK_OFFSET: u32 = 0x208;
const ADF_VINTSOU_BUN: u32 = 1 << 0;
const ADF_VINTSOU_PF2VF: u32 = 1 << 1;

static mut adf_vf_stop_wq: *mut workqueue_struct = core::ptr::null_mut();

#[repr(C)]
struct adf_vf_stop_data {
    accel_dev: *mut adf_accel_dev,
    work: work_struct,
}

extern "C" {
    type adf_accel_dev;
    type workqueue_struct;
    type work_struct;
    type pci_dev;
    type adf_hw_device_data;
    type adf_hw_csr_ops;
    type adf_bar;
    type adf_etr_data;
    type adf_etr_bank_data;

    fn adf_get_pmisc_base(accel_dev: *mut adf_accel_dev) -> *mut core::ffi::c_void;
    fn adf_dev_restarting_notify(accel_dev: *mut adf_accel_dev);
    fn adf_dev_down(accel_dev: *mut adf_accel_dev);
    fn adf_vf2pf_notify_restart_complete(accel_dev: *mut adf_accel_dev);
    fn adf_recv_and_handle_pf2vf_msg(accel_dev: *mut adf_accel_dev) -> bool;
    fn adf_response_handler(data: usize);
    fn accel_to_pci_dev(accel_dev: *mut adf_accel_dev) -> *mut pci_dev;
    fn pci_alloc_irq_vectors(dev: *mut pci_dev, min: i32, max: i32, flags: u32) -> i32;
    fn pci_free_irq_vectors(dev: *mut pci_dev);
    fn request_irq(irq: u32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
                   flags: u32, name: *const i8, dev: *mut core::ffi::c_void) -> i32;
    fn free_irq(irq: u32, dev: *mut core::ffi::c_void);
    fn irq_set_affinity_hint(irq: u32, mask: *const core::ffi::c_void);
    fn get_cpu_mask(cpu: u32) -> *const core::ffi::c_void;
    fn num_online_cpus() -> u32;
    fn tasklet_init(tasklet: *mut core::ffi::c_void, func: *mut core::ffi::c_void, data: usize);
    fn tasklet_hi_schedule(tasklet: *mut core::ffi::c_void);
    fn tasklet_disable(tasklet: *mut core::ffi::c_void);
    fn tasklet_kill(tasklet: *mut core::ffi::c_void);
    fn mutex_init(lock: *mut core::ffi::c_void);
    fn mutex_destroy(lock: *mut core::ffi::c_void);
    fn flush_workqueue(wq: *mut workqueue_struct);
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn alloc_workqueue(name: *const i8, flags: u32, max_active: u32) -> *mut workqueue_struct;
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn kfree(ptr: *mut core::ffi::c_void);
}

type irqreturn_t = i32;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_NONE: irqreturn_t = 0;
const EFAULT: i32 = 14;
const ENOMEM: i32 = 12;
const PCI_IRQ_MSI: u32 = 1 << 1;
const GFP_ATOMIC: u32 = 0;
const WQ_MEM_RECLAIM: u32 = 1 << 5;
const WQ_PERCPU: u32 = 1 << 8;

unsafe fn adf_enable_pf2vf_interrupts(accel_dev: *mut adf_accel_dev) {
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    ADF_CSR_WR(pmisc_addr, ADF_VINTMSK_OFFSET, 0x0);
}

unsafe fn adf_disable_pf2vf_interrupts(accel_dev: *mut adf_accel_dev) {
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    ADF_CSR_WR(pmisc_addr, ADF_VINTMSK_OFFSET, 0x2);
}

unsafe fn adf_enable_msi(accel_dev: *mut adf_accel_dev) -> i32 {
    let pci_dev_info = &mut (*accel_dev).accel_pci_dev;
    let stat = pci_alloc_irq_vectors(pci_dev_info.pci_dev, 1, 1, PCI_IRQ_MSI);
    if stat < 0 { return stat; }
    0
}

unsafe fn adf_disable_msi(accel_dev: *mut adf_accel_dev) {
    pci_free_irq_vectors(accel_to_pci_dev(accel_dev));
}

unsafe extern "C" fn adf_dev_stop_async(work: *mut work_struct) {
    let stop_data = container_of!(work, adf_vf_stop_data, work);
    let accel_dev = (*stop_data).accel_dev;
    adf_dev_restarting_notify(accel_dev);
    adf_dev_down(accel_dev);
    adf_enable_pf2vf_interrupts(accel_dev);
    adf_vf2pf_notify_restart_complete(accel_dev);
    kfree(stop_data.cast());
}

unsafe fn adf_pf2vf_handle_pf_restarting(accel_dev: *mut adf_accel_dev) -> i32 {
    clear_bit!(ADF_STATUS_PF_RUNNING, &mut (*accel_dev).status);
    let stop_data = kzalloc_obj!(adf_vf_stop_data, GFP_ATOMIC);
    if stop_data.is_null() { return -ENOMEM; }
    (*stop_data).accel_dev = accel_dev;
    INIT_WORK!(&mut (*stop_data).work, adf_dev_stop_async);
    queue_work(adf_vf_stop_wq, &mut (*stop_data).work);
    0
}

unsafe extern "C" fn adf_pf2vf_bh_handler(data: *mut core::ffi::c_void) {
    let accel_dev = data.cast::<adf_accel_dev>();
    if adf_recv_and_handle_pf2vf_msg(accel_dev) { adf_enable_pf2vf_interrupts(accel_dev); }
}

unsafe fn adf_setup_pf2vf_bh(accel_dev: *mut adf_accel_dev) -> i32 {
    tasklet_init(&mut (*accel_dev).vf.pf2vf_bh_tasklet, adf_pf2vf_bh_handler as *mut _, accel_dev as usize);
    mutex_init(&mut (*accel_dev).vf.vf2pf_lock);
    0
}

unsafe fn adf_cleanup_pf2vf_bh(accel_dev: *mut adf_accel_dev) {
    tasklet_disable(&mut (*accel_dev).vf.pf2vf_bh_tasklet);
    tasklet_kill(&mut (*accel_dev).vf.pf2vf_bh_tasklet);
    mutex_destroy(&mut (*accel_dev).vf.vf2pf_lock);
}

unsafe extern "C" fn adf_isr(_irq: i32, privdata: *mut core::ffi::c_void) -> irqreturn_t {
    let accel_dev = privdata.cast::<adf_accel_dev>();
    let hw_data = (*accel_dev).hw_device;
    let csr_ops = &mut (*hw_data).csr_ops;
    let pmisc = &mut (*accel_dev).bars[((*hw_data).get_misc_bar_id)(hw_data) as usize];
    let pmisc_bar_addr = pmisc.virt_addr;
    let mut v_int = ADF_CSR_RD(pmisc_bar_addr, ADF_VINTSOU_OFFSET);
    let v_mask = ADF_CSR_RD(pmisc_bar_addr, ADF_VINTMSK_OFFSET);
    v_int &= !v_mask;
    let mut handled = false;
    if v_int & ADF_VINTSOU_PF2VF != 0 {
        adf_disable_pf2vf_interrupts(accel_dev);
        tasklet_hi_schedule(&mut (*accel_dev).vf.pf2vf_bh_tasklet);
        handled = true;
    }
    if v_int & ADF_VINTSOU_BUN != 0 {
        let etr_data = (*accel_dev).transport;
        let bank = &mut (*etr_data).banks[0];
        ((*csr_ops).write_csr_int_flag_and_col)(bank.csr_addr, bank.bank_number, 0);
        tasklet_hi_schedule(&mut bank.resp_handler);
        handled = true;
    }
    if handled { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe fn adf_request_msi_irq(accel_dev: *mut adf_accel_dev) -> i32 {
    let pdev = accel_to_pci_dev(accel_dev);
    let cpu = (*accel_dev).accel_id % num_online_cpus();
    let ret = request_irq((*pdev).irq, adf_isr, 0, (*accel_dev).vf.irq_name.as_ptr(), accel_dev.cast());
    if ret != 0 { return ret; }
    irq_set_affinity_hint((*pdev).irq, get_cpu_mask(cpu));
    (*accel_dev).vf.irq_enabled = true;
    ret
}

unsafe fn adf_setup_bh(accel_dev: *mut adf_accel_dev) -> i32 {
    let priv_data = (*accel_dev).transport;
    tasklet_init(&mut (*priv_data).banks[0].resp_handler, adf_response_handler as *mut _, (*priv_data).banks.as_mut_ptr() as usize);
    0
}

unsafe fn adf_cleanup_bh(accel_dev: *mut adf_accel_dev) {
    let priv_data = (*accel_dev).transport;
    tasklet_disable(&mut (*priv_data).banks[0].resp_handler);
    tasklet_kill(&mut (*priv_data).banks[0].resp_handler);
}

// The remaining resource-allocation and workqueue entry points preserve the C API.
pub unsafe fn adf_vf_isr_resource_free(accel_dev: *mut adf_accel_dev) {
    let pdev = accel_to_pci_dev(accel_dev);
    if (*accel_dev).vf.irq_enabled {
        irq_set_affinity_hint((*pdev).irq, core::ptr::null());
        free_irq((*pdev).irq, accel_dev.cast());
    }
    adf_cleanup_bh(accel_dev);
    adf_cleanup_pf2vf_bh(accel_dev);
    adf_disable_msi(accel_dev);
}

pub unsafe fn adf_vf_isr_resource_alloc(accel_dev: *mut adf_accel_dev) -> i32 {
    if adf_enable_msi(accel_dev) != 0 { return -EFAULT; }
    if adf_setup_pf2vf_bh(accel_dev) != 0 { adf_disable_msi(accel_dev); return -EFAULT; }
    if adf_setup_bh(accel_dev) != 0 { adf_cleanup_pf2vf_bh(accel_dev); adf_disable_msi(accel_dev); return -EFAULT; }
    if adf_request_msi_irq(accel_dev) != 0 { adf_cleanup_bh(accel_dev); adf_cleanup_pf2vf_bh(accel_dev); adf_disable_msi(accel_dev); return -EFAULT; }
    0
}

pub unsafe fn adf_flush_vf_wq(accel_dev: *mut adf_accel_dev) {
    adf_disable_pf2vf_interrupts(accel_dev);
    flush_workqueue(adf_vf_stop_wq);
}

pub unsafe fn adf_init_vf_wq() -> i32 {
    adf_vf_stop_wq = alloc_workqueue(b"adf_vf_stop_wq\0".as_ptr() as *const i8, WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if adf_vf_stop_wq.is_null() { -EFAULT } else { 0 }
}

pub unsafe fn adf_exit_vf_wq() {
    if !adf_vf_stop_wq.is_null() { destroy_workqueue(adf_vf_stop_wq); }
    adf_vf_stop_wq = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
