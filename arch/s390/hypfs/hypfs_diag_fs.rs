// SPDX-License-Identifier: GPL-2.0
/*
 *    Hypervisor filesystem for Linux on s390. Diag 204 and 224
 *    implementation.
 *
 *    Copyright IBM Corp. 2006, 2008
 *    Author(s): Michael Holzheu <holzheu@de.ibm.com>
 */

// pr_fmt(fmt) = "hypfs: " fmt
// C dependencies are supplied by the surrounding kernel translation.

const TMP_SIZE: usize = 64;

static mut diag224_cpu_names: *mut i8 = core::ptr::null_mut();

unsafe fn info_blk_hdr__size(type_: diag204_format) -> usize {
    if type_ == DIAG204_INFO_SIMPLE { core::mem::size_of::<diag204_info_blk_hdr>() }
    else { core::mem::size_of::<diag204_x_info_blk_hdr>() }
}

unsafe fn info_blk_hdr__npar(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u8 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_info_blk_hdr)).npar }
    else { (*(hdr as *mut diag204_x_info_blk_hdr)).npar }
}

unsafe fn info_blk_hdr__flags(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u8 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_info_blk_hdr)).flags }
    else { (*(hdr as *mut diag204_x_info_blk_hdr)).flags }
}

unsafe fn part_hdr__size(type_: diag204_format) -> usize {
    if type_ == DIAG204_INFO_SIMPLE { core::mem::size_of::<diag204_part_hdr>() }
    else { core::mem::size_of::<diag204_x_part_hdr>() }
}

unsafe fn part_hdr__rcpus(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u8 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_part_hdr)).cpus }
    else { (*(hdr as *mut diag204_x_part_hdr)).rcpus }
}

unsafe fn part_hdr__part_name(type_: diag204_format, hdr: *mut core::ffi::c_void, name: *mut i8) {
    if type_ == DIAG204_INFO_SIMPLE {
        core::ptr::copy_nonoverlapping((*(hdr as *mut diag204_part_hdr)).part_name.as_ptr(), name, DIAG204_LPAR_NAME_LEN as usize);
    } else {
        core::ptr::copy_nonoverlapping((*(hdr as *mut diag204_x_part_hdr)).part_name.as_ptr(), name, DIAG204_LPAR_NAME_LEN as usize);
    }
    unsafe { EBCASC(name, DIAG204_LPAR_NAME_LEN); }
    *name.add(DIAG204_LPAR_NAME_LEN as usize) = 0;
    unsafe { strim(name); }
}

unsafe fn cpu_info__size(type_: diag204_format) -> usize {
    if type_ == DIAG204_INFO_SIMPLE { core::mem::size_of::<diag204_cpu_info>() }
    else { core::mem::size_of::<diag204_x_cpu_info>() }
}
unsafe fn cpu_info__ctidx(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u8 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_cpu_info)).ctidx } else { (*(hdr as *mut diag204_x_cpu_info)).ctidx }
}
unsafe fn cpu_info__cpu_addr(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u16 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_cpu_info)).cpu_addr } else { (*(hdr as *mut diag204_x_cpu_info)).cpu_addr }
}
unsafe fn cpu_info__acc_time(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u64 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_cpu_info)).acc_time } else { (*(hdr as *mut diag204_x_cpu_info)).acc_time }
}
unsafe fn cpu_info__lp_time(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u64 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_cpu_info)).lp_time } else { (*(hdr as *mut diag204_x_cpu_info)).lp_time }
}
unsafe fn cpu_info__online_time(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u64 {
    if type_ == DIAG204_INFO_SIMPLE { 0 } else { (*(hdr as *mut diag204_x_cpu_info)).online_time }
}
unsafe fn phys_hdr__size(type_: diag204_format) -> usize {
    if type_ == DIAG204_INFO_SIMPLE { core::mem::size_of::<diag204_phys_hdr>() } else { core::mem::size_of::<diag204_x_phys_hdr>() }
}
unsafe fn phys_hdr__cpus(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u8 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_phys_hdr)).cpus } else { (*(hdr as *mut diag204_x_phys_hdr)).cpus }
}
unsafe fn phys_cpu__size(type_: diag204_format) -> usize {
    if type_ == DIAG204_INFO_SIMPLE { core::mem::size_of::<diag204_phys_cpu>() } else { core::mem::size_of::<diag204_x_phys_cpu>() }
}
unsafe fn phys_cpu__cpu_addr(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u16 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_phys_cpu)).cpu_addr } else { (*(hdr as *mut diag204_x_phys_cpu)).cpu_addr }
}
unsafe fn phys_cpu__mgm_time(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u64 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_phys_cpu)).mgm_time } else { (*(hdr as *mut diag204_x_phys_cpu)).mgm_time }
}
unsafe fn phys_cpu__ctidx(type_: diag204_format, hdr: *mut core::ffi::c_void) -> u64 {
    if type_ == DIAG204_INFO_SIMPLE { (*(hdr as *mut diag204_phys_cpu)).ctidx } else { (*(hdr as *mut diag204_x_phys_cpu)).ctidx }
}

unsafe fn hypfs_create_cpu_files(cpus_dir: *mut dentry, cpu_info: *mut core::ffi::c_void) -> i32 {
    let mut buffer = [0i8; TMP_SIZE];
    snprintf(buffer.as_mut_ptr(), TMP_SIZE, b"%d\0".as_ptr() as *const i8, cpu_info__cpu_addr(diag204_get_info_type(), cpu_info));
    let cpu_dir = hypfs_mkdir(cpus_dir, buffer.as_ptr());
    if IS_ERR(cpu_dir) { return PTR_ERR(cpu_dir); }
    let ty = diag204_get_info_type();
    let mut rc = hypfs_create_u64(cpu_dir, b"mgmtime\0".as_ptr() as *const i8, cpu_info__acc_time(ty, cpu_info).wrapping_sub(cpu_info__lp_time(ty, cpu_info)));
    if rc != 0 { return rc; }
    rc = hypfs_create_u64(cpu_dir, b"cputime\0".as_ptr() as *const i8, cpu_info__lp_time(ty, cpu_info));
    if rc != 0 { return rc; }
    if ty == DIAG204_INFO_EXT {
        rc = hypfs_create_u64(cpu_dir, b"onlinetime\0".as_ptr() as *const i8, cpu_info__online_time(ty, cpu_info));
        if rc != 0 { return rc; }
    }
    diag224_idx2name(cpu_info__ctidx(ty, cpu_info) as i32, buffer.as_mut_ptr());
    hypfs_create_str(cpu_dir, b"type\0".as_ptr() as *const i8, buffer.as_ptr())
}

unsafe fn hypfs_create_lpar_files(systems_dir: *mut dentry, part_hdr: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let mut lpar_name = [0i8; (DIAG204_LPAR_NAME_LEN as usize) + 1];
    let ty = diag204_get_info_type();
    part_hdr__part_name(ty, part_hdr, lpar_name.as_mut_ptr());
    let lpar_dir = hypfs_mkdir(systems_dir, lpar_name.as_ptr());
    if IS_ERR(lpar_dir) { return lpar_dir as *mut core::ffi::c_void; }
    let cpus_dir = hypfs_mkdir(lpar_dir, b"cpus\0".as_ptr() as *const i8);
    if IS_ERR(cpus_dir) { return cpus_dir as *mut core::ffi::c_void; }
    let mut cpu_info = (part_hdr as *mut u8).add(part_hdr__size(ty)) as *mut core::ffi::c_void;
    for _ in 0..part_hdr__rcpus(ty, part_hdr) {
        let rc = hypfs_create_cpu_files(cpus_dir, cpu_info);
        if rc != 0 { return ERR_PTR(rc) as *mut core::ffi::c_void; }
        cpu_info = (cpu_info as *mut u8).add(cpu_info__size(ty)) as *mut core::ffi::c_void;
    }
    cpu_info
}

unsafe fn hypfs_create_phys_cpu_files(cpus_dir: *mut dentry, cpu_info: *mut core::ffi::c_void) -> i32 {
    let mut buffer = [0i8; TMP_SIZE];
    let ty = diag204_get_info_type();
    snprintf(buffer.as_mut_ptr(), TMP_SIZE, b"%i\0".as_ptr() as *const i8, phys_cpu__cpu_addr(ty, cpu_info));
    let cpu_dir = hypfs_mkdir(cpus_dir, buffer.as_ptr());
    if IS_ERR(cpu_dir) { return PTR_ERR(cpu_dir); }
    let rc = hypfs_create_u64(cpu_dir, b"mgmtime\0".as_ptr() as *const i8, phys_cpu__mgm_time(ty, cpu_info));
    if rc != 0 { return rc; }
    diag224_idx2name(phys_cpu__ctidx(ty, cpu_info) as i32, buffer.as_mut_ptr());
    hypfs_create_str(cpu_dir, b"type\0".as_ptr() as *const i8, buffer.as_ptr())
}

unsafe fn hypfs_create_phys_files(parent_dir: *mut dentry, phys_hdr: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let ty = diag204_get_info_type();
    let cpus_dir = hypfs_mkdir(parent_dir, b"cpus\0".as_ptr() as *const i8);
    if IS_ERR(cpus_dir) { return cpus_dir as *mut core::ffi::c_void; }
    let mut cpu_info = (phys_hdr as *mut u8).add(phys_hdr__size(ty)) as *mut core::ffi::c_void;
    for _ in 0..phys_hdr__cpus(ty, phys_hdr) {
        let rc = hypfs_create_phys_cpu_files(cpus_dir, cpu_info);
        if rc != 0 { return ERR_PTR(rc) as *mut core::ffi::c_void; }
        cpu_info = (cpu_info as *mut u8).add(phys_cpu__size(ty)) as *mut core::ffi::c_void;
    }
    cpu_info
}

pub unsafe fn hypfs_diag_create_files(root: *mut dentry) -> i32 {
    let ty = diag204_get_info_type();
    let mut pages = 0i32;
    let buffer = diag204_get_buffer(ty, &mut pages);
    if IS_ERR(buffer) { return PTR_ERR(buffer); }
    let rc = diag204_store(buffer, pages);
    if rc != 0 { return rc; }
    let systems_dir = hypfs_mkdir(root, b"systems\0".as_ptr() as *const i8);
    if IS_ERR(systems_dir) { return PTR_ERR(systems_dir); }
    let time_hdr = buffer;
    let mut part_hdr = (buffer as *mut u8).add(info_blk_hdr__size(ty)) as *mut core::ffi::c_void;
    for _ in 0..info_blk_hdr__npar(ty, time_hdr) {
        part_hdr = hypfs_create_lpar_files(systems_dir, part_hdr);
        if IS_ERR(part_hdr) { return PTR_ERR(part_hdr); }
    }
    if (info_blk_hdr__flags(ty, time_hdr) & DIAG204_LPAR_PHYS_FLG) != 0 {
        let ptr = hypfs_create_phys_files(root, part_hdr);
        if IS_ERR(ptr) { return PTR_ERR(ptr); }
    }
    let hyp_dir = hypfs_mkdir(root, b"hyp\0".as_ptr() as *const i8);
    if IS_ERR(hyp_dir) { return PTR_ERR(hyp_dir); }
    hypfs_create_str(hyp_dir, b"type\0".as_ptr() as *const i8, b"LPAR Hypervisor\0".as_ptr() as *const i8)
}

unsafe fn diag224_idx2name(index: i32, name: *mut i8) -> i32 {
    core::ptr::copy_nonoverlapping(diag224_cpu_names.add(((index + 1) as usize) * DIAG204_CPU_NAME_LEN as usize), name, DIAG204_CPU_NAME_LEN as usize);
    *name.add(DIAG204_CPU_NAME_LEN as usize) = 0;
    strim(name);
    0
}

unsafe fn diag224_get_name_table() -> i32 {
    diag224_cpu_names = __get_free_page(GFP_KERNEL | GFP_DMA) as *mut i8;
    if diag224_cpu_names.is_null() { return -ENOMEM; }
    if diag224(diag224_cpu_names) != 0 {
        free_page(diag224_cpu_names as usize);
        return -EOPNOTSUPP;
    }
    EBCASC(diag224_cpu_names.add(16), (*diag224_cpu_names.add(0) as u8 + 1) as usize * 16);
    0
}

unsafe fn diag224_delete_name_table() { free_page(diag224_cpu_names as usize); }

pub unsafe fn __hypfs_diag_fs_init() -> i32 {
    if machine_is_lpar() { diag224_get_name_table() } else { 0 }
}

pub unsafe fn __hypfs_diag_fs_exit() { diag224_delete_name_table(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
