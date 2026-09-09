// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 */

// Dependencies supplied by the surrounding EFI, device-tree, architecture,
// unaligned-access, and EFI stub environments are intentionally external.

type JumpKernelFunc = unsafe extern "C" fn(usize, usize) -> !;

static mut hartid: usize = 0;

unsafe fn get_boot_hartid_from_fdt() -> i32 {
    let fdt: *const core::ffi::c_void;
    let chosen_node: i32;
    let mut len: i32 = 0;
    let prop: *const core::ffi::c_void;

    fdt = get_efi_config_table(DEVICE_TREE_GUID);
    if fdt.is_null() {
        return -EINVAL;
    }

    chosen_node = fdt_path_offset(fdt, b"/chosen\0".as_ptr() as *const i8);
    if chosen_node < 0 {
        return -EINVAL;
    }

    prop = fdt_getprop(
        fdt as *mut core::ffi::c_void,
        chosen_node,
        b"boot-hartid\0".as_ptr() as *const i8,
        &mut len,
    );
    if prop.is_null() {
        return -EINVAL;
    }

    if len as usize == core::mem::size_of::<u32>() {
        hartid = fdt32_to_cpu(*(prop as *const u32)) as usize;
    } else if len as usize == core::mem::size_of::<u64>() {
        hartid = fdt64_to_cpu(__get_unaligned_t_fdt64(prop)) as usize;
    } else {
        return -EINVAL;
    }

    0
}

unsafe fn get_boot_hartid_from_efi() -> efi_status_t {
    let boot_protocol_guid: efi_guid_t = RISCV_EFI_BOOT_PROTOCOL_GUID;
    let mut boot_protocol: *mut riscv_efi_boot_protocol = core::ptr::null_mut();
    let status: efi_status_t;

    status = efi_bs_call_locate_protocol(
        &boot_protocol_guid,
        core::ptr::null_mut(),
        &mut boot_protocol as *mut _ as *mut *mut core::ffi::c_void,
    );
    if status != EFI_SUCCESS {
        return status;
    }
    efi_call_proto_get_boot_hartid(boot_protocol, &mut hartid)
}

pub unsafe extern "C" fn check_platform_features() -> efi_status_t {
    let status: efi_status_t;
    let ret: i32;

    status = get_boot_hartid_from_efi();
    if status != EFI_SUCCESS {
        ret = get_boot_hartid_from_fdt();
        if ret != 0 {
            efi_err(b"Failed to get boot hartid!\n\0".as_ptr() as *const i8);
            return EFI_UNSUPPORTED;
        }
    }
    EFI_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn stext_offset() -> usize {
    /*
     * This fallback definition is used by the EFI zboot stub, which loads
     * the entire image so it can branch via the image header at offset #0.
     */
    0
}

pub unsafe extern "C" fn efi_enter_kernel(
    entrypoint: usize,
    fdt: usize,
    _fdt_size: usize,
) -> ! {
    let kernel_entry = entrypoint.wrapping_add(stext_offset());
    let jump_kernel: JumpKernelFunc = core::mem::transmute(kernel_entry);

    /*
     * Jump to real kernel here with following constraints.
     * 1. MMU should be disabled.
     * 2. a0 should contain hartid
     * 3. a1 should DT address
     */
    csr_write(CSR_SATP, 0);
    jump_kernel(hartid, fdt)
}

extern "C" {
    static DEVICE_TREE_GUID: efi_guid_t;
    static RISCV_EFI_BOOT_PROTOCOL_GUID: efi_guid_t;
    static EFI_SUCCESS: efi_status_t;
    static EFI_UNSUPPORTED: efi_status_t;
    static EINVAL: i32;

    fn get_efi_config_table(guid: efi_guid_t) -> *const core::ffi::c_void;
    fn fdt_path_offset(fdt: *const core::ffi::c_void, path: *const i8) -> i32;
    fn fdt_getprop(
        fdt: *mut core::ffi::c_void,
        node: i32,
        name: *const i8,
        len: *mut i32,
    ) -> *const core::ffi::c_void;
    fn fdt32_to_cpu(value: u32) -> u32;
    fn fdt64_to_cpu(value: u64) -> u64;
    fn __get_unaligned_t_fdt64(ptr: *const core::ffi::c_void) -> u64;
    fn efi_bs_call_locate_protocol(
        protocol: *const efi_guid_t,
        registration: *mut core::ffi::c_void,
        interface: *mut *mut core::ffi::c_void,
    ) -> efi_status_t;
    fn efi_call_proto_get_boot_hartid(
        protocol: *mut riscv_efi_boot_protocol,
        hartid: *mut usize,
    ) -> efi_status_t;
    fn efi_err(message: *const i8);
    fn csr_write(csr: usize, value: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
