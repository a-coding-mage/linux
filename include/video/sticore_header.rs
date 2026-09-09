/* SPDX-License-Identifier: GPL-2.0 */

// Translated from sticore.h. Kernel-provided types and functions are external
// dependencies supplied by the surrounding translation unit.

use core::ffi::c_void;

pub enum device {}
pub enum pci_dev {}
pub enum spinlock_t {}

pub const MAX_STI_ROMS: usize = 4;
pub const STI_REGION_MAX: usize = 8;
pub const STI_DEV_NAME_LENGTH: usize = 32;
pub const STI_MONITOR_MAX: usize = 256;
pub const STI_FONT_HPROMAN8: u32 = 1;
pub const STI_FONT_KANA8: u32 = 2;
pub const ALT_CODE_TYPE_UNKNOWN: u8 = 0x00;
pub const ALT_CODE_TYPE_PA_RISC_64: u8 = 0x01;
pub const STI_WAIT: u32 = 1;

#[repr(C)]
pub union region_t {
    pub region_desc: region_desc,
    pub region: u32,
}
#[repr(C)]
pub struct region_desc {
    pub offset: u32,
    pub sys_only: u32,
    pub cache: u32,
    pub btlb: u32,
    pub last: u32,
    pub length: u32,
}

pub unsafe fn region_offset_to_phys(rt: &region_t, hpa: u32) -> u32 {
    ((*rt).region_desc.offset << 12).wrapping_add(hpa)
}

#[repr(C)]
pub struct sti_glob_cfg_ext { pub curr_mon: u8, pub friendly_boot: u8, pub power: i16, pub freq_ref: i32, pub sti_mem_addr: *mut u32, pub future_ptr: *mut u32 }
#[repr(C)]
pub struct sti_glob_cfg { pub text_planes: i32, pub onscreen_x: i16, pub onscreen_y: i16, pub offscreen_x: i16, pub offscreen_y: i16, pub total_x: i16, pub total_y: i16, pub region_ptrs: [*mut u32; STI_REGION_MAX], pub reent_lvl: i32, pub save_addr: *mut u32, pub ext_ptr: *mut u32 }

#[repr(C)]
pub struct sti_init_flags { pub bits: u32, pub future_ptr: *mut u32 }
#[repr(C)]
pub struct sti_init_inptr_ext { pub config_mon_type: u8, pub pad: [u8; 1], pub inflight_data: u16, pub future_ptr: *mut u32 }
#[repr(C)] pub struct sti_init_inptr { pub text_planes: i32, pub ext_ptr: *mut u32 }
#[repr(C)] pub struct sti_init_outptr { pub errno: i32, pub text_planes: i32, pub future_ptr: *mut u32 }
#[repr(C)] pub struct sti_conf_flags { pub bits: u32, pub future_ptr: *mut u32 }
#[repr(C)] pub struct sti_conf_inptr { pub future_ptr: *mut u32 }
#[repr(C)] pub struct sti_conf_outptr_ext { pub crt_config: [u32; 3], pub crt_hdw: [u32; 3], pub future_ptr: *mut u32 }
#[repr(C)] pub struct sti_conf_outptr { pub errno: i32, pub onscreen_x: i16, pub onscreen_y: i16, pub offscreen_x: i16, pub offscreen_y: i16, pub total_x: i16, pub total_y: i16, pub bits_per_pixel: i32, pub bits_used: i32, pub planes: i32, pub dev_name: [u8; STI_DEV_NAME_LENGTH], pub attributes: u32, pub ext_ptr: *mut u32 }

#[repr(C)]
pub struct sti_rom {
    pub type_: [u8; 4], pub res004: u8, pub num_mons: u8, pub revno: [u8; 2], pub graphics_id: [u32; 2],
    pub font_start: u32, pub statesize: u32, pub last_addr: u32, pub region_list: u32, pub reentsize: u16, pub maxtime: u16,
    pub mon_tbl_addr: u32, pub user_data_addr: u32, pub sti_mem_req: u32, pub user_data_size: u32, pub power: u16,
    pub bus_support: u8, pub ext_bus_support: u8, pub alt_code_type: u8, pub ext_dd_struct: [u8; 3], pub cfb_addr: u32,
    pub init_graph: u32, pub state_mgmt: u32, pub font_unpmv: u32, pub block_move: u32, pub self_test: u32, pub excep_hdlr: u32,
    pub inq_conf: u32, pub set_cm_entry: u32, pub dma_ctrl: u32, pub res040: [u8; 28], pub init_graph_addr: u32,
    pub state_mgmt_addr: u32, pub font_unp_addr: u32, pub block_move_addr: u32, pub self_test_addr: u32, pub excep_hdlr_addr: u32,
    pub inq_conf_addr: u32, pub set_cm_entry_addr: u32, pub image_unpack_addr: u32, pub pa_risx_addrs: [u32; 7],
}
#[repr(C)] pub struct sti_rom_font { pub first_char: u16, pub last_char: u16, pub width: u8, pub height: u8, pub font_type: u8, pub bytes_per_char: u8, pub next_font: i32, pub underline_height: u8, pub underline_pos: u8, pub res008: [u8; 2] }
#[repr(C)] pub struct sti_cooked_font { pub raw: *mut sti_rom_font, pub raw_ptr: *mut c_void, pub next_font: *mut sti_cooked_font, pub height: i32, pub width: i32, pub refcount: i32, pub crc: u32 }
#[repr(C)] pub struct sti_cooked_rom { pub raw: *mut sti_rom, pub font_start: *mut sti_cooked_font }

#[repr(C)] pub struct sti_font_inptr { pub font_start_addr: *mut u32, pub index: i16, pub fg_color: u8, pub bg_color: u8, pub dest_x: i16, pub dest_y: i16, pub future_ptr: *mut u32 }
#[repr(C)] pub struct sti_font_flags { pub bits: u32, pub future_ptr: *mut u32 }
#[repr(C)] pub struct sti_font_outptr { pub errno: i32, pub future_ptr: *mut u32 }
#[repr(C)] pub struct sti_blkmv_flags { pub bits: u32, pub future_ptr: *mut u32 }
#[repr(C)] pub struct sti_blkmv_inptr { pub fg_color: u8, pub bg_color: u8, pub src_x: i16, pub src_y: i16, pub dest_x: i16, pub dest_y: i16, pub width: i16, pub height: i16, pub future_ptr: *mut u32 }
#[repr(C)] pub struct sti_blkmv_outptr { pub errno: i32, pub future_ptr: *mut u32 }

#[repr(C)] pub struct sti_all_data {
    pub glob_cfg: sti_glob_cfg, pub glob_cfg_ext: sti_glob_cfg_ext, pub inq_inptr: sti_conf_inptr, pub inq_outptr: sti_conf_outptr, pub inq_outptr_ext: sti_conf_outptr_ext,
    pub init_inptr_ext: sti_init_inptr_ext, pub init_inptr: sti_init_inptr, pub init_outptr: sti_init_outptr, pub blkmv_inptr: sti_blkmv_inptr, pub blkmv_outptr: sti_blkmv_outptr,
    pub font_inptr: sti_font_inptr, pub font_outptr: sti_font_outptr, pub save_addr: [usize; 1024 / core::mem::size_of::<usize>()], pub sti_mem_addr: [usize; 256 / core::mem::size_of::<usize>()],
}
#[repr(C)] pub struct sti_struct {
    pub lock: *mut spinlock_t, pub sti_mem_request: i32, pub graphics_id: [u32; 2], pub rom: *mut sti_cooked_rom, pub font_unpmv: usize, pub block_move: usize, pub init_graph: usize, pub inq_conf: usize, pub do_call64: i32,
    pub regions: [region_t; STI_REGION_MAX], pub regions_phys: [usize; STI_REGION_MAX], pub glob_cfg: *mut sti_glob_cfg, pub wordmode: i32, pub font: *mut sti_cooked_font, pub pd: *mut pci_dev, pub rm_entry: [u8; 16], pub dev: *mut device, pub sti_data: *mut sti_all_data, pub pa_path: [u8; 24],
}

pub unsafe fn sti_onscreen_x(sti: *const sti_struct) -> i16 { (*(*sti).glob_cfg).onscreen_x }
pub unsafe fn sti_onscreen_y(sti: *const sti_struct) -> i16 { (*(*sti).glob_cfg).onscreen_y }

extern "C" {
    pub fn sti_get_rom(index: u32) -> *mut sti_struct;
    pub fn sti_font_convert_bytemode(sti: *mut sti_struct, f: *mut sti_cooked_font);
    pub fn sti_call(sti: *const sti_struct, func: usize, flags: *const c_void, inptr: *mut c_void, outptr: *mut c_void, glob_cfg: *mut sti_glob_cfg) -> i32;
    pub fn sti_putc(sti: *mut sti_struct, c: i32, y: i32, x: i32, font: *mut sti_cooked_font);
    pub fn sti_set(sti: *mut sti_struct, src_y: i32, src_x: i32, height: i32, width: i32, color: u8);
    pub fn sti_clear(sti: *mut sti_struct, src_y: i32, src_x: i32, height: i32, width: i32, c: i32, font: *mut sti_cooked_font);
    pub fn sti_bmove(sti: *mut sti_struct, src_y: i32, src_x: i32, dst_y: i32, dst_x: i32, height: i32, width: i32, font: *mut sti_cooked_font);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
