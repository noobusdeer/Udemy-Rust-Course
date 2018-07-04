#[cfg(test)]
mod tests{

    extern crate phrases;

    #[test]
    fn russian_greeting_correct() {
        assert_eq!("привет", phrases::greetings::russian::hello());
    }

    #[test]
    #[should_panic]
    fn russian_greeting_incorrect() {
        assert_eq!("helloooo", phrases::greetings::russian::hello());
    }

    #[test]
    #[ignore]
    fn russian_greeting_ignore() {
        assert_eq!("привет", phrases::greetings::russian::hello());
    }
}
