const GPIO_BASE       :*mut usize = 0x3F200000 as *mut usize;

//const LED_GPFSEL      :*const usize = GPIO_GPFSEL4 as *const usize;
//const LED_GPFBIT      :*const usize = 21 as *const usize;
//const LED_GPSET       :*const usize = GPIO_GPSET1 as *const usize;
//const LED_GPCLR       :*const usize = GPIO_GPCLR1 as *const usize;
//const LED_GPIO_BIT    :*const usize = 15 as *const usize;
//
//const GPIO_GPFSEL0    :*const usize = 0  as *const usize;
//const GPIO_GPFSEL1    :*const usize = 1  as *const usize;
//const GPIO_GPFSEL2    :*const usize = 2  as *const usize;
//const GPIO_GPFSEL3    :*const usize = 3  as *const usize;
//const GPIO_GPFSEL4    :*const usize = 4  as *const usize;
//const GPIO_GPFSEL5    :*const usize = 5  as *const usize;
//
//const GPIO_GPSET0     :*const usize = 7  as *const usize;
//const GPIO_GPSET1     :*const usize = 8  as *const usize;
//
//const GPIO_GPCLR0     :*const usize = 10 as *const usize;
//const GPIO_GPCLR1     :*const usize = 11 as *const usize;
//
//const GPIO_GPLEV0     :*const usize = 13 as *const usize;
//const GPIO_GPLEV1     :*const usize = 14 as *const usize;
//
//const GPIO_GPEDS0     :*const usize = 16 as *const usize;
//const GPIO_GPEDS1     :*const usize = 17 as *const usize;
//
//const GPIO_GPREN0     :*const usize = 19 as *const usize;
//const GPIO_GPREN1     :*const usize = 20 as *const usize;
//
//const GPIO_GPFEN0     :*const usize = 22 as *const usize;
//const GPIO_GPFEN1     :*const usize = 23 as *const usize;
//
//const GPIO_GPHEN0     :*const usize = 25 as *const usize;
//const GPIO_GPHEN1     :*const usize = 26 as *const usize;
//
//const GPIO_GPLEN0     :*const usize = 28 as *const usize;
//const GPIO_GPLEN1     :*const usize = 29 as *const usize;
//
//const GPIO_GPAREN0    :*const usize = 31 as *const usize;
//const GPIO_GPAREN1    :*const usize = 32 as *const usize;
//
//const GPIO_GPAFEN0    :*const usize = 34 as *const usize;
//const GPIO_GPAFEN1    :*const usize = 35 as *const usize;
//
//const GPIO_GPPUD      :*const usize = 37 as *const usize;
//const GPIO_GPPUDCLK0  :*const usize = 38 as *const usize;
//const GPIO_GPPUDCLK1  :*const usize = 39 as *const usize;

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
    unsafe {
    /* Assign the address of the GPIO peripheral (Using ARM Physical Address) */
    let gpio = GPIO_BASE;

    /* Write 1 to the GPIO16 init nibble in the Function Select 1 GPIO
       peripheral register to enable GPIO16 as an output */
    *gpio.offset(LED_GPFSEL) |= (1 << LED_GPFBIT);

    /* Never exit as there is no OS to exit to! */
    loop
    {
        for i in 0..50_000{;}

        /* Set the LED GPIO pin low ( Turn OK LED on for original Pi, and off
           for plus models )*/
        *gpio.offset(LED_GPCLR) = (1 << LED_GPIO_BIT);

        for i in 0..50_000{;}

        /* Set the LED GPIO pin high ( Turn OK LED off for original Pi, and on
           for plus models )*/
        *gpio.offset(LED_GPSET) = (1 << LED_GPIO_BIT);
    }
    }
}
