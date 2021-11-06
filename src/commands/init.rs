use dialoguer::theme::ColorfulTheme;
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

fn get_model_program_output<'a>() -> BTreeMap<&'a str, String> {
    let model_program_path = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter model program path")
        .with_initial_text("./")
        .interact_text()
        .unwrap();
    btreemap! {
        "type" => "model-program".to_string(),
        "path" => model_program_path,
    }
}

fn get_files_output<'a>() -> BTreeMap<&'a str, String> {
    let path = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter output files path")
        .with_initial_text("./out/")
        .interact_text()
        .unwrap();
    btreemap! {
        "type" => "files".to_string(),
        "path" => path,
    }
}

pub fn init() {
    let theme = ColorfulTheme::default();

    let input_selection = Select::with_theme(&theme)
        .with_prompt("Pick input type")
        .default(0)
        .items(&["Input files", "Generator script"])
        .interact()
        .unwrap();
    let input_selection = match input_selection {
        0 => InputSelection::Files,
        1 => InputSelection::Generator,
        _ => panic!(),
    };

    let input_value = match input_selection {
        InputSelection::Files => {
            let path = Input::with_theme(&ColorfulTheme::default())
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
            let program_path = Input::with_theme(&ColorfulTheme::default())
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

    let use_verifier_script = Confirm::with_theme(&theme)
        .with_prompt("Use a verifier script?")
        .interact()
        .unwrap();

    let (output_value, verifier_value) = if use_verifier_script {
        let verifier_program_path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter verifier program path")
            .with_initial_text("./")
            .interact_text()
            .unwrap();
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
    fs::write("iobot.yaml", serde_yaml::to_string(&result).unwrap()).expect("Unable to write file");
}
