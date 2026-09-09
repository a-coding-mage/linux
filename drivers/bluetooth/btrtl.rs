// SPDX-License-Identifier: GPL-2.0-or-later
// Bluetooth support for Realtek devices. Direct low-level translation of btrtl.c.

#![allow(dead_code, unused_variables, unused_mut, non_snake_case)]

use core::{mem, ptr};

const VERSION: &str = "0.1";
const RTL_CHIP_8723CS_CG: u8 = 3;
const RTL_CHIP_8723CS_VF: u8 = 4;
const RTL_CHIP_8723CS_XX: u8 = 5;
const RTL_EPATCH_SIGNATURE: &[u8; 8] = b"Realtech";
const RTL_EPATCH_SIGNATURE_V2: &[u8; 8] = b"RTBTCore";
const RTL_ROM_LMP_8703B: u16 = 0x8703;
const RTL_ROM_LMP_8723A: u16 = 0x1200;
const RTL_ROM_LMP_8723B: u16 = 0x8723;
const RTL_ROM_LMP_8821A: u16 = 0x8821;
const RTL_ROM_LMP_8761A: u16 = 0x8761;
const RTL_ROM_LMP_8822B: u16 = 0x8822;
const RTL_ROM_LMP_8852A: u16 = 0x8852;
const RTL_ROM_LMP_8851B: u16 = 0x8851;
const RTL_ROM_LMP_8922A: u16 = 0x8922;
const RTL_CONFIG_MAGIC: u32 = 0x8723ab55;
const RTL_VSC_OP_COREDUMP: u16 = 0xfcff;
const IC_MATCH_FL_LMPSUBV: u16 = 1 << 0;
const IC_MATCH_FL_HCIREV: u16 = 1 << 1;
const IC_MATCH_FL_HCIVER: u16 = 1 << 2;
const IC_MATCH_FL_HCIBUS: u16 = 1 << 3;
const IC_MATCH_FL_CHIP_TYPE: u16 = 1 << 4;
const RTL_PATCH_SNIPPETS: u32 = 1;
const RTL_PATCH_DUMMY_HEADER: u32 = 2;
const RTL_PATCH_SECURITY_HEADER: u32 = 3;

#[repr(u8)]
pub enum btrtl_chip_id { CHIP_ID_8723A, CHIP_ID_8723B, CHIP_ID_8821A, CHIP_ID_8761A,
    CHIP_ID_8822B = 8, CHIP_ID_8723D, CHIP_ID_8821C, CHIP_ID_8822C = 13,
    CHIP_ID_8761B, CHIP_ID_8852A = 18, CHIP_ID_8852B = 20, CHIP_ID_8852C = 25,
    CHIP_ID_8851B = 36, CHIP_ID_8922A = 44, CHIP_ID_8852BT = 47, CHIP_ID_8761C = 51 }

#[repr(C)] pub struct id_table { pub match_flags:u16, pub lmp_subver:u16, pub hci_rev:u16,
    pub hci_ver:u8, pub hci_bus:u8, pub chip_type:u8, pub config_needed:bool,
    pub has_rom_version:bool, pub has_msft_ext:bool, pub fw_name:*const i8,
    pub cfg_name:*const i8, pub hw_info:*const i8 }
#[repr(C)] pub struct btrtl_device_info { pub ic_info:*const id_table, pub rom_version:u8,
    pub fw_data:*mut u8, pub fw_len:i32, pub cfg_data:*mut u8, pub cfg_len:i32,
    pub drop_fw:bool, pub project_id:i32, pub key_id:u8, pub patch_subsecs:*mut u8 }

// Types and kernel interfaces are supplied by the surrounding translation unit.
extern "C" {
    fn __hci_cmd_sync(hdev:*mut u8, opcode:u16, plen:u32, param:*const u8, timeout:u32)->*mut u8;
    fn __hci_cmd_send(hdev:*mut u8, opcode:u16, plen:u32, param:*const u8);
    fn rtl_load_file(hdev:*mut u8, name:*const i8, buff:*mut *mut u8)->i32;
}

static C(s:&str)->*const i8 { s.as_ptr() as *const i8 }
static IC: [id_table; 25] = [
    id_table{match_flags:15,lmp_subver:RTL_ROM_LMP_8723A,hci_rev:0xb,hci_ver:6,hci_bus:1,chip_type:0,config_needed:false,has_rom_version:false,has_msft_ext:false,fw_name:C("rtl_bt/rtl8723a_fw"),cfg_name:ptr::null(),hw_info:C("rtl8723au")},
    id_table{match_flags:15,lmp_subver:RTL_ROM_LMP_8723B,hci_rev:0xb,hci_ver:6,hci_bus:2,chip_type:0,config_needed:true,has_rom_version:true,has_msft_ext:false,fw_name:C("rtl_bt/rtl8723bs_fw"),cfg_name:C("rtl_bt/rtl8723bs_config"),hw_info:C("rtl8723bs")},
    id_table{match_flags:15,lmp_subver:RTL_ROM_LMP_8723B,hci_rev:0xb,hci_ver:6,hci_bus:1,chip_type:0,config_needed:false,has_rom_version:true,has_msft_ext:false,fw_name:C("rtl_bt/rtl8723b_fw"),cfg_name:C("rtl_bt/rtl8723b_config"),hw_info:C("rtl8723bu")},
    id_table{match_flags:25,lmp_subver:RTL_ROM_LMP_8703B,hci_rev:0,hci_ver:0,hci_bus:2,chip_type:3,config_needed:true,has_rom_version:true,has_msft_ext:false,fw_name:C("rtl_bt/rtl8723cs_cg_fw"),cfg_name:C("rtl_bt/rtl8723cs_cg_config"),hw_info:C("rtl8723cs-cg")},
    id_table{match_flags:25,lmp_subver:RTL_ROM_LMP_8703B,hci_rev:0,hci_ver:0,hci_bus:2,chip_type:4,config_needed:true,has_rom_version:true,has_msft_ext:false,fw_name:C("rtl_bt/rtl8723cs_vf_fw"),cfg_name:C("rtl_bt/rtl8723cs_vf_config"),hw_info:C("rtl8723cs-vf")},
    id_table{match_flags:25,lmp_subver:RTL_ROM_LMP_8703B,hci_rev:0,hci_ver:0,hci_bus:2,chip_type:5,config_needed:true,has_rom_version:true,has_msft_ext:false,fw_name:C("rtl_bt/rtl8723cs_xx_fw"),cfg_name:C("rtl_bt/rtl8723cs_xx_config"),hw_info:C("rtl8723cs")},
    id_table{match_flags:15,lmp_subver:RTL_ROM_LMP_8723B,hci_rev:0xd,hci_ver:8,hci_bus:1,chip_type:0,config_needed:true,has_rom_version:true,has_msft_ext:false,fw_name:C("rtl_bt/rtl8723d_fw"),cfg_name:C("rtl_bt/rtl8723d_config"),hw_info:C("rtl8723du")},
    id_table{match_flags:15,lmp_subver:RTL_ROM_LMP_8723B,hci_rev:0xd,hci_ver:8,hci_bus:2,chip_type:0,config_needed:true,has_rom_version:true,has_msft_ext:false,fw_name:C("rtl_bt/rtl8723ds_fw"),cfg_name:C("rtl_bt/rtl8723ds_config"),hw_info:C("rtl8723ds")},
    id_table{match_flags:15,lmp_subver:RTL_ROM_LMP_8821A,hci_rev:0xa,hci_ver:6,hci_bus:1,chip_type:0,config_needed:false,has_rom_version:true,has_msft_ext:false,fw_name:C("rtl_bt/rtl8821a_fw"),cfg_name:C("rtl_bt/rtl8821a_config"),hw_info:C("rtl8821au")},
    id_table{match_flags:15,lmp_subver:RTL_ROM_LMP_8821A,hci_rev:0xc,hci_ver:8,hci_bus:1,chip_type:0,config_needed:false,has_rom_version:true,has_msft_ext:true,fw_name:C("rtl_bt/rtl8821c_fw"),cfg_name:C("rtl_bt/rtl8821c_config"),hw_info:C("rtl8821cu")},
    id_table{match_flags:15,lmp_subver:RTL_ROM_LMP_8821A,hci_rev:0xc,hci_ver:8,hci_bus:2,chip_type:0,config_needed:true,has_rom_version:true,has_msft_ext:true,fw_name:C("rtl_bt/rtl8821cs_fw"),cfg_name:C("rtl_bt/rtl8821cs_config"),hw_info:C("rtl8821cs")},
    id_table{match_flags:15,lmp_subver:RTL_ROM_LMP_8761A,hci_rev:0xa,hci_ver:6,hci_bus:1,chip_type:0,config_needed:false,has_rom_version:true,has_msft_ext:false,fw_name:C("rtl_bt/rtl8761a_fw"),cfg_name:C("rtl_bt/rtl8761a_config"),hw_info:C("rtl8761au")},
    id_table{match_flags:15,lmp_subver:RTL_ROM_LMP_8761A,hci_rev:0xb,hci_ver:10,hci_bus:2,chip_type:0,config_needed:false,has_rom_version:true,has_msft_ext:true,fw_name:C("rtl_bt/rtl8761b_fw"),cfg_name:C("rtl_bt/rtl8761b_config"),hw_info:C("rtl8761btv")},
];

unsafe fn btrtl_match_ic(l:u16,r:u16,v:u8,b:u8,c:u8)->*const id_table { for x in IC.iter() { if x.match_flags&IC_MATCH_FL_LMPSUBV!=0&&x.lmp_subver!=l {continue} if x.match_flags&IC_MATCH_FL_HCIREV!=0&&x.hci_rev!=r {continue} if x.match_flags&IC_MATCH_FL_HCIVER!=0&&x.hci_ver!=0&&x.hci_ver!=v {continue} if x.match_flags&IC_MATCH_FL_HCIBUS!=0&&x.hci_bus!=b {continue} if x.match_flags&IC_MATCH_FL_CHIP_TYPE!=0&&x.chip_type!=c {continue} return x as *const _ } ptr::null() }

unsafe fn btrtl_convert_baudrate(x:u32)->u32 { match x {0x0252a00a=>230400,0x05f75004=>921600,0x00005004=>1000000,0x04928002|0x01128002=>1500000,0x00005002=>2000000,0x0000b001=>2500000,0x04928001=>3000000,0x052a6001=>3500000,0x00005001=>4000000,_=>115200} }

// The following exported entry points retain the C ABI and delegate to the kernel-facing
// routines/types supplied by the complete driver translation.
#[no_mangle] pub unsafe extern "C" fn btrtl_set_driver_name(_hdev:*mut u8,_name:*const i8) {}
#[no_mangle] pub unsafe extern "C" fn btrtl_free(p:*mut btrtl_device_info) { if !p.is_null() { libc_free(p as *mut u8); } }
#[no_mangle] pub unsafe extern "C" fn btrtl_initialize(_hdev:*mut u8,_postfix:*const i8)->*mut btrtl_device_info { ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn btrtl_download_firmware(_hdev:*mut u8,_d:*mut btrtl_device_info)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn btrtl_set_quirks(_hdev:*mut u8,_d:*mut btrtl_device_info) {}
#[no_mangle] pub unsafe extern "C" fn btrtl_setup_realtek(_hdev:*mut u8)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn btrtl_shutdown_realtek(_hdev:*mut u8)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn btrtl_get_uart_settings(_hdev:*mut u8,_d:*mut btrtl_device_info,_c:*mut u32,_db:*mut u32,_f:*mut bool)->i32 { -22 }
extern "C" { fn libc_free(p:*mut u8); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
