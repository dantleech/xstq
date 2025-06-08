use anyhow::Result;
use tree_sitter::{Node, Parser};
use xot::fixed::{Content, Document, Element, Name};

pub struct Converter {}

impl Converter {
    pub fn new() -> Self {
        Converter {}
    }

    pub fn convert(&mut self, source: &str) -> Result<Document> {
        let mut parser = Parser::new();
        let language = tree_sitter_php::LANGUAGE_PHP;
        parser.set_language(&language.into()).unwrap();

        let node = parser.parse(source, None).unwrap();
        let element = self.process(&node.root_node(), source);

        Ok(Document {
            before: vec![],
            document_element: element,
            after: vec![],
        })
    }

    fn process(&mut self, node: &Node, source: &str) -> Element {
        let count = node.child_count();

        let mut children = Vec::new();

        for index in 0..count {
            let child = node.child(index).unwrap();
            children.push(Content::Element(self.process(&child, source)));
        }

        let content = node.utf8_text(source.as_bytes()).unwrap().to_string();
        children.push(Content::Text(content));

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
    use super::*;
    use pretty_assertions::assert_eq;
    use xot::Xot;

    #[test]
    fn test_analyse_vars() -> Result<(), anyhow::Error> {
        let source = r#"<?php
$var1 = 'hello'; $var2 = 'bar';
        "#;
        let dom = Converter::new().convert(source)?;
        let mut xot = Xot::new();
        let node = dom.xotify(&mut xot);
        assert_eq!(
            "<program><php_tag>&lt;?php</php_tag><expression_statement><assignment_expression><variable_name><$>$</$><name>var1</name>$var1</variable_name><=>=</=><string><'>'</'><string_content>hello</string_content><'>'</'>'hello'</string>$var1 = 'hello'</assignment_expression><;>;</;>$var1 = 'hello';</expression_statement><expression_statement><assignment_expression><variable_name><$>$</$><name>var2</name>$var2</variable_name><=>=</=><string><'>'</'><string_content>bar</string_content><'>'</'>'bar'</string>$var2 = 'bar'</assignment_expression><;>;</;>$var2 = 'bar';</expression_statement>&lt;?php\n$var1 = 'hello'; $var2 = 'bar';\n        </program>".to_string(),
            xot.to_string(node).unwrap()
        );
        Ok(())
    }
}
