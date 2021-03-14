use glib::types::Type;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    prelude::TreeStoreExtManual, BoxExt, CellLayoutExt, ComboBox, ComboBoxExt, ContainerExt,
    GtkWindowExt, Inhibit, TreeModelExt, TreeStoreExt, WidgetExt, Window, WindowType,
};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use units::{length, mass, UnitType};

mod units;

struct Converter {
    unit_types: Vec<UnitType>,
    from_unit_type_idx: usize,
    from_unit_idx: usize,
    to_unit_idx: usize,
}

#[derive(Msg)]
enum Msg {
    FromComboChanged,
    Quit,
    ToComboChanged,
}

struct Win {
    from_combo: ComboBox,
    model: Converter,
    to_combo: ComboBox,
    window: Window,
}

impl Update for Win {
    type Model = Converter;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Converter {
        Converter {
            unit_types: vec![length::init(), mass::init()],
            from_unit_type_idx: 0,
            from_unit_idx: 0,
            to_unit_idx: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::FromComboChanged => {
                let iter = self.from_combo.get_active_iter().unwrap();
                let from_model = self.from_combo.get_model().unwrap();

                let val1 = from_model.get_value(&iter, 1);
                let from_unit_type_idx: u64 = val1.get().unwrap().unwrap();
                let val2 = from_model.get_value(&iter, 2);
                let from_unit_idx: u64 = val2.get().unwrap().unwrap();

                self.model.from_unit_type_idx = from_unit_type_idx as usize;
                self.model.from_unit_idx = from_unit_idx as usize;
                self.model.to_unit_idx = 0;
                let to_model = create_to_store(&self.model);
                self.to_combo.set_model(Some(&to_model));
            }
            Msg::Quit => gtk::main_quit(),
            Msg::ToComboChanged => {
                let iter = self.from_combo.get_active_iter().unwrap();
                let from_model = self.from_combo.get_model().unwrap();

                let val1 = from_model.get_value(&iter, 1);
                let to_unit_idx: u64 = val1.get().unwrap().unwrap();

                self.model.to_unit_idx = to_unit_idx as usize;
            }
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = gtk::Window::new(WindowType::Toplevel);
        let vbox = gtk::Box::new(Vertical, 10);
        let hbox = gtk::Box::new(Horizontal, 10);
        let cell = gtk::CellRendererText::new();

        window.set_title("Converter");
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(550, 300);

        let from_store = create_from_store(&model);
        let from_combo = ComboBox::with_model(&from_store);
        from_combo.set_entry_text_column(0);
        from_combo.pack_start(&cell, true);
        from_combo.add_attribute(&cell, "text", 0);
        hbox.pack_start(&from_combo, true, true, 0);

        let to_store = create_to_store(&model);
        let to_combo = ComboBox::with_model(&to_store);
        to_combo.set_entry_text_column(0);
        to_combo.pack_start(&cell, true);
        to_combo.add_attribute(&cell, "text", 0);
        hbox.pack_start(&to_combo, true, true, 0);

        vbox.pack_start(&hbox, false, true, 0);
        window.add(&vbox);

        window.show_all();

        connect!(relm, from_combo, connect_changed(_), Msg::FromComboChanged);
        connect!(relm, to_combo, connect_changed(_), Msg::ToComboChanged);
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Win {
            from_combo,
            model,
            to_combo,
            window,
        }
    }
}

fn create_from_store(model: &Converter) -> gtk::TreeStore {
    let store = gtk::TreeStore::new(&[Type::String, Type::U64, Type::U64]);

    for (unit_type_idx, unit_type) in model.unit_types.iter().enumerate() {
        let top = store.append(None);
        store.set(&top, &[0], &[&unit_type.name]);

        for (unit_idx, unit) in unit_type.units.iter().enumerate() {
            let entries = store.append(Some(&top));
            store.set(&entries, &[0], &[&unit.name]);
            store.set(&entries, &[1], &[&(unit_type_idx as u64)]);
            store.set(&entries, &[2], &[&(unit_idx as u64)]);
        }
    }
    store
}

fn create_to_store(model: &Converter) -> gtk::TreeStore {
    let store = gtk::TreeStore::new(&[Type::String, Type::U64]);
    let from_unit_type: &UnitType = &model.unit_types[model.from_unit_type_idx];

    for (unit_idx, unit) in from_unit_type.units.iter().enumerate() {
        let entries = store.append(None);
        store.set(&entries, &[0], &[&unit.name]);
        store.set(&entries, &[1], &[&(unit_idx as u64)]);
    }
    store
}

fn main() {
    let lengths_units = length::init();
    dbg!(lengths_units.name);
    dbg!(lengths_units.units[0].convert(50.0, &lengths_units.units[2]));
    Win::run(()).expect("Win::run failed");
}
