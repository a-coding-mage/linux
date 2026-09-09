/* SPDX-License-Identifier: GPL-2.0 */

/* UltraSparc-III/III+ Dispatch Control Register, ASR 0x12 */
pub const DCR_DPE: u64 = 0x0000_0000_0000_1000; /* III+: D$ Parity Error Enable */
pub const DCR_OBS: u64 = 0x0000_0000_0000_0fc0; /* Observability Bus Controls */
pub const DCR_BPE: u64 = 0x0000_0000_0000_0020; /* Branch Predict Enable */
pub const DCR_RPE: u64 = 0x0000_0000_0000_0010; /* Return Address Prediction Enable */
pub const DCR_SI: u64 = 0x0000_0000_0000_0008; /* Single Instruction Disable */
pub const DCR_IPE: u64 = 0x0000_0000_0000_0004; /* III+: I$ Parity Error Enable */
pub const DCR_IFPOE: u64 = 0x0000_0000_0000_0002; /* IRQ FP Operation Enable */
pub const DCR_MS: u64 = 0x0000_0000_0000_0001; /* Multi-Scalar dispatch */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
