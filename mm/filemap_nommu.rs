// SPDX-License-Identifier: GPL-2.0-only
// Exact original !CONFIG_MMU bodies, not fallback translation stubs.
#[no_mangle]
pub unsafe extern "C" fn filemap_page_mkwrite(_vmf: *mut vm_fault) -> vm_fault_t {
    RUST_FILEMAP_VM_FAULT_SIGBUS
}
#[no_mangle]
pub unsafe extern "C" fn generic_file_mmap(_file: *mut file, _vma: *mut vm_area_struct) -> c_int {
    -(ENOSYS as c_int)
}
#[no_mangle]
pub unsafe extern "C" fn generic_file_mmap_prepare(_desc: *mut vm_area_desc) -> c_int {
    -(ENOSYS as c_int)
}
#[no_mangle]
pub unsafe extern "C" fn generic_file_readonly_mmap(
    _file: *mut file,
    _vma: *mut vm_area_struct,
) -> c_int {
    -(ENOSYS as c_int)
}
#[no_mangle]
pub unsafe extern "C" fn generic_file_readonly_mmap_prepare(_desc: *mut vm_area_desc) -> c_int {
    -(ENOSYS as c_int)
}
