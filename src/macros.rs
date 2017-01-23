#[macro_export]
macro_rules! url
{
	($api_class:expr, $api_function:expr) =>
	(
		{
			use BASE_URL;
			use API_VERSION;
			debug!("Requesting url: {:?}",
				format!("{}/{}/{}/{}", BASE_URL, API_VERSION, $api_class, $api_function));
			&format!("{}/{}/{}/{}", BASE_URL, API_VERSION, $api_class, $api_function)
		}
	);
}

#[macro_export]
macro_rules! check_error
{
	($error:expr, $code:expr, $message:expr) =>
	(
		{
			use error::*;
			if $error == 1
			{
				error!("Found error: {}:{}", $code, DecodeErrorCode::decode($code));
				return Err(Error::ProxerError($code, $message))
			}
		}
	);
}

#[macro_export]
macro_rules! param_build
{
	( $( $name:expr => $value:expr ),* ) =>
	(
		{
			let mut body: String = String::new();
			$(
				match $value
				{
					None => {},
					Some(r) => if body.len() <= 1
					{
						body.push_str(&format!("{}={}", $name, r));
					}
					else
					{
						body.push_str(&format!("&{}={}", $name, r));
					},
				}
			)*
			debug!("Build body with: {:?}", &body);
			body
		}
	);
}

#[macro_export]
macro_rules! check_data
{
	( $data:expr ) =>
	(
		{
			match $data
			{
				None =>
				{
					error!("The received data was none.");
					Err(Error::Other(format!("Found Empty data after error check.")))
				},
				Some(r) => Ok(r),
			}
		}
	);
}
