// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Based on Ocelot Linux port, which is
 * Copyright 2001 MontaVista Software Inc.
 * Author: jsun@mvista.com or jsun@junsun.net
 *
 * Copyright 2003 ICT CAS
 * Author: Michael Guo <guoyi@ict.ac.cn>
 *
 * Copyright (C) 2007 Lemote Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 *
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

const HOST_BRIDGE_CONFIG_ADDR: *mut core::ffi::c_void = 0x1a000000usize as *mut core::ffi::c_void;

pub static mut cpu_clock_freq: u32 = 0;
pub static mut loongson_memmap: *mut efi_memory_map_loongson = core::ptr::null_mut();
pub static mut loongson_sysconf: loongson_system_configuration = unsafe { core::mem::zeroed() };

pub static mut eboard: *mut board_devices = core::ptr::null_mut();
pub static mut einter: *mut interface_info = core::ptr::null_mut();
pub static mut especial: *mut loongson_special_attribute = core::ptr::null_mut();

pub static mut loongson_chipcfg: [u64; MAX_PACKAGES] = [0xffffffffbfc00180; MAX_PACKAGES];
pub static mut loongson_chiptemp: [u64; MAX_PACKAGES] = [0; MAX_PACKAGES];
pub static mut loongson_freqctrl: [u64; MAX_PACKAGES] = [0; MAX_PACKAGES];
pub static mut smp_group: [u64; 4] = [0; 4];

pub unsafe fn get_system_type() -> *const core::ffi::c_char {
    b"Generic Loongson64 System\0".as_ptr() as *const core::ffi::c_char
}

pub unsafe fn prom_dtb_init_env() {
    if (fw_arg2 < CKSEG0 || fw_arg2 > CKSEG1) && (fw_arg2 < XKPHYS || fw_arg2 > XKSEG) {
        loongson_fdt_blob = __dtb_loongson64_2core_2k1000_begin;
    } else {
        loongson_fdt_blob = fw_arg2 as *mut core::ffi::c_void;
    }
}

unsafe fn lefi_fixup_fdt_serial(fdt: *mut core::ffi::c_void, uart_addr: u64, uart_clk: u32) -> i32 {
    let mut node: i32;
    let mut len: i32 = 0;
    let mut depth: i32 = -1;
    let mut reg: *const fdt64_t;
    let mut clk: *mut fdt32_t;

    node = fdt_next_node(fdt, -1, &mut depth);
    while node >= 0 && depth >= 0 {
        reg = fdt_getprop(fdt, node, b"reg\0".as_ptr() as *const _, &mut len);
        if !reg.is_null() && len > 8 && fdt64_ld(reg) == uart_addr {
            clk = fdt_getprop_w(fdt, node, b"clock-frequency\0".as_ptr() as *const _, &mut len);
            if clk.is_null() {
                pr_warn(b"UART 0x%llx misses clock-frequency property\n\0".as_ptr() as *const _, uart_addr);
                return -ENOENT;
            } else if len != 4 {
                pr_warn(b"UART 0x%llx has invalid clock-frequency property\n\0".as_ptr() as *const _, uart_addr);
                return -EINVAL;
            }
            fdt32_st(clk, uart_clk);
            return 0;
        }
        node = fdt_next_node(fdt, node, &mut depth);
    }
    -ENODEV
}

unsafe fn lefi_fixup_fdt(system: *mut system_loongson) {
    static mut fdt_buf: [u8; 16 << 10] = [0; 16 << 10];
    let mut uartdev: *mut uart_device;
    let is_loongson64g = (read_c0_prid() & PRID_IMP_MASK) == PRID_IMP_LOONGSON_64G;
    let mut uart_base: u64 = 0;
    let mut ret: i32;

    ret = fdt_open_into(loongson_fdt_blob, fdt_buf.as_mut_ptr() as *mut _, core::mem::size_of_val(&fdt_buf));
    if ret != 0 { pr_err(b"Failed to open FDT to fix up\n\0".as_ptr() as *const _); return; }

    let nr = core::cmp::min((*system).nr_uarts as usize, MAX_UARTS as usize);
    for i in 0..nr {
        uartdev = (*system).uarts.as_mut_ptr().add(i);
        if (*uartdev).uart_base == 0 { continue; }
        if (*uartdev).iotype != UPIO_MEM {
            pr_warn(b"Ignore UART 0x%llx with iotype %u passed by firmware\n\0".as_ptr() as *const _, (*uartdev).uart_base, (*uartdev).iotype);
            continue;
        }
        ret = lefi_fixup_fdt_serial(fdt_buf.as_mut_ptr() as *mut _, (*uartdev).uart_base, (*uartdev).uartclk);
        if ret == -ENODEV && is_loongson64g {
            uart_base = match (*uartdev).uart_base {
                0x1fe00100 => 0x1fe001e0, 0x1fe00110 => 0x1fe001e8,
                0x1fe001e0 => 0x1fe00100, 0x1fe001e8 => 0x1fe00110,
                _ => { pr_err(b"Unexpected UART address 0x%llx passed by firmware\n\0".as_ptr() as *const _, (*uartdev).uart_base); ret = -EINVAL; 0 }
            };
            if ret == 0 { ret = lefi_fixup_fdt_serial(fdt_buf.as_mut_ptr() as *mut _, uart_base, (*uartdev).uartclk); }
        }
        if ret != 0 { pr_err(b"Couldn't fix up FDT node for UART 0x%llx\n\0".as_ptr() as *const _, (*uartdev).uart_base); }
    }
    loongson_fdt_blob = fdt_buf.as_mut_ptr() as *mut _;
}

pub unsafe fn prom_lefi_init_env() {
    let boot_p = fw_arg2 as *mut boot_params;
    let loongson_p = &mut (*boot_p).efi.smbios.lp;
    let esys = (loongson_p as *mut _ as u64 + loongson_p.system_offset as u64) as *mut system_loongson;
    let ecpu = (loongson_p as *mut _ as u64 + loongson_p.cpu_offset as u64) as *mut efi_cpuinfo_loongson;
    eboard = (loongson_p as *mut _ as u64 + loongson_p.boarddev_table_offset as u64) as *mut board_devices;
    einter = (loongson_p as *mut _ as u64 + loongson_p.interface_offset as u64) as *mut interface_info;
    especial = (loongson_p as *mut _ as u64 + loongson_p.special_offset as u64) as *mut loongson_special_attribute;
    let eirq_source = (loongson_p as *mut _ as u64 + loongson_p.irq_offset as u64) as *mut irq_source_routing_table;
    loongson_memmap = (loongson_p as *mut _ as u64 + loongson_p.memory_offset as u64) as *mut efi_memory_map_loongson;
    cpu_clock_freq = (*ecpu).cpu_clock_freq;
    loongson_sysconf.cputype = (*ecpu).cputype;
    match (*ecpu).cputype {
        Legacy_2K | Loongson_2K => { smp_group[0] = 0x900000001fe11000; loongson_sysconf.cores_per_node = 2; loongson_sysconf.cores_per_package = 2; }
        Legacy_3A | Loongson_3A => { loongson_sysconf.cores_per_node = 4; loongson_sysconf.cores_per_package = 4; smp_group = [0x900000003ff01000,0x900010003ff01000,0x900020003ff01000,0x900030003ff01000]; loongson_chipcfg = [0x900000001fe00180,0x900010001fe00180,0x900020001fe00180,0x900030001fe00180]; loongson_chiptemp = [0x900000001fe0019c,0x900010001fe0019c,0x900020001fe0019c,0x900030001fe0019c]; loongson_freqctrl = [0x900000001fe001d0,0x900010001fe001d0,0x900020001fe001d0,0x900030001fe001d0]; loongson_sysconf.workarounds = WORKAROUND_CPUFREQ; }
        Legacy_3B | Loongson_3B => { loongson_sysconf.cores_per_node = 4; loongson_sysconf.cores_per_package = 8; smp_group = [0x900000003ff01000,0x900010003ff05000,0x900020003ff09000,0x900030003ff0d000]; loongson_chipcfg = [0x900000001fe00180,0x900020001fe00180,0x900040001fe00180,0x900060001fe00180]; loongson_chiptemp = [0x900000001fe0019c,0x900020001fe0019c,0x900040001fe0019c,0x900060001fe0019c]; loongson_freqctrl = [0x900000001fe001d0,0x900020001fe001d0,0x900040001fe001d0,0x900060001fe001d0]; loongson_sysconf.workarounds = WORKAROUND_CPUHOTPLUG; }
        _ => { loongson_sysconf.cores_per_node = 1; loongson_sysconf.cores_per_package = 1; loongson_chipcfg[0] = 0x900000001fe00180; }
    }
    loongson_sysconf.nr_cpus = (*ecpu).nr_cpus;
    loongson_sysconf.boot_cpu_id = (*ecpu).cpu_startup_core_id;
    loongson_sysconf.reserved_cpus_mask = (*ecpu).reserved_cores_mask;
    if (*ecpu).nr_cpus > NR_CPUS || (*ecpu).nr_cpus == 0 { loongson_sysconf.nr_cpus = NR_CPUS; }
    loongson_sysconf.nr_nodes = (loongson_sysconf.nr_cpus + loongson_sysconf.cores_per_node - 1) / loongson_sysconf.cores_per_node;
    loongson_sysconf.dma_mask_bits = (*eirq_source).dma_mask_bits;
    if loongson_sysconf.dma_mask_bits < 32 || loongson_sysconf.dma_mask_bits > 64 { loongson_sysconf.dma_mask_bits = 32; dma_default_coherent = true; } else { dma_default_coherent = !(*eirq_source).dma_noncoherent; }
    loongson_sysconf.restart_addr = (*boot_p).reset_system.ResetWarm; loongson_sysconf.poweroff_addr = (*boot_p).reset_system.Shutdown; loongson_sysconf.suspend_addr = (*boot_p).reset_system.DoSuspend;
    loongson_sysconf.vgabios_addr = (*boot_p).efi.smbios.vga_bios;
    loongson_sysconf.workarounds |= (*esys).workarounds;
    let id = readl(HOST_BRIDGE_CONFIG_ADDR); let vendor = id & 0xffff;
    match vendor { PCI_VENDOR_ID_LOONGSON => { loongson_sysconf.bridgetype = LS7A; loongson_sysconf.early_config = ls7a_early_config; }, PCI_VENDOR_ID_AMD | PCI_VENDOR_ID_ATI => { loongson_sysconf.bridgetype = RS780E; loongson_sysconf.early_config = rs780e_early_config; }, _ => { loongson_sysconf.bridgetype = VIRTUAL; loongson_sysconf.early_config = virtual_early_config; loongson_fdt_blob = __dtb_loongson64v_4core_virtio_begin; } }
    let prid = read_c0_prid();
    if (prid & PRID_IMP_MASK) == PRID_IMP_LOONGSON_64C {
        match prid & PRID_REV_MASK {
            PRID_REV_LOONGSON3A_R1 | PRID_REV_LOONGSON3A_R2_0 | PRID_REV_LOONGSON3A_R2_1 | PRID_REV_LOONGSON3A_R3_0 | PRID_REV_LOONGSON3A_R3_1 => {
                match loongson_sysconf.bridgetype { LS7A => loongson_fdt_blob = __dtb_loongson64c_4core_ls7a_begin, RS780E => loongson_fdt_blob = __dtb_loongson64c_4core_rs780e_begin, _ => {} }
            }
            PRID_REV_LOONGSON3B_R1 | PRID_REV_LOONGSON3B_R2 => { if loongson_sysconf.bridgetype == RS780E { loongson_fdt_blob = __dtb_loongson64c_8core_rs780e_begin; } }
            _ => {}
        }
    } else if (prid & PRID_IMP_MASK) == PRID_IMP_LOONGSON_64R {
        loongson_fdt_blob = __dtb_loongson64_2core_2k1000_begin;
    } else if (prid & PRID_IMP_MASK) == PRID_IMP_LOONGSON_64G {
        if loongson_sysconf.bridgetype == LS7A { loongson_fdt_blob = __dtb_loongson64g_4core_ls7a_begin; }
    }
    if !loongson_fdt_blob.is_null() { lefi_fixup_fdt(esys); } else { pr_err(b"Failed to determine built-in Loongson64 dtb\n\0".as_ptr() as *const _); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
