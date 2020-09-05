extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_youtube_url(query: &str) -> String {
    if query == "yt" {
        String::from("https://youtube.com")
    } else if &query[..4] == "yt @" {
        construct_youtube_profile_url(&query[4..])
    } else {
        construct_youtube_search_url(&query[3..])
    }
}

pub fn construct_youtube_profile_url(profile: &str) -> String {
    format!("https://youtube.com/c/{}", profile)
}

pub fn construct_youtube_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT);
    let youtube_search_url = format!("https://youtube.com/search?q={}", encoded_query);

    youtube_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_youtube_url() {
        let fake_query = "yt";
        assert_eq!(construct_youtube_url(fake_query), "https://youtube.com");
    }

    #[test]
    fn test_construct_youtube_url_query() {
        let fake_query = "yt hello world";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://youtube.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_youtube_url_profile() {
        let fake_query = "yt @JonGjengset";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://youtube.com/c/JonGjengset"
        );
    }

    #[test]
    fn test_construct_youtube_profile_url() {
        let fake_profile = "jongjengset";
        assert_eq!(
            construct_youtube_profile_url(fake_profile),
            "https://youtube.com/c/jongjengset"
        );
    }

    #[test]
    fn test_construct_youtube_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_youtube_search_url(fake_query),
            "https://youtube.com/search?q=hello%20world"
        );
    }
}
