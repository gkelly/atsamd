#[doc = "Reader of register CLKCTRL"]
pub type R = crate::R<u16, super::CLKCTRL>;
#[doc = "Writer for register CLKCTRL"]
pub type W = crate::W<u16, super::CLKCTRL>;
#[doc = "Register CLKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Generic Clock Selection ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ID_A {
    #[doc = "0: DFLL48"]
    DFLL48,
    #[doc = "1: FDPLL"]
    FDPLL,
    #[doc = "2: FDPLL32K"]
    FDPLL32K,
    #[doc = "3: WDT"]
    WDT,
    #[doc = "4: RTC"]
    RTC,
    #[doc = "5: EIC"]
    EIC,
    #[doc = "6: USB"]
    USB,
    #[doc = "7: EVSYS_0"]
    EVSYS_0,
    #[doc = "8: EVSYS_1"]
    EVSYS_1,
    #[doc = "9: EVSYS_2"]
    EVSYS_2,
    #[doc = "10: EVSYS_3"]
    EVSYS_3,
    #[doc = "11: EVSYS_4"]
    EVSYS_4,
    #[doc = "12: EVSYS_5"]
    EVSYS_5,
    #[doc = "13: SERCOMX_SLOW"]
    SERCOMX_SLOW,
    #[doc = "14: SERCOM0_CORE"]
    SERCOM0_CORE,
    #[doc = "15: SERCOM1_CORE"]
    SERCOM1_CORE,
    #[doc = "16: SERCOM2_CORE"]
    SERCOM2_CORE,
    #[doc = "17: TCC0"]
    TCC0,
    #[doc = "18: TC1_TC2"]
    TC1_TC2,
    #[doc = "19: ADC"]
    ADC,
    #[doc = "20: AC_DIG"]
    AC_DIG,
    #[doc = "21: AC_ANA"]
    AC_ANA,
    #[doc = "22: DAC"]
    DAC,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        match variant {
            ID_A::DFLL48 => 0,
            ID_A::FDPLL => 1,
            ID_A::FDPLL32K => 2,
            ID_A::WDT => 3,
            ID_A::RTC => 4,
            ID_A::EIC => 5,
            ID_A::USB => 6,
            ID_A::EVSYS_0 => 7,
            ID_A::EVSYS_1 => 8,
            ID_A::EVSYS_2 => 9,
            ID_A::EVSYS_3 => 10,
            ID_A::EVSYS_4 => 11,
            ID_A::EVSYS_5 => 12,
            ID_A::SERCOMX_SLOW => 13,
            ID_A::SERCOM0_CORE => 14,
            ID_A::SERCOM1_CORE => 15,
            ID_A::SERCOM2_CORE => 16,
            ID_A::TCC0 => 17,
            ID_A::TC1_TC2 => 18,
            ID_A::ADC => 19,
            ID_A::AC_DIG => 20,
            ID_A::AC_ANA => 21,
            ID_A::DAC => 22,
        }
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, ID_A>;
impl ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ID_A::DFLL48),
            1 => Val(ID_A::FDPLL),
            2 => Val(ID_A::FDPLL32K),
            3 => Val(ID_A::WDT),
            4 => Val(ID_A::RTC),
            5 => Val(ID_A::EIC),
            6 => Val(ID_A::USB),
            7 => Val(ID_A::EVSYS_0),
            8 => Val(ID_A::EVSYS_1),
            9 => Val(ID_A::EVSYS_2),
            10 => Val(ID_A::EVSYS_3),
            11 => Val(ID_A::EVSYS_4),
            12 => Val(ID_A::EVSYS_5),
            13 => Val(ID_A::SERCOMX_SLOW),
            14 => Val(ID_A::SERCOM0_CORE),
            15 => Val(ID_A::SERCOM1_CORE),
            16 => Val(ID_A::SERCOM2_CORE),
            17 => Val(ID_A::TCC0),
            18 => Val(ID_A::TC1_TC2),
            19 => Val(ID_A::ADC),
            20 => Val(ID_A::AC_DIG),
            21 => Val(ID_A::AC_ANA),
            22 => Val(ID_A::DAC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DFLL48`"]
    #[inline(always)]
    pub fn is_dfll48(&self) -> bool {
        *self == ID_A::DFLL48
    }
    #[doc = "Checks if the value of the field is `FDPLL`"]
    #[inline(always)]
    pub fn is_fdpll(&self) -> bool {
        *self == ID_A::FDPLL
    }
    #[doc = "Checks if the value of the field is `FDPLL32K`"]
    #[inline(always)]
    pub fn is_fdpll32k(&self) -> bool {
        *self == ID_A::FDPLL32K
    }
    #[doc = "Checks if the value of the field is `WDT`"]
    #[inline(always)]
    pub fn is_wdt(&self) -> bool {
        *self == ID_A::WDT
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == ID_A::RTC
    }
    #[doc = "Checks if the value of the field is `EIC`"]
    #[inline(always)]
    pub fn is_eic(&self) -> bool {
        *self == ID_A::EIC
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == ID_A::USB
    }
    #[doc = "Checks if the value of the field is `EVSYS_0`"]
    #[inline(always)]
    pub fn is_evsys_0(&self) -> bool {
        *self == ID_A::EVSYS_0
    }
    #[doc = "Checks if the value of the field is `EVSYS_1`"]
    #[inline(always)]
    pub fn is_evsys_1(&self) -> bool {
        *self == ID_A::EVSYS_1
    }
    #[doc = "Checks if the value of the field is `EVSYS_2`"]
    #[inline(always)]
    pub fn is_evsys_2(&self) -> bool {
        *self == ID_A::EVSYS_2
    }
    #[doc = "Checks if the value of the field is `EVSYS_3`"]
    #[inline(always)]
    pub fn is_evsys_3(&self) -> bool {
        *self == ID_A::EVSYS_3
    }
    #[doc = "Checks if the value of the field is `EVSYS_4`"]
    #[inline(always)]
    pub fn is_evsys_4(&self) -> bool {
        *self == ID_A::EVSYS_4
    }
    #[doc = "Checks if the value of the field is `EVSYS_5`"]
    #[inline(always)]
    pub fn is_evsys_5(&self) -> bool {
        *self == ID_A::EVSYS_5
    }
    #[doc = "Checks if the value of the field is `SERCOMX_SLOW`"]
    #[inline(always)]
    pub fn is_sercomx_slow(&self) -> bool {
        *self == ID_A::SERCOMX_SLOW
    }
    #[doc = "Checks if the value of the field is `SERCOM0_CORE`"]
    #[inline(always)]
    pub fn is_sercom0_core(&self) -> bool {
        *self == ID_A::SERCOM0_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM1_CORE`"]
    #[inline(always)]
    pub fn is_sercom1_core(&self) -> bool {
        *self == ID_A::SERCOM1_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM2_CORE`"]
    #[inline(always)]
    pub fn is_sercom2_core(&self) -> bool {
        *self == ID_A::SERCOM2_CORE
    }
    #[doc = "Checks if the value of the field is `TCC0`"]
    #[inline(always)]
    pub fn is_tcc0(&self) -> bool {
        *self == ID_A::TCC0
    }
    #[doc = "Checks if the value of the field is `TC1_TC2`"]
    #[inline(always)]
    pub fn is_tc1_tc2(&self) -> bool {
        *self == ID_A::TC1_TC2
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == ID_A::ADC
    }
    #[doc = "Checks if the value of the field is `AC_DIG`"]
    #[inline(always)]
    pub fn is_ac_dig(&self) -> bool {
        *self == ID_A::AC_DIG
    }
    #[doc = "Checks if the value of the field is `AC_ANA`"]
    #[inline(always)]
    pub fn is_ac_ana(&self) -> bool {
        *self == ID_A::AC_ANA
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == ID_A::DAC
    }
}
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DFLL48"]
    #[inline(always)]
    pub fn dfll48(self) -> &'a mut W {
        self.variant(ID_A::DFLL48)
    }
    #[doc = "FDPLL"]
    #[inline(always)]
    pub fn fdpll(self) -> &'a mut W {
        self.variant(ID_A::FDPLL)
    }
    #[doc = "FDPLL32K"]
    #[inline(always)]
    pub fn fdpll32k(self) -> &'a mut W {
        self.variant(ID_A::FDPLL32K)
    }
    #[doc = "WDT"]
    #[inline(always)]
    pub fn wdt(self) -> &'a mut W {
        self.variant(ID_A::WDT)
    }
    #[doc = "RTC"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut W {
        self.variant(ID_A::RTC)
    }
    #[doc = "EIC"]
    #[inline(always)]
    pub fn eic(self) -> &'a mut W {
        self.variant(ID_A::EIC)
    }
    #[doc = "USB"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut W {
        self.variant(ID_A::USB)
    }
    #[doc = "EVSYS_0"]
    #[inline(always)]
    pub fn evsys_0(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_0)
    }
    #[doc = "EVSYS_1"]
    #[inline(always)]
    pub fn evsys_1(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_1)
    }
    #[doc = "EVSYS_2"]
    #[inline(always)]
    pub fn evsys_2(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_2)
    }
    #[doc = "EVSYS_3"]
    #[inline(always)]
    pub fn evsys_3(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_3)
    }
    #[doc = "EVSYS_4"]
    #[inline(always)]
    pub fn evsys_4(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_4)
    }
    #[doc = "EVSYS_5"]
    #[inline(always)]
    pub fn evsys_5(self) -> &'a mut W {
        self.variant(ID_A::EVSYS_5)
    }
    #[doc = "SERCOMX_SLOW"]
    #[inline(always)]
    pub fn sercomx_slow(self) -> &'a mut W {
        self.variant(ID_A::SERCOMX_SLOW)
    }
    #[doc = "SERCOM0_CORE"]
    #[inline(always)]
    pub fn sercom0_core(self) -> &'a mut W {
        self.variant(ID_A::SERCOM0_CORE)
    }
    #[doc = "SERCOM1_CORE"]
    #[inline(always)]
    pub fn sercom1_core(self) -> &'a mut W {
        self.variant(ID_A::SERCOM1_CORE)
    }
    #[doc = "SERCOM2_CORE"]
    #[inline(always)]
    pub fn sercom2_core(self) -> &'a mut W {
        self.variant(ID_A::SERCOM2_CORE)
    }
    #[doc = "TCC0"]
    #[inline(always)]
    pub fn tcc0(self) -> &'a mut W {
        self.variant(ID_A::TCC0)
    }
    #[doc = "TC1_TC2"]
    #[inline(always)]
    pub fn tc1_tc2(self) -> &'a mut W {
        self.variant(ID_A::TC1_TC2)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(ID_A::ADC)
    }
    #[doc = "AC_DIG"]
    #[inline(always)]
    pub fn ac_dig(self) -> &'a mut W {
        self.variant(ID_A::AC_DIG)
    }
    #[doc = "AC_ANA"]
    #[inline(always)]
    pub fn ac_ana(self) -> &'a mut W {
        self.variant(ID_A::AC_ANA)
    }
    #[doc = "DAC"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(ID_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Generic Clock Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GEN_A {
    #[doc = "0: Generic clock generator 0"]
    GCLK0,
    #[doc = "1: Generic clock generator 1"]
    GCLK1,
    #[doc = "2: Generic clock generator 2"]
    GCLK2,
    #[doc = "3: Generic clock generator 3"]
    GCLK3,
    #[doc = "4: Generic clock generator 4"]
    GCLK4,
    #[doc = "5: Generic clock generator 5"]
    GCLK5,
}
impl From<GEN_A> for u8 {
    #[inline(always)]
    fn from(variant: GEN_A) -> Self {
        match variant {
            GEN_A::GCLK0 => 0,
            GEN_A::GCLK1 => 1,
            GEN_A::GCLK2 => 2,
            GEN_A::GCLK3 => 3,
            GEN_A::GCLK4 => 4,
            GEN_A::GCLK5 => 5,
        }
    }
}
#[doc = "Reader of field `GEN`"]
pub type GEN_R = crate::R<u8, GEN_A>;
impl GEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GEN_A::GCLK0),
            1 => Val(GEN_A::GCLK1),
            2 => Val(GEN_A::GCLK2),
            3 => Val(GEN_A::GCLK3),
            4 => Val(GEN_A::GCLK4),
            5 => Val(GEN_A::GCLK5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GEN_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GEN_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GEN_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GEN_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GEN_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GEN_A::GCLK5
    }
}
#[doc = "Write proxy for field `GEN`"]
pub struct GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut W {
        self.variant(GEN_A::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut W {
        self.variant(GEN_A::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut W {
        self.variant(GEN_A::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut W {
        self.variant(GEN_A::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut W {
        self.variant(GEN_A::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut W {
        self.variant(GEN_A::GCLK5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `WRTLOCK`"]
pub type WRTLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRTLOCK`"]
pub struct WRTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRTLOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&mut self) -> GEN_W {
        GEN_W { w: self }
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WRTLOCK_W {
        WRTLOCK_W { w: self }
    }
}
