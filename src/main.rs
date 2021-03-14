use std::fs;
use std::path::PathBuf;

use glib::types::Type;
use gtk::{
    prelude::TreeStoreExtManual, CellLayoutExt, ComboBox, ComboBoxExt, ContainerExt, GtkWindowExt,
    Inhibit, TreeModelExt, WidgetExt, Window, WindowType,
};
use gtk::{Orientation::Vertical, TreeStoreExt};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

use units::lengths;

mod units;

struct Directory {
    current_dir: PathBuf,
}

#[derive(Msg)]
enum Msg {
    ItemSelect,
    Quit,
}

struct Win {
    combo_box: ComboBox,
    model: Directory,
    window: Window,
}

impl Update for Win {
    type Model = Directory;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Directory {
        let working_directory = fs::canonicalize(".").expect("Failed to open directory");
        Directory {
            current_dir: working_directory,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::ItemSelect => {
                // comboBox.GetActiveIter(out tree);
                // TreeModel = comboBox.Model ();
                // String selectedText = (String) comboBox.Model.GetValue (tree, 0);

                let iter = self.combo_box.get_active_iter().unwrap();
                let model = self.combo_box.get_model().unwrap();
                let val = model.get_value(&iter, 0);
                let got: &str = val.get().unwrap().unwrap();
                let val2 = model.get_value(&iter, 1);
                let got2: u64 = val2.get().unwrap().unwrap();
                dbg!(got);
                dbg!(got2);
                // print!(model.get_value(&iter, 1);

                // let iter = self.combo_box.get_active_iter();
                // let model = self.combo_box.get_model();
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
        let vbox = gtk::Box::new(Vertical, 0);
        let cell = gtk::CellRendererText::new();

        window.set_title("TreeView example file browser");
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 70);

        let store_model = create_and_fill_model();
        let combo_box = ComboBox::with_model(&store_model);
        combo_box.set_entry_text_column(0);

        combo_box.pack_start(&cell, true);
        combo_box.add_attribute(&cell, "text", 0);

        vbox.add(&combo_box);
        window.add(&vbox);

        window.show_all();

        combo_box.connect_changed(|ref dropdown| {
            dbg!(dropdown.get_active_id());
        });
        connect!(relm, combo_box, connect_changed(_), Msg::ItemSelect);
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Win {
            combo_box,
            model,
            window,
        }
    }
}

fn create_and_fill_model() -> gtk::TreeStore {
    let model = gtk::TreeStore::new(&[Type::String, Type::U64]);

    for i in 1..4 {
        let top = model.append(None);
        model.set(&top, &[0], &[&format!("upper {}", i)]);
        model.set(&top, &[1], &[&45]);
        for j in 1..6 {
            let entries = model.append(Some(&top));
            model.set(&entries, &[0], &[&format!("lower {}", j)]);
            model.set(&entries, &[1], &[&55]);
        }
    }
    model
}

fn main() {
    let lengths_units = lengths::init();
    dbg!(lengths_units.name);
    dbg!(lengths_units.units[0].convert(50.0, &lengths_units.units[2]));
    Win::run(()).expect("Win::run failed");
}
