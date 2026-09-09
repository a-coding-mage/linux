/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from xen-ops.h. C header dependencies are supplied externally. */

extern "C" {
    pub static mut xen_vcpu: *mut vcpu_info;
    pub static mut xen_vcpu_id: u32;

    pub fn xen_arch_pre_suspend();
    pub fn xen_arch_post_suspend(suspend_cancelled: c_int);

    pub fn xen_timer_resume();
    pub fn xen_arch_resume();
    pub fn xen_arch_suspend();

    pub fn xen_reboot(reason: c_int);

    pub fn xen_resume_notifier_register(nb: *mut notifier_block);

    pub fn xen_vcpu_stolen(vcpu: c_int) -> bool;
    pub fn xen_setup_runstate_info(cpu: c_int);
    pub fn xen_time_setup_guest();
    pub fn xen_manage_runstate_time(action: c_int);
    pub fn xen_steal_clock(cpu: c_int) -> u64;

    pub fn xen_setup_shutdown_event() -> c_int;

    pub static mut xen_contiguous_bitmap: *mut c_ulong;

    pub fn xen_xlate_remap_gfn_array(
        vma: *mut vm_area_struct,
        addr: c_ulong,
        gfn: *mut xen_pfn_t,
        nr: c_int,
        err_ptr: *mut c_int,
        prot: pgprot_t,
        domid: c_uint,
        pages: *mut *mut page,
    ) -> c_int;
    pub fn xen_xlate_unmap_gfn_range(
        vma: *mut vm_area_struct,
        nr: c_int,
        pages: *mut *mut page,
    ) -> c_int;

    pub fn xen_remap_vma_range(
        vma: *mut vm_area_struct,
        addr: c_ulong,
        len: c_ulong,
    ) -> c_int;

    pub fn xen_unmap_domain_gfn_range(
        vma: *mut vm_area_struct,
        numpgs: c_int,
        pages: *mut *mut page,
    ) -> c_int;

    pub fn xen_xlate_map_ballooned_pages(
        pfns: *mut *mut xen_pfn_t,
        vaddr: *mut *mut c_void,
        nr_grant_frames: c_ulong,
    ) -> c_int;

    pub fn xen_running_on_version_or_later(major: c_uint, minor: c_uint) -> bool;
    pub fn xen_efi_runtime_setup();
}

pub const XEN_VCPU_ID_INVALID: u32 = u32::MAX;

#[inline]
pub unsafe fn xen_vcpu_nr(cpu: c_int) -> u32 {
    // Equivalent to per_cpu(xen_vcpu_id, cpu); the per-CPU implementation is external.
    per_cpu_xen_vcpu_id(cpu)
}

extern "C" {
    fn per_cpu_xen_vcpu_id(cpu: c_int) -> u32;
}

#[cfg(feature = "CONFIG_XEN_PV")]
extern "C" {
    pub fn xen_remap_pfn(
        vma: *mut vm_area_struct,
        addr: c_ulong,
        pfn: *mut xen_pfn_t,
        nr: c_int,
        err_ptr: *mut c_int,
        prot: pgprot_t,
        domid: c_uint,
        no_translate: bool,
    ) -> c_int;
}

#[cfg(not(feature = "CONFIG_XEN_PV"))]
#[inline]
pub unsafe fn xen_remap_pfn(
    _vma: *mut vm_area_struct,
    _addr: c_ulong,
    _pfn: *mut xen_pfn_t,
    _nr: c_int,
    _err_ptr: *mut c_int,
    _prot: pgprot_t,
    _domid: c_uint,
    _no_translate: bool,
) -> c_int {
    unsafe { BUG() };
    0
}

#[inline]
pub unsafe fn xen_remap_domain_gfn_array(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    gfn: *mut xen_pfn_t,
    nr: c_int,
    err_ptr: *mut c_int,
    prot: pgprot_t,
    domid: c_uint,
    pages: *mut *mut page,
) -> c_int {
    if !unsafe { xen_pv_domain() } {
        return xen_xlate_remap_gfn_array(vma, addr, gfn, nr, err_ptr, prot, domid, pages);
    }
    unsafe { BUG_ON(err_ptr.is_null()) };
    xen_remap_pfn(vma, addr, gfn, nr, err_ptr, prot, domid, false)
}

#[inline]
pub unsafe fn xen_remap_domain_mfn_array(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    mfn: *mut xen_pfn_t,
    nr: c_int,
    err_ptr: *mut c_int,
    prot: pgprot_t,
    domid: c_uint,
) -> c_int {
    if !unsafe { xen_pv_domain() } { return -EOPNOTSUPP; }
    xen_remap_pfn(vma, addr, mfn, nr, err_ptr, prot, domid, true)
}

#[inline]
pub unsafe fn xen_remap_domain_gfn_range(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    mut gfn: xen_pfn_t,
    nr: c_int,
    prot: pgprot_t,
    domid: c_uint,
    _pages: *mut *mut page,
) -> c_int {
    if !unsafe { xen_pv_domain() } { return -EOPNOTSUPP; }
    xen_remap_pfn(vma, addr, &mut gfn, nr, core::ptr::null_mut(), prot, domid, false)
}

#[cfg(all(feature = "CONFIG_XEN_PV", not(feature = "CONFIG_PREEMPTION")))]
extern "C" {
    pub static mut xen_in_preemptible_hcall: bool;
}

#[inline]
pub unsafe fn xen_preemptible_hcall_begin() {
    #[cfg(all(feature = "CONFIG_XEN_PV", not(feature = "CONFIG_PREEMPTION")))]
    { xen_in_preemptible_hcall = true; }
}

#[inline]
pub unsafe fn xen_preemptible_hcall_end() {
    #[cfg(all(feature = "CONFIG_XEN_PV", not(feature = "CONFIG_PREEMPTION")))]
    { xen_in_preemptible_hcall = false; }
}

#[cfg(feature = "CONFIG_XEN_GRANT_DMA_OPS")]
extern "C" { pub fn xen_virtio_restricted_mem_acc(dev: *mut virtio_device) -> bool; }

#[cfg(not(feature = "CONFIG_XEN_GRANT_DMA_OPS"))]
#[inline]
pub unsafe fn xen_virtio_restricted_mem_acc(_dev: *mut virtio_device) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
