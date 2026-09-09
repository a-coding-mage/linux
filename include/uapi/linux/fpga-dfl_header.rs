/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Header File for FPGA DFL User API */

// Dependency intent: the C header includes linux/types.h and linux/ioctl.h.
// The ioctl encoding helpers (_IO, _IOR, _IOW) are supplied by those dependencies.

pub const DFL_FPGA_API_VERSION: u32 = 0;
pub const DFL_FPGA_MAGIC: u32 = 0xB6;
pub const DFL_FPGA_BASE: u32 = 0;
pub const DFL_PORT_BASE: u32 = 0x40;
pub const DFL_FME_BASE: u32 = 0x80;

pub const DFL_FPGA_GET_API_VERSION: _ = _IO(DFL_FPGA_MAGIC, DFL_FPGA_BASE + 0);
pub const DFL_FPGA_CHECK_EXTENSION: _ = _IO(DFL_FPGA_MAGIC, DFL_FPGA_BASE + 1);

pub const DFL_FPGA_PORT_RESET: _ = _IO(DFL_FPGA_MAGIC, DFL_PORT_BASE + 0);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_port_info {
    pub argsz: u32,
    pub flags: u32,
    pub num_regions: u32,
    pub num_umsgs: u32,
}

pub const DFL_FPGA_PORT_GET_INFO: _ = _IO(DFL_FPGA_MAGIC, DFL_PORT_BASE + 1);

pub const DFL_PORT_REGION_READ: u32 = 1 << 0;
pub const DFL_PORT_REGION_WRITE: u32 = 1 << 1;
pub const DFL_PORT_REGION_MMAP: u32 = 1 << 2;
pub const DFL_PORT_REGION_INDEX_AFU: u32 = 0;
pub const DFL_PORT_REGION_INDEX_STP: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_port_region_info {
    pub argsz: u32,
    pub flags: u32,
    pub index: u32,
    pub padding: u32,
    pub size: u64,
    pub offset: u64,
}

pub const DFL_FPGA_PORT_GET_REGION_INFO: _ = _IO(DFL_FPGA_MAGIC, DFL_PORT_BASE + 2);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_port_dma_map {
    pub argsz: u32,
    pub flags: u32,
    pub user_addr: u64,
    pub length: u64,
    pub iova: u64,
}

pub const DFL_FPGA_PORT_DMA_MAP: _ = _IO(DFL_FPGA_MAGIC, DFL_PORT_BASE + 3);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_port_dma_unmap {
    pub argsz: u32,
    pub flags: u32,
    pub iova: u64,
}

pub const DFL_FPGA_PORT_DMA_UNMAP: _ = _IO(DFL_FPGA_MAGIC, DFL_PORT_BASE + 4);

#[repr(C)]
pub struct dfl_fpga_irq_set {
    pub start: u32,
    pub count: u32,
    pub evtfds: [i32; 0],
}

pub const DFL_FPGA_PORT_ERR_GET_IRQ_NUM: _ =
    _IOR(DFL_FPGA_MAGIC, DFL_PORT_BASE + 5, u32);
pub const DFL_FPGA_PORT_ERR_SET_IRQ: _ =
    _IOW(DFL_FPGA_MAGIC, DFL_PORT_BASE + 6, dfl_fpga_irq_set);
pub const DFL_FPGA_PORT_UINT_GET_IRQ_NUM: _ =
    _IOR(DFL_FPGA_MAGIC, DFL_PORT_BASE + 7, u32);
pub const DFL_FPGA_PORT_UINT_SET_IRQ: _ =
    _IOW(DFL_FPGA_MAGIC, DFL_PORT_BASE + 8, dfl_fpga_irq_set);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_fme_port_pr {
    pub argsz: u32,
    pub flags: u32,
    pub port_id: u32,
    pub buffer_size: u32,
    pub buffer_address: u64,
}

pub const DFL_FPGA_FME_PORT_PR: _ = _IO(DFL_FPGA_MAGIC, DFL_FME_BASE + 0);
pub const DFL_FPGA_FME_PORT_RELEASE: _ =
    _IOW(DFL_FPGA_MAGIC, DFL_FME_BASE + 1, i32);
pub const DFL_FPGA_FME_PORT_ASSIGN: _ =
    _IOW(DFL_FPGA_MAGIC, DFL_FME_BASE + 2, i32);
pub const DFL_FPGA_FME_ERR_GET_IRQ_NUM: _ =
    _IOR(DFL_FPGA_MAGIC, DFL_FME_BASE + 3, u32);
pub const DFL_FPGA_FME_ERR_SET_IRQ: _ =
    _IOW(DFL_FPGA_MAGIC, DFL_FME_BASE + 4, dfl_fpga_irq_set);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
