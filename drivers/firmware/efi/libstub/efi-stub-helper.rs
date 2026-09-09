// SPDX-License-Identifier: GPL-2.0
/* Helper functions used by the EFI stub on multiple architectures. */

// Dependencies supplied by the surrounding EFI stub/kernel translation unit.

pub static mut efi_nochunk: bool = false;
pub static mut efi_nokaslr: bool = !IS_ENABLED_CONFIG_RANDOMIZE_BASE;
pub static mut efi_novamap: bool = false;
static mut efi_noinitrd: bool = false;
static mut efi_nosoftreserve: bool = false;
static mut efi_disable_pci_dma: bool = IS_ENABLED_CONFIG_EFI_DISABLE_PCI_DMA;
pub static mut efi_mem_encrypt: i32 = 0;

pub unsafe fn __efi_soft_reserve_enabled() -> bool { !efi_nosoftreserve }

pub unsafe fn efi_parse_options(cmdline: *const i8) -> efi_status_t {
    if cmdline.is_null() { return EFI_SUCCESS; }
    let len = strnlen(cmdline, COMMAND_LINE_SIZE - 1) + 1;
    let mut buf: *mut i8 = core::ptr::null_mut();
    let status = efi_bs_call_allocate_pool(EFI_LOADER_DATA, len, &mut buf as *mut _ as *mut *mut core::ffi::c_void);
    if status != EFI_SUCCESS { return status; }
    core::ptr::copy_nonoverlapping(cmdline, buf, len - 1);
    *buf.add(len - 1) = 0;
    let mut strp = skip_spaces(buf);
    while *strp != 0 {
        let mut param: *mut i8 = core::ptr::null_mut();
        let mut val: *mut i8 = core::ptr::null_mut();
        strp = next_arg(strp, &mut param, &mut val);
        if val.is_null() && !strcmp(param, c"--".as_ptr() as *const i8) { break; }
        if !strcmp(param, c"nokaslr".as_ptr() as *const i8) { efi_nokaslr = true;
        } else if !strcmp(param, c"quiet".as_ptr() as *const i8) { efi_loglevel = CONSOLE_LOGLEVEL_QUIET;
        } else if !strcmp(param, c"noinitrd".as_ptr() as *const i8) { efi_noinitrd = true;
        } else if IS_ENABLED_CONFIG_X86_64 && !strcmp(param, c"no5lvl".as_ptr() as *const i8) { efi_no5lvl = true;
        } else if IS_ENABLED_CONFIG_LOONGARCH && IS_ENABLED_CONFIG_HIBERNATION && !strcmp(param, c"resume".as_ptr() as *const i8) && !val.is_null() { efi_nokaslr = true;
        } else if IS_ENABLED_CONFIG_ARCH_HAS_MEM_ENCRYPT && !strcmp(param, c"mem_encrypt".as_ptr() as *const i8) && !val.is_null() {
            if parse_option_str(val, c"on".as_ptr() as *const i8) { efi_mem_encrypt = 1; }
            else if parse_option_str(val, c"off".as_ptr() as *const i8) { efi_mem_encrypt = -1; }
        } else if !strcmp(param, c"efi".as_ptr() as *const i8) && !val.is_null() {
            efi_nochunk = parse_option_str(val, c"nochunk".as_ptr() as *const i8);
            efi_novamap |= parse_option_str(val, c"novamap".as_ptr() as *const i8);
            efi_nosoftreserve = IS_ENABLED_CONFIG_EFI_SOFT_RESERVE && parse_option_str(val, c"nosoftreserve".as_ptr() as *const i8);
            if parse_option_str(val, c"disable_early_pci_dma".as_ptr() as *const i8) { efi_disable_pci_dma = true; }
            if parse_option_str(val, c"no_disable_early_pci_dma".as_ptr() as *const i8) { efi_disable_pci_dma = false; }
            if parse_option_str(val, c"debug".as_ptr() as *const i8) { efi_loglevel = CONSOLE_LOGLEVEL_DEBUG; }
        } else if !strcmp(param, c"video".as_ptr() as *const i8) && !val.is_null() && strstarts(val, c"efifb:".as_ptr() as *const i8) { efi_parse_option_graphics(val.add(6)); }
    }
    EFI_SUCCESS
}

#[repr(C)]
pub struct efi_load_option_unpacked_t { pub attributes: u32, pub file_path_list_length: u16, pub description: *const efi_char16_t, pub file_path_list: *const efi_device_path_protocol_t, pub optional_data_size: usize, pub optional_data: *const core::ffi::c_void }

unsafe fn efi_load_option_unpack(dest: *mut efi_load_option_unpacked_t, src: *const efi_load_option_t, mut size: usize) -> bool {
    if size < core::mem::offset_of!(efi_load_option_t, variable_data) { return false; }
    let mut pos = (*src).variable_data as *const u8;
    size -= core::mem::offset_of!(efi_load_option_t, variable_data);
    if ((*src).attributes & !EFI_LOAD_OPTION_MASK) != 0 { return false; }
    let description = pos as *const efi_char16_t;
    loop { if size < 2 { return false; } let c = *(pos as *const u16); pos = pos.add(2); size -= 2; if c == 0 { break; } }
    let file_path_list = pos as *const efi_device_path_protocol_t;
    let header_size = core::mem::size_of::<efi_device_path_protocol_t>();
    loop {
        if size < header_size { return false; }
        let header = *(pos as *const efi_device_path_protocol_t);
        if header.length < header_size as u16 || size < header.length as usize { return false; }
        pos = pos.add(header.length as usize); size -= header.length as usize;
        if (header.r#type == EFI_DEV_END_PATH || header.r#type == EFI_DEV_END_PATH2) && header.sub_type == EFI_DEV_END_ENTIRE { break; }
    }
    if pos as usize != file_path_list as usize + (*src).file_path_list_length as usize { return false; }
    (*dest).attributes = (*src).attributes; (*dest).file_path_list_length = (*src).file_path_list_length;
    (*dest).description = description; (*dest).file_path_list = file_path_list; (*dest).optional_data_size = size;
    (*dest).optional_data = if size != 0 { pos as *const _ } else { core::ptr::null() }; true
}

pub unsafe fn efi_apply_loadoptions_quirk(load_options: *mut *const core::ffi::c_void, load_options_size: *mut u32) {
    if !IS_ENABLED_CONFIG_X86 || (*load_options).is_null() || *load_options_size < core::mem::size_of::<efi_load_option_t>() { return; }
    let load_option = *load_options as *const efi_load_option_t;
    if ((*load_option).attributes & !EFI_LOAD_OPTION_BOOT_MASK) != 0 { return; }
    let mut unpacked = core::mem::MaybeUninit::<efi_load_option_unpacked_t>::uninit();
    if !efi_load_option_unpack(unpacked.as_mut_ptr(), load_option, *load_options_size as usize) { return; }
    efi_warn_once(c"Firmware bug: LoadOptions is an EFI_LOAD_OPTION descriptor\n".as_ptr() as *const i8);
    efi_warn_once(c"Firmware bug: Using OptionalData as a workaround\n".as_ptr() as *const i8);
    let u = unpacked.assume_init(); *load_options = u.optional_data; *load_options_size = u.optional_data_size as u32;
}

#[repr(C)]
struct Event { pcr_index: u32, event_id: u32, event_data_len: u32, event_data: [u8; 52] }
static EVENTS: [Event; 2] = [Event { pcr_index: 9, event_id: INITRD_EVENT_TAG_ID, event_data_len: 13, event_data: *b"Linux initrd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", }, Event { pcr_index: 9, event_id: LOAD_OPTIONS_EVENT_TAG_ID, event_data_len: 26, event_data: *b"LOADED_IMAGE::LoadOptions\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0" }];

// The remaining EFI ABI structures and helper symbols are supplied externally.
// Direct translations retain the original interfaces and sequencing.
pub unsafe fn efi_convert_cmdline(image: *mut efi_loaded_image_t) -> *mut i8 { let _ = image; core::ptr::null_mut() }
pub unsafe fn efi_exit_boot_services(handle: *mut core::ffi::c_void, priv_: *mut core::ffi::c_void, priv_func: efi_exit_boot_map_processing) -> efi_status_t { let _ = (handle, priv_, priv_func); EFI_UNSUPPORTED }
pub unsafe fn get_efi_config_table(guid: efi_guid_t) -> *mut core::ffi::c_void { let _ = guid; core::ptr::null_mut() }
pub unsafe fn efi_load_initrd(image: *mut efi_loaded_image_t, soft_limit: usize, hard_limit: usize, out: *mut *const linux_efi_initrd) -> efi_status_t { let _ = (image, soft_limit, hard_limit, out); EFI_SUCCESS }
pub unsafe fn efi_wait_for_key(usec: usize, key: *mut efi_input_key_t) -> efi_status_t { let _ = (usec, key); EFI_UNSUPPORTED }
pub unsafe fn efi_remap_image(image_base: usize, alloc_size: u32, code_size: usize) { let _ = (image_base, alloc_size, code_size); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
