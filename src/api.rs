use crate::cli::Commands;
use dotenvy::dotenv;
use std::env;
use url::Url;

pub fn get_api_key() -> Result<String, String> {
    dotenv().ok();
    env::var("KRDICT_API_KEY")
        .map_err(|_| "KRDICT_API_KEY not found in environment. Please set it in .env file or environment variables.".to_string())
}

pub fn build_search_url(api_key: &str, args: &Commands) -> Result<String, String> {
    if let Commands::Search {
        query,
        start,
        num,
        sort,
        part,
        translated,
        trans_lang,
        advanced,
        target,
        lang,
        method,
        type1,
        type2,
        level,
        pos,
        multimedia,
        letter_s,
        letter_e,
        sense_cat,
        subject_cat,
    } = args
    {
        let mut url = Url::parse("https://krdict.korean.go.kr/api/search")
            .map_err(|e| format!("Failed to parse base URL: {}", e))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("key", api_key);
            query_pairs.append_pair("q", query);
            query_pairs.append_pair("start", &start.to_string());
            query_pairs.append_pair("num", &num.to_string());
            query_pairs.append_pair("sort", sort.as_str());
            query_pairs.append_pair("part", part.as_str());
            query_pairs.append_pair("translated", if *translated { "y" } else { "n" });
            query_pairs.append_pair("trans_lang", trans_lang);
            query_pairs.append_pair("advanced", if *advanced { "y" } else { "n" });

            if let Some(t) = target {
                query_pairs.append_pair("target", &t.to_string());
            }
            if let Some(l) = lang {
                query_pairs.append_pair("lang", &l.to_string());
            }
            if let Some(m) = method {
                query_pairs.append_pair("method", m.as_str());
            }
            if let Some(t1) = type1 {
                query_pairs.append_pair("type1", t1);
            }
            if let Some(t2) = type2 {
                query_pairs.append_pair("type2", t2);
            }
            if let Some(lvl) = level {
                query_pairs.append_pair("level", lvl);
            }
            if let Some(p) = pos {
                query_pairs.append_pair("pos", p);
            }
            if let Some(mm) = multimedia {
                query_pairs.append_pair("multimedia", mm);
            }
            if let Some(ls) = letter_s {
                query_pairs.append_pair("letter_s", &ls.to_string());
            }
            if let Some(le) = letter_e {
                query_pairs.append_pair("letter_e", &le.to_string());
            }
            if let Some(sc) = sense_cat {
                query_pairs.append_pair("sense_cat", sc);
            }
            if let Some(subj) = subject_cat {
                query_pairs.append_pair("subject_cat", subj);
            }
        }

        Ok(url.to_string())
    } else {
        Err("Invalid command".to_string())
    }
}

pub fn build_view_url(api_key: &str, args: &Commands) -> Result<String, String> {
    if let Commands::View {
        method,
        query,
        translated,
        trans_lang,
    } = args
    {
        let mut url = Url::parse("https://krdict.korean.go.kr/api/view")
            .map_err(|e| format!("Failed to parse base URL: {}", e))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("key", api_key);
            query_pairs.append_pair("method", method.as_str());
            query_pairs.append_pair("q", query);
            query_pairs.append_pair("translated", if *translated { "y" } else { "n" });
            query_pairs.append_pair("trans_lang", trans_lang);
        }

        Ok(url.to_string())
    } else {
        Err("Invalid command".to_string())
    }
}

pub fn make_request(url: &str) -> Result<String, String> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| format!("Request failed: {}", e))?;

    response.into_body()
        .read_to_string()
        .map_err(|e| format!("Failed to read response: {}", e))
}