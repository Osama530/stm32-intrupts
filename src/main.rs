//GPIO and interupts
#![no_std]
#![no_main]

#[macro_use]

#[allow(unused_imports)]
extern crate panic_semihosting ;
extern crate cortex_m_rt;
extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate stm32f3;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f3::stm32f303;

// use `main` as the entry point of this application
// `main` is not allowed to return

#[entry]
fn main() ->! {
    //initializing peripherals
    let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
    //calling rcc from library
    let rcc = peripherals.RCC;
    let gpioa = peripherals.GPIOA;
    
    
    //let interrupt = stm32f3::stm32f303::interrupt;
    
    
//********RCC Peripheral: Enable GPIOA and SYSCFG clocks********/  
    //enabling input/output port
    rcc.ahbenr.write(|w|
        w 
            .iopaen().set_bit()
    );

    //enabling system clock 
    rcc.apb2enr.write(|w|
        w 
            .syscfgen().set_bit()
    );

//********GPIOA: Configure PA0 pin as an input in pull-down mode********/    
   
    //mode selection
    gpioa.moder.write(|w|
    w
        .moder0().bits(0b00) //0b00 = intput
    );

    //input type selection
    gpioa.otyper.write(|w|
        w
            .ot0().clear_bit() //pushpull
    );

    //input speed type selection
    gpioa.ospeedr.write(|w|
        w
            .ospeedr0().bits(0b00) //low
    );

    //pullup/pulldown selection
    gpioa.pupdr.write(|w| unsafe { 
    w
        .pupdr0().bits(0b10) // pull down
    });

//********SYSCFG: Connect EXTI0 line to PA0 pin********/ 
    //initializing
    let syscfg = peripherals.SYSCFG;
    //seting pin A0 t0 intrupt
    syscfg.exticr1.write(|w| unsafe{
    w
        .exti0().bits(0b0000)});

//***********EXTI: Configure EXTI0 line***************/
    //initializing
    let exti = peripherals.EXTI;
    //external interupt masking enable
    exti.imr1.write(|w|
    w
        .mr0().set_bit() );

    //external interupt falling edge selection
    exti.ftsr1.write(|w|
        w
            .tr0().set_bit() );

    //external interupt rising edge disable
    exti.rtsr1.write(|w|
        w
            .tr0().clear_bit() );

//***********Move shared state into mutexes (to prevent rece condition)***************/
//no need for now

//******NVIC: Enable EXTI0 interrupt line and enter main loop*****/
    //initializing
    let cortexm_peripharals = cortex_m::Peripherals::take().unwrap();
    let mut nvic = cortexm_peripharals.NVIC;

    //enabling nvic
    nvic.enable(stm32f3::stm32f303::Interrupt::EXTI0);
    loop {

            
        }    

            
}
// 8. Handle interrupt
use stm32f303::interrupt;
//use stm32f303::Peripherals;

#[interrupt]
fn EXTI0() {
    let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
    let exti =peripherals.EXTI;
    // clear the EXTI line 0 pending bit
    exti.pr1.modify(|_, w| w.pr0().set_bit());
    hprintln!("hellow interupt");
}
