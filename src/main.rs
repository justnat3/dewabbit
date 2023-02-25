// prevent the application from opening a terminal
#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwg::NativeUi;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Default)]
struct Dupers {
    window: nwg::Window,
    go_button: nwg::Button,
    select_button: nwg::Button,
    dialog: nwg::FileDialog,
}

impl Dupers {
    fn bye(&self) -> std::io::Result<()> {
        if let Some(path) = nwg::Clipboard::data_text(&self.window) {
            // this is where we are going to store all of our file hashes
            let mut map: HashMap<String, PathBuf> = HashMap::new();

            let path = Path::new(&path);
            let mut dupes_path = path.to_path_buf();
            dupes_path.push("dupes\\");
            let exists = dupes_path.exists();
            let dupes_path = dupes_path.to_str().unwrap().to_string();
            if !exists {
                std::fs::create_dir(&dupes_path)?;
            }

            for entry in std::fs::read_dir(path).unwrap() {
                let entry_path = entry.unwrap().path();
                let name_of = entry_path.file_name();

                // TODO: set text
                println!("{:#?}", &entry_path);

                let meta = match std::fs::metadata(&entry_path) {
                    Ok(val) => val,
                    Err(x) => {
                        let v = x.raw_os_error().unwrap();

                        //"permission denied"
                        if v == 5 {
                            continue;
                        }

                        // Bug: "Cloud Provider not avaliable"
                        if v == 362 {
                            let p = nwg::MessageParams {
                                title: "WARNING",
                                content: "Please log into OneDrive",
                                buttons: nwg::MessageButtons::Ok,
                                icons: nwg::MessageIcons::Warning,
                            };

                            nwg::message(&p);
                            nwg::stop_thread_dispatch();
                        }

                        // FIXME: The reason this panic is here (yikes) is because the types in the
                        // arms of the meta->match dont have the same types. Fix me, please
                        // probably in a refactor
                        panic!()
                    }
                };
                if meta.is_dir() {
                    continue;
                }

                let hash = sha256::digest(&*std::fs::read(&entry_path).unwrap());

                match map.get(&hash) {
                    Some(val) => {
                        let mut fp = PathBuf::from(dupes_path.clone());
                        let name_of = PathBuf::from(name_of.unwrap()).into_boxed_path();
                        fp.push(&*name_of);

                        let ree = entry_path.to_str().clone().unwrap();

                        std::fs::copy(PathBuf::from(ree).into_boxed_path(), fp.into_boxed_path())?;
                        std::fs::remove_file(val)?;
                    }

                    None => {
                        map.insert(hash, entry_path);
                        continue;
                    }
                };
            }
        } else {
            let p = nwg::MessageParams {
                title: "WARNING",
                content: "A folder with duplicates has not been selected",
                buttons: nwg::MessageButtons::Ok,
                icons: nwg::MessageIcons::Warning,
            };
            nwg::message(&p);
        }
        let p = nwg::MessageParams {
            title: "Good to Go!",
            content: "All done!",
            buttons: nwg::MessageButtons::Ok,
            icons: nwg::MessageIcons::Warning,
        };
        nwg::message(&p);

        Ok(())
    }

    fn open_dialog(&self) {
        if self.dialog.run(Some(&self.window)) {
            if let Ok(directory) = self.dialog.get_selected_item() {
                let dir = directory.into_string().unwrap();
                nwg::Clipboard::set_data_text(&self.window, &dir);
            }
        }
    }
}

struct Ui {
    // The reason we have to wrap in Rc is because this is the safest & simplist way
    // to handle callbacks
    inner: Rc<Dupers>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl nwg::NativeUi<Ui> for Dupers {
    fn build_ui(mut data: Dupers) -> Result<Ui, nwg::NwgError> {
        use nwg::Event as E;

        nwg::Window::builder()
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .size((256, 200))
            .center(true)
            .title("Dupers")
            .icon(Some(&nwg::Icon::from_system(nwg::OemIcon::Ques)))
            .build(&mut data.window)?;

        nwg::Button::builder()
            .size((160, 60))
            .position((45, 20))
            .text("Go!")
            .parent(&data.window)
            .build(&mut data.go_button)?;

        nwg::Button::builder()
            .size((160, 60))
            .position((45, 90))
            .text("Select a Folder")
            .parent(&data.window)
            .build(&mut data.select_button)?;

        nwg::FileDialog::builder()
            .title("Pick a Directory with Duplicates")
            .action(nwg::FileDialogAction::OpenDirectory)
            .multiselect(false)
            .build(&mut data.dialog)?;

        let ui = Ui {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                match evt {
                    E::OnButtonClick => {
                        if &handle == &ui.go_button {
                            match Dupers::bye(&ui) {
                                Ok(_) => nwg::stop_thread_dispatch(),
                                Err(v) => {
                                    let p = nwg::MessageParams {
                                        title: "WARNING",
                                        content: &format!("Unexpected Error {}", v),
                                        buttons: nwg::MessageButtons::Ok,
                                        icons: nwg::MessageIcons::Warning,
                                    };
                                    nwg::message(&p);
                                }
                            };
                        }
                        if &handle == &ui.select_button {
                            Dupers::open_dialog(&ui);
                        }
                    }
                    E::OnWindowClose => nwg::stop_thread_dispatch(),
                    _ => {}
                }
            };
        };

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
            &ui.inner.window.handle,
            handle_events,
        ));

        return Ok(ui);
    }
}

fn main() {
    nwg::init().unwrap();
    let _app = Dupers::build_ui(Default::default()).unwrap();
    nwg::dispatch_thread_events();
}
