use vrchatapi::{apis::{self, authentication_api::verify2_fa_email_code}, models::TwoFactorEmailCode};

pub(crate) fn vrchat_api() -> bool {
    let mut config = apis::configuration::Configuration::default();
    config.basic_auth = Some((String::from(""), Some(String::from(""))));

	let email_code = TwoFactorEmailCode {
		code: String::from(""),
	};

    verify2_fa_email_code(&config, Some(email_code)).unwrap().verified
}