// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux and Xen interfaces, and by xen-ops.h,
// remain external to this translation unit.

#[cfg(CONFIG_PROC_VMCORE)]
/*
 * The kdump kernel has to check whether a pfn of the crashed kernel
 * was a ballooned page. vmcore is using this function to decide
 * whether to access a pfn of the crashed kernel.
 * Returns "false" if the pfn is not backed by a RAM page, the caller may
 * handle the pfn special in this case.
 */
unsafe fn xen_vmcore_pfn_is_ram(cb: *mut vmcore_cb, pfn: ::core::ffi::c_ulong) -> bool {
    let mut a = xen_hvm_get_mem_type {
        domid: DOMID_SELF,
        pfn,
        ..::core::mem::zeroed()
    };

    if HYPERVISOR_hvm_op(HVMOP_get_mem_type, &mut a) != 0 {
        pr_warn_once!("Unexpected HVMOP_get_mem_type failure\n");
        return true;
    }
    a.mem_type != HVMMEM_mmio_dm
}

#[cfg(CONFIG_PROC_VMCORE)]
static mut xen_vmcore_cb: vmcore_cb = vmcore_cb {
    pfn_is_ram: Some(xen_vmcore_pfn_is_ram),
    ..unsafe { ::core::mem::zeroed() }
};

unsafe fn xen_hvm_exit_mmap(mm: *mut mm_struct) {
    let mut a: xen_hvm_pagetable_dying = ::core::mem::zeroed();
    let rc: ::core::ffi::c_int;

    a.domid = DOMID_SELF;
    a.gpa = __pa((*mm).pgd);
    rc = HYPERVISOR_hvm_op(HVMOP_pagetable_dying, &mut a);
    WARN_ON_ONCE!(rc < 0);
}

unsafe fn is_pagetable_dying_supported() -> ::core::ffi::c_int {
    let mut a: xen_hvm_pagetable_dying = ::core::mem::zeroed();
    let mut rc: ::core::ffi::c_int = 0;

    a.domid = DOMID_SELF;
    a.gpa = 0x00;
    rc = HYPERVISOR_hvm_op(HVMOP_pagetable_dying, &mut a);
    if rc < 0 {
        printk!(KERN_DEBUG "HVMOP_pagetable_dying not supported\n");
        return 0;
    }
    1
}

pub unsafe fn xen_hvm_init_mmu_ops() {
    if is_pagetable_dying_supported() != 0 {
        pv_ops.mmu.exit_mmap = Some(xen_hvm_exit_mmap);
    }
    #[cfg(CONFIG_PROC_VMCORE)]
    {
        register_vmcore_cb(&mut xen_vmcore_cb);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
