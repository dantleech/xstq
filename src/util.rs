use std::fs;

use glob::glob;
use xot::fixed::Document;
use xot::output::xml::Parameters;
use xot::output::Indentation;
use xot::Xot;

pub fn find(path_or_globs: &Vec<String>) -> Vec<String> {
    let mut paths: Vec<String> = vec![];
    for path_or_glob in path_or_globs {
        if fs::exists(path_or_glob).unwrap() {
            paths.push(path_or_glob.to_string())
        }
        for entry in glob(path_or_glob).unwrap() {
            if entry.is_err() {
                continue;
            }
            paths.push(entry.unwrap().to_string_lossy().to_string());
        }
    }

    paths
}

#[allow(dead_code)]
pub fn pretty_xml(dom: Document) -> String {
    let mut xot = Xot::new();
    let node = dom.xotify(&mut xot);
    xot.serialize_xml_string(
        Parameters {
            indentation: Some(Indentation {
                suppress: Vec::new(),
            }),
            cdata_section_elements: Vec::new(),
            declaration: None,
            doctype: None,
            unescaped_gt: true,
        },
        node,
    )
    .unwrap()
}
