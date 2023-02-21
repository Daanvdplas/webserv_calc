pub struct Stack(pub Vec<i64>);

impl Stack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn operations(&mut self, operator: char) -> Result<(), &'static str> {
        match operator {
            '+' => self.execute(i64::checked_add),
            '-' => self.execute(i64::checked_sub),
            '/' => self.execute(i64::checked_div),
            '*' => self.execute(i64::checked_mul),
            _ => self.execute_pow(),
        }
    }

    pub fn execute(&mut self, f: fn(i64, i64) -> Option<i64>) -> Result<(), &'static str> {
        if let Some(b) = self.0.pop() {
            if let Some(a) = self.0.pop() {
                if let Some(res) = f(a, b) {
                    self.0.push(res);
                } else {
                    return Err("Error: overflow");
                }
            } else {
                return Err("Error: stack is empty");
            }
        } else {
            return Err("Error: stack is empty");
        }
        Ok(())
    }

    pub fn execute_pow(&mut self) -> Result<(), &'static str> {
        if let Some(b) = self.0.pop() {
            if b < 0 {
                return Err("Error: negative power is not possible with this calculator");
            }
            if let Some(a) = self.0.pop() {
                if let Some(res) = a.checked_pow(b as u32) {
                    self.0.push(res);
                } else {
                    return Err("Error: overflow");
                }
            } else {
                return Err("Error: stack is empty");
            }
        } else {
            return Err("Error: stack is empty");
        }
        Ok(())
    }
}
