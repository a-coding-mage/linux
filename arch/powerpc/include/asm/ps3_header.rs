/* SPDX-License-Identifier: GPL-2.0-only */
/* PS3 platform declarations. */

#[repr(C)]
pub union ps3_firmware_version {
    pub raw: u64,
    pub fields: ps3_firmware_version_fields,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_firmware_version_fields { pub pad: u16, pub major: u16, pub minor: u16, pub rev: u16 }

extern "C" {
    pub fn ps3_get_firmware_version(v: *mut ps3_firmware_version);
    pub fn ps3_compare_firmware_version(major: u16, minor: u16, rev: u16) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ps3_param_av_multi_out { PS3_PARAM_AV_MULTI_OUT_NTSC=0, PS3_PARAM_AV_MULTI_OUT_PAL_RGB=1, PS3_PARAM_AV_MULTI_OUT_PAL_YCBCR=2, PS3_PARAM_AV_MULTI_OUT_SECAM=3 }
extern "C" { pub fn ps3_os_area_get_av_multi_out() -> ps3_param_av_multi_out; pub fn ps3_os_area_get_rtc_diff() -> u64; pub fn ps3_os_area_set_rtc_diff(rtc_diff: u64); }

#[repr(C)]
pub struct ps3_os_area_flash_ops { pub read: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, loff_t) -> isize>, pub write: Option<unsafe extern "C" fn(*const core::ffi::c_void, usize, loff_t) -> isize> }
extern "C" { pub fn ps3_os_area_flash_register(ops: *const ps3_os_area_flash_ops); }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ps3_dma_page_size { PS3_DMA_4K=12, PS3_DMA_64K=16, PS3_DMA_1M=20, PS3_DMA_16M=24 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ps3_dma_region_type { PS3_DMA_OTHER=0, PS3_DMA_INTERNAL=2 }
#[repr(C)]
pub struct ps3_dma_region { pub dev: *mut ps3_system_bus_device, pub region_ops: *const ps3_dma_region_ops, pub ioid: u8, pub page_size: ps3_dma_page_size, pub region_type: ps3_dma_region_type, pub len: usize, pub offset: usize, pub dma_mask: u64, pub bus_addr: usize, pub chunk_list: ps3_dma_chunk_list }
#[repr(C)]
pub struct ps3_dma_chunk_list { pub lock: spinlock_t, pub head: list_head }
#[repr(C)]
pub struct ps3_dma_region_ops { pub create: Option<unsafe extern "C" fn(*mut ps3_dma_region)->i32>, pub free: Option<unsafe extern "C" fn(*mut ps3_dma_region)->i32>, pub map: Option<unsafe extern "C" fn(*mut ps3_dma_region, usize, usize, *mut dma_addr_t, u64)->i32>, pub unmap: Option<unsafe extern "C" fn(*mut ps3_dma_region, dma_addr_t, usize)->i32> }
extern "C" { pub fn ps3_dma_region_init(dev:*mut ps3_system_bus_device,r:*mut ps3_dma_region,page_size:ps3_dma_page_size,region_type:ps3_dma_region_type,addr:*mut core::ffi::c_void,len:usize)->i32; pub fn ps3_dma_region_create(r:*mut ps3_dma_region)->i32; pub fn ps3_dma_region_free(r:*mut ps3_dma_region)->i32; pub fn ps3_dma_map(r:*mut ps3_dma_region,virt_addr:usize,len:usize,bus_addr:*mut dma_addr_t,iopte_pp:u64)->i32; pub fn ps3_dma_unmap(r:*mut ps3_dma_region,bus_addr:dma_addr_t,len:usize)->i32; }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ps3_mmio_page_size { PS3_MMIO_4K=12, PS3_MMIO_64K=16 }
#[repr(C)] pub struct ps3_mmio_region { pub dev:*mut ps3_system_bus_device, pub mmio_ops:*const ps3_mmio_region_ops, pub bus_addr:usize, pub len:usize, pub page_size:ps3_mmio_page_size, pub lpar_addr:usize }
#[repr(C)] pub struct ps3_mmio_region_ops { pub create:Option<unsafe extern "C" fn(*mut ps3_mmio_region)->i32>, pub free:Option<unsafe extern "C" fn(*mut ps3_mmio_region)->i32> }
extern "C" { pub fn ps3_mmio_region_init(dev:*mut ps3_system_bus_device,r:*mut ps3_mmio_region,bus_addr:usize,len:usize,page_size:ps3_mmio_page_size)->i32; pub fn ps3_mmio_region_create(r:*mut ps3_mmio_region)->i32; pub fn ps3_free_mmio_region(r:*mut ps3_mmio_region)->i32; pub fn ps3_mm_phys_to_lpar(phys_addr:usize)->usize; }

#[repr(C)] #[derive(Copy,Clone)] pub enum ps3_cpu_binding { PS3_BINDING_CPU_ANY=-1, PS3_BINDING_CPU_0=0, PS3_BINDING_CPU_1=1 }
extern "C" { pub fn ps3_irq_plug_setup(cpu:ps3_cpu_binding,outlet:usize,virq:*mut u32)->i32; pub fn ps3_irq_plug_destroy(virq:u32)->i32; pub fn ps3_event_receive_port_setup(cpu:ps3_cpu_binding,virq:*mut u32)->i32; pub fn ps3_event_receive_port_destroy(virq:u32)->i32; pub fn ps3_send_event_locally(virq:u32)->i32; pub fn ps3_io_irq_setup(cpu:ps3_cpu_binding,interrupt_id:u32,virq:*mut u32)->i32; pub fn ps3_io_irq_destroy(virq:u32)->i32; pub fn ps3_vuart_irq_setup(cpu:ps3_cpu_binding,virt_addr_bmp:*mut core::ffi::c_void,virq:*mut u32)->i32; pub fn ps3_vuart_irq_destroy(virq:u32)->i32; pub fn ps3_spe_irq_setup(cpu:ps3_cpu_binding,spe_id:usize,class:u32,virq:*mut u32)->i32; pub fn ps3_spe_irq_destroy(virq:u32)->i32; pub fn ps3_sb_event_receive_port_setup(dev:*mut ps3_system_bus_device,cpu:ps3_cpu_binding,virq:*mut u32)->i32; pub fn ps3_sb_event_receive_port_destroy(dev:*mut ps3_system_bus_device,virq:u32)->i32; }

#[repr(C)] #[derive(Copy,Clone)] pub enum lv1_result { LV1_SUCCESS=0, LV1_RESOURCE_SHORTAGE=-2, LV1_NO_PRIVILEGE=-3, LV1_DENIED_BY_POLICY=-4, LV1_ACCESS_VIOLATION=-5, LV1_NO_ENTRY=-6, LV1_DUPLICATE_ENTRY=-7, LV1_TYPE_MISMATCH=-8, LV1_BUSY=-9, LV1_EMPTY=-10, LV1_WRONG_STATE=-11, LV1_NO_MATCH=-13, LV1_ALREADY_CONNECTED=-14, LV1_UNSUPPORTED_PARAMETER_VALUE=-15, LV1_CONDITION_NOT_SATISFIED=-16, LV1_ILLEGAL_PARAMETER_VALUE=-17, LV1_BAD_OPTION=-18, LV1_IMPLEMENTATION_LIMITATION=-19, LV1_NOT_IMPLEMENTED=-20, LV1_INVALID_CLASS_ID=-21, LV1_CONSTRAINT_NOT_SATISFIED=-22, LV1_ALIGNMENT_ERROR=-23, LV1_HARDWARE_ERROR=-24, LV1_INVALID_DATA_FORMAT=-25, LV1_INVALID_OPERATION=-26, LV1_INTERNAL_ERROR=-32768 }
// The C inline ps3_result is configuration-dependent (DEBUG/PS3_VERBOSE_RESULT/CONFIG_PS3_VERBOSE_RESULT).
pub unsafe fn ps3_result(result:i32) -> *const core::ffi::c_char { match result { 0=>b"LV1_SUCCESS (0)\0".as_ptr() as _, -2=>b"LV1_RESOURCE_SHORTAGE (-2)\0".as_ptr() as _, -3=>b"LV1_NO_PRIVILEGE (-3)\0".as_ptr() as _, -4=>b"LV1_DENIED_BY_POLICY (-4)\0".as_ptr() as _, -5=>b"LV1_ACCESS_VIOLATION (-5)\0".as_ptr() as _, -6=>b"LV1_NO_ENTRY (-6)\0".as_ptr() as _, -7=>b"LV1_DUPLICATE_ENTRY (-7)\0".as_ptr() as _, -8=>b"LV1_TYPE_MISMATCH (-8)\0".as_ptr() as _, -9=>b"LV1_BUSY (-9)\0".as_ptr() as _, -10=>b"LV1_EMPTY (-10)\0".as_ptr() as _, -11=>b"LV1_WRONG_STATE (-11)\0".as_ptr() as _, -13=>b"LV1_NO_MATCH (-13)\0".as_ptr() as _, -14=>b"LV1_ALREADY_CONNECTED (-14)\0".as_ptr() as _, -15=>b"LV1_UNSUPPORTED_PARAMETER_VALUE (-15)\0".as_ptr() as _, -16=>b"LV1_CONDITION_NOT_SATISFIED (-16)\0".as_ptr() as _, -17=>b"LV1_ILLEGAL_PARAMETER_VALUE (-17)\0".as_ptr() as _, -18=>b"LV1_BAD_OPTION (-18)\0".as_ptr() as _, -19=>b"LV1_IMPLEMENTATION_LIMITATION (-19)\0".as_ptr() as _, -20=>b"LV1_NOT_IMPLEMENTED (-20)\0".as_ptr() as _, -21=>b"LV1_INVALID_CLASS_ID (-21)\0".as_ptr() as _, -22=>b"LV1_CONSTRAINT_NOT_SATISFIED (-22)\0".as_ptr() as _, -23=>b"LV1_ALIGNMENT_ERROR (-23)\0".as_ptr() as _, -24=>b"LV1_HARDWARE_ERROR (-24)\0".as_ptr() as _, -25=>b"LV1_INVALID_DATA_FORMAT (-25)\0".as_ptr() as _, -26=>b"LV1_INVALID_OPERATION (-26)\0".as_ptr() as _, -32768=>b"LV1_INTERNAL_ERROR (-32768)\0".as_ptr() as _, _=>b"** unknown result **\0".as_ptr() as _ } }

#[repr(C)] #[derive(Copy,Clone)] pub enum ps3_match_id { PS3_MATCH_ID_EHCI=1, PS3_MATCH_ID_OHCI=2, PS3_MATCH_ID_GELIC=3, PS3_MATCH_ID_AV_SETTINGS=4, PS3_MATCH_ID_SYSTEM_MANAGER=5, PS3_MATCH_ID_STOR_DISK=6, PS3_MATCH_ID_STOR_ROM=7, PS3_MATCH_ID_STOR_FLASH=8, PS3_MATCH_ID_SOUND=9, PS3_MATCH_ID_GPU=10, PS3_MATCH_ID_LPM=11 }
#[repr(C)] #[derive(Copy,Clone)] pub enum ps3_match_sub_id { PS3_MATCH_SUB_ID_GPU_FB=1, PS3_MATCH_SUB_ID_GPU_RAMDISK=2 }
pub const PS3_MODULE_ALIAS_EHCI:&str="ps3:1:0"; pub const PS3_MODULE_ALIAS_OHCI:&str="ps3:2:0"; pub const PS3_MODULE_ALIAS_GELIC:&str="ps3:3:0"; pub const PS3_MODULE_ALIAS_AV_SETTINGS:&str="ps3:4:0"; pub const PS3_MODULE_ALIAS_SYSTEM_MANAGER:&str="ps3:5:0"; pub const PS3_MODULE_ALIAS_STOR_DISK:&str="ps3:6:0"; pub const PS3_MODULE_ALIAS_STOR_ROM:&str="ps3:7:0"; pub const PS3_MODULE_ALIAS_STOR_FLASH:&str="ps3:8:0"; pub const PS3_MODULE_ALIAS_SOUND:&str="ps3:9:0"; pub const PS3_MODULE_ALIAS_GPU_FB:&str="ps3:10:1"; pub const PS3_MODULE_ALIAS_GPU_RAMDISK:&str="ps3:10:2"; pub const PS3_MODULE_ALIAS_LPM:&str="ps3:11:0";
#[repr(C)] #[derive(Copy,Clone)] pub enum ps3_system_bus_device_type { PS3_DEVICE_TYPE_IOC0=1, PS3_DEVICE_TYPE_SB=2, PS3_DEVICE_TYPE_VUART=3, PS3_DEVICE_TYPE_LPM=4 }
#[repr(C)] pub struct ps3_system_bus_device { pub match_id:ps3_match_id, pub match_sub_id:ps3_match_sub_id, pub dev_type:ps3_system_bus_device_type, pub bus_id:u64, pub dev_id:u64, pub interrupt_id:u32, pub d_region:*mut ps3_dma_region, pub m_region:*mut ps3_mmio_region, pub port_number:u32, pub lpm:ps3_lpm_info, pub core:device, pub driver_priv:*mut core::ffi::c_void }
#[repr(C)] pub struct ps3_lpm_info { pub node_id:u64, pub pu_id:u64, pub rights:u64 }
extern "C" { pub fn ps3_open_hv_device(dev:*mut ps3_system_bus_device)->i32; pub fn ps3_close_hv_device(dev:*mut ps3_system_bus_device)->i32; }

#[repr(C)] pub struct ps3_system_bus_driver { pub match_id:ps3_match_id, pub match_sub_id:ps3_match_sub_id, pub core:device_driver, pub probe:Option<unsafe extern "C" fn(*mut ps3_system_bus_device)->i32>, pub remove:Option<unsafe extern "C" fn(*mut ps3_system_bus_device)>, pub shutdown:Option<unsafe extern "C" fn(*mut ps3_system_bus_device)> }
extern "C" { pub fn ps3_system_bus_device_register(dev:*mut ps3_system_bus_device)->i32; pub fn ps3_system_bus_driver_register(drv:*mut ps3_system_bus_driver)->i32; pub fn ps3_system_bus_driver_unregister(drv:*mut ps3_system_bus_driver); }
// container_of_const and container_of preserve the C kernel layout conversions.

#[repr(C)] pub struct ps3_sys_manager_ops { pub dev:*mut ps3_system_bus_device, pub power_off:Option<unsafe extern "C" fn(*mut ps3_system_bus_device)>, pub restart:Option<unsafe extern "C" fn(*mut ps3_system_bus_device)> }
extern "C" { pub fn ps3_sys_manager_register_ops(ops:*const ps3_sys_manager_ops); pub fn ps3_sys_manager_power_off()->!; pub fn ps3_sys_manager_restart()->!; pub fn ps3_sys_manager_halt()->!; pub fn ps3_sys_manager_get_wol()->i32; pub fn ps3_sys_manager_set_wol(state:i32); }
#[repr(C)] pub struct ps3_prealloc { pub name:*const core::ffi::c_char, pub address:*mut core::ffi::c_void, pub size:usize, pub align:usize }
extern "C" { pub static mut ps3fb_videomemory:ps3_prealloc; pub static mut ps3flash_bounce_buffer:ps3_prealloc; }
#[repr(C)] #[derive(Copy,Clone)] pub enum ps3_lpm_rights { PS3_LPM_RIGHTS_USE_LPM=0x001, PS3_LPM_RIGHTS_USE_TB=0x100 }
#[repr(C)] #[derive(Copy,Clone)] pub enum ps3_lpm_tb_type { PS3_LPM_TB_TYPE_NONE=0, PS3_LPM_TB_TYPE_INTERNAL=1 }
extern "C" { pub fn ps3_lpm_open(tb_type:ps3_lpm_tb_type,tb_cache:*mut core::ffi::c_void,tb_cache_size:u64)->i32; pub fn ps3_lpm_close()->i32; pub fn ps3_lpm_copy_tb(offset:usize,buf:*mut core::ffi::c_void,count:usize,bytes_copied:*mut usize)->i32; pub fn ps3_lpm_copy_tb_to_user(offset:usize,buf:*mut core::ffi::c_void,count:usize,bytes_copied:*mut usize)->i32; pub fn ps3_set_bookmark(bookmark:u64); pub fn ps3_set_pm_bookmark(tag:u64,incident:u64,th_id:u64); pub fn ps3_set_signal(rtas_signal_group:u64,signal_bit:u8,sub_unit:u16,bus_word:u8)->i32; pub fn ps3_read_phys_ctr(cpu:u32,phys_ctr:u32)->u32; pub fn ps3_write_phys_ctr(cpu:u32,phys_ctr:u32,val:u32); pub fn ps3_read_ctr(cpu:u32,ctr:u32)->u32; pub fn ps3_write_ctr(cpu:u32,ctr:u32,val:u32); pub fn ps3_read_pm07_control(cpu:u32,ctr:u32)->u32; pub fn ps3_write_pm07_control(cpu:u32,ctr:u32,val:u32); pub fn ps3_read_pm(cpu:u32,reg:pm_reg_name)->u32; pub fn ps3_write_pm(cpu:u32,reg:pm_reg_name,val:u32); pub fn ps3_get_ctr_size(cpu:u32,phys_ctr:u32)->u32; pub fn ps3_set_ctr_size(cpu:u32,phys_ctr:u32,ctr_size:u32); pub fn ps3_enable_pm(cpu:u32); pub fn ps3_disable_pm(cpu:u32); pub fn ps3_enable_pm_interrupts(cpu:u32,thread:u32,mask:u32); pub fn ps3_disable_pm_interrupts(cpu:u32); pub fn ps3_get_and_clear_pm_interrupts(cpu:u32)->u32; pub fn ps3_sync_irq(node:i32); pub fn ps3_get_hw_thread_id(cpu:i32)->u32; pub fn ps3_get_spe_id(arg:*mut core::ffi::c_void)->u64; pub fn ps3_early_mm_init(); pub fn udbg_shutdown_ps3gelic(); }

// External dependency types supplied by the included kernel headers.
// type aliases are intentionally not invented here: device, spinlock_t, list_head,
// dma_addr_t, loff_t, pm_reg_name, and device_driver are future dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
