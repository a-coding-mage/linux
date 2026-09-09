/* SPDX-License-Identifier: GPL-2.0-only */
/* PS3 platform declarations. */

/* C header dependencies: linux/rtc.h, scsi/scsi.h, and asm/ps3.h. */

extern "C" {
    pub fn ps3_hpte_init(htab_size: usize);
    pub fn ps3_map_htab();
    pub fn ps3_mm_init();
    pub fn ps3_mm_vas_create(htab_size: *mut usize);
    pub fn ps3_mm_vas_destroy();
    pub fn ps3_mm_shutdown();
    pub fn ps3_init_IRQ();
    pub fn ps3_shutdown_IRQ(cpu: i32);
    pub fn ps3_register_ipi_debug_brk(cpu: u32, virq: u32);
    pub fn ps3_register_ipi_irq(cpu: u32, virq: u32);
    pub fn smp_init_ps3();
    pub fn ps3_calibrate_decr();
    pub fn ps3_get_boot_time() -> time64_t;
    pub fn ps3_get_rtc_time(time: *mut rtc_time);
    pub fn ps3_set_rtc_time(time: *mut rtc_time) -> i32;
    pub fn ps3_os_area_save_params();
    pub fn ps3_os_area_init();
    pub fn ps3_spu_set_platform();

    pub fn ps3_repository_read_bus_str(bus_index: u32, bus_str: *const i8, value: *mut u64) -> i32;
    pub fn ps3_repository_read_bus_id(bus_index: u32, bus_id: *mut u64) -> i32;
    pub fn ps3_repository_read_bus_type(bus_index: u32, bus_type: *mut ps3_bus_type) -> i32;
    pub fn ps3_repository_read_bus_num_dev(bus_index: u32, num_dev: *mut u32) -> i32;
    pub fn ps3_repository_read_dev_str(bus_index: u32, dev_index: u32, dev_str: *const i8, value: *mut u64) -> i32;
    pub fn ps3_repository_read_dev_id(bus_index: u32, dev_index: u32, dev_id: *mut u64) -> i32;
    pub fn ps3_repository_read_dev_type(bus_index: u32, dev_index: u32, dev_type: *mut ps3_dev_type) -> i32;
    pub fn ps3_repository_read_dev_intr(bus_index: u32, dev_index: u32, intr_index: u32, intr_type: *mut ps3_interrupt_type, interrupt_id: *mut u32) -> i32;
    pub fn ps3_repository_read_dev_reg_type(bus_index: u32, dev_index: u32, reg_index: u32, reg_type: *mut ps3_reg_type) -> i32;
    pub fn ps3_repository_read_dev_reg_addr(bus_index: u32, dev_index: u32, reg_index: u32, bus_addr: *mut u64, len: *mut u64) -> i32;
    pub fn ps3_repository_read_dev_reg(bus_index: u32, dev_index: u32, reg_index: u32, reg_type: *mut ps3_reg_type, bus_addr: *mut u64, len: *mut u64) -> i32;

    pub fn ps3_repository_find_device(repo: *mut ps3_repository_device) -> i32;
    pub fn ps3_repository_find_device_by_id(repo: *mut ps3_repository_device, bus_id: u64, dev_id: u64) -> i32;
    pub fn ps3_repository_find_devices(bus_type: ps3_bus_type, callback: Option<unsafe extern "C" fn(*const ps3_repository_device) -> i32>) -> i32;
    pub fn ps3_repository_find_bus(bus_type: ps3_bus_type, from: u32, bus_index: *mut u32) -> i32;
    pub fn ps3_repository_find_interrupt(repo: *const ps3_repository_device, intr_type: ps3_interrupt_type, interrupt_id: *mut u32) -> i32;
    pub fn ps3_repository_find_reg(repo: *const ps3_repository_device, reg_type: ps3_reg_type, bus_addr: *mut u64, len: *mut u64) -> i32;

    pub fn ps3_repository_read_stor_dev_port(bus_index: u32, dev_index: u32, port: *mut u64) -> i32;
    pub fn ps3_repository_read_stor_dev_blk_size(bus_index: u32, dev_index: u32, blk_size: *mut u64) -> i32;
    pub fn ps3_repository_read_stor_dev_num_blocks(bus_index: u32, dev_index: u32, num_blocks: *mut u64) -> i32;
    pub fn ps3_repository_read_stor_dev_num_regions(bus_index: u32, dev_index: u32, num_regions: *mut u32) -> i32;
    pub fn ps3_repository_read_stor_dev_region_id(bus_index: u32, dev_index: u32, region_index: u32, region_id: *mut u32) -> i32;
    pub fn ps3_repository_read_stor_dev_region_size(bus_index: u32, dev_index: u32, region_index: u32, region_size: *mut u64) -> i32;
    pub fn ps3_repository_read_stor_dev_region_start(bus_index: u32, dev_index: u32, region_index: u32, region_start: *mut u64) -> i32;
    pub fn ps3_repository_read_stor_dev_info(bus_index: u32, dev_index: u32, port: *mut u64, blk_size: *mut u64, num_blocks: *mut u64, num_regions: *mut u32) -> i32;
    pub fn ps3_repository_read_stor_dev_region(bus_index: u32, dev_index: u32, region_index: u32, region_id: *mut u32, region_start: *mut u64, region_size: *mut u64) -> i32;

    pub fn ps3_repository_read_num_pu(num_pu: *mut u64) -> i32;
    pub fn ps3_repository_read_pu_id(pu_index: u32, pu_id: *mut u64) -> i32;
    pub fn ps3_repository_read_rm_base(ppe_id: u32, rm_base: *mut u64) -> i32;
    pub fn ps3_repository_read_rm_size(ppe_id: u32, rm_size: *mut u64) -> i32;
    pub fn ps3_repository_read_region_total(region_total: *mut u64) -> i32;
    pub fn ps3_repository_read_mm_info(rm_base: *mut u64, rm_size: *mut u64, region_total: *mut u64) -> i32;
    pub fn ps3_repository_read_highmem_region_count(region_count: *mut u32) -> i32;
    pub fn ps3_repository_read_highmem_base(region_index: u32, highmem_base: *mut u64) -> i32;
    pub fn ps3_repository_read_highmem_size(region_index: u32, highmem_size: *mut u64) -> i32;
    pub fn ps3_repository_read_highmem_info(region_index: u32, highmem_base: *mut u64, highmem_size: *mut u64) -> i32;
    pub fn ps3_repository_read_num_be(num_be: *mut u32) -> i32;
    pub fn ps3_repository_read_be_node_id(be_index: u32, node_id: *mut u64) -> i32;
    pub fn ps3_repository_read_be_id(node_id: u64, be_id: *mut u64) -> i32;
    pub fn ps3_repository_read_tb_freq(node_id: u64, tb_freq: *mut u64) -> i32;
    pub fn ps3_repository_read_be_tb_freq(be_index: u32, tb_freq: *mut u64) -> i32;
    pub fn ps3_repository_read_lpm_privileges(be_index: u32, lpar: *mut u64, rights: *mut u64) -> i32;
    pub fn ps3_repository_read_boot_dat_addr(lpar_addr: *mut u64) -> i32;
    pub fn ps3_repository_read_boot_dat_size(size: *mut u32) -> i32;
    pub fn ps3_repository_read_boot_dat_info(lpar_addr: *mut u64, size: *mut u32) -> i32;
    pub fn ps3_repository_read_num_spu_reserved(num_spu_reserved: *mut u32) -> i32;
    pub fn ps3_repository_read_num_spu_resource_id(num_resource_id: *mut u32) -> i32;
    pub fn ps3_repository_read_spu_resource_id(res_index: u32, resource_type: *mut ps3_spu_resource_type, resource_id: *mut u32) -> i32;
    pub fn ps3_repository_read_vuart_av_port(port: *mut u32) -> i32;
    pub fn ps3_repository_read_vuart_sysmgr_port(port: *mut u32) -> i32;
}

pub type time64_t = i64;
#[repr(C)]
pub struct rtc_time { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ps3_bus_type { PS3_BUS_TYPE_SB = 4, PS3_BUS_TYPE_STORAGE = 5 }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ps3_dev_type { PS3_DEV_TYPE_STOR_DISK = 0, PS3_DEV_TYPE_SB_GELIC = 3, PS3_DEV_TYPE_SB_USB = 4, PS3_DEV_TYPE_STOR_ROM = 5, PS3_DEV_TYPE_SB_GPIO = 6, PS3_DEV_TYPE_STOR_FLASH = 14 }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ps3_interrupt_type { PS3_INTERRUPT_TYPE_EVENT_PORT = 2, PS3_INTERRUPT_TYPE_SB_OHCI = 3, PS3_INTERRUPT_TYPE_SB_EHCI = 4, PS3_INTERRUPT_TYPE_OTHER = 5 }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ps3_reg_type { PS3_REG_TYPE_SB_OHCI = 3, PS3_REG_TYPE_SB_EHCI = 4, PS3_REG_TYPE_SB_GPIO = 5 }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ps3_repository_device { pub bus_index: u32, pub dev_index: u32, pub bus_type: ps3_bus_type, pub dev_type: ps3_dev_type, pub bus_id: u64, pub dev_id: u64 }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ps3_spu_resource_type { PS3_SPU_RESOURCE_TYPE_SHARED = 0, PS3_SPU_RESOURCE_TYPE_EXCLUSIVE = 0x8000000000000000u64 as isize }

/* CONFIG_SMP, CONFIG_SPU_BASE, and CONFIG_PS3_REPOSITORY_WRITE conditional declarations. */
#[inline]
pub fn ps3_smp_cleanup_cpu(_cpu: i32) {}
#[inline]
pub fn ps3_repository_write_highmem_region_count(_region_count: u32) -> i32 { 0 }
#[inline]
pub fn ps3_repository_write_highmem_base(_region_index: u32, _highmem_base: u64) -> i32 { 0 }
#[inline]
pub fn ps3_repository_write_highmem_size(_region_index: u32, _highmem_size: u64) -> i32 { 0 }
#[inline]
pub fn ps3_repository_write_highmem_info(_region_index: u32, _highmem_base: u64, _highmem_size: u64) -> i32 { 0 }
#[inline]
pub fn ps3_repository_delete_highmem_info(_region_index: u32) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
