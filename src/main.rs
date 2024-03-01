extern crate swc_common;
extern crate swc_ecma_parser;

use swc_common::GLOBALS;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use swc::{try_with_handler};
use swc::config::{Config, JscConfig, Options};
use swc_common::{
    SourceMap,
};

fn main() {
    let cm = Arc::<SourceMap>::default();

    let c = swc::Compiler::new(cm.clone());
    let output = GLOBALS
        .set(&Default::default(), || {
            try_with_handler(cm.clone(), Default::default(), |handler| {
                let fm = cm
                    .load_file(Path::new("./src/app/src/index.ts"))
                    .expect("failed to load file");
                // let mut paths = Paths::new();
                let k = "$utils".into();
                let v:Vec<String> = ["./src/utils.ts".into()].to_vec();

                let p:Vec<(String, Vec<String>)> = [(k, v)].to_vec();
                let paths = p
                    .into_iter()
                    .map(|(from, to)| {

                        (from, to)
                    })
                    .collect();


                let opt = &Options {
                    config: Config {
                        jsc: JscConfig {
                            base_url: PathBuf::from(r"/[replace to your path]/swc-bug-demo"),
                            paths,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    swcrc: false,
                    ..Default::default()
                };
                c.process_js_file(fm, handler, opt)
            })
        })
        .unwrap();

    println!("{}", output.code);


}
