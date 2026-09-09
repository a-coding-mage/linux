/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Copyright (c) 2022/23 Siemens Mobility GmbH */

// C dependencies: linux/const.h, linux/if.h, linux/ioctl.h, linux/types.h

/*
 * flags definition for n_gsm
 *
 * Used by:
 * struct gsm_config_ext.flags
 * struct gsm_dlci_config.flags
 */
/* Forces a DLCI reset if set. Otherwise, a DLCI reset is only done if
 * incompatible settings were provided. Always cleared on retrieval.
 */
pub const GSM_FL_RESTART: u32 = _BITUL!(0);

/**
 * struct gsm_config - n_gsm basic configuration parameters
 *
 * This structure is used in combination with GSMIOC_GETCONF and GSMIOC_SETCONF
 * to retrieve and set the basic parameters of an n_gsm ldisc.
 * struct gsm_config_ext can be used to configure extended ldisc parameters.
 *
 * All timers are in units of 1/100th of a second.
 *
 * @adaption:      Convergence layer type
 * @encapsulation: Framing (0 = basic option, 1 = advanced option)
 * @initiator:     Initiator or responder
 * @t1:            Acknowledgment timer
 * @t2:            Response timer for multiplexer control channel
 * @t3:            Response timer for wake-up procedure
 * @n2:            Maximum number of retransmissions
 * @mru:           Maximum incoming frame payload size
 * @mtu:           Maximum outgoing frame payload size
 * @k:             Window size
 * @i:             Frame type (1 = UIH, 2 = UI)
 * @unused:        Can not be used
 */
#[repr(C)]
pub struct gsm_config {
    pub adaption: u32,
    pub encapsulation: u32,
    pub initiator: u32,
    pub t1: u32,
    pub t2: u32,
    pub t3: u32,
    pub n2: u32,
    pub mru: u32,
    pub mtu: u32,
    pub k: u32,
    pub i: u32,
    pub unused: [u32; 8],
}

pub const GSMIOC_GETCONF: _IOC_TYPE = _IOR!('G', 0, gsm_config);
pub const GSMIOC_SETCONF: _IOC_TYPE = _IOW!('G', 1, gsm_config);

/** struct gsm_netconfig - n_gsm network configuration parameters */
#[repr(C)]
pub struct gsm_netconfig {
    pub adaption: u32,
    pub protocol: u16,
    pub unused2: u16,
    pub if_name: [core::ffi::c_char; IFNAMSIZ],
    pub unused: [u8; 28],
}

pub const GSMIOC_ENABLE_NET: _IOC_TYPE = _IOW!('G', 2, gsm_netconfig);
pub const GSMIOC_DISABLE_NET: _IOC_TYPE = _IO!('G', 3);

/* get the base tty number for a configured gsmmux tty */
pub const GSMIOC_GETFIRST: _IOC_TYPE = _IOR!('G', 4, u32);

/** struct gsm_config_ext - n_gsm extended configuration parameters */
#[repr(C)]
pub struct gsm_config_ext {
    pub keep_alive: u32,
    pub wait_config: u32,
    pub flags: u32,
    pub reserved: [u32; 5],
}

pub const GSMIOC_GETCONF_EXT: _IOC_TYPE = _IOR!('G', 5, gsm_config_ext);
pub const GSMIOC_SETCONF_EXT: _IOC_TYPE = _IOW!('G', 6, gsm_config_ext);

/** struct gsm_dlci_config - n_gsm channel configuration parameters */
#[repr(C)]
pub struct gsm_dlci_config {
    pub channel: u32,
    pub adaption: u32,
    pub mtu: u32,
    pub priority: u32,
    pub i: u32,
    pub k: u32,
    pub flags: u32,
    pub reserved: [u32; 7],
}

pub const GSMIOC_GETCONF_DLCI: _IOC_TYPE = _IOWR!('G', 7, gsm_dlci_config);
pub const GSMIOC_SETCONF_DLCI: _IOC_TYPE = _IOW!('G', 8, gsm_dlci_config);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
