use crate::types::{SearchChannel, ViewChannel};

pub fn print_search_results(channel: &SearchChannel) {
    println!("╔═══════════════════════════════════════════════════════════════");
    println!("║ Search Results");
    println!("╠═══════════════════════════════════════════════════════════════");
    println!("║ Total: {} | Start: {} | Displayed: {}", 
        channel.total, channel.start, channel.num);
    println!("╚═══════════════════════════════════════════════════════════════\n");

    for (idx, item) in channel.items.iter().enumerate() {
        println!("┌─ Result {} ─────────────────────────────────────────────────", idx + 1);
        println!("│ Word: {}", item.word);
        println!("│ Target Code: {}", item.target_code);
        
        if let Some(pronunciation) = &item.pronunciation {
            println!("│ Pronunciation: {}", pronunciation);
        }
        if let Some(pos) = &item.pos {
            println!("│ Part of Speech: {}", pos);
        }
        if let Some(grade) = &item.word_grade {
            println!("│ Grade: {}", grade);
        }
        if let Some(origin) = &item.origin {
            println!("│ Origin: {}", origin);
        }
        
        // Print example if this is an example search
        if let Some(example) = &item.example {
            println!("│ Example: {}", example);
        }
        
        // Print senses
        if !item.senses.is_empty() {
            println!("│");
            println!("│ Definitions:");
            for sense in &item.senses {
                println!("│   {}. {}", sense.sense_order, sense.definition);
                
                // Print translations
                for trans in &sense.translations {
                    println!("│      [{}] {}", trans.trans_lang, 
                        trans.trans_dfn.as_ref().unwrap_or(&"".to_string()));
                }
            }
        }
        
        println!("│ Link: {}", item.link);
        println!("└───────────────────────────────────────────────────────────────\n");
    }
}

pub fn print_view_results(channel: &ViewChannel) {
    println!("╔═══════════════════════════════════════════════════════════════");
    println!("║ Dictionary Entry Details");
    println!("╚═══════════════════════════════════════════════════════════════\n");

    for item in &channel.items {
        let word_info = &item.word_info;
        
        println!("┌─ Entry ───────────────────────────────────────────────────────");
        println!("│ Word: {}", word_info.word);
        println!("│ Target Code: {}", item.target_code);
        println!("│ Homonym Number: {}", word_info.sup_no);
        println!("│ Word Unit: {}", word_info.word_unit);
        println!("│ Part of Speech: {}", word_info.pos);
        
        if let Some(word_type) = &word_info.word_type {
            println!("│ Word Type: {}", word_type);
        }
        if let Some(grade) = &word_info.word_grade {
            println!("│ Grade: {}", grade);
        }
        
        // Original language info
        if let Some(orig) = &word_info.original_language_info {
            println!("│");
            println!("│ Original Language: {}", orig.original_language);
            println!("│ Language Type: {}", orig.language_type);
        }
        
        // Pronunciation
        if let Some(pron) = &word_info.pronunciation_info {
            println!("│");
            println!("│ Pronunciation: {}", pron.pronunciation);
        }
        
        // Categories
        if !word_info.category_info.is_empty() {
            println!("│");
            println!("│ Categories:");
            for cat in &word_info.category_info {
                println!("│   {}: {}", cat.category_type, cat.written_form);
            }
        }
        
        // Definitions
        if !word_info.sense_info.is_empty() {
            println!("│");
            println!("│ Definitions:");
            for (idx, sense) in word_info.sense_info.iter().enumerate() {
                println!("│   {}. {}", idx + 1, sense.definition);
                
                // Examples
                if !sense.example_info.is_empty() {
                    println!("│      Examples:");
                    for ex in &sense.example_info {
                        println!("│        - {}", ex.example);
                    }
                }
                
                // Related words
                if !sense.rel_info.is_empty() {
                    println!("│      Related:");
                    for rel in &sense.rel_info {
                        println!("│        - {} ({})", rel.word, rel.rel_type);
                    }
                }
            }
        }
        
        // Derivatives
        if !word_info.der_info.is_empty() {
            println!("│");
            println!("│ Derivatives:");
            for der in &word_info.der_info {
                println!("│   - {}", der.word);
            }
        }
        
        println!("└───────────────────────────────────────────────────────────────\n");
    }
}