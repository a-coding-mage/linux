/* Translated from amdgpu_bios.c. */

const AMD_VBIOS_SIGNATURE: &[u8] = b" 761295520";
const AMD_VBIOS_SIGNATURE_OFFSET: usize = 0x30;
const AMD_VBIOS_SIGNATURE_SIZE: usize = AMD_VBIOS_SIGNATURE.len();
const AMD_VBIOS_SIGNATURE_END: usize = AMD_VBIOS_SIGNATURE_OFFSET + AMD_VBIOS_SIGNATURE_SIZE;
const ATRM_BIOS_PAGE: usize = 4096;

unsafe fn check_atom_bios(adev: *mut amdgpu_device, size: usize) -> bool {
    let bios = (*adev).bios as *const u8;
    if bios.is_null() || size < 0x49 { return false; }
    if *bios != 0x55 || *bios.add(1) != 0xaa { return false; }
    let bios_header_start = (*bios.add(0x48) as u16) | ((*bios.add(0x49) as u16) << 8);
    if bios_header_start == 0 { return false; }
    let tmp = bios_header_start as usize + 4;
    if size < tmp { return false; }
    let tag = core::slice::from_raw_parts(bios.add(tmp), 4);
    tag == b"ATOM" || tag == b"MOTA"
}

pub unsafe fn amdgpu_bios_release(adev: *mut amdgpu_device) {
    kfree((*adev).bios);
    (*adev).bios = core::ptr::null_mut();
    (*adev).bios_size = 0;
}

unsafe fn amdgpu_read_bios_from_vram(adev: *mut amdgpu_device) -> bool {
    let mut bios: *mut u8 = core::ptr::null_mut();
    let vram_base: resource_size_t;
    let mut size: u32 = 256 * 1024;
    if (*adev).flags & AMD_IS_APU == 0 && amdgpu_device_need_post(adev) { return false; }
    if pci_resource_len((*adev).pdev, 0) == 0 { return false; }
    (*adev).bios = core::ptr::null_mut();
    vram_base = pci_resource_start((*adev).pdev, 0);
    (*adev).bios = kmalloc(size as usize, GFP_KERNEL);
    if (*adev).bios.is_null() { return false; }
    if amdgpu_sriov_vf(adev) && (*adev).virt.is_dynamic_crit_regn_enabled {
        if amdgpu_virt_get_dynamic_data_info(adev, AMD_SRIOV_MSG_VBIOS_IMG_TABLE_ID, (*adev).bios, &mut size) != 0 {
            amdgpu_bios_release(adev); return false;
        }
    } else {
        bios = ioremap_wc(vram_base, size as usize);
        if bios.is_null() { amdgpu_bios_release(adev); return false; }
        memcpy_fromio((*adev).bios, bios, size as usize);
        iounmap(bios);
    }
    (*adev).bios_size = size as usize;
    if !check_atom_bios(adev, size as usize) { amdgpu_bios_release(adev); return false; }
    true
}

pub unsafe fn amdgpu_read_bios(adev: *mut amdgpu_device) -> bool {
    let mut size: usize = 0;
    (*adev).bios = core::ptr::null_mut();
    let bios = pci_map_rom((*adev).pdev, &mut size);
    if bios.is_null() { return false; }
    (*adev).bios = kzalloc(size, GFP_KERNEL);
    if (*adev).bios.is_null() { pci_unmap_rom((*adev).pdev, bios); return false; }
    (*adev).bios_size = size;
    memcpy_fromio((*adev).bios, bios, size);
    pci_unmap_rom((*adev).pdev, bios);
    if !check_atom_bios(adev, size) { amdgpu_bios_release(adev); return false; }
    true
}

unsafe fn amdgpu_read_bios_from_rom(adev: *mut amdgpu_device) -> bool {
    let mut header = [0u8; AMD_VBIOS_SIGNATURE_END + 1];
    if (*adev).asic_funcs.is_null() || (*(*adev).asic_funcs).read_bios_from_rom.is_none() { return false; }
    if !amdgpu_asic_read_bios_from_rom(adev, header.as_mut_ptr(), header.len()) { return false; }
    header[AMD_VBIOS_SIGNATURE_END] = 0;
    if header[0] != 0x55 || header[1] != 0xaa || &header[AMD_VBIOS_SIGNATURE_OFFSET..AMD_VBIOS_SIGNATURE_OFFSET + AMD_VBIOS_SIGNATURE_SIZE] != AMD_VBIOS_SIGNATURE { return false; }
    let mut len = ((header[2] as usize) << 9 + 3) & !3;
    (*adev).bios = kmalloc(len, GFP_KERNEL);
    if (*adev).bios.is_null() { return false; }
    (*adev).bios_size = len;
    amdgpu_asic_read_bios_from_rom(adev, (*adev).bios, len);
    if !check_atom_bios(adev, len) { amdgpu_bios_release(adev); return false; }
    true
}

unsafe fn amdgpu_read_platform_bios(adev: *mut amdgpu_device) -> bool {
    let rom = (*(*adev).pdev).rom;
    let romlen = (*(*adev).pdev).romlen;
    (*adev).bios = core::ptr::null_mut();
    if rom == 0 || romlen == 0 { return false; }
    (*adev).bios = kzalloc(romlen, GFP_KERNEL);
    if (*adev).bios.is_null() { return false; }
    let bios = ioremap(rom, romlen);
    if bios.is_null() { amdgpu_bios_release(adev); return false; }
    memcpy_fromio((*adev).bios, bios, romlen); iounmap(bios);
    if !check_atom_bios(adev, romlen) { amdgpu_bios_release(adev); return false; }
    (*adev).bios_size = romlen; true
}

#[cfg(not(CONFIG_ACPI))]
unsafe fn amdgpu_atrm_get_bios(_adev: *mut amdgpu_device) -> bool { false }

unsafe fn amdgpu_read_disabled_bios(adev: *mut amdgpu_device) -> bool {
    if (*adev).asic_funcs.is_null() || (*(*adev).asic_funcs).read_disabled_bios.is_none() { false } else { amdgpu_asic_read_disabled_bios(adev) }
}

#[cfg(not(CONFIG_ACPI))]
unsafe fn amdgpu_acpi_vfct_bios(_adev: *mut amdgpu_device) -> bool { false }

unsafe fn amdgpu_get_bios_apu(adev: *mut amdgpu_device) -> bool {
    if amdgpu_acpi_vfct_bios(adev) || amdgpu_read_bios_from_vram(adev) || amdgpu_read_bios(adev) || amdgpu_read_platform_bios(adev) { true } else { false }
}

unsafe fn amdgpu_prefer_rom_resource(adev: *mut amdgpu_device) -> bool {
    let res = &(*(*adev).pdev).resource[PCI_ROM_RESOURCE];
    (res.flags & IORESOURCE_ROM_SHADOW) != 0 || (*adev).pdev == vga_default_device()
}

unsafe fn amdgpu_get_bios_dgpu(adev: *mut amdgpu_device) -> bool {
    if amdgpu_atrm_get_bios(adev) || amdgpu_acpi_vfct_bios(adev) || amdgpu_read_bios_from_vram(adev) { return true; }
    if amdgpu_prefer_rom_resource(adev) {
        if amdgpu_read_bios(adev) || amdgpu_read_platform_bios(adev) { return true; }
    } else if amdgpu_read_platform_bios(adev) || amdgpu_read_bios(adev) { return true; }
    amdgpu_read_bios_from_rom(adev) || amdgpu_read_disabled_bios(adev)
}

pub unsafe fn amdgpu_get_bios(adev: *mut amdgpu_device) -> bool {
    let found = if (*adev).flags & AMD_IS_APU != 0 { amdgpu_get_bios_apu(adev) } else { amdgpu_get_bios_dgpu(adev) };
    if found { (*adev).is_atom_fw = (*adev).asic_type >= CHIP_VEGA10; }
    found
}

pub unsafe fn amdgpu_soc15_read_bios_from_rom(adev: *mut amdgpu_device, bios: *mut u8, length_bytes: u32) -> bool {
    if bios.is_null() || length_bytes == 0 || (*adev).flags & AMD_IS_APU != 0 { return false; }
    if (*adev).smuio.funcs.is_null() || (*(*adev).smuio.funcs).get_rom_index_offset.is_none() || (*(*adev).smuio.funcs).get_rom_data_offset.is_none() { return false; }
    let length_dw = ((length_bytes + 3) & !3) / 4;
    let rom_index_offset = ((*(*adev).smuio.funcs).get_rom_index_offset.unwrap())(adev);
    let rom_data_offset = ((*(*adev).smuio.funcs).get_rom_data_offset.unwrap())(adev);
    let rom_offset = if !(*adev).nbio.funcs.is_null() && (*(*adev).nbio.funcs).get_rom_offset.is_some() { ((*(*adev).nbio.funcs).get_rom_offset.unwrap())(adev) << 17 } else { 0 };
    WREG32(rom_index_offset, rom_offset);
    for i in 0..length_dw { *(bios as *mut u32).add(i as usize) = RREG32(rom_data_offset); }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
