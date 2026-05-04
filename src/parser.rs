use roxmltree::Document;
use crate::types::{
    ViewChannel, ViewItem, WordInfo, OriginalLanguageInfo, PronunciationInfo,
    CategoryInfo, SenseInfo, ExampleInfo, SubwordInfo, SubsenseInfo, PatternInfo
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
    
    // Handle duplicate pos fields - collect all and use first unique value
    let mut pos_values = Vec::new();
    for child in node.children().filter(|n| n.has_tag_name("pos")) {
        if let Some(text) = child.text() {
            pos_values.push(text.to_string());
        }
    }
    let pos = pos_values.into_iter().next()?;
    
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
        conju_info: None,
        der_info: Vec::new(),
        ref_info: Vec::new(),
        word_grade,
        category_info,
        sense_info,
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
    
    Some(SenseInfo {
        definition,
        reference,
        pattern_info,
        example_info,
        rel_info: Vec::new(),
        multimedia_info: Vec::new(),
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
            
            Some(SubsenseInfo {
                definition,
                reference,
                pattern_info,
                example_info,
                rel_info: Vec::new(),
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