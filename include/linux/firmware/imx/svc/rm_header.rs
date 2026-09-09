/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017-2020 NXP
 *
 * Header file containing the public API for the System Controller (SC)
 * Resource Management (RM) function. This includes functions for
 * partitioning resources, pads, and memory regions.
 *
 * RM_SVC (SVC) Resource Management Service
 *
 * Module for the Resource Management (RM) service.
 */

/* Dependency: linux/firmware/imx/sci.h */

/*
 * This type is used to indicate RPC RM function calls.
 */
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImxScRmFunc {
    ImxScRmFuncUnknown = 0,
    ImxScRmFuncPartitionAlloc = 1,
    ImxScRmFuncSetConfidential = 31,
    ImxScRmFuncPartitionFree = 2,
    ImxScRmFuncGetDid = 26,
    ImxScRmFuncPartitionStatic = 3,
    ImxScRmFuncPartitionLock = 4,
    ImxScRmFuncGetPartition = 5,
    ImxScRmFuncSetParent = 6,
    ImxScRmFuncMoveAll = 7,
    ImxScRmFuncAssignResource = 8,
    ImxScRmFuncSetResourceMovable = 9,
    ImxScRmFuncSetSubsysRsrcMovable = 28,
    ImxScRmFuncSetMasterAttributes = 10,
    ImxScRmFuncSetMasterSid = 11,
    ImxScRmFuncSetPeripheralPermissions = 12,
    ImxScRmFuncIsResourceOwned = 13,
    ImxScRmFuncGetResourceOwner = 33,
    ImxScRmFuncIsResourceMaster = 14,
    ImxScRmFuncIsResourcePeripheral = 15,
    ImxScRmFuncGetResourceInfo = 16,
    ImxScRmFuncMemregAlloc = 17,
    ImxScRmFuncMemregSplit = 29,
    ImxScRmFuncMemregFrag = 32,
    ImxScRmFuncMemregFree = 18,
    ImxScRmFuncFindMemreg = 30,
    ImxScRmFuncAssignMemreg = 19,
    ImxScRmFuncSetMemregPermissions = 20,
    ImxScRmFuncIsMemregOwned = 21,
    ImxScRmFuncGetMemregInfo = 22,
    ImxScRmFuncAssignPad = 23,
    ImxScRmFuncSetPadMovable = 24,
    ImxScRmFuncIsPadOwned = 25,
    ImxScRmFuncDump = 27,
}

/* CONFIG_IMX_SCU is a build-time configuration condition. */
#[cfg(feature = "CONFIG_IMX_SCU")]
extern "C" {
    pub fn imx_sc_rm_is_resource_owned(
        ipc: *mut crate::imx_sc_ipc,
        resource: u16,
    ) -> bool;
    pub fn imx_sc_rm_get_resource_owner(
        ipc: *mut crate::imx_sc_ipc,
        resource: u16,
        pt: *mut u8,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub fn imx_sc_rm_is_resource_owned(_ipc: *mut crate::imx_sc_ipc, _resource: u16) -> bool {
    true
}

#[cfg(not(feature = "CONFIG_IMX_SCU"))]
#[inline]
pub fn imx_sc_rm_get_resource_owner(
    _ipc: *mut crate::imx_sc_ipc,
    _resource: u16,
    _pt: *mut u8,
) -> i32 {
    /* -EOPNOTSUPP */
    -95
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
