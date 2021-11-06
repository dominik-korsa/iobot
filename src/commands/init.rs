use crate::commands::get_theme;
use console::style;
use dialoguer::{Confirm, Input, Select};
use maplit::btreemap;
use serde_yaml::to_value;
use std::collections::BTreeMap;
use std::fs;

enum InputSelection {
    Files,
    Generator,
}

enum OutputSelection {
    Files,
    ModelProgram,
}

fn prompt_input() -> (InputSelection, BTreeMap<&'static str, String>) {
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

fn prompt_model_program() -> String {
    Input::with_theme(&get_theme())
        .with_prompt("Enter model program path")
        .with_initial_text("./")
        .interact_text()
        .unwrap()
}

fn prompt_verifier() -> String {
    Input::with_theme(&get_theme())
        .with_prompt("Enter verifier program path")
        .with_initial_text("./")
        .interact_text()
        .unwrap()
}

fn prompt_output_files<'a>() -> BTreeMap<&'a str, String> {
    let path = Input::with_theme(&get_theme())
        .with_prompt("Enter output files path")
        .with_initial_text("./out/")
        .interact_text()
        .unwrap();
    btreemap! {
        "path" => path,
    }
}

pub fn run() {
    let theme = get_theme();

    let (input_selection, input_value) = prompt_input();

    let mut result = btreemap! {
        "input" => to_value(&input_value).unwrap(),
    };

    let use_verifier_script = Confirm::with_theme(&theme)
        .with_prompt("Use a verifier script?")
        .interact()
        .unwrap();

    if use_verifier_script {
        result.insert("verifier", to_value(&prompt_verifier()).unwrap());
        match input_selection {
            InputSelection::Files => {
                let output_selection = Select::with_theme(&theme)
                    .with_prompt("Pick output type (supplied to verifier)")
                    .default(0)
                    .items(&["None", "Output files", "Model program"])
                    .interact()
                    .unwrap();
                let output_selection = match output_selection {
                    0 => None,
                    1 => Some(OutputSelection::Files),
                    2 => Some(OutputSelection::ModelProgram),
                    _ => panic!(),
                };
                match output_selection {
                    Some(OutputSelection::Files) => {
                        result.insert("outputFiles", to_value(&prompt_output_files()).unwrap());
                    }
                    Some(OutputSelection::ModelProgram) => {
                        result.insert("modelProgram", to_value(&prompt_model_program()).unwrap());
                    }
                    None => {}
                }
            }
            InputSelection::Generator => {
                let use_model_program = Confirm::with_theme(&theme)
                    .with_prompt("Use a model program (output supplied to verifier)?")
                    .interact()
                    .unwrap();
                if use_model_program {
                    result.insert("modelProgram", to_value(&prompt_model_program()).unwrap());
                }
            }
        };
    } else {
        match input_selection {
            InputSelection::Files => {
                let output_selection = Select::with_theme(&theme)
                    .with_prompt("Pick output type")
                    .default(0)
                    .items(&["Output files", "Model program"])
                    .interact()
                    .unwrap();
                let output_selection = match output_selection {
                    0 => OutputSelection::Files,
                    1 => OutputSelection::ModelProgram,
                    _ => panic!(),
                };
                match output_selection {
                    OutputSelection::Files => {
                        result.insert("outputFiles", to_value(&prompt_output_files()).unwrap())
                    }
                    OutputSelection::ModelProgram => {
                        result.insert("modelProgram", to_value(&prompt_model_program()).unwrap())
                    }
                }
            }
            InputSelection::Generator => {
                result.insert("modelProgram", to_value(&prompt_model_program()).unwrap())
            }
        };
    };

    let yaml = serde_yaml::to_string(&result).unwrap();
    fs::write("iobot.yaml", &yaml).expect("Unable to write file");
    println!(
        "{}",
        style(format!("Saved to file {}", style("iobot.yaml").bold())).green()
    );
    println!("{}", yaml);
}
