use super::{GPCommand, GX_PIPE};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct BPReg(u8);
impl BPReg {
	pub const GEN_MODE: Self = Self(0x00);
	pub const DISP_COPY_FILT0: Self = Self(0x01);
	pub const DISP_COPY_FILT1: Self = Self(0x02);
	pub const DISP_COPY_FILT2: Self = Self(0x03);
	pub const DISP_COPY_FILT3: Self = Self(0x04);
	pub const IND_MTXA0: Self = Self(0x06);
	pub const IND_MTXB0: Self = Self(0x07);
	pub const IND_MTXC0: Self = Self(0x08);
	pub const IND_MTXA1: Self = Self(0x09);
	pub const IND_MTXB1: Self = Self(0x0A);
	pub const IND_MTXC1: Self = Self(0x0B);
	pub const IND_MTXA2: Self = Self(0x0C);
	pub const IND_MTXB2: Self = Self(0x0D);
	pub const IND_MTXC2: Self = Self(0x0E);
	pub const IND_IMASK: Self = Self(0x0F);
	pub const IND_CMD0: Self = Self(0x10);
	pub const IND_CMD1: Self = Self(0x11);
	pub const IND_CMD2: Self = Self(0x12);
	pub const IND_CMD3: Self = Self(0x13);
	pub const IND_CMD4: Self = Self(0x14);
	pub const IND_CMD5: Self = Self(0x15);
	pub const IND_CMD6: Self = Self(0x16);
	pub const IND_CMD7: Self = Self(0x17);
	pub const IND_CMD8: Self = Self(0x18);
	pub const IND_CMD9: Self = Self(0x19);
	pub const IND_CMDA: Self = Self(0x1A);
	pub const IND_CMDB: Self = Self(0x1B);
	pub const IND_CMDC: Self = Self(0x1C);
	pub const IND_CMDD: Self = Self(0x1D);
	pub const IND_CMDE: Self = Self(0x1E);
	pub const IND_CMDF: Self = Self(0x1F);
	pub const SU_SCIS0: Self = Self(0x20);
	pub const SU_SCIS1: Self = Self(0x21);
	pub const SU_LPSIZE: Self = Self(0x22);
	pub const SU_CNTR: Self = Self(0x23);
	pub const RAS_CNTR: Self = Self(0x24);
	pub const RAS1_SS0: Self = Self(0x25);
	pub const RAS1_SS1: Self = Self(0x26);
	pub const RAS1_IREF: Self = Self(0x27);
	pub const RAS1_TREF0: Self = Self(0x28);
	pub const RAS1_TREF1: Self = Self(0x29);
	pub const RAS1_TREF2: Self = Self(0x2A);
	pub const RAS1_TREF3: Self = Self(0x2B);
	pub const RAS1_TREF4: Self = Self(0x2C);
	pub const RAS1_TREF5: Self = Self(0x2D);
	pub const RAS1_TREF6: Self = Self(0x2E);
	pub const RAS1_TREF7: Self = Self(0x2F);
	pub const SU_SSIZE0: Self = Self(0x30);
	pub const SU_TSIZE0: Self = Self(0x31);
	pub const SU_SSIZE1: Self = Self(0x32);
	pub const SU_TSIZE1: Self = Self(0x33);
	pub const SU_SSIZE2: Self = Self(0x34);
	pub const SU_TSIZE2: Self = Self(0x35);
	pub const SU_SSIZE3: Self = Self(0x36);
	pub const SU_TSIZE3: Self = Self(0x37);
	pub const SU_SSIZE4: Self = Self(0x38);
	pub const SU_TSIZE4: Self = Self(0x39);
	pub const SU_SSIZE5: Self = Self(0x3A);
	pub const SU_TSIZE5: Self = Self(0x3B);
	pub const SU_SSIZE6: Self = Self(0x3C);
	pub const SU_TSIZE6: Self = Self(0x3D);
	pub const SU_SSIZE7: Self = Self(0x3E);
	pub const SU_TSIZE7: Self = Self(0x3F);
	pub const PE_ZMODE: Self = Self(0x40);
	pub const PE_CMODE0: Self = Self(0x41);
	pub const PE_CMODE1: Self = Self(0x42);
	pub const PE_CTRL: Self = Self(0x43);
	pub const FIELD_MASK: Self = Self(0x44);
	pub const PE_DONE: Self = Self(0x45);
	pub const CLOCK0: Self = Self(0x46);
	pub const PE_TOKEN: Self = Self(0x47);
	pub const PE_TOKEN_INT: Self = Self(0x48);
	pub const EFB_ADDR_TOP_LEFT: Self = Self(0x49);
	pub const EFB_ADDR_DIMENSIONS: Self = Self(0x4A);
	pub const XFB_ADDR: Self = Self(0x4B);
	// 0x4C
	pub const MIPMAP_STRIDE: Self = Self(0x4D);
	pub const DISP_COPY_Y_SCALE: Self = Self(0x4E);
	pub const PE_CLEAR_AR: Self = Self(0x4F);
	pub const PE_CLEAR_GB: Self = Self(0x50);
	pub const PE_CLEAR_Z: Self = Self(0x51);
	// 0x52 Something PE Related
	pub const TRGT_COPY_FILT0: Self = Self(0x53);
	pub const TRGT_COPY_FILT1: Self = Self(0x54);
	pub const BOUNDING_BOX0: Self = Self(0x55);
	pub const BOUNDING_BOX1: Self = Self(0x56);
	// 0x57
	pub const REV_STUFF: Self = Self(0x58);
	pub const SU_SCISOFF: Self = Self(0x59);
	// 0x5A- 0x62
	pub const TEX_MODE_SYNC: Self = Self(0x63);
	pub const TEX_TLUT0: Self = Self(0x64);
	pub const TEX_TLUT1: Self = Self(0x65);
	pub const TEX_INVALIDATE: Self = Self(0x66);
	pub const PERF_METRIC: Self = Self(0x67);
	pub const FIELD_MODE: Self = Self(0x68);
	pub const CLOCK1: Self = Self(0x69);
	// 0x6a - 0x7f
	pub const TX_SETMODE0_I0: Self = Self(0x80);
	pub const TX_SETMODE0_I1: Self = Self(0x81);
	pub const TX_SETMODE0_I2: Self = Self(0x82);
	pub const TX_SETMODE0_I3: Self = Self(0x83);
	pub const TX_SETMODE1_I0: Self = Self(0x84);
	pub const TX_SETMODE1_I1: Self = Self(0x85);
	pub const TX_SETMODE1_I2: Self = Self(0x86);
	pub const TX_SETMODE1_I3: Self = Self(0x87);
	pub const TX_SETIMAGE0_I0: Self = Self(0x88);
	pub const TX_SETIMAGE0_I1: Self = Self(0x89);
	pub const TX_SETIMAGE0_I2: Self = Self(0x8A);
	pub const TX_SETIMAGE0_I3: Self = Self(0x8B);
	pub const TX_SETIMAGE1_I0: Self = Self(0x8C);
	pub const TX_SETIMAGE1_I1: Self = Self(0x8D);
	pub const TX_SETIMAGE1_I2: Self = Self(0x8E);
	pub const TX_SETIMAGE1_I3: Self = Self(0x8F);
	pub const TX_SETIMAGE2_I0: Self = Self(0x90);
	pub const TX_SETIMAGE2_I1: Self = Self(0x91);
	pub const TX_SETIMAGE2_I2: Self = Self(0x92);
	pub const TX_SETIMAGE2_I3: Self = Self(0x93);
	pub const TX_SETIMAGE3_I0: Self = Self(0x94);
	pub const TX_SETIMAGE3_I1: Self = Self(0x95);
	pub const TX_SETIMAGE3_I2: Self = Self(0x96);
	pub const TX_SETIMAGE3_I3: Self = Self(0x97);
	pub const TX_SETTLUT_0: Self = Self(0x98);
	pub const TX_SETTLUT_1: Self = Self(0x99);
	pub const TX_SETTLUT_2: Self = Self(0x9A);
	pub const TX_SETTLUT_3: Self = Self(0x9B);
	// 0x9C - 0x9F
	pub const TX_SETMODE0_I4: Self = Self(0xA0);
	pub const TX_SETMODE0_I5: Self = Self(0xA1);
	pub const TX_SETMODE0_I6: Self = Self(0xA2);
	pub const TX_SETMODE0_I7: Self = Self(0xA3);
	pub const TX_SETMODE1_I4: Self = Self(0xA4);
	pub const TX_SETMODE1_I5: Self = Self(0xA5);
	pub const TX_SETMODE1_I6: Self = Self(0xA6);
	pub const TX_SETMODE1_I7: Self = Self(0xA7);
	pub const TX_SETIMAGE0_I4: Self = Self(0xA8);
	pub const TX_SETIMAGE0_I5: Self = Self(0xA9);
	pub const TX_SETIMAGE0_I6: Self = Self(0xAA);
	pub const TX_SETIMAGE0_I7: Self = Self(0xAB);
	pub const TX_SETIMAGE1_I4: Self = Self(0xAC);
	pub const TX_SETIMAGE1_I5: Self = Self(0xAD);
	pub const TX_SETIMAGE1_I6: Self = Self(0xAE);
	pub const TX_SETIMAGE1_I7: Self = Self(0xAF);
	pub const TX_SETIMAGE2_I4: Self = Self(0xB0);
	pub const TX_SETIMAGE2_I5: Self = Self(0xB1);
	pub const TX_SETIMAGE2_I6: Self = Self(0xB2);
	pub const TX_SETIMAGE2_I7: Self = Self(0xB3);
	pub const TX_SETIMAGE3_I4: Self = Self(0xB4);
	pub const TX_SETIMAGE3_I5: Self = Self(0xB5);
	pub const TX_SETIMAGE3_I6: Self = Self(0xB6);
	pub const TX_SETIMAGE3_I7: Self = Self(0xB7);
	pub const TX_SETTLUT_4: Self = Self(0xB8);
	pub const TX_SETTLUT_5: Self = Self(0xB9);
	pub const TX_SETTLUT_6: Self = Self(0xBA);
	pub const TX_SETTLUT_7: Self = Self(0xBB);
	// 0xBC - 0xBF
	pub const TEV_COLOR_ENV_0: Self = Self(0xC0);
	pub const TEV_ALPHA_ENV_0: Self = Self(0xC1);
	pub const TEV_COLOR_ENV_1: Self = Self(0xC2);
	pub const TEV_ALPHA_ENV_1: Self = Self(0xC3);
	pub const TEV_COLOR_ENV_2: Self = Self(0xC4);
	pub const TEV_ALPHA_ENV_2: Self = Self(0xC5);
	pub const TEV_COLOR_ENV_3: Self = Self(0xC6);
	pub const TEV_ALPHA_ENV_3: Self = Self(0xC7);
	pub const TEV_COLOR_ENV_4: Self = Self(0xC8);
	pub const TEV_ALPHA_ENV_4: Self = Self(0xC9);
	pub const TEV_COLOR_ENV_5: Self = Self(0xCA);
	pub const TEV_ALPHA_ENV_5: Self = Self(0xCB);
	pub const TEV_COLOR_ENV_6: Self = Self(0xCC);
	pub const TEV_ALPHA_ENV_6: Self = Self(0xCD);
	pub const TEV_COLOR_ENV_7: Self = Self(0xCE);
	pub const TEV_ALPHA_ENV_7: Self = Self(0xCF);
	pub const TEV_COLOR_ENV_8: Self = Self(0xD0);
	pub const TEV_ALPHA_ENV_8: Self = Self(0xD1);
	pub const TEV_COLOR_ENV_9: Self = Self(0xD2);
	pub const TEV_ALPHA_ENV_9: Self = Self(0xD3);
	pub const TEV_COLOR_ENV_A: Self = Self(0xD4);
	pub const TEV_ALPHA_ENV_A: Self = Self(0xD5);
	pub const TEV_COLOR_ENV_B: Self = Self(0xD6);
	pub const TEV_ALPHA_ENV_B: Self = Self(0xD7);
	pub const TEV_COLOR_ENV_C: Self = Self(0xD8);
	pub const TEV_ALPHA_ENV_C: Self = Self(0xD9);
	pub const TEV_COLOR_ENV_D: Self = Self(0xDA);
	pub const TEV_ALPHA_ENV_D: Self = Self(0xDB);
	pub const TEV_COLOR_ENV_E: Self = Self(0xDC);
	pub const TEV_ALPHA_ENV_E: Self = Self(0xDD);
	pub const TEV_COLOR_ENV_F: Self = Self(0xDE);
	pub const TEV_ALPHA_ENV_F: Self = Self(0xDF);
	pub const TEV_REGISTER_L0: Self = Self(0xE0);
	pub const TEV_REGISTER_H0: Self = Self(0xE1);
	pub const TEV_REGISTER_L1: Self = Self(0xE2);
	pub const TEV_REGISTER_H1: Self = Self(0xE3);
	pub const TEV_REGISTER_L2: Self = Self(0xE4);
	pub const TEV_REGISTER_H2: Self = Self(0xE5);
	pub const TEV_REGISTER_L3: Self = Self(0xE6);
	pub const TEV_REGISTER_H3: Self = Self(0xE7);
	pub const FOG_ENABLE_RANGE: Self = Self(0xE8);
	pub const FOG_RANGE_0: Self = Self(0xE9);
	pub const FOG_RANGE_1: Self = Self(0xEA);
	pub const FOG_RANGE_2: Self = Self(0xEB);
	pub const FOG_RANGE_3: Self = Self(0xEC);
	pub const FOG_RANGE_4: Self = Self(0xED);
	pub const FOG_PARAM_0: Self = Self(0xEE);
	pub const FOG_PARAM_1: Self = Self(0xEF);
	pub const FOG_PARAM_2: Self = Self(0xF0);
	pub const FOG_PARAM_3: Self = Self(0xF1);
	pub const FOG_COLOR: Self = Self(0xF2);
	pub const TEV_ALPHAFUNC: Self = Self(0xF3);
	pub const TEV_Z_Z_ENV_0: Self = Self(0xF4);
	pub const TEV_Z_ENV_1: Self = Self(0xF5);
	pub const TEV_KSEL_0: Self = Self(0xF6);
	pub const TEV_KSEL_1: Self = Self(0xF7);
	pub const TEV_KSEL_2: Self = Self(0xF8);
	pub const TEV_KSEL_3: Self = Self(0xF9);
	pub const TEV_KSEL_4: Self = Self(0xFA);
	pub const TEV_KSEL_5: Self = Self(0xFB);
	pub const TEV_KSEL_6: Self = Self(0xFC);
	pub const TEV_KSEL_7: Self = Self(0xFD);
	pub const SS_MASK: Self = Self(0xFE);
	// 0xFF

	// Loads and write a specific value `val` to self,
	pub fn load(&self, val: u32) {
		assert!(val <= 0xFFFFFF);
		GX_PIPE.write(GPCommand::LoadBPReg as u8);
		GX_PIPE.write(self.0);
		// We only want the bottom 24 bits so we only grab the bottom 3 bytes
		for byte in &val.to_be_bytes()[1..=3] {
			GX_PIPE.write(*byte);
		}
	}
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct CPReg(u8);
impl CPReg {
	pub const CP_PERF_MODE: Self = Self(0x20);
	pub const MTXIDX_0: Self = Self(0x30);
	pub const MTXIDX_1: Self = Self(0x40);
	pub const VERT_DESC_LO_0: Self = Self(0x50);
	pub const VERT_DESC_LO_1: Self = Self(0x51);
	pub const VERT_DESC_LO_2: Self = Self(0x52);
	pub const VERT_DESC_LO_3: Self = Self(0x53);
	pub const VERT_DESC_LO_4: Self = Self(0x54);
	pub const VERT_DESC_LO_5: Self = Self(0x55);
	pub const VERT_DESC_LO_6: Self = Self(0x56);
	pub const VERT_DESC_LO_7: Self = Self(0x57);
	pub const VERT_DESC_HI_0: Self = Self(0x60);
	pub const VERT_DESC_HI_1: Self = Self(0x61);
	pub const VERT_DESC_HI_2: Self = Self(0x62);
	pub const VERT_DESC_HI_3: Self = Self(0x63);
	pub const VERT_DESC_HI_4: Self = Self(0x64);
	pub const VERT_DESC_HI_5: Self = Self(0x65);
	pub const VERT_DESC_HI_6: Self = Self(0x66);
	pub const VERT_DESC_HI_7: Self = Self(0x67);
	pub const VAT_A_FORMAT0: Self = Self(0x70);
	pub const VAT_A_FORMAT1: Self = Self(0x71);
	pub const VAT_A_FORMAT2: Self = Self(0x72);
	pub const VAT_A_FORMAT3: Self = Self(0x73);
	pub const VAT_A_FORMAT4: Self = Self(0x74);
	pub const VAT_A_FORMAT5: Self = Self(0x75);
	pub const VAT_A_FORMAT6: Self = Self(0x76);
	pub const VAT_A_FORMAT7: Self = Self(0x77);
	pub const VAT_B_FORMAT0: Self = Self(0x80);
	pub const VAT_B_FORMAT1: Self = Self(0x81);
	pub const VAT_B_FORMAT2: Self = Self(0x82);
	pub const VAT_B_FORMAT3: Self = Self(0x83);
	pub const VAT_B_FORMAT4: Self = Self(0x84);
	pub const VAT_B_FORMAT5: Self = Self(0x85);
	pub const VAT_B_FORMAT6: Self = Self(0x86);
	pub const VAT_B_FORMAT7: Self = Self(0x87);
	pub const VAT_C_FORMAT0: Self = Self(0x90);
	pub const VAT_C_FORMAT1: Self = Self(0x91);
	pub const VAT_C_FORMAT2: Self = Self(0x92);
	pub const VAT_C_FORMAT3: Self = Self(0x93);
	pub const VAT_C_FORMAT4: Self = Self(0x94);
	pub const VAT_C_FORMAT5: Self = Self(0x95);
	pub const VAT_C_FORMAT6: Self = Self(0x96);
	pub const VAT_C_FORMAT7: Self = Self(0x97);
	pub const VERT_PTR: Self = Self(0xA0);
	pub const NORM_PTR: Self = Self(0xA1);
	pub const COL0_PTR: Self = Self(0xA2);
	pub const COL1_PTR: Self = Self(0xA3);
	pub const TEX0_PTR: Self = Self(0xA4);
	pub const TEX1_PTR: Self = Self(0xA5);
	pub const TEX2_PTR: Self = Self(0xA6);
	pub const TEX3_PTR: Self = Self(0xA7);
	pub const TEX4_PTR: Self = Self(0xA8);
	pub const TEX5_PTR: Self = Self(0xA9);
	pub const TEX6_PTR: Self = Self(0xAA);
	pub const TEX7_PTR: Self = Self(0xAB);
	pub const IDXA_PTR: Self = Self(0xAC);
	pub const IDXB_PTR: Self = Self(0xAD);
	pub const IDXC_PTR: Self = Self(0xAE);
	pub const IDXD_PTR: Self = Self(0xAF);
	pub const VERT_SIZE: Self = Self(0xB0);
	pub const NORM_SIZE: Self = Self(0xB1);
	pub const COL0_SIZE: Self = Self(0xB2);
	pub const COL1_SIZE: Self = Self(0xB3);
	pub const TEX0_SIZE: Self = Self(0xB4);
	pub const TEX1_SIZE: Self = Self(0xB5);
	pub const TEX2_SIZE: Self = Self(0xB6);
	pub const TEX3_SIZE: Self = Self(0xB7);
	pub const TEX4_SIZE: Self = Self(0xB8);
	pub const TEX5_SIZE: Self = Self(0xB9);
	pub const TEX6_SIZE: Self = Self(0xBA);
	pub const TEX7_SIZE: Self = Self(0xBB);
	pub const IDXA_SIZE: Self = Self(0xBC);
	pub const IDXB_SIZE: Self = Self(0xBD);
	pub const IDXC_SIZE: Self = Self(0xBE);
	pub const IDXD_SIZE: Self = Self(0xBF);

	// Loads and write a specific value `val` to self,
	pub fn load(&self, val: u32) {
		GX_PIPE.write(GPCommand::LoadCPReg as u8);
		GX_PIPE.write(self.0);

		for byte in val.to_be_bytes() {
			GX_PIPE.write(byte);
		}
	}
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct XFReg(u16);
impl XFReg {
	pub const ERROR: Self = Self(0x1000);
	pub const DIAG: Self = Self(0x1001);
	pub const STATE0: Self = Self(0x1002);
	pub const STATE1: Self = Self(0x1003);
	pub const XF_CLCK: Self = Self(0x1004);
	pub const CLIP_DISABLE: Self = Self(0x1005);
	pub const PERF0: Self = Self(0x1006);
	pub const PERF1: Self = Self(0x1007);
	pub const INVTXSPEC: Self = Self(0x1008);
	pub const NUMCOLORS: Self = Self(0x1009);
	pub const AMBIENT0: Self = Self(0x100A);
	pub const AMBIENT1: Self = Self(0x100B);
	pub const MATERIAL0: Self = Self(0x100C);
	pub const MATERIAL1: Self = Self(0x100D);
	pub const COL0CTRL: Self = Self(0x100E);
	pub const COL1CTRL: Self = Self(0x100F);
	pub const ALPHA0CTRL: Self = Self(0x1010);
	pub const ALPHA1CTRL: Self = Self(0x1011);
	pub const DUALTEXTRANS: Self = Self(0x1012);
	// 0x1013 - 0x1017
	pub const MTXIDX_A: Self = Self(0x1018);
	pub const MTXIDX_B: Self = Self(0x1019);
	pub const VIEW_SCALE_X: Self = Self(0x101A);
	pub const VIEW_SCALE_Y: Self = Self(0x101B);
	pub const VIEW_SCALE_Z: Self = Self(0x101C);
	pub const VIEW_OFF_X: Self = Self(0x101D);
	pub const VIEW_OFF_Y: Self = Self(0x101E);
	pub const VIEW_OFF_Z: Self = Self(0x101F);
	pub const PROJ_PRM_A: Self = Self(0x1020);
	pub const PROJ_PRM_B: Self = Self(0x1021);
	pub const PROJ_PRM_C: Self = Self(0x1022);
	pub const PROJ_PRM_D: Self = Self(0x1023);
	pub const PROJ_PRM_E: Self = Self(0x1024);
	pub const PROJ_PRM_F: Self = Self(0x1025);
	pub const PROJ_ORTHO: Self = Self(0x1026);
	pub const NUM_TEX: Self = Self(0x103f);
	pub const TEX0: Self = Self(0x1040);
	pub const TEX1: Self = Self(0x1041);
	pub const TEX2: Self = Self(0x1042);
	pub const TEX3: Self = Self(0x1043);
	pub const TEX4: Self = Self(0x1044);
	pub const TEX5: Self = Self(0x1045);
	pub const TEX6: Self = Self(0x1046);
	pub const TEX7: Self = Self(0x1047);
	pub const DUALTEX0: Self = Self(0x1050);
	pub const DUALTEX1: Self = Self(0x1051);
	pub const DUALTEX2: Self = Self(0x1052);
	pub const DUALTEX3: Self = Self(0x1053);
	pub const DUALTEX4: Self = Self(0x1054);
	pub const DUALTEX5: Self = Self(0x1055);
	pub const DUALTEX6: Self = Self(0x1056);
	pub const DUALTEX7: Self = Self(0x1057);

	// Loads and write a specific value `val` to self
	pub fn load(&self, val: u32) {
		GX_PIPE.write(GPCommand::LoadXFReg as u8);

		for byte in 0u16.to_be_bytes() {
			GX_PIPE.write(byte);
		}

		for byte in self.0.to_be_bytes() {
			GX_PIPE.write(byte);
		}

		for byte in val.to_be_bytes() {
			GX_PIPE.write(byte);
		}
	}

	// Using self as the base load multiple registers and write `vals` to them
	// vals need to have the same legth as the `length`
	pub fn load_multi(&self, length: u16, vals: &[[u8; 4]]) {
		assert!(vals.len() == length.try_into().unwrap());

		GX_PIPE.write(GPCommand::LoadXFReg as u8);
		for byte in (length - 1).to_be_bytes() {
			GX_PIPE.write(byte);
		}

		for byte in self.0.to_be_bytes() {
			GX_PIPE.write(byte);
		}

		for val in vals {
			for byte in val {
				GX_PIPE.write(*byte);
			}
		}
	}
}
