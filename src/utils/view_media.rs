pub fn video_media_url(query: &str) -> String {
    play_video(&query[5..])
}

pub fn play_video(video_link: &str) -> String {
    format!(
        "https://vj-duardo.github.io/media-links-in-browser/?{}",
        video_link
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_media() {
        let actual = play_video("https://cdn.discordapp.com/attachments/575540483053453314/1049370018854473898/Frango_andando_shitpost240P.mp4");
        let expected = "https://vj-duardo.github.io/media-links-in-browser/?https://cdn.discordapp.com/attachments/575540483053453314/1049370018854473898/Frango_andando_shitpost240P.mp4";
        assert_eq!(actual, expected);
    }
}
