pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            let msg = format!("Error: You are over your quote! ({} / {})", self.value, self.max);
            self.messenger.send(&msg[..]);
        }
        else if percentage_of_max >= 0.9 {
            let msg = format!("Warning: You are over 90% of your quote! ({} / {})", self.value, self.max);
            self.messenger.send(&msg[..]);
        }
        else if percentage_of_max >= 0.75 {
            let msg = format!("Info: You are over 75% of your quote! ({} / {})", self.value, self.max);
            self.messenger.send(&msg[..]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            let mut b1 = self.sent_messages.borrow_mut();
            b1.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // Arrange
        let mock_messenger = MockMessenger::new();
        let mut subject = LimitTracker::new(&mock_messenger, 100);

        // Act
        subject.set_value(80);

        // Assert
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}