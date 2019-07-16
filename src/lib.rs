use std::io::{self, Write};

pub fn wrap(mut input: &[u8], line_len: usize, line_ending: &str, mut out: impl Write) -> io::Result<()> {
    while input.len() > line_len {
        out.write_all(&input[..line_len])?;
        out.write_all(line_ending.as_bytes())?;
        input = &input[line_len..];
    }

    out.write_all(&input[..])?;

    Ok(())
}

pub fn wrap_string(input: &str, line_len: usize, line_ending: &str) -> Result<String, std::string::FromUtf8Error> {
    let mut out = Vec::with_capacity(input.len());

    // can't fail, writing to vec is fine
    wrap(input.as_bytes(), line_len, line_ending, &mut out).unwrap();

    String::from_utf8(out)
}

#[cfg(test)]
mod tests {
    mod raw {
        use crate::*;

        #[test]
        fn untouched_if_short() {
            let input = "abc";

            let mut out = Vec::new();
            wrap(input.as_bytes(), 76, "\n", &mut out).unwrap();
            assert_eq!(out, input.as_bytes());
        }

        #[test]
        fn wraps_short() {
            let input = "abcdefghij";
            let expected = "abcde\nfghij";

            let mut out = Vec::new();
            wrap(input.as_bytes(), 5, "\n", &mut out).unwrap();
            assert_eq!(out, expected.as_bytes());
        }

        #[test]
        fn mime_style_long_lines() {
            let expected = r#"mQAzBFtZ7AQWCSsGAQQB2kcPAQEHQGvHkLPhAe6UZsg6keDVgnLhwHg0XoTjAvG0IrqeR6KPtQAf
SmFuLUVyaWsgPGpyZWRpZ2VyQG1vemlsbGEuY29tPokAewQTFggAIxYhBEnczPRGmPEZjSAz6wgQ
DBmGIxBxBQJbWewEAhsDAheAAAoJEAgQDBmGIxBxnJABAOJaDl/b/LFeTBoMMJq92rCi1ojLu7y+
ksXbETx03BenAP9jvkf3sZE/sRNnSofwW4u7XjqAUNR6pDG2Bp60vZ7MDQ=="#;

            let input = "mQAzBFtZ7AQWCSsGAQQB2kcPAQEHQGvHkLPhAe6UZsg6keDVgnLhwHg0XoTjAvG0IrqeR6KPtQAfSmFuLUVyaWsgPGpyZWRpZ2VyQG1vemlsbGEuY29tPokAewQTFggAIxYhBEnczPRGmPEZjSAz6wgQDBmGIxBxBQJbWewEAhsDAheAAAoJEAgQDBmGIxBxnJABAOJaDl/b/LFeTBoMMJq92rCi1ojLu7y+ksXbETx03BenAP9jvkf3sZE/sRNnSofwW4u7XjqAUNR6pDG2Bp60vZ7MDQ==";

            let mut out = Vec::new();
            wrap(input.as_bytes(), 76, "\n", &mut out).unwrap();
            assert_eq!(out, expected.as_bytes());
        }
    }

    mod string {
        use crate::*;

        #[test]
        fn untouched_if_short() {
            let input = "abc";

            assert_eq!(input, wrap_string(input, 76, "\n").unwrap());
        }

        #[test]
        fn wraps_short() {
            let input = "abcdefghij";
            let expected = "abcde\nfghij";

            assert_eq!(expected, wrap_string(input, 5, "\n").unwrap());
        }

        #[test]
        fn mime_style_long_lines() {
            let expected = r#"mQAzBFtZ7AQWCSsGAQQB2kcPAQEHQGvHkLPhAe6UZsg6keDVgnLhwHg0XoTjAvG0IrqeR6KPtQAf
SmFuLUVyaWsgPGpyZWRpZ2VyQG1vemlsbGEuY29tPokAewQTFggAIxYhBEnczPRGmPEZjSAz6wgQ
DBmGIxBxBQJbWewEAhsDAheAAAoJEAgQDBmGIxBxnJABAOJaDl/b/LFeTBoMMJq92rCi1ojLu7y+
ksXbETx03BenAP9jvkf3sZE/sRNnSofwW4u7XjqAUNR6pDG2Bp60vZ7MDQ=="#;

            let input = "mQAzBFtZ7AQWCSsGAQQB2kcPAQEHQGvHkLPhAe6UZsg6keDVgnLhwHg0XoTjAvG0IrqeR6KPtQAfSmFuLUVyaWsgPGpyZWRpZ2VyQG1vemlsbGEuY29tPokAewQTFggAIxYhBEnczPRGmPEZjSAz6wgQDBmGIxBxBQJbWewEAhsDAheAAAoJEAgQDBmGIxBxnJABAOJaDl/b/LFeTBoMMJq92rCi1ojLu7y+ksXbETx03BenAP9jvkf3sZE/sRNnSofwW4u7XjqAUNR6pDG2Bp60vZ7MDQ==";

            assert_eq!(expected, wrap_string(input, 76, "\n").unwrap());
        }
    }
}
