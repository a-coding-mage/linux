/* SPDX-License-Identifier: ((GPL-2.0+ WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Freescale hypervisor ioctl and kernel interface. */

#[repr(C)]
pub struct fsl_hv_ioctl_restart {
    pub ret: u32,
    pub partition: u32,
}

#[repr(C)]
pub struct fsl_hv_ioctl_status {
    pub ret: u32,
    pub partition: u32,
    pub status: u32,
}

#[repr(C)]
pub struct fsl_hv_ioctl_start {
    pub ret: u32,
    pub partition: u32,
    pub entry_point: u32,
    pub load: u32,
}

#[repr(C)]
pub struct fsl_hv_ioctl_stop {
    pub ret: u32,
    pub partition: u32,
}

#[repr(C)]
pub struct fsl_hv_ioctl_memcpy {
    pub ret: u32,
    pub source: u32,
    pub target: u32,
    pub reserved: u32,
    pub local_vaddr: u64,
    pub remote_paddr: u64,
    pub count: u64,
}

#[repr(C)]
pub struct fsl_hv_ioctl_doorbell {
    pub ret: u32,
    pub doorbell: u32,
}

#[repr(C)]
pub struct fsl_hv_ioctl_prop {
    pub ret: u32,
    pub handle: u32,
    pub path: u64,
    pub propname: u64,
    pub propval: u64,
    pub proplen: u32,
    pub reserved: u32,
}

/* The ioctl type, documented in ioctl-number.txt. */
pub const FSL_HV_IOCTL_TYPE: u32 = 0xAF;

/* These values use the Linux _IOWR(type, number, structure) encoding.
 * The _IOWR dependency is supplied by the target environment. */
#[allow(unused_macros)]
macro_rules! FSL_HV_IOCTL_PARTITION_RESTART {
    () => { _IOWR!(FSL_HV_IOCTL_TYPE, 1, fsl_hv_ioctl_restart) };
}
#[allow(unused_macros)]
macro_rules! FSL_HV_IOCTL_PARTITION_GET_STATUS {
    () => { _IOWR!(FSL_HV_IOCTL_TYPE, 2, fsl_hv_ioctl_status) };
}
#[allow(unused_macros)]
macro_rules! FSL_HV_IOCTL_PARTITION_START {
    () => { _IOWR!(FSL_HV_IOCTL_TYPE, 3, fsl_hv_ioctl_start) };
}
#[allow(unused_macros)]
macro_rules! FSL_HV_IOCTL_PARTITION_STOP {
    () => { _IOWR!(FSL_HV_IOCTL_TYPE, 4, fsl_hv_ioctl_stop) };
}
#[allow(unused_macros)]
macro_rules! FSL_HV_IOCTL_MEMCPY {
    () => { _IOWR!(FSL_HV_IOCTL_TYPE, 5, fsl_hv_ioctl_memcpy) };
}
#[allow(unused_macros)]
macro_rules! FSL_HV_IOCTL_DOORBELL {
    () => { _IOWR!(FSL_HV_IOCTL_TYPE, 6, fsl_hv_ioctl_doorbell) };
}
#[allow(unused_macros)]
macro_rules! FSL_HV_IOCTL_GETPROP {
    () => { _IOWR!(FSL_HV_IOCTL_TYPE, 7, fsl_hv_ioctl_prop) };
}
#[allow(unused_macros)]
macro_rules! FSL_HV_IOCTL_SETPROP {
    () => { _IOWR!(FSL_HV_IOCTL_TYPE, 8, fsl_hv_ioctl_prop) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
