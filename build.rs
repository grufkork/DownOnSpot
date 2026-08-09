fn main() {
	#[cfg(windows)]
	{
		winres::WindowsResource::new().compile().unwrap();
	}
}
