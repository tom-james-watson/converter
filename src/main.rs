use std::num::ParseFloatError;

use glib::types::Type;
use gtk::{
    prelude::ComboBoxExtManual,
    EditableSignals, EntryExt, GridExt, GtkWindowExt, OrientableExt,
    Orientation::{Horizontal, Vertical},
};
use gtk::{
    prelude::TreeStoreExtManual, BoxExt, CellLayoutExt, ComboBox, ComboBoxExt, ContainerExt, Entry,
    Inhibit, TreeModelExt, TreeStoreExt, WidgetExt, Window, WindowType,
};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use units::{length, mass, UnitType};

mod units;

const SPACING: u32 = 10;

struct Converter {
    unit_types: Vec<UnitType>,
    from_unit_type_idx: Option<usize>,
    from_unit_idx: Option<usize>,
    from_value: Option<Result<f64, ParseFloatError>>,
    to_unit_idx: Option<usize>,
    to_value: Option<Result<f64, ParseFloatError>>,
}

#[derive(Msg)]
enum Msg {
    FromComboChanged,
    FromEntryChanged,
    Quit,
    ToComboChanged,
    ToEntryChanged,
}

struct Win {
    from_combo: ComboBox,
    from_entry: Entry,
    model: Converter,
    to_combo: ComboBox,
    to_entry: Entry,
    window: Window,
}

impl Update for Win {
    type Model = Converter;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Converter {
        Converter {
            unit_types: vec![length::init(), mass::init()],
            from_unit_type_idx: None,
            from_unit_idx: None,
            from_value: None,
            to_unit_idx: None,
            to_value: None,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::FromComboChanged => {
                let iter = self.from_combo.get_active_iter().unwrap();
                let from_model = self.from_combo.get_model().unwrap();

                let val0 = from_model.get_value(&iter, 0);
                let from_title: &str = val0.get().unwrap().unwrap();

                if from_title.eq("") {
                    self.model.from_unit_type_idx = None;
                    self.model.from_unit_idx = None;
                    self.model.to_unit_idx = None;

                    self.model.to_unit_idx = None;
                    let to_model = Self::create_to_model(&self.model);
                    self.to_combo.set_model(Some(&to_model));
                    self.to_combo
                        .set_active_iter(Some(&to_model.get_iter_first().unwrap()));
                } else {
                    let val1 = from_model.get_value(&iter, 1);
                    let from_unit_type_idx: u64 = val1.get().unwrap().unwrap();
                    let val2 = from_model.get_value(&iter, 2);
                    let from_unit_idx: u64 = val2.get().unwrap().unwrap();
                    let current_from_unit_type_idx = self.model.from_unit_type_idx;

                    self.model.from_unit_type_idx = Some(from_unit_type_idx as usize);
                    self.model.from_unit_idx = Some(from_unit_idx as usize);

                    if current_from_unit_type_idx != self.model.from_unit_type_idx {
                        self.model.to_unit_idx = None;
                        let to_model = Self::create_to_model(&self.model);
                        self.to_combo.set_model(Some(&to_model));
                        self.to_combo
                            .set_active_iter(Some(&to_model.get_iter_first().unwrap()));
                    }
                }

                self.write_to_value();
            }
            Msg::FromEntryChanged => {
                let from_value = self.from_entry.get_text();
                dbg!(&from_value);
                self.model.from_value = if from_value.eq("") {
                    None
                } else {
                    Some(from_value.parse::<f64>())
                };
                self.write_to_value();
            }
            Msg::Quit => gtk::main_quit(),
            Msg::ToComboChanged => {
                let iter = self.to_combo.get_active_iter().unwrap();
                let to_model = self.to_combo.get_model().unwrap();

                let val0 = to_model.get_value(&iter, 0);
                let to_title: &str = val0.get().unwrap().unwrap();

                if to_title.eq("") {
                    self.model.to_unit_idx = None;
                } else {
                    let val1 = to_model.get_value(&iter, 1);
                    let to_unit_idx: u64 = val1.get().unwrap().unwrap();

                    self.model.to_unit_idx = Some(to_unit_idx as usize);
                }
                self.write_to_value();
            }
            Msg::ToEntryChanged => {
                let to_value = self.to_entry.get_text();
                dbg!(&to_value);
                self.model.to_value = if to_value.eq("") {
                    None
                } else {
                    Some(to_value.parse::<f64>())
                };
                self.write_from_value();
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
        let vbox = gtk::Box::new(Vertical, SPACING as i32);
        let grid = gtk::Grid::new();
        grid.set_orientation(Horizontal);
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);
        grid.set_row_spacing(SPACING);
        grid.set_column_spacing(SPACING);
        let cell = gtk::CellRendererText::new();

        window.set_title("Converter");
        window.set_border_width(SPACING);
        window.set_position(gtk::WindowPosition::Center);
        // window.set_default_size(550, 300);

        let from_entry = gtk::Entry::new();
        grid.attach(&from_entry, 1, 1, 1, 1);

        let from_model = Self::create_from_model(&model);
        let from_combo = ComboBox::with_model(&from_model);
        from_combo.set_entry_text_column(0);
        from_combo.pack_start(&cell, true);
        from_combo.add_attribute(&cell, "text", 0);
        from_combo.set_active(Some(0));
        from_combo.set_active_iter(Some(&from_model.get_iter_first().unwrap()));
        grid.attach(&from_combo, 2, 1, 1, 1);

        let to_model = Self::create_to_model(&model);
        let to_combo = ComboBox::with_model(&to_model);
        to_combo.set_entry_text_column(0);
        to_combo.pack_start(&cell, true);
        to_combo.add_attribute(&cell, "text", 0);
        to_combo.set_active_iter(Some(&to_model.get_iter_first().unwrap()));
        grid.attach(&to_combo, 3, 1, 1, 1);

        let to_entry = gtk::Entry::new();
        grid.attach(&to_entry, 4, 1, 1, 1);

        vbox.pack_start(&grid, false, true, 0);
        window.add(&vbox);

        window.show_all();

        connect!(relm, from_entry, connect_changed(_), Msg::FromEntryChanged);
        connect!(relm, from_combo, connect_changed(_), Msg::FromComboChanged);
        connect!(relm, to_combo, connect_changed(_), Msg::ToComboChanged);
        connect!(relm, to_entry, connect_changed(_), Msg::ToEntryChanged);
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
            to_entry,
            window,
        }
    }
}

impl Win {
    fn create_from_model(model: &Converter) -> gtk::TreeStore {
        let store = gtk::TreeStore::new(&[Type::String, Type::U64, Type::U64]);

        let empty_row = store.append(None);
        store.set(&empty_row, &[0], &[&""]);

        for (unit_type_idx, unit_type) in model.unit_types.iter().enumerate() {
            let unit_type_row = store.append(None);
            store.set(&unit_type_row, &[0], &[&unit_type.name]);

            for (unit_idx, unit) in unit_type.units.iter().enumerate() {
                let unit_row = store.append(Some(&unit_type_row));
                store.set(&unit_row, &[0], &[&unit.get_title()]);
                store.set(&unit_row, &[1], &[&(unit_type_idx as u64)]);
                store.set(&unit_row, &[2], &[&(unit_idx as u64)]);
            }
        }
        store
    }

    fn create_to_model(model: &Converter) -> gtk::TreeStore {
        let store = gtk::TreeStore::new(&[Type::String, Type::U64]);

        let empty_row = store.append(None);
        store.set(&empty_row, &[0], &[&""]);

        match model.from_unit_type_idx {
            None => {}
            Some(from_unit_type_idx) => {
                let from_unit_type: &UnitType = &model.unit_types[from_unit_type_idx];

                for (unit_idx, unit) in from_unit_type.units.iter().enumerate() {
                    let unit_row = store.append(None);
                    store.set(&unit_row, &[0], &[&unit.get_title()]);
                    store.set(&unit_row, &[1], &[&(unit_idx as u64)]);
                }
            }
        }
        store
    }

    fn get_to_value(&self) -> String {
        if self.model.from_unit_type_idx == None
            || self.model.from_unit_idx == None
            || self.model.to_unit_idx == None
            || self.model.from_value == None
        {
            return String::from("");
        }

        match self.model.from_value {
            None => String::from(""),
            Some(Ok(v)) => {
                let units = &self.model.unit_types[self.model.from_unit_type_idx.unwrap()].units;
                units[self.model.from_unit_idx.unwrap()]
                    .convert_as_string(v, &units[self.model.to_unit_idx.unwrap()])
            }
            Some(Err(_)) => String::from("test"),
        }
    }

    fn write_to_value(&self) {
        let current_to_value = self.to_entry.get_text();
        let to_value = &self.get_to_value();
        dbg!(&current_to_value);
        dbg!(&to_value);
        dbg!(current_to_value.eq(to_value));

        if current_to_value.parse::<f64>() != to_value.parse::<f64>() {
            self.to_entry.set_text(to_value);
        }
    }

    fn get_from_value(&self) -> String {
        if self.model.from_unit_type_idx == None
            || self.model.from_unit_idx == None
            || self.model.to_unit_idx == None
            || self.model.to_value == None
        {
            return String::from("");
        }

        match self.model.to_value {
            None => String::from(""),
            Some(Ok(v)) => {
                let units = &self.model.unit_types[self.model.from_unit_type_idx.unwrap()].units;
                units[self.model.to_unit_idx.unwrap()]
                    .convert_as_string(v, &units[self.model.from_unit_idx.unwrap()])
            }
            Some(Err(_)) => String::from("test"),
        }
    }

    fn write_from_value(&self) {
        let current_from_value = self.from_entry.get_text();
        let from_value = &self.get_from_value();
        dbg!(&current_from_value);
        dbg!(&from_value);
        dbg!(current_from_value.eq(from_value));

        if current_from_value.parse::<f64>() != from_value.parse::<f64>() {
            self.from_entry.set_text(from_value);
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
