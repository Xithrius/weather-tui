use std::vec::IntoIter;

use textwrap::core::display_width;

#[allow(dead_code)]
pub fn align_text(text: &str, alignment: &str, maximum_length: usize) -> String {
    assert!(
        maximum_length >= 1,
        "Parameter of 'maximum_length' cannot be below 1."
    );

    // Compute the display width of `text` with support of emojis and CJK characters
    let mut dw = display_width(text);

    if dw > maximum_length {
        dw = maximum_length;
    }

    match alignment {
        "right" => format!("{}{}", " ".repeat(maximum_length - dw), text),
        "center" => {
            let side_spaces = " ".repeat(
                ((maximum_length as u16 / 2) - (((dw / 2) as f32).floor() as u16)) as usize,
            );
            format!("{}{}{}", side_spaces, text, side_spaces)
        }
        _ => text.to_string(),
    }
}

pub fn vector_column_max<T>(v: &[Vec<T>]) -> IntoIter<u16>
where
    T: AsRef<str>,
{
    assert!(
        !v.is_empty(),
        "Vector length should be greater than or equal to 1."
    );

    let column_max = |vec: &[Vec<T>], index: usize| -> u16 {
        vec.iter().map(|v| v[index].as_ref().len()).max().unwrap() as u16
    };

    let column_amount = v[0].len();

    let mut column_max_lengths: Vec<u16> = vec![];

    for i in 0..column_amount {
        column_max_lengths.push(column_max(v, i));
    }

    column_max_lengths.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Parameter of 'maximum_length' cannot be below 1.")]
    fn test_text_align_maximum_length() {
        align_text("", "left", 0);
    }

    #[test]
    fn test_text_align_left() {
        assert_eq!(align_text("a", "left", 10), "a".to_string());
        assert_eq!(align_text("a", "left", 1), "a".to_string());
    }

    #[test]
    fn test_text_align_right() {
        assert_eq!(
            align_text("a", "right", 10),
            format!("{}{}", " ".repeat(9), "a")
        );
        assert_eq!(align_text("a", "right", 1), "a".to_string());
        assert_eq!(align_text("你好", "right", 5), " 你好");
        assert_eq!(align_text("👑123", "right", 6), " 👑123");
    }

    #[test]
    fn test_text_align_center() {
        assert_eq!(
            align_text("a", "center", 11),
            format!("{}{}{}", " ".repeat(5), "a", " ".repeat(5))
        );
        assert_eq!(align_text("a", "center", 1), "a".to_string());
        assert_eq!(align_text("你好", "center", 6), " 你好 ");
        assert_eq!(align_text("👑123", "center", 7), " 👑123 ");
    }

    #[test]
    #[should_panic(expected = "Vector length should be greater than or equal to 1.")]
    fn test_vector_column_max_empty_vector() {
        let vec: Vec<Vec<String>> = vec![];

        vector_column_max(&vec);
    }

    #[test]
    fn test_vector_column_max_reference_strings() {
        let vec = vec![vec!["", "s"], vec!["longer string", "lll"]];

        let mut output_vec_all = vector_column_max(&vec);

        assert_eq!(output_vec_all.next(), Some(13));
        assert_eq!(output_vec_all.next(), Some(3));
    }

    #[test]
    fn test_vector_column_max_strings() {
        let vec = vec![
            vec![String::new(), "another".to_string()],
            vec![String::new(), "the last string".to_string()],
        ];

        let mut output_vec_all = vector_column_max(&vec);

        assert_eq!(output_vec_all.next(), Some(0));
        assert_eq!(output_vec_all.next(), Some(15));
    }
}
