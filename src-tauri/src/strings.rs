/// Transforms a filename to fit within a character limit while preserving the most useful context
///
/// This function prioritizes preserving:
/// 1. File extension (if reasonable length ≤ 5 chars including dot)
/// 2. Beginning of filename (for identification)
/// 3. End of filename before extension (often contains important info like numbers)
///
/// # Arguments
/// * `filename` - The filename to transform
/// * `limit` - Maximum number of characters
///
/// # Returns
/// * Transformed filename that fits within the limit, using ellipsis (...) to indicate truncation
///
/// # Examples
/// ```
/// use byte_me_lib::strings::transform_filename;
///
/// // Short filenames remain unchanged
/// assert_eq!(transform_filename("test.mp4", 20), "test.mp4");
///
/// // Long filename with extension - preserve extension and context
/// assert_eq!(transform_filename("very_long_video_file_name.mp4", 18), "ver...ile_name.mp4");
///
/// // Numeric sequences - preserve start and end numbers
/// assert_eq!(transform_filename("43509374693.TS.mp4", 15), "435...93.TS.mp4");
///
/// // No extension - preserve start and end of name
/// assert_eq!(transform_filename("very_long_document_name", 15), "ver...ment_name");
///
/// // Long extension treated as part of name
/// assert_eq!(transform_filename("file.verylongextension", 15), "fil...extension");
/// ```
pub fn transform_filename(filename: &str, limit: usize) -> String {
    // Handle edge cases
    if limit == 0 || filename.is_empty() {
        return String::new();
    }

    if filename.len() <= limit {
        return filename.to_string();
    }

    // Find potential extension (last dot, not at start or end)
    let extension_start = filename
        .rfind('.')
        .filter(|&pos| pos > 0 && pos < filename.len() - 1);

    let (name_part, extension_part) = if let Some(ext_pos) = extension_start {
        let ext = &filename[ext_pos..];
        // Only treat as extension if it's reasonable length (≤ 5 chars including dot)
        // and doesn't contain additional dots (compound extensions like .TS.mp4)
        if ext.len() <= 5 && !ext[1..].contains('.') {
            (&filename[..ext_pos], ext)
        } else {
            (filename, "")
        }
    } else {
        (filename, "")
    };

    // If even just the extension is too long, truncate the whole thing
    if extension_part.len() >= limit {
        return truncate_string(filename, limit);
    }

    // Calculate space available for the name part
    let name_limit = limit - extension_part.len();

    // If name fits in available space, no truncation needed
    if name_part.len() <= name_limit {
        return filename.to_string();
    }

    // Need to truncate the name part
    let truncated_name = truncate_string(name_part, name_limit);
    format!("{}{}", truncated_name, extension_part)
}

/// Helper function to truncate a string with ellipsis, preserving start and end context
pub fn truncate_string(s: &str, limit: usize) -> String {
    if s.len() <= limit {
        return s.to_string();
    }

    // For very small limits, just truncate without ellipsis
    if limit < 5 {
        return s.chars().take(limit).collect();
    }

    // For limits 5 and above, use start + "..." + end pattern
    // Strategy: Use 3 chars for ellipsis, split remaining between start and end
    // But ensure we get meaningful chunks from both ends

    let available_for_content = limit - 3; // Reserve 3 for "..."

    // Determine start and end characters based on available space
    let (start_chars, end_chars) = if available_for_content <= 4 {
        // Very limited space: minimal start, rest for end
        (1, available_for_content - 1)
    } else if available_for_content <= 6 {
        // Medium space: balanced approach
        let start = available_for_content / 2;
        (start, available_for_content - start)
    } else {
        // Plenty of space: cap start at 3, use more for end to preserve context
        let start = 3;
        (start, available_for_content - start)
    };

    let start: String = s.chars().take(start_chars).collect();
    let end: String = s
        .chars()
        .rev()
        .take(end_chars)
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    format!("{}...{}", start, end)
}
