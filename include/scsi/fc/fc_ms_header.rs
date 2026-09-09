/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2011 Intel Corporation. All rights reserved.
 *
 * Maintained at www.Open-FCoE.org
 */

/* Fibre Channel Services - Management Service (MS) */

/* Common-transport sub-type for FDMI. */
pub const FC_FDMI_SUBTYPE: u32 = 0x10;

/* Management server FDMI specifications. */
pub const FDMI_V1: u32 = 1;
pub const FDMI_V2: u32 = 2;

/* Management server FDMI Requests. */
#[repr(i32)]
pub enum FcFdmiReq {
    FC_FDMI_GRHL = 0x0100,
    FC_FDMI_GHAT = 0x0101,
    FC_FDMI_GRPL = 0x0102,
    FC_FDMI_GPAT = 0x0110,
    FC_FDMI_RHBA = 0x0200,
    FC_FDMI_RHAT = 0x0201,
    FC_FDMI_RPRT = 0x0210,
    FC_FDMI_RPA = 0x0211,
    FC_FDMI_DHBA = 0x0300,
    FC_FDMI_DHAT = 0x0301,
    FC_FDMI_DPRT = 0x0310,
    FC_FDMI_DPA = 0x0311,
}

/* HBA Attribute Entry Type. */
#[repr(i32)]
pub enum FcFdmiHbaAttrType {
    FC_FDMI_HBA_ATTR_NODENAME = 0x0001,
    FC_FDMI_HBA_ATTR_MANUFACTURER = 0x0002,
    FC_FDMI_HBA_ATTR_SERIALNUMBER = 0x0003,
    FC_FDMI_HBA_ATTR_MODEL = 0x0004,
    FC_FDMI_HBA_ATTR_MODELDESCRIPTION = 0x0005,
    FC_FDMI_HBA_ATTR_HARDWAREVERSION = 0x0006,
    FC_FDMI_HBA_ATTR_DRIVERVERSION = 0x0007,
    FC_FDMI_HBA_ATTR_OPTIONROMVERSION = 0x0008,
    FC_FDMI_HBA_ATTR_FIRMWAREVERSION = 0x0009,
    FC_FDMI_HBA_ATTR_OSNAMEVERSION = 0x000A,
    FC_FDMI_HBA_ATTR_MAXCTPAYLOAD = 0x000B,
    FC_FDMI_HBA_ATTR_NODESYMBLNAME = 0x000C,
    FC_FDMI_HBA_ATTR_VENDORSPECIFICINFO = 0x000D,
    FC_FDMI_HBA_ATTR_NUMBEROFPORTS = 0x000E,
    FC_FDMI_HBA_ATTR_FABRICNAME = 0x000F,
    FC_FDMI_HBA_ATTR_BIOSVERSION = 0x0010,
    FC_FDMI_HBA_ATTR_BIOSSTATE = 0x0011,
    FC_FDMI_HBA_ATTR_VENDORIDENTIFIER = 0x00E0,
}

pub const FC_FDMI_HBA_ATTR_NODENAME_LEN: usize = 8;
pub const FC_FDMI_HBA_ATTR_MANUFACTURER_LEN: usize = 64;
pub const FC_FDMI_HBA_ATTR_SERIALNUMBER_LEN: usize = 64;
pub const FC_FDMI_HBA_ATTR_MODEL_LEN: usize = 64;
pub const FC_FDMI_HBA_ATTR_MODELDESCR_LEN: usize = 64;
pub const FC_FDMI_HBA_ATTR_HARDWAREVERSION_LEN: usize = 64;
pub const FC_FDMI_HBA_ATTR_DRIVERVERSION_LEN: usize = 64;
pub const FC_FDMI_HBA_ATTR_OPTIONROMVERSION_LEN: usize = 64;
pub const FC_FDMI_HBA_ATTR_FIRMWAREVERSION_LEN: usize = 64;
pub const FC_FDMI_HBA_ATTR_OSNAMEVERSION_LEN: usize = 128;
pub const FC_FDMI_HBA_ATTR_MAXCTPAYLOAD_LEN: usize = 4;
pub const FC_FDMI_HBA_ATTR_NODESYMBLNAME_LEN: usize = 64;
pub const FC_FDMI_HBA_ATTR_VENDORSPECIFICINFO_LEN: usize = 4;
pub const FC_FDMI_HBA_ATTR_NUMBEROFPORTS_LEN: usize = 4;
pub const FC_FDMI_HBA_ATTR_FABRICNAME_LEN: usize = 8;
pub const FC_FDMI_HBA_ATTR_BIOSVERSION_LEN: usize = 64;
pub const FC_FDMI_HBA_ATTR_BIOSSTATE_LEN: usize = 4;
pub const FC_FDMI_HBA_ATTR_VENDORIDENTIFIER_LEN: usize = 8;

/* Port Attribute Type. */
#[repr(i32)]
pub enum FcFdmiPortAttrType {
    FC_FDMI_PORT_ATTR_FC4TYPES = 0x0001,
    FC_FDMI_PORT_ATTR_SUPPORTEDSPEED = 0x0002,
    FC_FDMI_PORT_ATTR_CURRENTPORTSPEED = 0x0003,
    FC_FDMI_PORT_ATTR_MAXFRAMESIZE = 0x0004,
    FC_FDMI_PORT_ATTR_OSDEVICENAME = 0x0005,
    FC_FDMI_PORT_ATTR_HOSTNAME = 0x0006,
    FC_FDMI_PORT_ATTR_NODENAME = 0x0007,
    FC_FDMI_PORT_ATTR_PORTNAME = 0x0008,
    FC_FDMI_PORT_ATTR_SYMBOLICNAME = 0x0009,
    FC_FDMI_PORT_ATTR_PORTTYPE = 0x000A,
    FC_FDMI_PORT_ATTR_SUPPORTEDCLASSSRVC = 0x000B,
    FC_FDMI_PORT_ATTR_FABRICNAME = 0x000C,
    FC_FDMI_PORT_ATTR_CURRENTFC4TYPE = 0x000D,
    FC_FDMI_PORT_ATTR_PORTSTATE = 0x101,
    FC_FDMI_PORT_ATTR_DISCOVEREDPORTS = 0x102,
    FC_FDMI_PORT_ATTR_PORTID = 0x103,
}

pub const FC_FDMI_PORT_ATTR_FC4TYPES_LEN: usize = 32;
pub const FC_FDMI_PORT_ATTR_SUPPORTEDSPEED_LEN: usize = 4;
pub const FC_FDMI_PORT_ATTR_CURRENTPORTSPEED_LEN: usize = 4;
pub const FC_FDMI_PORT_ATTR_MAXFRAMESIZE_LEN: usize = 4;
pub const FC_FDMI_PORT_ATTR_OSDEVICENAME_LEN: usize = 256;
pub const FC_FDMI_PORT_ATTR_HOSTNAME_LEN: usize = 256;
pub const FC_FDMI_PORT_ATTR_NODENAME_LEN: usize = 8;
pub const FC_FDMI_PORT_ATTR_PORTNAME_LEN: usize = 8;
pub const FC_FDMI_PORT_ATTR_SYMBOLICNAME_LEN: usize = 256;
pub const FC_FDMI_PORT_ATTR_PORTTYPE_LEN: usize = 4;
pub const FC_FDMI_PORT_ATTR_SUPPORTEDCLASSSRVC_LEN: usize = 4;
pub const FC_FDMI_PORT_ATTR_FABRICNAME_LEN: usize = 8;
pub const FC_FDMI_PORT_ATTR_CURRENTFC4TYPE_LEN: usize = 32;
pub const FC_FDMI_PORT_ATTR_PORTSTATE_LEN: usize = 4;
pub const FC_FDMI_PORT_ATTR_DISCOVEREDPORTS_LEN: usize = 4;
pub const FC_FDMI_PORT_ATTR_PORTID_LEN: usize = 4;

#[repr(C)]
pub struct FcFdmiHbaIdentifier { pub id: __be64 }

#[repr(C)]
pub struct FcFdmiPortName { pub portname: __be64 }

#[repr(C, packed)]
pub struct FcFdmiAttrEntry {
    pub type_: __be16,
    pub len: __be16,
    pub value: [__u8; 0],
}

#[repr(C, packed)]
pub struct FsFdmiAttrs {
    pub numattrs: __be32,
    pub attr: [FcFdmiAttrEntry; 0],
}

#[repr(C, packed)]
pub struct FcFdmiRpl {
    pub numport: __be32,
    pub port: [FcFdmiPortName; 1],
}

#[repr(C, packed)]
pub struct FcFdmiRhba {
    pub hbaid: FcFdmiHbaIdentifier,
    pub port: FcFdmiRpl,
    pub hba_attrs: FsFdmiAttrs,
}

#[repr(C, packed)]
pub struct FcFdmiRhat {
    pub hbaid: FcFdmiHbaIdentifier,
    pub hba_attrs: FsFdmiAttrs,
}

#[repr(C, packed)]
pub struct FcFdmiRprt {
    pub hbaid: FcFdmiHbaIdentifier,
    pub port: FcFdmiPortName,
    pub hba_attrs: FsFdmiAttrs,
}

#[repr(C, packed)]
pub struct FcFdmiRpa {
    pub port: FcFdmiPortName,
    pub hba_attrs: FsFdmiAttrs,
}

#[repr(C, packed)]
pub struct FcFdmiDprt { pub port: FcFdmiPortName }

#[repr(C, packed)]
pub struct FcFdmiDpa {
    pub port: FcFdmiPortName,
    pub hba_attrs: FsFdmiAttrs,
}

#[repr(C, packed)]
pub struct FcFdmiDhat { pub hbaid: FcFdmiHbaIdentifier }

#[repr(C, packed)]
pub struct FcFdmiDhba { pub hbaid: FcFdmiHbaIdentifier }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
