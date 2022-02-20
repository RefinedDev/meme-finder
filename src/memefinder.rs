use eframe::egui::{Label, TextStyle, Separator, Window,};
use eframe::egui::{FontDefinitions, FontData, FontFamily, CtxRef, Ui, Button, RichText};

use serde::{Deserialize, Serialize};
use serde_json;

use reqwest;

use std::path::Path;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufReader;
use std::vec;

#[derive(Serialize, Deserialize)]
struct SubReddit {
    name: String,
    api_url: String,
}

#[derive(Deserialize)]
struct Meme {
    page_url: String,
    title: String,
    author: String,
    upvotes: String,
    comments: String
}

pub struct MemeFinder { 
    subreddits: Vec<SubReddit>,
    memes: Option<Vec<Meme>>,

    add_subreddit_window: bool,
    remove_subreddit_window: bool,
    sub_red_input: String
}

impl MemeFinder {
    pub fn new() -> MemeFinder {
        if ! Path::new("meme-finder-saved.json").exists() {
            let file = File::create("meme-finder-saved.json").unwrap();
            
            let memes = vec![SubReddit {
                name: "memes".to_string(),
                api_url: "https://www.reddit.com/r/memes/new.json?sort=hot,".to_string()
            }];

            serde_json::to_writer_pretty(file, &memes).unwrap();
        }

        let file = OpenOptions::new().read(true).open("meme-finder-saved.json").unwrap();
        let reader = BufReader::new(file);
        let subreddits: Vec<SubReddit> = serde_json::from_reader(reader).unwrap();
        
        MemeFinder {
            subreddits: subreddits,
            memes: None,

            add_subreddit_window: false,
            remove_subreddit_window: false,
            sub_red_input: String::new()
        }
    }

    pub fn load(&mut self, ui: &mut Ui) {
        // Navbar
        ui.vertical_centered_justified(|ui|{
            ui.add(Label::new(RichText::new("MemeFinder").fallback_text_style(TextStyle::Heading)));
        });

        ui.add_space(10.);
        ui.add(Separator::default());

        ui.horizontal_wrapped(|ui|{
            for subreddit in &self.subreddits {
                let btn = ui.add(Button::new(RichText::new(subreddit.name.as_str())));
                if btn.clicked() {
                    self.memes = None;
                   // Reload Memes
                   let memes_request = reqwest::blocking::get(format!("{}", subreddit.api_url)).unwrap();
         
                    if memes_request.status().is_success() {
                        let json = memes_request.json::<serde_json::Value>().unwrap();
                        
                        for i in 1..50 {
                            let meme = &json["data"]["children"][i]["data"];

                            let page_url = format!("https://www.reddit.com{}", &meme["permalink"]).replace('"', "");    
                            let title = format!("{}",&meme["title"]);
                            
                            if title == "null" { return }
                            
                            let author = format!("{}",&meme["author"]);
                            let upvotes = format!("{}",&meme["ups"]);
                            let comments = format!("{}",&meme["num_comments"]);
                            
                            match self.memes {
                                None => {self.memes = Some(vec![Meme { page_url, title, author, upvotes, comments}])}
                                Some(_) => {self.memes.get_or_insert(vec![Meme { page_url: "lol".to_string(), title: "lol".to_string(), author: "lol".to_string(), upvotes: "lol".to_string(), comments: "lol".to_string() }]).push(Meme { page_url, title, author, upvotes, comments })}
                            }
                        }
                    } else {
                        self.memes = None;
                    }
                }
            }
        });

        ui.add(Separator::default());
        
        // Memes
        match &self.memes {
            None => {
                ui.vertical_centered_justified(|ui|{
                    ui.add(Label::new(RichText::new("No memes? Here's probably why\n🔴 You have not selected a SubReddit from the top-bar\n🔴 Failed to load the memes").fallback_text_style(TextStyle::Heading)));
                });
            }

            Some(memes) => {
                for meme in memes {
                    let meme_button = ui.add(Button::new(meme.title.as_str()))
                        .on_hover_text(format!("Upvotes - {}\nComments - {}\nAuthor - {}", meme.upvotes, meme.comments, meme.author));
                    
                    if meme_button.clicked() {
                        match open::that(meme.page_url.as_str()) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
            }
        };

        ui.add(Separator::default());
    }

    pub fn load_buttons(&mut self, ctx: &CtxRef, ui: &mut Ui) {
        // Buttons
        ui.checkbox(&mut self.add_subreddit_window,RichText::new("Add Subreddit").fallback_text_style(TextStyle::Small));
        ui.checkbox(&mut self.remove_subreddit_window,RichText::new("Remove Subreddit").fallback_text_style(TextStyle::Small));

        if self.add_subreddit_window {
            Window::new("Add Subreddit").show(ctx, |ui|{
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.text_edit_singleline(&mut self.sub_red_input);
                });

                if ui.add(Button::new("Add")).clicked() && ! self.sub_red_input.is_empty() {
                    let validate_subreddit = reqwest::blocking::get(format!("https://www.reddit.com/r/{}/new.json?sort=hot,", self.sub_red_input)).unwrap();
                    
                    if validate_subreddit.status().is_success() {
                        self.subreddits.push(SubReddit {
                            name: format!("{}", self.sub_red_input),
                            api_url: format!("https://www.reddit.com/r/{}/new.json?sort=hot,", self.sub_red_input)
                        }); 
    
                        self.sub_red_input = String::new();
                    } else if validate_subreddit.status().is_server_error() {
                        ui.add(Label::new(RichText::new("Server Error!").fallback_text_style(TextStyle::Heading)));
                    } else {
                        ui.add(Label::new(format!("Client Error: {}", validate_subreddit.status())));
                    }
                }
            });
        }

        if self.remove_subreddit_window {
            Window::new("Remove Subreddit").show(ctx, |ui|{
                let file = OpenOptions::new().read(true).open("meme-finder-saved.json").unwrap();
                let reader = BufReader::new(file);
                let subreddits: Vec<SubReddit> = serde_json::from_reader(reader).unwrap();

               for (i, subreddit) in subreddits.iter().enumerate() {
                    if ui.button(&subreddit.name).clicked() {
                        self.subreddits.swap_remove(i);
                    }
               }
            });
        }
    }

    pub fn save_data(&self) {
        // Save in a JSON File
        let file = OpenOptions::new().write(true).truncate(true).open("meme-finder-saved.json").unwrap();
        serde_json::to_writer_pretty(file, &self.subreddits).unwrap();
    }

    pub fn add_font(&mut self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();

        font_def.font_data.insert("Arial".to_string(),FontData::from_static(include_bytes!("../Assets/arial.ttf")));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Heading,(FontFamily::Proportional, 40.));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Body,(FontFamily::Proportional, 20.));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Small,(FontFamily::Proportional, 30.));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Button,(FontFamily::Proportional, 35.));

        font_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, "Arial".to_string());

        ctx.set_fonts(font_def);
    }
}
// 200