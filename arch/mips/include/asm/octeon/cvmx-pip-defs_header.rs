// Translated from cvmx-pip-defs.h.
// C bitfield widths are retained in comments; fields use u64 storage.

extern "C" { pub fn CVMX_ADD_IO_SEG(value: u64) -> u64; }

#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cvmx_pip_port_parse_mode {
    CVMX_PIP_PORT_CFG_MODE_NONE = 0,
    CVMX_PIP_PORT_CFG_MODE_SKIPL2 = 1,
    CVMX_PIP_PORT_CFG_MODE_SKIPIP = 2,
}

#[inline] pub unsafe fn CVMX_PIP_ALT_SKIP_CFGX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002A00u64) + ((offset) & 3) * 8 }

#[inline] pub unsafe fn CVMX_PIP_BCK_PRS() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000038u64) }

#[inline] pub unsafe fn CVMX_PIP_BIST_STATUS() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000000u64) }

#[inline] pub unsafe fn CVMX_PIP_BSEL_EXT_CFGX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002800u64) + ((offset) & 3) * 16 }

#[inline] pub unsafe fn CVMX_PIP_BSEL_EXT_POSX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002808u64) + ((offset) & 3) * 16 }

#[inline] pub unsafe fn CVMX_PIP_BSEL_TBL_ENTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0003000u64) + ((offset) & 511) * 8 }

#[inline] pub unsafe fn CVMX_PIP_CLKEN() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000040u64) }

#[inline] pub unsafe fn CVMX_PIP_CRC_CTLX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000040u64) + ((offset) & 1) * 8 }

#[inline] pub unsafe fn CVMX_PIP_CRC_IVX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000050u64) + ((offset) & 1) * 8 }

#[inline] pub unsafe fn CVMX_PIP_DEC_IPSECX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000080u64) + ((offset) & 3) * 8 }

#[inline] pub unsafe fn CVMX_PIP_DSA_SRC_GRP() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000190u64) }

#[inline] pub unsafe fn CVMX_PIP_DSA_VID_GRP() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000198u64) }

#[inline] pub unsafe fn CVMX_PIP_FRM_LEN_CHKX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000180u64) + ((offset) & 1) * 8 }

#[inline] pub unsafe fn CVMX_PIP_GBL_CFG() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000028u64) }

#[inline] pub unsafe fn CVMX_PIP_GBL_CTL() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000020u64) }

#[inline] pub unsafe fn CVMX_PIP_HG_PRI_QOS() -> u64 { CVMX_ADD_IO_SEG(0x00011800A00001A0u64) }

#[inline] pub unsafe fn CVMX_PIP_INT_EN() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000010u64) }

#[inline] pub unsafe fn CVMX_PIP_INT_REG() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000008u64) }

#[inline] pub unsafe fn CVMX_PIP_IP_OFFSET() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000060u64) }

#[inline] pub unsafe fn CVMX_PIP_PRI_TBLX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0004000u64) + ((offset) & 255) * 8 }

#[inline] pub unsafe fn CVMX_PIP_PRT_CFGBX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0008000u64) + ((offset) & 63) * 8 }

#[inline] pub unsafe fn CVMX_PIP_PRT_CFGX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000200u64) + ((offset) & 63) * 8 }

#[inline] pub unsafe fn CVMX_PIP_PRT_TAGX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000400u64) + ((offset) & 63) * 8 }

#[inline] pub unsafe fn CVMX_PIP_QOS_DIFFX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000600u64) + ((offset) & 63) * 8 }

#[inline] pub unsafe fn CVMX_PIP_QOS_VLANX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A00000C0u64) + ((offset) & 7) * 8 }

#[inline] pub unsafe fn CVMX_PIP_QOS_WATCHX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000100u64) + ((offset) & 7) * 8 }

#[inline] pub unsafe fn CVMX_PIP_RAW_WORD() -> u64 { CVMX_ADD_IO_SEG(0x00011800A00000B0u64) }

#[inline] pub unsafe fn CVMX_PIP_SFT_RST() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000030u64) }

#[inline] pub unsafe fn CVMX_PIP_STAT0_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000800u64) + ((offset) & 63) * 80 }

#[inline] pub unsafe fn CVMX_PIP_STAT0_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040000u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT10_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0001480u64) + ((offset) & 63) * 16 }

#[inline] pub unsafe fn CVMX_PIP_STAT10_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040050u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT11_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0001488u64) + ((offset) & 63) * 16 }

#[inline] pub unsafe fn CVMX_PIP_STAT11_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040058u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT1_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000808u64) + ((offset) & 63) * 80 }

#[inline] pub unsafe fn CVMX_PIP_STAT1_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040008u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT2_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000810u64) + ((offset) & 63) * 80 }

#[inline] pub unsafe fn CVMX_PIP_STAT2_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040010u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT3_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000818u64) + ((offset) & 63) * 80 }

#[inline] pub unsafe fn CVMX_PIP_STAT3_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040018u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT4_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000820u64) + ((offset) & 63) * 80 }

#[inline] pub unsafe fn CVMX_PIP_STAT4_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040020u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT5_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000828u64) + ((offset) & 63) * 80 }

#[inline] pub unsafe fn CVMX_PIP_STAT5_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040028u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT6_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000830u64) + ((offset) & 63) * 80 }

#[inline] pub unsafe fn CVMX_PIP_STAT6_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040030u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT7_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000838u64) + ((offset) & 63) * 80 }

#[inline] pub unsafe fn CVMX_PIP_STAT7_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040038u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT8_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000840u64) + ((offset) & 63) * 80 }

#[inline] pub unsafe fn CVMX_PIP_STAT8_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040040u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT9_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000848u64) + ((offset) & 63) * 80 }

#[inline] pub unsafe fn CVMX_PIP_STAT9_X(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0040048u64) + ((offset) & 63) * 128 }

#[inline] pub unsafe fn CVMX_PIP_STAT_CTL() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000018u64) }

#[inline] pub unsafe fn CVMX_PIP_STAT_INB_ERRSX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0001A10u64) + ((offset) & 63) * 32 }

#[inline] pub unsafe fn CVMX_PIP_STAT_INB_ERRS_PKNDX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0020010u64) + ((offset) & 63) * 32 }

#[inline] pub unsafe fn CVMX_PIP_STAT_INB_OCTSX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0001A08u64) + ((offset) & 63) * 32 }

#[inline] pub unsafe fn CVMX_PIP_STAT_INB_OCTS_PKNDX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0020008u64) + ((offset) & 63) * 32 }

#[inline] pub unsafe fn CVMX_PIP_STAT_INB_PKTSX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0001A00u64) + ((offset) & 63) * 32 }

#[inline] pub unsafe fn CVMX_PIP_STAT_INB_PKTS_PKNDX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0020000u64) + ((offset) & 63) * 32 }

#[inline] pub unsafe fn CVMX_PIP_SUB_PKIND_FCSX(block_id: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0080000u64) }

#[inline] pub unsafe fn CVMX_PIP_TAG_INCX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0001800u64) + ((offset) & 63) * 8 }

#[inline] pub unsafe fn CVMX_PIP_TAG_MASK() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000070u64) }

#[inline] pub unsafe fn CVMX_PIP_TAG_SECRET() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000068u64) }

#[inline] pub unsafe fn CVMX_PIP_TODO_ENTRY() -> u64 { CVMX_ADD_IO_SEG(0x00011800A0000078u64) }

#[inline] pub unsafe fn CVMX_PIP_VLAN_ETYPESX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A00001C0u64) + ((offset) & 1) * 8 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT0_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002000u64) + ((offset) & 63) * 80 - 80*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT10_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0001700u64) + ((offset) & 63) * 16 - 16*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT11_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0001708u64) + ((offset) & 63) * 16 - 16*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT1_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002008u64) + ((offset) & 63) * 80 - 80*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT2_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002010u64) + ((offset) & 63) * 80 - 80*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT3_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002018u64) + ((offset) & 63) * 80 - 80*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT4_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002020u64) + ((offset) & 63) * 80 - 80*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT5_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002028u64) + ((offset) & 63) * 80 - 80*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT6_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002030u64) + ((offset) & 63) * 80 - 80*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT7_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002038u64) + ((offset) & 63) * 80 - 80*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT8_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002040u64) + ((offset) & 63) * 80 - 80*40 }

#[inline] pub unsafe fn CVMX_PIP_XSTAT9_PRTX(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x00011800A0002048u64) + ((offset) & 63) * 80 - 80*40 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_alt_skip_cfgx_s {
    pub skip1: u64, // C bitfield width: 7
    pub reserved_7_7: u64, // C bitfield width: 1
    pub skip2: u64, // C bitfield width: 7
    pub reserved_15_15: u64, // C bitfield width: 1
    pub skip3: u64, // C bitfield width: 7
    pub reserved_23_31: u64, // C bitfield width: 9
    pub bit0: u64, // C bitfield width: 6
    pub reserved_38_39: u64, // C bitfield width: 2
    pub bit1: u64, // C bitfield width: 6
    pub reserved_46_55: u64, // C bitfield width: 10
    pub len: u64, // C bitfield width: 1
    pub reserved_57_63: u64, // C bitfield width: 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_alt_skip_cfgx {
    pub u64: u64,
    pub cvmx_pip_alt_skip_cfgx_s: cvmx_pip_alt_skip_cfgx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_bck_prs_s {
    pub lowater: u64, // C bitfield width: 5
    pub reserved_5_7: u64, // C bitfield width: 3
    pub hiwater: u64, // C bitfield width: 5
    pub reserved_13_62: u64, // C bitfield width: 50
    pub bckprs: u64, // C bitfield width: 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_bck_prs {
    pub u64: u64,
    pub cvmx_pip_bck_prs_s: cvmx_pip_bck_prs_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_bist_status_s {
    pub bist: u64, // C bitfield width: 22
    pub reserved_22_63: u64, // C bitfield width: 42
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_bist_status_cn30xx {
    pub bist: u64, // C bitfield width: 18
    pub reserved_18_63: u64, // C bitfield width: 46
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_bist_status_cn50xx {
    pub bist: u64, // C bitfield width: 17
    pub reserved_17_63: u64, // C bitfield width: 47
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_bist_status_cn61xx {
    pub bist: u64, // C bitfield width: 20
    pub reserved_20_63: u64, // C bitfield width: 44
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_bist_status {
    pub u64: u64,
    pub cvmx_pip_bist_status_s: cvmx_pip_bist_status_s,
    pub cvmx_pip_bist_status_cn30xx: cvmx_pip_bist_status_cn30xx,
    pub cvmx_pip_bist_status_cn50xx: cvmx_pip_bist_status_cn50xx,
    pub cvmx_pip_bist_status_cn61xx: cvmx_pip_bist_status_cn61xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_bsel_ext_cfgx_s {
    pub skip: u64, // C bitfield width: 7
    pub reserved_7_15: u64, // C bitfield width: 9
    pub offset: u64, // C bitfield width: 9
    pub reserved_25_31: u64, // C bitfield width: 7
    pub tag: u64, // C bitfield width: 8
    pub upper_tag: u64, // C bitfield width: 16
    pub reserved_56_63: u64, // C bitfield width: 8
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_bsel_ext_cfgx {
    pub u64: u64,
    pub cvmx_pip_bsel_ext_cfgx_s: cvmx_pip_bsel_ext_cfgx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_bsel_ext_posx_s {
    pub pos0: u64, // C bitfield width: 7
    pub pos0_val: u64, // C bitfield width: 1
    pub pos1: u64, // C bitfield width: 7
    pub pos1_val: u64, // C bitfield width: 1
    pub pos2: u64, // C bitfield width: 7
    pub pos2_val: u64, // C bitfield width: 1
    pub pos3: u64, // C bitfield width: 7
    pub pos3_val: u64, // C bitfield width: 1
    pub pos4: u64, // C bitfield width: 7
    pub pos4_val: u64, // C bitfield width: 1
    pub pos5: u64, // C bitfield width: 7
    pub pos5_val: u64, // C bitfield width: 1
    pub pos6: u64, // C bitfield width: 7
    pub pos6_val: u64, // C bitfield width: 1
    pub pos7: u64, // C bitfield width: 7
    pub pos7_val: u64, // C bitfield width: 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_bsel_ext_posx {
    pub u64: u64,
    pub cvmx_pip_bsel_ext_posx_s: cvmx_pip_bsel_ext_posx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_bsel_tbl_entx_s {
    pub qos: u64, // C bitfield width: 3
    pub reserved_3_7: u64, // C bitfield width: 5
    pub tt: u64, // C bitfield width: 2
    pub reserved_10_15: u64, // C bitfield width: 6
    pub grp: u64, // C bitfield width: 6
    pub reserved_22_31: u64, // C bitfield width: 10
    pub tag: u64, // C bitfield width: 8
    pub reserved_40_59: u64, // C bitfield width: 20
    pub qos_en: u64, // C bitfield width: 1
    pub tt_en: u64, // C bitfield width: 1
    pub grp_en: u64, // C bitfield width: 1
    pub tag_en: u64, // C bitfield width: 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_bsel_tbl_entx_cn61xx {
    pub qos: u64, // C bitfield width: 3
    pub reserved_3_7: u64, // C bitfield width: 5
    pub tt: u64, // C bitfield width: 2
    pub reserved_10_15: u64, // C bitfield width: 6
    pub grp: u64, // C bitfield width: 4
    pub reserved_20_31: u64, // C bitfield width: 12
    pub tag: u64, // C bitfield width: 8
    pub reserved_40_59: u64, // C bitfield width: 20
    pub qos_en: u64, // C bitfield width: 1
    pub tt_en: u64, // C bitfield width: 1
    pub grp_en: u64, // C bitfield width: 1
    pub tag_en: u64, // C bitfield width: 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_bsel_tbl_entx {
    pub u64: u64,
    pub cvmx_pip_bsel_tbl_entx_s: cvmx_pip_bsel_tbl_entx_s,
    pub cvmx_pip_bsel_tbl_entx_cn61xx: cvmx_pip_bsel_tbl_entx_cn61xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_clken_s {
    pub clken: u64, // C bitfield width: 1
    pub reserved_1_63: u64, // C bitfield width: 63
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_clken {
    pub u64: u64,
    pub cvmx_pip_clken_s: cvmx_pip_clken_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_crc_ctlx_s {
    pub reflect: u64, // C bitfield width: 1
    pub invres: u64, // C bitfield width: 1
    pub reserved_2_63: u64, // C bitfield width: 62
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_crc_ctlx {
    pub u64: u64,
    pub cvmx_pip_crc_ctlx_s: cvmx_pip_crc_ctlx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_crc_ivx_s {
    pub iv: u64, // C bitfield width: 32
    pub reserved_32_63: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_crc_ivx {
    pub u64: u64,
    pub cvmx_pip_crc_ivx_s: cvmx_pip_crc_ivx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_dec_ipsecx_s {
    pub dprt: u64, // C bitfield width: 16
    pub udp: u64, // C bitfield width: 1
    pub tcp: u64, // C bitfield width: 1
    pub reserved_18_63: u64, // C bitfield width: 46
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_dec_ipsecx {
    pub u64: u64,
    pub cvmx_pip_dec_ipsecx_s: cvmx_pip_dec_ipsecx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_dsa_src_grp_s {
    pub map0: u64, // C bitfield width: 4
    pub map1: u64, // C bitfield width: 4
    pub map2: u64, // C bitfield width: 4
    pub map3: u64, // C bitfield width: 4
    pub map4: u64, // C bitfield width: 4
    pub map5: u64, // C bitfield width: 4
    pub map6: u64, // C bitfield width: 4
    pub map7: u64, // C bitfield width: 4
    pub map8: u64, // C bitfield width: 4
    pub map9: u64, // C bitfield width: 4
    pub map10: u64, // C bitfield width: 4
    pub map11: u64, // C bitfield width: 4
    pub map12: u64, // C bitfield width: 4
    pub map13: u64, // C bitfield width: 4
    pub map14: u64, // C bitfield width: 4
    pub map15: u64, // C bitfield width: 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_dsa_src_grp {
    pub u64: u64,
    pub cvmx_pip_dsa_src_grp_s: cvmx_pip_dsa_src_grp_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_dsa_vid_grp_s {
    pub map0: u64, // C bitfield width: 4
    pub map1: u64, // C bitfield width: 4
    pub map2: u64, // C bitfield width: 4
    pub map3: u64, // C bitfield width: 4
    pub map4: u64, // C bitfield width: 4
    pub map5: u64, // C bitfield width: 4
    pub map6: u64, // C bitfield width: 4
    pub map7: u64, // C bitfield width: 4
    pub map8: u64, // C bitfield width: 4
    pub map9: u64, // C bitfield width: 4
    pub map10: u64, // C bitfield width: 4
    pub map11: u64, // C bitfield width: 4
    pub map12: u64, // C bitfield width: 4
    pub map13: u64, // C bitfield width: 4
    pub map14: u64, // C bitfield width: 4
    pub map15: u64, // C bitfield width: 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_dsa_vid_grp {
    pub u64: u64,
    pub cvmx_pip_dsa_vid_grp_s: cvmx_pip_dsa_vid_grp_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_frm_len_chkx_s {
    pub minlen: u64, // C bitfield width: 16
    pub maxlen: u64, // C bitfield width: 16
    pub reserved_32_63: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_frm_len_chkx {
    pub u64: u64,
    pub cvmx_pip_frm_len_chkx_s: cvmx_pip_frm_len_chkx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_gbl_cfg_s {
    pub nip_shf: u64, // C bitfield width: 3
    pub reserved_3_7: u64, // C bitfield width: 5
    pub raw_shf: u64, // C bitfield width: 3
    pub reserved_11_15: u64, // C bitfield width: 5
    pub max_l2: u64, // C bitfield width: 1
    pub ip6_udp: u64, // C bitfield width: 1
    pub tag_syn: u64, // C bitfield width: 1
    pub reserved_19_63: u64, // C bitfield width: 45
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_gbl_cfg {
    pub u64: u64,
    pub cvmx_pip_gbl_cfg_s: cvmx_pip_gbl_cfg_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_gbl_ctl_s {
    pub ip_chk: u64, // C bitfield width: 1
    pub ip_mal: u64, // C bitfield width: 1
    pub ip_hop: u64, // C bitfield width: 1
    pub ip4_opts: u64, // C bitfield width: 1
    pub ip6_eext: u64, // C bitfield width: 2
    pub reserved_6_7: u64, // C bitfield width: 2
    pub l4_mal: u64, // C bitfield width: 1
    pub l4_prt: u64, // C bitfield width: 1
    pub l4_chk: u64, // C bitfield width: 1
    pub l4_len: u64, // C bitfield width: 1
    pub tcp_flag: u64, // C bitfield width: 1
    pub l2_mal: u64, // C bitfield width: 1
    pub vs_qos: u64, // C bitfield width: 1
    pub vs_wqe: u64, // C bitfield width: 1
    pub ignrs: u64, // C bitfield width: 1
    pub reserved_17_19: u64, // C bitfield width: 3
    pub ring_en: u64, // C bitfield width: 1
    pub reserved_21_23: u64, // C bitfield width: 3
    pub dsa_grp_sid: u64, // C bitfield width: 1
    pub dsa_grp_scmd: u64, // C bitfield width: 1
    pub dsa_grp_tvid: u64, // C bitfield width: 1
    pub ihmsk_dis: u64, // C bitfield width: 1
    pub egrp_dis: u64, // C bitfield width: 1
    pub reserved_29_63: u64, // C bitfield width: 35
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_gbl_ctl_cn30xx {
    pub ip_chk: u64, // C bitfield width: 1
    pub ip_mal: u64, // C bitfield width: 1
    pub ip_hop: u64, // C bitfield width: 1
    pub ip4_opts: u64, // C bitfield width: 1
    pub ip6_eext: u64, // C bitfield width: 2
    pub reserved_6_7: u64, // C bitfield width: 2
    pub l4_mal: u64, // C bitfield width: 1
    pub l4_prt: u64, // C bitfield width: 1
    pub l4_chk: u64, // C bitfield width: 1
    pub l4_len: u64, // C bitfield width: 1
    pub tcp_flag: u64, // C bitfield width: 1
    pub l2_mal: u64, // C bitfield width: 1
    pub vs_qos: u64, // C bitfield width: 1
    pub vs_wqe: u64, // C bitfield width: 1
    pub ignrs: u64, // C bitfield width: 1
    pub reserved_17_63: u64, // C bitfield width: 47
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_gbl_ctl_cn52xx {
    pub ip_chk: u64, // C bitfield width: 1
    pub ip_mal: u64, // C bitfield width: 1
    pub ip_hop: u64, // C bitfield width: 1
    pub ip4_opts: u64, // C bitfield width: 1
    pub ip6_eext: u64, // C bitfield width: 2
    pub reserved_6_7: u64, // C bitfield width: 2
    pub l4_mal: u64, // C bitfield width: 1
    pub l4_prt: u64, // C bitfield width: 1
    pub l4_chk: u64, // C bitfield width: 1
    pub l4_len: u64, // C bitfield width: 1
    pub tcp_flag: u64, // C bitfield width: 1
    pub l2_mal: u64, // C bitfield width: 1
    pub vs_qos: u64, // C bitfield width: 1
    pub vs_wqe: u64, // C bitfield width: 1
    pub ignrs: u64, // C bitfield width: 1
    pub reserved_17_19: u64, // C bitfield width: 3
    pub ring_en: u64, // C bitfield width: 1
    pub reserved_21_23: u64, // C bitfield width: 3
    pub dsa_grp_sid: u64, // C bitfield width: 1
    pub dsa_grp_scmd: u64, // C bitfield width: 1
    pub dsa_grp_tvid: u64, // C bitfield width: 1
    pub reserved_27_63: u64, // C bitfield width: 37
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_gbl_ctl_cn56xxp1 {
    pub ip_chk: u64, // C bitfield width: 1
    pub ip_mal: u64, // C bitfield width: 1
    pub ip_hop: u64, // C bitfield width: 1
    pub ip4_opts: u64, // C bitfield width: 1
    pub ip6_eext: u64, // C bitfield width: 2
    pub reserved_6_7: u64, // C bitfield width: 2
    pub l4_mal: u64, // C bitfield width: 1
    pub l4_prt: u64, // C bitfield width: 1
    pub l4_chk: u64, // C bitfield width: 1
    pub l4_len: u64, // C bitfield width: 1
    pub tcp_flag: u64, // C bitfield width: 1
    pub l2_mal: u64, // C bitfield width: 1
    pub vs_qos: u64, // C bitfield width: 1
    pub vs_wqe: u64, // C bitfield width: 1
    pub ignrs: u64, // C bitfield width: 1
    pub reserved_17_19: u64, // C bitfield width: 3
    pub ring_en: u64, // C bitfield width: 1
    pub reserved_21_63: u64, // C bitfield width: 43
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_gbl_ctl_cn61xx {
    pub ip_chk: u64, // C bitfield width: 1
    pub ip_mal: u64, // C bitfield width: 1
    pub ip_hop: u64, // C bitfield width: 1
    pub ip4_opts: u64, // C bitfield width: 1
    pub ip6_eext: u64, // C bitfield width: 2
    pub reserved_6_7: u64, // C bitfield width: 2
    pub l4_mal: u64, // C bitfield width: 1
    pub l4_prt: u64, // C bitfield width: 1
    pub l4_chk: u64, // C bitfield width: 1
    pub l4_len: u64, // C bitfield width: 1
    pub tcp_flag: u64, // C bitfield width: 1
    pub l2_mal: u64, // C bitfield width: 1
    pub vs_qos: u64, // C bitfield width: 1
    pub vs_wqe: u64, // C bitfield width: 1
    pub ignrs: u64, // C bitfield width: 1
    pub reserved_17_19: u64, // C bitfield width: 3
    pub ring_en: u64, // C bitfield width: 1
    pub reserved_21_23: u64, // C bitfield width: 3
    pub dsa_grp_sid: u64, // C bitfield width: 1
    pub dsa_grp_scmd: u64, // C bitfield width: 1
    pub dsa_grp_tvid: u64, // C bitfield width: 1
    pub ihmsk_dis: u64, // C bitfield width: 1
    pub reserved_28_63: u64, // C bitfield width: 36
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_gbl_ctl_cn68xx {
    pub ip_chk: u64, // C bitfield width: 1
    pub ip_mal: u64, // C bitfield width: 1
    pub ip_hop: u64, // C bitfield width: 1
    pub ip4_opts: u64, // C bitfield width: 1
    pub ip6_eext: u64, // C bitfield width: 2
    pub reserved_6_7: u64, // C bitfield width: 2
    pub l4_mal: u64, // C bitfield width: 1
    pub l4_prt: u64, // C bitfield width: 1
    pub l4_chk: u64, // C bitfield width: 1
    pub l4_len: u64, // C bitfield width: 1
    pub tcp_flag: u64, // C bitfield width: 1
    pub l2_mal: u64, // C bitfield width: 1
    pub vs_qos: u64, // C bitfield width: 1
    pub vs_wqe: u64, // C bitfield width: 1
    pub ignrs: u64, // C bitfield width: 1
    pub reserved_17_23: u64, // C bitfield width: 7
    pub dsa_grp_sid: u64, // C bitfield width: 1
    pub dsa_grp_scmd: u64, // C bitfield width: 1
    pub dsa_grp_tvid: u64, // C bitfield width: 1
    pub ihmsk_dis: u64, // C bitfield width: 1
    pub egrp_dis: u64, // C bitfield width: 1
    pub reserved_29_63: u64, // C bitfield width: 35
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_gbl_ctl_cn68xxp1 {
    pub ip_chk: u64, // C bitfield width: 1
    pub ip_mal: u64, // C bitfield width: 1
    pub ip_hop: u64, // C bitfield width: 1
    pub ip4_opts: u64, // C bitfield width: 1
    pub ip6_eext: u64, // C bitfield width: 2
    pub reserved_6_7: u64, // C bitfield width: 2
    pub l4_mal: u64, // C bitfield width: 1
    pub l4_prt: u64, // C bitfield width: 1
    pub l4_chk: u64, // C bitfield width: 1
    pub l4_len: u64, // C bitfield width: 1
    pub tcp_flag: u64, // C bitfield width: 1
    pub l2_mal: u64, // C bitfield width: 1
    pub vs_qos: u64, // C bitfield width: 1
    pub vs_wqe: u64, // C bitfield width: 1
    pub ignrs: u64, // C bitfield width: 1
    pub reserved_17_23: u64, // C bitfield width: 7
    pub dsa_grp_sid: u64, // C bitfield width: 1
    pub dsa_grp_scmd: u64, // C bitfield width: 1
    pub dsa_grp_tvid: u64, // C bitfield width: 1
    pub ihmsk_dis: u64, // C bitfield width: 1
    pub reserved_28_63: u64, // C bitfield width: 36
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_gbl_ctl {
    pub u64: u64,
    pub cvmx_pip_gbl_ctl_s: cvmx_pip_gbl_ctl_s,
    pub cvmx_pip_gbl_ctl_cn30xx: cvmx_pip_gbl_ctl_cn30xx,
    pub cvmx_pip_gbl_ctl_cn52xx: cvmx_pip_gbl_ctl_cn52xx,
    pub cvmx_pip_gbl_ctl_cn56xxp1: cvmx_pip_gbl_ctl_cn56xxp1,
    pub cvmx_pip_gbl_ctl_cn61xx: cvmx_pip_gbl_ctl_cn61xx,
    pub cvmx_pip_gbl_ctl_cn68xx: cvmx_pip_gbl_ctl_cn68xx,
    pub cvmx_pip_gbl_ctl_cn68xxp1: cvmx_pip_gbl_ctl_cn68xxp1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_hg_pri_qos_s {
    pub pri: u64, // C bitfield width: 6
    pub reserved_6_7: u64, // C bitfield width: 2
    pub qos: u64, // C bitfield width: 3
    pub reserved_11_11: u64, // C bitfield width: 1
    pub up_qos: u64, // C bitfield width: 1
    pub reserved_13_63: u64, // C bitfield width: 51
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_hg_pri_qos {
    pub u64: u64,
    pub cvmx_pip_hg_pri_qos_s: cvmx_pip_hg_pri_qos_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_en_s {
    pub pktdrp: u64, // C bitfield width: 1
    pub crcerr: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub minerr: u64, // C bitfield width: 1
    pub maxerr: u64, // C bitfield width: 1
    pub lenerr: u64, // C bitfield width: 1
    pub punyerr: u64, // C bitfield width: 1
    pub reserved_13_63: u64, // C bitfield width: 51
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_en_cn30xx {
    pub pktdrp: u64, // C bitfield width: 1
    pub crcerr: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub reserved_9_63: u64, // C bitfield width: 55
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_en_cn50xx {
    pub pktdrp: u64, // C bitfield width: 1
    pub reserved_1_1: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub minerr: u64, // C bitfield width: 1
    pub maxerr: u64, // C bitfield width: 1
    pub lenerr: u64, // C bitfield width: 1
    pub reserved_12_63: u64, // C bitfield width: 52
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_en_cn52xx {
    pub pktdrp: u64, // C bitfield width: 1
    pub reserved_1_1: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub minerr: u64, // C bitfield width: 1
    pub maxerr: u64, // C bitfield width: 1
    pub lenerr: u64, // C bitfield width: 1
    pub punyerr: u64, // C bitfield width: 1
    pub reserved_13_63: u64, // C bitfield width: 51
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_en_cn56xxp1 {
    pub pktdrp: u64, // C bitfield width: 1
    pub crcerr: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub minerr: u64, // C bitfield width: 1
    pub maxerr: u64, // C bitfield width: 1
    pub lenerr: u64, // C bitfield width: 1
    pub reserved_12_63: u64, // C bitfield width: 52
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_en_cn58xx {
    pub pktdrp: u64, // C bitfield width: 1
    pub crcerr: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub reserved_9_11: u64, // C bitfield width: 3
    pub punyerr: u64, // C bitfield width: 1
    pub reserved_13_63: u64, // C bitfield width: 51
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_int_en {
    pub u64: u64,
    pub cvmx_pip_int_en_s: cvmx_pip_int_en_s,
    pub cvmx_pip_int_en_cn30xx: cvmx_pip_int_en_cn30xx,
    pub cvmx_pip_int_en_cn50xx: cvmx_pip_int_en_cn50xx,
    pub cvmx_pip_int_en_cn52xx: cvmx_pip_int_en_cn52xx,
    pub cvmx_pip_int_en_cn56xxp1: cvmx_pip_int_en_cn56xxp1,
    pub cvmx_pip_int_en_cn58xx: cvmx_pip_int_en_cn58xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_reg_s {
    pub pktdrp: u64, // C bitfield width: 1
    pub crcerr: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub minerr: u64, // C bitfield width: 1
    pub maxerr: u64, // C bitfield width: 1
    pub lenerr: u64, // C bitfield width: 1
    pub punyerr: u64, // C bitfield width: 1
    pub reserved_13_63: u64, // C bitfield width: 51
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_reg_cn30xx {
    pub pktdrp: u64, // C bitfield width: 1
    pub crcerr: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub reserved_9_63: u64, // C bitfield width: 55
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_reg_cn50xx {
    pub pktdrp: u64, // C bitfield width: 1
    pub reserved_1_1: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub minerr: u64, // C bitfield width: 1
    pub maxerr: u64, // C bitfield width: 1
    pub lenerr: u64, // C bitfield width: 1
    pub reserved_12_63: u64, // C bitfield width: 52
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_reg_cn52xx {
    pub pktdrp: u64, // C bitfield width: 1
    pub reserved_1_1: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub minerr: u64, // C bitfield width: 1
    pub maxerr: u64, // C bitfield width: 1
    pub lenerr: u64, // C bitfield width: 1
    pub punyerr: u64, // C bitfield width: 1
    pub reserved_13_63: u64, // C bitfield width: 51
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_reg_cn56xxp1 {
    pub pktdrp: u64, // C bitfield width: 1
    pub crcerr: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub minerr: u64, // C bitfield width: 1
    pub maxerr: u64, // C bitfield width: 1
    pub lenerr: u64, // C bitfield width: 1
    pub reserved_12_63: u64, // C bitfield width: 52
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_int_reg_cn58xx {
    pub pktdrp: u64, // C bitfield width: 1
    pub crcerr: u64, // C bitfield width: 1
    pub bckprs: u64, // C bitfield width: 1
    pub prtnxa: u64, // C bitfield width: 1
    pub badtag: u64, // C bitfield width: 1
    pub skprunt: u64, // C bitfield width: 1
    pub todoovr: u64, // C bitfield width: 1
    pub feperr: u64, // C bitfield width: 1
    pub beperr: u64, // C bitfield width: 1
    pub reserved_9_11: u64, // C bitfield width: 3
    pub punyerr: u64, // C bitfield width: 1
    pub reserved_13_63: u64, // C bitfield width: 51
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_int_reg {
    pub u64: u64,
    pub cvmx_pip_int_reg_s: cvmx_pip_int_reg_s,
    pub cvmx_pip_int_reg_cn30xx: cvmx_pip_int_reg_cn30xx,
    pub cvmx_pip_int_reg_cn50xx: cvmx_pip_int_reg_cn50xx,
    pub cvmx_pip_int_reg_cn52xx: cvmx_pip_int_reg_cn52xx,
    pub cvmx_pip_int_reg_cn56xxp1: cvmx_pip_int_reg_cn56xxp1,
    pub cvmx_pip_int_reg_cn58xx: cvmx_pip_int_reg_cn58xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_ip_offset_s {
    pub offset: u64, // C bitfield width: 3
    pub reserved_3_63: u64, // C bitfield width: 61
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_ip_offset {
    pub u64: u64,
    pub cvmx_pip_ip_offset_s: cvmx_pip_ip_offset_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_pri_tblx_s {
    pub vlan2_qos: u64, // C bitfield width: 3
    pub reserved_3_3: u64, // C bitfield width: 1
    pub hg2_qos: u64, // C bitfield width: 3
    pub reserved_7_7: u64, // C bitfield width: 1
    pub diff2_qos: u64, // C bitfield width: 3
    pub reserved_11_15: u64, // C bitfield width: 5
    pub vlan2_bpid: u64, // C bitfield width: 6
    pub reserved_22_23: u64, // C bitfield width: 2
    pub hg2_bpid: u64, // C bitfield width: 6
    pub reserved_30_31: u64, // C bitfield width: 2
    pub diff2_bpid: u64, // C bitfield width: 6
    pub reserved_38_39: u64, // C bitfield width: 2
    pub vlan2_padd: u64, // C bitfield width: 8
    pub hg2_padd: u64, // C bitfield width: 8
    pub diff2_padd: u64, // C bitfield width: 8
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_pri_tblx {
    pub u64: u64,
    pub cvmx_pip_pri_tblx_s: cvmx_pip_pri_tblx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgx_s {
    pub skip: u64, // C bitfield width: 7
    pub reserved_7_7: u64, // C bitfield width: 1
    pub mode: u64, // C bitfield width: 2
    pub dsa_en: u64, // C bitfield width: 1
    pub higig_en: u64, // C bitfield width: 1
    pub crc_en: u64, // C bitfield width: 1
    pub reserved_13_15: u64, // C bitfield width: 3
    pub qos_vlan: u64, // C bitfield width: 1
    pub qos_diff: u64, // C bitfield width: 1
    pub qos_vod: u64, // C bitfield width: 1
    pub qos_vsel: u64, // C bitfield width: 1
    pub qos_wat: u64, // C bitfield width: 4
    pub qos: u64, // C bitfield width: 3
    pub hg_qos: u64, // C bitfield width: 1
    pub grp_wat: u64, // C bitfield width: 4
    pub inst_hdr: u64, // C bitfield width: 1
    pub dyn_rs: u64, // C bitfield width: 1
    pub tag_inc: u64, // C bitfield width: 2
    pub rawdrp: u64, // C bitfield width: 1
    pub reserved_37_39: u64, // C bitfield width: 3
    pub qos_wat_47: u64, // C bitfield width: 4
    pub grp_wat_47: u64, // C bitfield width: 4
    pub minerr_en: u64, // C bitfield width: 1
    pub maxerr_en: u64, // C bitfield width: 1
    pub lenerr_en: u64, // C bitfield width: 1
    pub vlan_len: u64, // C bitfield width: 1
    pub pad_len: u64, // C bitfield width: 1
    pub len_chk_sel: u64, // C bitfield width: 1
    pub ih_pri: u64, // C bitfield width: 1
    pub reserved_55_63: u64, // C bitfield width: 9
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgx_cn30xx {
    pub skip: u64, // C bitfield width: 7
    pub reserved_7_7: u64, // C bitfield width: 1
    pub mode: u64, // C bitfield width: 2
    pub reserved_10_15: u64, // C bitfield width: 6
    pub qos_vlan: u64, // C bitfield width: 1
    pub qos_diff: u64, // C bitfield width: 1
    pub reserved_18_19: u64, // C bitfield width: 2
    pub qos_wat: u64, // C bitfield width: 4
    pub qos: u64, // C bitfield width: 3
    pub reserved_27_27: u64, // C bitfield width: 1
    pub grp_wat: u64, // C bitfield width: 4
    pub inst_hdr: u64, // C bitfield width: 1
    pub dyn_rs: u64, // C bitfield width: 1
    pub tag_inc: u64, // C bitfield width: 2
    pub rawdrp: u64, // C bitfield width: 1
    pub reserved_37_63: u64, // C bitfield width: 27
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgx_cn38xx {
    pub skip: u64, // C bitfield width: 7
    pub reserved_7_7: u64, // C bitfield width: 1
    pub mode: u64, // C bitfield width: 2
    pub reserved_10_11: u64, // C bitfield width: 2
    pub crc_en: u64, // C bitfield width: 1
    pub reserved_13_15: u64, // C bitfield width: 3
    pub qos_vlan: u64, // C bitfield width: 1
    pub qos_diff: u64, // C bitfield width: 1
    pub reserved_18_19: u64, // C bitfield width: 2
    pub qos_wat: u64, // C bitfield width: 4
    pub qos: u64, // C bitfield width: 3
    pub reserved_27_27: u64, // C bitfield width: 1
    pub grp_wat: u64, // C bitfield width: 4
    pub inst_hdr: u64, // C bitfield width: 1
    pub dyn_rs: u64, // C bitfield width: 1
    pub tag_inc: u64, // C bitfield width: 2
    pub rawdrp: u64, // C bitfield width: 1
    pub reserved_37_63: u64, // C bitfield width: 27
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgx_cn50xx {
    pub skip: u64, // C bitfield width: 7
    pub reserved_7_7: u64, // C bitfield width: 1
    pub mode: u64, // C bitfield width: 2
    pub reserved_10_11: u64, // C bitfield width: 2
    pub crc_en: u64, // C bitfield width: 1
    pub reserved_13_15: u64, // C bitfield width: 3
    pub qos_vlan: u64, // C bitfield width: 1
    pub qos_diff: u64, // C bitfield width: 1
    pub qos_vod: u64, // C bitfield width: 1
    pub reserved_19_19: u64, // C bitfield width: 1
    pub qos_wat: u64, // C bitfield width: 4
    pub qos: u64, // C bitfield width: 3
    pub reserved_27_27: u64, // C bitfield width: 1
    pub grp_wat: u64, // C bitfield width: 4
    pub inst_hdr: u64, // C bitfield width: 1
    pub dyn_rs: u64, // C bitfield width: 1
    pub tag_inc: u64, // C bitfield width: 2
    pub rawdrp: u64, // C bitfield width: 1
    pub reserved_37_39: u64, // C bitfield width: 3
    pub qos_wat_47: u64, // C bitfield width: 4
    pub grp_wat_47: u64, // C bitfield width: 4
    pub minerr_en: u64, // C bitfield width: 1
    pub maxerr_en: u64, // C bitfield width: 1
    pub lenerr_en: u64, // C bitfield width: 1
    pub vlan_len: u64, // C bitfield width: 1
    pub pad_len: u64, // C bitfield width: 1
    pub reserved_53_63: u64, // C bitfield width: 11
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgx_cn52xx {
    pub skip: u64, // C bitfield width: 7
    pub reserved_7_7: u64, // C bitfield width: 1
    pub mode: u64, // C bitfield width: 2
    pub dsa_en: u64, // C bitfield width: 1
    pub higig_en: u64, // C bitfield width: 1
    pub crc_en: u64, // C bitfield width: 1
    pub reserved_13_15: u64, // C bitfield width: 3
    pub qos_vlan: u64, // C bitfield width: 1
    pub qos_diff: u64, // C bitfield width: 1
    pub qos_vod: u64, // C bitfield width: 1
    pub qos_vsel: u64, // C bitfield width: 1
    pub qos_wat: u64, // C bitfield width: 4
    pub qos: u64, // C bitfield width: 3
    pub hg_qos: u64, // C bitfield width: 1
    pub grp_wat: u64, // C bitfield width: 4
    pub inst_hdr: u64, // C bitfield width: 1
    pub dyn_rs: u64, // C bitfield width: 1
    pub tag_inc: u64, // C bitfield width: 2
    pub rawdrp: u64, // C bitfield width: 1
    pub reserved_37_39: u64, // C bitfield width: 3
    pub qos_wat_47: u64, // C bitfield width: 4
    pub grp_wat_47: u64, // C bitfield width: 4
    pub minerr_en: u64, // C bitfield width: 1
    pub maxerr_en: u64, // C bitfield width: 1
    pub lenerr_en: u64, // C bitfield width: 1
    pub vlan_len: u64, // C bitfield width: 1
    pub pad_len: u64, // C bitfield width: 1
    pub reserved_53_63: u64, // C bitfield width: 11
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgx_cn58xx {
    pub skip: u64, // C bitfield width: 7
    pub reserved_7_7: u64, // C bitfield width: 1
    pub mode: u64, // C bitfield width: 2
    pub reserved_10_11: u64, // C bitfield width: 2
    pub crc_en: u64, // C bitfield width: 1
    pub reserved_13_15: u64, // C bitfield width: 3
    pub qos_vlan: u64, // C bitfield width: 1
    pub qos_diff: u64, // C bitfield width: 1
    pub qos_vod: u64, // C bitfield width: 1
    pub reserved_19_19: u64, // C bitfield width: 1
    pub qos_wat: u64, // C bitfield width: 4
    pub qos: u64, // C bitfield width: 3
    pub reserved_27_27: u64, // C bitfield width: 1
    pub grp_wat: u64, // C bitfield width: 4
    pub inst_hdr: u64, // C bitfield width: 1
    pub dyn_rs: u64, // C bitfield width: 1
    pub tag_inc: u64, // C bitfield width: 2
    pub rawdrp: u64, // C bitfield width: 1
    pub reserved_37_63: u64, // C bitfield width: 27
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgx_cn68xx {
    pub skip: u64, // C bitfield width: 7
    pub reserved_7_7: u64, // C bitfield width: 1
    pub mode: u64, // C bitfield width: 2
    pub dsa_en: u64, // C bitfield width: 1
    pub higig_en: u64, // C bitfield width: 1
    pub crc_en: u64, // C bitfield width: 1
    pub reserved_13_15: u64, // C bitfield width: 3
    pub qos_vlan: u64, // C bitfield width: 1
    pub qos_diff: u64, // C bitfield width: 1
    pub qos_vod: u64, // C bitfield width: 1
    pub reserved_19_19: u64, // C bitfield width: 1
    pub qos_wat: u64, // C bitfield width: 4
    pub qos: u64, // C bitfield width: 3
    pub hg_qos: u64, // C bitfield width: 1
    pub grp_wat: u64, // C bitfield width: 4
    pub inst_hdr: u64, // C bitfield width: 1
    pub dyn_rs: u64, // C bitfield width: 1
    pub tag_inc: u64, // C bitfield width: 2
    pub rawdrp: u64, // C bitfield width: 1
    pub reserved_37_39: u64, // C bitfield width: 3
    pub qos_wat_47: u64, // C bitfield width: 4
    pub grp_wat_47: u64, // C bitfield width: 4
    pub minerr_en: u64, // C bitfield width: 1
    pub maxerr_en: u64, // C bitfield width: 1
    pub lenerr_en: u64, // C bitfield width: 1
    pub vlan_len: u64, // C bitfield width: 1
    pub pad_len: u64, // C bitfield width: 1
    pub len_chk_sel: u64, // C bitfield width: 1
    pub ih_pri: u64, // C bitfield width: 1
    pub reserved_55_63: u64, // C bitfield width: 9
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_prt_cfgx {
    pub u64: u64,
    pub cvmx_pip_prt_cfgx_s: cvmx_pip_prt_cfgx_s,
    pub cvmx_pip_prt_cfgx_cn30xx: cvmx_pip_prt_cfgx_cn30xx,
    pub cvmx_pip_prt_cfgx_cn38xx: cvmx_pip_prt_cfgx_cn38xx,
    pub cvmx_pip_prt_cfgx_cn50xx: cvmx_pip_prt_cfgx_cn50xx,
    pub cvmx_pip_prt_cfgx_cn52xx: cvmx_pip_prt_cfgx_cn52xx,
    pub cvmx_pip_prt_cfgx_cn58xx: cvmx_pip_prt_cfgx_cn58xx,
    pub cvmx_pip_prt_cfgx_cn68xx: cvmx_pip_prt_cfgx_cn68xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgbx_s {
    pub bpid: u64, // C bitfield width: 6
    pub reserved_6_15: u64, // C bitfield width: 10
    pub base: u64, // C bitfield width: 8
    pub reserved_24_31: u64, // C bitfield width: 8
    pub bsel_en: u64, // C bitfield width: 1
    pub bsel_num: u64, // C bitfield width: 2
    pub reserved_35_35: u64, // C bitfield width: 1
    pub alt_skp_en: u64, // C bitfield width: 1
    pub alt_skp_sel: u64, // C bitfield width: 2
    pub reserved_39_63: u64, // C bitfield width: 25
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgbx_cn61xx {
    pub reserved_0_31: u64, // C bitfield width: 32
    pub bsel_en: u64, // C bitfield width: 1
    pub bsel_num: u64, // C bitfield width: 2
    pub reserved_35_35: u64, // C bitfield width: 1
    pub alt_skp_en: u64, // C bitfield width: 1
    pub alt_skp_sel: u64, // C bitfield width: 2
    pub reserved_39_63: u64, // C bitfield width: 25
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgbx_cn66xx {
    pub reserved_0_35: u64, // C bitfield width: 36
    pub alt_skp_en: u64, // C bitfield width: 1
    pub alt_skp_sel: u64, // C bitfield width: 2
    pub reserved_39_63: u64, // C bitfield width: 25
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_cfgbx_cn68xxp1 {
    pub bpid: u64, // C bitfield width: 6
    pub reserved_6_15: u64, // C bitfield width: 10
    pub base: u64, // C bitfield width: 8
    pub reserved_24_63: u64, // C bitfield width: 40
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_prt_cfgbx {
    pub u64: u64,
    pub cvmx_pip_prt_cfgbx_s: cvmx_pip_prt_cfgbx_s,
    pub cvmx_pip_prt_cfgbx_cn61xx: cvmx_pip_prt_cfgbx_cn61xx,
    pub cvmx_pip_prt_cfgbx_cn66xx: cvmx_pip_prt_cfgbx_cn66xx,
    pub cvmx_pip_prt_cfgbx_cn68xxp1: cvmx_pip_prt_cfgbx_cn68xxp1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_tagx_s {
    pub grp: u64, // C bitfield width: 4
    pub non_tag_type: u64, // C bitfield width: 2
    pub ip4_tag_type: u64, // C bitfield width: 2
    pub ip6_tag_type: u64, // C bitfield width: 2
    pub tcp4_tag_type: u64, // C bitfield width: 2
    pub tcp6_tag_type: u64, // C bitfield width: 2
    pub ip4_src_flag: u64, // C bitfield width: 1
    pub ip6_src_flag: u64, // C bitfield width: 1
    pub ip4_dst_flag: u64, // C bitfield width: 1
    pub ip6_dst_flag: u64, // C bitfield width: 1
    pub ip4_pctl_flag: u64, // C bitfield width: 1
    pub ip6_nxth_flag: u64, // C bitfield width: 1
    pub ip4_sprt_flag: u64, // C bitfield width: 1
    pub ip6_sprt_flag: u64, // C bitfield width: 1
    pub ip4_dprt_flag: u64, // C bitfield width: 1
    pub ip6_dprt_flag: u64, // C bitfield width: 1
    pub inc_prt_flag: u64, // C bitfield width: 1
    pub inc_vlan: u64, // C bitfield width: 1
    pub inc_vs: u64, // C bitfield width: 2
    pub tag_mode: u64, // C bitfield width: 2
    pub grptag_mskip: u64, // C bitfield width: 1
    pub grptag: u64, // C bitfield width: 1
    pub grptagmask: u64, // C bitfield width: 4
    pub grptagbase: u64, // C bitfield width: 4
    pub grp_msb: u64, // C bitfield width: 2
    pub reserved_42_43: u64, // C bitfield width: 2
    pub grptagmask_msb: u64, // C bitfield width: 2
    pub reserved_46_47: u64, // C bitfield width: 2
    pub grptagbase_msb: u64, // C bitfield width: 2
    pub reserved_50_51: u64, // C bitfield width: 2
    pub inc_hwchk: u64, // C bitfield width: 1
    pub portadd_en: u64, // C bitfield width: 1
    pub reserved_54_63: u64, // C bitfield width: 10
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_tagx_cn30xx {
    pub grp: u64, // C bitfield width: 4
    pub non_tag_type: u64, // C bitfield width: 2
    pub ip4_tag_type: u64, // C bitfield width: 2
    pub ip6_tag_type: u64, // C bitfield width: 2
    pub tcp4_tag_type: u64, // C bitfield width: 2
    pub tcp6_tag_type: u64, // C bitfield width: 2
    pub ip4_src_flag: u64, // C bitfield width: 1
    pub ip6_src_flag: u64, // C bitfield width: 1
    pub ip4_dst_flag: u64, // C bitfield width: 1
    pub ip6_dst_flag: u64, // C bitfield width: 1
    pub ip4_pctl_flag: u64, // C bitfield width: 1
    pub ip6_nxth_flag: u64, // C bitfield width: 1
    pub ip4_sprt_flag: u64, // C bitfield width: 1
    pub ip6_sprt_flag: u64, // C bitfield width: 1
    pub ip4_dprt_flag: u64, // C bitfield width: 1
    pub ip6_dprt_flag: u64, // C bitfield width: 1
    pub inc_prt_flag: u64, // C bitfield width: 1
    pub inc_vlan: u64, // C bitfield width: 1
    pub inc_vs: u64, // C bitfield width: 2
    pub tag_mode: u64, // C bitfield width: 2
    pub reserved_30_30: u64, // C bitfield width: 1
    pub grptag: u64, // C bitfield width: 1
    pub grptagmask: u64, // C bitfield width: 4
    pub grptagbase: u64, // C bitfield width: 4
    pub reserved_40_63: u64, // C bitfield width: 24
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_prt_tagx_cn50xx {
    pub grp: u64, // C bitfield width: 4
    pub non_tag_type: u64, // C bitfield width: 2
    pub ip4_tag_type: u64, // C bitfield width: 2
    pub ip6_tag_type: u64, // C bitfield width: 2
    pub tcp4_tag_type: u64, // C bitfield width: 2
    pub tcp6_tag_type: u64, // C bitfield width: 2
    pub ip4_src_flag: u64, // C bitfield width: 1
    pub ip6_src_flag: u64, // C bitfield width: 1
    pub ip4_dst_flag: u64, // C bitfield width: 1
    pub ip6_dst_flag: u64, // C bitfield width: 1
    pub ip4_pctl_flag: u64, // C bitfield width: 1
    pub ip6_nxth_flag: u64, // C bitfield width: 1
    pub ip4_sprt_flag: u64, // C bitfield width: 1
    pub ip6_sprt_flag: u64, // C bitfield width: 1
    pub ip4_dprt_flag: u64, // C bitfield width: 1
    pub ip6_dprt_flag: u64, // C bitfield width: 1
    pub inc_prt_flag: u64, // C bitfield width: 1
    pub inc_vlan: u64, // C bitfield width: 1
    pub inc_vs: u64, // C bitfield width: 2
    pub tag_mode: u64, // C bitfield width: 2
    pub grptag_mskip: u64, // C bitfield width: 1
    pub grptag: u64, // C bitfield width: 1
    pub grptagmask: u64, // C bitfield width: 4
    pub grptagbase: u64, // C bitfield width: 4
    pub reserved_40_63: u64, // C bitfield width: 24
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_prt_tagx {
    pub u64: u64,
    pub cvmx_pip_prt_tagx_s: cvmx_pip_prt_tagx_s,
    pub cvmx_pip_prt_tagx_cn30xx: cvmx_pip_prt_tagx_cn30xx,
    pub cvmx_pip_prt_tagx_cn50xx: cvmx_pip_prt_tagx_cn50xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_qos_diffx_s {
    pub qos: u64, // C bitfield width: 3
    pub reserved_3_63: u64, // C bitfield width: 61
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_qos_diffx {
    pub u64: u64,
    pub cvmx_pip_qos_diffx_s: cvmx_pip_qos_diffx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_qos_vlanx_s {
    pub qos: u64, // C bitfield width: 3
    pub reserved_3_3: u64, // C bitfield width: 1
    pub qos1: u64, // C bitfield width: 3
    pub reserved_7_63: u64, // C bitfield width: 57
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_qos_vlanx_cn30xx {
    pub qos: u64, // C bitfield width: 3
    pub reserved_3_63: u64, // C bitfield width: 61
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_qos_vlanx {
    pub u64: u64,
    pub cvmx_pip_qos_vlanx_s: cvmx_pip_qos_vlanx_s,
    pub cvmx_pip_qos_vlanx_cn30xx: cvmx_pip_qos_vlanx_cn30xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_qos_watchx_s {
    pub match_value: u64, // C bitfield width: 16
    pub match_type: u64, // C bitfield width: 3
    pub reserved_19_19: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
    pub reserved_23_23: u64, // C bitfield width: 1
    pub grp: u64, // C bitfield width: 6
    pub reserved_30_31: u64, // C bitfield width: 2
    pub mask: u64, // C bitfield width: 16
    pub reserved_48_63: u64, // C bitfield width: 16
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_qos_watchx_cn30xx {
    pub match_value: u64, // C bitfield width: 16
    pub match_type: u64, // C bitfield width: 2
    pub reserved_18_19: u64, // C bitfield width: 2
    pub qos: u64, // C bitfield width: 3
    pub reserved_23_23: u64, // C bitfield width: 1
    pub grp: u64, // C bitfield width: 4
    pub reserved_28_31: u64, // C bitfield width: 4
    pub mask: u64, // C bitfield width: 16
    pub reserved_48_63: u64, // C bitfield width: 16
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_qos_watchx_cn50xx {
    pub match_value: u64, // C bitfield width: 16
    pub match_type: u64, // C bitfield width: 3
    pub reserved_19_19: u64, // C bitfield width: 1
    pub qos: u64, // C bitfield width: 3
    pub reserved_23_23: u64, // C bitfield width: 1
    pub grp: u64, // C bitfield width: 4
    pub reserved_28_31: u64, // C bitfield width: 4
    pub mask: u64, // C bitfield width: 16
    pub reserved_48_63: u64, // C bitfield width: 16
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_qos_watchx {
    pub u64: u64,
    pub cvmx_pip_qos_watchx_s: cvmx_pip_qos_watchx_s,
    pub cvmx_pip_qos_watchx_cn30xx: cvmx_pip_qos_watchx_cn30xx,
    pub cvmx_pip_qos_watchx_cn50xx: cvmx_pip_qos_watchx_cn50xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_raw_word_s {
    pub word: u64, // C bitfield width: 56
    pub reserved_56_63: u64, // C bitfield width: 8
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_raw_word {
    pub u64: u64,
    pub cvmx_pip_raw_word_s: cvmx_pip_raw_word_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_sft_rst_s {
    pub rst: u64, // C bitfield width: 1
    pub reserved_1_63: u64, // C bitfield width: 63
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_sft_rst {
    pub u64: u64,
    pub cvmx_pip_sft_rst_s: cvmx_pip_sft_rst_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat0_x_s {
    pub drp_octs: u64, // C bitfield width: 32
    pub drp_pkts: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat0_x {
    pub u64: u64,
    pub cvmx_pip_stat0_x_s: cvmx_pip_stat0_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat0_prtx_s {
    pub drp_octs: u64, // C bitfield width: 32
    pub drp_pkts: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat0_prtx {
    pub u64: u64,
    pub cvmx_pip_stat0_prtx_s: cvmx_pip_stat0_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat10_x_s {
    pub mcast: u64, // C bitfield width: 32
    pub bcast: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat10_x {
    pub u64: u64,
    pub cvmx_pip_stat10_x_s: cvmx_pip_stat10_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat10_prtx_s {
    pub mcast: u64, // C bitfield width: 32
    pub bcast: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat10_prtx {
    pub u64: u64,
    pub cvmx_pip_stat10_prtx_s: cvmx_pip_stat10_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat11_x_s {
    pub mcast: u64, // C bitfield width: 32
    pub bcast: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat11_x {
    pub u64: u64,
    pub cvmx_pip_stat11_x_s: cvmx_pip_stat11_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat11_prtx_s {
    pub mcast: u64, // C bitfield width: 32
    pub bcast: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat11_prtx {
    pub u64: u64,
    pub cvmx_pip_stat11_prtx_s: cvmx_pip_stat11_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat1_x_s {
    pub octs: u64, // C bitfield width: 48
    pub reserved_48_63: u64, // C bitfield width: 16
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat1_x {
    pub u64: u64,
    pub cvmx_pip_stat1_x_s: cvmx_pip_stat1_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat1_prtx_s {
    pub octs: u64, // C bitfield width: 48
    pub reserved_48_63: u64, // C bitfield width: 16
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat1_prtx {
    pub u64: u64,
    pub cvmx_pip_stat1_prtx_s: cvmx_pip_stat1_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat2_x_s {
    pub raw: u64, // C bitfield width: 32
    pub pkts: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat2_x {
    pub u64: u64,
    pub cvmx_pip_stat2_x_s: cvmx_pip_stat2_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat2_prtx_s {
    pub raw: u64, // C bitfield width: 32
    pub pkts: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat2_prtx {
    pub u64: u64,
    pub cvmx_pip_stat2_prtx_s: cvmx_pip_stat2_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat3_x_s {
    pub mcst: u64, // C bitfield width: 32
    pub bcst: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat3_x {
    pub u64: u64,
    pub cvmx_pip_stat3_x_s: cvmx_pip_stat3_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat3_prtx_s {
    pub mcst: u64, // C bitfield width: 32
    pub bcst: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat3_prtx {
    pub u64: u64,
    pub cvmx_pip_stat3_prtx_s: cvmx_pip_stat3_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat4_x_s {
    pub h64: u64, // C bitfield width: 32
    pub h65to127: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat4_x {
    pub u64: u64,
    pub cvmx_pip_stat4_x_s: cvmx_pip_stat4_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat4_prtx_s {
    pub h64: u64, // C bitfield width: 32
    pub h65to127: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat4_prtx {
    pub u64: u64,
    pub cvmx_pip_stat4_prtx_s: cvmx_pip_stat4_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat5_x_s {
    pub h128to255: u64, // C bitfield width: 32
    pub h256to511: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat5_x {
    pub u64: u64,
    pub cvmx_pip_stat5_x_s: cvmx_pip_stat5_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat5_prtx_s {
    pub h128to255: u64, // C bitfield width: 32
    pub h256to511: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat5_prtx {
    pub u64: u64,
    pub cvmx_pip_stat5_prtx_s: cvmx_pip_stat5_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat6_x_s {
    pub h512to1023: u64, // C bitfield width: 32
    pub h1024to1518: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat6_x {
    pub u64: u64,
    pub cvmx_pip_stat6_x_s: cvmx_pip_stat6_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat6_prtx_s {
    pub h512to1023: u64, // C bitfield width: 32
    pub h1024to1518: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat6_prtx {
    pub u64: u64,
    pub cvmx_pip_stat6_prtx_s: cvmx_pip_stat6_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat7_x_s {
    pub h1519: u64, // C bitfield width: 32
    pub fcs: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat7_x {
    pub u64: u64,
    pub cvmx_pip_stat7_x_s: cvmx_pip_stat7_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat7_prtx_s {
    pub h1519: u64, // C bitfield width: 32
    pub fcs: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat7_prtx {
    pub u64: u64,
    pub cvmx_pip_stat7_prtx_s: cvmx_pip_stat7_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat8_x_s {
    pub undersz: u64, // C bitfield width: 32
    pub frag: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat8_x {
    pub u64: u64,
    pub cvmx_pip_stat8_x_s: cvmx_pip_stat8_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat8_prtx_s {
    pub undersz: u64, // C bitfield width: 32
    pub frag: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat8_prtx {
    pub u64: u64,
    pub cvmx_pip_stat8_prtx_s: cvmx_pip_stat8_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat9_x_s {
    pub oversz: u64, // C bitfield width: 32
    pub jabber: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat9_x {
    pub u64: u64,
    pub cvmx_pip_stat9_x_s: cvmx_pip_stat9_x_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat9_prtx_s {
    pub oversz: u64, // C bitfield width: 32
    pub jabber: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat9_prtx {
    pub u64: u64,
    pub cvmx_pip_stat9_prtx_s: cvmx_pip_stat9_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat_ctl_s {
    pub rdclr: u64, // C bitfield width: 1
    pub reserved_1_7: u64, // C bitfield width: 7
    pub mode: u64, // C bitfield width: 1
    pub reserved_9_63: u64, // C bitfield width: 55
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat_ctl_cn30xx {
    pub rdclr: u64, // C bitfield width: 1
    pub reserved_1_63: u64, // C bitfield width: 63
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat_ctl {
    pub u64: u64,
    pub cvmx_pip_stat_ctl_s: cvmx_pip_stat_ctl_s,
    pub cvmx_pip_stat_ctl_cn30xx: cvmx_pip_stat_ctl_cn30xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat_inb_errsx_s {
    pub errs: u64, // C bitfield width: 16
    pub reserved_16_63: u64, // C bitfield width: 48
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat_inb_errsx {
    pub u64: u64,
    pub cvmx_pip_stat_inb_errsx_s: cvmx_pip_stat_inb_errsx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat_inb_errs_pkndx_s {
    pub errs: u64, // C bitfield width: 16
    pub reserved_16_63: u64, // C bitfield width: 48
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat_inb_errs_pkndx {
    pub u64: u64,
    pub cvmx_pip_stat_inb_errs_pkndx_s: cvmx_pip_stat_inb_errs_pkndx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat_inb_octsx_s {
    pub octs: u64, // C bitfield width: 48
    pub reserved_48_63: u64, // C bitfield width: 16
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat_inb_octsx {
    pub u64: u64,
    pub cvmx_pip_stat_inb_octsx_s: cvmx_pip_stat_inb_octsx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat_inb_octs_pkndx_s {
    pub octs: u64, // C bitfield width: 48
    pub reserved_48_63: u64, // C bitfield width: 16
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat_inb_octs_pkndx {
    pub u64: u64,
    pub cvmx_pip_stat_inb_octs_pkndx_s: cvmx_pip_stat_inb_octs_pkndx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat_inb_pktsx_s {
    pub pkts: u64, // C bitfield width: 32
    pub reserved_32_63: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat_inb_pktsx {
    pub u64: u64,
    pub cvmx_pip_stat_inb_pktsx_s: cvmx_pip_stat_inb_pktsx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_stat_inb_pkts_pkndx_s {
    pub pkts: u64, // C bitfield width: 32
    pub reserved_32_63: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_stat_inb_pkts_pkndx {
    pub u64: u64,
    pub cvmx_pip_stat_inb_pkts_pkndx_s: cvmx_pip_stat_inb_pkts_pkndx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_sub_pkind_fcsx_s {
    pub port_bit: u64, // C bitfield width: 64
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_sub_pkind_fcsx {
    pub u64: u64,
    pub cvmx_pip_sub_pkind_fcsx_s: cvmx_pip_sub_pkind_fcsx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_tag_incx_s {
    pub en: u64, // C bitfield width: 8
    pub reserved_8_63: u64, // C bitfield width: 56
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_tag_incx {
    pub u64: u64,
    pub cvmx_pip_tag_incx_s: cvmx_pip_tag_incx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_tag_mask_s {
    pub mask: u64, // C bitfield width: 16
    pub reserved_16_63: u64, // C bitfield width: 48
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_tag_mask {
    pub u64: u64,
    pub cvmx_pip_tag_mask_s: cvmx_pip_tag_mask_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_tag_secret_s {
    pub src: u64, // C bitfield width: 16
    pub dst: u64, // C bitfield width: 16
    pub reserved_32_63: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_tag_secret {
    pub u64: u64,
    pub cvmx_pip_tag_secret_s: cvmx_pip_tag_secret_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_todo_entry_s {
    pub entry: u64, // C bitfield width: 62
    pub reserved_62_62: u64, // C bitfield width: 1
    pub val: u64, // C bitfield width: 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_todo_entry {
    pub u64: u64,
    pub cvmx_pip_todo_entry_s: cvmx_pip_todo_entry_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_vlan_etypesx_s {
    pub type0: u64, // C bitfield width: 16
    pub type1: u64, // C bitfield width: 16
    pub type2: u64, // C bitfield width: 16
    pub type3: u64, // C bitfield width: 16
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_vlan_etypesx {
    pub u64: u64,
    pub cvmx_pip_vlan_etypesx_s: cvmx_pip_vlan_etypesx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat0_prtx_s {
    pub drp_octs: u64, // C bitfield width: 32
    pub drp_pkts: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat0_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat0_prtx_s: cvmx_pip_xstat0_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat10_prtx_s {
    pub mcast: u64, // C bitfield width: 32
    pub bcast: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat10_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat10_prtx_s: cvmx_pip_xstat10_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat11_prtx_s {
    pub mcast: u64, // C bitfield width: 32
    pub bcast: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat11_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat11_prtx_s: cvmx_pip_xstat11_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat1_prtx_s {
    pub octs: u64, // C bitfield width: 48
    pub reserved_48_63: u64, // C bitfield width: 16
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat1_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat1_prtx_s: cvmx_pip_xstat1_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat2_prtx_s {
    pub raw: u64, // C bitfield width: 32
    pub pkts: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat2_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat2_prtx_s: cvmx_pip_xstat2_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat3_prtx_s {
    pub mcst: u64, // C bitfield width: 32
    pub bcst: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat3_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat3_prtx_s: cvmx_pip_xstat3_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat4_prtx_s {
    pub h64: u64, // C bitfield width: 32
    pub h65to127: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat4_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat4_prtx_s: cvmx_pip_xstat4_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat5_prtx_s {
    pub h128to255: u64, // C bitfield width: 32
    pub h256to511: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat5_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat5_prtx_s: cvmx_pip_xstat5_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat6_prtx_s {
    pub h512to1023: u64, // C bitfield width: 32
    pub h1024to1518: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat6_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat6_prtx_s: cvmx_pip_xstat6_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat7_prtx_s {
    pub h1519: u64, // C bitfield width: 32
    pub fcs: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat7_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat7_prtx_s: cvmx_pip_xstat7_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat8_prtx_s {
    pub undersz: u64, // C bitfield width: 32
    pub frag: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat8_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat8_prtx_s: cvmx_pip_xstat8_prtx_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_xstat9_prtx_s {
    pub oversz: u64, // C bitfield width: 32
    pub jabber: u64, // C bitfield width: 32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pip_xstat9_prtx {
    pub u64: u64,
    pub cvmx_pip_xstat9_prtx_s: cvmx_pip_xstat9_prtx_s,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
