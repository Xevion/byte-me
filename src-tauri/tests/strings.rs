use byte_me_lib::strings::{transform_filename, truncate_string};

#[test]
fn test_transform_filename() {
    // Test cases focusing on practical, readable outputs

    // 1. Short filenames should remain unchanged
    assert_eq!(transform_filename("test.mp4", 20), "test.mp4");
    assert_eq!(transform_filename("short.txt", 15), "short.txt");
    assert_eq!(transform_filename("a.b", 10), "a.b");

    // 2. No extension cases - preserve meaningful start and end
    assert_eq!(transform_filename("short_name", 15), "short_name");
    assert_eq!(
        transform_filename("very_long_document_name", 15),
        "ver...ment_name"
    );
    assert_eq!(
        transform_filename("medium_length_name", 13),
        "med...th_name"
    );

    // 3. Normal extension cases (preserving extension)
    assert_eq!(
        transform_filename("very_long_video_file_name.mp4", 18),
        "ver...ile_name.mp4"
    );
    assert_eq!(
        transform_filename("document_with_long_name.pdf", 15),
        "doc..._name.pdf"
    );
    assert_eq!(
        transform_filename("image_file_name.jpeg", 15),
        "ima...name.jpeg"
    );

    // 4. Numeric sequences (like user's example) - preserve start and end numbers
    assert_eq!(
        transform_filename("43509374693.TS.mp4", 15),
        "435...93.TS.mp4"
    );
    assert_eq!(
        transform_filename("20231201_video.mp4", 15),
        "202...video.mp4"
    );
    assert_eq!(transform_filename("file_v2.1.3.tar", 12), "fi...1.3.tar");

    // 5. Long extensions (treated as part of filename)
    assert_eq!(
        transform_filename("file.verylongextension", 15),
        "fil...extension"
    );
    assert_eq!(
        transform_filename("document.backup_old", 15),
        "doc...ackup_old"
    );

    // 6. Edge cases
    assert_eq!(transform_filename("", 10), "");
    assert_eq!(transform_filename("a", 0), "");
    assert_eq!(transform_filename("test", 4), "test");
    assert_eq!(transform_filename("test", 3), "tes");
    assert_eq!(transform_filename("ab", 2), "ab");

    // 7. Very short limits - graceful degradation
    assert_eq!(transform_filename("test.mp4", 8), "test.mp4");
    assert_eq!(transform_filename("verylongname", 8), "ve...ame");
    assert_eq!(transform_filename("test.mp4", 7), "tes.mp4");
    assert_eq!(transform_filename("hello.txt", 9), "hello.txt");

    // 8. Extension edge cases
    assert_eq!(transform_filename("file.", 10), "file.");
    assert_eq!(transform_filename(".hidden", 10), ".hidden");
    assert_eq!(transform_filename("test.a", 10), "test.a");

    // 9. Real-world examples
    assert_eq!(
        transform_filename("IMG_20231201_143022.jpg", 15),
        "IMG...43022.jpg"
    );
    assert_eq!(
        transform_filename("meeting_recording_final_v2.mp4", 20),
        "mee...g_final_v2.mp4"
    );
    assert_eq!(
        transform_filename("my document (copy).docx", 15),
        "my ...opy).docx"
    );
}

#[test]
fn test_truncate_string() {
    // Test the helper function directly
    assert_eq!(truncate_string("hello", 10), "hello");
    assert_eq!(truncate_string("hello", 5), "hello");
    assert_eq!(truncate_string("hello_world", 8), "he...rld");
    assert_eq!(truncate_string("test", 4), "test");
    assert_eq!(truncate_string("test", 3), "tes");
    assert_eq!(truncate_string("ab", 2), "ab");
    assert_eq!(truncate_string("a", 1), "a");
    assert_eq!(truncate_string("hello", 1), "h");
    assert_eq!(truncate_string("hello", 0), "");
    assert_eq!(truncate_string("very_long_name", 10), "ver...name");
    assert_eq!(truncate_string("document_name", 9), "doc...ame");
}
