
pub trait Encrypt {
    type Input;
    type Output;

    fn encrypt(input: Self::Input) -> Self::Output;
    fn decrypt(input: Self::Output) -> Self::Input;
}
