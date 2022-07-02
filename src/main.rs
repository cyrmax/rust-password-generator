#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::{NativeUi, CheckBoxState};
use rand::prelude::*;


enum Condition {
    Lowercase, Uppercase, Digit
}


#[derive(Default, NwgUi)]
pub struct RustPasswordGenerator {
    
    #[nwg_control(size: (300, 150), position: (300, 300), title: "Rust password generator")]
    #[nwg_events( OnWindowClose: [ RustPasswordGenerator::exit ])]
    window: nwg::Window,
    
    #[nwg_layout(parent: window, spacing: 5, min_size: [290, 140])]
    grid: nwg::GridLayout,
    
    #[nwg_control(text: "Include uppercase letters", check_state: CheckBoxState::Checked)]
    #[nwg_layout_item(layout: grid, col: 0, row: 0)]
    upper_cb: nwg::CheckBox,
    
    #[nwg_control(text: "Include lowercase letters", check_state: CheckBoxState::Checked)]
    #[nwg_layout_item(layout: grid, col: 1, row: 0)]
    lower_cb: nwg::CheckBox,
    
    #[nwg_control(text: "Include numbers", check_state: CheckBoxState::Checked)]
    #[nwg_layout_item(layout: grid, col: 2, row: 0)]
    number_cb: nwg::CheckBox,
    
    #[nwg_control(text: "Password length")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1)]
    pass_len_tb: nwg::Label,
    
    #[nwg_control(text: "12", flags: "VISIBLE|NUMBER", align: nwg::HTextAlign::Center, focus: true)]
    #[nwg_layout_item(layout: grid, col: 1, row: 1, col_span: 3)]
    passlen: nwg::TextInput,
    
    #[nwg_control(text: "Result")]
    #[nwg_layout_item(layout: grid, col: 0, row: 2)]
    result_tb: nwg::Label,
    
    #[nwg_control(text: "", align: nwg::HTextAlign::Center, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 1, row: 2, col_span: 5)]
    result_text: nwg::TextInput,
    
    #[nwg_control(text: "Generate")]
    #[nwg_layout_item(layout: grid, col: 4, row: 3)]
    #[nwg_events( OnButtonClick: [ RustPasswordGenerator::generate_password ] )]
    generate_btn: nwg::Button
}


impl RustPasswordGenerator {
    fn generate_password(&self) {
        let is_upper = state_to_bool(self.upper_cb.check_state());
        let is_lower = state_to_bool(self.lower_cb.check_state());
        let is_number = state_to_bool(self.number_cb.check_state());
        
        if !(is_lower || is_upper || is_number) {
            self.show_error("You should select at least one symbol group");
            return;
        }
        
        if self.passlen.len() < 1 {
            self.show_error("Password can not be 0 symbols in length");
            return;
        }
        
        let pass_length: u32 = self.passlen.text().parse().unwrap();
        if pass_length < 1 {
            self.show_error("Password length cannot be less than 1");
            return;
        }

        let mut password = String::new();
        for _ in 0..pass_length {
            password.push(get_random_symbol(is_upper, is_lower, is_number));
        }
        self.result_text.set_text(&password);
    }
    
    fn show_error(&self, message: &str) {
        nwg::modal_error_message(&self.window, "error!", message);
    }
    
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}


fn main() {
    nwg::init().expect("failed to init nwg");
    nwg::Font::set_global_family("Segoe UI").expect("failed to set font");
    let _rpg = RustPasswordGenerator::build_ui(Default::default()).expect("failed to build UI");
    
    nwg::dispatch_thread_events();
}


fn get_random_symbol(include_upper: bool, include_lower: bool, include_num: bool) -> char {
    let mut generator = rand::thread_rng();
    let mut choices: Vec<Condition> = Vec::new();
    
    if include_lower {
        choices.push(Condition::Lowercase);
    }
    
    if include_upper {
        choices.push(Condition::Uppercase);
    }
    
    if include_num {
        choices.push(Condition::Digit);
    }
    
    let choice = &choices[generator.gen_range(0..choices.len())];
    
    match choice {
        Condition::Digit => char::from(generator.gen_range(48..58)),
        Condition::Lowercase => char::from(generator.gen_range(97..123)),
        Condition::Uppercase => char::from(generator.gen_range(65..91))
    }
}


fn state_to_bool(state: nwg::CheckBoxState) -> bool {
    match state {
        CheckBoxState::Checked => true,
        _ => false
    }
}