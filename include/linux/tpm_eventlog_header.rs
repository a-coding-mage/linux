/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding Linux translation.

pub const TCG_EVENT_NAME_LEN_MAX: usize = 255;
pub const MAX_TEXT_EVENT: usize = 1000; /* Max event string length */
pub const ACPI_TCPA_SIG: &[u8] = b"TCPA"; /* 0x41504354 /'TCPA' */

pub const EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2: u32 = 0x1;
pub const EFI_TCG2_EVENT_LOG_FORMAT_TCG_2: u32 = 0x2;

#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn do_endian_conversion(x: u32) -> u32 { x.to_be() }

#[cfg(not(target_arch = "powerpc64"))]
#[inline(always)]
pub unsafe fn do_endian_conversion(x: u32) -> u32 { x }

#[repr(u32)]
pub enum bios_platform_class { BIOS_CLIENT = 0x00, BIOS_SERVER = 0x01 }

#[repr(C)]
pub struct tcpa_event { pub pcr_index: u32, pub event_type: u32, pub pcr_value: [u8; 20], pub event_size: u32, pub event_data: [u8; 0] }

#[repr(u32)]
pub enum tcpa_event_types { PREBOOT = 0, POST_CODE, UNUSED, NO_ACTION, SEPARATOR, ACTION, EVENT_TAG, SCRTM_CONTENTS, SCRTM_VERSION, CPU_MICROCODE, PLATFORM_CONFIG_FLAGS, TABLE_OF_DEVICES, COMPACT_HASH, IPL, IPL_PARTITION_DATA, NONHOST_CODE, NONHOST_CONFIG, NONHOST_INFO }

#[repr(C)]
pub struct tcpa_pc_event { pub event_id: u32, pub event_size: u32, pub event_data: [u8; 0] }

#[repr(u32)]
pub enum tcpa_pc_event_ids { SMBIOS = 1, BIS_CERT, POST_BIOS_ROM, ESCD, CMOS, NVRAM, OPTION_ROM_EXEC, OPTION_ROM_CONFIG, OPTION_ROM_MICROCODE = 10, S_CRTM_VERSION, S_CRTM_CONTENTS, POST_CONTENTS, HOST_TABLE_OF_DEVICES }

#[repr(C, packed)]
pub struct tcg_efi_specid_event_algs { pub alg_id: u16, pub digest_size: u16 }

pub const TCG_SPECID_SIG: &[u8] = b"Spec ID Event03";

#[repr(C, packed)]
pub struct tcg_efi_specid_event_head { pub signature: [u8; 16], pub platform_class: u32, pub spec_version_minor: u8, pub spec_version_major: u8, pub spec_errata: u8, pub uintnsize: u8, pub num_algs: u32, pub digest_sizes: [tcg_efi_specid_event_algs; 0] }

#[repr(C, packed)]
pub struct tcg_pcr_event { pub pcr_idx: u32, pub event_type: u32, pub digest: [u8; 20], pub event_size: u32, pub event: [u8; 0] }

#[repr(C, packed)]
pub struct tcg_event_field { pub event_size: u32, pub event: [u8; 0] }

#[repr(C, packed)]
pub struct tcg_pcr_event2_head { pub pcr_idx: u32, pub event_type: u32, pub count: u32, pub digests: [tpm_digest; 0] }

#[repr(C)]
pub struct tcg_algorithm_size { pub algorithm_id: u16, pub algorithm_size: u16 }

#[repr(C)]
pub struct tcg_algorithm_info { pub signature: [u8; 16], pub platform_class: u32, pub spec_version_minor: u8, pub spec_version_major: u8, pub spec_errata: u8, pub uintn_size: u8, pub number_of_algorithms: u32, pub digest_sizes: [tcg_algorithm_size; 0] }

#[inline]
pub unsafe fn TPM_MEMREMAP(_start: usize, _size: usize) -> *mut core::ffi::c_void { core::ptr::null_mut() }

#[inline]
pub unsafe fn TPM_MEMUNMAP(_mapping: *mut core::ffi::c_void, _size: usize) {}

unsafe extern "C" {
    fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> i32;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
}

#[inline(always)]
pub unsafe fn __calc_tpm2_event_size(event: *mut tcg_pcr_event2_head, event_header: *mut tcg_pcr_event, do_mapping: bool) -> u32 {
    let mut efispecid: *mut tcg_efi_specid_event_head;
    let mut event_field: *mut tcg_event_field;
    let mut mapping: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut mapping_size: usize = 0;
    let marker_start = event as *mut u8;
    let mut marker = marker_start.add(12);
    let mut size: usize = 0;
    let mut halg_size: usize;
    let mut halg: u16 = 0;
    let mut i: u32;
    let mut j: u32;
    let count: u32;
    let event_type: u32;
    let zero_digest = [0u8; 20];

    if do_mapping {
        mapping_size = marker.offset_from(marker_start) as usize;
        mapping = TPM_MEMREMAP(marker_start as usize, mapping_size);
        if mapping.is_null() { return 0; }
    } else { mapping = marker_start as *mut core::ffi::c_void; }
    event = mapping as *mut tcg_pcr_event2_head;
    count = (*event).count;
    event_type = (*event).event_type;
    if (*event_header).pcr_idx != 0 || (*event_header).event_type != tcpa_event_types::NO_ACTION as u32 || memcmp((*event_header).digest.as_ptr() as _, zero_digest.as_ptr() as _, 20) != 0 { if do_mapping { TPM_MEMUNMAP(mapping, mapping_size); } return 0; }
    efispecid = (*event_header).event as *mut tcg_efi_specid_event_head;
    if memcmp((*efispecid).signature.as_ptr() as _, TCG_SPECID_SIG.as_ptr() as _, TCG_SPECID_SIG.len()) != 0 || (*efispecid).num_algs == 0 || count != (*efispecid).num_algs { if do_mapping { TPM_MEMUNMAP(mapping, mapping_size); } return 0; }
    i = 0;
    while i < count {
        halg_size = 2;
        if do_mapping { TPM_MEMUNMAP(mapping, mapping_size); mapping_size = halg_size; mapping = TPM_MEMREMAP(marker as usize, mapping_size); if mapping.is_null() { return 0; } } else { mapping = marker as *mut core::ffi::c_void; }
        memcpy((&mut halg as *mut u16) as _, mapping, halg_size); marker = marker.add(halg_size);
        j = 0; while j < (*efispecid).num_algs { let alg = (*efispecid).digest_sizes.as_ptr().add(j as usize); if halg == (*alg).alg_id { marker = marker.add((*alg).digest_size as usize); break; } j += 1; }
        if j == (*efispecid).num_algs { return 0; }
        i += 1;
    }
    if do_mapping { TPM_MEMUNMAP(mapping, mapping_size); mapping_size += 4; mapping = TPM_MEMREMAP(marker as usize, mapping_size); if mapping.is_null() { return 0; } } else { mapping = marker as *mut core::ffi::c_void; }
    event_field = mapping as *mut tcg_event_field;
    marker = marker.add(4).add((*event_field).event_size as usize);
    size = marker.offset_from(marker_start) as usize;
    if event_type == 0 && (*event_field).event_size == 0 { size = 0; }
    if do_mapping { TPM_MEMUNMAP(mapping, mapping_size); }
    size as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
