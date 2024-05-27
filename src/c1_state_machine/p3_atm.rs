//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

use std::clone;

use super::StateMachine;

/// The keys on the ATM keypad
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum Key {
    One,
    Two,
    Three,
    Four,
    Enter,
}

/// Something you can do to the ATM
pub enum Action {
    /// Swipe your card at the ATM. The attached value is the hash of the pin
    /// that should be keyed in on the keypad next.
    SwipeCard(u64),
    /// Press a key on the keypad
    PressKey(Key),
}

fn clone_and_add(vec: &Vec<Key>, element: Key) -> Vec<Key> {
    let mut new_vec = vec.clone();
    new_vec.push(element);
    new_vec
}

/// The various states of authentication possible with the ATM
#[derive(Debug, PartialEq, Eq, Clone)]
enum Auth {
    /// No session has begun yet. Waiting for the user to swipe their card
    Waiting,
    /// The user has swiped their card, providing the enclosed PIN hash.
    /// Waiting for the user to key in their pin
    Authenticating(u64),
    /// The user has authenticated. Waiting for them to key in the amount
    /// of cash to withdraw
    Authenticated,
}

/// The ATM. When a card is swiped, the ATM learns the correct pin's hash.
/// It waits for you to key in your pin. You can press as many numeric keys as
/// you like followed by enter. If the pin is incorrect, your card is returned
/// and the ATM automatically goes back to the main menu. If your pin is correct,
/// the ATM waits for you to key in an amount of money to withdraw. Withdraws
/// are bounded only by the cash in the machine (there is no account balance).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Atm {
    /// How much money is in the ATM
    cash_inside: u64,
    /// The machine's authentication status.
    expected_pin_hash: Auth,
    /// All the keys that have been pressed since the last `Enter`
    keystroke_register: Vec<Key>,
}

impl StateMachine for Atm {
    // Notice that we are using the same type for the state as we are using for the machine this time.
    type State = Self;
    type Transition = Action;

    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State {
        match t {
            Action::SwipeCard(pin_hash) => {
                if starting_state.expected_pin_hash != Auth::Waiting {
                    return starting_state.clone();
                }

                Atm {
                    cash_inside: starting_state.cash_inside,
                    expected_pin_hash: Auth::Authenticating(*pin_hash),
                    keystroke_register: Vec::new(),
                }
            } ,
            Action::PressKey(Key::Enter) => {
                let pin = starting_state.keystroke_register.clone();
                let pin_hash = crate::hash(&pin);
                match &starting_state.expected_pin_hash {
                    Auth::Authenticating(expected_hash) => {
                        if *expected_hash == pin_hash {
                            Atm {
                                cash_inside: starting_state.cash_inside - 1,
                                expected_pin_hash: Auth::Authenticated,
                                keystroke_register: Vec::new(),
                            }
                        } else {
                            Atm {
                                cash_inside: starting_state.cash_inside,
                                expected_pin_hash: Auth::Waiting,
                                keystroke_register: Vec::new(),
                            }
                        }
                    },
                    Auth::Authenticated => {
                        let amount_keys = starting_state.keystroke_register.clone();
                        let amount = amount_keys.iter().fold(0, |acc, key| {
                            match key {
                                Key::One => acc * 10 + 1,
                                Key::Two => acc * 10 + 2,
                                Key::Three => acc * 10 + 3,
                                Key::Four => acc * 10 + 4,
                                _ => acc,
                            }
                        });
                        if amount > starting_state.cash_inside {
                            Atm {
                                cash_inside: starting_state.cash_inside,
                                expected_pin_hash: Auth::Waiting,
                                keystroke_register: Vec::new(),
                            }
                        } else {
                            Atm {
                                cash_inside: starting_state.cash_inside - amount,
                                expected_pin_hash: Auth::Waiting,
                                keystroke_register: Vec::new(),
                            }
                        }
                    },
                    _ => Atm {
                        cash_inside: starting_state.cash_inside,
                        expected_pin_hash: Auth::Waiting,
                        keystroke_register: Vec::new(),
                    },
                }
            },
            Action::PressKey(Key::One) => Atm {
                cash_inside: starting_state.cash_inside,
                expected_pin_hash: match &starting_state.expected_pin_hash {
                    Auth::Authenticating(pin) => Auth::Authenticating(*pin),
                    _ => Auth::Waiting,
                },
                keystroke_register: clone_and_add(&starting_state.keystroke_register, Key::One),
            },
            Action::PressKey(Key::Two) => Atm {
                cash_inside: starting_state.cash_inside,
                expected_pin_hash: match &starting_state.expected_pin_hash {
                    Auth::Authenticating(pin) => Auth::Authenticating(*pin),
                    _ => Auth::Waiting,
                },
                keystroke_register: clone_and_add(&starting_state.keystroke_register, Key::Two),
            },
            Action::PressKey(Key::Three) => Atm {
                cash_inside: starting_state.cash_inside,
                expected_pin_hash: match &starting_state.expected_pin_hash {
                    Auth::Authenticating(pin) => Auth::Authenticating(*pin),
                    _ => Auth::Waiting,
                },
                keystroke_register: clone_and_add(&starting_state.keystroke_register, Key::Three),
            },
            Action::PressKey(Key::Four) => Atm {
                cash_inside: starting_state.cash_inside,
                expected_pin_hash: match &starting_state.expected_pin_hash {
                    Auth::Authenticating(pin) => Auth::Authenticating(*pin),
                    _ => Auth::Waiting,
                },
                keystroke_register: clone_and_add(&starting_state.keystroke_register, Key::Four),
            },
        }
    }
}

#[test]
fn sm_3_simple_swipe_card() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_swipe_card_again_part_way_through() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_press_key_before_card_swipe() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_pin() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Two));
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Two],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_enter_wrong_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = crate::hash(&pin);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::Three, Key::Three, Key::Three, Key::Three],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_correct_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = crate::hash(&pin);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::One, Key::Two, Key::Three, Key::Four],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_withdraw_amount() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Four));
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_try_to_withdraw_too_much() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_withdraw_acceptable_amount() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 9,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}
