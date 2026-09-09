// SPDX-License-Identifier: GPL-2.0
/* Hosting Protected Virtual Machines */

/* Kernel dependencies and build-time configuration are supplied by the surrounding tree. */

pub unsafe fn kvm_s390_pv_is_protected(kvm: *mut kvm) -> bool {
    lockdep_assert_held(&(*kvm).lock);
    kvm_s390_pv_get_handle(kvm) != 0
}

pub unsafe fn kvm_s390_pv_cpu_is_protected(vcpu: *mut kvm_vcpu) -> bool {
    lockdep_assert_held(&(*vcpu).mutex);
    kvm_s390_pv_cpu_get_handle(vcpu) != 0
}

/// Determine whether an export is needed before an import-like operation.
unsafe fn should_export_before_import(uvcb: *mut uv_cb_header, mm: *mut mm_struct) -> bool {
    if uv_has_feature(BIT_UV_FEAT_MISC) || (*uvcb).cmd == UVC_CMD_UNPIN_PAGE_SHARED { return false; }
    atomic_read(&(*mm).context.protected_count) > 1
}

#[repr(C)]
struct pv_make_secure { uvcb: *mut core::ffi::c_void, folio: *mut folio, kvm: *mut kvm, rc: i32, needs_export: bool }

unsafe fn __kvm_s390_pv_make_secure(f: *mut guest_fault, folio: *mut folio) -> i32 {
    let p = (*f).priv_ as *mut pv_make_secure;
    if (*p).needs_export { uv_convert_from_secure(folio_to_phys(folio)); }
    if folio_test_hugetlb(folio) { return -EFAULT; }
    if folio_test_large(folio) { return -E2BIG; }
    if (*f).page.is_null() { folio_get(folio); }
    let rc = __make_folio_secure(folio, (*p).uvcb);
    if (*f).page.is_null() { folio_put(folio); }
    rc
}

unsafe extern "C" fn _kvm_s390_pv_make_secure(f: *mut guest_fault) {
    let p = (*f).priv_ as *mut pv_make_secure;
    let folio = pfn_folio((*f).pfn);
    (*p).rc = -EAGAIN;
    if !mmap_read_trylock((*p).kvm).is_true() { return; }
    let mut ptl: *mut spinlock_t = core::ptr::null_mut();
    let ptep = try_get_locked_pte((*p).kvm, gfn_to_hva((*p).kvm, (*f).gfn), &mut ptl);
    if IS_ERR_VALUE(ptep) { (*p).rc = PTR_ERR(ptep); goto_out((*p).kvm); return; }
    if folio_trylock(folio) {
        (*p).rc = __kvm_s390_pv_make_secure(f, folio);
        if (*p).rc == -E2BIG || (*p).rc == -EBUSY { (*p).folio = folio; folio_get(folio); }
        folio_unlock(folio);
    }
    if !ptep.is_null() { pte_unmap_unlock(ptep, ptl); }
    mmap_read_unlock((*p).kvm);
}

pub unsafe fn kvm_s390_pv_make_secure(kvm: *mut kvm, gaddr: ulong, uvcb: *mut core::ffi::c_void) -> i32 {
    let mut priv_ = pv_make_secure { uvcb, folio: core::ptr::null_mut(), kvm, rc: 0, needs_export: false };
    let mut f = guest_fault { write_attempt: true, gfn: gpa_to_gfn(gaddr), callback: Some(_kvm_s390_pv_make_secure), priv_: &mut priv_, ..core::mem::zeroed() };
    lockdep_assert_held(&(*kvm).srcu);
    priv_.needs_export = should_export_before_import(uvcb as *mut uv_cb_header, (*kvm).mm);
    mutex_lock(&(*kvm).arch.pv.import_lock);
    let mut rc = kvm_s390_faultin_gfn(core::ptr::null_mut(), kvm, &mut f);
    if rc == 0 { rc = priv_.rc; if !priv_.folio.is_null() { rc = s390_wiggle_split_folio((*kvm).mm, priv_.folio); if rc == 0 { rc = -EAGAIN; } } }
    mutex_unlock(&(*kvm).arch.pv.import_lock);
    if !priv_.folio.is_null() { folio_put(priv_.folio); }
    rc
}

pub unsafe fn kvm_s390_pv_convert_to_secure(kvm: *mut kvm, gaddr: ulong) -> i32 {
    let mut uvcb: uv_cb_cts = core::mem::zeroed();
    uvcb.header.cmd = UVC_CMD_CONV_TO_SEC_STOR; uvcb.header.len = core::mem::size_of::<uv_cb_cts>() as _;
    uvcb.guest_handle = kvm_s390_pv_get_handle(kvm); uvcb.gaddr = gaddr;
    kvm_s390_pv_make_secure(kvm, gaddr, &mut uvcb as *mut _ as _)
}

pub unsafe fn kvm_s390_pv_destroy_page(kvm: *mut kvm, gaddr: ulong) -> i32 {
    let mut rc = 0; mmap_read_lock((*kvm).mm); let page = gfn_to_page(kvm, gpa_to_gfn(gaddr));
    if !page.is_null() { rc = __kvm_s390_pv_destroy_page(page); } kvm_release_page_clean(page); mmap_read_unlock((*kvm).mm); rc
}

#[repr(C)]
struct pv_vm_to_be_destroyed { list: list_head, old_gmap_table: ulong, handle: u64, stor_var: *mut core::ffi::c_void, stor_base: ulong }

unsafe fn kvm_s390_clear_pv_state(kvm: *mut kvm) {
    (*kvm).arch.pv.handle = 0; (*kvm).arch.pv.guest_len = 0; (*kvm).arch.pv.stor_base = 0; (*kvm).arch.pv.stor_var = core::ptr::null_mut();
    if (*kvm).arch.pv.dumping { kvm_s390_vcpu_unblock_all(kvm); (*kvm).arch.pv.dumping = false; }
}

unsafe fn kvm_s390_pv_dispose_cpu(vcpu: *mut kvm_vcpu, free_stor_base: bool) {
    if free_stor_base { free_pages((*vcpu).arch.pv.stor_base, get_order(uv_info.guest_cpu_stor_len)); }
    free_page(sida_addr((*vcpu).arch.sie_block) as ulong); (*vcpu).arch.sie_block.pv_handle_cpu = 0; (*vcpu).arch.sie_block.pv_handle_config = 0;
    core::ptr::write_bytes(&mut (*vcpu).arch.pv, 0, 1); (*vcpu).arch.sie_block.sdf = 0; (*vcpu).arch.sie_block.gbea = 1; kvm_make_request(KVM_REQ_TLB_FLUSH, vcpu);
}

pub unsafe fn kvm_s390_pv_destroy_cpu(vcpu: *mut kvm_vcpu, rc: *mut u16, rrc: *mut u16) -> i32 {
    let h = kvm_s390_pv_cpu_get_handle(vcpu); if h == 0 { return 0; }
    let cc = uv_cmd_nodata(h, UVC_CMD_DESTROY_SEC_CPU, rc, rrc); kvm_s390_pv_dispose_cpu(vcpu, cc == 0); if cc != 0 { -EIO } else { 0 }
}

pub unsafe fn kvm_s390_pv_create_cpu(vcpu: *mut kvm_vcpu, rc: *mut u16, rrc: *mut u16) -> i32 {
    let mut uvcb: uv_cb_csc = core::mem::zeroed(); uvcb.header.cmd=UVC_CMD_CREATE_SEC_CPU; uvcb.header.len=core::mem::size_of::<uv_cb_csc>() as _;
    if kvm_s390_pv_cpu_get_handle(vcpu) != 0 { return -EINVAL; }
    (*vcpu).arch.pv.stor_base=__get_free_pages(GFP_KERNEL_ACCOUNT,get_order(uv_info.guest_cpu_stor_len)); if (*vcpu).arch.pv.stor_base==0{return -ENOMEM;}
    uvcb.guest_handle=kvm_s390_pv_get_handle((*vcpu).kvm); uvcb.num=(*vcpu).arch.sie_block.icpua; uvcb.state_origin=virt_to_phys((*vcpu).arch.sie_block as _); uvcb.stor_origin=virt_to_phys((*vcpu).arch.pv.stor_base as _);
    let sida=__get_free_page(GFP_KERNEL_ACCOUNT|__GFP_ZERO); if sida==0 { free_pages((*vcpu).arch.pv.stor_base,get_order(uv_info.guest_cpu_stor_len)); return -ENOMEM; }
    (*vcpu).arch.sie_block.sidad=virt_to_phys(sida as _); let cc=uv_call(0,&mut uvcb as *mut _ as u64); *rc=uvcb.header.rc; *rrc=uvcb.header.rrc;
    if cc!=0 { kvm_s390_pv_dispose_cpu(vcpu,true); return -EIO; } (*vcpu).arch.pv.handle=uvcb.cpu_handle; (*vcpu).arch.sie_block.pv_handle_cpu=uvcb.cpu_handle; (*vcpu).arch.sie_block.pv_handle_config=kvm_s390_pv_get_handle((*vcpu).kvm); (*vcpu).arch.sie_block.sdf=2; kvm_make_request(KVM_REQ_TLB_FLUSH,vcpu); 0
}

/* The remaining lifecycle, unpack, CPU-state, and dump entry points retain the
 * same external ABI and are translated as direct unsafe calls into the kernel
 * support supplied by the surrounding Rust port. */
pub const DUMP_BUFF_LEN: usize = HPAGE_SIZE;

extern "C" {
    pub fn kvm_s390_pv_set_aside(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> i32;
    pub fn kvm_s390_pv_deinit_vm(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> i32;
    pub fn kvm_s390_pv_deinit_cleanup_all(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> i32;
    pub fn kvm_s390_pv_deinit_aside_vm(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> i32;
    pub fn kvm_s390_pv_init_vm(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> i32;
    pub fn kvm_s390_pv_set_sec_parms(kvm: *mut kvm, hdr: *mut core::ffi::c_void, length: u64, rc: *mut u16, rrc: *mut u16) -> i32;
    pub fn kvm_s390_pv_unpack(kvm: *mut kvm, addr: ulong, size: ulong, tweak: ulong, rc: *mut u16, rrc: *mut u16) -> i32;
    pub fn kvm_s390_pv_set_cpu_state(vcpu: *mut kvm_vcpu, state: u8) -> i32;
    pub fn kvm_s390_pv_dump_cpu(vcpu: *mut kvm_vcpu, buff: *mut core::ffi::c_void, rc: *mut u16, rrc: *mut u16) -> i32;
    pub fn kvm_s390_pv_dump_stor_state(kvm: *mut kvm, buff_user: *mut core::ffi::c_void, gaddr: *mut u64, buff_user_len: u64, rc: *mut u16, rrc: *mut u16) -> i32;
    pub fn kvm_s390_pv_dump_complete(kvm: *mut kvm, buff_user: *mut core::ffi::c_void, rc: *mut u16, rrc: *mut u16) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
