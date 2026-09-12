#[inline]
pub unsafe fn GetProcAddress<P1>(hmodule: HMODULE, lpprocname: P1) -> FARPROC
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : windows_core::PCSTR) -> FARPROC);
    unsafe { GetProcAddress(hmodule, lpprocname.param().abi()) }
}
#[inline]
pub unsafe fn LoadLibraryExA<P0>(lplibfilename: P0, hfile: Option<HANDLE>, dwflags: u32) -> HMODULE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : windows_core::PCSTR, hfile : HANDLE, dwflags : u32) -> HMODULE);
    unsafe {
        LoadLibraryExA(
            lplibfilename.param().abi(),
            hfile.unwrap_or(core::mem::zeroed()) as _,
            dwflags,
        )
    }
}
pub type BasicType = i32;
pub const CHKSUM_TYPE_MD5: CV_SourceChksum_t = 1;
pub const CHKSUM_TYPE_NONE: CV_SourceChksum_t = 0;
pub const CHKSUM_TYPE_SHA1: CV_SourceChksum_t = 2;
pub const CHKSUM_TYPE_SHA_256: CV_SourceChksum_t = 3;
pub const CHKSUM_TYPE_SHA_384: CV_SourceChksum_t = 4;
pub const CHKSUM_TYPE_SHA_512: CV_SourceChksum_t = 5;
pub const CV_ALLREG_CMDLN: CV_HREG_e = 30012;
pub const CV_ALLREG_EFAD1: CV_HREG_e = 30003;
pub const CV_ALLREG_EFAD2: CV_HREG_e = 30004;
pub const CV_ALLREG_EFAD3: CV_HREG_e = 30005;
pub const CV_ALLREG_ENV: CV_HREG_e = 30011;
pub const CV_ALLREG_ERR: CV_HREG_e = 30000;
pub const CV_ALLREG_HANDLE: CV_HREG_e = 30007;
pub const CV_ALLREG_LOCALS: CV_HREG_e = 30009;
pub const CV_ALLREG_PARAMS: CV_HREG_e = 30008;
pub const CV_ALLREG_TEB: CV_HREG_e = 30001;
pub const CV_ALLREG_TID: CV_HREG_e = 30010;
pub const CV_ALLREG_TIMER: CV_HREG_e = 30002;
pub const CV_ALLREG_VFRAME: CV_HREG_e = 30006;
pub const CV_ALPHA_Fir: CV_HREG_e = 75;
pub const CV_ALPHA_FltF0: CV_HREG_e = 10;
pub const CV_ALPHA_FltF1: CV_HREG_e = 11;
pub const CV_ALPHA_FltF10: CV_HREG_e = 20;
pub const CV_ALPHA_FltF11: CV_HREG_e = 21;
pub const CV_ALPHA_FltF12: CV_HREG_e = 22;
pub const CV_ALPHA_FltF13: CV_HREG_e = 23;
pub const CV_ALPHA_FltF14: CV_HREG_e = 24;
pub const CV_ALPHA_FltF15: CV_HREG_e = 25;
pub const CV_ALPHA_FltF16: CV_HREG_e = 26;
pub const CV_ALPHA_FltF17: CV_HREG_e = 27;
pub const CV_ALPHA_FltF18: CV_HREG_e = 28;
pub const CV_ALPHA_FltF19: CV_HREG_e = 29;
pub const CV_ALPHA_FltF2: CV_HREG_e = 12;
pub const CV_ALPHA_FltF20: CV_HREG_e = 30;
pub const CV_ALPHA_FltF21: CV_HREG_e = 31;
pub const CV_ALPHA_FltF22: CV_HREG_e = 32;
pub const CV_ALPHA_FltF23: CV_HREG_e = 33;
pub const CV_ALPHA_FltF24: CV_HREG_e = 34;
pub const CV_ALPHA_FltF25: CV_HREG_e = 35;
pub const CV_ALPHA_FltF26: CV_HREG_e = 36;
pub const CV_ALPHA_FltF27: CV_HREG_e = 37;
pub const CV_ALPHA_FltF28: CV_HREG_e = 38;
pub const CV_ALPHA_FltF29: CV_HREG_e = 39;
pub const CV_ALPHA_FltF3: CV_HREG_e = 13;
pub const CV_ALPHA_FltF30: CV_HREG_e = 40;
pub const CV_ALPHA_FltF31: CV_HREG_e = 41;
pub const CV_ALPHA_FltF4: CV_HREG_e = 14;
pub const CV_ALPHA_FltF5: CV_HREG_e = 15;
pub const CV_ALPHA_FltF6: CV_HREG_e = 16;
pub const CV_ALPHA_FltF7: CV_HREG_e = 17;
pub const CV_ALPHA_FltF8: CV_HREG_e = 18;
pub const CV_ALPHA_FltF9: CV_HREG_e = 19;
pub const CV_ALPHA_FltFsr: CV_HREG_e = 77;
pub const CV_ALPHA_Fpcr: CV_HREG_e = 74;
pub const CV_ALPHA_IntA0: CV_HREG_e = 58;
pub const CV_ALPHA_IntA1: CV_HREG_e = 59;
pub const CV_ALPHA_IntA2: CV_HREG_e = 60;
pub const CV_ALPHA_IntA3: CV_HREG_e = 61;
pub const CV_ALPHA_IntA4: CV_HREG_e = 62;
pub const CV_ALPHA_IntA5: CV_HREG_e = 63;
pub const CV_ALPHA_IntAT: CV_HREG_e = 70;
pub const CV_ALPHA_IntFP: CV_HREG_e = 57;
pub const CV_ALPHA_IntGP: CV_HREG_e = 71;
pub const CV_ALPHA_IntRA: CV_HREG_e = 68;
pub const CV_ALPHA_IntS0: CV_HREG_e = 51;
pub const CV_ALPHA_IntS1: CV_HREG_e = 52;
pub const CV_ALPHA_IntS2: CV_HREG_e = 53;
pub const CV_ALPHA_IntS3: CV_HREG_e = 54;
pub const CV_ALPHA_IntS4: CV_HREG_e = 55;
pub const CV_ALPHA_IntS5: CV_HREG_e = 56;
pub const CV_ALPHA_IntSP: CV_HREG_e = 72;
pub const CV_ALPHA_IntT0: CV_HREG_e = 43;
pub const CV_ALPHA_IntT1: CV_HREG_e = 44;
pub const CV_ALPHA_IntT10: CV_HREG_e = 66;
pub const CV_ALPHA_IntT11: CV_HREG_e = 67;
pub const CV_ALPHA_IntT12: CV_HREG_e = 69;
pub const CV_ALPHA_IntT2: CV_HREG_e = 45;
pub const CV_ALPHA_IntT3: CV_HREG_e = 46;
pub const CV_ALPHA_IntT4: CV_HREG_e = 47;
pub const CV_ALPHA_IntT5: CV_HREG_e = 48;
pub const CV_ALPHA_IntT6: CV_HREG_e = 49;
pub const CV_ALPHA_IntT7: CV_HREG_e = 50;
pub const CV_ALPHA_IntT8: CV_HREG_e = 64;
pub const CV_ALPHA_IntT9: CV_HREG_e = 65;
pub const CV_ALPHA_IntV0: CV_HREG_e = 42;
pub const CV_ALPHA_IntZERO: CV_HREG_e = 73;
pub const CV_ALPHA_NOREG: CV_HREG_e = 0;
pub const CV_ALPHA_Psr: CV_HREG_e = 76;
pub const CV_ALPHA_SoftFpcr: CV_HREG_e = 78;
pub const CV_AM33_A0: CV_HREG_e = 20;
pub const CV_AM33_A1: CV_HREG_e = 21;
pub const CV_AM33_A2: CV_HREG_e = 22;
pub const CV_AM33_A3: CV_HREG_e = 23;
pub const CV_AM33_D0: CV_HREG_e = 30;
pub const CV_AM33_D1: CV_HREG_e = 31;
pub const CV_AM33_D2: CV_HREG_e = 32;
pub const CV_AM33_D3: CV_HREG_e = 33;
pub const CV_AM33_E0: CV_HREG_e = 10;
pub const CV_AM33_E1: CV_HREG_e = 11;
pub const CV_AM33_E2: CV_HREG_e = 12;
pub const CV_AM33_E3: CV_HREG_e = 13;
pub const CV_AM33_E4: CV_HREG_e = 14;
pub const CV_AM33_E5: CV_HREG_e = 15;
pub const CV_AM33_E6: CV_HREG_e = 16;
pub const CV_AM33_E7: CV_HREG_e = 17;
pub const CV_AM33_EPSW: CV_HREG_e = 87;
pub const CV_AM33_FPCR: CV_HREG_e = 88;
pub const CV_AM33_FS0: CV_HREG_e = 40;
pub const CV_AM33_FS1: CV_HREG_e = 41;
pub const CV_AM33_FS10: CV_HREG_e = 50;
pub const CV_AM33_FS11: CV_HREG_e = 51;
pub const CV_AM33_FS12: CV_HREG_e = 52;
pub const CV_AM33_FS13: CV_HREG_e = 53;
pub const CV_AM33_FS14: CV_HREG_e = 54;
pub const CV_AM33_FS15: CV_HREG_e = 55;
pub const CV_AM33_FS16: CV_HREG_e = 56;
pub const CV_AM33_FS17: CV_HREG_e = 57;
pub const CV_AM33_FS18: CV_HREG_e = 58;
pub const CV_AM33_FS19: CV_HREG_e = 59;
pub const CV_AM33_FS2: CV_HREG_e = 42;
pub const CV_AM33_FS20: CV_HREG_e = 60;
pub const CV_AM33_FS21: CV_HREG_e = 61;
pub const CV_AM33_FS22: CV_HREG_e = 62;
pub const CV_AM33_FS23: CV_HREG_e = 63;
pub const CV_AM33_FS24: CV_HREG_e = 64;
pub const CV_AM33_FS25: CV_HREG_e = 65;
pub const CV_AM33_FS26: CV_HREG_e = 66;
pub const CV_AM33_FS27: CV_HREG_e = 67;
pub const CV_AM33_FS28: CV_HREG_e = 68;
pub const CV_AM33_FS29: CV_HREG_e = 69;
pub const CV_AM33_FS3: CV_HREG_e = 43;
pub const CV_AM33_FS30: CV_HREG_e = 70;
pub const CV_AM33_FS31: CV_HREG_e = 71;
pub const CV_AM33_FS4: CV_HREG_e = 44;
pub const CV_AM33_FS5: CV_HREG_e = 45;
pub const CV_AM33_FS6: CV_HREG_e = 46;
pub const CV_AM33_FS7: CV_HREG_e = 47;
pub const CV_AM33_FS8: CV_HREG_e = 48;
pub const CV_AM33_FS9: CV_HREG_e = 49;
pub const CV_AM33_LAR: CV_HREG_e = 90;
pub const CV_AM33_LIR: CV_HREG_e = 89;
pub const CV_AM33_MCRH: CV_HREG_e = 84;
pub const CV_AM33_MCRL: CV_HREG_e = 85;
pub const CV_AM33_MCVF: CV_HREG_e = 86;
pub const CV_AM33_MDR: CV_HREG_e = 82;
pub const CV_AM33_MDRQ: CV_HREG_e = 83;
pub const CV_AM33_NOREG: CV_HREG_e = 0;
pub const CV_AM33_PC: CV_HREG_e = 81;
pub const CV_AM33_SP: CV_HREG_e = 80;
pub const CV_AMD64_AH: CV_HREG_e = 5;
pub const CV_AMD64_AL: CV_HREG_e = 1;
pub const CV_AMD64_AX: CV_HREG_e = 9;
pub const CV_AMD64_BH: CV_HREG_e = 8;
pub const CV_AMD64_BL: CV_HREG_e = 4;
pub const CV_AMD64_BND0: CV_HREG_e = 688;
pub const CV_AMD64_BND1: CV_HREG_e = 689;
pub const CV_AMD64_BND2: CV_HREG_e = 690;
pub const CV_AMD64_BND3: CV_HREG_e = 691;
pub const CV_AMD64_BNDCFGU: CV_HREG_e = 692;
pub const CV_AMD64_BNDSTATUS: CV_HREG_e = 693;
pub const CV_AMD64_BP: CV_HREG_e = 14;
pub const CV_AMD64_BPL: CV_HREG_e = 326;
pub const CV_AMD64_BX: CV_HREG_e = 12;
pub const CV_AMD64_CH: CV_HREG_e = 6;
pub const CV_AMD64_CL: CV_HREG_e = 2;
pub const CV_AMD64_CR0: CV_HREG_e = 80;
pub const CV_AMD64_CR1: CV_HREG_e = 81;
pub const CV_AMD64_CR2: CV_HREG_e = 82;
pub const CV_AMD64_CR3: CV_HREG_e = 83;
pub const CV_AMD64_CR4: CV_HREG_e = 84;
pub const CV_AMD64_CR8: CV_HREG_e = 88;
pub const CV_AMD64_CS: CV_HREG_e = 26;
pub const CV_AMD64_CTRL: CV_HREG_e = 136;
pub const CV_AMD64_CX: CV_HREG_e = 10;
pub const CV_AMD64_DH: CV_HREG_e = 7;
pub const CV_AMD64_DI: CV_HREG_e = 16;
pub const CV_AMD64_DIL: CV_HREG_e = 325;
pub const CV_AMD64_DL: CV_HREG_e = 3;
pub const CV_AMD64_DR0: CV_HREG_e = 90;
pub const CV_AMD64_DR1: CV_HREG_e = 91;
pub const CV_AMD64_DR10: CV_HREG_e = 100;
pub const CV_AMD64_DR11: CV_HREG_e = 101;
pub const CV_AMD64_DR12: CV_HREG_e = 102;
pub const CV_AMD64_DR13: CV_HREG_e = 103;
pub const CV_AMD64_DR14: CV_HREG_e = 104;
pub const CV_AMD64_DR15: CV_HREG_e = 105;
pub const CV_AMD64_DR2: CV_HREG_e = 92;
pub const CV_AMD64_DR3: CV_HREG_e = 93;
pub const CV_AMD64_DR4: CV_HREG_e = 94;
pub const CV_AMD64_DR5: CV_HREG_e = 95;
pub const CV_AMD64_DR6: CV_HREG_e = 96;
pub const CV_AMD64_DR7: CV_HREG_e = 97;
pub const CV_AMD64_DR8: CV_HREG_e = 98;
pub const CV_AMD64_DR9: CV_HREG_e = 99;
pub const CV_AMD64_DS: CV_HREG_e = 28;
pub const CV_AMD64_DX: CV_HREG_e = 11;
pub const CV_AMD64_EAX: CV_HREG_e = 17;
pub const CV_AMD64_EBP: CV_HREG_e = 22;
pub const CV_AMD64_EBX: CV_HREG_e = 20;
pub const CV_AMD64_ECX: CV_HREG_e = 18;
pub const CV_AMD64_EDI: CV_HREG_e = 24;
pub const CV_AMD64_EDX: CV_HREG_e = 19;
pub const CV_AMD64_EFLAGS: CV_HREG_e = 34;
pub const CV_AMD64_EMM0H: CV_HREG_e = 228;
pub const CV_AMD64_EMM0L: CV_HREG_e = 220;
pub const CV_AMD64_EMM10H: CV_HREG_e = 318;
pub const CV_AMD64_EMM10L: CV_HREG_e = 310;
pub const CV_AMD64_EMM11H: CV_HREG_e = 319;
pub const CV_AMD64_EMM11L: CV_HREG_e = 311;
pub const CV_AMD64_EMM12H: CV_HREG_e = 320;
pub const CV_AMD64_EMM12L: CV_HREG_e = 312;
pub const CV_AMD64_EMM13H: CV_HREG_e = 321;
pub const CV_AMD64_EMM13L: CV_HREG_e = 313;
pub const CV_AMD64_EMM14H: CV_HREG_e = 322;
pub const CV_AMD64_EMM14L: CV_HREG_e = 314;
pub const CV_AMD64_EMM15H: CV_HREG_e = 323;
pub const CV_AMD64_EMM15L: CV_HREG_e = 315;
pub const CV_AMD64_EMM16H: CV_HREG_e = 830;
pub const CV_AMD64_EMM17H: CV_HREG_e = 831;
pub const CV_AMD64_EMM18H: CV_HREG_e = 832;
pub const CV_AMD64_EMM19H: CV_HREG_e = 833;
pub const CV_AMD64_EMM1H: CV_HREG_e = 229;
pub const CV_AMD64_EMM1L: CV_HREG_e = 221;
pub const CV_AMD64_EMM20H: CV_HREG_e = 834;
pub const CV_AMD64_EMM21H: CV_HREG_e = 835;
pub const CV_AMD64_EMM22H: CV_HREG_e = 836;
pub const CV_AMD64_EMM23H: CV_HREG_e = 837;
pub const CV_AMD64_EMM24H: CV_HREG_e = 838;
pub const CV_AMD64_EMM25H: CV_HREG_e = 839;
pub const CV_AMD64_EMM26H: CV_HREG_e = 840;
pub const CV_AMD64_EMM27H: CV_HREG_e = 841;
pub const CV_AMD64_EMM28H: CV_HREG_e = 842;
pub const CV_AMD64_EMM29H: CV_HREG_e = 843;
pub const CV_AMD64_EMM2H: CV_HREG_e = 230;
pub const CV_AMD64_EMM2L: CV_HREG_e = 222;
pub const CV_AMD64_EMM30H: CV_HREG_e = 844;
pub const CV_AMD64_EMM31H: CV_HREG_e = 845;
pub const CV_AMD64_EMM3H: CV_HREG_e = 231;
pub const CV_AMD64_EMM3L: CV_HREG_e = 223;
pub const CV_AMD64_EMM4H: CV_HREG_e = 232;
pub const CV_AMD64_EMM4L: CV_HREG_e = 224;
pub const CV_AMD64_EMM5H: CV_HREG_e = 233;
pub const CV_AMD64_EMM5L: CV_HREG_e = 225;
pub const CV_AMD64_EMM6H: CV_HREG_e = 234;
pub const CV_AMD64_EMM6L: CV_HREG_e = 226;
pub const CV_AMD64_EMM7H: CV_HREG_e = 235;
pub const CV_AMD64_EMM7L: CV_HREG_e = 227;
pub const CV_AMD64_EMM8H: CV_HREG_e = 316;
pub const CV_AMD64_EMM8L: CV_HREG_e = 308;
pub const CV_AMD64_EMM9H: CV_HREG_e = 317;
pub const CV_AMD64_EMM9L: CV_HREG_e = 309;
pub const CV_AMD64_ES: CV_HREG_e = 25;
pub const CV_AMD64_ESI: CV_HREG_e = 23;
pub const CV_AMD64_ESP: CV_HREG_e = 21;
pub const CV_AMD64_FLAGS: CV_HREG_e = 32;
pub const CV_AMD64_FPCS: CV_HREG_e = 140;
pub const CV_AMD64_FPDO: CV_HREG_e = 141;
pub const CV_AMD64_FPDS: CV_HREG_e = 142;
pub const CV_AMD64_FPEDO: CV_HREG_e = 145;
pub const CV_AMD64_FPEIP: CV_HREG_e = 144;
pub const CV_AMD64_FPIP: CV_HREG_e = 139;
pub const CV_AMD64_FS: CV_HREG_e = 29;
pub const CV_AMD64_GDTL: CV_HREG_e = 111;
pub const CV_AMD64_GDTR: CV_HREG_e = 110;
pub const CV_AMD64_GS: CV_HREG_e = 30;
pub const CV_AMD64_IDTL: CV_HREG_e = 113;
pub const CV_AMD64_IDTR: CV_HREG_e = 112;
pub const CV_AMD64_ISEM: CV_HREG_e = 143;
pub const CV_AMD64_K0: CV_HREG_e = 758;
pub const CV_AMD64_K1: CV_HREG_e = 759;
pub const CV_AMD64_K2: CV_HREG_e = 760;
pub const CV_AMD64_K3: CV_HREG_e = 761;
pub const CV_AMD64_K4: CV_HREG_e = 762;
pub const CV_AMD64_K5: CV_HREG_e = 763;
pub const CV_AMD64_K6: CV_HREG_e = 764;
pub const CV_AMD64_K7: CV_HREG_e = 765;
pub const CV_AMD64_LDTR: CV_HREG_e = 114;
pub const CV_AMD64_MM0: CV_HREG_e = 146;
pub const CV_AMD64_MM00: CV_HREG_e = 236;
pub const CV_AMD64_MM01: CV_HREG_e = 237;
pub const CV_AMD64_MM1: CV_HREG_e = 147;
pub const CV_AMD64_MM10: CV_HREG_e = 238;
pub const CV_AMD64_MM11: CV_HREG_e = 239;
pub const CV_AMD64_MM2: CV_HREG_e = 148;
pub const CV_AMD64_MM20: CV_HREG_e = 240;
pub const CV_AMD64_MM21: CV_HREG_e = 241;
pub const CV_AMD64_MM3: CV_HREG_e = 149;
pub const CV_AMD64_MM30: CV_HREG_e = 242;
pub const CV_AMD64_MM31: CV_HREG_e = 243;
pub const CV_AMD64_MM4: CV_HREG_e = 150;
pub const CV_AMD64_MM40: CV_HREG_e = 244;
pub const CV_AMD64_MM41: CV_HREG_e = 245;
pub const CV_AMD64_MM5: CV_HREG_e = 151;
pub const CV_AMD64_MM50: CV_HREG_e = 246;
pub const CV_AMD64_MM51: CV_HREG_e = 247;
pub const CV_AMD64_MM6: CV_HREG_e = 152;
pub const CV_AMD64_MM60: CV_HREG_e = 248;
pub const CV_AMD64_MM61: CV_HREG_e = 249;
pub const CV_AMD64_MM7: CV_HREG_e = 153;
pub const CV_AMD64_MM70: CV_HREG_e = 250;
pub const CV_AMD64_MM71: CV_HREG_e = 251;
pub const CV_AMD64_MXCSR: CV_HREG_e = 211;
pub const CV_AMD64_R10: CV_HREG_e = 338;
pub const CV_AMD64_R10B: CV_HREG_e = 346;
pub const CV_AMD64_R10D: CV_HREG_e = 362;
pub const CV_AMD64_R10W: CV_HREG_e = 354;
pub const CV_AMD64_R11: CV_HREG_e = 339;
pub const CV_AMD64_R11B: CV_HREG_e = 347;
pub const CV_AMD64_R11D: CV_HREG_e = 363;
pub const CV_AMD64_R11W: CV_HREG_e = 355;
pub const CV_AMD64_R12: CV_HREG_e = 340;
pub const CV_AMD64_R12B: CV_HREG_e = 348;
pub const CV_AMD64_R12D: CV_HREG_e = 364;
pub const CV_AMD64_R12W: CV_HREG_e = 356;
pub const CV_AMD64_R13: CV_HREG_e = 341;
pub const CV_AMD64_R13B: CV_HREG_e = 349;
pub const CV_AMD64_R13D: CV_HREG_e = 365;
pub const CV_AMD64_R13W: CV_HREG_e = 357;
pub const CV_AMD64_R14: CV_HREG_e = 342;
pub const CV_AMD64_R14B: CV_HREG_e = 350;
pub const CV_AMD64_R14D: CV_HREG_e = 366;
pub const CV_AMD64_R14W: CV_HREG_e = 358;
pub const CV_AMD64_R15: CV_HREG_e = 343;
pub const CV_AMD64_R15B: CV_HREG_e = 351;
pub const CV_AMD64_R15D: CV_HREG_e = 367;
pub const CV_AMD64_R15W: CV_HREG_e = 359;
pub const CV_AMD64_R16: CV_HREG_e = 856;
pub const CV_AMD64_R16B: CV_HREG_e = 872;
pub const CV_AMD64_R16D: CV_HREG_e = 904;
pub const CV_AMD64_R16W: CV_HREG_e = 888;
pub const CV_AMD64_R17: CV_HREG_e = 857;
pub const CV_AMD64_R17B: CV_HREG_e = 873;
pub const CV_AMD64_R17D: CV_HREG_e = 905;
pub const CV_AMD64_R17W: CV_HREG_e = 889;
pub const CV_AMD64_R18: CV_HREG_e = 858;
pub const CV_AMD64_R18B: CV_HREG_e = 874;
pub const CV_AMD64_R18D: CV_HREG_e = 906;
pub const CV_AMD64_R18W: CV_HREG_e = 890;
pub const CV_AMD64_R19: CV_HREG_e = 859;
pub const CV_AMD64_R19B: CV_HREG_e = 875;
pub const CV_AMD64_R19D: CV_HREG_e = 907;
pub const CV_AMD64_R19W: CV_HREG_e = 891;
pub const CV_AMD64_R20: CV_HREG_e = 860;
pub const CV_AMD64_R20B: CV_HREG_e = 876;
pub const CV_AMD64_R20D: CV_HREG_e = 908;
pub const CV_AMD64_R20W: CV_HREG_e = 892;
pub const CV_AMD64_R21: CV_HREG_e = 861;
pub const CV_AMD64_R21B: CV_HREG_e = 877;
pub const CV_AMD64_R21D: CV_HREG_e = 909;
pub const CV_AMD64_R21W: CV_HREG_e = 893;
pub const CV_AMD64_R22: CV_HREG_e = 862;
pub const CV_AMD64_R22B: CV_HREG_e = 878;
pub const CV_AMD64_R22D: CV_HREG_e = 910;
pub const CV_AMD64_R22W: CV_HREG_e = 894;
pub const CV_AMD64_R23: CV_HREG_e = 863;
pub const CV_AMD64_R23B: CV_HREG_e = 879;
pub const CV_AMD64_R23D: CV_HREG_e = 911;
pub const CV_AMD64_R23W: CV_HREG_e = 895;
pub const CV_AMD64_R24: CV_HREG_e = 864;
pub const CV_AMD64_R24B: CV_HREG_e = 880;
pub const CV_AMD64_R24D: CV_HREG_e = 912;
pub const CV_AMD64_R24W: CV_HREG_e = 896;
pub const CV_AMD64_R25: CV_HREG_e = 865;
pub const CV_AMD64_R25B: CV_HREG_e = 881;
pub const CV_AMD64_R25D: CV_HREG_e = 913;
pub const CV_AMD64_R25W: CV_HREG_e = 897;
pub const CV_AMD64_R26: CV_HREG_e = 866;
pub const CV_AMD64_R26B: CV_HREG_e = 882;
pub const CV_AMD64_R26D: CV_HREG_e = 914;
pub const CV_AMD64_R26W: CV_HREG_e = 898;
pub const CV_AMD64_R27: CV_HREG_e = 867;
pub const CV_AMD64_R27B: CV_HREG_e = 883;
pub const CV_AMD64_R27D: CV_HREG_e = 915;
pub const CV_AMD64_R27W: CV_HREG_e = 899;
pub const CV_AMD64_R28: CV_HREG_e = 868;
pub const CV_AMD64_R28B: CV_HREG_e = 884;
pub const CV_AMD64_R28D: CV_HREG_e = 916;
pub const CV_AMD64_R28W: CV_HREG_e = 900;
pub const CV_AMD64_R29: CV_HREG_e = 869;
pub const CV_AMD64_R29B: CV_HREG_e = 885;
pub const CV_AMD64_R29D: CV_HREG_e = 917;
pub const CV_AMD64_R29W: CV_HREG_e = 901;
pub const CV_AMD64_R30: CV_HREG_e = 870;
pub const CV_AMD64_R30B: CV_HREG_e = 886;
pub const CV_AMD64_R30D: CV_HREG_e = 918;
pub const CV_AMD64_R30W: CV_HREG_e = 902;
pub const CV_AMD64_R31: CV_HREG_e = 871;
pub const CV_AMD64_R31B: CV_HREG_e = 887;
pub const CV_AMD64_R31D: CV_HREG_e = 919;
pub const CV_AMD64_R31W: CV_HREG_e = 903;
pub const CV_AMD64_R8: CV_HREG_e = 336;
pub const CV_AMD64_R8B: CV_HREG_e = 344;
pub const CV_AMD64_R8D: CV_HREG_e = 360;
pub const CV_AMD64_R8W: CV_HREG_e = 352;
pub const CV_AMD64_R9: CV_HREG_e = 337;
pub const CV_AMD64_R9B: CV_HREG_e = 345;
pub const CV_AMD64_R9D: CV_HREG_e = 361;
pub const CV_AMD64_R9W: CV_HREG_e = 353;
pub const CV_AMD64_RAX: CV_HREG_e = 328;
pub const CV_AMD64_RBP: CV_HREG_e = 334;
pub const CV_AMD64_RBX: CV_HREG_e = 329;
pub const CV_AMD64_RCX: CV_HREG_e = 330;
pub const CV_AMD64_RDI: CV_HREG_e = 333;
pub const CV_AMD64_RDX: CV_HREG_e = 331;
pub const CV_AMD64_RIP: CV_HREG_e = 33;
pub const CV_AMD64_RSI: CV_HREG_e = 332;
pub const CV_AMD64_RSP: CV_HREG_e = 335;
pub const CV_AMD64_SI: CV_HREG_e = 15;
pub const CV_AMD64_SIL: CV_HREG_e = 324;
pub const CV_AMD64_SP: CV_HREG_e = 13;
pub const CV_AMD64_SPL: CV_HREG_e = 327;
pub const CV_AMD64_SS: CV_HREG_e = 27;
pub const CV_AMD64_SSP: CV_HREG_e = 846;
pub const CV_AMD64_ST0: CV_HREG_e = 128;
pub const CV_AMD64_ST1: CV_HREG_e = 129;
pub const CV_AMD64_ST2: CV_HREG_e = 130;
pub const CV_AMD64_ST3: CV_HREG_e = 131;
pub const CV_AMD64_ST4: CV_HREG_e = 132;
pub const CV_AMD64_ST5: CV_HREG_e = 133;
pub const CV_AMD64_ST6: CV_HREG_e = 134;
pub const CV_AMD64_ST7: CV_HREG_e = 135;
pub const CV_AMD64_STAT: CV_HREG_e = 137;
pub const CV_AMD64_TAG: CV_HREG_e = 138;
pub const CV_AMD64_TILECFG: CV_HREG_e = 855;
pub const CV_AMD64_TMM0: CV_HREG_e = 847;
pub const CV_AMD64_TMM1: CV_HREG_e = 848;
pub const CV_AMD64_TMM2: CV_HREG_e = 849;
pub const CV_AMD64_TMM3: CV_HREG_e = 850;
pub const CV_AMD64_TMM4: CV_HREG_e = 851;
pub const CV_AMD64_TMM5: CV_HREG_e = 852;
pub const CV_AMD64_TMM6: CV_HREG_e = 853;
pub const CV_AMD64_TMM7: CV_HREG_e = 854;
pub const CV_AMD64_TR: CV_HREG_e = 115;
pub const CV_AMD64_XMM0: CV_HREG_e = 154;
pub const CV_AMD64_XMM0H: CV_HREG_e = 202;
pub const CV_AMD64_XMM0IH: CV_HREG_e = 416;
pub const CV_AMD64_XMM0IL: CV_HREG_e = 400;
pub const CV_AMD64_XMM0L: CV_HREG_e = 194;
pub const CV_AMD64_XMM0_0: CV_HREG_e = 162;
pub const CV_AMD64_XMM0_1: CV_HREG_e = 163;
pub const CV_AMD64_XMM0_2: CV_HREG_e = 164;
pub const CV_AMD64_XMM0_3: CV_HREG_e = 165;
pub const CV_AMD64_XMM1: CV_HREG_e = 155;
pub const CV_AMD64_XMM10: CV_HREG_e = 254;
pub const CV_AMD64_XMM10H: CV_HREG_e = 302;
pub const CV_AMD64_XMM10IH: CV_HREG_e = 426;
pub const CV_AMD64_XMM10IL: CV_HREG_e = 410;
pub const CV_AMD64_XMM10L: CV_HREG_e = 294;
pub const CV_AMD64_XMM10_0: CV_HREG_e = 268;
pub const CV_AMD64_XMM10_1: CV_HREG_e = 269;
pub const CV_AMD64_XMM10_2: CV_HREG_e = 270;
pub const CV_AMD64_XMM10_3: CV_HREG_e = 271;
pub const CV_AMD64_XMM11: CV_HREG_e = 255;
pub const CV_AMD64_XMM11H: CV_HREG_e = 303;
pub const CV_AMD64_XMM11IH: CV_HREG_e = 427;
pub const CV_AMD64_XMM11IL: CV_HREG_e = 411;
pub const CV_AMD64_XMM11L: CV_HREG_e = 295;
pub const CV_AMD64_XMM11_0: CV_HREG_e = 272;
pub const CV_AMD64_XMM11_1: CV_HREG_e = 273;
pub const CV_AMD64_XMM11_2: CV_HREG_e = 274;
pub const CV_AMD64_XMM11_3: CV_HREG_e = 275;
pub const CV_AMD64_XMM12: CV_HREG_e = 256;
pub const CV_AMD64_XMM12H: CV_HREG_e = 304;
pub const CV_AMD64_XMM12IH: CV_HREG_e = 428;
pub const CV_AMD64_XMM12IL: CV_HREG_e = 412;
pub const CV_AMD64_XMM12L: CV_HREG_e = 296;
pub const CV_AMD64_XMM12_0: CV_HREG_e = 276;
pub const CV_AMD64_XMM12_1: CV_HREG_e = 277;
pub const CV_AMD64_XMM12_2: CV_HREG_e = 278;
pub const CV_AMD64_XMM12_3: CV_HREG_e = 279;
pub const CV_AMD64_XMM13: CV_HREG_e = 257;
pub const CV_AMD64_XMM13H: CV_HREG_e = 305;
pub const CV_AMD64_XMM13IH: CV_HREG_e = 429;
pub const CV_AMD64_XMM13IL: CV_HREG_e = 413;
pub const CV_AMD64_XMM13L: CV_HREG_e = 297;
pub const CV_AMD64_XMM13_0: CV_HREG_e = 280;
pub const CV_AMD64_XMM13_1: CV_HREG_e = 281;
pub const CV_AMD64_XMM13_2: CV_HREG_e = 282;
pub const CV_AMD64_XMM13_3: CV_HREG_e = 283;
pub const CV_AMD64_XMM14: CV_HREG_e = 258;
pub const CV_AMD64_XMM14H: CV_HREG_e = 306;
pub const CV_AMD64_XMM14IH: CV_HREG_e = 430;
pub const CV_AMD64_XMM14IL: CV_HREG_e = 414;
pub const CV_AMD64_XMM14L: CV_HREG_e = 298;
pub const CV_AMD64_XMM14_0: CV_HREG_e = 284;
pub const CV_AMD64_XMM14_1: CV_HREG_e = 285;
pub const CV_AMD64_XMM14_2: CV_HREG_e = 286;
pub const CV_AMD64_XMM14_3: CV_HREG_e = 287;
pub const CV_AMD64_XMM15: CV_HREG_e = 259;
pub const CV_AMD64_XMM15H: CV_HREG_e = 307;
pub const CV_AMD64_XMM15IH: CV_HREG_e = 431;
pub const CV_AMD64_XMM15IL: CV_HREG_e = 415;
pub const CV_AMD64_XMM15L: CV_HREG_e = 299;
pub const CV_AMD64_XMM15_0: CV_HREG_e = 288;
pub const CV_AMD64_XMM15_1: CV_HREG_e = 289;
pub const CV_AMD64_XMM15_2: CV_HREG_e = 290;
pub const CV_AMD64_XMM15_3: CV_HREG_e = 291;
pub const CV_AMD64_XMM16: CV_HREG_e = 694;
pub const CV_AMD64_XMM16H: CV_HREG_e = 814;
pub const CV_AMD64_XMM16L: CV_HREG_e = 782;
pub const CV_AMD64_XMM16_0: CV_HREG_e = 798;
pub const CV_AMD64_XMM17: CV_HREG_e = 695;
pub const CV_AMD64_XMM17H: CV_HREG_e = 815;
pub const CV_AMD64_XMM17L: CV_HREG_e = 783;
pub const CV_AMD64_XMM17_0: CV_HREG_e = 799;
pub const CV_AMD64_XMM18: CV_HREG_e = 696;
pub const CV_AMD64_XMM18H: CV_HREG_e = 816;
pub const CV_AMD64_XMM18L: CV_HREG_e = 784;
pub const CV_AMD64_XMM18_0: CV_HREG_e = 800;
pub const CV_AMD64_XMM19: CV_HREG_e = 697;
pub const CV_AMD64_XMM19H: CV_HREG_e = 817;
pub const CV_AMD64_XMM19L: CV_HREG_e = 785;
pub const CV_AMD64_XMM19_0: CV_HREG_e = 801;
pub const CV_AMD64_XMM1H: CV_HREG_e = 203;
pub const CV_AMD64_XMM1IH: CV_HREG_e = 417;
pub const CV_AMD64_XMM1IL: CV_HREG_e = 401;
pub const CV_AMD64_XMM1L: CV_HREG_e = 195;
pub const CV_AMD64_XMM1_0: CV_HREG_e = 166;
pub const CV_AMD64_XMM1_1: CV_HREG_e = 167;
pub const CV_AMD64_XMM1_2: CV_HREG_e = 168;
pub const CV_AMD64_XMM1_3: CV_HREG_e = 169;
pub const CV_AMD64_XMM2: CV_HREG_e = 156;
pub const CV_AMD64_XMM20: CV_HREG_e = 698;
pub const CV_AMD64_XMM20H: CV_HREG_e = 818;
pub const CV_AMD64_XMM20L: CV_HREG_e = 786;
pub const CV_AMD64_XMM20_0: CV_HREG_e = 802;
pub const CV_AMD64_XMM21: CV_HREG_e = 699;
pub const CV_AMD64_XMM21H: CV_HREG_e = 819;
pub const CV_AMD64_XMM21L: CV_HREG_e = 787;
pub const CV_AMD64_XMM21_0: CV_HREG_e = 803;
pub const CV_AMD64_XMM22: CV_HREG_e = 700;
pub const CV_AMD64_XMM22H: CV_HREG_e = 820;
pub const CV_AMD64_XMM22L: CV_HREG_e = 788;
pub const CV_AMD64_XMM22_0: CV_HREG_e = 804;
pub const CV_AMD64_XMM23: CV_HREG_e = 701;
pub const CV_AMD64_XMM23H: CV_HREG_e = 821;
pub const CV_AMD64_XMM23L: CV_HREG_e = 789;
pub const CV_AMD64_XMM23_0: CV_HREG_e = 805;
pub const CV_AMD64_XMM24: CV_HREG_e = 702;
pub const CV_AMD64_XMM24H: CV_HREG_e = 822;
pub const CV_AMD64_XMM24L: CV_HREG_e = 790;
pub const CV_AMD64_XMM24_0: CV_HREG_e = 806;
pub const CV_AMD64_XMM25: CV_HREG_e = 703;
pub const CV_AMD64_XMM25H: CV_HREG_e = 823;
pub const CV_AMD64_XMM25L: CV_HREG_e = 791;
pub const CV_AMD64_XMM25_0: CV_HREG_e = 807;
pub const CV_AMD64_XMM26: CV_HREG_e = 704;
pub const CV_AMD64_XMM26H: CV_HREG_e = 824;
pub const CV_AMD64_XMM26L: CV_HREG_e = 792;
pub const CV_AMD64_XMM26_0: CV_HREG_e = 808;
pub const CV_AMD64_XMM27: CV_HREG_e = 705;
pub const CV_AMD64_XMM27H: CV_HREG_e = 825;
pub const CV_AMD64_XMM27L: CV_HREG_e = 793;
pub const CV_AMD64_XMM27_0: CV_HREG_e = 809;
pub const CV_AMD64_XMM28: CV_HREG_e = 706;
pub const CV_AMD64_XMM28H: CV_HREG_e = 826;
pub const CV_AMD64_XMM28L: CV_HREG_e = 794;
pub const CV_AMD64_XMM28_0: CV_HREG_e = 810;
pub const CV_AMD64_XMM29: CV_HREG_e = 707;
pub const CV_AMD64_XMM29H: CV_HREG_e = 827;
pub const CV_AMD64_XMM29L: CV_HREG_e = 795;
pub const CV_AMD64_XMM29_0: CV_HREG_e = 811;
pub const CV_AMD64_XMM2H: CV_HREG_e = 204;
pub const CV_AMD64_XMM2IH: CV_HREG_e = 418;
pub const CV_AMD64_XMM2IL: CV_HREG_e = 402;
pub const CV_AMD64_XMM2L: CV_HREG_e = 196;
pub const CV_AMD64_XMM2_0: CV_HREG_e = 170;
pub const CV_AMD64_XMM2_1: CV_HREG_e = 171;
pub const CV_AMD64_XMM2_2: CV_HREG_e = 172;
pub const CV_AMD64_XMM2_3: CV_HREG_e = 173;
pub const CV_AMD64_XMM3: CV_HREG_e = 157;
pub const CV_AMD64_XMM30: CV_HREG_e = 708;
pub const CV_AMD64_XMM30H: CV_HREG_e = 828;
pub const CV_AMD64_XMM30L: CV_HREG_e = 796;
pub const CV_AMD64_XMM30_0: CV_HREG_e = 812;
pub const CV_AMD64_XMM31: CV_HREG_e = 709;
pub const CV_AMD64_XMM31H: CV_HREG_e = 829;
pub const CV_AMD64_XMM31L: CV_HREG_e = 797;
pub const CV_AMD64_XMM31_0: CV_HREG_e = 813;
pub const CV_AMD64_XMM3H: CV_HREG_e = 205;
pub const CV_AMD64_XMM3IH: CV_HREG_e = 419;
pub const CV_AMD64_XMM3IL: CV_HREG_e = 403;
pub const CV_AMD64_XMM3L: CV_HREG_e = 197;
pub const CV_AMD64_XMM3_0: CV_HREG_e = 174;
pub const CV_AMD64_XMM3_1: CV_HREG_e = 175;
pub const CV_AMD64_XMM3_2: CV_HREG_e = 176;
pub const CV_AMD64_XMM3_3: CV_HREG_e = 177;
pub const CV_AMD64_XMM4: CV_HREG_e = 158;
pub const CV_AMD64_XMM4H: CV_HREG_e = 206;
pub const CV_AMD64_XMM4IH: CV_HREG_e = 420;
pub const CV_AMD64_XMM4IL: CV_HREG_e = 404;
pub const CV_AMD64_XMM4L: CV_HREG_e = 198;
pub const CV_AMD64_XMM4_0: CV_HREG_e = 178;
pub const CV_AMD64_XMM4_1: CV_HREG_e = 179;
pub const CV_AMD64_XMM4_2: CV_HREG_e = 180;
pub const CV_AMD64_XMM4_3: CV_HREG_e = 181;
pub const CV_AMD64_XMM5: CV_HREG_e = 159;
pub const CV_AMD64_XMM5H: CV_HREG_e = 207;
pub const CV_AMD64_XMM5IH: CV_HREG_e = 421;
pub const CV_AMD64_XMM5IL: CV_HREG_e = 405;
pub const CV_AMD64_XMM5L: CV_HREG_e = 199;
pub const CV_AMD64_XMM5_0: CV_HREG_e = 182;
pub const CV_AMD64_XMM5_1: CV_HREG_e = 183;
pub const CV_AMD64_XMM5_2: CV_HREG_e = 184;
pub const CV_AMD64_XMM5_3: CV_HREG_e = 185;
pub const CV_AMD64_XMM6: CV_HREG_e = 160;
pub const CV_AMD64_XMM6H: CV_HREG_e = 208;
pub const CV_AMD64_XMM6IH: CV_HREG_e = 422;
pub const CV_AMD64_XMM6IL: CV_HREG_e = 406;
pub const CV_AMD64_XMM6L: CV_HREG_e = 200;
pub const CV_AMD64_XMM6_0: CV_HREG_e = 186;
pub const CV_AMD64_XMM6_1: CV_HREG_e = 187;
pub const CV_AMD64_XMM6_2: CV_HREG_e = 188;
pub const CV_AMD64_XMM6_3: CV_HREG_e = 189;
pub const CV_AMD64_XMM7: CV_HREG_e = 161;
pub const CV_AMD64_XMM7H: CV_HREG_e = 209;
pub const CV_AMD64_XMM7IH: CV_HREG_e = 423;
pub const CV_AMD64_XMM7IL: CV_HREG_e = 407;
pub const CV_AMD64_XMM7L: CV_HREG_e = 201;
pub const CV_AMD64_XMM7_0: CV_HREG_e = 190;
pub const CV_AMD64_XMM7_1: CV_HREG_e = 191;
pub const CV_AMD64_XMM7_2: CV_HREG_e = 192;
pub const CV_AMD64_XMM7_3: CV_HREG_e = 193;
pub const CV_AMD64_XMM8: CV_HREG_e = 252;
pub const CV_AMD64_XMM8H: CV_HREG_e = 300;
pub const CV_AMD64_XMM8IH: CV_HREG_e = 424;
pub const CV_AMD64_XMM8IL: CV_HREG_e = 408;
pub const CV_AMD64_XMM8L: CV_HREG_e = 292;
pub const CV_AMD64_XMM8_0: CV_HREG_e = 260;
pub const CV_AMD64_XMM8_1: CV_HREG_e = 261;
pub const CV_AMD64_XMM8_2: CV_HREG_e = 262;
pub const CV_AMD64_XMM8_3: CV_HREG_e = 263;
pub const CV_AMD64_XMM9: CV_HREG_e = 253;
pub const CV_AMD64_XMM9H: CV_HREG_e = 301;
pub const CV_AMD64_XMM9IH: CV_HREG_e = 425;
pub const CV_AMD64_XMM9IL: CV_HREG_e = 409;
pub const CV_AMD64_XMM9L: CV_HREG_e = 293;
pub const CV_AMD64_XMM9_0: CV_HREG_e = 264;
pub const CV_AMD64_XMM9_1: CV_HREG_e = 265;
pub const CV_AMD64_XMM9_2: CV_HREG_e = 266;
pub const CV_AMD64_XMM9_3: CV_HREG_e = 267;
pub const CV_AMD64_YMM0: CV_HREG_e = 368;
pub const CV_AMD64_YMM0D0: CV_HREG_e = 624;
pub const CV_AMD64_YMM0D1: CV_HREG_e = 625;
pub const CV_AMD64_YMM0D2: CV_HREG_e = 626;
pub const CV_AMD64_YMM0D3: CV_HREG_e = 627;
pub const CV_AMD64_YMM0F0: CV_HREG_e = 496;
pub const CV_AMD64_YMM0F1: CV_HREG_e = 497;
pub const CV_AMD64_YMM0F2: CV_HREG_e = 498;
pub const CV_AMD64_YMM0F3: CV_HREG_e = 499;
pub const CV_AMD64_YMM0F4: CV_HREG_e = 500;
pub const CV_AMD64_YMM0F5: CV_HREG_e = 501;
pub const CV_AMD64_YMM0F6: CV_HREG_e = 502;
pub const CV_AMD64_YMM0F7: CV_HREG_e = 503;
pub const CV_AMD64_YMM0H: CV_HREG_e = 384;
pub const CV_AMD64_YMM0I0: CV_HREG_e = 432;
pub const CV_AMD64_YMM0I1: CV_HREG_e = 433;
pub const CV_AMD64_YMM0I2: CV_HREG_e = 434;
pub const CV_AMD64_YMM0I3: CV_HREG_e = 435;
pub const CV_AMD64_YMM1: CV_HREG_e = 369;
pub const CV_AMD64_YMM10: CV_HREG_e = 378;
pub const CV_AMD64_YMM10D0: CV_HREG_e = 664;
pub const CV_AMD64_YMM10D1: CV_HREG_e = 665;
pub const CV_AMD64_YMM10D2: CV_HREG_e = 666;
pub const CV_AMD64_YMM10D3: CV_HREG_e = 667;
pub const CV_AMD64_YMM10F0: CV_HREG_e = 576;
pub const CV_AMD64_YMM10F1: CV_HREG_e = 577;
pub const CV_AMD64_YMM10F2: CV_HREG_e = 578;
pub const CV_AMD64_YMM10F3: CV_HREG_e = 579;
pub const CV_AMD64_YMM10F4: CV_HREG_e = 580;
pub const CV_AMD64_YMM10F5: CV_HREG_e = 581;
pub const CV_AMD64_YMM10F6: CV_HREG_e = 582;
pub const CV_AMD64_YMM10F7: CV_HREG_e = 583;
pub const CV_AMD64_YMM10H: CV_HREG_e = 394;
pub const CV_AMD64_YMM10I0: CV_HREG_e = 472;
pub const CV_AMD64_YMM10I1: CV_HREG_e = 473;
pub const CV_AMD64_YMM10I2: CV_HREG_e = 474;
pub const CV_AMD64_YMM10I3: CV_HREG_e = 475;
pub const CV_AMD64_YMM11: CV_HREG_e = 379;
pub const CV_AMD64_YMM11D0: CV_HREG_e = 668;
pub const CV_AMD64_YMM11D1: CV_HREG_e = 669;
pub const CV_AMD64_YMM11D2: CV_HREG_e = 670;
pub const CV_AMD64_YMM11D3: CV_HREG_e = 671;
pub const CV_AMD64_YMM11F0: CV_HREG_e = 584;
pub const CV_AMD64_YMM11F1: CV_HREG_e = 585;
pub const CV_AMD64_YMM11F2: CV_HREG_e = 586;
pub const CV_AMD64_YMM11F3: CV_HREG_e = 587;
pub const CV_AMD64_YMM11F4: CV_HREG_e = 588;
pub const CV_AMD64_YMM11F5: CV_HREG_e = 589;
pub const CV_AMD64_YMM11F6: CV_HREG_e = 590;
pub const CV_AMD64_YMM11F7: CV_HREG_e = 591;
pub const CV_AMD64_YMM11H: CV_HREG_e = 395;
pub const CV_AMD64_YMM11I0: CV_HREG_e = 476;
pub const CV_AMD64_YMM11I1: CV_HREG_e = 477;
pub const CV_AMD64_YMM11I2: CV_HREG_e = 478;
pub const CV_AMD64_YMM11I3: CV_HREG_e = 479;
pub const CV_AMD64_YMM12: CV_HREG_e = 380;
pub const CV_AMD64_YMM12D0: CV_HREG_e = 672;
pub const CV_AMD64_YMM12D1: CV_HREG_e = 673;
pub const CV_AMD64_YMM12D2: CV_HREG_e = 674;
pub const CV_AMD64_YMM12D3: CV_HREG_e = 675;
pub const CV_AMD64_YMM12F0: CV_HREG_e = 592;
pub const CV_AMD64_YMM12F1: CV_HREG_e = 593;
pub const CV_AMD64_YMM12F2: CV_HREG_e = 594;
pub const CV_AMD64_YMM12F3: CV_HREG_e = 595;
pub const CV_AMD64_YMM12F4: CV_HREG_e = 596;
pub const CV_AMD64_YMM12F5: CV_HREG_e = 597;
pub const CV_AMD64_YMM12F6: CV_HREG_e = 598;
pub const CV_AMD64_YMM12F7: CV_HREG_e = 599;
pub const CV_AMD64_YMM12H: CV_HREG_e = 396;
pub const CV_AMD64_YMM12I0: CV_HREG_e = 480;
pub const CV_AMD64_YMM12I1: CV_HREG_e = 481;
pub const CV_AMD64_YMM12I2: CV_HREG_e = 482;
pub const CV_AMD64_YMM12I3: CV_HREG_e = 483;
pub const CV_AMD64_YMM13: CV_HREG_e = 381;
pub const CV_AMD64_YMM13D0: CV_HREG_e = 676;
pub const CV_AMD64_YMM13D1: CV_HREG_e = 677;
pub const CV_AMD64_YMM13D2: CV_HREG_e = 678;
pub const CV_AMD64_YMM13D3: CV_HREG_e = 679;
pub const CV_AMD64_YMM13F0: CV_HREG_e = 600;
pub const CV_AMD64_YMM13F1: CV_HREG_e = 601;
pub const CV_AMD64_YMM13F2: CV_HREG_e = 602;
pub const CV_AMD64_YMM13F3: CV_HREG_e = 603;
pub const CV_AMD64_YMM13F4: CV_HREG_e = 604;
pub const CV_AMD64_YMM13F5: CV_HREG_e = 605;
pub const CV_AMD64_YMM13F6: CV_HREG_e = 606;
pub const CV_AMD64_YMM13F7: CV_HREG_e = 607;
pub const CV_AMD64_YMM13H: CV_HREG_e = 397;
pub const CV_AMD64_YMM13I0: CV_HREG_e = 484;
pub const CV_AMD64_YMM13I1: CV_HREG_e = 485;
pub const CV_AMD64_YMM13I2: CV_HREG_e = 486;
pub const CV_AMD64_YMM13I3: CV_HREG_e = 487;
pub const CV_AMD64_YMM14: CV_HREG_e = 382;
pub const CV_AMD64_YMM14D0: CV_HREG_e = 680;
pub const CV_AMD64_YMM14D1: CV_HREG_e = 681;
pub const CV_AMD64_YMM14D2: CV_HREG_e = 682;
pub const CV_AMD64_YMM14D3: CV_HREG_e = 683;
pub const CV_AMD64_YMM14F0: CV_HREG_e = 608;
pub const CV_AMD64_YMM14F1: CV_HREG_e = 609;
pub const CV_AMD64_YMM14F2: CV_HREG_e = 610;
pub const CV_AMD64_YMM14F3: CV_HREG_e = 611;
pub const CV_AMD64_YMM14F4: CV_HREG_e = 612;
pub const CV_AMD64_YMM14F5: CV_HREG_e = 613;
pub const CV_AMD64_YMM14F6: CV_HREG_e = 614;
pub const CV_AMD64_YMM14F7: CV_HREG_e = 615;
pub const CV_AMD64_YMM14H: CV_HREG_e = 398;
pub const CV_AMD64_YMM14I0: CV_HREG_e = 488;
pub const CV_AMD64_YMM14I1: CV_HREG_e = 489;
pub const CV_AMD64_YMM14I2: CV_HREG_e = 490;
pub const CV_AMD64_YMM14I3: CV_HREG_e = 491;
pub const CV_AMD64_YMM15: CV_HREG_e = 383;
pub const CV_AMD64_YMM15D0: CV_HREG_e = 684;
pub const CV_AMD64_YMM15D1: CV_HREG_e = 685;
pub const CV_AMD64_YMM15D2: CV_HREG_e = 686;
pub const CV_AMD64_YMM15D3: CV_HREG_e = 687;
pub const CV_AMD64_YMM15F0: CV_HREG_e = 616;
pub const CV_AMD64_YMM15F1: CV_HREG_e = 617;
pub const CV_AMD64_YMM15F2: CV_HREG_e = 618;
pub const CV_AMD64_YMM15F3: CV_HREG_e = 619;
pub const CV_AMD64_YMM15F4: CV_HREG_e = 620;
pub const CV_AMD64_YMM15F5: CV_HREG_e = 621;
pub const CV_AMD64_YMM15F6: CV_HREG_e = 622;
pub const CV_AMD64_YMM15F7: CV_HREG_e = 623;
pub const CV_AMD64_YMM15H: CV_HREG_e = 399;
pub const CV_AMD64_YMM15I0: CV_HREG_e = 492;
pub const CV_AMD64_YMM15I1: CV_HREG_e = 493;
pub const CV_AMD64_YMM15I2: CV_HREG_e = 494;
pub const CV_AMD64_YMM15I3: CV_HREG_e = 495;
pub const CV_AMD64_YMM16: CV_HREG_e = 710;
pub const CV_AMD64_YMM17: CV_HREG_e = 711;
pub const CV_AMD64_YMM18: CV_HREG_e = 712;
pub const CV_AMD64_YMM19: CV_HREG_e = 713;
pub const CV_AMD64_YMM1D0: CV_HREG_e = 628;
pub const CV_AMD64_YMM1D1: CV_HREG_e = 629;
pub const CV_AMD64_YMM1D2: CV_HREG_e = 630;
pub const CV_AMD64_YMM1D3: CV_HREG_e = 631;
pub const CV_AMD64_YMM1F0: CV_HREG_e = 504;
pub const CV_AMD64_YMM1F1: CV_HREG_e = 505;
pub const CV_AMD64_YMM1F2: CV_HREG_e = 506;
pub const CV_AMD64_YMM1F3: CV_HREG_e = 507;
pub const CV_AMD64_YMM1F4: CV_HREG_e = 508;
pub const CV_AMD64_YMM1F5: CV_HREG_e = 509;
pub const CV_AMD64_YMM1F6: CV_HREG_e = 510;
pub const CV_AMD64_YMM1F7: CV_HREG_e = 511;
pub const CV_AMD64_YMM1H: CV_HREG_e = 385;
pub const CV_AMD64_YMM1I0: CV_HREG_e = 436;
pub const CV_AMD64_YMM1I1: CV_HREG_e = 437;
pub const CV_AMD64_YMM1I2: CV_HREG_e = 438;
pub const CV_AMD64_YMM1I3: CV_HREG_e = 439;
pub const CV_AMD64_YMM2: CV_HREG_e = 370;
pub const CV_AMD64_YMM20: CV_HREG_e = 714;
pub const CV_AMD64_YMM21: CV_HREG_e = 715;
pub const CV_AMD64_YMM22: CV_HREG_e = 716;
pub const CV_AMD64_YMM23: CV_HREG_e = 717;
pub const CV_AMD64_YMM24: CV_HREG_e = 718;
pub const CV_AMD64_YMM25: CV_HREG_e = 719;
pub const CV_AMD64_YMM26: CV_HREG_e = 720;
pub const CV_AMD64_YMM27: CV_HREG_e = 721;
pub const CV_AMD64_YMM28: CV_HREG_e = 722;
pub const CV_AMD64_YMM29: CV_HREG_e = 723;
pub const CV_AMD64_YMM2D0: CV_HREG_e = 632;
pub const CV_AMD64_YMM2D1: CV_HREG_e = 633;
pub const CV_AMD64_YMM2D2: CV_HREG_e = 634;
pub const CV_AMD64_YMM2D3: CV_HREG_e = 635;
pub const CV_AMD64_YMM2F0: CV_HREG_e = 512;
pub const CV_AMD64_YMM2F1: CV_HREG_e = 513;
pub const CV_AMD64_YMM2F2: CV_HREG_e = 514;
pub const CV_AMD64_YMM2F3: CV_HREG_e = 515;
pub const CV_AMD64_YMM2F4: CV_HREG_e = 516;
pub const CV_AMD64_YMM2F5: CV_HREG_e = 517;
pub const CV_AMD64_YMM2F6: CV_HREG_e = 518;
pub const CV_AMD64_YMM2F7: CV_HREG_e = 519;
pub const CV_AMD64_YMM2H: CV_HREG_e = 386;
pub const CV_AMD64_YMM2I0: CV_HREG_e = 440;
pub const CV_AMD64_YMM2I1: CV_HREG_e = 441;
pub const CV_AMD64_YMM2I2: CV_HREG_e = 442;
pub const CV_AMD64_YMM2I3: CV_HREG_e = 443;
pub const CV_AMD64_YMM3: CV_HREG_e = 371;
pub const CV_AMD64_YMM30: CV_HREG_e = 724;
pub const CV_AMD64_YMM31: CV_HREG_e = 725;
pub const CV_AMD64_YMM3D0: CV_HREG_e = 636;
pub const CV_AMD64_YMM3D1: CV_HREG_e = 637;
pub const CV_AMD64_YMM3D2: CV_HREG_e = 638;
pub const CV_AMD64_YMM3D3: CV_HREG_e = 639;
pub const CV_AMD64_YMM3F0: CV_HREG_e = 520;
pub const CV_AMD64_YMM3F1: CV_HREG_e = 521;
pub const CV_AMD64_YMM3F2: CV_HREG_e = 522;
pub const CV_AMD64_YMM3F3: CV_HREG_e = 523;
pub const CV_AMD64_YMM3F4: CV_HREG_e = 524;
pub const CV_AMD64_YMM3F5: CV_HREG_e = 525;
pub const CV_AMD64_YMM3F6: CV_HREG_e = 526;
pub const CV_AMD64_YMM3F7: CV_HREG_e = 527;
pub const CV_AMD64_YMM3H: CV_HREG_e = 387;
pub const CV_AMD64_YMM3I0: CV_HREG_e = 444;
pub const CV_AMD64_YMM3I1: CV_HREG_e = 445;
pub const CV_AMD64_YMM3I2: CV_HREG_e = 446;
pub const CV_AMD64_YMM3I3: CV_HREG_e = 447;
pub const CV_AMD64_YMM4: CV_HREG_e = 372;
pub const CV_AMD64_YMM4D0: CV_HREG_e = 640;
pub const CV_AMD64_YMM4D1: CV_HREG_e = 641;
pub const CV_AMD64_YMM4D2: CV_HREG_e = 642;
pub const CV_AMD64_YMM4D3: CV_HREG_e = 643;
pub const CV_AMD64_YMM4F0: CV_HREG_e = 528;
pub const CV_AMD64_YMM4F1: CV_HREG_e = 529;
pub const CV_AMD64_YMM4F2: CV_HREG_e = 530;
pub const CV_AMD64_YMM4F3: CV_HREG_e = 531;
pub const CV_AMD64_YMM4F4: CV_HREG_e = 532;
pub const CV_AMD64_YMM4F5: CV_HREG_e = 533;
pub const CV_AMD64_YMM4F6: CV_HREG_e = 534;
pub const CV_AMD64_YMM4F7: CV_HREG_e = 535;
pub const CV_AMD64_YMM4H: CV_HREG_e = 388;
pub const CV_AMD64_YMM4I0: CV_HREG_e = 448;
pub const CV_AMD64_YMM4I1: CV_HREG_e = 449;
pub const CV_AMD64_YMM4I2: CV_HREG_e = 450;
pub const CV_AMD64_YMM4I3: CV_HREG_e = 451;
pub const CV_AMD64_YMM5: CV_HREG_e = 373;
pub const CV_AMD64_YMM5D0: CV_HREG_e = 644;
pub const CV_AMD64_YMM5D1: CV_HREG_e = 645;
pub const CV_AMD64_YMM5D2: CV_HREG_e = 646;
pub const CV_AMD64_YMM5D3: CV_HREG_e = 647;
pub const CV_AMD64_YMM5F0: CV_HREG_e = 536;
pub const CV_AMD64_YMM5F1: CV_HREG_e = 537;
pub const CV_AMD64_YMM5F2: CV_HREG_e = 538;
pub const CV_AMD64_YMM5F3: CV_HREG_e = 539;
pub const CV_AMD64_YMM5F4: CV_HREG_e = 540;
pub const CV_AMD64_YMM5F5: CV_HREG_e = 541;
pub const CV_AMD64_YMM5F6: CV_HREG_e = 542;
pub const CV_AMD64_YMM5F7: CV_HREG_e = 543;
pub const CV_AMD64_YMM5H: CV_HREG_e = 389;
pub const CV_AMD64_YMM5I0: CV_HREG_e = 452;
pub const CV_AMD64_YMM5I1: CV_HREG_e = 453;
pub const CV_AMD64_YMM5I2: CV_HREG_e = 454;
pub const CV_AMD64_YMM5I3: CV_HREG_e = 455;
pub const CV_AMD64_YMM6: CV_HREG_e = 374;
pub const CV_AMD64_YMM6D0: CV_HREG_e = 648;
pub const CV_AMD64_YMM6D1: CV_HREG_e = 649;
pub const CV_AMD64_YMM6D2: CV_HREG_e = 650;
pub const CV_AMD64_YMM6D3: CV_HREG_e = 651;
pub const CV_AMD64_YMM6F0: CV_HREG_e = 544;
pub const CV_AMD64_YMM6F1: CV_HREG_e = 545;
pub const CV_AMD64_YMM6F2: CV_HREG_e = 546;
pub const CV_AMD64_YMM6F3: CV_HREG_e = 547;
pub const CV_AMD64_YMM6F4: CV_HREG_e = 548;
pub const CV_AMD64_YMM6F5: CV_HREG_e = 549;
pub const CV_AMD64_YMM6F6: CV_HREG_e = 550;
pub const CV_AMD64_YMM6F7: CV_HREG_e = 551;
pub const CV_AMD64_YMM6H: CV_HREG_e = 390;
pub const CV_AMD64_YMM6I0: CV_HREG_e = 456;
pub const CV_AMD64_YMM6I1: CV_HREG_e = 457;
pub const CV_AMD64_YMM6I2: CV_HREG_e = 458;
pub const CV_AMD64_YMM6I3: CV_HREG_e = 459;
pub const CV_AMD64_YMM7: CV_HREG_e = 375;
pub const CV_AMD64_YMM7D0: CV_HREG_e = 652;
pub const CV_AMD64_YMM7D1: CV_HREG_e = 653;
pub const CV_AMD64_YMM7D2: CV_HREG_e = 654;
pub const CV_AMD64_YMM7D3: CV_HREG_e = 655;
pub const CV_AMD64_YMM7F0: CV_HREG_e = 552;
pub const CV_AMD64_YMM7F1: CV_HREG_e = 553;
pub const CV_AMD64_YMM7F2: CV_HREG_e = 554;
pub const CV_AMD64_YMM7F3: CV_HREG_e = 555;
pub const CV_AMD64_YMM7F4: CV_HREG_e = 556;
pub const CV_AMD64_YMM7F5: CV_HREG_e = 557;
pub const CV_AMD64_YMM7F6: CV_HREG_e = 558;
pub const CV_AMD64_YMM7F7: CV_HREG_e = 559;
pub const CV_AMD64_YMM7H: CV_HREG_e = 391;
pub const CV_AMD64_YMM7I0: CV_HREG_e = 460;
pub const CV_AMD64_YMM7I1: CV_HREG_e = 461;
pub const CV_AMD64_YMM7I2: CV_HREG_e = 462;
pub const CV_AMD64_YMM7I3: CV_HREG_e = 463;
pub const CV_AMD64_YMM8: CV_HREG_e = 376;
pub const CV_AMD64_YMM8D0: CV_HREG_e = 656;
pub const CV_AMD64_YMM8D1: CV_HREG_e = 657;
pub const CV_AMD64_YMM8D2: CV_HREG_e = 658;
pub const CV_AMD64_YMM8D3: CV_HREG_e = 659;
pub const CV_AMD64_YMM8F0: CV_HREG_e = 560;
pub const CV_AMD64_YMM8F1: CV_HREG_e = 561;
pub const CV_AMD64_YMM8F2: CV_HREG_e = 562;
pub const CV_AMD64_YMM8F3: CV_HREG_e = 563;
pub const CV_AMD64_YMM8F4: CV_HREG_e = 564;
pub const CV_AMD64_YMM8F5: CV_HREG_e = 565;
pub const CV_AMD64_YMM8F6: CV_HREG_e = 566;
pub const CV_AMD64_YMM8F7: CV_HREG_e = 567;
pub const CV_AMD64_YMM8H: CV_HREG_e = 392;
pub const CV_AMD64_YMM8I0: CV_HREG_e = 464;
pub const CV_AMD64_YMM8I1: CV_HREG_e = 465;
pub const CV_AMD64_YMM8I2: CV_HREG_e = 466;
pub const CV_AMD64_YMM8I3: CV_HREG_e = 467;
pub const CV_AMD64_YMM9: CV_HREG_e = 377;
pub const CV_AMD64_YMM9D0: CV_HREG_e = 660;
pub const CV_AMD64_YMM9D1: CV_HREG_e = 661;
pub const CV_AMD64_YMM9D2: CV_HREG_e = 662;
pub const CV_AMD64_YMM9D3: CV_HREG_e = 663;
pub const CV_AMD64_YMM9F0: CV_HREG_e = 568;
pub const CV_AMD64_YMM9F1: CV_HREG_e = 569;
pub const CV_AMD64_YMM9F2: CV_HREG_e = 570;
pub const CV_AMD64_YMM9F3: CV_HREG_e = 571;
pub const CV_AMD64_YMM9F4: CV_HREG_e = 572;
pub const CV_AMD64_YMM9F5: CV_HREG_e = 573;
pub const CV_AMD64_YMM9F6: CV_HREG_e = 574;
pub const CV_AMD64_YMM9F7: CV_HREG_e = 575;
pub const CV_AMD64_YMM9H: CV_HREG_e = 393;
pub const CV_AMD64_YMM9I0: CV_HREG_e = 468;
pub const CV_AMD64_YMM9I1: CV_HREG_e = 469;
pub const CV_AMD64_YMM9I2: CV_HREG_e = 470;
pub const CV_AMD64_YMM9I3: CV_HREG_e = 471;
pub const CV_AMD64_ZMM0: CV_HREG_e = 726;
pub const CV_AMD64_ZMM0H: CV_HREG_e = 766;
pub const CV_AMD64_ZMM1: CV_HREG_e = 727;
pub const CV_AMD64_ZMM10: CV_HREG_e = 736;
pub const CV_AMD64_ZMM10H: CV_HREG_e = 776;
pub const CV_AMD64_ZMM11: CV_HREG_e = 737;
pub const CV_AMD64_ZMM11H: CV_HREG_e = 777;
pub const CV_AMD64_ZMM12: CV_HREG_e = 738;
pub const CV_AMD64_ZMM12H: CV_HREG_e = 778;
pub const CV_AMD64_ZMM13: CV_HREG_e = 739;
pub const CV_AMD64_ZMM13H: CV_HREG_e = 779;
pub const CV_AMD64_ZMM14: CV_HREG_e = 740;
pub const CV_AMD64_ZMM14H: CV_HREG_e = 780;
pub const CV_AMD64_ZMM15: CV_HREG_e = 741;
pub const CV_AMD64_ZMM15H: CV_HREG_e = 781;
pub const CV_AMD64_ZMM16: CV_HREG_e = 742;
pub const CV_AMD64_ZMM17: CV_HREG_e = 743;
pub const CV_AMD64_ZMM18: CV_HREG_e = 744;
pub const CV_AMD64_ZMM19: CV_HREG_e = 745;
pub const CV_AMD64_ZMM1H: CV_HREG_e = 767;
pub const CV_AMD64_ZMM2: CV_HREG_e = 728;
pub const CV_AMD64_ZMM20: CV_HREG_e = 746;
pub const CV_AMD64_ZMM21: CV_HREG_e = 747;
pub const CV_AMD64_ZMM22: CV_HREG_e = 748;
pub const CV_AMD64_ZMM23: CV_HREG_e = 749;
pub const CV_AMD64_ZMM24: CV_HREG_e = 750;
pub const CV_AMD64_ZMM25: CV_HREG_e = 751;
pub const CV_AMD64_ZMM26: CV_HREG_e = 752;
pub const CV_AMD64_ZMM27: CV_HREG_e = 753;
pub const CV_AMD64_ZMM28: CV_HREG_e = 754;
pub const CV_AMD64_ZMM29: CV_HREG_e = 755;
pub const CV_AMD64_ZMM2H: CV_HREG_e = 768;
pub const CV_AMD64_ZMM3: CV_HREG_e = 729;
pub const CV_AMD64_ZMM30: CV_HREG_e = 756;
pub const CV_AMD64_ZMM31: CV_HREG_e = 757;
pub const CV_AMD64_ZMM3H: CV_HREG_e = 769;
pub const CV_AMD64_ZMM4: CV_HREG_e = 730;
pub const CV_AMD64_ZMM4H: CV_HREG_e = 770;
pub const CV_AMD64_ZMM5: CV_HREG_e = 731;
pub const CV_AMD64_ZMM5H: CV_HREG_e = 771;
pub const CV_AMD64_ZMM6: CV_HREG_e = 732;
pub const CV_AMD64_ZMM6H: CV_HREG_e = 772;
pub const CV_AMD64_ZMM7: CV_HREG_e = 733;
pub const CV_AMD64_ZMM7H: CV_HREG_e = 773;
pub const CV_AMD64_ZMM8: CV_HREG_e = 734;
pub const CV_AMD64_ZMM8H: CV_HREG_e = 774;
pub const CV_AMD64_ZMM9: CV_HREG_e = 735;
pub const CV_AMD64_ZMM9H: CV_HREG_e = 775;
pub const CV_ARM64_B0: CV_HREG_e = 230;
pub const CV_ARM64_B1: CV_HREG_e = 231;
pub const CV_ARM64_B10: CV_HREG_e = 240;
pub const CV_ARM64_B11: CV_HREG_e = 241;
pub const CV_ARM64_B12: CV_HREG_e = 242;
pub const CV_ARM64_B13: CV_HREG_e = 243;
pub const CV_ARM64_B14: CV_HREG_e = 244;
pub const CV_ARM64_B15: CV_HREG_e = 245;
pub const CV_ARM64_B16: CV_HREG_e = 246;
pub const CV_ARM64_B17: CV_HREG_e = 247;
pub const CV_ARM64_B18: CV_HREG_e = 248;
pub const CV_ARM64_B19: CV_HREG_e = 249;
pub const CV_ARM64_B2: CV_HREG_e = 232;
pub const CV_ARM64_B20: CV_HREG_e = 250;
pub const CV_ARM64_B21: CV_HREG_e = 251;
pub const CV_ARM64_B22: CV_HREG_e = 252;
pub const CV_ARM64_B23: CV_HREG_e = 253;
pub const CV_ARM64_B24: CV_HREG_e = 254;
pub const CV_ARM64_B25: CV_HREG_e = 255;
pub const CV_ARM64_B26: CV_HREG_e = 256;
pub const CV_ARM64_B27: CV_HREG_e = 257;
pub const CV_ARM64_B28: CV_HREG_e = 258;
pub const CV_ARM64_B29: CV_HREG_e = 259;
pub const CV_ARM64_B3: CV_HREG_e = 233;
pub const CV_ARM64_B30: CV_HREG_e = 260;
pub const CV_ARM64_B31: CV_HREG_e = 261;
pub const CV_ARM64_B4: CV_HREG_e = 234;
pub const CV_ARM64_B5: CV_HREG_e = 235;
pub const CV_ARM64_B6: CV_HREG_e = 236;
pub const CV_ARM64_B7: CV_HREG_e = 237;
pub const CV_ARM64_B8: CV_HREG_e = 238;
pub const CV_ARM64_B9: CV_HREG_e = 239;
pub const CV_ARM64_CPSR: CV_HREG_e = 91;
pub const CV_ARM64_D0: CV_HREG_e = 140;
pub const CV_ARM64_D1: CV_HREG_e = 141;
pub const CV_ARM64_D10: CV_HREG_e = 150;
pub const CV_ARM64_D11: CV_HREG_e = 151;
pub const CV_ARM64_D12: CV_HREG_e = 152;
pub const CV_ARM64_D13: CV_HREG_e = 153;
pub const CV_ARM64_D14: CV_HREG_e = 154;
pub const CV_ARM64_D15: CV_HREG_e = 155;
pub const CV_ARM64_D16: CV_HREG_e = 156;
pub const CV_ARM64_D17: CV_HREG_e = 157;
pub const CV_ARM64_D18: CV_HREG_e = 158;
pub const CV_ARM64_D19: CV_HREG_e = 159;
pub const CV_ARM64_D2: CV_HREG_e = 142;
pub const CV_ARM64_D20: CV_HREG_e = 160;
pub const CV_ARM64_D21: CV_HREG_e = 161;
pub const CV_ARM64_D22: CV_HREG_e = 162;
pub const CV_ARM64_D23: CV_HREG_e = 163;
pub const CV_ARM64_D24: CV_HREG_e = 164;
pub const CV_ARM64_D25: CV_HREG_e = 165;
pub const CV_ARM64_D26: CV_HREG_e = 166;
pub const CV_ARM64_D27: CV_HREG_e = 167;
pub const CV_ARM64_D28: CV_HREG_e = 168;
pub const CV_ARM64_D29: CV_HREG_e = 169;
pub const CV_ARM64_D3: CV_HREG_e = 143;
pub const CV_ARM64_D30: CV_HREG_e = 170;
pub const CV_ARM64_D31: CV_HREG_e = 171;
pub const CV_ARM64_D4: CV_HREG_e = 144;
pub const CV_ARM64_D5: CV_HREG_e = 145;
pub const CV_ARM64_D6: CV_HREG_e = 146;
pub const CV_ARM64_D7: CV_HREG_e = 147;
pub const CV_ARM64_D8: CV_HREG_e = 148;
pub const CV_ARM64_D9: CV_HREG_e = 149;
pub const CV_ARM64_FFR: CV_HREG_e = 430;
pub const CV_ARM64_FP: CV_HREG_e = 79;
pub const CV_ARM64_FPCR: CV_HREG_e = 221;
pub const CV_ARM64_FPSR: CV_HREG_e = 220;
pub const CV_ARM64_H0: CV_HREG_e = 270;
pub const CV_ARM64_H1: CV_HREG_e = 271;
pub const CV_ARM64_H10: CV_HREG_e = 280;
pub const CV_ARM64_H11: CV_HREG_e = 281;
pub const CV_ARM64_H12: CV_HREG_e = 282;
pub const CV_ARM64_H13: CV_HREG_e = 283;
pub const CV_ARM64_H14: CV_HREG_e = 284;
pub const CV_ARM64_H15: CV_HREG_e = 285;
pub const CV_ARM64_H16: CV_HREG_e = 286;
pub const CV_ARM64_H17: CV_HREG_e = 287;
pub const CV_ARM64_H18: CV_HREG_e = 288;
pub const CV_ARM64_H19: CV_HREG_e = 289;
pub const CV_ARM64_H2: CV_HREG_e = 272;
pub const CV_ARM64_H20: CV_HREG_e = 290;
pub const CV_ARM64_H21: CV_HREG_e = 291;
pub const CV_ARM64_H22: CV_HREG_e = 292;
pub const CV_ARM64_H23: CV_HREG_e = 293;
pub const CV_ARM64_H24: CV_HREG_e = 294;
pub const CV_ARM64_H25: CV_HREG_e = 295;
pub const CV_ARM64_H26: CV_HREG_e = 296;
pub const CV_ARM64_H27: CV_HREG_e = 297;
pub const CV_ARM64_H28: CV_HREG_e = 298;
pub const CV_ARM64_H29: CV_HREG_e = 299;
pub const CV_ARM64_H3: CV_HREG_e = 273;
pub const CV_ARM64_H30: CV_HREG_e = 300;
pub const CV_ARM64_H31: CV_HREG_e = 301;
pub const CV_ARM64_H4: CV_HREG_e = 274;
pub const CV_ARM64_H5: CV_HREG_e = 275;
pub const CV_ARM64_H6: CV_HREG_e = 276;
pub const CV_ARM64_H7: CV_HREG_e = 277;
pub const CV_ARM64_H8: CV_HREG_e = 278;
pub const CV_ARM64_H9: CV_HREG_e = 279;
pub const CV_ARM64_IP0: CV_HREG_e = 66;
pub const CV_ARM64_IP1: CV_HREG_e = 67;
pub const CV_ARM64_LR: CV_HREG_e = 80;
pub const CV_ARM64_NOREG: CV_HREG_e = 0;
pub const CV_ARM64_NZCV: CV_HREG_e = 90;
pub const CV_ARM64_P0: CV_HREG_e = 414;
pub const CV_ARM64_P1: CV_HREG_e = 415;
pub const CV_ARM64_P10: CV_HREG_e = 424;
pub const CV_ARM64_P11: CV_HREG_e = 425;
pub const CV_ARM64_P12: CV_HREG_e = 426;
pub const CV_ARM64_P13: CV_HREG_e = 427;
pub const CV_ARM64_P14: CV_HREG_e = 428;
pub const CV_ARM64_P15: CV_HREG_e = 429;
pub const CV_ARM64_P2: CV_HREG_e = 416;
pub const CV_ARM64_P3: CV_HREG_e = 417;
pub const CV_ARM64_P4: CV_HREG_e = 418;
pub const CV_ARM64_P5: CV_HREG_e = 419;
pub const CV_ARM64_P6: CV_HREG_e = 420;
pub const CV_ARM64_P7: CV_HREG_e = 421;
pub const CV_ARM64_P8: CV_HREG_e = 422;
pub const CV_ARM64_P9: CV_HREG_e = 423;
pub const CV_ARM64_PC: CV_HREG_e = 83;
pub const CV_ARM64_Q0: CV_HREG_e = 180;
pub const CV_ARM64_Q0H: CV_HREG_e = 350;
pub const CV_ARM64_Q1: CV_HREG_e = 181;
pub const CV_ARM64_Q10: CV_HREG_e = 190;
pub const CV_ARM64_Q10H: CV_HREG_e = 360;
pub const CV_ARM64_Q11: CV_HREG_e = 191;
pub const CV_ARM64_Q11H: CV_HREG_e = 361;
pub const CV_ARM64_Q12: CV_HREG_e = 192;
pub const CV_ARM64_Q12H: CV_HREG_e = 362;
pub const CV_ARM64_Q13: CV_HREG_e = 193;
pub const CV_ARM64_Q13H: CV_HREG_e = 363;
pub const CV_ARM64_Q14: CV_HREG_e = 194;
pub const CV_ARM64_Q14H: CV_HREG_e = 364;
pub const CV_ARM64_Q15: CV_HREG_e = 195;
pub const CV_ARM64_Q15H: CV_HREG_e = 365;
pub const CV_ARM64_Q16: CV_HREG_e = 196;
pub const CV_ARM64_Q16H: CV_HREG_e = 366;
pub const CV_ARM64_Q17: CV_HREG_e = 197;
pub const CV_ARM64_Q17H: CV_HREG_e = 367;
pub const CV_ARM64_Q18: CV_HREG_e = 198;
pub const CV_ARM64_Q18H: CV_HREG_e = 368;
pub const CV_ARM64_Q19: CV_HREG_e = 199;
pub const CV_ARM64_Q19H: CV_HREG_e = 369;
pub const CV_ARM64_Q1H: CV_HREG_e = 351;
pub const CV_ARM64_Q2: CV_HREG_e = 182;
pub const CV_ARM64_Q20: CV_HREG_e = 200;
pub const CV_ARM64_Q20H: CV_HREG_e = 370;
pub const CV_ARM64_Q21: CV_HREG_e = 201;
pub const CV_ARM64_Q21H: CV_HREG_e = 371;
pub const CV_ARM64_Q22: CV_HREG_e = 202;
pub const CV_ARM64_Q22H: CV_HREG_e = 372;
pub const CV_ARM64_Q23: CV_HREG_e = 203;
pub const CV_ARM64_Q23H: CV_HREG_e = 373;
pub const CV_ARM64_Q24: CV_HREG_e = 204;
pub const CV_ARM64_Q24H: CV_HREG_e = 374;
pub const CV_ARM64_Q25: CV_HREG_e = 205;
pub const CV_ARM64_Q25H: CV_HREG_e = 375;
pub const CV_ARM64_Q26: CV_HREG_e = 206;
pub const CV_ARM64_Q26H: CV_HREG_e = 376;
pub const CV_ARM64_Q27: CV_HREG_e = 207;
pub const CV_ARM64_Q27H: CV_HREG_e = 377;
pub const CV_ARM64_Q28: CV_HREG_e = 208;
pub const CV_ARM64_Q28H: CV_HREG_e = 378;
pub const CV_ARM64_Q29: CV_HREG_e = 209;
pub const CV_ARM64_Q29H: CV_HREG_e = 379;
pub const CV_ARM64_Q2H: CV_HREG_e = 352;
pub const CV_ARM64_Q3: CV_HREG_e = 183;
pub const CV_ARM64_Q30: CV_HREG_e = 210;
pub const CV_ARM64_Q30H: CV_HREG_e = 380;
pub const CV_ARM64_Q31: CV_HREG_e = 211;
pub const CV_ARM64_Q31H: CV_HREG_e = 381;
pub const CV_ARM64_Q3H: CV_HREG_e = 353;
pub const CV_ARM64_Q4: CV_HREG_e = 184;
pub const CV_ARM64_Q4H: CV_HREG_e = 354;
pub const CV_ARM64_Q5: CV_HREG_e = 185;
pub const CV_ARM64_Q5H: CV_HREG_e = 355;
pub const CV_ARM64_Q6: CV_HREG_e = 186;
pub const CV_ARM64_Q6H: CV_HREG_e = 356;
pub const CV_ARM64_Q7: CV_HREG_e = 187;
pub const CV_ARM64_Q7H: CV_HREG_e = 357;
pub const CV_ARM64_Q8: CV_HREG_e = 188;
pub const CV_ARM64_Q8H: CV_HREG_e = 358;
pub const CV_ARM64_Q9: CV_HREG_e = 189;
pub const CV_ARM64_Q9H: CV_HREG_e = 359;
pub const CV_ARM64_S0: CV_HREG_e = 100;
pub const CV_ARM64_S1: CV_HREG_e = 101;
pub const CV_ARM64_S10: CV_HREG_e = 110;
pub const CV_ARM64_S11: CV_HREG_e = 111;
pub const CV_ARM64_S12: CV_HREG_e = 112;
pub const CV_ARM64_S13: CV_HREG_e = 113;
pub const CV_ARM64_S14: CV_HREG_e = 114;
pub const CV_ARM64_S15: CV_HREG_e = 115;
pub const CV_ARM64_S16: CV_HREG_e = 116;
pub const CV_ARM64_S17: CV_HREG_e = 117;
pub const CV_ARM64_S18: CV_HREG_e = 118;
pub const CV_ARM64_S19: CV_HREG_e = 119;
pub const CV_ARM64_S2: CV_HREG_e = 102;
pub const CV_ARM64_S20: CV_HREG_e = 120;
pub const CV_ARM64_S21: CV_HREG_e = 121;
pub const CV_ARM64_S22: CV_HREG_e = 122;
pub const CV_ARM64_S23: CV_HREG_e = 123;
pub const CV_ARM64_S24: CV_HREG_e = 124;
pub const CV_ARM64_S25: CV_HREG_e = 125;
pub const CV_ARM64_S26: CV_HREG_e = 126;
pub const CV_ARM64_S27: CV_HREG_e = 127;
pub const CV_ARM64_S28: CV_HREG_e = 128;
pub const CV_ARM64_S29: CV_HREG_e = 129;
pub const CV_ARM64_S3: CV_HREG_e = 103;
pub const CV_ARM64_S30: CV_HREG_e = 130;
pub const CV_ARM64_S31: CV_HREG_e = 131;
pub const CV_ARM64_S4: CV_HREG_e = 104;
pub const CV_ARM64_S5: CV_HREG_e = 105;
pub const CV_ARM64_S6: CV_HREG_e = 106;
pub const CV_ARM64_S7: CV_HREG_e = 107;
pub const CV_ARM64_S8: CV_HREG_e = 108;
pub const CV_ARM64_S9: CV_HREG_e = 109;
pub const CV_ARM64_SP: CV_HREG_e = 81;
pub const CV_ARM64_V0: CV_HREG_e = 310;
pub const CV_ARM64_V1: CV_HREG_e = 311;
pub const CV_ARM64_V10: CV_HREG_e = 320;
pub const CV_ARM64_V11: CV_HREG_e = 321;
pub const CV_ARM64_V12: CV_HREG_e = 322;
pub const CV_ARM64_V13: CV_HREG_e = 323;
pub const CV_ARM64_V14: CV_HREG_e = 324;
pub const CV_ARM64_V15: CV_HREG_e = 325;
pub const CV_ARM64_V16: CV_HREG_e = 326;
pub const CV_ARM64_V17: CV_HREG_e = 327;
pub const CV_ARM64_V18: CV_HREG_e = 328;
pub const CV_ARM64_V19: CV_HREG_e = 329;
pub const CV_ARM64_V2: CV_HREG_e = 312;
pub const CV_ARM64_V20: CV_HREG_e = 330;
pub const CV_ARM64_V21: CV_HREG_e = 331;
pub const CV_ARM64_V22: CV_HREG_e = 332;
pub const CV_ARM64_V23: CV_HREG_e = 333;
pub const CV_ARM64_V24: CV_HREG_e = 334;
pub const CV_ARM64_V25: CV_HREG_e = 335;
pub const CV_ARM64_V26: CV_HREG_e = 336;
pub const CV_ARM64_V27: CV_HREG_e = 337;
pub const CV_ARM64_V28: CV_HREG_e = 338;
pub const CV_ARM64_V29: CV_HREG_e = 339;
pub const CV_ARM64_V3: CV_HREG_e = 313;
pub const CV_ARM64_V30: CV_HREG_e = 340;
pub const CV_ARM64_V31: CV_HREG_e = 341;
pub const CV_ARM64_V4: CV_HREG_e = 314;
pub const CV_ARM64_V5: CV_HREG_e = 315;
pub const CV_ARM64_V6: CV_HREG_e = 316;
pub const CV_ARM64_V7: CV_HREG_e = 317;
pub const CV_ARM64_V8: CV_HREG_e = 318;
pub const CV_ARM64_V9: CV_HREG_e = 319;
pub const CV_ARM64_W0: CV_HREG_e = 10;
pub const CV_ARM64_W1: CV_HREG_e = 11;
pub const CV_ARM64_W10: CV_HREG_e = 20;
pub const CV_ARM64_W11: CV_HREG_e = 21;
pub const CV_ARM64_W12: CV_HREG_e = 22;
pub const CV_ARM64_W13: CV_HREG_e = 23;
pub const CV_ARM64_W14: CV_HREG_e = 24;
pub const CV_ARM64_W15: CV_HREG_e = 25;
pub const CV_ARM64_W16: CV_HREG_e = 26;
pub const CV_ARM64_W17: CV_HREG_e = 27;
pub const CV_ARM64_W18: CV_HREG_e = 28;
pub const CV_ARM64_W19: CV_HREG_e = 29;
pub const CV_ARM64_W2: CV_HREG_e = 12;
pub const CV_ARM64_W20: CV_HREG_e = 30;
pub const CV_ARM64_W21: CV_HREG_e = 31;
pub const CV_ARM64_W22: CV_HREG_e = 32;
pub const CV_ARM64_W23: CV_HREG_e = 33;
pub const CV_ARM64_W24: CV_HREG_e = 34;
pub const CV_ARM64_W25: CV_HREG_e = 35;
pub const CV_ARM64_W26: CV_HREG_e = 36;
pub const CV_ARM64_W27: CV_HREG_e = 37;
pub const CV_ARM64_W28: CV_HREG_e = 38;
pub const CV_ARM64_W29: CV_HREG_e = 39;
pub const CV_ARM64_W3: CV_HREG_e = 13;
pub const CV_ARM64_W30: CV_HREG_e = 40;
pub const CV_ARM64_W4: CV_HREG_e = 14;
pub const CV_ARM64_W5: CV_HREG_e = 15;
pub const CV_ARM64_W6: CV_HREG_e = 16;
pub const CV_ARM64_W7: CV_HREG_e = 17;
pub const CV_ARM64_W8: CV_HREG_e = 18;
pub const CV_ARM64_W9: CV_HREG_e = 19;
pub const CV_ARM64_WZR: CV_HREG_e = 41;
pub const CV_ARM64_X0: CV_HREG_e = 50;
pub const CV_ARM64_X1: CV_HREG_e = 51;
pub const CV_ARM64_X10: CV_HREG_e = 60;
pub const CV_ARM64_X11: CV_HREG_e = 61;
pub const CV_ARM64_X12: CV_HREG_e = 62;
pub const CV_ARM64_X13: CV_HREG_e = 63;
pub const CV_ARM64_X14: CV_HREG_e = 64;
pub const CV_ARM64_X15: CV_HREG_e = 65;
pub const CV_ARM64_X18: CV_HREG_e = 68;
pub const CV_ARM64_X19: CV_HREG_e = 69;
pub const CV_ARM64_X2: CV_HREG_e = 52;
pub const CV_ARM64_X20: CV_HREG_e = 70;
pub const CV_ARM64_X21: CV_HREG_e = 71;
pub const CV_ARM64_X22: CV_HREG_e = 72;
pub const CV_ARM64_X23: CV_HREG_e = 73;
pub const CV_ARM64_X24: CV_HREG_e = 74;
pub const CV_ARM64_X25: CV_HREG_e = 75;
pub const CV_ARM64_X26: CV_HREG_e = 76;
pub const CV_ARM64_X27: CV_HREG_e = 77;
pub const CV_ARM64_X28: CV_HREG_e = 78;
pub const CV_ARM64_X3: CV_HREG_e = 53;
pub const CV_ARM64_X4: CV_HREG_e = 54;
pub const CV_ARM64_X5: CV_HREG_e = 55;
pub const CV_ARM64_X6: CV_HREG_e = 56;
pub const CV_ARM64_X7: CV_HREG_e = 57;
pub const CV_ARM64_X8: CV_HREG_e = 58;
pub const CV_ARM64_X9: CV_HREG_e = 59;
pub const CV_ARM64_Z0: CV_HREG_e = 382;
pub const CV_ARM64_Z1: CV_HREG_e = 383;
pub const CV_ARM64_Z10: CV_HREG_e = 392;
pub const CV_ARM64_Z11: CV_HREG_e = 393;
pub const CV_ARM64_Z12: CV_HREG_e = 394;
pub const CV_ARM64_Z13: CV_HREG_e = 395;
pub const CV_ARM64_Z14: CV_HREG_e = 396;
pub const CV_ARM64_Z15: CV_HREG_e = 397;
pub const CV_ARM64_Z16: CV_HREG_e = 398;
pub const CV_ARM64_Z17: CV_HREG_e = 399;
pub const CV_ARM64_Z18: CV_HREG_e = 400;
pub const CV_ARM64_Z19: CV_HREG_e = 401;
pub const CV_ARM64_Z2: CV_HREG_e = 384;
pub const CV_ARM64_Z20: CV_HREG_e = 402;
pub const CV_ARM64_Z21: CV_HREG_e = 403;
pub const CV_ARM64_Z22: CV_HREG_e = 404;
pub const CV_ARM64_Z23: CV_HREG_e = 405;
pub const CV_ARM64_Z24: CV_HREG_e = 406;
pub const CV_ARM64_Z25: CV_HREG_e = 407;
pub const CV_ARM64_Z26: CV_HREG_e = 408;
pub const CV_ARM64_Z27: CV_HREG_e = 409;
pub const CV_ARM64_Z28: CV_HREG_e = 410;
pub const CV_ARM64_Z29: CV_HREG_e = 411;
pub const CV_ARM64_Z3: CV_HREG_e = 385;
pub const CV_ARM64_Z30: CV_HREG_e = 412;
pub const CV_ARM64_Z31: CV_HREG_e = 413;
pub const CV_ARM64_Z4: CV_HREG_e = 386;
pub const CV_ARM64_Z5: CV_HREG_e = 387;
pub const CV_ARM64_Z6: CV_HREG_e = 388;
pub const CV_ARM64_Z7: CV_HREG_e = 389;
pub const CV_ARM64_Z8: CV_HREG_e = 390;
pub const CV_ARM64_Z9: CV_HREG_e = 391;
pub const CV_ARM64_ZR: CV_HREG_e = 82;
pub const CV_ARM_ACC0: CV_HREG_e = 27;
pub const CV_ARM_CPSR: CV_HREG_e = 26;
pub const CV_ARM_FPEXC: CV_HREG_e = 41;
pub const CV_ARM_FPEXTRA0: CV_HREG_e = 90;
pub const CV_ARM_FPEXTRA1: CV_HREG_e = 91;
pub const CV_ARM_FPEXTRA2: CV_HREG_e = 92;
pub const CV_ARM_FPEXTRA3: CV_HREG_e = 93;
pub const CV_ARM_FPEXTRA4: CV_HREG_e = 94;
pub const CV_ARM_FPEXTRA5: CV_HREG_e = 95;
pub const CV_ARM_FPEXTRA6: CV_HREG_e = 96;
pub const CV_ARM_FPEXTRA7: CV_HREG_e = 97;
pub const CV_ARM_FPSCR: CV_HREG_e = 40;
pub const CV_ARM_FS0: CV_HREG_e = 50;
pub const CV_ARM_FS1: CV_HREG_e = 51;
pub const CV_ARM_FS10: CV_HREG_e = 60;
pub const CV_ARM_FS11: CV_HREG_e = 61;
pub const CV_ARM_FS12: CV_HREG_e = 62;
pub const CV_ARM_FS13: CV_HREG_e = 63;
pub const CV_ARM_FS14: CV_HREG_e = 64;
pub const CV_ARM_FS15: CV_HREG_e = 65;
pub const CV_ARM_FS16: CV_HREG_e = 66;
pub const CV_ARM_FS17: CV_HREG_e = 67;
pub const CV_ARM_FS18: CV_HREG_e = 68;
pub const CV_ARM_FS19: CV_HREG_e = 69;
pub const CV_ARM_FS2: CV_HREG_e = 52;
pub const CV_ARM_FS20: CV_HREG_e = 70;
pub const CV_ARM_FS21: CV_HREG_e = 71;
pub const CV_ARM_FS22: CV_HREG_e = 72;
pub const CV_ARM_FS23: CV_HREG_e = 73;
pub const CV_ARM_FS24: CV_HREG_e = 74;
pub const CV_ARM_FS25: CV_HREG_e = 75;
pub const CV_ARM_FS26: CV_HREG_e = 76;
pub const CV_ARM_FS27: CV_HREG_e = 77;
pub const CV_ARM_FS28: CV_HREG_e = 78;
pub const CV_ARM_FS29: CV_HREG_e = 79;
pub const CV_ARM_FS3: CV_HREG_e = 53;
pub const CV_ARM_FS30: CV_HREG_e = 80;
pub const CV_ARM_FS31: CV_HREG_e = 81;
pub const CV_ARM_FS32: CV_HREG_e = 200;
pub const CV_ARM_FS33: CV_HREG_e = 201;
pub const CV_ARM_FS34: CV_HREG_e = 202;
pub const CV_ARM_FS35: CV_HREG_e = 203;
pub const CV_ARM_FS36: CV_HREG_e = 204;
pub const CV_ARM_FS37: CV_HREG_e = 205;
pub const CV_ARM_FS38: CV_HREG_e = 206;
pub const CV_ARM_FS39: CV_HREG_e = 207;
pub const CV_ARM_FS4: CV_HREG_e = 54;
pub const CV_ARM_FS40: CV_HREG_e = 208;
pub const CV_ARM_FS41: CV_HREG_e = 209;
pub const CV_ARM_FS42: CV_HREG_e = 210;
pub const CV_ARM_FS43: CV_HREG_e = 211;
pub const CV_ARM_FS44: CV_HREG_e = 212;
pub const CV_ARM_FS45: CV_HREG_e = 213;
pub const CV_ARM_FS46: CV_HREG_e = 214;
pub const CV_ARM_FS47: CV_HREG_e = 215;
pub const CV_ARM_FS48: CV_HREG_e = 216;
pub const CV_ARM_FS49: CV_HREG_e = 217;
pub const CV_ARM_FS5: CV_HREG_e = 55;
pub const CV_ARM_FS50: CV_HREG_e = 218;
pub const CV_ARM_FS51: CV_HREG_e = 219;
pub const CV_ARM_FS52: CV_HREG_e = 220;
pub const CV_ARM_FS53: CV_HREG_e = 221;
pub const CV_ARM_FS54: CV_HREG_e = 222;
pub const CV_ARM_FS55: CV_HREG_e = 223;
pub const CV_ARM_FS56: CV_HREG_e = 224;
pub const CV_ARM_FS57: CV_HREG_e = 225;
pub const CV_ARM_FS58: CV_HREG_e = 226;
pub const CV_ARM_FS59: CV_HREG_e = 227;
pub const CV_ARM_FS6: CV_HREG_e = 56;
pub const CV_ARM_FS60: CV_HREG_e = 228;
pub const CV_ARM_FS61: CV_HREG_e = 229;
pub const CV_ARM_FS62: CV_HREG_e = 230;
pub const CV_ARM_FS63: CV_HREG_e = 231;
pub const CV_ARM_FS7: CV_HREG_e = 57;
pub const CV_ARM_FS8: CV_HREG_e = 58;
pub const CV_ARM_FS9: CV_HREG_e = 59;
pub const CV_ARM_LR: CV_HREG_e = 24;
pub const CV_ARM_ND0: CV_HREG_e = 300;
pub const CV_ARM_ND1: CV_HREG_e = 301;
pub const CV_ARM_ND10: CV_HREG_e = 310;
pub const CV_ARM_ND11: CV_HREG_e = 311;
pub const CV_ARM_ND12: CV_HREG_e = 312;
pub const CV_ARM_ND13: CV_HREG_e = 313;
pub const CV_ARM_ND14: CV_HREG_e = 314;
pub const CV_ARM_ND15: CV_HREG_e = 315;
pub const CV_ARM_ND16: CV_HREG_e = 316;
pub const CV_ARM_ND17: CV_HREG_e = 317;
pub const CV_ARM_ND18: CV_HREG_e = 318;
pub const CV_ARM_ND19: CV_HREG_e = 319;
pub const CV_ARM_ND2: CV_HREG_e = 302;
pub const CV_ARM_ND20: CV_HREG_e = 320;
pub const CV_ARM_ND21: CV_HREG_e = 321;
pub const CV_ARM_ND22: CV_HREG_e = 322;
pub const CV_ARM_ND23: CV_HREG_e = 323;
pub const CV_ARM_ND24: CV_HREG_e = 324;
pub const CV_ARM_ND25: CV_HREG_e = 325;
pub const CV_ARM_ND26: CV_HREG_e = 326;
pub const CV_ARM_ND27: CV_HREG_e = 327;
pub const CV_ARM_ND28: CV_HREG_e = 328;
pub const CV_ARM_ND29: CV_HREG_e = 329;
pub const CV_ARM_ND3: CV_HREG_e = 303;
pub const CV_ARM_ND30: CV_HREG_e = 330;
pub const CV_ARM_ND31: CV_HREG_e = 331;
pub const CV_ARM_ND4: CV_HREG_e = 304;
pub const CV_ARM_ND5: CV_HREG_e = 305;
pub const CV_ARM_ND6: CV_HREG_e = 306;
pub const CV_ARM_ND7: CV_HREG_e = 307;
pub const CV_ARM_ND8: CV_HREG_e = 308;
pub const CV_ARM_ND9: CV_HREG_e = 309;
pub const CV_ARM_NOREG: CV_HREG_e = 0;
pub const CV_ARM_NQ0: CV_HREG_e = 400;
pub const CV_ARM_NQ1: CV_HREG_e = 401;
pub const CV_ARM_NQ10: CV_HREG_e = 410;
pub const CV_ARM_NQ11: CV_HREG_e = 411;
pub const CV_ARM_NQ12: CV_HREG_e = 412;
pub const CV_ARM_NQ13: CV_HREG_e = 413;
pub const CV_ARM_NQ14: CV_HREG_e = 414;
pub const CV_ARM_NQ15: CV_HREG_e = 415;
pub const CV_ARM_NQ2: CV_HREG_e = 402;
pub const CV_ARM_NQ3: CV_HREG_e = 403;
pub const CV_ARM_NQ4: CV_HREG_e = 404;
pub const CV_ARM_NQ5: CV_HREG_e = 405;
pub const CV_ARM_NQ6: CV_HREG_e = 406;
pub const CV_ARM_NQ7: CV_HREG_e = 407;
pub const CV_ARM_NQ8: CV_HREG_e = 408;
pub const CV_ARM_NQ9: CV_HREG_e = 409;
pub const CV_ARM_PC: CV_HREG_e = 25;
pub const CV_ARM_R0: CV_HREG_e = 10;
pub const CV_ARM_R1: CV_HREG_e = 11;
pub const CV_ARM_R10: CV_HREG_e = 20;
pub const CV_ARM_R11: CV_HREG_e = 21;
pub const CV_ARM_R12: CV_HREG_e = 22;
pub const CV_ARM_R2: CV_HREG_e = 12;
pub const CV_ARM_R3: CV_HREG_e = 13;
pub const CV_ARM_R4: CV_HREG_e = 14;
pub const CV_ARM_R5: CV_HREG_e = 15;
pub const CV_ARM_R6: CV_HREG_e = 16;
pub const CV_ARM_R7: CV_HREG_e = 17;
pub const CV_ARM_R8: CV_HREG_e = 18;
pub const CV_ARM_R9: CV_HREG_e = 19;
pub const CV_ARM_SP: CV_HREG_e = 23;
pub const CV_ARM_WC12: CV_HREG_e = 156;
pub const CV_ARM_WC13: CV_HREG_e = 157;
pub const CV_ARM_WC14: CV_HREG_e = 158;
pub const CV_ARM_WC15: CV_HREG_e = 159;
pub const CV_ARM_WC4: CV_HREG_e = 148;
pub const CV_ARM_WC5: CV_HREG_e = 149;
pub const CV_ARM_WC6: CV_HREG_e = 150;
pub const CV_ARM_WC7: CV_HREG_e = 151;
pub const CV_ARM_WCASF: CV_HREG_e = 147;
pub const CV_ARM_WCGR0: CV_HREG_e = 152;
pub const CV_ARM_WCGR1: CV_HREG_e = 153;
pub const CV_ARM_WCGR2: CV_HREG_e = 154;
pub const CV_ARM_WCGR3: CV_HREG_e = 155;
pub const CV_ARM_WCID: CV_HREG_e = 144;
pub const CV_ARM_WCON: CV_HREG_e = 145;
pub const CV_ARM_WCSSF: CV_HREG_e = 146;
pub const CV_ARM_WR0: CV_HREG_e = 128;
pub const CV_ARM_WR1: CV_HREG_e = 129;
pub const CV_ARM_WR10: CV_HREG_e = 138;
pub const CV_ARM_WR11: CV_HREG_e = 139;
pub const CV_ARM_WR12: CV_HREG_e = 140;
pub const CV_ARM_WR13: CV_HREG_e = 141;
pub const CV_ARM_WR14: CV_HREG_e = 142;
pub const CV_ARM_WR15: CV_HREG_e = 143;
pub const CV_ARM_WR2: CV_HREG_e = 130;
pub const CV_ARM_WR3: CV_HREG_e = 131;
pub const CV_ARM_WR4: CV_HREG_e = 132;
pub const CV_ARM_WR5: CV_HREG_e = 133;
pub const CV_ARM_WR6: CV_HREG_e = 134;
pub const CV_ARM_WR7: CV_HREG_e = 135;
pub const CV_ARM_WR8: CV_HREG_e = 136;
pub const CV_ARM_WR9: CV_HREG_e = 137;
pub const CV_ASSOCIATIONKIND_COROUTINE: CV_AssociationKind_e = 1;
pub const CV_ASSOCIATIONKIND_NONE: CV_AssociationKind_e = 0;
pub type CV_AssociationKind_e = i32;
pub const CV_BI_HLSL_APPEND_STRUCTURED_BUFFER: CV_builtin_e = 540;
pub const CV_BI_HLSL_BUFFER: CV_builtin_e = 524;
pub const CV_BI_HLSL_BYTEADDRESS_BUFFER: CV_builtin_e = 536;
pub const CV_BI_HLSL_CONSTANT_BUFFER: CV_builtin_e = 548;
pub const CV_BI_HLSL_CONSUME_STRUCTURED_BUFFER: CV_builtin_e = 541;
pub const CV_BI_HLSL_INPUTPATCH: CV_builtin_e = 528;
pub const CV_BI_HLSL_INTERFACE_POINTER: CV_builtin_e = 512;
pub const CV_BI_HLSL_LINESTREAM: CV_builtin_e = 526;
pub const CV_BI_HLSL_MIN10FLOAT: CV_builtin_e = 543;
pub const CV_BI_HLSL_MIN12INT: CV_builtin_e = 545;
pub const CV_BI_HLSL_MIN16FLOAT: CV_builtin_e = 544;
pub const CV_BI_HLSL_MIN16INT: CV_builtin_e = 546;
pub const CV_BI_HLSL_MIN16UINT: CV_builtin_e = 547;
pub const CV_BI_HLSL_MIN8FLOAT: CV_builtin_e = 542;
pub const CV_BI_HLSL_OUTPUTPATCH: CV_builtin_e = 529;
pub const CV_BI_HLSL_POINTSTREAM: CV_builtin_e = 525;
pub const CV_BI_HLSL_RWBUFFER: CV_builtin_e = 535;
pub const CV_BI_HLSL_RWBYTEADDRESS_BUFFER: CV_builtin_e = 537;
pub const CV_BI_HLSL_RWSTRUCTURED_BUFFER: CV_builtin_e = 539;
pub const CV_BI_HLSL_RWTEXTURE1D: CV_builtin_e = 530;
pub const CV_BI_HLSL_RWTEXTURE1D_ARRAY: CV_builtin_e = 531;
pub const CV_BI_HLSL_RWTEXTURE2D: CV_builtin_e = 532;
pub const CV_BI_HLSL_RWTEXTURE2D_ARRAY: CV_builtin_e = 533;
pub const CV_BI_HLSL_RWTEXTURE3D: CV_builtin_e = 534;
pub const CV_BI_HLSL_SAMPLER: CV_builtin_e = 522;
pub const CV_BI_HLSL_SAMPLERCOMPARISON: CV_builtin_e = 523;
pub const CV_BI_HLSL_STRUCTURED_BUFFER: CV_builtin_e = 538;
pub const CV_BI_HLSL_TEXTURE1D: CV_builtin_e = 513;
pub const CV_BI_HLSL_TEXTURE1D_ARRAY: CV_builtin_e = 514;
pub const CV_BI_HLSL_TEXTURE2D: CV_builtin_e = 515;
pub const CV_BI_HLSL_TEXTURE2DMS: CV_builtin_e = 520;
pub const CV_BI_HLSL_TEXTURE2DMS_ARRAY: CV_builtin_e = 521;
pub const CV_BI_HLSL_TEXTURE2D_ARRAY: CV_builtin_e = 516;
pub const CV_BI_HLSL_TEXTURE3D: CV_builtin_e = 517;
pub const CV_BI_HLSL_TEXTURECUBE: CV_builtin_e = 518;
pub const CV_BI_HLSL_TEXTURECUBE_ARRAY: CV_builtin_e = 519;
pub const CV_BI_HLSL_TRIANGLESTREAM: CV_builtin_e = 527;
pub const CV_BI_INVALID: CV_builtin_e = 0;
pub const CV_CALL_ALPHACALL: CV_call_e = 14;
pub const CV_CALL_AM33CALL: CV_call_e = 18;
pub const CV_CALL_ARMCALL: CV_call_e = 17;
pub const CV_CALL_CLRCALL: CV_call_e = 22;
pub const CV_CALL_FAR_C: CV_call_e = 1;
pub const CV_CALL_FAR_FAST: CV_call_e = 5;
pub const CV_CALL_FAR_PASCAL: CV_call_e = 3;
pub const CV_CALL_FAR_STD: CV_call_e = 8;
pub const CV_CALL_FAR_SYS: CV_call_e = 10;
pub const CV_CALL_GENERIC: CV_call_e = 13;
pub const CV_CALL_INLINE: CV_call_e = 23;
pub const CV_CALL_M32RCALL: CV_call_e = 21;
pub const CV_CALL_MIPSCALL: CV_call_e = 12;
pub const CV_CALL_NEAR_C: CV_call_e = 0;
pub const CV_CALL_NEAR_FAST: CV_call_e = 4;
pub const CV_CALL_NEAR_PASCAL: CV_call_e = 2;
pub const CV_CALL_NEAR_STD: CV_call_e = 7;
pub const CV_CALL_NEAR_SYS: CV_call_e = 9;
pub const CV_CALL_NEAR_VECTOR: CV_call_e = 24;
pub const CV_CALL_PPCCALL: CV_call_e = 15;
pub const CV_CALL_PRESERVE_NONE: CV_call_e = 32;
pub const CV_CALL_RESERVED: CV_call_e = 33;
pub const CV_CALL_SH5CALL: CV_call_e = 20;
pub const CV_CALL_SHCALL: CV_call_e = 16;
pub const CV_CALL_SKIPPED: CV_call_e = 6;
pub const CV_CALL_SWIFT: CV_call_e = 25;
pub const CV_CALL_THISCALL: CV_call_e = 11;
pub const CV_CALL_TRICALL: CV_call_e = 19;
pub const CV_CFL_80286: CV_CPU_TYPE_e = 2;
pub const CV_CFL_80386: CV_CPU_TYPE_e = 3;
pub const CV_CFL_80486: CV_CPU_TYPE_e = 4;
pub const CV_CFL_8080: CV_CPU_TYPE_e = 0;
pub const CV_CFL_8086: CV_CPU_TYPE_e = 1;
pub const CV_CFL_ALIASOBJ: CV_CFL_LANG = 20;
pub const CV_CFL_ALPHA: CV_CPU_TYPE_e = 48;
pub const CV_CFL_ALPHA_21064: CV_CPU_TYPE_e = 48;
pub const CV_CFL_ALPHA_21164: CV_CPU_TYPE_e = 49;
pub const CV_CFL_ALPHA_21164A: CV_CPU_TYPE_e = 50;
pub const CV_CFL_ALPHA_21264: CV_CPU_TYPE_e = 51;
pub const CV_CFL_ALPHA_21364: CV_CPU_TYPE_e = 52;
pub const CV_CFL_AM33: CV_CPU_TYPE_e = 160;
pub const CV_CFL_AMD64: CV_CPU_TYPE_e = 208;
pub const CV_CFL_ARM3: CV_CPU_TYPE_e = 96;
pub const CV_CFL_ARM4: CV_CPU_TYPE_e = 97;
pub const CV_CFL_ARM4T: CV_CPU_TYPE_e = 98;
pub const CV_CFL_ARM5: CV_CPU_TYPE_e = 99;
pub const CV_CFL_ARM5T: CV_CPU_TYPE_e = 100;
pub const CV_CFL_ARM6: CV_CPU_TYPE_e = 101;
pub const CV_CFL_ARM64: CV_CPU_TYPE_e = 246;
pub const CV_CFL_ARM64EC: CV_CPU_TYPE_e = 248;
pub const CV_CFL_ARM64X: CV_CPU_TYPE_e = 249;
pub const CV_CFL_ARM7: CV_CPU_TYPE_e = 104;
pub const CV_CFL_ARMNT: CV_CPU_TYPE_e = 244;
pub const CV_CFL_ARM_WMMX: CV_CPU_TYPE_e = 103;
pub const CV_CFL_ARM_XMAC: CV_CPU_TYPE_e = 102;
pub const CV_CFL_BASIC: CV_CFL_LANG = 5;
pub const CV_CFL_C: CV_CFL_LANG = 0;
pub const CV_CFL_CEE: CV_CPU_TYPE_e = 144;
pub const CV_CFL_COBOL: CV_CFL_LANG = 6;
pub const CV_CFL_CSHARP: CV_CFL_LANG = 10;
pub const CV_CFL_CVTPGD: CV_CFL_LANG = 9;
pub const CV_CFL_CVTRES: CV_CFL_LANG = 8;
pub const CV_CFL_CXX: CV_CFL_LANG = 1;
pub const CV_CFL_D3D11_SHADER: CV_CPU_TYPE_e = 256;
pub const CV_CFL_EBC: CV_CPU_TYPE_e = 224;
pub const CV_CFL_FORTRAN: CV_CFL_LANG = 2;
pub const CV_CFL_GO: CV_CFL_LANG = 22;
pub const CV_CFL_HLSL: CV_CFL_LANG = 16;
pub const CV_CFL_HYBRID_X86_ARM64: CV_CPU_TYPE_e = 247;
pub const CV_CFL_IA64: CV_CPU_TYPE_e = 128;
pub const CV_CFL_IA64_1: CV_CPU_TYPE_e = 128;
pub const CV_CFL_IA64_2: CV_CPU_TYPE_e = 129;
pub const CV_CFL_ILASM: CV_CFL_LANG = 12;
pub const CV_CFL_JAVA: CV_CFL_LANG = 13;
pub const CV_CFL_JSCRIPT: CV_CFL_LANG = 14;
pub type CV_CFL_LANG = i32;
pub const CV_CFL_LINK: CV_CFL_LANG = 7;
pub const CV_CFL_M32R: CV_CPU_TYPE_e = 176;
pub const CV_CFL_M68000: CV_CPU_TYPE_e = 32;
pub const CV_CFL_M68010: CV_CPU_TYPE_e = 33;
pub const CV_CFL_M68020: CV_CPU_TYPE_e = 34;
pub const CV_CFL_M68030: CV_CPU_TYPE_e = 35;
pub const CV_CFL_M68040: CV_CPU_TYPE_e = 36;
pub const CV_CFL_MASM: CV_CFL_LANG = 3;
pub const CV_CFL_MIPS: CV_CPU_TYPE_e = 16;
pub const CV_CFL_MIPS16: CV_CPU_TYPE_e = 17;
pub const CV_CFL_MIPS32: CV_CPU_TYPE_e = 18;
pub const CV_CFL_MIPS64: CV_CPU_TYPE_e = 19;
pub const CV_CFL_MIPSI: CV_CPU_TYPE_e = 20;
pub const CV_CFL_MIPSII: CV_CPU_TYPE_e = 21;
pub const CV_CFL_MIPSIII: CV_CPU_TYPE_e = 22;
pub const CV_CFL_MIPSIV: CV_CPU_TYPE_e = 23;
pub const CV_CFL_MIPSR4000: CV_CPU_TYPE_e = 16;
pub const CV_CFL_MIPSV: CV_CPU_TYPE_e = 24;
pub const CV_CFL_MSIL: CV_CFL_LANG = 15;
pub const CV_CFL_OBJC: CV_CFL_LANG = 17;
pub const CV_CFL_OBJCXX: CV_CFL_LANG = 18;
pub const CV_CFL_OMNI: CV_CPU_TYPE_e = 112;
pub const CV_CFL_PASCAL: CV_CFL_LANG = 4;
pub const CV_CFL_PENTIUM: CV_CPU_TYPE_e = 5;
pub const CV_CFL_PENTIUMII: CV_CPU_TYPE_e = 6;
pub const CV_CFL_PENTIUMIII: CV_CPU_TYPE_e = 7;
pub const CV_CFL_PENTIUMPRO: CV_CPU_TYPE_e = 6;
pub const CV_CFL_PPC601: CV_CPU_TYPE_e = 64;
pub const CV_CFL_PPC603: CV_CPU_TYPE_e = 65;
pub const CV_CFL_PPC604: CV_CPU_TYPE_e = 66;
pub const CV_CFL_PPC620: CV_CPU_TYPE_e = 67;
pub const CV_CFL_PPCBE: CV_CPU_TYPE_e = 69;
pub const CV_CFL_PPCFP: CV_CPU_TYPE_e = 68;
pub const CV_CFL_RUST: CV_CFL_LANG = 21;
pub const CV_CFL_SH3: CV_CPU_TYPE_e = 80;
pub const CV_CFL_SH3DSP: CV_CPU_TYPE_e = 82;
pub const CV_CFL_SH3E: CV_CPU_TYPE_e = 81;
pub const CV_CFL_SH4: CV_CPU_TYPE_e = 83;
pub const CV_CFL_SHMEDIA: CV_CPU_TYPE_e = 84;
pub const CV_CFL_SWIFT: CV_CFL_LANG = 19;
pub const CV_CFL_THUMB: CV_CPU_TYPE_e = 240;
pub const CV_CFL_TRICORE: CV_CPU_TYPE_e = 192;
pub const CV_CFL_UNKNOWN: CV_CPU_TYPE_e = 255;
pub const CV_CFL_VB: CV_CFL_LANG = 11;
pub const CV_CFL_X64: CV_CPU_TYPE_e = 208;
pub const CV_COROUTINEKIND_DESTROY: CV_CoroutineKind_e = 4;
pub const CV_COROUTINEKIND_INIT: CV_CoroutineKind_e = 2;
pub const CV_COROUTINEKIND_NONE: CV_CoroutineKind_e = 0;
pub const CV_COROUTINEKIND_PRIMARY: CV_CoroutineKind_e = 1;
pub const CV_COROUTINEKIND_RESUME: CV_CoroutineKind_e = 3;
pub type CV_CPU_TYPE_e = i32;
pub type CV_CoroutineKind_e = i32;
pub type CV_HLSLMemorySpace_e = i32;
pub const CV_HLSLREG_CONSTANT_BUFFER: CV_HLSLREG_e = 8;
pub const CV_HLSLREG_CYCLE_COUNTER: CV_HLSLREG_e = 40;
pub const CV_HLSLREG_FUNCTION_BODY: CV_HLSLREG_e = 17;
pub const CV_HLSLREG_FUNCTION_INPUT: CV_HLSLREG_e = 20;
pub const CV_HLSLREG_FUNCTION_OUTPUT: CV_HLSLREG_e = 21;
pub const CV_HLSLREG_FUNCTION_TABLE: CV_HLSLREG_e = 18;
pub const CV_HLSLREG_IMMEDIATE32: CV_HLSLREG_e = 4;
pub const CV_HLSLREG_IMMEDIATE64: CV_HLSLREG_e = 5;
pub const CV_HLSLREG_IMMEDIATE_CONSTANT_BUFFER: CV_HLSLREG_e = 9;
pub const CV_HLSLREG_INDEXABLE_TEMP: CV_HLSLREG_e = 3;
pub const CV_HLSLREG_INPUT: CV_HLSLREG_e = 1;
pub const CV_HLSLREG_INPUT_CONTROL_POINT: CV_HLSLREG_e = 25;
pub const CV_HLSLREG_INPUT_COVERAGE_MASK: CV_HLSLREG_e = 35;
pub const CV_HLSLREG_INPUT_DOMAIN_POINT: CV_HLSLREG_e = 28;
pub const CV_HLSLREG_INPUT_FORK_INSTANCE_ID: CV_HLSLREG_e = 23;
pub const CV_HLSLREG_INPUT_GS_INSTANCE_ID: CV_HLSLREG_e = 37;
pub const CV_HLSLREG_INPUT_JOIN_INSTANCE_ID: CV_HLSLREG_e = 24;
pub const CV_HLSLREG_INPUT_PATCH_CONSTANT: CV_HLSLREG_e = 27;
pub const CV_HLSLREG_INPUT_PRIMITIVEID: CV_HLSLREG_e = 11;
pub const CV_HLSLREG_INPUT_THREAD_GROUP_ID: CV_HLSLREG_e = 33;
pub const CV_HLSLREG_INPUT_THREAD_ID: CV_HLSLREG_e = 32;
pub const CV_HLSLREG_INPUT_THREAD_ID_IN_GROUP: CV_HLSLREG_e = 34;
pub const CV_HLSLREG_INPUT_THREAD_ID_IN_GROUP_FLATTENED: CV_HLSLREG_e = 36;
pub const CV_HLSLREG_INTERFACE: CV_HLSLREG_e = 19;
pub const CV_HLSLREG_LABEL: CV_HLSLREG_e = 10;
pub const CV_HLSLREG_NULL: CV_HLSLREG_e = 13;
pub const CV_HLSLREG_OUTPUT: CV_HLSLREG_e = 2;
pub const CV_HLSLREG_OUTPUT_CONTROL_POINT: CV_HLSLREG_e = 26;
pub const CV_HLSLREG_OUTPUT_CONTROL_POINT_ID: CV_HLSLREG_e = 22;
pub const CV_HLSLREG_OUTPUT_COVERAGE_MASK: CV_HLSLREG_e = 15;
pub const CV_HLSLREG_OUTPUT_DEPTH: CV_HLSLREG_e = 12;
pub const CV_HLSLREG_OUTPUT_DEPTH_GREATER_EQUAL: CV_HLSLREG_e = 38;
pub const CV_HLSLREG_OUTPUT_DEPTH_LESS_EQUAL: CV_HLSLREG_e = 39;
pub const CV_HLSLREG_RASTERIZER: CV_HLSLREG_e = 14;
pub const CV_HLSLREG_RESOURCE: CV_HLSLREG_e = 7;
pub const CV_HLSLREG_SAMPLER: CV_HLSLREG_e = 6;
pub const CV_HLSLREG_STREAM: CV_HLSLREG_e = 16;
pub const CV_HLSLREG_TEMP: CV_HLSLREG_e = 0;
pub const CV_HLSLREG_THIS_POINTER: CV_HLSLREG_e = 29;
pub const CV_HLSLREG_THREAD_GROUP_SHARED_MEMORY: CV_HLSLREG_e = 31;
pub const CV_HLSLREG_UNORDERED_ACCESS_VIEW: CV_HLSLREG_e = 30;
pub type CV_HLSLREG_e = i32;
pub const CV_HLSL_MEMSPACE_DATA: CV_HLSLMemorySpace_e = 0;
pub const CV_HLSL_MEMSPACE_MAX: CV_HLSLMemorySpace_e = 15;
pub const CV_HLSL_MEMSPACE_RESOURCE: CV_HLSLMemorySpace_e = 2;
pub const CV_HLSL_MEMSPACE_RWRESOURCE: CV_HLSLMemorySpace_e = 3;
pub const CV_HLSL_MEMSPACE_SAMPLER: CV_HLSLMemorySpace_e = 1;
pub type CV_HREG_e = i32;
pub const CV_IA64_AR10: CV_HREG_e = 3082;
pub const CV_IA64_AR100: CV_HREG_e = 3172;
pub const CV_IA64_AR101: CV_HREG_e = 3173;
pub const CV_IA64_AR102: CV_HREG_e = 3174;
pub const CV_IA64_AR103: CV_HREG_e = 3175;
pub const CV_IA64_AR104: CV_HREG_e = 3176;
pub const CV_IA64_AR105: CV_HREG_e = 3177;
pub const CV_IA64_AR106: CV_HREG_e = 3178;
pub const CV_IA64_AR107: CV_HREG_e = 3179;
pub const CV_IA64_AR108: CV_HREG_e = 3180;
pub const CV_IA64_AR109: CV_HREG_e = 3181;
pub const CV_IA64_AR11: CV_HREG_e = 3083;
pub const CV_IA64_AR110: CV_HREG_e = 3182;
pub const CV_IA64_AR111: CV_HREG_e = 3183;
pub const CV_IA64_AR112: CV_HREG_e = 3184;
pub const CV_IA64_AR113: CV_HREG_e = 3185;
pub const CV_IA64_AR114: CV_HREG_e = 3186;
pub const CV_IA64_AR115: CV_HREG_e = 3187;
pub const CV_IA64_AR116: CV_HREG_e = 3188;
pub const CV_IA64_AR117: CV_HREG_e = 3189;
pub const CV_IA64_AR118: CV_HREG_e = 3190;
pub const CV_IA64_AR119: CV_HREG_e = 3191;
pub const CV_IA64_AR12: CV_HREG_e = 3084;
pub const CV_IA64_AR120: CV_HREG_e = 3192;
pub const CV_IA64_AR121: CV_HREG_e = 3193;
pub const CV_IA64_AR122: CV_HREG_e = 3194;
pub const CV_IA64_AR123: CV_HREG_e = 3195;
pub const CV_IA64_AR124: CV_HREG_e = 3196;
pub const CV_IA64_AR125: CV_HREG_e = 3197;
pub const CV_IA64_AR126: CV_HREG_e = 3198;
pub const CV_IA64_AR127: CV_HREG_e = 3199;
pub const CV_IA64_AR13: CV_HREG_e = 3085;
pub const CV_IA64_AR14: CV_HREG_e = 3086;
pub const CV_IA64_AR15: CV_HREG_e = 3087;
pub const CV_IA64_AR20: CV_HREG_e = 3092;
pub const CV_IA64_AR22: CV_HREG_e = 3094;
pub const CV_IA64_AR23: CV_HREG_e = 3095;
pub const CV_IA64_AR31: CV_HREG_e = 3103;
pub const CV_IA64_AR33: CV_HREG_e = 3105;
pub const CV_IA64_AR34: CV_HREG_e = 3106;
pub const CV_IA64_AR35: CV_HREG_e = 3107;
pub const CV_IA64_AR37: CV_HREG_e = 3109;
pub const CV_IA64_AR38: CV_HREG_e = 3110;
pub const CV_IA64_AR39: CV_HREG_e = 3111;
pub const CV_IA64_AR41: CV_HREG_e = 3113;
pub const CV_IA64_AR42: CV_HREG_e = 3114;
pub const CV_IA64_AR43: CV_HREG_e = 3115;
pub const CV_IA64_AR45: CV_HREG_e = 3117;
pub const CV_IA64_AR46: CV_HREG_e = 3118;
pub const CV_IA64_AR47: CV_HREG_e = 3119;
pub const CV_IA64_AR48: CV_HREG_e = 3120;
pub const CV_IA64_AR49: CV_HREG_e = 3121;
pub const CV_IA64_AR50: CV_HREG_e = 3122;
pub const CV_IA64_AR51: CV_HREG_e = 3123;
pub const CV_IA64_AR52: CV_HREG_e = 3124;
pub const CV_IA64_AR53: CV_HREG_e = 3125;
pub const CV_IA64_AR54: CV_HREG_e = 3126;
pub const CV_IA64_AR55: CV_HREG_e = 3127;
pub const CV_IA64_AR56: CV_HREG_e = 3128;
pub const CV_IA64_AR57: CV_HREG_e = 3129;
pub const CV_IA64_AR58: CV_HREG_e = 3130;
pub const CV_IA64_AR59: CV_HREG_e = 3131;
pub const CV_IA64_AR60: CV_HREG_e = 3132;
pub const CV_IA64_AR61: CV_HREG_e = 3133;
pub const CV_IA64_AR62: CV_HREG_e = 3134;
pub const CV_IA64_AR63: CV_HREG_e = 3135;
pub const CV_IA64_AR67: CV_HREG_e = 3139;
pub const CV_IA64_AR68: CV_HREG_e = 3140;
pub const CV_IA64_AR69: CV_HREG_e = 3141;
pub const CV_IA64_AR70: CV_HREG_e = 3142;
pub const CV_IA64_AR71: CV_HREG_e = 3143;
pub const CV_IA64_AR72: CV_HREG_e = 3144;
pub const CV_IA64_AR73: CV_HREG_e = 3145;
pub const CV_IA64_AR74: CV_HREG_e = 3146;
pub const CV_IA64_AR75: CV_HREG_e = 3147;
pub const CV_IA64_AR76: CV_HREG_e = 3148;
pub const CV_IA64_AR77: CV_HREG_e = 3149;
pub const CV_IA64_AR78: CV_HREG_e = 3150;
pub const CV_IA64_AR79: CV_HREG_e = 3151;
pub const CV_IA64_AR8: CV_HREG_e = 3080;
pub const CV_IA64_AR80: CV_HREG_e = 3152;
pub const CV_IA64_AR81: CV_HREG_e = 3153;
pub const CV_IA64_AR82: CV_HREG_e = 3154;
pub const CV_IA64_AR83: CV_HREG_e = 3155;
pub const CV_IA64_AR84: CV_HREG_e = 3156;
pub const CV_IA64_AR85: CV_HREG_e = 3157;
pub const CV_IA64_AR86: CV_HREG_e = 3158;
pub const CV_IA64_AR87: CV_HREG_e = 3159;
pub const CV_IA64_AR88: CV_HREG_e = 3160;
pub const CV_IA64_AR89: CV_HREG_e = 3161;
pub const CV_IA64_AR9: CV_HREG_e = 3081;
pub const CV_IA64_AR90: CV_HREG_e = 3162;
pub const CV_IA64_AR91: CV_HREG_e = 3163;
pub const CV_IA64_AR92: CV_HREG_e = 3164;
pub const CV_IA64_AR93: CV_HREG_e = 3165;
pub const CV_IA64_AR94: CV_HREG_e = 3166;
pub const CV_IA64_AR95: CV_HREG_e = 3167;
pub const CV_IA64_AR96: CV_HREG_e = 3168;
pub const CV_IA64_AR97: CV_HREG_e = 3169;
pub const CV_IA64_AR98: CV_HREG_e = 3170;
pub const CV_IA64_AR99: CV_HREG_e = 3171;
pub const CV_IA64_ApCCV: CV_HREG_e = 3104;
pub const CV_IA64_ApDCR: CV_HREG_e = 4096;
pub const CV_IA64_ApEC: CV_HREG_e = 3138;
pub const CV_IA64_ApGPTA: CV_HREG_e = 4105;
pub const CV_IA64_ApITC: CV_HREG_e = 3116;
pub const CV_IA64_ApITM: CV_HREG_e = 4097;
pub const CV_IA64_ApIVA: CV_HREG_e = 4098;
pub const CV_IA64_ApKR0: CV_HREG_e = 3072;
pub const CV_IA64_ApKR1: CV_HREG_e = 3073;
pub const CV_IA64_ApKR2: CV_HREG_e = 3074;
pub const CV_IA64_ApKR3: CV_HREG_e = 3075;
pub const CV_IA64_ApKR4: CV_HREG_e = 3076;
pub const CV_IA64_ApKR5: CV_HREG_e = 3077;
pub const CV_IA64_ApKR6: CV_HREG_e = 3078;
pub const CV_IA64_ApKR7: CV_HREG_e = 3079;
pub const CV_IA64_ApLC: CV_HREG_e = 3137;
pub const CV_IA64_ApPTA: CV_HREG_e = 4104;
pub const CV_IA64_ApUNAT: CV_HREG_e = 3108;
pub const CV_IA64_Br0: CV_HREG_e = 512;
pub const CV_IA64_Br1: CV_HREG_e = 513;
pub const CV_IA64_Br2: CV_HREG_e = 514;
pub const CV_IA64_Br3: CV_HREG_e = 515;
pub const CV_IA64_Br4: CV_HREG_e = 516;
pub const CV_IA64_Br5: CV_HREG_e = 517;
pub const CV_IA64_Br6: CV_HREG_e = 518;
pub const CV_IA64_Br7: CV_HREG_e = 519;
pub const CV_IA64_CFLG: CV_HREG_e = 3099;
pub const CV_IA64_CPUID0: CV_HREG_e = 3328;
pub const CV_IA64_CPUID1: CV_HREG_e = 3329;
pub const CV_IA64_CPUID2: CV_HREG_e = 3330;
pub const CV_IA64_CPUID3: CV_HREG_e = 3331;
pub const CV_IA64_CPUID4: CV_HREG_e = 3332;
pub const CV_IA64_CR10: CV_HREG_e = 4106;
pub const CV_IA64_CR100: CV_HREG_e = 4196;
pub const CV_IA64_CR101: CV_HREG_e = 4197;
pub const CV_IA64_CR102: CV_HREG_e = 4198;
pub const CV_IA64_CR103: CV_HREG_e = 4199;
pub const CV_IA64_CR104: CV_HREG_e = 4200;
pub const CV_IA64_CR105: CV_HREG_e = 4201;
pub const CV_IA64_CR106: CV_HREG_e = 4202;
pub const CV_IA64_CR107: CV_HREG_e = 4203;
pub const CV_IA64_CR108: CV_HREG_e = 4204;
pub const CV_IA64_CR109: CV_HREG_e = 4205;
pub const CV_IA64_CR11: CV_HREG_e = 4107;
pub const CV_IA64_CR110: CV_HREG_e = 4206;
pub const CV_IA64_CR111: CV_HREG_e = 4207;
pub const CV_IA64_CR112: CV_HREG_e = 4208;
pub const CV_IA64_CR113: CV_HREG_e = 4209;
pub const CV_IA64_CR114: CV_HREG_e = 4210;
pub const CV_IA64_CR115: CV_HREG_e = 4211;
pub const CV_IA64_CR116: CV_HREG_e = 4212;
pub const CV_IA64_CR117: CV_HREG_e = 4213;
pub const CV_IA64_CR118: CV_HREG_e = 4214;
pub const CV_IA64_CR119: CV_HREG_e = 4215;
pub const CV_IA64_CR12: CV_HREG_e = 4108;
pub const CV_IA64_CR120: CV_HREG_e = 4216;
pub const CV_IA64_CR121: CV_HREG_e = 4217;
pub const CV_IA64_CR122: CV_HREG_e = 4218;
pub const CV_IA64_CR123: CV_HREG_e = 4219;
pub const CV_IA64_CR124: CV_HREG_e = 4220;
pub const CV_IA64_CR125: CV_HREG_e = 4221;
pub const CV_IA64_CR126: CV_HREG_e = 4222;
pub const CV_IA64_CR127: CV_HREG_e = 4223;
pub const CV_IA64_CR13: CV_HREG_e = 4109;
pub const CV_IA64_CR14: CV_HREG_e = 4110;
pub const CV_IA64_CR15: CV_HREG_e = 4111;
pub const CV_IA64_CR18: CV_HREG_e = 4114;
pub const CV_IA64_CR26: CV_HREG_e = 4122;
pub const CV_IA64_CR27: CV_HREG_e = 4123;
pub const CV_IA64_CR28: CV_HREG_e = 4124;
pub const CV_IA64_CR29: CV_HREG_e = 4125;
pub const CV_IA64_CR3: CV_HREG_e = 4099;
pub const CV_IA64_CR30: CV_HREG_e = 4126;
pub const CV_IA64_CR31: CV_HREG_e = 4127;
pub const CV_IA64_CR32: CV_HREG_e = 4128;
pub const CV_IA64_CR33: CV_HREG_e = 4129;
pub const CV_IA64_CR34: CV_HREG_e = 4130;
pub const CV_IA64_CR35: CV_HREG_e = 4131;
pub const CV_IA64_CR36: CV_HREG_e = 4132;
pub const CV_IA64_CR37: CV_HREG_e = 4133;
pub const CV_IA64_CR38: CV_HREG_e = 4134;
pub const CV_IA64_CR39: CV_HREG_e = 4135;
pub const CV_IA64_CR4: CV_HREG_e = 4100;
pub const CV_IA64_CR40: CV_HREG_e = 4136;
pub const CV_IA64_CR41: CV_HREG_e = 4137;
pub const CV_IA64_CR42: CV_HREG_e = 4138;
pub const CV_IA64_CR43: CV_HREG_e = 4139;
pub const CV_IA64_CR44: CV_HREG_e = 4140;
pub const CV_IA64_CR45: CV_HREG_e = 4141;
pub const CV_IA64_CR46: CV_HREG_e = 4142;
pub const CV_IA64_CR47: CV_HREG_e = 4143;
pub const CV_IA64_CR48: CV_HREG_e = 4144;
pub const CV_IA64_CR49: CV_HREG_e = 4145;
pub const CV_IA64_CR5: CV_HREG_e = 4101;
pub const CV_IA64_CR50: CV_HREG_e = 4146;
pub const CV_IA64_CR51: CV_HREG_e = 4147;
pub const CV_IA64_CR52: CV_HREG_e = 4148;
pub const CV_IA64_CR53: CV_HREG_e = 4149;
pub const CV_IA64_CR54: CV_HREG_e = 4150;
pub const CV_IA64_CR55: CV_HREG_e = 4151;
pub const CV_IA64_CR56: CV_HREG_e = 4152;
pub const CV_IA64_CR57: CV_HREG_e = 4153;
pub const CV_IA64_CR58: CV_HREG_e = 4154;
pub const CV_IA64_CR59: CV_HREG_e = 4155;
pub const CV_IA64_CR6: CV_HREG_e = 4102;
pub const CV_IA64_CR60: CV_HREG_e = 4156;
pub const CV_IA64_CR61: CV_HREG_e = 4157;
pub const CV_IA64_CR62: CV_HREG_e = 4158;
pub const CV_IA64_CR63: CV_HREG_e = 4159;
pub const CV_IA64_CR7: CV_HREG_e = 4103;
pub const CV_IA64_CR75: CV_HREG_e = 4171;
pub const CV_IA64_CR76: CV_HREG_e = 4172;
pub const CV_IA64_CR77: CV_HREG_e = 4173;
pub const CV_IA64_CR78: CV_HREG_e = 4174;
pub const CV_IA64_CR79: CV_HREG_e = 4175;
pub const CV_IA64_CR82: CV_HREG_e = 4178;
pub const CV_IA64_CR83: CV_HREG_e = 4179;
pub const CV_IA64_CR84: CV_HREG_e = 4180;
pub const CV_IA64_CR85: CV_HREG_e = 4181;
pub const CV_IA64_CR86: CV_HREG_e = 4182;
pub const CV_IA64_CR87: CV_HREG_e = 4183;
pub const CV_IA64_CR88: CV_HREG_e = 4184;
pub const CV_IA64_CR89: CV_HREG_e = 4185;
pub const CV_IA64_CR90: CV_HREG_e = 4186;
pub const CV_IA64_CR91: CV_HREG_e = 4187;
pub const CV_IA64_CR92: CV_HREG_e = 4188;
pub const CV_IA64_CR93: CV_HREG_e = 4189;
pub const CV_IA64_CR94: CV_HREG_e = 4190;
pub const CV_IA64_CR95: CV_HREG_e = 4191;
pub const CV_IA64_CR96: CV_HREG_e = 4192;
pub const CV_IA64_CR97: CV_HREG_e = 4193;
pub const CV_IA64_CR98: CV_HREG_e = 4194;
pub const CV_IA64_CR99: CV_HREG_e = 4195;
pub const CV_IA64_CSD: CV_HREG_e = 3097;
pub const CV_IA64_Cfm: CV_HREG_e = 1018;
pub const CV_IA64_DbD0: CV_HREG_e = 8576;
pub const CV_IA64_DbD1: CV_HREG_e = 8577;
pub const CV_IA64_DbD2: CV_HREG_e = 8578;
pub const CV_IA64_DbD3: CV_HREG_e = 8579;
pub const CV_IA64_DbD4: CV_HREG_e = 8580;
pub const CV_IA64_DbD5: CV_HREG_e = 8581;
pub const CV_IA64_DbD6: CV_HREG_e = 8582;
pub const CV_IA64_DbD7: CV_HREG_e = 8583;
pub const CV_IA64_DbI0: CV_HREG_e = 8448;
pub const CV_IA64_DbI1: CV_HREG_e = 8449;
pub const CV_IA64_DbI2: CV_HREG_e = 8450;
pub const CV_IA64_DbI3: CV_HREG_e = 8451;
pub const CV_IA64_DbI4: CV_HREG_e = 8452;
pub const CV_IA64_DbI5: CV_HREG_e = 8453;
pub const CV_IA64_DbI6: CV_HREG_e = 8454;
pub const CV_IA64_DbI7: CV_HREG_e = 8455;
pub const CV_IA64_EFLAG: CV_HREG_e = 3096;
pub const CV_IA64_FltF0: CV_HREG_e = 2048;
pub const CV_IA64_FltF1: CV_HREG_e = 2049;
pub const CV_IA64_FltF10: CV_HREG_e = 2058;
pub const CV_IA64_FltF100: CV_HREG_e = 2148;
pub const CV_IA64_FltF101: CV_HREG_e = 2149;
pub const CV_IA64_FltF102: CV_HREG_e = 2150;
pub const CV_IA64_FltF103: CV_HREG_e = 2151;
pub const CV_IA64_FltF104: CV_HREG_e = 2152;
pub const CV_IA64_FltF105: CV_HREG_e = 2153;
pub const CV_IA64_FltF106: CV_HREG_e = 2154;
pub const CV_IA64_FltF107: CV_HREG_e = 2155;
pub const CV_IA64_FltF108: CV_HREG_e = 2156;
pub const CV_IA64_FltF109: CV_HREG_e = 2157;
pub const CV_IA64_FltF11: CV_HREG_e = 2059;
pub const CV_IA64_FltF110: CV_HREG_e = 2158;
pub const CV_IA64_FltF111: CV_HREG_e = 2159;
pub const CV_IA64_FltF112: CV_HREG_e = 2160;
pub const CV_IA64_FltF113: CV_HREG_e = 2161;
pub const CV_IA64_FltF114: CV_HREG_e = 2162;
pub const CV_IA64_FltF115: CV_HREG_e = 2163;
pub const CV_IA64_FltF116: CV_HREG_e = 2164;
pub const CV_IA64_FltF117: CV_HREG_e = 2165;
pub const CV_IA64_FltF118: CV_HREG_e = 2166;
pub const CV_IA64_FltF119: CV_HREG_e = 2167;
pub const CV_IA64_FltF12: CV_HREG_e = 2060;
pub const CV_IA64_FltF120: CV_HREG_e = 2168;
pub const CV_IA64_FltF121: CV_HREG_e = 2169;
pub const CV_IA64_FltF122: CV_HREG_e = 2170;
pub const CV_IA64_FltF123: CV_HREG_e = 2171;
pub const CV_IA64_FltF124: CV_HREG_e = 2172;
pub const CV_IA64_FltF125: CV_HREG_e = 2173;
pub const CV_IA64_FltF126: CV_HREG_e = 2174;
pub const CV_IA64_FltF127: CV_HREG_e = 2175;
pub const CV_IA64_FltF13: CV_HREG_e = 2061;
pub const CV_IA64_FltF14: CV_HREG_e = 2062;
pub const CV_IA64_FltF15: CV_HREG_e = 2063;
pub const CV_IA64_FltF16: CV_HREG_e = 2064;
pub const CV_IA64_FltF17: CV_HREG_e = 2065;
pub const CV_IA64_FltF18: CV_HREG_e = 2066;
pub const CV_IA64_FltF19: CV_HREG_e = 2067;
pub const CV_IA64_FltF2: CV_HREG_e = 2050;
pub const CV_IA64_FltF20: CV_HREG_e = 2068;
pub const CV_IA64_FltF21: CV_HREG_e = 2069;
pub const CV_IA64_FltF22: CV_HREG_e = 2070;
pub const CV_IA64_FltF23: CV_HREG_e = 2071;
pub const CV_IA64_FltF24: CV_HREG_e = 2072;
pub const CV_IA64_FltF25: CV_HREG_e = 2073;
pub const CV_IA64_FltF26: CV_HREG_e = 2074;
pub const CV_IA64_FltF27: CV_HREG_e = 2075;
pub const CV_IA64_FltF28: CV_HREG_e = 2076;
pub const CV_IA64_FltF29: CV_HREG_e = 2077;
pub const CV_IA64_FltF3: CV_HREG_e = 2051;
pub const CV_IA64_FltF30: CV_HREG_e = 2078;
pub const CV_IA64_FltF31: CV_HREG_e = 2079;
pub const CV_IA64_FltF32: CV_HREG_e = 2080;
pub const CV_IA64_FltF33: CV_HREG_e = 2081;
pub const CV_IA64_FltF34: CV_HREG_e = 2082;
pub const CV_IA64_FltF35: CV_HREG_e = 2083;
pub const CV_IA64_FltF36: CV_HREG_e = 2084;
pub const CV_IA64_FltF37: CV_HREG_e = 2085;
pub const CV_IA64_FltF38: CV_HREG_e = 2086;
pub const CV_IA64_FltF39: CV_HREG_e = 2087;
pub const CV_IA64_FltF4: CV_HREG_e = 2052;
pub const CV_IA64_FltF40: CV_HREG_e = 2088;
pub const CV_IA64_FltF41: CV_HREG_e = 2089;
pub const CV_IA64_FltF42: CV_HREG_e = 2090;
pub const CV_IA64_FltF43: CV_HREG_e = 2091;
pub const CV_IA64_FltF44: CV_HREG_e = 2092;
pub const CV_IA64_FltF45: CV_HREG_e = 2093;
pub const CV_IA64_FltF46: CV_HREG_e = 2094;
pub const CV_IA64_FltF47: CV_HREG_e = 2095;
pub const CV_IA64_FltF48: CV_HREG_e = 2096;
pub const CV_IA64_FltF49: CV_HREG_e = 2097;
pub const CV_IA64_FltF5: CV_HREG_e = 2053;
pub const CV_IA64_FltF50: CV_HREG_e = 2098;
pub const CV_IA64_FltF51: CV_HREG_e = 2099;
pub const CV_IA64_FltF52: CV_HREG_e = 2100;
pub const CV_IA64_FltF53: CV_HREG_e = 2101;
pub const CV_IA64_FltF54: CV_HREG_e = 2102;
pub const CV_IA64_FltF55: CV_HREG_e = 2103;
pub const CV_IA64_FltF56: CV_HREG_e = 2104;
pub const CV_IA64_FltF57: CV_HREG_e = 2105;
pub const CV_IA64_FltF58: CV_HREG_e = 2106;
pub const CV_IA64_FltF59: CV_HREG_e = 2107;
pub const CV_IA64_FltF6: CV_HREG_e = 2054;
pub const CV_IA64_FltF60: CV_HREG_e = 2108;
pub const CV_IA64_FltF61: CV_HREG_e = 2109;
pub const CV_IA64_FltF62: CV_HREG_e = 2110;
pub const CV_IA64_FltF63: CV_HREG_e = 2111;
pub const CV_IA64_FltF64: CV_HREG_e = 2112;
pub const CV_IA64_FltF65: CV_HREG_e = 2113;
pub const CV_IA64_FltF66: CV_HREG_e = 2114;
pub const CV_IA64_FltF67: CV_HREG_e = 2115;
pub const CV_IA64_FltF68: CV_HREG_e = 2116;
pub const CV_IA64_FltF69: CV_HREG_e = 2117;
pub const CV_IA64_FltF7: CV_HREG_e = 2055;
pub const CV_IA64_FltF70: CV_HREG_e = 2118;
pub const CV_IA64_FltF71: CV_HREG_e = 2119;
pub const CV_IA64_FltF72: CV_HREG_e = 2120;
pub const CV_IA64_FltF73: CV_HREG_e = 2121;
pub const CV_IA64_FltF74: CV_HREG_e = 2122;
pub const CV_IA64_FltF75: CV_HREG_e = 2123;
pub const CV_IA64_FltF76: CV_HREG_e = 2124;
pub const CV_IA64_FltF77: CV_HREG_e = 2125;
pub const CV_IA64_FltF78: CV_HREG_e = 2126;
pub const CV_IA64_FltF79: CV_HREG_e = 2127;
pub const CV_IA64_FltF8: CV_HREG_e = 2056;
pub const CV_IA64_FltF80: CV_HREG_e = 2128;
pub const CV_IA64_FltF81: CV_HREG_e = 2129;
pub const CV_IA64_FltF82: CV_HREG_e = 2130;
pub const CV_IA64_FltF83: CV_HREG_e = 2131;
pub const CV_IA64_FltF84: CV_HREG_e = 2132;
pub const CV_IA64_FltF85: CV_HREG_e = 2133;
pub const CV_IA64_FltF86: CV_HREG_e = 2134;
pub const CV_IA64_FltF87: CV_HREG_e = 2135;
pub const CV_IA64_FltF88: CV_HREG_e = 2136;
pub const CV_IA64_FltF89: CV_HREG_e = 2137;
pub const CV_IA64_FltF9: CV_HREG_e = 2057;
pub const CV_IA64_FltF90: CV_HREG_e = 2138;
pub const CV_IA64_FltF91: CV_HREG_e = 2139;
pub const CV_IA64_FltF92: CV_HREG_e = 2140;
pub const CV_IA64_FltF93: CV_HREG_e = 2141;
pub const CV_IA64_FltF94: CV_HREG_e = 2142;
pub const CV_IA64_FltF95: CV_HREG_e = 2143;
pub const CV_IA64_FltF96: CV_HREG_e = 2144;
pub const CV_IA64_FltF97: CV_HREG_e = 2145;
pub const CV_IA64_FltF98: CV_HREG_e = 2146;
pub const CV_IA64_FltF99: CV_HREG_e = 2147;
pub const CV_IA64_IntH0: CV_HREG_e = 832;
pub const CV_IA64_IntH1: CV_HREG_e = 833;
pub const CV_IA64_IntH10: CV_HREG_e = 842;
pub const CV_IA64_IntH11: CV_HREG_e = 843;
pub const CV_IA64_IntH12: CV_HREG_e = 844;
pub const CV_IA64_IntH13: CV_HREG_e = 845;
pub const CV_IA64_IntH14: CV_HREG_e = 846;
pub const CV_IA64_IntH15: CV_HREG_e = 847;
pub const CV_IA64_IntH2: CV_HREG_e = 834;
pub const CV_IA64_IntH3: CV_HREG_e = 835;
pub const CV_IA64_IntH4: CV_HREG_e = 836;
pub const CV_IA64_IntH5: CV_HREG_e = 837;
pub const CV_IA64_IntH6: CV_HREG_e = 838;
pub const CV_IA64_IntH7: CV_HREG_e = 839;
pub const CV_IA64_IntH8: CV_HREG_e = 840;
pub const CV_IA64_IntH9: CV_HREG_e = 841;
pub const CV_IA64_IntR0: CV_HREG_e = 1024;
pub const CV_IA64_IntR1: CV_HREG_e = 1025;
pub const CV_IA64_IntR10: CV_HREG_e = 1034;
pub const CV_IA64_IntR100: CV_HREG_e = 1124;
pub const CV_IA64_IntR101: CV_HREG_e = 1125;
pub const CV_IA64_IntR102: CV_HREG_e = 1126;
pub const CV_IA64_IntR103: CV_HREG_e = 1127;
pub const CV_IA64_IntR104: CV_HREG_e = 1128;
pub const CV_IA64_IntR105: CV_HREG_e = 1129;
pub const CV_IA64_IntR106: CV_HREG_e = 1130;
pub const CV_IA64_IntR107: CV_HREG_e = 1131;
pub const CV_IA64_IntR108: CV_HREG_e = 1132;
pub const CV_IA64_IntR109: CV_HREG_e = 1133;
pub const CV_IA64_IntR11: CV_HREG_e = 1035;
pub const CV_IA64_IntR110: CV_HREG_e = 1134;
pub const CV_IA64_IntR111: CV_HREG_e = 1135;
pub const CV_IA64_IntR112: CV_HREG_e = 1136;
pub const CV_IA64_IntR113: CV_HREG_e = 1137;
pub const CV_IA64_IntR114: CV_HREG_e = 1138;
pub const CV_IA64_IntR115: CV_HREG_e = 1139;
pub const CV_IA64_IntR116: CV_HREG_e = 1140;
pub const CV_IA64_IntR117: CV_HREG_e = 1141;
pub const CV_IA64_IntR118: CV_HREG_e = 1142;
pub const CV_IA64_IntR119: CV_HREG_e = 1143;
pub const CV_IA64_IntR12: CV_HREG_e = 1036;
pub const CV_IA64_IntR120: CV_HREG_e = 1144;
pub const CV_IA64_IntR121: CV_HREG_e = 1145;
pub const CV_IA64_IntR122: CV_HREG_e = 1146;
pub const CV_IA64_IntR123: CV_HREG_e = 1147;
pub const CV_IA64_IntR124: CV_HREG_e = 1148;
pub const CV_IA64_IntR125: CV_HREG_e = 1149;
pub const CV_IA64_IntR126: CV_HREG_e = 1150;
pub const CV_IA64_IntR127: CV_HREG_e = 1151;
pub const CV_IA64_IntR13: CV_HREG_e = 1037;
pub const CV_IA64_IntR14: CV_HREG_e = 1038;
pub const CV_IA64_IntR15: CV_HREG_e = 1039;
pub const CV_IA64_IntR16: CV_HREG_e = 1040;
pub const CV_IA64_IntR17: CV_HREG_e = 1041;
pub const CV_IA64_IntR18: CV_HREG_e = 1042;
pub const CV_IA64_IntR19: CV_HREG_e = 1043;
pub const CV_IA64_IntR2: CV_HREG_e = 1026;
pub const CV_IA64_IntR20: CV_HREG_e = 1044;
pub const CV_IA64_IntR21: CV_HREG_e = 1045;
pub const CV_IA64_IntR22: CV_HREG_e = 1046;
pub const CV_IA64_IntR23: CV_HREG_e = 1047;
pub const CV_IA64_IntR24: CV_HREG_e = 1048;
pub const CV_IA64_IntR25: CV_HREG_e = 1049;
pub const CV_IA64_IntR26: CV_HREG_e = 1050;
pub const CV_IA64_IntR27: CV_HREG_e = 1051;
pub const CV_IA64_IntR28: CV_HREG_e = 1052;
pub const CV_IA64_IntR29: CV_HREG_e = 1053;
pub const CV_IA64_IntR3: CV_HREG_e = 1027;
pub const CV_IA64_IntR30: CV_HREG_e = 1054;
pub const CV_IA64_IntR31: CV_HREG_e = 1055;
pub const CV_IA64_IntR32: CV_HREG_e = 1056;
pub const CV_IA64_IntR33: CV_HREG_e = 1057;
pub const CV_IA64_IntR34: CV_HREG_e = 1058;
pub const CV_IA64_IntR35: CV_HREG_e = 1059;
pub const CV_IA64_IntR36: CV_HREG_e = 1060;
pub const CV_IA64_IntR37: CV_HREG_e = 1061;
pub const CV_IA64_IntR38: CV_HREG_e = 1062;
pub const CV_IA64_IntR39: CV_HREG_e = 1063;
pub const CV_IA64_IntR4: CV_HREG_e = 1028;
pub const CV_IA64_IntR40: CV_HREG_e = 1064;
pub const CV_IA64_IntR41: CV_HREG_e = 1065;
pub const CV_IA64_IntR42: CV_HREG_e = 1066;
pub const CV_IA64_IntR43: CV_HREG_e = 1067;
pub const CV_IA64_IntR44: CV_HREG_e = 1068;
pub const CV_IA64_IntR45: CV_HREG_e = 1069;
pub const CV_IA64_IntR46: CV_HREG_e = 1070;
pub const CV_IA64_IntR47: CV_HREG_e = 1071;
pub const CV_IA64_IntR48: CV_HREG_e = 1072;
pub const CV_IA64_IntR49: CV_HREG_e = 1073;
pub const CV_IA64_IntR5: CV_HREG_e = 1029;
pub const CV_IA64_IntR50: CV_HREG_e = 1074;
pub const CV_IA64_IntR51: CV_HREG_e = 1075;
pub const CV_IA64_IntR52: CV_HREG_e = 1076;
pub const CV_IA64_IntR53: CV_HREG_e = 1077;
pub const CV_IA64_IntR54: CV_HREG_e = 1078;
pub const CV_IA64_IntR55: CV_HREG_e = 1079;
pub const CV_IA64_IntR56: CV_HREG_e = 1080;
pub const CV_IA64_IntR57: CV_HREG_e = 1081;
pub const CV_IA64_IntR58: CV_HREG_e = 1082;
pub const CV_IA64_IntR59: CV_HREG_e = 1083;
pub const CV_IA64_IntR6: CV_HREG_e = 1030;
pub const CV_IA64_IntR60: CV_HREG_e = 1084;
pub const CV_IA64_IntR61: CV_HREG_e = 1085;
pub const CV_IA64_IntR62: CV_HREG_e = 1086;
pub const CV_IA64_IntR63: CV_HREG_e = 1087;
pub const CV_IA64_IntR64: CV_HREG_e = 1088;
pub const CV_IA64_IntR65: CV_HREG_e = 1089;
pub const CV_IA64_IntR66: CV_HREG_e = 1090;
pub const CV_IA64_IntR67: CV_HREG_e = 1091;
pub const CV_IA64_IntR68: CV_HREG_e = 1092;
pub const CV_IA64_IntR69: CV_HREG_e = 1093;
pub const CV_IA64_IntR7: CV_HREG_e = 1031;
pub const CV_IA64_IntR70: CV_HREG_e = 1094;
pub const CV_IA64_IntR71: CV_HREG_e = 1095;
pub const CV_IA64_IntR72: CV_HREG_e = 1096;
pub const CV_IA64_IntR73: CV_HREG_e = 1097;
pub const CV_IA64_IntR74: CV_HREG_e = 1098;
pub const CV_IA64_IntR75: CV_HREG_e = 1099;
pub const CV_IA64_IntR76: CV_HREG_e = 1100;
pub const CV_IA64_IntR77: CV_HREG_e = 1101;
pub const CV_IA64_IntR78: CV_HREG_e = 1102;
pub const CV_IA64_IntR79: CV_HREG_e = 1103;
pub const CV_IA64_IntR8: CV_HREG_e = 1032;
pub const CV_IA64_IntR80: CV_HREG_e = 1104;
pub const CV_IA64_IntR81: CV_HREG_e = 1105;
pub const CV_IA64_IntR82: CV_HREG_e = 1106;
pub const CV_IA64_IntR83: CV_HREG_e = 1107;
pub const CV_IA64_IntR84: CV_HREG_e = 1108;
pub const CV_IA64_IntR85: CV_HREG_e = 1109;
pub const CV_IA64_IntR86: CV_HREG_e = 1110;
pub const CV_IA64_IntR87: CV_HREG_e = 1111;
pub const CV_IA64_IntR88: CV_HREG_e = 1112;
pub const CV_IA64_IntR89: CV_HREG_e = 1113;
pub const CV_IA64_IntR9: CV_HREG_e = 1033;
pub const CV_IA64_IntR90: CV_HREG_e = 1114;
pub const CV_IA64_IntR91: CV_HREG_e = 1115;
pub const CV_IA64_IntR92: CV_HREG_e = 1116;
pub const CV_IA64_IntR93: CV_HREG_e = 1117;
pub const CV_IA64_IntR94: CV_HREG_e = 1118;
pub const CV_IA64_IntR95: CV_HREG_e = 1119;
pub const CV_IA64_IntR96: CV_HREG_e = 1120;
pub const CV_IA64_IntR97: CV_HREG_e = 1121;
pub const CV_IA64_IntR98: CV_HREG_e = 1122;
pub const CV_IA64_IntR99: CV_HREG_e = 1123;
pub const CV_IA64_Ip: CV_HREG_e = 1016;
pub const CV_IA64_NOREG: CV_HREG_e = 0;
pub const CV_IA64_Nats: CV_HREG_e = 1020;
pub const CV_IA64_Nats2: CV_HREG_e = 1021;
pub const CV_IA64_Nats3: CV_HREG_e = 1022;
pub const CV_IA64_P0: CV_HREG_e = 704;
pub const CV_IA64_P1: CV_HREG_e = 705;
pub const CV_IA64_P10: CV_HREG_e = 714;
pub const CV_IA64_P11: CV_HREG_e = 715;
pub const CV_IA64_P12: CV_HREG_e = 716;
pub const CV_IA64_P13: CV_HREG_e = 717;
pub const CV_IA64_P14: CV_HREG_e = 718;
pub const CV_IA64_P15: CV_HREG_e = 719;
pub const CV_IA64_P16: CV_HREG_e = 720;
pub const CV_IA64_P17: CV_HREG_e = 721;
pub const CV_IA64_P18: CV_HREG_e = 722;
pub const CV_IA64_P19: CV_HREG_e = 723;
pub const CV_IA64_P2: CV_HREG_e = 706;
pub const CV_IA64_P20: CV_HREG_e = 724;
pub const CV_IA64_P21: CV_HREG_e = 725;
pub const CV_IA64_P22: CV_HREG_e = 726;
pub const CV_IA64_P23: CV_HREG_e = 727;
pub const CV_IA64_P24: CV_HREG_e = 728;
pub const CV_IA64_P25: CV_HREG_e = 729;
pub const CV_IA64_P26: CV_HREG_e = 730;
pub const CV_IA64_P27: CV_HREG_e = 731;
pub const CV_IA64_P28: CV_HREG_e = 732;
pub const CV_IA64_P29: CV_HREG_e = 733;
pub const CV_IA64_P3: CV_HREG_e = 707;
pub const CV_IA64_P30: CV_HREG_e = 734;
pub const CV_IA64_P31: CV_HREG_e = 735;
pub const CV_IA64_P32: CV_HREG_e = 736;
pub const CV_IA64_P33: CV_HREG_e = 737;
pub const CV_IA64_P34: CV_HREG_e = 738;
pub const CV_IA64_P35: CV_HREG_e = 739;
pub const CV_IA64_P36: CV_HREG_e = 740;
pub const CV_IA64_P37: CV_HREG_e = 741;
pub const CV_IA64_P38: CV_HREG_e = 742;
pub const CV_IA64_P39: CV_HREG_e = 743;
pub const CV_IA64_P4: CV_HREG_e = 708;
pub const CV_IA64_P40: CV_HREG_e = 744;
pub const CV_IA64_P41: CV_HREG_e = 745;
pub const CV_IA64_P42: CV_HREG_e = 746;
pub const CV_IA64_P43: CV_HREG_e = 747;
pub const CV_IA64_P44: CV_HREG_e = 748;
pub const CV_IA64_P45: CV_HREG_e = 749;
pub const CV_IA64_P46: CV_HREG_e = 750;
pub const CV_IA64_P47: CV_HREG_e = 751;
pub const CV_IA64_P48: CV_HREG_e = 752;
pub const CV_IA64_P49: CV_HREG_e = 753;
pub const CV_IA64_P5: CV_HREG_e = 709;
pub const CV_IA64_P50: CV_HREG_e = 754;
pub const CV_IA64_P51: CV_HREG_e = 755;
pub const CV_IA64_P52: CV_HREG_e = 756;
pub const CV_IA64_P53: CV_HREG_e = 757;
pub const CV_IA64_P54: CV_HREG_e = 758;
pub const CV_IA64_P55: CV_HREG_e = 759;
pub const CV_IA64_P56: CV_HREG_e = 760;
pub const CV_IA64_P57: CV_HREG_e = 761;
pub const CV_IA64_P58: CV_HREG_e = 762;
pub const CV_IA64_P59: CV_HREG_e = 763;
pub const CV_IA64_P6: CV_HREG_e = 710;
pub const CV_IA64_P60: CV_HREG_e = 764;
pub const CV_IA64_P61: CV_HREG_e = 765;
pub const CV_IA64_P62: CV_HREG_e = 766;
pub const CV_IA64_P63: CV_HREG_e = 767;
pub const CV_IA64_P7: CV_HREG_e = 711;
pub const CV_IA64_P8: CV_HREG_e = 712;
pub const CV_IA64_P9: CV_HREG_e = 713;
pub const CV_IA64_PFC0: CV_HREG_e = 7424;
pub const CV_IA64_PFC1: CV_HREG_e = 7425;
pub const CV_IA64_PFC10: CV_HREG_e = 7434;
pub const CV_IA64_PFC11: CV_HREG_e = 7435;
pub const CV_IA64_PFC12: CV_HREG_e = 7436;
pub const CV_IA64_PFC13: CV_HREG_e = 7437;
pub const CV_IA64_PFC14: CV_HREG_e = 7438;
pub const CV_IA64_PFC15: CV_HREG_e = 7439;
pub const CV_IA64_PFC2: CV_HREG_e = 7426;
pub const CV_IA64_PFC3: CV_HREG_e = 7427;
pub const CV_IA64_PFC4: CV_HREG_e = 7428;
pub const CV_IA64_PFC5: CV_HREG_e = 7429;
pub const CV_IA64_PFC6: CV_HREG_e = 7430;
pub const CV_IA64_PFC7: CV_HREG_e = 7431;
pub const CV_IA64_PFC8: CV_HREG_e = 7432;
pub const CV_IA64_PFC9: CV_HREG_e = 7433;
pub const CV_IA64_PFD0: CV_HREG_e = 7168;
pub const CV_IA64_PFD1: CV_HREG_e = 7169;
pub const CV_IA64_PFD10: CV_HREG_e = 7178;
pub const CV_IA64_PFD11: CV_HREG_e = 7179;
pub const CV_IA64_PFD12: CV_HREG_e = 7180;
pub const CV_IA64_PFD13: CV_HREG_e = 7181;
pub const CV_IA64_PFD14: CV_HREG_e = 7182;
pub const CV_IA64_PFD15: CV_HREG_e = 7183;
pub const CV_IA64_PFD16: CV_HREG_e = 7184;
pub const CV_IA64_PFD17: CV_HREG_e = 7185;
pub const CV_IA64_PFD2: CV_HREG_e = 7170;
pub const CV_IA64_PFD3: CV_HREG_e = 7171;
pub const CV_IA64_PFD4: CV_HREG_e = 7172;
pub const CV_IA64_PFD5: CV_HREG_e = 7173;
pub const CV_IA64_PFD6: CV_HREG_e = 7174;
pub const CV_IA64_PFD7: CV_HREG_e = 7175;
pub const CV_IA64_PFD8: CV_HREG_e = 7176;
pub const CV_IA64_PFD9: CV_HREG_e = 7177;
pub const CV_IA64_Pkr0: CV_HREG_e = 5120;
pub const CV_IA64_Pkr1: CV_HREG_e = 5121;
pub const CV_IA64_Pkr10: CV_HREG_e = 5130;
pub const CV_IA64_Pkr11: CV_HREG_e = 5131;
pub const CV_IA64_Pkr12: CV_HREG_e = 5132;
pub const CV_IA64_Pkr13: CV_HREG_e = 5133;
pub const CV_IA64_Pkr14: CV_HREG_e = 5134;
pub const CV_IA64_Pkr15: CV_HREG_e = 5135;
pub const CV_IA64_Pkr2: CV_HREG_e = 5122;
pub const CV_IA64_Pkr3: CV_HREG_e = 5123;
pub const CV_IA64_Pkr4: CV_HREG_e = 5124;
pub const CV_IA64_Pkr5: CV_HREG_e = 5125;
pub const CV_IA64_Pkr6: CV_HREG_e = 5126;
pub const CV_IA64_Pkr7: CV_HREG_e = 5127;
pub const CV_IA64_Pkr8: CV_HREG_e = 5128;
pub const CV_IA64_Pkr9: CV_HREG_e = 5129;
pub const CV_IA64_Preds: CV_HREG_e = 768;
pub const CV_IA64_Psr: CV_HREG_e = 1019;
pub const CV_IA64_Rr0: CV_HREG_e = 6144;
pub const CV_IA64_Rr1: CV_HREG_e = 6145;
pub const CV_IA64_Rr2: CV_HREG_e = 6146;
pub const CV_IA64_Rr3: CV_HREG_e = 6147;
pub const CV_IA64_Rr4: CV_HREG_e = 6148;
pub const CV_IA64_Rr5: CV_HREG_e = 6149;
pub const CV_IA64_Rr6: CV_HREG_e = 6150;
pub const CV_IA64_Rr7: CV_HREG_e = 6151;
pub const CV_IA64_RsBSP: CV_HREG_e = 3089;
pub const CV_IA64_RsBSPSTORE: CV_HREG_e = 3090;
pub const CV_IA64_RsPFS: CV_HREG_e = 3136;
pub const CV_IA64_RsRNAT: CV_HREG_e = 3091;
pub const CV_IA64_RsRSC: CV_HREG_e = 3088;
pub const CV_IA64_SSD: CV_HREG_e = 3098;
pub const CV_IA64_SaCMCV: CV_HREG_e = 4170;
pub const CV_IA64_SaEOI: CV_HREG_e = 4163;
pub const CV_IA64_SaIRR0: CV_HREG_e = 4164;
pub const CV_IA64_SaIRR1: CV_HREG_e = 4165;
pub const CV_IA64_SaIRR2: CV_HREG_e = 4166;
pub const CV_IA64_SaIRR3: CV_HREG_e = 4167;
pub const CV_IA64_SaITV: CV_HREG_e = 4168;
pub const CV_IA64_SaIVR: CV_HREG_e = 4161;
pub const CV_IA64_SaLID: CV_HREG_e = 4160;
pub const CV_IA64_SaLRR0: CV_HREG_e = 4176;
pub const CV_IA64_SaLRR1: CV_HREG_e = 4177;
pub const CV_IA64_SaPMV: CV_HREG_e = 4169;
pub const CV_IA64_SaTPR: CV_HREG_e = 4162;
pub const CV_IA64_StFCR: CV_HREG_e = 3093;
pub const CV_IA64_StFDR: CV_HREG_e = 3102;
pub const CV_IA64_StFIR: CV_HREG_e = 3101;
pub const CV_IA64_StFPSR: CV_HREG_e = 3112;
pub const CV_IA64_StFSR: CV_HREG_e = 3100;
pub const CV_IA64_StIFA: CV_HREG_e = 4116;
pub const CV_IA64_StIFS: CV_HREG_e = 4119;
pub const CV_IA64_StIHA: CV_HREG_e = 4121;
pub const CV_IA64_StIIM: CV_HREG_e = 4120;
pub const CV_IA64_StIIP: CV_HREG_e = 4115;
pub const CV_IA64_StIIPA: CV_HREG_e = 4118;
pub const CV_IA64_StIPSR: CV_HREG_e = 4112;
pub const CV_IA64_StISR: CV_HREG_e = 4113;
pub const CV_IA64_StITIR: CV_HREG_e = 4117;
pub const CV_IA64_TrD0: CV_HREG_e = 8320;
pub const CV_IA64_TrD1: CV_HREG_e = 8321;
pub const CV_IA64_TrD2: CV_HREG_e = 8322;
pub const CV_IA64_TrD3: CV_HREG_e = 8323;
pub const CV_IA64_TrD4: CV_HREG_e = 8324;
pub const CV_IA64_TrD5: CV_HREG_e = 8325;
pub const CV_IA64_TrD6: CV_HREG_e = 8326;
pub const CV_IA64_TrD7: CV_HREG_e = 8327;
pub const CV_IA64_TrI0: CV_HREG_e = 8192;
pub const CV_IA64_TrI1: CV_HREG_e = 8193;
pub const CV_IA64_TrI2: CV_HREG_e = 8194;
pub const CV_IA64_TrI3: CV_HREG_e = 8195;
pub const CV_IA64_TrI4: CV_HREG_e = 8196;
pub const CV_IA64_TrI5: CV_HREG_e = 8197;
pub const CV_IA64_TrI6: CV_HREG_e = 8198;
pub const CV_IA64_TrI7: CV_HREG_e = 8199;
pub const CV_IA64_Umask: CV_HREG_e = 1017;
pub const CV_JAVA_PC: CV_HREG_e = 1;
pub const CV_M32R_ACHI: CV_HREG_e = 32;
pub const CV_M32R_ACLO: CV_HREG_e = 33;
pub const CV_M32R_BPC: CV_HREG_e = 31;
pub const CV_M32R_CBR: CV_HREG_e = 27;
pub const CV_M32R_NOREG: CV_HREG_e = 0;
pub const CV_M32R_PC: CV_HREG_e = 34;
pub const CV_M32R_PSW: CV_HREG_e = 26;
pub const CV_M32R_R0: CV_HREG_e = 10;
pub const CV_M32R_R1: CV_HREG_e = 11;
pub const CV_M32R_R10: CV_HREG_e = 20;
pub const CV_M32R_R11: CV_HREG_e = 21;
pub const CV_M32R_R12: CV_HREG_e = 22;
pub const CV_M32R_R13: CV_HREG_e = 23;
pub const CV_M32R_R14: CV_HREG_e = 24;
pub const CV_M32R_R15: CV_HREG_e = 25;
pub const CV_M32R_R2: CV_HREG_e = 12;
pub const CV_M32R_R3: CV_HREG_e = 13;
pub const CV_M32R_R4: CV_HREG_e = 14;
pub const CV_M32R_R5: CV_HREG_e = 15;
pub const CV_M32R_R6: CV_HREG_e = 16;
pub const CV_M32R_R7: CV_HREG_e = 17;
pub const CV_M32R_R8: CV_HREG_e = 18;
pub const CV_M32R_R9: CV_HREG_e = 19;
pub const CV_M32R_SPI: CV_HREG_e = 28;
pub const CV_M32R_SPO: CV_HREG_e = 30;
pub const CV_M32R_SPU: CV_HREG_e = 29;
pub const CV_M4_Fir: CV_HREG_e = 50;
pub const CV_M4_FltF0: CV_HREG_e = 60;
pub const CV_M4_FltF1: CV_HREG_e = 61;
pub const CV_M4_FltF10: CV_HREG_e = 70;
pub const CV_M4_FltF11: CV_HREG_e = 71;
pub const CV_M4_FltF12: CV_HREG_e = 72;
pub const CV_M4_FltF13: CV_HREG_e = 73;
pub const CV_M4_FltF14: CV_HREG_e = 74;
pub const CV_M4_FltF15: CV_HREG_e = 75;
pub const CV_M4_FltF16: CV_HREG_e = 76;
pub const CV_M4_FltF17: CV_HREG_e = 77;
pub const CV_M4_FltF18: CV_HREG_e = 78;
pub const CV_M4_FltF19: CV_HREG_e = 79;
pub const CV_M4_FltF2: CV_HREG_e = 62;
pub const CV_M4_FltF20: CV_HREG_e = 80;
pub const CV_M4_FltF21: CV_HREG_e = 81;
pub const CV_M4_FltF22: CV_HREG_e = 82;
pub const CV_M4_FltF23: CV_HREG_e = 83;
pub const CV_M4_FltF24: CV_HREG_e = 84;
pub const CV_M4_FltF25: CV_HREG_e = 85;
pub const CV_M4_FltF26: CV_HREG_e = 86;
pub const CV_M4_FltF27: CV_HREG_e = 87;
pub const CV_M4_FltF28: CV_HREG_e = 88;
pub const CV_M4_FltF29: CV_HREG_e = 89;
pub const CV_M4_FltF3: CV_HREG_e = 63;
pub const CV_M4_FltF30: CV_HREG_e = 90;
pub const CV_M4_FltF31: CV_HREG_e = 91;
pub const CV_M4_FltF4: CV_HREG_e = 64;
pub const CV_M4_FltF5: CV_HREG_e = 65;
pub const CV_M4_FltF6: CV_HREG_e = 66;
pub const CV_M4_FltF7: CV_HREG_e = 67;
pub const CV_M4_FltF8: CV_HREG_e = 68;
pub const CV_M4_FltF9: CV_HREG_e = 69;
pub const CV_M4_FltFsr: CV_HREG_e = 92;
pub const CV_M4_IntA0: CV_HREG_e = 14;
pub const CV_M4_IntA1: CV_HREG_e = 15;
pub const CV_M4_IntA2: CV_HREG_e = 16;
pub const CV_M4_IntA3: CV_HREG_e = 17;
pub const CV_M4_IntAT: CV_HREG_e = 11;
pub const CV_M4_IntGP: CV_HREG_e = 38;
pub const CV_M4_IntHI: CV_HREG_e = 43;
pub const CV_M4_IntKT0: CV_HREG_e = 36;
pub const CV_M4_IntKT1: CV_HREG_e = 37;
pub const CV_M4_IntLO: CV_HREG_e = 42;
pub const CV_M4_IntRA: CV_HREG_e = 41;
pub const CV_M4_IntS0: CV_HREG_e = 26;
pub const CV_M4_IntS1: CV_HREG_e = 27;
pub const CV_M4_IntS2: CV_HREG_e = 28;
pub const CV_M4_IntS3: CV_HREG_e = 29;
pub const CV_M4_IntS4: CV_HREG_e = 30;
pub const CV_M4_IntS5: CV_HREG_e = 31;
pub const CV_M4_IntS6: CV_HREG_e = 32;
pub const CV_M4_IntS7: CV_HREG_e = 33;
pub const CV_M4_IntS8: CV_HREG_e = 40;
pub const CV_M4_IntSP: CV_HREG_e = 39;
pub const CV_M4_IntT0: CV_HREG_e = 18;
pub const CV_M4_IntT1: CV_HREG_e = 19;
pub const CV_M4_IntT2: CV_HREG_e = 20;
pub const CV_M4_IntT3: CV_HREG_e = 21;
pub const CV_M4_IntT4: CV_HREG_e = 22;
pub const CV_M4_IntT5: CV_HREG_e = 23;
pub const CV_M4_IntT6: CV_HREG_e = 24;
pub const CV_M4_IntT7: CV_HREG_e = 25;
pub const CV_M4_IntT8: CV_HREG_e = 34;
pub const CV_M4_IntT9: CV_HREG_e = 35;
pub const CV_M4_IntV0: CV_HREG_e = 12;
pub const CV_M4_IntV1: CV_HREG_e = 13;
pub const CV_M4_IntZERO: CV_HREG_e = 10;
pub const CV_M4_NOREG: CV_HREG_e = 0;
pub const CV_M4_Psr: CV_HREG_e = 51;
pub const CV_MOD_ATOMIC: CV_modifier_e = 4;
pub const CV_MOD_CONST: CV_modifier_e = 1;
pub const CV_MOD_HLSL_CENTER: CV_modifier_e = 522;
pub const CV_MOD_HLSL_CENTROID: CV_modifier_e = 518;
pub const CV_MOD_HLSL_CONSTINTERP: CV_modifier_e = 519;
pub const CV_MOD_HLSL_LINE: CV_modifier_e = 513;
pub const CV_MOD_HLSL_LINEADJ: CV_modifier_e = 515;
pub const CV_MOD_HLSL_LINEAR: CV_modifier_e = 517;
pub const CV_MOD_HLSL_NOPERSPECTIVE: CV_modifier_e = 520;
pub const CV_MOD_HLSL_PRECISE: CV_modifier_e = 525;
pub const CV_MOD_HLSL_SAMPLE: CV_modifier_e = 521;
pub const CV_MOD_HLSL_SNORM: CV_modifier_e = 523;
pub const CV_MOD_HLSL_TRIANGLE: CV_modifier_e = 514;
pub const CV_MOD_HLSL_TRIANGLEADJ: CV_modifier_e = 516;
pub const CV_MOD_HLSL_UAV_GLOBALLY_COHERENT: CV_modifier_e = 526;
pub const CV_MOD_HLSL_UNIFORM: CV_modifier_e = 512;
pub const CV_MOD_HLSL_UNORM: CV_modifier_e = 524;
pub const CV_MOD_INVALID: CV_modifier_e = 0;
pub const CV_MOD_UNALIGNED: CV_modifier_e = 3;
pub const CV_MOD_VOLATILE: CV_modifier_e = 2;
pub const CV_PPC_ASR: CV_HREG_e = 280;
pub const CV_PPC_BAT0L: CV_HREG_e = 629;
pub const CV_PPC_BAT0U: CV_HREG_e = 628;
pub const CV_PPC_BAT1L: CV_HREG_e = 631;
pub const CV_PPC_BAT1U: CV_HREG_e = 630;
pub const CV_PPC_BAT2L: CV_HREG_e = 633;
pub const CV_PPC_BAT2U: CV_HREG_e = 632;
pub const CV_PPC_BAT3L: CV_HREG_e = 635;
pub const CV_PPC_BAT3U: CV_HREG_e = 634;
pub const CV_PPC_COMPARE: CV_HREG_e = 110;
pub const CV_PPC_COUNT: CV_HREG_e = 111;
pub const CV_PPC_CR: CV_HREG_e = 33;
pub const CV_PPC_CR0: CV_HREG_e = 34;
pub const CV_PPC_CR1: CV_HREG_e = 35;
pub const CV_PPC_CR2: CV_HREG_e = 36;
pub const CV_PPC_CR3: CV_HREG_e = 37;
pub const CV_PPC_CR4: CV_HREG_e = 38;
pub const CV_PPC_CR5: CV_HREG_e = 39;
pub const CV_PPC_CR6: CV_HREG_e = 40;
pub const CV_PPC_CR7: CV_HREG_e = 41;
pub const CV_PPC_CTR: CV_HREG_e = 109;
pub const CV_PPC_DAR: CV_HREG_e = 119;
pub const CV_PPC_DBAT0L: CV_HREG_e = 637;
pub const CV_PPC_DBAT0U: CV_HREG_e = 636;
pub const CV_PPC_DBAT1L: CV_HREG_e = 639;
pub const CV_PPC_DBAT1U: CV_HREG_e = 638;
pub const CV_PPC_DBAT2L: CV_HREG_e = 641;
pub const CV_PPC_DBAT2U: CV_HREG_e = 640;
pub const CV_PPC_DBAT3L: CV_HREG_e = 643;
pub const CV_PPC_DBAT3U: CV_HREG_e = 642;
pub const CV_PPC_DCMP: CV_HREG_e = 1077;
pub const CV_PPC_DEC: CV_HREG_e = 122;
pub const CV_PPC_DMISS: CV_HREG_e = 1076;
pub const CV_PPC_DSISR: CV_HREG_e = 118;
pub const CV_PPC_EAR: CV_HREG_e = 382;
pub const CV_PPC_FPR0: CV_HREG_e = 42;
pub const CV_PPC_FPR1: CV_HREG_e = 43;
pub const CV_PPC_FPR10: CV_HREG_e = 52;
pub const CV_PPC_FPR11: CV_HREG_e = 53;
pub const CV_PPC_FPR12: CV_HREG_e = 54;
pub const CV_PPC_FPR13: CV_HREG_e = 55;
pub const CV_PPC_FPR14: CV_HREG_e = 56;
pub const CV_PPC_FPR15: CV_HREG_e = 57;
pub const CV_PPC_FPR16: CV_HREG_e = 58;
pub const CV_PPC_FPR17: CV_HREG_e = 59;
pub const CV_PPC_FPR18: CV_HREG_e = 60;
pub const CV_PPC_FPR19: CV_HREG_e = 61;
pub const CV_PPC_FPR2: CV_HREG_e = 44;
pub const CV_PPC_FPR20: CV_HREG_e = 62;
pub const CV_PPC_FPR21: CV_HREG_e = 63;
pub const CV_PPC_FPR22: CV_HREG_e = 64;
pub const CV_PPC_FPR23: CV_HREG_e = 65;
pub const CV_PPC_FPR24: CV_HREG_e = 66;
pub const CV_PPC_FPR25: CV_HREG_e = 67;
pub const CV_PPC_FPR26: CV_HREG_e = 68;
pub const CV_PPC_FPR27: CV_HREG_e = 69;
pub const CV_PPC_FPR28: CV_HREG_e = 70;
pub const CV_PPC_FPR29: CV_HREG_e = 71;
pub const CV_PPC_FPR3: CV_HREG_e = 45;
pub const CV_PPC_FPR30: CV_HREG_e = 72;
pub const CV_PPC_FPR31: CV_HREG_e = 73;
pub const CV_PPC_FPR4: CV_HREG_e = 46;
pub const CV_PPC_FPR5: CV_HREG_e = 47;
pub const CV_PPC_FPR6: CV_HREG_e = 48;
pub const CV_PPC_FPR7: CV_HREG_e = 49;
pub const CV_PPC_FPR8: CV_HREG_e = 50;
pub const CV_PPC_FPR9: CV_HREG_e = 51;
pub const CV_PPC_FPSCR: CV_HREG_e = 74;
pub const CV_PPC_GPR0: CV_HREG_e = 1;
pub const CV_PPC_GPR1: CV_HREG_e = 2;
pub const CV_PPC_GPR10: CV_HREG_e = 11;
pub const CV_PPC_GPR11: CV_HREG_e = 12;
pub const CV_PPC_GPR12: CV_HREG_e = 13;
pub const CV_PPC_GPR13: CV_HREG_e = 14;
pub const CV_PPC_GPR14: CV_HREG_e = 15;
pub const CV_PPC_GPR15: CV_HREG_e = 16;
pub const CV_PPC_GPR16: CV_HREG_e = 17;
pub const CV_PPC_GPR17: CV_HREG_e = 18;
pub const CV_PPC_GPR18: CV_HREG_e = 19;
pub const CV_PPC_GPR19: CV_HREG_e = 20;
pub const CV_PPC_GPR2: CV_HREG_e = 3;
pub const CV_PPC_GPR20: CV_HREG_e = 21;
pub const CV_PPC_GPR21: CV_HREG_e = 22;
pub const CV_PPC_GPR22: CV_HREG_e = 23;
pub const CV_PPC_GPR23: CV_HREG_e = 24;
pub const CV_PPC_GPR24: CV_HREG_e = 25;
pub const CV_PPC_GPR25: CV_HREG_e = 26;
pub const CV_PPC_GPR26: CV_HREG_e = 27;
pub const CV_PPC_GPR27: CV_HREG_e = 28;
pub const CV_PPC_GPR28: CV_HREG_e = 29;
pub const CV_PPC_GPR29: CV_HREG_e = 30;
pub const CV_PPC_GPR3: CV_HREG_e = 4;
pub const CV_PPC_GPR30: CV_HREG_e = 31;
pub const CV_PPC_GPR31: CV_HREG_e = 32;
pub const CV_PPC_GPR4: CV_HREG_e = 5;
pub const CV_PPC_GPR5: CV_HREG_e = 6;
pub const CV_PPC_GPR6: CV_HREG_e = 7;
pub const CV_PPC_GPR7: CV_HREG_e = 8;
pub const CV_PPC_GPR8: CV_HREG_e = 9;
pub const CV_PPC_GPR9: CV_HREG_e = 10;
pub const CV_PPC_HASH1: CV_HREG_e = 1078;
pub const CV_PPC_HASH2: CV_HREG_e = 1079;
pub const CV_PPC_HID0: CV_HREG_e = 1108;
pub const CV_PPC_HID1: CV_HREG_e = 1109;
pub const CV_PPC_HID10: CV_HREG_e = 1118;
pub const CV_PPC_HID11: CV_HREG_e = 1119;
pub const CV_PPC_HID12: CV_HREG_e = 1120;
pub const CV_PPC_HID13: CV_HREG_e = 1121;
pub const CV_PPC_HID14: CV_HREG_e = 1122;
pub const CV_PPC_HID15: CV_HREG_e = 1123;
pub const CV_PPC_HID2: CV_HREG_e = 1110;
pub const CV_PPC_HID3: CV_HREG_e = 1111;
pub const CV_PPC_HID4: CV_HREG_e = 1112;
pub const CV_PPC_HID5: CV_HREG_e = 1113;
pub const CV_PPC_HID6: CV_HREG_e = 1114;
pub const CV_PPC_HID7: CV_HREG_e = 1115;
pub const CV_PPC_HID8: CV_HREG_e = 1116;
pub const CV_PPC_HID9: CV_HREG_e = 1117;
pub const CV_PPC_ICMP: CV_HREG_e = 1081;
pub const CV_PPC_IMISS: CV_HREG_e = 1080;
pub const CV_PPC_LR: CV_HREG_e = 108;
pub const CV_PPC_MQ: CV_HREG_e = 100;
pub const CV_PPC_MSR: CV_HREG_e = 75;
pub const CV_PPC_PC: CV_HREG_e = 99;
pub const CV_PPC_PMR0: CV_HREG_e = 1044;
pub const CV_PPC_PMR1: CV_HREG_e = 1045;
pub const CV_PPC_PMR10: CV_HREG_e = 1054;
pub const CV_PPC_PMR11: CV_HREG_e = 1055;
pub const CV_PPC_PMR12: CV_HREG_e = 1056;
pub const CV_PPC_PMR13: CV_HREG_e = 1057;
pub const CV_PPC_PMR14: CV_HREG_e = 1058;
pub const CV_PPC_PMR15: CV_HREG_e = 1059;
pub const CV_PPC_PMR2: CV_HREG_e = 1046;
pub const CV_PPC_PMR3: CV_HREG_e = 1047;
pub const CV_PPC_PMR4: CV_HREG_e = 1048;
pub const CV_PPC_PMR5: CV_HREG_e = 1049;
pub const CV_PPC_PMR6: CV_HREG_e = 1050;
pub const CV_PPC_PMR7: CV_HREG_e = 1051;
pub const CV_PPC_PMR8: CV_HREG_e = 1052;
pub const CV_PPC_PMR9: CV_HREG_e = 1053;
pub const CV_PPC_PVR: CV_HREG_e = 287;
pub const CV_PPC_RPA: CV_HREG_e = 1082;
pub const CV_PPC_RTCL: CV_HREG_e = 105;
pub const CV_PPC_RTCU: CV_HREG_e = 104;
pub const CV_PPC_SDR1: CV_HREG_e = 125;
pub const CV_PPC_SPRG0: CV_HREG_e = 372;
pub const CV_PPC_SPRG1: CV_HREG_e = 373;
pub const CV_PPC_SPRG2: CV_HREG_e = 374;
pub const CV_PPC_SPRG3: CV_HREG_e = 375;
pub const CV_PPC_SR0: CV_HREG_e = 76;
pub const CV_PPC_SR1: CV_HREG_e = 77;
pub const CV_PPC_SR10: CV_HREG_e = 86;
pub const CV_PPC_SR11: CV_HREG_e = 87;
pub const CV_PPC_SR12: CV_HREG_e = 88;
pub const CV_PPC_SR13: CV_HREG_e = 89;
pub const CV_PPC_SR14: CV_HREG_e = 90;
pub const CV_PPC_SR15: CV_HREG_e = 91;
pub const CV_PPC_SR2: CV_HREG_e = 78;
pub const CV_PPC_SR3: CV_HREG_e = 79;
pub const CV_PPC_SR4: CV_HREG_e = 80;
pub const CV_PPC_SR5: CV_HREG_e = 81;
pub const CV_PPC_SR6: CV_HREG_e = 82;
pub const CV_PPC_SR7: CV_HREG_e = 83;
pub const CV_PPC_SR8: CV_HREG_e = 84;
pub const CV_PPC_SR9: CV_HREG_e = 85;
pub const CV_PPC_SRR0: CV_HREG_e = 126;
pub const CV_PPC_SRR1: CV_HREG_e = 127;
pub const CV_PPC_XER: CV_HREG_e = 101;
pub const CV_R68_A0: CV_HREG_e = 8;
pub const CV_R68_A1: CV_HREG_e = 9;
pub const CV_R68_A2: CV_HREG_e = 10;
pub const CV_R68_A3: CV_HREG_e = 11;
pub const CV_R68_A4: CV_HREG_e = 12;
pub const CV_R68_A5: CV_HREG_e = 13;
pub const CV_R68_A6: CV_HREG_e = 14;
pub const CV_R68_A7: CV_HREG_e = 15;
pub const CV_R68_AC: CV_HREG_e = 58;
pub const CV_R68_BAC0: CV_HREG_e = 72;
pub const CV_R68_BAC1: CV_HREG_e = 73;
pub const CV_R68_BAC2: CV_HREG_e = 74;
pub const CV_R68_BAC3: CV_HREG_e = 75;
pub const CV_R68_BAC4: CV_HREG_e = 76;
pub const CV_R68_BAC5: CV_HREG_e = 77;
pub const CV_R68_BAC6: CV_HREG_e = 78;
pub const CV_R68_BAC7: CV_HREG_e = 79;
pub const CV_R68_BAD0: CV_HREG_e = 64;
pub const CV_R68_BAD1: CV_HREG_e = 65;
pub const CV_R68_BAD2: CV_HREG_e = 66;
pub const CV_R68_BAD3: CV_HREG_e = 67;
pub const CV_R68_BAD4: CV_HREG_e = 68;
pub const CV_R68_BAD5: CV_HREG_e = 69;
pub const CV_R68_BAD6: CV_HREG_e = 70;
pub const CV_R68_BAD7: CV_HREG_e = 71;
pub const CV_R68_CAAR: CV_HREG_e = 24;
pub const CV_R68_CACR: CV_HREG_e = 22;
pub const CV_R68_CAL: CV_HREG_e = 60;
pub const CV_R68_CCR: CV_HREG_e = 16;
pub const CV_R68_CRP: CV_HREG_e = 54;
pub const CV_R68_D0: CV_HREG_e = 0;
pub const CV_R68_D1: CV_HREG_e = 1;
pub const CV_R68_D2: CV_HREG_e = 2;
pub const CV_R68_D3: CV_HREG_e = 3;
pub const CV_R68_D4: CV_HREG_e = 4;
pub const CV_R68_D5: CV_HREG_e = 5;
pub const CV_R68_D6: CV_HREG_e = 6;
pub const CV_R68_D7: CV_HREG_e = 7;
pub const CV_R68_DFC: CV_HREG_e = 21;
pub const CV_R68_DRP: CV_HREG_e = 56;
pub const CV_R68_DTT0: CV_HREG_e = 44;
pub const CV_R68_DTT1: CV_HREG_e = 45;
pub const CV_R68_FP0: CV_HREG_e = 32;
pub const CV_R68_FP1: CV_HREG_e = 33;
pub const CV_R68_FP2: CV_HREG_e = 34;
pub const CV_R68_FP3: CV_HREG_e = 35;
pub const CV_R68_FP4: CV_HREG_e = 36;
pub const CV_R68_FP5: CV_HREG_e = 37;
pub const CV_R68_FP6: CV_HREG_e = 38;
pub const CV_R68_FP7: CV_HREG_e = 39;
pub const CV_R68_FPCR: CV_HREG_e = 28;
pub const CV_R68_FPIAR: CV_HREG_e = 30;
pub const CV_R68_FPSR: CV_HREG_e = 29;
pub const CV_R68_ISP: CV_HREG_e = 25;
pub const CV_R68_ITT0: CV_HREG_e = 46;
pub const CV_R68_ITT1: CV_HREG_e = 47;
pub const CV_R68_MMUSR: CV_HREG_e = 42;
pub const CV_R68_MMUSR030: CV_HREG_e = 41;
pub const CV_R68_MSP: CV_HREG_e = 19;
pub const CV_R68_PC: CV_HREG_e = 26;
pub const CV_R68_PCSR: CV_HREG_e = 52;
pub const CV_R68_PSR: CV_HREG_e = 51;
pub const CV_R68_SCC: CV_HREG_e = 59;
pub const CV_R68_SFC: CV_HREG_e = 20;
pub const CV_R68_SR: CV_HREG_e = 17;
pub const CV_R68_SRP: CV_HREG_e = 55;
pub const CV_R68_TC: CV_HREG_e = 57;
pub const CV_R68_TT0: CV_HREG_e = 61;
pub const CV_R68_TT1: CV_HREG_e = 62;
pub const CV_R68_URP: CV_HREG_e = 43;
pub const CV_R68_USP: CV_HREG_e = 18;
pub const CV_R68_VAL: CV_HREG_e = 53;
pub const CV_R68_VBR: CV_HREG_e = 23;
pub const CV_REG_AH: CV_HREG_e = 5;
pub const CV_REG_AL: CV_HREG_e = 1;
pub const CV_REG_AX: CV_HREG_e = 9;
pub const CV_REG_BH: CV_HREG_e = 8;
pub const CV_REG_BL: CV_HREG_e = 4;
pub const CV_REG_BND0: CV_HREG_e = 396;
pub const CV_REG_BND1: CV_HREG_e = 397;
pub const CV_REG_BND2: CV_HREG_e = 398;
pub const CV_REG_BND3: CV_HREG_e = 399;
pub const CV_REG_BNDCFGU: CV_HREG_e = 400;
pub const CV_REG_BNDSTATUS: CV_HREG_e = 401;
pub const CV_REG_BP: CV_HREG_e = 14;
pub const CV_REG_BX: CV_HREG_e = 12;
pub const CV_REG_CH: CV_HREG_e = 6;
pub const CV_REG_CL: CV_HREG_e = 2;
pub const CV_REG_CR0: CV_HREG_e = 80;
pub const CV_REG_CR1: CV_HREG_e = 81;
pub const CV_REG_CR2: CV_HREG_e = 82;
pub const CV_REG_CR3: CV_HREG_e = 83;
pub const CV_REG_CR4: CV_HREG_e = 84;
pub const CV_REG_CS: CV_HREG_e = 26;
pub const CV_REG_CTRL: CV_HREG_e = 136;
pub const CV_REG_CX: CV_HREG_e = 10;
pub const CV_REG_DH: CV_HREG_e = 7;
pub const CV_REG_DI: CV_HREG_e = 16;
pub const CV_REG_DL: CV_HREG_e = 3;
pub const CV_REG_DR0: CV_HREG_e = 90;
pub const CV_REG_DR1: CV_HREG_e = 91;
pub const CV_REG_DR2: CV_HREG_e = 92;
pub const CV_REG_DR3: CV_HREG_e = 93;
pub const CV_REG_DR4: CV_HREG_e = 94;
pub const CV_REG_DR5: CV_HREG_e = 95;
pub const CV_REG_DR6: CV_HREG_e = 96;
pub const CV_REG_DR7: CV_HREG_e = 97;
pub const CV_REG_DS: CV_HREG_e = 28;
pub const CV_REG_DX: CV_HREG_e = 11;
pub const CV_REG_EAX: CV_HREG_e = 17;
pub const CV_REG_EBP: CV_HREG_e = 22;
pub const CV_REG_EBX: CV_HREG_e = 20;
pub const CV_REG_ECX: CV_HREG_e = 18;
pub const CV_REG_EDI: CV_HREG_e = 24;
pub const CV_REG_EDX: CV_HREG_e = 19;
pub const CV_REG_EDXEAX: CV_HREG_e = 212;
pub const CV_REG_EFLAGS: CV_HREG_e = 34;
pub const CV_REG_EIP: CV_HREG_e = 33;
pub const CV_REG_EMM0H: CV_HREG_e = 228;
pub const CV_REG_EMM0L: CV_HREG_e = 220;
pub const CV_REG_EMM1H: CV_HREG_e = 229;
pub const CV_REG_EMM1L: CV_HREG_e = 221;
pub const CV_REG_EMM2H: CV_HREG_e = 230;
pub const CV_REG_EMM2L: CV_HREG_e = 222;
pub const CV_REG_EMM3H: CV_HREG_e = 231;
pub const CV_REG_EMM3L: CV_HREG_e = 223;
pub const CV_REG_EMM4H: CV_HREG_e = 232;
pub const CV_REG_EMM4L: CV_HREG_e = 224;
pub const CV_REG_EMM5H: CV_HREG_e = 233;
pub const CV_REG_EMM5L: CV_HREG_e = 225;
pub const CV_REG_EMM6H: CV_HREG_e = 234;
pub const CV_REG_EMM6L: CV_HREG_e = 226;
pub const CV_REG_EMM7H: CV_HREG_e = 235;
pub const CV_REG_EMM7L: CV_HREG_e = 227;
pub const CV_REG_ES: CV_HREG_e = 25;
pub const CV_REG_ESI: CV_HREG_e = 23;
pub const CV_REG_ESP: CV_HREG_e = 21;
pub const CV_REG_FLAGS: CV_HREG_e = 32;
pub const CV_REG_FPCS: CV_HREG_e = 140;
pub const CV_REG_FPDO: CV_HREG_e = 141;
pub const CV_REG_FPDS: CV_HREG_e = 142;
pub const CV_REG_FPEDO: CV_HREG_e = 145;
pub const CV_REG_FPEIP: CV_HREG_e = 144;
pub const CV_REG_FPIP: CV_HREG_e = 139;
pub const CV_REG_FS: CV_HREG_e = 29;
pub const CV_REG_GDTL: CV_HREG_e = 111;
pub const CV_REG_GDTR: CV_HREG_e = 110;
pub const CV_REG_GS: CV_HREG_e = 30;
pub const CV_REG_IDTL: CV_HREG_e = 113;
pub const CV_REG_IDTR: CV_HREG_e = 112;
pub const CV_REG_IP: CV_HREG_e = 31;
pub const CV_REG_ISEM: CV_HREG_e = 143;
pub const CV_REG_K0: CV_HREG_e = 418;
pub const CV_REG_K1: CV_HREG_e = 419;
pub const CV_REG_K2: CV_HREG_e = 420;
pub const CV_REG_K3: CV_HREG_e = 421;
pub const CV_REG_K4: CV_HREG_e = 422;
pub const CV_REG_K5: CV_HREG_e = 423;
pub const CV_REG_K6: CV_HREG_e = 424;
pub const CV_REG_K7: CV_HREG_e = 425;
pub const CV_REG_LDTR: CV_HREG_e = 114;
pub const CV_REG_MM0: CV_HREG_e = 146;
pub const CV_REG_MM00: CV_HREG_e = 236;
pub const CV_REG_MM01: CV_HREG_e = 237;
pub const CV_REG_MM1: CV_HREG_e = 147;
pub const CV_REG_MM10: CV_HREG_e = 238;
pub const CV_REG_MM11: CV_HREG_e = 239;
pub const CV_REG_MM2: CV_HREG_e = 148;
pub const CV_REG_MM20: CV_HREG_e = 240;
pub const CV_REG_MM21: CV_HREG_e = 241;
pub const CV_REG_MM3: CV_HREG_e = 149;
pub const CV_REG_MM30: CV_HREG_e = 242;
pub const CV_REG_MM31: CV_HREG_e = 243;
pub const CV_REG_MM4: CV_HREG_e = 150;
pub const CV_REG_MM40: CV_HREG_e = 244;
pub const CV_REG_MM41: CV_HREG_e = 245;
pub const CV_REG_MM5: CV_HREG_e = 151;
pub const CV_REG_MM50: CV_HREG_e = 246;
pub const CV_REG_MM51: CV_HREG_e = 247;
pub const CV_REG_MM6: CV_HREG_e = 152;
pub const CV_REG_MM60: CV_HREG_e = 248;
pub const CV_REG_MM61: CV_HREG_e = 249;
pub const CV_REG_MM7: CV_HREG_e = 153;
pub const CV_REG_MM70: CV_HREG_e = 250;
pub const CV_REG_MM71: CV_HREG_e = 251;
pub const CV_REG_MXCSR: CV_HREG_e = 211;
pub const CV_REG_NONE: CV_HREG_e = 0;
pub const CV_REG_PCDR3: CV_HREG_e = 43;
pub const CV_REG_PCDR4: CV_HREG_e = 44;
pub const CV_REG_PCDR5: CV_HREG_e = 45;
pub const CV_REG_PCDR6: CV_HREG_e = 46;
pub const CV_REG_PCDR7: CV_HREG_e = 47;
pub const CV_REG_PSEUDO1: CV_HREG_e = 116;
pub const CV_REG_PSEUDO2: CV_HREG_e = 117;
pub const CV_REG_PSEUDO3: CV_HREG_e = 118;
pub const CV_REG_PSEUDO4: CV_HREG_e = 119;
pub const CV_REG_PSEUDO5: CV_HREG_e = 120;
pub const CV_REG_PSEUDO6: CV_HREG_e = 121;
pub const CV_REG_PSEUDO7: CV_HREG_e = 122;
pub const CV_REG_PSEUDO8: CV_HREG_e = 123;
pub const CV_REG_PSEUDO9: CV_HREG_e = 124;
pub const CV_REG_QUOTE: CV_HREG_e = 42;
pub const CV_REG_SI: CV_HREG_e = 15;
pub const CV_REG_SP: CV_HREG_e = 13;
pub const CV_REG_SS: CV_HREG_e = 27;
pub const CV_REG_SSP: CV_HREG_e = 426;
pub const CV_REG_ST0: CV_HREG_e = 128;
pub const CV_REG_ST1: CV_HREG_e = 129;
pub const CV_REG_ST2: CV_HREG_e = 130;
pub const CV_REG_ST3: CV_HREG_e = 131;
pub const CV_REG_ST4: CV_HREG_e = 132;
pub const CV_REG_ST5: CV_HREG_e = 133;
pub const CV_REG_ST6: CV_HREG_e = 134;
pub const CV_REG_ST7: CV_HREG_e = 135;
pub const CV_REG_STAT: CV_HREG_e = 137;
pub const CV_REG_TAG: CV_HREG_e = 138;
pub const CV_REG_TEMP: CV_HREG_e = 40;
pub const CV_REG_TEMPH: CV_HREG_e = 41;
pub const CV_REG_TR: CV_HREG_e = 115;
pub const CV_REG_XMM0: CV_HREG_e = 154;
pub const CV_REG_XMM00: CV_HREG_e = 162;
pub const CV_REG_XMM01: CV_HREG_e = 163;
pub const CV_REG_XMM02: CV_HREG_e = 164;
pub const CV_REG_XMM03: CV_HREG_e = 165;
pub const CV_REG_XMM0H: CV_HREG_e = 202;
pub const CV_REG_XMM0L: CV_HREG_e = 194;
pub const CV_REG_XMM1: CV_HREG_e = 155;
pub const CV_REG_XMM10: CV_HREG_e = 166;
pub const CV_REG_XMM11: CV_HREG_e = 167;
pub const CV_REG_XMM12: CV_HREG_e = 168;
pub const CV_REG_XMM13: CV_HREG_e = 169;
pub const CV_REG_XMM1H: CV_HREG_e = 203;
pub const CV_REG_XMM1L: CV_HREG_e = 195;
pub const CV_REG_XMM2: CV_HREG_e = 156;
pub const CV_REG_XMM20: CV_HREG_e = 170;
pub const CV_REG_XMM21: CV_HREG_e = 171;
pub const CV_REG_XMM22: CV_HREG_e = 172;
pub const CV_REG_XMM23: CV_HREG_e = 173;
pub const CV_REG_XMM2H: CV_HREG_e = 204;
pub const CV_REG_XMM2L: CV_HREG_e = 196;
pub const CV_REG_XMM3: CV_HREG_e = 157;
pub const CV_REG_XMM30: CV_HREG_e = 174;
pub const CV_REG_XMM31: CV_HREG_e = 175;
pub const CV_REG_XMM32: CV_HREG_e = 176;
pub const CV_REG_XMM33: CV_HREG_e = 177;
pub const CV_REG_XMM3H: CV_HREG_e = 205;
pub const CV_REG_XMM3L: CV_HREG_e = 197;
pub const CV_REG_XMM4: CV_HREG_e = 158;
pub const CV_REG_XMM40: CV_HREG_e = 178;
pub const CV_REG_XMM41: CV_HREG_e = 179;
pub const CV_REG_XMM42: CV_HREG_e = 180;
pub const CV_REG_XMM43: CV_HREG_e = 181;
pub const CV_REG_XMM4H: CV_HREG_e = 206;
pub const CV_REG_XMM4L: CV_HREG_e = 198;
pub const CV_REG_XMM5: CV_HREG_e = 159;
pub const CV_REG_XMM50: CV_HREG_e = 182;
pub const CV_REG_XMM51: CV_HREG_e = 183;
pub const CV_REG_XMM52: CV_HREG_e = 184;
pub const CV_REG_XMM53: CV_HREG_e = 185;
pub const CV_REG_XMM5H: CV_HREG_e = 207;
pub const CV_REG_XMM5L: CV_HREG_e = 199;
pub const CV_REG_XMM6: CV_HREG_e = 160;
pub const CV_REG_XMM60: CV_HREG_e = 186;
pub const CV_REG_XMM61: CV_HREG_e = 187;
pub const CV_REG_XMM62: CV_HREG_e = 188;
pub const CV_REG_XMM63: CV_HREG_e = 189;
pub const CV_REG_XMM6H: CV_HREG_e = 208;
pub const CV_REG_XMM6L: CV_HREG_e = 200;
pub const CV_REG_XMM7: CV_HREG_e = 161;
pub const CV_REG_XMM70: CV_HREG_e = 190;
pub const CV_REG_XMM71: CV_HREG_e = 191;
pub const CV_REG_XMM72: CV_HREG_e = 192;
pub const CV_REG_XMM73: CV_HREG_e = 193;
pub const CV_REG_XMM7H: CV_HREG_e = 209;
pub const CV_REG_XMM7L: CV_HREG_e = 201;
pub const CV_REG_YMM0: CV_HREG_e = 252;
pub const CV_REG_YMM0D0: CV_HREG_e = 364;
pub const CV_REG_YMM0D1: CV_HREG_e = 365;
pub const CV_REG_YMM0D2: CV_HREG_e = 366;
pub const CV_REG_YMM0D3: CV_HREG_e = 367;
pub const CV_REG_YMM0F0: CV_HREG_e = 300;
pub const CV_REG_YMM0F1: CV_HREG_e = 301;
pub const CV_REG_YMM0F2: CV_HREG_e = 302;
pub const CV_REG_YMM0F3: CV_HREG_e = 303;
pub const CV_REG_YMM0F4: CV_HREG_e = 304;
pub const CV_REG_YMM0F5: CV_HREG_e = 305;
pub const CV_REG_YMM0F6: CV_HREG_e = 306;
pub const CV_REG_YMM0F7: CV_HREG_e = 307;
pub const CV_REG_YMM0H: CV_HREG_e = 260;
pub const CV_REG_YMM0I0: CV_HREG_e = 268;
pub const CV_REG_YMM0I1: CV_HREG_e = 269;
pub const CV_REG_YMM0I2: CV_HREG_e = 270;
pub const CV_REG_YMM0I3: CV_HREG_e = 271;
pub const CV_REG_YMM1: CV_HREG_e = 253;
pub const CV_REG_YMM1D0: CV_HREG_e = 368;
pub const CV_REG_YMM1D1: CV_HREG_e = 369;
pub const CV_REG_YMM1D2: CV_HREG_e = 370;
pub const CV_REG_YMM1D3: CV_HREG_e = 371;
pub const CV_REG_YMM1F0: CV_HREG_e = 308;
pub const CV_REG_YMM1F1: CV_HREG_e = 309;
pub const CV_REG_YMM1F2: CV_HREG_e = 310;
pub const CV_REG_YMM1F3: CV_HREG_e = 311;
pub const CV_REG_YMM1F4: CV_HREG_e = 312;
pub const CV_REG_YMM1F5: CV_HREG_e = 313;
pub const CV_REG_YMM1F6: CV_HREG_e = 314;
pub const CV_REG_YMM1F7: CV_HREG_e = 315;
pub const CV_REG_YMM1H: CV_HREG_e = 261;
pub const CV_REG_YMM1I0: CV_HREG_e = 272;
pub const CV_REG_YMM1I1: CV_HREG_e = 273;
pub const CV_REG_YMM1I2: CV_HREG_e = 274;
pub const CV_REG_YMM1I3: CV_HREG_e = 275;
pub const CV_REG_YMM2: CV_HREG_e = 254;
pub const CV_REG_YMM2D0: CV_HREG_e = 372;
pub const CV_REG_YMM2D1: CV_HREG_e = 373;
pub const CV_REG_YMM2D2: CV_HREG_e = 374;
pub const CV_REG_YMM2D3: CV_HREG_e = 375;
pub const CV_REG_YMM2F0: CV_HREG_e = 316;
pub const CV_REG_YMM2F1: CV_HREG_e = 317;
pub const CV_REG_YMM2F2: CV_HREG_e = 318;
pub const CV_REG_YMM2F3: CV_HREG_e = 319;
pub const CV_REG_YMM2F4: CV_HREG_e = 320;
pub const CV_REG_YMM2F5: CV_HREG_e = 321;
pub const CV_REG_YMM2F6: CV_HREG_e = 322;
pub const CV_REG_YMM2F7: CV_HREG_e = 323;
pub const CV_REG_YMM2H: CV_HREG_e = 262;
pub const CV_REG_YMM2I0: CV_HREG_e = 276;
pub const CV_REG_YMM2I1: CV_HREG_e = 277;
pub const CV_REG_YMM2I2: CV_HREG_e = 278;
pub const CV_REG_YMM2I3: CV_HREG_e = 279;
pub const CV_REG_YMM3: CV_HREG_e = 255;
pub const CV_REG_YMM3D0: CV_HREG_e = 376;
pub const CV_REG_YMM3D1: CV_HREG_e = 377;
pub const CV_REG_YMM3D2: CV_HREG_e = 378;
pub const CV_REG_YMM3D3: CV_HREG_e = 379;
pub const CV_REG_YMM3F0: CV_HREG_e = 324;
pub const CV_REG_YMM3F1: CV_HREG_e = 325;
pub const CV_REG_YMM3F2: CV_HREG_e = 326;
pub const CV_REG_YMM3F3: CV_HREG_e = 327;
pub const CV_REG_YMM3F4: CV_HREG_e = 328;
pub const CV_REG_YMM3F5: CV_HREG_e = 329;
pub const CV_REG_YMM3F6: CV_HREG_e = 330;
pub const CV_REG_YMM3F7: CV_HREG_e = 331;
pub const CV_REG_YMM3H: CV_HREG_e = 263;
pub const CV_REG_YMM3I0: CV_HREG_e = 280;
pub const CV_REG_YMM3I1: CV_HREG_e = 281;
pub const CV_REG_YMM3I2: CV_HREG_e = 282;
pub const CV_REG_YMM3I3: CV_HREG_e = 283;
pub const CV_REG_YMM4: CV_HREG_e = 256;
pub const CV_REG_YMM4D0: CV_HREG_e = 380;
pub const CV_REG_YMM4D1: CV_HREG_e = 381;
pub const CV_REG_YMM4D2: CV_HREG_e = 382;
pub const CV_REG_YMM4D3: CV_HREG_e = 383;
pub const CV_REG_YMM4F0: CV_HREG_e = 332;
pub const CV_REG_YMM4F1: CV_HREG_e = 333;
pub const CV_REG_YMM4F2: CV_HREG_e = 334;
pub const CV_REG_YMM4F3: CV_HREG_e = 335;
pub const CV_REG_YMM4F4: CV_HREG_e = 336;
pub const CV_REG_YMM4F5: CV_HREG_e = 337;
pub const CV_REG_YMM4F6: CV_HREG_e = 338;
pub const CV_REG_YMM4F7: CV_HREG_e = 339;
pub const CV_REG_YMM4H: CV_HREG_e = 264;
pub const CV_REG_YMM4I0: CV_HREG_e = 284;
pub const CV_REG_YMM4I1: CV_HREG_e = 285;
pub const CV_REG_YMM4I2: CV_HREG_e = 286;
pub const CV_REG_YMM4I3: CV_HREG_e = 287;
pub const CV_REG_YMM5: CV_HREG_e = 257;
pub const CV_REG_YMM5D0: CV_HREG_e = 384;
pub const CV_REG_YMM5D1: CV_HREG_e = 385;
pub const CV_REG_YMM5D2: CV_HREG_e = 386;
pub const CV_REG_YMM5D3: CV_HREG_e = 387;
pub const CV_REG_YMM5F0: CV_HREG_e = 340;
pub const CV_REG_YMM5F1: CV_HREG_e = 341;
pub const CV_REG_YMM5F2: CV_HREG_e = 342;
pub const CV_REG_YMM5F3: CV_HREG_e = 343;
pub const CV_REG_YMM5F4: CV_HREG_e = 344;
pub const CV_REG_YMM5F5: CV_HREG_e = 345;
pub const CV_REG_YMM5F6: CV_HREG_e = 346;
pub const CV_REG_YMM5F7: CV_HREG_e = 347;
pub const CV_REG_YMM5H: CV_HREG_e = 265;
pub const CV_REG_YMM5I0: CV_HREG_e = 288;
pub const CV_REG_YMM5I1: CV_HREG_e = 289;
pub const CV_REG_YMM5I2: CV_HREG_e = 290;
pub const CV_REG_YMM5I3: CV_HREG_e = 291;
pub const CV_REG_YMM6: CV_HREG_e = 258;
pub const CV_REG_YMM6D0: CV_HREG_e = 388;
pub const CV_REG_YMM6D1: CV_HREG_e = 389;
pub const CV_REG_YMM6D2: CV_HREG_e = 390;
pub const CV_REG_YMM6D3: CV_HREG_e = 391;
pub const CV_REG_YMM6F0: CV_HREG_e = 348;
pub const CV_REG_YMM6F1: CV_HREG_e = 349;
pub const CV_REG_YMM6F2: CV_HREG_e = 350;
pub const CV_REG_YMM6F3: CV_HREG_e = 351;
pub const CV_REG_YMM6F4: CV_HREG_e = 352;
pub const CV_REG_YMM6F5: CV_HREG_e = 353;
pub const CV_REG_YMM6F6: CV_HREG_e = 354;
pub const CV_REG_YMM6F7: CV_HREG_e = 355;
pub const CV_REG_YMM6H: CV_HREG_e = 266;
pub const CV_REG_YMM6I0: CV_HREG_e = 292;
pub const CV_REG_YMM6I1: CV_HREG_e = 293;
pub const CV_REG_YMM6I2: CV_HREG_e = 294;
pub const CV_REG_YMM6I3: CV_HREG_e = 295;
pub const CV_REG_YMM7: CV_HREG_e = 259;
pub const CV_REG_YMM7D0: CV_HREG_e = 392;
pub const CV_REG_YMM7D1: CV_HREG_e = 393;
pub const CV_REG_YMM7D2: CV_HREG_e = 394;
pub const CV_REG_YMM7D3: CV_HREG_e = 395;
pub const CV_REG_YMM7F0: CV_HREG_e = 356;
pub const CV_REG_YMM7F1: CV_HREG_e = 357;
pub const CV_REG_YMM7F2: CV_HREG_e = 358;
pub const CV_REG_YMM7F3: CV_HREG_e = 359;
pub const CV_REG_YMM7F4: CV_HREG_e = 360;
pub const CV_REG_YMM7F5: CV_HREG_e = 361;
pub const CV_REG_YMM7F6: CV_HREG_e = 362;
pub const CV_REG_YMM7F7: CV_HREG_e = 363;
pub const CV_REG_YMM7H: CV_HREG_e = 267;
pub const CV_REG_YMM7I0: CV_HREG_e = 296;
pub const CV_REG_YMM7I1: CV_HREG_e = 297;
pub const CV_REG_YMM7I2: CV_HREG_e = 298;
pub const CV_REG_YMM7I3: CV_HREG_e = 299;
pub const CV_REG_ZMM0: CV_HREG_e = 402;
pub const CV_REG_ZMM0H: CV_HREG_e = 410;
pub const CV_REG_ZMM1: CV_HREG_e = 403;
pub const CV_REG_ZMM1H: CV_HREG_e = 411;
pub const CV_REG_ZMM2: CV_HREG_e = 404;
pub const CV_REG_ZMM2H: CV_HREG_e = 412;
pub const CV_REG_ZMM3: CV_HREG_e = 405;
pub const CV_REG_ZMM3H: CV_HREG_e = 413;
pub const CV_REG_ZMM4: CV_HREG_e = 406;
pub const CV_REG_ZMM4H: CV_HREG_e = 414;
pub const CV_REG_ZMM5: CV_HREG_e = 407;
pub const CV_REG_ZMM5H: CV_HREG_e = 415;
pub const CV_REG_ZMM6: CV_HREG_e = 408;
pub const CV_REG_ZMM6H: CV_HREG_e = 416;
pub const CV_REG_ZMM7: CV_HREG_e = 409;
pub const CV_REG_ZMM7H: CV_HREG_e = 417;
pub const CV_SH3_BamrA: CV_HREG_e = 62;
pub const CV_SH3_BamrB: CV_HREG_e = 66;
pub const CV_SH3_BarA: CV_HREG_e = 60;
pub const CV_SH3_BarB: CV_HREG_e = 64;
pub const CV_SH3_BasrA: CV_HREG_e = 61;
pub const CV_SH3_BasrB: CV_HREG_e = 65;
pub const CV_SH3_BbrA: CV_HREG_e = 63;
pub const CV_SH3_BbrB: CV_HREG_e = 67;
pub const CV_SH3_BdmrB: CV_HREG_e = 69;
pub const CV_SH3_BdrB: CV_HREG_e = 68;
pub const CV_SH3_Brcr: CV_HREG_e = 70;
pub const CV_SH3_Gbr: CV_HREG_e = 38;
pub const CV_SH3_IntFp: CV_HREG_e = 24;
pub const CV_SH3_IntR0: CV_HREG_e = 10;
pub const CV_SH3_IntR1: CV_HREG_e = 11;
pub const CV_SH3_IntR10: CV_HREG_e = 20;
pub const CV_SH3_IntR11: CV_HREG_e = 21;
pub const CV_SH3_IntR12: CV_HREG_e = 22;
pub const CV_SH3_IntR13: CV_HREG_e = 23;
pub const CV_SH3_IntR2: CV_HREG_e = 12;
pub const CV_SH3_IntR3: CV_HREG_e = 13;
pub const CV_SH3_IntR4: CV_HREG_e = 14;
pub const CV_SH3_IntR5: CV_HREG_e = 15;
pub const CV_SH3_IntR6: CV_HREG_e = 16;
pub const CV_SH3_IntR7: CV_HREG_e = 17;
pub const CV_SH3_IntR8: CV_HREG_e = 18;
pub const CV_SH3_IntR9: CV_HREG_e = 19;
pub const CV_SH3_IntSp: CV_HREG_e = 25;
pub const CV_SH3_Mach: CV_HREG_e = 40;
pub const CV_SH3_Macl: CV_HREG_e = 41;
pub const CV_SH3_NOREG: CV_HREG_e = 0;
pub const CV_SH3_Pc: CV_HREG_e = 50;
pub const CV_SH3_Pr: CV_HREG_e = 39;
pub const CV_SH3_Sr: CV_HREG_e = 51;
pub const CV_SHMEDIA_CR0: CV_HREG_e = 2000;
pub const CV_SHMEDIA_CR1: CV_HREG_e = 2001;
pub const CV_SHMEDIA_CR10: CV_HREG_e = 2010;
pub const CV_SHMEDIA_CR11: CV_HREG_e = 2011;
pub const CV_SHMEDIA_CR12: CV_HREG_e = 2012;
pub const CV_SHMEDIA_CR13: CV_HREG_e = 2013;
pub const CV_SHMEDIA_CR14: CV_HREG_e = 2014;
pub const CV_SHMEDIA_CR15: CV_HREG_e = 2015;
pub const CV_SHMEDIA_CR16: CV_HREG_e = 2016;
pub const CV_SHMEDIA_CR17: CV_HREG_e = 2017;
pub const CV_SHMEDIA_CR18: CV_HREG_e = 2018;
pub const CV_SHMEDIA_CR19: CV_HREG_e = 2019;
pub const CV_SHMEDIA_CR2: CV_HREG_e = 2002;
pub const CV_SHMEDIA_CR20: CV_HREG_e = 2020;
pub const CV_SHMEDIA_CR21: CV_HREG_e = 2021;
pub const CV_SHMEDIA_CR22: CV_HREG_e = 2022;
pub const CV_SHMEDIA_CR23: CV_HREG_e = 2023;
pub const CV_SHMEDIA_CR24: CV_HREG_e = 2024;
pub const CV_SHMEDIA_CR25: CV_HREG_e = 2025;
pub const CV_SHMEDIA_CR26: CV_HREG_e = 2026;
pub const CV_SHMEDIA_CR27: CV_HREG_e = 2027;
pub const CV_SHMEDIA_CR28: CV_HREG_e = 2028;
pub const CV_SHMEDIA_CR29: CV_HREG_e = 2029;
pub const CV_SHMEDIA_CR3: CV_HREG_e = 2003;
pub const CV_SHMEDIA_CR30: CV_HREG_e = 2030;
pub const CV_SHMEDIA_CR31: CV_HREG_e = 2031;
pub const CV_SHMEDIA_CR32: CV_HREG_e = 2032;
pub const CV_SHMEDIA_CR33: CV_HREG_e = 2033;
pub const CV_SHMEDIA_CR34: CV_HREG_e = 2034;
pub const CV_SHMEDIA_CR35: CV_HREG_e = 2035;
pub const CV_SHMEDIA_CR36: CV_HREG_e = 2036;
pub const CV_SHMEDIA_CR37: CV_HREG_e = 2037;
pub const CV_SHMEDIA_CR38: CV_HREG_e = 2038;
pub const CV_SHMEDIA_CR39: CV_HREG_e = 2039;
pub const CV_SHMEDIA_CR4: CV_HREG_e = 2004;
pub const CV_SHMEDIA_CR40: CV_HREG_e = 2040;
pub const CV_SHMEDIA_CR41: CV_HREG_e = 2041;
pub const CV_SHMEDIA_CR42: CV_HREG_e = 2042;
pub const CV_SHMEDIA_CR43: CV_HREG_e = 2043;
pub const CV_SHMEDIA_CR44: CV_HREG_e = 2044;
pub const CV_SHMEDIA_CR45: CV_HREG_e = 2045;
pub const CV_SHMEDIA_CR46: CV_HREG_e = 2046;
pub const CV_SHMEDIA_CR47: CV_HREG_e = 2047;
pub const CV_SHMEDIA_CR48: CV_HREG_e = 2048;
pub const CV_SHMEDIA_CR49: CV_HREG_e = 2049;
pub const CV_SHMEDIA_CR5: CV_HREG_e = 2005;
pub const CV_SHMEDIA_CR50: CV_HREG_e = 2050;
pub const CV_SHMEDIA_CR51: CV_HREG_e = 2051;
pub const CV_SHMEDIA_CR52: CV_HREG_e = 2052;
pub const CV_SHMEDIA_CR53: CV_HREG_e = 2053;
pub const CV_SHMEDIA_CR54: CV_HREG_e = 2054;
pub const CV_SHMEDIA_CR55: CV_HREG_e = 2055;
pub const CV_SHMEDIA_CR56: CV_HREG_e = 2056;
pub const CV_SHMEDIA_CR57: CV_HREG_e = 2057;
pub const CV_SHMEDIA_CR58: CV_HREG_e = 2058;
pub const CV_SHMEDIA_CR59: CV_HREG_e = 2059;
pub const CV_SHMEDIA_CR6: CV_HREG_e = 2006;
pub const CV_SHMEDIA_CR60: CV_HREG_e = 2060;
pub const CV_SHMEDIA_CR61: CV_HREG_e = 2061;
pub const CV_SHMEDIA_CR62: CV_HREG_e = 2062;
pub const CV_SHMEDIA_CR63: CV_HREG_e = 2063;
pub const CV_SHMEDIA_CR7: CV_HREG_e = 2007;
pub const CV_SHMEDIA_CR8: CV_HREG_e = 2008;
pub const CV_SHMEDIA_CR9: CV_HREG_e = 2009;
pub const CV_SHMEDIA_DR0: CV_HREG_e = 256;
pub const CV_SHMEDIA_DR10: CV_HREG_e = 266;
pub const CV_SHMEDIA_DR12: CV_HREG_e = 268;
pub const CV_SHMEDIA_DR14: CV_HREG_e = 270;
pub const CV_SHMEDIA_DR16: CV_HREG_e = 272;
pub const CV_SHMEDIA_DR18: CV_HREG_e = 274;
pub const CV_SHMEDIA_DR2: CV_HREG_e = 258;
pub const CV_SHMEDIA_DR20: CV_HREG_e = 276;
pub const CV_SHMEDIA_DR22: CV_HREG_e = 278;
pub const CV_SHMEDIA_DR24: CV_HREG_e = 280;
pub const CV_SHMEDIA_DR26: CV_HREG_e = 282;
pub const CV_SHMEDIA_DR28: CV_HREG_e = 284;
pub const CV_SHMEDIA_DR30: CV_HREG_e = 286;
pub const CV_SHMEDIA_DR32: CV_HREG_e = 288;
pub const CV_SHMEDIA_DR34: CV_HREG_e = 290;
pub const CV_SHMEDIA_DR36: CV_HREG_e = 292;
pub const CV_SHMEDIA_DR38: CV_HREG_e = 294;
pub const CV_SHMEDIA_DR4: CV_HREG_e = 260;
pub const CV_SHMEDIA_DR40: CV_HREG_e = 296;
pub const CV_SHMEDIA_DR42: CV_HREG_e = 298;
pub const CV_SHMEDIA_DR44: CV_HREG_e = 300;
pub const CV_SHMEDIA_DR46: CV_HREG_e = 302;
pub const CV_SHMEDIA_DR48: CV_HREG_e = 304;
pub const CV_SHMEDIA_DR50: CV_HREG_e = 306;
pub const CV_SHMEDIA_DR52: CV_HREG_e = 308;
pub const CV_SHMEDIA_DR54: CV_HREG_e = 310;
pub const CV_SHMEDIA_DR56: CV_HREG_e = 312;
pub const CV_SHMEDIA_DR58: CV_HREG_e = 314;
pub const CV_SHMEDIA_DR6: CV_HREG_e = 262;
pub const CV_SHMEDIA_DR60: CV_HREG_e = 316;
pub const CV_SHMEDIA_DR62: CV_HREG_e = 318;
pub const CV_SHMEDIA_DR8: CV_HREG_e = 264;
pub const CV_SHMEDIA_FPSCR: CV_HREG_e = 2064;
pub const CV_SHMEDIA_FPUL: CV_HREG_e = 160;
pub const CV_SHMEDIA_FR0: CV_HREG_e = 128;
pub const CV_SHMEDIA_FR1: CV_HREG_e = 129;
pub const CV_SHMEDIA_FR10: CV_HREG_e = 138;
pub const CV_SHMEDIA_FR11: CV_HREG_e = 139;
pub const CV_SHMEDIA_FR12: CV_HREG_e = 140;
pub const CV_SHMEDIA_FR13: CV_HREG_e = 141;
pub const CV_SHMEDIA_FR14: CV_HREG_e = 142;
pub const CV_SHMEDIA_FR15: CV_HREG_e = 143;
pub const CV_SHMEDIA_FR16: CV_HREG_e = 144;
pub const CV_SHMEDIA_FR17: CV_HREG_e = 145;
pub const CV_SHMEDIA_FR18: CV_HREG_e = 146;
pub const CV_SHMEDIA_FR19: CV_HREG_e = 147;
pub const CV_SHMEDIA_FR2: CV_HREG_e = 130;
pub const CV_SHMEDIA_FR20: CV_HREG_e = 148;
pub const CV_SHMEDIA_FR21: CV_HREG_e = 149;
pub const CV_SHMEDIA_FR22: CV_HREG_e = 150;
pub const CV_SHMEDIA_FR23: CV_HREG_e = 151;
pub const CV_SHMEDIA_FR24: CV_HREG_e = 152;
pub const CV_SHMEDIA_FR25: CV_HREG_e = 153;
pub const CV_SHMEDIA_FR26: CV_HREG_e = 154;
pub const CV_SHMEDIA_FR27: CV_HREG_e = 155;
pub const CV_SHMEDIA_FR28: CV_HREG_e = 156;
pub const CV_SHMEDIA_FR29: CV_HREG_e = 157;
pub const CV_SHMEDIA_FR3: CV_HREG_e = 131;
pub const CV_SHMEDIA_FR30: CV_HREG_e = 158;
pub const CV_SHMEDIA_FR31: CV_HREG_e = 159;
pub const CV_SHMEDIA_FR32: CV_HREG_e = 160;
pub const CV_SHMEDIA_FR33: CV_HREG_e = 161;
pub const CV_SHMEDIA_FR34: CV_HREG_e = 162;
pub const CV_SHMEDIA_FR35: CV_HREG_e = 163;
pub const CV_SHMEDIA_FR36: CV_HREG_e = 164;
pub const CV_SHMEDIA_FR37: CV_HREG_e = 165;
pub const CV_SHMEDIA_FR38: CV_HREG_e = 166;
pub const CV_SHMEDIA_FR39: CV_HREG_e = 167;
pub const CV_SHMEDIA_FR4: CV_HREG_e = 132;
pub const CV_SHMEDIA_FR40: CV_HREG_e = 168;
pub const CV_SHMEDIA_FR41: CV_HREG_e = 169;
pub const CV_SHMEDIA_FR42: CV_HREG_e = 170;
pub const CV_SHMEDIA_FR43: CV_HREG_e = 171;
pub const CV_SHMEDIA_FR44: CV_HREG_e = 172;
pub const CV_SHMEDIA_FR45: CV_HREG_e = 173;
pub const CV_SHMEDIA_FR46: CV_HREG_e = 174;
pub const CV_SHMEDIA_FR47: CV_HREG_e = 175;
pub const CV_SHMEDIA_FR48: CV_HREG_e = 176;
pub const CV_SHMEDIA_FR49: CV_HREG_e = 177;
pub const CV_SHMEDIA_FR5: CV_HREG_e = 133;
pub const CV_SHMEDIA_FR50: CV_HREG_e = 178;
pub const CV_SHMEDIA_FR51: CV_HREG_e = 179;
pub const CV_SHMEDIA_FR52: CV_HREG_e = 180;
pub const CV_SHMEDIA_FR53: CV_HREG_e = 181;
pub const CV_SHMEDIA_FR54: CV_HREG_e = 182;
pub const CV_SHMEDIA_FR55: CV_HREG_e = 183;
pub const CV_SHMEDIA_FR56: CV_HREG_e = 184;
pub const CV_SHMEDIA_FR57: CV_HREG_e = 185;
pub const CV_SHMEDIA_FR58: CV_HREG_e = 186;
pub const CV_SHMEDIA_FR59: CV_HREG_e = 187;
pub const CV_SHMEDIA_FR6: CV_HREG_e = 134;
pub const CV_SHMEDIA_FR60: CV_HREG_e = 188;
pub const CV_SHMEDIA_FR61: CV_HREG_e = 189;
pub const CV_SHMEDIA_FR62: CV_HREG_e = 190;
pub const CV_SHMEDIA_FR63: CV_HREG_e = 191;
pub const CV_SHMEDIA_FR7: CV_HREG_e = 135;
pub const CV_SHMEDIA_FR8: CV_HREG_e = 136;
pub const CV_SHMEDIA_FR9: CV_HREG_e = 137;
pub const CV_SHMEDIA_FV0: CV_HREG_e = 512;
pub const CV_SHMEDIA_FV12: CV_HREG_e = 524;
pub const CV_SHMEDIA_FV16: CV_HREG_e = 528;
pub const CV_SHMEDIA_FV20: CV_HREG_e = 532;
pub const CV_SHMEDIA_FV24: CV_HREG_e = 536;
pub const CV_SHMEDIA_FV28: CV_HREG_e = 540;
pub const CV_SHMEDIA_FV32: CV_HREG_e = 544;
pub const CV_SHMEDIA_FV36: CV_HREG_e = 548;
pub const CV_SHMEDIA_FV4: CV_HREG_e = 516;
pub const CV_SHMEDIA_FV40: CV_HREG_e = 552;
pub const CV_SHMEDIA_FV44: CV_HREG_e = 556;
pub const CV_SHMEDIA_FV48: CV_HREG_e = 560;
pub const CV_SHMEDIA_FV52: CV_HREG_e = 564;
pub const CV_SHMEDIA_FV56: CV_HREG_e = 568;
pub const CV_SHMEDIA_FV60: CV_HREG_e = 572;
pub const CV_SHMEDIA_FV8: CV_HREG_e = 520;
pub const CV_SHMEDIA_GBR: CV_HREG_e = 26;
pub const CV_SHMEDIA_MACH: CV_HREG_e = 91;
pub const CV_SHMEDIA_MACL: CV_HREG_e = 90;
pub const CV_SHMEDIA_MTRX0: CV_HREG_e = 1024;
pub const CV_SHMEDIA_MTRX16: CV_HREG_e = 1040;
pub const CV_SHMEDIA_MTRX32: CV_HREG_e = 1056;
pub const CV_SHMEDIA_MTRX48: CV_HREG_e = 1072;
pub const CV_SHMEDIA_NOREG: CV_HREG_e = 0;
pub const CV_SHMEDIA_PC: CV_HREG_e = 93;
pub const CV_SHMEDIA_PR: CV_HREG_e = 28;
pub const CV_SHMEDIA_R0: CV_HREG_e = 10;
pub const CV_SHMEDIA_R1: CV_HREG_e = 11;
pub const CV_SHMEDIA_R10: CV_HREG_e = 20;
pub const CV_SHMEDIA_R11: CV_HREG_e = 21;
pub const CV_SHMEDIA_R12: CV_HREG_e = 22;
pub const CV_SHMEDIA_R13: CV_HREG_e = 23;
pub const CV_SHMEDIA_R14: CV_HREG_e = 24;
pub const CV_SHMEDIA_R15: CV_HREG_e = 25;
pub const CV_SHMEDIA_R16: CV_HREG_e = 26;
pub const CV_SHMEDIA_R17: CV_HREG_e = 27;
pub const CV_SHMEDIA_R18: CV_HREG_e = 28;
pub const CV_SHMEDIA_R19: CV_HREG_e = 29;
pub const CV_SHMEDIA_R2: CV_HREG_e = 12;
pub const CV_SHMEDIA_R20: CV_HREG_e = 30;
pub const CV_SHMEDIA_R21: CV_HREG_e = 31;
pub const CV_SHMEDIA_R22: CV_HREG_e = 32;
pub const CV_SHMEDIA_R23: CV_HREG_e = 33;
pub const CV_SHMEDIA_R24: CV_HREG_e = 34;
pub const CV_SHMEDIA_R25: CV_HREG_e = 35;
pub const CV_SHMEDIA_R26: CV_HREG_e = 36;
pub const CV_SHMEDIA_R27: CV_HREG_e = 37;
pub const CV_SHMEDIA_R28: CV_HREG_e = 38;
pub const CV_SHMEDIA_R29: CV_HREG_e = 39;
pub const CV_SHMEDIA_R3: CV_HREG_e = 13;
pub const CV_SHMEDIA_R30: CV_HREG_e = 40;
pub const CV_SHMEDIA_R31: CV_HREG_e = 41;
pub const CV_SHMEDIA_R32: CV_HREG_e = 42;
pub const CV_SHMEDIA_R33: CV_HREG_e = 43;
pub const CV_SHMEDIA_R34: CV_HREG_e = 44;
pub const CV_SHMEDIA_R35: CV_HREG_e = 45;
pub const CV_SHMEDIA_R36: CV_HREG_e = 46;
pub const CV_SHMEDIA_R37: CV_HREG_e = 47;
pub const CV_SHMEDIA_R38: CV_HREG_e = 48;
pub const CV_SHMEDIA_R39: CV_HREG_e = 49;
pub const CV_SHMEDIA_R4: CV_HREG_e = 14;
pub const CV_SHMEDIA_R40: CV_HREG_e = 50;
pub const CV_SHMEDIA_R41: CV_HREG_e = 51;
pub const CV_SHMEDIA_R42: CV_HREG_e = 52;
pub const CV_SHMEDIA_R43: CV_HREG_e = 53;
pub const CV_SHMEDIA_R44: CV_HREG_e = 54;
pub const CV_SHMEDIA_R45: CV_HREG_e = 55;
pub const CV_SHMEDIA_R46: CV_HREG_e = 56;
pub const CV_SHMEDIA_R47: CV_HREG_e = 57;
pub const CV_SHMEDIA_R48: CV_HREG_e = 58;
pub const CV_SHMEDIA_R49: CV_HREG_e = 59;
pub const CV_SHMEDIA_R5: CV_HREG_e = 15;
pub const CV_SHMEDIA_R50: CV_HREG_e = 60;
pub const CV_SHMEDIA_R51: CV_HREG_e = 61;
pub const CV_SHMEDIA_R52: CV_HREG_e = 62;
pub const CV_SHMEDIA_R53: CV_HREG_e = 63;
pub const CV_SHMEDIA_R54: CV_HREG_e = 64;
pub const CV_SHMEDIA_R55: CV_HREG_e = 65;
pub const CV_SHMEDIA_R56: CV_HREG_e = 66;
pub const CV_SHMEDIA_R57: CV_HREG_e = 67;
pub const CV_SHMEDIA_R58: CV_HREG_e = 68;
pub const CV_SHMEDIA_R59: CV_HREG_e = 69;
pub const CV_SHMEDIA_R6: CV_HREG_e = 16;
pub const CV_SHMEDIA_R60: CV_HREG_e = 70;
pub const CV_SHMEDIA_R61: CV_HREG_e = 71;
pub const CV_SHMEDIA_R62: CV_HREG_e = 72;
pub const CV_SHMEDIA_R63: CV_HREG_e = 73;
pub const CV_SHMEDIA_R7: CV_HREG_e = 17;
pub const CV_SHMEDIA_R8: CV_HREG_e = 18;
pub const CV_SHMEDIA_R9: CV_HREG_e = 19;
pub const CV_SHMEDIA_SR: CV_HREG_e = 2000;
pub const CV_SHMEDIA_T: CV_HREG_e = 92;
pub const CV_SHMEDIA_TR0: CV_HREG_e = 74;
pub const CV_SHMEDIA_TR1: CV_HREG_e = 75;
pub const CV_SHMEDIA_TR10: CV_HREG_e = 84;
pub const CV_SHMEDIA_TR11: CV_HREG_e = 85;
pub const CV_SHMEDIA_TR12: CV_HREG_e = 86;
pub const CV_SHMEDIA_TR13: CV_HREG_e = 87;
pub const CV_SHMEDIA_TR14: CV_HREG_e = 88;
pub const CV_SHMEDIA_TR15: CV_HREG_e = 89;
pub const CV_SHMEDIA_TR2: CV_HREG_e = 76;
pub const CV_SHMEDIA_TR3: CV_HREG_e = 77;
pub const CV_SHMEDIA_TR4: CV_HREG_e = 78;
pub const CV_SHMEDIA_TR5: CV_HREG_e = 79;
pub const CV_SHMEDIA_TR6: CV_HREG_e = 80;
pub const CV_SHMEDIA_TR7: CV_HREG_e = 81;
pub const CV_SHMEDIA_TR8: CV_HREG_e = 82;
pub const CV_SHMEDIA_TR9: CV_HREG_e = 83;
pub const CV_SH_FpR0: CV_HREG_e = 80;
pub const CV_SH_FpR1: CV_HREG_e = 81;
pub const CV_SH_FpR10: CV_HREG_e = 90;
pub const CV_SH_FpR11: CV_HREG_e = 91;
pub const CV_SH_FpR12: CV_HREG_e = 92;
pub const CV_SH_FpR13: CV_HREG_e = 93;
pub const CV_SH_FpR14: CV_HREG_e = 94;
pub const CV_SH_FpR15: CV_HREG_e = 95;
pub const CV_SH_FpR2: CV_HREG_e = 82;
pub const CV_SH_FpR3: CV_HREG_e = 83;
pub const CV_SH_FpR4: CV_HREG_e = 84;
pub const CV_SH_FpR5: CV_HREG_e = 85;
pub const CV_SH_FpR6: CV_HREG_e = 86;
pub const CV_SH_FpR7: CV_HREG_e = 87;
pub const CV_SH_FpR8: CV_HREG_e = 88;
pub const CV_SH_FpR9: CV_HREG_e = 89;
pub const CV_SH_Fpscr: CV_HREG_e = 75;
pub const CV_SH_Fpul: CV_HREG_e = 76;
pub const CV_SH_XFpR0: CV_HREG_e = 96;
pub const CV_SH_XFpR1: CV_HREG_e = 97;
pub const CV_SH_XFpR10: CV_HREG_e = 106;
pub const CV_SH_XFpR11: CV_HREG_e = 107;
pub const CV_SH_XFpR12: CV_HREG_e = 108;
pub const CV_SH_XFpR13: CV_HREG_e = 109;
pub const CV_SH_XFpR14: CV_HREG_e = 110;
pub const CV_SH_XFpR15: CV_HREG_e = 111;
pub const CV_SH_XFpR2: CV_HREG_e = 98;
pub const CV_SH_XFpR3: CV_HREG_e = 99;
pub const CV_SH_XFpR4: CV_HREG_e = 100;
pub const CV_SH_XFpR5: CV_HREG_e = 101;
pub const CV_SH_XFpR6: CV_HREG_e = 102;
pub const CV_SH_XFpR7: CV_HREG_e = 103;
pub const CV_SH_XFpR8: CV_HREG_e = 104;
pub const CV_SH_XFpR9: CV_HREG_e = 105;
pub type CV_SourceChksum_t = i32;
pub const CV_TRI_A0: CV_HREG_e = 26;
pub const CV_TRI_A1: CV_HREG_e = 27;
pub const CV_TRI_A10: CV_HREG_e = 36;
pub const CV_TRI_A11: CV_HREG_e = 37;
pub const CV_TRI_A12: CV_HREG_e = 38;
pub const CV_TRI_A13: CV_HREG_e = 39;
pub const CV_TRI_A14: CV_HREG_e = 40;
pub const CV_TRI_A15: CV_HREG_e = 41;
pub const CV_TRI_A2: CV_HREG_e = 28;
pub const CV_TRI_A3: CV_HREG_e = 29;
pub const CV_TRI_A4: CV_HREG_e = 30;
pub const CV_TRI_A5: CV_HREG_e = 31;
pub const CV_TRI_A6: CV_HREG_e = 32;
pub const CV_TRI_A7: CV_HREG_e = 33;
pub const CV_TRI_A8: CV_HREG_e = 34;
pub const CV_TRI_A9: CV_HREG_e = 35;
pub const CV_TRI_ASI: CV_HREG_e = 78;
pub const CV_TRI_BIV: CV_HREG_e = 65;
pub const CV_TRI_BTV: CV_HREG_e = 66;
pub const CV_TRI_CPMx_0: CV_HREG_e = 68;
pub const CV_TRI_CPMx_1: CV_HREG_e = 69;
pub const CV_TRI_CPMx_2: CV_HREG_e = 70;
pub const CV_TRI_CPMx_3: CV_HREG_e = 71;
pub const CV_TRI_CPRx_0: CV_HREG_e = 68;
pub const CV_TRI_CPRx_1: CV_HREG_e = 69;
pub const CV_TRI_CPRx_2: CV_HREG_e = 70;
pub const CV_TRI_CPRx_3: CV_HREG_e = 71;
pub const CV_TRI_CREVT: CV_HREG_e = 75;
pub const CV_TRI_D0: CV_HREG_e = 10;
pub const CV_TRI_D1: CV_HREG_e = 11;
pub const CV_TRI_D10: CV_HREG_e = 20;
pub const CV_TRI_D11: CV_HREG_e = 21;
pub const CV_TRI_D12: CV_HREG_e = 22;
pub const CV_TRI_D13: CV_HREG_e = 23;
pub const CV_TRI_D14: CV_HREG_e = 24;
pub const CV_TRI_D15: CV_HREG_e = 25;
pub const CV_TRI_D2: CV_HREG_e = 12;
pub const CV_TRI_D3: CV_HREG_e = 13;
pub const CV_TRI_D4: CV_HREG_e = 14;
pub const CV_TRI_D5: CV_HREG_e = 15;
pub const CV_TRI_D6: CV_HREG_e = 16;
pub const CV_TRI_D7: CV_HREG_e = 17;
pub const CV_TRI_D8: CV_HREG_e = 18;
pub const CV_TRI_D9: CV_HREG_e = 19;
pub const CV_TRI_DBGSSR: CV_HREG_e = 72;
pub const CV_TRI_DPMx_0: CV_HREG_e = 68;
pub const CV_TRI_DPMx_1: CV_HREG_e = 69;
pub const CV_TRI_DPMx_2: CV_HREG_e = 70;
pub const CV_TRI_DPMx_3: CV_HREG_e = 71;
pub const CV_TRI_DPRx_0: CV_HREG_e = 68;
pub const CV_TRI_DPRx_1: CV_HREG_e = 69;
pub const CV_TRI_DPRx_2: CV_HREG_e = 70;
pub const CV_TRI_DPRx_3: CV_HREG_e = 71;
pub const CV_TRI_E0: CV_HREG_e = 42;
pub const CV_TRI_E10: CV_HREG_e = 47;
pub const CV_TRI_E12: CV_HREG_e = 48;
pub const CV_TRI_E14: CV_HREG_e = 49;
pub const CV_TRI_E2: CV_HREG_e = 43;
pub const CV_TRI_E4: CV_HREG_e = 44;
pub const CV_TRI_E6: CV_HREG_e = 45;
pub const CV_TRI_E8: CV_HREG_e = 46;
pub const CV_TRI_EA0: CV_HREG_e = 50;
pub const CV_TRI_EA10: CV_HREG_e = 55;
pub const CV_TRI_EA12: CV_HREG_e = 56;
pub const CV_TRI_EA14: CV_HREG_e = 57;
pub const CV_TRI_EA2: CV_HREG_e = 51;
pub const CV_TRI_EA4: CV_HREG_e = 52;
pub const CV_TRI_EA6: CV_HREG_e = 53;
pub const CV_TRI_EA8: CV_HREG_e = 54;
pub const CV_TRI_EXEVT: CV_HREG_e = 73;
pub const CV_TRI_FCX: CV_HREG_e = 61;
pub const CV_TRI_ICR: CV_HREG_e = 64;
pub const CV_TRI_ISP: CV_HREG_e = 63;
pub const CV_TRI_LCX: CV_HREG_e = 62;
pub const CV_TRI_MMUCON: CV_HREG_e = 77;
pub const CV_TRI_NOREG: CV_HREG_e = 0;
pub const CV_TRI_PC: CV_HREG_e = 60;
pub const CV_TRI_PCXI: CV_HREG_e = 59;
pub const CV_TRI_PSW: CV_HREG_e = 58;
pub const CV_TRI_SWEVT: CV_HREG_e = 74;
pub const CV_TRI_SYSCON: CV_HREG_e = 67;
pub const CV_TRI_TFA: CV_HREG_e = 82;
pub const CV_TRI_TPA: CV_HREG_e = 80;
pub const CV_TRI_TPX: CV_HREG_e = 81;
pub const CV_TRI_TRnEVT: CV_HREG_e = 76;
pub const CV_TRI_TVA: CV_HREG_e = 79;
pub type CV_access_e = i32;
pub type CV_builtin_e = i32;
pub type CV_call_e = i32;
pub type CV_modifier_e = i32;
pub const CV_private: CV_access_e = 1;
pub const CV_protected: CV_access_e = 2;
pub const CV_public: CV_access_e = 3;
pub const DataIsConstant: DataKind = 9;
pub const DataIsFileStatic: DataKind = 5;
pub const DataIsGlobal: DataKind = 6;
pub const DataIsLocal: DataKind = 1;
pub const DataIsMember: DataKind = 7;
pub const DataIsObjectPtr: DataKind = 4;
pub const DataIsParam: DataKind = 3;
pub const DataIsStaticLocal: DataKind = 2;
pub const DataIsStaticMember: DataKind = 8;
pub const DataIsUnknown: DataKind = 0;
pub type DataKind = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DiaAddressMapEntry {
    pub rva: u32,
    pub rvaTo: u32,
}
pub const DiaSource: windows_core::GUID =
    windows_core::GUID::from_u128(0xe6756135_1e65_4d17_8576_610761398c3c);
pub const DiaSourceAlt: windows_core::GUID =
    windows_core::GUID::from_u128(0x91904831_49ca_4766_b95c_25397e2dd6dc);
pub const DiaStackWalker: windows_core::GUID =
    windows_core::GUID::from_u128(0xce4a85db_5768_475b_a4e1_c0bca2112a6b);
pub const DiaTable_Dbg: windows_core::PCWSTR = windows_core::w!("Dbg");
pub const DiaTable_FrameData: windows_core::PCWSTR = windows_core::w!("FrameData");
pub const DiaTable_InjSrc: windows_core::PCWSTR = windows_core::w!("InjectedSource");
pub const DiaTable_InputAssemblyFiles: windows_core::PCWSTR =
    windows_core::w!("InputAssemblyFiles");
pub const DiaTable_LineNums: windows_core::PCWSTR = windows_core::w!("LineNumbers");
pub const DiaTable_Sections: windows_core::PCWSTR = windows_core::w!("Sections");
pub const DiaTable_SegMap: windows_core::PCWSTR = windows_core::w!("SegmentMap");
pub const DiaTable_SrcFiles: windows_core::PCWSTR = windows_core::w!("SourceFiles");
pub const DiaTable_Symbols: windows_core::PCWSTR = windows_core::w!("Symbols");
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiaTagValue {
    pub value: [u8; 16],
    pub valueSizeBytes: u8,
}
impl Default for DiaTagValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const E_DIA_COFF_ACCESS: i32 = -2140340024;
pub const E_DIA_COMP_PDB_ACCESS: i32 = -2140340023;
pub const E_DIA_FRAME_ACCESS: i32 = -2140340122;
pub const E_DIA_INPROLOG: i32 = -2140340124;
pub const E_DIA_SYNTAX: i32 = -2140340123;
pub const E_DIA_VALUE: i32 = -2140340121;
pub const E_PDB_ACCESS_DENIED: i32 = -2140340208;
pub const E_PDB_CORRUPT: i32 = -2140340210;
pub const E_PDB_DBG_NOT_FOUND: i32 = -2140340205;
pub const E_PDB_DEBUG_INFO_NOT_IN_PDB: i32 = -2140340201;
pub const E_PDB_FILE_SYSTEM: i32 = -2140340220;
pub const E_PDB_FORMAT: i32 = -2140340212;
pub const E_PDB_IFC_DEBUG_STREAM_EMPTY: i32 = -2140340191;
pub const E_PDB_IFC_DEBUG_STREAM_FAILED_OPEN: i32 = -2140340192;
pub const E_PDB_IFC_DEBUG_STREAM_HASH_MISMATCH: i32 = -2140340190;
pub const E_PDB_IFC_FAILED_TO_LOAD: i32 = -2140340194;
pub const E_PDB_IFC_FAILED_TO_LOAD_MISMATCH_HASH: i32 = -2140340193;
pub const E_PDB_IFC_RECORD_MISSING_DESIGNATOR: i32 = -2140340196;
pub const E_PDB_IFC_RECORD_MISSING_REFERENCE: i32 = -2140340195;
pub const E_PDB_ILLEGAL_TYPE_EDIT: i32 = -2140340207;
pub const E_PDB_INVALID_AGE: i32 = -2140340217;
pub const E_PDB_INVALID_EXECUTABLE: i32 = -2140340206;
pub const E_PDB_INVALID_EXE_TIMESTAMP: i32 = -2140340203;
pub const E_PDB_INVALID_SIG: i32 = -2140340218;
pub const E_PDB_LIMIT: i32 = -2140340211;
pub const E_PDB_MAX: i32 = -2140340197;
pub const E_PDB_NOT_FOUND: i32 = -2140340219;
pub const E_PDB_NOT_IMPLEMENTED: i32 = -2140340214;
pub const E_PDB_NO_DEBUG_INFO: i32 = -2140340204;
pub const E_PDB_OBJECT_DISPOSED: i32 = -2140340198;
pub const E_PDB_OK: i32 = -2140340223;
pub const E_PDB_OUT_OF_MEMORY: i32 = -2140340221;
pub const E_PDB_OUT_OF_TI: i32 = -2140340215;
pub const E_PDB_PRECOMP_REQUIRED: i32 = -2140340216;
pub const E_PDB_RESERVED: i32 = -2140340202;
pub const E_PDB_SYMSRV_BAD_CACHE_PATH: i32 = -2140340200;
pub const E_PDB_SYMSRV_CACHE_FULL: i32 = -2140340199;
pub const E_PDB_TI16: i32 = -2140340209;
pub const E_PDB_USAGE: i32 = -2140340222;
pub const E_PDB_V1_PDB: i32 = -2140340213;
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FPODATA {
    pub ulOffStart: u32,
    pub cbProcSize: u32,
    pub cdwLocals: u32,
    pub cdwParams: u16,
    pub cdwFlags: u16,
}
pub const FrameTypeFPO: StackFrameTypeEnum = 0;
pub const FrameTypeFrameData: StackFrameTypeEnum = 4;
pub const FrameTypeStandard: StackFrameTypeEnum = 3;
pub const FrameTypeTSS: StackFrameTypeEnum = 2;
pub const FrameTypeTrap: StackFrameTypeEnum = 1;
pub const FrameTypeUnknown: StackFrameTypeEnum = -1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HINSTANCE(pub *mut core::ffi::c_void);
pub type HMODULE = HINSTANCE;
windows_core::imp::define_interface!(
    IClassFactory,
    IClassFactory_Vtbl,
    0x00000001_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IClassFactory, windows_core::IUnknown);
impl IClassFactory {
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).CreateInstance)(
                windows_core::Interface::as_raw(self),
                punkouter.param().abi(),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn LockServer(&self, flock: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).LockServer)(
                windows_core::Interface::as_raw(self),
                flock.into(),
            )
        }
    }
}
#[repr(C)]
pub struct IClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LockServer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
pub trait IClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        punkouter: windows_core::Ref<windows_core::IUnknown>,
        riid: *const windows_core::GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn LockServer(&self, flock: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IClassFactory_Vtbl {
    pub const fn new<Identity: IClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IClassFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            punkouter: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            ppvobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&punkouter),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppvobject),
                )
                .into()
            }
        }
        unsafe extern "system" fn LockServer<Identity: IClassFactory_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            flock: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory_Impl::LockServer(this, core::mem::transmute_copy(&flock)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            LockServer: LockServer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IClassFactory {}
windows_core::imp::define_interface!(
    IDiaAddressMap,
    IDiaAddressMap_Vtbl,
    0xb62a2e7a_067a_4ea3_b598_04c09717502c
);
windows_core::imp::interface_hierarchy!(IDiaAddressMap, windows_core::IUnknown);
impl IDiaAddressMap {
    pub unsafe fn addressMapEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressMapEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetaddressMapEnabled(&self, newval: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetaddressMapEnabled)(
                windows_core::Interface::as_raw(self),
                newval.into(),
            )
        }
    }
    pub unsafe fn relativeVirtualAddressEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).relativeVirtualAddressEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetrelativeVirtualAddressEnabled(&self, newval: bool) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetrelativeVirtualAddressEnabled)(
                windows_core::Interface::as_raw(self),
                newval.into(),
            )
        }
    }
    pub unsafe fn imageAlign(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).imageAlign)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetimageAlign(&self, newval: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetimageAlign)(
                windows_core::Interface::as_raw(self),
                newval,
            )
        }
    }
    pub unsafe fn set_imageHeaders(
        &self,
        cbdata: u32,
        pbdata: *const u8,
        originalheaders: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).set_imageHeaders)(
                windows_core::Interface::as_raw(self),
                cbdata,
                pbdata,
                originalheaders.into(),
            )
        }
    }
    pub unsafe fn set_addressMap(
        &self,
        cdata: u32,
        pdata: *const DiaAddressMapEntry,
        imagetosymbols: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).set_addressMap)(
                windows_core::Interface::as_raw(self),
                cdata,
                pdata,
                imagetosymbols.into(),
            )
        }
    }
}
#[repr(C)]
pub struct IDiaAddressMap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub addressMapEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetaddressMapEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub relativeVirtualAddressEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetrelativeVirtualAddressEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub imageAlign:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetimageAlign:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub set_imageHeaders: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const u8,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub set_addressMap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const DiaAddressMapEntry,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
pub trait IDiaAddressMap_Impl: windows_core::IUnknownImpl {
    fn addressMapEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetaddressMapEnabled(&self, newval: windows_core::BOOL) -> windows_core::Result<()>;
    fn relativeVirtualAddressEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetrelativeVirtualAddressEnabled(
        &self,
        newval: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn imageAlign(&self) -> windows_core::Result<u32>;
    fn SetimageAlign(&self, newval: u32) -> windows_core::Result<()>;
    fn set_imageHeaders(
        &self,
        cbdata: u32,
        pbdata: *const u8,
        originalheaders: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn set_addressMap(
        &self,
        cdata: u32,
        pdata: *const DiaAddressMapEntry,
        imagetosymbols: windows_core::BOOL,
    ) -> windows_core::Result<()>;
}
impl IDiaAddressMap_Vtbl {
    pub const fn new<Identity: IDiaAddressMap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn addressMapEnabled<
            Identity: IDiaAddressMap_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaAddressMap_Impl::addressMapEnabled(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetaddressMapEnabled<
            Identity: IDiaAddressMap_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            newval: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaAddressMap_Impl::SetaddressMapEnabled(this, core::mem::transmute_copy(&newval))
                    .into()
            }
        }
        unsafe extern "system" fn relativeVirtualAddressEnabled<
            Identity: IDiaAddressMap_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaAddressMap_Impl::relativeVirtualAddressEnabled(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetrelativeVirtualAddressEnabled<
            Identity: IDiaAddressMap_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            newval: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaAddressMap_Impl::SetrelativeVirtualAddressEnabled(
                    this,
                    core::mem::transmute_copy(&newval),
                )
                .into()
            }
        }
        unsafe extern "system" fn imageAlign<Identity: IDiaAddressMap_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaAddressMap_Impl::imageAlign(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetimageAlign<
            Identity: IDiaAddressMap_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            newval: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaAddressMap_Impl::SetimageAlign(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn set_imageHeaders<
            Identity: IDiaAddressMap_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cbdata: u32,
            pbdata: *const u8,
            originalheaders: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaAddressMap_Impl::set_imageHeaders(
                    this,
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pbdata),
                    core::mem::transmute_copy(&originalheaders),
                )
                .into()
            }
        }
        unsafe extern "system" fn set_addressMap<
            Identity: IDiaAddressMap_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cdata: u32,
            pdata: *const DiaAddressMapEntry,
            imagetosymbols: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaAddressMap_Impl::set_addressMap(
                    this,
                    core::mem::transmute_copy(&cdata),
                    core::mem::transmute_copy(&pdata),
                    core::mem::transmute_copy(&imagetosymbols),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            addressMapEnabled: addressMapEnabled::<Identity, OFFSET>,
            SetaddressMapEnabled: SetaddressMapEnabled::<Identity, OFFSET>,
            relativeVirtualAddressEnabled: relativeVirtualAddressEnabled::<Identity, OFFSET>,
            SetrelativeVirtualAddressEnabled: SetrelativeVirtualAddressEnabled::<Identity, OFFSET>,
            imageAlign: imageAlign::<Identity, OFFSET>,
            SetimageAlign: SetimageAlign::<Identity, OFFSET>,
            set_imageHeaders: set_imageHeaders::<Identity, OFFSET>,
            set_addressMap: set_addressMap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaAddressMap as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaAddressMap {}
windows_core::imp::define_interface!(
    IDiaDataSource,
    IDiaDataSource_Vtbl,
    0x79f1bb5f_b66e_48e5_b6a9_1545c323ca3d
);
windows_core::imp::interface_hierarchy!(IDiaDataSource, windows_core::IUnknown);
impl IDiaDataSource {
    pub unsafe fn lastError(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lastError)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn loadDataFromPdb<P0>(&self, pdbpath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).loadDataFromPdb)(
                windows_core::Interface::as_raw(self),
                pdbpath.param().abi(),
            )
        }
    }
    pub unsafe fn loadAndValidateDataFromPdb<P0>(
        &self,
        pdbpath: P0,
        pcsig70: *const windows_core::GUID,
        sig: u32,
        age: u32,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).loadAndValidateDataFromPdb)(
                windows_core::Interface::as_raw(self),
                pdbpath.param().abi(),
                pcsig70,
                sig,
                age,
            )
        }
    }
    pub unsafe fn loadDataForExe<P0, P1, P2>(
        &self,
        executable: P0,
        searchpath: P1,
        pcallback: P2,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).loadDataForExe)(
                windows_core::Interface::as_raw(self),
                executable.param().abi(),
                searchpath.param().abi(),
                pcallback.param().abi(),
            )
        }
    }
    pub unsafe fn openSession(&self) -> windows_core::Result<IDiaSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).openSession)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn loadDataFromCodeViewInfo<P0, P1, P4>(
        &self,
        executable: P0,
        searchpath: P1,
        cbcvinfo: u32,
        pbcvinfo: *const u8,
        pcallback: P4,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).loadDataFromCodeViewInfo)(
                windows_core::Interface::as_raw(self),
                executable.param().abi(),
                searchpath.param().abi(),
                cbcvinfo,
                pbcvinfo,
                pcallback.param().abi(),
            )
        }
    }
    pub unsafe fn loadDataFromMiscInfo<P0, P1, P7>(
        &self,
        executable: P0,
        searchpath: P1,
        timestampexe: u32,
        timestampdbg: u32,
        sizeofexe: u32,
        cbmiscinfo: u32,
        pbmiscinfo: *const u8,
        pcallback: P7,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).loadDataFromMiscInfo)(
                windows_core::Interface::as_raw(self),
                executable.param().abi(),
                searchpath.param().abi(),
                timestampexe,
                timestampdbg,
                sizeofexe,
                cbmiscinfo,
                pbmiscinfo,
                pcallback.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct IDiaDataSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub lastError: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub loadDataFromPdb: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub loadAndValidateDataFromPdb: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *const windows_core::GUID,
        u32,
        u32,
    ) -> windows_core::HRESULT,
    pub loadDataForExe: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    loadDataFromIStream: usize,
    pub openSession: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub loadDataFromCodeViewInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        u32,
        *const u8,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub loadDataFromMiscInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        u32,
        u32,
        u32,
        u32,
        *const u8,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaDataSource {}
windows_core::imp::define_interface!(
    IDiaDataSourceEx,
    IDiaDataSourceEx_Vtbl,
    0x1a21eb69_962a_4bc4_8bd3_681797d38b23
);
impl core::ops::Deref for IDiaDataSourceEx {
    type Target = IDiaDataSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiaDataSourceEx, windows_core::IUnknown, IDiaDataSource);
impl IDiaDataSourceEx {
    pub unsafe fn loadDataFromPdbEx<P0>(
        &self,
        pdbpath: P0,
        fpdbprefetching: bool,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).loadDataFromPdbEx)(
                windows_core::Interface::as_raw(self),
                pdbpath.param().abi(),
                fpdbprefetching.into(),
            )
        }
    }
    pub unsafe fn loadAndValidateDataFromPdbEx<P0>(
        &self,
        pdbpath: P0,
        pcsig70: *const windows_core::GUID,
        sig: u32,
        age: u32,
        fpdbprefetching: bool,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).loadAndValidateDataFromPdbEx)(
                windows_core::Interface::as_raw(self),
                pdbpath.param().abi(),
                pcsig70,
                sig,
                age,
                fpdbprefetching.into(),
            )
        }
    }
    pub unsafe fn loadDataForExeEx<P0, P1, P2>(
        &self,
        executable: P0,
        searchpath: P1,
        pcallback: P2,
        fpdbprefetching: bool,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).loadDataForExeEx)(
                windows_core::Interface::as_raw(self),
                executable.param().abi(),
                searchpath.param().abi(),
                pcallback.param().abi(),
                fpdbprefetching.into(),
            )
        }
    }
    pub unsafe fn getStreamSize<P0>(&self, stream: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getStreamSize)(
                windows_core::Interface::as_raw(self),
                stream.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn getStreamRawData<P0>(
        &self,
        stream: P0,
        cboffset: u64,
        cbread: u64,
        pcbread: *mut u64,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).getStreamRawData)(
                windows_core::Interface::as_raw(self),
                stream.param().abi(),
                cboffset,
                cbread,
                pcbread as _,
                pbdata as _,
            )
        }
    }
    pub unsafe fn setPfnMiniPDBErrorCallback2(
        &self,
        pvcontext: *const core::ffi::c_void,
        pfn: PFNMINIPDBERRORCALLBACK2,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).setPfnMiniPDBErrorCallback2)(
                windows_core::Interface::as_raw(self),
                pvcontext,
                pfn,
            )
        }
    }
    pub unsafe fn ValidatePdb<P0>(
        &self,
        pdbpath: P0,
        pcsig70: *const windows_core::GUID,
        sig: u32,
        age: u32,
    ) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidatePdb)(
                windows_core::Interface::as_raw(self),
                pdbpath.param().abi(),
                pcsig70,
                sig,
                age,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaDataSourceEx_Vtbl {
    pub base__: IDiaDataSource_Vtbl,
    pub loadDataFromPdbEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub loadAndValidateDataFromPdbEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *const windows_core::GUID,
        u32,
        u32,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub loadDataForExeEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    loadDataFromIStreamEx: usize,
    pub getStreamSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut u64,
    ) -> windows_core::HRESULT,
    pub getStreamRawData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        u64,
        u64,
        *mut u64,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub setPfnMiniPDBErrorCallback2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        PFNMINIPDBERRORCALLBACK2,
    ) -> windows_core::HRESULT,
    pub ValidatePdb: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *const windows_core::GUID,
        u32,
        u32,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaDataSourceEx {}
windows_core::imp::define_interface!(
    IDiaDataSourceEx2,
    IDiaDataSourceEx2_Vtbl,
    0xd240c8dd_1a0f_456e_80a6_4f1d06bf5df4
);
impl core::ops::Deref for IDiaDataSourceEx2 {
    type Target = IDiaDataSourceEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaDataSourceEx2,
    windows_core::IUnknown,
    IDiaDataSource,
    IDiaDataSourceEx
);
impl IDiaDataSourceEx2 {
    pub unsafe fn findNamedStreams<P0>(
        &self,
        name: P0,
        compareflags: u32,
    ) -> windows_core::Result<IDiaEnumNamedStreams>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findNamedStreams)(
                windows_core::Interface::as_raw(self),
                name.param().abi(),
                compareflags,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaDataSourceEx2_Vtbl {
    pub base__: IDiaDataSourceEx_Vtbl,
    pub findNamedStreams: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaDataSourceEx2 {}
windows_core::imp::define_interface!(
    IDiaEnumDebugStreamData,
    IDiaEnumDebugStreamData_Vtbl,
    0x486943e8_d187_4a6b_a3c4_291259fff60d
);
windows_core::imp::interface_hierarchy!(IDiaEnumDebugStreamData, windows_core::IUnknown);
impl IDiaEnumDebugStreamData {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).name)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Item(
        &self,
        index: u32,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Item)(
                windows_core::Interface::as_raw(self),
                index,
                cbdata,
                pcbdata as _,
                pbdata as _,
            )
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                cbdata,
                pcbdata as _,
                pbdata as _,
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumDebugStreamData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut u32,
        *mut u8,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumDebugStreamData_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Item(
        &self,
        index: u32,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::Result<()>;
    fn Next(
        &self,
        celt: u32,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumDebugStreamData>;
}
impl IDiaEnumDebugStreamData_Vtbl {
    pub const fn new<Identity: IDiaEnumDebugStreamData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<
            Identity: IDiaEnumDebugStreamData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumDebugStreamData_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<
            Identity: IDiaEnumDebugStreamData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumDebugStreamData_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn name<
            Identity: IDiaEnumDebugStreamData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumDebugStreamData_Impl::name(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<
            Identity: IDiaEnumDebugStreamData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            cbdata: u32,
            pcbdata: *mut u32,
            pbdata: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumDebugStreamData_Impl::Item(
                    this,
                    core::mem::transmute_copy(&index),
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pcbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn Next<
            Identity: IDiaEnumDebugStreamData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            celt: u32,
            cbdata: u32,
            pcbdata: *mut u32,
            pbdata: *mut u8,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumDebugStreamData_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pcbdata),
                    core::mem::transmute_copy(&pbdata),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<
            Identity: IDiaEnumDebugStreamData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumDebugStreamData_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<
            Identity: IDiaEnumDebugStreamData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumDebugStreamData_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<
            Identity: IDiaEnumDebugStreamData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumDebugStreamData_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            name: name::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumDebugStreamData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumDebugStreamData {}
windows_core::imp::define_interface!(
    IDiaEnumDebugStreams,
    IDiaEnumDebugStreams_Vtbl,
    0x08cbb41e_47a6_4f87_92f1_1c9c87ced044
);
windows_core::imp::interface_hierarchy!(IDiaEnumDebugStreams, windows_core::IUnknown);
impl IDiaEnumDebugStreams {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaEnumDebugStreamData>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumDebugStreams_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    Item: usize,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaEnumDebugStreams {}
windows_core::imp::define_interface!(
    IDiaEnumFrameData,
    IDiaEnumFrameData_Vtbl,
    0x9fc77a4b_3c1c_44ed_a798_6c1deea53e1f
);
windows_core::imp::interface_hierarchy!(IDiaEnumFrameData, windows_core::IUnknown);
impl IDiaEnumFrameData {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Item(&self, index: u32) -> windows_core::Result<IDiaFrameData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaFrameData>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn frameByRVA(
        &self,
        relativevirtualaddress: u32,
    ) -> windows_core::Result<IDiaFrameData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).frameByRVA)(
                windows_core::Interface::as_raw(self),
                relativevirtualaddress,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn frameByVA(&self, virtualaddress: u64) -> windows_core::Result<IDiaFrameData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).frameByVA)(
                windows_core::Interface::as_raw(self),
                virtualaddress,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumFrameData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub frameByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub frameByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumFrameData_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: u32) -> windows_core::Result<IDiaFrameData>;
    fn Next(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaFrameData>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumFrameData>;
    fn frameByRVA(&self, relativevirtualaddress: u32) -> windows_core::Result<IDiaFrameData>;
    fn frameByVA(&self, virtualaddress: u64) -> windows_core::Result<IDiaFrameData>;
}
impl IDiaEnumFrameData_Vtbl {
    pub const fn new<Identity: IDiaEnumFrameData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<
            Identity: IDiaEnumFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumFrameData_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IDiaEnumFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumFrameData_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IDiaEnumFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            index: u32,
            frame: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumFrameData_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        frame.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IDiaEnumFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumFrameData_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IDiaEnumFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumFrameData_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IDiaEnumFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumFrameData_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IDiaEnumFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumFrameData_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn frameByRVA<
            Identity: IDiaEnumFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            relativevirtualaddress: u32,
            frame: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumFrameData_Impl::frameByRVA(
                    this,
                    core::mem::transmute_copy(&relativevirtualaddress),
                ) {
                    Ok(ok__) => {
                        frame.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn frameByVA<
            Identity: IDiaEnumFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            virtualaddress: u64,
            frame: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumFrameData_Impl::frameByVA(
                    this,
                    core::mem::transmute_copy(&virtualaddress),
                ) {
                    Ok(ok__) => {
                        frame.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            frameByRVA: frameByRVA::<Identity, OFFSET>,
            frameByVA: frameByVA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumFrameData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumFrameData {}
windows_core::imp::define_interface!(
    IDiaEnumInjectedSources,
    IDiaEnumInjectedSources_Vtbl,
    0xd5612573_6925_4468_8883_98cdec8c384a
);
windows_core::imp::interface_hierarchy!(IDiaEnumInjectedSources, windows_core::IUnknown);
impl IDiaEnumInjectedSources {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Item(&self, index: u32) -> windows_core::Result<IDiaInjectedSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaInjectedSource>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumInjectedSources_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumInjectedSources_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: u32) -> windows_core::Result<IDiaInjectedSource>;
    fn Next(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaInjectedSource>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumInjectedSources>;
}
impl IDiaEnumInjectedSources_Vtbl {
    pub const fn new<Identity: IDiaEnumInjectedSources_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<
            Identity: IDiaEnumInjectedSources_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumInjectedSources_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<
            Identity: IDiaEnumInjectedSources_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumInjectedSources_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<
            Identity: IDiaEnumInjectedSources_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            injectedsource: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumInjectedSources_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        injectedsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<
            Identity: IDiaEnumInjectedSources_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumInjectedSources_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<
            Identity: IDiaEnumInjectedSources_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumInjectedSources_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<
            Identity: IDiaEnumInjectedSources_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumInjectedSources_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<
            Identity: IDiaEnumInjectedSources_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumInjectedSources_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumInjectedSources as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumInjectedSources {}
windows_core::imp::define_interface!(
    IDiaEnumInputAssemblyFiles,
    IDiaEnumInputAssemblyFiles_Vtbl,
    0x1c7ff653_51f7_457e_8419_b20f57ef7e4d
);
windows_core::imp::interface_hierarchy!(IDiaEnumInputAssemblyFiles, windows_core::IUnknown);
impl IDiaEnumInputAssemblyFiles {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Item(&self, index: u32) -> windows_core::Result<IDiaInputAssemblyFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaInputAssemblyFile>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumInputAssemblyFiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumInputAssemblyFiles_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: u32) -> windows_core::Result<IDiaInputAssemblyFile>;
    fn Next(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaInputAssemblyFile>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumInputAssemblyFiles>;
}
impl IDiaEnumInputAssemblyFiles_Vtbl {
    pub const fn new<Identity: IDiaEnumInputAssemblyFiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<
            Identity: IDiaEnumInputAssemblyFiles_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumInputAssemblyFiles_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<
            Identity: IDiaEnumInputAssemblyFiles_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumInputAssemblyFiles_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<
            Identity: IDiaEnumInputAssemblyFiles_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            file: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumInputAssemblyFiles_Impl::Item(this, core::mem::transmute_copy(&index))
                {
                    Ok(ok__) => {
                        file.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<
            Identity: IDiaEnumInputAssemblyFiles_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumInputAssemblyFiles_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<
            Identity: IDiaEnumInputAssemblyFiles_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumInputAssemblyFiles_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<
            Identity: IDiaEnumInputAssemblyFiles_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumInputAssemblyFiles_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<
            Identity: IDiaEnumInputAssemblyFiles_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumInputAssemblyFiles_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumInputAssemblyFiles as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumInputAssemblyFiles {}
windows_core::imp::define_interface!(
    IDiaEnumLineNumbers,
    IDiaEnumLineNumbers_Vtbl,
    0xfe30e878_54ac_44f1_81ba_39de940f6052
);
windows_core::imp::interface_hierarchy!(IDiaEnumLineNumbers, windows_core::IUnknown);
impl IDiaEnumLineNumbers {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Item(&self, index: u32) -> windows_core::Result<IDiaLineNumber> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaLineNumber>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumLineNumbers_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumLineNumbers_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: u32) -> windows_core::Result<IDiaLineNumber>;
    fn Next(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaLineNumber>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumLineNumbers>;
}
impl IDiaEnumLineNumbers_Vtbl {
    pub const fn new<Identity: IDiaEnumLineNumbers_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<
            Identity: IDiaEnumLineNumbers_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumLineNumbers_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumLineNumbers_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            index: u32,
            linenumber: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumLineNumbers_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        linenumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumLineNumbers_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumLineNumbers_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumLineNumbers_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IDiaEnumLineNumbers_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumLineNumbers_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumLineNumbers as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumLineNumbers {}
windows_core::imp::define_interface!(
    IDiaEnumNamedStreams,
    IDiaEnumNamedStreams_Vtbl,
    0x2b01f5e0_98db_4824_a9a0_5192833bef47
);
windows_core::imp::interface_hierarchy!(IDiaEnumNamedStreams, windows_core::IUnknown);
impl IDiaEnumNamedStreams {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Next(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumNamedStreams_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumNamedStreams_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Next(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumNamedStreams>;
}
impl IDiaEnumNamedStreams_Vtbl {
    pub const fn new<Identity: IDiaEnumNamedStreams_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<
            Identity: IDiaEnumNamedStreams_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumNamedStreams_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<
            Identity: IDiaEnumNamedStreams_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumNamedStreams_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IDiaEnumNamedStreams_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pname: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumNamedStreams_Impl::Next(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Skip<Identity: IDiaEnumNamedStreams_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumNamedStreams_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<
            Identity: IDiaEnumNamedStreams_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumNamedStreams_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<
            Identity: IDiaEnumNamedStreams_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumNamedStreams_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumNamedStreams as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumNamedStreams {}
windows_core::imp::define_interface!(
    IDiaEnumSectionContribs,
    IDiaEnumSectionContribs_Vtbl,
    0x1994deb2_2c82_4b1d_a57f_aff424d54a68
);
windows_core::imp::interface_hierarchy!(IDiaEnumSectionContribs, windows_core::IUnknown);
impl IDiaEnumSectionContribs {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Item(&self, index: u32) -> windows_core::Result<IDiaSectionContrib> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaSectionContrib>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumSectionContribs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumSectionContribs_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: u32) -> windows_core::Result<IDiaSectionContrib>;
    fn Next(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaSectionContrib>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumSectionContribs>;
}
impl IDiaEnumSectionContribs_Vtbl {
    pub const fn new<Identity: IDiaEnumSectionContribs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<
            Identity: IDiaEnumSectionContribs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSectionContribs_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<
            Identity: IDiaEnumSectionContribs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSectionContribs_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<
            Identity: IDiaEnumSectionContribs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            section: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSectionContribs_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        section.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<
            Identity: IDiaEnumSectionContribs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSectionContribs_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<
            Identity: IDiaEnumSectionContribs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSectionContribs_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<
            Identity: IDiaEnumSectionContribs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSectionContribs_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<
            Identity: IDiaEnumSectionContribs_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSectionContribs_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumSectionContribs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumSectionContribs {}
windows_core::imp::define_interface!(
    IDiaEnumSegments,
    IDiaEnumSegments_Vtbl,
    0xe8368ca9_01d1_419d_ac0c_e31235dbda9f
);
windows_core::imp::interface_hierarchy!(IDiaEnumSegments, windows_core::IUnknown);
impl IDiaEnumSegments {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Item(&self, index: u32) -> windows_core::Result<IDiaSegment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaSegment>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumSegments_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumSegments_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: u32) -> windows_core::Result<IDiaSegment>;
    fn Next(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaSegment>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumSegments>;
}
impl IDiaEnumSegments_Vtbl {
    pub const fn new<Identity: IDiaEnumSegments_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IDiaEnumSegments_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSegments_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IDiaEnumSegments_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSegments_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IDiaEnumSegments_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            index: u32,
            segment: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSegments_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        segment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IDiaEnumSegments_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSegments_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IDiaEnumSegments_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSegments_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IDiaEnumSegments_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSegments_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IDiaEnumSegments_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSegments_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumSegments as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumSegments {}
windows_core::imp::define_interface!(
    IDiaEnumSourceFiles,
    IDiaEnumSourceFiles_Vtbl,
    0x10f3dbd9_664f_4469_b808_9471c7a50538
);
windows_core::imp::interface_hierarchy!(IDiaEnumSourceFiles, windows_core::IUnknown);
impl IDiaEnumSourceFiles {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Item(&self, index: u32) -> windows_core::Result<IDiaSourceFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaSourceFile>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumSourceFiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumSourceFiles_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: u32) -> windows_core::Result<IDiaSourceFile>;
    fn Next(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaSourceFile>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumSourceFiles>;
}
impl IDiaEnumSourceFiles_Vtbl {
    pub const fn new<Identity: IDiaEnumSourceFiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<
            Identity: IDiaEnumSourceFiles_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSourceFiles_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSourceFiles_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            index: u32,
            sourcefile: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSourceFiles_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        sourcefile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSourceFiles_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSourceFiles_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSourceFiles_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IDiaEnumSourceFiles_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSourceFiles_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumSourceFiles as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumSourceFiles {}
windows_core::imp::define_interface!(
    IDiaEnumSourceLink,
    IDiaEnumSourceLink_Vtbl,
    0x45cd1eb3_5c6c_43e3_b20a_a4d8035de4e2
);
windows_core::imp::interface_hierarchy!(IDiaEnumSourceLink, windows_core::IUnknown);
impl IDiaEnumSourceLink {
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SizeOfNext(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeOfNext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Next(&self, cb: u32, pcb: *mut u32, pb: *mut u8) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                cb,
                pcb as _,
                pb as _,
            )
        }
    }
    pub unsafe fn Skip(&self, cnt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cnt)
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumSourceLink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SizeOfNext:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumSourceLink_Impl: windows_core::IUnknownImpl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn SizeOfNext(&self) -> windows_core::Result<u32>;
    fn Next(&self, cb: u32, pcb: *mut u32, pb: *mut u8) -> windows_core::Result<()>;
    fn Skip(&self, cnt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumSourceLink>;
}
impl IDiaEnumSourceLink_Vtbl {
    pub const fn new<Identity: IDiaEnumSourceLink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IDiaEnumSourceLink_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pcnt: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSourceLink_Impl::Count(this) {
                    Ok(ok__) => {
                        pcnt.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SizeOfNext<
            Identity: IDiaEnumSourceLink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcb: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSourceLink_Impl::SizeOfNext(this) {
                    Ok(ok__) => {
                        pcb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IDiaEnumSourceLink_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            cb: u32,
            pcb: *mut u32,
            pb: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSourceLink_Impl::Next(
                    this,
                    core::mem::transmute_copy(&cb),
                    core::mem::transmute_copy(&pcb),
                    core::mem::transmute_copy(&pb),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IDiaEnumSourceLink_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            cnt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSourceLink_Impl::Skip(this, core::mem::transmute_copy(&cnt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IDiaEnumSourceLink_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSourceLink_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IDiaEnumSourceLink_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSourceLink_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            SizeOfNext: SizeOfNext::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumSourceLink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumSourceLink {}
windows_core::imp::define_interface!(
    IDiaEnumSourceLink2,
    IDiaEnumSourceLink2_Vtbl,
    0x136d8151_ade7_4704_af13_324080762e8f
);
impl core::ops::Deref for IDiaEnumSourceLink2 {
    type Target = IDiaEnumSourceLink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaEnumSourceLink2,
    windows_core::IUnknown,
    IDiaEnumSourceLink
);
impl IDiaEnumSourceLink2 {
    pub unsafe fn SizeOfNext2(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeOfNext2)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Next2(&self, cb: u64, pcb: *mut u64, pb: *mut u8) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next2)(
                windows_core::Interface::as_raw(self),
                cb,
                pcb as _,
                pb as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDiaEnumSourceLink2_Vtbl {
    pub base__: IDiaEnumSourceLink_Vtbl,
    pub SizeOfNext2:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Next2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut u64,
        *mut u8,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumSourceLink2_Impl: IDiaEnumSourceLink_Impl {
    fn SizeOfNext2(&self) -> windows_core::Result<u64>;
    fn Next2(&self, cb: u64, pcb: *mut u64, pb: *mut u8) -> windows_core::Result<()>;
}
impl IDiaEnumSourceLink2_Vtbl {
    pub const fn new<Identity: IDiaEnumSourceLink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SizeOfNext2<
            Identity: IDiaEnumSourceLink2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcb: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSourceLink2_Impl::SizeOfNext2(this) {
                    Ok(ok__) => {
                        pcb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next2<Identity: IDiaEnumSourceLink2_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            cb: u64,
            pcb: *mut u64,
            pb: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSourceLink2_Impl::Next2(
                    this,
                    core::mem::transmute_copy(&cb),
                    core::mem::transmute_copy(&pcb),
                    core::mem::transmute_copy(&pb),
                )
                .into()
            }
        }
        Self {
            base__: IDiaEnumSourceLink_Vtbl::new::<Identity, OFFSET>(),
            SizeOfNext2: SizeOfNext2::<Identity, OFFSET>,
            Next2: Next2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumSourceLink2 as windows_core::Interface>::IID
            || iid == &<IDiaEnumSourceLink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumSourceLink2 {}
windows_core::imp::define_interface!(
    IDiaEnumStackFrames,
    IDiaEnumStackFrames_Vtbl,
    0xec9d461d_ce74_4711_a020_7d8f9a1dd255
);
windows_core::imp::interface_hierarchy!(IDiaEnumStackFrames, windows_core::IUnknown);
impl IDiaEnumStackFrames {
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaStackFrame>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumStackFrames_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDiaEnumStackFrames_Impl: windows_core::IUnknownImpl {
    fn Next(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaStackFrame>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl IDiaEnumStackFrames_Vtbl {
    pub const fn new<Identity: IDiaEnumStackFrames_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IDiaEnumStackFrames_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumStackFrames_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IDiaEnumStackFrames_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumStackFrames_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumStackFrames as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumStackFrames {}
windows_core::imp::define_interface!(
    IDiaEnumSymbols,
    IDiaEnumSymbols_Vtbl,
    0xcab72c48_443b_48f5_9b0b_42f0820ab29a
);
windows_core::imp::interface_hierarchy!(IDiaEnumSymbols, windows_core::IUnknown);
impl IDiaEnumSymbols {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Item(&self, index: u32) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaSymbol>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumSymbols_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumSymbols_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: u32) -> windows_core::Result<IDiaSymbol>;
    fn Next(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaSymbol>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumSymbols>;
}
impl IDiaEnumSymbols_Vtbl {
    pub const fn new<Identity: IDiaEnumSymbols_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IDiaEnumSymbols_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbols_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IDiaEnumSymbols_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbols_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IDiaEnumSymbols_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            index: u32,
            symbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbols_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        symbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IDiaEnumSymbols_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSymbols_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IDiaEnumSymbols_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSymbols_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IDiaEnumSymbols_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSymbols_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IDiaEnumSymbols_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbols_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumSymbols as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumSymbols {}
windows_core::imp::define_interface!(
    IDiaEnumSymbolsByAddr,
    IDiaEnumSymbolsByAddr_Vtbl,
    0x624b7d9c_24ea_4421_9d06_3b577471c1fa
);
windows_core::imp::interface_hierarchy!(IDiaEnumSymbolsByAddr, windows_core::IUnknown);
impl IDiaEnumSymbolsByAddr {
    pub unsafe fn symbolByAddr(&self, isect: u32, offset: u32) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symbolByAddr)(
                windows_core::Interface::as_raw(self),
                isect,
                offset,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn symbolByRVA(
        &self,
        relativevirtualaddress: u32,
    ) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symbolByRVA)(
                windows_core::Interface::as_raw(self),
                relativevirtualaddress,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn symbolByVA(&self, virtualaddress: u64) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symbolByVA)(
                windows_core::Interface::as_raw(self),
                virtualaddress,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaSymbol>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Prev(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaSymbol>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Prev)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumSymbolsByAddr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub symbolByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub symbolByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub symbolByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Prev: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumSymbolsByAddr_Impl: windows_core::IUnknownImpl {
    fn symbolByAddr(&self, isect: u32, offset: u32) -> windows_core::Result<IDiaSymbol>;
    fn symbolByRVA(&self, relativevirtualaddress: u32) -> windows_core::Result<IDiaSymbol>;
    fn symbolByVA(&self, virtualaddress: u64) -> windows_core::Result<IDiaSymbol>;
    fn Next(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaSymbol>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Prev(
        &self,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaSymbol>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IDiaEnumSymbolsByAddr>;
}
impl IDiaEnumSymbolsByAddr_Vtbl {
    pub const fn new<Identity: IDiaEnumSymbolsByAddr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn symbolByAddr<
            Identity: IDiaEnumSymbolsByAddr_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            isect: u32,
            offset: u32,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbolsByAddr_Impl::symbolByAddr(
                    this,
                    core::mem::transmute_copy(&isect),
                    core::mem::transmute_copy(&offset),
                ) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn symbolByRVA<
            Identity: IDiaEnumSymbolsByAddr_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            relativevirtualaddress: u32,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbolsByAddr_Impl::symbolByRVA(
                    this,
                    core::mem::transmute_copy(&relativevirtualaddress),
                ) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn symbolByVA<
            Identity: IDiaEnumSymbolsByAddr_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            virtualaddress: u64,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbolsByAddr_Impl::symbolByVA(
                    this,
                    core::mem::transmute_copy(&virtualaddress),
                ) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<
            Identity: IDiaEnumSymbolsByAddr_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSymbolsByAddr_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Prev<
            Identity: IDiaEnumSymbolsByAddr_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSymbolsByAddr_Impl::Prev(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Clone<
            Identity: IDiaEnumSymbolsByAddr_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbolsByAddr_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            symbolByAddr: symbolByAddr::<Identity, OFFSET>,
            symbolByRVA: symbolByRVA::<Identity, OFFSET>,
            symbolByVA: symbolByVA::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Prev: Prev::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumSymbolsByAddr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumSymbolsByAddr {}
windows_core::imp::define_interface!(
    IDiaEnumSymbolsByAddr2,
    IDiaEnumSymbolsByAddr2_Vtbl,
    0x1e45bd02_be45_4d71_ba32_0e576cfcd59f
);
impl core::ops::Deref for IDiaEnumSymbolsByAddr2 {
    type Target = IDiaEnumSymbolsByAddr;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaEnumSymbolsByAddr2,
    windows_core::IUnknown,
    IDiaEnumSymbolsByAddr
);
impl IDiaEnumSymbolsByAddr2 {
    pub unsafe fn symbolByAddrEx(
        &self,
        fpromoteblocksym: bool,
        isect: u32,
        offset: u32,
    ) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symbolByAddrEx)(
                windows_core::Interface::as_raw(self),
                fpromoteblocksym.into(),
                isect,
                offset,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn symbolByRVAEx(
        &self,
        fpromoteblocksym: bool,
        relativevirtualaddress: u32,
    ) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symbolByRVAEx)(
                windows_core::Interface::as_raw(self),
                fpromoteblocksym.into(),
                relativevirtualaddress,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn symbolByVAEx(
        &self,
        fpromoteblocksym: bool,
        virtualaddress: u64,
    ) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symbolByVAEx)(
                windows_core::Interface::as_raw(self),
                fpromoteblocksym.into(),
                virtualaddress,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn NextEx(
        &self,
        fpromoteblocksym: bool,
        celt: u32,
        rgelt: *mut Option<IDiaSymbol>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).NextEx)(
                windows_core::Interface::as_raw(self),
                fpromoteblocksym.into(),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn PrevEx(
        &self,
        fpromoteblocksym: bool,
        celt: u32,
        rgelt: *mut Option<IDiaSymbol>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).PrevEx)(
                windows_core::Interface::as_raw(self),
                fpromoteblocksym.into(),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDiaEnumSymbolsByAddr2_Vtbl {
    pub base__: IDiaEnumSymbolsByAddr_Vtbl,
    pub symbolByAddrEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub symbolByRVAEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub symbolByVAEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NextEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub PrevEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
}
pub trait IDiaEnumSymbolsByAddr2_Impl: IDiaEnumSymbolsByAddr_Impl {
    fn symbolByAddrEx(
        &self,
        fpromoteblocksym: windows_core::BOOL,
        isect: u32,
        offset: u32,
    ) -> windows_core::Result<IDiaSymbol>;
    fn symbolByRVAEx(
        &self,
        fpromoteblocksym: windows_core::BOOL,
        relativevirtualaddress: u32,
    ) -> windows_core::Result<IDiaSymbol>;
    fn symbolByVAEx(
        &self,
        fpromoteblocksym: windows_core::BOOL,
        virtualaddress: u64,
    ) -> windows_core::Result<IDiaSymbol>;
    fn NextEx(
        &self,
        fpromoteblocksym: windows_core::BOOL,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaSymbol>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn PrevEx(
        &self,
        fpromoteblocksym: windows_core::BOOL,
        celt: u32,
        rgelt: windows_core::OutRef<IDiaSymbol>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
}
impl IDiaEnumSymbolsByAddr2_Vtbl {
    pub const fn new<Identity: IDiaEnumSymbolsByAddr2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn symbolByAddrEx<
            Identity: IDiaEnumSymbolsByAddr2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fpromoteblocksym: windows_core::BOOL,
            isect: u32,
            offset: u32,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbolsByAddr2_Impl::symbolByAddrEx(
                    this,
                    core::mem::transmute_copy(&fpromoteblocksym),
                    core::mem::transmute_copy(&isect),
                    core::mem::transmute_copy(&offset),
                ) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn symbolByRVAEx<
            Identity: IDiaEnumSymbolsByAddr2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fpromoteblocksym: windows_core::BOOL,
            relativevirtualaddress: u32,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbolsByAddr2_Impl::symbolByRVAEx(
                    this,
                    core::mem::transmute_copy(&fpromoteblocksym),
                    core::mem::transmute_copy(&relativevirtualaddress),
                ) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn symbolByVAEx<
            Identity: IDiaEnumSymbolsByAddr2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fpromoteblocksym: windows_core::BOOL,
            virtualaddress: u64,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaEnumSymbolsByAddr2_Impl::symbolByVAEx(
                    this,
                    core::mem::transmute_copy(&fpromoteblocksym),
                    core::mem::transmute_copy(&virtualaddress),
                ) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextEx<
            Identity: IDiaEnumSymbolsByAddr2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fpromoteblocksym: windows_core::BOOL,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSymbolsByAddr2_Impl::NextEx(
                    this,
                    core::mem::transmute_copy(&fpromoteblocksym),
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn PrevEx<
            Identity: IDiaEnumSymbolsByAddr2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fpromoteblocksym: windows_core::BOOL,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaEnumSymbolsByAddr2_Impl::PrevEx(
                    this,
                    core::mem::transmute_copy(&fpromoteblocksym),
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        Self {
            base__: IDiaEnumSymbolsByAddr_Vtbl::new::<Identity, OFFSET>(),
            symbolByAddrEx: symbolByAddrEx::<Identity, OFFSET>,
            symbolByRVAEx: symbolByRVAEx::<Identity, OFFSET>,
            symbolByVAEx: symbolByVAEx::<Identity, OFFSET>,
            NextEx: NextEx::<Identity, OFFSET>,
            PrevEx: PrevEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaEnumSymbolsByAddr2 as windows_core::Interface>::IID
            || iid == &<IDiaEnumSymbolsByAddr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaEnumSymbolsByAddr2 {}
windows_core::imp::define_interface!(
    IDiaEnumTables,
    IDiaEnumTables_Vtbl,
    0xc65c2b0a_1150_4d7a_afcc_e05bf3dee81e
);
windows_core::imp::interface_hierarchy!(IDiaEnumTables, windows_core::IUnknown);
impl IDiaEnumTables {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<IDiaTable>,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaEnumTables_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    Item: usize,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaEnumTables {}
windows_core::imp::define_interface!(
    IDiaFrameData,
    IDiaFrameData_Vtbl,
    0xa39184b7_6a36_42de_8eec_7df9f3f59f33
);
windows_core::imp::interface_hierarchy!(IDiaFrameData, windows_core::IUnknown);
impl IDiaFrameData {
    pub unsafe fn addressSection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressSection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn addressOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn relativeVirtualAddress(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).relativeVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lengthBlock(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lengthBlock)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lengthLocals(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lengthLocals)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lengthParams(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lengthParams)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn maxStack(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).maxStack)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lengthProlog(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lengthProlog)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lengthSavedRegisters(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lengthSavedRegisters)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn program(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).program)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn systemExceptionHandling(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).systemExceptionHandling)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn cplusplusExceptionHandling(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).cplusplusExceptionHandling)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn functionStart(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).functionStart)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn allocatesBasePointer(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).allocatesBasePointer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn r#type(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#type)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn functionParent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).functionParent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn execute<P0>(&self, frame: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiaStackWalkFrame>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).execute)(
                windows_core::Interface::as_raw(self),
                frame.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct IDiaFrameData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub addressSection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub addressOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub relativeVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub virtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub lengthBlock:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub lengthLocals:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub lengthParams:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub maxStack:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub lengthProlog:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub lengthSavedRegisters:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub program: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub systemExceptionHandling: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub cplusplusExceptionHandling: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub functionStart: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub allocatesBasePointer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub r#type:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub functionParent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub execute: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaFrameData_Impl: windows_core::IUnknownImpl {
    fn addressSection(&self) -> windows_core::Result<u32>;
    fn addressOffset(&self) -> windows_core::Result<u32>;
    fn relativeVirtualAddress(&self) -> windows_core::Result<u32>;
    fn virtualAddress(&self) -> windows_core::Result<u64>;
    fn lengthBlock(&self) -> windows_core::Result<u32>;
    fn lengthLocals(&self) -> windows_core::Result<u32>;
    fn lengthParams(&self) -> windows_core::Result<u32>;
    fn maxStack(&self) -> windows_core::Result<u32>;
    fn lengthProlog(&self) -> windows_core::Result<u32>;
    fn lengthSavedRegisters(&self) -> windows_core::Result<u32>;
    fn program(&self) -> windows_core::Result<windows_core::BSTR>;
    fn systemExceptionHandling(&self) -> windows_core::Result<windows_core::BOOL>;
    fn cplusplusExceptionHandling(&self) -> windows_core::Result<windows_core::BOOL>;
    fn functionStart(&self) -> windows_core::Result<windows_core::BOOL>;
    fn allocatesBasePointer(&self) -> windows_core::Result<windows_core::BOOL>;
    fn r#type(&self) -> windows_core::Result<u32>;
    fn functionParent(&self) -> windows_core::Result<IDiaFrameData>;
    fn execute(&self, frame: windows_core::Ref<IDiaStackWalkFrame>) -> windows_core::Result<()>;
}
impl IDiaFrameData_Vtbl {
    pub const fn new<Identity: IDiaFrameData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn addressSection<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::addressSection(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn addressOffset<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::addressOffset(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn relativeVirtualAddress<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::relativeVirtualAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn virtualAddress<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::virtualAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lengthBlock<Identity: IDiaFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::lengthBlock(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lengthLocals<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::lengthLocals(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lengthParams<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::lengthParams(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn maxStack<Identity: IDiaFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::maxStack(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lengthProlog<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::lengthProlog(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lengthSavedRegisters<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::lengthSavedRegisters(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn program<Identity: IDiaFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::program(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn systemExceptionHandling<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::systemExceptionHandling(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn cplusplusExceptionHandling<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::cplusplusExceptionHandling(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn functionStart<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::functionStart(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn allocatesBasePointer<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::allocatesBasePointer(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn r#type<Identity: IDiaFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::r#type(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn functionParent<
            Identity: IDiaFrameData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaFrameData_Impl::functionParent(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn execute<Identity: IDiaFrameData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            frame: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaFrameData_Impl::execute(this, core::mem::transmute_copy(&frame)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            addressSection: addressSection::<Identity, OFFSET>,
            addressOffset: addressOffset::<Identity, OFFSET>,
            relativeVirtualAddress: relativeVirtualAddress::<Identity, OFFSET>,
            virtualAddress: virtualAddress::<Identity, OFFSET>,
            lengthBlock: lengthBlock::<Identity, OFFSET>,
            lengthLocals: lengthLocals::<Identity, OFFSET>,
            lengthParams: lengthParams::<Identity, OFFSET>,
            maxStack: maxStack::<Identity, OFFSET>,
            lengthProlog: lengthProlog::<Identity, OFFSET>,
            lengthSavedRegisters: lengthSavedRegisters::<Identity, OFFSET>,
            program: program::<Identity, OFFSET>,
            systemExceptionHandling: systemExceptionHandling::<Identity, OFFSET>,
            cplusplusExceptionHandling: cplusplusExceptionHandling::<Identity, OFFSET>,
            functionStart: functionStart::<Identity, OFFSET>,
            allocatesBasePointer: allocatesBasePointer::<Identity, OFFSET>,
            r#type: r#type::<Identity, OFFSET>,
            functionParent: functionParent::<Identity, OFFSET>,
            execute: execute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaFrameData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaFrameData {}
windows_core::imp::define_interface!(
    IDiaImageData,
    IDiaImageData_Vtbl,
    0xc8e40ed2_a1d9_4221_8692_3ce661184b44
);
windows_core::imp::interface_hierarchy!(IDiaImageData, windows_core::IUnknown);
impl IDiaImageData {
    pub unsafe fn relativeVirtualAddress(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).relativeVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn imageBase(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).imageBase)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaImageData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub relativeVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub virtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub imageBase:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IDiaImageData_Impl: windows_core::IUnknownImpl {
    fn relativeVirtualAddress(&self) -> windows_core::Result<u32>;
    fn virtualAddress(&self) -> windows_core::Result<u64>;
    fn imageBase(&self) -> windows_core::Result<u64>;
}
impl IDiaImageData_Vtbl {
    pub const fn new<Identity: IDiaImageData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn relativeVirtualAddress<
            Identity: IDiaImageData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaImageData_Impl::relativeVirtualAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn virtualAddress<
            Identity: IDiaImageData_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaImageData_Impl::virtualAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn imageBase<Identity: IDiaImageData_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaImageData_Impl::imageBase(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            relativeVirtualAddress: relativeVirtualAddress::<Identity, OFFSET>,
            virtualAddress: virtualAddress::<Identity, OFFSET>,
            imageBase: imageBase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaImageData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaImageData {}
windows_core::imp::define_interface!(
    IDiaInjectedSource,
    IDiaInjectedSource_Vtbl,
    0xae605cdc_8105_4a23_b710_3259f1e26112
);
windows_core::imp::interface_hierarchy!(IDiaInjectedSource, windows_core::IUnknown);
impl IDiaInjectedSource {
    pub unsafe fn crc(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).crc)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn length(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn filename(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).filename)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn objectFilename(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).objectFilename)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn virtualFilename(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualFilename)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn sourceCompression(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).sourceCompression)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn get_source(
        &self,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_source)(
                windows_core::Interface::as_raw(self),
                cbdata,
                pcbdata as _,
                pbdata as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDiaInjectedSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub crc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub length:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub filename: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub objectFilename: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub virtualFilename: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub sourceCompression:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub get_source: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
}
pub trait IDiaInjectedSource_Impl: windows_core::IUnknownImpl {
    fn crc(&self) -> windows_core::Result<u32>;
    fn length(&self) -> windows_core::Result<u64>;
    fn filename(&self) -> windows_core::Result<windows_core::BSTR>;
    fn objectFilename(&self) -> windows_core::Result<windows_core::BSTR>;
    fn virtualFilename(&self) -> windows_core::Result<windows_core::BSTR>;
    fn sourceCompression(&self) -> windows_core::Result<u32>;
    fn get_source(
        &self,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::Result<()>;
}
impl IDiaInjectedSource_Vtbl {
    pub const fn new<Identity: IDiaInjectedSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn crc<Identity: IDiaInjectedSource_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInjectedSource_Impl::crc(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: IDiaInjectedSource_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInjectedSource_Impl::length(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn filename<
            Identity: IDiaInjectedSource_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInjectedSource_Impl::filename(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn objectFilename<
            Identity: IDiaInjectedSource_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInjectedSource_Impl::objectFilename(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn virtualFilename<
            Identity: IDiaInjectedSource_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInjectedSource_Impl::virtualFilename(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn sourceCompression<
            Identity: IDiaInjectedSource_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInjectedSource_Impl::sourceCompression(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_source<
            Identity: IDiaInjectedSource_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cbdata: u32,
            pcbdata: *mut u32,
            pbdata: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaInjectedSource_Impl::get_source(
                    this,
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pcbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            crc: crc::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            filename: filename::<Identity, OFFSET>,
            objectFilename: objectFilename::<Identity, OFFSET>,
            virtualFilename: virtualFilename::<Identity, OFFSET>,
            sourceCompression: sourceCompression::<Identity, OFFSET>,
            get_source: get_source::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaInjectedSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaInjectedSource {}
windows_core::imp::define_interface!(
    IDiaInputAssemblyFile,
    IDiaInputAssemblyFile_Vtbl,
    0x3bfe56b0_390c_4863_9430_1f3d083b7684
);
windows_core::imp::interface_hierarchy!(IDiaInputAssemblyFile, windows_core::IUnknown);
impl IDiaInputAssemblyFile {
    pub unsafe fn uniqueId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).uniqueId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn index(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).index)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn timestamp(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).timestamp)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn pdbAvailableAtILMerge(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).pdbAvailableAtILMerge)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn fileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fileName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_version(
        &self,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_version)(
                windows_core::Interface::as_raw(self),
                cbdata,
                pcbdata as _,
                pbdata as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDiaInputAssemblyFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub uniqueId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub timestamp:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub pdbAvailableAtILMerge: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub fileName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_version: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
}
pub trait IDiaInputAssemblyFile_Impl: windows_core::IUnknownImpl {
    fn uniqueId(&self) -> windows_core::Result<u32>;
    fn index(&self) -> windows_core::Result<u32>;
    fn timestamp(&self) -> windows_core::Result<u32>;
    fn pdbAvailableAtILMerge(&self) -> windows_core::Result<windows_core::BOOL>;
    fn fileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_version(
        &self,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::Result<()>;
}
impl IDiaInputAssemblyFile_Vtbl {
    pub const fn new<Identity: IDiaInputAssemblyFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn uniqueId<
            Identity: IDiaInputAssemblyFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInputAssemblyFile_Impl::uniqueId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn index<
            Identity: IDiaInputAssemblyFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInputAssemblyFile_Impl::index(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn timestamp<
            Identity: IDiaInputAssemblyFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInputAssemblyFile_Impl::timestamp(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn pdbAvailableAtILMerge<
            Identity: IDiaInputAssemblyFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInputAssemblyFile_Impl::pdbAvailableAtILMerge(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fileName<
            Identity: IDiaInputAssemblyFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaInputAssemblyFile_Impl::fileName(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_version<
            Identity: IDiaInputAssemblyFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cbdata: u32,
            pcbdata: *mut u32,
            pbdata: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaInputAssemblyFile_Impl::get_version(
                    this,
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pcbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            uniqueId: uniqueId::<Identity, OFFSET>,
            index: index::<Identity, OFFSET>,
            timestamp: timestamp::<Identity, OFFSET>,
            pdbAvailableAtILMerge: pdbAvailableAtILMerge::<Identity, OFFSET>,
            fileName: fileName::<Identity, OFFSET>,
            get_version: get_version::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaInputAssemblyFile as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaInputAssemblyFile {}
windows_core::imp::define_interface!(
    IDiaLineNumber,
    IDiaLineNumber_Vtbl,
    0xb388eb14_be4d_421d_a8a1_6cf7ab057086
);
windows_core::imp::interface_hierarchy!(IDiaLineNumber, windows_core::IUnknown);
impl IDiaLineNumber {
    pub unsafe fn compiland(&self) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).compiland)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn sourceFile(&self) -> windows_core::Result<IDiaSourceFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).sourceFile)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn lineNumber(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lineNumber)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lineNumberEnd(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lineNumberEnd)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn columnNumber(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).columnNumber)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn columnNumberEnd(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).columnNumberEnd)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn addressSection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressSection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn addressOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn relativeVirtualAddress(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).relativeVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn length(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn sourceFileId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).sourceFileId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn statement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).statement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn compilandId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).compilandId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaLineNumber_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub compiland: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub sourceFile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub lineNumber:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub lineNumberEnd:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub columnNumber:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub columnNumberEnd:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub addressSection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub addressOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub relativeVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub virtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub length:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub sourceFileId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub statement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub compilandId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDiaLineNumber_Impl: windows_core::IUnknownImpl {
    fn compiland(&self) -> windows_core::Result<IDiaSymbol>;
    fn sourceFile(&self) -> windows_core::Result<IDiaSourceFile>;
    fn lineNumber(&self) -> windows_core::Result<u32>;
    fn lineNumberEnd(&self) -> windows_core::Result<u32>;
    fn columnNumber(&self) -> windows_core::Result<u32>;
    fn columnNumberEnd(&self) -> windows_core::Result<u32>;
    fn addressSection(&self) -> windows_core::Result<u32>;
    fn addressOffset(&self) -> windows_core::Result<u32>;
    fn relativeVirtualAddress(&self) -> windows_core::Result<u32>;
    fn virtualAddress(&self) -> windows_core::Result<u64>;
    fn length(&self) -> windows_core::Result<u32>;
    fn sourceFileId(&self) -> windows_core::Result<u32>;
    fn statement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn compilandId(&self) -> windows_core::Result<u32>;
}
impl IDiaLineNumber_Vtbl {
    pub const fn new<Identity: IDiaLineNumber_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn compiland<Identity: IDiaLineNumber_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::compiland(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn sourceFile<Identity: IDiaLineNumber_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::sourceFile(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lineNumber<Identity: IDiaLineNumber_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::lineNumber(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lineNumberEnd<
            Identity: IDiaLineNumber_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::lineNumberEnd(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn columnNumber<
            Identity: IDiaLineNumber_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::columnNumber(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn columnNumberEnd<
            Identity: IDiaLineNumber_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::columnNumberEnd(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn addressSection<
            Identity: IDiaLineNumber_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::addressSection(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn addressOffset<
            Identity: IDiaLineNumber_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::addressOffset(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn relativeVirtualAddress<
            Identity: IDiaLineNumber_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::relativeVirtualAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn virtualAddress<
            Identity: IDiaLineNumber_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::virtualAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: IDiaLineNumber_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::length(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn sourceFileId<
            Identity: IDiaLineNumber_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::sourceFileId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn statement<Identity: IDiaLineNumber_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::statement(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn compilandId<
            Identity: IDiaLineNumber_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaLineNumber_Impl::compilandId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            compiland: compiland::<Identity, OFFSET>,
            sourceFile: sourceFile::<Identity, OFFSET>,
            lineNumber: lineNumber::<Identity, OFFSET>,
            lineNumberEnd: lineNumberEnd::<Identity, OFFSET>,
            columnNumber: columnNumber::<Identity, OFFSET>,
            columnNumberEnd: columnNumberEnd::<Identity, OFFSET>,
            addressSection: addressSection::<Identity, OFFSET>,
            addressOffset: addressOffset::<Identity, OFFSET>,
            relativeVirtualAddress: relativeVirtualAddress::<Identity, OFFSET>,
            virtualAddress: virtualAddress::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            sourceFileId: sourceFileId::<Identity, OFFSET>,
            statement: statement::<Identity, OFFSET>,
            compilandId: compilandId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaLineNumber as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaLineNumber {}
windows_core::imp::define_interface!(
    IDiaLoadCallback,
    IDiaLoadCallback_Vtbl,
    0xc32adb82_73f4_421b_95d5_a4706edf5dbe
);
windows_core::imp::interface_hierarchy!(IDiaLoadCallback, windows_core::IUnknown);
impl IDiaLoadCallback {
    pub unsafe fn NotifyDebugDir(
        &self,
        fexecutable: bool,
        cbdata: u32,
        pbdata: *const u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).NotifyDebugDir)(
                windows_core::Interface::as_raw(self),
                fexecutable.into(),
                cbdata,
                pbdata,
            )
        }
    }
    pub unsafe fn NotifyOpenDBG<P0>(
        &self,
        dbgpath: P0,
        resultcode: windows_core::HRESULT,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).NotifyOpenDBG)(
                windows_core::Interface::as_raw(self),
                dbgpath.param().abi(),
                resultcode,
            )
        }
    }
    pub unsafe fn NotifyOpenPDB<P0>(
        &self,
        pdbpath: P0,
        resultcode: windows_core::HRESULT,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).NotifyOpenPDB)(
                windows_core::Interface::as_raw(self),
                pdbpath.param().abi(),
                resultcode,
            )
        }
    }
    pub unsafe fn RestrictRegistryAccess(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RestrictRegistryAccess)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn RestrictSymbolServerAccess(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RestrictSymbolServerAccess)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
}
#[repr(C)]
pub struct IDiaLoadCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifyDebugDir: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        u32,
        *const u8,
    ) -> windows_core::HRESULT,
    pub NotifyOpenDBG: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::HRESULT,
    ) -> windows_core::HRESULT,
    pub NotifyOpenPDB: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::HRESULT,
    ) -> windows_core::HRESULT,
    pub RestrictRegistryAccess:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestrictSymbolServerAccess:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDiaLoadCallback_Impl: windows_core::IUnknownImpl {
    fn NotifyDebugDir(
        &self,
        fexecutable: windows_core::BOOL,
        cbdata: u32,
        pbdata: *const u8,
    ) -> windows_core::Result<()>;
    fn NotifyOpenDBG(
        &self,
        dbgpath: &windows_core::PCWSTR,
        resultcode: windows_core::HRESULT,
    ) -> windows_core::Result<()>;
    fn NotifyOpenPDB(
        &self,
        pdbpath: &windows_core::PCWSTR,
        resultcode: windows_core::HRESULT,
    ) -> windows_core::Result<()>;
    fn RestrictRegistryAccess(&self) -> windows_core::Result<()>;
    fn RestrictSymbolServerAccess(&self) -> windows_core::Result<()>;
}
impl IDiaLoadCallback_Vtbl {
    pub const fn new<Identity: IDiaLoadCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyDebugDir<
            Identity: IDiaLoadCallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fexecutable: windows_core::BOOL,
            cbdata: u32,
            pbdata: *const u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaLoadCallback_Impl::NotifyDebugDir(
                    this,
                    core::mem::transmute_copy(&fexecutable),
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn NotifyOpenDBG<
            Identity: IDiaLoadCallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dbgpath: windows_core::PCWSTR,
            resultcode: windows_core::HRESULT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaLoadCallback_Impl::NotifyOpenDBG(
                    this,
                    core::mem::transmute(&dbgpath),
                    core::mem::transmute_copy(&resultcode),
                )
                .into()
            }
        }
        unsafe extern "system" fn NotifyOpenPDB<
            Identity: IDiaLoadCallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdbpath: windows_core::PCWSTR,
            resultcode: windows_core::HRESULT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaLoadCallback_Impl::NotifyOpenPDB(
                    this,
                    core::mem::transmute(&pdbpath),
                    core::mem::transmute_copy(&resultcode),
                )
                .into()
            }
        }
        unsafe extern "system" fn RestrictRegistryAccess<
            Identity: IDiaLoadCallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaLoadCallback_Impl::RestrictRegistryAccess(this).into()
            }
        }
        unsafe extern "system" fn RestrictSymbolServerAccess<
            Identity: IDiaLoadCallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaLoadCallback_Impl::RestrictSymbolServerAccess(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifyDebugDir: NotifyDebugDir::<Identity, OFFSET>,
            NotifyOpenDBG: NotifyOpenDBG::<Identity, OFFSET>,
            NotifyOpenPDB: NotifyOpenPDB::<Identity, OFFSET>,
            RestrictRegistryAccess: RestrictRegistryAccess::<Identity, OFFSET>,
            RestrictSymbolServerAccess: RestrictSymbolServerAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaLoadCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaLoadCallback {}
windows_core::imp::define_interface!(
    IDiaLoadCallback2,
    IDiaLoadCallback2_Vtbl,
    0x4688a074_5a4d_4486_aea8_7b90711d9f7c
);
impl core::ops::Deref for IDiaLoadCallback2 {
    type Target = IDiaLoadCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaLoadCallback2,
    windows_core::IUnknown,
    IDiaLoadCallback
);
impl IDiaLoadCallback2 {
    pub unsafe fn RestrictOriginalPathAccess(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RestrictOriginalPathAccess)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn RestrictReferencePathAccess(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RestrictReferencePathAccess)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn RestrictDBGAccess(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RestrictDBGAccess)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn RestrictSystemRootAccess(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RestrictSystemRootAccess)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
}
#[repr(C)]
pub struct IDiaLoadCallback2_Vtbl {
    pub base__: IDiaLoadCallback_Vtbl,
    pub RestrictOriginalPathAccess:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestrictReferencePathAccess:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestrictDBGAccess:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestrictSystemRootAccess:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDiaLoadCallback2_Impl: IDiaLoadCallback_Impl {
    fn RestrictOriginalPathAccess(&self) -> windows_core::Result<()>;
    fn RestrictReferencePathAccess(&self) -> windows_core::Result<()>;
    fn RestrictDBGAccess(&self) -> windows_core::Result<()>;
    fn RestrictSystemRootAccess(&self) -> windows_core::Result<()>;
}
impl IDiaLoadCallback2_Vtbl {
    pub const fn new<Identity: IDiaLoadCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RestrictOriginalPathAccess<
            Identity: IDiaLoadCallback2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaLoadCallback2_Impl::RestrictOriginalPathAccess(this).into()
            }
        }
        unsafe extern "system" fn RestrictReferencePathAccess<
            Identity: IDiaLoadCallback2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaLoadCallback2_Impl::RestrictReferencePathAccess(this).into()
            }
        }
        unsafe extern "system" fn RestrictDBGAccess<
            Identity: IDiaLoadCallback2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaLoadCallback2_Impl::RestrictDBGAccess(this).into()
            }
        }
        unsafe extern "system" fn RestrictSystemRootAccess<
            Identity: IDiaLoadCallback2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaLoadCallback2_Impl::RestrictSystemRootAccess(this).into()
            }
        }
        Self {
            base__: IDiaLoadCallback_Vtbl::new::<Identity, OFFSET>(),
            RestrictOriginalPathAccess: RestrictOriginalPathAccess::<Identity, OFFSET>,
            RestrictReferencePathAccess: RestrictReferencePathAccess::<Identity, OFFSET>,
            RestrictDBGAccess: RestrictDBGAccess::<Identity, OFFSET>,
            RestrictSystemRootAccess: RestrictSystemRootAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaLoadCallback2 as windows_core::Interface>::IID
            || iid == &<IDiaLoadCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaLoadCallback2 {}
windows_core::imp::define_interface!(
    IDiaPropertyStorage,
    IDiaPropertyStorage_Vtbl,
    0x9d416f9c_e184_45b2_a4f0_ce517f719e9b
);
windows_core::imp::interface_hierarchy!(IDiaPropertyStorage, windows_core::IUnknown);
#[repr(C)]
pub struct IDiaPropertyStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    ReadMultiple: usize,
    ReadPropertyNames: usize,
    Enum: usize,
    ReadDWORD: usize,
    ReadLONG: usize,
    ReadBOOL: usize,
    ReadULONGLONG: usize,
    ReadBSTR: usize,
}
impl windows_core::RuntimeName for IDiaPropertyStorage {}
windows_core::imp::define_interface!(
    IDiaReadExeAtOffsetCallback,
    IDiaReadExeAtOffsetCallback_Vtbl,
    0x587a461c_b80b_4f54_9194_5032589a6319
);
windows_core::imp::interface_hierarchy!(IDiaReadExeAtOffsetCallback, windows_core::IUnknown);
#[repr(C)]
pub struct IDiaReadExeAtOffsetCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    ReadExecutableAt: usize,
}
impl windows_core::RuntimeName for IDiaReadExeAtOffsetCallback {}
windows_core::imp::define_interface!(
    IDiaReadExeAtRVACallback,
    IDiaReadExeAtRVACallback_Vtbl,
    0x8e3f80ca_7517_432a_ba07_285134aaea8e
);
windows_core::imp::interface_hierarchy!(IDiaReadExeAtRVACallback, windows_core::IUnknown);
impl IDiaReadExeAtRVACallback {
    pub unsafe fn ReadExecutableAtRVA(
        &self,
        relativevirtualaddress: u32,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ReadExecutableAtRVA)(
                windows_core::Interface::as_raw(self),
                relativevirtualaddress,
                cbdata,
                pcbdata as _,
                pbdata as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDiaReadExeAtRVACallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReadExecutableAtRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
}
pub trait IDiaReadExeAtRVACallback_Impl: windows_core::IUnknownImpl {
    fn ReadExecutableAtRVA(
        &self,
        relativevirtualaddress: u32,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::Result<()>;
}
impl IDiaReadExeAtRVACallback_Vtbl {
    pub const fn new<Identity: IDiaReadExeAtRVACallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadExecutableAtRVA<
            Identity: IDiaReadExeAtRVACallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            relativevirtualaddress: u32,
            cbdata: u32,
            pcbdata: *mut u32,
            pbdata: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaReadExeAtRVACallback_Impl::ReadExecutableAtRVA(
                    this,
                    core::mem::transmute_copy(&relativevirtualaddress),
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pcbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadExecutableAtRVA: ReadExecutableAtRVA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaReadExeAtRVACallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaReadExeAtRVACallback {}
windows_core::imp::define_interface!(
    IDiaSectionContrib,
    IDiaSectionContrib_Vtbl,
    0x0cf4b60e_35b1_4c6c_bdd8_854b9c8e3857
);
windows_core::imp::interface_hierarchy!(IDiaSectionContrib, windows_core::IUnknown);
impl IDiaSectionContrib {
    pub unsafe fn compiland(&self) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).compiland)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn addressSection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressSection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn addressOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn relativeVirtualAddress(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).relativeVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn length(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn notPaged(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).notPaged)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn code(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).code)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn initializedData(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).initializedData)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn uninitializedData(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).uninitializedData)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn remove(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).remove)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn comdat(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).comdat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn discardable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).discardable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn notCached(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).notCached)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn share(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).share)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn execute(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).execute)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn read(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).read)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn write(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).write)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn dataCrc(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).dataCrc)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn relocationsCrc(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).relocationsCrc)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn compilandId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).compilandId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn code16bit(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).code16bit)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSectionContrib_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub compiland: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub addressSection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub addressOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub relativeVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub virtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub length:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub notPaged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub code: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub initializedData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub uninitializedData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub remove: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub comdat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub discardable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub notCached: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub share: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub execute: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub read: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub write: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub dataCrc:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub relocationsCrc:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub compilandId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub code16bit: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
pub trait IDiaSectionContrib_Impl: windows_core::IUnknownImpl {
    fn compiland(&self) -> windows_core::Result<IDiaSymbol>;
    fn addressSection(&self) -> windows_core::Result<u32>;
    fn addressOffset(&self) -> windows_core::Result<u32>;
    fn relativeVirtualAddress(&self) -> windows_core::Result<u32>;
    fn virtualAddress(&self) -> windows_core::Result<u64>;
    fn length(&self) -> windows_core::Result<u32>;
    fn notPaged(&self) -> windows_core::Result<windows_core::BOOL>;
    fn code(&self) -> windows_core::Result<windows_core::BOOL>;
    fn initializedData(&self) -> windows_core::Result<windows_core::BOOL>;
    fn uninitializedData(&self) -> windows_core::Result<windows_core::BOOL>;
    fn remove(&self) -> windows_core::Result<windows_core::BOOL>;
    fn comdat(&self) -> windows_core::Result<windows_core::BOOL>;
    fn discardable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn notCached(&self) -> windows_core::Result<windows_core::BOOL>;
    fn share(&self) -> windows_core::Result<windows_core::BOOL>;
    fn execute(&self) -> windows_core::Result<windows_core::BOOL>;
    fn read(&self) -> windows_core::Result<windows_core::BOOL>;
    fn write(&self) -> windows_core::Result<windows_core::BOOL>;
    fn dataCrc(&self) -> windows_core::Result<u32>;
    fn relocationsCrc(&self) -> windows_core::Result<u32>;
    fn compilandId(&self) -> windows_core::Result<u32>;
    fn code16bit(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IDiaSectionContrib_Vtbl {
    pub const fn new<Identity: IDiaSectionContrib_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn compiland<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::compiland(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn addressSection<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::addressSection(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn addressOffset<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::addressOffset(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn relativeVirtualAddress<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::relativeVirtualAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn virtualAddress<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::virtualAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: IDiaSectionContrib_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::length(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn notPaged<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::notPaged(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn code<Identity: IDiaSectionContrib_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::code(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn initializedData<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::initializedData(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn uninitializedData<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::uninitializedData(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn remove<Identity: IDiaSectionContrib_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::remove(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn comdat<Identity: IDiaSectionContrib_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::comdat(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn discardable<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::discardable(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn notCached<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::notCached(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn share<Identity: IDiaSectionContrib_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::share(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn execute<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::execute(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn read<Identity: IDiaSectionContrib_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::read(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn write<Identity: IDiaSectionContrib_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::write(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn dataCrc<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::dataCrc(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn relocationsCrc<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::relocationsCrc(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn compilandId<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::compilandId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn code16bit<
            Identity: IDiaSectionContrib_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSectionContrib_Impl::code16bit(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            compiland: compiland::<Identity, OFFSET>,
            addressSection: addressSection::<Identity, OFFSET>,
            addressOffset: addressOffset::<Identity, OFFSET>,
            relativeVirtualAddress: relativeVirtualAddress::<Identity, OFFSET>,
            virtualAddress: virtualAddress::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            notPaged: notPaged::<Identity, OFFSET>,
            code: code::<Identity, OFFSET>,
            initializedData: initializedData::<Identity, OFFSET>,
            uninitializedData: uninitializedData::<Identity, OFFSET>,
            remove: remove::<Identity, OFFSET>,
            comdat: comdat::<Identity, OFFSET>,
            discardable: discardable::<Identity, OFFSET>,
            notCached: notCached::<Identity, OFFSET>,
            share: share::<Identity, OFFSET>,
            execute: execute::<Identity, OFFSET>,
            read: read::<Identity, OFFSET>,
            write: write::<Identity, OFFSET>,
            dataCrc: dataCrc::<Identity, OFFSET>,
            relocationsCrc: relocationsCrc::<Identity, OFFSET>,
            compilandId: compilandId::<Identity, OFFSET>,
            code16bit: code16bit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaSectionContrib as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaSectionContrib {}
windows_core::imp::define_interface!(
    IDiaSegment,
    IDiaSegment_Vtbl,
    0x0775b784_c75b_4449_848b_b7bd3159545b
);
windows_core::imp::interface_hierarchy!(IDiaSegment, windows_core::IUnknown);
impl IDiaSegment {
    pub unsafe fn frame(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).frame)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn offset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).offset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn length(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn read(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).read)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn write(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).write)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn execute(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).execute)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn addressSection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressSection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn relativeVirtualAddress(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).relativeVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSegment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub offset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub length:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub read: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub write: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub execute: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub addressSection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub relativeVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub virtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IDiaSegment_Impl: windows_core::IUnknownImpl {
    fn frame(&self) -> windows_core::Result<u32>;
    fn offset(&self) -> windows_core::Result<u32>;
    fn length(&self) -> windows_core::Result<u32>;
    fn read(&self) -> windows_core::Result<windows_core::BOOL>;
    fn write(&self) -> windows_core::Result<windows_core::BOOL>;
    fn execute(&self) -> windows_core::Result<windows_core::BOOL>;
    fn addressSection(&self) -> windows_core::Result<u32>;
    fn relativeVirtualAddress(&self) -> windows_core::Result<u32>;
    fn virtualAddress(&self) -> windows_core::Result<u64>;
}
impl IDiaSegment_Vtbl {
    pub const fn new<Identity: IDiaSegment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn frame<Identity: IDiaSegment_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSegment_Impl::frame(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn offset<Identity: IDiaSegment_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSegment_Impl::offset(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: IDiaSegment_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSegment_Impl::length(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn read<Identity: IDiaSegment_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSegment_Impl::read(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn write<Identity: IDiaSegment_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSegment_Impl::write(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn execute<Identity: IDiaSegment_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSegment_Impl::execute(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn addressSection<
            Identity: IDiaSegment_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSegment_Impl::addressSection(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn relativeVirtualAddress<
            Identity: IDiaSegment_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSegment_Impl::relativeVirtualAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn virtualAddress<
            Identity: IDiaSegment_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSegment_Impl::virtualAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            frame: frame::<Identity, OFFSET>,
            offset: offset::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            read: read::<Identity, OFFSET>,
            write: write::<Identity, OFFSET>,
            execute: execute::<Identity, OFFSET>,
            addressSection: addressSection::<Identity, OFFSET>,
            relativeVirtualAddress: relativeVirtualAddress::<Identity, OFFSET>,
            virtualAddress: virtualAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaSegment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaSegment {}
windows_core::imp::define_interface!(
    IDiaSession,
    IDiaSession_Vtbl,
    0x2f609ee1_d1c8_4e24_8288_3326badcd211
);
windows_core::imp::interface_hierarchy!(IDiaSession, windows_core::IUnknown);
impl IDiaSession {
    pub unsafe fn loadAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).loadAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetloadAddress(&self, newval: u64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetloadAddress)(
                windows_core::Interface::as_raw(self),
                newval,
            )
        }
    }
    pub unsafe fn globalScope(&self) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).globalScope)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getEnumTables(&self) -> windows_core::Result<IDiaEnumTables> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getEnumTables)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getSymbolsByAddr(&self) -> windows_core::Result<IDiaEnumSymbolsByAddr> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getSymbolsByAddr)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findChildren<P0, P2>(
        &self,
        parent: P0,
        symtag: SymTagEnum,
        name: P2,
        compareflags: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<IDiaSymbol>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findChildren)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                symtag,
                name.param().abi(),
                compareflags,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findChildrenEx<P0, P2>(
        &self,
        parent: P0,
        symtag: SymTagEnum,
        name: P2,
        compareflags: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<IDiaSymbol>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findChildrenEx)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                symtag,
                name.param().abi(),
                compareflags,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findChildrenExByAddr<P0, P2>(
        &self,
        parent: P0,
        symtag: SymTagEnum,
        name: P2,
        compareflags: u32,
        isect: u32,
        offset: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<IDiaSymbol>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findChildrenExByAddr)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                symtag,
                name.param().abi(),
                compareflags,
                isect,
                offset,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findChildrenExByVA<P0, P2>(
        &self,
        parent: P0,
        symtag: SymTagEnum,
        name: P2,
        compareflags: u32,
        va: u64,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<IDiaSymbol>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findChildrenExByVA)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                symtag,
                name.param().abi(),
                compareflags,
                va,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findChildrenExByRVA<P0, P2>(
        &self,
        parent: P0,
        symtag: SymTagEnum,
        name: P2,
        compareflags: u32,
        rva: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<IDiaSymbol>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findChildrenExByRVA)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                symtag,
                name.param().abi(),
                compareflags,
                rva,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findSymbolByAddr(
        &self,
        isect: u32,
        offset: u32,
        symtag: SymTagEnum,
    ) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findSymbolByAddr)(
                windows_core::Interface::as_raw(self),
                isect,
                offset,
                symtag,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findSymbolByRVA(
        &self,
        rva: u32,
        symtag: SymTagEnum,
    ) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findSymbolByRVA)(
                windows_core::Interface::as_raw(self),
                rva,
                symtag,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findSymbolByVA(
        &self,
        va: u64,
        symtag: SymTagEnum,
    ) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findSymbolByVA)(
                windows_core::Interface::as_raw(self),
                va,
                symtag,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findSymbolByToken(
        &self,
        token: u32,
        symtag: SymTagEnum,
    ) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findSymbolByToken)(
                windows_core::Interface::as_raw(self),
                token,
                symtag,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn symsAreEquiv<P0, P1>(&self, symbola: P0, symbolb: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiaSymbol>,
        P1: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).symsAreEquiv)(
                windows_core::Interface::as_raw(self),
                symbola.param().abi(),
                symbolb.param().abi(),
            )
        }
    }
    pub unsafe fn symbolById(&self, id: u32) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symbolById)(
                windows_core::Interface::as_raw(self),
                id,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findSymbolByRVAEx(
        &self,
        rva: u32,
        symtag: SymTagEnum,
        ppsymbol: *mut Option<IDiaSymbol>,
        displacement: *mut i32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).findSymbolByRVAEx)(
                windows_core::Interface::as_raw(self),
                rva,
                symtag,
                core::mem::transmute(ppsymbol),
                displacement as _,
            )
        }
    }
    pub unsafe fn findSymbolByVAEx(
        &self,
        va: u64,
        symtag: SymTagEnum,
        ppsymbol: *mut Option<IDiaSymbol>,
        displacement: *mut i32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).findSymbolByVAEx)(
                windows_core::Interface::as_raw(self),
                va,
                symtag,
                core::mem::transmute(ppsymbol),
                displacement as _,
            )
        }
    }
    pub unsafe fn findFile<P0, P1>(
        &self,
        pcompiland: P0,
        name: P1,
        compareflags: u32,
    ) -> windows_core::Result<IDiaEnumSourceFiles>
    where
        P0: windows_core::Param<IDiaSymbol>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findFile)(
                windows_core::Interface::as_raw(self),
                pcompiland.param().abi(),
                name.param().abi(),
                compareflags,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findFileById(&self, uniqueid: u32) -> windows_core::Result<IDiaSourceFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findFileById)(
                windows_core::Interface::as_raw(self),
                uniqueid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findLines<P0, P1>(
        &self,
        compiland: P0,
        file: P1,
    ) -> windows_core::Result<IDiaEnumLineNumbers>
    where
        P0: windows_core::Param<IDiaSymbol>,
        P1: windows_core::Param<IDiaSourceFile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findLines)(
                windows_core::Interface::as_raw(self),
                compiland.param().abi(),
                file.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findLinesByAddr(
        &self,
        seg: u32,
        offset: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findLinesByAddr)(
                windows_core::Interface::as_raw(self),
                seg,
                offset,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findLinesByRVA(
        &self,
        rva: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findLinesByRVA)(
                windows_core::Interface::as_raw(self),
                rva,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findLinesByVA(
        &self,
        va: u64,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findLinesByVA)(
                windows_core::Interface::as_raw(self),
                va,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findLinesByLinenum<P0, P1>(
        &self,
        compiland: P0,
        file: P1,
        linenum: u32,
        column: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>
    where
        P0: windows_core::Param<IDiaSymbol>,
        P1: windows_core::Param<IDiaSourceFile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findLinesByLinenum)(
                windows_core::Interface::as_raw(self),
                compiland.param().abi(),
                file.param().abi(),
                linenum,
                column,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInjectedSource<P0>(
        &self,
        srcfile: P0,
    ) -> windows_core::Result<IDiaEnumInjectedSources>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInjectedSource)(
                windows_core::Interface::as_raw(self),
                srcfile.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getEnumDebugStreams(&self) -> windows_core::Result<IDiaEnumDebugStreams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getEnumDebugStreams)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineFramesByAddr<P0>(
        &self,
        parent: P0,
        isect: u32,
        offset: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineFramesByAddr)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                isect,
                offset,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineFramesByRVA<P0>(
        &self,
        parent: P0,
        rva: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineFramesByRVA)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                rva,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineFramesByVA<P0>(
        &self,
        parent: P0,
        va: u64,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineFramesByVA)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                va,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineeLines<P0>(
        &self,
        parent: P0,
    ) -> windows_core::Result<IDiaEnumLineNumbers>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineeLines)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineeLinesByAddr<P0>(
        &self,
        parent: P0,
        isect: u32,
        offset: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineeLinesByAddr)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                isect,
                offset,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineeLinesByRVA<P0>(
        &self,
        parent: P0,
        rva: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineeLinesByRVA)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                rva,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineeLinesByVA<P0>(
        &self,
        parent: P0,
        va: u64,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineeLinesByVA)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                va,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineeLinesByLinenum<P0, P1>(
        &self,
        compiland: P0,
        file: P1,
        linenum: u32,
        column: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>
    where
        P0: windows_core::Param<IDiaSymbol>,
        P1: windows_core::Param<IDiaSourceFile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineeLinesByLinenum)(
                windows_core::Interface::as_raw(self),
                compiland.param().abi(),
                file.param().abi(),
                linenum,
                column,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineesByName<P0>(
        &self,
        name: P0,
        option: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineesByName)(
                windows_core::Interface::as_raw(self),
                name.param().abi(),
                option,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findAcceleratorInlineeLinesByLinenum<P0, P1>(
        &self,
        parent: P0,
        file: P1,
        linenum: u32,
        column: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>
    where
        P0: windows_core::Param<IDiaSymbol>,
        P1: windows_core::Param<IDiaSourceFile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findAcceleratorInlineeLinesByLinenum)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                file.param().abi(),
                linenum,
                column,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findSymbolsForAcceleratorPointerTag<P0>(
        &self,
        parent: P0,
        tagvalue: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findSymbolsForAcceleratorPointerTag)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                tagvalue,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findSymbolsByRVAForAcceleratorPointerTag<P0>(
        &self,
        parent: P0,
        tagvalue: u32,
        rva: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findSymbolsByRVAForAcceleratorPointerTag)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                tagvalue,
                rva,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findAcceleratorInlineesByName<P0>(
        &self,
        name: P0,
        option: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findAcceleratorInlineesByName)(
                windows_core::Interface::as_raw(self),
                name.param().abi(),
                option,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn addressForVA(
        &self,
        va: u64,
        pisect: *mut u32,
        poffset: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).addressForVA)(
                windows_core::Interface::as_raw(self),
                va,
                pisect as _,
                poffset as _,
            )
        }
    }
    pub unsafe fn addressForRVA(
        &self,
        rva: u32,
        pisect: *mut u32,
        poffset: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).addressForRVA)(
                windows_core::Interface::as_raw(self),
                rva,
                pisect as _,
                poffset as _,
            )
        }
    }
    pub unsafe fn findILOffsetsByAddr(
        &self,
        isect: u32,
        offset: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findILOffsetsByAddr)(
                windows_core::Interface::as_raw(self),
                isect,
                offset,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findILOffsetsByRVA(
        &self,
        rva: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findILOffsetsByRVA)(
                windows_core::Interface::as_raw(self),
                rva,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findILOffsetsByVA(
        &self,
        va: u64,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findILOffsetsByVA)(
                windows_core::Interface::as_raw(self),
                va,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInputAssemblyFiles(
        &self,
    ) -> windows_core::Result<IDiaEnumInputAssemblyFiles> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInputAssemblyFiles)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInputAssembly(
        &self,
        index: u32,
    ) -> windows_core::Result<IDiaInputAssemblyFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInputAssembly)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInputAssemblyById(
        &self,
        uniqueid: u32,
    ) -> windows_core::Result<IDiaInputAssemblyFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInputAssemblyById)(
                windows_core::Interface::as_raw(self),
                uniqueid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getFuncMDTokenMapSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getFuncMDTokenMapSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn getFuncMDTokenMap(
        &self,
        cb: u32,
        pcb: *mut u32,
        pb: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).getFuncMDTokenMap)(
                windows_core::Interface::as_raw(self),
                cb,
                pcb as _,
                pb as _,
            )
        }
    }
    pub unsafe fn getTypeMDTokenMapSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getTypeMDTokenMapSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn getTypeMDTokenMap(
        &self,
        cb: u32,
        pcb: *mut u32,
        pb: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).getTypeMDTokenMap)(
                windows_core::Interface::as_raw(self),
                cb,
                pcb as _,
                pb as _,
            )
        }
    }
    pub unsafe fn getNumberOfFunctionFragments_VA(
        &self,
        vafunc: u64,
        cbfunc: u32,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getNumberOfFunctionFragments_VA)(
                windows_core::Interface::as_raw(self),
                vafunc,
                cbfunc,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn getNumberOfFunctionFragments_RVA(
        &self,
        rvafunc: u32,
        cbfunc: u32,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getNumberOfFunctionFragments_RVA)(
                windows_core::Interface::as_raw(self),
                rvafunc,
                cbfunc,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn getFunctionFragments_VA(
        &self,
        vafunc: u64,
        cbfunc: u32,
        cfragments: u32,
        pvafragment: *mut u64,
        plenfragment: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).getFunctionFragments_VA)(
                windows_core::Interface::as_raw(self),
                vafunc,
                cbfunc,
                cfragments,
                pvafragment as _,
                plenfragment as _,
            )
        }
    }
    pub unsafe fn getFunctionFragments_RVA(
        &self,
        rvafunc: u32,
        cbfunc: u32,
        cfragments: u32,
        prvafragment: *mut u32,
        plenfragment: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).getFunctionFragments_RVA)(
                windows_core::Interface::as_raw(self),
                rvafunc,
                cbfunc,
                cfragments,
                prvafragment as _,
                plenfragment as _,
            )
        }
    }
    pub unsafe fn getExports(&self) -> windows_core::Result<IDiaEnumSymbols> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getExports)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getHeapAllocationSites(&self) -> windows_core::Result<IDiaEnumSymbols> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getHeapAllocationSites)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInputAssemblyFile<P0>(
        &self,
        psymbol: P0,
    ) -> windows_core::Result<IDiaInputAssemblyFile>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInputAssemblyFile)(
                windows_core::Interface::as_raw(self),
                psymbol.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub loadAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetloadAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub globalScope: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub getEnumTables: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub getSymbolsByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findChildren: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        SymTagEnum,
        windows_core::PCWSTR,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findChildrenEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        SymTagEnum,
        windows_core::PCWSTR,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findChildrenExByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        SymTagEnum,
        windows_core::PCWSTR,
        u32,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findChildrenExByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        SymTagEnum,
        windows_core::PCWSTR,
        u32,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findChildrenExByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        SymTagEnum,
        windows_core::PCWSTR,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findSymbolByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        SymTagEnum,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findSymbolByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        SymTagEnum,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findSymbolByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        SymTagEnum,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findSymbolByToken: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        SymTagEnum,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub symsAreEquiv: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub symbolById: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findSymbolByRVAEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        SymTagEnum,
        *mut *mut core::ffi::c_void,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub findSymbolByVAEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        SymTagEnum,
        *mut *mut core::ffi::c_void,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub findFile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findFileById: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findLines: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findLinesByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findLinesByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findLinesByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findLinesByLinenum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInjectedSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub getEnumDebugStreams: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineFramesByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineFramesByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineFramesByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineeLines: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineeLinesByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineeLinesByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineeLinesByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u64,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineeLinesByLinenum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineesByName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findAcceleratorInlineeLinesByLinenum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub findSymbolsForAcceleratorPointerTag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub findSymbolsByRVAForAcceleratorPointerTag:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            u32,
            u32,
            *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
    pub findAcceleratorInlineesByName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub addressForVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub addressForRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub findILOffsetsByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findILOffsetsByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findILOffsetsByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInputAssemblyFiles: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInputAssembly: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInputAssemblyById: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub getFuncMDTokenMapSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub getFuncMDTokenMap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub getTypeMDTokenMapSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub getTypeMDTokenMap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub getNumberOfFunctionFragments_VA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub getNumberOfFunctionFragments_RVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub getFunctionFragments_VA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        u32,
        u32,
        *mut u64,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub getFunctionFragments_RVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        u32,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub getExports: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub getHeapAllocationSites: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInputAssemblyFile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaSession_Impl: windows_core::IUnknownImpl {
    fn loadAddress(&self) -> windows_core::Result<u64>;
    fn SetloadAddress(&self, newval: u64) -> windows_core::Result<()>;
    fn globalScope(&self) -> windows_core::Result<IDiaSymbol>;
    fn getEnumTables(&self) -> windows_core::Result<IDiaEnumTables>;
    fn getSymbolsByAddr(&self) -> windows_core::Result<IDiaEnumSymbolsByAddr>;
    fn findChildren(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        symtag: SymTagEnum,
        name: &windows_core::PCWSTR,
        compareflags: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findChildrenEx(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        symtag: SymTagEnum,
        name: &windows_core::PCWSTR,
        compareflags: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findChildrenExByAddr(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        symtag: SymTagEnum,
        name: &windows_core::PCWSTR,
        compareflags: u32,
        isect: u32,
        offset: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findChildrenExByVA(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        symtag: SymTagEnum,
        name: &windows_core::PCWSTR,
        compareflags: u32,
        va: u64,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findChildrenExByRVA(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        symtag: SymTagEnum,
        name: &windows_core::PCWSTR,
        compareflags: u32,
        rva: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findSymbolByAddr(
        &self,
        isect: u32,
        offset: u32,
        symtag: SymTagEnum,
    ) -> windows_core::Result<IDiaSymbol>;
    fn findSymbolByRVA(&self, rva: u32, symtag: SymTagEnum) -> windows_core::Result<IDiaSymbol>;
    fn findSymbolByVA(&self, va: u64, symtag: SymTagEnum) -> windows_core::Result<IDiaSymbol>;
    fn findSymbolByToken(&self, token: u32, symtag: SymTagEnum)
    -> windows_core::Result<IDiaSymbol>;
    fn symsAreEquiv(
        &self,
        symbola: windows_core::Ref<IDiaSymbol>,
        symbolb: windows_core::Ref<IDiaSymbol>,
    ) -> windows_core::Result<()>;
    fn symbolById(&self, id: u32) -> windows_core::Result<IDiaSymbol>;
    fn findSymbolByRVAEx(
        &self,
        rva: u32,
        symtag: SymTagEnum,
        ppsymbol: windows_core::OutRef<IDiaSymbol>,
        displacement: *mut i32,
    ) -> windows_core::Result<()>;
    fn findSymbolByVAEx(
        &self,
        va: u64,
        symtag: SymTagEnum,
        ppsymbol: windows_core::OutRef<IDiaSymbol>,
        displacement: *mut i32,
    ) -> windows_core::Result<()>;
    fn findFile(
        &self,
        pcompiland: windows_core::Ref<IDiaSymbol>,
        name: &windows_core::PCWSTR,
        compareflags: u32,
    ) -> windows_core::Result<IDiaEnumSourceFiles>;
    fn findFileById(&self, uniqueid: u32) -> windows_core::Result<IDiaSourceFile>;
    fn findLines(
        &self,
        compiland: windows_core::Ref<IDiaSymbol>,
        file: windows_core::Ref<IDiaSourceFile>,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findLinesByAddr(
        &self,
        seg: u32,
        offset: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findLinesByRVA(&self, rva: u32, length: u32) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findLinesByVA(&self, va: u64, length: u32) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findLinesByLinenum(
        &self,
        compiland: windows_core::Ref<IDiaSymbol>,
        file: windows_core::Ref<IDiaSourceFile>,
        linenum: u32,
        column: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findInjectedSource(
        &self,
        srcfile: &windows_core::PCWSTR,
    ) -> windows_core::Result<IDiaEnumInjectedSources>;
    fn getEnumDebugStreams(&self) -> windows_core::Result<IDiaEnumDebugStreams>;
    fn findInlineFramesByAddr(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        isect: u32,
        offset: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findInlineFramesByRVA(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        rva: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findInlineFramesByVA(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        va: u64,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findInlineeLines(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByAddr(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        isect: u32,
        offset: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByRVA(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        rva: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByVA(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        va: u64,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findInlineeLinesByLinenum(
        &self,
        compiland: windows_core::Ref<IDiaSymbol>,
        file: windows_core::Ref<IDiaSourceFile>,
        linenum: u32,
        column: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findInlineesByName(
        &self,
        name: &windows_core::PCWSTR,
        option: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findAcceleratorInlineeLinesByLinenum(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        file: windows_core::Ref<IDiaSourceFile>,
        linenum: u32,
        column: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findSymbolsForAcceleratorPointerTag(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        tagvalue: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findSymbolsByRVAForAcceleratorPointerTag(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
        tagvalue: u32,
        rva: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn findAcceleratorInlineesByName(
        &self,
        name: &windows_core::PCWSTR,
        option: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>;
    fn addressForVA(
        &self,
        va: u64,
        pisect: *mut u32,
        poffset: *mut u32,
    ) -> windows_core::Result<()>;
    fn addressForRVA(
        &self,
        rva: u32,
        pisect: *mut u32,
        poffset: *mut u32,
    ) -> windows_core::Result<()>;
    fn findILOffsetsByAddr(
        &self,
        isect: u32,
        offset: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findILOffsetsByRVA(
        &self,
        rva: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findILOffsetsByVA(&self, va: u64, length: u32) -> windows_core::Result<IDiaEnumLineNumbers>;
    fn findInputAssemblyFiles(&self) -> windows_core::Result<IDiaEnumInputAssemblyFiles>;
    fn findInputAssembly(&self, index: u32) -> windows_core::Result<IDiaInputAssemblyFile>;
    fn findInputAssemblyById(&self, uniqueid: u32) -> windows_core::Result<IDiaInputAssemblyFile>;
    fn getFuncMDTokenMapSize(&self) -> windows_core::Result<u32>;
    fn getFuncMDTokenMap(&self, cb: u32, pcb: *mut u32, pb: *mut u8) -> windows_core::Result<()>;
    fn getTypeMDTokenMapSize(&self) -> windows_core::Result<u32>;
    fn getTypeMDTokenMap(&self, cb: u32, pcb: *mut u32, pb: *mut u8) -> windows_core::Result<()>;
    fn getNumberOfFunctionFragments_VA(
        &self,
        vafunc: u64,
        cbfunc: u32,
    ) -> windows_core::Result<u32>;
    fn getNumberOfFunctionFragments_RVA(
        &self,
        rvafunc: u32,
        cbfunc: u32,
    ) -> windows_core::Result<u32>;
    fn getFunctionFragments_VA(
        &self,
        vafunc: u64,
        cbfunc: u32,
        cfragments: u32,
        pvafragment: *mut u64,
        plenfragment: *mut u32,
    ) -> windows_core::Result<()>;
    fn getFunctionFragments_RVA(
        &self,
        rvafunc: u32,
        cbfunc: u32,
        cfragments: u32,
        prvafragment: *mut u32,
        plenfragment: *mut u32,
    ) -> windows_core::Result<()>;
    fn getExports(&self) -> windows_core::Result<IDiaEnumSymbols>;
    fn getHeapAllocationSites(&self) -> windows_core::Result<IDiaEnumSymbols>;
    fn findInputAssemblyFile(
        &self,
        psymbol: windows_core::Ref<IDiaSymbol>,
    ) -> windows_core::Result<IDiaInputAssemblyFile>;
}
impl IDiaSession_Vtbl {
    pub const fn new<Identity: IDiaSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn loadAddress<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::loadAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetloadAddress<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            newval: u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSession_Impl::SetloadAddress(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn globalScope<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::globalScope(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getEnumTables<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenumtables: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::getEnumTables(this) {
                    Ok(ok__) => {
                        ppenumtables.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getSymbolsByAddr<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppenumbyaddr: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::getSymbolsByAddr(this) {
                    Ok(ok__) => {
                        ppenumbyaddr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findChildren<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            symtag: SymTagEnum,
            name: windows_core::PCWSTR,
            compareflags: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findChildren(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&symtag),
                    core::mem::transmute(&name),
                    core::mem::transmute_copy(&compareflags),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findChildrenEx<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            symtag: SymTagEnum,
            name: windows_core::PCWSTR,
            compareflags: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findChildrenEx(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&symtag),
                    core::mem::transmute(&name),
                    core::mem::transmute_copy(&compareflags),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findChildrenExByAddr<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            symtag: SymTagEnum,
            name: windows_core::PCWSTR,
            compareflags: u32,
            isect: u32,
            offset: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findChildrenExByAddr(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&symtag),
                    core::mem::transmute(&name),
                    core::mem::transmute_copy(&compareflags),
                    core::mem::transmute_copy(&isect),
                    core::mem::transmute_copy(&offset),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findChildrenExByVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            symtag: SymTagEnum,
            name: windows_core::PCWSTR,
            compareflags: u32,
            va: u64,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findChildrenExByVA(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&symtag),
                    core::mem::transmute(&name),
                    core::mem::transmute_copy(&compareflags),
                    core::mem::transmute_copy(&va),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findChildrenExByRVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            symtag: SymTagEnum,
            name: windows_core::PCWSTR,
            compareflags: u32,
            rva: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findChildrenExByRVA(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&symtag),
                    core::mem::transmute(&name),
                    core::mem::transmute_copy(&compareflags),
                    core::mem::transmute_copy(&rva),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findSymbolByAddr<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            isect: u32,
            offset: u32,
            symtag: SymTagEnum,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findSymbolByAddr(
                    this,
                    core::mem::transmute_copy(&isect),
                    core::mem::transmute_copy(&offset),
                    core::mem::transmute_copy(&symtag),
                ) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findSymbolByRVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rva: u32,
            symtag: SymTagEnum,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findSymbolByRVA(
                    this,
                    core::mem::transmute_copy(&rva),
                    core::mem::transmute_copy(&symtag),
                ) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findSymbolByVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            va: u64,
            symtag: SymTagEnum,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findSymbolByVA(
                    this,
                    core::mem::transmute_copy(&va),
                    core::mem::transmute_copy(&symtag),
                ) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findSymbolByToken<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            token: u32,
            symtag: SymTagEnum,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findSymbolByToken(
                    this,
                    core::mem::transmute_copy(&token),
                    core::mem::transmute_copy(&symtag),
                ) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn symsAreEquiv<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            symbola: *mut core::ffi::c_void,
            symbolb: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSession_Impl::symsAreEquiv(
                    this,
                    core::mem::transmute_copy(&symbola),
                    core::mem::transmute_copy(&symbolb),
                )
                .into()
            }
        }
        unsafe extern "system" fn symbolById<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            id: u32,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::symbolById(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findSymbolByRVAEx<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rva: u32,
            symtag: SymTagEnum,
            ppsymbol: *mut *mut core::ffi::c_void,
            displacement: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSession_Impl::findSymbolByRVAEx(
                    this,
                    core::mem::transmute_copy(&rva),
                    core::mem::transmute_copy(&symtag),
                    core::mem::transmute_copy(&ppsymbol),
                    core::mem::transmute_copy(&displacement),
                )
                .into()
            }
        }
        unsafe extern "system" fn findSymbolByVAEx<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            va: u64,
            symtag: SymTagEnum,
            ppsymbol: *mut *mut core::ffi::c_void,
            displacement: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSession_Impl::findSymbolByVAEx(
                    this,
                    core::mem::transmute_copy(&va),
                    core::mem::transmute_copy(&symtag),
                    core::mem::transmute_copy(&ppsymbol),
                    core::mem::transmute_copy(&displacement),
                )
                .into()
            }
        }
        unsafe extern "system" fn findFile<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pcompiland: *mut core::ffi::c_void,
            name: windows_core::PCWSTR,
            compareflags: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findFile(
                    this,
                    core::mem::transmute_copy(&pcompiland),
                    core::mem::transmute(&name),
                    core::mem::transmute_copy(&compareflags),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findFileById<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            uniqueid: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findFileById(this, core::mem::transmute_copy(&uniqueid)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findLines<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            compiland: *mut core::ffi::c_void,
            file: *mut core::ffi::c_void,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findLines(
                    this,
                    core::mem::transmute_copy(&compiland),
                    core::mem::transmute_copy(&file),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findLinesByAddr<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            seg: u32,
            offset: u32,
            length: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findLinesByAddr(
                    this,
                    core::mem::transmute_copy(&seg),
                    core::mem::transmute_copy(&offset),
                    core::mem::transmute_copy(&length),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findLinesByRVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rva: u32,
            length: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findLinesByRVA(
                    this,
                    core::mem::transmute_copy(&rva),
                    core::mem::transmute_copy(&length),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findLinesByVA<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            va: u64,
            length: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findLinesByVA(
                    this,
                    core::mem::transmute_copy(&va),
                    core::mem::transmute_copy(&length),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findLinesByLinenum<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            compiland: *mut core::ffi::c_void,
            file: *mut core::ffi::c_void,
            linenum: u32,
            column: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findLinesByLinenum(
                    this,
                    core::mem::transmute_copy(&compiland),
                    core::mem::transmute_copy(&file),
                    core::mem::transmute_copy(&linenum),
                    core::mem::transmute_copy(&column),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInjectedSource<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            srcfile: windows_core::PCWSTR,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInjectedSource(this, core::mem::transmute(&srcfile)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getEnumDebugStreams<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppenumdebugstreams: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::getEnumDebugStreams(this) {
                    Ok(ok__) => {
                        ppenumdebugstreams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInlineFramesByAddr<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            isect: u32,
            offset: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInlineFramesByAddr(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&isect),
                    core::mem::transmute_copy(&offset),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInlineFramesByRVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            rva: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInlineFramesByRVA(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&rva),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInlineFramesByVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            va: u64,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInlineFramesByVA(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&va),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInlineeLines<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInlineeLines(this, core::mem::transmute_copy(&parent)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInlineeLinesByAddr<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            isect: u32,
            offset: u32,
            length: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInlineeLinesByAddr(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&isect),
                    core::mem::transmute_copy(&offset),
                    core::mem::transmute_copy(&length),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInlineeLinesByRVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            rva: u32,
            length: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInlineeLinesByRVA(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&rva),
                    core::mem::transmute_copy(&length),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInlineeLinesByVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            va: u64,
            length: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInlineeLinesByVA(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&va),
                    core::mem::transmute_copy(&length),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInlineeLinesByLinenum<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            compiland: *mut core::ffi::c_void,
            file: *mut core::ffi::c_void,
            linenum: u32,
            column: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInlineeLinesByLinenum(
                    this,
                    core::mem::transmute_copy(&compiland),
                    core::mem::transmute_copy(&file),
                    core::mem::transmute_copy(&linenum),
                    core::mem::transmute_copy(&column),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInlineesByName<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            name: windows_core::PCWSTR,
            option: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInlineesByName(
                    this,
                    core::mem::transmute(&name),
                    core::mem::transmute_copy(&option),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findAcceleratorInlineeLinesByLinenum<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            file: *mut core::ffi::c_void,
            linenum: u32,
            column: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findAcceleratorInlineeLinesByLinenum(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&file),
                    core::mem::transmute_copy(&linenum),
                    core::mem::transmute_copy(&column),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findSymbolsForAcceleratorPointerTag<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            tagvalue: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findSymbolsForAcceleratorPointerTag(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&tagvalue),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findSymbolsByRVAForAcceleratorPointerTag<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            tagvalue: u32,
            rva: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findSymbolsByRVAForAcceleratorPointerTag(
                    this,
                    core::mem::transmute_copy(&parent),
                    core::mem::transmute_copy(&tagvalue),
                    core::mem::transmute_copy(&rva),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findAcceleratorInlineesByName<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            name: windows_core::PCWSTR,
            option: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findAcceleratorInlineesByName(
                    this,
                    core::mem::transmute(&name),
                    core::mem::transmute_copy(&option),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn addressForVA<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            va: u64,
            pisect: *mut u32,
            poffset: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSession_Impl::addressForVA(
                    this,
                    core::mem::transmute_copy(&va),
                    core::mem::transmute_copy(&pisect),
                    core::mem::transmute_copy(&poffset),
                )
                .into()
            }
        }
        unsafe extern "system" fn addressForRVA<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            rva: u32,
            pisect: *mut u32,
            poffset: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSession_Impl::addressForRVA(
                    this,
                    core::mem::transmute_copy(&rva),
                    core::mem::transmute_copy(&pisect),
                    core::mem::transmute_copy(&poffset),
                )
                .into()
            }
        }
        unsafe extern "system" fn findILOffsetsByAddr<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            isect: u32,
            offset: u32,
            length: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findILOffsetsByAddr(
                    this,
                    core::mem::transmute_copy(&isect),
                    core::mem::transmute_copy(&offset),
                    core::mem::transmute_copy(&length),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findILOffsetsByRVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rva: u32,
            length: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findILOffsetsByRVA(
                    this,
                    core::mem::transmute_copy(&rva),
                    core::mem::transmute_copy(&length),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findILOffsetsByVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            va: u64,
            length: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findILOffsetsByVA(
                    this,
                    core::mem::transmute_copy(&va),
                    core::mem::transmute_copy(&length),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInputAssemblyFiles<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInputAssemblyFiles(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInputAssembly<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInputAssembly(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInputAssemblyById<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            uniqueid: u32,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInputAssemblyById(
                    this,
                    core::mem::transmute_copy(&uniqueid),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getFuncMDTokenMapSize<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcb: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::getFuncMDTokenMapSize(this) {
                    Ok(ok__) => {
                        pcb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getFuncMDTokenMap<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cb: u32,
            pcb: *mut u32,
            pb: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSession_Impl::getFuncMDTokenMap(
                    this,
                    core::mem::transmute_copy(&cb),
                    core::mem::transmute_copy(&pcb),
                    core::mem::transmute_copy(&pb),
                )
                .into()
            }
        }
        unsafe extern "system" fn getTypeMDTokenMapSize<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcb: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::getTypeMDTokenMapSize(this) {
                    Ok(ok__) => {
                        pcb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getTypeMDTokenMap<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cb: u32,
            pcb: *mut u32,
            pb: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSession_Impl::getTypeMDTokenMap(
                    this,
                    core::mem::transmute_copy(&cb),
                    core::mem::transmute_copy(&pcb),
                    core::mem::transmute_copy(&pb),
                )
                .into()
            }
        }
        unsafe extern "system" fn getNumberOfFunctionFragments_VA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            vafunc: u64,
            cbfunc: u32,
            pnumfragments: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::getNumberOfFunctionFragments_VA(
                    this,
                    core::mem::transmute_copy(&vafunc),
                    core::mem::transmute_copy(&cbfunc),
                ) {
                    Ok(ok__) => {
                        pnumfragments.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getNumberOfFunctionFragments_RVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rvafunc: u32,
            cbfunc: u32,
            pnumfragments: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::getNumberOfFunctionFragments_RVA(
                    this,
                    core::mem::transmute_copy(&rvafunc),
                    core::mem::transmute_copy(&cbfunc),
                ) {
                    Ok(ok__) => {
                        pnumfragments.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getFunctionFragments_VA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            vafunc: u64,
            cbfunc: u32,
            cfragments: u32,
            pvafragment: *mut u64,
            plenfragment: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSession_Impl::getFunctionFragments_VA(
                    this,
                    core::mem::transmute_copy(&vafunc),
                    core::mem::transmute_copy(&cbfunc),
                    core::mem::transmute_copy(&cfragments),
                    core::mem::transmute_copy(&pvafragment),
                    core::mem::transmute_copy(&plenfragment),
                )
                .into()
            }
        }
        unsafe extern "system" fn getFunctionFragments_RVA<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rvafunc: u32,
            cbfunc: u32,
            cfragments: u32,
            prvafragment: *mut u32,
            plenfragment: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSession_Impl::getFunctionFragments_RVA(
                    this,
                    core::mem::transmute_copy(&rvafunc),
                    core::mem::transmute_copy(&cbfunc),
                    core::mem::transmute_copy(&cfragments),
                    core::mem::transmute_copy(&prvafragment),
                    core::mem::transmute_copy(&plenfragment),
                )
                .into()
            }
        }
        unsafe extern "system" fn getExports<Identity: IDiaSession_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::getExports(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getHeapAllocationSites<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::getHeapAllocationSites(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn findInputAssemblyFile<
            Identity: IDiaSession_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            psymbol: *mut core::ffi::c_void,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSession_Impl::findInputAssemblyFile(
                    this,
                    core::mem::transmute_copy(&psymbol),
                ) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            loadAddress: loadAddress::<Identity, OFFSET>,
            SetloadAddress: SetloadAddress::<Identity, OFFSET>,
            globalScope: globalScope::<Identity, OFFSET>,
            getEnumTables: getEnumTables::<Identity, OFFSET>,
            getSymbolsByAddr: getSymbolsByAddr::<Identity, OFFSET>,
            findChildren: findChildren::<Identity, OFFSET>,
            findChildrenEx: findChildrenEx::<Identity, OFFSET>,
            findChildrenExByAddr: findChildrenExByAddr::<Identity, OFFSET>,
            findChildrenExByVA: findChildrenExByVA::<Identity, OFFSET>,
            findChildrenExByRVA: findChildrenExByRVA::<Identity, OFFSET>,
            findSymbolByAddr: findSymbolByAddr::<Identity, OFFSET>,
            findSymbolByRVA: findSymbolByRVA::<Identity, OFFSET>,
            findSymbolByVA: findSymbolByVA::<Identity, OFFSET>,
            findSymbolByToken: findSymbolByToken::<Identity, OFFSET>,
            symsAreEquiv: symsAreEquiv::<Identity, OFFSET>,
            symbolById: symbolById::<Identity, OFFSET>,
            findSymbolByRVAEx: findSymbolByRVAEx::<Identity, OFFSET>,
            findSymbolByVAEx: findSymbolByVAEx::<Identity, OFFSET>,
            findFile: findFile::<Identity, OFFSET>,
            findFileById: findFileById::<Identity, OFFSET>,
            findLines: findLines::<Identity, OFFSET>,
            findLinesByAddr: findLinesByAddr::<Identity, OFFSET>,
            findLinesByRVA: findLinesByRVA::<Identity, OFFSET>,
            findLinesByVA: findLinesByVA::<Identity, OFFSET>,
            findLinesByLinenum: findLinesByLinenum::<Identity, OFFSET>,
            findInjectedSource: findInjectedSource::<Identity, OFFSET>,
            getEnumDebugStreams: getEnumDebugStreams::<Identity, OFFSET>,
            findInlineFramesByAddr: findInlineFramesByAddr::<Identity, OFFSET>,
            findInlineFramesByRVA: findInlineFramesByRVA::<Identity, OFFSET>,
            findInlineFramesByVA: findInlineFramesByVA::<Identity, OFFSET>,
            findInlineeLines: findInlineeLines::<Identity, OFFSET>,
            findInlineeLinesByAddr: findInlineeLinesByAddr::<Identity, OFFSET>,
            findInlineeLinesByRVA: findInlineeLinesByRVA::<Identity, OFFSET>,
            findInlineeLinesByVA: findInlineeLinesByVA::<Identity, OFFSET>,
            findInlineeLinesByLinenum: findInlineeLinesByLinenum::<Identity, OFFSET>,
            findInlineesByName: findInlineesByName::<Identity, OFFSET>,
            findAcceleratorInlineeLinesByLinenum: findAcceleratorInlineeLinesByLinenum::<
                Identity,
                OFFSET,
            >,
            findSymbolsForAcceleratorPointerTag: findSymbolsForAcceleratorPointerTag::<
                Identity,
                OFFSET,
            >,
            findSymbolsByRVAForAcceleratorPointerTag: findSymbolsByRVAForAcceleratorPointerTag::<
                Identity,
                OFFSET,
            >,
            findAcceleratorInlineesByName: findAcceleratorInlineesByName::<Identity, OFFSET>,
            addressForVA: addressForVA::<Identity, OFFSET>,
            addressForRVA: addressForRVA::<Identity, OFFSET>,
            findILOffsetsByAddr: findILOffsetsByAddr::<Identity, OFFSET>,
            findILOffsetsByRVA: findILOffsetsByRVA::<Identity, OFFSET>,
            findILOffsetsByVA: findILOffsetsByVA::<Identity, OFFSET>,
            findInputAssemblyFiles: findInputAssemblyFiles::<Identity, OFFSET>,
            findInputAssembly: findInputAssembly::<Identity, OFFSET>,
            findInputAssemblyById: findInputAssemblyById::<Identity, OFFSET>,
            getFuncMDTokenMapSize: getFuncMDTokenMapSize::<Identity, OFFSET>,
            getFuncMDTokenMap: getFuncMDTokenMap::<Identity, OFFSET>,
            getTypeMDTokenMapSize: getTypeMDTokenMapSize::<Identity, OFFSET>,
            getTypeMDTokenMap: getTypeMDTokenMap::<Identity, OFFSET>,
            getNumberOfFunctionFragments_VA: getNumberOfFunctionFragments_VA::<Identity, OFFSET>,
            getNumberOfFunctionFragments_RVA: getNumberOfFunctionFragments_RVA::<Identity, OFFSET>,
            getFunctionFragments_VA: getFunctionFragments_VA::<Identity, OFFSET>,
            getFunctionFragments_RVA: getFunctionFragments_RVA::<Identity, OFFSET>,
            getExports: getExports::<Identity, OFFSET>,
            getHeapAllocationSites: getHeapAllocationSites::<Identity, OFFSET>,
            findInputAssemblyFile: findInputAssemblyFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaSession {}
windows_core::imp::define_interface!(
    IDiaSessionEx,
    IDiaSessionEx_Vtbl,
    0xcd24eed5_5fea_4742_a320_6254c920e78b
);
impl core::ops::Deref for IDiaSessionEx {
    type Target = IDiaSession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiaSessionEx, windows_core::IUnknown, IDiaSession);
impl IDiaSessionEx {
    pub unsafe fn isFastLinkPDB(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isFastLinkPDB)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isPortablePDB(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isPortablePDB)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn getSourceLinkInfo<P0>(
        &self,
        parent: P0,
    ) -> windows_core::Result<IDiaEnumSourceLink>
    where
        P0: windows_core::Param<IDiaSymbol>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getSourceLinkInfo)(
                windows_core::Interface::as_raw(self),
                parent.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaSessionEx_Vtbl {
    pub base__: IDiaSession_Vtbl,
    pub isFastLinkPDB: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isPortablePDB: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub getSourceLinkInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaSessionEx_Impl: IDiaSession_Impl {
    fn isFastLinkPDB(&self) -> windows_core::Result<windows_core::BOOL>;
    fn isPortablePDB(&self) -> windows_core::Result<windows_core::BOOL>;
    fn getSourceLinkInfo(
        &self,
        parent: windows_core::Ref<IDiaSymbol>,
    ) -> windows_core::Result<IDiaEnumSourceLink>;
}
impl IDiaSessionEx_Vtbl {
    pub const fn new<Identity: IDiaSessionEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn isFastLinkPDB<
            Identity: IDiaSessionEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pffastlinkpdb: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSessionEx_Impl::isFastLinkPDB(this) {
                    Ok(ok__) => {
                        pffastlinkpdb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn isPortablePDB<
            Identity: IDiaSessionEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pfportablepdb: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSessionEx_Impl::isPortablePDB(this) {
                    Ok(ok__) => {
                        pfportablepdb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getSourceLinkInfo<
            Identity: IDiaSessionEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSessionEx_Impl::getSourceLinkInfo(
                    this,
                    core::mem::transmute_copy(&parent),
                ) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDiaSession_Vtbl::new::<Identity, OFFSET>(),
            isFastLinkPDB: isFastLinkPDB::<Identity, OFFSET>,
            isPortablePDB: isPortablePDB::<Identity, OFFSET>,
            getSourceLinkInfo: getSourceLinkInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaSessionEx as windows_core::Interface>::IID
            || iid == &<IDiaSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaSessionEx {}
windows_core::imp::define_interface!(
    IDiaSourceFile,
    IDiaSourceFile_Vtbl,
    0xa2ef5353_f5a8_4eb3_90d2_cb526acb3cdd
);
windows_core::imp::interface_hierarchy!(IDiaSourceFile, windows_core::IUnknown);
impl IDiaSourceFile {
    pub unsafe fn uniqueId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).uniqueId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn fileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fileName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn checksumType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).checksumType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn compilands(&self) -> windows_core::Result<IDiaEnumSymbols> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).compilands)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_checksum(
        &self,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_checksum)(
                windows_core::Interface::as_raw(self),
                cbdata,
                pcbdata as _,
                pbdata as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDiaSourceFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub uniqueId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub fileName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub checksumType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub compilands: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_checksum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
}
pub trait IDiaSourceFile_Impl: windows_core::IUnknownImpl {
    fn uniqueId(&self) -> windows_core::Result<u32>;
    fn fileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn checksumType(&self) -> windows_core::Result<u32>;
    fn compilands(&self) -> windows_core::Result<IDiaEnumSymbols>;
    fn get_checksum(
        &self,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::Result<()>;
}
impl IDiaSourceFile_Vtbl {
    pub const fn new<Identity: IDiaSourceFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn uniqueId<Identity: IDiaSourceFile_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSourceFile_Impl::uniqueId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fileName<Identity: IDiaSourceFile_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSourceFile_Impl::fileName(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn checksumType<
            Identity: IDiaSourceFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSourceFile_Impl::checksumType(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn compilands<Identity: IDiaSourceFile_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaSourceFile_Impl::compilands(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_checksum<
            Identity: IDiaSourceFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cbdata: u32,
            pcbdata: *mut u32,
            pbdata: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaSourceFile_Impl::get_checksum(
                    this,
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pcbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            uniqueId: uniqueId::<Identity, OFFSET>,
            fileName: fileName::<Identity, OFFSET>,
            checksumType: checksumType::<Identity, OFFSET>,
            compilands: compilands::<Identity, OFFSET>,
            get_checksum: get_checksum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaSourceFile as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaSourceFile {}
windows_core::imp::define_interface!(
    IDiaStackFrame,
    IDiaStackFrame_Vtbl,
    0x5edbc96d_cdd6_4792_afbe_cc89007d9610
);
windows_core::imp::interface_hierarchy!(IDiaStackFrame, windows_core::IUnknown);
impl IDiaStackFrame {
    pub unsafe fn r#type(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#type)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn base(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).base)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn size(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn returnAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).returnAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn localsBase(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).localsBase)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lengthLocals(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lengthLocals)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lengthParams(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lengthParams)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lengthProlog(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lengthProlog)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lengthSavedRegisters(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lengthSavedRegisters)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn systemExceptionHandling(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).systemExceptionHandling)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn cplusplusExceptionHandling(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).cplusplusExceptionHandling)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn functionStart(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).functionStart)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn allocatesBasePointer(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).allocatesBasePointer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn maxStack(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).maxStack)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn registerValue(&self, index: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).registerValue)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaStackFrame_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub r#type:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub base: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub returnAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub localsBase:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub lengthLocals:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub lengthParams:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub lengthProlog:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub lengthSavedRegisters:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub systemExceptionHandling: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub cplusplusExceptionHandling: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub functionStart: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub allocatesBasePointer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub maxStack:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub registerValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
}
pub trait IDiaStackFrame_Impl: windows_core::IUnknownImpl {
    fn r#type(&self) -> windows_core::Result<u32>;
    fn base(&self) -> windows_core::Result<u64>;
    fn size(&self) -> windows_core::Result<u32>;
    fn returnAddress(&self) -> windows_core::Result<u64>;
    fn localsBase(&self) -> windows_core::Result<u64>;
    fn lengthLocals(&self) -> windows_core::Result<u32>;
    fn lengthParams(&self) -> windows_core::Result<u32>;
    fn lengthProlog(&self) -> windows_core::Result<u32>;
    fn lengthSavedRegisters(&self) -> windows_core::Result<u32>;
    fn systemExceptionHandling(&self) -> windows_core::Result<windows_core::BOOL>;
    fn cplusplusExceptionHandling(&self) -> windows_core::Result<windows_core::BOOL>;
    fn functionStart(&self) -> windows_core::Result<windows_core::BOOL>;
    fn allocatesBasePointer(&self) -> windows_core::Result<windows_core::BOOL>;
    fn maxStack(&self) -> windows_core::Result<u32>;
    fn registerValue(&self, index: u32) -> windows_core::Result<u64>;
}
impl IDiaStackFrame_Vtbl {
    pub const fn new<Identity: IDiaStackFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn r#type<Identity: IDiaStackFrame_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::r#type(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn base<Identity: IDiaStackFrame_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::base(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn size<Identity: IDiaStackFrame_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::size(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn returnAddress<
            Identity: IDiaStackFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::returnAddress(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn localsBase<Identity: IDiaStackFrame_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::localsBase(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lengthLocals<
            Identity: IDiaStackFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::lengthLocals(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lengthParams<
            Identity: IDiaStackFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::lengthParams(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lengthProlog<
            Identity: IDiaStackFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::lengthProlog(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lengthSavedRegisters<
            Identity: IDiaStackFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::lengthSavedRegisters(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn systemExceptionHandling<
            Identity: IDiaStackFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::systemExceptionHandling(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn cplusplusExceptionHandling<
            Identity: IDiaStackFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::cplusplusExceptionHandling(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn functionStart<
            Identity: IDiaStackFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::functionStart(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn allocatesBasePointer<
            Identity: IDiaStackFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::allocatesBasePointer(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn maxStack<Identity: IDiaStackFrame_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::maxStack(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn registerValue<
            Identity: IDiaStackFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackFrame_Impl::registerValue(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            r#type: r#type::<Identity, OFFSET>,
            base: base::<Identity, OFFSET>,
            size: size::<Identity, OFFSET>,
            returnAddress: returnAddress::<Identity, OFFSET>,
            localsBase: localsBase::<Identity, OFFSET>,
            lengthLocals: lengthLocals::<Identity, OFFSET>,
            lengthParams: lengthParams::<Identity, OFFSET>,
            lengthProlog: lengthProlog::<Identity, OFFSET>,
            lengthSavedRegisters: lengthSavedRegisters::<Identity, OFFSET>,
            systemExceptionHandling: systemExceptionHandling::<Identity, OFFSET>,
            cplusplusExceptionHandling: cplusplusExceptionHandling::<Identity, OFFSET>,
            functionStart: functionStart::<Identity, OFFSET>,
            allocatesBasePointer: allocatesBasePointer::<Identity, OFFSET>,
            maxStack: maxStack::<Identity, OFFSET>,
            registerValue: registerValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaStackFrame as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaStackFrame {}
windows_core::imp::define_interface!(
    IDiaStackWalkFrame,
    IDiaStackWalkFrame_Vtbl,
    0x07c590c1_438d_4f47_bdcd_4397bc81ad75
);
windows_core::imp::interface_hierarchy!(IDiaStackWalkFrame, windows_core::IUnknown);
impl IDiaStackWalkFrame {
    pub unsafe fn registerValue(&self, index: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).registerValue)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetregisterValue(&self, index: u32, newval: u64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetregisterValue)(
                windows_core::Interface::as_raw(self),
                index,
                newval,
            )
        }
    }
    pub unsafe fn readMemory(
        &self,
        r#type: MemoryTypeEnum,
        va: u64,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).readMemory)(
                windows_core::Interface::as_raw(self),
                r#type,
                va,
                cbdata,
                pcbdata as _,
                pbdata as _,
            )
        }
    }
    pub unsafe fn searchForReturnAddress<P0>(&self, frame: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<IDiaFrameData>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).searchForReturnAddress)(
                windows_core::Interface::as_raw(self),
                frame.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn searchForReturnAddressStart<P0>(
        &self,
        frame: P0,
        startaddress: u64,
    ) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<IDiaFrameData>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).searchForReturnAddressStart)(
                windows_core::Interface::as_raw(self),
                frame.param().abi(),
                startaddress,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaStackWalkFrame_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub registerValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
    pub SetregisterValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64) -> windows_core::HRESULT,
    pub readMemory: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        MemoryTypeEnum,
        u64,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub searchForReturnAddress: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u64,
    ) -> windows_core::HRESULT,
    pub searchForReturnAddressStart: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u64,
        *mut u64,
    ) -> windows_core::HRESULT,
}
pub trait IDiaStackWalkFrame_Impl: windows_core::IUnknownImpl {
    fn registerValue(&self, index: u32) -> windows_core::Result<u64>;
    fn SetregisterValue(&self, index: u32, newval: u64) -> windows_core::Result<()>;
    fn readMemory(
        &self,
        r#type: MemoryTypeEnum,
        va: u64,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::Result<()>;
    fn searchForReturnAddress(
        &self,
        frame: windows_core::Ref<IDiaFrameData>,
    ) -> windows_core::Result<u64>;
    fn searchForReturnAddressStart(
        &self,
        frame: windows_core::Ref<IDiaFrameData>,
        startaddress: u64,
    ) -> windows_core::Result<u64>;
}
impl IDiaStackWalkFrame_Vtbl {
    pub const fn new<Identity: IDiaStackWalkFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn registerValue<
            Identity: IDiaStackWalkFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkFrame_Impl::registerValue(
                    this,
                    core::mem::transmute_copy(&index),
                ) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetregisterValue<
            Identity: IDiaStackWalkFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            newval: u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaStackWalkFrame_Impl::SetregisterValue(
                    this,
                    core::mem::transmute_copy(&index),
                    core::mem::transmute_copy(&newval),
                )
                .into()
            }
        }
        unsafe extern "system" fn readMemory<
            Identity: IDiaStackWalkFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            r#type: MemoryTypeEnum,
            va: u64,
            cbdata: u32,
            pcbdata: *mut u32,
            pbdata: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaStackWalkFrame_Impl::readMemory(
                    this,
                    core::mem::transmute_copy(&r#type),
                    core::mem::transmute_copy(&va),
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pcbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn searchForReturnAddress<
            Identity: IDiaStackWalkFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            frame: *mut core::ffi::c_void,
            returnaddress: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkFrame_Impl::searchForReturnAddress(
                    this,
                    core::mem::transmute_copy(&frame),
                ) {
                    Ok(ok__) => {
                        returnaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn searchForReturnAddressStart<
            Identity: IDiaStackWalkFrame_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            frame: *mut core::ffi::c_void,
            startaddress: u64,
            returnaddress: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkFrame_Impl::searchForReturnAddressStart(
                    this,
                    core::mem::transmute_copy(&frame),
                    core::mem::transmute_copy(&startaddress),
                ) {
                    Ok(ok__) => {
                        returnaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            registerValue: registerValue::<Identity, OFFSET>,
            SetregisterValue: SetregisterValue::<Identity, OFFSET>,
            readMemory: readMemory::<Identity, OFFSET>,
            searchForReturnAddress: searchForReturnAddress::<Identity, OFFSET>,
            searchForReturnAddressStart: searchForReturnAddressStart::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaStackWalkFrame as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaStackWalkFrame {}
windows_core::imp::define_interface!(
    IDiaStackWalkHelper,
    IDiaStackWalkHelper_Vtbl,
    0x21f81b1b_c5bb_42a3_bc4f_ccbaa75b9f19
);
windows_core::imp::interface_hierarchy!(IDiaStackWalkHelper, windows_core::IUnknown);
impl IDiaStackWalkHelper {
    pub unsafe fn registerValue(&self, index: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).registerValue)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetregisterValue(&self, index: u32, newval: u64) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetregisterValue)(
                windows_core::Interface::as_raw(self),
                index,
                newval,
            )
        }
    }
    pub unsafe fn readMemory(
        &self,
        r#type: MemoryTypeEnum,
        va: u64,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).readMemory)(
                windows_core::Interface::as_raw(self),
                r#type,
                va,
                cbdata,
                pcbdata as _,
                pbdata as _,
            )
        }
    }
    pub unsafe fn searchForReturnAddress<P0>(&self, frame: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<IDiaFrameData>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).searchForReturnAddress)(
                windows_core::Interface::as_raw(self),
                frame.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn searchForReturnAddressStart<P0>(
        &self,
        frame: P0,
        startaddress: u64,
    ) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<IDiaFrameData>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).searchForReturnAddressStart)(
                windows_core::Interface::as_raw(self),
                frame.param().abi(),
                startaddress,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn frameForVA(&self, va: u64) -> windows_core::Result<IDiaFrameData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).frameForVA)(
                windows_core::Interface::as_raw(self),
                va,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn symbolForVA(&self, va: u64) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symbolForVA)(
                windows_core::Interface::as_raw(self),
                va,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn pdataForVA(
        &self,
        va: u64,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).pdataForVA)(
                windows_core::Interface::as_raw(self),
                va,
                cbdata,
                pcbdata as _,
                pbdata as _,
            )
        }
    }
    pub unsafe fn imageForVA(&self, vacontext: u64) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).imageForVA)(
                windows_core::Interface::as_raw(self),
                vacontext,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn addressForVA(
        &self,
        va: u64,
        pisect: *mut u32,
        poffset: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).addressForVA)(
                windows_core::Interface::as_raw(self),
                va,
                pisect as _,
                poffset as _,
            )
        }
    }
    pub unsafe fn numberOfFunctionFragmentsForVA(
        &self,
        vafunc: u64,
        cbfunc: u32,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).numberOfFunctionFragmentsForVA)(
                windows_core::Interface::as_raw(self),
                vafunc,
                cbfunc,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn functionFragmentsForVA(
        &self,
        vafunc: u64,
        cbfunc: u32,
        cfragments: u32,
        pvafragment: *mut u64,
        plenfragment: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).functionFragmentsForVA)(
                windows_core::Interface::as_raw(self),
                vafunc,
                cbfunc,
                cfragments,
                pvafragment as _,
                plenfragment as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDiaStackWalkHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub registerValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
    pub SetregisterValue:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64) -> windows_core::HRESULT,
    pub readMemory: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        MemoryTypeEnum,
        u64,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub searchForReturnAddress: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u64,
    ) -> windows_core::HRESULT,
    pub searchForReturnAddressStart: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u64,
        *mut u64,
    ) -> windows_core::HRESULT,
    pub frameForVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub symbolForVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub pdataForVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub imageForVA:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut u64) -> windows_core::HRESULT,
    pub addressForVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub numberOfFunctionFragmentsForVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub functionFragmentsForVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        u32,
        u32,
        *mut u64,
        *mut u32,
    ) -> windows_core::HRESULT,
}
pub trait IDiaStackWalkHelper_Impl: windows_core::IUnknownImpl {
    fn registerValue(&self, index: u32) -> windows_core::Result<u64>;
    fn SetregisterValue(&self, index: u32, newval: u64) -> windows_core::Result<()>;
    fn readMemory(
        &self,
        r#type: MemoryTypeEnum,
        va: u64,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::Result<()>;
    fn searchForReturnAddress(
        &self,
        frame: windows_core::Ref<IDiaFrameData>,
    ) -> windows_core::Result<u64>;
    fn searchForReturnAddressStart(
        &self,
        frame: windows_core::Ref<IDiaFrameData>,
        startaddress: u64,
    ) -> windows_core::Result<u64>;
    fn frameForVA(&self, va: u64) -> windows_core::Result<IDiaFrameData>;
    fn symbolForVA(&self, va: u64) -> windows_core::Result<IDiaSymbol>;
    fn pdataForVA(
        &self,
        va: u64,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::Result<()>;
    fn imageForVA(&self, vacontext: u64) -> windows_core::Result<u64>;
    fn addressForVA(
        &self,
        va: u64,
        pisect: *mut u32,
        poffset: *mut u32,
    ) -> windows_core::Result<()>;
    fn numberOfFunctionFragmentsForVA(&self, vafunc: u64, cbfunc: u32)
    -> windows_core::Result<u32>;
    fn functionFragmentsForVA(
        &self,
        vafunc: u64,
        cbfunc: u32,
        cfragments: u32,
        pvafragment: *mut u64,
        plenfragment: *mut u32,
    ) -> windows_core::Result<()>;
}
impl IDiaStackWalkHelper_Vtbl {
    pub const fn new<Identity: IDiaStackWalkHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn registerValue<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            pretval: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkHelper_Impl::registerValue(
                    this,
                    core::mem::transmute_copy(&index),
                ) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetregisterValue<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            newval: u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaStackWalkHelper_Impl::SetregisterValue(
                    this,
                    core::mem::transmute_copy(&index),
                    core::mem::transmute_copy(&newval),
                )
                .into()
            }
        }
        unsafe extern "system" fn readMemory<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            r#type: MemoryTypeEnum,
            va: u64,
            cbdata: u32,
            pcbdata: *mut u32,
            pbdata: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaStackWalkHelper_Impl::readMemory(
                    this,
                    core::mem::transmute_copy(&r#type),
                    core::mem::transmute_copy(&va),
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pcbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn searchForReturnAddress<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            frame: *mut core::ffi::c_void,
            returnaddress: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkHelper_Impl::searchForReturnAddress(
                    this,
                    core::mem::transmute_copy(&frame),
                ) {
                    Ok(ok__) => {
                        returnaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn searchForReturnAddressStart<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            frame: *mut core::ffi::c_void,
            startaddress: u64,
            returnaddress: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkHelper_Impl::searchForReturnAddressStart(
                    this,
                    core::mem::transmute_copy(&frame),
                    core::mem::transmute_copy(&startaddress),
                ) {
                    Ok(ok__) => {
                        returnaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn frameForVA<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            va: u64,
            ppframe: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkHelper_Impl::frameForVA(this, core::mem::transmute_copy(&va)) {
                    Ok(ok__) => {
                        ppframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn symbolForVA<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            va: u64,
            ppsymbol: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkHelper_Impl::symbolForVA(this, core::mem::transmute_copy(&va)) {
                    Ok(ok__) => {
                        ppsymbol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn pdataForVA<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            va: u64,
            cbdata: u32,
            pcbdata: *mut u32,
            pbdata: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaStackWalkHelper_Impl::pdataForVA(
                    this,
                    core::mem::transmute_copy(&va),
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pcbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn imageForVA<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            vacontext: u64,
            pvaimagestart: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkHelper_Impl::imageForVA(
                    this,
                    core::mem::transmute_copy(&vacontext),
                ) {
                    Ok(ok__) => {
                        pvaimagestart.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn addressForVA<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            va: u64,
            pisect: *mut u32,
            poffset: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaStackWalkHelper_Impl::addressForVA(
                    this,
                    core::mem::transmute_copy(&va),
                    core::mem::transmute_copy(&pisect),
                    core::mem::transmute_copy(&poffset),
                )
                .into()
            }
        }
        unsafe extern "system" fn numberOfFunctionFragmentsForVA<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            vafunc: u64,
            cbfunc: u32,
            pnumfragments: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkHelper_Impl::numberOfFunctionFragmentsForVA(
                    this,
                    core::mem::transmute_copy(&vafunc),
                    core::mem::transmute_copy(&cbfunc),
                ) {
                    Ok(ok__) => {
                        pnumfragments.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn functionFragmentsForVA<
            Identity: IDiaStackWalkHelper_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            vafunc: u64,
            cbfunc: u32,
            cfragments: u32,
            pvafragment: *mut u64,
            plenfragment: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaStackWalkHelper_Impl::functionFragmentsForVA(
                    this,
                    core::mem::transmute_copy(&vafunc),
                    core::mem::transmute_copy(&cbfunc),
                    core::mem::transmute_copy(&cfragments),
                    core::mem::transmute_copy(&pvafragment),
                    core::mem::transmute_copy(&plenfragment),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            registerValue: registerValue::<Identity, OFFSET>,
            SetregisterValue: SetregisterValue::<Identity, OFFSET>,
            readMemory: readMemory::<Identity, OFFSET>,
            searchForReturnAddress: searchForReturnAddress::<Identity, OFFSET>,
            searchForReturnAddressStart: searchForReturnAddressStart::<Identity, OFFSET>,
            frameForVA: frameForVA::<Identity, OFFSET>,
            symbolForVA: symbolForVA::<Identity, OFFSET>,
            pdataForVA: pdataForVA::<Identity, OFFSET>,
            imageForVA: imageForVA::<Identity, OFFSET>,
            addressForVA: addressForVA::<Identity, OFFSET>,
            numberOfFunctionFragmentsForVA: numberOfFunctionFragmentsForVA::<Identity, OFFSET>,
            functionFragmentsForVA: functionFragmentsForVA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaStackWalkHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaStackWalkHelper {}
windows_core::imp::define_interface!(
    IDiaStackWalkHelper2,
    IDiaStackWalkHelper2_Vtbl,
    0x8222c490_507b_4bef_b3bd_41dca7b5934c
);
impl core::ops::Deref for IDiaStackWalkHelper2 {
    type Target = IDiaStackWalkHelper;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaStackWalkHelper2,
    windows_core::IUnknown,
    IDiaStackWalkHelper
);
impl IDiaStackWalkHelper2 {
    pub unsafe fn GetPointerAuthenticationMask(&self, ptrval: u64) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPointerAuthenticationMask)(
                windows_core::Interface::as_raw(self),
                ptrval,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaStackWalkHelper2_Vtbl {
    pub base__: IDiaStackWalkHelper_Vtbl,
    pub GetPointerAuthenticationMask:
        unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut u64) -> windows_core::HRESULT,
}
pub trait IDiaStackWalkHelper2_Impl: IDiaStackWalkHelper_Impl {
    fn GetPointerAuthenticationMask(&self, ptrval: u64) -> windows_core::Result<u64>;
}
impl IDiaStackWalkHelper2_Vtbl {
    pub const fn new<Identity: IDiaStackWalkHelper2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPointerAuthenticationMask<
            Identity: IDiaStackWalkHelper2_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ptrval: u64,
            authmask: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalkHelper2_Impl::GetPointerAuthenticationMask(
                    this,
                    core::mem::transmute_copy(&ptrval),
                ) {
                    Ok(ok__) => {
                        authmask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDiaStackWalkHelper_Vtbl::new::<Identity, OFFSET>(),
            GetPointerAuthenticationMask: GetPointerAuthenticationMask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaStackWalkHelper2 as windows_core::Interface>::IID
            || iid == &<IDiaStackWalkHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaStackWalkHelper2 {}
windows_core::imp::define_interface!(
    IDiaStackWalkHelper3,
    IDiaStackWalkHelper3_Vtbl,
    0x9e8a18e3_4d15_4b9a_9c8e_3f2a5b6c7d8e
);
impl core::ops::Deref for IDiaStackWalkHelper3 {
    type Target = IDiaStackWalkHelper2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaStackWalkHelper3,
    windows_core::IUnknown,
    IDiaStackWalkHelper,
    IDiaStackWalkHelper2
);
impl IDiaStackWalkHelper3 {
    pub unsafe fn get_registerValue(
        &self,
        index: u32,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_registerValue)(
                windows_core::Interface::as_raw(self),
                index,
                cbdata,
                pcbdata as _,
                pbdata as _,
            )
        }
    }
    pub unsafe fn put_registerValue(
        &self,
        index: u32,
        cbdata: u32,
        pbdata: *const u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).put_registerValue)(
                windows_core::Interface::as_raw(self),
                index,
                cbdata,
                pbdata,
            )
        }
    }
}
#[repr(C)]
pub struct IDiaStackWalkHelper3_Vtbl {
    pub base__: IDiaStackWalkHelper2_Vtbl,
    pub get_registerValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub put_registerValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *const u8,
    ) -> windows_core::HRESULT,
}
pub trait IDiaStackWalkHelper3_Impl: IDiaStackWalkHelper2_Impl {
    fn get_registerValue(
        &self,
        index: u32,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::Result<()>;
    fn put_registerValue(
        &self,
        index: u32,
        cbdata: u32,
        pbdata: *const u8,
    ) -> windows_core::Result<()>;
}
impl IDiaStackWalkHelper3_Vtbl {
    pub const fn new<Identity: IDiaStackWalkHelper3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_registerValue<
            Identity: IDiaStackWalkHelper3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            cbdata: u32,
            pcbdata: *mut u32,
            pbdata: *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaStackWalkHelper3_Impl::get_registerValue(
                    this,
                    core::mem::transmute_copy(&index),
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pcbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn put_registerValue<
            Identity: IDiaStackWalkHelper3_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            cbdata: u32,
            pbdata: *const u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiaStackWalkHelper3_Impl::put_registerValue(
                    this,
                    core::mem::transmute_copy(&index),
                    core::mem::transmute_copy(&cbdata),
                    core::mem::transmute_copy(&pbdata),
                )
                .into()
            }
        }
        Self {
            base__: IDiaStackWalkHelper2_Vtbl::new::<Identity, OFFSET>(),
            get_registerValue: get_registerValue::<Identity, OFFSET>,
            put_registerValue: put_registerValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaStackWalkHelper3 as windows_core::Interface>::IID
            || iid == &<IDiaStackWalkHelper as windows_core::Interface>::IID
            || iid == &<IDiaStackWalkHelper2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaStackWalkHelper3 {}
windows_core::imp::define_interface!(
    IDiaStackWalker,
    IDiaStackWalker_Vtbl,
    0x5485216b_a54c_469f_9670_52b24d5229bb
);
windows_core::imp::interface_hierarchy!(IDiaStackWalker, windows_core::IUnknown);
impl IDiaStackWalker {
    pub unsafe fn getEnumFrames<P0>(&self, phelper: P0) -> windows_core::Result<IDiaEnumStackFrames>
    where
        P0: windows_core::Param<IDiaStackWalkHelper>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getEnumFrames)(
                windows_core::Interface::as_raw(self),
                phelper.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getEnumFrames2<P1>(
        &self,
        cpuid: CV_CPU_TYPE_e,
        phelper: P1,
    ) -> windows_core::Result<IDiaEnumStackFrames>
    where
        P1: windows_core::Param<IDiaStackWalkHelper>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getEnumFrames2)(
                windows_core::Interface::as_raw(self),
                cpuid,
                phelper.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaStackWalker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub getEnumFrames: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub getEnumFrames2: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        CV_CPU_TYPE_e,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaStackWalker_Impl: windows_core::IUnknownImpl {
    fn getEnumFrames(
        &self,
        phelper: windows_core::Ref<IDiaStackWalkHelper>,
    ) -> windows_core::Result<IDiaEnumStackFrames>;
    fn getEnumFrames2(
        &self,
        cpuid: CV_CPU_TYPE_e,
        phelper: windows_core::Ref<IDiaStackWalkHelper>,
    ) -> windows_core::Result<IDiaEnumStackFrames>;
}
impl IDiaStackWalker_Vtbl {
    pub const fn new<Identity: IDiaStackWalker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getEnumFrames<
            Identity: IDiaStackWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            phelper: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalker_Impl::getEnumFrames(this, core::mem::transmute_copy(&phelper))
                {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getEnumFrames2<
            Identity: IDiaStackWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cpuid: CV_CPU_TYPE_e,
            phelper: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaStackWalker_Impl::getEnumFrames2(
                    this,
                    core::mem::transmute_copy(&cpuid),
                    core::mem::transmute_copy(&phelper),
                ) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getEnumFrames: getEnumFrames::<Identity, OFFSET>,
            getEnumFrames2: getEnumFrames2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaStackWalker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaStackWalker {}
windows_core::imp::define_interface!(
    IDiaStackWalker2,
    IDiaStackWalker2_Vtbl,
    0x7c185885_a015_4cac_9411_0f4fb39b1f3a
);
impl core::ops::Deref for IDiaStackWalker2 {
    type Target = IDiaStackWalker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiaStackWalker2, windows_core::IUnknown, IDiaStackWalker);
#[repr(C)]
pub struct IDiaStackWalker2_Vtbl {
    pub base__: IDiaStackWalker_Vtbl,
}
pub trait IDiaStackWalker2_Impl: IDiaStackWalker_Impl {}
impl IDiaStackWalker2_Vtbl {
    pub const fn new<Identity: IDiaStackWalker2_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: IDiaStackWalker_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaStackWalker2 as windows_core::Interface>::IID
            || iid == &<IDiaStackWalker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaStackWalker2 {}
windows_core::imp::define_interface!(
    IDiaSymbol,
    IDiaSymbol_Vtbl,
    0xcb787b2f_bd6c_4635_ba52_933126bd2dcd
);
windows_core::imp::interface_hierarchy!(IDiaSymbol, windows_core::IUnknown);
impl IDiaSymbol {
    pub unsafe fn symIndexId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symIndexId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn symTag(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symTag)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).name)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn lexicalParent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lexicalParent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn classParent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).classParent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn r#type(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#type)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn dataKind(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).dataKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn locationType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).locationType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn addressSection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressSection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn addressOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn relativeVirtualAddress(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).relativeVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn registerId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).registerId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn offset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).offset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn length(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn slot(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).slot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn volatileType(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).volatileType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn constType(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).constType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn unalignedType(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).unalignedType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn access(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).access)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn libraryName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).libraryName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn platform(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).platform)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn language(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).language)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn editAndContinueEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).editAndContinueEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn frontEndMajor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).frontEndMajor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn frontEndMinor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).frontEndMinor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn frontEndBuild(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).frontEndBuild)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn backEndMajor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).backEndMajor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn backEndMinor(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).backEndMinor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn backEndBuild(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).backEndBuild)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn sourceFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).sourceFileName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn unused(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).unused)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn thunkOrdinal(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).thunkOrdinal)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn thisAdjust(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).thisAdjust)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualBaseOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualBaseOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn r#virtual(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#virtual)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn intro(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).intro)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn pure(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).pure)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn callingConvention(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).callingConvention)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn baseType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).baseType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn token(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).token)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn timeStamp(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).timeStamp)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn guid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).guid)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn symbolsFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).symbolsFileName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn reference(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).reference)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn bitPosition(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).bitPosition)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn arrayIndexType(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).arrayIndexType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn packed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).packed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn constructor(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).constructor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn overloadedOperator(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).overloadedOperator)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn nested(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nested)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasNestedTypes(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasNestedTypes)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasAssignmentOperator(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasAssignmentOperator)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasCastOperator(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasCastOperator)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn scoped(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).scoped)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualBaseClass(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualBaseClass)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn indirectVirtualBaseClass(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).indirectVirtualBaseClass)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualBasePointerOffset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualBasePointerOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualTableShape(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualTableShape)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn lexicalParentId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lexicalParentId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn classParentId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).classParentId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn typeId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).typeId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn arrayIndexTypeId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).arrayIndexTypeId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualTableShapeId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualTableShapeId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn code(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).code)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn function(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).function)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn managed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).managed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn msil(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).msil)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualBaseDispIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualBaseDispIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn undecoratedName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).undecoratedName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn age(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).age)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn signature(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).signature)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn compilerGenerated(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).compilerGenerated)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn addressTaken(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).addressTaken)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn rank(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).rank)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn lowerBound(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lowerBound)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn upperBound(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).upperBound)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn lowerBoundId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lowerBoundId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn upperBoundId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).upperBoundId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn get_dataBytes(
        &self,
        cbdata: u32,
        pcbdata: *mut u32,
        pbdata: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_dataBytes)(
                windows_core::Interface::as_raw(self),
                cbdata,
                pcbdata as _,
                pbdata as _,
            )
        }
    }
    pub unsafe fn findChildren<P1>(
        &self,
        symtag: SymTagEnum,
        name: P1,
        compareflags: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findChildren)(
                windows_core::Interface::as_raw(self),
                symtag,
                name.param().abi(),
                compareflags,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findChildrenEx<P1>(
        &self,
        symtag: SymTagEnum,
        name: P1,
        compareflags: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findChildrenEx)(
                windows_core::Interface::as_raw(self),
                symtag,
                name.param().abi(),
                compareflags,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findChildrenExByAddr<P1>(
        &self,
        symtag: SymTagEnum,
        name: P1,
        compareflags: u32,
        isect: u32,
        offset: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findChildrenExByAddr)(
                windows_core::Interface::as_raw(self),
                symtag,
                name.param().abi(),
                compareflags,
                isect,
                offset,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findChildrenExByVA<P1>(
        &self,
        symtag: SymTagEnum,
        name: P1,
        compareflags: u32,
        va: u64,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findChildrenExByVA)(
                windows_core::Interface::as_raw(self),
                symtag,
                name.param().abi(),
                compareflags,
                va,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findChildrenExByRVA<P1>(
        &self,
        symtag: SymTagEnum,
        name: P1,
        compareflags: u32,
        rva: u32,
    ) -> windows_core::Result<IDiaEnumSymbols>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findChildrenExByRVA)(
                windows_core::Interface::as_raw(self),
                symtag,
                name.param().abi(),
                compareflags,
                rva,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn targetSection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).targetSection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn targetOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).targetOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn targetRelativeVirtualAddress(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).targetRelativeVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn targetVirtualAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).targetVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn machineType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).machineType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn oemId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).oemId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn oemSymbolId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).oemSymbolId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn get_types(
        &self,
        ctypes: u32,
        pctypes: *mut u32,
        ptypes: *mut Option<Self>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_types)(
                windows_core::Interface::as_raw(self),
                ctypes,
                pctypes as _,
                core::mem::transmute(ptypes),
            )
        }
    }
    pub unsafe fn get_typeIds(
        &self,
        ctypeids: u32,
        pctypeids: *mut u32,
        pdwtypeids: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_typeIds)(
                windows_core::Interface::as_raw(self),
                ctypeids,
                pctypeids as _,
                pdwtypeids as _,
            )
        }
    }
    pub unsafe fn objectPointerType(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).objectPointerType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn udtKind(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).udtKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn get_undecoratedNameEx(
        &self,
        undecorateoptions: u32,
    ) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_undecoratedNameEx)(
                windows_core::Interface::as_raw(self),
                undecorateoptions,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn noReturn(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).noReturn)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn customCallingConvention(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).customCallingConvention)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn noInline(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).noInline)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn optimizedCodeDebugInfo(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).optimizedCodeDebugInfo)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn notReached(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).notReached)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn interruptReturn(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).interruptReturn)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn farReturn(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).farReturn)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isStatic(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isStatic)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasDebugInfo(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasDebugInfo)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isLTCG(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isLTCG)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isDataAligned(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isDataAligned)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasSecurityChecks(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasSecurityChecks)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn compilerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).compilerName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn hasAlloca(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasAlloca)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasSetJump(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasSetJump)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasLongJump(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasLongJump)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasInlAsm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasInlAsm)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasEH(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasEH)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasSEH(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasSEH)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasEHa(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasEHa)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isNaked(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isNaked)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isAggregated(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isAggregated)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isSplitted(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isSplitted)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn container(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).container)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn inlSpec(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).inlSpec)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn noStackOrdering(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).noStackOrdering)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn virtualBaseTableType(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).virtualBaseTableType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn hasManagedCode(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasManagedCode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isHotpatchable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isHotpatchable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isCVTCIL(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isCVTCIL)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isMSILNetmodule(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isMSILNetmodule)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isCTypes(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isCTypes)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isStripped(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isStripped)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn frontEndQFE(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).frontEndQFE)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn backEndQFE(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).backEndQFE)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn wasInlined(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).wasInlined)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn strictGSCheck(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).strictGSCheck)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isCxxReturnUdt(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isCxxReturnUdt)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isConstructorVirtualBase(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isConstructorVirtualBase)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn RValueReference(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RValueReference)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn unmodifiedType(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).unmodifiedType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn framePointerPresent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).framePointerPresent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isSafeBuffers(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isSafeBuffers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn intrinsic(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).intrinsic)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn sealed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).sealed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hfaFloat(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hfaFloat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hfaDouble(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hfaDouble)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn liveRangeStartAddressSection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).liveRangeStartAddressSection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn liveRangeStartAddressOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).liveRangeStartAddressOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn liveRangeStartRelativeVirtualAddress(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).liveRangeStartRelativeVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn countLiveRanges(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).countLiveRanges)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn liveRangeLength(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).liveRangeLength)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn offsetInUdt(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).offsetInUdt)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn paramBasePointerRegisterId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).paramBasePointerRegisterId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn localBasePointerRegisterId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).localBasePointerRegisterId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isLocationControlFlowDependent(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isLocationControlFlowDependent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn stride(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).stride)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn numberOfRows(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).numberOfRows)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn numberOfColumns(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).numberOfColumns)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isMatrixRowMajor(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isMatrixRowMajor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn get_numericProperties(
        &self,
        cnt: u32,
        pcnt: *mut u32,
        pproperties: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_numericProperties)(
                windows_core::Interface::as_raw(self),
                cnt,
                pcnt as _,
                pproperties as _,
            )
        }
    }
    pub unsafe fn get_modifierValues(
        &self,
        cnt: u32,
        pcnt: *mut u32,
        pmodifiers: *mut u16,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_modifierValues)(
                windows_core::Interface::as_raw(self),
                cnt,
                pcnt as _,
                pmodifiers as _,
            )
        }
    }
    pub unsafe fn isReturnValue(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isReturnValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isOptimizedAway(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isOptimizedAway)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn builtInKind(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).builtInKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn registerType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).registerType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn baseDataSlot(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).baseDataSlot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn baseDataOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).baseDataOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn textureSlot(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).textureSlot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn samplerSlot(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).samplerSlot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn uavSlot(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).uavSlot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn sizeInUdt(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).sizeInUdt)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn memorySpaceKind(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).memorySpaceKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn unmodifiedTypeId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).unmodifiedTypeId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn subTypeId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).subTypeId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn subType(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).subType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn numberOfModifiers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).numberOfModifiers)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn numberOfRegisterIndices(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).numberOfRegisterIndices)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isHLSLData(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isHLSLData)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isPointerToDataMember(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isPointerToDataMember)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isPointerToMemberFunction(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isPointerToMemberFunction)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isSingleInheritance(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isSingleInheritance)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isMultipleInheritance(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isMultipleInheritance)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isVirtualInheritance(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isVirtualInheritance)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn restrictedType(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).restrictedType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isPointerBasedOnSymbolValue(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isPointerBasedOnSymbolValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn baseSymbol(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).baseSymbol)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn baseSymbolId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).baseSymbolId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn objectFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).objectFileName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn isAcceleratorGroupSharedLocal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isAcceleratorGroupSharedLocal)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isAcceleratorPointerTagLiveRange(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isAcceleratorPointerTagLiveRange)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isAcceleratorStubFunction(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isAcceleratorStubFunction)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn numberOfAcceleratorPointerTags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).numberOfAcceleratorPointerTags)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isSdl(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isSdl)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isWinRTPointer(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isWinRTPointer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isRefUdt(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isRefUdt)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isValueUdt(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isValueUdt)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isInterfaceUdt(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isInterfaceUdt)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn findInlineFramesByAddr(
        &self,
        isect: u32,
        offset: u32,
    ) -> windows_core::Result<IDiaEnumSymbols> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineFramesByAddr)(
                windows_core::Interface::as_raw(self),
                isect,
                offset,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineFramesByRVA(&self, rva: u32) -> windows_core::Result<IDiaEnumSymbols> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineFramesByRVA)(
                windows_core::Interface::as_raw(self),
                rva,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineFramesByVA(&self, va: u64) -> windows_core::Result<IDiaEnumSymbols> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineFramesByVA)(
                windows_core::Interface::as_raw(self),
                va,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineeLines(&self) -> windows_core::Result<IDiaEnumLineNumbers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineeLines)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineeLinesByAddr(
        &self,
        isect: u32,
        offset: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineeLinesByAddr)(
                windows_core::Interface::as_raw(self),
                isect,
                offset,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineeLinesByRVA(
        &self,
        rva: u32,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineeLinesByRVA)(
                windows_core::Interface::as_raw(self),
                rva,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findInlineeLinesByVA(
        &self,
        va: u64,
        length: u32,
    ) -> windows_core::Result<IDiaEnumLineNumbers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInlineeLinesByVA)(
                windows_core::Interface::as_raw(self),
                va,
                length,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findSymbolsForAcceleratorPointerTag(
        &self,
        tagvalue: u32,
    ) -> windows_core::Result<IDiaEnumSymbols> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findSymbolsForAcceleratorPointerTag)(
                windows_core::Interface::as_raw(self),
                tagvalue,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn findSymbolsByRVAForAcceleratorPointerTag(
        &self,
        tagvalue: u32,
        rva: u32,
    ) -> windows_core::Result<IDiaEnumSymbols> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findSymbolsByRVAForAcceleratorPointerTag)(
                windows_core::Interface::as_raw(self),
                tagvalue,
                rva,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_acceleratorPointerTags(
        &self,
        cnt: u32,
        pcnt: *mut u32,
        ppointertags: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_acceleratorPointerTags)(
                windows_core::Interface::as_raw(self),
                cnt,
                pcnt as _,
                ppointertags as _,
            )
        }
    }
    pub unsafe fn getSrcLineOnTypeDefn(&self) -> windows_core::Result<IDiaLineNumber> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getSrcLineOnTypeDefn)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn isPGO(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isPGO)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn hasValidPGOCounts(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasValidPGOCounts)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isOptimizedForSpeed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isOptimizedForSpeed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn PGOEntryCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PGOEntryCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn PGOEdgeCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PGOEdgeCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn PGODynamicInstructionCount(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PGODynamicInstructionCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn staticSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).staticSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn finalLiveStaticSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).finalLiveStaticSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn phaseName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).phaseName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn hasControlFlowCheck(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasControlFlowCheck)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn constantExport(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).constantExport)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn dataExport(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).dataExport)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn privateExport(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).privateExport)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn noNameExport(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).noNameExport)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn exportHasExplicitlyAssignedOrdinal(
        &self,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).exportHasExplicitlyAssignedOrdinal)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn exportIsForwarder(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).exportIsForwarder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn ordinal(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ordinal)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn frameSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).frameSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn exceptionHandlerAddressSection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).exceptionHandlerAddressSection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn exceptionHandlerAddressOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).exceptionHandlerAddressOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn exceptionHandlerRelativeVirtualAddress(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).exceptionHandlerRelativeVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn exceptionHandlerVirtualAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).exceptionHandlerVirtualAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn findInputAssemblyFile(&self) -> windows_core::Result<IDiaInputAssemblyFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).findInputAssemblyFile)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn characteristics(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).characteristics)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn coffGroup(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).coffGroup)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn bindID(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).bindID)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn bindSpace(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).bindSpace)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn bindSlot(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).bindSlot)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub symIndexId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub symTag:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub lexicalParent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub classParent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub r#type: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub dataKind:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub locationType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub addressSection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub addressOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub relativeVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub virtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub registerId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub offset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub length:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub slot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub volatileType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub constType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub unalignedType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub access:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub libraryName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub platform:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub language:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub editAndContinueEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub frontEndMajor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub frontEndMinor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub frontEndBuild:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub backEndMajor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub backEndMinor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub backEndBuild:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub sourceFileName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub unused: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub thunkOrdinal:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub thisAdjust:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub virtualBaseOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub r#virtual: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub intro: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub pure: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub callingConvention:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    value: usize,
    pub baseType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub token: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub timeStamp:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub guid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub symbolsFileName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub reference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub bitPosition:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub arrayIndexType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub packed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub constructor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub overloadedOperator: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub nested: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasNestedTypes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasAssignmentOperator: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasCastOperator: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub scoped: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub virtualBaseClass: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub indirectVirtualBaseClass: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub virtualBasePointerOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub virtualTableShape: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub lexicalParentId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub classParentId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub typeId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub arrayIndexTypeId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub virtualTableShapeId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub code: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub function: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub managed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub msil: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub virtualBaseDispIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub undecoratedName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub age: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub signature:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub compilerGenerated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub addressTaken: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub rank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub lowerBound: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub upperBound: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub lowerBoundId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub upperBoundId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub get_dataBytes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
    pub findChildren: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SymTagEnum,
        windows_core::PCWSTR,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findChildrenEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SymTagEnum,
        windows_core::PCWSTR,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findChildrenExByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SymTagEnum,
        windows_core::PCWSTR,
        u32,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findChildrenExByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SymTagEnum,
        windows_core::PCWSTR,
        u32,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findChildrenExByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SymTagEnum,
        windows_core::PCWSTR,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub targetSection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub targetOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub targetRelativeVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub targetVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub machineType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub oemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub oemSymbolId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub get_types: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub get_typeIds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub objectPointerType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub udtKind:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub get_undecoratedNameEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub noReturn: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub customCallingConvention: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub noInline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub optimizedCodeDebugInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub notReached: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub interruptReturn: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub farReturn: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isStatic: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasDebugInfo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isLTCG: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isDataAligned: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasSecurityChecks: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub compilerName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub hasAlloca: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasSetJump: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasLongJump: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasInlAsm: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasEH: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasSEH: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasEHa: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isNaked: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isAggregated: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isSplitted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub container: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub inlSpec: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub noStackOrdering: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub virtualBaseTableType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub hasManagedCode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isHotpatchable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isCVTCIL: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isMSILNetmodule: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isCTypes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isStripped: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub frontEndQFE:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub backEndQFE:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub wasInlined: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub strictGSCheck: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isCxxReturnUdt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isConstructorVirtualBase: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub RValueReference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub unmodifiedType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub framePointerPresent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isSafeBuffers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub intrinsic: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub sealed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hfaFloat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hfaDouble: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub liveRangeStartAddressSection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub liveRangeStartAddressOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub liveRangeStartRelativeVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub countLiveRanges:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub liveRangeLength:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub offsetInUdt:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub paramBasePointerRegisterId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub localBasePointerRegisterId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub isLocationControlFlowDependent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub stride:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub numberOfRows:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub numberOfColumns:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub isMatrixRowMajor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub get_numericProperties: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub get_modifierValues: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u16,
    ) -> windows_core::HRESULT,
    pub isReturnValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isOptimizedAway: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub builtInKind:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub registerType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub baseDataSlot:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub baseDataOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub textureSlot:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub samplerSlot:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub uavSlot:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub sizeInUdt:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub memorySpaceKind:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub unmodifiedTypeId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub subTypeId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub subType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub numberOfModifiers:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub numberOfRegisterIndices:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub isHLSLData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isPointerToDataMember: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isPointerToMemberFunction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isSingleInheritance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isMultipleInheritance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isVirtualInheritance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub restrictedType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isPointerBasedOnSymbolValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub baseSymbol: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub baseSymbolId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub objectFileName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub isAcceleratorGroupSharedLocal: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isAcceleratorPointerTagLiveRange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isAcceleratorStubFunction: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub numberOfAcceleratorPointerTags:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub isSdl: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isWinRTPointer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isRefUdt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isValueUdt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isInterfaceUdt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub findInlineFramesByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineFramesByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineFramesByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineeLines: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineeLinesByAddr: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineeLinesByRVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findInlineeLinesByVA: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub findSymbolsForAcceleratorPointerTag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub findSymbolsByRVAForAcceleratorPointerTag:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            u32,
            u32,
            *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
    pub get_acceleratorPointerTags: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub getSrcLineOnTypeDefn: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub isPGO: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub hasValidPGOCounts: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isOptimizedForSpeed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub PGOEntryCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PGOEdgeCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PGODynamicInstructionCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub staticSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub finalLiveStaticSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub phaseName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub hasControlFlowCheck: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub constantExport: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub dataExport: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub privateExport: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub noNameExport: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub exportHasExplicitlyAssignedOrdinal: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub exportIsForwarder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub ordinal:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub frameSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub exceptionHandlerAddressSection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub exceptionHandlerAddressOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub exceptionHandlerRelativeVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub exceptionHandlerVirtualAddress:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub findInputAssemblyFile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub characteristics:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub coffGroup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub bindID:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub bindSpace:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub bindSlot:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol {}
windows_core::imp::define_interface!(
    IDiaSymbol10,
    IDiaSymbol10_Vtbl,
    0x9034a70b_b0b7_4605_8a97_33772f3a7b8c
);
impl core::ops::Deref for IDiaSymbol10 {
    type Target = IDiaSymbol9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaSymbol10,
    windows_core::IUnknown,
    IDiaSymbol,
    IDiaSymbol2,
    IDiaSymbol3,
    IDiaSymbol4,
    IDiaSymbol5,
    IDiaSymbol6,
    IDiaSymbol7,
    IDiaSymbol8,
    IDiaSymbol9
);
impl IDiaSymbol10 {
    pub unsafe fn get_sourceLink(
        &self,
        cb: u32,
        pcb: *mut u32,
        pb: *mut u8,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_sourceLink)(
                windows_core::Interface::as_raw(self),
                cb,
                pcb as _,
                pb as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol10_Vtbl {
    pub base__: IDiaSymbol9_Vtbl,
    pub get_sourceLink: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut u8,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol10 {}
windows_core::imp::define_interface!(
    IDiaSymbol11,
    IDiaSymbol11_Vtbl,
    0xb6f54fcd_05e3_433d_b305_b0c1437d2d16
);
impl core::ops::Deref for IDiaSymbol11 {
    type Target = IDiaSymbol10;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaSymbol11,
    windows_core::IUnknown,
    IDiaSymbol,
    IDiaSymbol2,
    IDiaSymbol3,
    IDiaSymbol4,
    IDiaSymbol5,
    IDiaSymbol6,
    IDiaSymbol7,
    IDiaSymbol8,
    IDiaSymbol9,
    IDiaSymbol10
);
impl IDiaSymbol11 {
    pub unsafe fn get_discriminatedUnionTag(
        &self,
        pptagtype: *mut Option<IDiaSymbol>,
        ptagoffset: *mut u32,
        ptagmask: *mut DiaTagValue,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_discriminatedUnionTag)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pptagtype),
                ptagoffset as _,
                ptagmask as _,
            )
        }
    }
    pub unsafe fn get_tagRanges(
        &self,
        count: u32,
        pcrangevalues: *mut u32,
        rangevalues: *mut DiaTagValue,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).get_tagRanges)(
                windows_core::Interface::as_raw(self),
                count,
                pcrangevalues as _,
                rangevalues as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol11_Vtbl {
    pub base__: IDiaSymbol10_Vtbl,
    pub get_discriminatedUnionTag: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
        *mut DiaTagValue,
    ) -> windows_core::HRESULT,
    pub get_tagRanges: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut DiaTagValue,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol11 {}
windows_core::imp::define_interface!(
    IDiaSymbol12,
    IDiaSymbol12_Vtbl,
    0xd4d55c6b_2c67_4c1a_b8f2_7e8f3d5e2a1d
);
impl core::ops::Deref for IDiaSymbol12 {
    type Target = IDiaSymbol11;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaSymbol12,
    windows_core::IUnknown,
    IDiaSymbol,
    IDiaSymbol2,
    IDiaSymbol3,
    IDiaSymbol4,
    IDiaSymbol5,
    IDiaSymbol6,
    IDiaSymbol7,
    IDiaSymbol8,
    IDiaSymbol9,
    IDiaSymbol10,
    IDiaSymbol11
);
impl IDiaSymbol12 {
    pub unsafe fn scalableRegisterType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).scalableRegisterType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn atomicType(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).atomicType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol12_Vtbl {
    pub base__: IDiaSymbol11_Vtbl,
    pub scalableRegisterType:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub atomicType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol12 {}
windows_core::imp::define_interface!(
    IDiaSymbol2,
    IDiaSymbol2_Vtbl,
    0x611e86cd_b7d1_4546_8a15_070e2b07a427
);
impl core::ops::Deref for IDiaSymbol2 {
    type Target = IDiaSymbol;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiaSymbol2, windows_core::IUnknown, IDiaSymbol);
impl IDiaSymbol2 {
    pub unsafe fn isObjCClass(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isObjCClass)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isObjCCategory(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isObjCCategory)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isObjCProtocol(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isObjCProtocol)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol2_Vtbl {
    pub base__: IDiaSymbol_Vtbl,
    pub isObjCClass: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isObjCCategory: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub isObjCProtocol: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol2 {}
windows_core::imp::define_interface!(
    IDiaSymbol3,
    IDiaSymbol3_Vtbl,
    0x99b665f7_c1b2_49d3_89b2_a384361acab5
);
impl core::ops::Deref for IDiaSymbol3 {
    type Target = IDiaSymbol2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaSymbol3,
    windows_core::IUnknown,
    IDiaSymbol,
    IDiaSymbol2
);
impl IDiaSymbol3 {
    pub unsafe fn inlinee(&self) -> windows_core::Result<IDiaSymbol> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).inlinee)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn inlineeId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).inlineeId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol3_Vtbl {
    pub base__: IDiaSymbol2_Vtbl,
    pub inlinee: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub inlineeId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol3 {}
windows_core::imp::define_interface!(
    IDiaSymbol4,
    IDiaSymbol4_Vtbl,
    0xbf6c88a7_e9d6_4346_99a1_d053de5a7808
);
impl core::ops::Deref for IDiaSymbol4 {
    type Target = IDiaSymbol3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaSymbol4,
    windows_core::IUnknown,
    IDiaSymbol,
    IDiaSymbol2,
    IDiaSymbol3
);
impl IDiaSymbol4 {
    pub unsafe fn noexcept(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).noexcept)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol4_Vtbl {
    pub base__: IDiaSymbol3_Vtbl,
    pub noexcept: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol4 {}
windows_core::imp::define_interface!(
    IDiaSymbol5,
    IDiaSymbol5_Vtbl,
    0xabe2de00_dc2d_4793_af9a_ef1d90832644
);
impl core::ops::Deref for IDiaSymbol5 {
    type Target = IDiaSymbol4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaSymbol5,
    windows_core::IUnknown,
    IDiaSymbol,
    IDiaSymbol2,
    IDiaSymbol3,
    IDiaSymbol4
);
impl IDiaSymbol5 {
    pub unsafe fn hasAbsoluteAddress(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasAbsoluteAddress)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol5_Vtbl {
    pub base__: IDiaSymbol4_Vtbl,
    pub hasAbsoluteAddress: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol5 {}
windows_core::imp::define_interface!(
    IDiaSymbol6,
    IDiaSymbol6_Vtbl,
    0x8133dad3_75fe_4234_ac7e_f8e7a1d3cbb3
);
impl core::ops::Deref for IDiaSymbol6 {
    type Target = IDiaSymbol5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaSymbol6,
    windows_core::IUnknown,
    IDiaSymbol,
    IDiaSymbol2,
    IDiaSymbol3,
    IDiaSymbol4,
    IDiaSymbol5
);
impl IDiaSymbol6 {
    pub unsafe fn isStaticMemberFunc(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isStaticMemberFunc)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol6_Vtbl {
    pub base__: IDiaSymbol5_Vtbl,
    pub isStaticMemberFunc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol6 {}
windows_core::imp::define_interface!(
    IDiaSymbol7,
    IDiaSymbol7_Vtbl,
    0x64ce6cd5_7315_4328_86d6_10e303e010b4
);
impl core::ops::Deref for IDiaSymbol7 {
    type Target = IDiaSymbol6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaSymbol7,
    windows_core::IUnknown,
    IDiaSymbol,
    IDiaSymbol2,
    IDiaSymbol3,
    IDiaSymbol4,
    IDiaSymbol5,
    IDiaSymbol6
);
impl IDiaSymbol7 {
    pub unsafe fn isSignRet(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isSignRet)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol7_Vtbl {
    pub base__: IDiaSymbol6_Vtbl,
    pub isSignRet: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol7 {}
windows_core::imp::define_interface!(
    IDiaSymbol8,
    IDiaSymbol8_Vtbl,
    0x7f2e041f_1294_41bd_b83a_e715972d2ce3
);
impl core::ops::Deref for IDiaSymbol8 {
    type Target = IDiaSymbol7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaSymbol8,
    windows_core::IUnknown,
    IDiaSymbol,
    IDiaSymbol2,
    IDiaSymbol3,
    IDiaSymbol4,
    IDiaSymbol5,
    IDiaSymbol6,
    IDiaSymbol7
);
impl IDiaSymbol8 {
    pub unsafe fn coroutineKind(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).coroutineKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn associatedSymbolKind(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).associatedSymbolKind)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn associatedSymbolSection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).associatedSymbolSection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn associatedSymbolOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).associatedSymbolOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn associatedSymbolRva(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).associatedSymbolRva)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn associatedSymbolAddr(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).associatedSymbolAddr)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol8_Vtbl {
    pub base__: IDiaSymbol7_Vtbl,
    pub coroutineKind:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub associatedSymbolKind:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub associatedSymbolSection:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub associatedSymbolOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub associatedSymbolRva:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub associatedSymbolAddr:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol8 {}
windows_core::imp::define_interface!(
    IDiaSymbol9,
    IDiaSymbol9_Vtbl,
    0xa89e5969_92a1_4f8a_b704_00121c37abbb
);
impl core::ops::Deref for IDiaSymbol9 {
    type Target = IDiaSymbol8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDiaSymbol9,
    windows_core::IUnknown,
    IDiaSymbol,
    IDiaSymbol2,
    IDiaSymbol3,
    IDiaSymbol4,
    IDiaSymbol5,
    IDiaSymbol6,
    IDiaSymbol7,
    IDiaSymbol8
);
impl IDiaSymbol9 {
    pub unsafe fn framePadSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).framePadSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn framePadOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).framePadOffset)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn isRTCs(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isRTCs)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDiaSymbol9_Vtbl {
    pub base__: IDiaSymbol8_Vtbl,
    pub framePadSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub framePadOffset:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub isRTCs: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IDiaSymbol9 {}
windows_core::imp::define_interface!(
    IDiaTable,
    IDiaTable_Vtbl,
    0x4a59fb77_abac_469b_a30b_9ecc85bfef14
);
impl core::ops::Deref for IDiaTable {
    type Target = IEnumUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDiaTable, windows_core::IUnknown, IEnumUnknown);
impl IDiaTable {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).name)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Item(&self, index: u32) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDiaTable_Vtbl {
    pub base__: IEnumUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDiaTable_Impl: IEnumUnknown_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: u32) -> windows_core::Result<windows_core::IUnknown>;
}
impl IDiaTable_Vtbl {
    pub const fn new<Identity: IDiaTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IDiaTable_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaTable_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn name<Identity: IDiaTable_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaTable_Impl::name(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IDiaTable_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pretval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaTable_Impl::Count(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IDiaTable_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            index: u32,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiaTable_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IEnumUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            name: name::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiaTable as windows_core::Interface>::IID
            || iid == &<IEnumUnknown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDiaTable {}
windows_core::imp::define_interface!(
    IEnumUnknown,
    IEnumUnknown_Vtbl,
    0x00000100_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IEnumUnknown, windows_core::IUnknown);
impl IEnumUnknown {
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<windows_core::IUnknown>,
        pceltfetched: Option<*mut u32>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                core::mem::transmute(rgelt),
                pceltfetched.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IEnumUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IEnumUnknown_Impl: windows_core::IUnknownImpl {
    fn Next(
        &self,
        celt: u32,
        rgelt: *mut Option<windows_core::IUnknown>,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumUnknown>;
}
impl IEnumUnknown_Vtbl {
    pub const fn new<Identity: IEnumUnknown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumUnknown_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut *mut core::ffi::c_void,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumUnknown_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumUnknown_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumUnknown_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumUnknown_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumUnknown_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumUnknown_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumUnknown_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumUnknown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumUnknown {}
pub const LOAD_WITH_ALTERED_SEARCH_PATH: i32 = 8;
pub type LPCOLESTR = *const OLECHAR;
pub const LocInMetaData: LocationType = 9;
pub const LocIsBitField: LocationType = 6;
pub const LocIsConstant: LocationType = 10;
pub const LocIsEnregistered: LocationType = 5;
pub const LocIsIlRel: LocationType = 8;
pub const LocIsNull: LocationType = 0;
pub const LocIsRegRel: LocationType = 3;
pub const LocIsRegRelAliasIndir: LocationType = 11;
pub const LocIsSlot: LocationType = 7;
pub const LocIsStatic: LocationType = 1;
pub const LocIsTLS: LocationType = 2;
pub const LocIsThisRel: LocationType = 4;
pub const LocTypeMax: LocationType = 12;
pub type LocationType = i32;
pub const MemTypeAny: MemoryTypeEnum = -1;
pub const MemTypeCode: MemoryTypeEnum = 0;
pub const MemTypeCodeOnHeap: MemoryTypeEnum = 3;
pub const MemTypeData: MemoryTypeEnum = 1;
pub const MemTypeStack: MemoryTypeEnum = 2;
pub type MemoryTypeEnum = i32;
pub const NAMEHASH_BUILD_COMPLETE: i32 = 3;
pub const NAMEHASH_BUILD_ERROR: i32 = 4;
pub const NAMEHASH_BUILD_FAIL_TO_OPEN_MOD: i32 = 5;
pub const NAMEHASH_BUILD_OOM: i32 = 4;
pub const NAMEHASH_BUILD_PAUSE: i32 = 1;
pub const NAMEHASH_BUILD_RESUME: i32 = 2;
pub const NAMEHASH_BUILD_START: i32 = 0;
pub type NameSearchOptions = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OLECHAR(pub u16);
pub type PFNMINIPDBERRORCALLBACK2 = Option<
    unsafe extern "C" fn(
        pvcontext: *mut core::ffi::c_void,
        dwerrorcode: u32,
        szobjorpdb: windows_core::PCWSTR,
        szlib: windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
>;
pub type PfnPDBDebugDirV =
    Option<unsafe extern "C" fn(param0: windows_core::BOOL, param1: *mut core::ffi::c_void)>;
pub const SVE_NONE: ScalableVectorType = 0;
pub const SVE_P: ScalableVectorType = 2;
pub const SVE_Z: ScalableVectorType = 1;
pub type ScalableVectorType = i32;
pub type StackFrameTypeEnum = i32;
pub const SymTagAnnotation: SymTagEnum = 8;
pub const SymTagArrayType: SymTagEnum = 15;
pub const SymTagBaseClass: SymTagEnum = 18;
pub const SymTagBaseInterface: SymTagEnum = 33;
pub const SymTagBaseType: SymTagEnum = 16;
pub const SymTagBlock: SymTagEnum = 6;
pub const SymTagCallSite: SymTagEnum = 31;
pub const SymTagCallee: SymTagEnum = 38;
pub const SymTagCaller: SymTagEnum = 37;
pub const SymTagCoffGroup: SymTagEnum = 41;
pub const SymTagCompiland: SymTagEnum = 2;
pub const SymTagCompilandDetails: SymTagEnum = 3;
pub const SymTagCompilandEnv: SymTagEnum = 4;
pub const SymTagCustom: SymTagEnum = 26;
pub const SymTagCustomType: SymTagEnum = 28;
pub const SymTagData: SymTagEnum = 7;
pub const SymTagDimension: SymTagEnum = 30;
pub type SymTagEnum = i32;
pub const SymTagEnum_: SymTagEnum = 12;
pub const SymTagExe: SymTagEnum = 1;
pub const SymTagExport: SymTagEnum = 39;
pub const SymTagFriend: SymTagEnum = 19;
pub const SymTagFuncDebugEnd: SymTagEnum = 22;
pub const SymTagFuncDebugStart: SymTagEnum = 21;
pub const SymTagFunction: SymTagEnum = 5;
pub const SymTagFunctionArgType: SymTagEnum = 20;
pub const SymTagFunctionType: SymTagEnum = 13;
pub const SymTagHLSLType: SymTagEnum = 36;
pub const SymTagHeapAllocationSite: SymTagEnum = 40;
pub const SymTagInlineSite: SymTagEnum = 32;
pub const SymTagInlinee: SymTagEnum = 42;
pub const SymTagLabel: SymTagEnum = 9;
pub const SymTagManagedType: SymTagEnum = 29;
pub const SymTagMatrixType: SymTagEnum = 35;
pub const SymTagMax: SymTagEnum = 44;
pub const SymTagNull: SymTagEnum = 0;
pub const SymTagPointerType: SymTagEnum = 14;
pub const SymTagPublicSymbol: SymTagEnum = 10;
pub const SymTagTaggedUnionCase: SymTagEnum = 43;
pub const SymTagThunk: SymTagEnum = 27;
pub const SymTagTypedef: SymTagEnum = 17;
pub const SymTagUDT: SymTagEnum = 11;
pub const SymTagUsingNamespace: SymTagEnum = 23;
pub const SymTagVTable: SymTagEnum = 25;
pub const SymTagVTableShape: SymTagEnum = 24;
pub const SymTagVectorType: SymTagEnum = 34;
pub type THUNK_ORDINAL = i32;
pub const THUNK_ORDINAL_ADJUSTOR: THUNK_ORDINAL = 1;
pub const THUNK_ORDINAL_LOAD: THUNK_ORDINAL = 4;
pub const THUNK_ORDINAL_NOTYPE: THUNK_ORDINAL = 0;
pub const THUNK_ORDINAL_PCODE: THUNK_ORDINAL = 3;
pub const THUNK_ORDINAL_TRAMP_ARM64XSAMEADDRESS: THUNK_ORDINAL = 8;
pub const THUNK_ORDINAL_TRAMP_BRANCHISLAND: THUNK_ORDINAL = 6;
pub const THUNK_ORDINAL_TRAMP_FUNCOVERRIDING: THUNK_ORDINAL = 9;
pub const THUNK_ORDINAL_TRAMP_INCREMENTAL: THUNK_ORDINAL = 5;
pub const THUNK_ORDINAL_TRAMP_STRICTICF: THUNK_ORDINAL = 7;
pub const THUNK_ORDINAL_VCALL: THUNK_ORDINAL = 2;
pub const UdtClass: UdtKind = 1;
pub const UdtInterface: UdtKind = 3;
pub type UdtKind = i32;
pub const UdtStruct: UdtKind = 0;
pub const UdtTaggedUnion: UdtKind = 4;
pub const UdtUnion: UdtKind = 2;
pub const btBCD: BasicType = 9;
pub const btBSTR: BasicType = 30;
pub const btBit: BasicType = 29;
pub const btBool: BasicType = 10;
pub const btChar: BasicType = 2;
pub const btChar16: BasicType = 32;
pub const btChar32: BasicType = 33;
pub const btChar8: BasicType = 34;
pub const btComplex: BasicType = 28;
pub const btCurrency: BasicType = 25;
pub const btDate: BasicType = 26;
pub const btFloat: BasicType = 8;
pub const btHresult: BasicType = 31;
pub const btInt: BasicType = 6;
pub const btLong: BasicType = 13;
pub const btNoType: BasicType = 0;
pub const btUInt: BasicType = 7;
pub const btULong: BasicType = 14;
pub const btVariant: BasicType = 27;
pub const btVector: BasicType = 35;
pub const btVoid: BasicType = 1;
pub const btWChar: BasicType = 3;
pub const nsCaseInRegularExpression: NameSearchOptions = 10;
pub const nsCaseInsensitive: NameSearchOptions = 2;
pub const nsCaseSensitive: NameSearchOptions = 1;
pub const nsFNameExt: NameSearchOptions = 6;
pub const nsNone: NameSearchOptions = 0;
pub const nsRegularExpression: NameSearchOptions = 9;
pub const nsfCaseInsensitive: NameSearchOptions = 2;
pub const nsfCaseSensitive: NameSearchOptions = 1;
pub const nsfFNameExt: NameSearchOptions = 4;
pub const nsfRegularExpression: NameSearchOptions = 8;
pub const nsfUndecoratedName: NameSearchOptions = 16;
