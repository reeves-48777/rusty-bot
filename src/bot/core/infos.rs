use std::{
	rc::Rc,
	fmt::Display
};
pub struct Info<T> {
	info_src: Rc<T>,
}

impl<T> Info<T> 
where T: Display {
	pub fn new(src: T) -> Self{
		Self {
			info_src: Rc::new(src)
		}
	}
	pub fn display_infos(&self) {
		println!("{}", self.info_src)
	}
}
