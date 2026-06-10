pub trait ConfigModule {
    fn defaults() -> Self
    where
        Self: Sized;
}
