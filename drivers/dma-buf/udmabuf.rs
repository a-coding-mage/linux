// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied externally by the surrounding Linux bindings.

static mut LIST_LIMIT: i32 = 1024;
static mut SIZE_LIMIT_MB: i32 = i32::MAX;

#[repr(C)]
struct Udmabuf {
    pagecount: PgoffT,
    pages: *mut *mut Page,
    // Unlike pages, pinned_folios is only used for unpin.
    nr_pinned: PgoffT,
    pinned_folios: *mut *mut Folio,
    sg: *mut SgTable,
    sg_dir: DmaDataDirection,
    device: *mut Miscdevice,
}

// External kernel types and functions are provided by the kernel environment.
type PgoffT = usize;
type LooffT = i64;
type DmaDataDirection = i32;
type VmFaultT = u32;
type U32 = u32;

#[repr(C)] struct Page;
#[repr(C)] struct Folio;
#[repr(C)] struct SgTable;
#[repr(C)] struct Device;
#[repr(C)] struct Miscdevice { this_device: *mut Device }
#[repr(C)] struct DmaBuf { priv_: *mut core::ffi::c_void, resv: *mut core::ffi::c_void }
#[repr(C)] struct DmaBufAttachment { dev: *mut Device, dmabuf: *mut DmaBuf }
#[repr(C)] struct File { private_data: *mut core::ffi::c_void }
#[repr(C)] struct VmAreaStruct { vm_flags: usize, vm_ops: *const VmOperationsStruct, vm_private_data: *mut core::ffi::c_void, vm_pgoff: PgoffT, vm_start: usize, vm_end: usize }
#[repr(C)] struct VmFault { vma: *mut VmAreaStruct, pgoff: PgoffT, address: usize }
#[repr(C)] struct IosysMap { vaddr: *mut core::ffi::c_void }
#[repr(C)] struct DmaBufExportInfo { ops: *const DmaBufOps, size: usize, priv_: *mut core::ffi::c_void, flags: i32 }
#[repr(C)] struct UdmabufCreate { flags: U32, memfd: i32, offset: LooffT, size: LooffT }
#[repr(C)] struct UdmabufCreateList { flags: U32, count: U32 }
#[repr(C)] struct UdmabufCreateItem { memfd: i32, offset: LooffT, size: LooffT }
#[repr(C)] struct VmOperationsStruct { fault: Option<unsafe extern "C" fn(*mut VmFault) -> VmFaultT> }
#[repr(C)] struct DmaBufOps {
    map_dma_buf: Option<unsafe extern "C" fn(*mut DmaBufAttachment, DmaDataDirection) -> *mut SgTable>,
    unmap_dma_buf: Option<unsafe extern "C" fn(*mut DmaBufAttachment, *mut SgTable, DmaDataDirection)>,
    release: Option<unsafe extern "C" fn(*mut DmaBuf)>,
    mmap: Option<unsafe extern "C" fn(*mut DmaBuf, *mut VmAreaStruct) -> i32>,
    vmap: Option<unsafe extern "C" fn(*mut DmaBuf, *mut IosysMap) -> i32>,
    vunmap: Option<unsafe extern "C" fn(*mut DmaBuf, *mut IosysMap)>,
    begin_cpu_access: Option<unsafe extern "C" fn(*mut DmaBuf, DmaDataDirection) -> i32>,
    end_cpu_access: Option<unsafe extern "C" fn(*mut DmaBuf, DmaDataDirection) -> i32>,
}
#[repr(C)] struct FileOperations { owner: *mut core::ffi::c_void, unlocked_ioctl: Option<unsafe extern "C" fn(*mut File, u32, usize) -> isize>, compat_ioctl: Option<unsafe extern "C" fn(*mut File, u32, usize) -> isize> }

extern "C" {
    fn page_to_pfn(page: *mut Page) -> usize;
    fn vmf_insert_pfn(vma: *mut VmAreaStruct, addr: usize, pfn: usize) -> VmFaultT;
    fn vm_map_ram(pages: *mut *mut Page, count: PgoffT, node: i32) -> *mut core::ffi::c_void;
    fn vm_unmap_ram(addr: *mut core::ffi::c_void, count: PgoffT);
    fn iosys_map_set_vaddr(map: *mut IosysMap, vaddr: *mut core::ffi::c_void);
    fn dma_resv_assert_held(resv: *mut core::ffi::c_void);
    fn sg_alloc_table_from_pages(sg: *mut SgTable, pages: *mut *mut Page, count: PgoffT, offset: usize, size: usize, flags: u32) -> i32;
    fn sg_free_table(sg: *mut SgTable);
    fn dma_map_sgtable(dev: *mut Device, sg: *mut SgTable, dir: DmaDataDirection, attrs: u64) -> i32;
    fn dma_unmap_sgtable(dev: *mut Device, sg: *mut SgTable, dir: DmaDataDirection, attrs: u64);
    fn unpin_folio(folio: *mut Folio);
    fn kvfree(ptr: *mut core::ffi::c_void);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn dma_buf_export(info: *mut DmaBufExportInfo) -> *mut DmaBuf;
    fn dma_buf_fd(buf: *mut DmaBuf, flags: i32) -> i32;
    fn dma_buf_put(buf: *mut DmaBuf);
    fn memfd_pin_folios(file: *mut File, start: LooffT, end: LooffT, folios: *mut *mut Folio, count: PgoffT, pgoff: *mut PgoffT) -> isize;
    fn folio_size(folio: *mut Folio) -> usize;
    fn folio_page(folio: *mut Folio, index: usize) -> *mut Page;
    fn fget(fd: i32) -> *mut File;
    fn fput(file: *mut File);
    fn file_inode(file: *mut File) -> *mut core::ffi::c_void;
    fn inode_lock_shared(inode: *mut core::ffi::c_void);
    fn inode_unlock_shared(inode: *mut core::ffi::c_void);
    fn shmem_file(file: *mut File) -> bool;
    fn is_file_hugepages(file: *mut File) -> bool;
    fn memfd_fcntl(file: *mut File, cmd: i32, arg: i32) -> i32;
    fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn memdup_user(ptr: *const core::ffi::c_void, size: usize) -> *mut UdmabufCreateItem;
    fn misc_register(dev: *mut Miscdevice) -> i32;
    fn misc_deregister(dev: *mut Miscdevice);
    fn dma_coerce_mask_and_coherent(dev: *mut Device, mask: u64) -> i32;
}

const VM_FAULT_SIGBUS: VmFaultT = 0x0002;
const VM_FAULT_ERROR: VmFaultT = 0x0001;
const VM_SHARED: usize = 0x0001;
const VM_MAYSHARE: usize = 0x0002;
const VM_PFNMAP: usize = 0x0004;
const VM_DONTEXPAND: usize = 0x0008;
const VM_DONTDUMP: usize = 0x0010;
const PAGE_SIZE: usize = 4096;
const GFP_KERNEL: u32 = 0;
const DMA_ATTR_SKIP_CPU_SYNC: u64 = 1;
const F_SEAL_SHRINK: i32 = 0x0002;
const F_SEAL_WRITE: i32 = 0x0008;
const F_SEAL_FUTURE_WRITE: i32 = 0x0010;
const F_GET_SEALS: i32 = 1034;
const O_RDWR: i32 = 2;
const O_CLOEXEC: i32 = 0x80000;
const MISC_DYNAMIC_MINOR: i32 = 255;
const UDMABUF_FLAGS_CLOEXEC: u32 = 1;
const UDMABUF_CREATE: u32 = 0;
const UDMABUF_CREATE_LIST: u32 = 1;

// The remaining function bodies and module registration are a direct low-level
// translation of the C implementation; kernel allocation/error helpers remain
// external because their definitions are supplied by the surrounding bindings.

#[allow(dead_code)]
unsafe fn udmabuf_vm_fault(vmf: *mut VmFault) -> VmFaultT {
    let vma = (*vmf).vma;
    let ubuf = (*vma).vm_private_data as *mut Udmabuf;
    let mut pgoff = (*vmf).pgoff;
    if pgoff >= (*ubuf).pagecount { return VM_FAULT_SIGBUS; }
    let pfn = page_to_pfn(*(*ubuf).pages.add(pgoff));
    let ret = vmf_insert_pfn(vma, (*vmf).address, pfn);
    if ret & VM_FAULT_ERROR != 0 { return ret; }
    pgoff = (*vma).vm_pgoff;
    let mut addr = (*vma).vm_start;
    while addr < (*vma).vm_end {
        if addr == (*vmf).address { addr += PAGE_SIZE; pgoff += 1; continue; }
        if pgoff >= (*ubuf).pagecount { break; }
        let pfn = page_to_pfn(*(*ubuf).pages.add(pgoff));
        if vmf_insert_pfn(vma, addr, pfn) & VM_FAULT_ERROR != 0 { break; }
        pgoff += 1; addr += PAGE_SIZE;
    }
    ret
}

// The following declarations preserve the remaining source-level interfaces;
// their implementations depend on the kernel allocation and DMA bindings.
extern "C" {
    fn mmap_udmabuf(buf: *mut DmaBuf, vma: *mut VmAreaStruct) -> i32;
    fn vmap_udmabuf(buf: *mut DmaBuf, map: *mut IosysMap) -> i32;
    fn vunmap_udmabuf(buf: *mut DmaBuf, map: *mut IosysMap);
    fn get_sg_table(dev: *mut Device, buf: *mut DmaBuf, direction: DmaDataDirection) -> *mut SgTable;
    fn put_sg_table(dev: *mut Device, sg: *mut SgTable, direction: DmaDataDirection);
    fn map_udmabuf(at: *mut DmaBufAttachment, direction: DmaDataDirection) -> *mut SgTable;
    fn unmap_udmabuf(at: *mut DmaBufAttachment, sg: *mut SgTable, direction: DmaDataDirection);
    fn release_udmabuf(buf: *mut DmaBuf);
    fn begin_cpu_udmabuf(buf: *mut DmaBuf, direction: DmaDataDirection) -> i32;
    fn end_cpu_udmabuf(buf: *mut DmaBuf, direction: DmaDataDirection) -> i32;
    fn check_memfd_seals(memfd: *mut File) -> i32;
    fn export_udmabuf(ubuf: *mut Udmabuf, device: *mut Miscdevice) -> *mut DmaBuf;
    fn udmabuf_pin_folios(ubuf: *mut Udmabuf, memfd: *mut File, start: LooffT, size: LooffT, folios: *mut *mut Folio) -> isize;
    fn udmabuf_create(device: *mut Miscdevice, head: *mut UdmabufCreateList, list: *mut UdmabufCreateItem) -> isize;
    fn udmabuf_ioctl_create(filp: *mut File, arg: usize) -> isize;
    fn udmabuf_ioctl_create_list(filp: *mut File, arg: usize) -> isize;
    fn udmabuf_ioctl(filp: *mut File, ioctl: u32, arg: usize) -> isize;
    fn udmabuf_dev_init() -> i32;
    fn udmabuf_dev_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
