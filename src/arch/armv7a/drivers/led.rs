use arch::io::mmio;


const GPIO_BASE       :*mut usize = 0x3F200000 as *mut usize;

const LED_GPFSEL      :isize = GPIO_GPFSEL4;
const LED_GPFBIT      :isize = 21;
const LED_GPSET       :isize = GPIO_GPSET1;
const LED_GPCLR       :isize = GPIO_GPCLR1;
const LED_GPIO_BIT    :isize = 15 ;

const GPIO_GPFSEL0    :isize = 0  ;
const GPIO_GPFSEL1    :isize = 1  ;
const GPIO_GPFSEL2    :isize = 2  ;
const GPIO_GPFSEL3    :isize = 3  ;
const GPIO_GPFSEL4    :isize = 4  ;
const GPIO_GPFSEL5    :isize = 5  ;

const GPIO_GPSET0     :isize = 7  ;
const GPIO_GPSET1     :isize = 8  ;

const GPIO_GPCLR0     :isize = 10 ;
const GPIO_GPCLR1     :isize = 11 ;

const GPIO_GPLEV0     :isize = 13 ;
const GPIO_GPLEV1     :isize = 14 ;

const GPIO_GPEDS0     :isize = 16 ;
const GPIO_GPEDS1     :isize = 17 ;

const GPIO_GPREN0     :isize = 19 ;
const GPIO_GPREN1     :isize = 20 ;

const GPIO_GPFEN0     :isize = 22 ;
const GPIO_GPFEN1     :isize = 23 ;

const GPIO_GPHEN0     :isize = 25 ;
const GPIO_GPHEN1     :isize = 26 ;

const GPIO_GPLEN0     :isize = 28 ;
const GPIO_GPLEN1     :isize = 29 ;

const GPIO_GPAREN0    :isize = 31 ;
const GPIO_GPAREN1    :isize = 32 ;

const GPIO_GPAFEN0    :isize = 34 ;
const GPIO_GPAFEN1    :isize = 35 ;

const GPIO_GPPUD      :isize = 37 ;
const GPIO_GPPUDCLK0  :isize = 38 ;
const GPIO_GPPUDCLK1  :isize = 39 ;

pub fn blink()
{
    #[cfg(debug_assertions)]
    const DELAY: usize = 50_000;

    #[cfg(not(debug_assertions))]
    const DELAY: usize = 500_000;

    unsafe {
    /* Assign the address of the GPIO peripheral (Using ARM Physical Address) */
    let gpio = GPIO_BASE;

    /* Write 1 to the GPIO16 init nibble in the Function Select 1 GPIO
       peripheral register to enable GPIO16 as an output */
    *gpio.offset(LED_GPFSEL) |= 1 << LED_GPFBIT;

    /* Never exit as there is no OS to exit to! */
    loop
    {
        mmio::delay(DELAY);

        /* Set the LED GPIO pin low ( Turn OK LED on for original Pi, and off
           for plus models )*/
        mmio::write(gpio.offset(LED_GPCLR), 1 << LED_GPIO_BIT);

        mmio::delay(DELAY);

        /* Set the LED GPIO pin high ( Turn OK LED off for original Pi, and on
           for plus models )*/
        mmio::write(gpio.offset(LED_GPSET), 1 << LED_GPIO_BIT);
    }
    }
}
