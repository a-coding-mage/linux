// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AMD Node helper functions and common defines
 *
 * Copyright (c) 2024, Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Author: Yazen Ghannam <Yazen.Ghannam@amd.com>
 */

// C dependencies supplied by the kernel are intentionally external here.

const SMN_INDEX_OFFSET: u8 = 0x60;
const SMN_DATA_OFFSET: u8 = 0x64;
const HSMP_INDEX_OFFSET: u8 = 0xc4;
const HSMP_DATA_OFFSET: u8 = 0xc8;

extern "C" {
    fn pci_get_domain_bus_and_slot(domain: u32, bus: u32, devfn: u32) -> *mut pci_dev;
    fn amd_num_nodes() -> u16;
    fn pci_write_config_dword(dev: *mut pci_dev, where_: u8, value: u32) -> i32;
    fn pci_read_config_dword(dev: *mut pci_dev, where_: u8, value: *mut u32) -> i32;
    fn pcibios_err_to_errno(err: i32) -> i32;
    fn kstrtou16_from_user(buf: *const core::ffi::c_char, count: usize, base: u32, res: *mut u16) -> i32;
    fn kstrtouint_from_user(buf: *const core::ffi::c_char, count: usize, base: u32, res: *mut u32) -> i32;
    fn seq_printf(m: *mut seq_file, fmt: *const core::ffi::c_char, ...) -> i32;
    fn amd_smn_read(node: u16, address: u32, value: *mut u32) -> i32;
    fn amd_smn_write(node: u16, address: u32, value: u32) -> i32;
    fn add_taint(taint: u32, lockdep_ok: u32);
    fn pci_get_class(class: u32, from: *mut pci_dev) -> *mut pci_dev;
    fn pci_request_config_region_exclusive(dev: *mut pci_dev, from: u32, size: u32, name: *const core::ffi::c_char) -> i32;
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn kzalloc_objs<T>(obj: T, count: u16) -> *mut *mut pci_dev;
    fn debugfs_create_dir(name: *const core::ffi::c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const core::ffi::c_char, mode: u32, parent: *mut dentry, data: *mut core::ffi::c_void, fops: *const core::ffi::c_void) -> *mut dentry;
}

#[repr(C)] pub struct pci_dev { pub devfn: u8, pub vendor: u16 }
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct file;
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct mutex;

static mut amd_roots: *mut *mut pci_dev = core::ptr::null_mut();
static mut smn_mutex: mutex = mutex { };
static mut smn_exclusive: bool = false;
static mut debugfs_dir: *mut dentry = core::ptr::null_mut();
static mut debug_node: u16 = 0;
static mut debug_address: u32 = 0;
static mut enable_dfs: bool = false;

pub unsafe fn amd_node_get_func(node: u16, func: u8) -> *mut pci_dev {
    if node >= MAX_AMD_NUM_NODES { return core::ptr::null_mut(); }
    pci_get_domain_bus_and_slot(0, 0, PCI_DEVFN(AMD_NODE0_PCI_SLOT + node, func))
}

unsafe fn __amd_smn_rw(i_off: u8, d_off: u8, node: u16, address: u32, value: *mut u32, write: bool) -> i32 {
    let mut err = -ENODEV;
    if node >= amd_num_nodes() { return err; }
    let root = *amd_roots.add(node as usize);
    if root.is_null() || !smn_exclusive { return err; }
    // guard(mutex)(&smn_mutex);
    err = pci_write_config_dword(root, i_off, address);
    if err != 0 { return pcibios_err_to_errno(err); }
    err = if write { pci_write_config_dword(root, d_off, *value) } else { pci_read_config_dword(root, d_off, value) };
    pcibios_err_to_errno(err)
}

pub unsafe fn amd_smn_read(node: u16, address: u32, value: *mut u32) -> i32 {
    let mut err = __amd_smn_rw(SMN_INDEX_OFFSET, SMN_DATA_OFFSET, node, address, value, false);
    if PCI_POSSIBLE_ERROR(*value) { err = -ENODEV; *value = 0; }
    err
}

pub unsafe fn amd_smn_write(node: u16, address: u32, mut value: u32) -> i32 {
    __amd_smn_rw(SMN_INDEX_OFFSET, SMN_DATA_OFFSET, node, address, &mut value, true)
}

pub unsafe fn amd_smn_hsmp_rdwr(node: u16, address: u32, value: *mut u32, write: bool) -> i32 {
    __amd_smn_rw(HSMP_INDEX_OFFSET, HSMP_DATA_OFFSET, node, address, value, write)
}

unsafe fn smn_node_write(_file: *mut file, userbuf: *const core::ffi::c_char, count: usize, _ppos: *mut i64) -> isize {
    let mut node = 0u16; let ret = kstrtou16_from_user(userbuf, count, 0, &mut node);
    if ret != 0 { return ret as isize; } if node >= amd_num_nodes() { return -ENODEV as isize; }
    debug_node = node; count as isize
}
unsafe fn smn_node_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { seq_printf(m, c"0x%08x\n".as_ptr(), debug_node as u32); 0 }
unsafe fn smn_address_write(_file: *mut file, userbuf: *const core::ffi::c_char, count: usize, _ppos: *mut i64) -> isize {
    let ret = kstrtouint_from_user(userbuf, count, 0, &mut debug_address); if ret != 0 { return ret as isize; } count as isize
}
unsafe fn smn_address_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { seq_printf(m, c"0x%08x\n".as_ptr(), debug_address); 0 }
unsafe fn smn_value_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { let mut val=0; let ret=amd_smn_read(debug_node,debug_address,&mut val); if ret!=0{return ret;} seq_printf(m,c"0x%08x\n".as_ptr(),val); 0 }
unsafe fn smn_value_write(_file:*mut file,userbuf:*const core::ffi::c_char,count:usize,_ppos:*mut i64)->isize { let mut val=0; let ret=kstrtouint_from_user(userbuf,count,0,&mut val); if ret!=0{return ret as isize;} add_taint(TAINT_CPU_OUT_OF_SPEC,LOCKDEP_STILL_OK); let ret=amd_smn_write(debug_node,debug_address,val); if ret!=0{return ret as isize;} count as isize }

unsafe fn get_next_root(mut root: *mut pci_dev) -> *mut pci_dev { loop { root=pci_get_class(PCI_CLASS_BRIDGE_HOST<<8,root); if root.is_null(){return root;} if (*root).devfn!=0 || ((*root).vendor!=PCI_VENDOR_ID_AMD && (*root).vendor!=PCI_VENDOR_ID_HYGON){continue;} return root; } }
unsafe fn amd_smn_enable_dfs(_str:*mut core::ffi::c_char)->i32 { enable_dfs=true; 1 }

unsafe fn amd_smn_init() -> i32 {
    if !cpu_feature_enabled(X86_FEATURE_ZEN) { return 0; }
    if !amd_roots.is_null() { return 0; }
    let mut num_roots=0u16; let mut root=core::ptr::null_mut();
    while { root=get_next_root(root); !root.is_null() } { num_roots+=1; }
    if num_roots==0{return -ENODEV;}
    let num_nodes=amd_num_nodes(); amd_roots=kzalloc_objs(core::ptr::null_mut(),num_nodes); if amd_roots.is_null(){return -ENOMEM;}
    let roots_per_node=num_roots/num_nodes; let mut count=0u16; let mut node=0u16; root=core::ptr::null_mut();
    while node<num_nodes { root=get_next_root(root); if root.is_null(){break;} count+=1; if count%roots_per_node!=0{continue;} *amd_roots.add(node as usize)=root; node+=1; }
    if enable_dfs { debugfs_dir=debugfs_create_dir(c"amd_smn".as_ptr(), core::ptr::null_mut()); }
    smn_exclusive=true; 0
}

// Kernel registration: __setup("amd_smn_debugfs_enable", amd_smn_enable_dfs);
// Kernel registration: fs_initcall(amd_smn_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
