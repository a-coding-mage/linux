/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Userspace interface to the pkey device driver. */

// The original header includes linux/ioctl.h and linux/types.h.

pub const PKEY_IOCTL_MAGIC: u8 = b'p';

pub const SECKEYBLOBSIZE: usize = 64;
pub const PROTKEYBLOBSIZE: usize = 80;
pub const MAXPROTKEYSIZE: usize = 64;
pub const MAXCLRKEYSIZE: usize = 32;
pub const MAXAESCIPHERKEYSIZE: usize = 136;
pub const MINEP11AESKEYBLOBSIZE: usize = 256;
pub const MAXEP11AESKEYBLOBSIZE: usize = 336;
pub const MINKEYBLOBSIZE: usize = SECKEYBLOBSIZE;

pub const PKEY_KEYTYPE_AES_128: u32 = 1;
pub const PKEY_KEYTYPE_AES_192: u32 = 2;
pub const PKEY_KEYTYPE_AES_256: u32 = 3;
pub const PKEY_KEYTYPE_ECC: u32 = 4;
pub const PKEY_KEYTYPE_ECC_P256: u32 = 5;
pub const PKEY_KEYTYPE_ECC_P384: u32 = 6;
pub const PKEY_KEYTYPE_ECC_P521: u32 = 7;
pub const PKEY_KEYTYPE_ECC_ED25519: u32 = 8;
pub const PKEY_KEYTYPE_ECC_ED448: u32 = 9;
pub const PKEY_KEYTYPE_AES_XTS_128: u32 = 10;
pub const PKEY_KEYTYPE_AES_XTS_256: u32 = 11;
pub const PKEY_KEYTYPE_HMAC_512: u32 = 12;
pub const PKEY_KEYTYPE_HMAC_1024: u32 = 13;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum pkey_key_type {
    PKEY_TYPE_CCA_DATA = 1,
    PKEY_TYPE_CCA_CIPHER = 2,
    PKEY_TYPE_EP11 = 3,
    PKEY_TYPE_CCA_ECC = 0x1f,
    PKEY_TYPE_EP11_AES = 6,
    PKEY_TYPE_EP11_ECC = 7,
    PKEY_TYPE_PROTKEY = 8,
    PKEY_TYPE_UVSECRET = 9,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum pkey_key_size {
    PKEY_SIZE_AES_128 = 128,
    PKEY_SIZE_AES_192 = 192,
    PKEY_SIZE_AES_256 = 256,
    PKEY_SIZE_UNKNOWN = 0xffff_ffff,
}

pub const PKEY_FLAGS_MATCH_CUR_MKVP: u32 = 0x0000_0002;
pub const PKEY_FLAGS_MATCH_ALT_MKVP: u32 = 0x0000_0004;
pub const PKEY_KEYGEN_XPRT_SYM: u32 = 0x0000_8000;
pub const PKEY_KEYGEN_XPRT_UASY: u32 = 0x0000_4000;
pub const PKEY_KEYGEN_XPRT_AASY: u32 = 0x0000_2000;
pub const PKEY_KEYGEN_XPRT_RAW: u32 = 0x0000_1000;
pub const PKEY_KEYGEN_XPRT_CPAC: u32 = 0x0000_0800;
pub const PKEY_KEYGEN_XPRT_DES: u32 = 0x0000_0080;
pub const PKEY_KEYGEN_XPRT_AES: u32 = 0x0000_0040;
pub const PKEY_KEYGEN_XPRT_RSA: u32 = 0x0000_0008;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_apqn { pub card: u16, pub domain: u16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_seckey { pub seckey: [u8; SECKEYBLOBSIZE] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_protkey { pub type_: u32, pub len: u32, pub protkey: [u8; MAXPROTKEYSIZE] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_clrkey { pub clrkey: [u8; MAXCLRKEYSIZE] }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ep11kblob_header {
    pub type_: u8, pub hver: u8, pub len: u16, pub version: u8,
    pub res0: u8, pub bitlen: u16, pub res1: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_genseck { pub cardnr: u16, pub domain: u16, pub keytype: u32, pub seckey: pkey_seckey }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_clr2seck { pub cardnr: u16, pub domain: u16, pub keytype: u32, pub clrkey: pkey_clrkey, pub seckey: pkey_seckey }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_sec2protk { pub cardnr: u16, pub domain: u16, pub seckey: pkey_seckey, pub protkey: pkey_protkey }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_clr2protk { pub keytype: u32, pub clrkey: pkey_clrkey, pub protkey: pkey_protkey }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_findcard { pub seckey: pkey_seckey, pub cardnr: u16, pub domain: u16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_skey2pkey { pub seckey: pkey_seckey, pub protkey: pkey_protkey }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_verifykey { pub seckey: pkey_seckey, pub cardnr: u16, pub domain: u16, pub keysize: u16, pub attributes: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_genprotk { pub keytype: u32, pub protkey: pkey_protkey }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_verifyprotk { pub protkey: pkey_protkey }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_kblob2pkey { pub key: *mut u8, pub keylen: u32, pub protkey: pkey_protkey }

pub const PKEY_VERIFY_ATTR_AES: u32 = 0x0000_0001;
pub const PKEY_VERIFY_ATTR_OLD_MKVP: u32 = 0x0000_0100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_genseck2 { pub apqns: *mut pkey_apqn, pub apqn_entries: u32, pub type_: pkey_key_type, pub size: pkey_key_size, pub keygenflags: u32, pub key: *mut u8, pub keylen: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_clr2seck2 { pub apqns: *mut pkey_apqn, pub apqn_entries: u32, pub type_: pkey_key_type, pub size: pkey_key_size, pub keygenflags: u32, pub clrkey: pkey_clrkey, pub key: *mut u8, pub keylen: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_verifykey2 { pub key: *mut u8, pub keylen: u32, pub cardnr: u16, pub domain: u16, pub type_: pkey_key_type, pub size: pkey_key_size, pub flags: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_kblob2pkey2 { pub key: *mut u8, pub keylen: u32, pub apqns: *mut pkey_apqn, pub apqn_entries: u32, pub protkey: pkey_protkey }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_apqns4key { pub key: *mut u8, pub keylen: u32, pub flags: u32, pub apqns: *mut pkey_apqn, pub apqn_entries: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_apqns4keytype { pub type_: pkey_key_type, pub cur_mkvp: [u8; 32], pub alt_mkvp: [u8; 32], pub flags: u32, pub apqns: *mut pkey_apqn, pub apqn_entries: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_kblob2pkey3 { pub key: *mut u8, pub keylen: u32, pub apqns: *mut pkey_apqn, pub apqn_entries: u32, pub pkeytype: u32, pub pkeylen: u32, pub pkey: *mut u8 }

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const fn ioc(dir: u32, ty: u8, nr: u8, size: usize) -> u32 { (dir << IOC_DIRSHIFT) | ((size as u32) << IOC_SIZESHIFT) | ((ty as u32) << IOC_TYPESHIFT) | (nr as u32) }
const fn iowr<T>(nr: u8) -> u32 { ioc(IOC_READ | IOC_WRITE, PKEY_IOCTL_MAGIC, nr, core::mem::size_of::<T>()) }
const fn iow<T>(nr: u8) -> u32 { ioc(IOC_WRITE, PKEY_IOCTL_MAGIC, nr, core::mem::size_of::<T>()) }

pub const PKEY_GENSECK: u32 = iowr::<pkey_genseck>(0x01);
pub const PKEY_CLR2SECK: u32 = iowr::<pkey_clr2seck>(0x02);
pub const PKEY_SEC2PROTK: u32 = iowr::<pkey_sec2protk>(0x03);
pub const PKEY_CLR2PROTK: u32 = iowr::<pkey_clr2protk>(0x04);
pub const PKEY_FINDCARD: u32 = iowr::<pkey_findcard>(0x05);
pub const PKEY_SKEY2PKEY: u32 = iowr::<pkey_skey2pkey>(0x06);
pub const PKEY_VERIFYKEY: u32 = iowr::<pkey_verifykey>(0x07);
pub const PKEY_GENPROTK: u32 = iowr::<pkey_genprotk>(0x08);
pub const PKEY_VERIFYPROTK: u32 = iow::<pkey_verifyprotk>(0x09);
pub const PKEY_KBLOB2PROTK: u32 = iowr::<pkey_kblob2pkey>(0x0A);
pub const PKEY_GENSECK2: u32 = iowr::<pkey_genseck2>(0x11);
pub const PKEY_CLR2SECK2: u32 = iowr::<pkey_clr2seck2>(0x12);
pub const PKEY_VERIFYKEY2: u32 = iowr::<pkey_verifykey2>(0x17);
pub const PKEY_KBLOB2PROTK2: u32 = iowr::<pkey_kblob2pkey2>(0x1A);
pub const PKEY_APQNS4K: u32 = iowr::<pkey_apqns4key>(0x1B);
pub const PKEY_APQNS4KT: u32 = iowr::<pkey_apqns4keytype>(0x1C);
pub const PKEY_KBLOB2PROTK3: u32 = iowr::<pkey_kblob2pkey3>(0x1D);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
