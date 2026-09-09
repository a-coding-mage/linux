/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org> et al. */

// C header dependencies are supplied by the surrounding translation unit.

#[cfg(CONFIG_MTD_CFI_I1)]
#[inline]
pub unsafe fn cfi_interleave(_cfi: *const cfi_private) -> i32 { 1 }
#[cfg(CONFIG_MTD_CFI_I2)]
#[inline]
pub unsafe fn cfi_interleave(_cfi: *const cfi_private) -> i32 { 2 }
#[cfg(CONFIG_MTD_CFI_I4)]
#[inline]
pub unsafe fn cfi_interleave(_cfi: *const cfi_private) -> i32 { 4 }
#[cfg(CONFIG_MTD_CFI_I8)]
#[inline]
pub unsafe fn cfi_interleave(_cfi: *const cfi_private) -> i32 { 8 }

#[cfg(not(any(CONFIG_MTD_CFI_I1, CONFIG_MTD_CFI_I2, CONFIG_MTD_CFI_I4, CONFIG_MTD_CFI_I8)))]
#[inline]
pub unsafe fn cfi_interleave(_cfi: *const cfi_private) -> i32 { panic!("BUG") }

#[inline]
pub unsafe fn cfi_interleave_is_1(cfi: *const cfi_private) -> bool { cfi_interleave(cfi) == 1 }
#[inline]
pub unsafe fn cfi_interleave_is_2(cfi: *const cfi_private) -> bool { cfi_interleave(cfi) == 2 }
#[inline]
pub unsafe fn cfi_interleave_is_4(cfi: *const cfi_private) -> bool { cfi_interleave(cfi) == 4 }
#[inline]
pub unsafe fn cfi_interleave_is_8(cfi: *const cfi_private) -> bool { cfi_interleave(cfi) == 8 }

#[inline]
pub const fn cfi_interleave_supported(i: i32) -> i32 {
    match i {
        #[cfg(CONFIG_MTD_CFI_I1)] 1 |
        #[cfg(CONFIG_MTD_CFI_I2)] 2 |
        #[cfg(CONFIG_MTD_CFI_I4)] 4 |
        #[cfg(CONFIG_MTD_CFI_I8)] 8 => 1,
        _ => 0,
    }
}

pub const CFI_DEVICETYPE_X8: u32 = 8 / 8;
pub const CFI_DEVICETYPE_X16: u32 = 16 / 8;
pub const CFI_DEVICETYPE_X32: u32 = 32 / 8;
pub const CFI_DEVICETYPE_X64: u32 = 64 / 8;

pub const CFI_INTERFACE_X8_ASYNC: u16 = 0x0000;
pub const CFI_INTERFACE_X16_ASYNC: u16 = 0x0001;
pub const CFI_INTERFACE_X8_BY_X16_ASYNC: u16 = 0x0002;
pub const CFI_INTERFACE_X32_ASYNC: u16 = 0x0003;
pub const CFI_INTERFACE_X16_BY_X32_ASYNC: u16 = 0x0005;
pub const CFI_INTERFACE_NOT_ALLOWED: u16 = 0xffff;

#[repr(C, packed)]
pub struct cfi_ident {
    pub qry: [u8; 3], pub P_ID: u16, pub P_ADR: u16, pub A_ID: u16, pub A_ADR: u16,
    pub VccMin: u8, pub VccMax: u8, pub VppMin: u8, pub VppMax: u8,
    pub WordWriteTimeoutTyp: u8, pub BufWriteTimeoutTyp: u8, pub BlockEraseTimeoutTyp: u8,
    pub ChipEraseTimeoutTyp: u8, pub WordWriteTimeoutMax: u8, pub BufWriteTimeoutMax: u8,
    pub BlockEraseTimeoutMax: u8, pub ChipEraseTimeoutMax: u8, pub DevSize: u8,
    pub InterfaceDesc: u16, pub MaxBufWriteSize: u16, pub NumEraseRegions: u8,
    pub EraseRegionInfo: [u32; 0],
}
#[repr(C, packed)] pub struct cfi_extquery { pub pri: [u8;3], pub MajorVersion: u8, pub MinorVersion: u8 }
#[repr(C, packed)] pub struct cfi_pri_intelext { pub pri:[u8;3], pub MajorVersion:u8, pub MinorVersion:u8, pub FeatureSupport:u32, pub SuspendCmdSupport:u8, pub BlkStatusRegMask:u16, pub VccOptimal:u8, pub VppOptimal:u8, pub NumProtectionFields:u8, pub ProtRegAddr:u16, pub FactProtRegSize:u8, pub UserProtRegSize:u8, pub extra:[u8;0] }
#[repr(C, packed)] pub struct cfi_intelext_otpinfo { pub ProtRegAddr:u32, pub FactGroups:u16, pub FactProtRegSize:u8, pub UserGroups:u16, pub UserProtRegSize:u8 }
#[repr(C, packed)] pub struct cfi_intelext_blockinfo { pub NumIdentBlocks:u16, pub BlockSize:u16, pub MinBlockEraseCycles:u16, pub BitsPerCell:u8, pub BlockCap:u8 }
#[repr(C, packed)] pub struct cfi_intelext_regioninfo { pub NumIdentPartitions:u16, pub NumOpAllowed:u8, pub NumOpAllowedSimProgMode:u8, pub NumOpAllowedSimEraMode:u8, pub NumBlockTypes:u8, pub BlockTypes:[cfi_intelext_blockinfo;1] }
#[repr(C, packed)] pub struct cfi_intelext_programming_regioninfo { pub ProgRegShift:u8, pub Reserved1:u8, pub ControlValid:u8, pub Reserved2:u8, pub ControlInvalid:u8, pub Reserved3:u8 }
#[repr(C, packed)] pub struct cfi_pri_amdstd { pub pri:[u8;3], pub MajorVersion:u8, pub MinorVersion:u8, pub SiliconRevision:u8, pub EraseSuspend:u8, pub BlkProt:u8, pub TmpBlkUnprotect:u8, pub BlkProtUnprot:u8, pub SimultaneousOps:u8, pub BurstMode:u8, pub PageMode:u8, pub VppMin:u8, pub VppMax:u8, pub TopBottom:u8, pub ProgramSuspend:u8, pub UnlockBypass:u8, pub SecureSiliconSector:u8, pub SoftwareFeatures:u8 }
pub const CFI_POLL_STATUS_REG: u8 = 1 << 0;
pub const CFI_POLL_DQ: u8 = 1 << 1;
#[repr(C, packed)] pub struct cfi_pri_atmel { pub pri:[u8;3], pub MajorVersion:u8, pub MinorVersion:u8, pub Features:u8, pub BottomBoot:u8, pub BurstMode:u8, pub PageMode:u8 }
#[repr(C, packed)] pub struct cfi_pri_query { pub NumFields:u8, pub ProtField:[u32;1] }
#[repr(C, packed)] pub struct cfi_bri_query { pub PageModeReadCap:u8, pub NumFields:u8, pub ConfField:[u32;1] }

pub const P_ID_NONE:u16=0x0000; pub const P_ID_INTEL_EXT:u16=0x0001; pub const P_ID_AMD_STD:u16=0x0002; pub const P_ID_INTEL_STD:u16=0x0003; pub const P_ID_AMD_EXT:u16=0x0004; pub const P_ID_WINBOND:u16=0x0006; pub const P_ID_ST_ADV:u16=0x0020; pub const P_ID_MITSUBISHI_STD:u16=0x0100; pub const P_ID_MITSUBISHI_EXT:u16=0x0101; pub const P_ID_SST_PAGE:u16=0x0102; pub const P_ID_SST_OLD:u16=0x0701; pub const P_ID_INTEL_PERFORMANCE:u16=0x0200; pub const P_ID_INTEL_DATA:u16=0x0210; pub const P_ID_RESERVED:u16=0xffff;
pub const CFI_MODE_CFI:i32=1; pub const CFI_MODE_JEDEC:i32=0;

#[repr(C)] pub struct cfi_private { pub cmdset:u16, pub cmdset_priv:*mut core::ffi::c_void, pub interleave:i32, pub device_type:i32, pub cfi_mode:i32, pub addr_unlock1:i32, pub addr_unlock2:i32, pub cmdset_setup:Option<unsafe extern "C" fn(*mut map_info)->*mut mtd_info>, pub cfiq:*mut cfi_ident, pub mfr:i32, pub id:i32, pub numchips:i32, pub sector_erase_cmd:map_word, pub chipshift:usize, pub im_name:*const core::ffi::c_char, pub quirks:usize, pub chips:[flchip;0] }

extern "C" { pub fn cfi_build_cmd_addr(cmd_ofs:u32,map:*mut map_info,cfi:*mut cfi_private)->u32; pub fn cfi_build_cmd(cmd:u64,map:*mut map_info,cfi:*mut cfi_private)->map_word; pub fn cfi_merge_status(val:map_word,map:*mut map_info,cfi:*mut cfi_private)->usize; pub fn cfi_send_gen_cmd(cmd:u8,cmd_addr:u32,base:u32,map:*mut map_info,cfi:*mut cfi_private,ty:i32,prev_val:*mut map_word)->u32; pub fn cfi_udelay(us:i32); pub fn cfi_qry_present(map:*mut map_info,base:u32,cfi:*mut cfi_private)->i32; pub fn cfi_qry_mode_on(base:u32,map:*mut map_info,cfi:*mut cfi_private)->i32; pub fn cfi_qry_mode_off(base:u32,map:*mut map_info,cfi:*mut cfi_private); pub fn cfi_read_pri(map:*mut map_info,adr:u16,size:u16,name:*const core::ffi::c_char)->*mut cfi_extquery; }

#[inline] pub unsafe fn cfi_read_query(map:*mut map_info,addr:u32)->u8 { let val=map_read(map,addr); if map_bankwidth_is_1(map){val.x[0]} else if map_bankwidth_is_2(map){cfi16_to_cpu(map,val.x[0]) as u8} else {cfi32_to_cpu(map,val.x[0]) as u8} }
#[inline] pub unsafe fn cfi_read_query16(map:*mut map_info,addr:u32)->u16 { let val=map_read(map,addr); if map_bankwidth_is_1(map){val.x[0]&0xff} else if map_bankwidth_is_2(map){cfi16_to_cpu(map,val.x[0])} else {cfi32_to_cpu(map,val.x[0]) as u16} }

#[repr(C)] pub struct cfi_fixup { pub mfr:u16, pub id:u16, pub fixup:Option<unsafe extern "C" fn(*mut mtd_info)> }
pub const CFI_MFR_ANY:u16=0xffff; pub const CFI_ID_ANY:u16=0xffff; pub const CFI_MFR_CONTINUATION:u16=0x007f; pub const CFI_MFR_AMD:u16=1; pub const CFI_MFR_AMIC:u16=0x37; pub const CFI_MFR_ATMEL:u16=0x1f; pub const CFI_MFR_EON:u16=0x1c; pub const CFI_MFR_FUJITSU:u16=4; pub const CFI_MFR_HYUNDAI:u16=0xad; pub const CFI_MFR_INTEL:u16=0x89; pub const CFI_MFR_MACRONIX:u16=0xc2; pub const CFI_MFR_NEC:u16=0x10; pub const CFI_MFR_PMC:u16=0x9d; pub const CFI_MFR_SAMSUNG:u16=0xec; pub const CFI_MFR_SHARP:u16=0xb0; pub const CFI_MFR_SST:u16=0xbf; pub const CFI_MFR_ST:u16=0x20; pub const CFI_MFR_MICRON:u16=0x2c; pub const CFI_MFR_TOSHIBA:u16=0x98; pub const CFI_MFR_WINBOND:u16=0xda;
extern "C" { pub fn cfi_fixup(mtd:*mut mtd_info,fixups:*mut cfi_fixup); }
pub type varsize_frob_t=unsafe extern "C" fn(*mut map_info,*mut flchip,usize,i32,*mut core::ffi::c_void)->i32;
extern "C" { pub fn cfi_varsize_frob(mtd:*mut mtd_info,frob:varsize_frob_t,ofs:i64,len:usize,thunk:*mut core::ffi::c_void)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
