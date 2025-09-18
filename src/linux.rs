// Linux runtime

use core::fmt;

use libc::{exit, STDOUT_FILENO};
use static_collections::string::StaticString;

use crate::main;

pub fn internal_print(args:fmt::Arguments)
{
	let mut w:StaticString<1024>=StaticString::new();
	if fmt::write(&mut w,args).is_ok()
	{
		// Not-sure about correctness. Is it okay to just write UTF-8 string to stdout?
		unsafe
		{
			libc::write(STDOUT_FILENO,w.as_str().as_ptr().cast(),w.len());
		}
	}
}

#[macro_export]
macro_rules! print
{
	($($arg:tt)*)=>
	{
		$crate::linux::internal_print(format_args!($($arg)*))
	};
}

#[unsafe(no_mangle)] extern "C" fn linux_start()
{
	main();
	unsafe
	{
		exit(0);
	}
}

#[unsafe(no_mangle)] extern "C" fn rust_eh_personality()
{
	
}