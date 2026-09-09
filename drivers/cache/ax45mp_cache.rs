// SPDX-License-Identifier: GPL-2.0
/*
 * non-coherent cache functions for Andes AX45MP
 *
 * Copyright (C) 2023 Renesas Electronics Corp.
 */

// Dependencies supplied by the kernel translation environment.

const AX45MP_L2C_REG_CTL_OFFSET: usize = 0x8;
const AX45MP_L2C_REG_C0_CMD_OFFSET: usize = 0x40;
const AX45MP_L2C_REG_C0_ACC_OFFSET: usize = 0x48;
const AX45MP_L2C_REG_STATUS_OFFSET: usize = 0x80;

const AX45MP_CCTL_L1D_VA_INVAL: u32 = 0;
const AX45MP_CCTL_L1D_VA_WB: u32 = 1;
const AX45MP_CCTL_L2_STATUS_IDLE: u32 = 0;
const AX45MP_CCTL_L2_STATUS_C0_MASK: u32 = 0xf;
const AX45MP_CCTL_L2_PA_INVAL: u32 = 0x8;
const AX45MP_CCTL_L2_PA_WB: u32 = 0x9;
const AX45MP_L2C_REG_PER_CORE_OFFSET: usize = 0x10;
const AX45MP_CCTL_L2_STATUS_PER_CORE_OFFSET: usize = 4;
const AX45MP_CCTL_REG_UCCTLBEGINADDR_NUM: usize = 0x80b;
const AX45MP_CCTL_REG_UCCTLCOMMAND_NUM: usize = 0x80c;
const AX45MP_CACHE_LINE_SIZE: u32 = 64;

#[repr(C)]
struct ax45mp_priv {
    l2c_base: *mut core::ffi::c_void,
    ax45mp_cache_line_size: u32,
}

static mut ax45mp_priv: ax45mp_priv = ax45mp_priv {
    l2c_base: core::ptr::null_mut(),
    ax45mp_cache_line_size: 0,
};

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn smp_processor_id() -> i32;
    fn csr_write(reg: usize, value: usize);
    fn virt_to_phys(addr: *mut core::ffi::c_void) -> usize;
    fn phys_to_virt(addr: usize) -> *mut core::ffi::c_void;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn of_property_read_u32(np: *mut device_node, name: *const u8, value: *mut u32) -> i32;
    fn pr_err(fmt: *const u8, ...);
    fn of_find_matching_node(from: *mut device_node, matches: *const of_device_id) -> *mut device_node;
    fn of_device_is_available(np: *mut device_node) -> bool;
    fn of_address_to_resource(np: *mut device_node, index: u32, res: *mut resource) -> i32;
    fn resource_size(res: *const resource) -> usize;
    fn ioremap(start: usize, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn riscv_noncoherent_register_cache_ops(ops: *const riscv_nonstd_cache_ops);
}

#[repr(C)] struct device_node;
#[repr(C)] struct resource { start: usize }
#[repr(C)] struct of_device_id { compatible: *const u8 }
#[repr(C)] struct riscv_nonstd_cache_ops {
    wback: Option<unsafe extern "C" fn(usize, usize)>,
    inv: Option<unsafe extern "C" fn(usize, usize)>,
    wback_inv: Option<unsafe extern "C" fn(usize, usize)>,
}

extern "C" { static riscv_cbom_block_size: usize; }

const fn ax45mp_l2c_reg_cn_cmd_offset(n: usize) -> usize {
    AX45MP_L2C_REG_C0_CMD_OFFSET + n * AX45MP_L2C_REG_PER_CORE_OFFSET
}
const fn ax45mp_l2c_reg_cn_acc_offset(n: usize) -> usize {
    AX45MP_L2C_REG_C0_ACC_OFFSET + n * AX45MP_L2C_REG_PER_CORE_OFFSET
}
const fn ax45mp_cctl_l2_status_cn_mask(n: usize) -> u32 {
    AX45MP_CCTL_L2_STATUS_C0_MASK << (n * AX45MP_CCTL_L2_STATUS_PER_CORE_OFFSET)
}

unsafe fn ax45mp_cpu_l2c_get_cctl_status() -> u32 {
    readl(ax45mp_priv.l2c_base.add(AX45MP_L2C_REG_STATUS_OFFSET))
}

unsafe fn ax45mp_cpu_cache_operation(mut start: usize, end: usize, l1_op: u32, l2_op: u32) {
    let line_size = ax45mp_priv.ax45mp_cache_line_size as usize;
    let base = ax45mp_priv.l2c_base;
    let mhartid = smp_processor_id() as usize;
    while end > start {
        csr_write(AX45MP_CCTL_REG_UCCTLBEGINADDR_NUM, start);
        csr_write(AX45MP_CCTL_REG_UCCTLCOMMAND_NUM, l1_op as usize);
        let pa = virt_to_phys(start as *mut core::ffi::c_void);
        writel(pa as u32, base.add(ax45mp_l2c_reg_cn_acc_offset(mhartid)));
        writel(l2_op, base.add(ax45mp_l2c_reg_cn_cmd_offset(mhartid)));
        while (ax45mp_cpu_l2c_get_cctl_status() & ax45mp_cctl_l2_status_cn_mask(mhartid)) != AX45MP_CCTL_L2_STATUS_IDLE {}
        start += line_size;
    }
}

unsafe extern "C" fn ax45mp_cpu_dcache_wb_range(start: usize, end: usize) {
    ax45mp_cpu_cache_operation(start, end, AX45MP_CCTL_L1D_VA_WB, AX45MP_CCTL_L2_PA_WB);
}
unsafe extern "C" fn ax45mp_cpu_dcache_inval_range(start: usize, end: usize) {
    ax45mp_cpu_cache_operation(start, end, AX45MP_CCTL_L1D_VA_INVAL, AX45MP_CCTL_L2_PA_INVAL);
}

unsafe extern "C" fn ax45mp_dma_cache_inv(paddr: usize, size: usize) {
    let mut start = phys_to_virt(paddr) as usize;
    let end = start + size;
    if start == end { return; }
    let line_size = ax45mp_priv.ax45mp_cache_line_size as usize;
    start &= !(line_size - 1);
    let end = (end + line_size - 1) & !(line_size - 1);
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    ax45mp_cpu_dcache_inval_range(start, end);
    local_irq_restore(flags);
}

unsafe extern "C" fn ax45mp_dma_cache_wback(paddr: usize, size: usize) {
    let mut start = phys_to_virt(paddr) as usize;
    let end = start + size;
    if start == end { return; }
    let line_size = ax45mp_priv.ax45mp_cache_line_size as usize;
    start &= !(line_size - 1);
    let end = (end + line_size - 1) & !(line_size - 1);
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    ax45mp_cpu_dcache_wb_range(start, end);
    local_irq_restore(flags);
}

unsafe extern "C" fn ax45mp_dma_cache_wback_inv(paddr: usize, size: usize) {
    ax45mp_dma_cache_wback(paddr, size);
    ax45mp_dma_cache_inv(paddr, size);
}

unsafe fn ax45mp_get_l2_line_size(np: *mut device_node) -> i32 {
    let ret = of_property_read_u32(np, b"cache-line-size\0".as_ptr(), &mut ax45mp_priv.ax45mp_cache_line_size);
    if ret != 0 { pr_err(b"Failed to get cache-line-size, defaulting to 64 bytes\n\0".as_ptr()); return ret; }
    if ax45mp_priv.ax45mp_cache_line_size != AX45MP_CACHE_LINE_SIZE {
        pr_err(b"Expected cache-line-size to be 64 bytes (found:%u)\n\0".as_ptr(), ax45mp_priv.ax45mp_cache_line_size);
        return -22;
    }
    0
}

static ax45mp_cmo_ops: riscv_nonstd_cache_ops = riscv_nonstd_cache_ops {
    wback: Some(ax45mp_dma_cache_wback), inv: Some(ax45mp_dma_cache_inv), wback_inv: Some(ax45mp_dma_cache_wback_inv),
};
static ax45mp_cache_ids: [of_device_id; 2] = [
    of_device_id { compatible: b"andestech,ax45mp-cache\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

unsafe extern "C" fn ax45mp_cache_init() -> i32 {
    let mut res = resource { start: 0 };
    let np = of_find_matching_node(core::ptr::null_mut(), ax45mp_cache_ids.as_ptr());
    if !of_device_is_available(np) { return -19; }
    let ret = of_address_to_resource(np, 0, &mut res);
    if ret != 0 { return ret; }
    if riscv_cbom_block_size == 0 { return 0; }
    ax45mp_priv.l2c_base = ioremap(res.start, resource_size(&res));
    if ax45mp_priv.l2c_base.is_null() { return -12; }
    let ret = ax45mp_get_l2_line_size(np);
    if ret != 0 { iounmap(ax45mp_priv.l2c_base); return ret; }
    riscv_noncoherent_register_cache_ops(&ax45mp_cmo_ops);
    0
}

// early_initcall(ax45mp_cache_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
