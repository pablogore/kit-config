pub trait ConfigModule {
    const NAME: &'static str;

    fn defaults() -> Self
    where
        Self: Sized;
}
