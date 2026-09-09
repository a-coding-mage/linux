/* SPDX-License-Identifier: GPL-2.0-or-later */
/* AFS Volume Location Service client interface
 *
 * Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependency supplied by afs.h in the original header. */

pub const AFS_VL_PORT: u32 = 7003; /* volume location service port */
pub const VL_SERVICE: u32 = 52; /* RxRPC service ID for the Volume Location service */
pub const YFS_VL_SERVICE: u32 = 2503; /* Service ID for AuriStor upgraded VL service */
pub const YFS_VL_MAXCELLNAME: u32 = 256; /* Maximum length of a cell name in YFS protocol */

#[repr(i32)]
pub enum AFSVL_Operations {
    VLGETENTRYBYID = 503, /* AFS Get VLDB entry by ID */
    VLGETENTRYBYNAME = 504, /* AFS Get VLDB entry by name */
    VLPROBE = 514, /* AFS probe VL service */
    VLGETENTRYBYIDU = 526, /* AFS Get VLDB entry by ID (UUID-variant) */
    VLGETENTRYBYNAMEU = 527, /* AFS Get VLDB entry by name (UUID-variant) */
    VLGETADDRSU = 533, /* AFS Get addrs for fileserver */
    YVLGETENDPOINTS = 64002, /* YFS Get endpoints for file/volume server */
    YVLGETCELLNAME = 64014, /* YFS Get actual cell name */
    VLGETCAPABILITIES = 65537, /* AFS Get server capabilities */
}

#[repr(i32)]
pub enum AFSVL_Errors {
    AFSVL_IDEXIST = 363520, /* Volume Id entry exists in vl database */
    AFSVL_IO = 363521, /* I/O related error */
    AFSVL_NAMEEXIST = 363522, /* Volume name entry exists in vl database */
    AFSVL_CREATEFAIL = 363523, /* Internal creation failure */
    AFSVL_NOENT = 363524, /* No such entry */
    AFSVL_EMPTY = 363525, /* Vl database is empty */
    AFSVL_ENTDELETED = 363526, /* Entry is deleted (soft delete) */
    AFSVL_BADNAME = 363527, /* Volume name is illegal */
    AFSVL_BADINDEX = 363528, /* Index is out of range */
    AFSVL_BADVOLTYPE = 363529, /* Bad volume type */
    AFSVL_BADSERVER = 363530, /* Illegal server number (out of range) */
    AFSVL_BADPARTITION = 363531, /* Bad partition number */
    AFSVL_REPSFULL = 363532, /* Run out of space for Replication sites */
    AFSVL_NOREPSERVER = 363533, /* No such Replication server site exists */
    AFSVL_DUPREPSERVER = 363534, /* Replication site already exists */
    AFSVL_RWNOTFOUND = 363535, /* Parent R/W entry not found */
    AFSVL_BADREFCOUNT = 363536, /* Illegal Reference Count number */
    AFSVL_SIZEEXCEEDED = 363537, /* Vl size for attributes exceeded */
    AFSVL_BADENTRY = 363538, /* Bad incoming vl entry */
    AFSVL_BADVOLIDBUMP = 363539, /* Illegal max volid increment */
    AFSVL_IDALREADYHASHED = 363540, /* RO/BACK id already hashed */
    AFSVL_ENTRYLOCKED = 363541, /* Vl entry is already locked */
    AFSVL_BADVOLOPER = 363542, /* Bad volume operation code */
    AFSVL_BADRELLOCKTYPE = 363543, /* Bad release lock type */
    AFSVL_RERELEASE = 363544, /* Status report: last release was aborted */
    AFSVL_BADSERVERFLAG = 363545, /* Invalid replication site server flag */
    AFSVL_PERM = 363546, /* No permission access */
    AFSVL_NOMEM = 363547, /* malloc/realloc failed to alloc enough memory */
}

pub const YFS_SERVER_INDEX: u32 = 0;
pub const YFS_SERVER_UUID: u32 = 1;
pub const YFS_SERVER_ENDPOINT: u32 = 2;
pub const YFS_ENDPOINT_IPV4: u32 = 0;
pub const YFS_ENDPOINT_IPV6: u32 = 1;
pub const YFS_MAXENDPOINTS: usize = 16;

pub const AFS_VLF_RWEXISTS: u32 = 0x1000;
pub const AFS_VLF_ROEXISTS: u32 = 0x2000;
pub const AFS_VLF_BACKEXISTS: u32 = 0x4000;
pub const AFS_VLSF_NEWREPSITE: u32 = 0x0001;
pub const AFS_VLSF_ROVOL: u32 = 0x0002;
pub const AFS_VLSF_RWVOL: u32 = 0x0004;
pub const AFS_VLSF_BACKVOL: u32 = 0x0008;
pub const AFS_VLSF_UUID: u32 = 0x0010;
pub const AFS_VLSF_DONTUSE: u32 = 0x0020;
pub const AFS_VLDB_MAXNAMELEN: usize = 65;

#[repr(C)]
pub struct afs_vldbentry_server {
    pub addr: in_addr,
    pub partition: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct afs_vldbentry {
    pub name: [::std::os::raw::c_char; 65],
    pub type_: afs_voltype_t,
    pub num_servers: u32,
    pub clone_id: u32,
    pub flags: u32,
    pub volume_ids: [afs_volid_t; 3],
    pub servers: [afs_vldbentry_server; 8],
}

#[repr(C)]
pub struct afs_ListAddrByAttributes__xdr {
    pub Mask: __be32,
    pub ipaddr: __be32,
    pub index: __be32,
    pub spare: __be32,
    pub uuid: afs_uuid__xdr,
}

#[repr(C)]
pub struct afs_uvldbentry__xdr {
    pub name: [__be32; AFS_VLDB_MAXNAMELEN],
    pub nServers: __be32,
    pub serverNumber: [afs_uuid__xdr; AFS_NMAXNSERVERS],
    pub serverUnique: [__be32; AFS_NMAXNSERVERS],
    pub serverPartition: [__be32; AFS_NMAXNSERVERS],
    pub serverFlags: [__be32; AFS_NMAXNSERVERS],
    pub volumeId: [__be32; AFS_MAXTYPES],
    pub cloneId: __be32,
    pub flags: __be32,
    pub spares1: __be32,
    pub spares2: __be32,
    pub spares3: __be32,
    pub spares4: __be32,
    pub spares5: __be32,
    pub spares6: __be32,
    pub spares7: __be32,
    pub spares8: __be32,
    pub spares9: __be32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
