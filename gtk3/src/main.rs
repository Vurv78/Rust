use gtk::glib;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, MessageDialog};

fn build_ui(application: &gtk::Application) {
	let glade_src = include_str!("main.ui");
	let builder = Builder::from_string(glade_src);

	let window: ApplicationWindow = builder.object("winMain").expect("Couldn't get winMain");
	window.set_application(Some(application));
	window.show_all();
}

fn main() {
	let application = gtk::Application::new(
		Some("autorun.rs"),
		Default::default(),
	);

	application.connect_activate(build_ui);

	application.run();
}