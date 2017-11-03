use std::io;

#[derive(Default)]
pub struct PasswordChecker {
    guesses: u32,
    password: String
}

impl PasswordChecker {
    pub fn new(password: &'static str) -> PasswordChecker {
        PasswordChecker::new_string(password.to_string())
    }

    pub fn new_string(password: String) -> PasswordChecker {
        PasswordChecker {
            password,
            ..PasswordChecker::default()
        }
    }

    pub fn check(&mut self, against: String) -> bool {
        if against == self.password { true } else {
            self.guesses += 1;
            false
        }
    }

    pub fn guesses(&self) -> u32 { self.guesses }

    pub fn check_loop(&mut self) {
        let mut last_password: String = String::new();
        let mut was_guessed: bool = false;
        while !was_guessed {
            match io::stdin().read_line(&mut last_password) {
                Ok(bytes) => {
                    was_guessed = self.check((&*last_password.clone()).trim().to_string());
                    if was_guessed {
                        println!("You guessed it! It took you {} {}!", self.guesses(), if self.guesses() == 1 { "try" } else { "tries" })
                    } else {
                        println!("Incorrect! Guesses: {}", self.guesses());
                    }
                    // Flush
                }
                Err(e) => {
                    //If an I/O error is encountered then buf may contain some bytes already read in the event that all data read so far was valid UTF-8.
                    // Can't have that, so clear the buffer for next use.
                    println!("Error: {:?} | Try Again.", e);
                }
            }
            last_password = String::new();
        }
    }
}


