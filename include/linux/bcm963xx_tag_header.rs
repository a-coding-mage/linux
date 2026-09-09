/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding translation unit: Linux __u32.

pub const TAGVER_LEN: usize = 4; // Length of Tag Version
pub const TAGLAYOUT_LEN: usize = 4; // Length of FlashLayoutVer
pub const SIG1_LEN: usize = 20; // Company Signature 1 Length
pub const SIG2_LEN: usize = 14; // Company Signature 2 Length
pub const BOARDID_LEN: usize = 16; // Length of BoardId
pub const ENDIANFLAG_LEN: usize = 2; // Endian Flag Length
pub const CHIPID_LEN: usize = 6; // Chip Id Length
pub const IMAGE_LEN: usize = 10; // Length of Length Field
pub const ADDRESS_LEN: usize = 12; // Length of Address field
pub const IMAGE_SEQUENCE_LEN: usize = 4; // Image sequence Length
pub const RSASIG_LEN: usize = 20; // Length of RSA Signature in tag
pub const TAGINFO1_LEN: usize = 30; // Length of vendor information field1 in tag
pub const FLASHLAYOUTVER_LEN: usize = 4; // Length of Flash Layout Version String tag
pub const TAGINFO2_LEN: usize = 16; // Length of vendor information field2 in tag
pub const ALTTAGINFO_LEN: usize = 54; // Alternate length for vendor information; Pirelli

pub const NUM_PIRELLI: usize = 2;
pub const IMAGETAG_CRC_START: u32 = 0xFFFF_FFFF;

pub const PIRELLI_BOARDS: [&str; NUM_PIRELLI] = ["AGPF-S0", "DWV-S0"];

/* Extended flash address, needs to be subtracted
 * from bcm_tag flash image offsets.
 */
pub const BCM963XX_EXTENDED_SIZE: u32 = 0xBFC0_0000;

/*
 * The broadcom firmware assumes the rootfs starts the image,
 * therefore uses the rootfs start (flash_image_address)
 * to determine where to flash the image.  Since we have the kernel first
 * we have to give it the kernel address, but the crc uses the length
 * associated with this address (root_length), which is added to the kernel
 * length (kernel_length) to determine the length of image to flash and thus
 * needs to be rootfs + deadcode (jffs2 EOF marker)
 */
#[repr(C)]
pub struct bcm_tag {
    /* 0-3: Version of the image tag */
    pub tag_version: [i8; TAGVER_LEN],
    /* 4-23: Company Line 1 */
    pub sig_1: [i8; SIG1_LEN],
    /* 24-37: Company Line 2 */
    pub sig_2: [i8; SIG2_LEN],
    /* 38-43: Chip this image is for */
    pub chip_id: [i8; CHIPID_LEN],
    /* 44-59: Board name */
    pub board_id: [i8; BOARDID_LEN],
    /* 60-61: Map endianness -- 1 BE 0 LE */
    pub big_endian: [i8; ENDIANFLAG_LEN],
    /* 62-71: Total length of image */
    pub total_length: [i8; IMAGE_LEN],
    /* 72-83: Address in memory of CFE */
    pub cfe__address: [i8; ADDRESS_LEN],
    /* 84-93: Size of CFE */
    pub cfe_length: [i8; IMAGE_LEN],
    /* 94-105: Address in memory of image start
     * (kernel for OpenWRT, rootfs for stock firmware)
     */
    pub flash_image_start: [i8; ADDRESS_LEN],
    /* 106-115: Size of rootfs */
    pub root_length: [i8; IMAGE_LEN],
    /* 116-127: Address in memory of kernel */
    pub kernel_address: [i8; ADDRESS_LEN],
    /* 128-137: Size of kernel */
    pub kernel_length: [i8; IMAGE_LEN],
    /* 138-141: Image sequence number
     * (to be incremented when flashed with a new image)
     */
    pub image_sequence: [i8; IMAGE_SEQUENCE_LEN],
    /* 142-161: RSA Signature (not used; some vendors may use this) */
    pub rsa_signature: [i8; RSASIG_LEN],
    /* 162-191: Compilation and related information (not used in OpenWrt) */
    pub information1: [i8; TAGINFO1_LEN],
    /* 192-195: Version flash layout */
    pub flash_layout_ver: [i8; FLASHLAYOUTVER_LEN],
    /* 196-199: kernel+rootfs CRC32 */
    pub fskernel_crc: u32,
    /* 200-215: Unused except on Alice Gate where it is information */
    pub information2: [i8; TAGINFO2_LEN],
    /* 216-219: CRC32 of image less imagetag (kernel for Alice Gate) */
    pub image_crc: u32,
    /* 220-223: CRC32 of rootfs partition */
    pub rootfs_crc: u32,
    /* 224-227: CRC32 of kernel partition */
    pub kernel_crc: u32,
    /* 228-235: Unused at present */
    pub reserved1: [i8; 8],
    /* 236-239: CRC32 of header excluding last 20 bytes */
    pub header_crc: u32,
    /* 240-255: Unused at present */
    pub reserved2: [i8; 16],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
