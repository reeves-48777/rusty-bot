use super::super::core::infos::Info;

pub trait Expandable<Loggable=Self> {
	fn share_infos(&self) -> Info<Loggable>;
}