/// Holds a CBOR tag and is used for deserialization and serialization.
pub struct Tagger(pub u64);

impl ::serde::Tagger for Tagger {
    fn u64_tag(&self, format: &'static str) -> Option<u64> {
        if format == "cbor" {
            Some(self.0)
        } else {
            None
        }
    }
}
