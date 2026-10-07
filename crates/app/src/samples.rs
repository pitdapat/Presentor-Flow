//! Sample songs added on first launch so the app can be tried straight away.
//! They are ordinary songs: edit or delete them like any other.

/// `(title, lyrics)` pairs.
pub const SAMPLE_SONGS: &[(&str, &str)] = &[
    (
        "Amazing Grace (sample)",
        // John Newton, 1779. Public domain.
        "[Verse 1]
Amazing grace, how sweet the sound
That saved a wretch like me
I once was lost, but now am found
Was blind, but now I see

[Verse 2]
'Twas grace that taught my heart to fear
And grace my fears relieved
How precious did that grace appear
The hour I first believed

[Verse 3]
Through many dangers, toils and snares
I have already come
'Tis grace hath brought me safe thus far
And grace will lead me home",
    ),
    (
        "中文测试 Chinese test (sample)",
        // Test text written for this app; not a published song.
        "[主歌 Verse]
晨光照亮新的一天
我们一同歌唱
Morning light, a brand new day
We sing as one

[副歌 Chorus]
感谢赞美 感谢赞美
从早晨到晚上
这一行很长用来测试中文在没有空格的情况下是否会自动换行显示在投影屏幕上",
    ),
    (
        "Layout test (sample)",
        // Checks wrapping and the too-long warning.
        "[Short]
One line

[Long line wraps]
This is a deliberately long line of text that should wrap onto several lines without the font getting smaller

[Too long]
Line one
Line two
Line three
Line four
Line five
Line six
Line seven
Line eight",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use presenter_core::library::Library;

    #[test]
    fn every_sample_is_a_valid_song() {
        let mut lib = Library::default();
        for (title, lyrics) in SAMPLE_SONGS {
            assert!(lib.create_song(title, lyrics).is_ok(), "{title}");
        }
    }
}
