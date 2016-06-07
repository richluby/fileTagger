#![deny(missing_docs)]

//! Provides common interface for modules to guarantee interoperability
trait FileModule{
	/// Returns the list of tags supported by this module.
	/// The module should provide a *GENERAL* tag if possible
	/// and the file type is able to store such a tag.
	fn supported_tags() -> Vec<&'static str>;

	/// Writes the tag to a field within a particular file
	/// The parameters should be in the order
	///     (file, tag_value, tag_field)
	/// The function returns true if the write was succesful.
	fn write_tag_to_field(std::fs::File, &str, &str) -> bool;

	/// Writes the tag to a particular file
	/// The parameters should be in the order
	///     (file, tag_value)
	/// The function returns true if the write was succesful.
	fn write_tag_to_file(std::fs::File, &str) -> bool;
}
