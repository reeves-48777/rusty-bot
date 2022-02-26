/// Impl this trait for the Plugins you want to add to the bot
/// Each Plugin has to be a singleton
pub trait Plugin {
	/// returns the instance of the plugin
	fn get_instance(&self) -> dyn Plugin;
	/// runs the plugin instance
	fn run(&self);
}