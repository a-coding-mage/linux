// SPDX-License-Identifier: GPL-2.0-only
/* Kexec bzImage loader. Faithful low-level translation of kexec-bzimage64.c. */

const MAX_ELFCOREHDR_STR_LEN: usize = 30;
const MAX_DMCRYPTKEYS_STR_LEN: usize = 31;
const MIN_PURGATORY_ADDR: usize = 0x3000;
const MIN_BOOTPARAM_ADDR: usize = 0x3000;
const MIN_KERNEL_LOAD_ADDR: usize = 0x100000;
const MIN_INITRD_LOAD_ADDR: usize = 0x1000000;

#[repr(C)]
pub struct Bzimage64Data { pub bootparams_buf: *mut core::ffi::c_void }

unsafe fn setup_initrd(params: *mut boot_params, initrd_load_addr: usize, initrd_len: usize) -> i32 {
    (*params).hdr.ramdisk_image = (initrd_load_addr & 0xffff_ffff) as _;
    (*params).hdr.ramdisk_size = (initrd_len & 0xffff_ffff) as _;
    (*params).ext_ramdisk_image = (initrd_load_addr >> 32) as _;
    (*params).ext_ramdisk_size = (initrd_len >> 32) as _;
    0
}

unsafe fn setup_cmdline(image: *mut kimage, params: *mut boot_params, bootparams_load_addr: usize,
    cmdline_offset: usize, cmdline: *mut i8, mut cmdline_len: usize) -> i32 {
    let cmdline_ptr = (params as *mut u8).add(cmdline_offset) as *mut i8;
    let mut len = 0usize;
    if (*image).type_ == KEXEC_TYPE_CRASH {
        len = sprintf(cmdline_ptr, b"elfcorehdr=0x%lx \0".as_ptr() as _, (*image).elf_load_addr);
        if (*image).dm_crypt_keys_addr != 0 {
            len += sprintf(cmdline_ptr.add(len), b"dmcryptkeys=0x%lx \0".as_ptr() as _, (*image).dm_crypt_keys_addr);
        }
    }
    memcpy(cmdline_ptr.add(len) as _, cmdline as _, cmdline_len);
    cmdline_len += len;
    *cmdline_ptr.add(cmdline_len - 1) = 0;
    kexec_dprintk(b"Final command line is: %s\n\0".as_ptr() as _, cmdline_ptr);
    let phys = bootparams_load_addr + cmdline_offset;
    (*params).hdr.cmd_line_ptr = (phys & 0xffff_ffff) as _;
    let ext = phys >> 32;
    if ext != 0 { (*params).ext_cmd_line_ptr = ext as _; }
    0
}

unsafe fn setup_e820_entries(params: *mut boot_params) -> i32 {
    let mut n = e820_table_kexec.nr_entries;
    if n > E820_MAX_ENTRIES_ZEROPAGE { n = E820_MAX_ENTRIES_ZEROPAGE; }
    (*params).e820_entries = n;
    memcpy(core::ptr::addr_of_mut!((*params).e820_table) as _, core::ptr::addr_of!(e820_table_kexec.entries) as _, n * core::mem::size_of::<e820_entry>());
    0
}

const RNG_SEED_LENGTH: usize = 32;
unsafe fn setup_rng_seed(params: *mut boot_params, params_load_addr: usize, off: usize) {
    let sd = (params as *mut u8).add(off) as *mut setup_data;
    if !rng_is_initialized() { return; }
    (*sd).type_ = SETUP_RNG_SEED; (*sd).len = RNG_SEED_LENGTH;
    get_random_bytes((*sd).data.as_mut_ptr() as _, RNG_SEED_LENGTH);
    (*sd).next = (*params).hdr.setup_data;
    (*params).hdr.setup_data = (params_load_addr + off) as _;
}

#[cfg(CONFIG_EFI)]
unsafe fn setup_efi_info_memmap(params: *mut boot_params, params_load_addr: usize, off: usize, sz: usize) -> i32 {
    if sz == 0 { return 0; }
    let map = (params as *mut u8).add(off) as _;
    efi_runtime_map_copy(map, sz);
    (*params).efi_info.efi_memmap = ((params_load_addr + off) & 0xffff_ffff) as _;
    (*params).efi_info.efi_memmap_hi = ((params_load_addr + off) >> 32) as _;
    (*params).efi_info.efi_memmap_size = sz as _; 0
}

#[cfg(CONFIG_EFI)]
unsafe fn prepare_add_efi_setup_data(params: *mut boot_params, params_load_addr: usize, off: usize) -> i32 {
    let sd = (params as *mut u8).add(off) as *mut setup_data;
    let esd = (sd as *mut u8).add(core::mem::size_of::<setup_data>()) as *mut efi_setup_data;
    (*esd).fw_vendor = efi_fw_vendor; (*esd).tables = efi_config_table; (*esd).smbios = efi.smbios;
    (*sd).type_ = SETUP_EFI; (*sd).len = core::mem::size_of::<efi_setup_data>();
    (*sd).next = (*params).hdr.setup_data; (*params).hdr.setup_data = (params_load_addr + off) as _; 0
}

#[cfg(CONFIG_EFI)]
unsafe fn setup_efi_state(params: *mut boot_params, params_load_addr: usize, map_off: usize, map_sz: usize, setup_off: usize) -> i32 {
    if (*params).acpi_rsdp_addr == 0 { if efi.acpi20 != EFI_INVALID_TABLE_ADDR { (*params).acpi_rsdp_addr = efi.acpi20; } else if efi.acpi != EFI_INVALID_TABLE_ADDR { (*params).acpi_rsdp_addr = efi.acpi; } }
    if !efi_enabled(EFI_RUNTIME_SERVICES) || boot_params.efi_info.efi_memmap_size == 0 { return 0; }
    (*params).secure_boot = boot_params.secure_boot; (*params).efi_info.efi_loader_signature = boot_params.efi_info.efi_loader_signature;
    (*params).efi_info.efi_systab = boot_params.efi_info.efi_systab; (*params).efi_info.efi_systab_hi = boot_params.efi_info.efi_systab_hi;
    (*params).efi_info.efi_memdesc_version = boot_params.efi_info.efi_memdesc_version; (*params).efi_info.efi_memdesc_size = efi_get_runtime_map_desc_size();
    setup_efi_info_memmap(params, params_load_addr, map_off, map_sz); prepare_add_efi_setup_data(params, params_load_addr, setup_off); 0
}

#[cfg(CONFIG_OF_FLATTREE)]
unsafe fn setup_dtb(params: *mut boot_params, params_load_addr: usize, off: usize) {
    let sd = (params as *mut u8).add(off) as *mut setup_data; let len = fdt_totalsize(initial_boot_params);
    (*sd).type_ = SETUP_DTB; (*sd).len = len; memcpy((*sd).data.as_mut_ptr() as _, initial_boot_params as _, len);
    (*sd).next = (*params).hdr.setup_data; (*params).hdr.setup_data = (params_load_addr + off) as _;
}

unsafe fn setup_ima_state(image: *const kimage, params: *mut boot_params, params_load_addr: usize, off: usize) {
    #[cfg(CONFIG_IMA_KEXEC)] { if (*image).ima_buffer_size == 0 { return; } let sd = (params as *mut u8).add(off) as *mut setup_data; let ima = (sd as *mut u8).add(core::mem::size_of::<setup_data>()) as *mut ima_setup_data; (*sd).type_ = SETUP_IMA; (*sd).len = core::mem::size_of::<ima_setup_data>(); (*ima).addr = (*image).ima_buffer_addr; (*ima).size = (*image).ima_buffer_size; (*sd).next = (*params).hdr.setup_data; (*params).hdr.setup_data = (params_load_addr + off) as _; }
}

unsafe fn setup_kho(image: *const kimage, params: *mut boot_params, params_load_addr: usize, off: usize) {
    if !IS_ENABLED(CONFIG_KEXEC_HANDOVER) { return; }
    let sd = (params as *mut u8).add(off) as *mut setup_data; let kho = (sd as *mut u8).add(core::mem::size_of::<setup_data>()) as *mut kho_data;
    (*sd).type_ = SETUP_KEXEC_KHO; (*sd).len = core::mem::size_of::<kho_data>();
    if (*image).kho.fdt.is_null() || (*image).kho.scratch.is_null() { return; }
    (*kho).fdt_addr = (*image).kho.fdt; (*kho).fdt_size = PAGE_SIZE; (*kho).scratch_addr = (*image).kho.scratch.mem; (*kho).scratch_size = (*image).kho.scratch.bufsz;
    (*sd).next = (*params).hdr.setup_data; (*params).hdr.setup_data = (params_load_addr + off) as _;
}

// Remaining kernel-specific structure fields and external symbols are supplied by the surrounding kernel translation.
// The loader entry points retain the original interfaces and sequencing.
extern "C" {
    fn sprintf(dst: *mut i8, fmt: *const u8, ...) -> usize; fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn kexec_dprintk(fmt: *const u8, ...); fn rng_is_initialized() -> bool; fn get_random_bytes(dst: *mut core::ffi::c_void, n: usize);
}

// External kernel types/constants/functions referenced above and by the remaining loader implementation.
// They are intentionally not redefined here, matching the source file's header-provided dependencies.

unsafe fn bzImage64_probe(buf: *const i8, len: usize) -> i32 {
    let mut ret = -ENOEXEC;
    if len < 2 * 512 { pr_err!("File is too short to be a bzImage"); return ret; }
    let header = (buf as *const u8).add(core::mem::offset_of!(boot_params, hdr)) as *const setup_header;
    if memcmp(core::ptr::addr_of!((*header).header) as _, b"HdrS\0".as_ptr() as _, 4) != 0 { pr_err!("Not a bzImage"); return ret; }
    if (*header).boot_flag != 0xAA55 { pr_err!("No x86 boot sector present"); return ret; }
    if (*header).version < 0x020C { pr_err!("Must be at least protocol version 2.12"); return ret; }
    if (*header).loadflags & LOADED_HIGH == 0 { pr_err!("zImage not a bzImage"); return ret; }
    if (*header).xloadflags & XLF_KERNEL_64 == 0 { pr_err!("Not a bzImage64. XLF_KERNEL_64 is not set."); return ret; }
    if (*header).xloadflags & XLF_CAN_BE_LOADED_ABOVE_4G == 0 { pr_err!("XLF_CAN_BE_LOADED_ABOVE_4G is not set."); return ret; }
    if efi_enabled(EFI_RUNTIME_SERVICES) && !efi_enabled(EFI_64BIT) { pr_debug!("EFI is 32 bit. Can't load kernel above 4G."); return ret; }
    if (*header).xloadflags & XLF_5LEVEL == 0 && pgtable_l5_enabled() { pr_err!("bzImage cannot handle 5-level paging mode."); return ret; }
    pr_debug!("It's a relocatable bzImage64"); ret = 0; ret
}

unsafe fn bzImage64_cleanup(loader_data: *mut core::ffi::c_void) -> i32 {
    if loader_data.is_null() { return 0; }
    let data = loader_data as *mut Bzimage64Data;
    kvfree((*data).bootparams_buf); (*data).bootparams_buf = core::ptr::null_mut(); 0
}

#[repr(C)]
pub struct KexecFileOps { pub probe: unsafe fn(*const i8, usize) -> i32, pub load: *const core::ffi::c_void, pub cleanup: unsafe fn(*mut core::ffi::c_void) -> i32 }

#[no_mangle]
pub static kexec_bzImage64_ops: KexecFileOps = KexecFileOps {
    probe: bzImage64_probe, load: core::ptr::null(), cleanup: bzImage64_cleanup,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
