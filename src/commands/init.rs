use crate::commands::get_theme;
use crate::init_common::{prompt_input, prompt_model_program, prompt_verifier, InputSelection};
use console::style;
use dialoguer::{Confirm, Input, Select};
use maplit::btreemap;
use serde_yaml::to_value;
use std::collections::BTreeMap;
use std::fs;

enum OutputSelection {
    Files,
    ModelProgram,
}

fn get_model_program_output<'a>() -> BTreeMap<&'a str, String> {
    btreemap! {
        "type" => "model-program".to_string(),
        "program" => prompt_model_program(),
    }
}

fn get_files_output<'a>() -> BTreeMap<&'a str, String> {
    let path = Input::with_theme(&get_theme())
        .with_prompt("Enter output files path")
        .with_initial_text("./out/")
        .interact_text()
        .unwrap();
    btreemap! {
        "type" => "files".to_string(),
        "path" => path,
    }
}

pub fn run() {
    let theme = get_theme();

    let (input_selection, input_value) = prompt_input();

    let use_verifier_script = Confirm::with_theme(&theme)
        .with_prompt("Use a verifier script?")
        .interact()
        .unwrap();

    let (output_value, verifier_value) = if use_verifier_script {
        let verifier_program_path = prompt_verifier();
        let output_value = match input_selection {
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
                    None => None,
                    Some(OutputSelection::Files) => Some(get_files_output()),
                    Some(OutputSelection::ModelProgram) => Some(get_model_program_output()),
                }
            }
            InputSelection::Generator => {
                let use_model_program = Confirm::with_theme(&theme)
                    .with_prompt("Use a model program (output supplied to verifier)?")
                    .interact()
                    .unwrap();
                if use_model_program {
                    Some(get_model_program_output())
                } else {
                    None
                }
            }
        };
        (output_value, Some(verifier_program_path))
    } else {
        let output_value = match input_selection {
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
                    OutputSelection::Files => get_files_output(),
                    OutputSelection::ModelProgram => get_model_program_output(),
                }
            }
            InputSelection::Generator => get_model_program_output(),
        };
        (Some(output_value), None)
    };

    let mut result = btreemap! {
        "input" => to_value(&input_value).unwrap(),
    };
    if let Some(output_value) = output_value {
        result.insert("output", to_value(&output_value).unwrap());
    }
    if let Some(verifier_value) = verifier_value {
        result.insert("verifier", to_value(&verifier_value).unwrap());
    }
    let yaml = serde_yaml::to_string(&result).unwrap();
    fs::write("iobot.yaml", &yaml).expect("Unable to write file");
    println!(
        "{}",
        style(format!("Saved to file {}", style("iobot.yaml").bold())).green()
    );
    println!("{}", yaml);
}
