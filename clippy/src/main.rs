use eframe::egui;
use std::path::PathBuf;
use fuse_rust::{ Fuse };
use std::process::Command;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 350.0])
            .with_position([310.0, 435.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native("Clippy", 
        options, 
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

trait Searcher {
    fn search(&self, query: &str, results: &mut Vec<SearchResult>);
}

struct AppSearcher {
    apps: Vec<App>,
    fuse: Fuse,
}
struct App {
    name: String,
    exec: String,
    keywords: Vec<String>,
}

impl Searcher for AppSearcher {
    fn search(&self, query: &str, results: &mut Vec<SearchResult>) {
        for app in &self.apps {
            if let Some(score) = fuzzy_score(&self.fuse, query, &app.name) {
                results.push(SearchResult {
                    title: app.name.clone(),
                    score,
                    result_type: ResultType::App,
                    runcmd: Action::Launch(app.exec.clone().into()),
                });
            }
        }
    }
}

impl AppSearcher {
    fn new() -> Self {
        Self {
            apps: scan_apps(),
            fuse: Fuse::default(),
        }
    }
}

fn fuzzy_score(fuse: &Fuse, query: &str, name: &str) -> Option<f32> {
    if query.is_empty() || name.is_empty() {return None;}
    let pattern = fuse.create_pattern(query);
    match fuse.search(pattern.as_ref(), name) {
        None => None,
        Some(result) => Some(result.score as f32),
    }
}

fn scan_apps() -> Vec<App> {
    let mut apps = Vec::new();
    for app in 0..9 {
        match app {
            0 => {
                apps.push(App {
                    name: format!("App #0"),
                    exec: format!("C:"),
                    keywords: {
                        let mut words: Vec<String> = Vec::new();
                        words.push(format!("{:?}", app));
                        words
                    },
                });
            },
            1 => {
                apps.push(App {
                    name: format!("Vim"),
                    exec: format!("C://Program Files/Vim/vim92/vim.exe"),
                    keywords: {
                        let mut words: Vec<String> = Vec::new();
                        words.push(format!("{:?}", app));
                        words
                    },
                });
            },
            2 => {
                apps.push(App {
                    name: format!("Obsidian"),
                    exec: format!("C://Users/Joseph/Desktop/Obsidian.lnk"),
                    keywords: {
                        let mut words: Vec<String> = Vec::new();
                        words.push(format!("{:?}", app));
                        words
                    },
                });
            },
            3 => {
                apps.push(App {
                    name: format!("Melty Blood: Type Lumina"),
                    exec: format!("C://Users/Joseph/Desktop/MBAA.exe"),
                    keywords: {
                        let mut words: Vec<String> = Vec::new();
                        words.push(format!("{:?}", app));
                        words
                    },
                });
            },
            4 => {
                apps.push(App {
                    name: format!("GitHub Desktop"),
                    exec: format!("C://Users/Joseph/Desktop/GitHub Desktop.lnk"),
                    keywords: {
                        let mut words: Vec<String> = Vec::new();
                        words.push(format!("{:?}", app));
                        words
                    },
                });
            },
            5 => {
                apps.push(App {
                    name: format!("BreeZip"),
                    exec: format!("C://Users/Joseph/Desktop/BreeZip.lnk"),
                    keywords: {
                        let mut words: Vec<String> = Vec::new();
                        words.push(format!("{:?}", app));
                        words
                    },
                });
            },
            6 => {
                apps.push(App {
                    name: format!("Firefox"),
                    exec: format!("C://Users/Joseph/Desktop/Firefox.exe"),
                    keywords: {
                        let mut words: Vec<String> = Vec::new();
                        words.push(format!("{:?}", app));
                        words
                    },
                });
            },
            7 => {
                apps.push(App {
                    name: format!("Flashpoint"),
                    exec: format!("C://Flashpoint/Flashpoint.lnk"),
                    keywords: {
                        let mut words: Vec<String> = Vec::new();
                        words.push(format!("{:?}", app));
                        words
                    },
                }); 
            },
            _ => {0;},
        }
    }
    apps
}

struct SearchResult {
    title: String,
    score: f32,
    result_type: ResultType,
    runcmd: Action,
}

enum ResultType {
    App,
    File,
    Calculation,
    Command,
}

enum Action {
    Launch(PathBuf),
    OpenFile(PathBuf),
    Calculator(Expr),
    RunCommand(CommandId),
    Url(String),
    Collection(Vec<Action>),
}

fn do_action(act: &Action) -> Command {
    match act {
        Action::Launch(path) => {
            if path.ends_with(".lnk") {
                let mut cmd = Command::new("cmd");
                cmd.args(["/c", "start", "", path.to_str().unwrap()]);
                cmd
            }
            else {
                Command::new(path)
            }
        },
        Action::OpenFile(path) => { 
            let mut cmd = Command::new("vim");
            cmd.arg(path);
            cmd
        },
        Action::Calculator(expr) => Command::new("calculator"),
        Action::RunCommand(cmd) => Command::new("cmd"),
        Action::Url(link) => {
            let mut cmd = Command::new("firefox");
            cmd.arg(link);
            cmd
        },
        Action::Collection(acts) => if let Some(thing) = acts.get(0) {do_action(thing)} else {Command::new("quit")},
    }
}

enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Number(i32),
}

enum CommandId {
    CMD,
}

struct MyApp {
    searchterm: String,
    first_frame: bool,
    results: Vec<SearchResult>,
    current: i32,
    searchers: Vec<Box<dyn Searcher>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            searchterm: String::new(),
            first_frame: true,
            searchers: vec![
                Box::new(AppSearcher::new()),
            ],
            current: 0,
            results: Vec::new(),
        }
    }
}

impl MyApp {
    fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();

        for searcher in &self.searchers {
            searcher.search(query, &mut results);
        }
        results.sort_by(|a,b| {b.score.total_cmp(&a.score)});
        results
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut design = egui::Visuals::dark();

        design.override_text_color = Some(egui::Color32::GRAY);
        design.panel_fill = egui::Color32::from_rgb(10,40,60);
        design.window_fill = egui::Color32::from_rgb(20,40,80);

        ctx.set_visuals(design);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(egui::RichText::new("Clippy").size(24.0).strong());

            ui.vertical_centered(|ui| {
                                
                for num in 0..5 {
                    egui::Frame::none()
                        .fill(egui::Color32::from_gray(if &num == &self.current {50} else {30}))
                        .rounding(8.0)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.set_min_size(egui::vec2(500.0, 30.0));

                            if let Some(result) = &self.results.get(num as usize) {
                                ui.label(egui::RichText::new(result.title.clone())
                                .font(egui::FontId::proportional(18.0)));
                            }
                            else {
                                ui.label(egui::RichText::new(format!("Generic Result {}", num))
                                .font(egui::FontId::proportional(18.0)));
                            }

                            ui.add_space(5.0);
                        });
                }
               
                ui.add_space(10.0);

                let search_box = ui.add_sized(
                    [500.0,40.0],
                    egui::TextEdit::singleline(&mut self.searchterm)
                    .font(egui::FontId::proportional(40.0))
                    .hint_text("Search :)")
                );

                if self.first_frame {
                    search_box.request_focus();
                    self.first_frame = false;
                }
                

                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                   if let Some(result) = &self.results.get(self.current as usize) {
                       let mut cmd = do_action(&result.runcmd);
                       cmd.status();
                   }
                }

                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if self.current < 4 {
                        self.current += 1;
                    }
                    else {
                        self.current = 0;
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if self.current > 0 {
                        self.current -= 1;
                    }
                    else {
                        self.current = 4;
                    }
                }

                if self.searchterm.is_empty() {
                    self.results = Vec::new();
                }
                if search_box.changed() {
                    self.results = self.search(&self.searchterm);
                }

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

