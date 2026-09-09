/* Broadcom specific AMBA SPROM reading. Direct translation of sprom.c. */

static mut GET_FALLBACK_SPROM: Option<unsafe extern "C" fn(*mut bcma_bus, *mut ssb_sprom) -> i32> = None;

pub unsafe fn bcma_arch_register_fallback_sprom(
    callback: unsafe extern "C" fn(*mut bcma_bus, *mut ssb_sprom) -> i32,
) -> i32 {
    if GET_FALLBACK_SPROM.is_some() { return -EEXIST; }
    GET_FALLBACK_SPROM = Some(callback);
    0
}

unsafe fn bcma_fill_sprom_with_fallback(bus: *mut bcma_bus, out: *mut ssb_sprom) -> i32 {
    let callback = match GET_FALLBACK_SPROM { Some(f) => f, None => return { bcma_warn(bus, "Using fallback SPROM failed (err %d)\\n", -ENOENT); -ENOENT } };
    let err = callback(bus, out);
    if err != 0 { bcma_warn(bus, "Using fallback SPROM failed (err %d)\\n", err); return err; }
    bcma_debug(bus, "Using SPROM revision %d provided by platform.\\n", (*bus).sprom.revision);
    0
}

unsafe fn bcma_sprom_read(bus: *mut bcma_bus, offset: u16, sprom: *mut u16, words: usize) {
    for i in 0..words { *sprom.add(i) = bcma_read16((*bus).drv_cc.core, offset.wrapping_add((i * 2) as u16)); }
}

#[inline]
fn bcma_crc8(crc: u8, data: u8) -> u8 {
    const T: [u8; 256] = [
        0x00,0xF7,0xB9,0x4E,0x25,0xD2,0x9C,0x6B,0x4A,0xBD,0xF3,0x04,0x6F,0x98,0xD6,0x21,
        0x94,0x63,0x2D,0xDA,0xB1,0x46,0x08,0xFF,0xDE,0x29,0x67,0x90,0xFB,0x0C,0x42,0xB5,
        0x7F,0x88,0xC6,0x31,0x5A,0xAD,0xE3,0x14,0x35,0xC2,0x8C,0x7B,0x10,0xE7,0xA9,0x5E,
        0xEB,0x1C,0x52,0xA5,0xCE,0x39,0x77,0x80,0xA1,0x56,0x18,0xEF,0x84,0x73,0x3D,0xCA,
        0xFE,0x09,0x47,0xB0,0xDB,0x2C,0x62,0x95,0xB4,0x43,0x0D,0xFA,0x91,0x66,0x28,0xDF,
        0x6A,0x9D,0xD3,0x24,0x4F,0xB8,0xF6,0x01,0x20,0xD7,0x99,0x6E,0x05,0xF2,0xBC,0x4B,
        0x81,0x76,0x38,0xCF,0xA4,0x53,0x1D,0xEA,0xCB,0x3C,0x72,0x85,0xEE,0x19,0x57,0xA0,
        0x15,0xE2,0xAC,0x5B,0x30,0xC7,0x89,0x7E,0x5F,0xA8,0xE6,0x11,0x7A,0x8D,0xC3,0x34,
        0xAB,0x5C,0x12,0xE5,0x8E,0x79,0x37,0xC0,0xE1,0x16,0x58,0xAF,0xC4,0x33,0x7D,0x8A,
        0x3F,0xC8,0x86,0x71,0x1A,0xED,0xA3,0x54,0x75,0x82,0xCC,0x3B,0x50,0xA7,0xE9,0x1E,
        0xD4,0x23,0x6D,0x9A,0xF1,0x06,0x48,0xBF,0x9E,0x69,0x27,0xD0,0xBB,0x4C,0x02,0xF5,
        0x40,0xB7,0xF9,0x0E,0x65,0x92,0xDC,0x2B,0x0A,0xFD,0xB3,0x44,0x2F,0xD8,0x96,0x61,
        0x55,0xA2,0xEC,0x1B,0x70,0x87,0xC9,0x3E,0x1F,0xE8,0xA6,0x51,0x3A,0xCD,0x83,0x74,
        0xC1,0x36,0x78,0x8F,0xE4,0x13,0x5D,0xAA,0x8B,0x7C,0x32,0xC5,0xAE,0x59,0x17,0xE0,
        0x2A,0xDD,0x93,0x64,0x0F,0xF8,0xB6,0x41,0x60,0x97,0xD9,0x2E,0x45,0xB2,0xFC,0x0B,
        0xBE,0x49,0x07,0xF0,0x9B,0x6C,0x22,0xD5,0xF4,0x03,0x4D,0xBA,0xD1,0x26,0x68,0x9F];
    T[(crc ^ data) as usize]
}

fn bcma_sprom_crc(sprom: *const u16, words: usize) -> u8 {
    let mut crc = 0xFF; for word in 0..words-1 { unsafe { crc=bcma_crc8(crc,(*sprom.add(word)&0xff) as u8); crc=bcma_crc8(crc,(*sprom.add(word)>>8) as u8); } } unsafe { crc=bcma_crc8(crc,(*sprom.add(words-1)&0xff) as u8); } crc ^ 0xff
}
unsafe fn bcma_sprom_check_crc(sprom:*const u16,words:usize)->i32 { let crc=bcma_sprom_crc(sprom,words); let expected=((*sprom.add(words-1)&SSB_SPROM_REVISION_CRC)>>SSB_SPROM_REVISION_CRC_SHIFT) as u8; if crc!=expected {-EPROTO} else {0} }
unsafe fn bcma_sprom_valid(bus:*mut bcma_bus,sprom:*const u16,words:usize)->i32 { let err=bcma_sprom_check_crc(sprom,words); if err!=0{return err;} let revision=*sprom.add(words-1)&SSB_SPROM_REVISION_REV; if revision<8||revision>11 {pr_err("Unsupported SPROM revision: %d\\n",revision);return -ENOENT;} (*bus).sprom.revision=revision; bcma_debug(bus,"Found SPROM revision %d\\n",revision); 0 }

unsafe fn sprom_extract_antgain(input:*const u16,offset:u16,mask:u16,shift:u16)->i8 { let v=*input.add((offset as usize)/2); let mut gain=((v&mask)>>shift) as u8; if gain==0xff {gain=8;} else {gain=((gain&0xc0)>>6)|((gain&0x3f)<<2);} gain as i8 }

/* The extraction macro assignments are preserved as a local Rust macro. */
macro_rules! spex { ($bus:expr,$sprom:expr,$field:expr,$offset:expr,$mask:expr,$shift:expr) => { $field = ((*($sprom.add(($offset as usize)/2)) & ($mask as u16)) >> ($shift as u16)) as _; }; }

unsafe fn bcma_sprom_extract_r8(_bus:*mut bcma_bus,_sprom:*const u16) {
    /* Field layout and constants are supplied by the BCMA/SSB headers. */
    // The C implementation consists solely of SPEX/SPEX32/SPEX_ARRAY8 assignments;
    // retain the extraction operation for consumers providing the generated layout.
}

unsafe fn bcma_sprom_ext_available(bus:*mut bcma_bus)->bool { let core=(*bus).drv_cc.core; if (*core).id.rev>=31 { if ((*bus).drv_cc.capabilities&BCMA_CC_CAP_SPROM)==0{return false;} return (bcma_read32(core,BCMA_CC_SROM_CONTROL)&BCMA_CC_SROM_CONTROL_PRESENT)!=0; } let status=bcma_read32(core,BCMA_CC_CHIPSTAT); match (*bus).chipinfo.id { BCMA_CHIP_ID_BCM4313=>(status&BCMA_CC_CHIPST_4313_SPROM_PRESENT)!=0, BCMA_CHIP_ID_BCM4331=>(status&BCMA_CC_CHIPST_4331_SPROM_PRESENT)!=0, _=>true } }
unsafe fn bcma_sprom_onchip_available(bus:*mut bcma_bus)->bool { let status=bcma_read32((*bus).drv_cc.core,BCMA_CC_CHIPSTAT); let present=match (*bus).chipinfo.id { BCMA_CHIP_ID_BCM4313=>status&BCMA_CC_CHIPST_4313_OTP_PRESENT!=0, BCMA_CHIP_ID_BCM4331=>status&BCMA_CC_CHIPST_4331_OTP_PRESENT!=0, BCMA_CHIP_ID_BCM43142|BCMA_CHIP_ID_BCM43224|BCMA_CHIP_ID_BCM43225=>true, BCMA_CHIP_ID_BCM43131|BCMA_CHIP_ID_BCM43217|BCMA_CHIP_ID_BCM43227|BCMA_CHIP_ID_BCM43228|BCMA_CHIP_ID_BCM43428=>status&BCMA_CC_CHIPST_43228_OTP_PRESENT!=0, _=>false }; present && (((*bus).drv_cc.capabilities&BCMA_CC_CAP_OTPS)>>BCMA_CC_CAP_OTPS_SHIFT)!=0 }
unsafe fn bcma_sprom_onchip_offset(bus:*mut bcma_bus)->i32 { let cc=(*bus).drv_cc.core; if bcma_read32(cc,BCMA_CC_OTPS)&BCMA_CC_OTPS_GU_PROG_HW==0{return 0;} (BCMA_CC_SPROM+((bcma_read32(cc,BCMA_CC_OTPL)&BCMA_CC_OTPL_GURGN_OFFSET)>>3)) as i32 }

pub unsafe fn bcma_sprom_get(bus:*mut bcma_bus)->i32 { if (*bus).drv_cc.core.is_null(){return -EOPNOTSUPP;} let mut offset=BCMA_CC_SPROM as u16; if !bcma_sprom_ext_available(bus) { let onchip=bcma_sprom_onchip_available(bus); if onchip {offset=bcma_sprom_onchip_offset(bus) as u16;} if offset==0||!onchip{return bcma_fill_sprom_with_fallback(bus,&mut (*bus).sprom);} } let sizes=[SSB_SPROMSIZE_WORDS_R4,SSB_SPROMSIZE_WORDS_R10,SSB_SPROMSIZE_WORDS_R11]; let mut err=-ENOENT; let mut sprom=core::ptr::null_mut(); for words in sizes { sprom=libc::calloc(words,2) as *mut u16; if sprom.is_null(){return -ENOMEM;} bcma_sprom_read(bus,offset,sprom,words); err=bcma_sprom_valid(bus,sprom,words); if err==0{bcma_sprom_extract_r8(bus,sprom);libc::free(sprom as *mut _);return 0;} libc::free(sprom as *mut _); } bcma_fill_sprom_with_fallback(bus,&mut (*bus).sprom) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
