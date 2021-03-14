use glib::types::Type;
use gtk::{BoxExt, CellLayoutExt, ComboBox, ComboBoxExt, ContainerExt, GtkWindowExt, Inhibit, TreeModelExt, TreeStoreExt, WidgetExt, Window, WindowType, prelude::TreeStoreExtManual};
use gtk::Orientation::{Horizontal, Vertical};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use units::{length, mass};
use crate::units::UnitType;

mod units;

struct Converter {
    unit_types: Vec<UnitType>,
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
        let unit_types: Vec<UnitType> = vec![
            length::init(),
            mass::init(),
        ];

        Converter {
            unit_types,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::FromComboChanged => {
                let iter = self.from_combo.get_active_iter().unwrap();
                let model = self.from_combo.get_model().unwrap();
                let val = model.get_value(&iter, 0);
                let got: &str = val.get().unwrap().unwrap();
                let val2 = model.get_value(&iter, 1);
                let got2: u64 = val2.get().unwrap().unwrap();
                dbg!(got);
                dbg!(got2);
                // print!(model.get_value(&iter, 1);

                // let iter = self.from_combo.get_active_iter();
                // let model = self.from_combo.get_model();
                // model.get_or_insert(value)
                // from_combo.get_model ().get (from_iter, 2, out from_unit, -1);

                // (model, pathlist) = tree_selection.get_selected_rows()
                // for path in pathlist :
                //     tree_iter = model.get_iter(path)
                //     value = model.get_value(tree_iter,0)
                //     print value

                // let selection = self.tree_view.get_selection();
                // if let Some((list_model, iter)) = selection.get_selected() {
                //     let is_dir: bool = list_model
                //         .get_value(&iter, IS_DIR_COL)
                //         .get::<bool>()
                //         .ok()
                //         .and_then(|value| value)
                //         .expect("get_value.get<bool> failed");

                //     if is_dir {
                //         let dir_name = list_model
                //             .get_value(&iter, VALUE_COL)
                //             .get::<String>()
                //             .ok()
                //             .and_then(|value| value)
                //             .expect("get_value.get<String> failed");

                //         println!("{:?} selected", dir_name);
                //         let new_dir = if dir_name == ".." {
                //             // Go up parent directory, if it exists
                //             self.model
                //                 .current_dir
                //                 .parent()
                //                 .unwrap_or(&self.model.current_dir)
                //                 .to_owned()
                //         } else {
                //             self.model.current_dir.join(dir_name)
                //         };
                //         self.model.current_dir = new_dir;
                //         let new_model = create_and_fill_model(&self.model.current_dir);

                //         self.tree_view.set_model(Some(&new_model));
                //     }
                // }
            }
            Msg::Quit => gtk::main_quit(),
            Msg::ToComboChanged => {
                let iter = self.to_combo.get_active_iter().unwrap();
                let model = self.to_combo.get_model().unwrap();
                let val = model.get_value(&iter, 0);
                let got: &str = val.get().unwrap().unwrap();
                let val2 = model.get_value(&iter, 1);
                let got2: u64 = val2.get().unwrap().unwrap();
                dbg!(got);
                dbg!(got2);
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

        let to_store = create_and_fill_model();
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
    let store = gtk::TreeStore::new(&[Type::String, Type::U64]);

    for unit_type in model.unit_types.iter() {
        let top = store.append(None);
        store.set(&top, &[0], &[&format!("{}", unit_type.name)]);
        store.set(&top, &[1], &[&45]);
        for unit in unit_type.units.iter() {
            let entries = store.append(Some(&top));
            store.set(&entries, &[0], &[&format!("{}", unit.name)]);
            store.set(&entries, &[1], &[&55]);
        }
    }
    store
}

fn create_and_fill_model() -> gtk::TreeStore {
    let store = gtk::TreeStore::new(&[Type::String, Type::U64]);

    for i in 1..4 {
        let top = store.append(None);
        store.set(&top, &[0], &[&format!("upper {}", i)]);
        store.set(&top, &[1], &[&45]);
        for j in 1..6 {
            let entries = store.append(Some(&top));
            store.set(&entries, &[0], &[&format!("lower {}", j)]);
            store.set(&entries, &[1], &[&55]);
        }
    }
    store
}

fn main() {
    let lengths_units = length::init();
    dbg!(lengths_units.name);
    dbg!(lengths_units.units[0].convert(50.0, &lengths_units.units[2]));
    Win::run(()).expect("Win::run failed");
}
