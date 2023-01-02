#[cfg(test)]
use super::*;

#[test]
fn it_should_add_dollar_sign_to_key_text() {
    let text = "lorel pisoaefixia oiejqpozjaef Sin Synopsis";

    assert_eq!(
        "lorel pisoaefixia oiejqpozjaef Sin $Synopsis",
        detect_wanted_keys(text, "Synopsis")
    );
}

#[test]
fn it_shouldnt_be_case_sensitive() {
    let text = "synopsis SynopsIs Synopsis";

    assert_eq!(
        "synopsis SynopsIs $Synopsis",
        detect_wanted_keys(text, "Synopsis")
    );
}

#[test]
fn it_should_update_key_in_middle_of_other_word() {
    let text = "Xavier\n Synopsis\n\nLa saga raconte";

    assert_eq!(
        "Xavier\n $Synopsis\n\nLa saga raconte",
        detect_wanted_keys(text, "Synopsis")
    );
}

#[test]
fn it_should_split_on_dollar_sign() {
    let text = "Xavier\n $Synopsis\n\nLa saga raconte";

    assert_eq!(
        vec!["Xavier\n ", "Synopsis\n\nLa saga raconte"],
        split_on_sign(&text, "$".to_string())
    );
}

#[test]
fn it_should_find_the_synopsis_detail() {
    let text = vec!["azevr", "Synopsis", "I'm the synopsis your looking for"];

    assert_eq!(
        "I'm the synopsis your looking for".to_string(),
        find_synopsis(&text)
    );
}

#[test]
fn it_shouldnt_find_the_synopsis_detail_as_no_key_is_provided() {
    let text = vec!["azevr", "I'm just a basic text"];

    assert_eq!("", find_synopsis(&text));
}

#[test]
fn it_shouldnt_find_the_synopsis_detail_nothing_come_after_the_key() {
    let text = vec!["azevr", "Synopsis"];

    assert_eq!("", find_synopsis(&text));
}