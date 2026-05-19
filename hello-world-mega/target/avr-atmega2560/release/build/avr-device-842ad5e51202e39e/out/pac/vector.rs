#[doc(hidden)]
#[macro_export]
macro_rules! __avr_device_trampoline {
    (@atmega2560, RESET, $it:item) => {
        #[export_name = "__vector_0"]
        $it
    };
    (@atmega2560, INT0, $it:item) => {
        #[export_name = "__vector_1"]
        $it
    };
    (@atmega2560, INT1, $it:item) => {
        #[export_name = "__vector_2"]
        $it
    };
    (@atmega2560, INT2, $it:item) => {
        #[export_name = "__vector_3"]
        $it
    };
    (@atmega2560, INT3, $it:item) => {
        #[export_name = "__vector_4"]
        $it
    };
    (@atmega2560, INT4, $it:item) => {
        #[export_name = "__vector_5"]
        $it
    };
    (@atmega2560, INT5, $it:item) => {
        #[export_name = "__vector_6"]
        $it
    };
    (@atmega2560, INT6, $it:item) => {
        #[export_name = "__vector_7"]
        $it
    };
    (@atmega2560, INT7, $it:item) => {
        #[export_name = "__vector_8"]
        $it
    };
    (@atmega2560, PCINT0, $it:item) => {
        #[export_name = "__vector_9"]
        $it
    };
    (@atmega2560, PCINT1, $it:item) => {
        #[export_name = "__vector_10"]
        $it
    };
    (@atmega2560, PCINT2, $it:item) => {
        #[export_name = "__vector_11"]
        $it
    };
    (@atmega2560, WDT, $it:item) => {
        #[export_name = "__vector_12"]
        $it
    };
    (@atmega2560, TIMER2_COMPA, $it:item) => {
        #[export_name = "__vector_13"]
        $it
    };
    (@atmega2560, TIMER2_COMPB, $it:item) => {
        #[export_name = "__vector_14"]
        $it
    };
    (@atmega2560, TIMER2_OVF, $it:item) => {
        #[export_name = "__vector_15"]
        $it
    };
    (@atmega2560, TIMER1_CAPT, $it:item) => {
        #[export_name = "__vector_16"]
        $it
    };
    (@atmega2560, TIMER1_COMPA, $it:item) => {
        #[export_name = "__vector_17"]
        $it
    };
    (@atmega2560, TIMER1_COMPB, $it:item) => {
        #[export_name = "__vector_18"]
        $it
    };
    (@atmega2560, TIMER1_COMPC, $it:item) => {
        #[export_name = "__vector_19"]
        $it
    };
    (@atmega2560, TIMER1_OVF, $it:item) => {
        #[export_name = "__vector_20"]
        $it
    };
    (@atmega2560, TIMER0_COMPA, $it:item) => {
        #[export_name = "__vector_21"]
        $it
    };
    (@atmega2560, TIMER0_COMPB, $it:item) => {
        #[export_name = "__vector_22"]
        $it
    };
    (@atmega2560, TIMER0_OVF, $it:item) => {
        #[export_name = "__vector_23"]
        $it
    };
    (@atmega2560, SPI_STC, $it:item) => {
        #[export_name = "__vector_24"]
        $it
    };
    (@atmega2560, USART0_RX, $it:item) => {
        #[export_name = "__vector_25"]
        $it
    };
    (@atmega2560, USART0_UDRE, $it:item) => {
        #[export_name = "__vector_26"]
        $it
    };
    (@atmega2560, USART0_TX, $it:item) => {
        #[export_name = "__vector_27"]
        $it
    };
    (@atmega2560, ANALOG_COMP, $it:item) => {
        #[export_name = "__vector_28"]
        $it
    };
    (@atmega2560, ADC, $it:item) => {
        #[export_name = "__vector_29"]
        $it
    };
    (@atmega2560, EE_READY, $it:item) => {
        #[export_name = "__vector_30"]
        $it
    };
    (@atmega2560, TIMER3_CAPT, $it:item) => {
        #[export_name = "__vector_31"]
        $it
    };
    (@atmega2560, TIMER3_COMPA, $it:item) => {
        #[export_name = "__vector_32"]
        $it
    };
    (@atmega2560, TIMER3_COMPB, $it:item) => {
        #[export_name = "__vector_33"]
        $it
    };
    (@atmega2560, TIMER3_COMPC, $it:item) => {
        #[export_name = "__vector_34"]
        $it
    };
    (@atmega2560, TIMER3_OVF, $it:item) => {
        #[export_name = "__vector_35"]
        $it
    };
    (@atmega2560, USART1_RX, $it:item) => {
        #[export_name = "__vector_36"]
        $it
    };
    (@atmega2560, USART1_UDRE, $it:item) => {
        #[export_name = "__vector_37"]
        $it
    };
    (@atmega2560, USART1_TX, $it:item) => {
        #[export_name = "__vector_38"]
        $it
    };
    (@atmega2560, TWI, $it:item) => {
        #[export_name = "__vector_39"]
        $it
    };
    (@atmega2560, SPM_READY, $it:item) => {
        #[export_name = "__vector_40"]
        $it
    };
    (@atmega2560, TIMER4_CAPT, $it:item) => {
        #[export_name = "__vector_41"]
        $it
    };
    (@atmega2560, TIMER4_COMPA, $it:item) => {
        #[export_name = "__vector_42"]
        $it
    };
    (@atmega2560, TIMER4_COMPB, $it:item) => {
        #[export_name = "__vector_43"]
        $it
    };
    (@atmega2560, TIMER4_COMPC, $it:item) => {
        #[export_name = "__vector_44"]
        $it
    };
    (@atmega2560, TIMER4_OVF, $it:item) => {
        #[export_name = "__vector_45"]
        $it
    };
    (@atmega2560, TIMER5_CAPT, $it:item) => {
        #[export_name = "__vector_46"]
        $it
    };
    (@atmega2560, TIMER5_COMPA, $it:item) => {
        #[export_name = "__vector_47"]
        $it
    };
    (@atmega2560, TIMER5_COMPB, $it:item) => {
        #[export_name = "__vector_48"]
        $it
    };
    (@atmega2560, TIMER5_COMPC, $it:item) => {
        #[export_name = "__vector_49"]
        $it
    };
    (@atmega2560, TIMER5_OVF, $it:item) => {
        #[export_name = "__vector_50"]
        $it
    };
    (@atmega2560, USART2_RX, $it:item) => {
        #[export_name = "__vector_51"]
        $it
    };
    (@atmega2560, USART2_UDRE, $it:item) => {
        #[export_name = "__vector_52"]
        $it
    };
    (@atmega2560, USART2_TX, $it:item) => {
        #[export_name = "__vector_53"]
        $it
    };
    (@atmega2560, USART3_RX, $it:item) => {
        #[export_name = "__vector_54"]
        $it
    };
    (@atmega2560, USART3_UDRE, $it:item) => {
        #[export_name = "__vector_55"]
        $it
    };
    (@atmega2560, USART3_TX, $it:item) => {
        #[export_name = "__vector_56"]
        $it
    };
    (@$mcu:ident, $name:ident, $it:item) => {
        compile_error!(concat!("Couldn't find interrupt ", stringify!($name), ", for MCU ", stringify!($mcu), "."));
    }
}
