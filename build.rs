fn main()
{
	match std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str()
	{
		"windows"=>
		{
			println!("cargo:rustc-link-arg=/ENTRY:msvc_start");
			println!("cargo:rustc-link-arg=/SUBSYSTEM:CONSOLE");
			println!("cargo:rustc-link-lib=kernel32");
			println!("cargo:rustc-link-lib=ntdllp");
		}
		"linux"=>
		{
			println!("cargo:rustc-link-arg=-e");
			println!("cargo:rustc-link-arg=linux_start");
		}
		target_os=>
		{
			println!("cargo:error=\"Unsupport target OS: {target_os}!\"")
		}
	}
}