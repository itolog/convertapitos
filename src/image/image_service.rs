use salvo::handler;

#[handler]
pub fn convert_image() -> &'static str {
    "HEllo"
}