/*
 * Copyright 2021 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

/* Dependency: Linux ioctl encoding (_IOWR). */

/* MMIO debugfs IOCTL structure */
#[repr(C)]
pub struct amdgpu_debugfs_regs2_iocdata_grbm {
    pub se: u32,
    pub sh: u32,
    pub instance: u32,
}

#[repr(C)]
pub struct amdgpu_debugfs_regs2_iocdata_srbm {
    pub me: u32,
    pub pipe: u32,
    pub queue: u32,
    pub vmid: u32,
}

#[repr(C)]
pub struct amdgpu_debugfs_regs2_iocdata {
    pub use_srbm: u32,
    pub use_grbm: u32,
    pub pg_lock: u32,
    pub grbm: amdgpu_debugfs_regs2_iocdata_grbm,
    pub srbm: amdgpu_debugfs_regs2_iocdata_srbm,
}

#[repr(C)]
pub struct amdgpu_debugfs_regs2_iocdata_v2 {
    pub use_srbm: u32,
    pub use_grbm: u32,
    pub pg_lock: u32,
    pub grbm: amdgpu_debugfs_regs2_iocdata_grbm,
    pub srbm: amdgpu_debugfs_regs2_iocdata_srbm,
    pub xcc_id: u32,
}

#[repr(C)]
pub struct amdgpu_debugfs_gprwave_iocdata_gpr {
    pub thread: u32,
    pub vpgr_or_sgpr: u32,
}

#[repr(C)]
pub struct amdgpu_debugfs_gprwave_iocdata {
    pub gpr_or_wave: u32,
    pub se: u32,
    pub sh: u32,
    pub cu: u32,
    pub wave: u32,
    pub simd: u32,
    pub xcc_id: u32,
    pub gpr: amdgpu_debugfs_gprwave_iocdata_gpr,
}

/* MMIO debugfs state data (per file* handle) */
#[repr(C)]
pub struct amdgpu_debugfs_regs2_data {
    pub adev: *mut amdgpu_device,
    pub lock: mutex,
    pub id: amdgpu_debugfs_regs2_iocdata_v2,
}

#[repr(C)]
pub struct amdgpu_debugfs_gprwave_data {
    pub adev: *mut amdgpu_device,
    pub lock: mutex,
    pub id: amdgpu_debugfs_gprwave_iocdata,
}

#[repr(i32)]
pub enum AMDGPU_DEBUGFS_REGS2_CMDS {
    AMDGPU_DEBUGFS_REGS2_CMD_SET_STATE = 0,
    AMDGPU_DEBUGFS_REGS2_CMD_SET_STATE_V2,
}

#[repr(i32)]
pub enum AMDGPU_DEBUGFS_GPRWAVE_CMDS {
    AMDGPU_DEBUGFS_GPRWAVE_CMD_SET_STATE = 0,
}

// reg2 interface
// _IOWR(0x20, AMDGPU_DEBUGFS_REGS2_CMD_SET_STATE, struct amdgpu_debugfs_regs2_iocdata)
pub const AMDGPU_DEBUGFS_REGS2_IOC_SET_STATE: _ = _IOWR(0x20, AMDGPU_DEBUGFS_REGS2_CMD_SET_STATE as _, amdgpu_debugfs_regs2_iocdata);
// _IOWR(0x20, AMDGPU_DEBUGFS_REGS2_CMD_SET_STATE_V2, struct amdgpu_debugfs_regs2_iocdata_v2)
pub const AMDGPU_DEBUGFS_REGS2_IOC_SET_STATE_V2: _ = _IOWR(0x20, AMDGPU_DEBUGFS_REGS2_CMD_SET_STATE_V2 as _, amdgpu_debugfs_regs2_iocdata_v2);

// gprwave interface
// _IOWR(0x20, AMDGPU_DEBUGFS_GPRWAVE_CMD_SET_STATE, struct amdgpu_debugfs_gprwave_iocdata)
pub const AMDGPU_DEBUGFS_GPRWAVE_IOC_SET_STATE: _ = _IOWR(0x20, AMDGPU_DEBUGFS_GPRWAVE_CMD_SET_STATE as _, amdgpu_debugfs_gprwave_iocdata);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
