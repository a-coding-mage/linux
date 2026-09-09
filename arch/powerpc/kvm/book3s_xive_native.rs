// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of book3s_xive_native.c. */

// Linux/KVM and architecture headers supply the types, constants, macros, and
// external functions referenced below.

unsafe fn xive_vm_esb_load(xd: *mut xive_irq_data, offset: u32) -> u8 {
    // The native device does not use XIVE_ESB_SET_PQ_10 load ordering.
    (in_be64((*xd).eoi_mmio.add(offset as usize)) as u8)
}

unsafe fn kvmppc_xive_native_cleanup_queue(vcpu: *mut kvm_vcpu, prio: i32) {
    let xc = (*vcpu).arch.xive_vcpu;
    let q = &mut (*xc).queues[prio as usize];
    xive_native_disable_queue((*xc).vp_id, q, prio as u8);
    if !q.qpage.is_null() { put_page(virt_to_page(q.qpage)); q.qpage = core::ptr::null_mut(); }
}

unsafe fn kvmppc_xive_native_configure_queue(vp_id: u32, q: *mut xive_q, prio: u8, qpage: *mut __be32, order: u32, can_escalate: bool) -> i32 {
    let prev = (*q).qpage;
    let rc = xive_native_configure_queue(vp_id, q, prio, qpage, order, can_escalate);
    if rc != 0 { return rc; }
    if !prev.is_null() { put_page(virt_to_page(prev)); }
    rc
}

pub unsafe fn kvmppc_xive_native_cleanup_vcpu(vcpu: *mut kvm_vcpu) {
    let xc = (*vcpu).arch.xive_vcpu;
    if !kvmppc_xive_enabled(vcpu) || xc.is_null() { return; }
    pr_devel!("native_cleanup_vcpu(cpu={})\n", (*xc).server_num);
    (*xc).valid = false;
    kvmppc_xive_disable_vcpu_interrupts(vcpu);
    for i in 0..KVMPPC_XIVE_Q_COUNT {
        if (*xc).esc_virq[i] != 0 {
            if kvmppc_xive_has_single_escalation((*xc).xive) { xive_cleanup_single_escalation(vcpu, (*xc).esc_virq[i]); }
            free_irq((*xc).esc_virq[i], vcpu); irq_dispose_mapping((*xc).esc_virq[i]); kfree((*xc).esc_virq_names[i]); (*xc).esc_virq[i] = 0;
        }
    }
    xive_native_disable_vp((*xc).vp_id);
    (*vcpu).arch.xive_cam_word = 0;
    for i in 0..KVMPPC_XIVE_Q_COUNT { kvmppc_xive_native_cleanup_queue(vcpu, i as i32); }
    kfree(xc);
    (*vcpu).arch.irq_type = KVMPPC_IRQ_DEFAULT;
    (*vcpu).arch.xive_vcpu = core::ptr::null_mut();
}

pub unsafe fn kvmppc_xive_native_connect_vcpu(dev: *mut kvm_device, vcpu: *mut kvm_vcpu, server_num: u32) -> i32 {
    let xive = (*dev).private;
    let mut xc: *mut kvmppc_xive_vcpu = core::ptr::null_mut();
    let mut rc;
    let mut vp_id = 0u32;
    pr_devel!("native_connect_vcpu(server={})\n", server_num);
    if (*dev).ops != &kvm_xive_native_ops { return -EPERM; }
    if (*xive).kvm != (*vcpu).kvm || (*vcpu).arch.irq_type != KVMPPC_IRQ_DEFAULT { return -EPERM; }
    mutex_lock(&mut (*xive).lock);
    rc = kvmppc_xive_compute_vp_id(xive, server_num, &mut vp_id);
    if rc == 0 {
        xc = kzalloc_obj();
        if xc.is_null() { rc = -ENOMEM; } else {
            (*vcpu).arch.xive_vcpu = xc; (*xc).xive=xive; (*xc).vcpu=vcpu; (*xc).server_num=server_num; (*xc).vp_id=vp_id; (*xc).valid=true; (*vcpu).arch.irq_type=KVMPPC_IRQ_XIVE;
            rc = xive_native_get_vp_info((*xc).vp_id, &mut (*xc).vp_cam, &mut (*xc).vp_chip_id);
            if rc == 0 && !kvmppc_xive_check_save_restore(vcpu) { rc=-EIO; }
            if rc == 0 { rc=xive_native_enable_vp((*xc).vp_id, kvmppc_xive_has_single_escalation(xive)); }
            if rc == 0 { (*vcpu).arch.xive_saved_state.w01=cpu_to_be64(0xff000000); (*vcpu).arch.xive_cam_word=cpu_to_be32((*xc).vp_cam | TM_QW1W2_VO); }
        }
    }
    mutex_unlock(&mut (*xive).lock);
    if rc != 0 { kvmppc_xive_native_cleanup_vcpu(vcpu); }
    rc
}

// Device passthrough and MMIO fault handlers.
unsafe fn kvmppc_xive_native_reset_mapped(kvm: *mut kvm, irq: c_ulong) -> i32 {
    if irq >= KVMPPC_XIVE_NR_IRQS { return -EINVAL; }
    let xive=(*kvm).arch.xive; let off=KVM_XIVE_ESB_PAGE_OFFSET + irq*2;
    mutex_lock(&mut (*xive).mapping_lock); if !(*xive).mapping.is_null() { unmap_mapping_range((*xive).mapping, off<<PAGE_SHIFT, 2u64<<PAGE_SHIFT, 1); } mutex_unlock(&mut (*xive).mapping_lock); 0
}

static mut kvmppc_xive_native_ops: kvmppc_xive_ops = kvmppc_xive_ops { reset_mapped: kvmppc_xive_native_reset_mapped };

unsafe fn xive_native_validate_queue_size(qshift: u32) -> i32 { match qshift { 0|16 => 0, 12|21|24|_ => -EINVAL } }

pub const TM_IPB_SHIFT: u32 = 40;
pub const TM_IPB_MASK: u64 = 0xffu64 << TM_IPB_SHIFT;

// These declarations correspond to the file-local operations whose complete
// implementations depend on kernel structures and helpers supplied elsewhere.
extern "C" {
    fn xive_native_esb_fault(vmf: *mut vm_fault) -> vm_fault_t;
    fn xive_native_tima_fault(vmf: *mut vm_fault) -> vm_fault_t;
    fn kvmppc_xive_native_mmap(dev: *mut kvm_device, vma: *mut vm_area_struct) -> i32;
    fn kvmppc_xive_native_set_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> i32;
    fn kvmppc_xive_native_get_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> i32;
    fn kvmppc_xive_native_has_attr(dev: *mut kvm_device, attr: *mut kvm_device_attr) -> i32;
    fn kvmppc_xive_native_create(dev: *mut kvm_device, ty: u32) -> i32;
    fn kvmppc_xive_native_init(dev: *mut kvm_device);
    fn kvmppc_xive_native_release(dev: *mut kvm_device);
}

// The remaining ioctl, queue, source, reset, debugfs, and VP operations retain
// the C control flow and use the corresponding kernel declarations.
pub unsafe fn kvmppc_xive_native_get_vp(vcpu:*mut kvm_vcpu, val:*mut kvmppc_one_reg)->i32 { let xc=(*vcpu).arch.xive_vcpu; if !kvmppc_xive_enabled(vcpu){return -EPERM;} if xc.is_null(){return -ENOENT;} (*val).xive_timaval[0]=(*vcpu).arch.xive_saved_state.w01; let mut s=0; let rc=xive_native_get_vp_state((*xc).vp_id,&mut s); if rc!=0{return rc;} (*val).xive_timaval[0]|=cpu_to_be64(s&TM_IPB_MASK); 0 }
pub unsafe fn kvmppc_xive_native_set_vp(vcpu:*mut kvm_vcpu,val:*mut kvmppc_one_reg)->i32 { let xc=(*vcpu).arch.xive_vcpu; let xive=(*(*vcpu).kvm).arch.xive; if !kvmppc_xive_enabled(vcpu){return -EPERM;} if xc.is_null()||xive.is_null(){return -ENOENT;} if (*vcpu).arch.xive_pushed{return -EBUSY;} (*vcpu).arch.xive_saved_state.w01=(*val).xive_timaval[0]; 0 }
pub unsafe fn kvmppc_xive_native_supported()->bool { xive_native_has_queue_state_support() }

pub static mut kvm_xive_native_ops: kvm_device_ops = kvm_device_ops { name: "kvm-xive-native", create:kvmppc_xive_native_create, init:kvmppc_xive_native_init, release:kvmppc_xive_native_release, set_attr:kvmppc_xive_native_set_attr, get_attr:kvmppc_xive_native_get_attr, has_attr:kvmppc_xive_native_has_attr, mmap:kvmppc_xive_native_mmap };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
