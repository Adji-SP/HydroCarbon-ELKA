///Number available in the NVIC for configuring priority
pub const NVIC_PRIO_BITS: u8 = 4;
#[doc(hidden)]
pub mod interrupt {
    ///Enumeration of all the interrupts.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[repr(u16)]
    pub enum Interrupt {
        ///0 - External Pin,Power-on Reset,Brown-out Reset,Watchdog Reset,and JTAG AVR Reset. See Datasheet.
        RESET = 0,
        ///1 - External Interrupt Request 0
        INT0 = 1,
        ///2 - External Interrupt Request 1
        INT1 = 2,
        ///3 - External Interrupt Request 2
        INT2 = 3,
        ///4 - External Interrupt Request 3
        INT3 = 4,
        ///5 - External Interrupt Request 4
        INT4 = 5,
        ///6 - External Interrupt Request 5
        INT5 = 6,
        ///7 - External Interrupt Request 6
        INT6 = 7,
        ///8 - External Interrupt Request 7
        INT7 = 8,
        ///9 - Pin Change Interrupt Request 0
        PCINT0 = 9,
        ///10 - Pin Change Interrupt Request 1
        PCINT1 = 10,
        ///11 - Pin Change Interrupt Request 2
        PCINT2 = 11,
        ///12 - Watchdog Time-out Interrupt
        WDT = 12,
        ///13 - Timer/Counter2 Compare Match A
        TIMER2_COMPA = 13,
        ///14 - Timer/Counter2 Compare Match B
        TIMER2_COMPB = 14,
        ///15 - Timer/Counter2 Overflow
        TIMER2_OVF = 15,
        ///16 - Timer/Counter1 Capture Event
        TIMER1_CAPT = 16,
        ///17 - Timer/Counter1 Compare Match A
        TIMER1_COMPA = 17,
        ///18 - Timer/Counter1 Compare Match B
        TIMER1_COMPB = 18,
        ///19 - Timer/Counter1 Compare Match C
        TIMER1_COMPC = 19,
        ///20 - Timer/Counter1 Overflow
        TIMER1_OVF = 20,
        ///21 - Timer/Counter0 Compare Match A
        TIMER0_COMPA = 21,
        ///22 - Timer/Counter0 Compare Match B
        TIMER0_COMPB = 22,
        ///23 - Timer/Counter0 Overflow
        TIMER0_OVF = 23,
        ///24 - SPI Serial Transfer Complete
        SPI_STC = 24,
        ///25 - USART0, Rx Complete
        USART0_RX = 25,
        ///26 - USART0 Data register Empty
        USART0_UDRE = 26,
        ///27 - USART0, Tx Complete
        USART0_TX = 27,
        ///28 - Analog Comparator
        ANALOG_COMP = 28,
        ///29 - ADC Conversion Complete
        ADC = 29,
        ///30 - EEPROM Ready
        EE_READY = 30,
        ///31 - Timer/Counter3 Capture Event
        TIMER3_CAPT = 31,
        ///32 - Timer/Counter3 Compare Match A
        TIMER3_COMPA = 32,
        ///33 - Timer/Counter3 Compare Match B
        TIMER3_COMPB = 33,
        ///34 - Timer/Counter3 Compare Match C
        TIMER3_COMPC = 34,
        ///35 - Timer/Counter3 Overflow
        TIMER3_OVF = 35,
        ///36 - USART1, Rx Complete
        USART1_RX = 36,
        ///37 - USART1 Data register Empty
        USART1_UDRE = 37,
        ///38 - USART1, Tx Complete
        USART1_TX = 38,
        ///39 - 2-wire Serial Interface
        TWI = 39,
        ///40 - Store Program Memory Read
        SPM_READY = 40,
        ///41 - Timer/Counter4 Capture Event
        TIMER4_CAPT = 41,
        ///42 - Timer/Counter4 Compare Match A
        TIMER4_COMPA = 42,
        ///43 - Timer/Counter4 Compare Match B
        TIMER4_COMPB = 43,
        ///44 - Timer/Counter4 Compare Match C
        TIMER4_COMPC = 44,
        ///45 - Timer/Counter4 Overflow
        TIMER4_OVF = 45,
        ///46 - Timer/Counter5 Capture Event
        TIMER5_CAPT = 46,
        ///47 - Timer/Counter5 Compare Match A
        TIMER5_COMPA = 47,
        ///48 - Timer/Counter5 Compare Match B
        TIMER5_COMPB = 48,
        ///49 - Timer/Counter5 Compare Match C
        TIMER5_COMPC = 49,
        ///50 - Timer/Counter5 Overflow
        TIMER5_OVF = 50,
        ///51 - USART2, Rx Complete
        USART2_RX = 51,
        ///52 - USART2 Data register Empty
        USART2_UDRE = 52,
        ///53 - USART2, Tx Complete
        USART2_TX = 53,
        ///54 - USART3, Rx Complete
        USART3_RX = 54,
        ///55 - USART3 Data register Empty
        USART3_UDRE = 55,
        ///56 - USART3, Tx Complete
        USART3_TX = 56,
    }
    /// TryFromInterruptError
    #[derive(Debug, Copy, Clone)]
    pub struct TryFromInterruptError(());
    impl Interrupt {
        /// Attempt to convert a given value into an `Interrupt`
        #[inline]
        pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
            match value {
                0 => Ok(Interrupt::RESET),
                1 => Ok(Interrupt::INT0),
                2 => Ok(Interrupt::INT1),
                3 => Ok(Interrupt::INT2),
                4 => Ok(Interrupt::INT3),
                5 => Ok(Interrupt::INT4),
                6 => Ok(Interrupt::INT5),
                7 => Ok(Interrupt::INT6),
                8 => Ok(Interrupt::INT7),
                9 => Ok(Interrupt::PCINT0),
                10 => Ok(Interrupt::PCINT1),
                11 => Ok(Interrupt::PCINT2),
                12 => Ok(Interrupt::WDT),
                13 => Ok(Interrupt::TIMER2_COMPA),
                14 => Ok(Interrupt::TIMER2_COMPB),
                15 => Ok(Interrupt::TIMER2_OVF),
                16 => Ok(Interrupt::TIMER1_CAPT),
                17 => Ok(Interrupt::TIMER1_COMPA),
                18 => Ok(Interrupt::TIMER1_COMPB),
                19 => Ok(Interrupt::TIMER1_COMPC),
                20 => Ok(Interrupt::TIMER1_OVF),
                21 => Ok(Interrupt::TIMER0_COMPA),
                22 => Ok(Interrupt::TIMER0_COMPB),
                23 => Ok(Interrupt::TIMER0_OVF),
                24 => Ok(Interrupt::SPI_STC),
                25 => Ok(Interrupt::USART0_RX),
                26 => Ok(Interrupt::USART0_UDRE),
                27 => Ok(Interrupt::USART0_TX),
                28 => Ok(Interrupt::ANALOG_COMP),
                29 => Ok(Interrupt::ADC),
                30 => Ok(Interrupt::EE_READY),
                31 => Ok(Interrupt::TIMER3_CAPT),
                32 => Ok(Interrupt::TIMER3_COMPA),
                33 => Ok(Interrupt::TIMER3_COMPB),
                34 => Ok(Interrupt::TIMER3_COMPC),
                35 => Ok(Interrupt::TIMER3_OVF),
                36 => Ok(Interrupt::USART1_RX),
                37 => Ok(Interrupt::USART1_UDRE),
                38 => Ok(Interrupt::USART1_TX),
                39 => Ok(Interrupt::TWI),
                40 => Ok(Interrupt::SPM_READY),
                41 => Ok(Interrupt::TIMER4_CAPT),
                42 => Ok(Interrupt::TIMER4_COMPA),
                43 => Ok(Interrupt::TIMER4_COMPB),
                44 => Ok(Interrupt::TIMER4_COMPC),
                45 => Ok(Interrupt::TIMER4_OVF),
                46 => Ok(Interrupt::TIMER5_CAPT),
                47 => Ok(Interrupt::TIMER5_COMPA),
                48 => Ok(Interrupt::TIMER5_COMPB),
                49 => Ok(Interrupt::TIMER5_COMPC),
                50 => Ok(Interrupt::TIMER5_OVF),
                51 => Ok(Interrupt::USART2_RX),
                52 => Ok(Interrupt::USART2_UDRE),
                53 => Ok(Interrupt::USART2_TX),
                54 => Ok(Interrupt::USART3_RX),
                55 => Ok(Interrupt::USART3_UDRE),
                56 => Ok(Interrupt::USART3_TX),
                _ => Err(TryFromInterruptError(())),
            }
        }
    }
}
pub use self::interrupt::Interrupt;
///Analog Comparator
pub type AC = crate::Periph<ac::RegisterBlock, 0x50>;
impl core::fmt::Debug for AC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AC").finish()
    }
}
///Analog Comparator
pub mod ac {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        acsr: ACSR,
        _reserved1: [u8; 0x2a],
        adcsrb: ADCSRB,
        _reserved2: [u8; 0x03],
        didr1: DIDR1,
    }
    impl RegisterBlock {
        ///0x00 - Analog Comparator Control And Status Register
        #[inline(always)]
        pub const fn acsr(&self) -> &ACSR {
            &self.acsr
        }
        ///0x2b - ADC Control and Status Register B
        #[inline(always)]
        pub const fn adcsrb(&self) -> &ADCSRB {
            &self.adcsrb
        }
        ///0x2f - Digital Input Disable Register 1
        #[inline(always)]
        pub const fn didr1(&self) -> &DIDR1 {
            &self.didr1
        }
    }
    /**ACSR (rw) register accessor: Analog Comparator Control And Status Register

You can [`read`](crate::Reg::read) this register and get [`acsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acsr`] module*/
    pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
    ///Analog Comparator Control And Status Register
    pub mod acsr {
        ///Register `ACSR` reader
        pub type R = crate::R<ACSR_SPEC>;
        ///Register `ACSR` writer
        pub type W = crate::W<ACSR_SPEC>;
        /**Analog Comparator Interrupt Mode Select

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ACIS_A {
            ///0: Interrupt on Toggle
            ON_TOGGLE = 0,
            ///2: Interrupt on Falling Edge
            ON_FALLING_EDGE = 2,
            ///3: Interrupt on Rising Edge
            ON_RISING_EDGE = 3,
        }
        impl From<ACIS_A> for u8 {
            #[inline(always)]
            fn from(variant: ACIS_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ACIS_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ACIS_A {}
        ///Field `ACIS` reader - Analog Comparator Interrupt Mode Select
        pub type ACIS_R = crate::FieldReader<ACIS_A>;
        impl ACIS_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<ACIS_A> {
                match self.bits {
                    0 => Some(ACIS_A::ON_TOGGLE),
                    2 => Some(ACIS_A::ON_FALLING_EDGE),
                    3 => Some(ACIS_A::ON_RISING_EDGE),
                    _ => None,
                }
            }
            ///Interrupt on Toggle
            #[inline(always)]
            pub fn is_on_toggle(&self) -> bool {
                *self == ACIS_A::ON_TOGGLE
            }
            ///Interrupt on Falling Edge
            #[inline(always)]
            pub fn is_on_falling_edge(&self) -> bool {
                *self == ACIS_A::ON_FALLING_EDGE
            }
            ///Interrupt on Rising Edge
            #[inline(always)]
            pub fn is_on_rising_edge(&self) -> bool {
                *self == ACIS_A::ON_RISING_EDGE
            }
        }
        ///Field `ACIS` writer - Analog Comparator Interrupt Mode Select
        pub type ACIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ACIS_A>;
        impl<'a, REG> ACIS_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Interrupt on Toggle
            #[inline(always)]
            pub fn on_toggle(self) -> &'a mut crate::W<REG> {
                self.variant(ACIS_A::ON_TOGGLE)
            }
            ///Interrupt on Falling Edge
            #[inline(always)]
            pub fn on_falling_edge(self) -> &'a mut crate::W<REG> {
                self.variant(ACIS_A::ON_FALLING_EDGE)
            }
            ///Interrupt on Rising Edge
            #[inline(always)]
            pub fn on_rising_edge(self) -> &'a mut crate::W<REG> {
                self.variant(ACIS_A::ON_RISING_EDGE)
            }
        }
        ///Field `ACIC` reader - Analog Comparator Input Capture Enable
        pub type ACIC_R = crate::BitReader;
        ///Field `ACIC` writer - Analog Comparator Input Capture Enable
        pub type ACIC_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ACIE` reader - Analog Comparator Interrupt Enable
        pub type ACIE_R = crate::BitReader;
        ///Field `ACIE` writer - Analog Comparator Interrupt Enable
        pub type ACIE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ACI` reader - Analog Comparator Interrupt Flag
        pub type ACI_R = crate::BitReader;
        ///Field `ACI` writer - Analog Comparator Interrupt Flag
        pub type ACI_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ACO` reader - Analog Compare Output
        pub type ACO_R = crate::BitReader;
        ///Field `ACBG` reader - Analog Comparator Bandgap Select
        pub type ACBG_R = crate::BitReader;
        ///Field `ACBG` writer - Analog Comparator Bandgap Select
        pub type ACBG_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ACD` reader - Analog Comparator Disable
        pub type ACD_R = crate::BitReader;
        ///Field `ACD` writer - Analog Comparator Disable
        pub type ACD_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:1 - Analog Comparator Interrupt Mode Select
            #[inline(always)]
            pub fn acis(&self) -> ACIS_R {
                ACIS_R::new(self.bits & 3)
            }
            ///Bit 2 - Analog Comparator Input Capture Enable
            #[inline(always)]
            pub fn acic(&self) -> ACIC_R {
                ACIC_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Analog Comparator Interrupt Enable
            #[inline(always)]
            pub fn acie(&self) -> ACIE_R {
                ACIE_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Analog Comparator Interrupt Flag
            #[inline(always)]
            pub fn aci(&self) -> ACI_R {
                ACI_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Analog Compare Output
            #[inline(always)]
            pub fn aco(&self) -> ACO_R {
                ACO_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Analog Comparator Bandgap Select
            #[inline(always)]
            pub fn acbg(&self) -> ACBG_R {
                ACBG_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Analog Comparator Disable
            #[inline(always)]
            pub fn acd(&self) -> ACD_R {
                ACD_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:1 - Analog Comparator Interrupt Mode Select
            #[inline(always)]
            pub fn acis(&mut self) -> ACIS_W<'_, ACSR_SPEC> {
                ACIS_W::new(self, 0)
            }
            ///Bit 2 - Analog Comparator Input Capture Enable
            #[inline(always)]
            pub fn acic(&mut self) -> ACIC_W<'_, ACSR_SPEC> {
                ACIC_W::new(self, 2)
            }
            ///Bit 3 - Analog Comparator Interrupt Enable
            #[inline(always)]
            pub fn acie(&mut self) -> ACIE_W<'_, ACSR_SPEC> {
                ACIE_W::new(self, 3)
            }
            ///Bit 4 - Analog Comparator Interrupt Flag
            #[inline(always)]
            pub fn aci(&mut self) -> ACI_W<'_, ACSR_SPEC> {
                ACI_W::new(self, 4)
            }
            ///Bit 6 - Analog Comparator Bandgap Select
            #[inline(always)]
            pub fn acbg(&mut self) -> ACBG_W<'_, ACSR_SPEC> {
                ACBG_W::new(self, 6)
            }
            ///Bit 7 - Analog Comparator Disable
            #[inline(always)]
            pub fn acd(&mut self) -> ACD_W<'_, ACSR_SPEC> {
                ACD_W::new(self, 7)
            }
        }
        /**Analog Comparator Control And Status Register

You can [`read`](crate::Reg::read) this register and get [`acsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ACSR_SPEC;
        impl crate::RegisterSpec for ACSR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`acsr::R`](R) reader structure
        impl crate::Readable for ACSR_SPEC {}
        ///`write(|w| ..)` method takes [`acsr::W`](W) writer structure
        impl crate::Writable for ACSR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets ACSR to value 0
        impl crate::Resettable for ACSR_SPEC {}
    }
    /**ADCSRB (rw) register accessor: ADC Control and Status Register B

You can [`read`](crate::Reg::read) this register and get [`adcsrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcsrb`] module*/
    pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
    ///ADC Control and Status Register B
    pub mod adcsrb {
        ///Register `ADCSRB` reader
        pub type R = crate::R<ADCSRB_SPEC>;
        ///Register `ADCSRB` writer
        pub type W = crate::W<ADCSRB_SPEC>;
        ///Field `ACME` reader - Analog Comparator Multiplexer Enable
        pub type ACME_R = crate::BitReader;
        ///Field `ACME` writer - Analog Comparator Multiplexer Enable
        pub type ACME_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 6 - Analog Comparator Multiplexer Enable
            #[inline(always)]
            pub fn acme(&self) -> ACME_R {
                ACME_R::new(((self.bits >> 6) & 1) != 0)
            }
        }
        impl W {
            ///Bit 6 - Analog Comparator Multiplexer Enable
            #[inline(always)]
            pub fn acme(&mut self) -> ACME_W<'_, ADCSRB_SPEC> {
                ACME_W::new(self, 6)
            }
        }
        /**ADC Control and Status Register B

You can [`read`](crate::Reg::read) this register and get [`adcsrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ADCSRB_SPEC;
        impl crate::RegisterSpec for ADCSRB_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`adcsrb::R`](R) reader structure
        impl crate::Readable for ADCSRB_SPEC {}
        ///`write(|w| ..)` method takes [`adcsrb::W`](W) writer structure
        impl crate::Writable for ADCSRB_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets ADCSRB to value 0
        impl crate::Resettable for ADCSRB_SPEC {}
    }
    /**DIDR1 (rw) register accessor: Digital Input Disable Register 1

You can [`read`](crate::Reg::read) this register and get [`didr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`didr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@didr1`] module*/
    pub type DIDR1 = crate::Reg<didr1::DIDR1_SPEC>;
    ///Digital Input Disable Register 1
    pub mod didr1 {
        ///Register `DIDR1` reader
        pub type R = crate::R<DIDR1_SPEC>;
        ///Register `DIDR1` writer
        pub type W = crate::W<DIDR1_SPEC>;
        ///Field `AIN0D` reader - AIN0 Digital Input Disable
        pub type AIN0D_R = crate::BitReader;
        ///Field `AIN0D` writer - AIN0 Digital Input Disable
        pub type AIN0D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `AIN1D` reader - AIN1 Digital Input Disable
        pub type AIN1D_R = crate::BitReader;
        ///Field `AIN1D` writer - AIN1 Digital Input Disable
        pub type AIN1D_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - AIN0 Digital Input Disable
            #[inline(always)]
            pub fn ain0d(&self) -> AIN0D_R {
                AIN0D_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - AIN1 Digital Input Disable
            #[inline(always)]
            pub fn ain1d(&self) -> AIN1D_R {
                AIN1D_R::new(((self.bits >> 1) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - AIN0 Digital Input Disable
            #[inline(always)]
            pub fn ain0d(&mut self) -> AIN0D_W<'_, DIDR1_SPEC> {
                AIN0D_W::new(self, 0)
            }
            ///Bit 1 - AIN1 Digital Input Disable
            #[inline(always)]
            pub fn ain1d(&mut self) -> AIN1D_W<'_, DIDR1_SPEC> {
                AIN1D_W::new(self, 1)
            }
        }
        /**Digital Input Disable Register 1

You can [`read`](crate::Reg::read) this register and get [`didr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`didr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DIDR1_SPEC;
        impl crate::RegisterSpec for DIDR1_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`didr1::R`](R) reader structure
        impl crate::Readable for DIDR1_SPEC {}
        ///`write(|w| ..)` method takes [`didr1::W`](W) writer structure
        impl crate::Writable for DIDR1_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DIDR1 to value 0
        impl crate::Resettable for DIDR1_SPEC {}
    }
}
///Analog-to-Digital Converter
pub type ADC = crate::Periph<adc::RegisterBlock, 0x78>;
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
///Analog-to-Digital Converter
pub mod adc {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        adc: ADC,
        adcsra: ADCSRA,
        adcsrb: ADCSRB,
        admux: ADMUX,
        didr2: DIDR2,
        didr0: DIDR0,
    }
    impl RegisterBlock {
        ///0x00 - ADC Data Register Bytes
        #[inline(always)]
        pub const fn adc(&self) -> &ADC {
            &self.adc
        }
        ///0x02 - The ADC Control and Status register A
        #[inline(always)]
        pub const fn adcsra(&self) -> &ADCSRA {
            &self.adcsra
        }
        ///0x03 - The ADC Control and Status register B
        #[inline(always)]
        pub const fn adcsrb(&self) -> &ADCSRB {
            &self.adcsrb
        }
        ///0x04 - The ADC multiplexer Selection Register
        #[inline(always)]
        pub const fn admux(&self) -> &ADMUX {
            &self.admux
        }
        ///0x05 - Digital Input Disable Register
        #[inline(always)]
        pub const fn didr2(&self) -> &DIDR2 {
            &self.didr2
        }
        ///0x06 - Digital Input Disable Register
        #[inline(always)]
        pub const fn didr0(&self) -> &DIDR0 {
            &self.didr0
        }
    }
    /**ADC (rw) register accessor: ADC Data Register Bytes

You can [`read`](crate::Reg::read) this register and get [`adc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adc`] module*/
    pub type ADC = crate::Reg<adc::ADC_SPEC>;
    ///ADC Data Register Bytes
    pub mod adc {
        ///Register `ADC` reader
        pub type R = crate::R<ADC_SPEC>;
        ///Register `ADC` writer
        pub type W = crate::W<ADC_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**ADC Data Register Bytes

You can [`read`](crate::Reg::read) this register and get [`adc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ADC_SPEC;
        impl crate::RegisterSpec for ADC_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`adc::R`](R) reader structure
        impl crate::Readable for ADC_SPEC {}
        ///`write(|w| ..)` method takes [`adc::W`](W) writer structure
        impl crate::Writable for ADC_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets ADC to value 0
        impl crate::Resettable for ADC_SPEC {}
    }
    /**ADCSRA (rw) register accessor: The ADC Control and Status register A

You can [`read`](crate::Reg::read) this register and get [`adcsra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcsra`] module*/
    pub type ADCSRA = crate::Reg<adcsra::ADCSRA_SPEC>;
    ///The ADC Control and Status register A
    pub mod adcsra {
        ///Register `ADCSRA` reader
        pub type R = crate::R<ADCSRA_SPEC>;
        ///Register `ADCSRA` writer
        pub type W = crate::W<ADCSRA_SPEC>;
        /**ADC Prescaler Select Bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ADPS_A {
            ///1: Prescaler Value 2
            PRESCALER_2 = 1,
            ///2: Prescaler Value 4
            PRESCALER_4 = 2,
            ///3: Prescaler Value 8
            PRESCALER_8 = 3,
            ///4: Prescaler Value 16
            PRESCALER_16 = 4,
            ///5: Prescaler Value 32
            PRESCALER_32 = 5,
            ///6: Prescaler Value 64
            PRESCALER_64 = 6,
            ///7: Prescaler Value 128
            PRESCALER_128 = 7,
        }
        impl From<ADPS_A> for u8 {
            #[inline(always)]
            fn from(variant: ADPS_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ADPS_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ADPS_A {}
        ///Field `ADPS` reader - ADC Prescaler Select Bits
        pub type ADPS_R = crate::FieldReader<ADPS_A>;
        impl ADPS_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<ADPS_A> {
                match self.bits {
                    1 => Some(ADPS_A::PRESCALER_2),
                    2 => Some(ADPS_A::PRESCALER_4),
                    3 => Some(ADPS_A::PRESCALER_8),
                    4 => Some(ADPS_A::PRESCALER_16),
                    5 => Some(ADPS_A::PRESCALER_32),
                    6 => Some(ADPS_A::PRESCALER_64),
                    7 => Some(ADPS_A::PRESCALER_128),
                    _ => None,
                }
            }
            ///Prescaler Value 2
            #[inline(always)]
            pub fn is_prescaler_2(&self) -> bool {
                *self == ADPS_A::PRESCALER_2
            }
            ///Prescaler Value 4
            #[inline(always)]
            pub fn is_prescaler_4(&self) -> bool {
                *self == ADPS_A::PRESCALER_4
            }
            ///Prescaler Value 8
            #[inline(always)]
            pub fn is_prescaler_8(&self) -> bool {
                *self == ADPS_A::PRESCALER_8
            }
            ///Prescaler Value 16
            #[inline(always)]
            pub fn is_prescaler_16(&self) -> bool {
                *self == ADPS_A::PRESCALER_16
            }
            ///Prescaler Value 32
            #[inline(always)]
            pub fn is_prescaler_32(&self) -> bool {
                *self == ADPS_A::PRESCALER_32
            }
            ///Prescaler Value 64
            #[inline(always)]
            pub fn is_prescaler_64(&self) -> bool {
                *self == ADPS_A::PRESCALER_64
            }
            ///Prescaler Value 128
            #[inline(always)]
            pub fn is_prescaler_128(&self) -> bool {
                *self == ADPS_A::PRESCALER_128
            }
        }
        ///Field `ADPS` writer - ADC Prescaler Select Bits
        pub type ADPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADPS_A>;
        impl<'a, REG> ADPS_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Prescaler Value 2
            #[inline(always)]
            pub fn prescaler_2(self) -> &'a mut crate::W<REG> {
                self.variant(ADPS_A::PRESCALER_2)
            }
            ///Prescaler Value 4
            #[inline(always)]
            pub fn prescaler_4(self) -> &'a mut crate::W<REG> {
                self.variant(ADPS_A::PRESCALER_4)
            }
            ///Prescaler Value 8
            #[inline(always)]
            pub fn prescaler_8(self) -> &'a mut crate::W<REG> {
                self.variant(ADPS_A::PRESCALER_8)
            }
            ///Prescaler Value 16
            #[inline(always)]
            pub fn prescaler_16(self) -> &'a mut crate::W<REG> {
                self.variant(ADPS_A::PRESCALER_16)
            }
            ///Prescaler Value 32
            #[inline(always)]
            pub fn prescaler_32(self) -> &'a mut crate::W<REG> {
                self.variant(ADPS_A::PRESCALER_32)
            }
            ///Prescaler Value 64
            #[inline(always)]
            pub fn prescaler_64(self) -> &'a mut crate::W<REG> {
                self.variant(ADPS_A::PRESCALER_64)
            }
            ///Prescaler Value 128
            #[inline(always)]
            pub fn prescaler_128(self) -> &'a mut crate::W<REG> {
                self.variant(ADPS_A::PRESCALER_128)
            }
        }
        ///Field `ADIE` reader - ADC Interrupt Enable
        pub type ADIE_R = crate::BitReader;
        ///Field `ADIE` writer - ADC Interrupt Enable
        pub type ADIE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADIF` reader - ADC Interrupt Flag
        pub type ADIF_R = crate::BitReader;
        ///Field `ADIF` writer - ADC Interrupt Flag
        pub type ADIF_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADATE` reader - ADC Auto Trigger Enable
        pub type ADATE_R = crate::BitReader;
        ///Field `ADATE` writer - ADC Auto Trigger Enable
        pub type ADATE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADSC` reader - ADC Start Conversion
        pub type ADSC_R = crate::BitReader;
        ///Field `ADSC` writer - ADC Start Conversion
        pub type ADSC_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADEN` reader - ADC Enable
        pub type ADEN_R = crate::BitReader;
        ///Field `ADEN` writer - ADC Enable
        pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:2 - ADC Prescaler Select Bits
            #[inline(always)]
            pub fn adps(&self) -> ADPS_R {
                ADPS_R::new(self.bits & 7)
            }
            ///Bit 3 - ADC Interrupt Enable
            #[inline(always)]
            pub fn adie(&self) -> ADIE_R {
                ADIE_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - ADC Interrupt Flag
            #[inline(always)]
            pub fn adif(&self) -> ADIF_R {
                ADIF_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - ADC Auto Trigger Enable
            #[inline(always)]
            pub fn adate(&self) -> ADATE_R {
                ADATE_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - ADC Start Conversion
            #[inline(always)]
            pub fn adsc(&self) -> ADSC_R {
                ADSC_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - ADC Enable
            #[inline(always)]
            pub fn aden(&self) -> ADEN_R {
                ADEN_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:2 - ADC Prescaler Select Bits
            #[inline(always)]
            pub fn adps(&mut self) -> ADPS_W<'_, ADCSRA_SPEC> {
                ADPS_W::new(self, 0)
            }
            ///Bit 3 - ADC Interrupt Enable
            #[inline(always)]
            pub fn adie(&mut self) -> ADIE_W<'_, ADCSRA_SPEC> {
                ADIE_W::new(self, 3)
            }
            ///Bit 4 - ADC Interrupt Flag
            #[inline(always)]
            pub fn adif(&mut self) -> ADIF_W<'_, ADCSRA_SPEC> {
                ADIF_W::new(self, 4)
            }
            ///Bit 5 - ADC Auto Trigger Enable
            #[inline(always)]
            pub fn adate(&mut self) -> ADATE_W<'_, ADCSRA_SPEC> {
                ADATE_W::new(self, 5)
            }
            ///Bit 6 - ADC Start Conversion
            #[inline(always)]
            pub fn adsc(&mut self) -> ADSC_W<'_, ADCSRA_SPEC> {
                ADSC_W::new(self, 6)
            }
            ///Bit 7 - ADC Enable
            #[inline(always)]
            pub fn aden(&mut self) -> ADEN_W<'_, ADCSRA_SPEC> {
                ADEN_W::new(self, 7)
            }
        }
        /**The ADC Control and Status register A

You can [`read`](crate::Reg::read) this register and get [`adcsra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ADCSRA_SPEC;
        impl crate::RegisterSpec for ADCSRA_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`adcsra::R`](R) reader structure
        impl crate::Readable for ADCSRA_SPEC {}
        ///`write(|w| ..)` method takes [`adcsra::W`](W) writer structure
        impl crate::Writable for ADCSRA_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets ADCSRA to value 0
        impl crate::Resettable for ADCSRA_SPEC {}
    }
    /**ADCSRB (rw) register accessor: The ADC Control and Status register B

You can [`read`](crate::Reg::read) this register and get [`adcsrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcsrb`] module*/
    pub type ADCSRB = crate::Reg<adcsrb::ADCSRB_SPEC>;
    ///The ADC Control and Status register B
    pub mod adcsrb {
        ///Register `ADCSRB` reader
        pub type R = crate::R<ADCSRB_SPEC>;
        ///Register `ADCSRB` writer
        pub type W = crate::W<ADCSRB_SPEC>;
        /**ADC Auto Trigger Source bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ADTS_A {
            ///0: Free Running mode
            VAL_0X00 = 0,
            ///1: Analog Comparator
            VAL_0X01 = 1,
            ///2: External Interrupt Request 0
            VAL_0X02 = 2,
            ///3: Timer/Counter0 Compare Match A
            VAL_0X03 = 3,
            ///4: Timer/Counter0 Overflow
            VAL_0X04 = 4,
            ///5: Timer/Counter1 Compare Match B
            VAL_0X05 = 5,
            ///6: Timer/Counter1 Overflow
            VAL_0X06 = 6,
            ///7: Timer/Counter1 Capture Event
            VAL_0X07 = 7,
        }
        impl From<ADTS_A> for u8 {
            #[inline(always)]
            fn from(variant: ADTS_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ADTS_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ADTS_A {}
        ///Field `ADTS` reader - ADC Auto Trigger Source bits
        pub type ADTS_R = crate::FieldReader<ADTS_A>;
        impl ADTS_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> ADTS_A {
                match self.bits {
                    0 => ADTS_A::VAL_0X00,
                    1 => ADTS_A::VAL_0X01,
                    2 => ADTS_A::VAL_0X02,
                    3 => ADTS_A::VAL_0X03,
                    4 => ADTS_A::VAL_0X04,
                    5 => ADTS_A::VAL_0X05,
                    6 => ADTS_A::VAL_0X06,
                    7 => ADTS_A::VAL_0X07,
                    _ => unreachable!(),
                }
            }
            ///Free Running mode
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == ADTS_A::VAL_0X00
            }
            ///Analog Comparator
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == ADTS_A::VAL_0X01
            }
            ///External Interrupt Request 0
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == ADTS_A::VAL_0X02
            }
            ///Timer/Counter0 Compare Match A
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == ADTS_A::VAL_0X03
            }
            ///Timer/Counter0 Overflow
            #[inline(always)]
            pub fn is_val_0x04(&self) -> bool {
                *self == ADTS_A::VAL_0X04
            }
            ///Timer/Counter1 Compare Match B
            #[inline(always)]
            pub fn is_val_0x05(&self) -> bool {
                *self == ADTS_A::VAL_0X05
            }
            ///Timer/Counter1 Overflow
            #[inline(always)]
            pub fn is_val_0x06(&self) -> bool {
                *self == ADTS_A::VAL_0X06
            }
            ///Timer/Counter1 Capture Event
            #[inline(always)]
            pub fn is_val_0x07(&self) -> bool {
                *self == ADTS_A::VAL_0X07
            }
        }
        ///Field `ADTS` writer - ADC Auto Trigger Source bits
        pub type ADTS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADTS_A, crate::Safe>;
        impl<'a, REG> ADTS_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Free Running mode
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(ADTS_A::VAL_0X00)
            }
            ///Analog Comparator
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(ADTS_A::VAL_0X01)
            }
            ///External Interrupt Request 0
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(ADTS_A::VAL_0X02)
            }
            ///Timer/Counter0 Compare Match A
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(ADTS_A::VAL_0X03)
            }
            ///Timer/Counter0 Overflow
            #[inline(always)]
            pub fn val_0x04(self) -> &'a mut crate::W<REG> {
                self.variant(ADTS_A::VAL_0X04)
            }
            ///Timer/Counter1 Compare Match B
            #[inline(always)]
            pub fn val_0x05(self) -> &'a mut crate::W<REG> {
                self.variant(ADTS_A::VAL_0X05)
            }
            ///Timer/Counter1 Overflow
            #[inline(always)]
            pub fn val_0x06(self) -> &'a mut crate::W<REG> {
                self.variant(ADTS_A::VAL_0X06)
            }
            ///Timer/Counter1 Capture Event
            #[inline(always)]
            pub fn val_0x07(self) -> &'a mut crate::W<REG> {
                self.variant(ADTS_A::VAL_0X07)
            }
        }
        ///Field `MUX5` reader - Analog Channel and Gain Selection Bits
        pub type MUX5_R = crate::BitReader;
        ///Field `MUX5` writer - Analog Channel and Gain Selection Bits
        pub type MUX5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ACME` reader - No Description.
        pub type ACME_R = crate::BitReader;
        ///Field `ACME` writer - No Description.
        pub type ACME_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:2 - ADC Auto Trigger Source bits
            #[inline(always)]
            pub fn adts(&self) -> ADTS_R {
                ADTS_R::new(self.bits & 7)
            }
            ///Bit 3 - Analog Channel and Gain Selection Bits
            #[inline(always)]
            pub fn mux5(&self) -> MUX5_R {
                MUX5_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 6 - No Description.
            #[inline(always)]
            pub fn acme(&self) -> ACME_R {
                ACME_R::new(((self.bits >> 6) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:2 - ADC Auto Trigger Source bits
            #[inline(always)]
            pub fn adts(&mut self) -> ADTS_W<'_, ADCSRB_SPEC> {
                ADTS_W::new(self, 0)
            }
            ///Bit 3 - Analog Channel and Gain Selection Bits
            #[inline(always)]
            pub fn mux5(&mut self) -> MUX5_W<'_, ADCSRB_SPEC> {
                MUX5_W::new(self, 3)
            }
            ///Bit 6 - No Description.
            #[inline(always)]
            pub fn acme(&mut self) -> ACME_W<'_, ADCSRB_SPEC> {
                ACME_W::new(self, 6)
            }
        }
        /**The ADC Control and Status register B

You can [`read`](crate::Reg::read) this register and get [`adcsrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ADCSRB_SPEC;
        impl crate::RegisterSpec for ADCSRB_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`adcsrb::R`](R) reader structure
        impl crate::Readable for ADCSRB_SPEC {}
        ///`write(|w| ..)` method takes [`adcsrb::W`](W) writer structure
        impl crate::Writable for ADCSRB_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets ADCSRB to value 0
        impl crate::Resettable for ADCSRB_SPEC {}
    }
    /**ADMUX (rw) register accessor: The ADC multiplexer Selection Register

You can [`read`](crate::Reg::read) this register and get [`admux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`admux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@admux`] module*/
    pub type ADMUX = crate::Reg<admux::ADMUX_SPEC>;
    ///The ADC multiplexer Selection Register
    pub mod admux {
        ///Register `ADMUX` reader
        pub type R = crate::R<ADMUX_SPEC>;
        ///Register `ADMUX` writer
        pub type W = crate::W<ADMUX_SPEC>;
        /**Analog Channel and Gain Selection Bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum MUX_A {
            ///0: ADC Single Ended Input pin 0
            ADC0 = 0,
            ///1: ADC Single Ended Input pin 1
            ADC1 = 1,
            ///2: ADC Single Ended Input pin 2
            ADC2 = 2,
            ///3: ADC Single Ended Input pin 3
            ADC3 = 3,
            ///4: ADC Single Ended Input pin 4
            ADC4 = 4,
            ///5: ADC Single Ended Input pin 5
            ADC5 = 5,
            ///6: ADC Single Ended Input pin 6
            ADC6 = 6,
            ///7: ADC Single Ended Input pin 7
            ADC7 = 7,
            ///8: ADC Differential Inputs Postive pin 0 Negative pin 0 10x Gain
            ADC0_ADC0_10X = 8,
            ///9: ADC Differential Inputs Postive pin 1 Negative pin 0 10x Gain
            ADC1_ADC0_10X = 9,
            ///10: ADC Differential Inputs Postive pin 0 Negative pin 0 200x Gain
            ADC0_ADC0_200X = 10,
            ///11: ADC Differential Inputs Postive pin 1 Negative pin 0 200x Gain
            ADC1_ADC0_200X = 11,
            ///12: ADC Differential Inputs Postive pin 2 Negative pin 2 10x Gain
            ADC2_ADC2_10X = 12,
            ///13: ADC Differential Inputs Postive pin 3 Negative pin 2 10x Gain
            ADC3_ADC2_10X = 13,
            ///14: ADC Differential Inputs Postive pin 2 Negative pin 2 200x Gain
            ADC2_ADC2_200X = 14,
            ///15: ADC Differential Inputs Postive pin 3 Negative pin 2 200x Gain
            ADC3_ADC2_200X = 15,
            ///16: ADC Differential Inputs Postive pin 0 Negative pin 1 1x Gain
            ADC0_ADC1_1X = 16,
            ///17: ADC Differential Inputs Postive pin 1 Negative pin 1 1x Gain
            ADC1_ADC1_1X = 17,
            ///18: ADC Differential Inputs Postive pin 2 Negative pin 1 1x Gain
            ADC2_ADC1_1X = 18,
            ///19: ADC Differential Inputs Postive pin 3 Negative pin 1 1x Gain
            ADC3_ADC1_1X = 19,
            ///20: ADC Differential Inputs Postive pin 4 Negative pin 1 1x Gain
            ADC4_ADC1_1X = 20,
            ///21: ADC Differential Inputs Postive pin 5 Negative pin 1 1x Gain
            ADC5_ADC1_1X = 21,
            ///22: ADC Differential Inputs Postive pin 6 Negative pin 1 1x Gain
            ADC6_ADC1_1X = 22,
            ///23: ADC Differential Inputs Postive pin 7 Negative pin 1 1x Gain
            ADC7_ADC1_1X = 23,
            ///24: ADC Differential Inputs Postive pin 0 Negative pin 2 1x Gain
            ADC0_ADC2_1X = 24,
            ///25: ADC Differential Inputs Postive pin 1 Negative pin 2 1x Gain
            ADC1_ADC2_1X = 25,
            ///26: ADC Differential Inputs Postive pin 2 Negative pin 2 1x Gain
            ADC2_ADC2_1X = 26,
            ///27: ADC Differential Inputs Postive pin 3 Negative pin 2 1x Gain
            ADC3_ADC2_1X = 27,
            ///28: ADC Differential Inputs Postive pin 4 Negative pin 2 1x Gain
            ADC4_ADC2_1X = 28,
            ///29: ADC Differential Inputs Postive pin 5 Negative pin 2 1x Gain
            ADC5_ADC2_1X = 29,
            ///30: Internal Reference (VBG)
            ADC_VBG = 30,
            ///31: 0V (GND)
            ADC_GND = 31,
        }
        impl From<MUX_A> for u8 {
            #[inline(always)]
            fn from(variant: MUX_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for MUX_A {
            type Ux = u8;
        }
        impl crate::IsEnum for MUX_A {}
        ///Field `MUX` reader - Analog Channel and Gain Selection Bits
        pub type MUX_R = crate::FieldReader<MUX_A>;
        impl MUX_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> MUX_A {
                match self.bits {
                    0 => MUX_A::ADC0,
                    1 => MUX_A::ADC1,
                    2 => MUX_A::ADC2,
                    3 => MUX_A::ADC3,
                    4 => MUX_A::ADC4,
                    5 => MUX_A::ADC5,
                    6 => MUX_A::ADC6,
                    7 => MUX_A::ADC7,
                    8 => MUX_A::ADC0_ADC0_10X,
                    9 => MUX_A::ADC1_ADC0_10X,
                    10 => MUX_A::ADC0_ADC0_200X,
                    11 => MUX_A::ADC1_ADC0_200X,
                    12 => MUX_A::ADC2_ADC2_10X,
                    13 => MUX_A::ADC3_ADC2_10X,
                    14 => MUX_A::ADC2_ADC2_200X,
                    15 => MUX_A::ADC3_ADC2_200X,
                    16 => MUX_A::ADC0_ADC1_1X,
                    17 => MUX_A::ADC1_ADC1_1X,
                    18 => MUX_A::ADC2_ADC1_1X,
                    19 => MUX_A::ADC3_ADC1_1X,
                    20 => MUX_A::ADC4_ADC1_1X,
                    21 => MUX_A::ADC5_ADC1_1X,
                    22 => MUX_A::ADC6_ADC1_1X,
                    23 => MUX_A::ADC7_ADC1_1X,
                    24 => MUX_A::ADC0_ADC2_1X,
                    25 => MUX_A::ADC1_ADC2_1X,
                    26 => MUX_A::ADC2_ADC2_1X,
                    27 => MUX_A::ADC3_ADC2_1X,
                    28 => MUX_A::ADC4_ADC2_1X,
                    29 => MUX_A::ADC5_ADC2_1X,
                    30 => MUX_A::ADC_VBG,
                    31 => MUX_A::ADC_GND,
                    _ => unreachable!(),
                }
            }
            ///ADC Single Ended Input pin 0
            #[inline(always)]
            pub fn is_adc0(&self) -> bool {
                *self == MUX_A::ADC0
            }
            ///ADC Single Ended Input pin 1
            #[inline(always)]
            pub fn is_adc1(&self) -> bool {
                *self == MUX_A::ADC1
            }
            ///ADC Single Ended Input pin 2
            #[inline(always)]
            pub fn is_adc2(&self) -> bool {
                *self == MUX_A::ADC2
            }
            ///ADC Single Ended Input pin 3
            #[inline(always)]
            pub fn is_adc3(&self) -> bool {
                *self == MUX_A::ADC3
            }
            ///ADC Single Ended Input pin 4
            #[inline(always)]
            pub fn is_adc4(&self) -> bool {
                *self == MUX_A::ADC4
            }
            ///ADC Single Ended Input pin 5
            #[inline(always)]
            pub fn is_adc5(&self) -> bool {
                *self == MUX_A::ADC5
            }
            ///ADC Single Ended Input pin 6
            #[inline(always)]
            pub fn is_adc6(&self) -> bool {
                *self == MUX_A::ADC6
            }
            ///ADC Single Ended Input pin 7
            #[inline(always)]
            pub fn is_adc7(&self) -> bool {
                *self == MUX_A::ADC7
            }
            ///ADC Differential Inputs Postive pin 0 Negative pin 0 10x Gain
            #[inline(always)]
            pub fn is_adc0_adc0_10x(&self) -> bool {
                *self == MUX_A::ADC0_ADC0_10X
            }
            ///ADC Differential Inputs Postive pin 1 Negative pin 0 10x Gain
            #[inline(always)]
            pub fn is_adc1_adc0_10x(&self) -> bool {
                *self == MUX_A::ADC1_ADC0_10X
            }
            ///ADC Differential Inputs Postive pin 0 Negative pin 0 200x Gain
            #[inline(always)]
            pub fn is_adc0_adc0_200x(&self) -> bool {
                *self == MUX_A::ADC0_ADC0_200X
            }
            ///ADC Differential Inputs Postive pin 1 Negative pin 0 200x Gain
            #[inline(always)]
            pub fn is_adc1_adc0_200x(&self) -> bool {
                *self == MUX_A::ADC1_ADC0_200X
            }
            ///ADC Differential Inputs Postive pin 2 Negative pin 2 10x Gain
            #[inline(always)]
            pub fn is_adc2_adc2_10x(&self) -> bool {
                *self == MUX_A::ADC2_ADC2_10X
            }
            ///ADC Differential Inputs Postive pin 3 Negative pin 2 10x Gain
            #[inline(always)]
            pub fn is_adc3_adc2_10x(&self) -> bool {
                *self == MUX_A::ADC3_ADC2_10X
            }
            ///ADC Differential Inputs Postive pin 2 Negative pin 2 200x Gain
            #[inline(always)]
            pub fn is_adc2_adc2_200x(&self) -> bool {
                *self == MUX_A::ADC2_ADC2_200X
            }
            ///ADC Differential Inputs Postive pin 3 Negative pin 2 200x Gain
            #[inline(always)]
            pub fn is_adc3_adc2_200x(&self) -> bool {
                *self == MUX_A::ADC3_ADC2_200X
            }
            ///ADC Differential Inputs Postive pin 0 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn is_adc0_adc1_1x(&self) -> bool {
                *self == MUX_A::ADC0_ADC1_1X
            }
            ///ADC Differential Inputs Postive pin 1 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn is_adc1_adc1_1x(&self) -> bool {
                *self == MUX_A::ADC1_ADC1_1X
            }
            ///ADC Differential Inputs Postive pin 2 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn is_adc2_adc1_1x(&self) -> bool {
                *self == MUX_A::ADC2_ADC1_1X
            }
            ///ADC Differential Inputs Postive pin 3 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn is_adc3_adc1_1x(&self) -> bool {
                *self == MUX_A::ADC3_ADC1_1X
            }
            ///ADC Differential Inputs Postive pin 4 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn is_adc4_adc1_1x(&self) -> bool {
                *self == MUX_A::ADC4_ADC1_1X
            }
            ///ADC Differential Inputs Postive pin 5 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn is_adc5_adc1_1x(&self) -> bool {
                *self == MUX_A::ADC5_ADC1_1X
            }
            ///ADC Differential Inputs Postive pin 6 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn is_adc6_adc1_1x(&self) -> bool {
                *self == MUX_A::ADC6_ADC1_1X
            }
            ///ADC Differential Inputs Postive pin 7 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn is_adc7_adc1_1x(&self) -> bool {
                *self == MUX_A::ADC7_ADC1_1X
            }
            ///ADC Differential Inputs Postive pin 0 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn is_adc0_adc2_1x(&self) -> bool {
                *self == MUX_A::ADC0_ADC2_1X
            }
            ///ADC Differential Inputs Postive pin 1 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn is_adc1_adc2_1x(&self) -> bool {
                *self == MUX_A::ADC1_ADC2_1X
            }
            ///ADC Differential Inputs Postive pin 2 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn is_adc2_adc2_1x(&self) -> bool {
                *self == MUX_A::ADC2_ADC2_1X
            }
            ///ADC Differential Inputs Postive pin 3 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn is_adc3_adc2_1x(&self) -> bool {
                *self == MUX_A::ADC3_ADC2_1X
            }
            ///ADC Differential Inputs Postive pin 4 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn is_adc4_adc2_1x(&self) -> bool {
                *self == MUX_A::ADC4_ADC2_1X
            }
            ///ADC Differential Inputs Postive pin 5 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn is_adc5_adc2_1x(&self) -> bool {
                *self == MUX_A::ADC5_ADC2_1X
            }
            ///Internal Reference (VBG)
            #[inline(always)]
            pub fn is_adc_vbg(&self) -> bool {
                *self == MUX_A::ADC_VBG
            }
            ///0V (GND)
            #[inline(always)]
            pub fn is_adc_gnd(&self) -> bool {
                *self == MUX_A::ADC_GND
            }
        }
        ///Field `MUX` writer - Analog Channel and Gain Selection Bits
        pub type MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 5, MUX_A, crate::Safe>;
        impl<'a, REG> MUX_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///ADC Single Ended Input pin 0
            #[inline(always)]
            pub fn adc0(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC0)
            }
            ///ADC Single Ended Input pin 1
            #[inline(always)]
            pub fn adc1(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC1)
            }
            ///ADC Single Ended Input pin 2
            #[inline(always)]
            pub fn adc2(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC2)
            }
            ///ADC Single Ended Input pin 3
            #[inline(always)]
            pub fn adc3(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC3)
            }
            ///ADC Single Ended Input pin 4
            #[inline(always)]
            pub fn adc4(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC4)
            }
            ///ADC Single Ended Input pin 5
            #[inline(always)]
            pub fn adc5(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC5)
            }
            ///ADC Single Ended Input pin 6
            #[inline(always)]
            pub fn adc6(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC6)
            }
            ///ADC Single Ended Input pin 7
            #[inline(always)]
            pub fn adc7(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC7)
            }
            ///ADC Differential Inputs Postive pin 0 Negative pin 0 10x Gain
            #[inline(always)]
            pub fn adc0_adc0_10x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC0_ADC0_10X)
            }
            ///ADC Differential Inputs Postive pin 1 Negative pin 0 10x Gain
            #[inline(always)]
            pub fn adc1_adc0_10x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC1_ADC0_10X)
            }
            ///ADC Differential Inputs Postive pin 0 Negative pin 0 200x Gain
            #[inline(always)]
            pub fn adc0_adc0_200x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC0_ADC0_200X)
            }
            ///ADC Differential Inputs Postive pin 1 Negative pin 0 200x Gain
            #[inline(always)]
            pub fn adc1_adc0_200x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC1_ADC0_200X)
            }
            ///ADC Differential Inputs Postive pin 2 Negative pin 2 10x Gain
            #[inline(always)]
            pub fn adc2_adc2_10x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC2_ADC2_10X)
            }
            ///ADC Differential Inputs Postive pin 3 Negative pin 2 10x Gain
            #[inline(always)]
            pub fn adc3_adc2_10x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC3_ADC2_10X)
            }
            ///ADC Differential Inputs Postive pin 2 Negative pin 2 200x Gain
            #[inline(always)]
            pub fn adc2_adc2_200x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC2_ADC2_200X)
            }
            ///ADC Differential Inputs Postive pin 3 Negative pin 2 200x Gain
            #[inline(always)]
            pub fn adc3_adc2_200x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC3_ADC2_200X)
            }
            ///ADC Differential Inputs Postive pin 0 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn adc0_adc1_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC0_ADC1_1X)
            }
            ///ADC Differential Inputs Postive pin 1 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn adc1_adc1_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC1_ADC1_1X)
            }
            ///ADC Differential Inputs Postive pin 2 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn adc2_adc1_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC2_ADC1_1X)
            }
            ///ADC Differential Inputs Postive pin 3 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn adc3_adc1_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC3_ADC1_1X)
            }
            ///ADC Differential Inputs Postive pin 4 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn adc4_adc1_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC4_ADC1_1X)
            }
            ///ADC Differential Inputs Postive pin 5 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn adc5_adc1_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC5_ADC1_1X)
            }
            ///ADC Differential Inputs Postive pin 6 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn adc6_adc1_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC6_ADC1_1X)
            }
            ///ADC Differential Inputs Postive pin 7 Negative pin 1 1x Gain
            #[inline(always)]
            pub fn adc7_adc1_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC7_ADC1_1X)
            }
            ///ADC Differential Inputs Postive pin 0 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn adc0_adc2_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC0_ADC2_1X)
            }
            ///ADC Differential Inputs Postive pin 1 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn adc1_adc2_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC1_ADC2_1X)
            }
            ///ADC Differential Inputs Postive pin 2 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn adc2_adc2_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC2_ADC2_1X)
            }
            ///ADC Differential Inputs Postive pin 3 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn adc3_adc2_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC3_ADC2_1X)
            }
            ///ADC Differential Inputs Postive pin 4 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn adc4_adc2_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC4_ADC2_1X)
            }
            ///ADC Differential Inputs Postive pin 5 Negative pin 2 1x Gain
            #[inline(always)]
            pub fn adc5_adc2_1x(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC5_ADC2_1X)
            }
            ///Internal Reference (VBG)
            #[inline(always)]
            pub fn adc_vbg(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC_VBG)
            }
            ///0V (GND)
            #[inline(always)]
            pub fn adc_gnd(self) -> &'a mut crate::W<REG> {
                self.variant(MUX_A::ADC_GND)
            }
        }
        ///Field `ADLAR` reader - Left Adjust Result
        pub type ADLAR_R = crate::BitReader;
        ///Field `ADLAR` writer - Left Adjust Result
        pub type ADLAR_W<'a, REG> = crate::BitWriter<'a, REG>;
        /**Reference Selection Bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum REFS_A {
            ///0: Aref Internal Vref turned off
            AREF = 0,
            ///1: AVcc with external capacitor at AREF pin
            AVCC = 1,
            ///3: Internal 1.1V Voltage Reference with external capacitor at AREF pin
            INTERNAL = 3,
        }
        impl From<REFS_A> for u8 {
            #[inline(always)]
            fn from(variant: REFS_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for REFS_A {
            type Ux = u8;
        }
        impl crate::IsEnum for REFS_A {}
        ///Field `REFS` reader - Reference Selection Bits
        pub type REFS_R = crate::FieldReader<REFS_A>;
        impl REFS_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<REFS_A> {
                match self.bits {
                    0 => Some(REFS_A::AREF),
                    1 => Some(REFS_A::AVCC),
                    3 => Some(REFS_A::INTERNAL),
                    _ => None,
                }
            }
            ///Aref Internal Vref turned off
            #[inline(always)]
            pub fn is_aref(&self) -> bool {
                *self == REFS_A::AREF
            }
            ///AVcc with external capacitor at AREF pin
            #[inline(always)]
            pub fn is_avcc(&self) -> bool {
                *self == REFS_A::AVCC
            }
            ///Internal 1.1V Voltage Reference with external capacitor at AREF pin
            #[inline(always)]
            pub fn is_internal(&self) -> bool {
                *self == REFS_A::INTERNAL
            }
        }
        ///Field `REFS` writer - Reference Selection Bits
        pub type REFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, REFS_A>;
        impl<'a, REG> REFS_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Aref Internal Vref turned off
            #[inline(always)]
            pub fn aref(self) -> &'a mut crate::W<REG> {
                self.variant(REFS_A::AREF)
            }
            ///AVcc with external capacitor at AREF pin
            #[inline(always)]
            pub fn avcc(self) -> &'a mut crate::W<REG> {
                self.variant(REFS_A::AVCC)
            }
            ///Internal 1.1V Voltage Reference with external capacitor at AREF pin
            #[inline(always)]
            pub fn internal(self) -> &'a mut crate::W<REG> {
                self.variant(REFS_A::INTERNAL)
            }
        }
        impl R {
            ///Bits 0:4 - Analog Channel and Gain Selection Bits
            #[inline(always)]
            pub fn mux(&self) -> MUX_R {
                MUX_R::new(self.bits & 0x1f)
            }
            ///Bit 5 - Left Adjust Result
            #[inline(always)]
            pub fn adlar(&self) -> ADLAR_R {
                ADLAR_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bits 6:7 - Reference Selection Bits
            #[inline(always)]
            pub fn refs(&self) -> REFS_R {
                REFS_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bits 0:4 - Analog Channel and Gain Selection Bits
            #[inline(always)]
            pub fn mux(&mut self) -> MUX_W<'_, ADMUX_SPEC> {
                MUX_W::new(self, 0)
            }
            ///Bit 5 - Left Adjust Result
            #[inline(always)]
            pub fn adlar(&mut self) -> ADLAR_W<'_, ADMUX_SPEC> {
                ADLAR_W::new(self, 5)
            }
            ///Bits 6:7 - Reference Selection Bits
            #[inline(always)]
            pub fn refs(&mut self) -> REFS_W<'_, ADMUX_SPEC> {
                REFS_W::new(self, 6)
            }
        }
        /**The ADC multiplexer Selection Register

You can [`read`](crate::Reg::read) this register and get [`admux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`admux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ADMUX_SPEC;
        impl crate::RegisterSpec for ADMUX_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`admux::R`](R) reader structure
        impl crate::Readable for ADMUX_SPEC {}
        ///`write(|w| ..)` method takes [`admux::W`](W) writer structure
        impl crate::Writable for ADMUX_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets ADMUX to value 0
        impl crate::Resettable for ADMUX_SPEC {}
    }
    /**DIDR0 (rw) register accessor: Digital Input Disable Register

You can [`read`](crate::Reg::read) this register and get [`didr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`didr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@didr0`] module*/
    pub type DIDR0 = crate::Reg<didr0::DIDR0_SPEC>;
    ///Digital Input Disable Register
    pub mod didr0 {
        ///Register `DIDR0` reader
        pub type R = crate::R<DIDR0_SPEC>;
        ///Register `DIDR0` writer
        pub type W = crate::W<DIDR0_SPEC>;
        ///Field `ADC0D` reader - No Description.
        pub type ADC0D_R = crate::BitReader;
        ///Field `ADC0D` writer - No Description.
        pub type ADC0D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC1D` reader - No Description.
        pub type ADC1D_R = crate::BitReader;
        ///Field `ADC1D` writer - No Description.
        pub type ADC1D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC2D` reader - No Description.
        pub type ADC2D_R = crate::BitReader;
        ///Field `ADC2D` writer - No Description.
        pub type ADC2D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC3D` reader - No Description.
        pub type ADC3D_R = crate::BitReader;
        ///Field `ADC3D` writer - No Description.
        pub type ADC3D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC4D` reader - No Description.
        pub type ADC4D_R = crate::BitReader;
        ///Field `ADC4D` writer - No Description.
        pub type ADC4D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC5D` reader - No Description.
        pub type ADC5D_R = crate::BitReader;
        ///Field `ADC5D` writer - No Description.
        pub type ADC5D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC6D` reader - No Description.
        pub type ADC6D_R = crate::BitReader;
        ///Field `ADC6D` writer - No Description.
        pub type ADC6D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC7D` reader - No Description.
        pub type ADC7D_R = crate::BitReader;
        ///Field `ADC7D` writer - No Description.
        pub type ADC7D_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - No Description.
            #[inline(always)]
            pub fn adc0d(&self) -> ADC0D_R {
                ADC0D_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - No Description.
            #[inline(always)]
            pub fn adc1d(&self) -> ADC1D_R {
                ADC1D_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - No Description.
            #[inline(always)]
            pub fn adc2d(&self) -> ADC2D_R {
                ADC2D_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - No Description.
            #[inline(always)]
            pub fn adc3d(&self) -> ADC3D_R {
                ADC3D_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - No Description.
            #[inline(always)]
            pub fn adc4d(&self) -> ADC4D_R {
                ADC4D_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - No Description.
            #[inline(always)]
            pub fn adc5d(&self) -> ADC5D_R {
                ADC5D_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - No Description.
            #[inline(always)]
            pub fn adc6d(&self) -> ADC6D_R {
                ADC6D_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - No Description.
            #[inline(always)]
            pub fn adc7d(&self) -> ADC7D_R {
                ADC7D_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - No Description.
            #[inline(always)]
            pub fn adc0d(&mut self) -> ADC0D_W<'_, DIDR0_SPEC> {
                ADC0D_W::new(self, 0)
            }
            ///Bit 1 - No Description.
            #[inline(always)]
            pub fn adc1d(&mut self) -> ADC1D_W<'_, DIDR0_SPEC> {
                ADC1D_W::new(self, 1)
            }
            ///Bit 2 - No Description.
            #[inline(always)]
            pub fn adc2d(&mut self) -> ADC2D_W<'_, DIDR0_SPEC> {
                ADC2D_W::new(self, 2)
            }
            ///Bit 3 - No Description.
            #[inline(always)]
            pub fn adc3d(&mut self) -> ADC3D_W<'_, DIDR0_SPEC> {
                ADC3D_W::new(self, 3)
            }
            ///Bit 4 - No Description.
            #[inline(always)]
            pub fn adc4d(&mut self) -> ADC4D_W<'_, DIDR0_SPEC> {
                ADC4D_W::new(self, 4)
            }
            ///Bit 5 - No Description.
            #[inline(always)]
            pub fn adc5d(&mut self) -> ADC5D_W<'_, DIDR0_SPEC> {
                ADC5D_W::new(self, 5)
            }
            ///Bit 6 - No Description.
            #[inline(always)]
            pub fn adc6d(&mut self) -> ADC6D_W<'_, DIDR0_SPEC> {
                ADC6D_W::new(self, 6)
            }
            ///Bit 7 - No Description.
            #[inline(always)]
            pub fn adc7d(&mut self) -> ADC7D_W<'_, DIDR0_SPEC> {
                ADC7D_W::new(self, 7)
            }
        }
        /**Digital Input Disable Register

You can [`read`](crate::Reg::read) this register and get [`didr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`didr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DIDR0_SPEC;
        impl crate::RegisterSpec for DIDR0_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`didr0::R`](R) reader structure
        impl crate::Readable for DIDR0_SPEC {}
        ///`write(|w| ..)` method takes [`didr0::W`](W) writer structure
        impl crate::Writable for DIDR0_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DIDR0 to value 0
        impl crate::Resettable for DIDR0_SPEC {}
    }
    /**DIDR2 (rw) register accessor: Digital Input Disable Register

You can [`read`](crate::Reg::read) this register and get [`didr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`didr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@didr2`] module*/
    pub type DIDR2 = crate::Reg<didr2::DIDR2_SPEC>;
    ///Digital Input Disable Register
    pub mod didr2 {
        ///Register `DIDR2` reader
        pub type R = crate::R<DIDR2_SPEC>;
        ///Register `DIDR2` writer
        pub type W = crate::W<DIDR2_SPEC>;
        ///Field `ADC8D` reader - No Description.
        pub type ADC8D_R = crate::BitReader;
        ///Field `ADC8D` writer - No Description.
        pub type ADC8D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC9D` reader - No Description.
        pub type ADC9D_R = crate::BitReader;
        ///Field `ADC9D` writer - No Description.
        pub type ADC9D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC10D` reader - No Description.
        pub type ADC10D_R = crate::BitReader;
        ///Field `ADC10D` writer - No Description.
        pub type ADC10D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC11D` reader - No Description.
        pub type ADC11D_R = crate::BitReader;
        ///Field `ADC11D` writer - No Description.
        pub type ADC11D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC12D` reader - No Description.
        pub type ADC12D_R = crate::BitReader;
        ///Field `ADC12D` writer - No Description.
        pub type ADC12D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC13D` reader - No Description.
        pub type ADC13D_R = crate::BitReader;
        ///Field `ADC13D` writer - No Description.
        pub type ADC13D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC14D` reader - No Description.
        pub type ADC14D_R = crate::BitReader;
        ///Field `ADC14D` writer - No Description.
        pub type ADC14D_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ADC15D` reader - No Description.
        pub type ADC15D_R = crate::BitReader;
        ///Field `ADC15D` writer - No Description.
        pub type ADC15D_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - No Description.
            #[inline(always)]
            pub fn adc8d(&self) -> ADC8D_R {
                ADC8D_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - No Description.
            #[inline(always)]
            pub fn adc9d(&self) -> ADC9D_R {
                ADC9D_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - No Description.
            #[inline(always)]
            pub fn adc10d(&self) -> ADC10D_R {
                ADC10D_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - No Description.
            #[inline(always)]
            pub fn adc11d(&self) -> ADC11D_R {
                ADC11D_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - No Description.
            #[inline(always)]
            pub fn adc12d(&self) -> ADC12D_R {
                ADC12D_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - No Description.
            #[inline(always)]
            pub fn adc13d(&self) -> ADC13D_R {
                ADC13D_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - No Description.
            #[inline(always)]
            pub fn adc14d(&self) -> ADC14D_R {
                ADC14D_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - No Description.
            #[inline(always)]
            pub fn adc15d(&self) -> ADC15D_R {
                ADC15D_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - No Description.
            #[inline(always)]
            pub fn adc8d(&mut self) -> ADC8D_W<'_, DIDR2_SPEC> {
                ADC8D_W::new(self, 0)
            }
            ///Bit 1 - No Description.
            #[inline(always)]
            pub fn adc9d(&mut self) -> ADC9D_W<'_, DIDR2_SPEC> {
                ADC9D_W::new(self, 1)
            }
            ///Bit 2 - No Description.
            #[inline(always)]
            pub fn adc10d(&mut self) -> ADC10D_W<'_, DIDR2_SPEC> {
                ADC10D_W::new(self, 2)
            }
            ///Bit 3 - No Description.
            #[inline(always)]
            pub fn adc11d(&mut self) -> ADC11D_W<'_, DIDR2_SPEC> {
                ADC11D_W::new(self, 3)
            }
            ///Bit 4 - No Description.
            #[inline(always)]
            pub fn adc12d(&mut self) -> ADC12D_W<'_, DIDR2_SPEC> {
                ADC12D_W::new(self, 4)
            }
            ///Bit 5 - No Description.
            #[inline(always)]
            pub fn adc13d(&mut self) -> ADC13D_W<'_, DIDR2_SPEC> {
                ADC13D_W::new(self, 5)
            }
            ///Bit 6 - No Description.
            #[inline(always)]
            pub fn adc14d(&mut self) -> ADC14D_W<'_, DIDR2_SPEC> {
                ADC14D_W::new(self, 6)
            }
            ///Bit 7 - No Description.
            #[inline(always)]
            pub fn adc15d(&mut self) -> ADC15D_W<'_, DIDR2_SPEC> {
                ADC15D_W::new(self, 7)
            }
        }
        /**Digital Input Disable Register

You can [`read`](crate::Reg::read) this register and get [`didr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`didr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DIDR2_SPEC;
        impl crate::RegisterSpec for DIDR2_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`didr2::R`](R) reader structure
        impl crate::Readable for DIDR2_SPEC {}
        ///`write(|w| ..)` method takes [`didr2::W`](W) writer structure
        impl crate::Writable for DIDR2_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DIDR2 to value 0
        impl crate::Resettable for DIDR2_SPEC {}
    }
}
///Bootloader
pub type BOOT_LOAD = crate::Periph<boot_load::RegisterBlock, 0x57>;
impl core::fmt::Debug for BOOT_LOAD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_LOAD").finish()
    }
}
///Bootloader
pub mod boot_load {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        spmcsr: SPMCSR,
    }
    impl RegisterBlock {
        ///0x00 - Store Program Memory Control Register
        #[inline(always)]
        pub const fn spmcsr(&self) -> &SPMCSR {
            &self.spmcsr
        }
    }
    /**SPMCSR (rw) register accessor: Store Program Memory Control Register

You can [`read`](crate::Reg::read) this register and get [`spmcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spmcsr`] module*/
    pub type SPMCSR = crate::Reg<spmcsr::SPMCSR_SPEC>;
    ///Store Program Memory Control Register
    pub mod spmcsr {
        ///Register `SPMCSR` reader
        pub type R = crate::R<SPMCSR_SPEC>;
        ///Register `SPMCSR` writer
        pub type W = crate::W<SPMCSR_SPEC>;
        ///Field `SPMEN` reader - Store Program Memory Enable
        pub type SPMEN_R = crate::BitReader;
        ///Field `SPMEN` writer - Store Program Memory Enable
        pub type SPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PGERS` reader - Page Erase
        pub type PGERS_R = crate::BitReader;
        ///Field `PGERS` writer - Page Erase
        pub type PGERS_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PGWRT` reader - Page Write
        pub type PGWRT_R = crate::BitReader;
        ///Field `PGWRT` writer - Page Write
        pub type PGWRT_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `BLBSET` reader - Boot Lock Bit Set
        pub type BLBSET_R = crate::BitReader;
        ///Field `BLBSET` writer - Boot Lock Bit Set
        pub type BLBSET_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RWWSRE` reader - Read While Write section read enable
        pub type RWWSRE_R = crate::BitReader;
        ///Field `RWWSRE` writer - Read While Write section read enable
        pub type RWWSRE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `SIGRD` reader - Signature Row Read
        pub type SIGRD_R = crate::BitReader;
        ///Field `SIGRD` writer - Signature Row Read
        pub type SIGRD_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RWWSB` reader - Read While Write Section Busy
        pub type RWWSB_R = crate::BitReader;
        ///Field `RWWSB` writer - Read While Write Section Busy
        pub type RWWSB_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `SPMIE` reader - SPM Interrupt Enable
        pub type SPMIE_R = crate::BitReader;
        ///Field `SPMIE` writer - SPM Interrupt Enable
        pub type SPMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Store Program Memory Enable
            #[inline(always)]
            pub fn spmen(&self) -> SPMEN_R {
                SPMEN_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Page Erase
            #[inline(always)]
            pub fn pgers(&self) -> PGERS_R {
                PGERS_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Page Write
            #[inline(always)]
            pub fn pgwrt(&self) -> PGWRT_R {
                PGWRT_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Boot Lock Bit Set
            #[inline(always)]
            pub fn blbset(&self) -> BLBSET_R {
                BLBSET_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Read While Write section read enable
            #[inline(always)]
            pub fn rwwsre(&self) -> RWWSRE_R {
                RWWSRE_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Signature Row Read
            #[inline(always)]
            pub fn sigrd(&self) -> SIGRD_R {
                SIGRD_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Read While Write Section Busy
            #[inline(always)]
            pub fn rwwsb(&self) -> RWWSB_R {
                RWWSB_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - SPM Interrupt Enable
            #[inline(always)]
            pub fn spmie(&self) -> SPMIE_R {
                SPMIE_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Store Program Memory Enable
            #[inline(always)]
            pub fn spmen(&mut self) -> SPMEN_W<'_, SPMCSR_SPEC> {
                SPMEN_W::new(self, 0)
            }
            ///Bit 1 - Page Erase
            #[inline(always)]
            pub fn pgers(&mut self) -> PGERS_W<'_, SPMCSR_SPEC> {
                PGERS_W::new(self, 1)
            }
            ///Bit 2 - Page Write
            #[inline(always)]
            pub fn pgwrt(&mut self) -> PGWRT_W<'_, SPMCSR_SPEC> {
                PGWRT_W::new(self, 2)
            }
            ///Bit 3 - Boot Lock Bit Set
            #[inline(always)]
            pub fn blbset(&mut self) -> BLBSET_W<'_, SPMCSR_SPEC> {
                BLBSET_W::new(self, 3)
            }
            ///Bit 4 - Read While Write section read enable
            #[inline(always)]
            pub fn rwwsre(&mut self) -> RWWSRE_W<'_, SPMCSR_SPEC> {
                RWWSRE_W::new(self, 4)
            }
            ///Bit 5 - Signature Row Read
            #[inline(always)]
            pub fn sigrd(&mut self) -> SIGRD_W<'_, SPMCSR_SPEC> {
                SIGRD_W::new(self, 5)
            }
            ///Bit 6 - Read While Write Section Busy
            #[inline(always)]
            pub fn rwwsb(&mut self) -> RWWSB_W<'_, SPMCSR_SPEC> {
                RWWSB_W::new(self, 6)
            }
            ///Bit 7 - SPM Interrupt Enable
            #[inline(always)]
            pub fn spmie(&mut self) -> SPMIE_W<'_, SPMCSR_SPEC> {
                SPMIE_W::new(self, 7)
            }
        }
        /**Store Program Memory Control Register

You can [`read`](crate::Reg::read) this register and get [`spmcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct SPMCSR_SPEC;
        impl crate::RegisterSpec for SPMCSR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`spmcsr::R`](R) reader structure
        impl crate::Readable for SPMCSR_SPEC {}
        ///`write(|w| ..)` method takes [`spmcsr::W`](W) writer structure
        impl crate::Writable for SPMCSR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets SPMCSR to value 0
        impl crate::Resettable for SPMCSR_SPEC {}
    }
}
///CPU Registers
pub type CPU = crate::Periph<cpu::RegisterBlock, 0x3e>;
impl core::fmt::Debug for CPU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU").finish()
    }
}
///CPU Registers
pub mod cpu {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        gpior0: GPIOR0,
        _reserved1: [u8; 0x0b],
        gpior1: GPIOR1,
        gpior2: GPIOR2,
        _reserved3: [u8; 0x07],
        smcr: SMCR,
        mcusr: MCUSR,
        mcucr: MCUCR,
        _reserved6: [u8; 0x05],
        rampz: RAMPZ,
        eind: EIND,
        _reserved8: [u8; 0x04],
        clkpr: CLKPR,
        _reserved9: [u8; 0x02],
        prr0: PRR0,
        prr1: PRR1,
        osccal: OSCCAL,
        _reserved12: [u8; 0x0d],
        xmcra: XMCRA,
        xmcrb: XMCRB,
    }
    impl RegisterBlock {
        ///0x00 - General Purpose IO Register 0
        #[inline(always)]
        pub const fn gpior0(&self) -> &GPIOR0 {
            &self.gpior0
        }
        ///0x0c - General Purpose IO Register 1
        #[inline(always)]
        pub const fn gpior1(&self) -> &GPIOR1 {
            &self.gpior1
        }
        ///0x0d - General Purpose IO Register 2
        #[inline(always)]
        pub const fn gpior2(&self) -> &GPIOR2 {
            &self.gpior2
        }
        ///0x15 - Sleep Mode Control Register
        #[inline(always)]
        pub const fn smcr(&self) -> &SMCR {
            &self.smcr
        }
        ///0x16 - MCU Status Register
        #[inline(always)]
        pub const fn mcusr(&self) -> &MCUSR {
            &self.mcusr
        }
        ///0x17 - MCU Control Register
        #[inline(always)]
        pub const fn mcucr(&self) -> &MCUCR {
            &self.mcucr
        }
        ///0x1d - RAM Page Z Select Register
        #[inline(always)]
        pub const fn rampz(&self) -> &RAMPZ {
            &self.rampz
        }
        ///0x1e - Extended Indirect Register
        #[inline(always)]
        pub const fn eind(&self) -> &EIND {
            &self.eind
        }
        ///0x23 - No Description.
        #[inline(always)]
        pub const fn clkpr(&self) -> &CLKPR {
            &self.clkpr
        }
        ///0x26 - Power Reduction Register0
        #[inline(always)]
        pub const fn prr0(&self) -> &PRR0 {
            &self.prr0
        }
        ///0x27 - Power Reduction Register1
        #[inline(always)]
        pub const fn prr1(&self) -> &PRR1 {
            &self.prr1
        }
        ///0x28 - Oscillator Calibration Value
        #[inline(always)]
        pub const fn osccal(&self) -> &OSCCAL {
            &self.osccal
        }
        ///0x36 - External Memory Control Register A
        #[inline(always)]
        pub const fn xmcra(&self) -> &XMCRA {
            &self.xmcra
        }
        ///0x37 - External Memory Control Register B
        #[inline(always)]
        pub const fn xmcrb(&self) -> &XMCRB {
            &self.xmcrb
        }
    }
    /**CLKPR (rw) register accessor: No Description.

You can [`read`](crate::Reg::read) this register and get [`clkpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clkpr`] module*/
    pub type CLKPR = crate::Reg<clkpr::CLKPR_SPEC>;
    ///No Description.
    pub mod clkpr {
        ///Register `CLKPR` reader
        pub type R = crate::R<CLKPR_SPEC>;
        ///Register `CLKPR` writer
        pub type W = crate::W<CLKPR_SPEC>;
        /**No Description.

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum CLKPS_A {
            ///0: 1
            VAL_0X00 = 0,
            ///1: 2
            VAL_0X01 = 1,
            ///2: 4
            VAL_0X02 = 2,
            ///3: 8
            VAL_0X03 = 3,
            ///4: 16
            VAL_0X04 = 4,
            ///5: 32
            VAL_0X05 = 5,
            ///6: 64
            VAL_0X06 = 6,
            ///7: 128
            VAL_0X07 = 7,
            ///8: 256
            VAL_0X08 = 8,
        }
        impl From<CLKPS_A> for u8 {
            #[inline(always)]
            fn from(variant: CLKPS_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for CLKPS_A {
            type Ux = u8;
        }
        impl crate::IsEnum for CLKPS_A {}
        ///Field `CLKPS` reader - No Description.
        pub type CLKPS_R = crate::FieldReader<CLKPS_A>;
        impl CLKPS_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<CLKPS_A> {
                match self.bits {
                    0 => Some(CLKPS_A::VAL_0X00),
                    1 => Some(CLKPS_A::VAL_0X01),
                    2 => Some(CLKPS_A::VAL_0X02),
                    3 => Some(CLKPS_A::VAL_0X03),
                    4 => Some(CLKPS_A::VAL_0X04),
                    5 => Some(CLKPS_A::VAL_0X05),
                    6 => Some(CLKPS_A::VAL_0X06),
                    7 => Some(CLKPS_A::VAL_0X07),
                    8 => Some(CLKPS_A::VAL_0X08),
                    _ => None,
                }
            }
            ///1
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == CLKPS_A::VAL_0X00
            }
            ///2
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == CLKPS_A::VAL_0X01
            }
            ///4
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == CLKPS_A::VAL_0X02
            }
            ///8
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == CLKPS_A::VAL_0X03
            }
            ///16
            #[inline(always)]
            pub fn is_val_0x04(&self) -> bool {
                *self == CLKPS_A::VAL_0X04
            }
            ///32
            #[inline(always)]
            pub fn is_val_0x05(&self) -> bool {
                *self == CLKPS_A::VAL_0X05
            }
            ///64
            #[inline(always)]
            pub fn is_val_0x06(&self) -> bool {
                *self == CLKPS_A::VAL_0X06
            }
            ///128
            #[inline(always)]
            pub fn is_val_0x07(&self) -> bool {
                *self == CLKPS_A::VAL_0X07
            }
            ///256
            #[inline(always)]
            pub fn is_val_0x08(&self) -> bool {
                *self == CLKPS_A::VAL_0X08
            }
        }
        ///Field `CLKPS` writer - No Description.
        pub type CLKPS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKPS_A>;
        impl<'a, REG> CLKPS_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///1
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(CLKPS_A::VAL_0X00)
            }
            ///2
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(CLKPS_A::VAL_0X01)
            }
            ///4
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(CLKPS_A::VAL_0X02)
            }
            ///8
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(CLKPS_A::VAL_0X03)
            }
            ///16
            #[inline(always)]
            pub fn val_0x04(self) -> &'a mut crate::W<REG> {
                self.variant(CLKPS_A::VAL_0X04)
            }
            ///32
            #[inline(always)]
            pub fn val_0x05(self) -> &'a mut crate::W<REG> {
                self.variant(CLKPS_A::VAL_0X05)
            }
            ///64
            #[inline(always)]
            pub fn val_0x06(self) -> &'a mut crate::W<REG> {
                self.variant(CLKPS_A::VAL_0X06)
            }
            ///128
            #[inline(always)]
            pub fn val_0x07(self) -> &'a mut crate::W<REG> {
                self.variant(CLKPS_A::VAL_0X07)
            }
            ///256
            #[inline(always)]
            pub fn val_0x08(self) -> &'a mut crate::W<REG> {
                self.variant(CLKPS_A::VAL_0X08)
            }
        }
        ///Field `CLKPCE` reader - No Description.
        pub type CLKPCE_R = crate::BitReader;
        ///Field `CLKPCE` writer - No Description.
        pub type CLKPCE_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:3 - No Description.
            #[inline(always)]
            pub fn clkps(&self) -> CLKPS_R {
                CLKPS_R::new(self.bits & 0x0f)
            }
            ///Bit 7 - No Description.
            #[inline(always)]
            pub fn clkpce(&self) -> CLKPCE_R {
                CLKPCE_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:3 - No Description.
            #[inline(always)]
            pub fn clkps(&mut self) -> CLKPS_W<'_, CLKPR_SPEC> {
                CLKPS_W::new(self, 0)
            }
            ///Bit 7 - No Description.
            #[inline(always)]
            pub fn clkpce(&mut self) -> CLKPCE_W<'_, CLKPR_SPEC> {
                CLKPCE_W::new(self, 7)
            }
        }
        /**No Description.

You can [`read`](crate::Reg::read) this register and get [`clkpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct CLKPR_SPEC;
        impl crate::RegisterSpec for CLKPR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`clkpr::R`](R) reader structure
        impl crate::Readable for CLKPR_SPEC {}
        ///`write(|w| ..)` method takes [`clkpr::W`](W) writer structure
        impl crate::Writable for CLKPR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets CLKPR to value 0
        impl crate::Resettable for CLKPR_SPEC {}
    }
    /**EIND (rw) register accessor: Extended Indirect Register

You can [`read`](crate::Reg::read) this register and get [`eind::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eind::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eind`] module*/
    pub type EIND = crate::Reg<eind::EIND_SPEC>;
    ///Extended Indirect Register
    pub mod eind {
        ///Register `EIND` reader
        pub type R = crate::R<EIND_SPEC>;
        ///Register `EIND` writer
        pub type W = crate::W<EIND_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Extended Indirect Register

You can [`read`](crate::Reg::read) this register and get [`eind::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eind::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct EIND_SPEC;
        impl crate::RegisterSpec for EIND_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`eind::R`](R) reader structure
        impl crate::Readable for EIND_SPEC {}
        ///`write(|w| ..)` method takes [`eind::W`](W) writer structure
        impl crate::Writable for EIND_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets EIND to value 0
        impl crate::Resettable for EIND_SPEC {}
    }
    /**GPIOR0 (rw) register accessor: General Purpose IO Register 0

You can [`read`](crate::Reg::read) this register and get [`gpior0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpior0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpior0`] module*/
    pub type GPIOR0 = crate::Reg<gpior0::GPIOR0_SPEC>;
    ///General Purpose IO Register 0
    pub mod gpior0 {
        ///Register `GPIOR0` reader
        pub type R = crate::R<GPIOR0_SPEC>;
        ///Register `GPIOR0` writer
        pub type W = crate::W<GPIOR0_SPEC>;
        ///Field `GPIOR00` reader - General Purpose IO Register 0 bit 0
        pub type GPIOR00_R = crate::BitReader;
        ///Field `GPIOR00` writer - General Purpose IO Register 0 bit 0
        pub type GPIOR00_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `GPIOR01` reader - General Purpose IO Register 0 bit 1
        pub type GPIOR01_R = crate::BitReader;
        ///Field `GPIOR01` writer - General Purpose IO Register 0 bit 1
        pub type GPIOR01_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `GPIOR02` reader - General Purpose IO Register 0 bit 2
        pub type GPIOR02_R = crate::BitReader;
        ///Field `GPIOR02` writer - General Purpose IO Register 0 bit 2
        pub type GPIOR02_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `GPIOR03` reader - General Purpose IO Register 0 bit 3
        pub type GPIOR03_R = crate::BitReader;
        ///Field `GPIOR03` writer - General Purpose IO Register 0 bit 3
        pub type GPIOR03_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `GPIOR04` reader - General Purpose IO Register 0 bit 4
        pub type GPIOR04_R = crate::BitReader;
        ///Field `GPIOR04` writer - General Purpose IO Register 0 bit 4
        pub type GPIOR04_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `GPIOR05` reader - General Purpose IO Register 0 bit 5
        pub type GPIOR05_R = crate::BitReader;
        ///Field `GPIOR05` writer - General Purpose IO Register 0 bit 5
        pub type GPIOR05_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `GPIOR06` reader - General Purpose IO Register 0 bit 6
        pub type GPIOR06_R = crate::BitReader;
        ///Field `GPIOR06` writer - General Purpose IO Register 0 bit 6
        pub type GPIOR06_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `GPIOR07` reader - General Purpose IO Register 0 bit 7
        pub type GPIOR07_R = crate::BitReader;
        ///Field `GPIOR07` writer - General Purpose IO Register 0 bit 7
        pub type GPIOR07_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - General Purpose IO Register 0 bit 0
            #[inline(always)]
            pub fn gpior00(&self) -> GPIOR00_R {
                GPIOR00_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - General Purpose IO Register 0 bit 1
            #[inline(always)]
            pub fn gpior01(&self) -> GPIOR01_R {
                GPIOR01_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - General Purpose IO Register 0 bit 2
            #[inline(always)]
            pub fn gpior02(&self) -> GPIOR02_R {
                GPIOR02_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - General Purpose IO Register 0 bit 3
            #[inline(always)]
            pub fn gpior03(&self) -> GPIOR03_R {
                GPIOR03_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - General Purpose IO Register 0 bit 4
            #[inline(always)]
            pub fn gpior04(&self) -> GPIOR04_R {
                GPIOR04_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - General Purpose IO Register 0 bit 5
            #[inline(always)]
            pub fn gpior05(&self) -> GPIOR05_R {
                GPIOR05_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - General Purpose IO Register 0 bit 6
            #[inline(always)]
            pub fn gpior06(&self) -> GPIOR06_R {
                GPIOR06_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - General Purpose IO Register 0 bit 7
            #[inline(always)]
            pub fn gpior07(&self) -> GPIOR07_R {
                GPIOR07_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - General Purpose IO Register 0 bit 0
            #[inline(always)]
            pub fn gpior00(&mut self) -> GPIOR00_W<'_, GPIOR0_SPEC> {
                GPIOR00_W::new(self, 0)
            }
            ///Bit 1 - General Purpose IO Register 0 bit 1
            #[inline(always)]
            pub fn gpior01(&mut self) -> GPIOR01_W<'_, GPIOR0_SPEC> {
                GPIOR01_W::new(self, 1)
            }
            ///Bit 2 - General Purpose IO Register 0 bit 2
            #[inline(always)]
            pub fn gpior02(&mut self) -> GPIOR02_W<'_, GPIOR0_SPEC> {
                GPIOR02_W::new(self, 2)
            }
            ///Bit 3 - General Purpose IO Register 0 bit 3
            #[inline(always)]
            pub fn gpior03(&mut self) -> GPIOR03_W<'_, GPIOR0_SPEC> {
                GPIOR03_W::new(self, 3)
            }
            ///Bit 4 - General Purpose IO Register 0 bit 4
            #[inline(always)]
            pub fn gpior04(&mut self) -> GPIOR04_W<'_, GPIOR0_SPEC> {
                GPIOR04_W::new(self, 4)
            }
            ///Bit 5 - General Purpose IO Register 0 bit 5
            #[inline(always)]
            pub fn gpior05(&mut self) -> GPIOR05_W<'_, GPIOR0_SPEC> {
                GPIOR05_W::new(self, 5)
            }
            ///Bit 6 - General Purpose IO Register 0 bit 6
            #[inline(always)]
            pub fn gpior06(&mut self) -> GPIOR06_W<'_, GPIOR0_SPEC> {
                GPIOR06_W::new(self, 6)
            }
            ///Bit 7 - General Purpose IO Register 0 bit 7
            #[inline(always)]
            pub fn gpior07(&mut self) -> GPIOR07_W<'_, GPIOR0_SPEC> {
                GPIOR07_W::new(self, 7)
            }
        }
        /**General Purpose IO Register 0

You can [`read`](crate::Reg::read) this register and get [`gpior0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpior0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct GPIOR0_SPEC;
        impl crate::RegisterSpec for GPIOR0_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`gpior0::R`](R) reader structure
        impl crate::Readable for GPIOR0_SPEC {}
        ///`write(|w| ..)` method takes [`gpior0::W`](W) writer structure
        impl crate::Writable for GPIOR0_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets GPIOR0 to value 0
        impl crate::Resettable for GPIOR0_SPEC {}
    }
    /**GPIOR1 (rw) register accessor: General Purpose IO Register 1

You can [`read`](crate::Reg::read) this register and get [`gpior1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpior1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpior1`] module*/
    pub type GPIOR1 = crate::Reg<gpior1::GPIOR1_SPEC>;
    ///General Purpose IO Register 1
    pub mod gpior1 {
        ///Register `GPIOR1` reader
        pub type R = crate::R<GPIOR1_SPEC>;
        ///Register `GPIOR1` writer
        pub type W = crate::W<GPIOR1_SPEC>;
        ///Field `GPIOR` reader - General Purpose IO Register 1 bis
        pub type GPIOR_R = crate::FieldReader;
        ///Field `GPIOR` writer - General Purpose IO Register 1 bis
        pub type GPIOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
        impl R {
            ///Bits 0:7 - General Purpose IO Register 1 bis
            #[inline(always)]
            pub fn gpior(&self) -> GPIOR_R {
                GPIOR_R::new(self.bits)
            }
        }
        impl W {
            ///Bits 0:7 - General Purpose IO Register 1 bis
            #[inline(always)]
            pub fn gpior(&mut self) -> GPIOR_W<'_, GPIOR1_SPEC> {
                GPIOR_W::new(self, 0)
            }
        }
        /**General Purpose IO Register 1

You can [`read`](crate::Reg::read) this register and get [`gpior1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpior1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct GPIOR1_SPEC;
        impl crate::RegisterSpec for GPIOR1_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`gpior1::R`](R) reader structure
        impl crate::Readable for GPIOR1_SPEC {}
        ///`write(|w| ..)` method takes [`gpior1::W`](W) writer structure
        impl crate::Writable for GPIOR1_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets GPIOR1 to value 0
        impl crate::Resettable for GPIOR1_SPEC {}
    }
    /**GPIOR2 (rw) register accessor: General Purpose IO Register 2

You can [`read`](crate::Reg::read) this register and get [`gpior2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpior2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpior2`] module*/
    pub type GPIOR2 = crate::Reg<gpior2::GPIOR2_SPEC>;
    ///General Purpose IO Register 2
    pub mod gpior2 {
        ///Register `GPIOR2` reader
        pub type R = crate::R<GPIOR2_SPEC>;
        ///Register `GPIOR2` writer
        pub type W = crate::W<GPIOR2_SPEC>;
        ///Field `GPIOR` reader - General Purpose IO Register 2 bis
        pub type GPIOR_R = crate::FieldReader;
        ///Field `GPIOR` writer - General Purpose IO Register 2 bis
        pub type GPIOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
        impl R {
            ///Bits 0:7 - General Purpose IO Register 2 bis
            #[inline(always)]
            pub fn gpior(&self) -> GPIOR_R {
                GPIOR_R::new(self.bits)
            }
        }
        impl W {
            ///Bits 0:7 - General Purpose IO Register 2 bis
            #[inline(always)]
            pub fn gpior(&mut self) -> GPIOR_W<'_, GPIOR2_SPEC> {
                GPIOR_W::new(self, 0)
            }
        }
        /**General Purpose IO Register 2

You can [`read`](crate::Reg::read) this register and get [`gpior2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpior2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct GPIOR2_SPEC;
        impl crate::RegisterSpec for GPIOR2_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`gpior2::R`](R) reader structure
        impl crate::Readable for GPIOR2_SPEC {}
        ///`write(|w| ..)` method takes [`gpior2::W`](W) writer structure
        impl crate::Writable for GPIOR2_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets GPIOR2 to value 0
        impl crate::Resettable for GPIOR2_SPEC {}
    }
    /**MCUCR (rw) register accessor: MCU Control Register

You can [`read`](crate::Reg::read) this register and get [`mcucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mcucr`] module*/
    pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
    ///MCU Control Register
    pub mod mcucr {
        ///Register `MCUCR` reader
        pub type R = crate::R<MCUCR_SPEC>;
        ///Register `MCUCR` writer
        pub type W = crate::W<MCUCR_SPEC>;
        ///Field `IVCE` reader - Interrupt Vector Change Enable
        pub type IVCE_R = crate::BitReader;
        ///Field `IVCE` writer - Interrupt Vector Change Enable
        pub type IVCE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `IVSEL` reader - Interrupt Vector Select
        pub type IVSEL_R = crate::BitReader;
        ///Field `IVSEL` writer - Interrupt Vector Select
        pub type IVSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PUD` reader - Pull-up disable
        pub type PUD_R = crate::BitReader;
        ///Field `PUD` writer - Pull-up disable
        pub type PUD_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `JTD` reader - JTAG Interface Disable
        pub type JTD_R = crate::BitReader;
        ///Field `JTD` writer - JTAG Interface Disable
        pub type JTD_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Interrupt Vector Change Enable
            #[inline(always)]
            pub fn ivce(&self) -> IVCE_R {
                IVCE_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Interrupt Vector Select
            #[inline(always)]
            pub fn ivsel(&self) -> IVSEL_R {
                IVSEL_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 4 - Pull-up disable
            #[inline(always)]
            pub fn pud(&self) -> PUD_R {
                PUD_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 7 - JTAG Interface Disable
            #[inline(always)]
            pub fn jtd(&self) -> JTD_R {
                JTD_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Interrupt Vector Change Enable
            #[inline(always)]
            pub fn ivce(&mut self) -> IVCE_W<'_, MCUCR_SPEC> {
                IVCE_W::new(self, 0)
            }
            ///Bit 1 - Interrupt Vector Select
            #[inline(always)]
            pub fn ivsel(&mut self) -> IVSEL_W<'_, MCUCR_SPEC> {
                IVSEL_W::new(self, 1)
            }
            ///Bit 4 - Pull-up disable
            #[inline(always)]
            pub fn pud(&mut self) -> PUD_W<'_, MCUCR_SPEC> {
                PUD_W::new(self, 4)
            }
            ///Bit 7 - JTAG Interface Disable
            #[inline(always)]
            pub fn jtd(&mut self) -> JTD_W<'_, MCUCR_SPEC> {
                JTD_W::new(self, 7)
            }
        }
        /**MCU Control Register

You can [`read`](crate::Reg::read) this register and get [`mcucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct MCUCR_SPEC;
        impl crate::RegisterSpec for MCUCR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`mcucr::R`](R) reader structure
        impl crate::Readable for MCUCR_SPEC {}
        ///`write(|w| ..)` method takes [`mcucr::W`](W) writer structure
        impl crate::Writable for MCUCR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets MCUCR to value 0
        impl crate::Resettable for MCUCR_SPEC {}
    }
    /**MCUSR (rw) register accessor: MCU Status Register

You can [`read`](crate::Reg::read) this register and get [`mcusr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcusr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mcusr`] module*/
    pub type MCUSR = crate::Reg<mcusr::MCUSR_SPEC>;
    ///MCU Status Register
    pub mod mcusr {
        ///Register `MCUSR` reader
        pub type R = crate::R<MCUSR_SPEC>;
        ///Register `MCUSR` writer
        pub type W = crate::W<MCUSR_SPEC>;
        ///Field `PORF` reader - Power-on reset flag
        pub type PORF_R = crate::BitReader;
        ///Field `PORF` writer - Power-on reset flag
        pub type PORF_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `EXTRF` reader - External Reset Flag
        pub type EXTRF_R = crate::BitReader;
        ///Field `EXTRF` writer - External Reset Flag
        pub type EXTRF_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `BORF` reader - Brown-out Reset Flag
        pub type BORF_R = crate::BitReader;
        ///Field `BORF` writer - Brown-out Reset Flag
        pub type BORF_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `WDRF` reader - Watchdog Reset Flag
        pub type WDRF_R = crate::BitReader;
        ///Field `WDRF` writer - Watchdog Reset Flag
        pub type WDRF_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `JTRF` reader - JTAG Reset Flag
        pub type JTRF_R = crate::BitReader;
        ///Field `JTRF` writer - JTAG Reset Flag
        pub type JTRF_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Power-on reset flag
            #[inline(always)]
            pub fn porf(&self) -> PORF_R {
                PORF_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - External Reset Flag
            #[inline(always)]
            pub fn extrf(&self) -> EXTRF_R {
                EXTRF_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Brown-out Reset Flag
            #[inline(always)]
            pub fn borf(&self) -> BORF_R {
                BORF_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Watchdog Reset Flag
            #[inline(always)]
            pub fn wdrf(&self) -> WDRF_R {
                WDRF_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - JTAG Reset Flag
            #[inline(always)]
            pub fn jtrf(&self) -> JTRF_R {
                JTRF_R::new(((self.bits >> 4) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Power-on reset flag
            #[inline(always)]
            pub fn porf(&mut self) -> PORF_W<'_, MCUSR_SPEC> {
                PORF_W::new(self, 0)
            }
            ///Bit 1 - External Reset Flag
            #[inline(always)]
            pub fn extrf(&mut self) -> EXTRF_W<'_, MCUSR_SPEC> {
                EXTRF_W::new(self, 1)
            }
            ///Bit 2 - Brown-out Reset Flag
            #[inline(always)]
            pub fn borf(&mut self) -> BORF_W<'_, MCUSR_SPEC> {
                BORF_W::new(self, 2)
            }
            ///Bit 3 - Watchdog Reset Flag
            #[inline(always)]
            pub fn wdrf(&mut self) -> WDRF_W<'_, MCUSR_SPEC> {
                WDRF_W::new(self, 3)
            }
            ///Bit 4 - JTAG Reset Flag
            #[inline(always)]
            pub fn jtrf(&mut self) -> JTRF_W<'_, MCUSR_SPEC> {
                JTRF_W::new(self, 4)
            }
        }
        /**MCU Status Register

You can [`read`](crate::Reg::read) this register and get [`mcusr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcusr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct MCUSR_SPEC;
        impl crate::RegisterSpec for MCUSR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`mcusr::R`](R) reader structure
        impl crate::Readable for MCUSR_SPEC {}
        ///`write(|w| ..)` method takes [`mcusr::W`](W) writer structure
        impl crate::Writable for MCUSR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets MCUSR to value 0
        impl crate::Resettable for MCUSR_SPEC {}
    }
    /**OSCCAL (rw) register accessor: Oscillator Calibration Value

You can [`read`](crate::Reg::read) this register and get [`osccal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osccal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@osccal`] module*/
    pub type OSCCAL = crate::Reg<osccal::OSCCAL_SPEC>;
    ///Oscillator Calibration Value
    pub mod osccal {
        ///Register `OSCCAL` reader
        pub type R = crate::R<OSCCAL_SPEC>;
        ///Register `OSCCAL` writer
        pub type W = crate::W<OSCCAL_SPEC>;
        ///Field `OSCCAL` reader - Oscillator Calibration
        pub type OSCCAL_R = crate::FieldReader;
        ///Field `OSCCAL` writer - Oscillator Calibration
        pub type OSCCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
        impl R {
            ///Bits 0:7 - Oscillator Calibration
            #[inline(always)]
            pub fn osccal(&self) -> OSCCAL_R {
                OSCCAL_R::new(self.bits)
            }
        }
        impl W {
            ///Bits 0:7 - Oscillator Calibration
            #[inline(always)]
            pub fn osccal(&mut self) -> OSCCAL_W<'_, OSCCAL_SPEC> {
                OSCCAL_W::new(self, 0)
            }
        }
        /**Oscillator Calibration Value

You can [`read`](crate::Reg::read) this register and get [`osccal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osccal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OSCCAL_SPEC;
        impl crate::RegisterSpec for OSCCAL_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`osccal::R`](R) reader structure
        impl crate::Readable for OSCCAL_SPEC {}
        ///`write(|w| ..)` method takes [`osccal::W`](W) writer structure
        impl crate::Writable for OSCCAL_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OSCCAL to value 0
        impl crate::Resettable for OSCCAL_SPEC {}
    }
    /**PRR0 (rw) register accessor: Power Reduction Register0

You can [`read`](crate::Reg::read) this register and get [`prr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@prr0`] module*/
    pub type PRR0 = crate::Reg<prr0::PRR0_SPEC>;
    ///Power Reduction Register0
    pub mod prr0 {
        ///Register `PRR0` reader
        pub type R = crate::R<PRR0_SPEC>;
        ///Register `PRR0` writer
        pub type W = crate::W<PRR0_SPEC>;
        ///Field `PRADC` reader - Power Reduction ADC
        pub type PRADC_R = crate::BitReader;
        ///Field `PRADC` writer - Power Reduction ADC
        pub type PRADC_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRUSART0` reader - Power Reduction USART0
        pub type PRUSART0_R = crate::BitReader;
        ///Field `PRUSART0` writer - Power Reduction USART0
        pub type PRUSART0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRSPI` reader - Power Reduction Serial Peripheral Interface
        pub type PRSPI_R = crate::BitReader;
        ///Field `PRSPI` writer - Power Reduction Serial Peripheral Interface
        pub type PRSPI_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRTIM1` reader - Power Reduction Timer/Counter1
        pub type PRTIM1_R = crate::BitReader;
        ///Field `PRTIM1` writer - Power Reduction Timer/Counter1
        pub type PRTIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRTIM0` reader - Power Reduction Timer/Counter0
        pub type PRTIM0_R = crate::BitReader;
        ///Field `PRTIM0` writer - Power Reduction Timer/Counter0
        pub type PRTIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRTIM2` reader - Power Reduction Timer/Counter2
        pub type PRTIM2_R = crate::BitReader;
        ///Field `PRTIM2` writer - Power Reduction Timer/Counter2
        pub type PRTIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRTWI` reader - Power Reduction TWI
        pub type PRTWI_R = crate::BitReader;
        ///Field `PRTWI` writer - Power Reduction TWI
        pub type PRTWI_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Power Reduction ADC
            #[inline(always)]
            pub fn pradc(&self) -> PRADC_R {
                PRADC_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Power Reduction USART0
            #[inline(always)]
            pub fn prusart0(&self) -> PRUSART0_R {
                PRUSART0_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Power Reduction Serial Peripheral Interface
            #[inline(always)]
            pub fn prspi(&self) -> PRSPI_R {
                PRSPI_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Power Reduction Timer/Counter1
            #[inline(always)]
            pub fn prtim1(&self) -> PRTIM1_R {
                PRTIM1_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 5 - Power Reduction Timer/Counter0
            #[inline(always)]
            pub fn prtim0(&self) -> PRTIM0_R {
                PRTIM0_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Power Reduction Timer/Counter2
            #[inline(always)]
            pub fn prtim2(&self) -> PRTIM2_R {
                PRTIM2_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Power Reduction TWI
            #[inline(always)]
            pub fn prtwi(&self) -> PRTWI_R {
                PRTWI_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Power Reduction ADC
            #[inline(always)]
            pub fn pradc(&mut self) -> PRADC_W<'_, PRR0_SPEC> {
                PRADC_W::new(self, 0)
            }
            ///Bit 1 - Power Reduction USART0
            #[inline(always)]
            pub fn prusart0(&mut self) -> PRUSART0_W<'_, PRR0_SPEC> {
                PRUSART0_W::new(self, 1)
            }
            ///Bit 2 - Power Reduction Serial Peripheral Interface
            #[inline(always)]
            pub fn prspi(&mut self) -> PRSPI_W<'_, PRR0_SPEC> {
                PRSPI_W::new(self, 2)
            }
            ///Bit 3 - Power Reduction Timer/Counter1
            #[inline(always)]
            pub fn prtim1(&mut self) -> PRTIM1_W<'_, PRR0_SPEC> {
                PRTIM1_W::new(self, 3)
            }
            ///Bit 5 - Power Reduction Timer/Counter0
            #[inline(always)]
            pub fn prtim0(&mut self) -> PRTIM0_W<'_, PRR0_SPEC> {
                PRTIM0_W::new(self, 5)
            }
            ///Bit 6 - Power Reduction Timer/Counter2
            #[inline(always)]
            pub fn prtim2(&mut self) -> PRTIM2_W<'_, PRR0_SPEC> {
                PRTIM2_W::new(self, 6)
            }
            ///Bit 7 - Power Reduction TWI
            #[inline(always)]
            pub fn prtwi(&mut self) -> PRTWI_W<'_, PRR0_SPEC> {
                PRTWI_W::new(self, 7)
            }
        }
        /**Power Reduction Register0

You can [`read`](crate::Reg::read) this register and get [`prr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PRR0_SPEC;
        impl crate::RegisterSpec for PRR0_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`prr0::R`](R) reader structure
        impl crate::Readable for PRR0_SPEC {}
        ///`write(|w| ..)` method takes [`prr0::W`](W) writer structure
        impl crate::Writable for PRR0_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PRR0 to value 0
        impl crate::Resettable for PRR0_SPEC {}
    }
    /**PRR1 (rw) register accessor: Power Reduction Register1

You can [`read`](crate::Reg::read) this register and get [`prr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@prr1`] module*/
    pub type PRR1 = crate::Reg<prr1::PRR1_SPEC>;
    ///Power Reduction Register1
    pub mod prr1 {
        ///Register `PRR1` reader
        pub type R = crate::R<PRR1_SPEC>;
        ///Register `PRR1` writer
        pub type W = crate::W<PRR1_SPEC>;
        ///Field `PRUSART1` reader - Power Reduction USART1
        pub type PRUSART1_R = crate::BitReader;
        ///Field `PRUSART1` writer - Power Reduction USART1
        pub type PRUSART1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRUSART2` reader - Power Reduction USART2
        pub type PRUSART2_R = crate::BitReader;
        ///Field `PRUSART2` writer - Power Reduction USART2
        pub type PRUSART2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRUSART3` reader - Power Reduction USART3
        pub type PRUSART3_R = crate::BitReader;
        ///Field `PRUSART3` writer - Power Reduction USART3
        pub type PRUSART3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRTIM3` reader - Power Reduction Timer/Counter3
        pub type PRTIM3_R = crate::BitReader;
        ///Field `PRTIM3` writer - Power Reduction Timer/Counter3
        pub type PRTIM3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRTIM4` reader - Power Reduction Timer/Counter4
        pub type PRTIM4_R = crate::BitReader;
        ///Field `PRTIM4` writer - Power Reduction Timer/Counter4
        pub type PRTIM4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PRTIM5` reader - Power Reduction Timer/Counter5
        pub type PRTIM5_R = crate::BitReader;
        ///Field `PRTIM5` writer - Power Reduction Timer/Counter5
        pub type PRTIM5_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Power Reduction USART1
            #[inline(always)]
            pub fn prusart1(&self) -> PRUSART1_R {
                PRUSART1_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Power Reduction USART2
            #[inline(always)]
            pub fn prusart2(&self) -> PRUSART2_R {
                PRUSART2_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Power Reduction USART3
            #[inline(always)]
            pub fn prusart3(&self) -> PRUSART3_R {
                PRUSART3_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Power Reduction Timer/Counter3
            #[inline(always)]
            pub fn prtim3(&self) -> PRTIM3_R {
                PRTIM3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Power Reduction Timer/Counter4
            #[inline(always)]
            pub fn prtim4(&self) -> PRTIM4_R {
                PRTIM4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Power Reduction Timer/Counter5
            #[inline(always)]
            pub fn prtim5(&self) -> PRTIM5_R {
                PRTIM5_R::new(((self.bits >> 5) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Power Reduction USART1
            #[inline(always)]
            pub fn prusart1(&mut self) -> PRUSART1_W<'_, PRR1_SPEC> {
                PRUSART1_W::new(self, 0)
            }
            ///Bit 1 - Power Reduction USART2
            #[inline(always)]
            pub fn prusart2(&mut self) -> PRUSART2_W<'_, PRR1_SPEC> {
                PRUSART2_W::new(self, 1)
            }
            ///Bit 2 - Power Reduction USART3
            #[inline(always)]
            pub fn prusart3(&mut self) -> PRUSART3_W<'_, PRR1_SPEC> {
                PRUSART3_W::new(self, 2)
            }
            ///Bit 3 - Power Reduction Timer/Counter3
            #[inline(always)]
            pub fn prtim3(&mut self) -> PRTIM3_W<'_, PRR1_SPEC> {
                PRTIM3_W::new(self, 3)
            }
            ///Bit 4 - Power Reduction Timer/Counter4
            #[inline(always)]
            pub fn prtim4(&mut self) -> PRTIM4_W<'_, PRR1_SPEC> {
                PRTIM4_W::new(self, 4)
            }
            ///Bit 5 - Power Reduction Timer/Counter5
            #[inline(always)]
            pub fn prtim5(&mut self) -> PRTIM5_W<'_, PRR1_SPEC> {
                PRTIM5_W::new(self, 5)
            }
        }
        /**Power Reduction Register1

You can [`read`](crate::Reg::read) this register and get [`prr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PRR1_SPEC;
        impl crate::RegisterSpec for PRR1_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`prr1::R`](R) reader structure
        impl crate::Readable for PRR1_SPEC {}
        ///`write(|w| ..)` method takes [`prr1::W`](W) writer structure
        impl crate::Writable for PRR1_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PRR1 to value 0
        impl crate::Resettable for PRR1_SPEC {}
    }
    /**RAMPZ (rw) register accessor: RAM Page Z Select Register

You can [`read`](crate::Reg::read) this register and get [`rampz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rampz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rampz`] module*/
    pub type RAMPZ = crate::Reg<rampz::RAMPZ_SPEC>;
    ///RAM Page Z Select Register
    pub mod rampz {
        ///Register `RAMPZ` reader
        pub type R = crate::R<RAMPZ_SPEC>;
        ///Register `RAMPZ` writer
        pub type W = crate::W<RAMPZ_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**RAM Page Z Select Register

You can [`read`](crate::Reg::read) this register and get [`rampz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rampz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct RAMPZ_SPEC;
        impl crate::RegisterSpec for RAMPZ_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`rampz::R`](R) reader structure
        impl crate::Readable for RAMPZ_SPEC {}
        ///`write(|w| ..)` method takes [`rampz::W`](W) writer structure
        impl crate::Writable for RAMPZ_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets RAMPZ to value 0
        impl crate::Resettable for RAMPZ_SPEC {}
    }
    /**SMCR (rw) register accessor: Sleep Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smcr`] module*/
    pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
    ///Sleep Mode Control Register
    pub mod smcr {
        ///Register `SMCR` reader
        pub type R = crate::R<SMCR_SPEC>;
        ///Register `SMCR` writer
        pub type W = crate::W<SMCR_SPEC>;
        ///Field `SE` reader - Sleep Enable
        pub type SE_R = crate::BitReader;
        ///Field `SE` writer - Sleep Enable
        pub type SE_W<'a, REG> = crate::BitWriter<'a, REG>;
        /**Sleep Mode Select bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum SM_A {
            ///0: Idle
            IDLE = 0,
            ///1: ADC Noise Reduction (If Available)
            ADC = 1,
            ///2: Power Down
            PDOWN = 2,
            ///3: Power Save
            PSAVE = 3,
            ///4: Reserved
            VAL_0X04 = 4,
            ///5: Reserved
            VAL_0X05 = 5,
            ///6: Standby
            STDBY = 6,
            ///7: Extended Standby
            ESTDBY = 7,
        }
        impl From<SM_A> for u8 {
            #[inline(always)]
            fn from(variant: SM_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for SM_A {
            type Ux = u8;
        }
        impl crate::IsEnum for SM_A {}
        ///Field `SM` reader - Sleep Mode Select bits
        pub type SM_R = crate::FieldReader<SM_A>;
        impl SM_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> SM_A {
                match self.bits {
                    0 => SM_A::IDLE,
                    1 => SM_A::ADC,
                    2 => SM_A::PDOWN,
                    3 => SM_A::PSAVE,
                    4 => SM_A::VAL_0X04,
                    5 => SM_A::VAL_0X05,
                    6 => SM_A::STDBY,
                    7 => SM_A::ESTDBY,
                    _ => unreachable!(),
                }
            }
            ///Idle
            #[inline(always)]
            pub fn is_idle(&self) -> bool {
                *self == SM_A::IDLE
            }
            ///ADC Noise Reduction (If Available)
            #[inline(always)]
            pub fn is_adc(&self) -> bool {
                *self == SM_A::ADC
            }
            ///Power Down
            #[inline(always)]
            pub fn is_pdown(&self) -> bool {
                *self == SM_A::PDOWN
            }
            ///Power Save
            #[inline(always)]
            pub fn is_psave(&self) -> bool {
                *self == SM_A::PSAVE
            }
            ///Reserved
            #[inline(always)]
            pub fn is_val_0x04(&self) -> bool {
                *self == SM_A::VAL_0X04
            }
            ///Reserved
            #[inline(always)]
            pub fn is_val_0x05(&self) -> bool {
                *self == SM_A::VAL_0X05
            }
            ///Standby
            #[inline(always)]
            pub fn is_stdby(&self) -> bool {
                *self == SM_A::STDBY
            }
            ///Extended Standby
            #[inline(always)]
            pub fn is_estdby(&self) -> bool {
                *self == SM_A::ESTDBY
            }
        }
        ///Field `SM` writer - Sleep Mode Select bits
        pub type SM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SM_A, crate::Safe>;
        impl<'a, REG> SM_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Idle
            #[inline(always)]
            pub fn idle(self) -> &'a mut crate::W<REG> {
                self.variant(SM_A::IDLE)
            }
            ///ADC Noise Reduction (If Available)
            #[inline(always)]
            pub fn adc(self) -> &'a mut crate::W<REG> {
                self.variant(SM_A::ADC)
            }
            ///Power Down
            #[inline(always)]
            pub fn pdown(self) -> &'a mut crate::W<REG> {
                self.variant(SM_A::PDOWN)
            }
            ///Power Save
            #[inline(always)]
            pub fn psave(self) -> &'a mut crate::W<REG> {
                self.variant(SM_A::PSAVE)
            }
            ///Reserved
            #[inline(always)]
            pub fn val_0x04(self) -> &'a mut crate::W<REG> {
                self.variant(SM_A::VAL_0X04)
            }
            ///Reserved
            #[inline(always)]
            pub fn val_0x05(self) -> &'a mut crate::W<REG> {
                self.variant(SM_A::VAL_0X05)
            }
            ///Standby
            #[inline(always)]
            pub fn stdby(self) -> &'a mut crate::W<REG> {
                self.variant(SM_A::STDBY)
            }
            ///Extended Standby
            #[inline(always)]
            pub fn estdby(self) -> &'a mut crate::W<REG> {
                self.variant(SM_A::ESTDBY)
            }
        }
        impl R {
            ///Bit 0 - Sleep Enable
            #[inline(always)]
            pub fn se(&self) -> SE_R {
                SE_R::new((self.bits & 1) != 0)
            }
            ///Bits 1:3 - Sleep Mode Select bits
            #[inline(always)]
            pub fn sm(&self) -> SM_R {
                SM_R::new((self.bits >> 1) & 7)
            }
        }
        impl W {
            ///Bit 0 - Sleep Enable
            #[inline(always)]
            pub fn se(&mut self) -> SE_W<'_, SMCR_SPEC> {
                SE_W::new(self, 0)
            }
            ///Bits 1:3 - Sleep Mode Select bits
            #[inline(always)]
            pub fn sm(&mut self) -> SM_W<'_, SMCR_SPEC> {
                SM_W::new(self, 1)
            }
        }
        /**Sleep Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct SMCR_SPEC;
        impl crate::RegisterSpec for SMCR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`smcr::R`](R) reader structure
        impl crate::Readable for SMCR_SPEC {}
        ///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
        impl crate::Writable for SMCR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets SMCR to value 0
        impl crate::Resettable for SMCR_SPEC {}
    }
    /**XMCRA (rw) register accessor: External Memory Control Register A

You can [`read`](crate::Reg::read) this register and get [`xmcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xmcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xmcra`] module*/
    pub type XMCRA = crate::Reg<xmcra::XMCRA_SPEC>;
    ///External Memory Control Register A
    pub mod xmcra {
        ///Register `XMCRA` reader
        pub type R = crate::R<XMCRA_SPEC>;
        ///Register `XMCRA` writer
        pub type W = crate::W<XMCRA_SPEC>;
        /**Wait state select bit lower page

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum SRW0_A {
            ///0: No wait-states
            VAL_0X00 = 0,
            ///1: Wait one cycle during read/write strobe
            VAL_0X01 = 1,
            ///2: Wait two cycles during read/write strobe
            VAL_0X02 = 2,
            ///3: Wait two cycles during read/write and wait one cycle before driving out new address
            VAL_0X03 = 3,
        }
        impl From<SRW0_A> for u8 {
            #[inline(always)]
            fn from(variant: SRW0_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for SRW0_A {
            type Ux = u8;
        }
        impl crate::IsEnum for SRW0_A {}
        ///Field `SRW0` reader - Wait state select bit lower page
        pub type SRW0_R = crate::FieldReader<SRW0_A>;
        impl SRW0_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> SRW0_A {
                match self.bits {
                    0 => SRW0_A::VAL_0X00,
                    1 => SRW0_A::VAL_0X01,
                    2 => SRW0_A::VAL_0X02,
                    3 => SRW0_A::VAL_0X03,
                    _ => unreachable!(),
                }
            }
            ///No wait-states
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == SRW0_A::VAL_0X00
            }
            ///Wait one cycle during read/write strobe
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == SRW0_A::VAL_0X01
            }
            ///Wait two cycles during read/write strobe
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == SRW0_A::VAL_0X02
            }
            ///Wait two cycles during read/write and wait one cycle before driving out new address
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == SRW0_A::VAL_0X03
            }
        }
        ///Field `SRW0` writer - Wait state select bit lower page
        pub type SRW0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SRW0_A, crate::Safe>;
        impl<'a, REG> SRW0_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///No wait-states
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(SRW0_A::VAL_0X00)
            }
            ///Wait one cycle during read/write strobe
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(SRW0_A::VAL_0X01)
            }
            ///Wait two cycles during read/write strobe
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(SRW0_A::VAL_0X02)
            }
            ///Wait two cycles during read/write and wait one cycle before driving out new address
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(SRW0_A::VAL_0X03)
            }
        }
        /**Wait state select bit upper page

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum SRW1_A {
            ///0: No wait-states
            VAL_0X00 = 0,
            ///1: Wait one cycle during read/write strobe
            VAL_0X01 = 1,
            ///2: Wait two cycles during read/write strobe
            VAL_0X02 = 2,
            ///3: Wait two cycles during read/write and wait one cycle before driving out new address
            VAL_0X03 = 3,
        }
        impl From<SRW1_A> for u8 {
            #[inline(always)]
            fn from(variant: SRW1_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for SRW1_A {
            type Ux = u8;
        }
        impl crate::IsEnum for SRW1_A {}
        ///Field `SRW1` reader - Wait state select bit upper page
        pub type SRW1_R = crate::FieldReader<SRW1_A>;
        impl SRW1_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> SRW1_A {
                match self.bits {
                    0 => SRW1_A::VAL_0X00,
                    1 => SRW1_A::VAL_0X01,
                    2 => SRW1_A::VAL_0X02,
                    3 => SRW1_A::VAL_0X03,
                    _ => unreachable!(),
                }
            }
            ///No wait-states
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == SRW1_A::VAL_0X00
            }
            ///Wait one cycle during read/write strobe
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == SRW1_A::VAL_0X01
            }
            ///Wait two cycles during read/write strobe
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == SRW1_A::VAL_0X02
            }
            ///Wait two cycles during read/write and wait one cycle before driving out new address
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == SRW1_A::VAL_0X03
            }
        }
        ///Field `SRW1` writer - Wait state select bit upper page
        pub type SRW1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SRW1_A, crate::Safe>;
        impl<'a, REG> SRW1_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///No wait-states
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(SRW1_A::VAL_0X00)
            }
            ///Wait one cycle during read/write strobe
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(SRW1_A::VAL_0X01)
            }
            ///Wait two cycles during read/write strobe
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(SRW1_A::VAL_0X02)
            }
            ///Wait two cycles during read/write and wait one cycle before driving out new address
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(SRW1_A::VAL_0X03)
            }
        }
        /**Wait state page limit

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum SRL_A {
            ///0: LS = N/A, US = 0x1100 - 0xFFFF
            VAL_0X00 = 0,
            ///1: LS = 0x2200 - 0x1FFF, US = 0x2000 - 0xFFFF
            VAL_0X01 = 1,
            ///2: LS = 0x2200 - 0x3FFF, US = 0x4000 - 0xFFFF
            VAL_0X02 = 2,
            ///3: LS = 0x2200 - 0x5FFF, US = 0x6000 - 0xFFFF
            VAL_0X03 = 3,
            ///4: LS = 0x2200 - 0x7FFF, US = 0x8000 - 0xFFFF
            VAL_0X04 = 4,
            ///5: LS = 0x2200 - 0x9FFF, US = 0xA000 - 0xFFFF
            VAL_0X05 = 5,
            ///6: LS = 0x2200 - 0xBFFF, US = 0xC000 - 0xFFFF
            VAL_0X06 = 6,
            ///7: LS = 0x2200 - 0xDFFF, US = 0xE000 - 0xFFFF
            VAL_0X07 = 7,
        }
        impl From<SRL_A> for u8 {
            #[inline(always)]
            fn from(variant: SRL_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for SRL_A {
            type Ux = u8;
        }
        impl crate::IsEnum for SRL_A {}
        ///Field `SRL` reader - Wait state page limit
        pub type SRL_R = crate::FieldReader<SRL_A>;
        impl SRL_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> SRL_A {
                match self.bits {
                    0 => SRL_A::VAL_0X00,
                    1 => SRL_A::VAL_0X01,
                    2 => SRL_A::VAL_0X02,
                    3 => SRL_A::VAL_0X03,
                    4 => SRL_A::VAL_0X04,
                    5 => SRL_A::VAL_0X05,
                    6 => SRL_A::VAL_0X06,
                    7 => SRL_A::VAL_0X07,
                    _ => unreachable!(),
                }
            }
            ///LS = N/A, US = 0x1100 - 0xFFFF
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == SRL_A::VAL_0X00
            }
            ///LS = 0x2200 - 0x1FFF, US = 0x2000 - 0xFFFF
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == SRL_A::VAL_0X01
            }
            ///LS = 0x2200 - 0x3FFF, US = 0x4000 - 0xFFFF
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == SRL_A::VAL_0X02
            }
            ///LS = 0x2200 - 0x5FFF, US = 0x6000 - 0xFFFF
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == SRL_A::VAL_0X03
            }
            ///LS = 0x2200 - 0x7FFF, US = 0x8000 - 0xFFFF
            #[inline(always)]
            pub fn is_val_0x04(&self) -> bool {
                *self == SRL_A::VAL_0X04
            }
            ///LS = 0x2200 - 0x9FFF, US = 0xA000 - 0xFFFF
            #[inline(always)]
            pub fn is_val_0x05(&self) -> bool {
                *self == SRL_A::VAL_0X05
            }
            ///LS = 0x2200 - 0xBFFF, US = 0xC000 - 0xFFFF
            #[inline(always)]
            pub fn is_val_0x06(&self) -> bool {
                *self == SRL_A::VAL_0X06
            }
            ///LS = 0x2200 - 0xDFFF, US = 0xE000 - 0xFFFF
            #[inline(always)]
            pub fn is_val_0x07(&self) -> bool {
                *self == SRL_A::VAL_0X07
            }
        }
        ///Field `SRL` writer - Wait state page limit
        pub type SRL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SRL_A, crate::Safe>;
        impl<'a, REG> SRL_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///LS = N/A, US = 0x1100 - 0xFFFF
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(SRL_A::VAL_0X00)
            }
            ///LS = 0x2200 - 0x1FFF, US = 0x2000 - 0xFFFF
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(SRL_A::VAL_0X01)
            }
            ///LS = 0x2200 - 0x3FFF, US = 0x4000 - 0xFFFF
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(SRL_A::VAL_0X02)
            }
            ///LS = 0x2200 - 0x5FFF, US = 0x6000 - 0xFFFF
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(SRL_A::VAL_0X03)
            }
            ///LS = 0x2200 - 0x7FFF, US = 0x8000 - 0xFFFF
            #[inline(always)]
            pub fn val_0x04(self) -> &'a mut crate::W<REG> {
                self.variant(SRL_A::VAL_0X04)
            }
            ///LS = 0x2200 - 0x9FFF, US = 0xA000 - 0xFFFF
            #[inline(always)]
            pub fn val_0x05(self) -> &'a mut crate::W<REG> {
                self.variant(SRL_A::VAL_0X05)
            }
            ///LS = 0x2200 - 0xBFFF, US = 0xC000 - 0xFFFF
            #[inline(always)]
            pub fn val_0x06(self) -> &'a mut crate::W<REG> {
                self.variant(SRL_A::VAL_0X06)
            }
            ///LS = 0x2200 - 0xDFFF, US = 0xE000 - 0xFFFF
            #[inline(always)]
            pub fn val_0x07(self) -> &'a mut crate::W<REG> {
                self.variant(SRL_A::VAL_0X07)
            }
        }
        ///Field `SRE` reader - External SRAM Enable
        pub type SRE_R = crate::BitReader;
        ///Field `SRE` writer - External SRAM Enable
        pub type SRE_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:1 - Wait state select bit lower page
            #[inline(always)]
            pub fn srw0(&self) -> SRW0_R {
                SRW0_R::new(self.bits & 3)
            }
            ///Bits 2:3 - Wait state select bit upper page
            #[inline(always)]
            pub fn srw1(&self) -> SRW1_R {
                SRW1_R::new((self.bits >> 2) & 3)
            }
            ///Bits 4:6 - Wait state page limit
            #[inline(always)]
            pub fn srl(&self) -> SRL_R {
                SRL_R::new((self.bits >> 4) & 7)
            }
            ///Bit 7 - External SRAM Enable
            #[inline(always)]
            pub fn sre(&self) -> SRE_R {
                SRE_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:1 - Wait state select bit lower page
            #[inline(always)]
            pub fn srw0(&mut self) -> SRW0_W<'_, XMCRA_SPEC> {
                SRW0_W::new(self, 0)
            }
            ///Bits 2:3 - Wait state select bit upper page
            #[inline(always)]
            pub fn srw1(&mut self) -> SRW1_W<'_, XMCRA_SPEC> {
                SRW1_W::new(self, 2)
            }
            ///Bits 4:6 - Wait state page limit
            #[inline(always)]
            pub fn srl(&mut self) -> SRL_W<'_, XMCRA_SPEC> {
                SRL_W::new(self, 4)
            }
            ///Bit 7 - External SRAM Enable
            #[inline(always)]
            pub fn sre(&mut self) -> SRE_W<'_, XMCRA_SPEC> {
                SRE_W::new(self, 7)
            }
        }
        /**External Memory Control Register A

You can [`read`](crate::Reg::read) this register and get [`xmcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xmcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct XMCRA_SPEC;
        impl crate::RegisterSpec for XMCRA_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`xmcra::R`](R) reader structure
        impl crate::Readable for XMCRA_SPEC {}
        ///`write(|w| ..)` method takes [`xmcra::W`](W) writer structure
        impl crate::Writable for XMCRA_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets XMCRA to value 0
        impl crate::Resettable for XMCRA_SPEC {}
    }
    /**XMCRB (rw) register accessor: External Memory Control Register B

You can [`read`](crate::Reg::read) this register and get [`xmcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xmcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xmcrb`] module*/
    pub type XMCRB = crate::Reg<xmcrb::XMCRB_SPEC>;
    ///External Memory Control Register B
    pub mod xmcrb {
        ///Register `XMCRB` reader
        pub type R = crate::R<XMCRB_SPEC>;
        ///Register `XMCRB` writer
        pub type W = crate::W<XMCRB_SPEC>;
        ///Field `XMM` reader - External Memory High Mask
        pub type XMM_R = crate::FieldReader;
        ///Field `XMM` writer - External Memory High Mask
        pub type XMM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
        ///Field `XMBK` reader - External Memory Bus Keeper Enable
        pub type XMBK_R = crate::BitReader;
        ///Field `XMBK` writer - External Memory Bus Keeper Enable
        pub type XMBK_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:2 - External Memory High Mask
            #[inline(always)]
            pub fn xmm(&self) -> XMM_R {
                XMM_R::new(self.bits & 7)
            }
            ///Bit 7 - External Memory Bus Keeper Enable
            #[inline(always)]
            pub fn xmbk(&self) -> XMBK_R {
                XMBK_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:2 - External Memory High Mask
            #[inline(always)]
            pub fn xmm(&mut self) -> XMM_W<'_, XMCRB_SPEC> {
                XMM_W::new(self, 0)
            }
            ///Bit 7 - External Memory Bus Keeper Enable
            #[inline(always)]
            pub fn xmbk(&mut self) -> XMBK_W<'_, XMCRB_SPEC> {
                XMBK_W::new(self, 7)
            }
        }
        /**External Memory Control Register B

You can [`read`](crate::Reg::read) this register and get [`xmcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xmcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct XMCRB_SPEC;
        impl crate::RegisterSpec for XMCRB_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`xmcrb::R`](R) reader structure
        impl crate::Readable for XMCRB_SPEC {}
        ///`write(|w| ..)` method takes [`xmcrb::W`](W) writer structure
        impl crate::Writable for XMCRB_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets XMCRB to value 0
        impl crate::Resettable for XMCRB_SPEC {}
    }
}
///EEPROM
pub type EEPROM = crate::Periph<eeprom::RegisterBlock, 0x3f>;
impl core::fmt::Debug for EEPROM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EEPROM").finish()
    }
}
///EEPROM
pub mod eeprom {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        eecr: EECR,
        eedr: EEDR,
        eear: EEAR,
    }
    impl RegisterBlock {
        ///0x00 - EEPROM Control Register
        #[inline(always)]
        pub const fn eecr(&self) -> &EECR {
            &self.eecr
        }
        ///0x01 - EEPROM Data Register
        #[inline(always)]
        pub const fn eedr(&self) -> &EEDR {
            &self.eedr
        }
        ///0x02 - EEPROM Address Register Low Bytes
        #[inline(always)]
        pub const fn eear(&self) -> &EEAR {
            &self.eear
        }
    }
    /**EEAR (rw) register accessor: EEPROM Address Register Low Bytes

You can [`read`](crate::Reg::read) this register and get [`eear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eear`] module*/
    pub type EEAR = crate::Reg<eear::EEAR_SPEC>;
    ///EEPROM Address Register Low Bytes
    pub mod eear {
        ///Register `EEAR` reader
        pub type R = crate::R<EEAR_SPEC>;
        ///Register `EEAR` writer
        pub type W = crate::W<EEAR_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**EEPROM Address Register Low Bytes

You can [`read`](crate::Reg::read) this register and get [`eear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct EEAR_SPEC;
        impl crate::RegisterSpec for EEAR_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`eear::R`](R) reader structure
        impl crate::Readable for EEAR_SPEC {}
        ///`write(|w| ..)` method takes [`eear::W`](W) writer structure
        impl crate::Writable for EEAR_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets EEAR to value 0
        impl crate::Resettable for EEAR_SPEC {}
    }
    /**EECR (rw) register accessor: EEPROM Control Register

You can [`read`](crate::Reg::read) this register and get [`eecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eecr`] module*/
    pub type EECR = crate::Reg<eecr::EECR_SPEC>;
    ///EEPROM Control Register
    pub mod eecr {
        ///Register `EECR` reader
        pub type R = crate::R<EECR_SPEC>;
        ///Register `EECR` writer
        pub type W = crate::W<EECR_SPEC>;
        ///Field `EERE` reader - EEPROM Read Enable
        pub type EERE_R = crate::BitReader;
        ///Field `EERE` writer - EEPROM Read Enable
        pub type EERE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `EEPE` reader - EEPROM Write Enable
        pub type EEPE_R = crate::BitReader;
        ///Field `EEPE` writer - EEPROM Write Enable
        pub type EEPE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `EEMPE` reader - EEPROM Master Write Enable
        pub type EEMPE_R = crate::BitReader;
        ///Field `EEMPE` writer - EEPROM Master Write Enable
        pub type EEMPE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `EERIE` reader - EEPROM Ready Interrupt Enable
        pub type EERIE_R = crate::BitReader;
        ///Field `EERIE` writer - EEPROM Ready Interrupt Enable
        pub type EERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
        /**EEPROM Programming Mode Bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum EEPM_A {
            ///0: Erase and Write in one operation
            VAL_0X00 = 0,
            ///1: Erase Only
            VAL_0X01 = 1,
            ///2: Write Only
            VAL_0X02 = 2,
        }
        impl From<EEPM_A> for u8 {
            #[inline(always)]
            fn from(variant: EEPM_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for EEPM_A {
            type Ux = u8;
        }
        impl crate::IsEnum for EEPM_A {}
        ///Field `EEPM` reader - EEPROM Programming Mode Bits
        pub type EEPM_R = crate::FieldReader<EEPM_A>;
        impl EEPM_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<EEPM_A> {
                match self.bits {
                    0 => Some(EEPM_A::VAL_0X00),
                    1 => Some(EEPM_A::VAL_0X01),
                    2 => Some(EEPM_A::VAL_0X02),
                    _ => None,
                }
            }
            ///Erase and Write in one operation
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == EEPM_A::VAL_0X00
            }
            ///Erase Only
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == EEPM_A::VAL_0X01
            }
            ///Write Only
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == EEPM_A::VAL_0X02
            }
        }
        ///Field `EEPM` writer - EEPROM Programming Mode Bits
        pub type EEPM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EEPM_A>;
        impl<'a, REG> EEPM_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Erase and Write in one operation
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(EEPM_A::VAL_0X00)
            }
            ///Erase Only
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(EEPM_A::VAL_0X01)
            }
            ///Write Only
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(EEPM_A::VAL_0X02)
            }
        }
        impl R {
            ///Bit 0 - EEPROM Read Enable
            #[inline(always)]
            pub fn eere(&self) -> EERE_R {
                EERE_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - EEPROM Write Enable
            #[inline(always)]
            pub fn eepe(&self) -> EEPE_R {
                EEPE_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - EEPROM Master Write Enable
            #[inline(always)]
            pub fn eempe(&self) -> EEMPE_R {
                EEMPE_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - EEPROM Ready Interrupt Enable
            #[inline(always)]
            pub fn eerie(&self) -> EERIE_R {
                EERIE_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bits 4:5 - EEPROM Programming Mode Bits
            #[inline(always)]
            pub fn eepm(&self) -> EEPM_R {
                EEPM_R::new((self.bits >> 4) & 3)
            }
        }
        impl W {
            ///Bit 0 - EEPROM Read Enable
            #[inline(always)]
            pub fn eere(&mut self) -> EERE_W<'_, EECR_SPEC> {
                EERE_W::new(self, 0)
            }
            ///Bit 1 - EEPROM Write Enable
            #[inline(always)]
            pub fn eepe(&mut self) -> EEPE_W<'_, EECR_SPEC> {
                EEPE_W::new(self, 1)
            }
            ///Bit 2 - EEPROM Master Write Enable
            #[inline(always)]
            pub fn eempe(&mut self) -> EEMPE_W<'_, EECR_SPEC> {
                EEMPE_W::new(self, 2)
            }
            ///Bit 3 - EEPROM Ready Interrupt Enable
            #[inline(always)]
            pub fn eerie(&mut self) -> EERIE_W<'_, EECR_SPEC> {
                EERIE_W::new(self, 3)
            }
            ///Bits 4:5 - EEPROM Programming Mode Bits
            #[inline(always)]
            pub fn eepm(&mut self) -> EEPM_W<'_, EECR_SPEC> {
                EEPM_W::new(self, 4)
            }
        }
        /**EEPROM Control Register

You can [`read`](crate::Reg::read) this register and get [`eecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct EECR_SPEC;
        impl crate::RegisterSpec for EECR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`eecr::R`](R) reader structure
        impl crate::Readable for EECR_SPEC {}
        ///`write(|w| ..)` method takes [`eecr::W`](W) writer structure
        impl crate::Writable for EECR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets EECR to value 0
        impl crate::Resettable for EECR_SPEC {}
    }
    /**EEDR (rw) register accessor: EEPROM Data Register

You can [`read`](crate::Reg::read) this register and get [`eedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eedr`] module*/
    pub type EEDR = crate::Reg<eedr::EEDR_SPEC>;
    ///EEPROM Data Register
    pub mod eedr {
        ///Register `EEDR` reader
        pub type R = crate::R<EEDR_SPEC>;
        ///Register `EEDR` writer
        pub type W = crate::W<EEDR_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**EEPROM Data Register

You can [`read`](crate::Reg::read) this register and get [`eedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct EEDR_SPEC;
        impl crate::RegisterSpec for EEDR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`eedr::R`](R) reader structure
        impl crate::Readable for EEDR_SPEC {}
        ///`write(|w| ..)` method takes [`eedr::W`](W) writer structure
        impl crate::Writable for EEDR_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets EEDR to value 0
        impl crate::Resettable for EEDR_SPEC {}
    }
}
///External Interrupts
pub type EXINT = crate::Periph<exint::RegisterBlock, 0x3b>;
impl core::fmt::Debug for EXINT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINT").finish()
    }
}
///External Interrupts
pub mod exint {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pcifr: PCIFR,
        eifr: EIFR,
        eimsk: EIMSK,
        _reserved3: [u8; 0x2a],
        pcicr: PCICR,
        eicra: EICRA,
        eicrb: EICRB,
        pcmsk0: PCMSK0,
        pcmsk1: PCMSK1,
        pcmsk2: PCMSK2,
    }
    impl RegisterBlock {
        ///0x00 - Pin Change Interrupt Flag Register
        #[inline(always)]
        pub const fn pcifr(&self) -> &PCIFR {
            &self.pcifr
        }
        ///0x01 - External Interrupt Flag Register
        #[inline(always)]
        pub const fn eifr(&self) -> &EIFR {
            &self.eifr
        }
        ///0x02 - External Interrupt Mask Register
        #[inline(always)]
        pub const fn eimsk(&self) -> &EIMSK {
            &self.eimsk
        }
        ///0x2d - Pin Change Interrupt Control Register
        #[inline(always)]
        pub const fn pcicr(&self) -> &PCICR {
            &self.pcicr
        }
        ///0x2e - External Interrupt Control Register A
        #[inline(always)]
        pub const fn eicra(&self) -> &EICRA {
            &self.eicra
        }
        ///0x2f - External Interrupt Control Register B
        #[inline(always)]
        pub const fn eicrb(&self) -> &EICRB {
            &self.eicrb
        }
        ///0x30 - Pin Change Mask Register 0
        #[inline(always)]
        pub const fn pcmsk0(&self) -> &PCMSK0 {
            &self.pcmsk0
        }
        ///0x31 - Pin Change Mask Register 1
        #[inline(always)]
        pub const fn pcmsk1(&self) -> &PCMSK1 {
            &self.pcmsk1
        }
        ///0x32 - Pin Change Mask Register 2
        #[inline(always)]
        pub const fn pcmsk2(&self) -> &PCMSK2 {
            &self.pcmsk2
        }
    }
    /**EICRA (rw) register accessor: External Interrupt Control Register A

You can [`read`](crate::Reg::read) this register and get [`eicra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eicra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eicra`] module*/
    pub type EICRA = crate::Reg<eicra::EICRA_SPEC>;
    ///External Interrupt Control Register A
    pub mod eicra {
        ///Register `EICRA` reader
        pub type R = crate::R<EICRA_SPEC>;
        ///Register `EICRA` writer
        pub type W = crate::W<EICRA_SPEC>;
        /**External Interrupt Sense Control Bit

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ISC0_A {
            ///0: Low Level of INTX
            VAL_0X00 = 0,
            ///1: Any Logical Change of INTX
            VAL_0X01 = 1,
            ///2: Falling Edge of INTX
            VAL_0X02 = 2,
            ///3: Rising Edge of INTX
            VAL_0X03 = 3,
        }
        impl From<ISC0_A> for u8 {
            #[inline(always)]
            fn from(variant: ISC0_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ISC0_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ISC0_A {}
        ///Field `ISC0` reader - External Interrupt Sense Control Bit
        pub type ISC0_R = crate::FieldReader<ISC0_A>;
        impl ISC0_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> ISC0_A {
                match self.bits {
                    0 => ISC0_A::VAL_0X00,
                    1 => ISC0_A::VAL_0X01,
                    2 => ISC0_A::VAL_0X02,
                    3 => ISC0_A::VAL_0X03,
                    _ => unreachable!(),
                }
            }
            ///Low Level of INTX
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == ISC0_A::VAL_0X00
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == ISC0_A::VAL_0X01
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == ISC0_A::VAL_0X02
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == ISC0_A::VAL_0X03
            }
        }
        ///Field `ISC0` writer - External Interrupt Sense Control Bit
        pub type ISC0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ISC0_A, crate::Safe>;
        impl<'a, REG> ISC0_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Low Level of INTX
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(ISC0_A::VAL_0X00)
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(ISC0_A::VAL_0X01)
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(ISC0_A::VAL_0X02)
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(ISC0_A::VAL_0X03)
            }
        }
        /**External Interrupt Sense Control Bit

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ISC1_A {
            ///0: Low Level of INTX
            VAL_0X00 = 0,
            ///1: Any Logical Change of INTX
            VAL_0X01 = 1,
            ///2: Falling Edge of INTX
            VAL_0X02 = 2,
            ///3: Rising Edge of INTX
            VAL_0X03 = 3,
        }
        impl From<ISC1_A> for u8 {
            #[inline(always)]
            fn from(variant: ISC1_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ISC1_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ISC1_A {}
        ///Field `ISC1` reader - External Interrupt Sense Control Bit
        pub type ISC1_R = crate::FieldReader<ISC1_A>;
        impl ISC1_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> ISC1_A {
                match self.bits {
                    0 => ISC1_A::VAL_0X00,
                    1 => ISC1_A::VAL_0X01,
                    2 => ISC1_A::VAL_0X02,
                    3 => ISC1_A::VAL_0X03,
                    _ => unreachable!(),
                }
            }
            ///Low Level of INTX
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == ISC1_A::VAL_0X00
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == ISC1_A::VAL_0X01
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == ISC1_A::VAL_0X02
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == ISC1_A::VAL_0X03
            }
        }
        ///Field `ISC1` writer - External Interrupt Sense Control Bit
        pub type ISC1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ISC1_A, crate::Safe>;
        impl<'a, REG> ISC1_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Low Level of INTX
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(ISC1_A::VAL_0X00)
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(ISC1_A::VAL_0X01)
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(ISC1_A::VAL_0X02)
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(ISC1_A::VAL_0X03)
            }
        }
        /**External Interrupt Sense Control Bit

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ISC2_A {
            ///0: Low Level of INTX
            VAL_0X00 = 0,
            ///1: Any Logical Change of INTX
            VAL_0X01 = 1,
            ///2: Falling Edge of INTX
            VAL_0X02 = 2,
            ///3: Rising Edge of INTX
            VAL_0X03 = 3,
        }
        impl From<ISC2_A> for u8 {
            #[inline(always)]
            fn from(variant: ISC2_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ISC2_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ISC2_A {}
        ///Field `ISC2` reader - External Interrupt Sense Control Bit
        pub type ISC2_R = crate::FieldReader<ISC2_A>;
        impl ISC2_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> ISC2_A {
                match self.bits {
                    0 => ISC2_A::VAL_0X00,
                    1 => ISC2_A::VAL_0X01,
                    2 => ISC2_A::VAL_0X02,
                    3 => ISC2_A::VAL_0X03,
                    _ => unreachable!(),
                }
            }
            ///Low Level of INTX
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == ISC2_A::VAL_0X00
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == ISC2_A::VAL_0X01
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == ISC2_A::VAL_0X02
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == ISC2_A::VAL_0X03
            }
        }
        ///Field `ISC2` writer - External Interrupt Sense Control Bit
        pub type ISC2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ISC2_A, crate::Safe>;
        impl<'a, REG> ISC2_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Low Level of INTX
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(ISC2_A::VAL_0X00)
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(ISC2_A::VAL_0X01)
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(ISC2_A::VAL_0X02)
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(ISC2_A::VAL_0X03)
            }
        }
        /**External Interrupt Sense Control Bit

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ISC3_A {
            ///0: Low Level of INTX
            VAL_0X00 = 0,
            ///1: Any Logical Change of INTX
            VAL_0X01 = 1,
            ///2: Falling Edge of INTX
            VAL_0X02 = 2,
            ///3: Rising Edge of INTX
            VAL_0X03 = 3,
        }
        impl From<ISC3_A> for u8 {
            #[inline(always)]
            fn from(variant: ISC3_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ISC3_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ISC3_A {}
        ///Field `ISC3` reader - External Interrupt Sense Control Bit
        pub type ISC3_R = crate::FieldReader<ISC3_A>;
        impl ISC3_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> ISC3_A {
                match self.bits {
                    0 => ISC3_A::VAL_0X00,
                    1 => ISC3_A::VAL_0X01,
                    2 => ISC3_A::VAL_0X02,
                    3 => ISC3_A::VAL_0X03,
                    _ => unreachable!(),
                }
            }
            ///Low Level of INTX
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == ISC3_A::VAL_0X00
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == ISC3_A::VAL_0X01
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == ISC3_A::VAL_0X02
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == ISC3_A::VAL_0X03
            }
        }
        ///Field `ISC3` writer - External Interrupt Sense Control Bit
        pub type ISC3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ISC3_A, crate::Safe>;
        impl<'a, REG> ISC3_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Low Level of INTX
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(ISC3_A::VAL_0X00)
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(ISC3_A::VAL_0X01)
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(ISC3_A::VAL_0X02)
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(ISC3_A::VAL_0X03)
            }
        }
        impl R {
            ///Bits 0:1 - External Interrupt Sense Control Bit
            #[inline(always)]
            pub fn isc0(&self) -> ISC0_R {
                ISC0_R::new(self.bits & 3)
            }
            ///Bits 2:3 - External Interrupt Sense Control Bit
            #[inline(always)]
            pub fn isc1(&self) -> ISC1_R {
                ISC1_R::new((self.bits >> 2) & 3)
            }
            ///Bits 4:5 - External Interrupt Sense Control Bit
            #[inline(always)]
            pub fn isc2(&self) -> ISC2_R {
                ISC2_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - External Interrupt Sense Control Bit
            #[inline(always)]
            pub fn isc3(&self) -> ISC3_R {
                ISC3_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bits 0:1 - External Interrupt Sense Control Bit
            #[inline(always)]
            pub fn isc0(&mut self) -> ISC0_W<'_, EICRA_SPEC> {
                ISC0_W::new(self, 0)
            }
            ///Bits 2:3 - External Interrupt Sense Control Bit
            #[inline(always)]
            pub fn isc1(&mut self) -> ISC1_W<'_, EICRA_SPEC> {
                ISC1_W::new(self, 2)
            }
            ///Bits 4:5 - External Interrupt Sense Control Bit
            #[inline(always)]
            pub fn isc2(&mut self) -> ISC2_W<'_, EICRA_SPEC> {
                ISC2_W::new(self, 4)
            }
            ///Bits 6:7 - External Interrupt Sense Control Bit
            #[inline(always)]
            pub fn isc3(&mut self) -> ISC3_W<'_, EICRA_SPEC> {
                ISC3_W::new(self, 6)
            }
        }
        /**External Interrupt Control Register A

You can [`read`](crate::Reg::read) this register and get [`eicra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eicra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct EICRA_SPEC;
        impl crate::RegisterSpec for EICRA_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`eicra::R`](R) reader structure
        impl crate::Readable for EICRA_SPEC {}
        ///`write(|w| ..)` method takes [`eicra::W`](W) writer structure
        impl crate::Writable for EICRA_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets EICRA to value 0
        impl crate::Resettable for EICRA_SPEC {}
    }
    /**EICRB (rw) register accessor: External Interrupt Control Register B

You can [`read`](crate::Reg::read) this register and get [`eicrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eicrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eicrb`] module*/
    pub type EICRB = crate::Reg<eicrb::EICRB_SPEC>;
    ///External Interrupt Control Register B
    pub mod eicrb {
        ///Register `EICRB` reader
        pub type R = crate::R<EICRB_SPEC>;
        ///Register `EICRB` writer
        pub type W = crate::W<EICRB_SPEC>;
        /**External Interrupt 7-4 Sense Control Bit

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ISC4_A {
            ///0: Low Level of INTX
            VAL_0X00 = 0,
            ///1: Any Logical Change of INTX
            VAL_0X01 = 1,
            ///2: Falling Edge of INTX
            VAL_0X02 = 2,
            ///3: Rising Edge of INTX
            VAL_0X03 = 3,
        }
        impl From<ISC4_A> for u8 {
            #[inline(always)]
            fn from(variant: ISC4_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ISC4_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ISC4_A {}
        ///Field `ISC4` reader - External Interrupt 7-4 Sense Control Bit
        pub type ISC4_R = crate::FieldReader<ISC4_A>;
        impl ISC4_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> ISC4_A {
                match self.bits {
                    0 => ISC4_A::VAL_0X00,
                    1 => ISC4_A::VAL_0X01,
                    2 => ISC4_A::VAL_0X02,
                    3 => ISC4_A::VAL_0X03,
                    _ => unreachable!(),
                }
            }
            ///Low Level of INTX
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == ISC4_A::VAL_0X00
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == ISC4_A::VAL_0X01
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == ISC4_A::VAL_0X02
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == ISC4_A::VAL_0X03
            }
        }
        ///Field `ISC4` writer - External Interrupt 7-4 Sense Control Bit
        pub type ISC4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ISC4_A, crate::Safe>;
        impl<'a, REG> ISC4_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Low Level of INTX
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(ISC4_A::VAL_0X00)
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(ISC4_A::VAL_0X01)
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(ISC4_A::VAL_0X02)
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(ISC4_A::VAL_0X03)
            }
        }
        /**External Interrupt 7-4 Sense Control Bit

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ISC5_A {
            ///0: Low Level of INTX
            VAL_0X00 = 0,
            ///1: Any Logical Change of INTX
            VAL_0X01 = 1,
            ///2: Falling Edge of INTX
            VAL_0X02 = 2,
            ///3: Rising Edge of INTX
            VAL_0X03 = 3,
        }
        impl From<ISC5_A> for u8 {
            #[inline(always)]
            fn from(variant: ISC5_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ISC5_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ISC5_A {}
        ///Field `ISC5` reader - External Interrupt 7-4 Sense Control Bit
        pub type ISC5_R = crate::FieldReader<ISC5_A>;
        impl ISC5_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> ISC5_A {
                match self.bits {
                    0 => ISC5_A::VAL_0X00,
                    1 => ISC5_A::VAL_0X01,
                    2 => ISC5_A::VAL_0X02,
                    3 => ISC5_A::VAL_0X03,
                    _ => unreachable!(),
                }
            }
            ///Low Level of INTX
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == ISC5_A::VAL_0X00
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == ISC5_A::VAL_0X01
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == ISC5_A::VAL_0X02
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == ISC5_A::VAL_0X03
            }
        }
        ///Field `ISC5` writer - External Interrupt 7-4 Sense Control Bit
        pub type ISC5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ISC5_A, crate::Safe>;
        impl<'a, REG> ISC5_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Low Level of INTX
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(ISC5_A::VAL_0X00)
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(ISC5_A::VAL_0X01)
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(ISC5_A::VAL_0X02)
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(ISC5_A::VAL_0X03)
            }
        }
        /**External Interrupt 7-4 Sense Control Bit

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ISC6_A {
            ///0: Low Level of INTX
            VAL_0X00 = 0,
            ///1: Any Logical Change of INTX
            VAL_0X01 = 1,
            ///2: Falling Edge of INTX
            VAL_0X02 = 2,
            ///3: Rising Edge of INTX
            VAL_0X03 = 3,
        }
        impl From<ISC6_A> for u8 {
            #[inline(always)]
            fn from(variant: ISC6_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ISC6_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ISC6_A {}
        ///Field `ISC6` reader - External Interrupt 7-4 Sense Control Bit
        pub type ISC6_R = crate::FieldReader<ISC6_A>;
        impl ISC6_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> ISC6_A {
                match self.bits {
                    0 => ISC6_A::VAL_0X00,
                    1 => ISC6_A::VAL_0X01,
                    2 => ISC6_A::VAL_0X02,
                    3 => ISC6_A::VAL_0X03,
                    _ => unreachable!(),
                }
            }
            ///Low Level of INTX
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == ISC6_A::VAL_0X00
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == ISC6_A::VAL_0X01
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == ISC6_A::VAL_0X02
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == ISC6_A::VAL_0X03
            }
        }
        ///Field `ISC6` writer - External Interrupt 7-4 Sense Control Bit
        pub type ISC6_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ISC6_A, crate::Safe>;
        impl<'a, REG> ISC6_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Low Level of INTX
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(ISC6_A::VAL_0X00)
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(ISC6_A::VAL_0X01)
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(ISC6_A::VAL_0X02)
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(ISC6_A::VAL_0X03)
            }
        }
        /**External Interrupt 7-4 Sense Control Bit

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ISC7_A {
            ///0: Low Level of INTX
            VAL_0X00 = 0,
            ///1: Any Logical Change of INTX
            VAL_0X01 = 1,
            ///2: Falling Edge of INTX
            VAL_0X02 = 2,
            ///3: Rising Edge of INTX
            VAL_0X03 = 3,
        }
        impl From<ISC7_A> for u8 {
            #[inline(always)]
            fn from(variant: ISC7_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for ISC7_A {
            type Ux = u8;
        }
        impl crate::IsEnum for ISC7_A {}
        ///Field `ISC7` reader - External Interrupt 7-4 Sense Control Bit
        pub type ISC7_R = crate::FieldReader<ISC7_A>;
        impl ISC7_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> ISC7_A {
                match self.bits {
                    0 => ISC7_A::VAL_0X00,
                    1 => ISC7_A::VAL_0X01,
                    2 => ISC7_A::VAL_0X02,
                    3 => ISC7_A::VAL_0X03,
                    _ => unreachable!(),
                }
            }
            ///Low Level of INTX
            #[inline(always)]
            pub fn is_val_0x00(&self) -> bool {
                *self == ISC7_A::VAL_0X00
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn is_val_0x01(&self) -> bool {
                *self == ISC7_A::VAL_0X01
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn is_val_0x02(&self) -> bool {
                *self == ISC7_A::VAL_0X02
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn is_val_0x03(&self) -> bool {
                *self == ISC7_A::VAL_0X03
            }
        }
        ///Field `ISC7` writer - External Interrupt 7-4 Sense Control Bit
        pub type ISC7_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ISC7_A, crate::Safe>;
        impl<'a, REG> ISC7_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Low Level of INTX
            #[inline(always)]
            pub fn val_0x00(self) -> &'a mut crate::W<REG> {
                self.variant(ISC7_A::VAL_0X00)
            }
            ///Any Logical Change of INTX
            #[inline(always)]
            pub fn val_0x01(self) -> &'a mut crate::W<REG> {
                self.variant(ISC7_A::VAL_0X01)
            }
            ///Falling Edge of INTX
            #[inline(always)]
            pub fn val_0x02(self) -> &'a mut crate::W<REG> {
                self.variant(ISC7_A::VAL_0X02)
            }
            ///Rising Edge of INTX
            #[inline(always)]
            pub fn val_0x03(self) -> &'a mut crate::W<REG> {
                self.variant(ISC7_A::VAL_0X03)
            }
        }
        impl R {
            ///Bits 0:1 - External Interrupt 7-4 Sense Control Bit
            #[inline(always)]
            pub fn isc4(&self) -> ISC4_R {
                ISC4_R::new(self.bits & 3)
            }
            ///Bits 2:3 - External Interrupt 7-4 Sense Control Bit
            #[inline(always)]
            pub fn isc5(&self) -> ISC5_R {
                ISC5_R::new((self.bits >> 2) & 3)
            }
            ///Bits 4:5 - External Interrupt 7-4 Sense Control Bit
            #[inline(always)]
            pub fn isc6(&self) -> ISC6_R {
                ISC6_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - External Interrupt 7-4 Sense Control Bit
            #[inline(always)]
            pub fn isc7(&self) -> ISC7_R {
                ISC7_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bits 0:1 - External Interrupt 7-4 Sense Control Bit
            #[inline(always)]
            pub fn isc4(&mut self) -> ISC4_W<'_, EICRB_SPEC> {
                ISC4_W::new(self, 0)
            }
            ///Bits 2:3 - External Interrupt 7-4 Sense Control Bit
            #[inline(always)]
            pub fn isc5(&mut self) -> ISC5_W<'_, EICRB_SPEC> {
                ISC5_W::new(self, 2)
            }
            ///Bits 4:5 - External Interrupt 7-4 Sense Control Bit
            #[inline(always)]
            pub fn isc6(&mut self) -> ISC6_W<'_, EICRB_SPEC> {
                ISC6_W::new(self, 4)
            }
            ///Bits 6:7 - External Interrupt 7-4 Sense Control Bit
            #[inline(always)]
            pub fn isc7(&mut self) -> ISC7_W<'_, EICRB_SPEC> {
                ISC7_W::new(self, 6)
            }
        }
        /**External Interrupt Control Register B

You can [`read`](crate::Reg::read) this register and get [`eicrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eicrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct EICRB_SPEC;
        impl crate::RegisterSpec for EICRB_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`eicrb::R`](R) reader structure
        impl crate::Readable for EICRB_SPEC {}
        ///`write(|w| ..)` method takes [`eicrb::W`](W) writer structure
        impl crate::Writable for EICRB_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets EICRB to value 0
        impl crate::Resettable for EICRB_SPEC {}
    }
    /**EIFR (rw) register accessor: External Interrupt Flag Register

You can [`read`](crate::Reg::read) this register and get [`eifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eifr`] module*/
    pub type EIFR = crate::Reg<eifr::EIFR_SPEC>;
    ///External Interrupt Flag Register
    pub mod eifr {
        ///Register `EIFR` reader
        pub type R = crate::R<EIFR_SPEC>;
        ///Register `EIFR` writer
        pub type W = crate::W<EIFR_SPEC>;
        ///Field `INTF` reader - External Interrupt Flags
        pub type INTF_R = crate::FieldReader;
        ///Field `INTF` writer - External Interrupt Flags
        pub type INTF_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
        impl R {
            ///Bits 0:7 - External Interrupt Flags
            #[inline(always)]
            pub fn intf(&self) -> INTF_R {
                INTF_R::new(self.bits)
            }
        }
        impl W {
            ///Bits 0:7 - External Interrupt Flags
            #[inline(always)]
            pub fn intf(&mut self) -> INTF_W<'_, EIFR_SPEC> {
                INTF_W::new(self, 0)
            }
        }
        /**External Interrupt Flag Register

You can [`read`](crate::Reg::read) this register and get [`eifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct EIFR_SPEC;
        impl crate::RegisterSpec for EIFR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`eifr::R`](R) reader structure
        impl crate::Readable for EIFR_SPEC {}
        ///`write(|w| ..)` method takes [`eifr::W`](W) writer structure
        impl crate::Writable for EIFR_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets EIFR to value 0
        impl crate::Resettable for EIFR_SPEC {}
    }
    /**EIMSK (rw) register accessor: External Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`eimsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eimsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eimsk`] module*/
    pub type EIMSK = crate::Reg<eimsk::EIMSK_SPEC>;
    ///External Interrupt Mask Register
    pub mod eimsk {
        ///Register `EIMSK` reader
        pub type R = crate::R<EIMSK_SPEC>;
        ///Register `EIMSK` writer
        pub type W = crate::W<EIMSK_SPEC>;
        ///Field `INT` reader - External Interrupt Request 7 Enable
        pub type INT_R = crate::FieldReader;
        ///Field `INT` writer - External Interrupt Request 7 Enable
        pub type INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
        impl R {
            ///Bits 0:7 - External Interrupt Request 7 Enable
            #[inline(always)]
            pub fn int(&self) -> INT_R {
                INT_R::new(self.bits)
            }
        }
        impl W {
            ///Bits 0:7 - External Interrupt Request 7 Enable
            #[inline(always)]
            pub fn int(&mut self) -> INT_W<'_, EIMSK_SPEC> {
                INT_W::new(self, 0)
            }
        }
        /**External Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`eimsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eimsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct EIMSK_SPEC;
        impl crate::RegisterSpec for EIMSK_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`eimsk::R`](R) reader structure
        impl crate::Readable for EIMSK_SPEC {}
        ///`write(|w| ..)` method takes [`eimsk::W`](W) writer structure
        impl crate::Writable for EIMSK_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets EIMSK to value 0
        impl crate::Resettable for EIMSK_SPEC {}
    }
    /**PCICR (rw) register accessor: Pin Change Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`pcicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcicr`] module*/
    pub type PCICR = crate::Reg<pcicr::PCICR_SPEC>;
    ///Pin Change Interrupt Control Register
    pub mod pcicr {
        ///Register `PCICR` reader
        pub type R = crate::R<PCICR_SPEC>;
        ///Register `PCICR` writer
        pub type W = crate::W<PCICR_SPEC>;
        ///Field `PCIE` reader - Pin Change Interrupt Enables
        pub type PCIE_R = crate::FieldReader;
        ///Field `PCIE` writer - Pin Change Interrupt Enables
        pub type PCIE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
        impl R {
            ///Bits 0:2 - Pin Change Interrupt Enables
            #[inline(always)]
            pub fn pcie(&self) -> PCIE_R {
                PCIE_R::new(self.bits & 7)
            }
        }
        impl W {
            ///Bits 0:2 - Pin Change Interrupt Enables
            #[inline(always)]
            pub fn pcie(&mut self) -> PCIE_W<'_, PCICR_SPEC> {
                PCIE_W::new(self, 0)
            }
        }
        /**Pin Change Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`pcicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PCICR_SPEC;
        impl crate::RegisterSpec for PCICR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pcicr::R`](R) reader structure
        impl crate::Readable for PCICR_SPEC {}
        ///`write(|w| ..)` method takes [`pcicr::W`](W) writer structure
        impl crate::Writable for PCICR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PCICR to value 0
        impl crate::Resettable for PCICR_SPEC {}
    }
    /**PCIFR (rw) register accessor: Pin Change Interrupt Flag Register

You can [`read`](crate::Reg::read) this register and get [`pcifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcifr`] module*/
    pub type PCIFR = crate::Reg<pcifr::PCIFR_SPEC>;
    ///Pin Change Interrupt Flag Register
    pub mod pcifr {
        ///Register `PCIFR` reader
        pub type R = crate::R<PCIFR_SPEC>;
        ///Register `PCIFR` writer
        pub type W = crate::W<PCIFR_SPEC>;
        ///Field `PCIF` reader - Pin Change Interrupt Flags
        pub type PCIF_R = crate::FieldReader;
        ///Field `PCIF` writer - Pin Change Interrupt Flags
        pub type PCIF_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
        impl R {
            ///Bits 0:2 - Pin Change Interrupt Flags
            #[inline(always)]
            pub fn pcif(&self) -> PCIF_R {
                PCIF_R::new(self.bits & 7)
            }
        }
        impl W {
            ///Bits 0:2 - Pin Change Interrupt Flags
            #[inline(always)]
            pub fn pcif(&mut self) -> PCIF_W<'_, PCIFR_SPEC> {
                PCIF_W::new(self, 0)
            }
        }
        /**Pin Change Interrupt Flag Register

You can [`read`](crate::Reg::read) this register and get [`pcifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PCIFR_SPEC;
        impl crate::RegisterSpec for PCIFR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pcifr::R`](R) reader structure
        impl crate::Readable for PCIFR_SPEC {}
        ///`write(|w| ..)` method takes [`pcifr::W`](W) writer structure
        impl crate::Writable for PCIFR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PCIFR to value 0
        impl crate::Resettable for PCIFR_SPEC {}
    }
    /**PCMSK0 (rw) register accessor: Pin Change Mask Register 0

You can [`read`](crate::Reg::read) this register and get [`pcmsk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmsk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcmsk0`] module*/
    pub type PCMSK0 = crate::Reg<pcmsk0::PCMSK0_SPEC>;
    ///Pin Change Mask Register 0
    pub mod pcmsk0 {
        ///Register `PCMSK0` reader
        pub type R = crate::R<PCMSK0_SPEC>;
        ///Register `PCMSK0` writer
        pub type W = crate::W<PCMSK0_SPEC>;
        ///Field `PCINT` reader - Pin Change Mask interrupt
        pub type PCINT_R = crate::FieldReader;
        ///Field `PCINT` writer - Pin Change Mask interrupt
        pub type PCINT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
        impl R {
            ///Bits 0:7 - Pin Change Mask interrupt
            #[inline(always)]
            pub fn pcint(&self) -> PCINT_R {
                PCINT_R::new(self.bits)
            }
        }
        impl W {
            ///Bits 0:7 - Pin Change Mask interrupt
            #[inline(always)]
            pub fn pcint(&mut self) -> PCINT_W<'_, PCMSK0_SPEC> {
                PCINT_W::new(self, 0)
            }
        }
        /**Pin Change Mask Register 0

You can [`read`](crate::Reg::read) this register and get [`pcmsk0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmsk0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PCMSK0_SPEC;
        impl crate::RegisterSpec for PCMSK0_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pcmsk0::R`](R) reader structure
        impl crate::Readable for PCMSK0_SPEC {}
        ///`write(|w| ..)` method takes [`pcmsk0::W`](W) writer structure
        impl crate::Writable for PCMSK0_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets PCMSK0 to value 0
        impl crate::Resettable for PCMSK0_SPEC {}
    }
    /**PCMSK1 (rw) register accessor: Pin Change Mask Register 1

You can [`read`](crate::Reg::read) this register and get [`pcmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcmsk1`] module*/
    pub type PCMSK1 = crate::Reg<pcmsk1::PCMSK1_SPEC>;
    ///Pin Change Mask Register 1
    pub mod pcmsk1 {
        ///Register `PCMSK1` reader
        pub type R = crate::R<PCMSK1_SPEC>;
        ///Register `PCMSK1` writer
        pub type W = crate::W<PCMSK1_SPEC>;
        ///Field `PCINT` reader - Pin Change Mask interrupt
        pub type PCINT_R = crate::FieldReader;
        ///Field `PCINT` writer - Pin Change Mask interrupt
        pub type PCINT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
        impl R {
            ///Bits 0:7 - Pin Change Mask interrupt
            #[inline(always)]
            pub fn pcint(&self) -> PCINT_R {
                PCINT_R::new(self.bits)
            }
        }
        impl W {
            ///Bits 0:7 - Pin Change Mask interrupt
            #[inline(always)]
            pub fn pcint(&mut self) -> PCINT_W<'_, PCMSK1_SPEC> {
                PCINT_W::new(self, 0)
            }
        }
        /**Pin Change Mask Register 1

You can [`read`](crate::Reg::read) this register and get [`pcmsk1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmsk1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PCMSK1_SPEC;
        impl crate::RegisterSpec for PCMSK1_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pcmsk1::R`](R) reader structure
        impl crate::Readable for PCMSK1_SPEC {}
        ///`write(|w| ..)` method takes [`pcmsk1::W`](W) writer structure
        impl crate::Writable for PCMSK1_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets PCMSK1 to value 0
        impl crate::Resettable for PCMSK1_SPEC {}
    }
    /**PCMSK2 (rw) register accessor: Pin Change Mask Register 2

You can [`read`](crate::Reg::read) this register and get [`pcmsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcmsk2`] module*/
    pub type PCMSK2 = crate::Reg<pcmsk2::PCMSK2_SPEC>;
    ///Pin Change Mask Register 2
    pub mod pcmsk2 {
        ///Register `PCMSK2` reader
        pub type R = crate::R<PCMSK2_SPEC>;
        ///Register `PCMSK2` writer
        pub type W = crate::W<PCMSK2_SPEC>;
        ///Field `PCINT` reader - Pin Change Mask interrupt
        pub type PCINT_R = crate::FieldReader;
        ///Field `PCINT` writer - Pin Change Mask interrupt
        pub type PCINT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
        impl R {
            ///Bits 0:7 - Pin Change Mask interrupt
            #[inline(always)]
            pub fn pcint(&self) -> PCINT_R {
                PCINT_R::new(self.bits)
            }
        }
        impl W {
            ///Bits 0:7 - Pin Change Mask interrupt
            #[inline(always)]
            pub fn pcint(&mut self) -> PCINT_W<'_, PCMSK2_SPEC> {
                PCINT_W::new(self, 0)
            }
        }
        /**Pin Change Mask Register 2

You can [`read`](crate::Reg::read) this register and get [`pcmsk2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmsk2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PCMSK2_SPEC;
        impl crate::RegisterSpec for PCMSK2_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pcmsk2::R`](R) reader structure
        impl crate::Readable for PCMSK2_SPEC {}
        ///`write(|w| ..)` method takes [`pcmsk2::W`](W) writer structure
        impl crate::Writable for PCMSK2_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets PCMSK2 to value 0
        impl crate::Resettable for PCMSK2_SPEC {}
    }
}
///Fuses
pub type FUSE = crate::Periph<fuse::RegisterBlock, 0>;
impl core::fmt::Debug for FUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUSE").finish()
    }
}
///Fuses
pub mod fuse {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        low: LOW,
        high: HIGH,
        extended: EXTENDED,
    }
    impl RegisterBlock {
        ///0x00 - No Description.
        #[inline(always)]
        pub const fn low(&self) -> &LOW {
            &self.low
        }
        ///0x01 - No Description.
        #[inline(always)]
        pub const fn high(&self) -> &HIGH {
            &self.high
        }
        ///0x02 - No Description.
        #[inline(always)]
        pub const fn extended(&self) -> &EXTENDED {
            &self.extended
        }
    }
    /**EXTENDED (rw) register accessor: No Description.

You can [`read`](crate::Reg::read) this register and get [`extended::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extended::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@extended`] module*/
    pub type EXTENDED = crate::Reg<extended::EXTENDED_SPEC>;
    ///No Description.
    pub mod extended {
        ///Register `EXTENDED` reader
        pub type R = crate::R<EXTENDED_SPEC>;
        ///Register `EXTENDED` writer
        pub type W = crate::W<EXTENDED_SPEC>;
        /**Brown-out Detector trigger level

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum BODLEVEL_A {
            ///4: Brown-out detection at VCC=4.3 V
            _4V3 = 4,
            ///5: Brown-out detection at VCC=2.7 V
            _2V7 = 5,
            ///6: Brown-out detection at VCC=1.8 V
            _1V8 = 6,
            ///7: Brown-out detection disabled
            DISABLED = 7,
        }
        impl From<BODLEVEL_A> for u8 {
            #[inline(always)]
            fn from(variant: BODLEVEL_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for BODLEVEL_A {
            type Ux = u8;
        }
        impl crate::IsEnum for BODLEVEL_A {}
        ///Field `BODLEVEL` reader - Brown-out Detector trigger level
        pub type BODLEVEL_R = crate::FieldReader<BODLEVEL_A>;
        impl BODLEVEL_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<BODLEVEL_A> {
                match self.bits {
                    4 => Some(BODLEVEL_A::_4V3),
                    5 => Some(BODLEVEL_A::_2V7),
                    6 => Some(BODLEVEL_A::_1V8),
                    7 => Some(BODLEVEL_A::DISABLED),
                    _ => None,
                }
            }
            ///Brown-out detection at VCC=4.3 V
            #[inline(always)]
            pub fn is_4v3(&self) -> bool {
                *self == BODLEVEL_A::_4V3
            }
            ///Brown-out detection at VCC=2.7 V
            #[inline(always)]
            pub fn is_2v7(&self) -> bool {
                *self == BODLEVEL_A::_2V7
            }
            ///Brown-out detection at VCC=1.8 V
            #[inline(always)]
            pub fn is_1v8(&self) -> bool {
                *self == BODLEVEL_A::_1V8
            }
            ///Brown-out detection disabled
            #[inline(always)]
            pub fn is_disabled(&self) -> bool {
                *self == BODLEVEL_A::DISABLED
            }
        }
        ///Field `BODLEVEL` writer - Brown-out Detector trigger level
        pub type BODLEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BODLEVEL_A>;
        impl<'a, REG> BODLEVEL_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Brown-out detection at VCC=4.3 V
            #[inline(always)]
            pub fn _4v3(self) -> &'a mut crate::W<REG> {
                self.variant(BODLEVEL_A::_4V3)
            }
            ///Brown-out detection at VCC=2.7 V
            #[inline(always)]
            pub fn _2v7(self) -> &'a mut crate::W<REG> {
                self.variant(BODLEVEL_A::_2V7)
            }
            ///Brown-out detection at VCC=1.8 V
            #[inline(always)]
            pub fn _1v8(self) -> &'a mut crate::W<REG> {
                self.variant(BODLEVEL_A::_1V8)
            }
            ///Brown-out detection disabled
            #[inline(always)]
            pub fn disabled(self) -> &'a mut crate::W<REG> {
                self.variant(BODLEVEL_A::DISABLED)
            }
        }
        impl R {
            ///Bits 0:2 - Brown-out Detector trigger level
            #[inline(always)]
            pub fn bodlevel(&self) -> BODLEVEL_R {
                BODLEVEL_R::new(self.bits & 7)
            }
        }
        impl W {
            ///Bits 0:2 - Brown-out Detector trigger level
            #[inline(always)]
            pub fn bodlevel(&mut self) -> BODLEVEL_W<'_, EXTENDED_SPEC> {
                BODLEVEL_W::new(self, 0)
            }
        }
        /**No Description.

You can [`read`](crate::Reg::read) this register and get [`extended::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extended::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct EXTENDED_SPEC;
        impl crate::RegisterSpec for EXTENDED_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`extended::R`](R) reader structure
        impl crate::Readable for EXTENDED_SPEC {}
        ///`write(|w| ..)` method takes [`extended::W`](W) writer structure
        impl crate::Writable for EXTENDED_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets EXTENDED to value 0
        impl crate::Resettable for EXTENDED_SPEC {}
    }
    /**HIGH (rw) register accessor: No Description.

You can [`read`](crate::Reg::read) this register and get [`high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@high`] module*/
    pub type HIGH = crate::Reg<high::HIGH_SPEC>;
    ///No Description.
    pub mod high {
        ///Register `HIGH` reader
        pub type R = crate::R<HIGH_SPEC>;
        ///Register `HIGH` writer
        pub type W = crate::W<HIGH_SPEC>;
        ///Field `BOOTRST` reader - Boot Reset vector Enabled
        pub type BOOTRST_R = crate::BitReader;
        ///Field `BOOTRST` writer - Boot Reset vector Enabled
        pub type BOOTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
        /**Select Boot Size

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum BOOTSZ_A {
            ///0: Boot Flash size=4096 words start address=$1F000
            _4096W_1F000 = 0,
            ///1: Boot Flash size=2048 words start address=$1F800
            _2048W_1F800 = 1,
            ///2: Boot Flash size=1024 words start address=$1FC00
            _1024W_1FC00 = 2,
            ///3: Boot Flash size=512 words start address=$1FE00
            _512W_1FE00 = 3,
        }
        impl From<BOOTSZ_A> for u8 {
            #[inline(always)]
            fn from(variant: BOOTSZ_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for BOOTSZ_A {
            type Ux = u8;
        }
        impl crate::IsEnum for BOOTSZ_A {}
        ///Field `BOOTSZ` reader - Select Boot Size
        pub type BOOTSZ_R = crate::FieldReader<BOOTSZ_A>;
        impl BOOTSZ_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> BOOTSZ_A {
                match self.bits {
                    0 => BOOTSZ_A::_4096W_1F000,
                    1 => BOOTSZ_A::_2048W_1F800,
                    2 => BOOTSZ_A::_1024W_1FC00,
                    3 => BOOTSZ_A::_512W_1FE00,
                    _ => unreachable!(),
                }
            }
            ///Boot Flash size=4096 words start address=$1F000
            #[inline(always)]
            pub fn is_4096w_1f000(&self) -> bool {
                *self == BOOTSZ_A::_4096W_1F000
            }
            ///Boot Flash size=2048 words start address=$1F800
            #[inline(always)]
            pub fn is_2048w_1f800(&self) -> bool {
                *self == BOOTSZ_A::_2048W_1F800
            }
            ///Boot Flash size=1024 words start address=$1FC00
            #[inline(always)]
            pub fn is_1024w_1fc00(&self) -> bool {
                *self == BOOTSZ_A::_1024W_1FC00
            }
            ///Boot Flash size=512 words start address=$1FE00
            #[inline(always)]
            pub fn is_512w_1fe00(&self) -> bool {
                *self == BOOTSZ_A::_512W_1FE00
            }
        }
        ///Field `BOOTSZ` writer - Select Boot Size
        pub type BOOTSZ_W<'a, REG> = crate::FieldWriter<
            'a,
            REG,
            2,
            BOOTSZ_A,
            crate::Safe,
        >;
        impl<'a, REG> BOOTSZ_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Boot Flash size=4096 words start address=$1F000
            #[inline(always)]
            pub fn _4096w_1f000(self) -> &'a mut crate::W<REG> {
                self.variant(BOOTSZ_A::_4096W_1F000)
            }
            ///Boot Flash size=2048 words start address=$1F800
            #[inline(always)]
            pub fn _2048w_1f800(self) -> &'a mut crate::W<REG> {
                self.variant(BOOTSZ_A::_2048W_1F800)
            }
            ///Boot Flash size=1024 words start address=$1FC00
            #[inline(always)]
            pub fn _1024w_1fc00(self) -> &'a mut crate::W<REG> {
                self.variant(BOOTSZ_A::_1024W_1FC00)
            }
            ///Boot Flash size=512 words start address=$1FE00
            #[inline(always)]
            pub fn _512w_1fe00(self) -> &'a mut crate::W<REG> {
                self.variant(BOOTSZ_A::_512W_1FE00)
            }
        }
        ///Field `EESAVE` reader - Preserve EEPROM through the Chip Erase cycle
        pub type EESAVE_R = crate::BitReader;
        ///Field `EESAVE` writer - Preserve EEPROM through the Chip Erase cycle
        pub type EESAVE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `WDTON` reader - Watchdog timer always on
        pub type WDTON_R = crate::BitReader;
        ///Field `WDTON` writer - Watchdog timer always on
        pub type WDTON_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `SPIEN` reader - Serial program downloading (SPI) enabled
        pub type SPIEN_R = crate::BitReader;
        ///Field `SPIEN` writer - Serial program downloading (SPI) enabled
        pub type SPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `JTAGEN` reader - JTAG Interface Enabled
        pub type JTAGEN_R = crate::BitReader;
        ///Field `JTAGEN` writer - JTAG Interface Enabled
        pub type JTAGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCDEN` reader - On-Chip Debug Enabled
        pub type OCDEN_R = crate::BitReader;
        ///Field `OCDEN` writer - On-Chip Debug Enabled
        pub type OCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Boot Reset vector Enabled
            #[inline(always)]
            pub fn bootrst(&self) -> BOOTRST_R {
                BOOTRST_R::new((self.bits & 1) != 0)
            }
            ///Bits 1:2 - Select Boot Size
            #[inline(always)]
            pub fn bootsz(&self) -> BOOTSZ_R {
                BOOTSZ_R::new((self.bits >> 1) & 3)
            }
            ///Bit 3 - Preserve EEPROM through the Chip Erase cycle
            #[inline(always)]
            pub fn eesave(&self) -> EESAVE_R {
                EESAVE_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Watchdog timer always on
            #[inline(always)]
            pub fn wdton(&self) -> WDTON_R {
                WDTON_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Serial program downloading (SPI) enabled
            #[inline(always)]
            pub fn spien(&self) -> SPIEN_R {
                SPIEN_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - JTAG Interface Enabled
            #[inline(always)]
            pub fn jtagen(&self) -> JTAGEN_R {
                JTAGEN_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - On-Chip Debug Enabled
            #[inline(always)]
            pub fn ocden(&self) -> OCDEN_R {
                OCDEN_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Boot Reset vector Enabled
            #[inline(always)]
            pub fn bootrst(&mut self) -> BOOTRST_W<'_, HIGH_SPEC> {
                BOOTRST_W::new(self, 0)
            }
            ///Bits 1:2 - Select Boot Size
            #[inline(always)]
            pub fn bootsz(&mut self) -> BOOTSZ_W<'_, HIGH_SPEC> {
                BOOTSZ_W::new(self, 1)
            }
            ///Bit 3 - Preserve EEPROM through the Chip Erase cycle
            #[inline(always)]
            pub fn eesave(&mut self) -> EESAVE_W<'_, HIGH_SPEC> {
                EESAVE_W::new(self, 3)
            }
            ///Bit 4 - Watchdog timer always on
            #[inline(always)]
            pub fn wdton(&mut self) -> WDTON_W<'_, HIGH_SPEC> {
                WDTON_W::new(self, 4)
            }
            ///Bit 5 - Serial program downloading (SPI) enabled
            #[inline(always)]
            pub fn spien(&mut self) -> SPIEN_W<'_, HIGH_SPEC> {
                SPIEN_W::new(self, 5)
            }
            ///Bit 6 - JTAG Interface Enabled
            #[inline(always)]
            pub fn jtagen(&mut self) -> JTAGEN_W<'_, HIGH_SPEC> {
                JTAGEN_W::new(self, 6)
            }
            ///Bit 7 - On-Chip Debug Enabled
            #[inline(always)]
            pub fn ocden(&mut self) -> OCDEN_W<'_, HIGH_SPEC> {
                OCDEN_W::new(self, 7)
            }
        }
        /**No Description.

You can [`read`](crate::Reg::read) this register and get [`high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct HIGH_SPEC;
        impl crate::RegisterSpec for HIGH_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`high::R`](R) reader structure
        impl crate::Readable for HIGH_SPEC {}
        ///`write(|w| ..)` method takes [`high::W`](W) writer structure
        impl crate::Writable for HIGH_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets HIGH to value 0
        impl crate::Resettable for HIGH_SPEC {}
    }
    /**LOW (rw) register accessor: No Description.

You can [`read`](crate::Reg::read) this register and get [`low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@low`] module*/
    pub type LOW = crate::Reg<low::LOW_SPEC>;
    ///No Description.
    pub mod low {
        ///Register `LOW` reader
        pub type R = crate::R<LOW_SPEC>;
        ///Register `LOW` writer
        pub type W = crate::W<LOW_SPEC>;
        /**Select Clock Source

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum SUT_CKSEL_A {
            ///0: Ext. Clock; Start-up time: 6 CK + 0 ms
            EXTCLK_6CK_0MS = 0,
            ///2: Int. RC Osc.; Start-up time: 6 CK + 0 ms
            INTRCOSC_6CK_0MS = 2,
            ///3: Int. 128kHz RC Osc.; Start-up time: 6 CK + 0 ms
            INTRCOSC_128KHZ_6CK_0MS = 3,
            ///4: Ext. Low-Freq. Crystal; Start-up time: 1K CK + 0 ms
            EXTLOFXTAL_1KCK_0MS = 4,
            ///5: Ext. Low-Freq. Crystal; Start-up time: 32K CK + 0 ms
            EXTLOFXTAL_32KCK_0MS = 5,
            ///6: Full Swing Oscillator; Start-up time: 258 CK + 4.1 ms; Ceramic res.; fast rising power
            FSOSC_258CK_4MS1_CRES_FASTPWR = 6,
            ///7: Full Swing Oscillator; Start-up time: 1K CK + 65 ms; Ceramic res.; slowly rising power
            FSOSC_1KCK_65MS_CRES_SLOWPWR = 7,
            ///8: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 4.1 ms
            EXTXOSC_0MHZ4_0MHZ9_258CK_4MS1 = 8,
            ///9: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 65 ms
            EXTXOSC_0MHZ4_0MHZ9_1KCK_65MS = 9,
            ///10: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 4.1 ms
            EXTXOSC_0MHZ9_3MHZ_258CK_4MS1 = 10,
            ///11: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 65 ms
            EXTXOSC_0MHZ9_3MHZ_1KCK_65MS = 11,
            ///12: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 4.1 ms
            EXTXOSC_3MHZ_8MHZ_258CK_4MS1 = 12,
            ///13: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 65 ms
            EXTXOSC_3MHZ_8MHZ_1KCK_65MS = 13,
            ///14: Ext. Crystal Osc. 8.0- MHz; Start-up time: 258 CK + 4.1 ms
            EXTXOSC_8MHZ_XX_258CK_4MS1 = 14,
            ///15: Ext. Crystal Osc. 8.0- MHz; Start-up time: 1K CK + 65 ms
            EXTXOSC_8MHZ_XX_1KCK_65MS = 15,
            ///16: Ext. Clock; Start-up time: 6 CK + 4.1 ms
            EXTCLK_6CK_4MS1 = 16,
            ///18: Int. RC Osc.; Start-up time: 6 CK + 4.1 ms
            INTRCOSC_6CK_4MS1 = 18,
            ///19: Int. 128kHz RC Osc.; Start-up time: 6 CK + 4 ms
            INTRCOSC_128KHZ_6CK_4MS = 19,
            ///20: Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4.1 ms
            EXTLOFXTAL_1KCK_4MS1 = 20,
            ///21: Ext. Low-Freq. Crystal; Start-up time: 32K CK + 4.1 ms
            EXTLOFXTAL_32KCK_4MS1 = 21,
            ///22: Full Swing Oscillator; Start-up time: 258 CK + 65 ms; Ceramic res.; slowly rising power
            FSOSC_258CK_65MS_CRES_SLOWPWR = 22,
            ///23: Full Swing Oscillator; Start-up time: 16K CK + 0 ms; Crystal Osc.; BOD enabled
            FSOSC_16KCK_0MS_XOSC_BODEN = 23,
            ///24: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 65 ms
            EXTXOSC_0MHZ4_0MHZ9_258CK_65MS = 24,
            ///25: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 0 ms
            EXTXOSC_0MHZ4_0MHZ9_16KCK_0MS = 25,
            ///26: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 65 ms
            EXTXOSC_0MHZ9_3MHZ_258CK_65MS = 26,
            ///27: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 0 ms
            EXTXOSC_0MHZ9_3MHZ_16KCK_0MS = 27,
            ///28: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 65 ms
            EXTXOSC_3MHZ_8MHZ_258CK_65MS = 28,
            ///29: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 0 ms
            EXTXOSC_3MHZ_8MHZ_16KCK_0MS = 29,
            ///30: Ext. Crystal Osc. 8.0- MHz; Start-up time: 258 CK + 65 ms
            EXTXOSC_8MHZ_XX_258CK_65MS = 30,
            ///31: Ext. Crystal Osc. 8.0- MHz; Start-up time: 16K CK + 0 ms
            EXTXOSC_8MHZ_XX_16KCK_0MS = 31,
            ///32: Ext. Clock; Start-up time: 6 CK + 65 ms
            EXTCLK_6CK_65MS = 32,
            ///34: Int. RC Osc.; Start-up time: 6 CK + 65 ms
            INTRCOSC_6CK_65MS = 34,
            ///35: Int. 128kHz RC Osc.; Start-up time: 6 CK + 64 ms
            INTRCOSC_128KHZ_6CK_64MS = 35,
            ///36: Ext. Low-Freq. Crystal; Start-up time: 1K CK + 65 ms
            EXTLOFXTAL_1KCK_65MS = 36,
            ///37: Ext. Low-Freq. Crystal; Start-up time: 32K CK + 65 ms
            EXTLOFXTAL_32KCK_65MS = 37,
            ///38: Full Swing Oscillator; Start-up time: 1K CK + 0 ms; Ceramic res.; BOD enable
            FSOSC_1KCK_0MS_CRES_BODEN = 38,
            ///39: Full Swing Oscillator; Start-up time: 16K CK + 4.1 ms; Crystal Osc.; fast rising power
            FSOSC_16KCK_4MS1_XOSC_FASTPWR = 39,
            ///40: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 0 ms
            EXTXOSC_0MHZ4_0MHZ9_1KCK_0MS = 40,
            ///41: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 4.1 ms
            EXTXOSC_0MHZ4_0MHZ9_16KCK_4MS1 = 41,
            ///42: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 0 ms
            EXTXOSC_0MHZ9_3MHZ_1KCK_0MS = 42,
            ///43: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 4.1 ms
            EXTXOSC_0MHZ9_3MHZ_16KCK_4MS1 = 43,
            ///44: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 0 ms
            EXTXOSC_3MHZ_8MHZ_1KCK_0MS = 44,
            ///45: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 4.1 ms
            EXTXOSC_3MHZ_8MHZ_16KCK_4MS1 = 45,
            ///46: Ext. Crystal Osc. 8.0- MHz; Start-up time: 1K CK + 0 ms
            EXTXOSC_8MHZ_XX_1KCK_0MS = 46,
            ///47: Ext. Crystal Osc. 8.0- MHz; Start-up time: 16K CK + 4.1 ms
            EXTXOSC_8MHZ_XX_16KCK_4MS1 = 47,
            ///54: Full Swing Oscillator; Start-up time: 1K CK + 4.1 ms; Ceramic res.; fast rising power
            FSOSC_1KCK_4MS1_CRES_FASTPWR = 54,
            ///55: Full Swing Oscillator; Start-up time: 16K CK + 65 ms; Crystal Osc.; slowly rising power
            FSOSC_16KCK_65MS_XOSC_SLOWPWR = 55,
            ///56: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 4.1 ms
            EXTXOSC_0MHZ4_0MHZ9_1KCK_4MS1 = 56,
            ///57: Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 65 ms
            EXTXOSC_0MHZ4_0MHZ9_16KCK_65MS = 57,
            ///58: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 4.1 ms
            EXTXOSC_0MHZ9_3MHZ_1KCK_4MS1 = 58,
            ///59: Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 65 ms
            EXTXOSC_0MHZ9_3MHZ_16KCK_65MS = 59,
            ///60: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 4.1 ms
            EXTXOSC_3MHZ_8MHZ_1KCK_4MS1 = 60,
            ///61: Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 65 ms
            EXTXOSC_3MHZ_8MHZ_16KCK_65MS = 61,
            ///62: Ext. Crystal Osc. 8.0- MHz; Start-up time: 1K CK + 4.1 ms
            EXTXOSC_8MHZ_XX_1KCK_4MS1 = 62,
            ///63: Ext. Crystal Osc. 8.0- MHz; Start-up time: 16K CK + 65 ms
            EXTXOSC_8MHZ_XX_16KCK_65MS = 63,
        }
        impl From<SUT_CKSEL_A> for u8 {
            #[inline(always)]
            fn from(variant: SUT_CKSEL_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for SUT_CKSEL_A {
            type Ux = u8;
        }
        impl crate::IsEnum for SUT_CKSEL_A {}
        ///Field `SUT_CKSEL` reader - Select Clock Source
        pub type SUT_CKSEL_R = crate::FieldReader<SUT_CKSEL_A>;
        impl SUT_CKSEL_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<SUT_CKSEL_A> {
                match self.bits {
                    0 => Some(SUT_CKSEL_A::EXTCLK_6CK_0MS),
                    2 => Some(SUT_CKSEL_A::INTRCOSC_6CK_0MS),
                    3 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_0MS),
                    4 => Some(SUT_CKSEL_A::EXTLOFXTAL_1KCK_0MS),
                    5 => Some(SUT_CKSEL_A::EXTLOFXTAL_32KCK_0MS),
                    6 => Some(SUT_CKSEL_A::FSOSC_258CK_4MS1_CRES_FASTPWR),
                    7 => Some(SUT_CKSEL_A::FSOSC_1KCK_65MS_CRES_SLOWPWR),
                    8 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_258CK_4MS1),
                    9 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_1KCK_65MS),
                    10 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_258CK_4MS1),
                    11 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_1KCK_65MS),
                    12 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_258CK_4MS1),
                    13 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_1KCK_65MS),
                    14 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_258CK_4MS1),
                    15 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_1KCK_65MS),
                    16 => Some(SUT_CKSEL_A::EXTCLK_6CK_4MS1),
                    18 => Some(SUT_CKSEL_A::INTRCOSC_6CK_4MS1),
                    19 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_4MS),
                    20 => Some(SUT_CKSEL_A::EXTLOFXTAL_1KCK_4MS1),
                    21 => Some(SUT_CKSEL_A::EXTLOFXTAL_32KCK_4MS1),
                    22 => Some(SUT_CKSEL_A::FSOSC_258CK_65MS_CRES_SLOWPWR),
                    23 => Some(SUT_CKSEL_A::FSOSC_16KCK_0MS_XOSC_BODEN),
                    24 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_258CK_65MS),
                    25 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_0MS),
                    26 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_258CK_65MS),
                    27 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_0MS),
                    28 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_258CK_65MS),
                    29 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_0MS),
                    30 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_258CK_65MS),
                    31 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_0MS),
                    32 => Some(SUT_CKSEL_A::EXTCLK_6CK_65MS),
                    34 => Some(SUT_CKSEL_A::INTRCOSC_6CK_65MS),
                    35 => Some(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_64MS),
                    36 => Some(SUT_CKSEL_A::EXTLOFXTAL_1KCK_65MS),
                    37 => Some(SUT_CKSEL_A::EXTLOFXTAL_32KCK_65MS),
                    38 => Some(SUT_CKSEL_A::FSOSC_1KCK_0MS_CRES_BODEN),
                    39 => Some(SUT_CKSEL_A::FSOSC_16KCK_4MS1_XOSC_FASTPWR),
                    40 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_1KCK_0MS),
                    41 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_4MS1),
                    42 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_1KCK_0MS),
                    43 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_4MS1),
                    44 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_1KCK_0MS),
                    45 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_4MS1),
                    46 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_1KCK_0MS),
                    47 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_4MS1),
                    54 => Some(SUT_CKSEL_A::FSOSC_1KCK_4MS1_CRES_FASTPWR),
                    55 => Some(SUT_CKSEL_A::FSOSC_16KCK_65MS_XOSC_SLOWPWR),
                    56 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_1KCK_4MS1),
                    57 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_65MS),
                    58 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_1KCK_4MS1),
                    59 => Some(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_65MS),
                    60 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_1KCK_4MS1),
                    61 => Some(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_65MS),
                    62 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_1KCK_4MS1),
                    63 => Some(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_65MS),
                    _ => None,
                }
            }
            ///Ext. Clock; Start-up time: 6 CK + 0 ms
            #[inline(always)]
            pub fn is_extclk_6ck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTCLK_6CK_0MS
            }
            ///Int. RC Osc.; Start-up time: 6 CK + 0 ms
            #[inline(always)]
            pub fn is_intrcosc_6ck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::INTRCOSC_6CK_0MS
            }
            ///Int. 128kHz RC Osc.; Start-up time: 6 CK + 0 ms
            #[inline(always)]
            pub fn is_intrcosc_128khz_6ck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_0MS
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 1K CK + 0 ms
            #[inline(always)]
            pub fn is_extlofxtal_1kck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTLOFXTAL_1KCK_0MS
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 32K CK + 0 ms
            #[inline(always)]
            pub fn is_extlofxtal_32kck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTLOFXTAL_32KCK_0MS
            }
            ///Full Swing Oscillator; Start-up time: 258 CK + 4.1 ms; Ceramic res.; fast rising power
            #[inline(always)]
            pub fn is_fsosc_258ck_4ms1_cres_fastpwr(&self) -> bool {
                *self == SUT_CKSEL_A::FSOSC_258CK_4MS1_CRES_FASTPWR
            }
            ///Full Swing Oscillator; Start-up time: 1K CK + 65 ms; Ceramic res.; slowly rising power
            #[inline(always)]
            pub fn is_fsosc_1kck_65ms_cres_slowpwr(&self) -> bool {
                *self == SUT_CKSEL_A::FSOSC_1KCK_65MS_CRES_SLOWPWR
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz4_0mhz9_258ck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_258CK_4MS1
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz4_0mhz9_1kck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_1KCK_65MS
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz9_3mhz_258ck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_258CK_4MS1
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz9_3mhz_1kck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_1KCK_65MS
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_3mhz_8mhz_258ck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_258CK_4MS1
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_3mhz_8mhz_1kck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_1KCK_65MS
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 258 CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_8mhz_xx_258ck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_258CK_4MS1
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 1K CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_8mhz_xx_1kck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_1KCK_65MS
            }
            ///Ext. Clock; Start-up time: 6 CK + 4.1 ms
            #[inline(always)]
            pub fn is_extclk_6ck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTCLK_6CK_4MS1
            }
            ///Int. RC Osc.; Start-up time: 6 CK + 4.1 ms
            #[inline(always)]
            pub fn is_intrcosc_6ck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::INTRCOSC_6CK_4MS1
            }
            ///Int. 128kHz RC Osc.; Start-up time: 6 CK + 4 ms
            #[inline(always)]
            pub fn is_intrcosc_128khz_6ck_4ms(&self) -> bool {
                *self == SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_4MS
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4.1 ms
            #[inline(always)]
            pub fn is_extlofxtal_1kck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTLOFXTAL_1KCK_4MS1
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 32K CK + 4.1 ms
            #[inline(always)]
            pub fn is_extlofxtal_32kck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTLOFXTAL_32KCK_4MS1
            }
            ///Full Swing Oscillator; Start-up time: 258 CK + 65 ms; Ceramic res.; slowly rising power
            #[inline(always)]
            pub fn is_fsosc_258ck_65ms_cres_slowpwr(&self) -> bool {
                *self == SUT_CKSEL_A::FSOSC_258CK_65MS_CRES_SLOWPWR
            }
            ///Full Swing Oscillator; Start-up time: 16K CK + 0 ms; Crystal Osc.; BOD enabled
            #[inline(always)]
            pub fn is_fsosc_16kck_0ms_xosc_boden(&self) -> bool {
                *self == SUT_CKSEL_A::FSOSC_16KCK_0MS_XOSC_BODEN
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz4_0mhz9_258ck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_258CK_65MS
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 0 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz4_0mhz9_16kck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_0MS
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz9_3mhz_258ck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_258CK_65MS
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 0 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz9_3mhz_16kck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_0MS
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_3mhz_8mhz_258ck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_258CK_65MS
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 0 ms
            #[inline(always)]
            pub fn is_extxosc_3mhz_8mhz_16kck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_0MS
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 258 CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_8mhz_xx_258ck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_258CK_65MS
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 16K CK + 0 ms
            #[inline(always)]
            pub fn is_extxosc_8mhz_xx_16kck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_0MS
            }
            ///Ext. Clock; Start-up time: 6 CK + 65 ms
            #[inline(always)]
            pub fn is_extclk_6ck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTCLK_6CK_65MS
            }
            ///Int. RC Osc.; Start-up time: 6 CK + 65 ms
            #[inline(always)]
            pub fn is_intrcosc_6ck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::INTRCOSC_6CK_65MS
            }
            ///Int. 128kHz RC Osc.; Start-up time: 6 CK + 64 ms
            #[inline(always)]
            pub fn is_intrcosc_128khz_6ck_64ms(&self) -> bool {
                *self == SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_64MS
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 1K CK + 65 ms
            #[inline(always)]
            pub fn is_extlofxtal_1kck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTLOFXTAL_1KCK_65MS
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 32K CK + 65 ms
            #[inline(always)]
            pub fn is_extlofxtal_32kck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTLOFXTAL_32KCK_65MS
            }
            ///Full Swing Oscillator; Start-up time: 1K CK + 0 ms; Ceramic res.; BOD enable
            #[inline(always)]
            pub fn is_fsosc_1kck_0ms_cres_boden(&self) -> bool {
                *self == SUT_CKSEL_A::FSOSC_1KCK_0MS_CRES_BODEN
            }
            ///Full Swing Oscillator; Start-up time: 16K CK + 4.1 ms; Crystal Osc.; fast rising power
            #[inline(always)]
            pub fn is_fsosc_16kck_4ms1_xosc_fastpwr(&self) -> bool {
                *self == SUT_CKSEL_A::FSOSC_16KCK_4MS1_XOSC_FASTPWR
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 0 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz4_0mhz9_1kck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_1KCK_0MS
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz4_0mhz9_16kck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_4MS1
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 0 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz9_3mhz_1kck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_1KCK_0MS
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz9_3mhz_16kck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_4MS1
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 0 ms
            #[inline(always)]
            pub fn is_extxosc_3mhz_8mhz_1kck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_1KCK_0MS
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_3mhz_8mhz_16kck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_4MS1
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 1K CK + 0 ms
            #[inline(always)]
            pub fn is_extxosc_8mhz_xx_1kck_0ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_1KCK_0MS
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 16K CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_8mhz_xx_16kck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_4MS1
            }
            ///Full Swing Oscillator; Start-up time: 1K CK + 4.1 ms; Ceramic res.; fast rising power
            #[inline(always)]
            pub fn is_fsosc_1kck_4ms1_cres_fastpwr(&self) -> bool {
                *self == SUT_CKSEL_A::FSOSC_1KCK_4MS1_CRES_FASTPWR
            }
            ///Full Swing Oscillator; Start-up time: 16K CK + 65 ms; Crystal Osc.; slowly rising power
            #[inline(always)]
            pub fn is_fsosc_16kck_65ms_xosc_slowpwr(&self) -> bool {
                *self == SUT_CKSEL_A::FSOSC_16KCK_65MS_XOSC_SLOWPWR
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz4_0mhz9_1kck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_1KCK_4MS1
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz4_0mhz9_16kck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_65MS
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz9_3mhz_1kck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_1KCK_4MS1
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_0mhz9_3mhz_16kck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_65MS
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_3mhz_8mhz_1kck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_1KCK_4MS1
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_3mhz_8mhz_16kck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_65MS
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 1K CK + 4.1 ms
            #[inline(always)]
            pub fn is_extxosc_8mhz_xx_1kck_4ms1(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_1KCK_4MS1
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 16K CK + 65 ms
            #[inline(always)]
            pub fn is_extxosc_8mhz_xx_16kck_65ms(&self) -> bool {
                *self == SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_65MS
            }
        }
        ///Field `SUT_CKSEL` writer - Select Clock Source
        pub type SUT_CKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, SUT_CKSEL_A>;
        impl<'a, REG> SUT_CKSEL_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Ext. Clock; Start-up time: 6 CK + 0 ms
            #[inline(always)]
            pub fn extclk_6ck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTCLK_6CK_0MS)
            }
            ///Int. RC Osc.; Start-up time: 6 CK + 0 ms
            #[inline(always)]
            pub fn intrcosc_6ck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::INTRCOSC_6CK_0MS)
            }
            ///Int. 128kHz RC Osc.; Start-up time: 6 CK + 0 ms
            #[inline(always)]
            pub fn intrcosc_128khz_6ck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_0MS)
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 1K CK + 0 ms
            #[inline(always)]
            pub fn extlofxtal_1kck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTLOFXTAL_1KCK_0MS)
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 32K CK + 0 ms
            #[inline(always)]
            pub fn extlofxtal_32kck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTLOFXTAL_32KCK_0MS)
            }
            ///Full Swing Oscillator; Start-up time: 258 CK + 4.1 ms; Ceramic res.; fast rising power
            #[inline(always)]
            pub fn fsosc_258ck_4ms1_cres_fastpwr(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::FSOSC_258CK_4MS1_CRES_FASTPWR)
            }
            ///Full Swing Oscillator; Start-up time: 1K CK + 65 ms; Ceramic res.; slowly rising power
            #[inline(always)]
            pub fn fsosc_1kck_65ms_cres_slowpwr(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::FSOSC_1KCK_65MS_CRES_SLOWPWR)
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_0mhz4_0mhz9_258ck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_258CK_4MS1)
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 65 ms
            #[inline(always)]
            pub fn extxosc_0mhz4_0mhz9_1kck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_1KCK_65MS)
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_0mhz9_3mhz_258ck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_258CK_4MS1)
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 65 ms
            #[inline(always)]
            pub fn extxosc_0mhz9_3mhz_1kck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_1KCK_65MS)
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_3mhz_8mhz_258ck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_258CK_4MS1)
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 65 ms
            #[inline(always)]
            pub fn extxosc_3mhz_8mhz_1kck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_1KCK_65MS)
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 258 CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_8mhz_xx_258ck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_258CK_4MS1)
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 1K CK + 65 ms
            #[inline(always)]
            pub fn extxosc_8mhz_xx_1kck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_1KCK_65MS)
            }
            ///Ext. Clock; Start-up time: 6 CK + 4.1 ms
            #[inline(always)]
            pub fn extclk_6ck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTCLK_6CK_4MS1)
            }
            ///Int. RC Osc.; Start-up time: 6 CK + 4.1 ms
            #[inline(always)]
            pub fn intrcosc_6ck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::INTRCOSC_6CK_4MS1)
            }
            ///Int. 128kHz RC Osc.; Start-up time: 6 CK + 4 ms
            #[inline(always)]
            pub fn intrcosc_128khz_6ck_4ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_4MS)
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4.1 ms
            #[inline(always)]
            pub fn extlofxtal_1kck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTLOFXTAL_1KCK_4MS1)
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 32K CK + 4.1 ms
            #[inline(always)]
            pub fn extlofxtal_32kck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTLOFXTAL_32KCK_4MS1)
            }
            ///Full Swing Oscillator; Start-up time: 258 CK + 65 ms; Ceramic res.; slowly rising power
            #[inline(always)]
            pub fn fsosc_258ck_65ms_cres_slowpwr(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::FSOSC_258CK_65MS_CRES_SLOWPWR)
            }
            ///Full Swing Oscillator; Start-up time: 16K CK + 0 ms; Crystal Osc.; BOD enabled
            #[inline(always)]
            pub fn fsosc_16kck_0ms_xosc_boden(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::FSOSC_16KCK_0MS_XOSC_BODEN)
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 65 ms
            #[inline(always)]
            pub fn extxosc_0mhz4_0mhz9_258ck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_258CK_65MS)
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 0 ms
            #[inline(always)]
            pub fn extxosc_0mhz4_0mhz9_16kck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_0MS)
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 65 ms
            #[inline(always)]
            pub fn extxosc_0mhz9_3mhz_258ck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_258CK_65MS)
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 0 ms
            #[inline(always)]
            pub fn extxosc_0mhz9_3mhz_16kck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_0MS)
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 65 ms
            #[inline(always)]
            pub fn extxosc_3mhz_8mhz_258ck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_258CK_65MS)
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 0 ms
            #[inline(always)]
            pub fn extxosc_3mhz_8mhz_16kck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_0MS)
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 258 CK + 65 ms
            #[inline(always)]
            pub fn extxosc_8mhz_xx_258ck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_258CK_65MS)
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 16K CK + 0 ms
            #[inline(always)]
            pub fn extxosc_8mhz_xx_16kck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_0MS)
            }
            ///Ext. Clock; Start-up time: 6 CK + 65 ms
            #[inline(always)]
            pub fn extclk_6ck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTCLK_6CK_65MS)
            }
            ///Int. RC Osc.; Start-up time: 6 CK + 65 ms
            #[inline(always)]
            pub fn intrcosc_6ck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::INTRCOSC_6CK_65MS)
            }
            ///Int. 128kHz RC Osc.; Start-up time: 6 CK + 64 ms
            #[inline(always)]
            pub fn intrcosc_128khz_6ck_64ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::INTRCOSC_128KHZ_6CK_64MS)
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 1K CK + 65 ms
            #[inline(always)]
            pub fn extlofxtal_1kck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTLOFXTAL_1KCK_65MS)
            }
            ///Ext. Low-Freq. Crystal; Start-up time: 32K CK + 65 ms
            #[inline(always)]
            pub fn extlofxtal_32kck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTLOFXTAL_32KCK_65MS)
            }
            ///Full Swing Oscillator; Start-up time: 1K CK + 0 ms; Ceramic res.; BOD enable
            #[inline(always)]
            pub fn fsosc_1kck_0ms_cres_boden(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::FSOSC_1KCK_0MS_CRES_BODEN)
            }
            ///Full Swing Oscillator; Start-up time: 16K CK + 4.1 ms; Crystal Osc.; fast rising power
            #[inline(always)]
            pub fn fsosc_16kck_4ms1_xosc_fastpwr(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::FSOSC_16KCK_4MS1_XOSC_FASTPWR)
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 0 ms
            #[inline(always)]
            pub fn extxosc_0mhz4_0mhz9_1kck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_1KCK_0MS)
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_0mhz4_0mhz9_16kck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_4MS1)
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 0 ms
            #[inline(always)]
            pub fn extxosc_0mhz9_3mhz_1kck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_1KCK_0MS)
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_0mhz9_3mhz_16kck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_4MS1)
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 0 ms
            #[inline(always)]
            pub fn extxosc_3mhz_8mhz_1kck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_1KCK_0MS)
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_3mhz_8mhz_16kck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_4MS1)
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 1K CK + 0 ms
            #[inline(always)]
            pub fn extxosc_8mhz_xx_1kck_0ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_1KCK_0MS)
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 16K CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_8mhz_xx_16kck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_4MS1)
            }
            ///Full Swing Oscillator; Start-up time: 1K CK + 4.1 ms; Ceramic res.; fast rising power
            #[inline(always)]
            pub fn fsosc_1kck_4ms1_cres_fastpwr(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::FSOSC_1KCK_4MS1_CRES_FASTPWR)
            }
            ///Full Swing Oscillator; Start-up time: 16K CK + 65 ms; Crystal Osc.; slowly rising power
            #[inline(always)]
            pub fn fsosc_16kck_65ms_xosc_slowpwr(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::FSOSC_16KCK_65MS_XOSC_SLOWPWR)
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_0mhz4_0mhz9_1kck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_1KCK_4MS1)
            }
            ///Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 65 ms
            #[inline(always)]
            pub fn extxosc_0mhz4_0mhz9_16kck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ4_0MHZ9_16KCK_65MS)
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_0mhz9_3mhz_1kck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_1KCK_4MS1)
            }
            ///Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 65 ms
            #[inline(always)]
            pub fn extxosc_0mhz9_3mhz_16kck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_0MHZ9_3MHZ_16KCK_65MS)
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_3mhz_8mhz_1kck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_1KCK_4MS1)
            }
            ///Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 65 ms
            #[inline(always)]
            pub fn extxosc_3mhz_8mhz_16kck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_3MHZ_8MHZ_16KCK_65MS)
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 1K CK + 4.1 ms
            #[inline(always)]
            pub fn extxosc_8mhz_xx_1kck_4ms1(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_1KCK_4MS1)
            }
            ///Ext. Crystal Osc. 8.0- MHz; Start-up time: 16K CK + 65 ms
            #[inline(always)]
            pub fn extxosc_8mhz_xx_16kck_65ms(self) -> &'a mut crate::W<REG> {
                self.variant(SUT_CKSEL_A::EXTXOSC_8MHZ_XX_16KCK_65MS)
            }
        }
        ///Field `CKOUT` reader - Clock output on PORTE7
        pub type CKOUT_R = crate::BitReader;
        ///Field `CKOUT` writer - Clock output on PORTE7
        pub type CKOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `CKDIV8` reader - Divide clock by 8 internally
        pub type CKDIV8_R = crate::BitReader;
        ///Field `CKDIV8` writer - Divide clock by 8 internally
        pub type CKDIV8_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:5 - Select Clock Source
            #[inline(always)]
            pub fn sut_cksel(&self) -> SUT_CKSEL_R {
                SUT_CKSEL_R::new(self.bits & 0x3f)
            }
            ///Bit 6 - Clock output on PORTE7
            #[inline(always)]
            pub fn ckout(&self) -> CKOUT_R {
                CKOUT_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Divide clock by 8 internally
            #[inline(always)]
            pub fn ckdiv8(&self) -> CKDIV8_R {
                CKDIV8_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:5 - Select Clock Source
            #[inline(always)]
            pub fn sut_cksel(&mut self) -> SUT_CKSEL_W<'_, LOW_SPEC> {
                SUT_CKSEL_W::new(self, 0)
            }
            ///Bit 6 - Clock output on PORTE7
            #[inline(always)]
            pub fn ckout(&mut self) -> CKOUT_W<'_, LOW_SPEC> {
                CKOUT_W::new(self, 6)
            }
            ///Bit 7 - Divide clock by 8 internally
            #[inline(always)]
            pub fn ckdiv8(&mut self) -> CKDIV8_W<'_, LOW_SPEC> {
                CKDIV8_W::new(self, 7)
            }
        }
        /**No Description.

You can [`read`](crate::Reg::read) this register and get [`low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct LOW_SPEC;
        impl crate::RegisterSpec for LOW_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`low::R`](R) reader structure
        impl crate::Readable for LOW_SPEC {}
        ///`write(|w| ..)` method takes [`low::W`](W) writer structure
        impl crate::Writable for LOW_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets LOW to value 0
        impl crate::Resettable for LOW_SPEC {}
    }
}
///JTAG Interface
pub type JTAG = crate::Periph<jtag::RegisterBlock, 0x51>;
impl core::fmt::Debug for JTAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAG").finish()
    }
}
///JTAG Interface
pub mod jtag {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        ocdr: OCDR,
        _reserved1: [u8; 0x02],
        mcusr: MCUSR,
        mcucr: MCUCR,
    }
    impl RegisterBlock {
        ///0x00 - On-Chip Debug Related Register in I/O Memory
        #[inline(always)]
        pub const fn ocdr(&self) -> &OCDR {
            &self.ocdr
        }
        ///0x03 - MCU Status Register
        #[inline(always)]
        pub const fn mcusr(&self) -> &MCUSR {
            &self.mcusr
        }
        ///0x04 - MCU Control Register
        #[inline(always)]
        pub const fn mcucr(&self) -> &MCUCR {
            &self.mcucr
        }
    }
    /**MCUCR (rw) register accessor: MCU Control Register

You can [`read`](crate::Reg::read) this register and get [`mcucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mcucr`] module*/
    pub type MCUCR = crate::Reg<mcucr::MCUCR_SPEC>;
    ///MCU Control Register
    pub mod mcucr {
        ///Register `MCUCR` reader
        pub type R = crate::R<MCUCR_SPEC>;
        ///Register `MCUCR` writer
        pub type W = crate::W<MCUCR_SPEC>;
        ///Field `JTD` reader - JTAG Interface Disable
        pub type JTD_R = crate::BitReader;
        ///Field `JTD` writer - JTAG Interface Disable
        pub type JTD_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 7 - JTAG Interface Disable
            #[inline(always)]
            pub fn jtd(&self) -> JTD_R {
                JTD_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 7 - JTAG Interface Disable
            #[inline(always)]
            pub fn jtd(&mut self) -> JTD_W<'_, MCUCR_SPEC> {
                JTD_W::new(self, 7)
            }
        }
        /**MCU Control Register

You can [`read`](crate::Reg::read) this register and get [`mcucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct MCUCR_SPEC;
        impl crate::RegisterSpec for MCUCR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`mcucr::R`](R) reader structure
        impl crate::Readable for MCUCR_SPEC {}
        ///`write(|w| ..)` method takes [`mcucr::W`](W) writer structure
        impl crate::Writable for MCUCR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets MCUCR to value 0
        impl crate::Resettable for MCUCR_SPEC {}
    }
    /**MCUSR (rw) register accessor: MCU Status Register

You can [`read`](crate::Reg::read) this register and get [`mcusr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcusr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mcusr`] module*/
    pub type MCUSR = crate::Reg<mcusr::MCUSR_SPEC>;
    ///MCU Status Register
    pub mod mcusr {
        ///Register `MCUSR` reader
        pub type R = crate::R<MCUSR_SPEC>;
        ///Register `MCUSR` writer
        pub type W = crate::W<MCUSR_SPEC>;
        ///Field `JTRF` reader - JTAG Reset Flag
        pub type JTRF_R = crate::BitReader;
        ///Field `JTRF` writer - JTAG Reset Flag
        pub type JTRF_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 4 - JTAG Reset Flag
            #[inline(always)]
            pub fn jtrf(&self) -> JTRF_R {
                JTRF_R::new(((self.bits >> 4) & 1) != 0)
            }
        }
        impl W {
            ///Bit 4 - JTAG Reset Flag
            #[inline(always)]
            pub fn jtrf(&mut self) -> JTRF_W<'_, MCUSR_SPEC> {
                JTRF_W::new(self, 4)
            }
        }
        /**MCU Status Register

You can [`read`](crate::Reg::read) this register and get [`mcusr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcusr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct MCUSR_SPEC;
        impl crate::RegisterSpec for MCUSR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`mcusr::R`](R) reader structure
        impl crate::Readable for MCUSR_SPEC {}
        ///`write(|w| ..)` method takes [`mcusr::W`](W) writer structure
        impl crate::Writable for MCUSR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets MCUSR to value 0
        impl crate::Resettable for MCUSR_SPEC {}
    }
    /**OCDR (rw) register accessor: On-Chip Debug Related Register in I/O Memory

You can [`read`](crate::Reg::read) this register and get [`ocdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocdr`] module*/
    pub type OCDR = crate::Reg<ocdr::OCDR_SPEC>;
    ///On-Chip Debug Related Register in I/O Memory
    pub mod ocdr {
        ///Register `OCDR` reader
        pub type R = crate::R<OCDR_SPEC>;
        ///Register `OCDR` writer
        pub type W = crate::W<OCDR_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**On-Chip Debug Related Register in I/O Memory

You can [`read`](crate::Reg::read) this register and get [`ocdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCDR_SPEC;
        impl crate::RegisterSpec for OCDR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ocdr::R`](R) reader structure
        impl crate::Readable for OCDR_SPEC {}
        ///`write(|w| ..)` method takes [`ocdr::W`](W) writer structure
        impl crate::Writable for OCDR_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCDR to value 0
        impl crate::Resettable for OCDR_SPEC {}
    }
}
///Lockbits
pub type LOCKBIT = crate::Periph<lockbit::RegisterBlock, 0>;
impl core::fmt::Debug for LOCKBIT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCKBIT").finish()
    }
}
///Lockbits
pub mod lockbit {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        lockbit: LOCKBIT,
    }
    impl RegisterBlock {
        ///0x00 - No Description.
        #[inline(always)]
        pub const fn lockbit(&self) -> &LOCKBIT {
            &self.lockbit
        }
    }
    /**LOCKBIT (rw) register accessor: No Description.

You can [`read`](crate::Reg::read) this register and get [`lockbit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockbit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lockbit`] module*/
    pub type LOCKBIT = crate::Reg<lockbit::LOCKBIT_SPEC>;
    ///No Description.
    pub mod lockbit {
        ///Register `LOCKBIT` reader
        pub type R = crate::R<LOCKBIT_SPEC>;
        ///Register `LOCKBIT` writer
        pub type W = crate::W<LOCKBIT_SPEC>;
        /**Memory Lock

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum LB_A {
            ///0: Further programming and verification disabled
            PROG_VER_DISABLED = 0,
            ///2: Further programming disabled
            PROG_DISABLED = 2,
            ///3: No memory lock features enabled
            NO_LOCK = 3,
        }
        impl From<LB_A> for u8 {
            #[inline(always)]
            fn from(variant: LB_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for LB_A {
            type Ux = u8;
        }
        impl crate::IsEnum for LB_A {}
        ///Field `LB` reader - Memory Lock
        pub type LB_R = crate::FieldReader<LB_A>;
        impl LB_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<LB_A> {
                match self.bits {
                    0 => Some(LB_A::PROG_VER_DISABLED),
                    2 => Some(LB_A::PROG_DISABLED),
                    3 => Some(LB_A::NO_LOCK),
                    _ => None,
                }
            }
            ///Further programming and verification disabled
            #[inline(always)]
            pub fn is_prog_ver_disabled(&self) -> bool {
                *self == LB_A::PROG_VER_DISABLED
            }
            ///Further programming disabled
            #[inline(always)]
            pub fn is_prog_disabled(&self) -> bool {
                *self == LB_A::PROG_DISABLED
            }
            ///No memory lock features enabled
            #[inline(always)]
            pub fn is_no_lock(&self) -> bool {
                *self == LB_A::NO_LOCK
            }
        }
        ///Field `LB` writer - Memory Lock
        pub type LB_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LB_A>;
        impl<'a, REG> LB_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Further programming and verification disabled
            #[inline(always)]
            pub fn prog_ver_disabled(self) -> &'a mut crate::W<REG> {
                self.variant(LB_A::PROG_VER_DISABLED)
            }
            ///Further programming disabled
            #[inline(always)]
            pub fn prog_disabled(self) -> &'a mut crate::W<REG> {
                self.variant(LB_A::PROG_DISABLED)
            }
            ///No memory lock features enabled
            #[inline(always)]
            pub fn no_lock(self) -> &'a mut crate::W<REG> {
                self.variant(LB_A::NO_LOCK)
            }
        }
        /**Boot Loader Protection Mode

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum BLB0_A {
            ///0: LPM and SPM prohibited in Application Section
            LPM_SPM_DISABLE = 0,
            ///1: LPM prohibited in Application Section
            LPM_DISABLE = 1,
            ///2: SPM prohibited in Application Section
            SPM_DISABLE = 2,
            ///3: No lock on SPM and LPM in Application Section
            NO_LOCK = 3,
        }
        impl From<BLB0_A> for u8 {
            #[inline(always)]
            fn from(variant: BLB0_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for BLB0_A {
            type Ux = u8;
        }
        impl crate::IsEnum for BLB0_A {}
        ///Field `BLB0` reader - Boot Loader Protection Mode
        pub type BLB0_R = crate::FieldReader<BLB0_A>;
        impl BLB0_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> BLB0_A {
                match self.bits {
                    0 => BLB0_A::LPM_SPM_DISABLE,
                    1 => BLB0_A::LPM_DISABLE,
                    2 => BLB0_A::SPM_DISABLE,
                    3 => BLB0_A::NO_LOCK,
                    _ => unreachable!(),
                }
            }
            ///LPM and SPM prohibited in Application Section
            #[inline(always)]
            pub fn is_lpm_spm_disable(&self) -> bool {
                *self == BLB0_A::LPM_SPM_DISABLE
            }
            ///LPM prohibited in Application Section
            #[inline(always)]
            pub fn is_lpm_disable(&self) -> bool {
                *self == BLB0_A::LPM_DISABLE
            }
            ///SPM prohibited in Application Section
            #[inline(always)]
            pub fn is_spm_disable(&self) -> bool {
                *self == BLB0_A::SPM_DISABLE
            }
            ///No lock on SPM and LPM in Application Section
            #[inline(always)]
            pub fn is_no_lock(&self) -> bool {
                *self == BLB0_A::NO_LOCK
            }
        }
        ///Field `BLB0` writer - Boot Loader Protection Mode
        pub type BLB0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BLB0_A, crate::Safe>;
        impl<'a, REG> BLB0_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///LPM and SPM prohibited in Application Section
            #[inline(always)]
            pub fn lpm_spm_disable(self) -> &'a mut crate::W<REG> {
                self.variant(BLB0_A::LPM_SPM_DISABLE)
            }
            ///LPM prohibited in Application Section
            #[inline(always)]
            pub fn lpm_disable(self) -> &'a mut crate::W<REG> {
                self.variant(BLB0_A::LPM_DISABLE)
            }
            ///SPM prohibited in Application Section
            #[inline(always)]
            pub fn spm_disable(self) -> &'a mut crate::W<REG> {
                self.variant(BLB0_A::SPM_DISABLE)
            }
            ///No lock on SPM and LPM in Application Section
            #[inline(always)]
            pub fn no_lock(self) -> &'a mut crate::W<REG> {
                self.variant(BLB0_A::NO_LOCK)
            }
        }
        /**Boot Loader Protection Mode

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum BLB1_A {
            ///0: LPM and SPM prohibited in Boot Section
            LPM_SPM_DISABLE = 0,
            ///1: LPM prohibited in Boot Section
            LPM_DISABLE = 1,
            ///2: SPM prohibited in Boot Section
            SPM_DISABLE = 2,
            ///3: No lock on SPM and LPM in Boot Section
            NO_LOCK = 3,
        }
        impl From<BLB1_A> for u8 {
            #[inline(always)]
            fn from(variant: BLB1_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for BLB1_A {
            type Ux = u8;
        }
        impl crate::IsEnum for BLB1_A {}
        ///Field `BLB1` reader - Boot Loader Protection Mode
        pub type BLB1_R = crate::FieldReader<BLB1_A>;
        impl BLB1_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> BLB1_A {
                match self.bits {
                    0 => BLB1_A::LPM_SPM_DISABLE,
                    1 => BLB1_A::LPM_DISABLE,
                    2 => BLB1_A::SPM_DISABLE,
                    3 => BLB1_A::NO_LOCK,
                    _ => unreachable!(),
                }
            }
            ///LPM and SPM prohibited in Boot Section
            #[inline(always)]
            pub fn is_lpm_spm_disable(&self) -> bool {
                *self == BLB1_A::LPM_SPM_DISABLE
            }
            ///LPM prohibited in Boot Section
            #[inline(always)]
            pub fn is_lpm_disable(&self) -> bool {
                *self == BLB1_A::LPM_DISABLE
            }
            ///SPM prohibited in Boot Section
            #[inline(always)]
            pub fn is_spm_disable(&self) -> bool {
                *self == BLB1_A::SPM_DISABLE
            }
            ///No lock on SPM and LPM in Boot Section
            #[inline(always)]
            pub fn is_no_lock(&self) -> bool {
                *self == BLB1_A::NO_LOCK
            }
        }
        ///Field `BLB1` writer - Boot Loader Protection Mode
        pub type BLB1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BLB1_A, crate::Safe>;
        impl<'a, REG> BLB1_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///LPM and SPM prohibited in Boot Section
            #[inline(always)]
            pub fn lpm_spm_disable(self) -> &'a mut crate::W<REG> {
                self.variant(BLB1_A::LPM_SPM_DISABLE)
            }
            ///LPM prohibited in Boot Section
            #[inline(always)]
            pub fn lpm_disable(self) -> &'a mut crate::W<REG> {
                self.variant(BLB1_A::LPM_DISABLE)
            }
            ///SPM prohibited in Boot Section
            #[inline(always)]
            pub fn spm_disable(self) -> &'a mut crate::W<REG> {
                self.variant(BLB1_A::SPM_DISABLE)
            }
            ///No lock on SPM and LPM in Boot Section
            #[inline(always)]
            pub fn no_lock(self) -> &'a mut crate::W<REG> {
                self.variant(BLB1_A::NO_LOCK)
            }
        }
        impl R {
            ///Bits 0:1 - Memory Lock
            #[inline(always)]
            pub fn lb(&self) -> LB_R {
                LB_R::new(self.bits & 3)
            }
            ///Bits 2:3 - Boot Loader Protection Mode
            #[inline(always)]
            pub fn blb0(&self) -> BLB0_R {
                BLB0_R::new((self.bits >> 2) & 3)
            }
            ///Bits 4:5 - Boot Loader Protection Mode
            #[inline(always)]
            pub fn blb1(&self) -> BLB1_R {
                BLB1_R::new((self.bits >> 4) & 3)
            }
        }
        impl W {
            ///Bits 0:1 - Memory Lock
            #[inline(always)]
            pub fn lb(&mut self) -> LB_W<'_, LOCKBIT_SPEC> {
                LB_W::new(self, 0)
            }
            ///Bits 2:3 - Boot Loader Protection Mode
            #[inline(always)]
            pub fn blb0(&mut self) -> BLB0_W<'_, LOCKBIT_SPEC> {
                BLB0_W::new(self, 2)
            }
            ///Bits 4:5 - Boot Loader Protection Mode
            #[inline(always)]
            pub fn blb1(&mut self) -> BLB1_W<'_, LOCKBIT_SPEC> {
                BLB1_W::new(self, 4)
            }
        }
        /**No Description.

You can [`read`](crate::Reg::read) this register and get [`lockbit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockbit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct LOCKBIT_SPEC;
        impl crate::RegisterSpec for LOCKBIT_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`lockbit::R`](R) reader structure
        impl crate::Readable for LOCKBIT_SPEC {}
        ///`write(|w| ..)` method takes [`lockbit::W`](W) writer structure
        impl crate::Writable for LOCKBIT_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets LOCKBIT to value 0
        impl crate::Resettable for LOCKBIT_SPEC {}
    }
}
///I/O Port
pub type PORTA = crate::Periph<porta::RegisterBlock, 0x20>;
impl core::fmt::Debug for PORTA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTA").finish()
    }
}
///I/O Port
pub mod porta {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pina: PINA,
        ddra: DDRA,
        porta: PORTA,
    }
    impl RegisterBlock {
        ///0x00 - Port A Input Pins
        #[inline(always)]
        pub const fn pina(&self) -> &PINA {
            &self.pina
        }
        ///0x01 - Port A Data Direction Register
        #[inline(always)]
        pub const fn ddra(&self) -> &DDRA {
            &self.ddra
        }
        ///0x02 - Port A Data Register
        #[inline(always)]
        pub const fn porta(&self) -> &PORTA {
            &self.porta
        }
    }
    /**DDRA (rw) register accessor: Port A Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddra`] module*/
    pub type DDRA = crate::Reg<ddra::DDRA_SPEC>;
    ///Port A Data Direction Register
    pub mod ddra {
        ///Register `DDRA` reader
        pub type R = crate::R<DDRA_SPEC>;
        ///Register `DDRA` writer
        pub type W = crate::W<DDRA_SPEC>;
        ///Field `PA0` reader - Pin A0
        pub type PA0_R = crate::BitReader;
        ///Field `PA0` writer - Pin A0
        pub type PA0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA1` reader - Pin A1
        pub type PA1_R = crate::BitReader;
        ///Field `PA1` writer - Pin A1
        pub type PA1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA2` reader - Pin A2
        pub type PA2_R = crate::BitReader;
        ///Field `PA2` writer - Pin A2
        pub type PA2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA3` reader - Pin A3
        pub type PA3_R = crate::BitReader;
        ///Field `PA3` writer - Pin A3
        pub type PA3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA4` reader - Pin A4
        pub type PA4_R = crate::BitReader;
        ///Field `PA4` writer - Pin A4
        pub type PA4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA5` reader - Pin A5
        pub type PA5_R = crate::BitReader;
        ///Field `PA5` writer - Pin A5
        pub type PA5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA6` reader - Pin A6
        pub type PA6_R = crate::BitReader;
        ///Field `PA6` writer - Pin A6
        pub type PA6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA7` reader - Pin A7
        pub type PA7_R = crate::BitReader;
        ///Field `PA7` writer - Pin A7
        pub type PA7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin A0
            #[inline(always)]
            pub fn pa0(&self) -> PA0_R {
                PA0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin A1
            #[inline(always)]
            pub fn pa1(&self) -> PA1_R {
                PA1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin A2
            #[inline(always)]
            pub fn pa2(&self) -> PA2_R {
                PA2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin A3
            #[inline(always)]
            pub fn pa3(&self) -> PA3_R {
                PA3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin A4
            #[inline(always)]
            pub fn pa4(&self) -> PA4_R {
                PA4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin A5
            #[inline(always)]
            pub fn pa5(&self) -> PA5_R {
                PA5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin A6
            #[inline(always)]
            pub fn pa6(&self) -> PA6_R {
                PA6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin A7
            #[inline(always)]
            pub fn pa7(&self) -> PA7_R {
                PA7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin A0
            #[inline(always)]
            pub fn pa0(&mut self) -> PA0_W<'_, DDRA_SPEC> {
                PA0_W::new(self, 0)
            }
            ///Bit 1 - Pin A1
            #[inline(always)]
            pub fn pa1(&mut self) -> PA1_W<'_, DDRA_SPEC> {
                PA1_W::new(self, 1)
            }
            ///Bit 2 - Pin A2
            #[inline(always)]
            pub fn pa2(&mut self) -> PA2_W<'_, DDRA_SPEC> {
                PA2_W::new(self, 2)
            }
            ///Bit 3 - Pin A3
            #[inline(always)]
            pub fn pa3(&mut self) -> PA3_W<'_, DDRA_SPEC> {
                PA3_W::new(self, 3)
            }
            ///Bit 4 - Pin A4
            #[inline(always)]
            pub fn pa4(&mut self) -> PA4_W<'_, DDRA_SPEC> {
                PA4_W::new(self, 4)
            }
            ///Bit 5 - Pin A5
            #[inline(always)]
            pub fn pa5(&mut self) -> PA5_W<'_, DDRA_SPEC> {
                PA5_W::new(self, 5)
            }
            ///Bit 6 - Pin A6
            #[inline(always)]
            pub fn pa6(&mut self) -> PA6_W<'_, DDRA_SPEC> {
                PA6_W::new(self, 6)
            }
            ///Bit 7 - Pin A7
            #[inline(always)]
            pub fn pa7(&mut self) -> PA7_W<'_, DDRA_SPEC> {
                PA7_W::new(self, 7)
            }
        }
        /**Port A Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRA_SPEC;
        impl crate::RegisterSpec for DDRA_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddra::R`](R) reader structure
        impl crate::Readable for DDRA_SPEC {}
        ///`write(|w| ..)` method takes [`ddra::W`](W) writer structure
        impl crate::Writable for DDRA_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRA to value 0
        impl crate::Resettable for DDRA_SPEC {}
    }
    /**PINA (rw) register accessor: Port A Input Pins

You can [`read`](crate::Reg::read) this register and get [`pina::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pina::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pina`] module*/
    pub type PINA = crate::Reg<pina::PINA_SPEC>;
    ///Port A Input Pins
    pub mod pina {
        ///Register `PINA` reader
        pub type R = crate::R<PINA_SPEC>;
        ///Register `PINA` writer
        pub type W = crate::W<PINA_SPEC>;
        ///Field `PA0` reader - Pin A0
        pub type PA0_R = crate::BitReader;
        ///Field `PA0` writer - Pin A0
        pub type PA0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA1` reader - Pin A1
        pub type PA1_R = crate::BitReader;
        ///Field `PA1` writer - Pin A1
        pub type PA1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA2` reader - Pin A2
        pub type PA2_R = crate::BitReader;
        ///Field `PA2` writer - Pin A2
        pub type PA2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA3` reader - Pin A3
        pub type PA3_R = crate::BitReader;
        ///Field `PA3` writer - Pin A3
        pub type PA3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA4` reader - Pin A4
        pub type PA4_R = crate::BitReader;
        ///Field `PA4` writer - Pin A4
        pub type PA4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA5` reader - Pin A5
        pub type PA5_R = crate::BitReader;
        ///Field `PA5` writer - Pin A5
        pub type PA5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA6` reader - Pin A6
        pub type PA6_R = crate::BitReader;
        ///Field `PA6` writer - Pin A6
        pub type PA6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA7` reader - Pin A7
        pub type PA7_R = crate::BitReader;
        ///Field `PA7` writer - Pin A7
        pub type PA7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin A0
            #[inline(always)]
            pub fn pa0(&self) -> PA0_R {
                PA0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin A1
            #[inline(always)]
            pub fn pa1(&self) -> PA1_R {
                PA1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin A2
            #[inline(always)]
            pub fn pa2(&self) -> PA2_R {
                PA2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin A3
            #[inline(always)]
            pub fn pa3(&self) -> PA3_R {
                PA3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin A4
            #[inline(always)]
            pub fn pa4(&self) -> PA4_R {
                PA4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin A5
            #[inline(always)]
            pub fn pa5(&self) -> PA5_R {
                PA5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin A6
            #[inline(always)]
            pub fn pa6(&self) -> PA6_R {
                PA6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin A7
            #[inline(always)]
            pub fn pa7(&self) -> PA7_R {
                PA7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin A0
            #[inline(always)]
            pub fn pa0(&mut self) -> PA0_W<'_, PINA_SPEC> {
                PA0_W::new(self, 0)
            }
            ///Bit 1 - Pin A1
            #[inline(always)]
            pub fn pa1(&mut self) -> PA1_W<'_, PINA_SPEC> {
                PA1_W::new(self, 1)
            }
            ///Bit 2 - Pin A2
            #[inline(always)]
            pub fn pa2(&mut self) -> PA2_W<'_, PINA_SPEC> {
                PA2_W::new(self, 2)
            }
            ///Bit 3 - Pin A3
            #[inline(always)]
            pub fn pa3(&mut self) -> PA3_W<'_, PINA_SPEC> {
                PA3_W::new(self, 3)
            }
            ///Bit 4 - Pin A4
            #[inline(always)]
            pub fn pa4(&mut self) -> PA4_W<'_, PINA_SPEC> {
                PA4_W::new(self, 4)
            }
            ///Bit 5 - Pin A5
            #[inline(always)]
            pub fn pa5(&mut self) -> PA5_W<'_, PINA_SPEC> {
                PA5_W::new(self, 5)
            }
            ///Bit 6 - Pin A6
            #[inline(always)]
            pub fn pa6(&mut self) -> PA6_W<'_, PINA_SPEC> {
                PA6_W::new(self, 6)
            }
            ///Bit 7 - Pin A7
            #[inline(always)]
            pub fn pa7(&mut self) -> PA7_W<'_, PINA_SPEC> {
                PA7_W::new(self, 7)
            }
        }
        /**Port A Input Pins

You can [`read`](crate::Reg::read) this register and get [`pina::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pina::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PINA_SPEC;
        impl crate::RegisterSpec for PINA_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pina::R`](R) reader structure
        impl crate::Readable for PINA_SPEC {}
        ///`write(|w| ..)` method takes [`pina::W`](W) writer structure
        impl crate::Writable for PINA_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PINA to value 0
        impl crate::Resettable for PINA_SPEC {}
    }
    /**PORTA (rw) register accessor: Port A Data Register

You can [`read`](crate::Reg::read) this register and get [`porta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@porta`] module*/
    pub type PORTA = crate::Reg<porta::PORTA_SPEC>;
    ///Port A Data Register
    pub mod porta {
        ///Register `PORTA` reader
        pub type R = crate::R<PORTA_SPEC>;
        ///Register `PORTA` writer
        pub type W = crate::W<PORTA_SPEC>;
        ///Field `PA0` reader - Pin A0
        pub type PA0_R = crate::BitReader;
        ///Field `PA0` writer - Pin A0
        pub type PA0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA1` reader - Pin A1
        pub type PA1_R = crate::BitReader;
        ///Field `PA1` writer - Pin A1
        pub type PA1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA2` reader - Pin A2
        pub type PA2_R = crate::BitReader;
        ///Field `PA2` writer - Pin A2
        pub type PA2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA3` reader - Pin A3
        pub type PA3_R = crate::BitReader;
        ///Field `PA3` writer - Pin A3
        pub type PA3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA4` reader - Pin A4
        pub type PA4_R = crate::BitReader;
        ///Field `PA4` writer - Pin A4
        pub type PA4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA5` reader - Pin A5
        pub type PA5_R = crate::BitReader;
        ///Field `PA5` writer - Pin A5
        pub type PA5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA6` reader - Pin A6
        pub type PA6_R = crate::BitReader;
        ///Field `PA6` writer - Pin A6
        pub type PA6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PA7` reader - Pin A7
        pub type PA7_R = crate::BitReader;
        ///Field `PA7` writer - Pin A7
        pub type PA7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin A0
            #[inline(always)]
            pub fn pa0(&self) -> PA0_R {
                PA0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin A1
            #[inline(always)]
            pub fn pa1(&self) -> PA1_R {
                PA1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin A2
            #[inline(always)]
            pub fn pa2(&self) -> PA2_R {
                PA2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin A3
            #[inline(always)]
            pub fn pa3(&self) -> PA3_R {
                PA3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin A4
            #[inline(always)]
            pub fn pa4(&self) -> PA4_R {
                PA4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin A5
            #[inline(always)]
            pub fn pa5(&self) -> PA5_R {
                PA5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin A6
            #[inline(always)]
            pub fn pa6(&self) -> PA6_R {
                PA6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin A7
            #[inline(always)]
            pub fn pa7(&self) -> PA7_R {
                PA7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin A0
            #[inline(always)]
            pub fn pa0(&mut self) -> PA0_W<'_, PORTA_SPEC> {
                PA0_W::new(self, 0)
            }
            ///Bit 1 - Pin A1
            #[inline(always)]
            pub fn pa1(&mut self) -> PA1_W<'_, PORTA_SPEC> {
                PA1_W::new(self, 1)
            }
            ///Bit 2 - Pin A2
            #[inline(always)]
            pub fn pa2(&mut self) -> PA2_W<'_, PORTA_SPEC> {
                PA2_W::new(self, 2)
            }
            ///Bit 3 - Pin A3
            #[inline(always)]
            pub fn pa3(&mut self) -> PA3_W<'_, PORTA_SPEC> {
                PA3_W::new(self, 3)
            }
            ///Bit 4 - Pin A4
            #[inline(always)]
            pub fn pa4(&mut self) -> PA4_W<'_, PORTA_SPEC> {
                PA4_W::new(self, 4)
            }
            ///Bit 5 - Pin A5
            #[inline(always)]
            pub fn pa5(&mut self) -> PA5_W<'_, PORTA_SPEC> {
                PA5_W::new(self, 5)
            }
            ///Bit 6 - Pin A6
            #[inline(always)]
            pub fn pa6(&mut self) -> PA6_W<'_, PORTA_SPEC> {
                PA6_W::new(self, 6)
            }
            ///Bit 7 - Pin A7
            #[inline(always)]
            pub fn pa7(&mut self) -> PA7_W<'_, PORTA_SPEC> {
                PA7_W::new(self, 7)
            }
        }
        /**Port A Data Register

You can [`read`](crate::Reg::read) this register and get [`porta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTA_SPEC;
        impl crate::RegisterSpec for PORTA_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`porta::R`](R) reader structure
        impl crate::Readable for PORTA_SPEC {}
        ///`write(|w| ..)` method takes [`porta::W`](W) writer structure
        impl crate::Writable for PORTA_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTA to value 0
        impl crate::Resettable for PORTA_SPEC {}
    }
}
///I/O Port
pub type PORTB = crate::Periph<portb::RegisterBlock, 0x23>;
impl core::fmt::Debug for PORTB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTB").finish()
    }
}
///I/O Port
pub mod portb {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pinb: PINB,
        ddrb: DDRB,
        portb: PORTB,
    }
    impl RegisterBlock {
        ///0x00 - Port B Input Pins
        #[inline(always)]
        pub const fn pinb(&self) -> &PINB {
            &self.pinb
        }
        ///0x01 - Port B Data Direction Register
        #[inline(always)]
        pub const fn ddrb(&self) -> &DDRB {
            &self.ddrb
        }
        ///0x02 - Port B Data Register
        #[inline(always)]
        pub const fn portb(&self) -> &PORTB {
            &self.portb
        }
    }
    /**DDRB (rw) register accessor: Port B Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddrb`] module*/
    pub type DDRB = crate::Reg<ddrb::DDRB_SPEC>;
    ///Port B Data Direction Register
    pub mod ddrb {
        ///Register `DDRB` reader
        pub type R = crate::R<DDRB_SPEC>;
        ///Register `DDRB` writer
        pub type W = crate::W<DDRB_SPEC>;
        ///Field `PB0` reader - Pin B0
        pub type PB0_R = crate::BitReader;
        ///Field `PB0` writer - Pin B0
        pub type PB0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB1` reader - Pin B1
        pub type PB1_R = crate::BitReader;
        ///Field `PB1` writer - Pin B1
        pub type PB1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB2` reader - Pin B2
        pub type PB2_R = crate::BitReader;
        ///Field `PB2` writer - Pin B2
        pub type PB2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB3` reader - Pin B3
        pub type PB3_R = crate::BitReader;
        ///Field `PB3` writer - Pin B3
        pub type PB3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB4` reader - Pin B4
        pub type PB4_R = crate::BitReader;
        ///Field `PB4` writer - Pin B4
        pub type PB4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB5` reader - Pin B5
        pub type PB5_R = crate::BitReader;
        ///Field `PB5` writer - Pin B5
        pub type PB5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB6` reader - Pin B6
        pub type PB6_R = crate::BitReader;
        ///Field `PB6` writer - Pin B6
        pub type PB6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB7` reader - Pin B7
        pub type PB7_R = crate::BitReader;
        ///Field `PB7` writer - Pin B7
        pub type PB7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin B0
            #[inline(always)]
            pub fn pb0(&self) -> PB0_R {
                PB0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin B1
            #[inline(always)]
            pub fn pb1(&self) -> PB1_R {
                PB1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin B2
            #[inline(always)]
            pub fn pb2(&self) -> PB2_R {
                PB2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin B3
            #[inline(always)]
            pub fn pb3(&self) -> PB3_R {
                PB3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin B4
            #[inline(always)]
            pub fn pb4(&self) -> PB4_R {
                PB4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin B5
            #[inline(always)]
            pub fn pb5(&self) -> PB5_R {
                PB5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin B6
            #[inline(always)]
            pub fn pb6(&self) -> PB6_R {
                PB6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin B7
            #[inline(always)]
            pub fn pb7(&self) -> PB7_R {
                PB7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin B0
            #[inline(always)]
            pub fn pb0(&mut self) -> PB0_W<'_, DDRB_SPEC> {
                PB0_W::new(self, 0)
            }
            ///Bit 1 - Pin B1
            #[inline(always)]
            pub fn pb1(&mut self) -> PB1_W<'_, DDRB_SPEC> {
                PB1_W::new(self, 1)
            }
            ///Bit 2 - Pin B2
            #[inline(always)]
            pub fn pb2(&mut self) -> PB2_W<'_, DDRB_SPEC> {
                PB2_W::new(self, 2)
            }
            ///Bit 3 - Pin B3
            #[inline(always)]
            pub fn pb3(&mut self) -> PB3_W<'_, DDRB_SPEC> {
                PB3_W::new(self, 3)
            }
            ///Bit 4 - Pin B4
            #[inline(always)]
            pub fn pb4(&mut self) -> PB4_W<'_, DDRB_SPEC> {
                PB4_W::new(self, 4)
            }
            ///Bit 5 - Pin B5
            #[inline(always)]
            pub fn pb5(&mut self) -> PB5_W<'_, DDRB_SPEC> {
                PB5_W::new(self, 5)
            }
            ///Bit 6 - Pin B6
            #[inline(always)]
            pub fn pb6(&mut self) -> PB6_W<'_, DDRB_SPEC> {
                PB6_W::new(self, 6)
            }
            ///Bit 7 - Pin B7
            #[inline(always)]
            pub fn pb7(&mut self) -> PB7_W<'_, DDRB_SPEC> {
                PB7_W::new(self, 7)
            }
        }
        /**Port B Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRB_SPEC;
        impl crate::RegisterSpec for DDRB_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddrb::R`](R) reader structure
        impl crate::Readable for DDRB_SPEC {}
        ///`write(|w| ..)` method takes [`ddrb::W`](W) writer structure
        impl crate::Writable for DDRB_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRB to value 0
        impl crate::Resettable for DDRB_SPEC {}
    }
    /**PINB (rw) register accessor: Port B Input Pins

You can [`read`](crate::Reg::read) this register and get [`pinb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pinb`] module*/
    pub type PINB = crate::Reg<pinb::PINB_SPEC>;
    ///Port B Input Pins
    pub mod pinb {
        ///Register `PINB` reader
        pub type R = crate::R<PINB_SPEC>;
        ///Register `PINB` writer
        pub type W = crate::W<PINB_SPEC>;
        ///Field `PB0` reader - Pin B0
        pub type PB0_R = crate::BitReader;
        ///Field `PB0` writer - Pin B0
        pub type PB0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB1` reader - Pin B1
        pub type PB1_R = crate::BitReader;
        ///Field `PB1` writer - Pin B1
        pub type PB1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB2` reader - Pin B2
        pub type PB2_R = crate::BitReader;
        ///Field `PB2` writer - Pin B2
        pub type PB2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB3` reader - Pin B3
        pub type PB3_R = crate::BitReader;
        ///Field `PB3` writer - Pin B3
        pub type PB3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB4` reader - Pin B4
        pub type PB4_R = crate::BitReader;
        ///Field `PB4` writer - Pin B4
        pub type PB4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB5` reader - Pin B5
        pub type PB5_R = crate::BitReader;
        ///Field `PB5` writer - Pin B5
        pub type PB5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB6` reader - Pin B6
        pub type PB6_R = crate::BitReader;
        ///Field `PB6` writer - Pin B6
        pub type PB6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB7` reader - Pin B7
        pub type PB7_R = crate::BitReader;
        ///Field `PB7` writer - Pin B7
        pub type PB7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin B0
            #[inline(always)]
            pub fn pb0(&self) -> PB0_R {
                PB0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin B1
            #[inline(always)]
            pub fn pb1(&self) -> PB1_R {
                PB1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin B2
            #[inline(always)]
            pub fn pb2(&self) -> PB2_R {
                PB2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin B3
            #[inline(always)]
            pub fn pb3(&self) -> PB3_R {
                PB3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin B4
            #[inline(always)]
            pub fn pb4(&self) -> PB4_R {
                PB4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin B5
            #[inline(always)]
            pub fn pb5(&self) -> PB5_R {
                PB5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin B6
            #[inline(always)]
            pub fn pb6(&self) -> PB6_R {
                PB6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin B7
            #[inline(always)]
            pub fn pb7(&self) -> PB7_R {
                PB7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin B0
            #[inline(always)]
            pub fn pb0(&mut self) -> PB0_W<'_, PINB_SPEC> {
                PB0_W::new(self, 0)
            }
            ///Bit 1 - Pin B1
            #[inline(always)]
            pub fn pb1(&mut self) -> PB1_W<'_, PINB_SPEC> {
                PB1_W::new(self, 1)
            }
            ///Bit 2 - Pin B2
            #[inline(always)]
            pub fn pb2(&mut self) -> PB2_W<'_, PINB_SPEC> {
                PB2_W::new(self, 2)
            }
            ///Bit 3 - Pin B3
            #[inline(always)]
            pub fn pb3(&mut self) -> PB3_W<'_, PINB_SPEC> {
                PB3_W::new(self, 3)
            }
            ///Bit 4 - Pin B4
            #[inline(always)]
            pub fn pb4(&mut self) -> PB4_W<'_, PINB_SPEC> {
                PB4_W::new(self, 4)
            }
            ///Bit 5 - Pin B5
            #[inline(always)]
            pub fn pb5(&mut self) -> PB5_W<'_, PINB_SPEC> {
                PB5_W::new(self, 5)
            }
            ///Bit 6 - Pin B6
            #[inline(always)]
            pub fn pb6(&mut self) -> PB6_W<'_, PINB_SPEC> {
                PB6_W::new(self, 6)
            }
            ///Bit 7 - Pin B7
            #[inline(always)]
            pub fn pb7(&mut self) -> PB7_W<'_, PINB_SPEC> {
                PB7_W::new(self, 7)
            }
        }
        /**Port B Input Pins

You can [`read`](crate::Reg::read) this register and get [`pinb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PINB_SPEC;
        impl crate::RegisterSpec for PINB_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pinb::R`](R) reader structure
        impl crate::Readable for PINB_SPEC {}
        ///`write(|w| ..)` method takes [`pinb::W`](W) writer structure
        impl crate::Writable for PINB_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PINB to value 0
        impl crate::Resettable for PINB_SPEC {}
    }
    /**PORTB (rw) register accessor: Port B Data Register

You can [`read`](crate::Reg::read) this register and get [`portb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@portb`] module*/
    pub type PORTB = crate::Reg<portb::PORTB_SPEC>;
    ///Port B Data Register
    pub mod portb {
        ///Register `PORTB` reader
        pub type R = crate::R<PORTB_SPEC>;
        ///Register `PORTB` writer
        pub type W = crate::W<PORTB_SPEC>;
        ///Field `PB0` reader - Pin B0
        pub type PB0_R = crate::BitReader;
        ///Field `PB0` writer - Pin B0
        pub type PB0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB1` reader - Pin B1
        pub type PB1_R = crate::BitReader;
        ///Field `PB1` writer - Pin B1
        pub type PB1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB2` reader - Pin B2
        pub type PB2_R = crate::BitReader;
        ///Field `PB2` writer - Pin B2
        pub type PB2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB3` reader - Pin B3
        pub type PB3_R = crate::BitReader;
        ///Field `PB3` writer - Pin B3
        pub type PB3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB4` reader - Pin B4
        pub type PB4_R = crate::BitReader;
        ///Field `PB4` writer - Pin B4
        pub type PB4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB5` reader - Pin B5
        pub type PB5_R = crate::BitReader;
        ///Field `PB5` writer - Pin B5
        pub type PB5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB6` reader - Pin B6
        pub type PB6_R = crate::BitReader;
        ///Field `PB6` writer - Pin B6
        pub type PB6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PB7` reader - Pin B7
        pub type PB7_R = crate::BitReader;
        ///Field `PB7` writer - Pin B7
        pub type PB7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin B0
            #[inline(always)]
            pub fn pb0(&self) -> PB0_R {
                PB0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin B1
            #[inline(always)]
            pub fn pb1(&self) -> PB1_R {
                PB1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin B2
            #[inline(always)]
            pub fn pb2(&self) -> PB2_R {
                PB2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin B3
            #[inline(always)]
            pub fn pb3(&self) -> PB3_R {
                PB3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin B4
            #[inline(always)]
            pub fn pb4(&self) -> PB4_R {
                PB4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin B5
            #[inline(always)]
            pub fn pb5(&self) -> PB5_R {
                PB5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin B6
            #[inline(always)]
            pub fn pb6(&self) -> PB6_R {
                PB6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin B7
            #[inline(always)]
            pub fn pb7(&self) -> PB7_R {
                PB7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin B0
            #[inline(always)]
            pub fn pb0(&mut self) -> PB0_W<'_, PORTB_SPEC> {
                PB0_W::new(self, 0)
            }
            ///Bit 1 - Pin B1
            #[inline(always)]
            pub fn pb1(&mut self) -> PB1_W<'_, PORTB_SPEC> {
                PB1_W::new(self, 1)
            }
            ///Bit 2 - Pin B2
            #[inline(always)]
            pub fn pb2(&mut self) -> PB2_W<'_, PORTB_SPEC> {
                PB2_W::new(self, 2)
            }
            ///Bit 3 - Pin B3
            #[inline(always)]
            pub fn pb3(&mut self) -> PB3_W<'_, PORTB_SPEC> {
                PB3_W::new(self, 3)
            }
            ///Bit 4 - Pin B4
            #[inline(always)]
            pub fn pb4(&mut self) -> PB4_W<'_, PORTB_SPEC> {
                PB4_W::new(self, 4)
            }
            ///Bit 5 - Pin B5
            #[inline(always)]
            pub fn pb5(&mut self) -> PB5_W<'_, PORTB_SPEC> {
                PB5_W::new(self, 5)
            }
            ///Bit 6 - Pin B6
            #[inline(always)]
            pub fn pb6(&mut self) -> PB6_W<'_, PORTB_SPEC> {
                PB6_W::new(self, 6)
            }
            ///Bit 7 - Pin B7
            #[inline(always)]
            pub fn pb7(&mut self) -> PB7_W<'_, PORTB_SPEC> {
                PB7_W::new(self, 7)
            }
        }
        /**Port B Data Register

You can [`read`](crate::Reg::read) this register and get [`portb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTB_SPEC;
        impl crate::RegisterSpec for PORTB_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`portb::R`](R) reader structure
        impl crate::Readable for PORTB_SPEC {}
        ///`write(|w| ..)` method takes [`portb::W`](W) writer structure
        impl crate::Writable for PORTB_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTB to value 0
        impl crate::Resettable for PORTB_SPEC {}
    }
}
///I/O Port
pub type PORTC = crate::Periph<portc::RegisterBlock, 0x26>;
impl core::fmt::Debug for PORTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTC").finish()
    }
}
///I/O Port
pub mod portc {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pinc: PINC,
        ddrc: DDRC,
        portc: PORTC,
    }
    impl RegisterBlock {
        ///0x00 - Port C Input Pins
        #[inline(always)]
        pub const fn pinc(&self) -> &PINC {
            &self.pinc
        }
        ///0x01 - Port C Data Direction Register
        #[inline(always)]
        pub const fn ddrc(&self) -> &DDRC {
            &self.ddrc
        }
        ///0x02 - Port C Data Register
        #[inline(always)]
        pub const fn portc(&self) -> &PORTC {
            &self.portc
        }
    }
    /**DDRC (rw) register accessor: Port C Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddrc`] module*/
    pub type DDRC = crate::Reg<ddrc::DDRC_SPEC>;
    ///Port C Data Direction Register
    pub mod ddrc {
        ///Register `DDRC` reader
        pub type R = crate::R<DDRC_SPEC>;
        ///Register `DDRC` writer
        pub type W = crate::W<DDRC_SPEC>;
        ///Field `PC0` reader - Pin C0
        pub type PC0_R = crate::BitReader;
        ///Field `PC0` writer - Pin C0
        pub type PC0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC1` reader - Pin C1
        pub type PC1_R = crate::BitReader;
        ///Field `PC1` writer - Pin C1
        pub type PC1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC2` reader - Pin C2
        pub type PC2_R = crate::BitReader;
        ///Field `PC2` writer - Pin C2
        pub type PC2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC3` reader - Pin C3
        pub type PC3_R = crate::BitReader;
        ///Field `PC3` writer - Pin C3
        pub type PC3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC4` reader - Pin C4
        pub type PC4_R = crate::BitReader;
        ///Field `PC4` writer - Pin C4
        pub type PC4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC5` reader - Pin C5
        pub type PC5_R = crate::BitReader;
        ///Field `PC5` writer - Pin C5
        pub type PC5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC6` reader - Pin C6
        pub type PC6_R = crate::BitReader;
        ///Field `PC6` writer - Pin C6
        pub type PC6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC7` reader - Pin C7
        pub type PC7_R = crate::BitReader;
        ///Field `PC7` writer - Pin C7
        pub type PC7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin C0
            #[inline(always)]
            pub fn pc0(&self) -> PC0_R {
                PC0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin C1
            #[inline(always)]
            pub fn pc1(&self) -> PC1_R {
                PC1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin C2
            #[inline(always)]
            pub fn pc2(&self) -> PC2_R {
                PC2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin C3
            #[inline(always)]
            pub fn pc3(&self) -> PC3_R {
                PC3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin C4
            #[inline(always)]
            pub fn pc4(&self) -> PC4_R {
                PC4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin C5
            #[inline(always)]
            pub fn pc5(&self) -> PC5_R {
                PC5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin C6
            #[inline(always)]
            pub fn pc6(&self) -> PC6_R {
                PC6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin C7
            #[inline(always)]
            pub fn pc7(&self) -> PC7_R {
                PC7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin C0
            #[inline(always)]
            pub fn pc0(&mut self) -> PC0_W<'_, DDRC_SPEC> {
                PC0_W::new(self, 0)
            }
            ///Bit 1 - Pin C1
            #[inline(always)]
            pub fn pc1(&mut self) -> PC1_W<'_, DDRC_SPEC> {
                PC1_W::new(self, 1)
            }
            ///Bit 2 - Pin C2
            #[inline(always)]
            pub fn pc2(&mut self) -> PC2_W<'_, DDRC_SPEC> {
                PC2_W::new(self, 2)
            }
            ///Bit 3 - Pin C3
            #[inline(always)]
            pub fn pc3(&mut self) -> PC3_W<'_, DDRC_SPEC> {
                PC3_W::new(self, 3)
            }
            ///Bit 4 - Pin C4
            #[inline(always)]
            pub fn pc4(&mut self) -> PC4_W<'_, DDRC_SPEC> {
                PC4_W::new(self, 4)
            }
            ///Bit 5 - Pin C5
            #[inline(always)]
            pub fn pc5(&mut self) -> PC5_W<'_, DDRC_SPEC> {
                PC5_W::new(self, 5)
            }
            ///Bit 6 - Pin C6
            #[inline(always)]
            pub fn pc6(&mut self) -> PC6_W<'_, DDRC_SPEC> {
                PC6_W::new(self, 6)
            }
            ///Bit 7 - Pin C7
            #[inline(always)]
            pub fn pc7(&mut self) -> PC7_W<'_, DDRC_SPEC> {
                PC7_W::new(self, 7)
            }
        }
        /**Port C Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRC_SPEC;
        impl crate::RegisterSpec for DDRC_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddrc::R`](R) reader structure
        impl crate::Readable for DDRC_SPEC {}
        ///`write(|w| ..)` method takes [`ddrc::W`](W) writer structure
        impl crate::Writable for DDRC_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRC to value 0
        impl crate::Resettable for DDRC_SPEC {}
    }
    /**PINC (rw) register accessor: Port C Input Pins

You can [`read`](crate::Reg::read) this register and get [`pinc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pinc`] module*/
    pub type PINC = crate::Reg<pinc::PINC_SPEC>;
    ///Port C Input Pins
    pub mod pinc {
        ///Register `PINC` reader
        pub type R = crate::R<PINC_SPEC>;
        ///Register `PINC` writer
        pub type W = crate::W<PINC_SPEC>;
        ///Field `PC0` reader - Pin C0
        pub type PC0_R = crate::BitReader;
        ///Field `PC0` writer - Pin C0
        pub type PC0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC1` reader - Pin C1
        pub type PC1_R = crate::BitReader;
        ///Field `PC1` writer - Pin C1
        pub type PC1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC2` reader - Pin C2
        pub type PC2_R = crate::BitReader;
        ///Field `PC2` writer - Pin C2
        pub type PC2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC3` reader - Pin C3
        pub type PC3_R = crate::BitReader;
        ///Field `PC3` writer - Pin C3
        pub type PC3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC4` reader - Pin C4
        pub type PC4_R = crate::BitReader;
        ///Field `PC4` writer - Pin C4
        pub type PC4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC5` reader - Pin C5
        pub type PC5_R = crate::BitReader;
        ///Field `PC5` writer - Pin C5
        pub type PC5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC6` reader - Pin C6
        pub type PC6_R = crate::BitReader;
        ///Field `PC6` writer - Pin C6
        pub type PC6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC7` reader - Pin C7
        pub type PC7_R = crate::BitReader;
        ///Field `PC7` writer - Pin C7
        pub type PC7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin C0
            #[inline(always)]
            pub fn pc0(&self) -> PC0_R {
                PC0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin C1
            #[inline(always)]
            pub fn pc1(&self) -> PC1_R {
                PC1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin C2
            #[inline(always)]
            pub fn pc2(&self) -> PC2_R {
                PC2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin C3
            #[inline(always)]
            pub fn pc3(&self) -> PC3_R {
                PC3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin C4
            #[inline(always)]
            pub fn pc4(&self) -> PC4_R {
                PC4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin C5
            #[inline(always)]
            pub fn pc5(&self) -> PC5_R {
                PC5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin C6
            #[inline(always)]
            pub fn pc6(&self) -> PC6_R {
                PC6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin C7
            #[inline(always)]
            pub fn pc7(&self) -> PC7_R {
                PC7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin C0
            #[inline(always)]
            pub fn pc0(&mut self) -> PC0_W<'_, PINC_SPEC> {
                PC0_W::new(self, 0)
            }
            ///Bit 1 - Pin C1
            #[inline(always)]
            pub fn pc1(&mut self) -> PC1_W<'_, PINC_SPEC> {
                PC1_W::new(self, 1)
            }
            ///Bit 2 - Pin C2
            #[inline(always)]
            pub fn pc2(&mut self) -> PC2_W<'_, PINC_SPEC> {
                PC2_W::new(self, 2)
            }
            ///Bit 3 - Pin C3
            #[inline(always)]
            pub fn pc3(&mut self) -> PC3_W<'_, PINC_SPEC> {
                PC3_W::new(self, 3)
            }
            ///Bit 4 - Pin C4
            #[inline(always)]
            pub fn pc4(&mut self) -> PC4_W<'_, PINC_SPEC> {
                PC4_W::new(self, 4)
            }
            ///Bit 5 - Pin C5
            #[inline(always)]
            pub fn pc5(&mut self) -> PC5_W<'_, PINC_SPEC> {
                PC5_W::new(self, 5)
            }
            ///Bit 6 - Pin C6
            #[inline(always)]
            pub fn pc6(&mut self) -> PC6_W<'_, PINC_SPEC> {
                PC6_W::new(self, 6)
            }
            ///Bit 7 - Pin C7
            #[inline(always)]
            pub fn pc7(&mut self) -> PC7_W<'_, PINC_SPEC> {
                PC7_W::new(self, 7)
            }
        }
        /**Port C Input Pins

You can [`read`](crate::Reg::read) this register and get [`pinc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PINC_SPEC;
        impl crate::RegisterSpec for PINC_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pinc::R`](R) reader structure
        impl crate::Readable for PINC_SPEC {}
        ///`write(|w| ..)` method takes [`pinc::W`](W) writer structure
        impl crate::Writable for PINC_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PINC to value 0
        impl crate::Resettable for PINC_SPEC {}
    }
    /**PORTC (rw) register accessor: Port C Data Register

You can [`read`](crate::Reg::read) this register and get [`portc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@portc`] module*/
    pub type PORTC = crate::Reg<portc::PORTC_SPEC>;
    ///Port C Data Register
    pub mod portc {
        ///Register `PORTC` reader
        pub type R = crate::R<PORTC_SPEC>;
        ///Register `PORTC` writer
        pub type W = crate::W<PORTC_SPEC>;
        ///Field `PC0` reader - Pin C0
        pub type PC0_R = crate::BitReader;
        ///Field `PC0` writer - Pin C0
        pub type PC0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC1` reader - Pin C1
        pub type PC1_R = crate::BitReader;
        ///Field `PC1` writer - Pin C1
        pub type PC1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC2` reader - Pin C2
        pub type PC2_R = crate::BitReader;
        ///Field `PC2` writer - Pin C2
        pub type PC2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC3` reader - Pin C3
        pub type PC3_R = crate::BitReader;
        ///Field `PC3` writer - Pin C3
        pub type PC3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC4` reader - Pin C4
        pub type PC4_R = crate::BitReader;
        ///Field `PC4` writer - Pin C4
        pub type PC4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC5` reader - Pin C5
        pub type PC5_R = crate::BitReader;
        ///Field `PC5` writer - Pin C5
        pub type PC5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC6` reader - Pin C6
        pub type PC6_R = crate::BitReader;
        ///Field `PC6` writer - Pin C6
        pub type PC6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PC7` reader - Pin C7
        pub type PC7_R = crate::BitReader;
        ///Field `PC7` writer - Pin C7
        pub type PC7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin C0
            #[inline(always)]
            pub fn pc0(&self) -> PC0_R {
                PC0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin C1
            #[inline(always)]
            pub fn pc1(&self) -> PC1_R {
                PC1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin C2
            #[inline(always)]
            pub fn pc2(&self) -> PC2_R {
                PC2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin C3
            #[inline(always)]
            pub fn pc3(&self) -> PC3_R {
                PC3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin C4
            #[inline(always)]
            pub fn pc4(&self) -> PC4_R {
                PC4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin C5
            #[inline(always)]
            pub fn pc5(&self) -> PC5_R {
                PC5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin C6
            #[inline(always)]
            pub fn pc6(&self) -> PC6_R {
                PC6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin C7
            #[inline(always)]
            pub fn pc7(&self) -> PC7_R {
                PC7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin C0
            #[inline(always)]
            pub fn pc0(&mut self) -> PC0_W<'_, PORTC_SPEC> {
                PC0_W::new(self, 0)
            }
            ///Bit 1 - Pin C1
            #[inline(always)]
            pub fn pc1(&mut self) -> PC1_W<'_, PORTC_SPEC> {
                PC1_W::new(self, 1)
            }
            ///Bit 2 - Pin C2
            #[inline(always)]
            pub fn pc2(&mut self) -> PC2_W<'_, PORTC_SPEC> {
                PC2_W::new(self, 2)
            }
            ///Bit 3 - Pin C3
            #[inline(always)]
            pub fn pc3(&mut self) -> PC3_W<'_, PORTC_SPEC> {
                PC3_W::new(self, 3)
            }
            ///Bit 4 - Pin C4
            #[inline(always)]
            pub fn pc4(&mut self) -> PC4_W<'_, PORTC_SPEC> {
                PC4_W::new(self, 4)
            }
            ///Bit 5 - Pin C5
            #[inline(always)]
            pub fn pc5(&mut self) -> PC5_W<'_, PORTC_SPEC> {
                PC5_W::new(self, 5)
            }
            ///Bit 6 - Pin C6
            #[inline(always)]
            pub fn pc6(&mut self) -> PC6_W<'_, PORTC_SPEC> {
                PC6_W::new(self, 6)
            }
            ///Bit 7 - Pin C7
            #[inline(always)]
            pub fn pc7(&mut self) -> PC7_W<'_, PORTC_SPEC> {
                PC7_W::new(self, 7)
            }
        }
        /**Port C Data Register

You can [`read`](crate::Reg::read) this register and get [`portc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTC_SPEC;
        impl crate::RegisterSpec for PORTC_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`portc::R`](R) reader structure
        impl crate::Readable for PORTC_SPEC {}
        ///`write(|w| ..)` method takes [`portc::W`](W) writer structure
        impl crate::Writable for PORTC_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTC to value 0
        impl crate::Resettable for PORTC_SPEC {}
    }
}
///I/O Port
pub type PORTD = crate::Periph<portd::RegisterBlock, 0x29>;
impl core::fmt::Debug for PORTD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTD").finish()
    }
}
///I/O Port
pub mod portd {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pind: PIND,
        ddrd: DDRD,
        portd: PORTD,
    }
    impl RegisterBlock {
        ///0x00 - Port D Input Pins
        #[inline(always)]
        pub const fn pind(&self) -> &PIND {
            &self.pind
        }
        ///0x01 - Port D Data Direction Register
        #[inline(always)]
        pub const fn ddrd(&self) -> &DDRD {
            &self.ddrd
        }
        ///0x02 - Port D Data Register
        #[inline(always)]
        pub const fn portd(&self) -> &PORTD {
            &self.portd
        }
    }
    /**DDRD (rw) register accessor: Port D Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddrd`] module*/
    pub type DDRD = crate::Reg<ddrd::DDRD_SPEC>;
    ///Port D Data Direction Register
    pub mod ddrd {
        ///Register `DDRD` reader
        pub type R = crate::R<DDRD_SPEC>;
        ///Register `DDRD` writer
        pub type W = crate::W<DDRD_SPEC>;
        ///Field `PD0` reader - Pin D0
        pub type PD0_R = crate::BitReader;
        ///Field `PD0` writer - Pin D0
        pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD1` reader - Pin D1
        pub type PD1_R = crate::BitReader;
        ///Field `PD1` writer - Pin D1
        pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD2` reader - Pin D2
        pub type PD2_R = crate::BitReader;
        ///Field `PD2` writer - Pin D2
        pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD3` reader - Pin D3
        pub type PD3_R = crate::BitReader;
        ///Field `PD3` writer - Pin D3
        pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD4` reader - Pin D4
        pub type PD4_R = crate::BitReader;
        ///Field `PD4` writer - Pin D4
        pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD5` reader - Pin D5
        pub type PD5_R = crate::BitReader;
        ///Field `PD5` writer - Pin D5
        pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD6` reader - Pin D6
        pub type PD6_R = crate::BitReader;
        ///Field `PD6` writer - Pin D6
        pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD7` reader - Pin D7
        pub type PD7_R = crate::BitReader;
        ///Field `PD7` writer - Pin D7
        pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin D0
            #[inline(always)]
            pub fn pd0(&self) -> PD0_R {
                PD0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin D1
            #[inline(always)]
            pub fn pd1(&self) -> PD1_R {
                PD1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin D2
            #[inline(always)]
            pub fn pd2(&self) -> PD2_R {
                PD2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin D3
            #[inline(always)]
            pub fn pd3(&self) -> PD3_R {
                PD3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin D4
            #[inline(always)]
            pub fn pd4(&self) -> PD4_R {
                PD4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin D5
            #[inline(always)]
            pub fn pd5(&self) -> PD5_R {
                PD5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin D6
            #[inline(always)]
            pub fn pd6(&self) -> PD6_R {
                PD6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin D7
            #[inline(always)]
            pub fn pd7(&self) -> PD7_R {
                PD7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin D0
            #[inline(always)]
            pub fn pd0(&mut self) -> PD0_W<'_, DDRD_SPEC> {
                PD0_W::new(self, 0)
            }
            ///Bit 1 - Pin D1
            #[inline(always)]
            pub fn pd1(&mut self) -> PD1_W<'_, DDRD_SPEC> {
                PD1_W::new(self, 1)
            }
            ///Bit 2 - Pin D2
            #[inline(always)]
            pub fn pd2(&mut self) -> PD2_W<'_, DDRD_SPEC> {
                PD2_W::new(self, 2)
            }
            ///Bit 3 - Pin D3
            #[inline(always)]
            pub fn pd3(&mut self) -> PD3_W<'_, DDRD_SPEC> {
                PD3_W::new(self, 3)
            }
            ///Bit 4 - Pin D4
            #[inline(always)]
            pub fn pd4(&mut self) -> PD4_W<'_, DDRD_SPEC> {
                PD4_W::new(self, 4)
            }
            ///Bit 5 - Pin D5
            #[inline(always)]
            pub fn pd5(&mut self) -> PD5_W<'_, DDRD_SPEC> {
                PD5_W::new(self, 5)
            }
            ///Bit 6 - Pin D6
            #[inline(always)]
            pub fn pd6(&mut self) -> PD6_W<'_, DDRD_SPEC> {
                PD6_W::new(self, 6)
            }
            ///Bit 7 - Pin D7
            #[inline(always)]
            pub fn pd7(&mut self) -> PD7_W<'_, DDRD_SPEC> {
                PD7_W::new(self, 7)
            }
        }
        /**Port D Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRD_SPEC;
        impl crate::RegisterSpec for DDRD_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddrd::R`](R) reader structure
        impl crate::Readable for DDRD_SPEC {}
        ///`write(|w| ..)` method takes [`ddrd::W`](W) writer structure
        impl crate::Writable for DDRD_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRD to value 0
        impl crate::Resettable for DDRD_SPEC {}
    }
    /**PIND (rw) register accessor: Port D Input Pins

You can [`read`](crate::Reg::read) this register and get [`pind::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pind::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pind`] module*/
    pub type PIND = crate::Reg<pind::PIND_SPEC>;
    ///Port D Input Pins
    pub mod pind {
        ///Register `PIND` reader
        pub type R = crate::R<PIND_SPEC>;
        ///Register `PIND` writer
        pub type W = crate::W<PIND_SPEC>;
        ///Field `PD0` reader - Pin D0
        pub type PD0_R = crate::BitReader;
        ///Field `PD0` writer - Pin D0
        pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD1` reader - Pin D1
        pub type PD1_R = crate::BitReader;
        ///Field `PD1` writer - Pin D1
        pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD2` reader - Pin D2
        pub type PD2_R = crate::BitReader;
        ///Field `PD2` writer - Pin D2
        pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD3` reader - Pin D3
        pub type PD3_R = crate::BitReader;
        ///Field `PD3` writer - Pin D3
        pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD4` reader - Pin D4
        pub type PD4_R = crate::BitReader;
        ///Field `PD4` writer - Pin D4
        pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD5` reader - Pin D5
        pub type PD5_R = crate::BitReader;
        ///Field `PD5` writer - Pin D5
        pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD6` reader - Pin D6
        pub type PD6_R = crate::BitReader;
        ///Field `PD6` writer - Pin D6
        pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD7` reader - Pin D7
        pub type PD7_R = crate::BitReader;
        ///Field `PD7` writer - Pin D7
        pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin D0
            #[inline(always)]
            pub fn pd0(&self) -> PD0_R {
                PD0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin D1
            #[inline(always)]
            pub fn pd1(&self) -> PD1_R {
                PD1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin D2
            #[inline(always)]
            pub fn pd2(&self) -> PD2_R {
                PD2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin D3
            #[inline(always)]
            pub fn pd3(&self) -> PD3_R {
                PD3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin D4
            #[inline(always)]
            pub fn pd4(&self) -> PD4_R {
                PD4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin D5
            #[inline(always)]
            pub fn pd5(&self) -> PD5_R {
                PD5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin D6
            #[inline(always)]
            pub fn pd6(&self) -> PD6_R {
                PD6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin D7
            #[inline(always)]
            pub fn pd7(&self) -> PD7_R {
                PD7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin D0
            #[inline(always)]
            pub fn pd0(&mut self) -> PD0_W<'_, PIND_SPEC> {
                PD0_W::new(self, 0)
            }
            ///Bit 1 - Pin D1
            #[inline(always)]
            pub fn pd1(&mut self) -> PD1_W<'_, PIND_SPEC> {
                PD1_W::new(self, 1)
            }
            ///Bit 2 - Pin D2
            #[inline(always)]
            pub fn pd2(&mut self) -> PD2_W<'_, PIND_SPEC> {
                PD2_W::new(self, 2)
            }
            ///Bit 3 - Pin D3
            #[inline(always)]
            pub fn pd3(&mut self) -> PD3_W<'_, PIND_SPEC> {
                PD3_W::new(self, 3)
            }
            ///Bit 4 - Pin D4
            #[inline(always)]
            pub fn pd4(&mut self) -> PD4_W<'_, PIND_SPEC> {
                PD4_W::new(self, 4)
            }
            ///Bit 5 - Pin D5
            #[inline(always)]
            pub fn pd5(&mut self) -> PD5_W<'_, PIND_SPEC> {
                PD5_W::new(self, 5)
            }
            ///Bit 6 - Pin D6
            #[inline(always)]
            pub fn pd6(&mut self) -> PD6_W<'_, PIND_SPEC> {
                PD6_W::new(self, 6)
            }
            ///Bit 7 - Pin D7
            #[inline(always)]
            pub fn pd7(&mut self) -> PD7_W<'_, PIND_SPEC> {
                PD7_W::new(self, 7)
            }
        }
        /**Port D Input Pins

You can [`read`](crate::Reg::read) this register and get [`pind::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pind::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PIND_SPEC;
        impl crate::RegisterSpec for PIND_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pind::R`](R) reader structure
        impl crate::Readable for PIND_SPEC {}
        ///`write(|w| ..)` method takes [`pind::W`](W) writer structure
        impl crate::Writable for PIND_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PIND to value 0
        impl crate::Resettable for PIND_SPEC {}
    }
    /**PORTD (rw) register accessor: Port D Data Register

You can [`read`](crate::Reg::read) this register and get [`portd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@portd`] module*/
    pub type PORTD = crate::Reg<portd::PORTD_SPEC>;
    ///Port D Data Register
    pub mod portd {
        ///Register `PORTD` reader
        pub type R = crate::R<PORTD_SPEC>;
        ///Register `PORTD` writer
        pub type W = crate::W<PORTD_SPEC>;
        ///Field `PD0` reader - Pin D0
        pub type PD0_R = crate::BitReader;
        ///Field `PD0` writer - Pin D0
        pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD1` reader - Pin D1
        pub type PD1_R = crate::BitReader;
        ///Field `PD1` writer - Pin D1
        pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD2` reader - Pin D2
        pub type PD2_R = crate::BitReader;
        ///Field `PD2` writer - Pin D2
        pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD3` reader - Pin D3
        pub type PD3_R = crate::BitReader;
        ///Field `PD3` writer - Pin D3
        pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD4` reader - Pin D4
        pub type PD4_R = crate::BitReader;
        ///Field `PD4` writer - Pin D4
        pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD5` reader - Pin D5
        pub type PD5_R = crate::BitReader;
        ///Field `PD5` writer - Pin D5
        pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD6` reader - Pin D6
        pub type PD6_R = crate::BitReader;
        ///Field `PD6` writer - Pin D6
        pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PD7` reader - Pin D7
        pub type PD7_R = crate::BitReader;
        ///Field `PD7` writer - Pin D7
        pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin D0
            #[inline(always)]
            pub fn pd0(&self) -> PD0_R {
                PD0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin D1
            #[inline(always)]
            pub fn pd1(&self) -> PD1_R {
                PD1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin D2
            #[inline(always)]
            pub fn pd2(&self) -> PD2_R {
                PD2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin D3
            #[inline(always)]
            pub fn pd3(&self) -> PD3_R {
                PD3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin D4
            #[inline(always)]
            pub fn pd4(&self) -> PD4_R {
                PD4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin D5
            #[inline(always)]
            pub fn pd5(&self) -> PD5_R {
                PD5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin D6
            #[inline(always)]
            pub fn pd6(&self) -> PD6_R {
                PD6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin D7
            #[inline(always)]
            pub fn pd7(&self) -> PD7_R {
                PD7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin D0
            #[inline(always)]
            pub fn pd0(&mut self) -> PD0_W<'_, PORTD_SPEC> {
                PD0_W::new(self, 0)
            }
            ///Bit 1 - Pin D1
            #[inline(always)]
            pub fn pd1(&mut self) -> PD1_W<'_, PORTD_SPEC> {
                PD1_W::new(self, 1)
            }
            ///Bit 2 - Pin D2
            #[inline(always)]
            pub fn pd2(&mut self) -> PD2_W<'_, PORTD_SPEC> {
                PD2_W::new(self, 2)
            }
            ///Bit 3 - Pin D3
            #[inline(always)]
            pub fn pd3(&mut self) -> PD3_W<'_, PORTD_SPEC> {
                PD3_W::new(self, 3)
            }
            ///Bit 4 - Pin D4
            #[inline(always)]
            pub fn pd4(&mut self) -> PD4_W<'_, PORTD_SPEC> {
                PD4_W::new(self, 4)
            }
            ///Bit 5 - Pin D5
            #[inline(always)]
            pub fn pd5(&mut self) -> PD5_W<'_, PORTD_SPEC> {
                PD5_W::new(self, 5)
            }
            ///Bit 6 - Pin D6
            #[inline(always)]
            pub fn pd6(&mut self) -> PD6_W<'_, PORTD_SPEC> {
                PD6_W::new(self, 6)
            }
            ///Bit 7 - Pin D7
            #[inline(always)]
            pub fn pd7(&mut self) -> PD7_W<'_, PORTD_SPEC> {
                PD7_W::new(self, 7)
            }
        }
        /**Port D Data Register

You can [`read`](crate::Reg::read) this register and get [`portd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTD_SPEC;
        impl crate::RegisterSpec for PORTD_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`portd::R`](R) reader structure
        impl crate::Readable for PORTD_SPEC {}
        ///`write(|w| ..)` method takes [`portd::W`](W) writer structure
        impl crate::Writable for PORTD_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTD to value 0
        impl crate::Resettable for PORTD_SPEC {}
    }
}
///I/O Port
pub type PORTE = crate::Periph<porte::RegisterBlock, 0x2c>;
impl core::fmt::Debug for PORTE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTE").finish()
    }
}
///I/O Port
pub mod porte {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pine: PINE,
        ddre: DDRE,
        porte: PORTE,
    }
    impl RegisterBlock {
        ///0x00 - Input Pins, Port E
        #[inline(always)]
        pub const fn pine(&self) -> &PINE {
            &self.pine
        }
        ///0x01 - Data Direction Register, Port E
        #[inline(always)]
        pub const fn ddre(&self) -> &DDRE {
            &self.ddre
        }
        ///0x02 - Data Register, Port E
        #[inline(always)]
        pub const fn porte(&self) -> &PORTE {
            &self.porte
        }
    }
    /**DDRE (rw) register accessor: Data Direction Register, Port E

You can [`read`](crate::Reg::read) this register and get [`ddre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddre`] module*/
    pub type DDRE = crate::Reg<ddre::DDRE_SPEC>;
    ///Data Direction Register, Port E
    pub mod ddre {
        ///Register `DDRE` reader
        pub type R = crate::R<DDRE_SPEC>;
        ///Register `DDRE` writer
        pub type W = crate::W<DDRE_SPEC>;
        ///Field `PE0` reader - Pin E0
        pub type PE0_R = crate::BitReader;
        ///Field `PE0` writer - Pin E0
        pub type PE0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE1` reader - Pin E1
        pub type PE1_R = crate::BitReader;
        ///Field `PE1` writer - Pin E1
        pub type PE1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE2` reader - Pin E2
        pub type PE2_R = crate::BitReader;
        ///Field `PE2` writer - Pin E2
        pub type PE2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE3` reader - Pin E3
        pub type PE3_R = crate::BitReader;
        ///Field `PE3` writer - Pin E3
        pub type PE3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE4` reader - Pin E4
        pub type PE4_R = crate::BitReader;
        ///Field `PE4` writer - Pin E4
        pub type PE4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE5` reader - Pin E5
        pub type PE5_R = crate::BitReader;
        ///Field `PE5` writer - Pin E5
        pub type PE5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE6` reader - Pin E6
        pub type PE6_R = crate::BitReader;
        ///Field `PE6` writer - Pin E6
        pub type PE6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE7` reader - Pin E7
        pub type PE7_R = crate::BitReader;
        ///Field `PE7` writer - Pin E7
        pub type PE7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin E0
            #[inline(always)]
            pub fn pe0(&self) -> PE0_R {
                PE0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin E1
            #[inline(always)]
            pub fn pe1(&self) -> PE1_R {
                PE1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin E2
            #[inline(always)]
            pub fn pe2(&self) -> PE2_R {
                PE2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin E3
            #[inline(always)]
            pub fn pe3(&self) -> PE3_R {
                PE3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin E4
            #[inline(always)]
            pub fn pe4(&self) -> PE4_R {
                PE4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin E5
            #[inline(always)]
            pub fn pe5(&self) -> PE5_R {
                PE5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin E6
            #[inline(always)]
            pub fn pe6(&self) -> PE6_R {
                PE6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin E7
            #[inline(always)]
            pub fn pe7(&self) -> PE7_R {
                PE7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin E0
            #[inline(always)]
            pub fn pe0(&mut self) -> PE0_W<'_, DDRE_SPEC> {
                PE0_W::new(self, 0)
            }
            ///Bit 1 - Pin E1
            #[inline(always)]
            pub fn pe1(&mut self) -> PE1_W<'_, DDRE_SPEC> {
                PE1_W::new(self, 1)
            }
            ///Bit 2 - Pin E2
            #[inline(always)]
            pub fn pe2(&mut self) -> PE2_W<'_, DDRE_SPEC> {
                PE2_W::new(self, 2)
            }
            ///Bit 3 - Pin E3
            #[inline(always)]
            pub fn pe3(&mut self) -> PE3_W<'_, DDRE_SPEC> {
                PE3_W::new(self, 3)
            }
            ///Bit 4 - Pin E4
            #[inline(always)]
            pub fn pe4(&mut self) -> PE4_W<'_, DDRE_SPEC> {
                PE4_W::new(self, 4)
            }
            ///Bit 5 - Pin E5
            #[inline(always)]
            pub fn pe5(&mut self) -> PE5_W<'_, DDRE_SPEC> {
                PE5_W::new(self, 5)
            }
            ///Bit 6 - Pin E6
            #[inline(always)]
            pub fn pe6(&mut self) -> PE6_W<'_, DDRE_SPEC> {
                PE6_W::new(self, 6)
            }
            ///Bit 7 - Pin E7
            #[inline(always)]
            pub fn pe7(&mut self) -> PE7_W<'_, DDRE_SPEC> {
                PE7_W::new(self, 7)
            }
        }
        /**Data Direction Register, Port E

You can [`read`](crate::Reg::read) this register and get [`ddre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRE_SPEC;
        impl crate::RegisterSpec for DDRE_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddre::R`](R) reader structure
        impl crate::Readable for DDRE_SPEC {}
        ///`write(|w| ..)` method takes [`ddre::W`](W) writer structure
        impl crate::Writable for DDRE_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRE to value 0
        impl crate::Resettable for DDRE_SPEC {}
    }
    /**PINE (rw) register accessor: Input Pins, Port E

You can [`read`](crate::Reg::read) this register and get [`pine::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pine::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pine`] module*/
    pub type PINE = crate::Reg<pine::PINE_SPEC>;
    ///Input Pins, Port E
    pub mod pine {
        ///Register `PINE` reader
        pub type R = crate::R<PINE_SPEC>;
        ///Register `PINE` writer
        pub type W = crate::W<PINE_SPEC>;
        ///Field `PE0` reader - Pin E0
        pub type PE0_R = crate::BitReader;
        ///Field `PE0` writer - Pin E0
        pub type PE0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE1` reader - Pin E1
        pub type PE1_R = crate::BitReader;
        ///Field `PE1` writer - Pin E1
        pub type PE1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE2` reader - Pin E2
        pub type PE2_R = crate::BitReader;
        ///Field `PE2` writer - Pin E2
        pub type PE2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE3` reader - Pin E3
        pub type PE3_R = crate::BitReader;
        ///Field `PE3` writer - Pin E3
        pub type PE3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE4` reader - Pin E4
        pub type PE4_R = crate::BitReader;
        ///Field `PE4` writer - Pin E4
        pub type PE4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE5` reader - Pin E5
        pub type PE5_R = crate::BitReader;
        ///Field `PE5` writer - Pin E5
        pub type PE5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE6` reader - Pin E6
        pub type PE6_R = crate::BitReader;
        ///Field `PE6` writer - Pin E6
        pub type PE6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE7` reader - Pin E7
        pub type PE7_R = crate::BitReader;
        ///Field `PE7` writer - Pin E7
        pub type PE7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin E0
            #[inline(always)]
            pub fn pe0(&self) -> PE0_R {
                PE0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin E1
            #[inline(always)]
            pub fn pe1(&self) -> PE1_R {
                PE1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin E2
            #[inline(always)]
            pub fn pe2(&self) -> PE2_R {
                PE2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin E3
            #[inline(always)]
            pub fn pe3(&self) -> PE3_R {
                PE3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin E4
            #[inline(always)]
            pub fn pe4(&self) -> PE4_R {
                PE4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin E5
            #[inline(always)]
            pub fn pe5(&self) -> PE5_R {
                PE5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin E6
            #[inline(always)]
            pub fn pe6(&self) -> PE6_R {
                PE6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin E7
            #[inline(always)]
            pub fn pe7(&self) -> PE7_R {
                PE7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin E0
            #[inline(always)]
            pub fn pe0(&mut self) -> PE0_W<'_, PINE_SPEC> {
                PE0_W::new(self, 0)
            }
            ///Bit 1 - Pin E1
            #[inline(always)]
            pub fn pe1(&mut self) -> PE1_W<'_, PINE_SPEC> {
                PE1_W::new(self, 1)
            }
            ///Bit 2 - Pin E2
            #[inline(always)]
            pub fn pe2(&mut self) -> PE2_W<'_, PINE_SPEC> {
                PE2_W::new(self, 2)
            }
            ///Bit 3 - Pin E3
            #[inline(always)]
            pub fn pe3(&mut self) -> PE3_W<'_, PINE_SPEC> {
                PE3_W::new(self, 3)
            }
            ///Bit 4 - Pin E4
            #[inline(always)]
            pub fn pe4(&mut self) -> PE4_W<'_, PINE_SPEC> {
                PE4_W::new(self, 4)
            }
            ///Bit 5 - Pin E5
            #[inline(always)]
            pub fn pe5(&mut self) -> PE5_W<'_, PINE_SPEC> {
                PE5_W::new(self, 5)
            }
            ///Bit 6 - Pin E6
            #[inline(always)]
            pub fn pe6(&mut self) -> PE6_W<'_, PINE_SPEC> {
                PE6_W::new(self, 6)
            }
            ///Bit 7 - Pin E7
            #[inline(always)]
            pub fn pe7(&mut self) -> PE7_W<'_, PINE_SPEC> {
                PE7_W::new(self, 7)
            }
        }
        /**Input Pins, Port E

You can [`read`](crate::Reg::read) this register and get [`pine::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pine::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PINE_SPEC;
        impl crate::RegisterSpec for PINE_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pine::R`](R) reader structure
        impl crate::Readable for PINE_SPEC {}
        ///`write(|w| ..)` method takes [`pine::W`](W) writer structure
        impl crate::Writable for PINE_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PINE to value 0
        impl crate::Resettable for PINE_SPEC {}
    }
    /**PORTE (rw) register accessor: Data Register, Port E

You can [`read`](crate::Reg::read) this register and get [`porte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@porte`] module*/
    pub type PORTE = crate::Reg<porte::PORTE_SPEC>;
    ///Data Register, Port E
    pub mod porte {
        ///Register `PORTE` reader
        pub type R = crate::R<PORTE_SPEC>;
        ///Register `PORTE` writer
        pub type W = crate::W<PORTE_SPEC>;
        ///Field `PE0` reader - Pin E0
        pub type PE0_R = crate::BitReader;
        ///Field `PE0` writer - Pin E0
        pub type PE0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE1` reader - Pin E1
        pub type PE1_R = crate::BitReader;
        ///Field `PE1` writer - Pin E1
        pub type PE1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE2` reader - Pin E2
        pub type PE2_R = crate::BitReader;
        ///Field `PE2` writer - Pin E2
        pub type PE2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE3` reader - Pin E3
        pub type PE3_R = crate::BitReader;
        ///Field `PE3` writer - Pin E3
        pub type PE3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE4` reader - Pin E4
        pub type PE4_R = crate::BitReader;
        ///Field `PE4` writer - Pin E4
        pub type PE4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE5` reader - Pin E5
        pub type PE5_R = crate::BitReader;
        ///Field `PE5` writer - Pin E5
        pub type PE5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE6` reader - Pin E6
        pub type PE6_R = crate::BitReader;
        ///Field `PE6` writer - Pin E6
        pub type PE6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PE7` reader - Pin E7
        pub type PE7_R = crate::BitReader;
        ///Field `PE7` writer - Pin E7
        pub type PE7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin E0
            #[inline(always)]
            pub fn pe0(&self) -> PE0_R {
                PE0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin E1
            #[inline(always)]
            pub fn pe1(&self) -> PE1_R {
                PE1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin E2
            #[inline(always)]
            pub fn pe2(&self) -> PE2_R {
                PE2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin E3
            #[inline(always)]
            pub fn pe3(&self) -> PE3_R {
                PE3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin E4
            #[inline(always)]
            pub fn pe4(&self) -> PE4_R {
                PE4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin E5
            #[inline(always)]
            pub fn pe5(&self) -> PE5_R {
                PE5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin E6
            #[inline(always)]
            pub fn pe6(&self) -> PE6_R {
                PE6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin E7
            #[inline(always)]
            pub fn pe7(&self) -> PE7_R {
                PE7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin E0
            #[inline(always)]
            pub fn pe0(&mut self) -> PE0_W<'_, PORTE_SPEC> {
                PE0_W::new(self, 0)
            }
            ///Bit 1 - Pin E1
            #[inline(always)]
            pub fn pe1(&mut self) -> PE1_W<'_, PORTE_SPEC> {
                PE1_W::new(self, 1)
            }
            ///Bit 2 - Pin E2
            #[inline(always)]
            pub fn pe2(&mut self) -> PE2_W<'_, PORTE_SPEC> {
                PE2_W::new(self, 2)
            }
            ///Bit 3 - Pin E3
            #[inline(always)]
            pub fn pe3(&mut self) -> PE3_W<'_, PORTE_SPEC> {
                PE3_W::new(self, 3)
            }
            ///Bit 4 - Pin E4
            #[inline(always)]
            pub fn pe4(&mut self) -> PE4_W<'_, PORTE_SPEC> {
                PE4_W::new(self, 4)
            }
            ///Bit 5 - Pin E5
            #[inline(always)]
            pub fn pe5(&mut self) -> PE5_W<'_, PORTE_SPEC> {
                PE5_W::new(self, 5)
            }
            ///Bit 6 - Pin E6
            #[inline(always)]
            pub fn pe6(&mut self) -> PE6_W<'_, PORTE_SPEC> {
                PE6_W::new(self, 6)
            }
            ///Bit 7 - Pin E7
            #[inline(always)]
            pub fn pe7(&mut self) -> PE7_W<'_, PORTE_SPEC> {
                PE7_W::new(self, 7)
            }
        }
        /**Data Register, Port E

You can [`read`](crate::Reg::read) this register and get [`porte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTE_SPEC;
        impl crate::RegisterSpec for PORTE_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`porte::R`](R) reader structure
        impl crate::Readable for PORTE_SPEC {}
        ///`write(|w| ..)` method takes [`porte::W`](W) writer structure
        impl crate::Writable for PORTE_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTE to value 0
        impl crate::Resettable for PORTE_SPEC {}
    }
}
///I/O Port
pub type PORTF = crate::Periph<portf::RegisterBlock, 0x2f>;
impl core::fmt::Debug for PORTF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTF").finish()
    }
}
///I/O Port
pub mod portf {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pinf: PINF,
        ddrf: DDRF,
        portf: PORTF,
    }
    impl RegisterBlock {
        ///0x00 - Input Pins, Port F
        #[inline(always)]
        pub const fn pinf(&self) -> &PINF {
            &self.pinf
        }
        ///0x01 - Data Direction Register, Port F
        #[inline(always)]
        pub const fn ddrf(&self) -> &DDRF {
            &self.ddrf
        }
        ///0x02 - Data Register, Port F
        #[inline(always)]
        pub const fn portf(&self) -> &PORTF {
            &self.portf
        }
    }
    /**DDRF (rw) register accessor: Data Direction Register, Port F

You can [`read`](crate::Reg::read) this register and get [`ddrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddrf`] module*/
    pub type DDRF = crate::Reg<ddrf::DDRF_SPEC>;
    ///Data Direction Register, Port F
    pub mod ddrf {
        ///Register `DDRF` reader
        pub type R = crate::R<DDRF_SPEC>;
        ///Register `DDRF` writer
        pub type W = crate::W<DDRF_SPEC>;
        ///Field `PF0` reader - Pin F0
        pub type PF0_R = crate::BitReader;
        ///Field `PF0` writer - Pin F0
        pub type PF0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF1` reader - Pin F1
        pub type PF1_R = crate::BitReader;
        ///Field `PF1` writer - Pin F1
        pub type PF1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF2` reader - Pin F2
        pub type PF2_R = crate::BitReader;
        ///Field `PF2` writer - Pin F2
        pub type PF2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF3` reader - Pin F3
        pub type PF3_R = crate::BitReader;
        ///Field `PF3` writer - Pin F3
        pub type PF3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF4` reader - Pin F4
        pub type PF4_R = crate::BitReader;
        ///Field `PF4` writer - Pin F4
        pub type PF4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF5` reader - Pin F5
        pub type PF5_R = crate::BitReader;
        ///Field `PF5` writer - Pin F5
        pub type PF5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF6` reader - Pin F6
        pub type PF6_R = crate::BitReader;
        ///Field `PF6` writer - Pin F6
        pub type PF6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF7` reader - Pin F7
        pub type PF7_R = crate::BitReader;
        ///Field `PF7` writer - Pin F7
        pub type PF7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin F0
            #[inline(always)]
            pub fn pf0(&self) -> PF0_R {
                PF0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin F1
            #[inline(always)]
            pub fn pf1(&self) -> PF1_R {
                PF1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin F2
            #[inline(always)]
            pub fn pf2(&self) -> PF2_R {
                PF2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin F3
            #[inline(always)]
            pub fn pf3(&self) -> PF3_R {
                PF3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin F4
            #[inline(always)]
            pub fn pf4(&self) -> PF4_R {
                PF4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin F5
            #[inline(always)]
            pub fn pf5(&self) -> PF5_R {
                PF5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin F6
            #[inline(always)]
            pub fn pf6(&self) -> PF6_R {
                PF6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin F7
            #[inline(always)]
            pub fn pf7(&self) -> PF7_R {
                PF7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin F0
            #[inline(always)]
            pub fn pf0(&mut self) -> PF0_W<'_, DDRF_SPEC> {
                PF0_W::new(self, 0)
            }
            ///Bit 1 - Pin F1
            #[inline(always)]
            pub fn pf1(&mut self) -> PF1_W<'_, DDRF_SPEC> {
                PF1_W::new(self, 1)
            }
            ///Bit 2 - Pin F2
            #[inline(always)]
            pub fn pf2(&mut self) -> PF2_W<'_, DDRF_SPEC> {
                PF2_W::new(self, 2)
            }
            ///Bit 3 - Pin F3
            #[inline(always)]
            pub fn pf3(&mut self) -> PF3_W<'_, DDRF_SPEC> {
                PF3_W::new(self, 3)
            }
            ///Bit 4 - Pin F4
            #[inline(always)]
            pub fn pf4(&mut self) -> PF4_W<'_, DDRF_SPEC> {
                PF4_W::new(self, 4)
            }
            ///Bit 5 - Pin F5
            #[inline(always)]
            pub fn pf5(&mut self) -> PF5_W<'_, DDRF_SPEC> {
                PF5_W::new(self, 5)
            }
            ///Bit 6 - Pin F6
            #[inline(always)]
            pub fn pf6(&mut self) -> PF6_W<'_, DDRF_SPEC> {
                PF6_W::new(self, 6)
            }
            ///Bit 7 - Pin F7
            #[inline(always)]
            pub fn pf7(&mut self) -> PF7_W<'_, DDRF_SPEC> {
                PF7_W::new(self, 7)
            }
        }
        /**Data Direction Register, Port F

You can [`read`](crate::Reg::read) this register and get [`ddrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRF_SPEC;
        impl crate::RegisterSpec for DDRF_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddrf::R`](R) reader structure
        impl crate::Readable for DDRF_SPEC {}
        ///`write(|w| ..)` method takes [`ddrf::W`](W) writer structure
        impl crate::Writable for DDRF_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRF to value 0
        impl crate::Resettable for DDRF_SPEC {}
    }
    /**PINF (rw) register accessor: Input Pins, Port F

You can [`read`](crate::Reg::read) this register and get [`pinf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pinf`] module*/
    pub type PINF = crate::Reg<pinf::PINF_SPEC>;
    ///Input Pins, Port F
    pub mod pinf {
        ///Register `PINF` reader
        pub type R = crate::R<PINF_SPEC>;
        ///Register `PINF` writer
        pub type W = crate::W<PINF_SPEC>;
        ///Field `PF0` reader - Pin F0
        pub type PF0_R = crate::BitReader;
        ///Field `PF0` writer - Pin F0
        pub type PF0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF1` reader - Pin F1
        pub type PF1_R = crate::BitReader;
        ///Field `PF1` writer - Pin F1
        pub type PF1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF2` reader - Pin F2
        pub type PF2_R = crate::BitReader;
        ///Field `PF2` writer - Pin F2
        pub type PF2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF3` reader - Pin F3
        pub type PF3_R = crate::BitReader;
        ///Field `PF3` writer - Pin F3
        pub type PF3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF4` reader - Pin F4
        pub type PF4_R = crate::BitReader;
        ///Field `PF4` writer - Pin F4
        pub type PF4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF5` reader - Pin F5
        pub type PF5_R = crate::BitReader;
        ///Field `PF5` writer - Pin F5
        pub type PF5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF6` reader - Pin F6
        pub type PF6_R = crate::BitReader;
        ///Field `PF6` writer - Pin F6
        pub type PF6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF7` reader - Pin F7
        pub type PF7_R = crate::BitReader;
        ///Field `PF7` writer - Pin F7
        pub type PF7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin F0
            #[inline(always)]
            pub fn pf0(&self) -> PF0_R {
                PF0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin F1
            #[inline(always)]
            pub fn pf1(&self) -> PF1_R {
                PF1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin F2
            #[inline(always)]
            pub fn pf2(&self) -> PF2_R {
                PF2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin F3
            #[inline(always)]
            pub fn pf3(&self) -> PF3_R {
                PF3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin F4
            #[inline(always)]
            pub fn pf4(&self) -> PF4_R {
                PF4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin F5
            #[inline(always)]
            pub fn pf5(&self) -> PF5_R {
                PF5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin F6
            #[inline(always)]
            pub fn pf6(&self) -> PF6_R {
                PF6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin F7
            #[inline(always)]
            pub fn pf7(&self) -> PF7_R {
                PF7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin F0
            #[inline(always)]
            pub fn pf0(&mut self) -> PF0_W<'_, PINF_SPEC> {
                PF0_W::new(self, 0)
            }
            ///Bit 1 - Pin F1
            #[inline(always)]
            pub fn pf1(&mut self) -> PF1_W<'_, PINF_SPEC> {
                PF1_W::new(self, 1)
            }
            ///Bit 2 - Pin F2
            #[inline(always)]
            pub fn pf2(&mut self) -> PF2_W<'_, PINF_SPEC> {
                PF2_W::new(self, 2)
            }
            ///Bit 3 - Pin F3
            #[inline(always)]
            pub fn pf3(&mut self) -> PF3_W<'_, PINF_SPEC> {
                PF3_W::new(self, 3)
            }
            ///Bit 4 - Pin F4
            #[inline(always)]
            pub fn pf4(&mut self) -> PF4_W<'_, PINF_SPEC> {
                PF4_W::new(self, 4)
            }
            ///Bit 5 - Pin F5
            #[inline(always)]
            pub fn pf5(&mut self) -> PF5_W<'_, PINF_SPEC> {
                PF5_W::new(self, 5)
            }
            ///Bit 6 - Pin F6
            #[inline(always)]
            pub fn pf6(&mut self) -> PF6_W<'_, PINF_SPEC> {
                PF6_W::new(self, 6)
            }
            ///Bit 7 - Pin F7
            #[inline(always)]
            pub fn pf7(&mut self) -> PF7_W<'_, PINF_SPEC> {
                PF7_W::new(self, 7)
            }
        }
        /**Input Pins, Port F

You can [`read`](crate::Reg::read) this register and get [`pinf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PINF_SPEC;
        impl crate::RegisterSpec for PINF_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pinf::R`](R) reader structure
        impl crate::Readable for PINF_SPEC {}
        ///`write(|w| ..)` method takes [`pinf::W`](W) writer structure
        impl crate::Writable for PINF_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PINF to value 0
        impl crate::Resettable for PINF_SPEC {}
    }
    /**PORTF (rw) register accessor: Data Register, Port F

You can [`read`](crate::Reg::read) this register and get [`portf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@portf`] module*/
    pub type PORTF = crate::Reg<portf::PORTF_SPEC>;
    ///Data Register, Port F
    pub mod portf {
        ///Register `PORTF` reader
        pub type R = crate::R<PORTF_SPEC>;
        ///Register `PORTF` writer
        pub type W = crate::W<PORTF_SPEC>;
        ///Field `PF0` reader - Pin F0
        pub type PF0_R = crate::BitReader;
        ///Field `PF0` writer - Pin F0
        pub type PF0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF1` reader - Pin F1
        pub type PF1_R = crate::BitReader;
        ///Field `PF1` writer - Pin F1
        pub type PF1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF2` reader - Pin F2
        pub type PF2_R = crate::BitReader;
        ///Field `PF2` writer - Pin F2
        pub type PF2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF3` reader - Pin F3
        pub type PF3_R = crate::BitReader;
        ///Field `PF3` writer - Pin F3
        pub type PF3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF4` reader - Pin F4
        pub type PF4_R = crate::BitReader;
        ///Field `PF4` writer - Pin F4
        pub type PF4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF5` reader - Pin F5
        pub type PF5_R = crate::BitReader;
        ///Field `PF5` writer - Pin F5
        pub type PF5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF6` reader - Pin F6
        pub type PF6_R = crate::BitReader;
        ///Field `PF6` writer - Pin F6
        pub type PF6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PF7` reader - Pin F7
        pub type PF7_R = crate::BitReader;
        ///Field `PF7` writer - Pin F7
        pub type PF7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin F0
            #[inline(always)]
            pub fn pf0(&self) -> PF0_R {
                PF0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin F1
            #[inline(always)]
            pub fn pf1(&self) -> PF1_R {
                PF1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin F2
            #[inline(always)]
            pub fn pf2(&self) -> PF2_R {
                PF2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin F3
            #[inline(always)]
            pub fn pf3(&self) -> PF3_R {
                PF3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin F4
            #[inline(always)]
            pub fn pf4(&self) -> PF4_R {
                PF4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin F5
            #[inline(always)]
            pub fn pf5(&self) -> PF5_R {
                PF5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin F6
            #[inline(always)]
            pub fn pf6(&self) -> PF6_R {
                PF6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin F7
            #[inline(always)]
            pub fn pf7(&self) -> PF7_R {
                PF7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin F0
            #[inline(always)]
            pub fn pf0(&mut self) -> PF0_W<'_, PORTF_SPEC> {
                PF0_W::new(self, 0)
            }
            ///Bit 1 - Pin F1
            #[inline(always)]
            pub fn pf1(&mut self) -> PF1_W<'_, PORTF_SPEC> {
                PF1_W::new(self, 1)
            }
            ///Bit 2 - Pin F2
            #[inline(always)]
            pub fn pf2(&mut self) -> PF2_W<'_, PORTF_SPEC> {
                PF2_W::new(self, 2)
            }
            ///Bit 3 - Pin F3
            #[inline(always)]
            pub fn pf3(&mut self) -> PF3_W<'_, PORTF_SPEC> {
                PF3_W::new(self, 3)
            }
            ///Bit 4 - Pin F4
            #[inline(always)]
            pub fn pf4(&mut self) -> PF4_W<'_, PORTF_SPEC> {
                PF4_W::new(self, 4)
            }
            ///Bit 5 - Pin F5
            #[inline(always)]
            pub fn pf5(&mut self) -> PF5_W<'_, PORTF_SPEC> {
                PF5_W::new(self, 5)
            }
            ///Bit 6 - Pin F6
            #[inline(always)]
            pub fn pf6(&mut self) -> PF6_W<'_, PORTF_SPEC> {
                PF6_W::new(self, 6)
            }
            ///Bit 7 - Pin F7
            #[inline(always)]
            pub fn pf7(&mut self) -> PF7_W<'_, PORTF_SPEC> {
                PF7_W::new(self, 7)
            }
        }
        /**Data Register, Port F

You can [`read`](crate::Reg::read) this register and get [`portf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTF_SPEC;
        impl crate::RegisterSpec for PORTF_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`portf::R`](R) reader structure
        impl crate::Readable for PORTF_SPEC {}
        ///`write(|w| ..)` method takes [`portf::W`](W) writer structure
        impl crate::Writable for PORTF_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTF to value 0
        impl crate::Resettable for PORTF_SPEC {}
    }
}
///I/O Port
pub type PORTG = crate::Periph<portg::RegisterBlock, 0x32>;
impl core::fmt::Debug for PORTG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTG").finish()
    }
}
///I/O Port
pub mod portg {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        ping: PING,
        ddrg: DDRG,
        portg: PORTG,
    }
    impl RegisterBlock {
        ///0x00 - Input Pins, Port G
        #[inline(always)]
        pub const fn ping(&self) -> &PING {
            &self.ping
        }
        ///0x01 - Data Direction Register, Port G
        #[inline(always)]
        pub const fn ddrg(&self) -> &DDRG {
            &self.ddrg
        }
        ///0x02 - Data Register, Port G
        #[inline(always)]
        pub const fn portg(&self) -> &PORTG {
            &self.portg
        }
    }
    /**DDRG (rw) register accessor: Data Direction Register, Port G

You can [`read`](crate::Reg::read) this register and get [`ddrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddrg`] module*/
    pub type DDRG = crate::Reg<ddrg::DDRG_SPEC>;
    ///Data Direction Register, Port G
    pub mod ddrg {
        ///Register `DDRG` reader
        pub type R = crate::R<DDRG_SPEC>;
        ///Register `DDRG` writer
        pub type W = crate::W<DDRG_SPEC>;
        ///Field `PG0` reader - Pin G0
        pub type PG0_R = crate::BitReader;
        ///Field `PG0` writer - Pin G0
        pub type PG0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG1` reader - Pin G1
        pub type PG1_R = crate::BitReader;
        ///Field `PG1` writer - Pin G1
        pub type PG1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG2` reader - Pin G2
        pub type PG2_R = crate::BitReader;
        ///Field `PG2` writer - Pin G2
        pub type PG2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG3` reader - Pin G3
        pub type PG3_R = crate::BitReader;
        ///Field `PG3` writer - Pin G3
        pub type PG3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG4` reader - Pin G4
        pub type PG4_R = crate::BitReader;
        ///Field `PG4` writer - Pin G4
        pub type PG4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG5` reader - Pin G5
        pub type PG5_R = crate::BitReader;
        ///Field `PG5` writer - Pin G5
        pub type PG5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG6` reader - Pin G6
        pub type PG6_R = crate::BitReader;
        ///Field `PG6` writer - Pin G6
        pub type PG6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG7` reader - Pin G7
        pub type PG7_R = crate::BitReader;
        ///Field `PG7` writer - Pin G7
        pub type PG7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin G0
            #[inline(always)]
            pub fn pg0(&self) -> PG0_R {
                PG0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin G1
            #[inline(always)]
            pub fn pg1(&self) -> PG1_R {
                PG1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin G2
            #[inline(always)]
            pub fn pg2(&self) -> PG2_R {
                PG2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin G3
            #[inline(always)]
            pub fn pg3(&self) -> PG3_R {
                PG3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin G4
            #[inline(always)]
            pub fn pg4(&self) -> PG4_R {
                PG4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin G5
            #[inline(always)]
            pub fn pg5(&self) -> PG5_R {
                PG5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin G6
            #[inline(always)]
            pub fn pg6(&self) -> PG6_R {
                PG6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin G7
            #[inline(always)]
            pub fn pg7(&self) -> PG7_R {
                PG7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin G0
            #[inline(always)]
            pub fn pg0(&mut self) -> PG0_W<'_, DDRG_SPEC> {
                PG0_W::new(self, 0)
            }
            ///Bit 1 - Pin G1
            #[inline(always)]
            pub fn pg1(&mut self) -> PG1_W<'_, DDRG_SPEC> {
                PG1_W::new(self, 1)
            }
            ///Bit 2 - Pin G2
            #[inline(always)]
            pub fn pg2(&mut self) -> PG2_W<'_, DDRG_SPEC> {
                PG2_W::new(self, 2)
            }
            ///Bit 3 - Pin G3
            #[inline(always)]
            pub fn pg3(&mut self) -> PG3_W<'_, DDRG_SPEC> {
                PG3_W::new(self, 3)
            }
            ///Bit 4 - Pin G4
            #[inline(always)]
            pub fn pg4(&mut self) -> PG4_W<'_, DDRG_SPEC> {
                PG4_W::new(self, 4)
            }
            ///Bit 5 - Pin G5
            #[inline(always)]
            pub fn pg5(&mut self) -> PG5_W<'_, DDRG_SPEC> {
                PG5_W::new(self, 5)
            }
            ///Bit 6 - Pin G6
            #[inline(always)]
            pub fn pg6(&mut self) -> PG6_W<'_, DDRG_SPEC> {
                PG6_W::new(self, 6)
            }
            ///Bit 7 - Pin G7
            #[inline(always)]
            pub fn pg7(&mut self) -> PG7_W<'_, DDRG_SPEC> {
                PG7_W::new(self, 7)
            }
        }
        /**Data Direction Register, Port G

You can [`read`](crate::Reg::read) this register and get [`ddrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRG_SPEC;
        impl crate::RegisterSpec for DDRG_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddrg::R`](R) reader structure
        impl crate::Readable for DDRG_SPEC {}
        ///`write(|w| ..)` method takes [`ddrg::W`](W) writer structure
        impl crate::Writable for DDRG_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRG to value 0
        impl crate::Resettable for DDRG_SPEC {}
    }
    /**PING (rw) register accessor: Input Pins, Port G

You can [`read`](crate::Reg::read) this register and get [`ping::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ping::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ping`] module*/
    pub type PING = crate::Reg<ping::PING_SPEC>;
    ///Input Pins, Port G
    pub mod ping {
        ///Register `PING` reader
        pub type R = crate::R<PING_SPEC>;
        ///Register `PING` writer
        pub type W = crate::W<PING_SPEC>;
        ///Field `PG0` reader - Pin G0
        pub type PG0_R = crate::BitReader;
        ///Field `PG0` writer - Pin G0
        pub type PG0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG1` reader - Pin G1
        pub type PG1_R = crate::BitReader;
        ///Field `PG1` writer - Pin G1
        pub type PG1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG2` reader - Pin G2
        pub type PG2_R = crate::BitReader;
        ///Field `PG2` writer - Pin G2
        pub type PG2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG3` reader - Pin G3
        pub type PG3_R = crate::BitReader;
        ///Field `PG3` writer - Pin G3
        pub type PG3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG4` reader - Pin G4
        pub type PG4_R = crate::BitReader;
        ///Field `PG4` writer - Pin G4
        pub type PG4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG5` reader - Pin G5
        pub type PG5_R = crate::BitReader;
        ///Field `PG5` writer - Pin G5
        pub type PG5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG6` reader - Pin G6
        pub type PG6_R = crate::BitReader;
        ///Field `PG6` writer - Pin G6
        pub type PG6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG7` reader - Pin G7
        pub type PG7_R = crate::BitReader;
        ///Field `PG7` writer - Pin G7
        pub type PG7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin G0
            #[inline(always)]
            pub fn pg0(&self) -> PG0_R {
                PG0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin G1
            #[inline(always)]
            pub fn pg1(&self) -> PG1_R {
                PG1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin G2
            #[inline(always)]
            pub fn pg2(&self) -> PG2_R {
                PG2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin G3
            #[inline(always)]
            pub fn pg3(&self) -> PG3_R {
                PG3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin G4
            #[inline(always)]
            pub fn pg4(&self) -> PG4_R {
                PG4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin G5
            #[inline(always)]
            pub fn pg5(&self) -> PG5_R {
                PG5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin G6
            #[inline(always)]
            pub fn pg6(&self) -> PG6_R {
                PG6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin G7
            #[inline(always)]
            pub fn pg7(&self) -> PG7_R {
                PG7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin G0
            #[inline(always)]
            pub fn pg0(&mut self) -> PG0_W<'_, PING_SPEC> {
                PG0_W::new(self, 0)
            }
            ///Bit 1 - Pin G1
            #[inline(always)]
            pub fn pg1(&mut self) -> PG1_W<'_, PING_SPEC> {
                PG1_W::new(self, 1)
            }
            ///Bit 2 - Pin G2
            #[inline(always)]
            pub fn pg2(&mut self) -> PG2_W<'_, PING_SPEC> {
                PG2_W::new(self, 2)
            }
            ///Bit 3 - Pin G3
            #[inline(always)]
            pub fn pg3(&mut self) -> PG3_W<'_, PING_SPEC> {
                PG3_W::new(self, 3)
            }
            ///Bit 4 - Pin G4
            #[inline(always)]
            pub fn pg4(&mut self) -> PG4_W<'_, PING_SPEC> {
                PG4_W::new(self, 4)
            }
            ///Bit 5 - Pin G5
            #[inline(always)]
            pub fn pg5(&mut self) -> PG5_W<'_, PING_SPEC> {
                PG5_W::new(self, 5)
            }
            ///Bit 6 - Pin G6
            #[inline(always)]
            pub fn pg6(&mut self) -> PG6_W<'_, PING_SPEC> {
                PG6_W::new(self, 6)
            }
            ///Bit 7 - Pin G7
            #[inline(always)]
            pub fn pg7(&mut self) -> PG7_W<'_, PING_SPEC> {
                PG7_W::new(self, 7)
            }
        }
        /**Input Pins, Port G

You can [`read`](crate::Reg::read) this register and get [`ping::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ping::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PING_SPEC;
        impl crate::RegisterSpec for PING_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ping::R`](R) reader structure
        impl crate::Readable for PING_SPEC {}
        ///`write(|w| ..)` method takes [`ping::W`](W) writer structure
        impl crate::Writable for PING_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PING to value 0
        impl crate::Resettable for PING_SPEC {}
    }
    /**PORTG (rw) register accessor: Data Register, Port G

You can [`read`](crate::Reg::read) this register and get [`portg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@portg`] module*/
    pub type PORTG = crate::Reg<portg::PORTG_SPEC>;
    ///Data Register, Port G
    pub mod portg {
        ///Register `PORTG` reader
        pub type R = crate::R<PORTG_SPEC>;
        ///Register `PORTG` writer
        pub type W = crate::W<PORTG_SPEC>;
        ///Field `PG0` reader - Pin G0
        pub type PG0_R = crate::BitReader;
        ///Field `PG0` writer - Pin G0
        pub type PG0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG1` reader - Pin G1
        pub type PG1_R = crate::BitReader;
        ///Field `PG1` writer - Pin G1
        pub type PG1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG2` reader - Pin G2
        pub type PG2_R = crate::BitReader;
        ///Field `PG2` writer - Pin G2
        pub type PG2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG3` reader - Pin G3
        pub type PG3_R = crate::BitReader;
        ///Field `PG3` writer - Pin G3
        pub type PG3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG4` reader - Pin G4
        pub type PG4_R = crate::BitReader;
        ///Field `PG4` writer - Pin G4
        pub type PG4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG5` reader - Pin G5
        pub type PG5_R = crate::BitReader;
        ///Field `PG5` writer - Pin G5
        pub type PG5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG6` reader - Pin G6
        pub type PG6_R = crate::BitReader;
        ///Field `PG6` writer - Pin G6
        pub type PG6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PG7` reader - Pin G7
        pub type PG7_R = crate::BitReader;
        ///Field `PG7` writer - Pin G7
        pub type PG7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin G0
            #[inline(always)]
            pub fn pg0(&self) -> PG0_R {
                PG0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin G1
            #[inline(always)]
            pub fn pg1(&self) -> PG1_R {
                PG1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin G2
            #[inline(always)]
            pub fn pg2(&self) -> PG2_R {
                PG2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin G3
            #[inline(always)]
            pub fn pg3(&self) -> PG3_R {
                PG3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin G4
            #[inline(always)]
            pub fn pg4(&self) -> PG4_R {
                PG4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin G5
            #[inline(always)]
            pub fn pg5(&self) -> PG5_R {
                PG5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin G6
            #[inline(always)]
            pub fn pg6(&self) -> PG6_R {
                PG6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin G7
            #[inline(always)]
            pub fn pg7(&self) -> PG7_R {
                PG7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin G0
            #[inline(always)]
            pub fn pg0(&mut self) -> PG0_W<'_, PORTG_SPEC> {
                PG0_W::new(self, 0)
            }
            ///Bit 1 - Pin G1
            #[inline(always)]
            pub fn pg1(&mut self) -> PG1_W<'_, PORTG_SPEC> {
                PG1_W::new(self, 1)
            }
            ///Bit 2 - Pin G2
            #[inline(always)]
            pub fn pg2(&mut self) -> PG2_W<'_, PORTG_SPEC> {
                PG2_W::new(self, 2)
            }
            ///Bit 3 - Pin G3
            #[inline(always)]
            pub fn pg3(&mut self) -> PG3_W<'_, PORTG_SPEC> {
                PG3_W::new(self, 3)
            }
            ///Bit 4 - Pin G4
            #[inline(always)]
            pub fn pg4(&mut self) -> PG4_W<'_, PORTG_SPEC> {
                PG4_W::new(self, 4)
            }
            ///Bit 5 - Pin G5
            #[inline(always)]
            pub fn pg5(&mut self) -> PG5_W<'_, PORTG_SPEC> {
                PG5_W::new(self, 5)
            }
            ///Bit 6 - Pin G6
            #[inline(always)]
            pub fn pg6(&mut self) -> PG6_W<'_, PORTG_SPEC> {
                PG6_W::new(self, 6)
            }
            ///Bit 7 - Pin G7
            #[inline(always)]
            pub fn pg7(&mut self) -> PG7_W<'_, PORTG_SPEC> {
                PG7_W::new(self, 7)
            }
        }
        /**Data Register, Port G

You can [`read`](crate::Reg::read) this register and get [`portg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTG_SPEC;
        impl crate::RegisterSpec for PORTG_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`portg::R`](R) reader structure
        impl crate::Readable for PORTG_SPEC {}
        ///`write(|w| ..)` method takes [`portg::W`](W) writer structure
        impl crate::Writable for PORTG_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTG to value 0
        impl crate::Resettable for PORTG_SPEC {}
    }
}
///I/O Port
pub type PORTH = crate::Periph<porth::RegisterBlock, 0x0100>;
impl core::fmt::Debug for PORTH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTH").finish()
    }
}
///I/O Port
pub mod porth {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pinh: PINH,
        ddrh: DDRH,
        porth: PORTH,
    }
    impl RegisterBlock {
        ///0x00 - PORT H Input Pins
        #[inline(always)]
        pub const fn pinh(&self) -> &PINH {
            &self.pinh
        }
        ///0x01 - PORT H Data Direction Register
        #[inline(always)]
        pub const fn ddrh(&self) -> &DDRH {
            &self.ddrh
        }
        ///0x02 - PORT H Data Register
        #[inline(always)]
        pub const fn porth(&self) -> &PORTH {
            &self.porth
        }
    }
    /**DDRH (rw) register accessor: PORT H Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddrh`] module*/
    pub type DDRH = crate::Reg<ddrh::DDRH_SPEC>;
    ///PORT H Data Direction Register
    pub mod ddrh {
        ///Register `DDRH` reader
        pub type R = crate::R<DDRH_SPEC>;
        ///Register `DDRH` writer
        pub type W = crate::W<DDRH_SPEC>;
        ///Field `PH0` reader - Pin H0
        pub type PH0_R = crate::BitReader;
        ///Field `PH0` writer - Pin H0
        pub type PH0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH1` reader - Pin H1
        pub type PH1_R = crate::BitReader;
        ///Field `PH1` writer - Pin H1
        pub type PH1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH2` reader - Pin H2
        pub type PH2_R = crate::BitReader;
        ///Field `PH2` writer - Pin H2
        pub type PH2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH3` reader - Pin H3
        pub type PH3_R = crate::BitReader;
        ///Field `PH3` writer - Pin H3
        pub type PH3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH4` reader - Pin H4
        pub type PH4_R = crate::BitReader;
        ///Field `PH4` writer - Pin H4
        pub type PH4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH5` reader - Pin H5
        pub type PH5_R = crate::BitReader;
        ///Field `PH5` writer - Pin H5
        pub type PH5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH6` reader - Pin H6
        pub type PH6_R = crate::BitReader;
        ///Field `PH6` writer - Pin H6
        pub type PH6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH7` reader - Pin H7
        pub type PH7_R = crate::BitReader;
        ///Field `PH7` writer - Pin H7
        pub type PH7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin H0
            #[inline(always)]
            pub fn ph0(&self) -> PH0_R {
                PH0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin H1
            #[inline(always)]
            pub fn ph1(&self) -> PH1_R {
                PH1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin H2
            #[inline(always)]
            pub fn ph2(&self) -> PH2_R {
                PH2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin H3
            #[inline(always)]
            pub fn ph3(&self) -> PH3_R {
                PH3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin H4
            #[inline(always)]
            pub fn ph4(&self) -> PH4_R {
                PH4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin H5
            #[inline(always)]
            pub fn ph5(&self) -> PH5_R {
                PH5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin H6
            #[inline(always)]
            pub fn ph6(&self) -> PH6_R {
                PH6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin H7
            #[inline(always)]
            pub fn ph7(&self) -> PH7_R {
                PH7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin H0
            #[inline(always)]
            pub fn ph0(&mut self) -> PH0_W<'_, DDRH_SPEC> {
                PH0_W::new(self, 0)
            }
            ///Bit 1 - Pin H1
            #[inline(always)]
            pub fn ph1(&mut self) -> PH1_W<'_, DDRH_SPEC> {
                PH1_W::new(self, 1)
            }
            ///Bit 2 - Pin H2
            #[inline(always)]
            pub fn ph2(&mut self) -> PH2_W<'_, DDRH_SPEC> {
                PH2_W::new(self, 2)
            }
            ///Bit 3 - Pin H3
            #[inline(always)]
            pub fn ph3(&mut self) -> PH3_W<'_, DDRH_SPEC> {
                PH3_W::new(self, 3)
            }
            ///Bit 4 - Pin H4
            #[inline(always)]
            pub fn ph4(&mut self) -> PH4_W<'_, DDRH_SPEC> {
                PH4_W::new(self, 4)
            }
            ///Bit 5 - Pin H5
            #[inline(always)]
            pub fn ph5(&mut self) -> PH5_W<'_, DDRH_SPEC> {
                PH5_W::new(self, 5)
            }
            ///Bit 6 - Pin H6
            #[inline(always)]
            pub fn ph6(&mut self) -> PH6_W<'_, DDRH_SPEC> {
                PH6_W::new(self, 6)
            }
            ///Bit 7 - Pin H7
            #[inline(always)]
            pub fn ph7(&mut self) -> PH7_W<'_, DDRH_SPEC> {
                PH7_W::new(self, 7)
            }
        }
        /**PORT H Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRH_SPEC;
        impl crate::RegisterSpec for DDRH_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddrh::R`](R) reader structure
        impl crate::Readable for DDRH_SPEC {}
        ///`write(|w| ..)` method takes [`ddrh::W`](W) writer structure
        impl crate::Writable for DDRH_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRH to value 0
        impl crate::Resettable for DDRH_SPEC {}
    }
    /**PINH (rw) register accessor: PORT H Input Pins

You can [`read`](crate::Reg::read) this register and get [`pinh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pinh`] module*/
    pub type PINH = crate::Reg<pinh::PINH_SPEC>;
    ///PORT H Input Pins
    pub mod pinh {
        ///Register `PINH` reader
        pub type R = crate::R<PINH_SPEC>;
        ///Register `PINH` writer
        pub type W = crate::W<PINH_SPEC>;
        ///Field `PH0` reader - Pin H0
        pub type PH0_R = crate::BitReader;
        ///Field `PH0` writer - Pin H0
        pub type PH0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH1` reader - Pin H1
        pub type PH1_R = crate::BitReader;
        ///Field `PH1` writer - Pin H1
        pub type PH1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH2` reader - Pin H2
        pub type PH2_R = crate::BitReader;
        ///Field `PH2` writer - Pin H2
        pub type PH2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH3` reader - Pin H3
        pub type PH3_R = crate::BitReader;
        ///Field `PH3` writer - Pin H3
        pub type PH3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH4` reader - Pin H4
        pub type PH4_R = crate::BitReader;
        ///Field `PH4` writer - Pin H4
        pub type PH4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH5` reader - Pin H5
        pub type PH5_R = crate::BitReader;
        ///Field `PH5` writer - Pin H5
        pub type PH5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH6` reader - Pin H6
        pub type PH6_R = crate::BitReader;
        ///Field `PH6` writer - Pin H6
        pub type PH6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH7` reader - Pin H7
        pub type PH7_R = crate::BitReader;
        ///Field `PH7` writer - Pin H7
        pub type PH7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin H0
            #[inline(always)]
            pub fn ph0(&self) -> PH0_R {
                PH0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin H1
            #[inline(always)]
            pub fn ph1(&self) -> PH1_R {
                PH1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin H2
            #[inline(always)]
            pub fn ph2(&self) -> PH2_R {
                PH2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin H3
            #[inline(always)]
            pub fn ph3(&self) -> PH3_R {
                PH3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin H4
            #[inline(always)]
            pub fn ph4(&self) -> PH4_R {
                PH4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin H5
            #[inline(always)]
            pub fn ph5(&self) -> PH5_R {
                PH5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin H6
            #[inline(always)]
            pub fn ph6(&self) -> PH6_R {
                PH6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin H7
            #[inline(always)]
            pub fn ph7(&self) -> PH7_R {
                PH7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin H0
            #[inline(always)]
            pub fn ph0(&mut self) -> PH0_W<'_, PINH_SPEC> {
                PH0_W::new(self, 0)
            }
            ///Bit 1 - Pin H1
            #[inline(always)]
            pub fn ph1(&mut self) -> PH1_W<'_, PINH_SPEC> {
                PH1_W::new(self, 1)
            }
            ///Bit 2 - Pin H2
            #[inline(always)]
            pub fn ph2(&mut self) -> PH2_W<'_, PINH_SPEC> {
                PH2_W::new(self, 2)
            }
            ///Bit 3 - Pin H3
            #[inline(always)]
            pub fn ph3(&mut self) -> PH3_W<'_, PINH_SPEC> {
                PH3_W::new(self, 3)
            }
            ///Bit 4 - Pin H4
            #[inline(always)]
            pub fn ph4(&mut self) -> PH4_W<'_, PINH_SPEC> {
                PH4_W::new(self, 4)
            }
            ///Bit 5 - Pin H5
            #[inline(always)]
            pub fn ph5(&mut self) -> PH5_W<'_, PINH_SPEC> {
                PH5_W::new(self, 5)
            }
            ///Bit 6 - Pin H6
            #[inline(always)]
            pub fn ph6(&mut self) -> PH6_W<'_, PINH_SPEC> {
                PH6_W::new(self, 6)
            }
            ///Bit 7 - Pin H7
            #[inline(always)]
            pub fn ph7(&mut self) -> PH7_W<'_, PINH_SPEC> {
                PH7_W::new(self, 7)
            }
        }
        /**PORT H Input Pins

You can [`read`](crate::Reg::read) this register and get [`pinh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PINH_SPEC;
        impl crate::RegisterSpec for PINH_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pinh::R`](R) reader structure
        impl crate::Readable for PINH_SPEC {}
        ///`write(|w| ..)` method takes [`pinh::W`](W) writer structure
        impl crate::Writable for PINH_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PINH to value 0
        impl crate::Resettable for PINH_SPEC {}
    }
    /**PORTH (rw) register accessor: PORT H Data Register

You can [`read`](crate::Reg::read) this register and get [`porth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@porth`] module*/
    pub type PORTH = crate::Reg<porth::PORTH_SPEC>;
    ///PORT H Data Register
    pub mod porth {
        ///Register `PORTH` reader
        pub type R = crate::R<PORTH_SPEC>;
        ///Register `PORTH` writer
        pub type W = crate::W<PORTH_SPEC>;
        ///Field `PH0` reader - Pin H0
        pub type PH0_R = crate::BitReader;
        ///Field `PH0` writer - Pin H0
        pub type PH0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH1` reader - Pin H1
        pub type PH1_R = crate::BitReader;
        ///Field `PH1` writer - Pin H1
        pub type PH1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH2` reader - Pin H2
        pub type PH2_R = crate::BitReader;
        ///Field `PH2` writer - Pin H2
        pub type PH2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH3` reader - Pin H3
        pub type PH3_R = crate::BitReader;
        ///Field `PH3` writer - Pin H3
        pub type PH3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH4` reader - Pin H4
        pub type PH4_R = crate::BitReader;
        ///Field `PH4` writer - Pin H4
        pub type PH4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH5` reader - Pin H5
        pub type PH5_R = crate::BitReader;
        ///Field `PH5` writer - Pin H5
        pub type PH5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH6` reader - Pin H6
        pub type PH6_R = crate::BitReader;
        ///Field `PH6` writer - Pin H6
        pub type PH6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PH7` reader - Pin H7
        pub type PH7_R = crate::BitReader;
        ///Field `PH7` writer - Pin H7
        pub type PH7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin H0
            #[inline(always)]
            pub fn ph0(&self) -> PH0_R {
                PH0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin H1
            #[inline(always)]
            pub fn ph1(&self) -> PH1_R {
                PH1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin H2
            #[inline(always)]
            pub fn ph2(&self) -> PH2_R {
                PH2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin H3
            #[inline(always)]
            pub fn ph3(&self) -> PH3_R {
                PH3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin H4
            #[inline(always)]
            pub fn ph4(&self) -> PH4_R {
                PH4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin H5
            #[inline(always)]
            pub fn ph5(&self) -> PH5_R {
                PH5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin H6
            #[inline(always)]
            pub fn ph6(&self) -> PH6_R {
                PH6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin H7
            #[inline(always)]
            pub fn ph7(&self) -> PH7_R {
                PH7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin H0
            #[inline(always)]
            pub fn ph0(&mut self) -> PH0_W<'_, PORTH_SPEC> {
                PH0_W::new(self, 0)
            }
            ///Bit 1 - Pin H1
            #[inline(always)]
            pub fn ph1(&mut self) -> PH1_W<'_, PORTH_SPEC> {
                PH1_W::new(self, 1)
            }
            ///Bit 2 - Pin H2
            #[inline(always)]
            pub fn ph2(&mut self) -> PH2_W<'_, PORTH_SPEC> {
                PH2_W::new(self, 2)
            }
            ///Bit 3 - Pin H3
            #[inline(always)]
            pub fn ph3(&mut self) -> PH3_W<'_, PORTH_SPEC> {
                PH3_W::new(self, 3)
            }
            ///Bit 4 - Pin H4
            #[inline(always)]
            pub fn ph4(&mut self) -> PH4_W<'_, PORTH_SPEC> {
                PH4_W::new(self, 4)
            }
            ///Bit 5 - Pin H5
            #[inline(always)]
            pub fn ph5(&mut self) -> PH5_W<'_, PORTH_SPEC> {
                PH5_W::new(self, 5)
            }
            ///Bit 6 - Pin H6
            #[inline(always)]
            pub fn ph6(&mut self) -> PH6_W<'_, PORTH_SPEC> {
                PH6_W::new(self, 6)
            }
            ///Bit 7 - Pin H7
            #[inline(always)]
            pub fn ph7(&mut self) -> PH7_W<'_, PORTH_SPEC> {
                PH7_W::new(self, 7)
            }
        }
        /**PORT H Data Register

You can [`read`](crate::Reg::read) this register and get [`porth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTH_SPEC;
        impl crate::RegisterSpec for PORTH_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`porth::R`](R) reader structure
        impl crate::Readable for PORTH_SPEC {}
        ///`write(|w| ..)` method takes [`porth::W`](W) writer structure
        impl crate::Writable for PORTH_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTH to value 0
        impl crate::Resettable for PORTH_SPEC {}
    }
}
///I/O Port
pub type PORTJ = crate::Periph<portj::RegisterBlock, 0x0103>;
impl core::fmt::Debug for PORTJ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTJ").finish()
    }
}
///I/O Port
pub mod portj {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pinj: PINJ,
        ddrj: DDRJ,
        portj: PORTJ,
    }
    impl RegisterBlock {
        ///0x00 - PORT J Input Pins
        #[inline(always)]
        pub const fn pinj(&self) -> &PINJ {
            &self.pinj
        }
        ///0x01 - PORT J Data Direction Register
        #[inline(always)]
        pub const fn ddrj(&self) -> &DDRJ {
            &self.ddrj
        }
        ///0x02 - PORT J Data Register
        #[inline(always)]
        pub const fn portj(&self) -> &PORTJ {
            &self.portj
        }
    }
    /**DDRJ (rw) register accessor: PORT J Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddrj`] module*/
    pub type DDRJ = crate::Reg<ddrj::DDRJ_SPEC>;
    ///PORT J Data Direction Register
    pub mod ddrj {
        ///Register `DDRJ` reader
        pub type R = crate::R<DDRJ_SPEC>;
        ///Register `DDRJ` writer
        pub type W = crate::W<DDRJ_SPEC>;
        ///Field `PJ0` reader - Pin J0
        pub type PJ0_R = crate::BitReader;
        ///Field `PJ0` writer - Pin J0
        pub type PJ0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ1` reader - Pin J1
        pub type PJ1_R = crate::BitReader;
        ///Field `PJ1` writer - Pin J1
        pub type PJ1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ2` reader - Pin J2
        pub type PJ2_R = crate::BitReader;
        ///Field `PJ2` writer - Pin J2
        pub type PJ2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ3` reader - Pin J3
        pub type PJ3_R = crate::BitReader;
        ///Field `PJ3` writer - Pin J3
        pub type PJ3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ4` reader - Pin J4
        pub type PJ4_R = crate::BitReader;
        ///Field `PJ4` writer - Pin J4
        pub type PJ4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ5` reader - Pin J5
        pub type PJ5_R = crate::BitReader;
        ///Field `PJ5` writer - Pin J5
        pub type PJ5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ6` reader - Pin J6
        pub type PJ6_R = crate::BitReader;
        ///Field `PJ6` writer - Pin J6
        pub type PJ6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ7` reader - Pin J7
        pub type PJ7_R = crate::BitReader;
        ///Field `PJ7` writer - Pin J7
        pub type PJ7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin J0
            #[inline(always)]
            pub fn pj0(&self) -> PJ0_R {
                PJ0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin J1
            #[inline(always)]
            pub fn pj1(&self) -> PJ1_R {
                PJ1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin J2
            #[inline(always)]
            pub fn pj2(&self) -> PJ2_R {
                PJ2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin J3
            #[inline(always)]
            pub fn pj3(&self) -> PJ3_R {
                PJ3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin J4
            #[inline(always)]
            pub fn pj4(&self) -> PJ4_R {
                PJ4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin J5
            #[inline(always)]
            pub fn pj5(&self) -> PJ5_R {
                PJ5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin J6
            #[inline(always)]
            pub fn pj6(&self) -> PJ6_R {
                PJ6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin J7
            #[inline(always)]
            pub fn pj7(&self) -> PJ7_R {
                PJ7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin J0
            #[inline(always)]
            pub fn pj0(&mut self) -> PJ0_W<'_, DDRJ_SPEC> {
                PJ0_W::new(self, 0)
            }
            ///Bit 1 - Pin J1
            #[inline(always)]
            pub fn pj1(&mut self) -> PJ1_W<'_, DDRJ_SPEC> {
                PJ1_W::new(self, 1)
            }
            ///Bit 2 - Pin J2
            #[inline(always)]
            pub fn pj2(&mut self) -> PJ2_W<'_, DDRJ_SPEC> {
                PJ2_W::new(self, 2)
            }
            ///Bit 3 - Pin J3
            #[inline(always)]
            pub fn pj3(&mut self) -> PJ3_W<'_, DDRJ_SPEC> {
                PJ3_W::new(self, 3)
            }
            ///Bit 4 - Pin J4
            #[inline(always)]
            pub fn pj4(&mut self) -> PJ4_W<'_, DDRJ_SPEC> {
                PJ4_W::new(self, 4)
            }
            ///Bit 5 - Pin J5
            #[inline(always)]
            pub fn pj5(&mut self) -> PJ5_W<'_, DDRJ_SPEC> {
                PJ5_W::new(self, 5)
            }
            ///Bit 6 - Pin J6
            #[inline(always)]
            pub fn pj6(&mut self) -> PJ6_W<'_, DDRJ_SPEC> {
                PJ6_W::new(self, 6)
            }
            ///Bit 7 - Pin J7
            #[inline(always)]
            pub fn pj7(&mut self) -> PJ7_W<'_, DDRJ_SPEC> {
                PJ7_W::new(self, 7)
            }
        }
        /**PORT J Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRJ_SPEC;
        impl crate::RegisterSpec for DDRJ_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddrj::R`](R) reader structure
        impl crate::Readable for DDRJ_SPEC {}
        ///`write(|w| ..)` method takes [`ddrj::W`](W) writer structure
        impl crate::Writable for DDRJ_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRJ to value 0
        impl crate::Resettable for DDRJ_SPEC {}
    }
    /**PINJ (rw) register accessor: PORT J Input Pins

You can [`read`](crate::Reg::read) this register and get [`pinj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pinj`] module*/
    pub type PINJ = crate::Reg<pinj::PINJ_SPEC>;
    ///PORT J Input Pins
    pub mod pinj {
        ///Register `PINJ` reader
        pub type R = crate::R<PINJ_SPEC>;
        ///Register `PINJ` writer
        pub type W = crate::W<PINJ_SPEC>;
        ///Field `PJ0` reader - Pin J0
        pub type PJ0_R = crate::BitReader;
        ///Field `PJ0` writer - Pin J0
        pub type PJ0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ1` reader - Pin J1
        pub type PJ1_R = crate::BitReader;
        ///Field `PJ1` writer - Pin J1
        pub type PJ1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ2` reader - Pin J2
        pub type PJ2_R = crate::BitReader;
        ///Field `PJ2` writer - Pin J2
        pub type PJ2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ3` reader - Pin J3
        pub type PJ3_R = crate::BitReader;
        ///Field `PJ3` writer - Pin J3
        pub type PJ3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ4` reader - Pin J4
        pub type PJ4_R = crate::BitReader;
        ///Field `PJ4` writer - Pin J4
        pub type PJ4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ5` reader - Pin J5
        pub type PJ5_R = crate::BitReader;
        ///Field `PJ5` writer - Pin J5
        pub type PJ5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ6` reader - Pin J6
        pub type PJ6_R = crate::BitReader;
        ///Field `PJ6` writer - Pin J6
        pub type PJ6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ7` reader - Pin J7
        pub type PJ7_R = crate::BitReader;
        ///Field `PJ7` writer - Pin J7
        pub type PJ7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin J0
            #[inline(always)]
            pub fn pj0(&self) -> PJ0_R {
                PJ0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin J1
            #[inline(always)]
            pub fn pj1(&self) -> PJ1_R {
                PJ1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin J2
            #[inline(always)]
            pub fn pj2(&self) -> PJ2_R {
                PJ2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin J3
            #[inline(always)]
            pub fn pj3(&self) -> PJ3_R {
                PJ3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin J4
            #[inline(always)]
            pub fn pj4(&self) -> PJ4_R {
                PJ4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin J5
            #[inline(always)]
            pub fn pj5(&self) -> PJ5_R {
                PJ5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin J6
            #[inline(always)]
            pub fn pj6(&self) -> PJ6_R {
                PJ6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin J7
            #[inline(always)]
            pub fn pj7(&self) -> PJ7_R {
                PJ7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin J0
            #[inline(always)]
            pub fn pj0(&mut self) -> PJ0_W<'_, PINJ_SPEC> {
                PJ0_W::new(self, 0)
            }
            ///Bit 1 - Pin J1
            #[inline(always)]
            pub fn pj1(&mut self) -> PJ1_W<'_, PINJ_SPEC> {
                PJ1_W::new(self, 1)
            }
            ///Bit 2 - Pin J2
            #[inline(always)]
            pub fn pj2(&mut self) -> PJ2_W<'_, PINJ_SPEC> {
                PJ2_W::new(self, 2)
            }
            ///Bit 3 - Pin J3
            #[inline(always)]
            pub fn pj3(&mut self) -> PJ3_W<'_, PINJ_SPEC> {
                PJ3_W::new(self, 3)
            }
            ///Bit 4 - Pin J4
            #[inline(always)]
            pub fn pj4(&mut self) -> PJ4_W<'_, PINJ_SPEC> {
                PJ4_W::new(self, 4)
            }
            ///Bit 5 - Pin J5
            #[inline(always)]
            pub fn pj5(&mut self) -> PJ5_W<'_, PINJ_SPEC> {
                PJ5_W::new(self, 5)
            }
            ///Bit 6 - Pin J6
            #[inline(always)]
            pub fn pj6(&mut self) -> PJ6_W<'_, PINJ_SPEC> {
                PJ6_W::new(self, 6)
            }
            ///Bit 7 - Pin J7
            #[inline(always)]
            pub fn pj7(&mut self) -> PJ7_W<'_, PINJ_SPEC> {
                PJ7_W::new(self, 7)
            }
        }
        /**PORT J Input Pins

You can [`read`](crate::Reg::read) this register and get [`pinj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PINJ_SPEC;
        impl crate::RegisterSpec for PINJ_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pinj::R`](R) reader structure
        impl crate::Readable for PINJ_SPEC {}
        ///`write(|w| ..)` method takes [`pinj::W`](W) writer structure
        impl crate::Writable for PINJ_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PINJ to value 0
        impl crate::Resettable for PINJ_SPEC {}
    }
    /**PORTJ (rw) register accessor: PORT J Data Register

You can [`read`](crate::Reg::read) this register and get [`portj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@portj`] module*/
    pub type PORTJ = crate::Reg<portj::PORTJ_SPEC>;
    ///PORT J Data Register
    pub mod portj {
        ///Register `PORTJ` reader
        pub type R = crate::R<PORTJ_SPEC>;
        ///Register `PORTJ` writer
        pub type W = crate::W<PORTJ_SPEC>;
        ///Field `PJ0` reader - Pin J0
        pub type PJ0_R = crate::BitReader;
        ///Field `PJ0` writer - Pin J0
        pub type PJ0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ1` reader - Pin J1
        pub type PJ1_R = crate::BitReader;
        ///Field `PJ1` writer - Pin J1
        pub type PJ1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ2` reader - Pin J2
        pub type PJ2_R = crate::BitReader;
        ///Field `PJ2` writer - Pin J2
        pub type PJ2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ3` reader - Pin J3
        pub type PJ3_R = crate::BitReader;
        ///Field `PJ3` writer - Pin J3
        pub type PJ3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ4` reader - Pin J4
        pub type PJ4_R = crate::BitReader;
        ///Field `PJ4` writer - Pin J4
        pub type PJ4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ5` reader - Pin J5
        pub type PJ5_R = crate::BitReader;
        ///Field `PJ5` writer - Pin J5
        pub type PJ5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ6` reader - Pin J6
        pub type PJ6_R = crate::BitReader;
        ///Field `PJ6` writer - Pin J6
        pub type PJ6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PJ7` reader - Pin J7
        pub type PJ7_R = crate::BitReader;
        ///Field `PJ7` writer - Pin J7
        pub type PJ7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin J0
            #[inline(always)]
            pub fn pj0(&self) -> PJ0_R {
                PJ0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin J1
            #[inline(always)]
            pub fn pj1(&self) -> PJ1_R {
                PJ1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin J2
            #[inline(always)]
            pub fn pj2(&self) -> PJ2_R {
                PJ2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin J3
            #[inline(always)]
            pub fn pj3(&self) -> PJ3_R {
                PJ3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin J4
            #[inline(always)]
            pub fn pj4(&self) -> PJ4_R {
                PJ4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin J5
            #[inline(always)]
            pub fn pj5(&self) -> PJ5_R {
                PJ5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin J6
            #[inline(always)]
            pub fn pj6(&self) -> PJ6_R {
                PJ6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin J7
            #[inline(always)]
            pub fn pj7(&self) -> PJ7_R {
                PJ7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin J0
            #[inline(always)]
            pub fn pj0(&mut self) -> PJ0_W<'_, PORTJ_SPEC> {
                PJ0_W::new(self, 0)
            }
            ///Bit 1 - Pin J1
            #[inline(always)]
            pub fn pj1(&mut self) -> PJ1_W<'_, PORTJ_SPEC> {
                PJ1_W::new(self, 1)
            }
            ///Bit 2 - Pin J2
            #[inline(always)]
            pub fn pj2(&mut self) -> PJ2_W<'_, PORTJ_SPEC> {
                PJ2_W::new(self, 2)
            }
            ///Bit 3 - Pin J3
            #[inline(always)]
            pub fn pj3(&mut self) -> PJ3_W<'_, PORTJ_SPEC> {
                PJ3_W::new(self, 3)
            }
            ///Bit 4 - Pin J4
            #[inline(always)]
            pub fn pj4(&mut self) -> PJ4_W<'_, PORTJ_SPEC> {
                PJ4_W::new(self, 4)
            }
            ///Bit 5 - Pin J5
            #[inline(always)]
            pub fn pj5(&mut self) -> PJ5_W<'_, PORTJ_SPEC> {
                PJ5_W::new(self, 5)
            }
            ///Bit 6 - Pin J6
            #[inline(always)]
            pub fn pj6(&mut self) -> PJ6_W<'_, PORTJ_SPEC> {
                PJ6_W::new(self, 6)
            }
            ///Bit 7 - Pin J7
            #[inline(always)]
            pub fn pj7(&mut self) -> PJ7_W<'_, PORTJ_SPEC> {
                PJ7_W::new(self, 7)
            }
        }
        /**PORT J Data Register

You can [`read`](crate::Reg::read) this register and get [`portj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTJ_SPEC;
        impl crate::RegisterSpec for PORTJ_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`portj::R`](R) reader structure
        impl crate::Readable for PORTJ_SPEC {}
        ///`write(|w| ..)` method takes [`portj::W`](W) writer structure
        impl crate::Writable for PORTJ_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTJ to value 0
        impl crate::Resettable for PORTJ_SPEC {}
    }
}
///I/O Port
pub type PORTK = crate::Periph<portk::RegisterBlock, 0x0106>;
impl core::fmt::Debug for PORTK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTK").finish()
    }
}
///I/O Port
pub mod portk {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pink: PINK,
        ddrk: DDRK,
        portk: PORTK,
    }
    impl RegisterBlock {
        ///0x00 - PORT K Input Pins
        #[inline(always)]
        pub const fn pink(&self) -> &PINK {
            &self.pink
        }
        ///0x01 - PORT K Data Direction Register
        #[inline(always)]
        pub const fn ddrk(&self) -> &DDRK {
            &self.ddrk
        }
        ///0x02 - PORT K Data Register
        #[inline(always)]
        pub const fn portk(&self) -> &PORTK {
            &self.portk
        }
    }
    /**DDRK (rw) register accessor: PORT K Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddrk`] module*/
    pub type DDRK = crate::Reg<ddrk::DDRK_SPEC>;
    ///PORT K Data Direction Register
    pub mod ddrk {
        ///Register `DDRK` reader
        pub type R = crate::R<DDRK_SPEC>;
        ///Register `DDRK` writer
        pub type W = crate::W<DDRK_SPEC>;
        ///Field `PK0` reader - Pin K0
        pub type PK0_R = crate::BitReader;
        ///Field `PK0` writer - Pin K0
        pub type PK0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK1` reader - Pin K1
        pub type PK1_R = crate::BitReader;
        ///Field `PK1` writer - Pin K1
        pub type PK1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK2` reader - Pin K2
        pub type PK2_R = crate::BitReader;
        ///Field `PK2` writer - Pin K2
        pub type PK2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK3` reader - Pin K3
        pub type PK3_R = crate::BitReader;
        ///Field `PK3` writer - Pin K3
        pub type PK3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK4` reader - Pin K4
        pub type PK4_R = crate::BitReader;
        ///Field `PK4` writer - Pin K4
        pub type PK4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK5` reader - Pin K5
        pub type PK5_R = crate::BitReader;
        ///Field `PK5` writer - Pin K5
        pub type PK5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK6` reader - Pin K6
        pub type PK6_R = crate::BitReader;
        ///Field `PK6` writer - Pin K6
        pub type PK6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK7` reader - Pin K7
        pub type PK7_R = crate::BitReader;
        ///Field `PK7` writer - Pin K7
        pub type PK7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin K0
            #[inline(always)]
            pub fn pk0(&self) -> PK0_R {
                PK0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin K1
            #[inline(always)]
            pub fn pk1(&self) -> PK1_R {
                PK1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin K2
            #[inline(always)]
            pub fn pk2(&self) -> PK2_R {
                PK2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin K3
            #[inline(always)]
            pub fn pk3(&self) -> PK3_R {
                PK3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin K4
            #[inline(always)]
            pub fn pk4(&self) -> PK4_R {
                PK4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin K5
            #[inline(always)]
            pub fn pk5(&self) -> PK5_R {
                PK5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin K6
            #[inline(always)]
            pub fn pk6(&self) -> PK6_R {
                PK6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin K7
            #[inline(always)]
            pub fn pk7(&self) -> PK7_R {
                PK7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin K0
            #[inline(always)]
            pub fn pk0(&mut self) -> PK0_W<'_, DDRK_SPEC> {
                PK0_W::new(self, 0)
            }
            ///Bit 1 - Pin K1
            #[inline(always)]
            pub fn pk1(&mut self) -> PK1_W<'_, DDRK_SPEC> {
                PK1_W::new(self, 1)
            }
            ///Bit 2 - Pin K2
            #[inline(always)]
            pub fn pk2(&mut self) -> PK2_W<'_, DDRK_SPEC> {
                PK2_W::new(self, 2)
            }
            ///Bit 3 - Pin K3
            #[inline(always)]
            pub fn pk3(&mut self) -> PK3_W<'_, DDRK_SPEC> {
                PK3_W::new(self, 3)
            }
            ///Bit 4 - Pin K4
            #[inline(always)]
            pub fn pk4(&mut self) -> PK4_W<'_, DDRK_SPEC> {
                PK4_W::new(self, 4)
            }
            ///Bit 5 - Pin K5
            #[inline(always)]
            pub fn pk5(&mut self) -> PK5_W<'_, DDRK_SPEC> {
                PK5_W::new(self, 5)
            }
            ///Bit 6 - Pin K6
            #[inline(always)]
            pub fn pk6(&mut self) -> PK6_W<'_, DDRK_SPEC> {
                PK6_W::new(self, 6)
            }
            ///Bit 7 - Pin K7
            #[inline(always)]
            pub fn pk7(&mut self) -> PK7_W<'_, DDRK_SPEC> {
                PK7_W::new(self, 7)
            }
        }
        /**PORT K Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRK_SPEC;
        impl crate::RegisterSpec for DDRK_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddrk::R`](R) reader structure
        impl crate::Readable for DDRK_SPEC {}
        ///`write(|w| ..)` method takes [`ddrk::W`](W) writer structure
        impl crate::Writable for DDRK_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRK to value 0
        impl crate::Resettable for DDRK_SPEC {}
    }
    /**PINK (rw) register accessor: PORT K Input Pins

You can [`read`](crate::Reg::read) this register and get [`pink::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pink::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pink`] module*/
    pub type PINK = crate::Reg<pink::PINK_SPEC>;
    ///PORT K Input Pins
    pub mod pink {
        ///Register `PINK` reader
        pub type R = crate::R<PINK_SPEC>;
        ///Register `PINK` writer
        pub type W = crate::W<PINK_SPEC>;
        ///Field `PK0` reader - Pin K0
        pub type PK0_R = crate::BitReader;
        ///Field `PK0` writer - Pin K0
        pub type PK0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK1` reader - Pin K1
        pub type PK1_R = crate::BitReader;
        ///Field `PK1` writer - Pin K1
        pub type PK1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK2` reader - Pin K2
        pub type PK2_R = crate::BitReader;
        ///Field `PK2` writer - Pin K2
        pub type PK2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK3` reader - Pin K3
        pub type PK3_R = crate::BitReader;
        ///Field `PK3` writer - Pin K3
        pub type PK3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK4` reader - Pin K4
        pub type PK4_R = crate::BitReader;
        ///Field `PK4` writer - Pin K4
        pub type PK4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK5` reader - Pin K5
        pub type PK5_R = crate::BitReader;
        ///Field `PK5` writer - Pin K5
        pub type PK5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK6` reader - Pin K6
        pub type PK6_R = crate::BitReader;
        ///Field `PK6` writer - Pin K6
        pub type PK6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK7` reader - Pin K7
        pub type PK7_R = crate::BitReader;
        ///Field `PK7` writer - Pin K7
        pub type PK7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin K0
            #[inline(always)]
            pub fn pk0(&self) -> PK0_R {
                PK0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin K1
            #[inline(always)]
            pub fn pk1(&self) -> PK1_R {
                PK1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin K2
            #[inline(always)]
            pub fn pk2(&self) -> PK2_R {
                PK2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin K3
            #[inline(always)]
            pub fn pk3(&self) -> PK3_R {
                PK3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin K4
            #[inline(always)]
            pub fn pk4(&self) -> PK4_R {
                PK4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin K5
            #[inline(always)]
            pub fn pk5(&self) -> PK5_R {
                PK5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin K6
            #[inline(always)]
            pub fn pk6(&self) -> PK6_R {
                PK6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin K7
            #[inline(always)]
            pub fn pk7(&self) -> PK7_R {
                PK7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin K0
            #[inline(always)]
            pub fn pk0(&mut self) -> PK0_W<'_, PINK_SPEC> {
                PK0_W::new(self, 0)
            }
            ///Bit 1 - Pin K1
            #[inline(always)]
            pub fn pk1(&mut self) -> PK1_W<'_, PINK_SPEC> {
                PK1_W::new(self, 1)
            }
            ///Bit 2 - Pin K2
            #[inline(always)]
            pub fn pk2(&mut self) -> PK2_W<'_, PINK_SPEC> {
                PK2_W::new(self, 2)
            }
            ///Bit 3 - Pin K3
            #[inline(always)]
            pub fn pk3(&mut self) -> PK3_W<'_, PINK_SPEC> {
                PK3_W::new(self, 3)
            }
            ///Bit 4 - Pin K4
            #[inline(always)]
            pub fn pk4(&mut self) -> PK4_W<'_, PINK_SPEC> {
                PK4_W::new(self, 4)
            }
            ///Bit 5 - Pin K5
            #[inline(always)]
            pub fn pk5(&mut self) -> PK5_W<'_, PINK_SPEC> {
                PK5_W::new(self, 5)
            }
            ///Bit 6 - Pin K6
            #[inline(always)]
            pub fn pk6(&mut self) -> PK6_W<'_, PINK_SPEC> {
                PK6_W::new(self, 6)
            }
            ///Bit 7 - Pin K7
            #[inline(always)]
            pub fn pk7(&mut self) -> PK7_W<'_, PINK_SPEC> {
                PK7_W::new(self, 7)
            }
        }
        /**PORT K Input Pins

You can [`read`](crate::Reg::read) this register and get [`pink::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pink::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PINK_SPEC;
        impl crate::RegisterSpec for PINK_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pink::R`](R) reader structure
        impl crate::Readable for PINK_SPEC {}
        ///`write(|w| ..)` method takes [`pink::W`](W) writer structure
        impl crate::Writable for PINK_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PINK to value 0
        impl crate::Resettable for PINK_SPEC {}
    }
    /**PORTK (rw) register accessor: PORT K Data Register

You can [`read`](crate::Reg::read) this register and get [`portk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@portk`] module*/
    pub type PORTK = crate::Reg<portk::PORTK_SPEC>;
    ///PORT K Data Register
    pub mod portk {
        ///Register `PORTK` reader
        pub type R = crate::R<PORTK_SPEC>;
        ///Register `PORTK` writer
        pub type W = crate::W<PORTK_SPEC>;
        ///Field `PK0` reader - Pin K0
        pub type PK0_R = crate::BitReader;
        ///Field `PK0` writer - Pin K0
        pub type PK0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK1` reader - Pin K1
        pub type PK1_R = crate::BitReader;
        ///Field `PK1` writer - Pin K1
        pub type PK1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK2` reader - Pin K2
        pub type PK2_R = crate::BitReader;
        ///Field `PK2` writer - Pin K2
        pub type PK2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK3` reader - Pin K3
        pub type PK3_R = crate::BitReader;
        ///Field `PK3` writer - Pin K3
        pub type PK3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK4` reader - Pin K4
        pub type PK4_R = crate::BitReader;
        ///Field `PK4` writer - Pin K4
        pub type PK4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK5` reader - Pin K5
        pub type PK5_R = crate::BitReader;
        ///Field `PK5` writer - Pin K5
        pub type PK5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK6` reader - Pin K6
        pub type PK6_R = crate::BitReader;
        ///Field `PK6` writer - Pin K6
        pub type PK6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PK7` reader - Pin K7
        pub type PK7_R = crate::BitReader;
        ///Field `PK7` writer - Pin K7
        pub type PK7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin K0
            #[inline(always)]
            pub fn pk0(&self) -> PK0_R {
                PK0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin K1
            #[inline(always)]
            pub fn pk1(&self) -> PK1_R {
                PK1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin K2
            #[inline(always)]
            pub fn pk2(&self) -> PK2_R {
                PK2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin K3
            #[inline(always)]
            pub fn pk3(&self) -> PK3_R {
                PK3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin K4
            #[inline(always)]
            pub fn pk4(&self) -> PK4_R {
                PK4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin K5
            #[inline(always)]
            pub fn pk5(&self) -> PK5_R {
                PK5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin K6
            #[inline(always)]
            pub fn pk6(&self) -> PK6_R {
                PK6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin K7
            #[inline(always)]
            pub fn pk7(&self) -> PK7_R {
                PK7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin K0
            #[inline(always)]
            pub fn pk0(&mut self) -> PK0_W<'_, PORTK_SPEC> {
                PK0_W::new(self, 0)
            }
            ///Bit 1 - Pin K1
            #[inline(always)]
            pub fn pk1(&mut self) -> PK1_W<'_, PORTK_SPEC> {
                PK1_W::new(self, 1)
            }
            ///Bit 2 - Pin K2
            #[inline(always)]
            pub fn pk2(&mut self) -> PK2_W<'_, PORTK_SPEC> {
                PK2_W::new(self, 2)
            }
            ///Bit 3 - Pin K3
            #[inline(always)]
            pub fn pk3(&mut self) -> PK3_W<'_, PORTK_SPEC> {
                PK3_W::new(self, 3)
            }
            ///Bit 4 - Pin K4
            #[inline(always)]
            pub fn pk4(&mut self) -> PK4_W<'_, PORTK_SPEC> {
                PK4_W::new(self, 4)
            }
            ///Bit 5 - Pin K5
            #[inline(always)]
            pub fn pk5(&mut self) -> PK5_W<'_, PORTK_SPEC> {
                PK5_W::new(self, 5)
            }
            ///Bit 6 - Pin K6
            #[inline(always)]
            pub fn pk6(&mut self) -> PK6_W<'_, PORTK_SPEC> {
                PK6_W::new(self, 6)
            }
            ///Bit 7 - Pin K7
            #[inline(always)]
            pub fn pk7(&mut self) -> PK7_W<'_, PORTK_SPEC> {
                PK7_W::new(self, 7)
            }
        }
        /**PORT K Data Register

You can [`read`](crate::Reg::read) this register and get [`portk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTK_SPEC;
        impl crate::RegisterSpec for PORTK_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`portk::R`](R) reader structure
        impl crate::Readable for PORTK_SPEC {}
        ///`write(|w| ..)` method takes [`portk::W`](W) writer structure
        impl crate::Writable for PORTK_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTK to value 0
        impl crate::Resettable for PORTK_SPEC {}
    }
}
///I/O Port
pub type PORTL = crate::Periph<portl::RegisterBlock, 0x0109>;
impl core::fmt::Debug for PORTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTL").finish()
    }
}
///I/O Port
pub mod portl {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        pinl: PINL,
        ddrl: DDRL,
        portl: PORTL,
    }
    impl RegisterBlock {
        ///0x00 - PORT L Input Pins
        #[inline(always)]
        pub const fn pinl(&self) -> &PINL {
            &self.pinl
        }
        ///0x01 - PORT L Data Direction Register
        #[inline(always)]
        pub const fn ddrl(&self) -> &DDRL {
            &self.ddrl
        }
        ///0x02 - PORT L Data Register
        #[inline(always)]
        pub const fn portl(&self) -> &PORTL {
            &self.portl
        }
    }
    /**DDRL (rw) register accessor: PORT L Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddrl`] module*/
    pub type DDRL = crate::Reg<ddrl::DDRL_SPEC>;
    ///PORT L Data Direction Register
    pub mod ddrl {
        ///Register `DDRL` reader
        pub type R = crate::R<DDRL_SPEC>;
        ///Register `DDRL` writer
        pub type W = crate::W<DDRL_SPEC>;
        ///Field `PL0` reader - Pin L0
        pub type PL0_R = crate::BitReader;
        ///Field `PL0` writer - Pin L0
        pub type PL0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL1` reader - Pin L1
        pub type PL1_R = crate::BitReader;
        ///Field `PL1` writer - Pin L1
        pub type PL1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL2` reader - Pin L2
        pub type PL2_R = crate::BitReader;
        ///Field `PL2` writer - Pin L2
        pub type PL2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL3` reader - Pin L3
        pub type PL3_R = crate::BitReader;
        ///Field `PL3` writer - Pin L3
        pub type PL3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL4` reader - Pin L4
        pub type PL4_R = crate::BitReader;
        ///Field `PL4` writer - Pin L4
        pub type PL4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL5` reader - Pin L5
        pub type PL5_R = crate::BitReader;
        ///Field `PL5` writer - Pin L5
        pub type PL5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL6` reader - Pin L6
        pub type PL6_R = crate::BitReader;
        ///Field `PL6` writer - Pin L6
        pub type PL6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL7` reader - Pin L7
        pub type PL7_R = crate::BitReader;
        ///Field `PL7` writer - Pin L7
        pub type PL7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin L0
            #[inline(always)]
            pub fn pl0(&self) -> PL0_R {
                PL0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin L1
            #[inline(always)]
            pub fn pl1(&self) -> PL1_R {
                PL1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin L2
            #[inline(always)]
            pub fn pl2(&self) -> PL2_R {
                PL2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin L3
            #[inline(always)]
            pub fn pl3(&self) -> PL3_R {
                PL3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin L4
            #[inline(always)]
            pub fn pl4(&self) -> PL4_R {
                PL4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin L5
            #[inline(always)]
            pub fn pl5(&self) -> PL5_R {
                PL5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin L6
            #[inline(always)]
            pub fn pl6(&self) -> PL6_R {
                PL6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin L7
            #[inline(always)]
            pub fn pl7(&self) -> PL7_R {
                PL7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin L0
            #[inline(always)]
            pub fn pl0(&mut self) -> PL0_W<'_, DDRL_SPEC> {
                PL0_W::new(self, 0)
            }
            ///Bit 1 - Pin L1
            #[inline(always)]
            pub fn pl1(&mut self) -> PL1_W<'_, DDRL_SPEC> {
                PL1_W::new(self, 1)
            }
            ///Bit 2 - Pin L2
            #[inline(always)]
            pub fn pl2(&mut self) -> PL2_W<'_, DDRL_SPEC> {
                PL2_W::new(self, 2)
            }
            ///Bit 3 - Pin L3
            #[inline(always)]
            pub fn pl3(&mut self) -> PL3_W<'_, DDRL_SPEC> {
                PL3_W::new(self, 3)
            }
            ///Bit 4 - Pin L4
            #[inline(always)]
            pub fn pl4(&mut self) -> PL4_W<'_, DDRL_SPEC> {
                PL4_W::new(self, 4)
            }
            ///Bit 5 - Pin L5
            #[inline(always)]
            pub fn pl5(&mut self) -> PL5_W<'_, DDRL_SPEC> {
                PL5_W::new(self, 5)
            }
            ///Bit 6 - Pin L6
            #[inline(always)]
            pub fn pl6(&mut self) -> PL6_W<'_, DDRL_SPEC> {
                PL6_W::new(self, 6)
            }
            ///Bit 7 - Pin L7
            #[inline(always)]
            pub fn pl7(&mut self) -> PL7_W<'_, DDRL_SPEC> {
                PL7_W::new(self, 7)
            }
        }
        /**PORT L Data Direction Register

You can [`read`](crate::Reg::read) this register and get [`ddrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct DDRL_SPEC;
        impl crate::RegisterSpec for DDRL_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ddrl::R`](R) reader structure
        impl crate::Readable for DDRL_SPEC {}
        ///`write(|w| ..)` method takes [`ddrl::W`](W) writer structure
        impl crate::Writable for DDRL_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets DDRL to value 0
        impl crate::Resettable for DDRL_SPEC {}
    }
    /**PINL (rw) register accessor: PORT L Input Pins

You can [`read`](crate::Reg::read) this register and get [`pinl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pinl`] module*/
    pub type PINL = crate::Reg<pinl::PINL_SPEC>;
    ///PORT L Input Pins
    pub mod pinl {
        ///Register `PINL` reader
        pub type R = crate::R<PINL_SPEC>;
        ///Register `PINL` writer
        pub type W = crate::W<PINL_SPEC>;
        ///Field `PL0` reader - Pin L0
        pub type PL0_R = crate::BitReader;
        ///Field `PL0` writer - Pin L0
        pub type PL0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL1` reader - Pin L1
        pub type PL1_R = crate::BitReader;
        ///Field `PL1` writer - Pin L1
        pub type PL1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL2` reader - Pin L2
        pub type PL2_R = crate::BitReader;
        ///Field `PL2` writer - Pin L2
        pub type PL2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL3` reader - Pin L3
        pub type PL3_R = crate::BitReader;
        ///Field `PL3` writer - Pin L3
        pub type PL3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL4` reader - Pin L4
        pub type PL4_R = crate::BitReader;
        ///Field `PL4` writer - Pin L4
        pub type PL4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL5` reader - Pin L5
        pub type PL5_R = crate::BitReader;
        ///Field `PL5` writer - Pin L5
        pub type PL5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL6` reader - Pin L6
        pub type PL6_R = crate::BitReader;
        ///Field `PL6` writer - Pin L6
        pub type PL6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL7` reader - Pin L7
        pub type PL7_R = crate::BitReader;
        ///Field `PL7` writer - Pin L7
        pub type PL7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin L0
            #[inline(always)]
            pub fn pl0(&self) -> PL0_R {
                PL0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin L1
            #[inline(always)]
            pub fn pl1(&self) -> PL1_R {
                PL1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin L2
            #[inline(always)]
            pub fn pl2(&self) -> PL2_R {
                PL2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin L3
            #[inline(always)]
            pub fn pl3(&self) -> PL3_R {
                PL3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin L4
            #[inline(always)]
            pub fn pl4(&self) -> PL4_R {
                PL4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin L5
            #[inline(always)]
            pub fn pl5(&self) -> PL5_R {
                PL5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin L6
            #[inline(always)]
            pub fn pl6(&self) -> PL6_R {
                PL6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin L7
            #[inline(always)]
            pub fn pl7(&self) -> PL7_R {
                PL7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin L0
            #[inline(always)]
            pub fn pl0(&mut self) -> PL0_W<'_, PINL_SPEC> {
                PL0_W::new(self, 0)
            }
            ///Bit 1 - Pin L1
            #[inline(always)]
            pub fn pl1(&mut self) -> PL1_W<'_, PINL_SPEC> {
                PL1_W::new(self, 1)
            }
            ///Bit 2 - Pin L2
            #[inline(always)]
            pub fn pl2(&mut self) -> PL2_W<'_, PINL_SPEC> {
                PL2_W::new(self, 2)
            }
            ///Bit 3 - Pin L3
            #[inline(always)]
            pub fn pl3(&mut self) -> PL3_W<'_, PINL_SPEC> {
                PL3_W::new(self, 3)
            }
            ///Bit 4 - Pin L4
            #[inline(always)]
            pub fn pl4(&mut self) -> PL4_W<'_, PINL_SPEC> {
                PL4_W::new(self, 4)
            }
            ///Bit 5 - Pin L5
            #[inline(always)]
            pub fn pl5(&mut self) -> PL5_W<'_, PINL_SPEC> {
                PL5_W::new(self, 5)
            }
            ///Bit 6 - Pin L6
            #[inline(always)]
            pub fn pl6(&mut self) -> PL6_W<'_, PINL_SPEC> {
                PL6_W::new(self, 6)
            }
            ///Bit 7 - Pin L7
            #[inline(always)]
            pub fn pl7(&mut self) -> PL7_W<'_, PINL_SPEC> {
                PL7_W::new(self, 7)
            }
        }
        /**PORT L Input Pins

You can [`read`](crate::Reg::read) this register and get [`pinl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PINL_SPEC;
        impl crate::RegisterSpec for PINL_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`pinl::R`](R) reader structure
        impl crate::Readable for PINL_SPEC {}
        ///`write(|w| ..)` method takes [`pinl::W`](W) writer structure
        impl crate::Writable for PINL_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PINL to value 0
        impl crate::Resettable for PINL_SPEC {}
    }
    /**PORTL (rw) register accessor: PORT L Data Register

You can [`read`](crate::Reg::read) this register and get [`portl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@portl`] module*/
    pub type PORTL = crate::Reg<portl::PORTL_SPEC>;
    ///PORT L Data Register
    pub mod portl {
        ///Register `PORTL` reader
        pub type R = crate::R<PORTL_SPEC>;
        ///Register `PORTL` writer
        pub type W = crate::W<PORTL_SPEC>;
        ///Field `PL0` reader - Pin L0
        pub type PL0_R = crate::BitReader;
        ///Field `PL0` writer - Pin L0
        pub type PL0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL1` reader - Pin L1
        pub type PL1_R = crate::BitReader;
        ///Field `PL1` writer - Pin L1
        pub type PL1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL2` reader - Pin L2
        pub type PL2_R = crate::BitReader;
        ///Field `PL2` writer - Pin L2
        pub type PL2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL3` reader - Pin L3
        pub type PL3_R = crate::BitReader;
        ///Field `PL3` writer - Pin L3
        pub type PL3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL4` reader - Pin L4
        pub type PL4_R = crate::BitReader;
        ///Field `PL4` writer - Pin L4
        pub type PL4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL5` reader - Pin L5
        pub type PL5_R = crate::BitReader;
        ///Field `PL5` writer - Pin L5
        pub type PL5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL6` reader - Pin L6
        pub type PL6_R = crate::BitReader;
        ///Field `PL6` writer - Pin L6
        pub type PL6_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `PL7` reader - Pin L7
        pub type PL7_R = crate::BitReader;
        ///Field `PL7` writer - Pin L7
        pub type PL7_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Pin L0
            #[inline(always)]
            pub fn pl0(&self) -> PL0_R {
                PL0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Pin L1
            #[inline(always)]
            pub fn pl1(&self) -> PL1_R {
                PL1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Pin L2
            #[inline(always)]
            pub fn pl2(&self) -> PL2_R {
                PL2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Pin L3
            #[inline(always)]
            pub fn pl3(&self) -> PL3_R {
                PL3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Pin L4
            #[inline(always)]
            pub fn pl4(&self) -> PL4_R {
                PL4_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Pin L5
            #[inline(always)]
            pub fn pl5(&self) -> PL5_R {
                PL5_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Pin L6
            #[inline(always)]
            pub fn pl6(&self) -> PL6_R {
                PL6_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Pin L7
            #[inline(always)]
            pub fn pl7(&self) -> PL7_R {
                PL7_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Pin L0
            #[inline(always)]
            pub fn pl0(&mut self) -> PL0_W<'_, PORTL_SPEC> {
                PL0_W::new(self, 0)
            }
            ///Bit 1 - Pin L1
            #[inline(always)]
            pub fn pl1(&mut self) -> PL1_W<'_, PORTL_SPEC> {
                PL1_W::new(self, 1)
            }
            ///Bit 2 - Pin L2
            #[inline(always)]
            pub fn pl2(&mut self) -> PL2_W<'_, PORTL_SPEC> {
                PL2_W::new(self, 2)
            }
            ///Bit 3 - Pin L3
            #[inline(always)]
            pub fn pl3(&mut self) -> PL3_W<'_, PORTL_SPEC> {
                PL3_W::new(self, 3)
            }
            ///Bit 4 - Pin L4
            #[inline(always)]
            pub fn pl4(&mut self) -> PL4_W<'_, PORTL_SPEC> {
                PL4_W::new(self, 4)
            }
            ///Bit 5 - Pin L5
            #[inline(always)]
            pub fn pl5(&mut self) -> PL5_W<'_, PORTL_SPEC> {
                PL5_W::new(self, 5)
            }
            ///Bit 6 - Pin L6
            #[inline(always)]
            pub fn pl6(&mut self) -> PL6_W<'_, PORTL_SPEC> {
                PL6_W::new(self, 6)
            }
            ///Bit 7 - Pin L7
            #[inline(always)]
            pub fn pl7(&mut self) -> PL7_W<'_, PORTL_SPEC> {
                PL7_W::new(self, 7)
            }
        }
        /**PORT L Data Register

You can [`read`](crate::Reg::read) this register and get [`portl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct PORTL_SPEC;
        impl crate::RegisterSpec for PORTL_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`portl::R`](R) reader structure
        impl crate::Readable for PORTL_SPEC {}
        ///`write(|w| ..)` method takes [`portl::W`](W) writer structure
        impl crate::Writable for PORTL_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets PORTL to value 0
        impl crate::Resettable for PORTL_SPEC {}
    }
}
///Serial Peripheral Interface
pub type SPI = crate::Periph<spi::RegisterBlock, 0x4c>;
impl core::fmt::Debug for SPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI").finish()
    }
}
///Serial Peripheral Interface
pub mod spi {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        spcr: SPCR,
        spsr: SPSR,
        spdr: SPDR,
    }
    impl RegisterBlock {
        ///0x00 - SPI Control Register
        #[inline(always)]
        pub const fn spcr(&self) -> &SPCR {
            &self.spcr
        }
        ///0x01 - SPI Status Register
        #[inline(always)]
        pub const fn spsr(&self) -> &SPSR {
            &self.spsr
        }
        ///0x02 - SPI Data Register
        #[inline(always)]
        pub const fn spdr(&self) -> &SPDR {
            &self.spdr
        }
    }
    /**SPCR (rw) register accessor: SPI Control Register

You can [`read`](crate::Reg::read) this register and get [`spcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spcr`] module*/
    pub type SPCR = crate::Reg<spcr::SPCR_SPEC>;
    ///SPI Control Register
    pub mod spcr {
        ///Register `SPCR` reader
        pub type R = crate::R<SPCR_SPEC>;
        ///Register `SPCR` writer
        pub type W = crate::W<SPCR_SPEC>;
        /**SPI Clock Rate Selects

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum SPR_A {
            ///0: Fosc/4 if SPI2X == 0 else Fosc/2
            FOSC_4_2 = 0,
            ///1: Fosc/16 if SPI2X == 0 else Fosc/8
            FOSC_16_8 = 1,
            ///2: Fosc/64 if SPI2X == 0 else Fosc/32
            FOSC_64_32 = 2,
            ///3: Fosc/128 if SPI2X == 0 else Fosc/64
            FOSC_128_64 = 3,
        }
        impl From<SPR_A> for u8 {
            #[inline(always)]
            fn from(variant: SPR_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for SPR_A {
            type Ux = u8;
        }
        impl crate::IsEnum for SPR_A {}
        ///Field `SPR` reader - SPI Clock Rate Selects
        pub type SPR_R = crate::FieldReader<SPR_A>;
        impl SPR_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> SPR_A {
                match self.bits {
                    0 => SPR_A::FOSC_4_2,
                    1 => SPR_A::FOSC_16_8,
                    2 => SPR_A::FOSC_64_32,
                    3 => SPR_A::FOSC_128_64,
                    _ => unreachable!(),
                }
            }
            ///Fosc/4 if SPI2X == 0 else Fosc/2
            #[inline(always)]
            pub fn is_fosc_4_2(&self) -> bool {
                *self == SPR_A::FOSC_4_2
            }
            ///Fosc/16 if SPI2X == 0 else Fosc/8
            #[inline(always)]
            pub fn is_fosc_16_8(&self) -> bool {
                *self == SPR_A::FOSC_16_8
            }
            ///Fosc/64 if SPI2X == 0 else Fosc/32
            #[inline(always)]
            pub fn is_fosc_64_32(&self) -> bool {
                *self == SPR_A::FOSC_64_32
            }
            ///Fosc/128 if SPI2X == 0 else Fosc/64
            #[inline(always)]
            pub fn is_fosc_128_64(&self) -> bool {
                *self == SPR_A::FOSC_128_64
            }
        }
        ///Field `SPR` writer - SPI Clock Rate Selects
        pub type SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPR_A, crate::Safe>;
        impl<'a, REG> SPR_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Fosc/4 if SPI2X == 0 else Fosc/2
            #[inline(always)]
            pub fn fosc_4_2(self) -> &'a mut crate::W<REG> {
                self.variant(SPR_A::FOSC_4_2)
            }
            ///Fosc/16 if SPI2X == 0 else Fosc/8
            #[inline(always)]
            pub fn fosc_16_8(self) -> &'a mut crate::W<REG> {
                self.variant(SPR_A::FOSC_16_8)
            }
            ///Fosc/64 if SPI2X == 0 else Fosc/32
            #[inline(always)]
            pub fn fosc_64_32(self) -> &'a mut crate::W<REG> {
                self.variant(SPR_A::FOSC_64_32)
            }
            ///Fosc/128 if SPI2X == 0 else Fosc/64
            #[inline(always)]
            pub fn fosc_128_64(self) -> &'a mut crate::W<REG> {
                self.variant(SPR_A::FOSC_128_64)
            }
        }
        ///Field `CPHA` reader - Clock Phase
        pub type CPHA_R = crate::BitReader;
        ///Field `CPHA` writer - Clock Phase
        pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `CPOL` reader - Clock polarity
        pub type CPOL_R = crate::BitReader;
        ///Field `CPOL` writer - Clock polarity
        pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `MSTR` reader - Master/Slave Select
        pub type MSTR_R = crate::BitReader;
        ///Field `MSTR` writer - Master/Slave Select
        pub type MSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `DORD` reader - Data Order
        pub type DORD_R = crate::BitReader;
        ///Field `DORD` writer - Data Order
        pub type DORD_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `SPE` reader - SPI Enable
        pub type SPE_R = crate::BitReader;
        ///Field `SPE` writer - SPI Enable
        pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `SPIE` reader - SPI Interrupt Enable
        pub type SPIE_R = crate::BitReader;
        ///Field `SPIE` writer - SPI Interrupt Enable
        pub type SPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:1 - SPI Clock Rate Selects
            #[inline(always)]
            pub fn spr(&self) -> SPR_R {
                SPR_R::new(self.bits & 3)
            }
            ///Bit 2 - Clock Phase
            #[inline(always)]
            pub fn cpha(&self) -> CPHA_R {
                CPHA_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Clock polarity
            #[inline(always)]
            pub fn cpol(&self) -> CPOL_R {
                CPOL_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Master/Slave Select
            #[inline(always)]
            pub fn mstr(&self) -> MSTR_R {
                MSTR_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Data Order
            #[inline(always)]
            pub fn dord(&self) -> DORD_R {
                DORD_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - SPI Enable
            #[inline(always)]
            pub fn spe(&self) -> SPE_R {
                SPE_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - SPI Interrupt Enable
            #[inline(always)]
            pub fn spie(&self) -> SPIE_R {
                SPIE_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:1 - SPI Clock Rate Selects
            #[inline(always)]
            pub fn spr(&mut self) -> SPR_W<'_, SPCR_SPEC> {
                SPR_W::new(self, 0)
            }
            ///Bit 2 - Clock Phase
            #[inline(always)]
            pub fn cpha(&mut self) -> CPHA_W<'_, SPCR_SPEC> {
                CPHA_W::new(self, 2)
            }
            ///Bit 3 - Clock polarity
            #[inline(always)]
            pub fn cpol(&mut self) -> CPOL_W<'_, SPCR_SPEC> {
                CPOL_W::new(self, 3)
            }
            ///Bit 4 - Master/Slave Select
            #[inline(always)]
            pub fn mstr(&mut self) -> MSTR_W<'_, SPCR_SPEC> {
                MSTR_W::new(self, 4)
            }
            ///Bit 5 - Data Order
            #[inline(always)]
            pub fn dord(&mut self) -> DORD_W<'_, SPCR_SPEC> {
                DORD_W::new(self, 5)
            }
            ///Bit 6 - SPI Enable
            #[inline(always)]
            pub fn spe(&mut self) -> SPE_W<'_, SPCR_SPEC> {
                SPE_W::new(self, 6)
            }
            ///Bit 7 - SPI Interrupt Enable
            #[inline(always)]
            pub fn spie(&mut self) -> SPIE_W<'_, SPCR_SPEC> {
                SPIE_W::new(self, 7)
            }
        }
        /**SPI Control Register

You can [`read`](crate::Reg::read) this register and get [`spcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct SPCR_SPEC;
        impl crate::RegisterSpec for SPCR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`spcr::R`](R) reader structure
        impl crate::Readable for SPCR_SPEC {}
        ///`write(|w| ..)` method takes [`spcr::W`](W) writer structure
        impl crate::Writable for SPCR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets SPCR to value 0
        impl crate::Resettable for SPCR_SPEC {}
    }
    /**SPDR (rw) register accessor: SPI Data Register

You can [`read`](crate::Reg::read) this register and get [`spdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spdr`] module*/
    pub type SPDR = crate::Reg<spdr::SPDR_SPEC>;
    ///SPI Data Register
    pub mod spdr {
        ///Register `SPDR` reader
        pub type R = crate::R<SPDR_SPEC>;
        ///Register `SPDR` writer
        pub type W = crate::W<SPDR_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**SPI Data Register

You can [`read`](crate::Reg::read) this register and get [`spdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct SPDR_SPEC;
        impl crate::RegisterSpec for SPDR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`spdr::R`](R) reader structure
        impl crate::Readable for SPDR_SPEC {}
        ///`write(|w| ..)` method takes [`spdr::W`](W) writer structure
        impl crate::Writable for SPDR_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets SPDR to value 0
        impl crate::Resettable for SPDR_SPEC {}
    }
    /**SPSR (rw) register accessor: SPI Status Register

You can [`read`](crate::Reg::read) this register and get [`spsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spsr`] module*/
    pub type SPSR = crate::Reg<spsr::SPSR_SPEC>;
    ///SPI Status Register
    pub mod spsr {
        ///Register `SPSR` reader
        pub type R = crate::R<SPSR_SPEC>;
        ///Register `SPSR` writer
        pub type W = crate::W<SPSR_SPEC>;
        ///Field `SPI2X` reader - Double SPI Speed Bit
        pub type SPI2X_R = crate::BitReader;
        ///Field `SPI2X` writer - Double SPI Speed Bit
        pub type SPI2X_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `WCOL` reader - Write Collision Flag
        pub type WCOL_R = crate::BitReader;
        ///Field `SPIF` reader - SPI Interrupt Flag
        pub type SPIF_R = crate::BitReader;
        impl R {
            ///Bit 0 - Double SPI Speed Bit
            #[inline(always)]
            pub fn spi2x(&self) -> SPI2X_R {
                SPI2X_R::new((self.bits & 1) != 0)
            }
            ///Bit 6 - Write Collision Flag
            #[inline(always)]
            pub fn wcol(&self) -> WCOL_R {
                WCOL_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - SPI Interrupt Flag
            #[inline(always)]
            pub fn spif(&self) -> SPIF_R {
                SPIF_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Double SPI Speed Bit
            #[inline(always)]
            pub fn spi2x(&mut self) -> SPI2X_W<'_, SPSR_SPEC> {
                SPI2X_W::new(self, 0)
            }
        }
        /**SPI Status Register

You can [`read`](crate::Reg::read) this register and get [`spsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct SPSR_SPEC;
        impl crate::RegisterSpec for SPSR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`spsr::R`](R) reader structure
        impl crate::Readable for SPSR_SPEC {}
        ///`write(|w| ..)` method takes [`spsr::W`](W) writer structure
        impl crate::Writable for SPSR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets SPSR to value 0
        impl crate::Resettable for SPSR_SPEC {}
    }
}
///Timer/Counter, 8-bit
pub type TC0 = crate::Periph<tc0::RegisterBlock, 0x35>;
impl core::fmt::Debug for TC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC0").finish()
    }
}
///Timer/Counter, 8-bit
pub mod tc0 {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        tifr0: TIFR0,
        _reserved1: [u8; 0x0d],
        gtccr: GTCCR,
        tccr0a: TCCR0A,
        tccr0b: TCCR0B,
        tcnt0: TCNT0,
        ocr0a: OCR0A,
        ocr0b: OCR0B,
        _reserved7: [u8; 0x25],
        timsk0: TIMSK0,
    }
    impl RegisterBlock {
        ///0x00 - Timer/Counter0 Interrupt Flag register
        #[inline(always)]
        pub const fn tifr0(&self) -> &TIFR0 {
            &self.tifr0
        }
        ///0x0e - General Timer/Counter Control Register
        #[inline(always)]
        pub const fn gtccr(&self) -> &GTCCR {
            &self.gtccr
        }
        ///0x0f - Timer/Counter Control Register A
        #[inline(always)]
        pub const fn tccr0a(&self) -> &TCCR0A {
            &self.tccr0a
        }
        ///0x10 - Timer/Counter Control Register B
        #[inline(always)]
        pub const fn tccr0b(&self) -> &TCCR0B {
            &self.tccr0b
        }
        ///0x11 - Timer/Counter0
        #[inline(always)]
        pub const fn tcnt0(&self) -> &TCNT0 {
            &self.tcnt0
        }
        ///0x12 - Timer/Counter0 Output Compare Register
        #[inline(always)]
        pub const fn ocr0a(&self) -> &OCR0A {
            &self.ocr0a
        }
        ///0x13 - Timer/Counter0 Output Compare Register
        #[inline(always)]
        pub const fn ocr0b(&self) -> &OCR0B {
            &self.ocr0b
        }
        ///0x39 - Timer/Counter0 Interrupt Mask Register
        #[inline(always)]
        pub const fn timsk0(&self) -> &TIMSK0 {
            &self.timsk0
        }
    }
    /**GTCCR (rw) register accessor: General Timer/Counter Control Register

You can [`read`](crate::Reg::read) this register and get [`gtccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccr`] module*/
    pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
    ///General Timer/Counter Control Register
    pub mod gtccr {
        ///Register `GTCCR` reader
        pub type R = crate::R<GTCCR_SPEC>;
        ///Register `GTCCR` writer
        pub type W = crate::W<GTCCR_SPEC>;
        ///Field `PSRSYNC` reader - Prescaler Reset Timer/Counter1 and Timer/Counter0
        pub type PSRSYNC_R = crate::BitReader;
        ///Field `PSRSYNC` writer - Prescaler Reset Timer/Counter1 and Timer/Counter0
        pub type PSRSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TSM` reader - Timer/Counter Synchronization Mode
        pub type TSM_R = crate::BitReader;
        ///Field `TSM` writer - Timer/Counter Synchronization Mode
        pub type TSM_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Prescaler Reset Timer/Counter1 and Timer/Counter0
            #[inline(always)]
            pub fn psrsync(&self) -> PSRSYNC_R {
                PSRSYNC_R::new((self.bits & 1) != 0)
            }
            ///Bit 7 - Timer/Counter Synchronization Mode
            #[inline(always)]
            pub fn tsm(&self) -> TSM_R {
                TSM_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Prescaler Reset Timer/Counter1 and Timer/Counter0
            #[inline(always)]
            pub fn psrsync(&mut self) -> PSRSYNC_W<'_, GTCCR_SPEC> {
                PSRSYNC_W::new(self, 0)
            }
            ///Bit 7 - Timer/Counter Synchronization Mode
            #[inline(always)]
            pub fn tsm(&mut self) -> TSM_W<'_, GTCCR_SPEC> {
                TSM_W::new(self, 7)
            }
        }
        /**General Timer/Counter Control Register

You can [`read`](crate::Reg::read) this register and get [`gtccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct GTCCR_SPEC;
        impl crate::RegisterSpec for GTCCR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`gtccr::R`](R) reader structure
        impl crate::Readable for GTCCR_SPEC {}
        ///`write(|w| ..)` method takes [`gtccr::W`](W) writer structure
        impl crate::Writable for GTCCR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets GTCCR to value 0
        impl crate::Resettable for GTCCR_SPEC {}
    }
    /**OCR0A (rw) register accessor: Timer/Counter0 Output Compare Register

You can [`read`](crate::Reg::read) this register and get [`ocr0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr0a`] module*/
    pub type OCR0A = crate::Reg<ocr0a::OCR0A_SPEC>;
    ///Timer/Counter0 Output Compare Register
    pub mod ocr0a {
        ///Register `OCR0A` reader
        pub type R = crate::R<OCR0A_SPEC>;
        ///Register `OCR0A` writer
        pub type W = crate::W<OCR0A_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter0 Output Compare Register

You can [`read`](crate::Reg::read) this register and get [`ocr0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR0A_SPEC;
        impl crate::RegisterSpec for OCR0A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ocr0a::R`](R) reader structure
        impl crate::Readable for OCR0A_SPEC {}
        ///`write(|w| ..)` method takes [`ocr0a::W`](W) writer structure
        impl crate::Writable for OCR0A_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR0A to value 0
        impl crate::Resettable for OCR0A_SPEC {}
    }
    /**OCR0B (rw) register accessor: Timer/Counter0 Output Compare Register

You can [`read`](crate::Reg::read) this register and get [`ocr0b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr0b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr0b`] module*/
    pub type OCR0B = crate::Reg<ocr0b::OCR0B_SPEC>;
    ///Timer/Counter0 Output Compare Register
    pub mod ocr0b {
        ///Register `OCR0B` reader
        pub type R = crate::R<OCR0B_SPEC>;
        ///Register `OCR0B` writer
        pub type W = crate::W<OCR0B_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter0 Output Compare Register

You can [`read`](crate::Reg::read) this register and get [`ocr0b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr0b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR0B_SPEC;
        impl crate::RegisterSpec for OCR0B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ocr0b::R`](R) reader structure
        impl crate::Readable for OCR0B_SPEC {}
        ///`write(|w| ..)` method takes [`ocr0b::W`](W) writer structure
        impl crate::Writable for OCR0B_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR0B to value 0
        impl crate::Resettable for OCR0B_SPEC {}
    }
    /**TCCR0A (rw) register accessor: Timer/Counter Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr0a`] module*/
    pub type TCCR0A = crate::Reg<tccr0a::TCCR0A_SPEC>;
    ///Timer/Counter Control Register A
    pub mod tccr0a {
        ///Register `TCCR0A` reader
        pub type R = crate::R<TCCR0A_SPEC>;
        ///Register `TCCR0A` writer
        pub type W = crate::W<TCCR0A_SPEC>;
        /**Waveform Generation Mode

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum WGM0_A {
            ///0: Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*
            NORMAL_TOP = 0,
            ///1: Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*
            PWM_PHASE = 1,
            ///2: CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*
            CTC = 2,
            ///3: Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*
            PWM_FAST = 3,
        }
        impl From<WGM0_A> for u8 {
            #[inline(always)]
            fn from(variant: WGM0_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for WGM0_A {
            type Ux = u8;
        }
        impl crate::IsEnum for WGM0_A {}
        ///Field `WGM0` reader - Waveform Generation Mode
        pub type WGM0_R = crate::FieldReader<WGM0_A>;
        impl WGM0_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> WGM0_A {
                match self.bits {
                    0 => WGM0_A::NORMAL_TOP,
                    1 => WGM0_A::PWM_PHASE,
                    2 => WGM0_A::CTC,
                    3 => WGM0_A::PWM_FAST,
                    _ => unreachable!(),
                }
            }
            ///Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*
            #[inline(always)]
            pub fn is_normal_top(&self) -> bool {
                *self == WGM0_A::NORMAL_TOP
            }
            ///Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*
            #[inline(always)]
            pub fn is_pwm_phase(&self) -> bool {
                *self == WGM0_A::PWM_PHASE
            }
            ///CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*
            #[inline(always)]
            pub fn is_ctc(&self) -> bool {
                *self == WGM0_A::CTC
            }
            ///Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*
            #[inline(always)]
            pub fn is_pwm_fast(&self) -> bool {
                *self == WGM0_A::PWM_FAST
            }
        }
        ///Field `WGM0` writer - Waveform Generation Mode
        pub type WGM0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WGM0_A, crate::Safe>;
        impl<'a, REG> WGM0_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*
            #[inline(always)]
            pub fn normal_top(self) -> &'a mut crate::W<REG> {
                self.variant(WGM0_A::NORMAL_TOP)
            }
            ///Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*
            #[inline(always)]
            pub fn pwm_phase(self) -> &'a mut crate::W<REG> {
                self.variant(WGM0_A::PWM_PHASE)
            }
            ///CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*
            #[inline(always)]
            pub fn ctc(self) -> &'a mut crate::W<REG> {
                self.variant(WGM0_A::CTC)
            }
            ///Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*
            #[inline(always)]
            pub fn pwm_fast(self) -> &'a mut crate::W<REG> {
                self.variant(WGM0_A::PWM_FAST)
            }
        }
        /**Compare Output B Mode

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum COM0B_A {
            ///0: Normal port operation, OCix disconnected
            DISCONNECTED = 0,
            ///1: Toggle OCix on Compare Match (Might depend on WGM)
            MATCH_TOGGLE = 1,
            ///2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)
            MATCH_CLEAR = 2,
            ///3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)
            MATCH_SET = 3,
        }
        impl From<COM0B_A> for u8 {
            #[inline(always)]
            fn from(variant: COM0B_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for COM0B_A {
            type Ux = u8;
        }
        impl crate::IsEnum for COM0B_A {}
        ///Field `COM0B` reader - Compare Output B Mode
        pub type COM0B_R = crate::FieldReader<COM0B_A>;
        impl COM0B_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> COM0B_A {
                match self.bits {
                    0 => COM0B_A::DISCONNECTED,
                    1 => COM0B_A::MATCH_TOGGLE,
                    2 => COM0B_A::MATCH_CLEAR,
                    3 => COM0B_A::MATCH_SET,
                    _ => unreachable!(),
                }
            }
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn is_disconnected(&self) -> bool {
                *self == COM0B_A::DISCONNECTED
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn is_match_toggle(&self) -> bool {
                *self == COM0B_A::MATCH_TOGGLE
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)
            #[inline(always)]
            pub fn is_match_clear(&self) -> bool {
                *self == COM0B_A::MATCH_CLEAR
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)
            #[inline(always)]
            pub fn is_match_set(&self) -> bool {
                *self == COM0B_A::MATCH_SET
            }
        }
        ///Field `COM0B` writer - Compare Output B Mode
        pub type COM0B_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COM0B_A, crate::Safe>;
        impl<'a, REG> COM0B_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn disconnected(self) -> &'a mut crate::W<REG> {
                self.variant(COM0B_A::DISCONNECTED)
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn match_toggle(self) -> &'a mut crate::W<REG> {
                self.variant(COM0B_A::MATCH_TOGGLE)
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)
            #[inline(always)]
            pub fn match_clear(self) -> &'a mut crate::W<REG> {
                self.variant(COM0B_A::MATCH_CLEAR)
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)
            #[inline(always)]
            pub fn match_set(self) -> &'a mut crate::W<REG> {
                self.variant(COM0B_A::MATCH_SET)
            }
        }
        ///Field `COM0A` reader - Compare Output A Mode
        pub use COM0B_R as COM0A_R;
        ///Field `COM0A` writer - Compare Output A Mode
        pub use COM0B_W as COM0A_W;
        impl R {
            ///Bits 0:1 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm0(&self) -> WGM0_R {
                WGM0_R::new(self.bits & 3)
            }
            ///Bits 4:5 - Compare Output B Mode
            #[inline(always)]
            pub fn com0b(&self) -> COM0B_R {
                COM0B_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - Compare Output A Mode
            #[inline(always)]
            pub fn com0a(&self) -> COM0A_R {
                COM0A_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bits 0:1 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm0(&mut self) -> WGM0_W<'_, TCCR0A_SPEC> {
                WGM0_W::new(self, 0)
            }
            ///Bits 4:5 - Compare Output B Mode
            #[inline(always)]
            pub fn com0b(&mut self) -> COM0B_W<'_, TCCR0A_SPEC> {
                COM0B_W::new(self, 4)
            }
            ///Bits 6:7 - Compare Output A Mode
            #[inline(always)]
            pub fn com0a(&mut self) -> COM0A_W<'_, TCCR0A_SPEC> {
                COM0A_W::new(self, 6)
            }
        }
        /**Timer/Counter Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR0A_SPEC;
        impl crate::RegisterSpec for TCCR0A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr0a::R`](R) reader structure
        impl crate::Readable for TCCR0A_SPEC {}
        ///`write(|w| ..)` method takes [`tccr0a::W`](W) writer structure
        impl crate::Writable for TCCR0A_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR0A to value 0
        impl crate::Resettable for TCCR0A_SPEC {}
    }
    /**TCCR0B (rw) register accessor: Timer/Counter Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr0b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr0b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr0b`] module*/
    pub type TCCR0B = crate::Reg<tccr0b::TCCR0B_SPEC>;
    ///Timer/Counter Control Register B
    pub mod tccr0b {
        ///Register `TCCR0B` reader
        pub type R = crate::R<TCCR0B_SPEC>;
        ///Register `TCCR0B` writer
        pub type W = crate::W<TCCR0B_SPEC>;
        /**Clock Select

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum CS0_A {
            ///0: No clock source (Timer/Counter stopped)
            NO_CLOCK = 0,
            ///1: Running, No Prescaling
            DIRECT = 1,
            ///2: Running, CLK/8
            PRESCALE_8 = 2,
            ///3: Running, CLK/64
            PRESCALE_64 = 3,
            ///4: Running, CLK/256
            PRESCALE_256 = 4,
            ///5: Running, CLK/1024
            PRESCALE_1024 = 5,
            ///6: Running, ExtClk Tx Falling Edge
            EXT_FALLING = 6,
            ///7: Running, ExtClk Tx Rising Edge
            EXT_RISING = 7,
        }
        impl From<CS0_A> for u8 {
            #[inline(always)]
            fn from(variant: CS0_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for CS0_A {
            type Ux = u8;
        }
        impl crate::IsEnum for CS0_A {}
        ///Field `CS0` reader - Clock Select
        pub type CS0_R = crate::FieldReader<CS0_A>;
        impl CS0_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> CS0_A {
                match self.bits {
                    0 => CS0_A::NO_CLOCK,
                    1 => CS0_A::DIRECT,
                    2 => CS0_A::PRESCALE_8,
                    3 => CS0_A::PRESCALE_64,
                    4 => CS0_A::PRESCALE_256,
                    5 => CS0_A::PRESCALE_1024,
                    6 => CS0_A::EXT_FALLING,
                    7 => CS0_A::EXT_RISING,
                    _ => unreachable!(),
                }
            }
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn is_no_clock(&self) -> bool {
                *self == CS0_A::NO_CLOCK
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn is_direct(&self) -> bool {
                *self == CS0_A::DIRECT
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn is_prescale_8(&self) -> bool {
                *self == CS0_A::PRESCALE_8
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn is_prescale_64(&self) -> bool {
                *self == CS0_A::PRESCALE_64
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn is_prescale_256(&self) -> bool {
                *self == CS0_A::PRESCALE_256
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn is_prescale_1024(&self) -> bool {
                *self == CS0_A::PRESCALE_1024
            }
            ///Running, ExtClk Tx Falling Edge
            #[inline(always)]
            pub fn is_ext_falling(&self) -> bool {
                *self == CS0_A::EXT_FALLING
            }
            ///Running, ExtClk Tx Rising Edge
            #[inline(always)]
            pub fn is_ext_rising(&self) -> bool {
                *self == CS0_A::EXT_RISING
            }
        }
        ///Field `CS0` writer - Clock Select
        pub type CS0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CS0_A, crate::Safe>;
        impl<'a, REG> CS0_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn no_clock(self) -> &'a mut crate::W<REG> {
                self.variant(CS0_A::NO_CLOCK)
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn direct(self) -> &'a mut crate::W<REG> {
                self.variant(CS0_A::DIRECT)
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn prescale_8(self) -> &'a mut crate::W<REG> {
                self.variant(CS0_A::PRESCALE_8)
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn prescale_64(self) -> &'a mut crate::W<REG> {
                self.variant(CS0_A::PRESCALE_64)
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn prescale_256(self) -> &'a mut crate::W<REG> {
                self.variant(CS0_A::PRESCALE_256)
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
                self.variant(CS0_A::PRESCALE_1024)
            }
            ///Running, ExtClk Tx Falling Edge
            #[inline(always)]
            pub fn ext_falling(self) -> &'a mut crate::W<REG> {
                self.variant(CS0_A::EXT_FALLING)
            }
            ///Running, ExtClk Tx Rising Edge
            #[inline(always)]
            pub fn ext_rising(self) -> &'a mut crate::W<REG> {
                self.variant(CS0_A::EXT_RISING)
            }
        }
        ///Field `WGM02` reader - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)
        pub type WGM02_R = crate::BitReader;
        ///Field `WGM02` writer - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)
        pub type WGM02_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC0B` writer - Force Output Compare B
        pub type FOC0B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC0A` writer - Force Output Compare A
        pub type FOC0A_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:2 - Clock Select
            #[inline(always)]
            pub fn cs0(&self) -> CS0_R {
                CS0_R::new(self.bits & 7)
            }
            ///Bit 3 - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)
            #[inline(always)]
            pub fn wgm02(&self) -> WGM02_R {
                WGM02_R::new(((self.bits >> 3) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:2 - Clock Select
            #[inline(always)]
            pub fn cs0(&mut self) -> CS0_W<'_, TCCR0B_SPEC> {
                CS0_W::new(self, 0)
            }
            ///Bit 3 - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)
            #[inline(always)]
            pub fn wgm02(&mut self) -> WGM02_W<'_, TCCR0B_SPEC> {
                WGM02_W::new(self, 3)
            }
            ///Bit 6 - Force Output Compare B
            #[inline(always)]
            pub fn foc0b(&mut self) -> FOC0B_W<'_, TCCR0B_SPEC> {
                FOC0B_W::new(self, 6)
            }
            ///Bit 7 - Force Output Compare A
            #[inline(always)]
            pub fn foc0a(&mut self) -> FOC0A_W<'_, TCCR0B_SPEC> {
                FOC0A_W::new(self, 7)
            }
        }
        /**Timer/Counter Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr0b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr0b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR0B_SPEC;
        impl crate::RegisterSpec for TCCR0B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr0b::R`](R) reader structure
        impl crate::Readable for TCCR0B_SPEC {}
        ///`write(|w| ..)` method takes [`tccr0b::W`](W) writer structure
        impl crate::Writable for TCCR0B_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR0B to value 0
        impl crate::Resettable for TCCR0B_SPEC {}
    }
    /**TCNT0 (rw) register accessor: Timer/Counter0

You can [`read`](crate::Reg::read) this register and get [`tcnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcnt0`] module*/
    pub type TCNT0 = crate::Reg<tcnt0::TCNT0_SPEC>;
    ///Timer/Counter0
    pub mod tcnt0 {
        ///Register `TCNT0` reader
        pub type R = crate::R<TCNT0_SPEC>;
        ///Register `TCNT0` writer
        pub type W = crate::W<TCNT0_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter0

You can [`read`](crate::Reg::read) this register and get [`tcnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCNT0_SPEC;
        impl crate::RegisterSpec for TCNT0_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tcnt0::R`](R) reader structure
        impl crate::Readable for TCNT0_SPEC {}
        ///`write(|w| ..)` method takes [`tcnt0::W`](W) writer structure
        impl crate::Writable for TCNT0_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets TCNT0 to value 0
        impl crate::Resettable for TCNT0_SPEC {}
    }
    /**TIFR0 (rw) register accessor: Timer/Counter0 Interrupt Flag register

You can [`read`](crate::Reg::read) this register and get [`tifr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tifr0`] module*/
    pub type TIFR0 = crate::Reg<tifr0::TIFR0_SPEC>;
    ///Timer/Counter0 Interrupt Flag register
    pub mod tifr0 {
        ///Register `TIFR0` reader
        pub type R = crate::R<TIFR0_SPEC>;
        ///Register `TIFR0` writer
        pub type W = crate::W<TIFR0_SPEC>;
        ///Field `TOV0` reader - Timer/Counter0 Overflow Flag
        pub type TOV0_R = crate::BitReader;
        ///Field `TOV0` writer - Timer/Counter0 Overflow Flag
        pub type TOV0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF0A` reader - Timer/Counter0 Output Compare Flag 0A
        pub type OCF0A_R = crate::BitReader;
        ///Field `OCF0A` writer - Timer/Counter0 Output Compare Flag 0A
        pub type OCF0A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF0B` reader - Timer/Counter0 Output Compare Flag 0B
        pub type OCF0B_R = crate::BitReader;
        ///Field `OCF0B` writer - Timer/Counter0 Output Compare Flag 0B
        pub type OCF0B_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter0 Overflow Flag
            #[inline(always)]
            pub fn tov0(&self) -> TOV0_R {
                TOV0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Timer/Counter0 Output Compare Flag 0A
            #[inline(always)]
            pub fn ocf0a(&self) -> OCF0A_R {
                OCF0A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Timer/Counter0 Output Compare Flag 0B
            #[inline(always)]
            pub fn ocf0b(&self) -> OCF0B_R {
                OCF0B_R::new(((self.bits >> 2) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter0 Overflow Flag
            #[inline(always)]
            pub fn tov0(&mut self) -> TOV0_W<'_, TIFR0_SPEC> {
                TOV0_W::new(self, 0)
            }
            ///Bit 1 - Timer/Counter0 Output Compare Flag 0A
            #[inline(always)]
            pub fn ocf0a(&mut self) -> OCF0A_W<'_, TIFR0_SPEC> {
                OCF0A_W::new(self, 1)
            }
            ///Bit 2 - Timer/Counter0 Output Compare Flag 0B
            #[inline(always)]
            pub fn ocf0b(&mut self) -> OCF0B_W<'_, TIFR0_SPEC> {
                OCF0B_W::new(self, 2)
            }
        }
        /**Timer/Counter0 Interrupt Flag register

You can [`read`](crate::Reg::read) this register and get [`tifr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIFR0_SPEC;
        impl crate::RegisterSpec for TIFR0_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tifr0::R`](R) reader structure
        impl crate::Readable for TIFR0_SPEC {}
        ///`write(|w| ..)` method takes [`tifr0::W`](W) writer structure
        impl crate::Writable for TIFR0_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIFR0 to value 0
        impl crate::Resettable for TIFR0_SPEC {}
    }
    /**TIMSK0 (rw) register accessor: Timer/Counter0 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`timsk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timsk0`] module*/
    pub type TIMSK0 = crate::Reg<timsk0::TIMSK0_SPEC>;
    ///Timer/Counter0 Interrupt Mask Register
    pub mod timsk0 {
        ///Register `TIMSK0` reader
        pub type R = crate::R<TIMSK0_SPEC>;
        ///Register `TIMSK0` writer
        pub type W = crate::W<TIMSK0_SPEC>;
        ///Field `TOIE0` reader - Timer/Counter0 Overflow Interrupt Enable
        pub type TOIE0_R = crate::BitReader;
        ///Field `TOIE0` writer - Timer/Counter0 Overflow Interrupt Enable
        pub type TOIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE0A` reader - Timer/Counter0 Output Compare Match A Interrupt Enable
        pub type OCIE0A_R = crate::BitReader;
        ///Field `OCIE0A` writer - Timer/Counter0 Output Compare Match A Interrupt Enable
        pub type OCIE0A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE0B` reader - Timer/Counter0 Output Compare Match B Interrupt Enable
        pub type OCIE0B_R = crate::BitReader;
        ///Field `OCIE0B` writer - Timer/Counter0 Output Compare Match B Interrupt Enable
        pub type OCIE0B_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter0 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie0(&self) -> TOIE0_R {
                TOIE0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Timer/Counter0 Output Compare Match A Interrupt Enable
            #[inline(always)]
            pub fn ocie0a(&self) -> OCIE0A_R {
                OCIE0A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Timer/Counter0 Output Compare Match B Interrupt Enable
            #[inline(always)]
            pub fn ocie0b(&self) -> OCIE0B_R {
                OCIE0B_R::new(((self.bits >> 2) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter0 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie0(&mut self) -> TOIE0_W<'_, TIMSK0_SPEC> {
                TOIE0_W::new(self, 0)
            }
            ///Bit 1 - Timer/Counter0 Output Compare Match A Interrupt Enable
            #[inline(always)]
            pub fn ocie0a(&mut self) -> OCIE0A_W<'_, TIMSK0_SPEC> {
                OCIE0A_W::new(self, 1)
            }
            ///Bit 2 - Timer/Counter0 Output Compare Match B Interrupt Enable
            #[inline(always)]
            pub fn ocie0b(&mut self) -> OCIE0B_W<'_, TIMSK0_SPEC> {
                OCIE0B_W::new(self, 2)
            }
        }
        /**Timer/Counter0 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`timsk0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIMSK0_SPEC;
        impl crate::RegisterSpec for TIMSK0_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`timsk0::R`](R) reader structure
        impl crate::Readable for TIMSK0_SPEC {}
        ///`write(|w| ..)` method takes [`timsk0::W`](W) writer structure
        impl crate::Writable for TIMSK0_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIMSK0 to value 0
        impl crate::Resettable for TIMSK0_SPEC {}
    }
}
///Timer/Counter, 16-bit
pub type TC1 = crate::Periph<tc1::RegisterBlock, 0x36>;
impl core::fmt::Debug for TC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC1").finish()
    }
}
///Timer/Counter, 16-bit
pub mod tc1 {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        tifr1: TIFR1,
        _reserved1: [u8; 0x38],
        timsk1: TIMSK1,
        _reserved2: [u8; 0x10],
        tccr1a: TCCR1A,
        tccr1b: TCCR1B,
        tccr1c: TCCR1C,
        _reserved5: [u8; 0x01],
        tcnt1: TCNT1,
        icr1: ICR1,
        ocr1a: OCR1A,
        ocr1b: OCR1B,
        ocr1c: OCR1C,
    }
    impl RegisterBlock {
        ///0x00 - Timer/Counter1 Interrupt Flag register
        #[inline(always)]
        pub const fn tifr1(&self) -> &TIFR1 {
            &self.tifr1
        }
        ///0x39 - Timer/Counter1 Interrupt Mask Register
        #[inline(always)]
        pub const fn timsk1(&self) -> &TIMSK1 {
            &self.timsk1
        }
        ///0x4a - Timer/Counter1 Control Register A
        #[inline(always)]
        pub const fn tccr1a(&self) -> &TCCR1A {
            &self.tccr1a
        }
        ///0x4b - Timer/Counter1 Control Register B
        #[inline(always)]
        pub const fn tccr1b(&self) -> &TCCR1B {
            &self.tccr1b
        }
        ///0x4c - Timer/Counter 1 Control Register C
        #[inline(always)]
        pub const fn tccr1c(&self) -> &TCCR1C {
            &self.tccr1c
        }
        ///0x4e - Timer/Counter1 Bytes
        #[inline(always)]
        pub const fn tcnt1(&self) -> &TCNT1 {
            &self.tcnt1
        }
        ///0x50 - Timer/Counter1 Input Capture Register Bytes
        #[inline(always)]
        pub const fn icr1(&self) -> &ICR1 {
            &self.icr1
        }
        ///0x52 - Timer/Counter1 Output Compare Register A Bytes
        #[inline(always)]
        pub const fn ocr1a(&self) -> &OCR1A {
            &self.ocr1a
        }
        ///0x54 - Timer/Counter1 Output Compare Register B Bytes
        #[inline(always)]
        pub const fn ocr1b(&self) -> &OCR1B {
            &self.ocr1b
        }
        ///0x56 - Timer/Counter1 Output Compare Register C Bytes
        #[inline(always)]
        pub const fn ocr1c(&self) -> &OCR1C {
            &self.ocr1c
        }
    }
    /**ICR1 (rw) register accessor: Timer/Counter1 Input Capture Register Bytes

You can [`read`](crate::Reg::read) this register and get [`icr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icr1`] module*/
    pub type ICR1 = crate::Reg<icr1::ICR1_SPEC>;
    ///Timer/Counter1 Input Capture Register Bytes
    pub mod icr1 {
        ///Register `ICR1` reader
        pub type R = crate::R<ICR1_SPEC>;
        ///Register `ICR1` writer
        pub type W = crate::W<ICR1_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter1 Input Capture Register Bytes

You can [`read`](crate::Reg::read) this register and get [`icr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ICR1_SPEC;
        impl crate::RegisterSpec for ICR1_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`icr1::R`](R) reader structure
        impl crate::Readable for ICR1_SPEC {}
        ///`write(|w| ..)` method takes [`icr1::W`](W) writer structure
        impl crate::Writable for ICR1_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets ICR1 to value 0
        impl crate::Resettable for ICR1_SPEC {}
    }
    /**OCR1A (rw) register accessor: Timer/Counter1 Output Compare Register A Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr1a`] module*/
    pub type OCR1A = crate::Reg<ocr1a::OCR1A_SPEC>;
    ///Timer/Counter1 Output Compare Register A Bytes
    pub mod ocr1a {
        ///Register `OCR1A` reader
        pub type R = crate::R<OCR1A_SPEC>;
        ///Register `OCR1A` writer
        pub type W = crate::W<OCR1A_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter1 Output Compare Register A Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr1a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr1a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR1A_SPEC;
        impl crate::RegisterSpec for OCR1A_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr1a::R`](R) reader structure
        impl crate::Readable for OCR1A_SPEC {}
        ///`write(|w| ..)` method takes [`ocr1a::W`](W) writer structure
        impl crate::Writable for OCR1A_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR1A to value 0
        impl crate::Resettable for OCR1A_SPEC {}
    }
    /**OCR1B (rw) register accessor: Timer/Counter1 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr1b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr1b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr1b`] module*/
    pub type OCR1B = crate::Reg<ocr1b::OCR1B_SPEC>;
    ///Timer/Counter1 Output Compare Register B Bytes
    pub mod ocr1b {
        ///Register `OCR1B` reader
        pub type R = crate::R<OCR1B_SPEC>;
        ///Register `OCR1B` writer
        pub type W = crate::W<OCR1B_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter1 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr1b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr1b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR1B_SPEC;
        impl crate::RegisterSpec for OCR1B_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr1b::R`](R) reader structure
        impl crate::Readable for OCR1B_SPEC {}
        ///`write(|w| ..)` method takes [`ocr1b::W`](W) writer structure
        impl crate::Writable for OCR1B_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR1B to value 0
        impl crate::Resettable for OCR1B_SPEC {}
    }
    /**OCR1C (rw) register accessor: Timer/Counter1 Output Compare Register C Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr1c`] module*/
    pub type OCR1C = crate::Reg<ocr1c::OCR1C_SPEC>;
    ///Timer/Counter1 Output Compare Register C Bytes
    pub mod ocr1c {
        ///Register `OCR1C` reader
        pub type R = crate::R<OCR1C_SPEC>;
        ///Register `OCR1C` writer
        pub type W = crate::W<OCR1C_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter1 Output Compare Register C Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR1C_SPEC;
        impl crate::RegisterSpec for OCR1C_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr1c::R`](R) reader structure
        impl crate::Readable for OCR1C_SPEC {}
        ///`write(|w| ..)` method takes [`ocr1c::W`](W) writer structure
        impl crate::Writable for OCR1C_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR1C to value 0
        impl crate::Resettable for OCR1C_SPEC {}
    }
    /**TCCR1A (rw) register accessor: Timer/Counter1 Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr1a`] module*/
    pub type TCCR1A = crate::Reg<tccr1a::TCCR1A_SPEC>;
    ///Timer/Counter1 Control Register A
    pub mod tccr1a {
        ///Register `TCCR1A` reader
        pub type R = crate::R<TCCR1A_SPEC>;
        ///Register `TCCR1A` writer
        pub type W = crate::W<TCCR1A_SPEC>;
        ///Field `WGM1` reader - Waveform Generation Mode
        pub type WGM1_R = crate::FieldReader;
        ///Field `WGM1` writer - Waveform Generation Mode
        pub type WGM1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
        /**Compare Output Mode 1C, bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum COM1C_A {
            ///0: Normal port operation, OCix disconnected
            DISCONNECTED = 0,
            ///1: Toggle OCix on Compare Match (Might depend on WGM)
            MATCH_TOGGLE = 1,
            ///2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            MATCH_CLEAR = 2,
            ///3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            MATCH_SET = 3,
        }
        impl From<COM1C_A> for u8 {
            #[inline(always)]
            fn from(variant: COM1C_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for COM1C_A {
            type Ux = u8;
        }
        impl crate::IsEnum for COM1C_A {}
        ///Field `COM1C` reader - Compare Output Mode 1C, bits
        pub type COM1C_R = crate::FieldReader<COM1C_A>;
        impl COM1C_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> COM1C_A {
                match self.bits {
                    0 => COM1C_A::DISCONNECTED,
                    1 => COM1C_A::MATCH_TOGGLE,
                    2 => COM1C_A::MATCH_CLEAR,
                    3 => COM1C_A::MATCH_SET,
                    _ => unreachable!(),
                }
            }
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn is_disconnected(&self) -> bool {
                *self == COM1C_A::DISCONNECTED
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn is_match_toggle(&self) -> bool {
                *self == COM1C_A::MATCH_TOGGLE
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            #[inline(always)]
            pub fn is_match_clear(&self) -> bool {
                *self == COM1C_A::MATCH_CLEAR
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            #[inline(always)]
            pub fn is_match_set(&self) -> bool {
                *self == COM1C_A::MATCH_SET
            }
        }
        ///Field `COM1C` writer - Compare Output Mode 1C, bits
        pub type COM1C_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COM1C_A, crate::Safe>;
        impl<'a, REG> COM1C_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn disconnected(self) -> &'a mut crate::W<REG> {
                self.variant(COM1C_A::DISCONNECTED)
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn match_toggle(self) -> &'a mut crate::W<REG> {
                self.variant(COM1C_A::MATCH_TOGGLE)
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            #[inline(always)]
            pub fn match_clear(self) -> &'a mut crate::W<REG> {
                self.variant(COM1C_A::MATCH_CLEAR)
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            #[inline(always)]
            pub fn match_set(self) -> &'a mut crate::W<REG> {
                self.variant(COM1C_A::MATCH_SET)
            }
        }
        ///Field `COM1B` reader - Compare Output Mode 1B, bits
        pub use COM1C_R as COM1B_R;
        ///Field `COM1B` writer - Compare Output Mode 1B, bits
        pub use COM1C_W as COM1B_W;
        ///Field `COM1A` reader - Compare Output Mode 1A, bits
        pub use COM1C_R as COM1A_R;
        ///Field `COM1A` writer - Compare Output Mode 1A, bits
        pub use COM1C_W as COM1A_W;
        impl R {
            ///Bits 0:1 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm1(&self) -> WGM1_R {
                WGM1_R::new(self.bits & 3)
            }
            ///Bits 2:3 - Compare Output Mode 1C, bits
            #[inline(always)]
            pub fn com1c(&self) -> COM1C_R {
                COM1C_R::new((self.bits >> 2) & 3)
            }
            ///Bits 4:5 - Compare Output Mode 1B, bits
            #[inline(always)]
            pub fn com1b(&self) -> COM1B_R {
                COM1B_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - Compare Output Mode 1A, bits
            #[inline(always)]
            pub fn com1a(&self) -> COM1A_R {
                COM1A_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bits 0:1 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm1(&mut self) -> WGM1_W<'_, TCCR1A_SPEC> {
                WGM1_W::new(self, 0)
            }
            ///Bits 2:3 - Compare Output Mode 1C, bits
            #[inline(always)]
            pub fn com1c(&mut self) -> COM1C_W<'_, TCCR1A_SPEC> {
                COM1C_W::new(self, 2)
            }
            ///Bits 4:5 - Compare Output Mode 1B, bits
            #[inline(always)]
            pub fn com1b(&mut self) -> COM1B_W<'_, TCCR1A_SPEC> {
                COM1B_W::new(self, 4)
            }
            ///Bits 6:7 - Compare Output Mode 1A, bits
            #[inline(always)]
            pub fn com1a(&mut self) -> COM1A_W<'_, TCCR1A_SPEC> {
                COM1A_W::new(self, 6)
            }
        }
        /**Timer/Counter1 Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr1a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr1a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR1A_SPEC;
        impl crate::RegisterSpec for TCCR1A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr1a::R`](R) reader structure
        impl crate::Readable for TCCR1A_SPEC {}
        ///`write(|w| ..)` method takes [`tccr1a::W`](W) writer structure
        impl crate::Writable for TCCR1A_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR1A to value 0
        impl crate::Resettable for TCCR1A_SPEC {}
    }
    /**TCCR1B (rw) register accessor: Timer/Counter1 Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr1b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr1b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr1b`] module*/
    pub type TCCR1B = crate::Reg<tccr1b::TCCR1B_SPEC>;
    ///Timer/Counter1 Control Register B
    pub mod tccr1b {
        ///Register `TCCR1B` reader
        pub type R = crate::R<TCCR1B_SPEC>;
        ///Register `TCCR1B` writer
        pub type W = crate::W<TCCR1B_SPEC>;
        /**Prescaler source of Timer/Counter 1

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum CS1_A {
            ///0: No clock source (Timer/Counter stopped)
            NO_CLOCK = 0,
            ///1: Running, No Prescaling
            DIRECT = 1,
            ///2: Running, CLK/8
            PRESCALE_8 = 2,
            ///3: Running, CLK/64
            PRESCALE_64 = 3,
            ///4: Running, CLK/256
            PRESCALE_256 = 4,
            ///5: Running, CLK/1024
            PRESCALE_1024 = 5,
            ///6: Running, ExtClk Tx Falling Edge
            EXT_FALLING = 6,
            ///7: Running, ExtClk Tx Rising Edge
            EXT_RISING = 7,
        }
        impl From<CS1_A> for u8 {
            #[inline(always)]
            fn from(variant: CS1_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for CS1_A {
            type Ux = u8;
        }
        impl crate::IsEnum for CS1_A {}
        ///Field `CS1` reader - Prescaler source of Timer/Counter 1
        pub type CS1_R = crate::FieldReader<CS1_A>;
        impl CS1_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> CS1_A {
                match self.bits {
                    0 => CS1_A::NO_CLOCK,
                    1 => CS1_A::DIRECT,
                    2 => CS1_A::PRESCALE_8,
                    3 => CS1_A::PRESCALE_64,
                    4 => CS1_A::PRESCALE_256,
                    5 => CS1_A::PRESCALE_1024,
                    6 => CS1_A::EXT_FALLING,
                    7 => CS1_A::EXT_RISING,
                    _ => unreachable!(),
                }
            }
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn is_no_clock(&self) -> bool {
                *self == CS1_A::NO_CLOCK
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn is_direct(&self) -> bool {
                *self == CS1_A::DIRECT
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn is_prescale_8(&self) -> bool {
                *self == CS1_A::PRESCALE_8
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn is_prescale_64(&self) -> bool {
                *self == CS1_A::PRESCALE_64
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn is_prescale_256(&self) -> bool {
                *self == CS1_A::PRESCALE_256
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn is_prescale_1024(&self) -> bool {
                *self == CS1_A::PRESCALE_1024
            }
            ///Running, ExtClk Tx Falling Edge
            #[inline(always)]
            pub fn is_ext_falling(&self) -> bool {
                *self == CS1_A::EXT_FALLING
            }
            ///Running, ExtClk Tx Rising Edge
            #[inline(always)]
            pub fn is_ext_rising(&self) -> bool {
                *self == CS1_A::EXT_RISING
            }
        }
        ///Field `CS1` writer - Prescaler source of Timer/Counter 1
        pub type CS1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CS1_A, crate::Safe>;
        impl<'a, REG> CS1_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn no_clock(self) -> &'a mut crate::W<REG> {
                self.variant(CS1_A::NO_CLOCK)
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn direct(self) -> &'a mut crate::W<REG> {
                self.variant(CS1_A::DIRECT)
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn prescale_8(self) -> &'a mut crate::W<REG> {
                self.variant(CS1_A::PRESCALE_8)
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn prescale_64(self) -> &'a mut crate::W<REG> {
                self.variant(CS1_A::PRESCALE_64)
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn prescale_256(self) -> &'a mut crate::W<REG> {
                self.variant(CS1_A::PRESCALE_256)
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
                self.variant(CS1_A::PRESCALE_1024)
            }
            ///Running, ExtClk Tx Falling Edge
            #[inline(always)]
            pub fn ext_falling(self) -> &'a mut crate::W<REG> {
                self.variant(CS1_A::EXT_FALLING)
            }
            ///Running, ExtClk Tx Rising Edge
            #[inline(always)]
            pub fn ext_rising(self) -> &'a mut crate::W<REG> {
                self.variant(CS1_A::EXT_RISING)
            }
        }
        ///Field `WGM1` reader - Waveform Generation Mode
        pub type WGM1_R = crate::FieldReader;
        ///Field `WGM1` writer - Waveform Generation Mode
        pub type WGM1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
        ///Field `ICES1` reader - Input Capture 1 Edge Select
        pub type ICES1_R = crate::BitReader;
        ///Field `ICES1` writer - Input Capture 1 Edge Select
        pub type ICES1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICNC1` reader - Input Capture 1 Noise Canceler
        pub type ICNC1_R = crate::BitReader;
        ///Field `ICNC1` writer - Input Capture 1 Noise Canceler
        pub type ICNC1_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:2 - Prescaler source of Timer/Counter 1
            #[inline(always)]
            pub fn cs1(&self) -> CS1_R {
                CS1_R::new(self.bits & 7)
            }
            ///Bits 3:4 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm1(&self) -> WGM1_R {
                WGM1_R::new((self.bits >> 3) & 3)
            }
            ///Bit 6 - Input Capture 1 Edge Select
            #[inline(always)]
            pub fn ices1(&self) -> ICES1_R {
                ICES1_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Input Capture 1 Noise Canceler
            #[inline(always)]
            pub fn icnc1(&self) -> ICNC1_R {
                ICNC1_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:2 - Prescaler source of Timer/Counter 1
            #[inline(always)]
            pub fn cs1(&mut self) -> CS1_W<'_, TCCR1B_SPEC> {
                CS1_W::new(self, 0)
            }
            ///Bits 3:4 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm1(&mut self) -> WGM1_W<'_, TCCR1B_SPEC> {
                WGM1_W::new(self, 3)
            }
            ///Bit 6 - Input Capture 1 Edge Select
            #[inline(always)]
            pub fn ices1(&mut self) -> ICES1_W<'_, TCCR1B_SPEC> {
                ICES1_W::new(self, 6)
            }
            ///Bit 7 - Input Capture 1 Noise Canceler
            #[inline(always)]
            pub fn icnc1(&mut self) -> ICNC1_W<'_, TCCR1B_SPEC> {
                ICNC1_W::new(self, 7)
            }
        }
        /**Timer/Counter1 Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr1b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr1b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR1B_SPEC;
        impl crate::RegisterSpec for TCCR1B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr1b::R`](R) reader structure
        impl crate::Readable for TCCR1B_SPEC {}
        ///`write(|w| ..)` method takes [`tccr1b::W`](W) writer structure
        impl crate::Writable for TCCR1B_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR1B to value 0
        impl crate::Resettable for TCCR1B_SPEC {}
    }
    /**TCCR1C (rw) register accessor: Timer/Counter 1 Control Register C

You can [`read`](crate::Reg::read) this register and get [`tccr1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr1c`] module*/
    pub type TCCR1C = crate::Reg<tccr1c::TCCR1C_SPEC>;
    ///Timer/Counter 1 Control Register C
    pub mod tccr1c {
        ///Register `TCCR1C` reader
        pub type R = crate::R<TCCR1C_SPEC>;
        ///Register `TCCR1C` writer
        pub type W = crate::W<TCCR1C_SPEC>;
        ///Field `FOC1C` writer - Force Output Compare 1C
        pub type FOC1C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC1B` writer - Force Output Compare 1B
        pub type FOC1B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC1A` writer - Force Output Compare 1A
        pub type FOC1A_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl W {
            ///Bit 5 - Force Output Compare 1C
            #[inline(always)]
            pub fn foc1c(&mut self) -> FOC1C_W<'_, TCCR1C_SPEC> {
                FOC1C_W::new(self, 5)
            }
            ///Bit 6 - Force Output Compare 1B
            #[inline(always)]
            pub fn foc1b(&mut self) -> FOC1B_W<'_, TCCR1C_SPEC> {
                FOC1B_W::new(self, 6)
            }
            ///Bit 7 - Force Output Compare 1A
            #[inline(always)]
            pub fn foc1a(&mut self) -> FOC1A_W<'_, TCCR1C_SPEC> {
                FOC1A_W::new(self, 7)
            }
        }
        /**Timer/Counter 1 Control Register C

You can [`read`](crate::Reg::read) this register and get [`tccr1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR1C_SPEC;
        impl crate::RegisterSpec for TCCR1C_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr1c::R`](R) reader structure
        impl crate::Readable for TCCR1C_SPEC {}
        ///`write(|w| ..)` method takes [`tccr1c::W`](W) writer structure
        impl crate::Writable for TCCR1C_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR1C to value 0
        impl crate::Resettable for TCCR1C_SPEC {}
    }
    /**TCNT1 (rw) register accessor: Timer/Counter1 Bytes

You can [`read`](crate::Reg::read) this register and get [`tcnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcnt1`] module*/
    pub type TCNT1 = crate::Reg<tcnt1::TCNT1_SPEC>;
    ///Timer/Counter1 Bytes
    pub mod tcnt1 {
        ///Register `TCNT1` reader
        pub type R = crate::R<TCNT1_SPEC>;
        ///Register `TCNT1` writer
        pub type W = crate::W<TCNT1_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter1 Bytes

You can [`read`](crate::Reg::read) this register and get [`tcnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCNT1_SPEC;
        impl crate::RegisterSpec for TCNT1_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`tcnt1::R`](R) reader structure
        impl crate::Readable for TCNT1_SPEC {}
        ///`write(|w| ..)` method takes [`tcnt1::W`](W) writer structure
        impl crate::Writable for TCNT1_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets TCNT1 to value 0
        impl crate::Resettable for TCNT1_SPEC {}
    }
    /**TIFR1 (rw) register accessor: Timer/Counter1 Interrupt Flag register

You can [`read`](crate::Reg::read) this register and get [`tifr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tifr1`] module*/
    pub type TIFR1 = crate::Reg<tifr1::TIFR1_SPEC>;
    ///Timer/Counter1 Interrupt Flag register
    pub mod tifr1 {
        ///Register `TIFR1` reader
        pub type R = crate::R<TIFR1_SPEC>;
        ///Register `TIFR1` writer
        pub type W = crate::W<TIFR1_SPEC>;
        ///Field `TOV1` reader - Timer/Counter1 Overflow Flag
        pub type TOV1_R = crate::BitReader;
        ///Field `TOV1` writer - Timer/Counter1 Overflow Flag
        pub type TOV1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF1A` reader - Output Compare Flag 1A
        pub type OCF1A_R = crate::BitReader;
        ///Field `OCF1A` writer - Output Compare Flag 1A
        pub type OCF1A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF1B` reader - Output Compare Flag 1B
        pub type OCF1B_R = crate::BitReader;
        ///Field `OCF1B` writer - Output Compare Flag 1B
        pub type OCF1B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF1C` reader - Output Compare Flag 1C
        pub type OCF1C_R = crate::BitReader;
        ///Field `OCF1C` writer - Output Compare Flag 1C
        pub type OCF1C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICF1` reader - Input Capture Flag 1
        pub type ICF1_R = crate::BitReader;
        ///Field `ICF1` writer - Input Capture Flag 1
        pub type ICF1_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter1 Overflow Flag
            #[inline(always)]
            pub fn tov1(&self) -> TOV1_R {
                TOV1_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Output Compare Flag 1A
            #[inline(always)]
            pub fn ocf1a(&self) -> OCF1A_R {
                OCF1A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Output Compare Flag 1B
            #[inline(always)]
            pub fn ocf1b(&self) -> OCF1B_R {
                OCF1B_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Output Compare Flag 1C
            #[inline(always)]
            pub fn ocf1c(&self) -> OCF1C_R {
                OCF1C_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 5 - Input Capture Flag 1
            #[inline(always)]
            pub fn icf1(&self) -> ICF1_R {
                ICF1_R::new(((self.bits >> 5) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter1 Overflow Flag
            #[inline(always)]
            pub fn tov1(&mut self) -> TOV1_W<'_, TIFR1_SPEC> {
                TOV1_W::new(self, 0)
            }
            ///Bit 1 - Output Compare Flag 1A
            #[inline(always)]
            pub fn ocf1a(&mut self) -> OCF1A_W<'_, TIFR1_SPEC> {
                OCF1A_W::new(self, 1)
            }
            ///Bit 2 - Output Compare Flag 1B
            #[inline(always)]
            pub fn ocf1b(&mut self) -> OCF1B_W<'_, TIFR1_SPEC> {
                OCF1B_W::new(self, 2)
            }
            ///Bit 3 - Output Compare Flag 1C
            #[inline(always)]
            pub fn ocf1c(&mut self) -> OCF1C_W<'_, TIFR1_SPEC> {
                OCF1C_W::new(self, 3)
            }
            ///Bit 5 - Input Capture Flag 1
            #[inline(always)]
            pub fn icf1(&mut self) -> ICF1_W<'_, TIFR1_SPEC> {
                ICF1_W::new(self, 5)
            }
        }
        /**Timer/Counter1 Interrupt Flag register

You can [`read`](crate::Reg::read) this register and get [`tifr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIFR1_SPEC;
        impl crate::RegisterSpec for TIFR1_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tifr1::R`](R) reader structure
        impl crate::Readable for TIFR1_SPEC {}
        ///`write(|w| ..)` method takes [`tifr1::W`](W) writer structure
        impl crate::Writable for TIFR1_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIFR1 to value 0
        impl crate::Resettable for TIFR1_SPEC {}
    }
    /**TIMSK1 (rw) register accessor: Timer/Counter1 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`timsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timsk1`] module*/
    pub type TIMSK1 = crate::Reg<timsk1::TIMSK1_SPEC>;
    ///Timer/Counter1 Interrupt Mask Register
    pub mod timsk1 {
        ///Register `TIMSK1` reader
        pub type R = crate::R<TIMSK1_SPEC>;
        ///Register `TIMSK1` writer
        pub type W = crate::W<TIMSK1_SPEC>;
        ///Field `TOIE1` reader - Timer/Counter1 Overflow Interrupt Enable
        pub type TOIE1_R = crate::BitReader;
        ///Field `TOIE1` writer - Timer/Counter1 Overflow Interrupt Enable
        pub type TOIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE1A` reader - Timer/Counter1 Output Compare A Match Interrupt Enable
        pub type OCIE1A_R = crate::BitReader;
        ///Field `OCIE1A` writer - Timer/Counter1 Output Compare A Match Interrupt Enable
        pub type OCIE1A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE1B` reader - Timer/Counter1 Output Compare B Match Interrupt Enable
        pub type OCIE1B_R = crate::BitReader;
        ///Field `OCIE1B` writer - Timer/Counter1 Output Compare B Match Interrupt Enable
        pub type OCIE1B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE1C` reader - Timer/Counter1 Output Compare C Match Interrupt Enable
        pub type OCIE1C_R = crate::BitReader;
        ///Field `OCIE1C` writer - Timer/Counter1 Output Compare C Match Interrupt Enable
        pub type OCIE1C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICIE1` reader - Timer/Counter1 Input Capture Interrupt Enable
        pub type ICIE1_R = crate::BitReader;
        ///Field `ICIE1` writer - Timer/Counter1 Input Capture Interrupt Enable
        pub type ICIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter1 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie1(&self) -> TOIE1_R {
                TOIE1_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Timer/Counter1 Output Compare A Match Interrupt Enable
            #[inline(always)]
            pub fn ocie1a(&self) -> OCIE1A_R {
                OCIE1A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Timer/Counter1 Output Compare B Match Interrupt Enable
            #[inline(always)]
            pub fn ocie1b(&self) -> OCIE1B_R {
                OCIE1B_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Timer/Counter1 Output Compare C Match Interrupt Enable
            #[inline(always)]
            pub fn ocie1c(&self) -> OCIE1C_R {
                OCIE1C_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 5 - Timer/Counter1 Input Capture Interrupt Enable
            #[inline(always)]
            pub fn icie1(&self) -> ICIE1_R {
                ICIE1_R::new(((self.bits >> 5) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter1 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie1(&mut self) -> TOIE1_W<'_, TIMSK1_SPEC> {
                TOIE1_W::new(self, 0)
            }
            ///Bit 1 - Timer/Counter1 Output Compare A Match Interrupt Enable
            #[inline(always)]
            pub fn ocie1a(&mut self) -> OCIE1A_W<'_, TIMSK1_SPEC> {
                OCIE1A_W::new(self, 1)
            }
            ///Bit 2 - Timer/Counter1 Output Compare B Match Interrupt Enable
            #[inline(always)]
            pub fn ocie1b(&mut self) -> OCIE1B_W<'_, TIMSK1_SPEC> {
                OCIE1B_W::new(self, 2)
            }
            ///Bit 3 - Timer/Counter1 Output Compare C Match Interrupt Enable
            #[inline(always)]
            pub fn ocie1c(&mut self) -> OCIE1C_W<'_, TIMSK1_SPEC> {
                OCIE1C_W::new(self, 3)
            }
            ///Bit 5 - Timer/Counter1 Input Capture Interrupt Enable
            #[inline(always)]
            pub fn icie1(&mut self) -> ICIE1_W<'_, TIMSK1_SPEC> {
                ICIE1_W::new(self, 5)
            }
        }
        /**Timer/Counter1 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`timsk1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIMSK1_SPEC;
        impl crate::RegisterSpec for TIMSK1_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`timsk1::R`](R) reader structure
        impl crate::Readable for TIMSK1_SPEC {}
        ///`write(|w| ..)` method takes [`timsk1::W`](W) writer structure
        impl crate::Writable for TIMSK1_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIMSK1 to value 0
        impl crate::Resettable for TIMSK1_SPEC {}
    }
}
///Timer/Counter, 8-bit Async
pub type TC2 = crate::Periph<tc2::RegisterBlock, 0x37>;
impl core::fmt::Debug for TC2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC2").finish()
    }
}
///Timer/Counter, 8-bit Async
pub mod tc2 {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        tifr2: TIFR2,
        _reserved1: [u8; 0x0b],
        gtccr: GTCCR,
        _reserved2: [u8; 0x2c],
        timsk2: TIMSK2,
        _reserved3: [u8; 0x3f],
        tccr2a: TCCR2A,
        tccr2b: TCCR2B,
        tcnt2: TCNT2,
        ocr2a: OCR2A,
        ocr2b: OCR2B,
        _reserved8: [u8; 0x01],
        assr: ASSR,
    }
    impl RegisterBlock {
        ///0x00 - Timer/Counter Interrupt Flag Register
        #[inline(always)]
        pub const fn tifr2(&self) -> &TIFR2 {
            &self.tifr2
        }
        ///0x0c - General Timer Counter Control register
        #[inline(always)]
        pub const fn gtccr(&self) -> &GTCCR {
            &self.gtccr
        }
        ///0x39 - Timer/Counter Interrupt Mask register
        #[inline(always)]
        pub const fn timsk2(&self) -> &TIMSK2 {
            &self.timsk2
        }
        ///0x79 - Timer/Counter2 Control Register A
        #[inline(always)]
        pub const fn tccr2a(&self) -> &TCCR2A {
            &self.tccr2a
        }
        ///0x7a - Timer/Counter2 Control Register B
        #[inline(always)]
        pub const fn tccr2b(&self) -> &TCCR2B {
            &self.tccr2b
        }
        ///0x7b - Timer/Counter2
        #[inline(always)]
        pub const fn tcnt2(&self) -> &TCNT2 {
            &self.tcnt2
        }
        ///0x7c - Timer/Counter2 Output Compare Register A
        #[inline(always)]
        pub const fn ocr2a(&self) -> &OCR2A {
            &self.ocr2a
        }
        ///0x7d - Timer/Counter2 Output Compare Register B
        #[inline(always)]
        pub const fn ocr2b(&self) -> &OCR2B {
            &self.ocr2b
        }
        ///0x7f - Asynchronous Status Register
        #[inline(always)]
        pub const fn assr(&self) -> &ASSR {
            &self.assr
        }
    }
    /**ASSR (rw) register accessor: Asynchronous Status Register

You can [`read`](crate::Reg::read) this register and get [`assr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`assr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@assr`] module*/
    pub type ASSR = crate::Reg<assr::ASSR_SPEC>;
    ///Asynchronous Status Register
    pub mod assr {
        ///Register `ASSR` reader
        pub type R = crate::R<ASSR_SPEC>;
        ///Register `ASSR` writer
        pub type W = crate::W<ASSR_SPEC>;
        ///Field `TCR2BUB` reader - Timer/Counter Control Register2 Update Busy
        pub type TCR2BUB_R = crate::BitReader;
        ///Field `TCR2BUB` writer - Timer/Counter Control Register2 Update Busy
        pub type TCR2BUB_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TCR2AUB` reader - Timer/Counter Control Register2 Update Busy
        pub type TCR2AUB_R = crate::BitReader;
        ///Field `TCR2AUB` writer - Timer/Counter Control Register2 Update Busy
        pub type TCR2AUB_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCR2BUB` reader - Output Compare Register 2 Update Busy
        pub type OCR2BUB_R = crate::BitReader;
        ///Field `OCR2BUB` writer - Output Compare Register 2 Update Busy
        pub type OCR2BUB_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCR2AUB` reader - Output Compare Register2 Update Busy
        pub type OCR2AUB_R = crate::BitReader;
        ///Field `OCR2AUB` writer - Output Compare Register2 Update Busy
        pub type OCR2AUB_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TCN2UB` reader - Timer/Counter2 Update Busy
        pub type TCN2UB_R = crate::BitReader;
        ///Field `TCN2UB` writer - Timer/Counter2 Update Busy
        pub type TCN2UB_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `AS2` reader - Asynchronous Timer/Counter2
        pub type AS2_R = crate::BitReader;
        ///Field `AS2` writer - Asynchronous Timer/Counter2
        pub type AS2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `EXCLK` reader - Enable External Clock Input
        pub type EXCLK_R = crate::BitReader;
        ///Field `EXCLK` writer - Enable External Clock Input
        pub type EXCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter Control Register2 Update Busy
            #[inline(always)]
            pub fn tcr2bub(&self) -> TCR2BUB_R {
                TCR2BUB_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Timer/Counter Control Register2 Update Busy
            #[inline(always)]
            pub fn tcr2aub(&self) -> TCR2AUB_R {
                TCR2AUB_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Output Compare Register 2 Update Busy
            #[inline(always)]
            pub fn ocr2bub(&self) -> OCR2BUB_R {
                OCR2BUB_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Output Compare Register2 Update Busy
            #[inline(always)]
            pub fn ocr2aub(&self) -> OCR2AUB_R {
                OCR2AUB_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Timer/Counter2 Update Busy
            #[inline(always)]
            pub fn tcn2ub(&self) -> TCN2UB_R {
                TCN2UB_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Asynchronous Timer/Counter2
            #[inline(always)]
            pub fn as2(&self) -> AS2_R {
                AS2_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Enable External Clock Input
            #[inline(always)]
            pub fn exclk(&self) -> EXCLK_R {
                EXCLK_R::new(((self.bits >> 6) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter Control Register2 Update Busy
            #[inline(always)]
            pub fn tcr2bub(&mut self) -> TCR2BUB_W<'_, ASSR_SPEC> {
                TCR2BUB_W::new(self, 0)
            }
            ///Bit 1 - Timer/Counter Control Register2 Update Busy
            #[inline(always)]
            pub fn tcr2aub(&mut self) -> TCR2AUB_W<'_, ASSR_SPEC> {
                TCR2AUB_W::new(self, 1)
            }
            ///Bit 2 - Output Compare Register 2 Update Busy
            #[inline(always)]
            pub fn ocr2bub(&mut self) -> OCR2BUB_W<'_, ASSR_SPEC> {
                OCR2BUB_W::new(self, 2)
            }
            ///Bit 3 - Output Compare Register2 Update Busy
            #[inline(always)]
            pub fn ocr2aub(&mut self) -> OCR2AUB_W<'_, ASSR_SPEC> {
                OCR2AUB_W::new(self, 3)
            }
            ///Bit 4 - Timer/Counter2 Update Busy
            #[inline(always)]
            pub fn tcn2ub(&mut self) -> TCN2UB_W<'_, ASSR_SPEC> {
                TCN2UB_W::new(self, 4)
            }
            ///Bit 5 - Asynchronous Timer/Counter2
            #[inline(always)]
            pub fn as2(&mut self) -> AS2_W<'_, ASSR_SPEC> {
                AS2_W::new(self, 5)
            }
            ///Bit 6 - Enable External Clock Input
            #[inline(always)]
            pub fn exclk(&mut self) -> EXCLK_W<'_, ASSR_SPEC> {
                EXCLK_W::new(self, 6)
            }
        }
        /**Asynchronous Status Register

You can [`read`](crate::Reg::read) this register and get [`assr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`assr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ASSR_SPEC;
        impl crate::RegisterSpec for ASSR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`assr::R`](R) reader structure
        impl crate::Readable for ASSR_SPEC {}
        ///`write(|w| ..)` method takes [`assr::W`](W) writer structure
        impl crate::Writable for ASSR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets ASSR to value 0
        impl crate::Resettable for ASSR_SPEC {}
    }
    /**GTCCR (rw) register accessor: General Timer Counter Control register

You can [`read`](crate::Reg::read) this register and get [`gtccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccr`] module*/
    pub type GTCCR = crate::Reg<gtccr::GTCCR_SPEC>;
    ///General Timer Counter Control register
    pub mod gtccr {
        ///Register `GTCCR` reader
        pub type R = crate::R<GTCCR_SPEC>;
        ///Register `GTCCR` writer
        pub type W = crate::W<GTCCR_SPEC>;
        ///Field `PSRASY` reader - Prescaler Reset Timer/Counter2
        pub type PSRASY_R = crate::BitReader;
        ///Field `PSRASY` writer - Prescaler Reset Timer/Counter2
        pub type PSRASY_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TSM` reader - Timer/Counter Synchronization Mode
        pub type TSM_R = crate::BitReader;
        ///Field `TSM` writer - Timer/Counter Synchronization Mode
        pub type TSM_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 1 - Prescaler Reset Timer/Counter2
            #[inline(always)]
            pub fn psrasy(&self) -> PSRASY_R {
                PSRASY_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 7 - Timer/Counter Synchronization Mode
            #[inline(always)]
            pub fn tsm(&self) -> TSM_R {
                TSM_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 1 - Prescaler Reset Timer/Counter2
            #[inline(always)]
            pub fn psrasy(&mut self) -> PSRASY_W<'_, GTCCR_SPEC> {
                PSRASY_W::new(self, 1)
            }
            ///Bit 7 - Timer/Counter Synchronization Mode
            #[inline(always)]
            pub fn tsm(&mut self) -> TSM_W<'_, GTCCR_SPEC> {
                TSM_W::new(self, 7)
            }
        }
        /**General Timer Counter Control register

You can [`read`](crate::Reg::read) this register and get [`gtccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct GTCCR_SPEC;
        impl crate::RegisterSpec for GTCCR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`gtccr::R`](R) reader structure
        impl crate::Readable for GTCCR_SPEC {}
        ///`write(|w| ..)` method takes [`gtccr::W`](W) writer structure
        impl crate::Writable for GTCCR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets GTCCR to value 0
        impl crate::Resettable for GTCCR_SPEC {}
    }
    /**OCR2A (rw) register accessor: Timer/Counter2 Output Compare Register A

You can [`read`](crate::Reg::read) this register and get [`ocr2a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr2a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr2a`] module*/
    pub type OCR2A = crate::Reg<ocr2a::OCR2A_SPEC>;
    ///Timer/Counter2 Output Compare Register A
    pub mod ocr2a {
        ///Register `OCR2A` reader
        pub type R = crate::R<OCR2A_SPEC>;
        ///Register `OCR2A` writer
        pub type W = crate::W<OCR2A_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter2 Output Compare Register A

You can [`read`](crate::Reg::read) this register and get [`ocr2a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr2a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR2A_SPEC;
        impl crate::RegisterSpec for OCR2A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ocr2a::R`](R) reader structure
        impl crate::Readable for OCR2A_SPEC {}
        ///`write(|w| ..)` method takes [`ocr2a::W`](W) writer structure
        impl crate::Writable for OCR2A_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR2A to value 0
        impl crate::Resettable for OCR2A_SPEC {}
    }
    /**OCR2B (rw) register accessor: Timer/Counter2 Output Compare Register B

You can [`read`](crate::Reg::read) this register and get [`ocr2b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr2b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr2b`] module*/
    pub type OCR2B = crate::Reg<ocr2b::OCR2B_SPEC>;
    ///Timer/Counter2 Output Compare Register B
    pub mod ocr2b {
        ///Register `OCR2B` reader
        pub type R = crate::R<OCR2B_SPEC>;
        ///Register `OCR2B` writer
        pub type W = crate::W<OCR2B_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter2 Output Compare Register B

You can [`read`](crate::Reg::read) this register and get [`ocr2b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr2b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR2B_SPEC;
        impl crate::RegisterSpec for OCR2B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ocr2b::R`](R) reader structure
        impl crate::Readable for OCR2B_SPEC {}
        ///`write(|w| ..)` method takes [`ocr2b::W`](W) writer structure
        impl crate::Writable for OCR2B_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR2B to value 0
        impl crate::Resettable for OCR2B_SPEC {}
    }
    /**TCCR2A (rw) register accessor: Timer/Counter2 Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr2a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr2a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr2a`] module*/
    pub type TCCR2A = crate::Reg<tccr2a::TCCR2A_SPEC>;
    ///Timer/Counter2 Control Register A
    pub mod tccr2a {
        ///Register `TCCR2A` reader
        pub type R = crate::R<TCCR2A_SPEC>;
        ///Register `TCCR2A` writer
        pub type W = crate::W<TCCR2A_SPEC>;
        /**Waveform Genration Mode

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum WGM2_A {
            ///0: Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*
            NORMAL_TOP = 0,
            ///1: Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*
            PWM_PHASE = 1,
            ///2: CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*
            CTC = 2,
            ///3: Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*
            PWM_FAST = 3,
        }
        impl From<WGM2_A> for u8 {
            #[inline(always)]
            fn from(variant: WGM2_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for WGM2_A {
            type Ux = u8;
        }
        impl crate::IsEnum for WGM2_A {}
        ///Field `WGM2` reader - Waveform Genration Mode
        pub type WGM2_R = crate::FieldReader<WGM2_A>;
        impl WGM2_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> WGM2_A {
                match self.bits {
                    0 => WGM2_A::NORMAL_TOP,
                    1 => WGM2_A::PWM_PHASE,
                    2 => WGM2_A::CTC,
                    3 => WGM2_A::PWM_FAST,
                    _ => unreachable!(),
                }
            }
            ///Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*
            #[inline(always)]
            pub fn is_normal_top(&self) -> bool {
                *self == WGM2_A::NORMAL_TOP
            }
            ///Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*
            #[inline(always)]
            pub fn is_pwm_phase(&self) -> bool {
                *self == WGM2_A::PWM_PHASE
            }
            ///CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*
            #[inline(always)]
            pub fn is_ctc(&self) -> bool {
                *self == WGM2_A::CTC
            }
            ///Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*
            #[inline(always)]
            pub fn is_pwm_fast(&self) -> bool {
                *self == WGM2_A::PWM_FAST
            }
        }
        ///Field `WGM2` writer - Waveform Genration Mode
        pub type WGM2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WGM2_A, crate::Safe>;
        impl<'a, REG> WGM2_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Normal, Top: `0xff`, Update: *Immediate*, Flag: *MAX*
            #[inline(always)]
            pub fn normal_top(self) -> &'a mut crate::W<REG> {
                self.variant(WGM2_A::NORMAL_TOP)
            }
            ///Phase Correct PWM, Top: `0xff`, Update: *TOP*, Flag: *BOTTOM*
            #[inline(always)]
            pub fn pwm_phase(self) -> &'a mut crate::W<REG> {
                self.variant(WGM2_A::PWM_PHASE)
            }
            ///CTC, Top: *OCRA*, Update: *Immediate*, Flag: *MAX*
            #[inline(always)]
            pub fn ctc(self) -> &'a mut crate::W<REG> {
                self.variant(WGM2_A::CTC)
            }
            ///Fast PWM, Top: `0xff`, Update: *TOP*, Flag: *MAX*
            #[inline(always)]
            pub fn pwm_fast(self) -> &'a mut crate::W<REG> {
                self.variant(WGM2_A::PWM_FAST)
            }
        }
        /**Compare Output B Mode

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum COM2B_A {
            ///0: Normal port operation, OCix disconnected
            DISCONNECTED = 0,
            ///1: Toggle OCix on Compare Match (Might depend on WGM)
            MATCH_TOGGLE = 1,
            ///2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)
            MATCH_CLEAR = 2,
            ///3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)
            MATCH_SET = 3,
        }
        impl From<COM2B_A> for u8 {
            #[inline(always)]
            fn from(variant: COM2B_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for COM2B_A {
            type Ux = u8;
        }
        impl crate::IsEnum for COM2B_A {}
        ///Field `COM2B` reader - Compare Output B Mode
        pub type COM2B_R = crate::FieldReader<COM2B_A>;
        impl COM2B_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> COM2B_A {
                match self.bits {
                    0 => COM2B_A::DISCONNECTED,
                    1 => COM2B_A::MATCH_TOGGLE,
                    2 => COM2B_A::MATCH_CLEAR,
                    3 => COM2B_A::MATCH_SET,
                    _ => unreachable!(),
                }
            }
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn is_disconnected(&self) -> bool {
                *self == COM2B_A::DISCONNECTED
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn is_match_toggle(&self) -> bool {
                *self == COM2B_A::MATCH_TOGGLE
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)
            #[inline(always)]
            pub fn is_match_clear(&self) -> bool {
                *self == COM2B_A::MATCH_CLEAR
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)
            #[inline(always)]
            pub fn is_match_set(&self) -> bool {
                *self == COM2B_A::MATCH_SET
            }
        }
        ///Field `COM2B` writer - Compare Output B Mode
        pub type COM2B_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COM2B_A, crate::Safe>;
        impl<'a, REG> COM2B_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn disconnected(self) -> &'a mut crate::W<REG> {
                self.variant(COM2B_A::DISCONNECTED)
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn match_toggle(self) -> &'a mut crate::W<REG> {
                self.variant(COM2B_A::MATCH_TOGGLE)
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at TOP)
            #[inline(always)]
            pub fn match_clear(self) -> &'a mut crate::W<REG> {
                self.variant(COM2B_A::MATCH_CLEAR)
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at TOP)
            #[inline(always)]
            pub fn match_set(self) -> &'a mut crate::W<REG> {
                self.variant(COM2B_A::MATCH_SET)
            }
        }
        ///Field `COM2A` reader - Compare Output A Mode
        pub use COM2B_R as COM2A_R;
        ///Field `COM2A` writer - Compare Output A Mode
        pub use COM2B_W as COM2A_W;
        impl R {
            ///Bits 0:1 - Waveform Genration Mode
            #[inline(always)]
            pub fn wgm2(&self) -> WGM2_R {
                WGM2_R::new(self.bits & 3)
            }
            ///Bits 4:5 - Compare Output B Mode
            #[inline(always)]
            pub fn com2b(&self) -> COM2B_R {
                COM2B_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - Compare Output A Mode
            #[inline(always)]
            pub fn com2a(&self) -> COM2A_R {
                COM2A_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bits 0:1 - Waveform Genration Mode
            #[inline(always)]
            pub fn wgm2(&mut self) -> WGM2_W<'_, TCCR2A_SPEC> {
                WGM2_W::new(self, 0)
            }
            ///Bits 4:5 - Compare Output B Mode
            #[inline(always)]
            pub fn com2b(&mut self) -> COM2B_W<'_, TCCR2A_SPEC> {
                COM2B_W::new(self, 4)
            }
            ///Bits 6:7 - Compare Output A Mode
            #[inline(always)]
            pub fn com2a(&mut self) -> COM2A_W<'_, TCCR2A_SPEC> {
                COM2A_W::new(self, 6)
            }
        }
        /**Timer/Counter2 Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr2a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr2a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR2A_SPEC;
        impl crate::RegisterSpec for TCCR2A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr2a::R`](R) reader structure
        impl crate::Readable for TCCR2A_SPEC {}
        ///`write(|w| ..)` method takes [`tccr2a::W`](W) writer structure
        impl crate::Writable for TCCR2A_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR2A to value 0
        impl crate::Resettable for TCCR2A_SPEC {}
    }
    /**TCCR2B (rw) register accessor: Timer/Counter2 Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr2b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr2b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr2b`] module*/
    pub type TCCR2B = crate::Reg<tccr2b::TCCR2B_SPEC>;
    ///Timer/Counter2 Control Register B
    pub mod tccr2b {
        ///Register `TCCR2B` reader
        pub type R = crate::R<TCCR2B_SPEC>;
        ///Register `TCCR2B` writer
        pub type W = crate::W<TCCR2B_SPEC>;
        /**Clock Select bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum CS2_A {
            ///0: No clock source (Timer/Counter stopped)
            NO_CLOCK = 0,
            ///1: Running, No Prescaling
            DIRECT = 1,
            ///2: Running, CLK/8
            PRESCALE_8 = 2,
            ///3: Running, CLK/32
            PRESCALE_32 = 3,
            ///4: Running, CLK/64
            PRESCALE_64 = 4,
            ///5: Running, CLK/128
            PRESCALE_128 = 5,
            ///6: Running, CLK/256
            PRESCALE_256 = 6,
            ///7: Running, CLK/1024
            PRESCALE_1024 = 7,
        }
        impl From<CS2_A> for u8 {
            #[inline(always)]
            fn from(variant: CS2_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for CS2_A {
            type Ux = u8;
        }
        impl crate::IsEnum for CS2_A {}
        ///Field `CS2` reader - Clock Select bits
        pub type CS2_R = crate::FieldReader<CS2_A>;
        impl CS2_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> CS2_A {
                match self.bits {
                    0 => CS2_A::NO_CLOCK,
                    1 => CS2_A::DIRECT,
                    2 => CS2_A::PRESCALE_8,
                    3 => CS2_A::PRESCALE_32,
                    4 => CS2_A::PRESCALE_64,
                    5 => CS2_A::PRESCALE_128,
                    6 => CS2_A::PRESCALE_256,
                    7 => CS2_A::PRESCALE_1024,
                    _ => unreachable!(),
                }
            }
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn is_no_clock(&self) -> bool {
                *self == CS2_A::NO_CLOCK
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn is_direct(&self) -> bool {
                *self == CS2_A::DIRECT
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn is_prescale_8(&self) -> bool {
                *self == CS2_A::PRESCALE_8
            }
            ///Running, CLK/32
            #[inline(always)]
            pub fn is_prescale_32(&self) -> bool {
                *self == CS2_A::PRESCALE_32
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn is_prescale_64(&self) -> bool {
                *self == CS2_A::PRESCALE_64
            }
            ///Running, CLK/128
            #[inline(always)]
            pub fn is_prescale_128(&self) -> bool {
                *self == CS2_A::PRESCALE_128
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn is_prescale_256(&self) -> bool {
                *self == CS2_A::PRESCALE_256
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn is_prescale_1024(&self) -> bool {
                *self == CS2_A::PRESCALE_1024
            }
        }
        ///Field `CS2` writer - Clock Select bits
        pub type CS2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CS2_A, crate::Safe>;
        impl<'a, REG> CS2_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn no_clock(self) -> &'a mut crate::W<REG> {
                self.variant(CS2_A::NO_CLOCK)
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn direct(self) -> &'a mut crate::W<REG> {
                self.variant(CS2_A::DIRECT)
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn prescale_8(self) -> &'a mut crate::W<REG> {
                self.variant(CS2_A::PRESCALE_8)
            }
            ///Running, CLK/32
            #[inline(always)]
            pub fn prescale_32(self) -> &'a mut crate::W<REG> {
                self.variant(CS2_A::PRESCALE_32)
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn prescale_64(self) -> &'a mut crate::W<REG> {
                self.variant(CS2_A::PRESCALE_64)
            }
            ///Running, CLK/128
            #[inline(always)]
            pub fn prescale_128(self) -> &'a mut crate::W<REG> {
                self.variant(CS2_A::PRESCALE_128)
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn prescale_256(self) -> &'a mut crate::W<REG> {
                self.variant(CS2_A::PRESCALE_256)
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
                self.variant(CS2_A::PRESCALE_1024)
            }
        }
        ///Field `WGM22` reader - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)
        pub type WGM22_R = crate::BitReader;
        ///Field `WGM22` writer - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)
        pub type WGM22_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC2B` writer - Force Output Compare B
        pub type FOC2B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC2A` writer - Force Output Compare A
        pub type FOC2A_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:2 - Clock Select bits
            #[inline(always)]
            pub fn cs2(&self) -> CS2_R {
                CS2_R::new(self.bits & 7)
            }
            ///Bit 3 - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)
            #[inline(always)]
            pub fn wgm22(&self) -> WGM22_R {
                WGM22_R::new(((self.bits >> 3) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:2 - Clock Select bits
            #[inline(always)]
            pub fn cs2(&mut self) -> CS2_W<'_, TCCR2B_SPEC> {
                CS2_W::new(self, 0)
            }
            ///Bit 3 - Waveform Generation Mode High Bit (Enable Top: *OCRA* for `PWM` modes)
            #[inline(always)]
            pub fn wgm22(&mut self) -> WGM22_W<'_, TCCR2B_SPEC> {
                WGM22_W::new(self, 3)
            }
            ///Bit 6 - Force Output Compare B
            #[inline(always)]
            pub fn foc2b(&mut self) -> FOC2B_W<'_, TCCR2B_SPEC> {
                FOC2B_W::new(self, 6)
            }
            ///Bit 7 - Force Output Compare A
            #[inline(always)]
            pub fn foc2a(&mut self) -> FOC2A_W<'_, TCCR2B_SPEC> {
                FOC2A_W::new(self, 7)
            }
        }
        /**Timer/Counter2 Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr2b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr2b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR2B_SPEC;
        impl crate::RegisterSpec for TCCR2B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr2b::R`](R) reader structure
        impl crate::Readable for TCCR2B_SPEC {}
        ///`write(|w| ..)` method takes [`tccr2b::W`](W) writer structure
        impl crate::Writable for TCCR2B_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR2B to value 0
        impl crate::Resettable for TCCR2B_SPEC {}
    }
    /**TCNT2 (rw) register accessor: Timer/Counter2

You can [`read`](crate::Reg::read) this register and get [`tcnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcnt2`] module*/
    pub type TCNT2 = crate::Reg<tcnt2::TCNT2_SPEC>;
    ///Timer/Counter2
    pub mod tcnt2 {
        ///Register `TCNT2` reader
        pub type R = crate::R<TCNT2_SPEC>;
        ///Register `TCNT2` writer
        pub type W = crate::W<TCNT2_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter2

You can [`read`](crate::Reg::read) this register and get [`tcnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCNT2_SPEC;
        impl crate::RegisterSpec for TCNT2_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tcnt2::R`](R) reader structure
        impl crate::Readable for TCNT2_SPEC {}
        ///`write(|w| ..)` method takes [`tcnt2::W`](W) writer structure
        impl crate::Writable for TCNT2_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets TCNT2 to value 0
        impl crate::Resettable for TCNT2_SPEC {}
    }
    /**TIFR2 (rw) register accessor: Timer/Counter Interrupt Flag Register

You can [`read`](crate::Reg::read) this register and get [`tifr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tifr2`] module*/
    pub type TIFR2 = crate::Reg<tifr2::TIFR2_SPEC>;
    ///Timer/Counter Interrupt Flag Register
    pub mod tifr2 {
        ///Register `TIFR2` reader
        pub type R = crate::R<TIFR2_SPEC>;
        ///Register `TIFR2` writer
        pub type W = crate::W<TIFR2_SPEC>;
        ///Field `TOV2` reader - Timer/Counter2 Overflow Flag
        pub type TOV2_R = crate::BitReader;
        ///Field `TOV2` writer - Timer/Counter2 Overflow Flag
        pub type TOV2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF2A` reader - Output Compare Flag 2A
        pub type OCF2A_R = crate::BitReader;
        ///Field `OCF2A` writer - Output Compare Flag 2A
        pub type OCF2A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF2B` reader - Output Compare Flag 2B
        pub type OCF2B_R = crate::BitReader;
        ///Field `OCF2B` writer - Output Compare Flag 2B
        pub type OCF2B_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter2 Overflow Flag
            #[inline(always)]
            pub fn tov2(&self) -> TOV2_R {
                TOV2_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Output Compare Flag 2A
            #[inline(always)]
            pub fn ocf2a(&self) -> OCF2A_R {
                OCF2A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Output Compare Flag 2B
            #[inline(always)]
            pub fn ocf2b(&self) -> OCF2B_R {
                OCF2B_R::new(((self.bits >> 2) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter2 Overflow Flag
            #[inline(always)]
            pub fn tov2(&mut self) -> TOV2_W<'_, TIFR2_SPEC> {
                TOV2_W::new(self, 0)
            }
            ///Bit 1 - Output Compare Flag 2A
            #[inline(always)]
            pub fn ocf2a(&mut self) -> OCF2A_W<'_, TIFR2_SPEC> {
                OCF2A_W::new(self, 1)
            }
            ///Bit 2 - Output Compare Flag 2B
            #[inline(always)]
            pub fn ocf2b(&mut self) -> OCF2B_W<'_, TIFR2_SPEC> {
                OCF2B_W::new(self, 2)
            }
        }
        /**Timer/Counter Interrupt Flag Register

You can [`read`](crate::Reg::read) this register and get [`tifr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIFR2_SPEC;
        impl crate::RegisterSpec for TIFR2_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tifr2::R`](R) reader structure
        impl crate::Readable for TIFR2_SPEC {}
        ///`write(|w| ..)` method takes [`tifr2::W`](W) writer structure
        impl crate::Writable for TIFR2_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIFR2 to value 0
        impl crate::Resettable for TIFR2_SPEC {}
    }
    /**TIMSK2 (rw) register accessor: Timer/Counter Interrupt Mask register

You can [`read`](crate::Reg::read) this register and get [`timsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timsk2`] module*/
    pub type TIMSK2 = crate::Reg<timsk2::TIMSK2_SPEC>;
    ///Timer/Counter Interrupt Mask register
    pub mod timsk2 {
        ///Register `TIMSK2` reader
        pub type R = crate::R<TIMSK2_SPEC>;
        ///Register `TIMSK2` writer
        pub type W = crate::W<TIMSK2_SPEC>;
        ///Field `TOIE2` reader - Timer/Counter2 Overflow Interrupt Enable
        pub type TOIE2_R = crate::BitReader;
        ///Field `TOIE2` writer - Timer/Counter2 Overflow Interrupt Enable
        pub type TOIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE2A` reader - Timer/Counter2 Output Compare Match A Interrupt Enable
        pub type OCIE2A_R = crate::BitReader;
        ///Field `OCIE2A` writer - Timer/Counter2 Output Compare Match A Interrupt Enable
        pub type OCIE2A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE2B` reader - Timer/Counter2 Output Compare Match B Interrupt Enable
        pub type OCIE2B_R = crate::BitReader;
        ///Field `OCIE2B` writer - Timer/Counter2 Output Compare Match B Interrupt Enable
        pub type OCIE2B_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter2 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie2(&self) -> TOIE2_R {
                TOIE2_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Timer/Counter2 Output Compare Match A Interrupt Enable
            #[inline(always)]
            pub fn ocie2a(&self) -> OCIE2A_R {
                OCIE2A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Timer/Counter2 Output Compare Match B Interrupt Enable
            #[inline(always)]
            pub fn ocie2b(&self) -> OCIE2B_R {
                OCIE2B_R::new(((self.bits >> 2) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter2 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie2(&mut self) -> TOIE2_W<'_, TIMSK2_SPEC> {
                TOIE2_W::new(self, 0)
            }
            ///Bit 1 - Timer/Counter2 Output Compare Match A Interrupt Enable
            #[inline(always)]
            pub fn ocie2a(&mut self) -> OCIE2A_W<'_, TIMSK2_SPEC> {
                OCIE2A_W::new(self, 1)
            }
            ///Bit 2 - Timer/Counter2 Output Compare Match B Interrupt Enable
            #[inline(always)]
            pub fn ocie2b(&mut self) -> OCIE2B_W<'_, TIMSK2_SPEC> {
                OCIE2B_W::new(self, 2)
            }
        }
        /**Timer/Counter Interrupt Mask register

You can [`read`](crate::Reg::read) this register and get [`timsk2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIMSK2_SPEC;
        impl crate::RegisterSpec for TIMSK2_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`timsk2::R`](R) reader structure
        impl crate::Readable for TIMSK2_SPEC {}
        ///`write(|w| ..)` method takes [`timsk2::W`](W) writer structure
        impl crate::Writable for TIMSK2_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIMSK2 to value 0
        impl crate::Resettable for TIMSK2_SPEC {}
    }
}
///Timer/Counter, 16-bit
pub type TC3 = crate::Periph<tc3::RegisterBlock, 0x38>;
impl core::fmt::Debug for TC3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC3").finish()
    }
}
///Timer/Counter, 16-bit
pub mod tc3 {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        tifr3: TIFR3,
        _reserved1: [u8; 0x38],
        timsk3: TIMSK3,
        _reserved2: [u8; 0x1e],
        tccr3a: TCCR3A,
        tccr3b: TCCR3B,
        tccr3c: TCCR3C,
        _reserved5: [u8; 0x01],
        tcnt3: TCNT3,
        icr3: ICR3,
        ocr3a: OCR3A,
        ocr3b: OCR3B,
        ocr3c: OCR3C,
    }
    impl RegisterBlock {
        ///0x00 - Timer/Counter3 Interrupt Flag register
        #[inline(always)]
        pub const fn tifr3(&self) -> &TIFR3 {
            &self.tifr3
        }
        ///0x39 - Timer/Counter3 Interrupt Mask Register
        #[inline(always)]
        pub const fn timsk3(&self) -> &TIMSK3 {
            &self.timsk3
        }
        ///0x58 - Timer/Counter3 Control Register A
        #[inline(always)]
        pub const fn tccr3a(&self) -> &TCCR3A {
            &self.tccr3a
        }
        ///0x59 - Timer/Counter3 Control Register B
        #[inline(always)]
        pub const fn tccr3b(&self) -> &TCCR3B {
            &self.tccr3b
        }
        ///0x5a - Timer/Counter 3 Control Register C
        #[inline(always)]
        pub const fn tccr3c(&self) -> &TCCR3C {
            &self.tccr3c
        }
        ///0x5c - Timer/Counter3 Bytes
        #[inline(always)]
        pub const fn tcnt3(&self) -> &TCNT3 {
            &self.tcnt3
        }
        ///0x5e - Timer/Counter3 Input Capture Register Bytes
        #[inline(always)]
        pub const fn icr3(&self) -> &ICR3 {
            &self.icr3
        }
        ///0x60 - Timer/Counter3 Output Compare Register A Bytes
        #[inline(always)]
        pub const fn ocr3a(&self) -> &OCR3A {
            &self.ocr3a
        }
        ///0x62 - Timer/Counter3 Output Compare Register B Bytes
        #[inline(always)]
        pub const fn ocr3b(&self) -> &OCR3B {
            &self.ocr3b
        }
        ///0x64 - Timer/Counter3 Output Compare Register B Bytes
        #[inline(always)]
        pub const fn ocr3c(&self) -> &OCR3C {
            &self.ocr3c
        }
    }
    /**ICR3 (rw) register accessor: Timer/Counter3 Input Capture Register Bytes

You can [`read`](crate::Reg::read) this register and get [`icr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icr3`] module*/
    pub type ICR3 = crate::Reg<icr3::ICR3_SPEC>;
    ///Timer/Counter3 Input Capture Register Bytes
    pub mod icr3 {
        ///Register `ICR3` reader
        pub type R = crate::R<ICR3_SPEC>;
        ///Register `ICR3` writer
        pub type W = crate::W<ICR3_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter3 Input Capture Register Bytes

You can [`read`](crate::Reg::read) this register and get [`icr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ICR3_SPEC;
        impl crate::RegisterSpec for ICR3_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`icr3::R`](R) reader structure
        impl crate::Readable for ICR3_SPEC {}
        ///`write(|w| ..)` method takes [`icr3::W`](W) writer structure
        impl crate::Writable for ICR3_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets ICR3 to value 0
        impl crate::Resettable for ICR3_SPEC {}
    }
    /**OCR3A (rw) register accessor: Timer/Counter3 Output Compare Register A Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr3a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr3a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr3a`] module*/
    pub type OCR3A = crate::Reg<ocr3a::OCR3A_SPEC>;
    ///Timer/Counter3 Output Compare Register A Bytes
    pub mod ocr3a {
        ///Register `OCR3A` reader
        pub type R = crate::R<OCR3A_SPEC>;
        ///Register `OCR3A` writer
        pub type W = crate::W<OCR3A_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter3 Output Compare Register A Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr3a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr3a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR3A_SPEC;
        impl crate::RegisterSpec for OCR3A_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr3a::R`](R) reader structure
        impl crate::Readable for OCR3A_SPEC {}
        ///`write(|w| ..)` method takes [`ocr3a::W`](W) writer structure
        impl crate::Writable for OCR3A_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR3A to value 0
        impl crate::Resettable for OCR3A_SPEC {}
    }
    /**OCR3B (rw) register accessor: Timer/Counter3 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr3b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr3b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr3b`] module*/
    pub type OCR3B = crate::Reg<ocr3b::OCR3B_SPEC>;
    ///Timer/Counter3 Output Compare Register B Bytes
    pub mod ocr3b {
        ///Register `OCR3B` reader
        pub type R = crate::R<OCR3B_SPEC>;
        ///Register `OCR3B` writer
        pub type W = crate::W<OCR3B_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter3 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr3b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr3b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR3B_SPEC;
        impl crate::RegisterSpec for OCR3B_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr3b::R`](R) reader structure
        impl crate::Readable for OCR3B_SPEC {}
        ///`write(|w| ..)` method takes [`ocr3b::W`](W) writer structure
        impl crate::Writable for OCR3B_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR3B to value 0
        impl crate::Resettable for OCR3B_SPEC {}
    }
    /**OCR3C (rw) register accessor: Timer/Counter3 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr3c`] module*/
    pub type OCR3C = crate::Reg<ocr3c::OCR3C_SPEC>;
    ///Timer/Counter3 Output Compare Register B Bytes
    pub mod ocr3c {
        ///Register `OCR3C` reader
        pub type R = crate::R<OCR3C_SPEC>;
        ///Register `OCR3C` writer
        pub type W = crate::W<OCR3C_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter3 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR3C_SPEC;
        impl crate::RegisterSpec for OCR3C_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr3c::R`](R) reader structure
        impl crate::Readable for OCR3C_SPEC {}
        ///`write(|w| ..)` method takes [`ocr3c::W`](W) writer structure
        impl crate::Writable for OCR3C_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR3C to value 0
        impl crate::Resettable for OCR3C_SPEC {}
    }
    /**TCCR3A (rw) register accessor: Timer/Counter3 Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr3a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr3a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr3a`] module*/
    pub type TCCR3A = crate::Reg<tccr3a::TCCR3A_SPEC>;
    ///Timer/Counter3 Control Register A
    pub mod tccr3a {
        ///Register `TCCR3A` reader
        pub type R = crate::R<TCCR3A_SPEC>;
        ///Register `TCCR3A` writer
        pub type W = crate::W<TCCR3A_SPEC>;
        ///Field `WGM3` reader - Waveform Generation Mode
        pub type WGM3_R = crate::FieldReader;
        ///Field `WGM3` writer - Waveform Generation Mode
        pub type WGM3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
        /**Compare Output Mode 3C, bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum COM3C_A {
            ///0: Normal port operation, OCix disconnected
            DISCONNECTED = 0,
            ///1: Toggle OCix on Compare Match (Might depend on WGM)
            MATCH_TOGGLE = 1,
            ///2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            MATCH_CLEAR = 2,
            ///3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            MATCH_SET = 3,
        }
        impl From<COM3C_A> for u8 {
            #[inline(always)]
            fn from(variant: COM3C_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for COM3C_A {
            type Ux = u8;
        }
        impl crate::IsEnum for COM3C_A {}
        ///Field `COM3C` reader - Compare Output Mode 3C, bits
        pub type COM3C_R = crate::FieldReader<COM3C_A>;
        impl COM3C_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> COM3C_A {
                match self.bits {
                    0 => COM3C_A::DISCONNECTED,
                    1 => COM3C_A::MATCH_TOGGLE,
                    2 => COM3C_A::MATCH_CLEAR,
                    3 => COM3C_A::MATCH_SET,
                    _ => unreachable!(),
                }
            }
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn is_disconnected(&self) -> bool {
                *self == COM3C_A::DISCONNECTED
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn is_match_toggle(&self) -> bool {
                *self == COM3C_A::MATCH_TOGGLE
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            #[inline(always)]
            pub fn is_match_clear(&self) -> bool {
                *self == COM3C_A::MATCH_CLEAR
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            #[inline(always)]
            pub fn is_match_set(&self) -> bool {
                *self == COM3C_A::MATCH_SET
            }
        }
        ///Field `COM3C` writer - Compare Output Mode 3C, bits
        pub type COM3C_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COM3C_A, crate::Safe>;
        impl<'a, REG> COM3C_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn disconnected(self) -> &'a mut crate::W<REG> {
                self.variant(COM3C_A::DISCONNECTED)
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn match_toggle(self) -> &'a mut crate::W<REG> {
                self.variant(COM3C_A::MATCH_TOGGLE)
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            #[inline(always)]
            pub fn match_clear(self) -> &'a mut crate::W<REG> {
                self.variant(COM3C_A::MATCH_CLEAR)
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            #[inline(always)]
            pub fn match_set(self) -> &'a mut crate::W<REG> {
                self.variant(COM3C_A::MATCH_SET)
            }
        }
        ///Field `COM3B` reader - Compare Output Mode 3B, bits
        pub use COM3C_R as COM3B_R;
        ///Field `COM3B` writer - Compare Output Mode 3B, bits
        pub use COM3C_W as COM3B_W;
        ///Field `COM3A` reader - Compare Output Mode 1A, bits
        pub use COM3C_R as COM3A_R;
        ///Field `COM3A` writer - Compare Output Mode 1A, bits
        pub use COM3C_W as COM3A_W;
        impl R {
            ///Bits 0:1 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm3(&self) -> WGM3_R {
                WGM3_R::new(self.bits & 3)
            }
            ///Bits 2:3 - Compare Output Mode 3C, bits
            #[inline(always)]
            pub fn com3c(&self) -> COM3C_R {
                COM3C_R::new((self.bits >> 2) & 3)
            }
            ///Bits 4:5 - Compare Output Mode 3B, bits
            #[inline(always)]
            pub fn com3b(&self) -> COM3B_R {
                COM3B_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - Compare Output Mode 1A, bits
            #[inline(always)]
            pub fn com3a(&self) -> COM3A_R {
                COM3A_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bits 0:1 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm3(&mut self) -> WGM3_W<'_, TCCR3A_SPEC> {
                WGM3_W::new(self, 0)
            }
            ///Bits 2:3 - Compare Output Mode 3C, bits
            #[inline(always)]
            pub fn com3c(&mut self) -> COM3C_W<'_, TCCR3A_SPEC> {
                COM3C_W::new(self, 2)
            }
            ///Bits 4:5 - Compare Output Mode 3B, bits
            #[inline(always)]
            pub fn com3b(&mut self) -> COM3B_W<'_, TCCR3A_SPEC> {
                COM3B_W::new(self, 4)
            }
            ///Bits 6:7 - Compare Output Mode 1A, bits
            #[inline(always)]
            pub fn com3a(&mut self) -> COM3A_W<'_, TCCR3A_SPEC> {
                COM3A_W::new(self, 6)
            }
        }
        /**Timer/Counter3 Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr3a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr3a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR3A_SPEC;
        impl crate::RegisterSpec for TCCR3A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr3a::R`](R) reader structure
        impl crate::Readable for TCCR3A_SPEC {}
        ///`write(|w| ..)` method takes [`tccr3a::W`](W) writer structure
        impl crate::Writable for TCCR3A_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR3A to value 0
        impl crate::Resettable for TCCR3A_SPEC {}
    }
    /**TCCR3B (rw) register accessor: Timer/Counter3 Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr3b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr3b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr3b`] module*/
    pub type TCCR3B = crate::Reg<tccr3b::TCCR3B_SPEC>;
    ///Timer/Counter3 Control Register B
    pub mod tccr3b {
        ///Register `TCCR3B` reader
        pub type R = crate::R<TCCR3B_SPEC>;
        ///Register `TCCR3B` writer
        pub type W = crate::W<TCCR3B_SPEC>;
        /**Prescaler source of Timer/Counter 3

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum CS3_A {
            ///0: No clock source (Timer/Counter stopped)
            NO_CLOCK = 0,
            ///1: Running, No Prescaling
            DIRECT = 1,
            ///2: Running, CLK/8
            PRESCALE_8 = 2,
            ///3: Running, CLK/64
            PRESCALE_64 = 3,
            ///4: Running, CLK/256
            PRESCALE_256 = 4,
            ///5: Running, CLK/1024
            PRESCALE_1024 = 5,
            ///6: Running, ExtClk Tx Falling Edge
            EXT_FALLING = 6,
            ///7: Running, ExtClk Tx Rising Edge
            EXT_RISING = 7,
        }
        impl From<CS3_A> for u8 {
            #[inline(always)]
            fn from(variant: CS3_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for CS3_A {
            type Ux = u8;
        }
        impl crate::IsEnum for CS3_A {}
        ///Field `CS3` reader - Prescaler source of Timer/Counter 3
        pub type CS3_R = crate::FieldReader<CS3_A>;
        impl CS3_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> CS3_A {
                match self.bits {
                    0 => CS3_A::NO_CLOCK,
                    1 => CS3_A::DIRECT,
                    2 => CS3_A::PRESCALE_8,
                    3 => CS3_A::PRESCALE_64,
                    4 => CS3_A::PRESCALE_256,
                    5 => CS3_A::PRESCALE_1024,
                    6 => CS3_A::EXT_FALLING,
                    7 => CS3_A::EXT_RISING,
                    _ => unreachable!(),
                }
            }
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn is_no_clock(&self) -> bool {
                *self == CS3_A::NO_CLOCK
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn is_direct(&self) -> bool {
                *self == CS3_A::DIRECT
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn is_prescale_8(&self) -> bool {
                *self == CS3_A::PRESCALE_8
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn is_prescale_64(&self) -> bool {
                *self == CS3_A::PRESCALE_64
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn is_prescale_256(&self) -> bool {
                *self == CS3_A::PRESCALE_256
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn is_prescale_1024(&self) -> bool {
                *self == CS3_A::PRESCALE_1024
            }
            ///Running, ExtClk Tx Falling Edge
            #[inline(always)]
            pub fn is_ext_falling(&self) -> bool {
                *self == CS3_A::EXT_FALLING
            }
            ///Running, ExtClk Tx Rising Edge
            #[inline(always)]
            pub fn is_ext_rising(&self) -> bool {
                *self == CS3_A::EXT_RISING
            }
        }
        ///Field `CS3` writer - Prescaler source of Timer/Counter 3
        pub type CS3_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CS3_A, crate::Safe>;
        impl<'a, REG> CS3_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn no_clock(self) -> &'a mut crate::W<REG> {
                self.variant(CS3_A::NO_CLOCK)
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn direct(self) -> &'a mut crate::W<REG> {
                self.variant(CS3_A::DIRECT)
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn prescale_8(self) -> &'a mut crate::W<REG> {
                self.variant(CS3_A::PRESCALE_8)
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn prescale_64(self) -> &'a mut crate::W<REG> {
                self.variant(CS3_A::PRESCALE_64)
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn prescale_256(self) -> &'a mut crate::W<REG> {
                self.variant(CS3_A::PRESCALE_256)
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
                self.variant(CS3_A::PRESCALE_1024)
            }
            ///Running, ExtClk Tx Falling Edge
            #[inline(always)]
            pub fn ext_falling(self) -> &'a mut crate::W<REG> {
                self.variant(CS3_A::EXT_FALLING)
            }
            ///Running, ExtClk Tx Rising Edge
            #[inline(always)]
            pub fn ext_rising(self) -> &'a mut crate::W<REG> {
                self.variant(CS3_A::EXT_RISING)
            }
        }
        ///Field `WGM3` reader - Waveform Generation Mode
        pub type WGM3_R = crate::FieldReader;
        ///Field `WGM3` writer - Waveform Generation Mode
        pub type WGM3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
        ///Field `ICES3` reader - Input Capture 3 Edge Select
        pub type ICES3_R = crate::BitReader;
        ///Field `ICES3` writer - Input Capture 3 Edge Select
        pub type ICES3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICNC3` reader - Input Capture 3 Noise Canceler
        pub type ICNC3_R = crate::BitReader;
        ///Field `ICNC3` writer - Input Capture 3 Noise Canceler
        pub type ICNC3_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:2 - Prescaler source of Timer/Counter 3
            #[inline(always)]
            pub fn cs3(&self) -> CS3_R {
                CS3_R::new(self.bits & 7)
            }
            ///Bits 3:4 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm3(&self) -> WGM3_R {
                WGM3_R::new((self.bits >> 3) & 3)
            }
            ///Bit 6 - Input Capture 3 Edge Select
            #[inline(always)]
            pub fn ices3(&self) -> ICES3_R {
                ICES3_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Input Capture 3 Noise Canceler
            #[inline(always)]
            pub fn icnc3(&self) -> ICNC3_R {
                ICNC3_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:2 - Prescaler source of Timer/Counter 3
            #[inline(always)]
            pub fn cs3(&mut self) -> CS3_W<'_, TCCR3B_SPEC> {
                CS3_W::new(self, 0)
            }
            ///Bits 3:4 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm3(&mut self) -> WGM3_W<'_, TCCR3B_SPEC> {
                WGM3_W::new(self, 3)
            }
            ///Bit 6 - Input Capture 3 Edge Select
            #[inline(always)]
            pub fn ices3(&mut self) -> ICES3_W<'_, TCCR3B_SPEC> {
                ICES3_W::new(self, 6)
            }
            ///Bit 7 - Input Capture 3 Noise Canceler
            #[inline(always)]
            pub fn icnc3(&mut self) -> ICNC3_W<'_, TCCR3B_SPEC> {
                ICNC3_W::new(self, 7)
            }
        }
        /**Timer/Counter3 Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr3b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr3b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR3B_SPEC;
        impl crate::RegisterSpec for TCCR3B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr3b::R`](R) reader structure
        impl crate::Readable for TCCR3B_SPEC {}
        ///`write(|w| ..)` method takes [`tccr3b::W`](W) writer structure
        impl crate::Writable for TCCR3B_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR3B to value 0
        impl crate::Resettable for TCCR3B_SPEC {}
    }
    /**TCCR3C (rw) register accessor: Timer/Counter 3 Control Register C

You can [`read`](crate::Reg::read) this register and get [`tccr3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr3c`] module*/
    pub type TCCR3C = crate::Reg<tccr3c::TCCR3C_SPEC>;
    ///Timer/Counter 3 Control Register C
    pub mod tccr3c {
        ///Register `TCCR3C` reader
        pub type R = crate::R<TCCR3C_SPEC>;
        ///Register `TCCR3C` writer
        pub type W = crate::W<TCCR3C_SPEC>;
        ///Field `FOC3C` writer - Force Output Compare 3C
        pub type FOC3C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC3B` writer - Force Output Compare 3B
        pub type FOC3B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC3A` writer - Force Output Compare 3A
        pub type FOC3A_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl W {
            ///Bit 5 - Force Output Compare 3C
            #[inline(always)]
            pub fn foc3c(&mut self) -> FOC3C_W<'_, TCCR3C_SPEC> {
                FOC3C_W::new(self, 5)
            }
            ///Bit 6 - Force Output Compare 3B
            #[inline(always)]
            pub fn foc3b(&mut self) -> FOC3B_W<'_, TCCR3C_SPEC> {
                FOC3B_W::new(self, 6)
            }
            ///Bit 7 - Force Output Compare 3A
            #[inline(always)]
            pub fn foc3a(&mut self) -> FOC3A_W<'_, TCCR3C_SPEC> {
                FOC3A_W::new(self, 7)
            }
        }
        /**Timer/Counter 3 Control Register C

You can [`read`](crate::Reg::read) this register and get [`tccr3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR3C_SPEC;
        impl crate::RegisterSpec for TCCR3C_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr3c::R`](R) reader structure
        impl crate::Readable for TCCR3C_SPEC {}
        ///`write(|w| ..)` method takes [`tccr3c::W`](W) writer structure
        impl crate::Writable for TCCR3C_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR3C to value 0
        impl crate::Resettable for TCCR3C_SPEC {}
    }
    /**TCNT3 (rw) register accessor: Timer/Counter3 Bytes

You can [`read`](crate::Reg::read) this register and get [`tcnt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcnt3`] module*/
    pub type TCNT3 = crate::Reg<tcnt3::TCNT3_SPEC>;
    ///Timer/Counter3 Bytes
    pub mod tcnt3 {
        ///Register `TCNT3` reader
        pub type R = crate::R<TCNT3_SPEC>;
        ///Register `TCNT3` writer
        pub type W = crate::W<TCNT3_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter3 Bytes

You can [`read`](crate::Reg::read) this register and get [`tcnt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCNT3_SPEC;
        impl crate::RegisterSpec for TCNT3_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`tcnt3::R`](R) reader structure
        impl crate::Readable for TCNT3_SPEC {}
        ///`write(|w| ..)` method takes [`tcnt3::W`](W) writer structure
        impl crate::Writable for TCNT3_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets TCNT3 to value 0
        impl crate::Resettable for TCNT3_SPEC {}
    }
    /**TIFR3 (rw) register accessor: Timer/Counter3 Interrupt Flag register

You can [`read`](crate::Reg::read) this register and get [`tifr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tifr3`] module*/
    pub type TIFR3 = crate::Reg<tifr3::TIFR3_SPEC>;
    ///Timer/Counter3 Interrupt Flag register
    pub mod tifr3 {
        ///Register `TIFR3` reader
        pub type R = crate::R<TIFR3_SPEC>;
        ///Register `TIFR3` writer
        pub type W = crate::W<TIFR3_SPEC>;
        ///Field `TOV3` reader - Timer/Counter3 Overflow Flag
        pub type TOV3_R = crate::BitReader;
        ///Field `TOV3` writer - Timer/Counter3 Overflow Flag
        pub type TOV3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF3A` reader - Output Compare Flag 3A
        pub type OCF3A_R = crate::BitReader;
        ///Field `OCF3A` writer - Output Compare Flag 3A
        pub type OCF3A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF3B` reader - Output Compare Flag 3B
        pub type OCF3B_R = crate::BitReader;
        ///Field `OCF3B` writer - Output Compare Flag 3B
        pub type OCF3B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF3C` reader - Output Compare Flag 3C
        pub type OCF3C_R = crate::BitReader;
        ///Field `OCF3C` writer - Output Compare Flag 3C
        pub type OCF3C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICF3` reader - Input Capture Flag 3
        pub type ICF3_R = crate::BitReader;
        ///Field `ICF3` writer - Input Capture Flag 3
        pub type ICF3_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter3 Overflow Flag
            #[inline(always)]
            pub fn tov3(&self) -> TOV3_R {
                TOV3_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Output Compare Flag 3A
            #[inline(always)]
            pub fn ocf3a(&self) -> OCF3A_R {
                OCF3A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Output Compare Flag 3B
            #[inline(always)]
            pub fn ocf3b(&self) -> OCF3B_R {
                OCF3B_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Output Compare Flag 3C
            #[inline(always)]
            pub fn ocf3c(&self) -> OCF3C_R {
                OCF3C_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 5 - Input Capture Flag 3
            #[inline(always)]
            pub fn icf3(&self) -> ICF3_R {
                ICF3_R::new(((self.bits >> 5) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter3 Overflow Flag
            #[inline(always)]
            pub fn tov3(&mut self) -> TOV3_W<'_, TIFR3_SPEC> {
                TOV3_W::new(self, 0)
            }
            ///Bit 1 - Output Compare Flag 3A
            #[inline(always)]
            pub fn ocf3a(&mut self) -> OCF3A_W<'_, TIFR3_SPEC> {
                OCF3A_W::new(self, 1)
            }
            ///Bit 2 - Output Compare Flag 3B
            #[inline(always)]
            pub fn ocf3b(&mut self) -> OCF3B_W<'_, TIFR3_SPEC> {
                OCF3B_W::new(self, 2)
            }
            ///Bit 3 - Output Compare Flag 3C
            #[inline(always)]
            pub fn ocf3c(&mut self) -> OCF3C_W<'_, TIFR3_SPEC> {
                OCF3C_W::new(self, 3)
            }
            ///Bit 5 - Input Capture Flag 3
            #[inline(always)]
            pub fn icf3(&mut self) -> ICF3_W<'_, TIFR3_SPEC> {
                ICF3_W::new(self, 5)
            }
        }
        /**Timer/Counter3 Interrupt Flag register

You can [`read`](crate::Reg::read) this register and get [`tifr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIFR3_SPEC;
        impl crate::RegisterSpec for TIFR3_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tifr3::R`](R) reader structure
        impl crate::Readable for TIFR3_SPEC {}
        ///`write(|w| ..)` method takes [`tifr3::W`](W) writer structure
        impl crate::Writable for TIFR3_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIFR3 to value 0
        impl crate::Resettable for TIFR3_SPEC {}
    }
    /**TIMSK3 (rw) register accessor: Timer/Counter3 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`timsk3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timsk3`] module*/
    pub type TIMSK3 = crate::Reg<timsk3::TIMSK3_SPEC>;
    ///Timer/Counter3 Interrupt Mask Register
    pub mod timsk3 {
        ///Register `TIMSK3` reader
        pub type R = crate::R<TIMSK3_SPEC>;
        ///Register `TIMSK3` writer
        pub type W = crate::W<TIMSK3_SPEC>;
        ///Field `TOIE3` reader - Timer/Counter3 Overflow Interrupt Enable
        pub type TOIE3_R = crate::BitReader;
        ///Field `TOIE3` writer - Timer/Counter3 Overflow Interrupt Enable
        pub type TOIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE3A` reader - Timer/Counter3 Output Compare A Match Interrupt Enable
        pub type OCIE3A_R = crate::BitReader;
        ///Field `OCIE3A` writer - Timer/Counter3 Output Compare A Match Interrupt Enable
        pub type OCIE3A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE3B` reader - Timer/Counter3 Output Compare B Match Interrupt Enable
        pub type OCIE3B_R = crate::BitReader;
        ///Field `OCIE3B` writer - Timer/Counter3 Output Compare B Match Interrupt Enable
        pub type OCIE3B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE3C` reader - Timer/Counter3 Output Compare C Match Interrupt Enable
        pub type OCIE3C_R = crate::BitReader;
        ///Field `OCIE3C` writer - Timer/Counter3 Output Compare C Match Interrupt Enable
        pub type OCIE3C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICIE3` reader - Timer/Counter3 Input Capture Interrupt Enable
        pub type ICIE3_R = crate::BitReader;
        ///Field `ICIE3` writer - Timer/Counter3 Input Capture Interrupt Enable
        pub type ICIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter3 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie3(&self) -> TOIE3_R {
                TOIE3_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Timer/Counter3 Output Compare A Match Interrupt Enable
            #[inline(always)]
            pub fn ocie3a(&self) -> OCIE3A_R {
                OCIE3A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Timer/Counter3 Output Compare B Match Interrupt Enable
            #[inline(always)]
            pub fn ocie3b(&self) -> OCIE3B_R {
                OCIE3B_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Timer/Counter3 Output Compare C Match Interrupt Enable
            #[inline(always)]
            pub fn ocie3c(&self) -> OCIE3C_R {
                OCIE3C_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 5 - Timer/Counter3 Input Capture Interrupt Enable
            #[inline(always)]
            pub fn icie3(&self) -> ICIE3_R {
                ICIE3_R::new(((self.bits >> 5) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter3 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie3(&mut self) -> TOIE3_W<'_, TIMSK3_SPEC> {
                TOIE3_W::new(self, 0)
            }
            ///Bit 1 - Timer/Counter3 Output Compare A Match Interrupt Enable
            #[inline(always)]
            pub fn ocie3a(&mut self) -> OCIE3A_W<'_, TIMSK3_SPEC> {
                OCIE3A_W::new(self, 1)
            }
            ///Bit 2 - Timer/Counter3 Output Compare B Match Interrupt Enable
            #[inline(always)]
            pub fn ocie3b(&mut self) -> OCIE3B_W<'_, TIMSK3_SPEC> {
                OCIE3B_W::new(self, 2)
            }
            ///Bit 3 - Timer/Counter3 Output Compare C Match Interrupt Enable
            #[inline(always)]
            pub fn ocie3c(&mut self) -> OCIE3C_W<'_, TIMSK3_SPEC> {
                OCIE3C_W::new(self, 3)
            }
            ///Bit 5 - Timer/Counter3 Input Capture Interrupt Enable
            #[inline(always)]
            pub fn icie3(&mut self) -> ICIE3_W<'_, TIMSK3_SPEC> {
                ICIE3_W::new(self, 5)
            }
        }
        /**Timer/Counter3 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`timsk3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIMSK3_SPEC;
        impl crate::RegisterSpec for TIMSK3_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`timsk3::R`](R) reader structure
        impl crate::Readable for TIMSK3_SPEC {}
        ///`write(|w| ..)` method takes [`timsk3::W`](W) writer structure
        impl crate::Writable for TIMSK3_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIMSK3 to value 0
        impl crate::Resettable for TIMSK3_SPEC {}
    }
}
///Timer/Counter, 16-bit
pub type TC4 = crate::Periph<tc4::RegisterBlock, 0x39>;
impl core::fmt::Debug for TC4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC4").finish()
    }
}
///Timer/Counter, 16-bit
pub mod tc4 {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        tifr4: TIFR4,
        _reserved1: [u8; 0x38],
        timsk4: TIMSK4,
        _reserved2: [u8; 0x2d],
        tccr4a: TCCR4A,
        tccr4b: TCCR4B,
        tccr4c: TCCR4C,
        _reserved5: [u8; 0x01],
        tcnt4: TCNT4,
        icr4: ICR4,
        ocr4a: OCR4A,
        ocr4b: OCR4B,
        ocr4c: OCR4C,
    }
    impl RegisterBlock {
        ///0x00 - Timer/Counter4 Interrupt Flag register
        #[inline(always)]
        pub const fn tifr4(&self) -> &TIFR4 {
            &self.tifr4
        }
        ///0x39 - Timer/Counter4 Interrupt Mask Register
        #[inline(always)]
        pub const fn timsk4(&self) -> &TIMSK4 {
            &self.timsk4
        }
        ///0x67 - Timer/Counter4 Control Register A
        #[inline(always)]
        pub const fn tccr4a(&self) -> &TCCR4A {
            &self.tccr4a
        }
        ///0x68 - Timer/Counter4 Control Register B
        #[inline(always)]
        pub const fn tccr4b(&self) -> &TCCR4B {
            &self.tccr4b
        }
        ///0x69 - Timer/Counter 4 Control Register C
        #[inline(always)]
        pub const fn tccr4c(&self) -> &TCCR4C {
            &self.tccr4c
        }
        ///0x6b - Timer/Counter4 Bytes
        #[inline(always)]
        pub const fn tcnt4(&self) -> &TCNT4 {
            &self.tcnt4
        }
        ///0x6d - Timer/Counter4 Input Capture Register Bytes
        #[inline(always)]
        pub const fn icr4(&self) -> &ICR4 {
            &self.icr4
        }
        ///0x6f - Timer/Counter4 Output Compare Register A Bytes
        #[inline(always)]
        pub const fn ocr4a(&self) -> &OCR4A {
            &self.ocr4a
        }
        ///0x71 - Timer/Counter4 Output Compare Register B Bytes
        #[inline(always)]
        pub const fn ocr4b(&self) -> &OCR4B {
            &self.ocr4b
        }
        ///0x73 - Timer/Counter4 Output Compare Register B Bytes
        #[inline(always)]
        pub const fn ocr4c(&self) -> &OCR4C {
            &self.ocr4c
        }
    }
    /**ICR4 (rw) register accessor: Timer/Counter4 Input Capture Register Bytes

You can [`read`](crate::Reg::read) this register and get [`icr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icr4`] module*/
    pub type ICR4 = crate::Reg<icr4::ICR4_SPEC>;
    ///Timer/Counter4 Input Capture Register Bytes
    pub mod icr4 {
        ///Register `ICR4` reader
        pub type R = crate::R<ICR4_SPEC>;
        ///Register `ICR4` writer
        pub type W = crate::W<ICR4_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter4 Input Capture Register Bytes

You can [`read`](crate::Reg::read) this register and get [`icr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ICR4_SPEC;
        impl crate::RegisterSpec for ICR4_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`icr4::R`](R) reader structure
        impl crate::Readable for ICR4_SPEC {}
        ///`write(|w| ..)` method takes [`icr4::W`](W) writer structure
        impl crate::Writable for ICR4_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets ICR4 to value 0
        impl crate::Resettable for ICR4_SPEC {}
    }
    /**OCR4A (rw) register accessor: Timer/Counter4 Output Compare Register A Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr4a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr4a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr4a`] module*/
    pub type OCR4A = crate::Reg<ocr4a::OCR4A_SPEC>;
    ///Timer/Counter4 Output Compare Register A Bytes
    pub mod ocr4a {
        ///Register `OCR4A` reader
        pub type R = crate::R<OCR4A_SPEC>;
        ///Register `OCR4A` writer
        pub type W = crate::W<OCR4A_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter4 Output Compare Register A Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr4a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr4a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR4A_SPEC;
        impl crate::RegisterSpec for OCR4A_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr4a::R`](R) reader structure
        impl crate::Readable for OCR4A_SPEC {}
        ///`write(|w| ..)` method takes [`ocr4a::W`](W) writer structure
        impl crate::Writable for OCR4A_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR4A to value 0
        impl crate::Resettable for OCR4A_SPEC {}
    }
    /**OCR4B (rw) register accessor: Timer/Counter4 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr4b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr4b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr4b`] module*/
    pub type OCR4B = crate::Reg<ocr4b::OCR4B_SPEC>;
    ///Timer/Counter4 Output Compare Register B Bytes
    pub mod ocr4b {
        ///Register `OCR4B` reader
        pub type R = crate::R<OCR4B_SPEC>;
        ///Register `OCR4B` writer
        pub type W = crate::W<OCR4B_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter4 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr4b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr4b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR4B_SPEC;
        impl crate::RegisterSpec for OCR4B_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr4b::R`](R) reader structure
        impl crate::Readable for OCR4B_SPEC {}
        ///`write(|w| ..)` method takes [`ocr4b::W`](W) writer structure
        impl crate::Writable for OCR4B_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR4B to value 0
        impl crate::Resettable for OCR4B_SPEC {}
    }
    /**OCR4C (rw) register accessor: Timer/Counter4 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr4c`] module*/
    pub type OCR4C = crate::Reg<ocr4c::OCR4C_SPEC>;
    ///Timer/Counter4 Output Compare Register B Bytes
    pub mod ocr4c {
        ///Register `OCR4C` reader
        pub type R = crate::R<OCR4C_SPEC>;
        ///Register `OCR4C` writer
        pub type W = crate::W<OCR4C_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter4 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR4C_SPEC;
        impl crate::RegisterSpec for OCR4C_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr4c::R`](R) reader structure
        impl crate::Readable for OCR4C_SPEC {}
        ///`write(|w| ..)` method takes [`ocr4c::W`](W) writer structure
        impl crate::Writable for OCR4C_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR4C to value 0
        impl crate::Resettable for OCR4C_SPEC {}
    }
    /**TCCR4A (rw) register accessor: Timer/Counter4 Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr4a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr4a`] module*/
    pub type TCCR4A = crate::Reg<tccr4a::TCCR4A_SPEC>;
    ///Timer/Counter4 Control Register A
    pub mod tccr4a {
        ///Register `TCCR4A` reader
        pub type R = crate::R<TCCR4A_SPEC>;
        ///Register `TCCR4A` writer
        pub type W = crate::W<TCCR4A_SPEC>;
        ///Field `WGM4` reader - Waveform Generation Mode
        pub type WGM4_R = crate::FieldReader;
        ///Field `WGM4` writer - Waveform Generation Mode
        pub type WGM4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
        /**Compare Output Mode 4C, bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum COM4C_A {
            ///0: Normal port operation, OCix disconnected
            DISCONNECTED = 0,
            ///1: Toggle OCix on Compare Match (Might depend on WGM)
            MATCH_TOGGLE = 1,
            ///2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            MATCH_CLEAR = 2,
            ///3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            MATCH_SET = 3,
        }
        impl From<COM4C_A> for u8 {
            #[inline(always)]
            fn from(variant: COM4C_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for COM4C_A {
            type Ux = u8;
        }
        impl crate::IsEnum for COM4C_A {}
        ///Field `COM4C` reader - Compare Output Mode 4C, bits
        pub type COM4C_R = crate::FieldReader<COM4C_A>;
        impl COM4C_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> COM4C_A {
                match self.bits {
                    0 => COM4C_A::DISCONNECTED,
                    1 => COM4C_A::MATCH_TOGGLE,
                    2 => COM4C_A::MATCH_CLEAR,
                    3 => COM4C_A::MATCH_SET,
                    _ => unreachable!(),
                }
            }
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn is_disconnected(&self) -> bool {
                *self == COM4C_A::DISCONNECTED
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn is_match_toggle(&self) -> bool {
                *self == COM4C_A::MATCH_TOGGLE
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            #[inline(always)]
            pub fn is_match_clear(&self) -> bool {
                *self == COM4C_A::MATCH_CLEAR
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            #[inline(always)]
            pub fn is_match_set(&self) -> bool {
                *self == COM4C_A::MATCH_SET
            }
        }
        ///Field `COM4C` writer - Compare Output Mode 4C, bits
        pub type COM4C_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COM4C_A, crate::Safe>;
        impl<'a, REG> COM4C_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn disconnected(self) -> &'a mut crate::W<REG> {
                self.variant(COM4C_A::DISCONNECTED)
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn match_toggle(self) -> &'a mut crate::W<REG> {
                self.variant(COM4C_A::MATCH_TOGGLE)
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            #[inline(always)]
            pub fn match_clear(self) -> &'a mut crate::W<REG> {
                self.variant(COM4C_A::MATCH_CLEAR)
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            #[inline(always)]
            pub fn match_set(self) -> &'a mut crate::W<REG> {
                self.variant(COM4C_A::MATCH_SET)
            }
        }
        ///Field `COM4B` reader - Compare Output Mode 4B, bits
        pub use COM4C_R as COM4B_R;
        ///Field `COM4B` writer - Compare Output Mode 4B, bits
        pub use COM4C_W as COM4B_W;
        ///Field `COM4A` reader - Compare Output Mode 1A, bits
        pub use COM4C_R as COM4A_R;
        ///Field `COM4A` writer - Compare Output Mode 1A, bits
        pub use COM4C_W as COM4A_W;
        impl R {
            ///Bits 0:1 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm4(&self) -> WGM4_R {
                WGM4_R::new(self.bits & 3)
            }
            ///Bits 2:3 - Compare Output Mode 4C, bits
            #[inline(always)]
            pub fn com4c(&self) -> COM4C_R {
                COM4C_R::new((self.bits >> 2) & 3)
            }
            ///Bits 4:5 - Compare Output Mode 4B, bits
            #[inline(always)]
            pub fn com4b(&self) -> COM4B_R {
                COM4B_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - Compare Output Mode 1A, bits
            #[inline(always)]
            pub fn com4a(&self) -> COM4A_R {
                COM4A_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bits 0:1 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm4(&mut self) -> WGM4_W<'_, TCCR4A_SPEC> {
                WGM4_W::new(self, 0)
            }
            ///Bits 2:3 - Compare Output Mode 4C, bits
            #[inline(always)]
            pub fn com4c(&mut self) -> COM4C_W<'_, TCCR4A_SPEC> {
                COM4C_W::new(self, 2)
            }
            ///Bits 4:5 - Compare Output Mode 4B, bits
            #[inline(always)]
            pub fn com4b(&mut self) -> COM4B_W<'_, TCCR4A_SPEC> {
                COM4B_W::new(self, 4)
            }
            ///Bits 6:7 - Compare Output Mode 1A, bits
            #[inline(always)]
            pub fn com4a(&mut self) -> COM4A_W<'_, TCCR4A_SPEC> {
                COM4A_W::new(self, 6)
            }
        }
        /**Timer/Counter4 Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr4a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR4A_SPEC;
        impl crate::RegisterSpec for TCCR4A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr4a::R`](R) reader structure
        impl crate::Readable for TCCR4A_SPEC {}
        ///`write(|w| ..)` method takes [`tccr4a::W`](W) writer structure
        impl crate::Writable for TCCR4A_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR4A to value 0
        impl crate::Resettable for TCCR4A_SPEC {}
    }
    /**TCCR4B (rw) register accessor: Timer/Counter4 Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr4b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr4b`] module*/
    pub type TCCR4B = crate::Reg<tccr4b::TCCR4B_SPEC>;
    ///Timer/Counter4 Control Register B
    pub mod tccr4b {
        ///Register `TCCR4B` reader
        pub type R = crate::R<TCCR4B_SPEC>;
        ///Register `TCCR4B` writer
        pub type W = crate::W<TCCR4B_SPEC>;
        /**Prescaler source of Timer/Counter 4

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum CS4_A {
            ///0: No clock source (Timer/Counter stopped)
            NO_CLOCK = 0,
            ///1: Running, No Prescaling
            DIRECT = 1,
            ///2: Running, CLK/8
            PRESCALE_8 = 2,
            ///3: Running, CLK/64
            PRESCALE_64 = 3,
            ///4: Running, CLK/256
            PRESCALE_256 = 4,
            ///5: Running, CLK/1024
            PRESCALE_1024 = 5,
            ///6: Running, ExtClk Tx Falling Edge
            EXT_FALLING = 6,
            ///7: Running, ExtClk Tx Rising Edge
            EXT_RISING = 7,
        }
        impl From<CS4_A> for u8 {
            #[inline(always)]
            fn from(variant: CS4_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for CS4_A {
            type Ux = u8;
        }
        impl crate::IsEnum for CS4_A {}
        ///Field `CS4` reader - Prescaler source of Timer/Counter 4
        pub type CS4_R = crate::FieldReader<CS4_A>;
        impl CS4_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> CS4_A {
                match self.bits {
                    0 => CS4_A::NO_CLOCK,
                    1 => CS4_A::DIRECT,
                    2 => CS4_A::PRESCALE_8,
                    3 => CS4_A::PRESCALE_64,
                    4 => CS4_A::PRESCALE_256,
                    5 => CS4_A::PRESCALE_1024,
                    6 => CS4_A::EXT_FALLING,
                    7 => CS4_A::EXT_RISING,
                    _ => unreachable!(),
                }
            }
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn is_no_clock(&self) -> bool {
                *self == CS4_A::NO_CLOCK
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn is_direct(&self) -> bool {
                *self == CS4_A::DIRECT
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn is_prescale_8(&self) -> bool {
                *self == CS4_A::PRESCALE_8
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn is_prescale_64(&self) -> bool {
                *self == CS4_A::PRESCALE_64
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn is_prescale_256(&self) -> bool {
                *self == CS4_A::PRESCALE_256
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn is_prescale_1024(&self) -> bool {
                *self == CS4_A::PRESCALE_1024
            }
            ///Running, ExtClk Tx Falling Edge
            #[inline(always)]
            pub fn is_ext_falling(&self) -> bool {
                *self == CS4_A::EXT_FALLING
            }
            ///Running, ExtClk Tx Rising Edge
            #[inline(always)]
            pub fn is_ext_rising(&self) -> bool {
                *self == CS4_A::EXT_RISING
            }
        }
        ///Field `CS4` writer - Prescaler source of Timer/Counter 4
        pub type CS4_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CS4_A, crate::Safe>;
        impl<'a, REG> CS4_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn no_clock(self) -> &'a mut crate::W<REG> {
                self.variant(CS4_A::NO_CLOCK)
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn direct(self) -> &'a mut crate::W<REG> {
                self.variant(CS4_A::DIRECT)
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn prescale_8(self) -> &'a mut crate::W<REG> {
                self.variant(CS4_A::PRESCALE_8)
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn prescale_64(self) -> &'a mut crate::W<REG> {
                self.variant(CS4_A::PRESCALE_64)
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn prescale_256(self) -> &'a mut crate::W<REG> {
                self.variant(CS4_A::PRESCALE_256)
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
                self.variant(CS4_A::PRESCALE_1024)
            }
            ///Running, ExtClk Tx Falling Edge
            #[inline(always)]
            pub fn ext_falling(self) -> &'a mut crate::W<REG> {
                self.variant(CS4_A::EXT_FALLING)
            }
            ///Running, ExtClk Tx Rising Edge
            #[inline(always)]
            pub fn ext_rising(self) -> &'a mut crate::W<REG> {
                self.variant(CS4_A::EXT_RISING)
            }
        }
        ///Field `WGM4` reader - Waveform Generation Mode
        pub type WGM4_R = crate::FieldReader;
        ///Field `WGM4` writer - Waveform Generation Mode
        pub type WGM4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
        ///Field `ICES4` reader - Input Capture 4 Edge Select
        pub type ICES4_R = crate::BitReader;
        ///Field `ICES4` writer - Input Capture 4 Edge Select
        pub type ICES4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICNC4` reader - Input Capture 4 Noise Canceler
        pub type ICNC4_R = crate::BitReader;
        ///Field `ICNC4` writer - Input Capture 4 Noise Canceler
        pub type ICNC4_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:2 - Prescaler source of Timer/Counter 4
            #[inline(always)]
            pub fn cs4(&self) -> CS4_R {
                CS4_R::new(self.bits & 7)
            }
            ///Bits 3:4 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm4(&self) -> WGM4_R {
                WGM4_R::new((self.bits >> 3) & 3)
            }
            ///Bit 6 - Input Capture 4 Edge Select
            #[inline(always)]
            pub fn ices4(&self) -> ICES4_R {
                ICES4_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Input Capture 4 Noise Canceler
            #[inline(always)]
            pub fn icnc4(&self) -> ICNC4_R {
                ICNC4_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:2 - Prescaler source of Timer/Counter 4
            #[inline(always)]
            pub fn cs4(&mut self) -> CS4_W<'_, TCCR4B_SPEC> {
                CS4_W::new(self, 0)
            }
            ///Bits 3:4 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm4(&mut self) -> WGM4_W<'_, TCCR4B_SPEC> {
                WGM4_W::new(self, 3)
            }
            ///Bit 6 - Input Capture 4 Edge Select
            #[inline(always)]
            pub fn ices4(&mut self) -> ICES4_W<'_, TCCR4B_SPEC> {
                ICES4_W::new(self, 6)
            }
            ///Bit 7 - Input Capture 4 Noise Canceler
            #[inline(always)]
            pub fn icnc4(&mut self) -> ICNC4_W<'_, TCCR4B_SPEC> {
                ICNC4_W::new(self, 7)
            }
        }
        /**Timer/Counter4 Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr4b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR4B_SPEC;
        impl crate::RegisterSpec for TCCR4B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr4b::R`](R) reader structure
        impl crate::Readable for TCCR4B_SPEC {}
        ///`write(|w| ..)` method takes [`tccr4b::W`](W) writer structure
        impl crate::Writable for TCCR4B_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR4B to value 0
        impl crate::Resettable for TCCR4B_SPEC {}
    }
    /**TCCR4C (rw) register accessor: Timer/Counter 4 Control Register C

You can [`read`](crate::Reg::read) this register and get [`tccr4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr4c`] module*/
    pub type TCCR4C = crate::Reg<tccr4c::TCCR4C_SPEC>;
    ///Timer/Counter 4 Control Register C
    pub mod tccr4c {
        ///Register `TCCR4C` reader
        pub type R = crate::R<TCCR4C_SPEC>;
        ///Register `TCCR4C` writer
        pub type W = crate::W<TCCR4C_SPEC>;
        ///Field `FOC4C` writer - Force Output Compare 4C
        pub type FOC4C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC4B` writer - Force Output Compare 4B
        pub type FOC4B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC4A` writer - Force Output Compare 4A
        pub type FOC4A_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl W {
            ///Bit 5 - Force Output Compare 4C
            #[inline(always)]
            pub fn foc4c(&mut self) -> FOC4C_W<'_, TCCR4C_SPEC> {
                FOC4C_W::new(self, 5)
            }
            ///Bit 6 - Force Output Compare 4B
            #[inline(always)]
            pub fn foc4b(&mut self) -> FOC4B_W<'_, TCCR4C_SPEC> {
                FOC4B_W::new(self, 6)
            }
            ///Bit 7 - Force Output Compare 4A
            #[inline(always)]
            pub fn foc4a(&mut self) -> FOC4A_W<'_, TCCR4C_SPEC> {
                FOC4A_W::new(self, 7)
            }
        }
        /**Timer/Counter 4 Control Register C

You can [`read`](crate::Reg::read) this register and get [`tccr4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR4C_SPEC;
        impl crate::RegisterSpec for TCCR4C_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr4c::R`](R) reader structure
        impl crate::Readable for TCCR4C_SPEC {}
        ///`write(|w| ..)` method takes [`tccr4c::W`](W) writer structure
        impl crate::Writable for TCCR4C_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR4C to value 0
        impl crate::Resettable for TCCR4C_SPEC {}
    }
    /**TCNT4 (rw) register accessor: Timer/Counter4 Bytes

You can [`read`](crate::Reg::read) this register and get [`tcnt4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcnt4`] module*/
    pub type TCNT4 = crate::Reg<tcnt4::TCNT4_SPEC>;
    ///Timer/Counter4 Bytes
    pub mod tcnt4 {
        ///Register `TCNT4` reader
        pub type R = crate::R<TCNT4_SPEC>;
        ///Register `TCNT4` writer
        pub type W = crate::W<TCNT4_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter4 Bytes

You can [`read`](crate::Reg::read) this register and get [`tcnt4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCNT4_SPEC;
        impl crate::RegisterSpec for TCNT4_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`tcnt4::R`](R) reader structure
        impl crate::Readable for TCNT4_SPEC {}
        ///`write(|w| ..)` method takes [`tcnt4::W`](W) writer structure
        impl crate::Writable for TCNT4_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets TCNT4 to value 0
        impl crate::Resettable for TCNT4_SPEC {}
    }
    /**TIFR4 (rw) register accessor: Timer/Counter4 Interrupt Flag register

You can [`read`](crate::Reg::read) this register and get [`tifr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tifr4`] module*/
    pub type TIFR4 = crate::Reg<tifr4::TIFR4_SPEC>;
    ///Timer/Counter4 Interrupt Flag register
    pub mod tifr4 {
        ///Register `TIFR4` reader
        pub type R = crate::R<TIFR4_SPEC>;
        ///Register `TIFR4` writer
        pub type W = crate::W<TIFR4_SPEC>;
        ///Field `TOV4` reader - Timer/Counter4 Overflow Flag
        pub type TOV4_R = crate::BitReader;
        ///Field `TOV4` writer - Timer/Counter4 Overflow Flag
        pub type TOV4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF4A` reader - Output Compare Flag 4A
        pub type OCF4A_R = crate::BitReader;
        ///Field `OCF4A` writer - Output Compare Flag 4A
        pub type OCF4A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF4B` reader - Output Compare Flag 4B
        pub type OCF4B_R = crate::BitReader;
        ///Field `OCF4B` writer - Output Compare Flag 4B
        pub type OCF4B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF4C` reader - Output Compare Flag 4C
        pub type OCF4C_R = crate::BitReader;
        ///Field `OCF4C` writer - Output Compare Flag 4C
        pub type OCF4C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICF4` reader - Input Capture Flag 4
        pub type ICF4_R = crate::BitReader;
        ///Field `ICF4` writer - Input Capture Flag 4
        pub type ICF4_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter4 Overflow Flag
            #[inline(always)]
            pub fn tov4(&self) -> TOV4_R {
                TOV4_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Output Compare Flag 4A
            #[inline(always)]
            pub fn ocf4a(&self) -> OCF4A_R {
                OCF4A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Output Compare Flag 4B
            #[inline(always)]
            pub fn ocf4b(&self) -> OCF4B_R {
                OCF4B_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Output Compare Flag 4C
            #[inline(always)]
            pub fn ocf4c(&self) -> OCF4C_R {
                OCF4C_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 5 - Input Capture Flag 4
            #[inline(always)]
            pub fn icf4(&self) -> ICF4_R {
                ICF4_R::new(((self.bits >> 5) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter4 Overflow Flag
            #[inline(always)]
            pub fn tov4(&mut self) -> TOV4_W<'_, TIFR4_SPEC> {
                TOV4_W::new(self, 0)
            }
            ///Bit 1 - Output Compare Flag 4A
            #[inline(always)]
            pub fn ocf4a(&mut self) -> OCF4A_W<'_, TIFR4_SPEC> {
                OCF4A_W::new(self, 1)
            }
            ///Bit 2 - Output Compare Flag 4B
            #[inline(always)]
            pub fn ocf4b(&mut self) -> OCF4B_W<'_, TIFR4_SPEC> {
                OCF4B_W::new(self, 2)
            }
            ///Bit 3 - Output Compare Flag 4C
            #[inline(always)]
            pub fn ocf4c(&mut self) -> OCF4C_W<'_, TIFR4_SPEC> {
                OCF4C_W::new(self, 3)
            }
            ///Bit 5 - Input Capture Flag 4
            #[inline(always)]
            pub fn icf4(&mut self) -> ICF4_W<'_, TIFR4_SPEC> {
                ICF4_W::new(self, 5)
            }
        }
        /**Timer/Counter4 Interrupt Flag register

You can [`read`](crate::Reg::read) this register and get [`tifr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIFR4_SPEC;
        impl crate::RegisterSpec for TIFR4_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tifr4::R`](R) reader structure
        impl crate::Readable for TIFR4_SPEC {}
        ///`write(|w| ..)` method takes [`tifr4::W`](W) writer structure
        impl crate::Writable for TIFR4_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIFR4 to value 0
        impl crate::Resettable for TIFR4_SPEC {}
    }
    /**TIMSK4 (rw) register accessor: Timer/Counter4 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`timsk4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timsk4`] module*/
    pub type TIMSK4 = crate::Reg<timsk4::TIMSK4_SPEC>;
    ///Timer/Counter4 Interrupt Mask Register
    pub mod timsk4 {
        ///Register `TIMSK4` reader
        pub type R = crate::R<TIMSK4_SPEC>;
        ///Register `TIMSK4` writer
        pub type W = crate::W<TIMSK4_SPEC>;
        ///Field `TOIE4` reader - Timer/Counter4 Overflow Interrupt Enable
        pub type TOIE4_R = crate::BitReader;
        ///Field `TOIE4` writer - Timer/Counter4 Overflow Interrupt Enable
        pub type TOIE4_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE4A` reader - Timer/Counter4 Output Compare A Match Interrupt Enable
        pub type OCIE4A_R = crate::BitReader;
        ///Field `OCIE4A` writer - Timer/Counter4 Output Compare A Match Interrupt Enable
        pub type OCIE4A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE4B` reader - Timer/Counter4 Output Compare B Match Interrupt Enable
        pub type OCIE4B_R = crate::BitReader;
        ///Field `OCIE4B` writer - Timer/Counter4 Output Compare B Match Interrupt Enable
        pub type OCIE4B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE4C` reader - Timer/Counter4 Output Compare C Match Interrupt Enable
        pub type OCIE4C_R = crate::BitReader;
        ///Field `OCIE4C` writer - Timer/Counter4 Output Compare C Match Interrupt Enable
        pub type OCIE4C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICIE4` reader - Timer/Counter4 Input Capture Interrupt Enable
        pub type ICIE4_R = crate::BitReader;
        ///Field `ICIE4` writer - Timer/Counter4 Input Capture Interrupt Enable
        pub type ICIE4_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter4 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie4(&self) -> TOIE4_R {
                TOIE4_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Timer/Counter4 Output Compare A Match Interrupt Enable
            #[inline(always)]
            pub fn ocie4a(&self) -> OCIE4A_R {
                OCIE4A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Timer/Counter4 Output Compare B Match Interrupt Enable
            #[inline(always)]
            pub fn ocie4b(&self) -> OCIE4B_R {
                OCIE4B_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Timer/Counter4 Output Compare C Match Interrupt Enable
            #[inline(always)]
            pub fn ocie4c(&self) -> OCIE4C_R {
                OCIE4C_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 5 - Timer/Counter4 Input Capture Interrupt Enable
            #[inline(always)]
            pub fn icie4(&self) -> ICIE4_R {
                ICIE4_R::new(((self.bits >> 5) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter4 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie4(&mut self) -> TOIE4_W<'_, TIMSK4_SPEC> {
                TOIE4_W::new(self, 0)
            }
            ///Bit 1 - Timer/Counter4 Output Compare A Match Interrupt Enable
            #[inline(always)]
            pub fn ocie4a(&mut self) -> OCIE4A_W<'_, TIMSK4_SPEC> {
                OCIE4A_W::new(self, 1)
            }
            ///Bit 2 - Timer/Counter4 Output Compare B Match Interrupt Enable
            #[inline(always)]
            pub fn ocie4b(&mut self) -> OCIE4B_W<'_, TIMSK4_SPEC> {
                OCIE4B_W::new(self, 2)
            }
            ///Bit 3 - Timer/Counter4 Output Compare C Match Interrupt Enable
            #[inline(always)]
            pub fn ocie4c(&mut self) -> OCIE4C_W<'_, TIMSK4_SPEC> {
                OCIE4C_W::new(self, 3)
            }
            ///Bit 5 - Timer/Counter4 Input Capture Interrupt Enable
            #[inline(always)]
            pub fn icie4(&mut self) -> ICIE4_W<'_, TIMSK4_SPEC> {
                ICIE4_W::new(self, 5)
            }
        }
        /**Timer/Counter4 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`timsk4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIMSK4_SPEC;
        impl crate::RegisterSpec for TIMSK4_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`timsk4::R`](R) reader structure
        impl crate::Readable for TIMSK4_SPEC {}
        ///`write(|w| ..)` method takes [`timsk4::W`](W) writer structure
        impl crate::Writable for TIMSK4_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIMSK4 to value 0
        impl crate::Resettable for TIMSK4_SPEC {}
    }
}
///Timer/Counter, 16-bit
pub type TC5 = crate::Periph<tc5::RegisterBlock, 0x3a>;
impl core::fmt::Debug for TC5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC5").finish()
    }
}
///Timer/Counter, 16-bit
pub mod tc5 {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        tifr5: TIFR5,
        _reserved1: [u8; 0x38],
        timsk5: TIMSK5,
        _reserved2: [u8; 0xac],
        tccr5a: TCCR5A,
        tccr5b: TCCR5B,
        tccr5c: TCCR5C,
        _reserved5: [u8; 0x01],
        tcnt5: TCNT5,
        icr5: ICR5,
        ocr5a: OCR5A,
        ocr5b: OCR5B,
        ocr5c: OCR5C,
    }
    impl RegisterBlock {
        ///0x00 - Timer/Counter5 Interrupt Flag register
        #[inline(always)]
        pub const fn tifr5(&self) -> &TIFR5 {
            &self.tifr5
        }
        ///0x39 - Timer/Counter5 Interrupt Mask Register
        #[inline(always)]
        pub const fn timsk5(&self) -> &TIMSK5 {
            &self.timsk5
        }
        ///0xe6 - Timer/Counter5 Control Register A
        #[inline(always)]
        pub const fn tccr5a(&self) -> &TCCR5A {
            &self.tccr5a
        }
        ///0xe7 - Timer/Counter5 Control Register B
        #[inline(always)]
        pub const fn tccr5b(&self) -> &TCCR5B {
            &self.tccr5b
        }
        ///0xe8 - Timer/Counter 5 Control Register C
        #[inline(always)]
        pub const fn tccr5c(&self) -> &TCCR5C {
            &self.tccr5c
        }
        ///0xea - Timer/Counter5 Bytes
        #[inline(always)]
        pub const fn tcnt5(&self) -> &TCNT5 {
            &self.tcnt5
        }
        ///0xec - Timer/Counter5 Input Capture Register Bytes
        #[inline(always)]
        pub const fn icr5(&self) -> &ICR5 {
            &self.icr5
        }
        ///0xee - Timer/Counter5 Output Compare Register A Bytes
        #[inline(always)]
        pub const fn ocr5a(&self) -> &OCR5A {
            &self.ocr5a
        }
        ///0xf0 - Timer/Counter5 Output Compare Register B Bytes
        #[inline(always)]
        pub const fn ocr5b(&self) -> &OCR5B {
            &self.ocr5b
        }
        ///0xf2 - Timer/Counter5 Output Compare Register B Bytes
        #[inline(always)]
        pub const fn ocr5c(&self) -> &OCR5C {
            &self.ocr5c
        }
    }
    /**ICR5 (rw) register accessor: Timer/Counter5 Input Capture Register Bytes

You can [`read`](crate::Reg::read) this register and get [`icr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icr5`] module*/
    pub type ICR5 = crate::Reg<icr5::ICR5_SPEC>;
    ///Timer/Counter5 Input Capture Register Bytes
    pub mod icr5 {
        ///Register `ICR5` reader
        pub type R = crate::R<ICR5_SPEC>;
        ///Register `ICR5` writer
        pub type W = crate::W<ICR5_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter5 Input Capture Register Bytes

You can [`read`](crate::Reg::read) this register and get [`icr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct ICR5_SPEC;
        impl crate::RegisterSpec for ICR5_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`icr5::R`](R) reader structure
        impl crate::Readable for ICR5_SPEC {}
        ///`write(|w| ..)` method takes [`icr5::W`](W) writer structure
        impl crate::Writable for ICR5_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets ICR5 to value 0
        impl crate::Resettable for ICR5_SPEC {}
    }
    /**OCR5A (rw) register accessor: Timer/Counter5 Output Compare Register A Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr5a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr5a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr5a`] module*/
    pub type OCR5A = crate::Reg<ocr5a::OCR5A_SPEC>;
    ///Timer/Counter5 Output Compare Register A Bytes
    pub mod ocr5a {
        ///Register `OCR5A` reader
        pub type R = crate::R<OCR5A_SPEC>;
        ///Register `OCR5A` writer
        pub type W = crate::W<OCR5A_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter5 Output Compare Register A Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr5a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr5a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR5A_SPEC;
        impl crate::RegisterSpec for OCR5A_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr5a::R`](R) reader structure
        impl crate::Readable for OCR5A_SPEC {}
        ///`write(|w| ..)` method takes [`ocr5a::W`](W) writer structure
        impl crate::Writable for OCR5A_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR5A to value 0
        impl crate::Resettable for OCR5A_SPEC {}
    }
    /**OCR5B (rw) register accessor: Timer/Counter5 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr5b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr5b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr5b`] module*/
    pub type OCR5B = crate::Reg<ocr5b::OCR5B_SPEC>;
    ///Timer/Counter5 Output Compare Register B Bytes
    pub mod ocr5b {
        ///Register `OCR5B` reader
        pub type R = crate::R<OCR5B_SPEC>;
        ///Register `OCR5B` writer
        pub type W = crate::W<OCR5B_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter5 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr5b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr5b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR5B_SPEC;
        impl crate::RegisterSpec for OCR5B_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr5b::R`](R) reader structure
        impl crate::Readable for OCR5B_SPEC {}
        ///`write(|w| ..)` method takes [`ocr5b::W`](W) writer structure
        impl crate::Writable for OCR5B_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR5B to value 0
        impl crate::Resettable for OCR5B_SPEC {}
    }
    /**OCR5C (rw) register accessor: Timer/Counter5 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ocr5c`] module*/
    pub type OCR5C = crate::Reg<ocr5c::OCR5C_SPEC>;
    ///Timer/Counter5 Output Compare Register B Bytes
    pub mod ocr5c {
        ///Register `OCR5C` reader
        pub type R = crate::R<OCR5C_SPEC>;
        ///Register `OCR5C` writer
        pub type W = crate::W<OCR5C_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter5 Output Compare Register B Bytes

You can [`read`](crate::Reg::read) this register and get [`ocr5c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr5c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct OCR5C_SPEC;
        impl crate::RegisterSpec for OCR5C_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ocr5c::R`](R) reader structure
        impl crate::Readable for OCR5C_SPEC {}
        ///`write(|w| ..)` method takes [`ocr5c::W`](W) writer structure
        impl crate::Writable for OCR5C_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets OCR5C to value 0
        impl crate::Resettable for OCR5C_SPEC {}
    }
    /**TCCR5A (rw) register accessor: Timer/Counter5 Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr5a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr5a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr5a`] module*/
    pub type TCCR5A = crate::Reg<tccr5a::TCCR5A_SPEC>;
    ///Timer/Counter5 Control Register A
    pub mod tccr5a {
        ///Register `TCCR5A` reader
        pub type R = crate::R<TCCR5A_SPEC>;
        ///Register `TCCR5A` writer
        pub type W = crate::W<TCCR5A_SPEC>;
        ///Field `WGM5` reader - Waveform Generation Mode
        pub type WGM5_R = crate::FieldReader;
        ///Field `WGM5` writer - Waveform Generation Mode
        pub type WGM5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
        /**Compare Output Mode 5C, bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum COM5C_A {
            ///0: Normal port operation, OCix disconnected
            DISCONNECTED = 0,
            ///1: Toggle OCix on Compare Match (Might depend on WGM)
            MATCH_TOGGLE = 1,
            ///2: Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            MATCH_CLEAR = 2,
            ///3: Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            MATCH_SET = 3,
        }
        impl From<COM5C_A> for u8 {
            #[inline(always)]
            fn from(variant: COM5C_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for COM5C_A {
            type Ux = u8;
        }
        impl crate::IsEnum for COM5C_A {}
        ///Field `COM5C` reader - Compare Output Mode 5C, bits
        pub type COM5C_R = crate::FieldReader<COM5C_A>;
        impl COM5C_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> COM5C_A {
                match self.bits {
                    0 => COM5C_A::DISCONNECTED,
                    1 => COM5C_A::MATCH_TOGGLE,
                    2 => COM5C_A::MATCH_CLEAR,
                    3 => COM5C_A::MATCH_SET,
                    _ => unreachable!(),
                }
            }
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn is_disconnected(&self) -> bool {
                *self == COM5C_A::DISCONNECTED
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn is_match_toggle(&self) -> bool {
                *self == COM5C_A::MATCH_TOGGLE
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            #[inline(always)]
            pub fn is_match_clear(&self) -> bool {
                *self == COM5C_A::MATCH_CLEAR
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            #[inline(always)]
            pub fn is_match_set(&self) -> bool {
                *self == COM5C_A::MATCH_SET
            }
        }
        ///Field `COM5C` writer - Compare Output Mode 5C, bits
        pub type COM5C_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COM5C_A, crate::Safe>;
        impl<'a, REG> COM5C_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Normal port operation, OCix disconnected
            #[inline(always)]
            pub fn disconnected(self) -> &'a mut crate::W<REG> {
                self.variant(COM5C_A::DISCONNECTED)
            }
            ///Toggle OCix on Compare Match (Might depend on WGM)
            #[inline(always)]
            pub fn match_toggle(self) -> &'a mut crate::W<REG> {
                self.variant(COM5C_A::MATCH_TOGGLE)
            }
            ///Clear OCix on Compare Match (If PWM is enabled, OCix is set at BOTTOM)
            #[inline(always)]
            pub fn match_clear(self) -> &'a mut crate::W<REG> {
                self.variant(COM5C_A::MATCH_CLEAR)
            }
            ///Set OCix on Compare Match (If PWM is enabled, OCix is cleared at BOTTOM)
            #[inline(always)]
            pub fn match_set(self) -> &'a mut crate::W<REG> {
                self.variant(COM5C_A::MATCH_SET)
            }
        }
        ///Field `COM5B` reader - Compare Output Mode 5B, bits
        pub use COM5C_R as COM5B_R;
        ///Field `COM5B` writer - Compare Output Mode 5B, bits
        pub use COM5C_W as COM5B_W;
        ///Field `COM5A` reader - Compare Output Mode 1A, bits
        pub use COM5C_R as COM5A_R;
        ///Field `COM5A` writer - Compare Output Mode 1A, bits
        pub use COM5C_W as COM5A_W;
        impl R {
            ///Bits 0:1 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm5(&self) -> WGM5_R {
                WGM5_R::new(self.bits & 3)
            }
            ///Bits 2:3 - Compare Output Mode 5C, bits
            #[inline(always)]
            pub fn com5c(&self) -> COM5C_R {
                COM5C_R::new((self.bits >> 2) & 3)
            }
            ///Bits 4:5 - Compare Output Mode 5B, bits
            #[inline(always)]
            pub fn com5b(&self) -> COM5B_R {
                COM5B_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - Compare Output Mode 1A, bits
            #[inline(always)]
            pub fn com5a(&self) -> COM5A_R {
                COM5A_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bits 0:1 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm5(&mut self) -> WGM5_W<'_, TCCR5A_SPEC> {
                WGM5_W::new(self, 0)
            }
            ///Bits 2:3 - Compare Output Mode 5C, bits
            #[inline(always)]
            pub fn com5c(&mut self) -> COM5C_W<'_, TCCR5A_SPEC> {
                COM5C_W::new(self, 2)
            }
            ///Bits 4:5 - Compare Output Mode 5B, bits
            #[inline(always)]
            pub fn com5b(&mut self) -> COM5B_W<'_, TCCR5A_SPEC> {
                COM5B_W::new(self, 4)
            }
            ///Bits 6:7 - Compare Output Mode 1A, bits
            #[inline(always)]
            pub fn com5a(&mut self) -> COM5A_W<'_, TCCR5A_SPEC> {
                COM5A_W::new(self, 6)
            }
        }
        /**Timer/Counter5 Control Register A

You can [`read`](crate::Reg::read) this register and get [`tccr5a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr5a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR5A_SPEC;
        impl crate::RegisterSpec for TCCR5A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr5a::R`](R) reader structure
        impl crate::Readable for TCCR5A_SPEC {}
        ///`write(|w| ..)` method takes [`tccr5a::W`](W) writer structure
        impl crate::Writable for TCCR5A_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR5A to value 0
        impl crate::Resettable for TCCR5A_SPEC {}
    }
    /**TCCR5B (rw) register accessor: Timer/Counter5 Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr5b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr5b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr5b`] module*/
    pub type TCCR5B = crate::Reg<tccr5b::TCCR5B_SPEC>;
    ///Timer/Counter5 Control Register B
    pub mod tccr5b {
        ///Register `TCCR5B` reader
        pub type R = crate::R<TCCR5B_SPEC>;
        ///Register `TCCR5B` writer
        pub type W = crate::W<TCCR5B_SPEC>;
        /**Prescaler source of Timer/Counter 5

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum CS5_A {
            ///0: No clock source (Timer/Counter stopped)
            NO_CLOCK = 0,
            ///1: Running, No Prescaling
            DIRECT = 1,
            ///2: Running, CLK/8
            PRESCALE_8 = 2,
            ///3: Running, CLK/64
            PRESCALE_64 = 3,
            ///4: Running, CLK/256
            PRESCALE_256 = 4,
            ///5: Running, CLK/1024
            PRESCALE_1024 = 5,
            ///6: Running, ExtClk Tx Falling Edge
            EXT_FALLING = 6,
            ///7: Running, ExtClk Tx Rising Edge
            EXT_RISING = 7,
        }
        impl From<CS5_A> for u8 {
            #[inline(always)]
            fn from(variant: CS5_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for CS5_A {
            type Ux = u8;
        }
        impl crate::IsEnum for CS5_A {}
        ///Field `CS5` reader - Prescaler source of Timer/Counter 5
        pub type CS5_R = crate::FieldReader<CS5_A>;
        impl CS5_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> CS5_A {
                match self.bits {
                    0 => CS5_A::NO_CLOCK,
                    1 => CS5_A::DIRECT,
                    2 => CS5_A::PRESCALE_8,
                    3 => CS5_A::PRESCALE_64,
                    4 => CS5_A::PRESCALE_256,
                    5 => CS5_A::PRESCALE_1024,
                    6 => CS5_A::EXT_FALLING,
                    7 => CS5_A::EXT_RISING,
                    _ => unreachable!(),
                }
            }
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn is_no_clock(&self) -> bool {
                *self == CS5_A::NO_CLOCK
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn is_direct(&self) -> bool {
                *self == CS5_A::DIRECT
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn is_prescale_8(&self) -> bool {
                *self == CS5_A::PRESCALE_8
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn is_prescale_64(&self) -> bool {
                *self == CS5_A::PRESCALE_64
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn is_prescale_256(&self) -> bool {
                *self == CS5_A::PRESCALE_256
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn is_prescale_1024(&self) -> bool {
                *self == CS5_A::PRESCALE_1024
            }
            ///Running, ExtClk Tx Falling Edge
            #[inline(always)]
            pub fn is_ext_falling(&self) -> bool {
                *self == CS5_A::EXT_FALLING
            }
            ///Running, ExtClk Tx Rising Edge
            #[inline(always)]
            pub fn is_ext_rising(&self) -> bool {
                *self == CS5_A::EXT_RISING
            }
        }
        ///Field `CS5` writer - Prescaler source of Timer/Counter 5
        pub type CS5_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CS5_A, crate::Safe>;
        impl<'a, REG> CS5_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///No clock source (Timer/Counter stopped)
            #[inline(always)]
            pub fn no_clock(self) -> &'a mut crate::W<REG> {
                self.variant(CS5_A::NO_CLOCK)
            }
            ///Running, No Prescaling
            #[inline(always)]
            pub fn direct(self) -> &'a mut crate::W<REG> {
                self.variant(CS5_A::DIRECT)
            }
            ///Running, CLK/8
            #[inline(always)]
            pub fn prescale_8(self) -> &'a mut crate::W<REG> {
                self.variant(CS5_A::PRESCALE_8)
            }
            ///Running, CLK/64
            #[inline(always)]
            pub fn prescale_64(self) -> &'a mut crate::W<REG> {
                self.variant(CS5_A::PRESCALE_64)
            }
            ///Running, CLK/256
            #[inline(always)]
            pub fn prescale_256(self) -> &'a mut crate::W<REG> {
                self.variant(CS5_A::PRESCALE_256)
            }
            ///Running, CLK/1024
            #[inline(always)]
            pub fn prescale_1024(self) -> &'a mut crate::W<REG> {
                self.variant(CS5_A::PRESCALE_1024)
            }
            ///Running, ExtClk Tx Falling Edge
            #[inline(always)]
            pub fn ext_falling(self) -> &'a mut crate::W<REG> {
                self.variant(CS5_A::EXT_FALLING)
            }
            ///Running, ExtClk Tx Rising Edge
            #[inline(always)]
            pub fn ext_rising(self) -> &'a mut crate::W<REG> {
                self.variant(CS5_A::EXT_RISING)
            }
        }
        ///Field `WGM5` reader - Waveform Generation Mode
        pub type WGM5_R = crate::FieldReader;
        ///Field `WGM5` writer - Waveform Generation Mode
        pub type WGM5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
        ///Field `ICES5` reader - Input Capture 5 Edge Select
        pub type ICES5_R = crate::BitReader;
        ///Field `ICES5` writer - Input Capture 5 Edge Select
        pub type ICES5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICNC5` reader - Input Capture 5 Noise Canceler
        pub type ICNC5_R = crate::BitReader;
        ///Field `ICNC5` writer - Input Capture 5 Noise Canceler
        pub type ICNC5_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:2 - Prescaler source of Timer/Counter 5
            #[inline(always)]
            pub fn cs5(&self) -> CS5_R {
                CS5_R::new(self.bits & 7)
            }
            ///Bits 3:4 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm5(&self) -> WGM5_R {
                WGM5_R::new((self.bits >> 3) & 3)
            }
            ///Bit 6 - Input Capture 5 Edge Select
            #[inline(always)]
            pub fn ices5(&self) -> ICES5_R {
                ICES5_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Input Capture 5 Noise Canceler
            #[inline(always)]
            pub fn icnc5(&self) -> ICNC5_R {
                ICNC5_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:2 - Prescaler source of Timer/Counter 5
            #[inline(always)]
            pub fn cs5(&mut self) -> CS5_W<'_, TCCR5B_SPEC> {
                CS5_W::new(self, 0)
            }
            ///Bits 3:4 - Waveform Generation Mode
            #[inline(always)]
            pub fn wgm5(&mut self) -> WGM5_W<'_, TCCR5B_SPEC> {
                WGM5_W::new(self, 3)
            }
            ///Bit 6 - Input Capture 5 Edge Select
            #[inline(always)]
            pub fn ices5(&mut self) -> ICES5_W<'_, TCCR5B_SPEC> {
                ICES5_W::new(self, 6)
            }
            ///Bit 7 - Input Capture 5 Noise Canceler
            #[inline(always)]
            pub fn icnc5(&mut self) -> ICNC5_W<'_, TCCR5B_SPEC> {
                ICNC5_W::new(self, 7)
            }
        }
        /**Timer/Counter5 Control Register B

You can [`read`](crate::Reg::read) this register and get [`tccr5b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr5b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR5B_SPEC;
        impl crate::RegisterSpec for TCCR5B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr5b::R`](R) reader structure
        impl crate::Readable for TCCR5B_SPEC {}
        ///`write(|w| ..)` method takes [`tccr5b::W`](W) writer structure
        impl crate::Writable for TCCR5B_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR5B to value 0
        impl crate::Resettable for TCCR5B_SPEC {}
    }
    /**TCCR5C (rw) register accessor: Timer/Counter 5 Control Register C

You can [`read`](crate::Reg::read) this register and get [`tccr5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tccr5c`] module*/
    pub type TCCR5C = crate::Reg<tccr5c::TCCR5C_SPEC>;
    ///Timer/Counter 5 Control Register C
    pub mod tccr5c {
        ///Register `TCCR5C` reader
        pub type R = crate::R<TCCR5C_SPEC>;
        ///Register `TCCR5C` writer
        pub type W = crate::W<TCCR5C_SPEC>;
        ///Field `FOC5C` writer - Force Output Compare 5C
        pub type FOC5C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC5B` writer - Force Output Compare 5B
        pub type FOC5B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `FOC5A` writer - Force Output Compare 5A
        pub type FOC5A_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl W {
            ///Bit 5 - Force Output Compare 5C
            #[inline(always)]
            pub fn foc5c(&mut self) -> FOC5C_W<'_, TCCR5C_SPEC> {
                FOC5C_W::new(self, 5)
            }
            ///Bit 6 - Force Output Compare 5B
            #[inline(always)]
            pub fn foc5b(&mut self) -> FOC5B_W<'_, TCCR5C_SPEC> {
                FOC5B_W::new(self, 6)
            }
            ///Bit 7 - Force Output Compare 5A
            #[inline(always)]
            pub fn foc5a(&mut self) -> FOC5A_W<'_, TCCR5C_SPEC> {
                FOC5A_W::new(self, 7)
            }
        }
        /**Timer/Counter 5 Control Register C

You can [`read`](crate::Reg::read) this register and get [`tccr5c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr5c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCCR5C_SPEC;
        impl crate::RegisterSpec for TCCR5C_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tccr5c::R`](R) reader structure
        impl crate::Readable for TCCR5C_SPEC {}
        ///`write(|w| ..)` method takes [`tccr5c::W`](W) writer structure
        impl crate::Writable for TCCR5C_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TCCR5C to value 0
        impl crate::Resettable for TCCR5C_SPEC {}
    }
    /**TCNT5 (rw) register accessor: Timer/Counter5 Bytes

You can [`read`](crate::Reg::read) this register and get [`tcnt5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcnt5`] module*/
    pub type TCNT5 = crate::Reg<tcnt5::TCNT5_SPEC>;
    ///Timer/Counter5 Bytes
    pub mod tcnt5 {
        ///Register `TCNT5` reader
        pub type R = crate::R<TCNT5_SPEC>;
        ///Register `TCNT5` writer
        pub type W = crate::W<TCNT5_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**Timer/Counter5 Bytes

You can [`read`](crate::Reg::read) this register and get [`tcnt5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TCNT5_SPEC;
        impl crate::RegisterSpec for TCNT5_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`tcnt5::R`](R) reader structure
        impl crate::Readable for TCNT5_SPEC {}
        ///`write(|w| ..)` method takes [`tcnt5::W`](W) writer structure
        impl crate::Writable for TCNT5_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets TCNT5 to value 0
        impl crate::Resettable for TCNT5_SPEC {}
    }
    /**TIFR5 (rw) register accessor: Timer/Counter5 Interrupt Flag register

You can [`read`](crate::Reg::read) this register and get [`tifr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tifr5`] module*/
    pub type TIFR5 = crate::Reg<tifr5::TIFR5_SPEC>;
    ///Timer/Counter5 Interrupt Flag register
    pub mod tifr5 {
        ///Register `TIFR5` reader
        pub type R = crate::R<TIFR5_SPEC>;
        ///Register `TIFR5` writer
        pub type W = crate::W<TIFR5_SPEC>;
        ///Field `TOV5` reader - Timer/Counter5 Overflow Flag
        pub type TOV5_R = crate::BitReader;
        ///Field `TOV5` writer - Timer/Counter5 Overflow Flag
        pub type TOV5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF5A` reader - Output Compare Flag 5A
        pub type OCF5A_R = crate::BitReader;
        ///Field `OCF5A` writer - Output Compare Flag 5A
        pub type OCF5A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF5B` reader - Output Compare Flag 5B
        pub type OCF5B_R = crate::BitReader;
        ///Field `OCF5B` writer - Output Compare Flag 5B
        pub type OCF5B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCF5C` reader - Output Compare Flag 5C
        pub type OCF5C_R = crate::BitReader;
        ///Field `OCF5C` writer - Output Compare Flag 5C
        pub type OCF5C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICF5` reader - Input Capture Flag 5
        pub type ICF5_R = crate::BitReader;
        ///Field `ICF5` writer - Input Capture Flag 5
        pub type ICF5_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter5 Overflow Flag
            #[inline(always)]
            pub fn tov5(&self) -> TOV5_R {
                TOV5_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Output Compare Flag 5A
            #[inline(always)]
            pub fn ocf5a(&self) -> OCF5A_R {
                OCF5A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Output Compare Flag 5B
            #[inline(always)]
            pub fn ocf5b(&self) -> OCF5B_R {
                OCF5B_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Output Compare Flag 5C
            #[inline(always)]
            pub fn ocf5c(&self) -> OCF5C_R {
                OCF5C_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 5 - Input Capture Flag 5
            #[inline(always)]
            pub fn icf5(&self) -> ICF5_R {
                ICF5_R::new(((self.bits >> 5) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter5 Overflow Flag
            #[inline(always)]
            pub fn tov5(&mut self) -> TOV5_W<'_, TIFR5_SPEC> {
                TOV5_W::new(self, 0)
            }
            ///Bit 1 - Output Compare Flag 5A
            #[inline(always)]
            pub fn ocf5a(&mut self) -> OCF5A_W<'_, TIFR5_SPEC> {
                OCF5A_W::new(self, 1)
            }
            ///Bit 2 - Output Compare Flag 5B
            #[inline(always)]
            pub fn ocf5b(&mut self) -> OCF5B_W<'_, TIFR5_SPEC> {
                OCF5B_W::new(self, 2)
            }
            ///Bit 3 - Output Compare Flag 5C
            #[inline(always)]
            pub fn ocf5c(&mut self) -> OCF5C_W<'_, TIFR5_SPEC> {
                OCF5C_W::new(self, 3)
            }
            ///Bit 5 - Input Capture Flag 5
            #[inline(always)]
            pub fn icf5(&mut self) -> ICF5_W<'_, TIFR5_SPEC> {
                ICF5_W::new(self, 5)
            }
        }
        /**Timer/Counter5 Interrupt Flag register

You can [`read`](crate::Reg::read) this register and get [`tifr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIFR5_SPEC;
        impl crate::RegisterSpec for TIFR5_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`tifr5::R`](R) reader structure
        impl crate::Readable for TIFR5_SPEC {}
        ///`write(|w| ..)` method takes [`tifr5::W`](W) writer structure
        impl crate::Writable for TIFR5_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIFR5 to value 0
        impl crate::Resettable for TIFR5_SPEC {}
    }
    /**TIMSK5 (rw) register accessor: Timer/Counter5 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`timsk5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timsk5`] module*/
    pub type TIMSK5 = crate::Reg<timsk5::TIMSK5_SPEC>;
    ///Timer/Counter5 Interrupt Mask Register
    pub mod timsk5 {
        ///Register `TIMSK5` reader
        pub type R = crate::R<TIMSK5_SPEC>;
        ///Register `TIMSK5` writer
        pub type W = crate::W<TIMSK5_SPEC>;
        ///Field `TOIE5` reader - Timer/Counter5 Overflow Interrupt Enable
        pub type TOIE5_R = crate::BitReader;
        ///Field `TOIE5` writer - Timer/Counter5 Overflow Interrupt Enable
        pub type TOIE5_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE5A` reader - Timer/Counter5 Output Compare A Match Interrupt Enable
        pub type OCIE5A_R = crate::BitReader;
        ///Field `OCIE5A` writer - Timer/Counter5 Output Compare A Match Interrupt Enable
        pub type OCIE5A_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE5B` reader - Timer/Counter5 Output Compare B Match Interrupt Enable
        pub type OCIE5B_R = crate::BitReader;
        ///Field `OCIE5B` writer - Timer/Counter5 Output Compare B Match Interrupt Enable
        pub type OCIE5B_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `OCIE5C` reader - Timer/Counter5 Output Compare C Match Interrupt Enable
        pub type OCIE5C_R = crate::BitReader;
        ///Field `OCIE5C` writer - Timer/Counter5 Output Compare C Match Interrupt Enable
        pub type OCIE5C_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `ICIE5` reader - Timer/Counter5 Input Capture Interrupt Enable
        pub type ICIE5_R = crate::BitReader;
        ///Field `ICIE5` writer - Timer/Counter5 Input Capture Interrupt Enable
        pub type ICIE5_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Timer/Counter5 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie5(&self) -> TOIE5_R {
                TOIE5_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Timer/Counter5 Output Compare A Match Interrupt Enable
            #[inline(always)]
            pub fn ocie5a(&self) -> OCIE5A_R {
                OCIE5A_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Timer/Counter5 Output Compare B Match Interrupt Enable
            #[inline(always)]
            pub fn ocie5b(&self) -> OCIE5B_R {
                OCIE5B_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Timer/Counter5 Output Compare C Match Interrupt Enable
            #[inline(always)]
            pub fn ocie5c(&self) -> OCIE5C_R {
                OCIE5C_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 5 - Timer/Counter5 Input Capture Interrupt Enable
            #[inline(always)]
            pub fn icie5(&self) -> ICIE5_R {
                ICIE5_R::new(((self.bits >> 5) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Timer/Counter5 Overflow Interrupt Enable
            #[inline(always)]
            pub fn toie5(&mut self) -> TOIE5_W<'_, TIMSK5_SPEC> {
                TOIE5_W::new(self, 0)
            }
            ///Bit 1 - Timer/Counter5 Output Compare A Match Interrupt Enable
            #[inline(always)]
            pub fn ocie5a(&mut self) -> OCIE5A_W<'_, TIMSK5_SPEC> {
                OCIE5A_W::new(self, 1)
            }
            ///Bit 2 - Timer/Counter5 Output Compare B Match Interrupt Enable
            #[inline(always)]
            pub fn ocie5b(&mut self) -> OCIE5B_W<'_, TIMSK5_SPEC> {
                OCIE5B_W::new(self, 2)
            }
            ///Bit 3 - Timer/Counter5 Output Compare C Match Interrupt Enable
            #[inline(always)]
            pub fn ocie5c(&mut self) -> OCIE5C_W<'_, TIMSK5_SPEC> {
                OCIE5C_W::new(self, 3)
            }
            ///Bit 5 - Timer/Counter5 Input Capture Interrupt Enable
            #[inline(always)]
            pub fn icie5(&mut self) -> ICIE5_W<'_, TIMSK5_SPEC> {
                ICIE5_W::new(self, 5)
            }
        }
        /**Timer/Counter5 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`timsk5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timsk5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TIMSK5_SPEC;
        impl crate::RegisterSpec for TIMSK5_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`timsk5::R`](R) reader structure
        impl crate::Readable for TIMSK5_SPEC {}
        ///`write(|w| ..)` method takes [`timsk5::W`](W) writer structure
        impl crate::Writable for TIMSK5_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TIMSK5 to value 0
        impl crate::Resettable for TIMSK5_SPEC {}
    }
}
///Two Wire Serial Interface
pub type TWI = crate::Periph<twi::RegisterBlock, 0xb8>;
impl core::fmt::Debug for TWI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWI").finish()
    }
}
///Two Wire Serial Interface
pub mod twi {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        twbr: TWBR,
        twsr: TWSR,
        twar: TWAR,
        twdr: TWDR,
        twcr: TWCR,
        twamr: TWAMR,
    }
    impl RegisterBlock {
        ///0x00 - TWI Bit Rate register
        #[inline(always)]
        pub const fn twbr(&self) -> &TWBR {
            &self.twbr
        }
        ///0x01 - TWI Status Register
        #[inline(always)]
        pub const fn twsr(&self) -> &TWSR {
            &self.twsr
        }
        ///0x02 - TWI (Slave) Address register
        #[inline(always)]
        pub const fn twar(&self) -> &TWAR {
            &self.twar
        }
        ///0x03 - TWI Data register
        #[inline(always)]
        pub const fn twdr(&self) -> &TWDR {
            &self.twdr
        }
        ///0x04 - TWI Control Register
        #[inline(always)]
        pub const fn twcr(&self) -> &TWCR {
            &self.twcr
        }
        ///0x05 - TWI (Slave) Address Mask Register
        #[inline(always)]
        pub const fn twamr(&self) -> &TWAMR {
            &self.twamr
        }
    }
    /**TWAMR (rw) register accessor: TWI (Slave) Address Mask Register

You can [`read`](crate::Reg::read) this register and get [`twamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@twamr`] module*/
    pub type TWAMR = crate::Reg<twamr::TWAMR_SPEC>;
    ///TWI (Slave) Address Mask Register
    pub mod twamr {
        ///Register `TWAMR` reader
        pub type R = crate::R<TWAMR_SPEC>;
        ///Register `TWAMR` writer
        pub type W = crate::W<TWAMR_SPEC>;
        ///Field `TWAM` reader - TWI (Slave) Address Mask Bits
        pub type TWAM_R = crate::FieldReader;
        ///Field `TWAM` writer - TWI (Slave) Address Mask Bits
        pub type TWAM_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
        impl R {
            ///Bits 1:7 - TWI (Slave) Address Mask Bits
            #[inline(always)]
            pub fn twam(&self) -> TWAM_R {
                TWAM_R::new((self.bits >> 1) & 0x7f)
            }
        }
        impl W {
            ///Bits 1:7 - TWI (Slave) Address Mask Bits
            #[inline(always)]
            pub fn twam(&mut self) -> TWAM_W<'_, TWAMR_SPEC> {
                TWAM_W::new(self, 1)
            }
        }
        /**TWI (Slave) Address Mask Register

You can [`read`](crate::Reg::read) this register and get [`twamr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twamr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TWAMR_SPEC;
        impl crate::RegisterSpec for TWAMR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`twamr::R`](R) reader structure
        impl crate::Readable for TWAMR_SPEC {}
        ///`write(|w| ..)` method takes [`twamr::W`](W) writer structure
        impl crate::Writable for TWAMR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TWAMR to value 0
        impl crate::Resettable for TWAMR_SPEC {}
    }
    /**TWAR (rw) register accessor: TWI (Slave) Address register

You can [`read`](crate::Reg::read) this register and get [`twar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@twar`] module*/
    pub type TWAR = crate::Reg<twar::TWAR_SPEC>;
    ///TWI (Slave) Address register
    pub mod twar {
        ///Register `TWAR` reader
        pub type R = crate::R<TWAR_SPEC>;
        ///Register `TWAR` writer
        pub type W = crate::W<TWAR_SPEC>;
        ///Field `TWGCE` reader - TWI General Call Recognition Enable Bit
        pub type TWGCE_R = crate::BitReader;
        ///Field `TWGCE` writer - TWI General Call Recognition Enable Bit
        pub type TWGCE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TWA` reader - TWI (Slave) Address register Bits
        pub type TWA_R = crate::FieldReader;
        ///Field `TWA` writer - TWI (Slave) Address register Bits
        pub type TWA_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
        impl R {
            ///Bit 0 - TWI General Call Recognition Enable Bit
            #[inline(always)]
            pub fn twgce(&self) -> TWGCE_R {
                TWGCE_R::new((self.bits & 1) != 0)
            }
            ///Bits 1:7 - TWI (Slave) Address register Bits
            #[inline(always)]
            pub fn twa(&self) -> TWA_R {
                TWA_R::new((self.bits >> 1) & 0x7f)
            }
        }
        impl W {
            ///Bit 0 - TWI General Call Recognition Enable Bit
            #[inline(always)]
            pub fn twgce(&mut self) -> TWGCE_W<'_, TWAR_SPEC> {
                TWGCE_W::new(self, 0)
            }
            ///Bits 1:7 - TWI (Slave) Address register Bits
            #[inline(always)]
            pub fn twa(&mut self) -> TWA_W<'_, TWAR_SPEC> {
                TWA_W::new(self, 1)
            }
        }
        /**TWI (Slave) Address register

You can [`read`](crate::Reg::read) this register and get [`twar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TWAR_SPEC;
        impl crate::RegisterSpec for TWAR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`twar::R`](R) reader structure
        impl crate::Readable for TWAR_SPEC {}
        ///`write(|w| ..)` method takes [`twar::W`](W) writer structure
        impl crate::Writable for TWAR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TWAR to value 0
        impl crate::Resettable for TWAR_SPEC {}
    }
    /**TWBR (rw) register accessor: TWI Bit Rate register

You can [`read`](crate::Reg::read) this register and get [`twbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@twbr`] module*/
    pub type TWBR = crate::Reg<twbr::TWBR_SPEC>;
    ///TWI Bit Rate register
    pub mod twbr {
        ///Register `TWBR` reader
        pub type R = crate::R<TWBR_SPEC>;
        ///Register `TWBR` writer
        pub type W = crate::W<TWBR_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**TWI Bit Rate register

You can [`read`](crate::Reg::read) this register and get [`twbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TWBR_SPEC;
        impl crate::RegisterSpec for TWBR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`twbr::R`](R) reader structure
        impl crate::Readable for TWBR_SPEC {}
        ///`write(|w| ..)` method takes [`twbr::W`](W) writer structure
        impl crate::Writable for TWBR_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets TWBR to value 0
        impl crate::Resettable for TWBR_SPEC {}
    }
    /**TWCR (rw) register accessor: TWI Control Register

You can [`read`](crate::Reg::read) this register and get [`twcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@twcr`] module*/
    pub type TWCR = crate::Reg<twcr::TWCR_SPEC>;
    ///TWI Control Register
    pub mod twcr {
        ///Register `TWCR` reader
        pub type R = crate::R<TWCR_SPEC>;
        ///Register `TWCR` writer
        pub type W = crate::W<TWCR_SPEC>;
        ///Field `TWIE` reader - TWI Interrupt Enable
        pub type TWIE_R = crate::BitReader;
        ///Field `TWIE` writer - TWI Interrupt Enable
        pub type TWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TWEN` reader - TWI Enable Bit
        pub type TWEN_R = crate::BitReader;
        ///Field `TWEN` writer - TWI Enable Bit
        pub type TWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TWWC` reader - TWI Write Collition Flag
        pub type TWWC_R = crate::BitReader;
        ///Field `TWSTO` reader - TWI Stop Condition Bit
        pub type TWSTO_R = crate::BitReader;
        ///Field `TWSTO` writer - TWI Stop Condition Bit
        pub type TWSTO_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TWSTA` reader - TWI Start Condition Bit
        pub type TWSTA_R = crate::BitReader;
        ///Field `TWSTA` writer - TWI Start Condition Bit
        pub type TWSTA_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TWEA` reader - TWI Enable Acknowledge Bit
        pub type TWEA_R = crate::BitReader;
        ///Field `TWEA` writer - TWI Enable Acknowledge Bit
        pub type TWEA_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TWINT` reader - TWI Interrupt Flag
        pub type TWINT_R = crate::BitReader;
        ///Field `TWINT` writer - TWI Interrupt Flag
        pub type TWINT_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - TWI Interrupt Enable
            #[inline(always)]
            pub fn twie(&self) -> TWIE_R {
                TWIE_R::new((self.bits & 1) != 0)
            }
            ///Bit 2 - TWI Enable Bit
            #[inline(always)]
            pub fn twen(&self) -> TWEN_R {
                TWEN_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - TWI Write Collition Flag
            #[inline(always)]
            pub fn twwc(&self) -> TWWC_R {
                TWWC_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - TWI Stop Condition Bit
            #[inline(always)]
            pub fn twsto(&self) -> TWSTO_R {
                TWSTO_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - TWI Start Condition Bit
            #[inline(always)]
            pub fn twsta(&self) -> TWSTA_R {
                TWSTA_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - TWI Enable Acknowledge Bit
            #[inline(always)]
            pub fn twea(&self) -> TWEA_R {
                TWEA_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - TWI Interrupt Flag
            #[inline(always)]
            pub fn twint(&self) -> TWINT_R {
                TWINT_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - TWI Interrupt Enable
            #[inline(always)]
            pub fn twie(&mut self) -> TWIE_W<'_, TWCR_SPEC> {
                TWIE_W::new(self, 0)
            }
            ///Bit 2 - TWI Enable Bit
            #[inline(always)]
            pub fn twen(&mut self) -> TWEN_W<'_, TWCR_SPEC> {
                TWEN_W::new(self, 2)
            }
            ///Bit 4 - TWI Stop Condition Bit
            #[inline(always)]
            pub fn twsto(&mut self) -> TWSTO_W<'_, TWCR_SPEC> {
                TWSTO_W::new(self, 4)
            }
            ///Bit 5 - TWI Start Condition Bit
            #[inline(always)]
            pub fn twsta(&mut self) -> TWSTA_W<'_, TWCR_SPEC> {
                TWSTA_W::new(self, 5)
            }
            ///Bit 6 - TWI Enable Acknowledge Bit
            #[inline(always)]
            pub fn twea(&mut self) -> TWEA_W<'_, TWCR_SPEC> {
                TWEA_W::new(self, 6)
            }
            ///Bit 7 - TWI Interrupt Flag
            #[inline(always)]
            pub fn twint(&mut self) -> TWINT_W<'_, TWCR_SPEC> {
                TWINT_W::new(self, 7)
            }
        }
        /**TWI Control Register

You can [`read`](crate::Reg::read) this register and get [`twcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TWCR_SPEC;
        impl crate::RegisterSpec for TWCR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`twcr::R`](R) reader structure
        impl crate::Readable for TWCR_SPEC {}
        ///`write(|w| ..)` method takes [`twcr::W`](W) writer structure
        impl crate::Writable for TWCR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TWCR to value 0
        impl crate::Resettable for TWCR_SPEC {}
    }
    /**TWDR (rw) register accessor: TWI Data register

You can [`read`](crate::Reg::read) this register and get [`twdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@twdr`] module*/
    pub type TWDR = crate::Reg<twdr::TWDR_SPEC>;
    ///TWI Data register
    pub mod twdr {
        ///Register `TWDR` reader
        pub type R = crate::R<TWDR_SPEC>;
        ///Register `TWDR` writer
        pub type W = crate::W<TWDR_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**TWI Data register

You can [`read`](crate::Reg::read) this register and get [`twdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TWDR_SPEC;
        impl crate::RegisterSpec for TWDR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`twdr::R`](R) reader structure
        impl crate::Readable for TWDR_SPEC {}
        ///`write(|w| ..)` method takes [`twdr::W`](W) writer structure
        impl crate::Writable for TWDR_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets TWDR to value 0
        impl crate::Resettable for TWDR_SPEC {}
    }
    /**TWSR (rw) register accessor: TWI Status Register

You can [`read`](crate::Reg::read) this register and get [`twsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@twsr`] module*/
    pub type TWSR = crate::Reg<twsr::TWSR_SPEC>;
    ///TWI Status Register
    pub mod twsr {
        ///Register `TWSR` reader
        pub type R = crate::R<TWSR_SPEC>;
        ///Register `TWSR` writer
        pub type W = crate::W<TWSR_SPEC>;
        /**TWI Prescaler

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum TWPS_A {
            ///0: Prescaler Value 1
            PRESCALER_1 = 0,
            ///1: Prescaler Value 4
            PRESCALER_4 = 1,
            ///2: Prescaler Value 16
            PRESCALER_16 = 2,
            ///3: Prescaler Value 64
            PRESCALER_64 = 3,
        }
        impl From<TWPS_A> for u8 {
            #[inline(always)]
            fn from(variant: TWPS_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for TWPS_A {
            type Ux = u8;
        }
        impl crate::IsEnum for TWPS_A {}
        ///Field `TWPS` reader - TWI Prescaler
        pub type TWPS_R = crate::FieldReader<TWPS_A>;
        impl TWPS_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> TWPS_A {
                match self.bits {
                    0 => TWPS_A::PRESCALER_1,
                    1 => TWPS_A::PRESCALER_4,
                    2 => TWPS_A::PRESCALER_16,
                    3 => TWPS_A::PRESCALER_64,
                    _ => unreachable!(),
                }
            }
            ///Prescaler Value 1
            #[inline(always)]
            pub fn is_prescaler_1(&self) -> bool {
                *self == TWPS_A::PRESCALER_1
            }
            ///Prescaler Value 4
            #[inline(always)]
            pub fn is_prescaler_4(&self) -> bool {
                *self == TWPS_A::PRESCALER_4
            }
            ///Prescaler Value 16
            #[inline(always)]
            pub fn is_prescaler_16(&self) -> bool {
                *self == TWPS_A::PRESCALER_16
            }
            ///Prescaler Value 64
            #[inline(always)]
            pub fn is_prescaler_64(&self) -> bool {
                *self == TWPS_A::PRESCALER_64
            }
        }
        ///Field `TWPS` writer - TWI Prescaler
        pub type TWPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TWPS_A, crate::Safe>;
        impl<'a, REG> TWPS_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Prescaler Value 1
            #[inline(always)]
            pub fn prescaler_1(self) -> &'a mut crate::W<REG> {
                self.variant(TWPS_A::PRESCALER_1)
            }
            ///Prescaler Value 4
            #[inline(always)]
            pub fn prescaler_4(self) -> &'a mut crate::W<REG> {
                self.variant(TWPS_A::PRESCALER_4)
            }
            ///Prescaler Value 16
            #[inline(always)]
            pub fn prescaler_16(self) -> &'a mut crate::W<REG> {
                self.variant(TWPS_A::PRESCALER_16)
            }
            ///Prescaler Value 64
            #[inline(always)]
            pub fn prescaler_64(self) -> &'a mut crate::W<REG> {
                self.variant(TWPS_A::PRESCALER_64)
            }
        }
        ///Field `TWS` reader - TWI Status
        pub type TWS_R = crate::FieldReader;
        impl R {
            ///Bits 0:1 - TWI Prescaler
            #[inline(always)]
            pub fn twps(&self) -> TWPS_R {
                TWPS_R::new(self.bits & 3)
            }
            ///Bits 3:7 - TWI Status
            #[inline(always)]
            pub fn tws(&self) -> TWS_R {
                TWS_R::new((self.bits >> 3) & 0x1f)
            }
        }
        impl W {
            ///Bits 0:1 - TWI Prescaler
            #[inline(always)]
            pub fn twps(&mut self) -> TWPS_W<'_, TWSR_SPEC> {
                TWPS_W::new(self, 0)
            }
        }
        /**TWI Status Register

You can [`read`](crate::Reg::read) this register and get [`twsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct TWSR_SPEC;
        impl crate::RegisterSpec for TWSR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`twsr::R`](R) reader structure
        impl crate::Readable for TWSR_SPEC {}
        ///`write(|w| ..)` method takes [`twsr::W`](W) writer structure
        impl crate::Writable for TWSR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets TWSR to value 0
        impl crate::Resettable for TWSR_SPEC {}
    }
}
///USART
pub type USART0 = crate::Periph<usart0::RegisterBlock, 0xc0>;
impl core::fmt::Debug for USART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART0").finish()
    }
}
///USART
pub mod usart0 {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        ucsr0a: UCSR0A,
        ucsr0b: UCSR0B,
        ucsr0c: UCSR0C,
        _reserved3: [u8; 0x01],
        ubrr0: UBRR0,
        udr0: UDR0,
    }
    impl RegisterBlock {
        ///0x00 - USART Control and Status Register A
        #[inline(always)]
        pub const fn ucsr0a(&self) -> &UCSR0A {
            &self.ucsr0a
        }
        ///0x01 - USART Control and Status Register B
        #[inline(always)]
        pub const fn ucsr0b(&self) -> &UCSR0B {
            &self.ucsr0b
        }
        ///0x02 - USART Control and Status Register C
        #[inline(always)]
        pub const fn ucsr0c(&self) -> &UCSR0C {
            &self.ucsr0c
        }
        ///0x04 - USART Baud Rate Register Bytes
        #[inline(always)]
        pub const fn ubrr0(&self) -> &UBRR0 {
            &self.ubrr0
        }
        ///0x06 - USART I/O Data Register
        #[inline(always)]
        pub const fn udr0(&self) -> &UDR0 {
            &self.udr0
        }
    }
    /**UBRR0 (rw) register accessor: USART Baud Rate Register Bytes

You can [`read`](crate::Reg::read) this register and get [`ubrr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubrr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ubrr0`] module*/
    pub type UBRR0 = crate::Reg<ubrr0::UBRR0_SPEC>;
    ///USART Baud Rate Register Bytes
    pub mod ubrr0 {
        ///Register `UBRR0` reader
        pub type R = crate::R<UBRR0_SPEC>;
        ///Register `UBRR0` writer
        pub type W = crate::W<UBRR0_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**USART Baud Rate Register Bytes

You can [`read`](crate::Reg::read) this register and get [`ubrr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubrr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UBRR0_SPEC;
        impl crate::RegisterSpec for UBRR0_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ubrr0::R`](R) reader structure
        impl crate::Readable for UBRR0_SPEC {}
        ///`write(|w| ..)` method takes [`ubrr0::W`](W) writer structure
        impl crate::Writable for UBRR0_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets UBRR0 to value 0
        impl crate::Resettable for UBRR0_SPEC {}
    }
    /**UCSR0A (rw) register accessor: USART Control and Status Register A

You can [`read`](crate::Reg::read) this register and get [`ucsr0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr0a`] module*/
    pub type UCSR0A = crate::Reg<ucsr0a::UCSR0A_SPEC>;
    ///USART Control and Status Register A
    pub mod ucsr0a {
        ///Register `UCSR0A` reader
        pub type R = crate::R<UCSR0A_SPEC>;
        ///Register `UCSR0A` writer
        pub type W = crate::W<UCSR0A_SPEC>;
        ///Field `MPCM0` reader - Multi-processor Communication Mode
        pub type MPCM0_R = crate::BitReader;
        ///Field `MPCM0` writer - Multi-processor Communication Mode
        pub type MPCM0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `U2X0` reader - Double the USART transmission speed
        pub type U2X0_R = crate::BitReader;
        ///Field `U2X0` writer - Double the USART transmission speed
        pub type U2X0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `UPE0` reader - Parity Error
        pub type UPE0_R = crate::BitReader;
        ///Field `DOR0` reader - Data overRun
        pub type DOR0_R = crate::BitReader;
        ///Field `FE0` reader - Framing Error
        pub type FE0_R = crate::BitReader;
        ///Field `UDRE0` reader - USART Data Register Empty
        pub type UDRE0_R = crate::BitReader;
        ///Field `TXC0` reader - USART Transmit Complete
        pub type TXC0_R = crate::BitReader;
        ///Field `TXC0` writer - USART Transmit Complete
        pub type TXC0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXC0` reader - USART Receive Complete
        pub type RXC0_R = crate::BitReader;
        impl R {
            ///Bit 0 - Multi-processor Communication Mode
            #[inline(always)]
            pub fn mpcm0(&self) -> MPCM0_R {
                MPCM0_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Double the USART transmission speed
            #[inline(always)]
            pub fn u2x0(&self) -> U2X0_R {
                U2X0_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Parity Error
            #[inline(always)]
            pub fn upe0(&self) -> UPE0_R {
                UPE0_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Data overRun
            #[inline(always)]
            pub fn dor0(&self) -> DOR0_R {
                DOR0_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Framing Error
            #[inline(always)]
            pub fn fe0(&self) -> FE0_R {
                FE0_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - USART Data Register Empty
            #[inline(always)]
            pub fn udre0(&self) -> UDRE0_R {
                UDRE0_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - USART Transmit Complete
            #[inline(always)]
            pub fn txc0(&self) -> TXC0_R {
                TXC0_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - USART Receive Complete
            #[inline(always)]
            pub fn rxc0(&self) -> RXC0_R {
                RXC0_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Multi-processor Communication Mode
            #[inline(always)]
            pub fn mpcm0(&mut self) -> MPCM0_W<'_, UCSR0A_SPEC> {
                MPCM0_W::new(self, 0)
            }
            ///Bit 1 - Double the USART transmission speed
            #[inline(always)]
            pub fn u2x0(&mut self) -> U2X0_W<'_, UCSR0A_SPEC> {
                U2X0_W::new(self, 1)
            }
            ///Bit 6 - USART Transmit Complete
            #[inline(always)]
            pub fn txc0(&mut self) -> TXC0_W<'_, UCSR0A_SPEC> {
                TXC0_W::new(self, 6)
            }
        }
        /**USART Control and Status Register A

You can [`read`](crate::Reg::read) this register and get [`ucsr0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR0A_SPEC;
        impl crate::RegisterSpec for UCSR0A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr0a::R`](R) reader structure
        impl crate::Readable for UCSR0A_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr0a::W`](W) writer structure
        impl crate::Writable for UCSR0A_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR0A to value 0
        impl crate::Resettable for UCSR0A_SPEC {}
    }
    /**UCSR0B (rw) register accessor: USART Control and Status Register B

You can [`read`](crate::Reg::read) this register and get [`ucsr0b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr0b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr0b`] module*/
    pub type UCSR0B = crate::Reg<ucsr0b::UCSR0B_SPEC>;
    ///USART Control and Status Register B
    pub mod ucsr0b {
        ///Register `UCSR0B` reader
        pub type R = crate::R<UCSR0B_SPEC>;
        ///Register `UCSR0B` writer
        pub type W = crate::W<UCSR0B_SPEC>;
        ///Field `TXB80` reader - Transmit Data Bit 8
        pub type TXB80_R = crate::BitReader;
        ///Field `TXB80` writer - Transmit Data Bit 8
        pub type TXB80_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXB80` reader - Receive Data Bit 8
        pub type RXB80_R = crate::BitReader;
        ///Field `UCSZ02` reader - Character Size
        pub type UCSZ02_R = crate::BitReader;
        ///Field `UCSZ02` writer - Character Size
        pub type UCSZ02_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TXEN0` reader - Transmitter Enable
        pub type TXEN0_R = crate::BitReader;
        ///Field `TXEN0` writer - Transmitter Enable
        pub type TXEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXEN0` reader - Receiver Enable
        pub type RXEN0_R = crate::BitReader;
        ///Field `RXEN0` writer - Receiver Enable
        pub type RXEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `UDRIE0` reader - USART Data register Empty Interrupt Enable
        pub type UDRIE0_R = crate::BitReader;
        ///Field `UDRIE0` writer - USART Data register Empty Interrupt Enable
        pub type UDRIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TXCIE0` reader - TX Complete Interrupt Enable
        pub type TXCIE0_R = crate::BitReader;
        ///Field `TXCIE0` writer - TX Complete Interrupt Enable
        pub type TXCIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXCIE0` reader - RX Complete Interrupt Enable
        pub type RXCIE0_R = crate::BitReader;
        ///Field `RXCIE0` writer - RX Complete Interrupt Enable
        pub type RXCIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Transmit Data Bit 8
            #[inline(always)]
            pub fn txb80(&self) -> TXB80_R {
                TXB80_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Receive Data Bit 8
            #[inline(always)]
            pub fn rxb80(&self) -> RXB80_R {
                RXB80_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Character Size
            #[inline(always)]
            pub fn ucsz02(&self) -> UCSZ02_R {
                UCSZ02_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Transmitter Enable
            #[inline(always)]
            pub fn txen0(&self) -> TXEN0_R {
                TXEN0_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Receiver Enable
            #[inline(always)]
            pub fn rxen0(&self) -> RXEN0_R {
                RXEN0_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - USART Data register Empty Interrupt Enable
            #[inline(always)]
            pub fn udrie0(&self) -> UDRIE0_R {
                UDRIE0_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - TX Complete Interrupt Enable
            #[inline(always)]
            pub fn txcie0(&self) -> TXCIE0_R {
                TXCIE0_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - RX Complete Interrupt Enable
            #[inline(always)]
            pub fn rxcie0(&self) -> RXCIE0_R {
                RXCIE0_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Transmit Data Bit 8
            #[inline(always)]
            pub fn txb80(&mut self) -> TXB80_W<'_, UCSR0B_SPEC> {
                TXB80_W::new(self, 0)
            }
            ///Bit 2 - Character Size
            #[inline(always)]
            pub fn ucsz02(&mut self) -> UCSZ02_W<'_, UCSR0B_SPEC> {
                UCSZ02_W::new(self, 2)
            }
            ///Bit 3 - Transmitter Enable
            #[inline(always)]
            pub fn txen0(&mut self) -> TXEN0_W<'_, UCSR0B_SPEC> {
                TXEN0_W::new(self, 3)
            }
            ///Bit 4 - Receiver Enable
            #[inline(always)]
            pub fn rxen0(&mut self) -> RXEN0_W<'_, UCSR0B_SPEC> {
                RXEN0_W::new(self, 4)
            }
            ///Bit 5 - USART Data register Empty Interrupt Enable
            #[inline(always)]
            pub fn udrie0(&mut self) -> UDRIE0_W<'_, UCSR0B_SPEC> {
                UDRIE0_W::new(self, 5)
            }
            ///Bit 6 - TX Complete Interrupt Enable
            #[inline(always)]
            pub fn txcie0(&mut self) -> TXCIE0_W<'_, UCSR0B_SPEC> {
                TXCIE0_W::new(self, 6)
            }
            ///Bit 7 - RX Complete Interrupt Enable
            #[inline(always)]
            pub fn rxcie0(&mut self) -> RXCIE0_W<'_, UCSR0B_SPEC> {
                RXCIE0_W::new(self, 7)
            }
        }
        /**USART Control and Status Register B

You can [`read`](crate::Reg::read) this register and get [`ucsr0b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr0b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR0B_SPEC;
        impl crate::RegisterSpec for UCSR0B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr0b::R`](R) reader structure
        impl crate::Readable for UCSR0B_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr0b::W`](W) writer structure
        impl crate::Writable for UCSR0B_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR0B to value 0
        impl crate::Resettable for UCSR0B_SPEC {}
    }
    /**UCSR0C (rw) register accessor: USART Control and Status Register C

You can [`read`](crate::Reg::read) this register and get [`ucsr0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr0c`] module*/
    pub type UCSR0C = crate::Reg<ucsr0c::UCSR0C_SPEC>;
    ///USART Control and Status Register C
    pub mod ucsr0c {
        ///Register `UCSR0C` reader
        pub type R = crate::R<UCSR0C_SPEC>;
        ///Register `UCSR0C` writer
        pub type W = crate::W<UCSR0C_SPEC>;
        /**Clock Polarity

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum UCPOL0_A {
            ///0: Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            RISING_EDGE = 0,
            ///1: Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            FALLING_EDGE = 1,
        }
        impl From<UCPOL0_A> for bool {
            #[inline(always)]
            fn from(variant: UCPOL0_A) -> Self {
                variant as u8 != 0
            }
        }
        ///Field `UCPOL0` reader - Clock Polarity
        pub type UCPOL0_R = crate::BitReader<UCPOL0_A>;
        impl UCPOL0_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> UCPOL0_A {
                match self.bits {
                    false => UCPOL0_A::RISING_EDGE,
                    true => UCPOL0_A::FALLING_EDGE,
                }
            }
            ///Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            #[inline(always)]
            pub fn is_rising_edge(&self) -> bool {
                *self == UCPOL0_A::RISING_EDGE
            }
            ///Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            #[inline(always)]
            pub fn is_falling_edge(&self) -> bool {
                *self == UCPOL0_A::FALLING_EDGE
            }
        }
        ///Field `UCPOL0` writer - Clock Polarity
        pub type UCPOL0_W<'a, REG> = crate::BitWriter<'a, REG, UCPOL0_A>;
        impl<'a, REG> UCPOL0_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
        {
            ///Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            #[inline(always)]
            pub fn rising_edge(self) -> &'a mut crate::W<REG> {
                self.variant(UCPOL0_A::RISING_EDGE)
            }
            ///Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            #[inline(always)]
            pub fn falling_edge(self) -> &'a mut crate::W<REG> {
                self.variant(UCPOL0_A::FALLING_EDGE)
            }
        }
        /**Character Size

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UCSZ0_A {
            ///0: Character Size: 5 bit
            CHR5 = 0,
            ///1: Character Size: 6 bit
            CHR6 = 1,
            ///2: Character Size: 7 bit
            CHR7 = 2,
            ///3: Character Size: 8 bit
            CHR8 = 3,
        }
        impl From<UCSZ0_A> for u8 {
            #[inline(always)]
            fn from(variant: UCSZ0_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UCSZ0_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UCSZ0_A {}
        ///Field `UCSZ0` reader - Character Size
        pub type UCSZ0_R = crate::FieldReader<UCSZ0_A>;
        impl UCSZ0_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> UCSZ0_A {
                match self.bits {
                    0 => UCSZ0_A::CHR5,
                    1 => UCSZ0_A::CHR6,
                    2 => UCSZ0_A::CHR7,
                    3 => UCSZ0_A::CHR8,
                    _ => unreachable!(),
                }
            }
            ///Character Size: 5 bit
            #[inline(always)]
            pub fn is_chr5(&self) -> bool {
                *self == UCSZ0_A::CHR5
            }
            ///Character Size: 6 bit
            #[inline(always)]
            pub fn is_chr6(&self) -> bool {
                *self == UCSZ0_A::CHR6
            }
            ///Character Size: 7 bit
            #[inline(always)]
            pub fn is_chr7(&self) -> bool {
                *self == UCSZ0_A::CHR7
            }
            ///Character Size: 8 bit
            #[inline(always)]
            pub fn is_chr8(&self) -> bool {
                *self == UCSZ0_A::CHR8
            }
        }
        ///Field `UCSZ0` writer - Character Size
        pub type UCSZ0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UCSZ0_A, crate::Safe>;
        impl<'a, REG> UCSZ0_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Character Size: 5 bit
            #[inline(always)]
            pub fn chr5(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ0_A::CHR5)
            }
            ///Character Size: 6 bit
            #[inline(always)]
            pub fn chr6(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ0_A::CHR6)
            }
            ///Character Size: 7 bit
            #[inline(always)]
            pub fn chr7(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ0_A::CHR7)
            }
            ///Character Size: 8 bit
            #[inline(always)]
            pub fn chr8(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ0_A::CHR8)
            }
        }
        /**Stop Bit Select

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum USBS0_A {
            ///0: 1-bit
            STOP1 = 0,
            ///1: 2-bit
            STOP2 = 1,
        }
        impl From<USBS0_A> for bool {
            #[inline(always)]
            fn from(variant: USBS0_A) -> Self {
                variant as u8 != 0
            }
        }
        ///Field `USBS0` reader - Stop Bit Select
        pub type USBS0_R = crate::BitReader<USBS0_A>;
        impl USBS0_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> USBS0_A {
                match self.bits {
                    false => USBS0_A::STOP1,
                    true => USBS0_A::STOP2,
                }
            }
            ///1-bit
            #[inline(always)]
            pub fn is_stop1(&self) -> bool {
                *self == USBS0_A::STOP1
            }
            ///2-bit
            #[inline(always)]
            pub fn is_stop2(&self) -> bool {
                *self == USBS0_A::STOP2
            }
        }
        ///Field `USBS0` writer - Stop Bit Select
        pub type USBS0_W<'a, REG> = crate::BitWriter<'a, REG, USBS0_A>;
        impl<'a, REG> USBS0_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
        {
            ///1-bit
            #[inline(always)]
            pub fn stop1(self) -> &'a mut crate::W<REG> {
                self.variant(USBS0_A::STOP1)
            }
            ///2-bit
            #[inline(always)]
            pub fn stop2(self) -> &'a mut crate::W<REG> {
                self.variant(USBS0_A::STOP2)
            }
        }
        /**Parity Mode Bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UPM0_A {
            ///0: Disabled
            DISABLED = 0,
            ///2: Enabled, Even Parity
            PARITY_EVEN = 2,
            ///3: Enabled, Odd Parity
            PARITY_ODD = 3,
        }
        impl From<UPM0_A> for u8 {
            #[inline(always)]
            fn from(variant: UPM0_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UPM0_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UPM0_A {}
        ///Field `UPM0` reader - Parity Mode Bits
        pub type UPM0_R = crate::FieldReader<UPM0_A>;
        impl UPM0_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<UPM0_A> {
                match self.bits {
                    0 => Some(UPM0_A::DISABLED),
                    2 => Some(UPM0_A::PARITY_EVEN),
                    3 => Some(UPM0_A::PARITY_ODD),
                    _ => None,
                }
            }
            ///Disabled
            #[inline(always)]
            pub fn is_disabled(&self) -> bool {
                *self == UPM0_A::DISABLED
            }
            ///Enabled, Even Parity
            #[inline(always)]
            pub fn is_parity_even(&self) -> bool {
                *self == UPM0_A::PARITY_EVEN
            }
            ///Enabled, Odd Parity
            #[inline(always)]
            pub fn is_parity_odd(&self) -> bool {
                *self == UPM0_A::PARITY_ODD
            }
        }
        ///Field `UPM0` writer - Parity Mode Bits
        pub type UPM0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UPM0_A>;
        impl<'a, REG> UPM0_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Disabled
            #[inline(always)]
            pub fn disabled(self) -> &'a mut crate::W<REG> {
                self.variant(UPM0_A::DISABLED)
            }
            ///Enabled, Even Parity
            #[inline(always)]
            pub fn parity_even(self) -> &'a mut crate::W<REG> {
                self.variant(UPM0_A::PARITY_EVEN)
            }
            ///Enabled, Odd Parity
            #[inline(always)]
            pub fn parity_odd(self) -> &'a mut crate::W<REG> {
                self.variant(UPM0_A::PARITY_ODD)
            }
        }
        /**USART Mode Select

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UMSEL0_A {
            ///0: Asynchronous USART
            USART_ASYNC = 0,
            ///1: Synchronous USART
            USART_SYNC = 1,
            ///3: Master SPI (MSPIM)
            SPI_MASTER = 3,
        }
        impl From<UMSEL0_A> for u8 {
            #[inline(always)]
            fn from(variant: UMSEL0_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UMSEL0_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UMSEL0_A {}
        ///Field `UMSEL0` reader - USART Mode Select
        pub type UMSEL0_R = crate::FieldReader<UMSEL0_A>;
        impl UMSEL0_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<UMSEL0_A> {
                match self.bits {
                    0 => Some(UMSEL0_A::USART_ASYNC),
                    1 => Some(UMSEL0_A::USART_SYNC),
                    3 => Some(UMSEL0_A::SPI_MASTER),
                    _ => None,
                }
            }
            ///Asynchronous USART
            #[inline(always)]
            pub fn is_usart_async(&self) -> bool {
                *self == UMSEL0_A::USART_ASYNC
            }
            ///Synchronous USART
            #[inline(always)]
            pub fn is_usart_sync(&self) -> bool {
                *self == UMSEL0_A::USART_SYNC
            }
            ///Master SPI (MSPIM)
            #[inline(always)]
            pub fn is_spi_master(&self) -> bool {
                *self == UMSEL0_A::SPI_MASTER
            }
        }
        ///Field `UMSEL0` writer - USART Mode Select
        pub type UMSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UMSEL0_A>;
        impl<'a, REG> UMSEL0_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Asynchronous USART
            #[inline(always)]
            pub fn usart_async(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL0_A::USART_ASYNC)
            }
            ///Synchronous USART
            #[inline(always)]
            pub fn usart_sync(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL0_A::USART_SYNC)
            }
            ///Master SPI (MSPIM)
            #[inline(always)]
            pub fn spi_master(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL0_A::SPI_MASTER)
            }
        }
        impl R {
            ///Bit 0 - Clock Polarity
            #[inline(always)]
            pub fn ucpol0(&self) -> UCPOL0_R {
                UCPOL0_R::new((self.bits & 1) != 0)
            }
            ///Bits 1:2 - Character Size
            #[inline(always)]
            pub fn ucsz0(&self) -> UCSZ0_R {
                UCSZ0_R::new((self.bits >> 1) & 3)
            }
            ///Bit 3 - Stop Bit Select
            #[inline(always)]
            pub fn usbs0(&self) -> USBS0_R {
                USBS0_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bits 4:5 - Parity Mode Bits
            #[inline(always)]
            pub fn upm0(&self) -> UPM0_R {
                UPM0_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - USART Mode Select
            #[inline(always)]
            pub fn umsel0(&self) -> UMSEL0_R {
                UMSEL0_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bit 0 - Clock Polarity
            #[inline(always)]
            pub fn ucpol0(&mut self) -> UCPOL0_W<'_, UCSR0C_SPEC> {
                UCPOL0_W::new(self, 0)
            }
            ///Bits 1:2 - Character Size
            #[inline(always)]
            pub fn ucsz0(&mut self) -> UCSZ0_W<'_, UCSR0C_SPEC> {
                UCSZ0_W::new(self, 1)
            }
            ///Bit 3 - Stop Bit Select
            #[inline(always)]
            pub fn usbs0(&mut self) -> USBS0_W<'_, UCSR0C_SPEC> {
                USBS0_W::new(self, 3)
            }
            ///Bits 4:5 - Parity Mode Bits
            #[inline(always)]
            pub fn upm0(&mut self) -> UPM0_W<'_, UCSR0C_SPEC> {
                UPM0_W::new(self, 4)
            }
            ///Bits 6:7 - USART Mode Select
            #[inline(always)]
            pub fn umsel0(&mut self) -> UMSEL0_W<'_, UCSR0C_SPEC> {
                UMSEL0_W::new(self, 6)
            }
        }
        /**USART Control and Status Register C

You can [`read`](crate::Reg::read) this register and get [`ucsr0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR0C_SPEC;
        impl crate::RegisterSpec for UCSR0C_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr0c::R`](R) reader structure
        impl crate::Readable for UCSR0C_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr0c::W`](W) writer structure
        impl crate::Writable for UCSR0C_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR0C to value 0
        impl crate::Resettable for UCSR0C_SPEC {}
    }
    /**UDR0 (rw) register accessor: USART I/O Data Register

You can [`read`](crate::Reg::read) this register and get [`udr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@udr0`] module*/
    pub type UDR0 = crate::Reg<udr0::UDR0_SPEC>;
    ///USART I/O Data Register
    pub mod udr0 {
        ///Register `UDR0` reader
        pub type R = crate::R<UDR0_SPEC>;
        ///Register `UDR0` writer
        pub type W = crate::W<UDR0_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**USART I/O Data Register

You can [`read`](crate::Reg::read) this register and get [`udr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UDR0_SPEC;
        impl crate::RegisterSpec for UDR0_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`udr0::R`](R) reader structure
        impl crate::Readable for UDR0_SPEC {}
        ///`write(|w| ..)` method takes [`udr0::W`](W) writer structure
        impl crate::Writable for UDR0_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets UDR0 to value 0
        impl crate::Resettable for UDR0_SPEC {}
    }
}
///USART
pub type USART1 = crate::Periph<usart1::RegisterBlock, 0xc8>;
impl core::fmt::Debug for USART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART1").finish()
    }
}
///USART
pub mod usart1 {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        ucsr1a: UCSR1A,
        ucsr1b: UCSR1B,
        ucsr1c: UCSR1C,
        _reserved3: [u8; 0x01],
        ubrr1: UBRR1,
        udr1: UDR1,
    }
    impl RegisterBlock {
        ///0x00 - USART Control and Status Register A
        #[inline(always)]
        pub const fn ucsr1a(&self) -> &UCSR1A {
            &self.ucsr1a
        }
        ///0x01 - USART Control and Status Register B
        #[inline(always)]
        pub const fn ucsr1b(&self) -> &UCSR1B {
            &self.ucsr1b
        }
        ///0x02 - USART Control and Status Register C
        #[inline(always)]
        pub const fn ucsr1c(&self) -> &UCSR1C {
            &self.ucsr1c
        }
        ///0x04 - USART Baud Rate Register Bytes
        #[inline(always)]
        pub const fn ubrr1(&self) -> &UBRR1 {
            &self.ubrr1
        }
        ///0x06 - USART I/O Data Register
        #[inline(always)]
        pub const fn udr1(&self) -> &UDR1 {
            &self.udr1
        }
    }
    /**UBRR1 (rw) register accessor: USART Baud Rate Register Bytes

You can [`read`](crate::Reg::read) this register and get [`ubrr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubrr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ubrr1`] module*/
    pub type UBRR1 = crate::Reg<ubrr1::UBRR1_SPEC>;
    ///USART Baud Rate Register Bytes
    pub mod ubrr1 {
        ///Register `UBRR1` reader
        pub type R = crate::R<UBRR1_SPEC>;
        ///Register `UBRR1` writer
        pub type W = crate::W<UBRR1_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**USART Baud Rate Register Bytes

You can [`read`](crate::Reg::read) this register and get [`ubrr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubrr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UBRR1_SPEC;
        impl crate::RegisterSpec for UBRR1_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ubrr1::R`](R) reader structure
        impl crate::Readable for UBRR1_SPEC {}
        ///`write(|w| ..)` method takes [`ubrr1::W`](W) writer structure
        impl crate::Writable for UBRR1_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets UBRR1 to value 0
        impl crate::Resettable for UBRR1_SPEC {}
    }
    /**UCSR1A (rw) register accessor: USART Control and Status Register A

You can [`read`](crate::Reg::read) this register and get [`ucsr1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr1a`] module*/
    pub type UCSR1A = crate::Reg<ucsr1a::UCSR1A_SPEC>;
    ///USART Control and Status Register A
    pub mod ucsr1a {
        ///Register `UCSR1A` reader
        pub type R = crate::R<UCSR1A_SPEC>;
        ///Register `UCSR1A` writer
        pub type W = crate::W<UCSR1A_SPEC>;
        ///Field `MPCM1` reader - Multi-processor Communication Mode
        pub type MPCM1_R = crate::BitReader;
        ///Field `MPCM1` writer - Multi-processor Communication Mode
        pub type MPCM1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `U2X1` reader - Double the USART transmission speed
        pub type U2X1_R = crate::BitReader;
        ///Field `U2X1` writer - Double the USART transmission speed
        pub type U2X1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `UPE1` reader - Parity Error
        pub type UPE1_R = crate::BitReader;
        ///Field `DOR1` reader - Data overRun
        pub type DOR1_R = crate::BitReader;
        ///Field `FE1` reader - Framing Error
        pub type FE1_R = crate::BitReader;
        ///Field `UDRE1` reader - USART Data Register Empty
        pub type UDRE1_R = crate::BitReader;
        ///Field `TXC1` reader - USART Transmit Complete
        pub type TXC1_R = crate::BitReader;
        ///Field `TXC1` writer - USART Transmit Complete
        pub type TXC1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXC1` reader - USART Receive Complete
        pub type RXC1_R = crate::BitReader;
        impl R {
            ///Bit 0 - Multi-processor Communication Mode
            #[inline(always)]
            pub fn mpcm1(&self) -> MPCM1_R {
                MPCM1_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Double the USART transmission speed
            #[inline(always)]
            pub fn u2x1(&self) -> U2X1_R {
                U2X1_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Parity Error
            #[inline(always)]
            pub fn upe1(&self) -> UPE1_R {
                UPE1_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Data overRun
            #[inline(always)]
            pub fn dor1(&self) -> DOR1_R {
                DOR1_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Framing Error
            #[inline(always)]
            pub fn fe1(&self) -> FE1_R {
                FE1_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - USART Data Register Empty
            #[inline(always)]
            pub fn udre1(&self) -> UDRE1_R {
                UDRE1_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - USART Transmit Complete
            #[inline(always)]
            pub fn txc1(&self) -> TXC1_R {
                TXC1_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - USART Receive Complete
            #[inline(always)]
            pub fn rxc1(&self) -> RXC1_R {
                RXC1_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Multi-processor Communication Mode
            #[inline(always)]
            pub fn mpcm1(&mut self) -> MPCM1_W<'_, UCSR1A_SPEC> {
                MPCM1_W::new(self, 0)
            }
            ///Bit 1 - Double the USART transmission speed
            #[inline(always)]
            pub fn u2x1(&mut self) -> U2X1_W<'_, UCSR1A_SPEC> {
                U2X1_W::new(self, 1)
            }
            ///Bit 6 - USART Transmit Complete
            #[inline(always)]
            pub fn txc1(&mut self) -> TXC1_W<'_, UCSR1A_SPEC> {
                TXC1_W::new(self, 6)
            }
        }
        /**USART Control and Status Register A

You can [`read`](crate::Reg::read) this register and get [`ucsr1a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr1a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR1A_SPEC;
        impl crate::RegisterSpec for UCSR1A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr1a::R`](R) reader structure
        impl crate::Readable for UCSR1A_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr1a::W`](W) writer structure
        impl crate::Writable for UCSR1A_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR1A to value 0
        impl crate::Resettable for UCSR1A_SPEC {}
    }
    /**UCSR1B (rw) register accessor: USART Control and Status Register B

You can [`read`](crate::Reg::read) this register and get [`ucsr1b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr1b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr1b`] module*/
    pub type UCSR1B = crate::Reg<ucsr1b::UCSR1B_SPEC>;
    ///USART Control and Status Register B
    pub mod ucsr1b {
        ///Register `UCSR1B` reader
        pub type R = crate::R<UCSR1B_SPEC>;
        ///Register `UCSR1B` writer
        pub type W = crate::W<UCSR1B_SPEC>;
        ///Field `TXB81` reader - Transmit Data Bit 8
        pub type TXB81_R = crate::BitReader;
        ///Field `TXB81` writer - Transmit Data Bit 8
        pub type TXB81_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXB81` reader - Receive Data Bit 8
        pub type RXB81_R = crate::BitReader;
        ///Field `UCSZ12` reader - Character Size
        pub type UCSZ12_R = crate::BitReader;
        ///Field `UCSZ12` writer - Character Size
        pub type UCSZ12_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TXEN1` reader - Transmitter Enable
        pub type TXEN1_R = crate::BitReader;
        ///Field `TXEN1` writer - Transmitter Enable
        pub type TXEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXEN1` reader - Receiver Enable
        pub type RXEN1_R = crate::BitReader;
        ///Field `RXEN1` writer - Receiver Enable
        pub type RXEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `UDRIE1` reader - USART Data register Empty Interrupt Enable
        pub type UDRIE1_R = crate::BitReader;
        ///Field `UDRIE1` writer - USART Data register Empty Interrupt Enable
        pub type UDRIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TXCIE1` reader - TX Complete Interrupt Enable
        pub type TXCIE1_R = crate::BitReader;
        ///Field `TXCIE1` writer - TX Complete Interrupt Enable
        pub type TXCIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXCIE1` reader - RX Complete Interrupt Enable
        pub type RXCIE1_R = crate::BitReader;
        ///Field `RXCIE1` writer - RX Complete Interrupt Enable
        pub type RXCIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Transmit Data Bit 8
            #[inline(always)]
            pub fn txb81(&self) -> TXB81_R {
                TXB81_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Receive Data Bit 8
            #[inline(always)]
            pub fn rxb81(&self) -> RXB81_R {
                RXB81_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Character Size
            #[inline(always)]
            pub fn ucsz12(&self) -> UCSZ12_R {
                UCSZ12_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Transmitter Enable
            #[inline(always)]
            pub fn txen1(&self) -> TXEN1_R {
                TXEN1_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Receiver Enable
            #[inline(always)]
            pub fn rxen1(&self) -> RXEN1_R {
                RXEN1_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - USART Data register Empty Interrupt Enable
            #[inline(always)]
            pub fn udrie1(&self) -> UDRIE1_R {
                UDRIE1_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - TX Complete Interrupt Enable
            #[inline(always)]
            pub fn txcie1(&self) -> TXCIE1_R {
                TXCIE1_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - RX Complete Interrupt Enable
            #[inline(always)]
            pub fn rxcie1(&self) -> RXCIE1_R {
                RXCIE1_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Transmit Data Bit 8
            #[inline(always)]
            pub fn txb81(&mut self) -> TXB81_W<'_, UCSR1B_SPEC> {
                TXB81_W::new(self, 0)
            }
            ///Bit 2 - Character Size
            #[inline(always)]
            pub fn ucsz12(&mut self) -> UCSZ12_W<'_, UCSR1B_SPEC> {
                UCSZ12_W::new(self, 2)
            }
            ///Bit 3 - Transmitter Enable
            #[inline(always)]
            pub fn txen1(&mut self) -> TXEN1_W<'_, UCSR1B_SPEC> {
                TXEN1_W::new(self, 3)
            }
            ///Bit 4 - Receiver Enable
            #[inline(always)]
            pub fn rxen1(&mut self) -> RXEN1_W<'_, UCSR1B_SPEC> {
                RXEN1_W::new(self, 4)
            }
            ///Bit 5 - USART Data register Empty Interrupt Enable
            #[inline(always)]
            pub fn udrie1(&mut self) -> UDRIE1_W<'_, UCSR1B_SPEC> {
                UDRIE1_W::new(self, 5)
            }
            ///Bit 6 - TX Complete Interrupt Enable
            #[inline(always)]
            pub fn txcie1(&mut self) -> TXCIE1_W<'_, UCSR1B_SPEC> {
                TXCIE1_W::new(self, 6)
            }
            ///Bit 7 - RX Complete Interrupt Enable
            #[inline(always)]
            pub fn rxcie1(&mut self) -> RXCIE1_W<'_, UCSR1B_SPEC> {
                RXCIE1_W::new(self, 7)
            }
        }
        /**USART Control and Status Register B

You can [`read`](crate::Reg::read) this register and get [`ucsr1b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr1b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR1B_SPEC;
        impl crate::RegisterSpec for UCSR1B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr1b::R`](R) reader structure
        impl crate::Readable for UCSR1B_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr1b::W`](W) writer structure
        impl crate::Writable for UCSR1B_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR1B to value 0
        impl crate::Resettable for UCSR1B_SPEC {}
    }
    /**UCSR1C (rw) register accessor: USART Control and Status Register C

You can [`read`](crate::Reg::read) this register and get [`ucsr1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr1c`] module*/
    pub type UCSR1C = crate::Reg<ucsr1c::UCSR1C_SPEC>;
    ///USART Control and Status Register C
    pub mod ucsr1c {
        ///Register `UCSR1C` reader
        pub type R = crate::R<UCSR1C_SPEC>;
        ///Register `UCSR1C` writer
        pub type W = crate::W<UCSR1C_SPEC>;
        /**Clock Polarity

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum UCPOL1_A {
            ///0: Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            RISING_EDGE = 0,
            ///1: Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            FALLING_EDGE = 1,
        }
        impl From<UCPOL1_A> for bool {
            #[inline(always)]
            fn from(variant: UCPOL1_A) -> Self {
                variant as u8 != 0
            }
        }
        ///Field `UCPOL1` reader - Clock Polarity
        pub type UCPOL1_R = crate::BitReader<UCPOL1_A>;
        impl UCPOL1_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> UCPOL1_A {
                match self.bits {
                    false => UCPOL1_A::RISING_EDGE,
                    true => UCPOL1_A::FALLING_EDGE,
                }
            }
            ///Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            #[inline(always)]
            pub fn is_rising_edge(&self) -> bool {
                *self == UCPOL1_A::RISING_EDGE
            }
            ///Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            #[inline(always)]
            pub fn is_falling_edge(&self) -> bool {
                *self == UCPOL1_A::FALLING_EDGE
            }
        }
        ///Field `UCPOL1` writer - Clock Polarity
        pub type UCPOL1_W<'a, REG> = crate::BitWriter<'a, REG, UCPOL1_A>;
        impl<'a, REG> UCPOL1_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
        {
            ///Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            #[inline(always)]
            pub fn rising_edge(self) -> &'a mut crate::W<REG> {
                self.variant(UCPOL1_A::RISING_EDGE)
            }
            ///Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            #[inline(always)]
            pub fn falling_edge(self) -> &'a mut crate::W<REG> {
                self.variant(UCPOL1_A::FALLING_EDGE)
            }
        }
        /**Character Size

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UCSZ1_A {
            ///0: Character Size: 5 bit
            CHR5 = 0,
            ///1: Character Size: 6 bit
            CHR6 = 1,
            ///2: Character Size: 7 bit
            CHR7 = 2,
            ///3: Character Size: 8 bit
            CHR8 = 3,
        }
        impl From<UCSZ1_A> for u8 {
            #[inline(always)]
            fn from(variant: UCSZ1_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UCSZ1_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UCSZ1_A {}
        ///Field `UCSZ1` reader - Character Size
        pub type UCSZ1_R = crate::FieldReader<UCSZ1_A>;
        impl UCSZ1_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> UCSZ1_A {
                match self.bits {
                    0 => UCSZ1_A::CHR5,
                    1 => UCSZ1_A::CHR6,
                    2 => UCSZ1_A::CHR7,
                    3 => UCSZ1_A::CHR8,
                    _ => unreachable!(),
                }
            }
            ///Character Size: 5 bit
            #[inline(always)]
            pub fn is_chr5(&self) -> bool {
                *self == UCSZ1_A::CHR5
            }
            ///Character Size: 6 bit
            #[inline(always)]
            pub fn is_chr6(&self) -> bool {
                *self == UCSZ1_A::CHR6
            }
            ///Character Size: 7 bit
            #[inline(always)]
            pub fn is_chr7(&self) -> bool {
                *self == UCSZ1_A::CHR7
            }
            ///Character Size: 8 bit
            #[inline(always)]
            pub fn is_chr8(&self) -> bool {
                *self == UCSZ1_A::CHR8
            }
        }
        ///Field `UCSZ1` writer - Character Size
        pub type UCSZ1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UCSZ1_A, crate::Safe>;
        impl<'a, REG> UCSZ1_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Character Size: 5 bit
            #[inline(always)]
            pub fn chr5(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ1_A::CHR5)
            }
            ///Character Size: 6 bit
            #[inline(always)]
            pub fn chr6(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ1_A::CHR6)
            }
            ///Character Size: 7 bit
            #[inline(always)]
            pub fn chr7(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ1_A::CHR7)
            }
            ///Character Size: 8 bit
            #[inline(always)]
            pub fn chr8(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ1_A::CHR8)
            }
        }
        /**Stop Bit Select

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum USBS1_A {
            ///0: 1-bit
            STOP1 = 0,
            ///1: 2-bit
            STOP2 = 1,
        }
        impl From<USBS1_A> for bool {
            #[inline(always)]
            fn from(variant: USBS1_A) -> Self {
                variant as u8 != 0
            }
        }
        ///Field `USBS1` reader - Stop Bit Select
        pub type USBS1_R = crate::BitReader<USBS1_A>;
        impl USBS1_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> USBS1_A {
                match self.bits {
                    false => USBS1_A::STOP1,
                    true => USBS1_A::STOP2,
                }
            }
            ///1-bit
            #[inline(always)]
            pub fn is_stop1(&self) -> bool {
                *self == USBS1_A::STOP1
            }
            ///2-bit
            #[inline(always)]
            pub fn is_stop2(&self) -> bool {
                *self == USBS1_A::STOP2
            }
        }
        ///Field `USBS1` writer - Stop Bit Select
        pub type USBS1_W<'a, REG> = crate::BitWriter<'a, REG, USBS1_A>;
        impl<'a, REG> USBS1_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
        {
            ///1-bit
            #[inline(always)]
            pub fn stop1(self) -> &'a mut crate::W<REG> {
                self.variant(USBS1_A::STOP1)
            }
            ///2-bit
            #[inline(always)]
            pub fn stop2(self) -> &'a mut crate::W<REG> {
                self.variant(USBS1_A::STOP2)
            }
        }
        /**Parity Mode Bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UPM1_A {
            ///0: Disabled
            DISABLED = 0,
            ///2: Enabled, Even Parity
            PARITY_EVEN = 2,
            ///3: Enabled, Odd Parity
            PARITY_ODD = 3,
        }
        impl From<UPM1_A> for u8 {
            #[inline(always)]
            fn from(variant: UPM1_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UPM1_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UPM1_A {}
        ///Field `UPM1` reader - Parity Mode Bits
        pub type UPM1_R = crate::FieldReader<UPM1_A>;
        impl UPM1_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<UPM1_A> {
                match self.bits {
                    0 => Some(UPM1_A::DISABLED),
                    2 => Some(UPM1_A::PARITY_EVEN),
                    3 => Some(UPM1_A::PARITY_ODD),
                    _ => None,
                }
            }
            ///Disabled
            #[inline(always)]
            pub fn is_disabled(&self) -> bool {
                *self == UPM1_A::DISABLED
            }
            ///Enabled, Even Parity
            #[inline(always)]
            pub fn is_parity_even(&self) -> bool {
                *self == UPM1_A::PARITY_EVEN
            }
            ///Enabled, Odd Parity
            #[inline(always)]
            pub fn is_parity_odd(&self) -> bool {
                *self == UPM1_A::PARITY_ODD
            }
        }
        ///Field `UPM1` writer - Parity Mode Bits
        pub type UPM1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UPM1_A>;
        impl<'a, REG> UPM1_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Disabled
            #[inline(always)]
            pub fn disabled(self) -> &'a mut crate::W<REG> {
                self.variant(UPM1_A::DISABLED)
            }
            ///Enabled, Even Parity
            #[inline(always)]
            pub fn parity_even(self) -> &'a mut crate::W<REG> {
                self.variant(UPM1_A::PARITY_EVEN)
            }
            ///Enabled, Odd Parity
            #[inline(always)]
            pub fn parity_odd(self) -> &'a mut crate::W<REG> {
                self.variant(UPM1_A::PARITY_ODD)
            }
        }
        /**USART Mode Select

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UMSEL1_A {
            ///0: Asynchronous USART
            USART_ASYNC = 0,
            ///1: Synchronous USART
            USART_SYNC = 1,
            ///3: Master SPI (MSPIM)
            SPI_MASTER = 3,
        }
        impl From<UMSEL1_A> for u8 {
            #[inline(always)]
            fn from(variant: UMSEL1_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UMSEL1_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UMSEL1_A {}
        ///Field `UMSEL1` reader - USART Mode Select
        pub type UMSEL1_R = crate::FieldReader<UMSEL1_A>;
        impl UMSEL1_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<UMSEL1_A> {
                match self.bits {
                    0 => Some(UMSEL1_A::USART_ASYNC),
                    1 => Some(UMSEL1_A::USART_SYNC),
                    3 => Some(UMSEL1_A::SPI_MASTER),
                    _ => None,
                }
            }
            ///Asynchronous USART
            #[inline(always)]
            pub fn is_usart_async(&self) -> bool {
                *self == UMSEL1_A::USART_ASYNC
            }
            ///Synchronous USART
            #[inline(always)]
            pub fn is_usart_sync(&self) -> bool {
                *self == UMSEL1_A::USART_SYNC
            }
            ///Master SPI (MSPIM)
            #[inline(always)]
            pub fn is_spi_master(&self) -> bool {
                *self == UMSEL1_A::SPI_MASTER
            }
        }
        ///Field `UMSEL1` writer - USART Mode Select
        pub type UMSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UMSEL1_A>;
        impl<'a, REG> UMSEL1_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Asynchronous USART
            #[inline(always)]
            pub fn usart_async(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL1_A::USART_ASYNC)
            }
            ///Synchronous USART
            #[inline(always)]
            pub fn usart_sync(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL1_A::USART_SYNC)
            }
            ///Master SPI (MSPIM)
            #[inline(always)]
            pub fn spi_master(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL1_A::SPI_MASTER)
            }
        }
        impl R {
            ///Bit 0 - Clock Polarity
            #[inline(always)]
            pub fn ucpol1(&self) -> UCPOL1_R {
                UCPOL1_R::new((self.bits & 1) != 0)
            }
            ///Bits 1:2 - Character Size
            #[inline(always)]
            pub fn ucsz1(&self) -> UCSZ1_R {
                UCSZ1_R::new((self.bits >> 1) & 3)
            }
            ///Bit 3 - Stop Bit Select
            #[inline(always)]
            pub fn usbs1(&self) -> USBS1_R {
                USBS1_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bits 4:5 - Parity Mode Bits
            #[inline(always)]
            pub fn upm1(&self) -> UPM1_R {
                UPM1_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - USART Mode Select
            #[inline(always)]
            pub fn umsel1(&self) -> UMSEL1_R {
                UMSEL1_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bit 0 - Clock Polarity
            #[inline(always)]
            pub fn ucpol1(&mut self) -> UCPOL1_W<'_, UCSR1C_SPEC> {
                UCPOL1_W::new(self, 0)
            }
            ///Bits 1:2 - Character Size
            #[inline(always)]
            pub fn ucsz1(&mut self) -> UCSZ1_W<'_, UCSR1C_SPEC> {
                UCSZ1_W::new(self, 1)
            }
            ///Bit 3 - Stop Bit Select
            #[inline(always)]
            pub fn usbs1(&mut self) -> USBS1_W<'_, UCSR1C_SPEC> {
                USBS1_W::new(self, 3)
            }
            ///Bits 4:5 - Parity Mode Bits
            #[inline(always)]
            pub fn upm1(&mut self) -> UPM1_W<'_, UCSR1C_SPEC> {
                UPM1_W::new(self, 4)
            }
            ///Bits 6:7 - USART Mode Select
            #[inline(always)]
            pub fn umsel1(&mut self) -> UMSEL1_W<'_, UCSR1C_SPEC> {
                UMSEL1_W::new(self, 6)
            }
        }
        /**USART Control and Status Register C

You can [`read`](crate::Reg::read) this register and get [`ucsr1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR1C_SPEC;
        impl crate::RegisterSpec for UCSR1C_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr1c::R`](R) reader structure
        impl crate::Readable for UCSR1C_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr1c::W`](W) writer structure
        impl crate::Writable for UCSR1C_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR1C to value 0
        impl crate::Resettable for UCSR1C_SPEC {}
    }
    /**UDR1 (rw) register accessor: USART I/O Data Register

You can [`read`](crate::Reg::read) this register and get [`udr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@udr1`] module*/
    pub type UDR1 = crate::Reg<udr1::UDR1_SPEC>;
    ///USART I/O Data Register
    pub mod udr1 {
        ///Register `UDR1` reader
        pub type R = crate::R<UDR1_SPEC>;
        ///Register `UDR1` writer
        pub type W = crate::W<UDR1_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**USART I/O Data Register

You can [`read`](crate::Reg::read) this register and get [`udr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UDR1_SPEC;
        impl crate::RegisterSpec for UDR1_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`udr1::R`](R) reader structure
        impl crate::Readable for UDR1_SPEC {}
        ///`write(|w| ..)` method takes [`udr1::W`](W) writer structure
        impl crate::Writable for UDR1_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets UDR1 to value 0
        impl crate::Resettable for UDR1_SPEC {}
    }
}
///USART
pub type USART2 = crate::Periph<usart2::RegisterBlock, 0xd0>;
impl core::fmt::Debug for USART2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART2").finish()
    }
}
///USART
pub mod usart2 {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        ucsr2a: UCSR2A,
        ucsr2b: UCSR2B,
        ucsr2c: UCSR2C,
        _reserved3: [u8; 0x01],
        ubrr2: UBRR2,
        udr2: UDR2,
    }
    impl RegisterBlock {
        ///0x00 - USART Control and Status Register A
        #[inline(always)]
        pub const fn ucsr2a(&self) -> &UCSR2A {
            &self.ucsr2a
        }
        ///0x01 - USART Control and Status Register B
        #[inline(always)]
        pub const fn ucsr2b(&self) -> &UCSR2B {
            &self.ucsr2b
        }
        ///0x02 - USART Control and Status Register C
        #[inline(always)]
        pub const fn ucsr2c(&self) -> &UCSR2C {
            &self.ucsr2c
        }
        ///0x04 - USART Baud Rate Register Bytes
        #[inline(always)]
        pub const fn ubrr2(&self) -> &UBRR2 {
            &self.ubrr2
        }
        ///0x06 - USART I/O Data Register
        #[inline(always)]
        pub const fn udr2(&self) -> &UDR2 {
            &self.udr2
        }
    }
    /**UBRR2 (rw) register accessor: USART Baud Rate Register Bytes

You can [`read`](crate::Reg::read) this register and get [`ubrr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubrr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ubrr2`] module*/
    pub type UBRR2 = crate::Reg<ubrr2::UBRR2_SPEC>;
    ///USART Baud Rate Register Bytes
    pub mod ubrr2 {
        ///Register `UBRR2` reader
        pub type R = crate::R<UBRR2_SPEC>;
        ///Register `UBRR2` writer
        pub type W = crate::W<UBRR2_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**USART Baud Rate Register Bytes

You can [`read`](crate::Reg::read) this register and get [`ubrr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubrr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UBRR2_SPEC;
        impl crate::RegisterSpec for UBRR2_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ubrr2::R`](R) reader structure
        impl crate::Readable for UBRR2_SPEC {}
        ///`write(|w| ..)` method takes [`ubrr2::W`](W) writer structure
        impl crate::Writable for UBRR2_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets UBRR2 to value 0
        impl crate::Resettable for UBRR2_SPEC {}
    }
    /**UCSR2A (rw) register accessor: USART Control and Status Register A

You can [`read`](crate::Reg::read) this register and get [`ucsr2a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr2a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr2a`] module*/
    pub type UCSR2A = crate::Reg<ucsr2a::UCSR2A_SPEC>;
    ///USART Control and Status Register A
    pub mod ucsr2a {
        ///Register `UCSR2A` reader
        pub type R = crate::R<UCSR2A_SPEC>;
        ///Register `UCSR2A` writer
        pub type W = crate::W<UCSR2A_SPEC>;
        ///Field `MPCM2` reader - Multi-processor Communication Mode
        pub type MPCM2_R = crate::BitReader;
        ///Field `MPCM2` writer - Multi-processor Communication Mode
        pub type MPCM2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `U2X2` reader - Double the USART transmission speed
        pub type U2X2_R = crate::BitReader;
        ///Field `U2X2` writer - Double the USART transmission speed
        pub type U2X2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `UPE2` reader - Parity Error
        pub type UPE2_R = crate::BitReader;
        ///Field `DOR2` reader - Data overRun
        pub type DOR2_R = crate::BitReader;
        ///Field `FE2` reader - Framing Error
        pub type FE2_R = crate::BitReader;
        ///Field `UDRE2` reader - USART Data Register Empty
        pub type UDRE2_R = crate::BitReader;
        ///Field `TXC2` reader - USART Transmit Complete
        pub type TXC2_R = crate::BitReader;
        ///Field `TXC2` writer - USART Transmit Complete
        pub type TXC2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXC2` reader - USART Receive Complete
        pub type RXC2_R = crate::BitReader;
        impl R {
            ///Bit 0 - Multi-processor Communication Mode
            #[inline(always)]
            pub fn mpcm2(&self) -> MPCM2_R {
                MPCM2_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Double the USART transmission speed
            #[inline(always)]
            pub fn u2x2(&self) -> U2X2_R {
                U2X2_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Parity Error
            #[inline(always)]
            pub fn upe2(&self) -> UPE2_R {
                UPE2_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Data overRun
            #[inline(always)]
            pub fn dor2(&self) -> DOR2_R {
                DOR2_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Framing Error
            #[inline(always)]
            pub fn fe2(&self) -> FE2_R {
                FE2_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - USART Data Register Empty
            #[inline(always)]
            pub fn udre2(&self) -> UDRE2_R {
                UDRE2_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - USART Transmit Complete
            #[inline(always)]
            pub fn txc2(&self) -> TXC2_R {
                TXC2_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - USART Receive Complete
            #[inline(always)]
            pub fn rxc2(&self) -> RXC2_R {
                RXC2_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Multi-processor Communication Mode
            #[inline(always)]
            pub fn mpcm2(&mut self) -> MPCM2_W<'_, UCSR2A_SPEC> {
                MPCM2_W::new(self, 0)
            }
            ///Bit 1 - Double the USART transmission speed
            #[inline(always)]
            pub fn u2x2(&mut self) -> U2X2_W<'_, UCSR2A_SPEC> {
                U2X2_W::new(self, 1)
            }
            ///Bit 6 - USART Transmit Complete
            #[inline(always)]
            pub fn txc2(&mut self) -> TXC2_W<'_, UCSR2A_SPEC> {
                TXC2_W::new(self, 6)
            }
        }
        /**USART Control and Status Register A

You can [`read`](crate::Reg::read) this register and get [`ucsr2a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr2a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR2A_SPEC;
        impl crate::RegisterSpec for UCSR2A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr2a::R`](R) reader structure
        impl crate::Readable for UCSR2A_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr2a::W`](W) writer structure
        impl crate::Writable for UCSR2A_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR2A to value 0
        impl crate::Resettable for UCSR2A_SPEC {}
    }
    /**UCSR2B (rw) register accessor: USART Control and Status Register B

You can [`read`](crate::Reg::read) this register and get [`ucsr2b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr2b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr2b`] module*/
    pub type UCSR2B = crate::Reg<ucsr2b::UCSR2B_SPEC>;
    ///USART Control and Status Register B
    pub mod ucsr2b {
        ///Register `UCSR2B` reader
        pub type R = crate::R<UCSR2B_SPEC>;
        ///Register `UCSR2B` writer
        pub type W = crate::W<UCSR2B_SPEC>;
        ///Field `TXB82` reader - Transmit Data Bit 8
        pub type TXB82_R = crate::BitReader;
        ///Field `TXB82` writer - Transmit Data Bit 8
        pub type TXB82_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXB82` reader - Receive Data Bit 8
        pub type RXB82_R = crate::BitReader;
        ///Field `UCSZ22` reader - Character Size
        pub type UCSZ22_R = crate::BitReader;
        ///Field `UCSZ22` writer - Character Size
        pub type UCSZ22_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TXEN2` reader - Transmitter Enable
        pub type TXEN2_R = crate::BitReader;
        ///Field `TXEN2` writer - Transmitter Enable
        pub type TXEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXEN2` reader - Receiver Enable
        pub type RXEN2_R = crate::BitReader;
        ///Field `RXEN2` writer - Receiver Enable
        pub type RXEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `UDRIE2` reader - USART Data register Empty Interrupt Enable
        pub type UDRIE2_R = crate::BitReader;
        ///Field `UDRIE2` writer - USART Data register Empty Interrupt Enable
        pub type UDRIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TXCIE2` reader - TX Complete Interrupt Enable
        pub type TXCIE2_R = crate::BitReader;
        ///Field `TXCIE2` writer - TX Complete Interrupt Enable
        pub type TXCIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXCIE2` reader - RX Complete Interrupt Enable
        pub type RXCIE2_R = crate::BitReader;
        ///Field `RXCIE2` writer - RX Complete Interrupt Enable
        pub type RXCIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Transmit Data Bit 8
            #[inline(always)]
            pub fn txb82(&self) -> TXB82_R {
                TXB82_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Receive Data Bit 8
            #[inline(always)]
            pub fn rxb82(&self) -> RXB82_R {
                RXB82_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Character Size
            #[inline(always)]
            pub fn ucsz22(&self) -> UCSZ22_R {
                UCSZ22_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Transmitter Enable
            #[inline(always)]
            pub fn txen2(&self) -> TXEN2_R {
                TXEN2_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Receiver Enable
            #[inline(always)]
            pub fn rxen2(&self) -> RXEN2_R {
                RXEN2_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - USART Data register Empty Interrupt Enable
            #[inline(always)]
            pub fn udrie2(&self) -> UDRIE2_R {
                UDRIE2_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - TX Complete Interrupt Enable
            #[inline(always)]
            pub fn txcie2(&self) -> TXCIE2_R {
                TXCIE2_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - RX Complete Interrupt Enable
            #[inline(always)]
            pub fn rxcie2(&self) -> RXCIE2_R {
                RXCIE2_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Transmit Data Bit 8
            #[inline(always)]
            pub fn txb82(&mut self) -> TXB82_W<'_, UCSR2B_SPEC> {
                TXB82_W::new(self, 0)
            }
            ///Bit 2 - Character Size
            #[inline(always)]
            pub fn ucsz22(&mut self) -> UCSZ22_W<'_, UCSR2B_SPEC> {
                UCSZ22_W::new(self, 2)
            }
            ///Bit 3 - Transmitter Enable
            #[inline(always)]
            pub fn txen2(&mut self) -> TXEN2_W<'_, UCSR2B_SPEC> {
                TXEN2_W::new(self, 3)
            }
            ///Bit 4 - Receiver Enable
            #[inline(always)]
            pub fn rxen2(&mut self) -> RXEN2_W<'_, UCSR2B_SPEC> {
                RXEN2_W::new(self, 4)
            }
            ///Bit 5 - USART Data register Empty Interrupt Enable
            #[inline(always)]
            pub fn udrie2(&mut self) -> UDRIE2_W<'_, UCSR2B_SPEC> {
                UDRIE2_W::new(self, 5)
            }
            ///Bit 6 - TX Complete Interrupt Enable
            #[inline(always)]
            pub fn txcie2(&mut self) -> TXCIE2_W<'_, UCSR2B_SPEC> {
                TXCIE2_W::new(self, 6)
            }
            ///Bit 7 - RX Complete Interrupt Enable
            #[inline(always)]
            pub fn rxcie2(&mut self) -> RXCIE2_W<'_, UCSR2B_SPEC> {
                RXCIE2_W::new(self, 7)
            }
        }
        /**USART Control and Status Register B

You can [`read`](crate::Reg::read) this register and get [`ucsr2b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr2b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR2B_SPEC;
        impl crate::RegisterSpec for UCSR2B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr2b::R`](R) reader structure
        impl crate::Readable for UCSR2B_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr2b::W`](W) writer structure
        impl crate::Writable for UCSR2B_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR2B to value 0
        impl crate::Resettable for UCSR2B_SPEC {}
    }
    /**UCSR2C (rw) register accessor: USART Control and Status Register C

You can [`read`](crate::Reg::read) this register and get [`ucsr2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr2c`] module*/
    pub type UCSR2C = crate::Reg<ucsr2c::UCSR2C_SPEC>;
    ///USART Control and Status Register C
    pub mod ucsr2c {
        ///Register `UCSR2C` reader
        pub type R = crate::R<UCSR2C_SPEC>;
        ///Register `UCSR2C` writer
        pub type W = crate::W<UCSR2C_SPEC>;
        /**Clock Polarity

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum UCPOL2_A {
            ///0: Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            RISING_EDGE = 0,
            ///1: Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            FALLING_EDGE = 1,
        }
        impl From<UCPOL2_A> for bool {
            #[inline(always)]
            fn from(variant: UCPOL2_A) -> Self {
                variant as u8 != 0
            }
        }
        ///Field `UCPOL2` reader - Clock Polarity
        pub type UCPOL2_R = crate::BitReader<UCPOL2_A>;
        impl UCPOL2_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> UCPOL2_A {
                match self.bits {
                    false => UCPOL2_A::RISING_EDGE,
                    true => UCPOL2_A::FALLING_EDGE,
                }
            }
            ///Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            #[inline(always)]
            pub fn is_rising_edge(&self) -> bool {
                *self == UCPOL2_A::RISING_EDGE
            }
            ///Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            #[inline(always)]
            pub fn is_falling_edge(&self) -> bool {
                *self == UCPOL2_A::FALLING_EDGE
            }
        }
        ///Field `UCPOL2` writer - Clock Polarity
        pub type UCPOL2_W<'a, REG> = crate::BitWriter<'a, REG, UCPOL2_A>;
        impl<'a, REG> UCPOL2_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
        {
            ///Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            #[inline(always)]
            pub fn rising_edge(self) -> &'a mut crate::W<REG> {
                self.variant(UCPOL2_A::RISING_EDGE)
            }
            ///Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            #[inline(always)]
            pub fn falling_edge(self) -> &'a mut crate::W<REG> {
                self.variant(UCPOL2_A::FALLING_EDGE)
            }
        }
        /**Character Size

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UCSZ2_A {
            ///0: Character Size: 5 bit
            CHR5 = 0,
            ///1: Character Size: 6 bit
            CHR6 = 1,
            ///2: Character Size: 7 bit
            CHR7 = 2,
            ///3: Character Size: 8 bit
            CHR8 = 3,
        }
        impl From<UCSZ2_A> for u8 {
            #[inline(always)]
            fn from(variant: UCSZ2_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UCSZ2_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UCSZ2_A {}
        ///Field `UCSZ2` reader - Character Size
        pub type UCSZ2_R = crate::FieldReader<UCSZ2_A>;
        impl UCSZ2_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> UCSZ2_A {
                match self.bits {
                    0 => UCSZ2_A::CHR5,
                    1 => UCSZ2_A::CHR6,
                    2 => UCSZ2_A::CHR7,
                    3 => UCSZ2_A::CHR8,
                    _ => unreachable!(),
                }
            }
            ///Character Size: 5 bit
            #[inline(always)]
            pub fn is_chr5(&self) -> bool {
                *self == UCSZ2_A::CHR5
            }
            ///Character Size: 6 bit
            #[inline(always)]
            pub fn is_chr6(&self) -> bool {
                *self == UCSZ2_A::CHR6
            }
            ///Character Size: 7 bit
            #[inline(always)]
            pub fn is_chr7(&self) -> bool {
                *self == UCSZ2_A::CHR7
            }
            ///Character Size: 8 bit
            #[inline(always)]
            pub fn is_chr8(&self) -> bool {
                *self == UCSZ2_A::CHR8
            }
        }
        ///Field `UCSZ2` writer - Character Size
        pub type UCSZ2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UCSZ2_A, crate::Safe>;
        impl<'a, REG> UCSZ2_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Character Size: 5 bit
            #[inline(always)]
            pub fn chr5(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ2_A::CHR5)
            }
            ///Character Size: 6 bit
            #[inline(always)]
            pub fn chr6(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ2_A::CHR6)
            }
            ///Character Size: 7 bit
            #[inline(always)]
            pub fn chr7(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ2_A::CHR7)
            }
            ///Character Size: 8 bit
            #[inline(always)]
            pub fn chr8(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ2_A::CHR8)
            }
        }
        /**Stop Bit Select

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum USBS2_A {
            ///0: 1-bit
            STOP1 = 0,
            ///1: 2-bit
            STOP2 = 1,
        }
        impl From<USBS2_A> for bool {
            #[inline(always)]
            fn from(variant: USBS2_A) -> Self {
                variant as u8 != 0
            }
        }
        ///Field `USBS2` reader - Stop Bit Select
        pub type USBS2_R = crate::BitReader<USBS2_A>;
        impl USBS2_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> USBS2_A {
                match self.bits {
                    false => USBS2_A::STOP1,
                    true => USBS2_A::STOP2,
                }
            }
            ///1-bit
            #[inline(always)]
            pub fn is_stop1(&self) -> bool {
                *self == USBS2_A::STOP1
            }
            ///2-bit
            #[inline(always)]
            pub fn is_stop2(&self) -> bool {
                *self == USBS2_A::STOP2
            }
        }
        ///Field `USBS2` writer - Stop Bit Select
        pub type USBS2_W<'a, REG> = crate::BitWriter<'a, REG, USBS2_A>;
        impl<'a, REG> USBS2_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
        {
            ///1-bit
            #[inline(always)]
            pub fn stop1(self) -> &'a mut crate::W<REG> {
                self.variant(USBS2_A::STOP1)
            }
            ///2-bit
            #[inline(always)]
            pub fn stop2(self) -> &'a mut crate::W<REG> {
                self.variant(USBS2_A::STOP2)
            }
        }
        /**Parity Mode Bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UPM2_A {
            ///0: Disabled
            DISABLED = 0,
            ///2: Enabled, Even Parity
            PARITY_EVEN = 2,
            ///3: Enabled, Odd Parity
            PARITY_ODD = 3,
        }
        impl From<UPM2_A> for u8 {
            #[inline(always)]
            fn from(variant: UPM2_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UPM2_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UPM2_A {}
        ///Field `UPM2` reader - Parity Mode Bits
        pub type UPM2_R = crate::FieldReader<UPM2_A>;
        impl UPM2_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<UPM2_A> {
                match self.bits {
                    0 => Some(UPM2_A::DISABLED),
                    2 => Some(UPM2_A::PARITY_EVEN),
                    3 => Some(UPM2_A::PARITY_ODD),
                    _ => None,
                }
            }
            ///Disabled
            #[inline(always)]
            pub fn is_disabled(&self) -> bool {
                *self == UPM2_A::DISABLED
            }
            ///Enabled, Even Parity
            #[inline(always)]
            pub fn is_parity_even(&self) -> bool {
                *self == UPM2_A::PARITY_EVEN
            }
            ///Enabled, Odd Parity
            #[inline(always)]
            pub fn is_parity_odd(&self) -> bool {
                *self == UPM2_A::PARITY_ODD
            }
        }
        ///Field `UPM2` writer - Parity Mode Bits
        pub type UPM2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UPM2_A>;
        impl<'a, REG> UPM2_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Disabled
            #[inline(always)]
            pub fn disabled(self) -> &'a mut crate::W<REG> {
                self.variant(UPM2_A::DISABLED)
            }
            ///Enabled, Even Parity
            #[inline(always)]
            pub fn parity_even(self) -> &'a mut crate::W<REG> {
                self.variant(UPM2_A::PARITY_EVEN)
            }
            ///Enabled, Odd Parity
            #[inline(always)]
            pub fn parity_odd(self) -> &'a mut crate::W<REG> {
                self.variant(UPM2_A::PARITY_ODD)
            }
        }
        /**USART Mode Select

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UMSEL2_A {
            ///0: Asynchronous USART
            USART_ASYNC = 0,
            ///1: Synchronous USART
            USART_SYNC = 1,
            ///3: Master SPI (MSPIM)
            SPI_MASTER = 3,
        }
        impl From<UMSEL2_A> for u8 {
            #[inline(always)]
            fn from(variant: UMSEL2_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UMSEL2_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UMSEL2_A {}
        ///Field `UMSEL2` reader - USART Mode Select
        pub type UMSEL2_R = crate::FieldReader<UMSEL2_A>;
        impl UMSEL2_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<UMSEL2_A> {
                match self.bits {
                    0 => Some(UMSEL2_A::USART_ASYNC),
                    1 => Some(UMSEL2_A::USART_SYNC),
                    3 => Some(UMSEL2_A::SPI_MASTER),
                    _ => None,
                }
            }
            ///Asynchronous USART
            #[inline(always)]
            pub fn is_usart_async(&self) -> bool {
                *self == UMSEL2_A::USART_ASYNC
            }
            ///Synchronous USART
            #[inline(always)]
            pub fn is_usart_sync(&self) -> bool {
                *self == UMSEL2_A::USART_SYNC
            }
            ///Master SPI (MSPIM)
            #[inline(always)]
            pub fn is_spi_master(&self) -> bool {
                *self == UMSEL2_A::SPI_MASTER
            }
        }
        ///Field `UMSEL2` writer - USART Mode Select
        pub type UMSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UMSEL2_A>;
        impl<'a, REG> UMSEL2_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Asynchronous USART
            #[inline(always)]
            pub fn usart_async(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL2_A::USART_ASYNC)
            }
            ///Synchronous USART
            #[inline(always)]
            pub fn usart_sync(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL2_A::USART_SYNC)
            }
            ///Master SPI (MSPIM)
            #[inline(always)]
            pub fn spi_master(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL2_A::SPI_MASTER)
            }
        }
        impl R {
            ///Bit 0 - Clock Polarity
            #[inline(always)]
            pub fn ucpol2(&self) -> UCPOL2_R {
                UCPOL2_R::new((self.bits & 1) != 0)
            }
            ///Bits 1:2 - Character Size
            #[inline(always)]
            pub fn ucsz2(&self) -> UCSZ2_R {
                UCSZ2_R::new((self.bits >> 1) & 3)
            }
            ///Bit 3 - Stop Bit Select
            #[inline(always)]
            pub fn usbs2(&self) -> USBS2_R {
                USBS2_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bits 4:5 - Parity Mode Bits
            #[inline(always)]
            pub fn upm2(&self) -> UPM2_R {
                UPM2_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - USART Mode Select
            #[inline(always)]
            pub fn umsel2(&self) -> UMSEL2_R {
                UMSEL2_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bit 0 - Clock Polarity
            #[inline(always)]
            pub fn ucpol2(&mut self) -> UCPOL2_W<'_, UCSR2C_SPEC> {
                UCPOL2_W::new(self, 0)
            }
            ///Bits 1:2 - Character Size
            #[inline(always)]
            pub fn ucsz2(&mut self) -> UCSZ2_W<'_, UCSR2C_SPEC> {
                UCSZ2_W::new(self, 1)
            }
            ///Bit 3 - Stop Bit Select
            #[inline(always)]
            pub fn usbs2(&mut self) -> USBS2_W<'_, UCSR2C_SPEC> {
                USBS2_W::new(self, 3)
            }
            ///Bits 4:5 - Parity Mode Bits
            #[inline(always)]
            pub fn upm2(&mut self) -> UPM2_W<'_, UCSR2C_SPEC> {
                UPM2_W::new(self, 4)
            }
            ///Bits 6:7 - USART Mode Select
            #[inline(always)]
            pub fn umsel2(&mut self) -> UMSEL2_W<'_, UCSR2C_SPEC> {
                UMSEL2_W::new(self, 6)
            }
        }
        /**USART Control and Status Register C

You can [`read`](crate::Reg::read) this register and get [`ucsr2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR2C_SPEC;
        impl crate::RegisterSpec for UCSR2C_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr2c::R`](R) reader structure
        impl crate::Readable for UCSR2C_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr2c::W`](W) writer structure
        impl crate::Writable for UCSR2C_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR2C to value 0
        impl crate::Resettable for UCSR2C_SPEC {}
    }
    /**UDR2 (rw) register accessor: USART I/O Data Register

You can [`read`](crate::Reg::read) this register and get [`udr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@udr2`] module*/
    pub type UDR2 = crate::Reg<udr2::UDR2_SPEC>;
    ///USART I/O Data Register
    pub mod udr2 {
        ///Register `UDR2` reader
        pub type R = crate::R<UDR2_SPEC>;
        ///Register `UDR2` writer
        pub type W = crate::W<UDR2_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**USART I/O Data Register

You can [`read`](crate::Reg::read) this register and get [`udr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UDR2_SPEC;
        impl crate::RegisterSpec for UDR2_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`udr2::R`](R) reader structure
        impl crate::Readable for UDR2_SPEC {}
        ///`write(|w| ..)` method takes [`udr2::W`](W) writer structure
        impl crate::Writable for UDR2_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets UDR2 to value 0
        impl crate::Resettable for UDR2_SPEC {}
    }
}
///USART
pub type USART3 = crate::Periph<usart3::RegisterBlock, 0x0130>;
impl core::fmt::Debug for USART3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART3").finish()
    }
}
///USART
pub mod usart3 {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        ucsr3a: UCSR3A,
        ucsr3b: UCSR3B,
        ucsr3c: UCSR3C,
        _reserved3: [u8; 0x01],
        ubrr3: UBRR3,
        udr3: UDR3,
    }
    impl RegisterBlock {
        ///0x00 - USART Control and Status Register A
        #[inline(always)]
        pub const fn ucsr3a(&self) -> &UCSR3A {
            &self.ucsr3a
        }
        ///0x01 - USART Control and Status Register B
        #[inline(always)]
        pub const fn ucsr3b(&self) -> &UCSR3B {
            &self.ucsr3b
        }
        ///0x02 - USART Control and Status Register C
        #[inline(always)]
        pub const fn ucsr3c(&self) -> &UCSR3C {
            &self.ucsr3c
        }
        ///0x04 - USART Baud Rate Register Bytes
        #[inline(always)]
        pub const fn ubrr3(&self) -> &UBRR3 {
            &self.ubrr3
        }
        ///0x06 - USART I/O Data Register
        #[inline(always)]
        pub const fn udr3(&self) -> &UDR3 {
            &self.udr3
        }
    }
    /**UBRR3 (rw) register accessor: USART Baud Rate Register Bytes

You can [`read`](crate::Reg::read) this register and get [`ubrr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubrr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ubrr3`] module*/
    pub type UBRR3 = crate::Reg<ubrr3::UBRR3_SPEC>;
    ///USART Baud Rate Register Bytes
    pub mod ubrr3 {
        ///Register `UBRR3` reader
        pub type R = crate::R<UBRR3_SPEC>;
        ///Register `UBRR3` writer
        pub type W = crate::W<UBRR3_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**USART Baud Rate Register Bytes

You can [`read`](crate::Reg::read) this register and get [`ubrr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ubrr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UBRR3_SPEC;
        impl crate::RegisterSpec for UBRR3_SPEC {
            type Ux = u16;
        }
        ///`read()` method returns [`ubrr3::R`](R) reader structure
        impl crate::Readable for UBRR3_SPEC {}
        ///`write(|w| ..)` method takes [`ubrr3::W`](W) writer structure
        impl crate::Writable for UBRR3_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets UBRR3 to value 0
        impl crate::Resettable for UBRR3_SPEC {}
    }
    /**UCSR3A (rw) register accessor: USART Control and Status Register A

You can [`read`](crate::Reg::read) this register and get [`ucsr3a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr3a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr3a`] module*/
    pub type UCSR3A = crate::Reg<ucsr3a::UCSR3A_SPEC>;
    ///USART Control and Status Register A
    pub mod ucsr3a {
        ///Register `UCSR3A` reader
        pub type R = crate::R<UCSR3A_SPEC>;
        ///Register `UCSR3A` writer
        pub type W = crate::W<UCSR3A_SPEC>;
        ///Field `MPCM3` reader - Multi-processor Communication Mode
        pub type MPCM3_R = crate::BitReader;
        ///Field `MPCM3` writer - Multi-processor Communication Mode
        pub type MPCM3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `U2X3` reader - Double the USART transmission speed
        pub type U2X3_R = crate::BitReader;
        ///Field `U2X3` writer - Double the USART transmission speed
        pub type U2X3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `UPE3` reader - Parity Error
        pub type UPE3_R = crate::BitReader;
        ///Field `DOR3` reader - Data overRun
        pub type DOR3_R = crate::BitReader;
        ///Field `FE3` reader - Framing Error
        pub type FE3_R = crate::BitReader;
        ///Field `UDRE3` reader - USART Data Register Empty
        pub type UDRE3_R = crate::BitReader;
        ///Field `TXC3` reader - USART Transmit Complete
        pub type TXC3_R = crate::BitReader;
        ///Field `TXC3` writer - USART Transmit Complete
        pub type TXC3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXC3` reader - USART Receive Complete
        pub type RXC3_R = crate::BitReader;
        impl R {
            ///Bit 0 - Multi-processor Communication Mode
            #[inline(always)]
            pub fn mpcm3(&self) -> MPCM3_R {
                MPCM3_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Double the USART transmission speed
            #[inline(always)]
            pub fn u2x3(&self) -> U2X3_R {
                U2X3_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Parity Error
            #[inline(always)]
            pub fn upe3(&self) -> UPE3_R {
                UPE3_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Data overRun
            #[inline(always)]
            pub fn dor3(&self) -> DOR3_R {
                DOR3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Framing Error
            #[inline(always)]
            pub fn fe3(&self) -> FE3_R {
                FE3_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - USART Data Register Empty
            #[inline(always)]
            pub fn udre3(&self) -> UDRE3_R {
                UDRE3_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - USART Transmit Complete
            #[inline(always)]
            pub fn txc3(&self) -> TXC3_R {
                TXC3_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - USART Receive Complete
            #[inline(always)]
            pub fn rxc3(&self) -> RXC3_R {
                RXC3_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Multi-processor Communication Mode
            #[inline(always)]
            pub fn mpcm3(&mut self) -> MPCM3_W<'_, UCSR3A_SPEC> {
                MPCM3_W::new(self, 0)
            }
            ///Bit 1 - Double the USART transmission speed
            #[inline(always)]
            pub fn u2x3(&mut self) -> U2X3_W<'_, UCSR3A_SPEC> {
                U2X3_W::new(self, 1)
            }
            ///Bit 6 - USART Transmit Complete
            #[inline(always)]
            pub fn txc3(&mut self) -> TXC3_W<'_, UCSR3A_SPEC> {
                TXC3_W::new(self, 6)
            }
        }
        /**USART Control and Status Register A

You can [`read`](crate::Reg::read) this register and get [`ucsr3a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr3a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR3A_SPEC;
        impl crate::RegisterSpec for UCSR3A_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr3a::R`](R) reader structure
        impl crate::Readable for UCSR3A_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr3a::W`](W) writer structure
        impl crate::Writable for UCSR3A_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR3A to value 0
        impl crate::Resettable for UCSR3A_SPEC {}
    }
    /**UCSR3B (rw) register accessor: USART Control and Status Register B

You can [`read`](crate::Reg::read) this register and get [`ucsr3b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr3b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr3b`] module*/
    pub type UCSR3B = crate::Reg<ucsr3b::UCSR3B_SPEC>;
    ///USART Control and Status Register B
    pub mod ucsr3b {
        ///Register `UCSR3B` reader
        pub type R = crate::R<UCSR3B_SPEC>;
        ///Register `UCSR3B` writer
        pub type W = crate::W<UCSR3B_SPEC>;
        ///Field `TXB83` reader - Transmit Data Bit 8
        pub type TXB83_R = crate::BitReader;
        ///Field `TXB83` writer - Transmit Data Bit 8
        pub type TXB83_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXB83` reader - Receive Data Bit 8
        pub type RXB83_R = crate::BitReader;
        ///Field `UCSZ32` reader - Character Size
        pub type UCSZ32_R = crate::BitReader;
        ///Field `UCSZ32` writer - Character Size
        pub type UCSZ32_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TXEN3` reader - Transmitter Enable
        pub type TXEN3_R = crate::BitReader;
        ///Field `TXEN3` writer - Transmitter Enable
        pub type TXEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXEN3` reader - Receiver Enable
        pub type RXEN3_R = crate::BitReader;
        ///Field `RXEN3` writer - Receiver Enable
        pub type RXEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `UDRIE3` reader - USART Data register Empty Interrupt Enable
        pub type UDRIE3_R = crate::BitReader;
        ///Field `UDRIE3` writer - USART Data register Empty Interrupt Enable
        pub type UDRIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `TXCIE3` reader - TX Complete Interrupt Enable
        pub type TXCIE3_R = crate::BitReader;
        ///Field `TXCIE3` writer - TX Complete Interrupt Enable
        pub type TXCIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `RXCIE3` reader - RX Complete Interrupt Enable
        pub type RXCIE3_R = crate::BitReader;
        ///Field `RXCIE3` writer - RX Complete Interrupt Enable
        pub type RXCIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bit 0 - Transmit Data Bit 8
            #[inline(always)]
            pub fn txb83(&self) -> TXB83_R {
                TXB83_R::new((self.bits & 1) != 0)
            }
            ///Bit 1 - Receive Data Bit 8
            #[inline(always)]
            pub fn rxb83(&self) -> RXB83_R {
                RXB83_R::new(((self.bits >> 1) & 1) != 0)
            }
            ///Bit 2 - Character Size
            #[inline(always)]
            pub fn ucsz32(&self) -> UCSZ32_R {
                UCSZ32_R::new(((self.bits >> 2) & 1) != 0)
            }
            ///Bit 3 - Transmitter Enable
            #[inline(always)]
            pub fn txen3(&self) -> TXEN3_R {
                TXEN3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Receiver Enable
            #[inline(always)]
            pub fn rxen3(&self) -> RXEN3_R {
                RXEN3_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - USART Data register Empty Interrupt Enable
            #[inline(always)]
            pub fn udrie3(&self) -> UDRIE3_R {
                UDRIE3_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - TX Complete Interrupt Enable
            #[inline(always)]
            pub fn txcie3(&self) -> TXCIE3_R {
                TXCIE3_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - RX Complete Interrupt Enable
            #[inline(always)]
            pub fn rxcie3(&self) -> RXCIE3_R {
                RXCIE3_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bit 0 - Transmit Data Bit 8
            #[inline(always)]
            pub fn txb83(&mut self) -> TXB83_W<'_, UCSR3B_SPEC> {
                TXB83_W::new(self, 0)
            }
            ///Bit 2 - Character Size
            #[inline(always)]
            pub fn ucsz32(&mut self) -> UCSZ32_W<'_, UCSR3B_SPEC> {
                UCSZ32_W::new(self, 2)
            }
            ///Bit 3 - Transmitter Enable
            #[inline(always)]
            pub fn txen3(&mut self) -> TXEN3_W<'_, UCSR3B_SPEC> {
                TXEN3_W::new(self, 3)
            }
            ///Bit 4 - Receiver Enable
            #[inline(always)]
            pub fn rxen3(&mut self) -> RXEN3_W<'_, UCSR3B_SPEC> {
                RXEN3_W::new(self, 4)
            }
            ///Bit 5 - USART Data register Empty Interrupt Enable
            #[inline(always)]
            pub fn udrie3(&mut self) -> UDRIE3_W<'_, UCSR3B_SPEC> {
                UDRIE3_W::new(self, 5)
            }
            ///Bit 6 - TX Complete Interrupt Enable
            #[inline(always)]
            pub fn txcie3(&mut self) -> TXCIE3_W<'_, UCSR3B_SPEC> {
                TXCIE3_W::new(self, 6)
            }
            ///Bit 7 - RX Complete Interrupt Enable
            #[inline(always)]
            pub fn rxcie3(&mut self) -> RXCIE3_W<'_, UCSR3B_SPEC> {
                RXCIE3_W::new(self, 7)
            }
        }
        /**USART Control and Status Register B

You can [`read`](crate::Reg::read) this register and get [`ucsr3b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr3b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR3B_SPEC;
        impl crate::RegisterSpec for UCSR3B_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr3b::R`](R) reader structure
        impl crate::Readable for UCSR3B_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr3b::W`](W) writer structure
        impl crate::Writable for UCSR3B_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR3B to value 0
        impl crate::Resettable for UCSR3B_SPEC {}
    }
    /**UCSR3C (rw) register accessor: USART Control and Status Register C

You can [`read`](crate::Reg::read) this register and get [`ucsr3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ucsr3c`] module*/
    pub type UCSR3C = crate::Reg<ucsr3c::UCSR3C_SPEC>;
    ///USART Control and Status Register C
    pub mod ucsr3c {
        ///Register `UCSR3C` reader
        pub type R = crate::R<UCSR3C_SPEC>;
        ///Register `UCSR3C` writer
        pub type W = crate::W<UCSR3C_SPEC>;
        /**Clock Polarity

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum UCPOL3_A {
            ///0: Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            RISING_EDGE = 0,
            ///1: Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            FALLING_EDGE = 1,
        }
        impl From<UCPOL3_A> for bool {
            #[inline(always)]
            fn from(variant: UCPOL3_A) -> Self {
                variant as u8 != 0
            }
        }
        ///Field `UCPOL3` reader - Clock Polarity
        pub type UCPOL3_R = crate::BitReader<UCPOL3_A>;
        impl UCPOL3_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> UCPOL3_A {
                match self.bits {
                    false => UCPOL3_A::RISING_EDGE,
                    true => UCPOL3_A::FALLING_EDGE,
                }
            }
            ///Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            #[inline(always)]
            pub fn is_rising_edge(&self) -> bool {
                *self == UCPOL3_A::RISING_EDGE
            }
            ///Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            #[inline(always)]
            pub fn is_falling_edge(&self) -> bool {
                *self == UCPOL3_A::FALLING_EDGE
            }
        }
        ///Field `UCPOL3` writer - Clock Polarity
        pub type UCPOL3_W<'a, REG> = crate::BitWriter<'a, REG, UCPOL3_A>;
        impl<'a, REG> UCPOL3_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
        {
            ///Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge
            #[inline(always)]
            pub fn rising_edge(self) -> &'a mut crate::W<REG> {
                self.variant(UCPOL3_A::RISING_EDGE)
            }
            ///Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge
            #[inline(always)]
            pub fn falling_edge(self) -> &'a mut crate::W<REG> {
                self.variant(UCPOL3_A::FALLING_EDGE)
            }
        }
        /**Character Size

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UCSZ3_A {
            ///0: Character Size: 5 bit
            CHR5 = 0,
            ///1: Character Size: 6 bit
            CHR6 = 1,
            ///2: Character Size: 7 bit
            CHR7 = 2,
            ///3: Character Size: 8 bit
            CHR8 = 3,
        }
        impl From<UCSZ3_A> for u8 {
            #[inline(always)]
            fn from(variant: UCSZ3_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UCSZ3_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UCSZ3_A {}
        ///Field `UCSZ3` reader - Character Size
        pub type UCSZ3_R = crate::FieldReader<UCSZ3_A>;
        impl UCSZ3_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> UCSZ3_A {
                match self.bits {
                    0 => UCSZ3_A::CHR5,
                    1 => UCSZ3_A::CHR6,
                    2 => UCSZ3_A::CHR7,
                    3 => UCSZ3_A::CHR8,
                    _ => unreachable!(),
                }
            }
            ///Character Size: 5 bit
            #[inline(always)]
            pub fn is_chr5(&self) -> bool {
                *self == UCSZ3_A::CHR5
            }
            ///Character Size: 6 bit
            #[inline(always)]
            pub fn is_chr6(&self) -> bool {
                *self == UCSZ3_A::CHR6
            }
            ///Character Size: 7 bit
            #[inline(always)]
            pub fn is_chr7(&self) -> bool {
                *self == UCSZ3_A::CHR7
            }
            ///Character Size: 8 bit
            #[inline(always)]
            pub fn is_chr8(&self) -> bool {
                *self == UCSZ3_A::CHR8
            }
        }
        ///Field `UCSZ3` writer - Character Size
        pub type UCSZ3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UCSZ3_A, crate::Safe>;
        impl<'a, REG> UCSZ3_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Character Size: 5 bit
            #[inline(always)]
            pub fn chr5(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ3_A::CHR5)
            }
            ///Character Size: 6 bit
            #[inline(always)]
            pub fn chr6(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ3_A::CHR6)
            }
            ///Character Size: 7 bit
            #[inline(always)]
            pub fn chr7(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ3_A::CHR7)
            }
            ///Character Size: 8 bit
            #[inline(always)]
            pub fn chr8(self) -> &'a mut crate::W<REG> {
                self.variant(UCSZ3_A::CHR8)
            }
        }
        /**Stop Bit Select

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum USBS3_A {
            ///0: 1-bit
            STOP1 = 0,
            ///1: 2-bit
            STOP2 = 1,
        }
        impl From<USBS3_A> for bool {
            #[inline(always)]
            fn from(variant: USBS3_A) -> Self {
                variant as u8 != 0
            }
        }
        ///Field `USBS3` reader - Stop Bit Select
        pub type USBS3_R = crate::BitReader<USBS3_A>;
        impl USBS3_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> USBS3_A {
                match self.bits {
                    false => USBS3_A::STOP1,
                    true => USBS3_A::STOP2,
                }
            }
            ///1-bit
            #[inline(always)]
            pub fn is_stop1(&self) -> bool {
                *self == USBS3_A::STOP1
            }
            ///2-bit
            #[inline(always)]
            pub fn is_stop2(&self) -> bool {
                *self == USBS3_A::STOP2
            }
        }
        ///Field `USBS3` writer - Stop Bit Select
        pub type USBS3_W<'a, REG> = crate::BitWriter<'a, REG, USBS3_A>;
        impl<'a, REG> USBS3_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
        {
            ///1-bit
            #[inline(always)]
            pub fn stop1(self) -> &'a mut crate::W<REG> {
                self.variant(USBS3_A::STOP1)
            }
            ///2-bit
            #[inline(always)]
            pub fn stop2(self) -> &'a mut crate::W<REG> {
                self.variant(USBS3_A::STOP2)
            }
        }
        /**Parity Mode Bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UPM3_A {
            ///0: Disabled
            DISABLED = 0,
            ///2: Enabled, Even Parity
            PARITY_EVEN = 2,
            ///3: Enabled, Odd Parity
            PARITY_ODD = 3,
        }
        impl From<UPM3_A> for u8 {
            #[inline(always)]
            fn from(variant: UPM3_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UPM3_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UPM3_A {}
        ///Field `UPM3` reader - Parity Mode Bits
        pub type UPM3_R = crate::FieldReader<UPM3_A>;
        impl UPM3_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<UPM3_A> {
                match self.bits {
                    0 => Some(UPM3_A::DISABLED),
                    2 => Some(UPM3_A::PARITY_EVEN),
                    3 => Some(UPM3_A::PARITY_ODD),
                    _ => None,
                }
            }
            ///Disabled
            #[inline(always)]
            pub fn is_disabled(&self) -> bool {
                *self == UPM3_A::DISABLED
            }
            ///Enabled, Even Parity
            #[inline(always)]
            pub fn is_parity_even(&self) -> bool {
                *self == UPM3_A::PARITY_EVEN
            }
            ///Enabled, Odd Parity
            #[inline(always)]
            pub fn is_parity_odd(&self) -> bool {
                *self == UPM3_A::PARITY_ODD
            }
        }
        ///Field `UPM3` writer - Parity Mode Bits
        pub type UPM3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UPM3_A>;
        impl<'a, REG> UPM3_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Disabled
            #[inline(always)]
            pub fn disabled(self) -> &'a mut crate::W<REG> {
                self.variant(UPM3_A::DISABLED)
            }
            ///Enabled, Even Parity
            #[inline(always)]
            pub fn parity_even(self) -> &'a mut crate::W<REG> {
                self.variant(UPM3_A::PARITY_EVEN)
            }
            ///Enabled, Odd Parity
            #[inline(always)]
            pub fn parity_odd(self) -> &'a mut crate::W<REG> {
                self.variant(UPM3_A::PARITY_ODD)
            }
        }
        /**USART Mode Select

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum UMSEL3_A {
            ///0: Asynchronous USART
            USART_ASYNC = 0,
            ///1: Synchronous USART
            USART_SYNC = 1,
            ///3: Master SPI (MSPIM)
            SPI_MASTER = 3,
        }
        impl From<UMSEL3_A> for u8 {
            #[inline(always)]
            fn from(variant: UMSEL3_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for UMSEL3_A {
            type Ux = u8;
        }
        impl crate::IsEnum for UMSEL3_A {}
        ///Field `UMSEL3` reader - USART Mode Select
        pub type UMSEL3_R = crate::FieldReader<UMSEL3_A>;
        impl UMSEL3_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> Option<UMSEL3_A> {
                match self.bits {
                    0 => Some(UMSEL3_A::USART_ASYNC),
                    1 => Some(UMSEL3_A::USART_SYNC),
                    3 => Some(UMSEL3_A::SPI_MASTER),
                    _ => None,
                }
            }
            ///Asynchronous USART
            #[inline(always)]
            pub fn is_usart_async(&self) -> bool {
                *self == UMSEL3_A::USART_ASYNC
            }
            ///Synchronous USART
            #[inline(always)]
            pub fn is_usart_sync(&self) -> bool {
                *self == UMSEL3_A::USART_SYNC
            }
            ///Master SPI (MSPIM)
            #[inline(always)]
            pub fn is_spi_master(&self) -> bool {
                *self == UMSEL3_A::SPI_MASTER
            }
        }
        ///Field `UMSEL3` writer - USART Mode Select
        pub type UMSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UMSEL3_A>;
        impl<'a, REG> UMSEL3_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///Asynchronous USART
            #[inline(always)]
            pub fn usart_async(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL3_A::USART_ASYNC)
            }
            ///Synchronous USART
            #[inline(always)]
            pub fn usart_sync(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL3_A::USART_SYNC)
            }
            ///Master SPI (MSPIM)
            #[inline(always)]
            pub fn spi_master(self) -> &'a mut crate::W<REG> {
                self.variant(UMSEL3_A::SPI_MASTER)
            }
        }
        impl R {
            ///Bit 0 - Clock Polarity
            #[inline(always)]
            pub fn ucpol3(&self) -> UCPOL3_R {
                UCPOL3_R::new((self.bits & 1) != 0)
            }
            ///Bits 1:2 - Character Size
            #[inline(always)]
            pub fn ucsz3(&self) -> UCSZ3_R {
                UCSZ3_R::new((self.bits >> 1) & 3)
            }
            ///Bit 3 - Stop Bit Select
            #[inline(always)]
            pub fn usbs3(&self) -> USBS3_R {
                USBS3_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bits 4:5 - Parity Mode Bits
            #[inline(always)]
            pub fn upm3(&self) -> UPM3_R {
                UPM3_R::new((self.bits >> 4) & 3)
            }
            ///Bits 6:7 - USART Mode Select
            #[inline(always)]
            pub fn umsel3(&self) -> UMSEL3_R {
                UMSEL3_R::new((self.bits >> 6) & 3)
            }
        }
        impl W {
            ///Bit 0 - Clock Polarity
            #[inline(always)]
            pub fn ucpol3(&mut self) -> UCPOL3_W<'_, UCSR3C_SPEC> {
                UCPOL3_W::new(self, 0)
            }
            ///Bits 1:2 - Character Size
            #[inline(always)]
            pub fn ucsz3(&mut self) -> UCSZ3_W<'_, UCSR3C_SPEC> {
                UCSZ3_W::new(self, 1)
            }
            ///Bit 3 - Stop Bit Select
            #[inline(always)]
            pub fn usbs3(&mut self) -> USBS3_W<'_, UCSR3C_SPEC> {
                USBS3_W::new(self, 3)
            }
            ///Bits 4:5 - Parity Mode Bits
            #[inline(always)]
            pub fn upm3(&mut self) -> UPM3_W<'_, UCSR3C_SPEC> {
                UPM3_W::new(self, 4)
            }
            ///Bits 6:7 - USART Mode Select
            #[inline(always)]
            pub fn umsel3(&mut self) -> UMSEL3_W<'_, UCSR3C_SPEC> {
                UMSEL3_W::new(self, 6)
            }
        }
        /**USART Control and Status Register C

You can [`read`](crate::Reg::read) this register and get [`ucsr3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucsr3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UCSR3C_SPEC;
        impl crate::RegisterSpec for UCSR3C_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`ucsr3c::R`](R) reader structure
        impl crate::Readable for UCSR3C_SPEC {}
        ///`write(|w| ..)` method takes [`ucsr3c::W`](W) writer structure
        impl crate::Writable for UCSR3C_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets UCSR3C to value 0
        impl crate::Resettable for UCSR3C_SPEC {}
    }
    /**UDR3 (rw) register accessor: USART I/O Data Register

You can [`read`](crate::Reg::read) this register and get [`udr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@udr3`] module*/
    pub type UDR3 = crate::Reg<udr3::UDR3_SPEC>;
    ///USART I/O Data Register
    pub mod udr3 {
        ///Register `UDR3` reader
        pub type R = crate::R<UDR3_SPEC>;
        ///Register `UDR3` writer
        pub type W = crate::W<UDR3_SPEC>;
        impl core::fmt::Debug for R {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.bits())
            }
        }
        impl W {}
        /**USART I/O Data Register

You can [`read`](crate::Reg::read) this register and get [`udr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct UDR3_SPEC;
        impl crate::RegisterSpec for UDR3_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`udr3::R`](R) reader structure
        impl crate::Readable for UDR3_SPEC {}
        ///`write(|w| ..)` method takes [`udr3::W`](W) writer structure
        impl crate::Writable for UDR3_SPEC {
            type Safety = crate::Safe;
        }
        ///`reset()` method sets UDR3 to value 0
        impl crate::Resettable for UDR3_SPEC {}
    }
}
///Watchdog Timer
pub type WDT = crate::Periph<wdt::RegisterBlock, 0x60>;
impl core::fmt::Debug for WDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT").finish()
    }
}
///Watchdog Timer
pub mod wdt {
    #[repr(C)]
    ///Register block
    pub struct RegisterBlock {
        wdtcsr: WDTCSR,
    }
    impl RegisterBlock {
        ///0x00 - Watchdog Timer Control Register
        #[inline(always)]
        pub const fn wdtcsr(&self) -> &WDTCSR {
            &self.wdtcsr
        }
    }
    /**WDTCSR (rw) register accessor: Watchdog Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`wdtcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtcsr`] module*/
    pub type WDTCSR = crate::Reg<wdtcsr::WDTCSR_SPEC>;
    ///Watchdog Timer Control Register
    pub mod wdtcsr {
        ///Register `WDTCSR` reader
        pub type R = crate::R<WDTCSR_SPEC>;
        ///Register `WDTCSR` writer
        pub type W = crate::W<WDTCSR_SPEC>;
        /**Watchdog Timer Prescaler - Low Bits

Value on reset: 0*/
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum WDPL_A {
            ///0: - 2048 cycles, ~16ms/512K (524288) cycles, ~4s if WDPH is set
            CYCLES_2K_512K = 0,
            ///1: - 4096 cycles, ~32ms/1024K (1048576) cycles, ~8s if WDPH is set
            CYCLES_4K_1024K = 1,
            ///2: - 8192 cycles, ~64ms
            CYCLES_8K = 2,
            ///3: - 16K (16384) cycles, ~0.125s
            CYCLES_16K = 3,
            ///4: - 32K (32768) cycles, ~0.25s
            CYCLES_32K = 4,
            ///5: - 64K (65536) cycles, ~0.5s
            CYCLES_64K = 5,
            ///6: - 128K (131072) cycles, ~1s
            CYCLES_128K = 6,
            ///7: - 256K (262144) cycles, ~2s
            CYCLES_256K = 7,
        }
        impl From<WDPL_A> for u8 {
            #[inline(always)]
            fn from(variant: WDPL_A) -> Self {
                variant as _
            }
        }
        impl crate::FieldSpec for WDPL_A {
            type Ux = u8;
        }
        impl crate::IsEnum for WDPL_A {}
        ///Field `WDPL` reader - Watchdog Timer Prescaler - Low Bits
        pub type WDPL_R = crate::FieldReader<WDPL_A>;
        impl WDPL_R {
            ///Get enumerated values variant
            #[inline(always)]
            pub const fn variant(&self) -> WDPL_A {
                match self.bits {
                    0 => WDPL_A::CYCLES_2K_512K,
                    1 => WDPL_A::CYCLES_4K_1024K,
                    2 => WDPL_A::CYCLES_8K,
                    3 => WDPL_A::CYCLES_16K,
                    4 => WDPL_A::CYCLES_32K,
                    5 => WDPL_A::CYCLES_64K,
                    6 => WDPL_A::CYCLES_128K,
                    7 => WDPL_A::CYCLES_256K,
                    _ => unreachable!(),
                }
            }
            ///- 2048 cycles, ~16ms/512K (524288) cycles, ~4s if WDPH is set
            #[inline(always)]
            pub fn is_cycles_2k_512k(&self) -> bool {
                *self == WDPL_A::CYCLES_2K_512K
            }
            ///- 4096 cycles, ~32ms/1024K (1048576) cycles, ~8s if WDPH is set
            #[inline(always)]
            pub fn is_cycles_4k_1024k(&self) -> bool {
                *self == WDPL_A::CYCLES_4K_1024K
            }
            ///- 8192 cycles, ~64ms
            #[inline(always)]
            pub fn is_cycles_8k(&self) -> bool {
                *self == WDPL_A::CYCLES_8K
            }
            ///- 16K (16384) cycles, ~0.125s
            #[inline(always)]
            pub fn is_cycles_16k(&self) -> bool {
                *self == WDPL_A::CYCLES_16K
            }
            ///- 32K (32768) cycles, ~0.25s
            #[inline(always)]
            pub fn is_cycles_32k(&self) -> bool {
                *self == WDPL_A::CYCLES_32K
            }
            ///- 64K (65536) cycles, ~0.5s
            #[inline(always)]
            pub fn is_cycles_64k(&self) -> bool {
                *self == WDPL_A::CYCLES_64K
            }
            ///- 128K (131072) cycles, ~1s
            #[inline(always)]
            pub fn is_cycles_128k(&self) -> bool {
                *self == WDPL_A::CYCLES_128K
            }
            ///- 256K (262144) cycles, ~2s
            #[inline(always)]
            pub fn is_cycles_256k(&self) -> bool {
                *self == WDPL_A::CYCLES_256K
            }
        }
        ///Field `WDPL` writer - Watchdog Timer Prescaler - Low Bits
        pub type WDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WDPL_A, crate::Safe>;
        impl<'a, REG> WDPL_W<'a, REG>
        where
            REG: crate::Writable + crate::RegisterSpec,
            REG::Ux: From<u8>,
        {
            ///- 2048 cycles, ~16ms/512K (524288) cycles, ~4s if WDPH is set
            #[inline(always)]
            pub fn cycles_2k_512k(self) -> &'a mut crate::W<REG> {
                self.variant(WDPL_A::CYCLES_2K_512K)
            }
            ///- 4096 cycles, ~32ms/1024K (1048576) cycles, ~8s if WDPH is set
            #[inline(always)]
            pub fn cycles_4k_1024k(self) -> &'a mut crate::W<REG> {
                self.variant(WDPL_A::CYCLES_4K_1024K)
            }
            ///- 8192 cycles, ~64ms
            #[inline(always)]
            pub fn cycles_8k(self) -> &'a mut crate::W<REG> {
                self.variant(WDPL_A::CYCLES_8K)
            }
            ///- 16K (16384) cycles, ~0.125s
            #[inline(always)]
            pub fn cycles_16k(self) -> &'a mut crate::W<REG> {
                self.variant(WDPL_A::CYCLES_16K)
            }
            ///- 32K (32768) cycles, ~0.25s
            #[inline(always)]
            pub fn cycles_32k(self) -> &'a mut crate::W<REG> {
                self.variant(WDPL_A::CYCLES_32K)
            }
            ///- 64K (65536) cycles, ~0.5s
            #[inline(always)]
            pub fn cycles_64k(self) -> &'a mut crate::W<REG> {
                self.variant(WDPL_A::CYCLES_64K)
            }
            ///- 128K (131072) cycles, ~1s
            #[inline(always)]
            pub fn cycles_128k(self) -> &'a mut crate::W<REG> {
                self.variant(WDPL_A::CYCLES_128K)
            }
            ///- 256K (262144) cycles, ~2s
            #[inline(always)]
            pub fn cycles_256k(self) -> &'a mut crate::W<REG> {
                self.variant(WDPL_A::CYCLES_256K)
            }
        }
        ///Field `WDE` reader - Watch Dog Enable
        pub type WDE_R = crate::BitReader;
        ///Field `WDE` writer - Watch Dog Enable
        pub type WDE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `WDCE` reader - Watchdog Change Enable
        pub type WDCE_R = crate::BitReader;
        ///Field `WDCE` writer - Watchdog Change Enable
        pub type WDCE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `WDPH` reader - Watchdog Timer Prescaler - High Bit
        pub type WDPH_R = crate::BitReader;
        ///Field `WDPH` writer - Watchdog Timer Prescaler - High Bit
        pub type WDPH_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `WDIE` reader - Watchdog Timeout Interrupt Enable
        pub type WDIE_R = crate::BitReader;
        ///Field `WDIE` writer - Watchdog Timeout Interrupt Enable
        pub type WDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
        ///Field `WDIF` reader - Watchdog Timeout Interrupt Flag
        pub type WDIF_R = crate::BitReader;
        ///Field `WDIF` writer - Watchdog Timeout Interrupt Flag
        pub type WDIF_W<'a, REG> = crate::BitWriter<'a, REG>;
        impl R {
            ///Bits 0:2 - Watchdog Timer Prescaler - Low Bits
            #[inline(always)]
            pub fn wdpl(&self) -> WDPL_R {
                WDPL_R::new(self.bits & 7)
            }
            ///Bit 3 - Watch Dog Enable
            #[inline(always)]
            pub fn wde(&self) -> WDE_R {
                WDE_R::new(((self.bits >> 3) & 1) != 0)
            }
            ///Bit 4 - Watchdog Change Enable
            #[inline(always)]
            pub fn wdce(&self) -> WDCE_R {
                WDCE_R::new(((self.bits >> 4) & 1) != 0)
            }
            ///Bit 5 - Watchdog Timer Prescaler - High Bit
            #[inline(always)]
            pub fn wdph(&self) -> WDPH_R {
                WDPH_R::new(((self.bits >> 5) & 1) != 0)
            }
            ///Bit 6 - Watchdog Timeout Interrupt Enable
            #[inline(always)]
            pub fn wdie(&self) -> WDIE_R {
                WDIE_R::new(((self.bits >> 6) & 1) != 0)
            }
            ///Bit 7 - Watchdog Timeout Interrupt Flag
            #[inline(always)]
            pub fn wdif(&self) -> WDIF_R {
                WDIF_R::new(((self.bits >> 7) & 1) != 0)
            }
        }
        impl W {
            ///Bits 0:2 - Watchdog Timer Prescaler - Low Bits
            #[inline(always)]
            pub fn wdpl(&mut self) -> WDPL_W<'_, WDTCSR_SPEC> {
                WDPL_W::new(self, 0)
            }
            ///Bit 3 - Watch Dog Enable
            #[inline(always)]
            pub fn wde(&mut self) -> WDE_W<'_, WDTCSR_SPEC> {
                WDE_W::new(self, 3)
            }
            ///Bit 4 - Watchdog Change Enable
            #[inline(always)]
            pub fn wdce(&mut self) -> WDCE_W<'_, WDTCSR_SPEC> {
                WDCE_W::new(self, 4)
            }
            ///Bit 5 - Watchdog Timer Prescaler - High Bit
            #[inline(always)]
            pub fn wdph(&mut self) -> WDPH_W<'_, WDTCSR_SPEC> {
                WDPH_W::new(self, 5)
            }
            ///Bit 6 - Watchdog Timeout Interrupt Enable
            #[inline(always)]
            pub fn wdie(&mut self) -> WDIE_W<'_, WDTCSR_SPEC> {
                WDIE_W::new(self, 6)
            }
            ///Bit 7 - Watchdog Timeout Interrupt Flag
            #[inline(always)]
            pub fn wdif(&mut self) -> WDIF_W<'_, WDTCSR_SPEC> {
                WDIF_W::new(self, 7)
            }
        }
        /**Watchdog Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`wdtcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
        pub struct WDTCSR_SPEC;
        impl crate::RegisterSpec for WDTCSR_SPEC {
            type Ux = u8;
        }
        ///`read()` method returns [`wdtcsr::R`](R) reader structure
        impl crate::Readable for WDTCSR_SPEC {}
        ///`write(|w| ..)` method takes [`wdtcsr::W`](W) writer structure
        impl crate::Writable for WDTCSR_SPEC {
            type Safety = crate::Unsafe;
        }
        ///`reset()` method sets WDTCSR to value 0
        impl crate::Resettable for WDTCSR_SPEC {}
    }
}
use super::DEVICE_PERIPHERALS;
/// All the peripherals.
#[allow(non_snake_case)]
pub struct Peripherals {
    ///AC
    pub AC: AC,
    ///ADC
    pub ADC: ADC,
    ///BOOT_LOAD
    pub BOOT_LOAD: BOOT_LOAD,
    ///CPU
    pub CPU: CPU,
    ///EEPROM
    pub EEPROM: EEPROM,
    ///EXINT
    pub EXINT: EXINT,
    ///FUSE
    pub FUSE: FUSE,
    ///JTAG
    pub JTAG: JTAG,
    ///LOCKBIT
    pub LOCKBIT: LOCKBIT,
    ///PORTA
    pub PORTA: PORTA,
    ///PORTB
    pub PORTB: PORTB,
    ///PORTC
    pub PORTC: PORTC,
    ///PORTD
    pub PORTD: PORTD,
    ///PORTE
    pub PORTE: PORTE,
    ///PORTF
    pub PORTF: PORTF,
    ///PORTG
    pub PORTG: PORTG,
    ///PORTH
    pub PORTH: PORTH,
    ///PORTJ
    pub PORTJ: PORTJ,
    ///PORTK
    pub PORTK: PORTK,
    ///PORTL
    pub PORTL: PORTL,
    ///SPI
    pub SPI: SPI,
    ///TC0
    pub TC0: TC0,
    ///TC1
    pub TC1: TC1,
    ///TC2
    pub TC2: TC2,
    ///TC3
    pub TC3: TC3,
    ///TC4
    pub TC4: TC4,
    ///TC5
    pub TC5: TC5,
    ///TWI
    pub TWI: TWI,
    ///USART0
    pub USART0: USART0,
    ///USART1
    pub USART1: USART1,
    ///USART2
    pub USART2: USART2,
    ///USART3
    pub USART3: USART3,
    ///WDT
    pub WDT: WDT,
}
impl Peripherals {
    /// Returns all the peripherals *once*.
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    /// Unchecked version of `Peripherals::take`.
    ///
    /// # Safety
    ///
    /// Each of the returned peripherals must be used at most once.
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AC: AC::steal(),
            ADC: ADC::steal(),
            BOOT_LOAD: BOOT_LOAD::steal(),
            CPU: CPU::steal(),
            EEPROM: EEPROM::steal(),
            EXINT: EXINT::steal(),
            FUSE: FUSE::steal(),
            JTAG: JTAG::steal(),
            LOCKBIT: LOCKBIT::steal(),
            PORTA: PORTA::steal(),
            PORTB: PORTB::steal(),
            PORTC: PORTC::steal(),
            PORTD: PORTD::steal(),
            PORTE: PORTE::steal(),
            PORTF: PORTF::steal(),
            PORTG: PORTG::steal(),
            PORTH: PORTH::steal(),
            PORTJ: PORTJ::steal(),
            PORTK: PORTK::steal(),
            PORTL: PORTL::steal(),
            SPI: SPI::steal(),
            TC0: TC0::steal(),
            TC1: TC1::steal(),
            TC2: TC2::steal(),
            TC3: TC3::steal(),
            TC4: TC4::steal(),
            TC5: TC5::steal(),
            TWI: TWI::steal(),
            USART0: USART0::steal(),
            USART1: USART1::steal(),
            USART2: USART2::steal(),
            USART3: USART3::steal(),
            WDT: WDT::steal(),
        }
    }
}
