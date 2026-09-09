#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

// Translation of cxl/core/edac.c. External Linux/CXL declarations are supplied
// by the surrounding translation unit.

pub const CXL_NR_EDAC_DEV_FEATURES: usize = 7;
pub const CXL_SCRUB_NO_REGION: i32 = -1;

#[repr(C)]
pub struct cxl_patrol_scrub_context {
    pub instance: u8,
    pub get_feat_size: u16,
    pub set_feat_size: u16,
    pub get_version: u8,
    pub set_version: u8,
    pub effects: u16,
    pub cxlmd: *mut cxl_memdev,
    pub cxlr: *mut cxl_region,
}

#[repr(C, packed)]
pub struct cxl_scrub_rd_attrbs {
    pub scrub_cycle_cap: u8,
    pub scrub_cycle_hours: u16,
    pub scrub_flags: u8,
}

#[repr(C, packed)]
pub struct cxl_scrub_wr_attrbs {
    pub scrub_cycle_hours: u8,
    pub scrub_flags: u8,
}

pub const CXL_SCRUB_CONTROL_CHANGEABLE: u8 = 1 << 0;
pub const CXL_SCRUB_CONTROL_REALTIME: u8 = 1 << 1;
pub const CXL_SCRUB_CONTROL_CYCLE_MASK: u8 = 0xff;
pub const CXL_SCRUB_CONTROL_MIN_CYCLE_MASK: u16 = 0xff00;
pub const CXL_SCRUB_CONTROL_ENABLE: u8 = 1;

#[inline]
pub const fn cxl_get_scrub_cycle_changeable(cap: u8) -> u8 { cap & 1 }
#[inline]
pub const fn cxl_get_scrub_cycle(cycle: u16) -> u8 { cycle as u8 }
#[inline]
pub const fn cxl_get_scrub_min_cycle(cycle: u16) -> u8 { (cycle >> 8) as u8 }
#[inline]
pub const fn cxl_get_scrub_en_sts(flags: u8) -> u8 { flags & 1 }
#[inline]
pub const fn cxl_set_scrub_cycle(cycle: u8) -> u8 { cycle }
#[inline]
pub const fn cxl_set_scrub_en(en: bool) -> u8 { en as u8 }

#[repr(C)]
pub struct cxl_memdev { _private: [u8; 0] }
#[repr(C)]
pub struct cxl_region { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
