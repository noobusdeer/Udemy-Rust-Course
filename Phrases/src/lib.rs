pub mod greetings {
    pub mod english;
    pub mod russian;
}

#[test]
fn english_greeting_correct() {
    assert_eq!("hello", greetings::english::hello());
}

#[test]
#[should_panic]
fn english_greeting_incorrect() {
    assert_eq!("helloooo", greetings::english::hello());
}

#[test]
#[ignore]
fn english_greeting_ignore() {
    assert_eq!("hello", greetings::english::hello());
}