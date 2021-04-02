use glib::types::Type;
use gtk::{
    prelude::ComboBoxExtManual,
    EditableSignals, EntryExt, GridExt, GtkWindowExt, OrientableExt,
    Orientation::{Horizontal, Vertical},
    TreePath,
};
use gtk::{
    prelude::TreeStoreExtManual, BoxExt, CellLayoutExt, ComboBox, ComboBoxExt, ContainerExt, Entry,
    Inhibit, TreeModelExt, TreeStoreExt, WidgetExt, Window, WindowType,
};
use regex::Regex;
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use std::num::ParseFloatError;
use units::{length, mass, UnitType};

mod units;

const SPACING: i32 = 10;

struct Converter {
    cmd_value: String,
    unit_types: Vec<UnitType>,
    from_unit_idx: usize,
    from_value: Option<Result<f64, ParseFloatError>>,
    to_unit_idx: usize,
    to_value: Option<Result<f64, ParseFloatError>>,
    unit_type_idx: usize,
}

#[derive(Msg)]
enum Msg {
    CmdEntryChanged,
    FromComboChanged,
    FromEntryChanged,
    Quit,
    ToComboChanged,
    ToEntryChanged,
    UnitTypeComboChanged,
}

struct Win {
    cmd_entry: Entry,
    from_combo: ComboBox,
    from_entry: Entry,
    model: Converter,
    to_combo: ComboBox,
    to_entry: Entry,
    unit_type_combo: ComboBox,
    window: Window,
}

impl Update for Win {
    type Model = Converter;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Converter {
        Converter {
            cmd_value: String::from(""),
            unit_types: vec![length::init(), mass::init()],
            from_unit_idx: 0,
            from_value: None,
            to_unit_idx: 0,
            to_value: None,
            unit_type_idx: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::CmdEntryChanged => {
                dbg!("CmdEntryChanged");
                let cmd_value = self.cmd_entry.get_text().to_string();
                dbg!(&cmd_value);
                self.model.cmd_value = cmd_value.clone();

                let re = Regex::new(r"^\s*(\d+)\s*(\w+)\s+(to|in)\s+(\w+)\s*$").unwrap();
                if !re.is_match(&cmd_value) {
                    dbg!("CmdEntryChanged ended early");
                    return;
                }

                let captures = re.captures(&cmd_value).unwrap();
                let from_value = &captures[1];
                let from_unit_title = &captures[2];
                let to_unit_title = &captures[4];

                let (from_unit_type_idx, from_unit_idx) =
                    self.get_unit_type_and_unit_idx_by_title(String::from(from_unit_title));

                if !from_unit_type_idx.is_some() || !from_unit_idx.is_some() {
                    dbg!("CmdEntryChanged ended early");
                    return;
                }

                let from_unit_type_idx = from_unit_type_idx.unwrap();
                let from_unit_idx = from_unit_idx.unwrap();

                let (to_unit_type_idx, to_unit_idx) =
                    self.get_unit_type_and_unit_idx_by_title(String::from(to_unit_title));

                if !to_unit_type_idx.is_some() || !to_unit_idx.is_some() {
                    dbg!("CmdEntryChanged ended early");
                    return;
                }

                let to_unit_type_idx = to_unit_type_idx.unwrap();
                let to_unit_idx = to_unit_idx.unwrap();

                if from_unit_type_idx != to_unit_type_idx {
                    dbg!("CmdEntryChanged ended early");
                    return;
                }

                dbg!(from_unit_type_idx);
                dbg!(from_unit_idx);
                dbg!(to_unit_type_idx);
                dbg!(to_unit_idx);

                if from_unit_type_idx != to_unit_type_idx {
                    dbg!("CmdEntryChanged ended early");
                    return;
                }

                self.model.unit_type_idx = from_unit_type_idx;
                self.model.from_unit_idx = from_unit_idx;
                self.model.to_unit_idx = to_unit_idx;

                println!("cmd set to_unit_idx to {}", &to_unit_idx);

                // TODO - only set these if necessary
                self.set_unit_type_combo();
                self.set_from_combo();
                self.set_to_combo();

                if self.model.from_value != Some(from_value.parse::<f64>()) {
                    self.from_entry.set_text(from_value);
                }
                dbg!("CmdEntryChanged ended");
            }
            Msg::FromComboChanged => {
                dbg!("FromComboChanged");
                let iter = self.from_combo.get_active_iter().unwrap();
                let from_model = self.from_combo.get_model().unwrap();

                let val1 = from_model.get_value(&iter, 1);
                let from_unit_idx: u64 = val1.get().unwrap().unwrap();
                let from_unit_idx: usize = from_unit_idx as usize;

                if from_unit_idx == self.model.from_unit_idx {
                    dbg!("FromComboChanged ended early");
                    return;
                }

                self.model.from_unit_idx = from_unit_idx;

                self.write_to_value();
                dbg!("FromComboChanged ended");
            }
            Msg::FromEntryChanged => {
                dbg!("FromEntryChanged");
                let from_value = self.from_entry.get_text();
                let from_value_parsed = Some(from_value.parse::<f64>());

                if self.model.from_value == from_value_parsed {
                    dbg!("FromEntryChanged ended early");
                    return;
                }

                self.model.from_value = if from_value.eq("") {
                    None
                } else {
                    from_value_parsed
                };

                self.write_to_value();
                dbg!("FromEntryChanged ended");
            }
            Msg::Quit => gtk::main_quit(),
            Msg::ToComboChanged => {
                dbg!("ToComboChanged");
                let iter = self.to_combo.get_active_iter().unwrap();
                let to_model = self.to_combo.get_model().unwrap();

                let val1 = to_model.get_value(&iter, 1);
                let to_unit_idx: u64 = val1.get().unwrap().unwrap();
                let to_unit_idx: usize = to_unit_idx as usize;

                dbg!(to_unit_idx);
                dbg!(self.model.to_unit_idx);

                if to_unit_idx == self.model.to_unit_idx {
                    dbg!("ToComboChanged ended early");
                    return;
                }

                self.model.to_unit_idx = to_unit_idx;

                self.write_to_value();
                dbg!("ToComboChanged ended");
            }
            Msg::ToEntryChanged => {
                dbg!("ToEntryChanged");
                let to_value = self.to_entry.get_text();
                let to_value_parsed = Some(to_value.parse::<f64>());

                if self.model.to_value == to_value_parsed {
                    dbg!("ToEntryChanged ended early");
                    return;
                }

                self.model.to_value = if to_value.eq("") {
                    None
                } else {
                    to_value_parsed
                };

                self.write_from_value();
                dbg!("ToEntryChanged ended");
            }
            Msg::UnitTypeComboChanged => {
                dbg!("UnitTypeComboChanged");
                let iter = self.unit_type_combo.get_active_iter().unwrap();
                let unit_type_model = self.unit_type_combo.get_model().unwrap();

                let val1 = unit_type_model.get_value(&iter, 1);
                let unit_type_idx: u64 = val1.get().unwrap().unwrap();

                if self.model.unit_type_idx == unit_type_idx as usize {
                    dbg!("UnitTypeComboChanged ended early");
                    return;
                }

                self.model.unit_type_idx = unit_type_idx as usize;

                self.model.from_unit_idx = 0;
                self.set_from_combo();
                self.model.to_unit_idx = 0;
                self.set_to_combo();
                self.write_to_value();
                dbg!("UnitTypeComboChanged ended");
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
        let vbox = gtk::Box::new(Vertical, SPACING * 2);
        let grid = gtk::Grid::new();
        grid.set_orientation(Horizontal);
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);
        grid.set_row_spacing(SPACING as u32);
        grid.set_column_spacing(SPACING as u32);
        let cell = gtk::CellRendererText::new();

        window.set_title("Converter");
        window.set_border_width((SPACING * 2) as u32);
        window.set_position(gtk::WindowPosition::Center);
        // window.set_default_size(550, 300);

        let unit_type_model = Self::create_unit_type_model(&model);
        let unit_type_combo = ComboBox::with_model(&unit_type_model);
        unit_type_combo.set_entry_text_column(0);
        unit_type_combo.pack_start(&cell, true);
        unit_type_combo.add_attribute(&cell, "text", 0);
        unit_type_combo.set_active(Some(0));
        unit_type_combo.set_active_iter(Some(&unit_type_model.get_iter_first().unwrap()));
        grid.attach(&unit_type_combo, 1, 1, 2, 1);

        let from_model = Self::create_units_model(&model);
        let from_combo = ComboBox::with_model(&from_model);
        from_combo.set_entry_text_column(0);
        from_combo.pack_start(&cell, true);
        from_combo.add_attribute(&cell, "text", 0);
        from_combo.set_active(Some(0));
        from_combo.set_active_iter(Some(&from_model.get_iter_first().unwrap()));
        grid.attach(&from_combo, 1, 2, 1, 1);

        let from_entry = gtk::Entry::new();
        from_entry.set_placeholder_text(Some("From"));
        grid.attach(&from_entry, 1, 3, 1, 1);

        let to_model = Self::create_units_model(&model);
        let to_combo = ComboBox::with_model(&to_model);
        to_combo.set_entry_text_column(0);
        to_combo.pack_start(&cell, true);
        to_combo.add_attribute(&cell, "text", 0);
        to_combo.set_active_iter(Some(&to_model.get_iter_first().unwrap()));
        grid.attach(&to_combo, 2, 2, 1, 1);

        let to_entry = gtk::Entry::new();
        to_entry.set_placeholder_text(Some("To"));
        grid.attach(&to_entry, 2, 3, 1, 1);
        vbox.pack_start(&grid, false, true, 0);

        let hsep = gtk::Separator::new(Horizontal);
        vbox.pack_start(&hsep, false, true, 0);

        let cmd_entry = gtk::Entry::new();
        cmd_entry.set_placeholder_text(Some("e.g. 15ft in m"));
        vbox.pack_start(&cmd_entry, true, true, 0);

        window.add(&vbox);

        window.show_all();

        cmd_entry.grab_focus();

        connect!(relm, cmd_entry, connect_changed(_), Msg::CmdEntryChanged);
        connect!(relm, from_entry, connect_changed(_), Msg::FromEntryChanged);
        connect!(relm, from_combo, connect_changed(_), Msg::FromComboChanged);
        connect!(relm, to_combo, connect_changed(_), Msg::ToComboChanged);
        connect!(relm, to_entry, connect_changed(_), Msg::ToEntryChanged);
        connect!(
            relm,
            unit_type_combo,
            connect_changed(_),
            Msg::UnitTypeComboChanged
        );
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Win {
            cmd_entry,
            from_combo,
            from_entry,
            model,
            to_combo,
            to_entry,
            unit_type_combo,
            window,
        }
    }
}

impl Win {
    fn create_unit_type_model(model: &Converter) -> gtk::TreeStore {
        let store = gtk::TreeStore::new(&[Type::String, Type::U64]);

        for (unit_type_idx, unit_type) in model.unit_types.iter().enumerate() {
            let unit_type_row = store.append(None);
            store.set(&unit_type_row, &[0], &[&unit_type.name]);
            store.set(&unit_type_row, &[1], &[&(unit_type_idx as u64)]);
        }

        store
    }

    fn create_units_model(model: &Converter) -> gtk::TreeStore {
        let store = gtk::TreeStore::new(&[Type::String, Type::U64]);
        let units = &model.unit_types[model.unit_type_idx].units;

        for (unit_idx, unit) in units.iter().enumerate() {
            let unit_row = store.append(None);
            store.set(&unit_row, &[0], &[&unit.get_title()]);
            store.set(&unit_row, &[1], &[&(unit_idx as u64)]);
        }

        store
    }

    fn get_to_value(&self) -> String {
        match self.model.from_value {
            None => String::from(""),
            Some(Ok(from_value)) => {
                let units = &self.model.unit_types[self.model.unit_type_idx].units;
                units[self.model.from_unit_idx]
                    .convert_as_string(from_value, &units[self.model.to_unit_idx])
            }
            Some(Err(_)) => String::from("-"),
        }
    }

    fn write_to_value(&self) {
        let current_to_value = self.to_entry.get_text();
        let to_value = &self.get_to_value();

        if current_to_value.parse::<f64>() != to_value.parse::<f64>() {
            self.to_entry.set_text(to_value);
        }

        self.write_cmd();
    }

    fn get_from_value(&self) -> String {
        match self.model.to_value {
            None => String::from(""),
            Some(Ok(to_value)) => {
                let units = &self.model.unit_types[self.model.unit_type_idx].units;
                units[self.model.to_unit_idx]
                    .convert_as_string(to_value, &units[self.model.from_unit_idx])
            }
            Some(Err(_)) => String::from("-"),
        }
    }

    fn write_from_value(&self) {
        let current_from_value = self.from_entry.get_text();
        let from_value = &self.get_from_value();

        if current_from_value.parse::<f64>() != from_value.parse::<f64>() {
            self.from_entry.set_text(from_value);
        }

        self.write_cmd();
    }

    fn write_cmd(&self) {
        match self.model.from_value {
            Some(Ok(from_value)) => {
                let units = &self.model.unit_types[self.model.unit_type_idx].units;
                let cmd_value = format!(
                    "{}{} in {}",
                    &from_value,
                    units[self.model.from_unit_idx].abbreviation,
                    units[self.model.to_unit_idx].abbreviation
                );
                dbg!(&cmd_value);
                dbg!(&self.model.cmd_value);
                if self.model.cmd_value != cmd_value {
                    self.cmd_entry.set_text(&cmd_value);
                }
            }
            _ => {}
        };
    }

    fn get_unit_type_and_unit_idx_by_title(&self, title: String) -> (Option<usize>, Option<usize>) {
        for (unit_type_idx, unit_type) in self.model.unit_types.iter().enumerate() {
            for (unit_idx, unit) in unit_type.units.iter().enumerate() {
                if unit.abbreviation == title {
                    return (Some(unit_type_idx), Some(unit_idx));
                }
            }
        }

        return (None, None);
    }

    fn set_from_combo(&self) {
        println!("set_from_combo");
        let from_model = Self::create_units_model(&self.model);
        self.from_combo.set_model(Some(&from_model));

        let path = TreePath::from_string(&self.model.from_unit_idx.to_string());
        self.from_combo
            .set_active_iter(Some(&from_model.get_iter(&path).unwrap()));
    }

    fn set_to_combo(&self) {
        println!("set_to_combo");
        let to_model = Self::create_units_model(&self.model);
        self.to_combo.set_model(Some(&to_model));

        let path = TreePath::from_string(&self.model.to_unit_idx.to_string());
        self.to_combo
            .set_active_iter(Some(&to_model.get_iter(&path).unwrap()));
    }

    fn set_unit_type_combo(&self) {
        println!("set_unit_type_combo");
        let to_model = Self::create_unit_type_model(&self.model);
        self.unit_type_combo.set_model(Some(&to_model));

        dbg!(&self.model.unit_type_idx);

        let path = TreePath::from_string(&self.model.unit_type_idx.to_string());
        self.unit_type_combo
            .set_active_iter(Some(&to_model.get_iter(&path).unwrap()));
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
