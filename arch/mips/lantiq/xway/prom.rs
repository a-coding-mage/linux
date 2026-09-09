// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2010 John Crispin <john@phrozen.org>
 *  Copyright (C) 2013-2015 Lantiq Beteiligungs-GmbH & Co.KG
 */

// Dependencies supplied by the surrounding kernel translation.

const SOC_DANUBE: &str = "Danube";
const SOC_TWINPASS: &str = "Twinpass";
const SOC_AMAZON_SE: &str = "Amazon_SE";
const SOC_AR9: &str = "AR9";
const SOC_GR9: &str = "GRX200";
const SOC_VR9: &str = "xRX200";
const SOC_VRX220: &str = "xRX220";
const SOC_AR10: &str = "xRX300";
const SOC_GRX390: &str = "xRX330";

const COMP_DANUBE: &str = "lantiq,danube";
const COMP_TWINPASS: &str = "lantiq,twinpass";
const COMP_AMAZON_SE: &str = "lantiq,ase";
const COMP_AR9: &str = "lantiq,ar9";
const COMP_GR9: &str = "lantiq,gr9";
const COMP_VR9: &str = "lantiq,vr9";
const COMP_AR10: &str = "lantiq,ar10";
const COMP_GRX390: &str = "lantiq,grx390";

const PART_SHIFT: u32 = 12;
const PART_MASK: u32 = 0x0FFFFFFF;
const REV_SHIFT: u32 = 28;
const REV_MASK: u32 = 0xF0000000;

pub unsafe fn ltq_soc_detect(i: *mut ltq_soc_info) {
    (*i).partnum = (ltq_r32(LTQ_MPS_CHIPID) & PART_MASK) >> PART_SHIFT;
    (*i).rev = (ltq_r32(LTQ_MPS_CHIPID) & REV_MASK) >> REV_SHIFT;
    sprintf((*i).rev_type.as_mut_ptr(), "1.%d", (*i).rev);
    match (*i).partnum {
        SOC_ID_DANUBE1 | SOC_ID_DANUBE2 => {
            (*i).name = SOC_DANUBE;
            (*i).type_ = SOC_TYPE_DANUBE;
            (*i).compatible = COMP_DANUBE;
        }

        SOC_ID_TWINPASS => {
            (*i).name = SOC_TWINPASS;
            (*i).type_ = SOC_TYPE_DANUBE;
            (*i).compatible = COMP_TWINPASS;
        }

        SOC_ID_ARX188 | SOC_ID_ARX168_1 | SOC_ID_ARX168_2 | SOC_ID_ARX182 => {
            (*i).name = SOC_AR9;
            (*i).type_ = SOC_TYPE_AR9;
            (*i).compatible = COMP_AR9;
        }

        SOC_ID_GRX188 | SOC_ID_GRX168 => {
            (*i).name = SOC_GR9;
            (*i).type_ = SOC_TYPE_AR9;
            (*i).compatible = COMP_GR9;
        }

        SOC_ID_AMAZON_SE_1 | SOC_ID_AMAZON_SE_2 => {
            // CONFIG_PCI: panic!("ase is only supported for non pci kernels");
            (*i).name = SOC_AMAZON_SE;
            (*i).type_ = SOC_TYPE_AMAZON_SE;
            (*i).compatible = COMP_AMAZON_SE;
        }

        SOC_ID_VRX282 | SOC_ID_VRX268 | SOC_ID_VRX288 => {
            (*i).name = SOC_VR9;
            (*i).type_ = SOC_TYPE_VR9;
            (*i).compatible = COMP_VR9;
        }

        SOC_ID_GRX268 | SOC_ID_GRX288 => {
            (*i).name = SOC_GR9;
            (*i).type_ = SOC_TYPE_VR9;
            (*i).compatible = COMP_GR9;
        }

        SOC_ID_VRX268_2 | SOC_ID_VRX288_2 => {
            (*i).name = SOC_VR9;
            (*i).type_ = SOC_TYPE_VR9_2;
            (*i).compatible = COMP_VR9;
        }

        SOC_ID_VRX220 => {
            (*i).name = SOC_VRX220;
            (*i).type_ = SOC_TYPE_VRX220;
            (*i).compatible = COMP_VR9;
        }

        SOC_ID_GRX282_2 | SOC_ID_GRX288_2 => {
            (*i).name = SOC_GR9;
            (*i).type_ = SOC_TYPE_VR9_2;
            (*i).compatible = COMP_GR9;
        }

        SOC_ID_ARX362 | SOC_ID_ARX368 | SOC_ID_ARX382 | SOC_ID_ARX388 | SOC_ID_URX388 => {
            (*i).name = SOC_AR10;
            (*i).type_ = SOC_TYPE_AR10;
            (*i).compatible = COMP_AR10;
        }

        SOC_ID_GRX383 | SOC_ID_GRX369 | SOC_ID_GRX387 | SOC_ID_GRX389 => {
            (*i).name = SOC_GRX390;
            (*i).type_ = SOC_TYPE_GRX390;
            (*i).compatible = COMP_GRX390;
        }

        _ => unreachable!(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
