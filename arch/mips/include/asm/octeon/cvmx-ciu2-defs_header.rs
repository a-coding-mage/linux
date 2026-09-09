/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2012 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, version 2, as
 * published by the Free Software Foundation.
 *
 * This file is distributed in the hope that it will be useful, but
 * AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
 * NONINFRINGEMENT.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this file; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
 * or visit http://www.gnu.org/licenses/.
 *
 * This file may also be available under a different license from Cavium.
 * Contact Cavium Networks for more information
 ***********************license end**************************************/

macro_rules! CVMX_CIU2_ACK_PPX_IP2 {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x00010701000C0000u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_ACK_PPX_IP3 {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x00010701000C0200u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_EN_PPX_IP2_RML {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x0001070100092000u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_EN_PPX_IP2_WDOG {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x0001070100091000u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_EN_PPX_IP2_WRKQ {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x0001070100090000u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_EN_PPX_IP2_WRKQ_W1C {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x00010701000B0000u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_EN_PPX_IP2_WRKQ_W1S {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x00010701000A0000u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_EN_PPX_IP3_MBOX_W1C {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x00010701000B8200u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_EN_PPX_IP3_MBOX_W1S {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x00010701000A8200u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_INTR_CIU_READY {
    () => { CVMX_ADD_IO_SEG(0x0001070100102008u64) };
}
macro_rules! CVMX_CIU2_RAW_PPX_IP2_WRKQ {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x0001070100040000u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_SRC_PPX_IP2_RML {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x0001070100082000u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_SRC_PPX_IP2_WDOG {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x0001070100081000u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_SRC_PPX_IP2_WRKQ {
    ($block_id:expr) => {
        CVMX_ADD_IO_SEG(0x0001070100080000u64) + (($block_id) & 31) * 0x200000u64
    };
}
macro_rules! CVMX_CIU2_SUM_PPX {
    ($offset:expr) => {
        CVMX_ADD_IO_SEG(0x0001070100000000u64) + (($offset) & 31) * 8u64
    };
}
macro_rules! CVMX_CIU2_SUM_PPX_IP3 {
    ($offset:expr) => {
        CVMX_ADD_IO_SEG(0x0001070100000200u64) + (($offset) & 31) * 8u64
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
