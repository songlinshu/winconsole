/// Describes an error related to an argument.
#[derive(Clone, Debug, PartialEq)]
pub struct ArgumentError {
	/// The name of the offending argument.
	pub argument: String,
	/// A message describing the error.
	pub message: String
}

impl ArgumentError {
	/**
	 Creates a new ArgumentError.
	
	 # Arguments
	 * `argument` - The name of the offending argument.
	 * `message` - A message describing the error.
	 */
	pub fn new(argument: &str, message: &str) -> ArgumentError {
		ArgumentError {
			argument: String::from(argument),
			message: String::from(message)
		}
	}
}

impl_err!(ArgumentError, "invalid argument",
	"argument {} is invalid: {}", argument, message);