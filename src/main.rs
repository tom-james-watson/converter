use std::num::ParseFloatError;

use glib::types::Type;
use gtk::{
    prelude::ComboBoxExtManual,
    EditableSignals, EntryExt, GtkWindowExt, LabelExt,
    Orientation::{Horizontal, Vertical},
};
use gtk::{
    prelude::TreeStoreExtManual, BoxExt, CellLayoutExt, ComboBox, ComboBoxExt, ContainerExt, Entry,
    Inhibit, Label, TreeModelExt, TreeStoreExt, WidgetExt, Window, WindowType,
};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use units::{length, mass, UnitType};

mod units;

struct Converter {
    unit_types: Vec<UnitType>,
    from_unit_type_idx: usize,
    from_unit_idx: usize,
    from_value: Option<Result<f64, ParseFloatError>>,
    to_unit_idx: usize,
}

#[derive(Msg)]
enum Msg {
    FromComboChanged,
    FromEntryChanged,
    Quit,
    ToComboChanged,
}

struct Win {
    from_combo: ComboBox,
    from_entry: Entry,
    model: Converter,
    to_combo: ComboBox,
    to_output: Label,
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
            from_value: None,
            to_unit_idx: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::FromComboChanged => {
                let iter = self.from_combo.get_active_iter().unwrap();
                let from_store = self.from_combo.get_model().unwrap();

                let val1 = from_store.get_value(&iter, 1);
                let from_unit_type_idx: u64 = val1.get().unwrap().unwrap();
                let val2 = from_store.get_value(&iter, 2);
                let from_unit_idx: u64 = val2.get().unwrap().unwrap();

                self.model.from_unit_type_idx = from_unit_type_idx as usize;
                self.model.from_unit_idx = from_unit_idx as usize;
                self.model.to_unit_idx = 0;
                let to_store = Self::create_to_store(&self.model);
                self.to_combo.set_model(Some(&to_store));
                self.to_combo
                    .set_active_iter(Some(&to_store.get_iter_first().unwrap()));
                self.write_output();
            }
            Msg::FromEntryChanged => {
                let from_value = self.from_entry.get_text();
                self.model.from_value = if from_value.eq("") {
                    None
                } else {
                    Some(from_value.parse::<f64>())
                };
                self.write_output();
            }
            Msg::Quit => gtk::main_quit(),
            Msg::ToComboChanged => {
                let iter = self.to_combo.get_active_iter().unwrap();
                let from_model = self.to_combo.get_model().unwrap();

                let val1 = from_model.get_value(&iter, 1);
                let to_unit_idx: u64 = val1.get().unwrap().unwrap();

                self.model.to_unit_idx = to_unit_idx as usize;
                self.write_output();
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

        let from_entry = gtk::Entry::new();
        hbox.pack_start(&from_entry, true, true, 0);

        let from_store = Self::create_from_store(&model);
        let from_combo = ComboBox::with_model(&from_store);
        from_combo.set_entry_text_column(0);
        from_combo.pack_start(&cell, true);
        from_combo.add_attribute(&cell, "text", 0);
        from_combo.set_active(Some(0));
        from_combo.set_active_iter(Some(
            &from_store
                .iter_nth_child(Some(&from_store.get_iter_first().unwrap()), 0)
                .unwrap(),
        ));
        hbox.pack_start(&from_combo, true, true, 0);

        let to_store = Self::create_to_store(&model);
        let to_combo = ComboBox::with_model(&to_store);
        to_combo.set_entry_text_column(0);
        to_combo.pack_start(&cell, true);
        to_combo.add_attribute(&cell, "text", 0);
        to_combo.set_active_iter(Some(&to_store.get_iter_first().unwrap()));
        hbox.pack_start(&to_combo, true, true, 0);

        let to_output = gtk::Label::new(None);
        hbox.pack_start(&to_output, true, true, 0);

        vbox.pack_start(&hbox, false, true, 0);
        window.add(&vbox);

        window.show_all();

        connect!(relm, from_entry, connect_changed(_), Msg::FromEntryChanged);
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
            from_entry,
            model,
            to_combo,
            to_output,
            window,
        }
    }
}

impl Win {
    fn create_from_store(model: &Converter) -> gtk::TreeStore {
        let store = gtk::TreeStore::new(&[Type::String, Type::U64, Type::U64]);

        for (unit_type_idx, unit_type) in model.unit_types.iter().enumerate() {
            let top = store.append(None);
            store.set(&top, &[0], &[&unit_type.name]);

            for (unit_idx, unit) in unit_type.units.iter().enumerate() {
                let entries = store.append(Some(&top));
                store.set(&entries, &[0], &[&unit.get_title()]);
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
            store.set(&entries, &[0], &[&unit.get_title()]);
            store.set(&entries, &[1], &[&(unit_idx as u64)]);
        }
        store
    }

    fn write_output(&self) {
        let output_text: String = match self.model.from_value {
            None => String::from(""),
            Some(Ok(v)) => {
                let units = &self.model.unit_types[self.model.from_unit_type_idx].units;
                units[self.model.from_unit_idx]
                    .convert(v, &units[self.model.to_unit_idx])
                    .to_string()
            }
            Some(Err(_)) => String::from("test"),
        };
        self.to_output.set_text(&output_text);
    }
}

fn main() {
    let lengths_units = length::init();
    dbg!(lengths_units.name);
    dbg!(lengths_units.units[0].convert(50.0, &lengths_units.units[2]));
    Win::run(()).expect("Win::run failed");
}
