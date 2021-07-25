#[cfg(not(target_arch = "wasm32"))]
use anyhow::Result;
#[cfg(not(target_arch = "wasm32"))]
use std::{env::args, fs};

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod interpreter;
pub use interpreter::*;
mod types;
pub use types::*;
mod ast;
pub use ast::*;
mod error;
pub use error::*;
mod parser;
pub use parser::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() -> Result<()> {
    let mut program = String::new();

    let args: Vec<String> = args().collect();

    if args.len() > 0 {
        if std::path::Path::new(&args[1]).is_file() {
            program = String::from_utf8_lossy(
                &fs::read(std::path::Path::new(&args[1])).expect("Failed to read file"),
            )
            .to_string();
        }
    }

    run_code(program, &mut String::new())
}

#[cfg(target_arch = "wasm32")]
mod web {

    use crate::*;
    use yew::prelude::*;

    pub enum Msg {
        Run,
        CodeChange(String),
    }

    #[derive(Debug)]
    pub struct Model {
        link: ComponentLink<Self>,
        code: String,
        output: String,
    }

    impl Component for Model {
        type Message = Msg;
        type Properties = ();

        fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
            Self {
                link,
                code: String::new(),
                output: String::new(),
            }
        }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            match msg {
                Msg::Run => {
                    self.output = String::new();
                    run_code(self.code.clone(), &mut self.output).unwrap();
                }
                Msg::CodeChange(code) => self.code = code,
            }
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            false
        }

        fn view(&self) -> Html {
            html! {
                <div>
                    <textarea cols="100" rows="20" onchange=self.link.callback(|event| Msg::CodeChange(if let ChangeData::Value(value) = event {value} else {String::new()}))></textarea>
                    <textarea cols="100" rows="20">{ self.output.clone() }</textarea>
                    <button onclick=self.link.callback(|_| Msg::Run)>{ "Run" }</button>
                </div>
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    yew::start_app::<web::Model>();
}
