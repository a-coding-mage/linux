/*
 * Copyright 2015 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency supplied by amd_pcie.h in the original C header.

pub unsafe fn is_pcie_gen3_supported(pcie_link_speed_cap: u32) -> bool {
    if pcie_link_speed_cap & CAIL_PCIE_LINK_SPEED_SUPPORT_GEN3 != 0 {
        return true;
    }

    false
}

pub unsafe fn is_pcie_gen2_supported(pcie_link_speed_cap: u32) -> bool {
    if pcie_link_speed_cap & CAIL_PCIE_LINK_SPEED_SUPPORT_GEN2 != 0 {
        return true;
    }

    false
}

/* Get the new PCIE speed given the ASIC PCIE Cap and the NewState's requested PCIE speed*/
pub unsafe fn get_pcie_gen_support(
    pcie_link_speed_cap: u32,
    ns_pcie_gen: u16,
) -> u16 {
    let asic_pcie_link_speed_cap =
        pcie_link_speed_cap & CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_MASK;
    let sys_pcie_link_speed_cap = pcie_link_speed_cap & CAIL_PCIE_LINK_SPEED_SUPPORT_MASK;

    match asic_pcie_link_speed_cap {
        CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_GEN1 => PP_PCIEGen1,
        CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_GEN2 => PP_PCIEGen2,
        CAIL_ASIC_PCIE_LINK_SPEED_SUPPORT_GEN3 => PP_PCIEGen3,
        _ => {
            if is_pcie_gen3_supported(sys_pcie_link_speed_cap) && ns_pcie_gen == PP_PCIEGen3 {
                PP_PCIEGen3
            } else if is_pcie_gen2_supported(sys_pcie_link_speed_cap)
                && (ns_pcie_gen == PP_PCIEGen3 || ns_pcie_gen == PP_PCIEGen2)
            {
                PP_PCIEGen2
            } else {
                PP_PCIEGen1
            }
        }
    }
}

pub unsafe fn get_pcie_lane_support(
    pcie_lane_width_cap: u32,
    ns_pcie_lanes: u16,
) -> u16 {
    let mut i: i32;
    let mut j: i32;
    let mut new_pcie_lanes = ns_pcie_lanes;
    let pcie_lanes: [u16; 7] = [1, 2, 4, 8, 12, 16, 32];

    match pcie_lane_width_cap {
        0 => pr_err!("No valid PCIE lane width reported\n"),
        CAIL_PCIE_LINK_WIDTH_SUPPORT_X1 => new_pcie_lanes = 1,
        CAIL_PCIE_LINK_WIDTH_SUPPORT_X2 => new_pcie_lanes = 2,
        CAIL_PCIE_LINK_WIDTH_SUPPORT_X4 => new_pcie_lanes = 4,
        CAIL_PCIE_LINK_WIDTH_SUPPORT_X8 => new_pcie_lanes = 8,
        CAIL_PCIE_LINK_WIDTH_SUPPORT_X12 => new_pcie_lanes = 12,
        CAIL_PCIE_LINK_WIDTH_SUPPORT_X16 => new_pcie_lanes = 16,
        CAIL_PCIE_LINK_WIDTH_SUPPORT_X32 => new_pcie_lanes = 32,
        _ => {
            i = 0;
            while i < 7 {
                if ns_pcie_lanes == pcie_lanes[i as usize] {
                    if pcie_lane_width_cap & (0x10000u32 << i) != 0 {
                        break;
                    } else {
                        j = i - 1;
                        while j >= 0 {
                            if pcie_lane_width_cap & (0x10000u32 << j) != 0 {
                                new_pcie_lanes = pcie_lanes[j as usize];
                                break;
                            }
                            j -= 1;
                        }

                        if j < 0 {
                            j = i + 1;
                            while j < 7 {
                                if pcie_lane_width_cap & (0x10000u32 << j) != 0 {
                                    new_pcie_lanes = pcie_lanes[j as usize];
                                    break;
                                }
                                j += 1;
                            }
                            if j > 7 {
                                pr_err!("Cannot find a valid PCIE lane width!\n");
                            }
                        }
                    }
                    break;
                }
                i += 1;
            }
        }
    }

    new_pcie_lanes
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
