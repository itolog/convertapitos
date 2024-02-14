use salvo::handler;

#[handler]
pub fn hello() -> &'static str {
    "HEllo"
}