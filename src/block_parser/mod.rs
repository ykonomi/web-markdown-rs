use block::Block;
use block::BlockType;
use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("block.pest");

#[derive(Parser)]
#[grammar = "block_parser/block.pest"]
struct BlockParser;

pub fn parse(line: &str) -> Block {
    let tokens = BlockParser::parse(Rule::document, line).unwrap_or_else(|e| panic!("{}", e));

    let mut root_block = Block {
        is_closed: false,
        block_type: BlockType::Document,
        raw_text: "".to_string(),
        children: vec![],
    };

    for token in tokens {
        match token.as_rule() {
            Rule::thematic_break => {
                root_block.add(BlockType::ThematicBreaks, "".to_string());
            }
            Rule::break_line | Rule::empty => {
                root_block.add(BlockType::BreakLine, "".to_string());
            }
            Rule::paragraph => {
                let mut update = false;
                match root_block.get_prev() {
                    None => (),
                    Some(prev) => match prev {
                        Block {
                            block_type: BlockType::Paragraph,
                            ..
                        } => {
                            prev.update("\n");
                            prev.update(token.as_str());
                            update = true;
                        }
                        _ => (),
                    },
                }

                if !update {
                    root_block.add(BlockType::Paragraph, token.as_str().to_string());
                }
            }
            Rule::atx_heading1 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading1, text.as_str().to_string());
            }
            Rule::atx_heading2 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading2, text.as_str().to_string());
            }
            Rule::atx_heading3 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading3, text.as_str().to_string());
            }
            Rule::atx_heading4 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading4, text.as_str().to_string());
            }
            Rule::atx_heading5 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading5, text.as_str().to_string());
            }
            Rule::atx_heading6 => {
                let text = token.into_inner().next().unwrap();
                root_block.add(BlockType::AtxHeading6, text.as_str().to_string());
            }
            _ => (),
        }
    }
    root_block
}

#[test]
fn test_parsing_themantic_break() {
    parses_to! {
        parser: BlockParser,
        input: "***\n---\n___",
        rule: Rule::document,
        tokens: [
          thematic_break(0, 3, [
          ]),
          thematic_break(4, 7, [
          ]),
          thematic_break(8, 11, [
          ]),
        ]
    };
}

#[test]
fn test_parsing_atx_headings() {
    parses_to! {
        parser: BlockParser,
        input: "# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo",
        rule: Rule::document,
        tokens: [
          atx_heading1(0, 5, [
            text(2,5,[]),
          ]),
          atx_heading2(6, 12, [
            text(9,12,[]),
          ]),
          atx_heading3(13, 20, [
            text(17,20,[]),
          ]),
          atx_heading4(21, 29, [
            text(26,29,[]),
          ]),
          atx_heading5(30, 39, [
            text(36,39,[]),
          ]),
          atx_heading6(40, 50, [
            text(47,50,[]),
          ]),
        ]
    };
}

#[test]
fn test_parsing_paragraph() {
    parses_to! {
        parser: BlockParser,
        input: "  aaa\nbbb\n\nccc\nd d d",
        rule: Rule::document,
        tokens: [
          paragraph(2, 5, [
          ]),
          paragraph(6, 9, [
          ]),
          empty(10, 10, [
          ]),
          paragraph(11, 14, [
          ]),
          paragraph(15, 20, [
          ])
        ]
    };
}

#[test]
fn test_check_tree() {
    //let tokens = parse("  aaa\n bbb\n");
    //println!("{:?}", tokens);
}

// > Lorem ipsum dolor
// sit amet.
// > - Qui *quodsi iracundia*
// > - aliquando id
//-> document
//  -> block_quote
//       paragraph
//         "Lorem ipsum dolor\nsit amet."
//    -> list (type=bullet tight=true bullet_char=-)
//         list_item
//           paragraph
//             "Qui *quodsi iracundia*"
//      -> list_item
//        -> paragraph
//             "aliquando id"
//    root_block.add(BlockType::BlockQuote, "".to_string());
//
//    let block2 = Block {
//        is_closed: false,
//        block_type: BlockType::Document,
//        raw_text: "".to_string(),
//        children: vec![Block {
//            is_closed: false,
//            block_type: BlockType::BlockQuote,
//            raw_text: "".to_string(),
//            children: vec![
//                Block {
//                    is_closed: false,
//                    block_type: BlockType::Paragraph,
//                    raw_text: "Lorem ipsum dolor\nsit amet.".to_string(),
//                    children: vec![],
//                },
//                Block {
//                    is_closed: false,
//                    block_type: BlockType::List,
//                    raw_text: "".to_string(),
//                    children: vec![],
//                },
//            ],
//        }],
//    };
