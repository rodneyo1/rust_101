use std::cell::RefCell;
use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T: Logger> {
    logger: &'a T,
    max: usize,
    last_warning_percentage: RefCell<usize>,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: usize) -> Self {
        Tracker {
            logger,
            max,
            last_warning_percentage: RefCell::new(0),
        }
    }

    pub fn set_value(&self, rc: &Rc<()>) {
        let ref_count = Rc::strong_count(rc);
        let percentage = (ref_count - 1) * 100 / self.max;

        if percentage >= 100 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70 {
            let mut last = self.last_warning_percentage.borrow_mut();
            // Only log if this is a new threshold (70% or 90%)
            if percentage >= 90 && *last < 90 {
                self.logger.warning(&format!(
                    "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                    percentage
                ));
                *last = 90;
            } else if percentage >= 70 && *last < 70 {
                self.logger.warning(&format!(
                    "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                    percentage
                ));
                *last = 70;
            }
        }
    }

    pub fn peek(&self, rc: &Rc<()>) {
        let ref_count = Rc::strong_count(rc);
        let percentage = (ref_count - 1) * 100 / self.max;
        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            percentage
        ));
    }
}