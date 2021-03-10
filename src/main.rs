use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use units::lengths;

mod units;

fn make_window(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    let button = gtk::Button::with_label("Close");
    button.connect_clicked(clone!(@weak window => move |_| window.close()));
    hbox.add(&button);

    let store = gtk::TreeStore::new(&[glib::Type::String]);
    store.insert_with_values(None, None, &[0], &[&"One"]);
    store.insert_with_values(None, None, &[0], &[&"Two"]);
    store.insert_with_values(None, None, &[0], &[&"Three"]);
    store.insert_with_values(None, None, &[0], &[&"Four"]);
    // let treeview = gtk::TreeModel::with_model(&store);
    let combo_box = gtk::ComboBox::with_model(&store);
    hbox.add(&combo_box);

    window.add(&hbox);
    window.show_all();
}

fn on_activate(application: &gtk::Application) {
    make_window(application)
}

fn main() {
    // Create a new application
    let app = gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
        .expect("Initialization failed...");

    app.connect_activate(|app| on_activate(app));

    let lengths_units = lengths::init();
    dbg!(lengths_units.name);
    dbg!(lengths_units.units[0].convert(50.0, &lengths_units.units[2]));

    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}
