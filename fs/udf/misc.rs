// SPDX-License-Identifier: GPL-2.0-only
/*
 * misc.c
 *
 * PURPOSE
 *	Miscellaneous routines for the OSTA-UDF(tm) filesystem.
 */

// Dependencies supplied by the surrounding UDF/Linux compatibility layer are
// intentionally left as external names.

pub unsafe fn udf_add_extendedattr(
    inode: *mut inode,
    mut size: u32,
    typ: u32,
    loc: u8,
) -> *mut genericFormat {
    let mut ea: *mut u8;
    let ad: *mut u8;
    let mut offset: i32;
    let crclen: u16;
    let iinfo = UDF_I(inode);

    ea = (*iinfo).i_data;
    if (*iinfo).i_lenEAttr != 0 {
        ad = (*iinfo).i_data.add((*iinfo).i_lenEAttr as usize);
    } else {
        ad = ea;
        size += core::mem::size_of::<extendedAttrHeaderDesc>() as u32;
    }

    offset = (*(*inode).i_sb).s_blocksize as i32
        - udf_file_entry_alloc_offset(inode) as i32
        - (*iinfo).i_lenAlloc as i32;

    if (loc & 0x01) != 0 && offset >= size as i32 {
        let eahd = ea as *mut extendedAttrHeaderDesc;

        if (*iinfo).i_lenAlloc != 0 {
            core::ptr::copy(ad, ad.add(size as usize), (*iinfo).i_lenAlloc as usize);
        }

        if (*iinfo).i_lenEAttr != 0 {
            if (*eahd).descTag.tagIdent != cpu_to_le16(TAG_IDENT_EAHD)
                || le32_to_cpu((*eahd).descTag.tagLocation)
                    != (*iinfo).i_location.logicalBlockNum
            {
                return core::ptr::null_mut();
            }
        } else {
            let sbi = UDF_SB((*inode).i_sb);
            size -= core::mem::size_of::<extendedAttrHeaderDesc>() as u32;
            (*iinfo).i_lenEAttr += core::mem::size_of::<extendedAttrHeaderDesc>() as u32;
            (*eahd).descTag.tagIdent = cpu_to_le16(TAG_IDENT_EAHD);
            (*eahd).descTag.descVersion = cpu_to_le16(if (*sbi).s_udfrev >= 0x0200 { 3 } else { 2 });
            (*eahd).descTag.tagSerialNum = cpu_to_le16((*sbi).s_serial_number);
            (*eahd).descTag.tagLocation = cpu_to_le32((*iinfo).i_location.logicalBlockNum);
            (*eahd).impAttrLocation = cpu_to_le32(0xFFFFFFFF);
            (*eahd).appAttrLocation = cpu_to_le32(0xFFFFFFFF);
        }

        offset = (*iinfo).i_lenEAttr as i32;
        if typ < 2048 {
            let aal = le32_to_cpu((*eahd).appAttrLocation);
            if aal < (*iinfo).i_lenEAttr {
                core::ptr::copy(ea.add(aal as usize), ea.add((offset as u32 - aal + size) as usize), (offset as u32 - aal) as usize);
                offset -= aal as i32;
                (*eahd).appAttrLocation = cpu_to_le32(aal + size);
            }
            let ial = le32_to_cpu((*eahd).impAttrLocation);
            if ial < (*iinfo).i_lenEAttr {
                core::ptr::copy(ea.add(ial as usize), ea.add((offset as u32 - ial + size) as usize), (offset as u32 - ial) as usize);
                offset -= ial as i32;
                (*eahd).impAttrLocation = cpu_to_le32(ial + size);
            }
        } else if typ < 65536 {
            let aal = le32_to_cpu((*eahd).appAttrLocation);
            if aal < (*iinfo).i_lenEAttr {
                core::ptr::copy(ea.add(aal as usize), ea.add((offset as u32 - aal + size) as usize), (offset as u32 - aal) as usize);
                offset -= aal as i32;
                (*eahd).appAttrLocation = cpu_to_le32(aal + size);
            }
        }
        crclen = (core::mem::size_of::<extendedAttrHeaderDesc>() - core::mem::size_of::<tag>()) as u16;
        (*eahd).descTag.descCRCLength = cpu_to_le16(crclen);
        (*eahd).descTag.descCRC = cpu_to_le16(crc_itu_t(0, (&(*eahd).descTag as *const tag).add(1) as *const i8, crclen));
        (*eahd).descTag.tagChecksum = udf_tag_checksum(&(*eahd).descTag);
        (*iinfo).i_lenEAttr += size;
        return ea.add(offset as usize) as *mut genericFormat;
    }
    core::ptr::null_mut()
}

pub unsafe fn udf_get_extendedattr(inode: *mut inode, typ: u32, subtype: u8) -> *mut genericFormat {
    let ea = (*UDF_I(inode)).i_data;
    let iinfo = UDF_I(inode);
    if (*iinfo).i_lenEAttr != 0 {
        let eahd = ea as *mut extendedAttrHeaderDesc;
        if (*eahd).descTag.tagIdent != cpu_to_le16(TAG_IDENT_EAHD)
            || le32_to_cpu((*eahd).descTag.tagLocation) != (*iinfo).i_location.logicalBlockNum { return core::ptr::null_mut(); }
        let mut offset = if typ < 2048 { core::mem::size_of::<extendedAttrHeaderDesc>() as u32 } else if typ < 65536 { le32_to_cpu((*eahd).impAttrLocation) } else { le32_to_cpu((*eahd).appAttrLocation) };
        while offset + core::mem::size_of::<genericFormat>() as u32 < (*iinfo).i_lenEAttr {
            let gaf = ea.add(offset as usize) as *mut genericFormat;
            let attr_length = le32_to_cpu((*gaf).attrLength);
            if attr_length < core::mem::size_of::<genericFormat>() as u32 || attr_length > (*iinfo).i_lenEAttr - offset { break; }
            if le32_to_cpu((*gaf).attrType) == typ && (*gaf).attrSubtype == subtype { return gaf; }
            offset += attr_length;
        }
    }
    core::ptr::null_mut()
}

pub unsafe fn udf_read_tagged(sb: *mut super_block, block: u32, location: u32, ident: *mut u16) -> *mut buffer_head {
    if block == 0xFFFFFFFF { return core::ptr::null_mut(); }
    let bh = sb_bread(sb, block);
    if bh.is_null() { udf_err(sb, "read failed, block=%u, location=%u\n", block, location); return core::ptr::null_mut(); }
    let tag_p = (*bh).b_data as *mut tag;
    *ident = le16_to_cpu((*tag_p).tagIdent);
    if location != le32_to_cpu((*tag_p).tagLocation) { udf_debug("location mismatch block %u, tag %u != %u\n", block, le32_to_cpu((*tag_p).tagLocation), location); brelse(bh); return core::ptr::null_mut(); }
    if udf_tag_checksum(&*tag_p) != (*tag_p).tagChecksum || ((*tag_p).descVersion != cpu_to_le16(2) && (*tag_p).descVersion != cpu_to_le16(3)) || le16_to_cpu((*tag_p).descCRCLength) as usize + core::mem::size_of::<tag>() > (*sb).s_blocksize { brelse(bh); return core::ptr::null_mut(); }
    if (*tag_p).descCRC == cpu_to_le16(crc_itu_t(0, (*bh).b_data.add(core::mem::size_of::<tag>()) as *const i8, le16_to_cpu((*tag_p).descCRCLength))) { return bh; }
    brelse(bh); core::ptr::null_mut()
}

pub unsafe fn udf_read_ptagged(sb: *mut super_block, loc: *mut kernel_lb_addr, offset: u32, ident: *mut u16) -> *mut buffer_head { udf_read_tagged(sb, udf_get_lb_pblock(sb, loc, offset), (*loc).logicalBlockNum + offset, ident) }

pub unsafe fn udf_update_tag(data: *mut i8, mut length: i32) { let tptr = data as *mut tag; length -= core::mem::size_of::<tag>() as i32; (*tptr).descCRCLength = cpu_to_le16(length as u16); (*tptr).descCRC = cpu_to_le16(crc_itu_t(0, data.add(core::mem::size_of::<tag>()), length as u16)); (*tptr).tagChecksum = udf_tag_checksum(&*tptr); }
pub unsafe fn udf_new_tag(data: *mut i8, ident: u16, version: u16, snum: u16, loc: u32, length: i32) { let tptr = data as *mut tag; (*tptr).tagIdent = cpu_to_le16(ident); (*tptr).descVersion = cpu_to_le16(version); (*tptr).tagSerialNum = cpu_to_le16(snum); (*tptr).tagLocation = cpu_to_le32(loc); udf_update_tag(data, length); }
pub unsafe fn udf_tag_checksum(t: *const tag) -> u8 { let data = t as *const u8; let mut checksum = 0u8; for i in 0..core::mem::size_of::<tag>() { if i != 4 { checksum = checksum.wrapping_add(*data.add(i)); } } checksum }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
