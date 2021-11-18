use std::{borrow::Borrow, cell::RefCell};

use comrak::{
    self,
    arena_tree::Node as ComrakNode,
    nodes::{Ast, NodeHeading, NodeValue},
};

type Node<'a> = ComrakNode<'a, RefCell<Ast>>;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Spek {
    modules: Vec<SpekModule>,
}

impl Spek {
    fn extend<I: Iterator<Item = SpekItem>>(mut self, items: I) -> Self {
        let ms = self
            .modules
            .last_mut()
            .expect("must have a module to extend");
        ms.items.extend(items);
        self
    }

    fn add(mut self, item: SpekItem) -> Self {
        self.extend([item].into_iter())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SpekModule {
    filename: String,
    name: String,
    doc: Option<String>,
    items: Vec<SpekItem>,
}

impl SpekModule {
    pub fn new(name: String, doc: Option<String>) -> Self {
        Self {
            filename: filenamify::filenamify(&name),
            name,
            doc,
            items: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpekItem {
    Test { name: String, subs: Vec<String> },
    Doc(String),
}

#[derive(Debug, Default, derive_more::Constructor)]
struct State {
    spek: Spek,
    builder: Option<Builder>,
}

#[derive(Debug, Clone)]
enum Builder {
    Module { name: String },
    Item { segments: Vec<String> },
}

impl State {
    fn fold_node<'a>(mut self, node: &'a Node<'a>) -> Self {
        use NodeValue::*;
        let data = node.data.borrow();
        let State { mut spek, builder } = self;
        let (spek, builder) = match (&data.value, builder) {
            (Paragraph, Some(Builder::Module { name })) => {
                let doc = get_text(node);
                spek.modules.push(SpekModule::new(name, Some(doc)));
                (spek, None)
            }
            (Paragraph, None) => {
                let text = get_text(node);
                (spek.add(SpekItem::Doc(text)), None)
            }
            (List(list), builder) => {
                if let Some(Builder::Module { name }) = builder {
                    spek.modules.push(SpekModule::new(name, None));
                }
                if list.bullet_char == 42 {
                    // asterisk
                    let items = parse_list(node);
                    (
                        spek.extend(items.into_iter().map(|name| SpekItem::Test {
                            name,
                            subs: Vec::new(),
                        })),
                        None,
                    )
                } else {
                    println!("TODO: handle non-asterisk bullet");
                    (spek, None)
                }
            }
            (Heading(heading), None) => {
                let name = get_text(node);
                if heading.level == 1 {
                    (spek, Some(Builder::Module { name }))
                } else {
                    todo!()
                }
            }
            (v, b) => {
                dbg!(v);
                (spek, b)
            }
        };
        State::new(spek, builder)
    }
}

impl Spek {
    pub fn from_markdown(buffer: &str) -> Spek {
        let arena = comrak::Arena::new();
        let root = comrak::parse_document(&arena, buffer, &Default::default());
        let init = State::default();
        let State { spek, .. } = root.children().fold(init, State::fold_node);
        spek
    }
}

fn parse_list<'a>(node: &'a Node<'a>) -> Vec<String> {
    node.children().map(item_text).collect()
}

fn item_text(node: &Node) -> String {
    get_text(node.first_child().expect("empty item"))
}

fn get_text(node: &Node) -> String {
    node.first_child()
        .map(|c| {
            if let NodeValue::Text(bytes) = &c.data.borrow().value {
                std::str::from_utf8(&bytes)
                    .expect("not valid utf8")
                    .to_string()
            } else {
                todo!("handle non-text first child")
            }
        })
        .expect("empty item")
}

#[test]
fn test_from_markdown() {
    let spek = Spek::from_markdown(
        r"
# Module 1

Docs for the module

* [ ] star 1
  * star 1.1
  * star 1.2
* [ ] star 2

Text

- dash 1
  - dash 1.1
",
    );

    let expected = Spek {
        modules: vec![SpekModule {
            filename: "module_1.rs".to_string(),
            name: "Module 1".to_string(),
            doc: Some("Docs for the module".to_string()),
            items: vec![
                SpekItem::Test {
                    name: "star 1".to_string(),
                    subs: vec!["star 1.1".to_string(), "star 1.2".to_string()],
                },
                SpekItem::Test {
                    name: "star 2".to_string(),
                    subs: vec![],
                },
                SpekItem::Doc(
                    "
Text

- dash 1
    - dash 1.1
                    "
                    .to_string(),
                ),
            ],
        }],
    };
    assert_eq!(spek, expected)
}
