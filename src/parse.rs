// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

use parse_wiki_text::{DefinitionListItemType, Node, TableCellType};

pub fn parse(wiki_text: &str, result_el: &::HTMLTextAreaElement) {
    result_el.set_inner_text(&"");
    let result = ::parse_wiki_text::Configuration::default().parse(&wiki_text);
    for warning in result.warnings {
        let warning_el = html!(div.node);
        add_node_head(
            &warning_el,
            warning.message.message(),
            warning.start,
            warning.end,
        );
        result_el.append_element(&warning_el);
    }
    add_nodes(&result_el, &result.nodes);
}

fn add_complex_attribute(node_el: &::HTMLTextAreaElement, label: &str, value: &[Node]) {
    let value_el = html!(div.node);
    value_el.append_text(label);
    add_nodes(&value_el, value);
    node_el.append_element(&value_el);
}

fn add_node_head(node_el: &::HTMLTextAreaElement, label: &str, start: usize, end: usize) {
    let name_el = html!(span.name);
    name_el.append_text(label);
    node_el.append_element(&name_el);
    let position_el = html!(span.position);
    position_el.append_text(&format!(" {}:{}", start, end));
    node_el.append_element(&position_el);
}

fn add_nodes(container_el: &::HTMLTextAreaElement, nodes: &[Node]) {
    for node in nodes {
        let node_el = html!(div.node);
        // All nodes are destructured without using the ".." syntax to make sure the code doesn't compile if new fields are added.
        match node {
            Node::Bold { end, start } => {
                add_node_head(&node_el, "bold", *start, *end);
            }
            Node::BoldItalic { end, start } => {
                add_node_head(&node_el, "bold italic", *start, *end);
            }
            Node::Category {
                end,
                ordinal,
                start,
                target,
            } => {
                add_node_head(&node_el, "category", *start, *end);
                add_text_attribute(&node_el, "target: ", target);
                add_complex_attribute(&node_el, "ordinal: ", ordinal);
            }
            Node::CharacterEntity {
                character,
                end,
                start,
            } => {
                add_node_head(&node_el, "character entity", *start, *end);
                add_text_attribute(&node_el, "character: ", &character.to_string());
            }
            Node::Comment { end, start } => {
                add_node_head(&node_el, "comment", *start, *end);
            }
            Node::DefinitionList { end, items, start } => {
                add_node_head(&node_el, "definition list", *start, *end);
                for item in items {
                    let item_el = html!(div.node);
                    let label = match item.type_ {
                        DefinitionListItemType::Details => "details",
                        DefinitionListItemType::Term => "term",
                    };
                    add_node_head(&item_el, label, item.start, item.end);
                    add_nodes(&item_el, &item.nodes);
                    node_el.append_element(&item_el);
                }
            }
            Node::EndTag { end, name, start } => {
                add_node_head(&node_el, "end tag", *start, *end);
                add_text_attribute(&node_el, "name: ", name);
            }
            Node::ExternalLink { end, nodes, start } => {
                add_node_head(&node_el, "external link", *start, *end);
                add_nodes(&node_el, nodes);
            }
            Node::Heading {
                end,
                level,
                nodes,
                start,
            } => {
                add_node_head(&node_el, "heading", *start, *end);
                add_text_attribute(&node_el, "level: ", level);
                add_nodes(&node_el, nodes);
            }
            Node::Image {
                end,
                start,
                target,
                text,
            } => {
                add_node_head(&node_el, "image", *start, *end);
                add_text_attribute(&node_el, "target: ", target);
                add_complex_attribute(&node_el, "text: ", text);
            }
            Node::HorizontalDivider { end, start } => {
                add_node_head(&node_el, "horizontal divider", *start, *end);
            }
            Node::Italic { end, start } => {
                add_node_head(&node_el, "italic", *start, *end);
            }
            Node::Link {
                end,
                start,
                target,
                text,
            } => {
                add_node_head(&node_el, "link", *start, *end);
                add_text_attribute(&node_el, "target: ", target);
                add_complex_attribute(&node_el, "text: ", text);
            }
            Node::MagicWord { end, start } => {
                add_node_head(&node_el, "magic word", *start, *end);
            }
            Node::OrderedList { end, items, start } => {
                add_node_head(&node_el, "ordered list", *start, *end);
                for item in items {
                    let item_el = html!(div.node);
                    add_node_head(&item_el, "item", item.start, item.end);
                    add_nodes(&item_el, &item.nodes);
                    node_el.append_element(&item_el);
                }
            }
            Node::ParagraphBreak { end, start } => {
                add_node_head(&node_el, "paragraph break", *start, *end);
            }
            Node::Parameter {
                default,
                end,
                name,
                start,
            } => {
                add_node_head(&node_el, "parameter", *start, *end);
                add_complex_attribute(&node_el, "name", name);
                if let Some(default) = default {
                    add_complex_attribute(&node_el, "default", default);
                }
            }
            Node::Preformatted { end, nodes, start } => {
                add_node_head(&node_el, "preformatted", *start, *end);
                add_nodes(&node_el, nodes);
            }
            Node::Redirect { end, start, target } => {
                add_node_head(&node_el, "redirect", *start, *end);
                add_text_attribute(&node_el, "target: ", target);
            }
            Node::StartTag { end, name, start } => {
                add_node_head(&node_el, "start tag", *start, *end);
                add_text_attribute(&node_el, "name: ", name);
            }
            Node::Table {
                attributes,
                captions,
                end,
                rows,
                start,
            } => {
                add_node_head(&node_el, "table", *start, *end);
                if !attributes.is_empty() {
                    add_complex_attribute(&node_el, "attributes", attributes);
                }
                for ::parse_wiki_text::TableCaption {
                    attributes,
                    content,
                    end,
                    start,
                } in captions
                {
                    let caption_el = html!(div.node);
                    add_node_head(&caption_el, "caption", *start, *end);
                    if let Some(attributes) = attributes {
                        add_complex_attribute(&caption_el, "attributes", attributes);
                    }
                    add_complex_attribute(&caption_el, "content", content);
                    node_el.append_element(&caption_el);
                }
                for ::parse_wiki_text::TableRow {
                    attributes,
                    cells,
                    end,
                    start,
                } in rows
                {
                    let row_el = html!(div.node);
                    add_node_head(&row_el, "row", *start, *end);
                    if !attributes.is_empty() {
                        add_complex_attribute(&row_el, "attributes", attributes);
                    }
                    for ::parse_wiki_text::TableCell {
                        attributes,
                        content,
                        end,
                        start,
                        type_,
                    } in cells
                    {
                        let cell_el = html!(div.node);
                        let label = match type_ {
                            TableCellType::Heading => "heading",
                            TableCellType::Ordinary => "cell",
                        };
                        add_node_head(&cell_el, label, *start, *end);
                        if let Some(attributes) = attributes {
                            add_complex_attribute(&cell_el, "attributes", attributes);
                        }
                        add_complex_attribute(&cell_el, "content", content);
                        row_el.append_element(&cell_el);
                    }
                    node_el.append_element(&row_el);
                }
            }
            Node::Tag {
                end,
                name,
                nodes,
                start,
            } => {
                add_node_head(&node_el, "tag", *start, *end);
                add_text_attribute(&node_el, "name: ", name);
                add_nodes(&node_el, nodes);
            }
            Node::Text { end, value, start } => {
                add_node_head(&node_el, "text", *start, *end);
                add_text_attribute(&node_el, "value: ", value);
            }
            Node::Template {
                end,
                name,
                parameters,
                start,
            } => {
                add_node_head(&node_el, "template", *start, *end);
                add_complex_attribute(&node_el, "name", name);
                for parameter in parameters {
                    let item_el = html!(div.node);
                    add_node_head(&item_el, "parameter", parameter.start, parameter.end);
                    if let Some(name) = &parameter.name {
                        add_complex_attribute(&item_el, "name", name);
                    }
                    add_complex_attribute(&item_el, "value", &parameter.value);
                    node_el.append_element(&item_el);
                }
            }
            Node::UnorderedList { end, items, start } => {
                add_node_head(&node_el, "unordered list", *start, *end);
                for item in items {
                    let item_el = html!(div.node);
                    add_node_head(&item_el, "item", item.start, item.end);
                    add_nodes(&item_el, &item.nodes);
                    node_el.append_element(&item_el);
                }
            }
        }
        container_el.append_element(&node_el);
    }
}

fn add_text_attribute(node_el: &::HTMLTextAreaElement, label: &str, value: impl ::std::fmt::Debug) {
    let value_el = html!(div.node);
    value_el.append_text(label);
    value_el.append_text(&format!("{:?}", value));
    node_el.append_element(&value_el);
}
