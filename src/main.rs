#![no_std]
#![no_main]

#[cfg(windows)] mod win;

fn main()
{
	println!("Hello world!");
}

#[cfg(not(test))]
mod panicking
{
    use core::panic::PanicInfo;

    use crate::println;

	#[panic_handler] fn panic(info:&PanicInfo)->!
	{
		println!("{info}");
		loop{}
	}
}