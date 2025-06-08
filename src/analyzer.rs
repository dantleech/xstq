use std::fs;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use xee_xpath::{Documents, Item, Queries, Query};
use xot::{
    output::{xml::Parameters, Indentation},
    Xot,
};

use crate::{config::Rule, source_to_dom::SourceToDom};

pub struct Analyzer {}

#[derive(Serialize, Deserialize)]
pub struct FileResults {
    pub path: String,
    pub results: Vec<FileResult>,
}

impl FileResults {
    pub(crate) fn len(&self) -> usize {
        self.results.len()
    }
}
#[derive(Serialize, Deserialize)]
pub struct FileResult {
    pub rule: String,
    pub result: String,
}

impl Analyzer {
    pub fn analyze_file(&self, path: &String, rules: &Vec<Rule>) -> Result<FileResults> {
        log::info!("Analyzing: {}", path);
        let source = fs::read_to_string(path.clone())?;
        Ok(FileResults {
            path: path.clone(),
            results: self.analyze(&source, rules)?,
        })
    }
    pub fn analyze(&self, source: &str, rules: &Vec<Rule>) -> Result<Vec<FileResult>> {
        let dom = SourceToDom::new().convert(source)?;
        let mut xot = Xot::new();
        let node = dom.xotify(&mut xot);

        log::debug!("... converting document to a string...");
        let dom_string = xot.to_string(node)?;
        let mut documents = Documents::new();

        log::debug!(
            "... parsing string to document 😬 ... ({} bytes)",
            dom_string.len()
        );
        let handle = match documents.add_string_without_uri(dom_string.as_str()) {
            Ok(h) => h,
            Err(e) => panic!("Could not parse dom: {} - {}", dom_string, e),
        };

        let mut results = Vec::new();
        for rule in rules {
            let queries = Queries::default();
            let q = match queries.sequence(&rule.xpath) {
                Ok(q) => q,
                Err(e) => panic!("{:?}", e),
            };
            log::debug!("... executing query: {}", rule.xpath);
            let r = match q.execute(&mut documents, handle) {
                Ok(q) => q,
                Err(e) => panic!("{:?}", e),
            };
            log::debug!("... processing");
            for e in r.iter() {
                results.push(FileResult {
                    rule: rule.name.clone(),
                    result: match e {
                        Item::Atomic(a) => a.to_string().unwrap_or_default(),
                        Item::Node(node) => xot
                            .serialize_xml_string(
                                Parameters {
                                    indentation: Some(Indentation {
                                        suppress: Vec::new(),
                                    }),
                                    cdata_section_elements: Vec::new(),
                                    declaration: None,
                                    doctype: None,
                                    unescaped_gt: true,
                                },
                                xot.parent(node).unwrap(),
                            )?
                            .to_string(),
                        Item::Function(_) => "".to_string(),
                    },
                });
            }
        }

        Ok(results)
    }

    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_analyse() -> Result<(), anyhow::Error> {
        let source = r#"<?php
echo 'hello';
echo 'goodbye';
        "#;
        let results = Analyzer::new().analyze(
            source,
            &vec![
                Rule{
                    name: "anon".to_string(),
                    xpath: "//echo_statement".to_string()
                }
            ],
        )?;
        assert_eq!(2, results.len());
        Ok(())
    }
}
