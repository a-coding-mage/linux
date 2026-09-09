// SPDX-License-Identifier: GPL-2.0
/*
 * Provide kernel BTF information for introspection and use by eBPF tools.
 */

// Kernel headers supplying these types, constants, and operations are external
// dependencies of this translation.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut __start_BTF: c_char;
    static mut __stop_BTF: c_char;

    static mut kernel_kobj: *mut Kobject;

    fn sysfs_bin_attr_simple_read(
        file: *mut File,
        kobj: *mut Kobject,
        attr: *mut BinAttribute,
        buf: *mut c_char,
        offset: U64,
        count: usize,
    ) -> isize;
    fn kobject_create_and_add(name: *const c_char, parent: *mut Kobject) -> *mut Kobject;
    fn sysfs_create_bin_file(kobj: *mut Kobject, attr: *mut BinAttribute) -> c_int;
    fn __pa_symbol(addr: *const c_void) -> PhysAddr;
    fn remap_pfn_range(
        vma: *mut VmAreaStruct,
        start: ULong,
        pfn: ULong,
        size: usize,
        prot: VmPageProt,
    ) -> c_int;
    fn vm_flags_mod(vma: *mut VmAreaStruct, set: ULong, clear: ULong);
}

type U64 = u64;
type ULong = usize;
type PhysAddr = u64;
type VmPageProt = usize;

#[repr(C)]
pub struct File {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VmAreaStruct {
    pub vm_start: ULong,
    pub vm_end: ULong,
    pub vm_flags: ULong,
    pub vm_pgoff: ULong,
    pub vm_page_prot: VmPageProt,
}

#[repr(C)]
pub struct Attribute {
    pub name: *const c_char,
    pub mode: u16,
}

#[repr(C)]
pub struct BinAttribute {
    pub attr: Attribute,
    pub size: usize,
    pub private: *mut c_void,
    pub read: Option<unsafe extern "C" fn(*mut File, *mut Kobject, *mut BinAttribute, *mut c_char, U64, usize) -> isize>,
    pub write: Option<unsafe extern "C" fn()>,
    pub mmap: Option<unsafe extern "C" fn(*mut File, *mut Kobject, *const BinAttribute, *mut VmAreaStruct) -> c_int>,
}

const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const VM_WRITE: ULong = 0x0000_0002;
const VM_EXEC: ULong = 0x0000_0004;
const VM_MAYWRITE: ULong = 0x0000_0020;
const VM_MAYSHARE: ULong = 0x0000_0040;
const VM_MAYEXEC: ULong = 0x0000_0010;
const VM_DONTDUMP: ULong = 0x0400_0000;

const EINVAL: c_int = 22;
const EACCES: c_int = 13;
const ENOMEM: c_int = 12;

unsafe fn page_align(value: usize) -> usize {
    value.wrapping_add(PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

unsafe extern "C" fn btf_sysfs_vmlinux_mmap(
    _filp: *mut File,
    _kobj: *mut Kobject,
    attr: *const BinAttribute,
    vma: *mut VmAreaStruct,
) -> c_int {
    let pages = page_align((*attr).size) >> PAGE_SHIFT;
    let vm_size = (*vma).vm_end.wrapping_sub((*vma).vm_start);
    let addr = __pa_symbol((&raw const __start_BTF).cast());
    let pfn = addr >> PAGE_SHIFT;

    if (*attr).private != (&raw mut __start_BTF).cast() || (addr & (PAGE_SIZE as u64 - 1)) != 0 {
        return -EINVAL;
    }
    if (*vma).vm_pgoff != 0 {
        return -EINVAL;
    }
    if (*vma).vm_flags & (VM_WRITE | VM_EXEC | VM_MAYSHARE) != 0 {
        return -EACCES;
    }
    if pfn.wrapping_add(pages as u64) < pfn {
        return -EINVAL;
    }
    if (vm_size >> PAGE_SHIFT) > pages {
        return -EINVAL;
    }

    vm_flags_mod(vma, VM_DONTDUMP, VM_MAYEXEC | VM_MAYWRITE);
    remap_pfn_range(vma, (*vma).vm_start, pfn as ULong, vm_size, (*vma).vm_page_prot)
}

static mut bin_attr_btf_vmlinux: BinAttribute = BinAttribute {
    attr: Attribute { name: b"vmlinux\0".as_ptr().cast(), mode: 0o444 },
    size: 0,
    private: core::ptr::null_mut(),
    read: Some(sysfs_bin_attr_simple_read),
    write: None,
    mmap: Some(btf_sysfs_vmlinux_mmap),
};

pub static mut btf_kobj: *mut Kobject = core::ptr::null_mut();

pub unsafe extern "C" fn btf_vmlinux_init() -> c_int {
    bin_attr_btf_vmlinux.private = (&raw mut __start_BTF).cast();
    bin_attr_btf_vmlinux.size = (&raw const __stop_BTF as *const c_char as usize)
        .wrapping_sub(&raw const __start_BTF as *const c_char as usize);

    if bin_attr_btf_vmlinux.size == 0 {
        return 0;
    }

    btf_kobj = kobject_create_and_add(b"btf\0".as_ptr().cast(), kernel_kobj);
    if btf_kobj.is_null() {
        return -ENOMEM;
    }

    sysfs_create_bin_file(btf_kobj, &raw mut bin_attr_btf_vmlinux)
}

// Equivalent to: subsys_initcall(btf_vmlinux_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
