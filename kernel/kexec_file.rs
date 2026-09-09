// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of kexec_file.c. Kernel-provided declarations are external. */

#[cfg(feature = "kexec_sig")]
static mut SIG_ENFORCE: bool = cfg!(feature = "kexec_sig_force");

#[cfg(feature = "kexec_sig")]
pub unsafe fn set_kexec_sig_enforced() { SIG_ENFORCE = true; }

unsafe fn check_ima_segment_index(image: *mut kimage, i: i32) -> bool {
    #[cfg(feature = "ima_kexec")]
    { return (*image).is_ima_segment_index_set && i == (*image).ima_segment_index; }
    #[cfg(not(feature = "ima_kexec"))]
    { let _ = (image, i); false }
}

unsafe extern "C" {
    static mut kexec_file_loaders: *const *const kexec_file_ops;
    static mut kexec_file_dbg_print: bool;
    static mut kexec_image: *mut kimage;
    static mut kexec_crash_image: *mut kimage;
    static kexec_purgatory: *const u8;
    static kexec_purgatory_size: isize;
}

unsafe fn kexec_calculate_store_digests(image: *mut kimage) -> i32 {
    if !IS_ENABLED(CONFIG_ARCH_SUPPORTS_KEXEC_PURGATORY) { return 0; }
    let mut digest = [0u8; SHA256_DIGEST_SIZE]; let mut ctx = sha256_ctx_zero();
    let regions = vzalloc(KEXEC_SEGMENT_MAX * core::mem::size_of::<kexec_sha_region>()); if regions.is_null() { return -ENOMEM; }
    sha256_init(&mut ctx); let mut j = 0usize;
    if (*image).type_ != KEXEC_TYPE_CRASH && (kho_is_enabled() || kexec_only_cma_segments(image)) { /* skip checksum */ }
    else { for i in 0..(*image).nr_segments { if check_ima_segment_index(image, i as i32) { continue; } let s = &(*image).segment[i]; sha256_update(&mut ctx, s.kbuf, s.bufsz); (*regions.add(j)).start = s.mem; (*regions.add(j)).len = s.memsz; j += 1; } }
    sha256_final(&mut ctx, digest.as_mut_ptr()); let mut ret = kexec_purgatory_get_set_symbol(image, c"purgatory_sha_regions".as_ptr(), regions, KEXEC_SEGMENT_MAX * core::mem::size_of::<kexec_sha_region>(), false);
    if ret == 0 { ret = kexec_purgatory_get_set_symbol(image, c"purgatory_sha256_digest".as_ptr(), digest.as_mut_ptr() as *mut _, SHA256_DIGEST_SIZE, false); } vfree(regions); ret
}

unsafe fn kexec_only_cma_segments(image: *mut kimage) -> bool { for i in 0..(*image).nr_segments { if (*image).segment_cma[i].is_null() { return false; } } true }

pub unsafe fn kexec_load_purgatory(image: *mut kimage, kbuf: *mut kexec_buf) -> i32 {
    let pi = &mut (*image).purgatory_info; if kexec_purgatory_size <= 0 { return -EINVAL; }
    pi.ehdr = kexec_purgatory as *const Elf_Ehdr; let ret = kexec_purgatory_setup_kbuf(pi, kbuf); if ret != 0 { return ret; }
    let ret = kexec_purgatory_setup_sechdrs(pi, kbuf); if ret != 0 { vfree(pi.purgatory_buf); pi.purgatory_buf = core::ptr::null_mut(); return ret; }
    let ret = kexec_apply_relocations(image); if ret != 0 { vfree(pi.sechdrs); pi.sechdrs = core::ptr::null_mut(); vfree(pi.purgatory_buf); pi.purgatory_buf = core::ptr::null_mut(); } ret
}

pub unsafe fn kexec_purgatory_get_symbol_addr(image: *mut kimage, name: *const i8) -> *mut core::ffi::c_void {
    let pi = &mut (*image).purgatory_info; let sym = kexec_purgatory_find_symbol(pi, name); if sym.is_null() { return ERR_PTR(-EINVAL); }
    ((*pi).sechdrs.add((*sym).st_shndx as usize).as_ref().unwrap().sh_addr + (*sym).st_value) as *mut core::ffi::c_void
}

pub unsafe fn kexec_purgatory_get_set_symbol(image: *mut kimage, name: *const i8, buf: *mut core::ffi::c_void, size: usize, get: bool) -> i32 {
    let pi = &mut (*image).purgatory_info; let sym = kexec_purgatory_find_symbol(pi, name); if sym.is_null() || (*sym).st_size != size { return -EINVAL; }
    let sec = pi.sechdrs.add((*sym).st_shndx as usize); if (*sec).sh_type == SHT_NOBITS { return -EINVAL; }
    let p = pi.purgatory_buf.add((*sec).sh_offset as usize + (*sym).st_value as usize); if get { memcpy(buf, p, size); } else { memcpy(p, buf, size); } 0
}

unsafe fn kexec_image_probe_default(image: *mut kimage, buf: *mut core::ffi::c_void, len: usize) -> i32 {
    let mut fops = kexec_file_loaders;
    let mut ret = -ENOEXEC;
    while !fops.is_null() && !(*fops).probe.is_none() {
        ret = ((*fops).probe.unwrap())(buf, len);
        if ret == 0 { (*image).fops = fops; return ret; }
        fops = fops.add(1);
    }
    ret
}

unsafe fn kexec_image_load_default(image: *mut kimage) -> *mut core::ffi::c_void {
    if (*image).fops.is_null() || (*(*image).fops).load.is_none() { return ERR_PTR(-ENOEXEC); }
    ((*(*image).fops).load.unwrap())(image, (*image).kernel_buf, (*image).kernel_buf_len,
        (*image).initrd_buf, (*image).initrd_buf_len, (*image).cmdline_buf, (*image).cmdline_buf_len)
}

pub unsafe fn kexec_image_post_load_cleanup_default(image: *mut kimage) -> i32 {
    if (*image).fops.is_null() || (*(*image).fops).cleanup.is_none() { return 0; }
    ((*(*image).fops).cleanup.unwrap())((*image).image_loader_data)
}

pub unsafe fn kimage_file_post_load_cleanup(image: *mut kimage) {
    let pi = &mut (*image).purgatory_info;
    vfree((*image).kernel_buf); (*image).kernel_buf = core::ptr::null_mut();
    vfree((*image).initrd_buf); (*image).initrd_buf = core::ptr::null_mut();
    kfree((*image).cmdline_buf); (*image).cmdline_buf = core::ptr::null_mut();
    vfree(pi.purgatory_buf); pi.purgatory_buf = core::ptr::null_mut();
    vfree(pi.sechdrs); pi.sechdrs = core::ptr::null_mut();
    #[cfg(feature = "ima_kexec")] { vfree((*image).ima_buffer); (*image).ima_buffer = core::ptr::null_mut(); }
    arch_kimage_file_post_load_cleanup(image);
    kfree((*image).image_loader_data); (*image).image_loader_data = core::ptr::null_mut();
    kexec_file_dbg_print = false;
}

#[cfg(feature = "kexec_sig")]
unsafe fn kexec_image_verify_sig(image: *mut kimage, buf: *mut core::ffi::c_void, len: usize) -> i32 {
    if (*image).fops.is_null() || (*(*image).fops).verify_sig.is_none() { pr_debug!("kernel loader does not support signature verification.\n"); return -EKEYREJECTED; }
    ((*(*image).fops).verify_sig.unwrap())(buf, len)
}

#[cfg(feature = "kexec_sig")]
unsafe fn kimage_validate_signature(image: *mut kimage) -> i32 {
    let ret = kexec_image_verify_sig(image, (*image).kernel_buf, (*image).kernel_buf_len);
    if ret != 0 {
        if SIG_ENFORCE { pr_notice!("Enforced kernel signature verification failed (%d).\n", ret); return ret; }
        if !ima_appraise_signature(READING_KEXEC_IMAGE) && security_locked_down(LOCKDOWN_KEXEC) { return -EPERM; }
        pr_debug!("kernel signature verification failed (%d).\n", ret);
    }
    0
}

unsafe fn kexec_post_load(image: *mut kimage, flags: usize) -> i32 {
    #[cfg(feature = "ima_kexec")] if flags & KEXEC_FILE_ON_CRASH == 0 { ima_kexec_post_load(image); }
    machine_kexec_post_load(image)
}

unsafe fn kimage_file_prepare_segments(image: *mut kimage, kernel_fd: i32, initrd_fd: i32,
    cmdline_ptr: *const u8, cmdline_len: usize, flags: usize) -> i32 {
    let mut ret = kernel_read_file_from_fd(kernel_fd, 0, &mut (*image).kernel_buf, KEXEC_FILE_SIZE_MAX, core::ptr::null_mut(), READING_KEXEC_IMAGE);
    if ret < 0 { return ret as i32; } (*image).kernel_buf_len = ret as usize;
    ret = arch_kexec_kernel_image_probe(image, (*image).kernel_buf, (*image).kernel_buf_len); if ret != 0 { return cleanup_prepare(image, ret); }
    #[cfg(feature = "kexec_sig")] { ret = kimage_validate_signature(image); if ret != 0 { return cleanup_prepare(image, ret); } }
    if flags & KEXEC_FILE_NO_INITRAMFS == 0 {
        ret = kernel_read_file_from_fd(initrd_fd, 0, &mut (*image).initrd_buf, KEXEC_FILE_SIZE_MAX, core::ptr::null_mut(), READING_KEXEC_INITRAMFS);
        if ret < 0 { return cleanup_prepare(image, ret); } (*image).initrd_buf_len = ret as usize;
    }
    (*image).no_cma = flags & KEXEC_FILE_NO_CMA != 0; (*image).force_dtb = flags & KEXEC_FILE_FORCE_DTB;
    if cmdline_len != 0 {
        (*image).cmdline_buf = memdup_user(cmdline_ptr, cmdline_len);
        if IS_ERR((*image).cmdline_buf) { ret = PTR_ERR((*image).cmdline_buf); (*image).cmdline_buf = core::ptr::null_mut(); return cleanup_prepare(image, ret); }
        (*image).cmdline_buf_len = cmdline_len;
        if *(*image).cmdline_buf.add(cmdline_len - 1) != 0 { return cleanup_prepare(image, -EINVAL); }
        ima_kexec_cmdline(kernel_fd, (*image).cmdline_buf, cmdline_len - 1);
    }
    ima_add_kexec_buffer(image); ret = kho_fill_kimage(image); if ret != 0 { return cleanup_prepare(image, ret); }
    let data = kexec_image_load_default(image); if IS_ERR(data) { return cleanup_prepare(image, PTR_ERR(data)); }
    (*image).image_loader_data = data; 0
}

unsafe fn cleanup_prepare(image: *mut kimage, ret: i64) -> i32 { kimage_file_post_load_cleanup(image); ret as i32 }

unsafe fn locate_mem_hole_top_down(start: usize, end: usize, kbuf: *mut kexec_buf) -> i32 {
    let image = (*kbuf).image; let mut ts = core::cmp::min(end, (*kbuf).buf_max); let mut te = ts - (*kbuf).memsz + 1;
    kexec_random_range_start(ts, te, kbuf, &mut ts);
    loop { ts = align_down(ts, (*kbuf).buf_align); if ts < start || ts < (*kbuf).buf_min { return 0; } te = ts + (*kbuf).memsz - 1;
        if kimage_is_destination_range(image, ts, te) || arch_check_excluded_range(image, ts, te) { ts -= PAGE_SIZE; continue; }
        (*kbuf).mem = ts; return 1;
    }
}

unsafe fn locate_mem_hole_bottom_up(start: usize, end: usize, kbuf: *mut kexec_buf) -> i32 {
    let image = (*kbuf).image; let mut ts = core::cmp::max(start, (*kbuf).buf_min); let mut te;
    kexec_random_range_start(ts, end, kbuf, &mut ts);
    loop { ts = align(ts, (*kbuf).buf_align); te = ts + (*kbuf).memsz - 1; if te > end || te > (*kbuf).buf_max { return 0; }
        if kimage_is_destination_range(image, ts, te) || arch_check_excluded_range(image, ts, te) { ts += PAGE_SIZE; continue; }
        (*kbuf).mem = ts; return 1;
    }
}

unsafe fn locate_mem_hole_callback(res: *mut resource, arg: *mut core::ffi::c_void) -> i32 {
    let kbuf = arg as *mut kexec_buf; let start = (*res).start; let end = (*res).end; let sz = end - start + 1;
    if (*res).flags & IORESOURCE_SYSRAM_DRIVER_MANAGED != 0 || sz < (*kbuf).memsz || end < (*kbuf).buf_min || start > (*kbuf).buf_max { return 0; }
    if (*kbuf).top_down { locate_mem_hole_top_down(start, end, kbuf) } else { locate_mem_hole_bottom_up(start, end, kbuf) }
}

pub unsafe fn kexec_locate_mem_hole(kbuf: *mut kexec_buf) -> i32 {
    if (*kbuf).mem != KEXEC_BUF_MEM_UNKNOWN { return 0; }
    let mut ret = kho_locate_mem_hole(kbuf, locate_mem_hole_callback); if ret <= 0 { return ret; }
    if kexec_alloc_contig(kbuf) == 0 { return 0; }
    ret = kexec_walk_resources(kbuf, locate_mem_hole_callback); if ret == 1 { 0 } else { -EADDRNOTAVAIL }
}

pub unsafe fn kexec_add_buffer(kbuf: *mut kexec_buf) -> i32 {
    if !(*(*kbuf).image).file_mode || (*(*kbuf).image).nr_segments >= KEXEC_SEGMENT_MAX || !list_empty(&(*(*kbuf).image).control_pages) { WARN_ON(1); return -EINVAL; }
    (*kbuf).memsz = align((*kbuf).memsz, PAGE_SIZE); (*kbuf).buf_align = core::cmp::max((*kbuf).buf_align, PAGE_SIZE); (*kbuf).cma = core::ptr::null_mut();
    let ret = arch_kexec_locate_mem_hole(kbuf); if ret != 0 { return ret; }
    let n = (*(*kbuf).image).nr_segments; let seg = &mut (*(*kbuf).image).segment[n]; seg.kbuf = (*kbuf).buffer; seg.bufsz = (*kbuf).bufsz; seg.mem = (*kbuf).mem; seg.memsz = (*kbuf).memsz;
    (*(*kbuf).image).segment_cma[n] = (*kbuf).cma; (*(*kbuf).image).nr_segments += 1; 0
}

// The remaining ELF/purgatory helpers retain the kernel's ABI and are declared externally;
// their definitions are supplied by the translated companion units.
unsafe extern "C" {
    fn kexec_alloc_contig(kbuf: *mut kexec_buf) -> i32;
    fn kexec_walk_resources(kbuf: *mut kexec_buf, f: unsafe fn(*mut resource,*mut core::ffi::c_void)->i32) -> i32;
    fn arch_kexec_locate_mem_hole(kbuf: *mut kexec_buf) -> i32;
    fn kho_locate_mem_hole(kbuf: *mut kexec_buf, f: unsafe fn(*mut resource,*mut core::ffi::c_void)->i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
