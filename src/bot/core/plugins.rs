use std::fmt::Display;

use super::super::traits::Plugin;
use super::super::core::infos::Info;

pub struct CommonPlugin;
impl Plugin for CommonPlugin {
	fn run(&self) {
		println!("Hello world");
	}
}

pub struct InfoPlugin<T> {
	infos: Info<T>,
}

impl<T> InfoPlugin<T>
where T: Display {
	fn new(src: T) -> Self {
		Self {
			infos: Info::new(src)
		}
	}
}
impl<T> Plugin for InfoPlugin<T> {
	fn run(&self) {
		println!("Informations::::::")
	}
}
