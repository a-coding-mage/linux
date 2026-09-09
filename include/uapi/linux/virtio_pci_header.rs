/*
 * Virtio PCI driver
 *
 * This module allows virtio devices to be used over a virtual PCI device.
 * This can be used with QEMU based VMMs like KVM or Xen.
 *
 * Copyright IBM Corp. 2007
 *
 * Authors:
 *  Anthony Liguori  <aliguori@us.ibm.com>
 *
 * This header is BSD licensed so anyone can use the definitions to implement
 * compatible drivers/servers.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of IBM nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

// The C header includes linux/types.h and linux/const.h. Their Rust names are
// supplied by the surrounding translation.

// The following legacy definitions are omitted when VIRTIO_PCI_NO_LEGACY is set.
pub const VIRTIO_PCI_HOST_FEATURES: usize = 0;
pub const VIRTIO_PCI_GUEST_FEATURES: usize = 4;
pub const VIRTIO_PCI_QUEUE_PFN: usize = 8;
pub const VIRTIO_PCI_QUEUE_NUM: usize = 12;
pub const VIRTIO_PCI_QUEUE_SEL: usize = 14;
pub const VIRTIO_PCI_QUEUE_NOTIFY: usize = 16;
pub const VIRTIO_PCI_STATUS: usize = 18;
pub const VIRTIO_PCI_ISR: usize = 19;
pub const VIRTIO_MSI_CONFIG_VECTOR: usize = 20;
pub const VIRTIO_MSI_QUEUE_VECTOR: usize = 22;

pub const fn VIRTIO_PCI_CONFIG_OFF(msix_enabled: bool) -> usize {
    if msix_enabled { 24 } else { 20 }
}

// Deprecated: use VIRTIO_PCI_CONFIG_OFF instead. The C macro takes dev->msix_enabled.
pub const VIRTIO_PCI_ABI_VERSION: u32 = 0;
pub const VIRTIO_PCI_QUEUE_ADDR_SHIFT: u32 = 12;
pub const VIRTIO_PCI_VRING_ALIGN: u32 = 4096;

pub const VIRTIO_PCI_ISR_CONFIG: u32 = 0x2;
pub const VIRTIO_MSI_NO_VECTOR: u16 = 0xffff;

// The following modern definitions are omitted when VIRTIO_PCI_NO_MODERN is set.
pub const VIRTIO_PCI_CAP_COMMON_CFG: u8 = 1;
pub const VIRTIO_PCI_CAP_NOTIFY_CFG: u8 = 2;
pub const VIRTIO_PCI_CAP_ISR_CFG: u8 = 3;
pub const VIRTIO_PCI_CAP_DEVICE_CFG: u8 = 4;
pub const VIRTIO_PCI_CAP_PCI_CFG: u8 = 5;
pub const VIRTIO_PCI_CAP_SHARED_MEMORY_CFG: u8 = 8;
pub const VIRTIO_PCI_CAP_VENDOR_CFG: u8 = 9;

#[repr(C)]
pub struct virtio_pci_cap {
    pub cap_vndr: __u8,
    pub cap_next: __u8,
    pub cap_len: __u8,
    pub cfg_type: __u8,
    pub bar: __u8,
    pub id: __u8,
    pub padding: [__u8; 2],
    pub offset: __le32,
    pub length: __le32,
}

#[repr(C)]
pub struct virtio_pci_vndr_data {
    pub cap_vndr: __u8,
    pub cap_next: __u8,
    pub cap_len: __u8,
    pub cfg_type: __u8,
    pub vendor_id: __u16,
    // Vendor definition follows; the structure is padded to a multiple of 4 bytes.
    // Reads must not have side effects.
}

#[repr(C)]
pub struct virtio_pci_cap64 {
    pub cap: virtio_pci_cap,
    pub offset_hi: __le32,
    pub length_hi: __le32,
}

#[repr(C)]
pub struct virtio_pci_notify_cap {
    pub cap: virtio_pci_cap,
    pub notify_off_multiplier: __le32,
}

#[repr(C)]
pub struct virtio_pci_common_cfg {
    pub device_feature_select: __le32,
    pub device_feature: __le32,
    pub guest_feature_select: __le32,
    pub guest_feature: __le32,
    pub msix_config: __le16,
    pub num_queues: __le16,
    pub device_status: __u8,
    pub config_generation: __u8,
    pub queue_select: __le16,
    pub queue_size: __le16,
    pub queue_msix_vector: __le16,
    pub queue_enable: __le16,
    pub queue_notify_off: __le16,
    pub queue_desc_lo: __le32,
    pub queue_desc_hi: __le32,
    pub queue_avail_lo: __le32,
    pub queue_avail_hi: __le32,
    pub queue_used_lo: __le32,
    pub queue_used_hi: __le32,
}

// Do not use sizeof on this; use offsetofend for specific fields.
#[repr(C)]
pub struct virtio_pci_modern_common_cfg {
    pub cfg: virtio_pci_common_cfg,
    pub queue_notify_data: __le16,
    pub queue_reset: __le16,
    pub admin_queue_index: __le16,
    pub admin_queue_num: __le16,
}

#[repr(C)]
pub struct virtio_pci_cfg_cap {
    pub cap: virtio_pci_cap,
    pub pci_cfg_data: [__u8; 4],
}

pub const VIRTIO_PCI_CAP_VNDR: usize = 0;
pub const VIRTIO_PCI_CAP_NEXT: usize = 1;
pub const VIRTIO_PCI_CAP_LEN: usize = 2;
pub const VIRTIO_PCI_CAP_CFG_TYPE: usize = 3;
pub const VIRTIO_PCI_CAP_BAR: usize = 4;
pub const VIRTIO_PCI_CAP_OFFSET: usize = 8;
pub const VIRTIO_PCI_CAP_LENGTH: usize = 12;
pub const VIRTIO_PCI_NOTIFY_CAP_MULT: usize = 16;
pub const VIRTIO_PCI_COMMON_DFSELECT: usize = 0;
pub const VIRTIO_PCI_COMMON_DF: usize = 4;
pub const VIRTIO_PCI_COMMON_GFSELECT: usize = 8;
pub const VIRTIO_PCI_COMMON_GF: usize = 12;
pub const VIRTIO_PCI_COMMON_MSIX: usize = 16;
pub const VIRTIO_PCI_COMMON_NUMQ: usize = 18;
pub const VIRTIO_PCI_COMMON_STATUS: usize = 20;
pub const VIRTIO_PCI_COMMON_CFGGENERATION: usize = 21;
pub const VIRTIO_PCI_COMMON_Q_SELECT: usize = 22;
pub const VIRTIO_PCI_COMMON_Q_SIZE: usize = 24;
pub const VIRTIO_PCI_COMMON_Q_MSIX: usize = 26;
pub const VIRTIO_PCI_COMMON_Q_ENABLE: usize = 28;
pub const VIRTIO_PCI_COMMON_Q_NOFF: usize = 30;
pub const VIRTIO_PCI_COMMON_Q_DESCLO: usize = 32;
pub const VIRTIO_PCI_COMMON_Q_DESCHI: usize = 36;
pub const VIRTIO_PCI_COMMON_Q_AVAILLO: usize = 40;
pub const VIRTIO_PCI_COMMON_Q_AVAILHI: usize = 44;
pub const VIRTIO_PCI_COMMON_Q_USEDLO: usize = 48;
pub const VIRTIO_PCI_COMMON_Q_USEDHI: usize = 52;
pub const VIRTIO_PCI_COMMON_Q_NDATA: usize = 56;
pub const VIRTIO_PCI_COMMON_Q_RESET: usize = 58;
pub const VIRTIO_PCI_COMMON_ADM_Q_IDX: usize = 60;
pub const VIRTIO_PCI_COMMON_ADM_Q_NUM: usize = 62;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
