/// Represents a known file type, including its name, extension, and hex digits.
#[derive(Debug)]
pub(crate) struct FileSignature {
    name: &'static str,
    extension: &'static str,
    hex_digits: &'static [u8],
}

impl FileSignature {
    pub(crate) const fn new(
        name: &'static str,
        extension: &'static str,
        hex_digits: &'static [u8],
    ) -> Self {
        FileSignature {
            extension,
            name,
            hex_digits,
        }
    }
}
