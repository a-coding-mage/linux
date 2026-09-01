/*
 * Several system calls take a flag to request "hugetlb" huge pages.
 * Without further specification, these system calls will use the
 * system's default huge page size.  If a system supports multiple
 * huge page sizes, the desired huge page size can be specified in
 * bits [26:31] of the flag arguments.  The value in these 6 bits
 * will encode the log2 of the huge page size.
 *
 * The following definitions are associated with this huge page size
 * encoding in flag arguments.  System call specific header files
 * that use this encoding should include this file.  They can then
 * provide definitions based on these with their own specific prefix.
 * for example:
 * #define MAP_HUGE_SHIFT HUGETLB_FLAG_ENCODE_SHIFT
 */

pub const HUGETLB_FLAG_ENCODE_SHIFT: u32 = 26;
pub const HUGETLB_FLAG_ENCODE_MASK: u32 = 0x3f;

pub const HUGETLB_FLAG_ENCODE_16KB: u32 = 14u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_64KB: u32 = 16u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_512KB: u32 = 19u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_1MB: u32 = 20u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_2MB: u32 = 21u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_8MB: u32 = 23u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_16MB: u32 = 24u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_32MB: u32 = 25u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_256MB: u32 = 28u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_512MB: u32 = 29u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_1GB: u32 = 30u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_2GB: u32 = 31u32 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_16GB: u32 = 34u32 << HUGETLB_FLAG_ENCODE_SHIFT;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
