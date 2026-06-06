pub trait ResultAny<T, E> {
    fn with_anyhow(self, ctx: &'static str) -> Result<T, anyhow::Error>;
}

impl<T, E: core::fmt::Debug> ResultAny<T, E> for Result<T, E> {
    fn with_anyhow(self, ctx: &'static str) -> Result<T, anyhow::Error> {
        self.map_err(|e| anyhow::anyhow!("{}: {:?}", ctx, e))
    }
}

