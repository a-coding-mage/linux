// SPDX-License-Identifier: GPL-2.0-or-later
// EFI GUID Partition Table handling. C dependencies and kernel-provided types
// are intentionally referenced but not reimplemented here.

static mut FORCE_GPT: i32 = 0;

unsafe extern "C" fn force_gpt_fn(_str: *mut core::ffi::c_char) -> i32 {
    FORCE_GPT = 1;
    1
}

#[inline]
unsafe fn efi_crc32(buf: *const core::ffi::c_void, len: libc::c_ulong) -> u32 {
    crc32(!0u64 as libc::c_ulong, buf, len) ^ !0u64 as libc::c_ulong
}

unsafe fn last_lba(disk: *mut gendisk) -> u64 {
    div_u64(
        bdev_nr_bytes((*disk).part0),
        queue_logical_block_size((*disk).queue),
    ) - 1
}

#[inline]
unsafe fn pmbr_part_valid(part: *mut gpt_mbr_record) -> i32 {
    if (*part).os_type != EFI_PMBR_OSTYPE_EFI_GPT {
        return 0;
    }
    if le32_to_cpu((*part).starting_lba) != GPT_PRIMARY_PARTITION_TABLE_LBA {
        return 0;
    }
    GPT_MBR_PROTECTIVE
}

unsafe fn is_pmbr_valid(mbr: *mut legacy_mbr, total_sectors: sector_t) -> i32 {
    let mut sz: u32 = 0;
    let mut part = 0usize;
    let mut ret = 0i32;
    if mbr.is_null() || le16_to_cpu((*mbr).signature) != MSDOS_MBR_SIGNATURE {
        return ret;
    }
    for i in 0..4 {
        ret = pmbr_part_valid(&mut (*mbr).partition_record[i]);
        if ret == GPT_MBR_PROTECTIVE {
            part = i;
            break;
        }
    }
    if ret != GPT_MBR_PROTECTIVE {
        return 0;
    }
    for i in 0..4 {
        if (*mbr).partition_record[i].os_type != EFI_PMBR_OSTYPE_EFI_GPT
            && (*mbr).partition_record[i].os_type != 0
        {
            ret = GPT_MBR_HYBRID;
        }
    }
    if ret == GPT_MBR_PROTECTIVE {
        sz = le32_to_cpu((*mbr).partition_record[part].size_in_lba);
        if sz != (total_sectors - 1) as u32 && sz != 0xffff_ffff {
            pr_debug!("GPT: mbr size in lba ({}) different than whole disk ({}).\n",
                sz, min(total_sectors - 1, 0xffff_ffff));
        }
    }
    ret
}

unsafe fn read_lba(state: *mut parsed_partitions, lba: u64, mut buffer: *mut u8, mut count: usize) -> usize {
    let mut totalreadcount = 0usize;
    let n0 = lba * (queue_logical_block_size((*state).disk.queue) / 512) as u64;
    if buffer.is_null() || lba > last_lba((*state).disk) { return 0; }
    let mut n = n0;
    while count != 0 {
        let mut copied = 512usize;
        let mut sect: Sector = core::mem::zeroed();
        let data = read_part_sector(state, n, &mut sect);
        n += 1;
        if data.is_null() { break; }
        if copied > count { copied = count; }
        core::ptr::copy_nonoverlapping(data, buffer, copied);
        put_dev_sector(sect);
        buffer = buffer.add(copied);
        totalreadcount += copied;
        count -= copied;
    }
    totalreadcount
}

unsafe fn alloc_read_gpt_entries(state: *mut parsed_partitions, gpt: *mut gpt_header) -> *mut gpt_entry {
    if gpt.is_null() { return core::ptr::null_mut(); }
    let count = le32_to_cpu((*gpt).num_partition_entries) as usize
        * le32_to_cpu((*gpt).sizeof_partition_entry) as usize;
    if count == 0 { return core::ptr::null_mut(); }
    let pte = kmalloc(count, GFP_KERNEL) as *mut gpt_entry;
    if pte.is_null() { return core::ptr::null_mut(); }
    if read_lba(state, le64_to_cpu((*gpt).partition_entry_lba), pte as *mut u8, count) < count {
        kfree(pte as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    pte
}

unsafe fn alloc_read_gpt_header(state: *mut parsed_partitions, lba: u64) -> *mut gpt_header {
    let ssz = queue_logical_block_size((*state).disk.queue) as usize;
    let gpt = kmalloc(ssz, GFP_KERNEL) as *mut gpt_header;
    if gpt.is_null() { return core::ptr::null_mut(); }
    if read_lba(state, lba, gpt as *mut u8, ssz) < ssz {
        kfree(gpt as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    gpt
}

unsafe fn is_gpt_valid(state: *mut parsed_partitions, lba: u64, gpt: *mut *mut gpt_header, ptes: *mut *mut gpt_entry) -> i32 {
    if ptes.is_null() { return 0; }
    *gpt = alloc_read_gpt_header(state, lba);
    if (*gpt).is_null() { return 0; }
    let h = *gpt;
    if le64_to_cpu((*h).signature) != GPT_HEADER_SIGNATURE
        || le32_to_cpu((*h).header_size) > queue_logical_block_size((*state).disk.queue)
        || le32_to_cpu((*h).header_size) < core::mem::size_of::<gpt_header>() as u32
    { goto_fail(gpt, ptes); return 0; }
    let origcrc = le32_to_cpu((*h).header_crc32);
    (*h).header_crc32 = 0;
    let crc = efi_crc32(h as *const core::ffi::c_void, le32_to_cpu((*h).header_size) as libc::c_ulong) as u32;
    (*h).header_crc32 = cpu_to_le32(origcrc);
    if crc != origcrc || le64_to_cpu((*h).my_lba) != lba { goto_fail(gpt, ptes); return 0; }
    let lastlba = last_lba((*state).disk);
    if le64_to_cpu((*h).first_usable_lba) > lastlba
        || le64_to_cpu((*h).last_usable_lba) > lastlba
        || le64_to_cpu((*h).last_usable_lba) < le64_to_cpu((*h).first_usable_lba)
        || le32_to_cpu((*h).sizeof_partition_entry) != core::mem::size_of::<gpt_entry>() as u32
    { goto_fail(gpt, ptes); return 0; }
    let pt_size = le32_to_cpu((*h).num_partition_entries) as u64 * le32_to_cpu((*h).sizeof_partition_entry) as u64;
    if pt_size > KMALLOC_MAX_SIZE as u64 { goto_fail(gpt, ptes); return 0; }
    *ptes = alloc_read_gpt_entries(state, h);
    if (*ptes).is_null() || efi_crc32(*ptes as *const core::ffi::c_void, pt_size as libc::c_ulong) as u32 != le32_to_cpu((*h).partition_entry_array_crc32) {
        if !(*ptes).is_null() { kfree(*ptes as *mut core::ffi::c_void); *ptes = core::ptr::null_mut(); }
        kfree(*gpt as *mut core::ffi::c_void); *gpt = core::ptr::null_mut();
        return 0;
    }
    1
}

unsafe fn goto_fail(gpt: *mut *mut gpt_header, ptes: *mut *mut gpt_entry) {
    if !(*ptes).is_null() { kfree(*ptes as *mut core::ffi::c_void); *ptes = core::ptr::null_mut(); }
    if !(*gpt).is_null() { kfree(*gpt as *mut core::ffi::c_void); *gpt = core::ptr::null_mut(); }
}

#[inline]
unsafe fn is_pte_valid(pte: *const gpt_entry, lastlba: u64) -> i32 {
    if efi_guidcmp((*pte).partition_type_guid, NULL_GUID) == 0
        || le64_to_cpu((*pte).starting_lba) > lastlba
        || le64_to_cpu((*pte).ending_lba) > lastlba { 0 } else { 1 }
}

unsafe fn compare_gpts(pgpt: *mut gpt_header, agpt: *mut gpt_header, lastlba: u64) {
    if pgpt.is_null() || agpt.is_null() { return; }
    if le64_to_cpu((*pgpt).my_lba) != le64_to_cpu((*agpt).alternate_lba) { pr_warn!("GPT:Primary header LBA != Alt. header alternate_lba\n"); }
    if le64_to_cpu((*pgpt).alternate_lba) != le64_to_cpu((*agpt).my_lba) { pr_warn!("GPT:Primary header alternate_lba != Alt. header my_lba\n"); }
    if le64_to_cpu((*pgpt).first_usable_lba) != le64_to_cpu((*agpt).first_usable_lba) { pr_warn!("GPT:first_usable_lbas don't match.\n"); }
    if le64_to_cpu((*pgpt).last_usable_lba) != le64_to_cpu((*agpt).last_usable_lba) { pr_warn!("GPT:last_usable_lbas don't match.\n"); }
    if efi_guidcmp((*pgpt).disk_guid, (*agpt).disk_guid) != 0 { pr_warn!("GPT:disk_guids don't match.\n"); }
    if le32_to_cpu((*pgpt).num_partition_entries) != le32_to_cpu((*agpt).num_partition_entries) { pr_warn!("GPT:num_partition_entries don't match\n"); }
    if le32_to_cpu((*pgpt).sizeof_partition_entry) != le32_to_cpu((*agpt).sizeof_partition_entry) { pr_warn!("GPT:sizeof_partition_entry values don't match\n"); }
    if le32_to_cpu((*pgpt).partition_entry_array_crc32) != le32_to_cpu((*agpt).partition_entry_array_crc32) { pr_warn!("GPT:partition_entry_array_crc32 values don't match\n"); }
    if le64_to_cpu((*pgpt).alternate_lba) != lastlba || le64_to_cpu((*agpt).my_lba) != lastlba { pr_warn!("GPT: GPT headers are not at the end of the disk.\n"); }
}

unsafe fn find_valid_gpt(state: *mut parsed_partitions, gpt: *mut *mut gpt_header, ptes: *mut *mut gpt_entry) -> i32 {
    if ptes.is_null() { return 0; }
    let mut pgpt = core::ptr::null_mut(); let mut agpt = core::ptr::null_mut();
    let mut pptes = core::ptr::null_mut(); let mut aptes = core::ptr::null_mut();
    let disk = (*state).disk; let lastlba = last_lba(disk);
    if FORCE_GPT == 0 {
        let legacymbr = kzalloc(core::mem::size_of::<legacy_mbr>(), GFP_KERNEL) as *mut legacy_mbr;
        if legacymbr.is_null() { return 0; }
        read_lba(state, 0, legacymbr as *mut u8, core::mem::size_of::<legacy_mbr>());
        let good = is_pmbr_valid(legacymbr, get_capacity(disk)); kfree(legacymbr as *mut core::ffi::c_void);
        if good == 0 { return 0; }
    }
    let good_pgpt = is_gpt_valid(state, GPT_PRIMARY_PARTITION_TABLE_LBA, &mut pgpt, &mut pptes);
    let mut good_agpt = 0;
    if good_pgpt != 0 { good_agpt = is_gpt_valid(state, le64_to_cpu((*pgpt).alternate_lba), &mut agpt, &mut aptes); }
    if good_agpt == 0 && FORCE_GPT != 0 { good_agpt = is_gpt_valid(state, lastlba, &mut agpt, &mut aptes); }
    if good_pgpt == 0 && good_agpt == 0 { goto_fail(&mut pgpt, &mut pptes); goto_fail(&mut agpt, &mut aptes); *gpt = core::ptr::null_mut(); *ptes = core::ptr::null_mut(); return 0; }
    compare_gpts(pgpt, agpt, lastlba);
    if good_pgpt != 0 { *gpt = pgpt; *ptes = pptes; kfree(agpt as *mut core::ffi::c_void); kfree(aptes as *mut core::ffi::c_void); }
    else { *gpt = agpt; *ptes = aptes; kfree(pgpt as *mut core::ffi::c_void); kfree(pptes as *mut core::ffi::c_void); }
    1
}

unsafe fn utf16_le_to_7bit(input: *const __le16, size: u32, out: *mut u8) {
    *out.add(size as usize) = 0;
    for i in 0..size as usize {
        let mut c = (le16_to_cpu(*input.add(i)) & 0x7f) as u8;
        if c != 0 && !isprint(c as i32) { c = b'!'; }
        *out.add(i) = c;
    }
}

pub unsafe fn efi_partition(state: *mut parsed_partitions) -> i32 {
    let mut gpt = core::ptr::null_mut(); let mut ptes = core::ptr::null_mut();
    let ssz = queue_logical_block_size((*state).disk.queue) / 512;
    if find_valid_gpt(state, &mut gpt, &mut ptes) == 0 || gpt.is_null() || ptes.is_null() { kfree(gpt as *mut core::ffi::c_void); kfree(ptes as *mut core::ffi::c_void); return 0; }
    for i in 0..le32_to_cpu((*gpt).num_partition_entries).min((*state).limit - 1) {
        let pte = ptes.add(i as usize);
        if is_pte_valid(pte, last_lba((*state).disk)) == 0 { continue; }
        let start = le64_to_cpu((*pte).starting_lba); let size = le64_to_cpu((*pte).ending_lba) - start + 1;
        put_partition(state, i + 1, start * ssz as u64, size * ssz as u64);
        if efi_guidcmp((*pte).partition_type_guid, PARTITION_LINUX_RAID_GUID) == 0 { (*state).parts.add((i + 1) as usize).as_mut().unwrap().flags = ADDPART_FLAG_RAID; }
        let info = &mut (*state).parts.add((i + 1) as usize).as_mut().unwrap().info;
        efi_guid_to_str(&(*pte).unique_partition_guid, info.uuid.as_mut_ptr());
        let label_max = min(ARRAY_SIZE(info.volname) - 1, ARRAY_SIZE((*pte).partition_name));
        utf16_le_to_7bit((*pte).partition_name.as_ptr(), label_max as u32, info.volname.as_mut_ptr());
        (*state).parts.add((i + 1) as usize).as_mut().unwrap().has_info = true;
    }
    kfree(ptes as *mut core::ffi::c_void); kfree(gpt as *mut core::ffi::c_void);
    seq_buf_puts(&mut (*state).pp_buf, "\n");
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
