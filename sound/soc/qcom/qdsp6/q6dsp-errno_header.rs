// SPDX-License-Identifier: GPL-2.0

// C header dependency: <linux/kernel.h>

/* Success. The operation completed with no errors. */
pub const ADSP_EOK: u32 = 0x00000000;
/* General failure. */
pub const ADSP_EFAILED: u32 = 0x00000001;
/* Bad operation parameter. */
pub const ADSP_EBADPARAM: u32 = 0x00000002;
/* Unsupported routine or operation. */
pub const ADSP_EUNSUPPORTED: u32 = 0x00000003;
/* Unsupported version. */
pub const ADSP_EVERSION: u32 = 0x00000004;
/* Unexpected problem encountered. */
pub const ADSP_EUNEXPECTED: u32 = 0x00000005;
/* Unhandled problem occurred. */
pub const ADSP_EPANIC: u32 = 0x00000006;
/* Unable to allocate resource. */
pub const ADSP_ENORESOURCE: u32 = 0x00000007;
/* Invalid handle. */
pub const ADSP_EHANDLE: u32 = 0x00000008;
/* Operation is already processed. */
pub const ADSP_EALREADY: u32 = 0x00000009;
/* Operation is not ready to be processed. */
pub const ADSP_ENOTREADY: u32 = 0x0000000A;
/* Operation is pending completion. */
pub const ADSP_EPENDING: u32 = 0x0000000B;
/* Operation could not be accepted or processed. */
pub const ADSP_EBUSY: u32 = 0x0000000C;
/* Operation aborted due to an error. */
pub const ADSP_EABORTED: u32 = 0x0000000D;
/* Operation preempted by a higher priority. */
pub const ADSP_EPREEMPTED: u32 = 0x0000000E;
/* Operation requests intervention to complete. */
pub const ADSP_ECONTINUE: u32 = 0x0000000F;
/* Operation requests immediate intervention to complete. */
pub const ADSP_EIMMEDIATE: u32 = 0x00000010;
/* Operation is not implemented. */
pub const ADSP_ENOTIMPL: u32 = 0x00000011;
/* Operation needs more data or resources. */
pub const ADSP_ENEEDMORE: u32 = 0x00000012;
/* Operation does not have memory. */
pub const ADSP_ENOMEMORY: u32 = 0x00000014;
/* Item does not exist. */
pub const ADSP_ENOTEXIST: u32 = 0x00000015;
/* Max count for adsp error code sent to HLOS*/

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
