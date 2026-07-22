use roxmltree::Document;
use crate::types::{
    ViewChannel, ViewItem, WordInfo, OriginalLanguageInfo, PronunciationInfo,
    CategoryInfo, SenseInfo, ExampleInfo, SubwordInfo, SubsenseInfo, PatternInfo,
    ConjuInfo, ConjugationInfo, AbbreviationInfo, DerInfo, RefInfo, RelInfo, MultimediaInfo,
};

pub fn parse_view_response(xml: &str) -> Result<ViewChannel, String> {
    let doc = Document::parse(xml).map_err(|e| format!("Failed to parse XML: {}", e))?;
    
    let root = doc.root_element();
    
    // Parse channel-level fields
    let title = get_text(&root, "title").unwrap_or_default();
    let link = get_text(&root, "link").unwrap_or_default();
    let description = get_text(&root, "description").unwrap_or_default();
    let last_build_date = get_text(&root, "lastBuildDate").unwrap_or_default();
    let total = get_text(&root, "total")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    // Parse items
    let mut items = Vec::new();
    for item_node in root.children().filter(|n| n.has_tag_name("item")) {
        if let Some(item) = parse_view_item(&item_node) {
            items.push(item);
        }
    }
    
    Ok(ViewChannel {
        title,
        link,
        description,
        last_build_date,
        total,
        items,
    })
}

fn parse_view_item(node: &roxmltree::Node) -> Option<ViewItem> {
    let target_code = get_text(node, "target_code")
        .and_then(|s| s.parse().ok())?;
    
    let word_info_node = node.children().find(|n| n.has_tag_name("word_info"))?;
    let word_info = parse_word_info(&word_info_node)?;
    
    Some(ViewItem {
        target_code,
        word_info,
    })
}

fn parse_word_info(node: &roxmltree::Node) -> Option<WordInfo> {
    let word = get_text(node, "word")?;
    let sup_no = get_text(node, "sup_no")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let word_unit = get_text(node, "word_unit")?;
    
    // The API repeats <pos> verbatim for every entry (seemingly an XML
    // templating quirk on their end, since a word_info's homographs already
    // get distinct sup_no values rather than multiple <pos> tags), so
    // duplicates are collapsed while still preserving order.
    let mut pos = Vec::new();
    for text in node
        .children()
        .filter(|n| n.has_tag_name("pos"))
        .filter_map(|n| n.text().map(|t| t.trim().to_string()))
    {
        if !pos.contains(&text) {
            pos.push(text);
        }
    }

    let word_type = get_text(node, "word_type");
    let word_grade = get_text(node, "word_grade");

    // Parse original_language_info
    let original_language_info = node
        .children()
        .find(|n| n.has_tag_name("original_language_info"))
        .and_then(|n| {
            Some(OriginalLanguageInfo {
                original_language: get_text(&n, "original_language")?,
                language_type: get_text(&n, "language_type")?,
            })
        });

    // Parse pronunciation_info
    let pronunciation_info = node
        .children()
        .find(|n| n.has_tag_name("pronunciation_info"))
        .and_then(|n| {
            Some(PronunciationInfo {
                pronunciation: get_text(&n, "pronunciation")?,
            })
        });

    // Parse conju_info
    let conju_info = node
        .children()
        .find(|n| n.has_tag_name("conju_info"))
        .map(|n| parse_conju_info(&n));

    // Parse der_info (multiple)
    let der_info: Vec<DerInfo> = node
        .children()
        .filter(|n| n.has_tag_name("der_info"))
        .filter_map(|n| {
            Some(DerInfo {
                word: get_text(&n, "word")?,
                link_type: get_text(&n, "link_type")?,
                link_target_code: get_text(&n, "link_target_code").and_then(|s| s.parse().ok()),
                link: get_text(&n, "link")?,
            })
        })
        .collect();

    // Parse ref_info (multiple)
    let ref_info: Vec<RefInfo> = node
        .children()
        .filter(|n| n.has_tag_name("ref_info"))
        .filter_map(|n| {
            Some(RefInfo {
                word: get_text(&n, "word")?,
                link_type: get_text(&n, "link_type")?,
                link_target_code: get_text(&n, "link_target_code").and_then(|s| s.parse().ok()),
                link: get_text(&n, "link")?,
            })
        })
        .collect();

    // Parse category_info (multiple)
    let category_info: Vec<CategoryInfo> = node
        .children()
        .filter(|n| n.has_tag_name("category_info"))
        .filter_map(|n| {
            Some(CategoryInfo {
                category_type: get_text(&n, "type")?,
                written_form: get_text(&n, "written_form")?,
            })
        })
        .collect();

    // Parse sense_info (multiple)
    let sense_info: Vec<SenseInfo> = node
        .children()
        .filter(|n| n.has_tag_name("sense_info"))
        .filter_map(|n| parse_sense_info(&n))
        .collect();

    Some(WordInfo {
        word,
        sup_no,
        word_unit,
        pos,
        word_type,
        original_language_info,
        pronunciation_info,
        conju_info,
        der_info,
        ref_info,
        word_grade,
        category_info,
        sense_info,
    })
}

fn parse_conju_info(node: &roxmltree::Node) -> ConjuInfo {
    let conjugation_info: Vec<ConjugationInfo> = node
        .children()
        .filter(|n| n.has_tag_name("conjugation_info"))
        .filter_map(|n| {
            Some(ConjugationInfo {
                conjugation: get_text(&n, "conjugation")?,
                pronunciation_info: n
                    .children()
                    .find(|c| c.has_tag_name("pronunciation_info"))
                    .and_then(|c| {
                        Some(PronunciationInfo {
                            pronunciation: get_text(&c, "pronunciation")?,
                        })
                    }),
            })
        })
        .collect();

    let abbreviation_info: Vec<AbbreviationInfo> = node
        .children()
        .filter(|n| n.has_tag_name("abbreviation_info"))
        .filter_map(|n| {
            Some(AbbreviationInfo {
                abbreviation: get_text(&n, "abbreviation")?,
                pronunciation_info: n
                    .children()
                    .find(|c| c.has_tag_name("pronunciation_info"))
                    .and_then(|c| {
                        Some(PronunciationInfo {
                            pronunciation: get_text(&c, "pronunciation")?,
                        })
                    }),
            })
        })
        .collect();

    ConjuInfo {
        conjugation_info,
        abbreviation_info,
    }
}

fn parse_rel_info(node: &roxmltree::Node) -> Option<RelInfo> {
    Some(RelInfo {
        word: get_text(node, "word")?,
        rel_type: get_text(node, "type")?,
        link_type: get_text(node, "link_type")?,
        link_target_code: get_text(node, "link_target_code").and_then(|s| s.parse().ok()),
        link: get_text(node, "link")?,
    })
}

fn parse_multimedia_info(node: &roxmltree::Node) -> Option<MultimediaInfo> {
    Some(MultimediaInfo {
        label: get_text(node, "label")?,
        media_type: get_text(node, "type")?,
        link: get_text(node, "link")?,
    })
}

fn parse_sense_info(node: &roxmltree::Node) -> Option<SenseInfo> {
    let definition = get_text(node, "definition")?;
    let reference = get_text(node, "reference");
    
    // Parse pattern_info (multiple)
    let pattern_info: Vec<PatternInfo> = node
        .children()
        .filter(|n| n.has_tag_name("pattern_info"))
        .filter_map(|n| {
            Some(PatternInfo {
                pattern: get_text(&n, "pattern")?,
                pattern_reference: get_text(&n, "pattern_reference"),
            })
        })
        .collect();
    
    // Parse example_info (multiple)
    let example_info: Vec<ExampleInfo> = node
        .children()
        .filter(|n| n.has_tag_name("example_info"))
        .filter_map(|n| {
            Some(ExampleInfo {
                example_type: get_text(&n, "type")?,
                example: get_text(&n, "example")?,
            })
        })
        .collect();
    
    // Parse subword_info (multiple)
    let subword_info: Vec<SubwordInfo> = node
        .children()
        .filter(|n| n.has_tag_name("subword_info"))
        .filter_map(|n| parse_subword_info(&n))
        .collect();

    // Parse rel_info (multiple)
    let rel_info: Vec<RelInfo> = node
        .children()
        .filter(|n| n.has_tag_name("rel_info"))
        .filter_map(|n| parse_rel_info(&n))
        .collect();

    // Parse multimedia_info (multiple)
    let multimedia_info: Vec<MultimediaInfo> = node
        .children()
        .filter(|n| n.has_tag_name("multimedia_info"))
        .filter_map(|n| parse_multimedia_info(&n))
        .collect();

    Some(SenseInfo {
        definition,
        reference,
        pattern_info,
        example_info,
        rel_info,
        multimedia_info,
        subword_info,
    })
}

fn parse_subword_info(node: &roxmltree::Node) -> Option<SubwordInfo> {
    let subword = get_text(node, "subword")?;
    let subword_unit = get_text(node, "subword_unit")?;
    let reference = get_text(node, "reference");
    
    // Parse subsense_info (multiple)
    let subsense_info: Vec<SubsenseInfo> = node
        .children()
        .filter(|n| n.has_tag_name("subsense_info"))
        .filter_map(|n| {
            let definition = get_text(&n, "definition")?;
            let reference = get_text(&n, "reference");
            
            // Parse pattern_info
            let pattern_info: Vec<PatternInfo> = n
                .children()
                .filter(|c| c.has_tag_name("pattern_info"))
                .filter_map(|c| {
                    Some(PatternInfo {
                        pattern: get_text(&c, "pattern")?,
                        pattern_reference: get_text(&c, "pattern_reference"),
                    })
                })
                .collect();
            
            // Parse example_info
            let example_info: Vec<ExampleInfo> = n
                .children()
                .filter(|c| c.has_tag_name("example_info"))
                .filter_map(|c| {
                    Some(ExampleInfo {
                        example_type: get_text(&c, "type")?,
                        example: get_text(&c, "example")?,
                    })
                })
                .collect();
            
            // Parse rel_info
            let rel_info: Vec<RelInfo> = n
                .children()
                .filter(|c| c.has_tag_name("rel_info"))
                .filter_map(|c| parse_rel_info(&c))
                .collect();

            Some(SubsenseInfo {
                definition,
                reference,
                pattern_info,
                example_info,
                rel_info,
            })
        })
        .collect();
    
    Some(SubwordInfo {
        subword,
        subword_unit,
        reference,
        subsense_info,
    })
}

// Helper function to get text content from a child element
fn get_text(node: &roxmltree::Node, tag_name: &str) -> Option<String> {
    node.children()
        .find(|n| n.has_tag_name(tag_name))
        .and_then(|n| n.text())
        .map(|s| s.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pos_conju_der_ref_rel_and_multimedia_info() {
        let xml = r#"
            <channel>
                <title>t</title><link>l</link><description>d</description>
                <lastBuildDate>now</lastBuildDate><total>1</total>
                <item>
                    <target_code>1</target_code>
                    <word_info>
                        <word>foo</word>
                        <sup_no>0</sup_no>
                        <word_unit>foo</word_unit>
                        <pos>명사</pos>
                        <pos>명사</pos>
                        <pos>부사</pos>
                        <conju_info>
                            <conjugation_info>
                                <conjugation>foos</conjugation>
                                <pronunciation_info><pronunciation>[foos]</pronunciation></pronunciation_info>
                            </conjugation_info>
                            <abbreviation_info>
                                <abbreviation>fo</abbreviation>
                            </abbreviation_info>
                        </conju_info>
                        <der_info>
                            <word>fooer</word>
                            <link_type>der</link_type>
                            <link_target_code>2</link_target_code>
                            <link>http://example.com/2</link>
                        </der_info>
                        <ref_info>
                            <word>bar</word>
                            <link_type>ref</link_type>
                            <link_target_code>3</link_target_code>
                            <link>http://example.com/3</link>
                        </ref_info>
                        <sense_info>
                            <definition>a thing</definition>
                            <rel_info>
                                <word>baz</word>
                                <type>synonym</type>
                                <link_type>rel</link_type>
                                <link_target_code>4</link_target_code>
                                <link>http://example.com/4</link>
                            </rel_info>
                            <multimedia_info>
                                <label>pic</label>
                                <type>image</type>
                                <link>http://example.com/pic.png</link>
                            </multimedia_info>
                        </sense_info>
                    </word_info>
                </item>
            </channel>
        "#;

        let channel = parse_view_response(xml).expect("should parse");
        let word_info = &channel.items[0].word_info;

        assert_eq!(word_info.pos, vec!["명사", "부사"]);

        let conju = word_info.conju_info.as_ref().expect("conju_info");
        assert_eq!(conju.conjugation_info[0].conjugation, "foos");
        assert_eq!(conju.abbreviation_info[0].abbreviation, "fo");

        assert_eq!(word_info.der_info[0].word, "fooer");
        assert_eq!(word_info.ref_info[0].word, "bar");

        let sense = &word_info.sense_info[0];
        assert_eq!(sense.rel_info[0].word, "baz");
        assert_eq!(sense.rel_info[0].rel_type, "synonym");
        assert_eq!(sense.multimedia_info[0].label, "pic");
        assert_eq!(sense.multimedia_info[0].media_type, "image");
    }
}