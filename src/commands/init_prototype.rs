use crate::commands::get_theme;
use crate::init_common::{prompt_input, prompt_model_program, prompt_verifier};
use console::style;
use dialoguer::Confirm;
use maplit::btreemap;
use serde_yaml::to_value;
use std::fs;

pub fn run() {
    let theme = get_theme();

    let (_, input_value) = prompt_input();
    let model_program_path = prompt_model_program();

    let mut result = btreemap! {
        "input" => to_value(&input_value).unwrap(),
        "modelProgram" => to_value(&model_program_path).unwrap(),
    };

    let use_verifier_script = Confirm::with_theme(&theme)
        .with_prompt("Use a verifier script?")
        .interact()
        .unwrap();

    if use_verifier_script {
        let verifier_path = prompt_verifier();
        result.insert("verifier", to_value(&verifier_path).unwrap());
    }

    let yaml = serde_yaml::to_string(&result).unwrap();
    fs::write("iobot-prototype.yaml", &yaml).expect("Unable to write file");
    println!(
        "{}",
        style(format!(
            "Saved to file {}",
            style("iobot-prototype.yaml").bold()
        ))
        .green()
    );
    println!("{}", yaml);
}
