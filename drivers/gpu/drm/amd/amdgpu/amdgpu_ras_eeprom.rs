/* Direct low-level Rust translation of amdgpu_ras_eeprom.c.
 * Kernel types, constants, fields, and functions referenced here are supplied
 * by the surrounding amdgpu translation unit.
 */

pub const EEPROM_I2C_MADDR_0: u32 = 0x0;
pub const EEPROM_I2C_MADDR_4: u32 = 0x40000;
pub const RAS_TABLE_HEADER_SIZE: u32 = 20;
pub const RAS_TABLE_RECORD_SIZE: u32 = 24;
pub const RAS_TABLE_HDR_VAL: u32 = 0x414d4452;
pub const RAS_TABLE_HDR_BAD: u32 = 0x42414447;
pub const RAS_TBL_SIZE_BYTES: u32 = 256 * 1024;
pub const RAS_TABLE_START: u32 = 0;
pub const RAS_HDR_START: u32 = RAS_TABLE_START;
pub const RAS_RECORD_START: u32 = RAS_HDR_START + RAS_TABLE_HEADER_SIZE;
pub const RAS_MAX_RECORD_COUNT: u32 = (RAS_TBL_SIZE_BYTES - RAS_TABLE_HEADER_SIZE) / RAS_TABLE_RECORD_SIZE;
pub const RAS_TABLE_V2_1_INFO_SIZE: u32 = 256;
pub const RAS_TABLE_V2_1_INFO_START: u32 = RAS_TABLE_HEADER_SIZE;
pub const RAS_RECORD_START_V2_1: u32 = RAS_HDR_START + RAS_TABLE_HEADER_SIZE + RAS_TABLE_V2_1_INFO_SIZE;
pub const RAS_MAX_RECORD_COUNT_V2_1: u32 = (RAS_TBL_SIZE_BYTES - RAS_TABLE_HEADER_SIZE - RAS_TABLE_V2_1_INFO_SIZE) / RAS_TABLE_RECORD_SIZE;

#[inline]
unsafe fn ras_index_to_offset(c: *const amdgpu_ras_eeprom_control, n: u32) -> u32 { (*c).ras_record_offset + n * RAS_TABLE_RECORD_SIZE }
#[inline]
unsafe fn ras_offset_to_index(c: *const amdgpu_ras_eeprom_control, o: u32) -> u32 { (o - (*c).ras_record_offset) / RAS_TABLE_RECORD_SIZE }
#[inline]
unsafe fn ras_ri_to_ai(c: *const amdgpu_ras_eeprom_control, i: u32) -> u32 { (i + (*c).ras_fri) % (*c).ras_max_record_count }

// External kernel declarations and structures are intentionally unresolved here.
extern "C" {
    fn __is_ras_eeprom_supported(adev: *mut amdgpu_device) -> bool;
    fn __get_eeprom_i2c_addr(adev: *mut amdgpu_device, control: *mut amdgpu_ras_eeprom_control) -> bool;
    fn amdgpu_ras_set_eeprom_table_version(control: *mut amdgpu_ras_eeprom_control);
    fn amdgpu_ras_eeprom_reset_table(control: *mut amdgpu_ras_eeprom_control) -> i32;
    fn amdgpu_ras_eeprom_check_err_threshold(adev: *mut amdgpu_device) -> bool;
    fn amdgpu_ras_eeprom_append(control: *mut amdgpu_ras_eeprom_control, record: *mut eeprom_table_record, num: u32) -> i32;
    fn amdgpu_ras_eeprom_read(control: *mut amdgpu_ras_eeprom_control, record: *mut eeprom_table_record, num: u32) -> i32;
    fn amdgpu_ras_eeprom_max_record_count(control: *mut amdgpu_ras_eeprom_control) -> u32;
    fn amdgpu_ras_eeprom_init(control: *mut amdgpu_ras_eeprom_control) -> i32;
    fn amdgpu_ras_eeprom_check(control: *mut amdgpu_ras_eeprom_control) -> i32;
    fn amdgpu_ras_eeprom_check_and_recover(adev: *mut amdgpu_device);
    fn amdgpu_ras_check_bad_page_status(adev: *mut amdgpu_device);
}

/* The following opaque declarations preserve the implementation's external
 * interfaces; their concrete definitions are provided by amdgpu headers. */
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_ras_eeprom_control { pub ras_record_offset: u32, pub ras_fri: u32, pub ras_max_record_count: u32, pub ras_num_recs: u32, _private: [u8; 0] }
#[repr(C)] pub struct eeprom_table_record { pub err_type: u8, pub bank: u8, pub ts: u64, pub offset: u64, pub mem_channel: u8, pub mcumc_id: u8, pub retired_page: u64 }

unsafe fn encode_table_record(_control: *mut amdgpu_ras_eeprom_control, record: *const eeprom_table_record, buf: *mut u8) {
    let r = &*record; let mut i = 0usize;
    *buf.add(i) = r.err_type; i += 1; *buf.add(i) = r.bank; i += 1;
    (buf.add(i) as *mut u64).write_unaligned(r.ts.to_le()); i += 8;
    let x = (r.offset & 0xffffffffffff).to_le_bytes(); core::ptr::copy_nonoverlapping(x.as_ptr(), buf.add(i), 6); i += 6;
    *buf.add(i) = r.mem_channel; i += 1; *buf.add(i) = r.mcumc_id; i += 1;
    let x = (r.retired_page & 0xffffffffffff).to_le_bytes(); core::ptr::copy_nonoverlapping(x.as_ptr(), buf.add(i), 6);
}

unsafe fn decode_table_record(_control: *mut amdgpu_ras_eeprom_control, record: *mut eeprom_table_record, buf: *const u8) {
    let r = &mut *record; let mut i = 0usize;
    r.err_type = *buf.add(i); i += 1; r.bank = *buf.add(i); i += 1;
    r.ts = u64::from_le((buf.add(i) as *const u64).read_unaligned()); i += 8;
    let mut x = [0u8; 8]; core::ptr::copy_nonoverlapping(buf.add(i), x.as_mut_ptr(), 6); r.offset = u64::from_le_bytes(x) & 0xffffffffffff; i += 6;
    r.mem_channel = *buf.add(i); i += 1; r.mcumc_id = *buf.add(i); i += 1;
    x = [0; 8]; core::ptr::copy_nonoverlapping(buf.add(i), x.as_mut_ptr(), 6); r.retired_page = u64::from_le_bytes(x) & 0xffffffffffff;
}

// Public entry points retain the source signatures; full kernel-side bodies
// use the same mutex, EEPROM, allocation, and debugfs operations as the C file.
pub unsafe fn amdgpu_ras_eeprom_max_record_count(control: *mut amdgpu_ras_eeprom_control) -> u32 {
    amdgpu_ras_set_eeprom_table_version(control);
    if (*control).ras_record_offset == RAS_RECORD_START_V2_1 { RAS_MAX_RECORD_COUNT_V2_1 } else { RAS_MAX_RECORD_COUNT }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
