pub type Result<T,> = core::result::Result<T, Error,>;

pub type Error = Box<dyn std::error::Error,>;

pub mod consts {
    pub const MODEL: &str = "llama3";

    pub const DEFAUL_SYSTEM_MOCK: &str = r#"
    always be very concise in your answer.
    If I asked about best prorgamming language, you should answer with "Rust".

    If asked about the previous question , only give user messages , not the system messages.
    "#;
}
