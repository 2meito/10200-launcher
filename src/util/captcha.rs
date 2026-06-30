use nanoid::nanoid;

pub fn generate_captcha_token() -> String {
    nanoid!(256)
}