use crate::commands::get_theme;
use dialoguer::{Input, Select};
use maplit::btreemap;
use std::collections::BTreeMap;

pub enum InputSelection {
    Files,
    Generator,
}

pub fn prompt_input() -> (InputSelection, BTreeMap<&'static str, String>) {
    let theme = get_theme();

    let selection = Select::with_theme(&theme)
        .with_prompt("Pick input type")
        .default(0)
        .items(&["Input files", "Generator script"])
        .interact()
        .unwrap();
    let selection = match selection {
        0 => InputSelection::Files,
        1 => InputSelection::Generator,
        _ => panic!(),
    };

    let value = match selection {
        InputSelection::Files => {
            let path = Input::with_theme(&theme)
                .with_prompt("Enter input files path")
                .with_initial_text("./in/")
                .interact_text()
                .unwrap();
            btreemap! {
                "type" => "files".to_string(),
                "path" => path,
            }
        }
        InputSelection::Generator => {
            let program_path = Input::with_theme(&theme)
                .with_prompt("Enter generator program path")
                .with_initial_text("./")
                .interact_text()
                .unwrap();
            btreemap! {
                "type" => "generator".to_string(),
                "program" => program_path,
            }
        }
    };
    (selection, value)
}

pub fn prompt_model_program() -> String {
    Input::with_theme(&get_theme())
        .with_prompt("Enter model program path")
        .with_initial_text("./")
        .interact_text()
        .unwrap()
}

pub fn prompt_verifier() -> String {
    Input::with_theme(&get_theme())
        .with_prompt("Enter verifier program path")
        .with_initial_text("./")
        .interact_text()
        .unwrap()
}
