use anyhow::Result;
use tree_sitter::{Node, Parser};
use xot::fixed::{Content, Document, Element, Name};

pub struct SourceToDom {}

impl SourceToDom {
    pub fn new() -> Self {
        SourceToDom {}
    }

    pub fn convert(&mut self, source: &str) -> Result<Document> {
        log::debug!("... parsing source file");
        let mut parser = Parser::new();
        let language = tree_sitter_php::LANGUAGE_PHP;
        parser.set_language(&language.into()).unwrap();

        let node = parser.parse(source, None).unwrap();
        log::debug!("... converting to DOM");
        let element = SourceToDom::process(&node.root_node(), source);

        Ok(Document {
            before: vec![],
            document_element: element,
            after: vec![],
        })
    }

    fn process(node: &Node, source: &str) -> Element {
        let child_count = node.child_count();
        let mut children = Vec::new();

        for index in 0..child_count {
            let child = node.child(index).unwrap();

            // unnamed nodes would register as "<$" or "="
            if !child.is_named() {
                continue;
            }

            children.push(Content::Element(SourceToDom::process(&child, source)));
        }

        // only add text content to leaf nodes
        if child_count == 0 {
            let content = node.utf8_text(source.as_bytes()).unwrap().to_string();
            children.push(Content::Text(content));
        }

        Element {
            name: Name {
                namespace: "".to_string(),
                localname: node.kind().to_string(),
            },
            prefixes: Vec::new(),
            attributes: Vec::new(),
            children,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::util::pretty_xml;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() -> Result<(), anyhow::Error> {
        let source = r#"<?php
$var1 = 'hello'; $var2 = 'bar';
        "#;
        let dom = SourceToDom::new().convert(source)?;
        assert_eq!(
            r#"<program>
  <php_tag>&lt;?php</php_tag>
  <expression_statement>
    <assignment_expression>
      <variable_name>
        <name>var1</name>
      </variable_name>
      <string>
        <string_content>hello</string_content>
      </string>
    </assignment_expression>
  </expression_statement>
  <expression_statement>
    <assignment_expression>
      <variable_name>
        <name>var2</name>
      </variable_name>
      <string>
        <string_content>bar</string_content>
      </string>
    </assignment_expression>
  </expression_statement>
</program>
"#,
            pretty_xml(dom)
        );
        Ok(())
    }
}
