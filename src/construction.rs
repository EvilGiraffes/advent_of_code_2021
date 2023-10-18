pub trait TryConstructor {
    type Constructing;
    type Error;
    fn new() -> Result<Self::Constructing, Self::Error>;
    fn new_boxed() -> Result<Box<Self::Constructing>, Self::Error> {
        Self::new().map(|x| Box::new(x))
    }
}
