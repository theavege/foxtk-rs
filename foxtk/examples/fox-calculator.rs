//! FOX desktop calculator, as a [`Component`](foxtk::prelude::Component).
//!
//! The keypad and the operator stack follow the FOX 1.6 calculator: scientific
//! functions, bases, angle units, memory, and parentheses. The window, title,
//! help, and memory icons are the original GIF and BMP files. The registry and
//! the color and font dialogs are not wrapped here.

use foxtk::prelude::*;

const STACK: usize = 32;
const DECIMAL_LIMIT: i32 = 16;
const MOD_INV: u8 = 1;
const MOD_HYP: u8 = 2;
const MOD_ENT: u8 = 4;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Angle {
    Deg,
    #[default]
    Rad,
    Gra,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Exponent {
    Always,
    Never,
    #[default]
    Maybe,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dy {
    Or,
    Xor,
    And,
    Sub,
    Add,
    Mod,
    Idiv,
    Div,
    Mul,
    XToY,
    XToInvY,
    Per,
    Com,
    LPar,
    RPar,
}

#[derive(Clone, Copy)]
enum Key {
    Digit(u32),
    Point,
    Exp,
    Delete,
    Clear,
    ClearAll,
    PlusMin,
    Inverse,
    Hyper,
    Sin,
    Cos,
    Tan,
    Log,
    Ln,
    Pi,
    Fac,
    Per,
    Com,
    Recip,
    Power,
    Sqrt,
    Shl,
    Shr,
    Log2,
    LPar,
    RPar,
    And,
    Or,
    Xor,
    Not,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    Enter,
    MemRec,
    MemAdd,
    MemSub,
    MemClr,
    Base(u32),
    Angle(Angle),
}

#[derive(Clone, Copy)]
enum Event {
    Key(Key),
    Precision(i32),
    Exponent,
    Beep(bool),
}

struct Model {
    display: String,
    numstack: [f64; STACK],
    numsp: usize,
    opstack: [Dy; STACK],
    opsp: isize,
    recall: f64,
    limit: i32,
    digits: i32,
    base: u32,
    angles: Angle,
    precision: usize,
    exponent: Exponent,
    beep: bool,
    modifiers: u8,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            display: "0".into(),
            numstack: [0.0; STACK],
            numsp: 0,
            opstack: [Dy::Add; STACK],
            opsp: -1,
            recall: 0.0,
            limit: DECIMAL_LIMIT,
            digits: 1,
            base: 10,
            angles: Angle::Rad,
            precision: 16,
            exponent: Exponent::Maybe,
            beep: true,
            modifiers: 0,
        }
    }
}

impl Model {
    fn getnum(&self) -> f64 {
        self.numstack[self.numsp]
    }

    fn setnum(&mut self, num: f64) {
        self.numstack[self.numsp] = num;
    }

    fn pushnum(&mut self, num: f64) -> bool {
        if self.numsp + 1 >= STACK {
            self.show_error();
            return false;
        }
        self.numsp += 1;
        self.numstack[self.numsp] = num;
        true
    }

    fn popnum(&mut self) -> f64 {
        let value = self.numstack[self.numsp];
        if self.numsp > 0 {
            self.numsp -= 1;
        }
        value
    }

    fn press(&mut self, key: Key) {
        match key {
            Key::Digit(digit) => self.digit(digit),
            Key::Point => self.point(),
            Key::Exp => self.exponent_key(),
            Key::Delete => self.delete(),
            Key::Clear => self.clear(),
            Key::ClearAll => self.clear_all(),
            Key::PlusMin => self.plus_min(),
            Key::Inverse => self.modifiers ^= MOD_INV,
            Key::Hyper => self.modifiers ^= MOD_HYP,
            Key::Sin => self.unary_trig(Trig::Sin),
            Key::Cos => self.unary_trig(Trig::Cos),
            Key::Tan => self.unary_trig(Trig::Tan),
            Key::Log => self.unary_log(false),
            Key::Ln => self.unary_log(true),
            Key::Pi => self.constant(),
            Key::Fac => self.unary(Un::Fac),
            Key::Per => self.dyadic(Dy::Per),
            Key::Com => self.dyadic(Dy::Com),
            Key::Recip => self.unary(Un::Recip),
            Key::Power => self.dyadic(if self.modifiers & MOD_INV == 0 {
                Dy::XToY
            } else {
                Dy::XToInvY
            }),
            Key::Sqrt => self.unary(if self.modifiers & MOD_INV == 0 {
                Un::Sqrt
            } else {
                Un::Square
            }),
            Key::Shl => self.unary(Un::Shl),
            Key::Shr => self.unary(if self.modifiers & MOD_INV == 0 {
                Un::Shr
            } else {
                Un::Sar
            }),
            Key::Log2 => self.unary(if self.modifiers & MOD_INV == 0 {
                Un::Log2
            } else {
                Un::Exp2
            }),
            Key::LPar => self.lparen(),
            Key::RPar => self.rparen(),
            Key::And => self.dyadic(Dy::And),
            Key::Or => self.dyadic(Dy::Or),
            Key::Xor => self.dyadic(Dy::Xor),
            Key::Not => self.unary(Un::Not),
            Key::Mul => self.dyadic(Dy::Mul),
            Key::Div => self.dyadic(Dy::Div),
            Key::Mod => self.dyadic(if self.modifiers & MOD_INV == 0 {
                Dy::Mod
            } else {
                Dy::Idiv
            }),
            Key::Add => self.dyadic(Dy::Add),
            Key::Sub => self.dyadic(Dy::Sub),
            Key::Enter => self.evaluate(),
            Key::MemRec => {
                let recall = self.recall;
                self.setnum(recall);
                self.set_display_value(recall);
                self.modifiers = 0;
            }
            Key::MemAdd => {
                self.recall += self.getnum();
                self.modifiers = 0;
            }
            Key::MemSub => {
                self.recall -= self.getnum();
                self.modifiers = 0;
            }
            Key::MemClr => {
                self.recall = 0.0;
                self.modifiers = 0;
            }
            Key::Base(base) => self.set_base(base),
            Key::Angle(angle) => self.angles = angle,
        }
    }

    fn digit(&mut self, digit: u32) {
        if self.base <= digit {
            return;
        }
        let hex = char::from_digit(digit, 16)
            .unwrap_or('0')
            .to_ascii_uppercase();
        if self.modifiers & MOD_ENT == 0 {
            self.display.clear();
            self.digits = 0;
        }
        if self.base == 10
            && let Some(mark) = self.display.find('E')
        {
            let mut pos = mark + 1;
            let bytes = self.display.as_bytes();
            if bytes.get(pos).is_some_and(|ch| *ch == b'+' || *ch == b'-') {
                pos += 1;
            }
            let exp = &self.display[pos..];
            let replace = exp.starts_with('0') || exp.len() >= 3;
            if replace {
                let mut shifted = exp.to_owned();
                if !shifted.is_empty() {
                    shifted.remove(0);
                }
                shifted.push(hex);
                self.display.truncate(pos);
                self.display.push_str(&shifted);
            } else {
                self.display.push(hex);
            }
        } else if self.digits < self.limit {
            self.display.push(hex);
            self.digits += 1;
        }
        let value = self.parse_display();
        self.setnum(value);
        self.modifiers |= MOD_ENT;
    }

    fn point(&mut self) {
        if self.modifiers & MOD_ENT == 0 {
            self.display = "0".into();
            self.digits = 1;
        }
        if self.base == 10 && !self.display.contains('.') && !self.display.contains('E') {
            self.display.push('.');
        }
        let value = self.parse_display();
        self.setnum(value);
        self.modifiers |= MOD_ENT;
    }

    fn exponent_key(&mut self) {
        if self.modifiers & MOD_ENT == 0 {
            self.display = "0".into();
            self.digits = 1;
        }
        if self.base == 10 && !self.display.contains('E') {
            self.display.push_str("E+0");
        }
        let value = self.parse_display();
        self.setnum(value);
        self.modifiers |= MOD_ENT;
    }

    fn plus_min(&mut self) {
        if self.modifiers & MOD_ENT == 0 {
            self.unary(Un::Neg);
            return;
        }
        if self.base == 10
            && let Some(mark) = self.display.find('E')
        {
            let sign_at = mark + 1;
            let sign = self.display.as_bytes().get(sign_at).copied();
            match sign {
                Some(b'+') => self.display.replace_range(sign_at..=sign_at, "-"),
                Some(b'-') => self.display.replace_range(sign_at..=sign_at, "+"),
                _ => self.display.insert(sign_at, '-'),
            }
        } else if let Some(rest) = self.display.strip_prefix('-') {
            self.display = rest.to_owned();
        } else if self.display.starts_with('+') {
            self.display.replace_range(0..1, "-");
        } else if self.display != "0" {
            self.display.insert(0, '-');
        }
        let value = self.parse_display();
        self.setnum(value);
    }

    fn delete(&mut self) {
        if self.modifiers & MOD_ENT == 0 {
            self.clear();
            return;
        }
        if self.base == 10 && self.display.contains('E') {
            self.display.pop();
            if self.display.ends_with('+') || self.display.ends_with('-') {
                self.display.pop();
            }
            if self.display.ends_with('E') {
                self.display.pop();
            }
        } else {
            let dropped = self.display.pop();
            if dropped.is_some_and(|ch| ch.is_ascii_hexdigit()) {
                self.digits -= 1;
            }
            if self.display.ends_with('+') || self.display.ends_with('-') {
                self.display.pop();
            }
            if self.display.ends_with('.') {
                self.display.pop();
            }
        }
        if self.display.is_empty() {
            self.display = "0".into();
            self.modifiers &= !MOD_ENT;
            self.digits = 1;
        }
        let value = self.parse_display();
        self.setnum(value);
    }

    fn clear(&mut self) {
        self.set_display_value(0.0);
        self.setnum(0.0);
        self.modifiers = 0;
    }

    fn clear_all(&mut self) {
        self.numstack = [0.0; STACK];
        self.numsp = 0;
        self.opsp = -1;
        self.modifiers = 0;
        self.set_display_value(0.0);
    }

    fn set_base(&mut self, base: u32) {
        let (base, limit) = match base {
            2 => (2, 32),
            8 => (8, 11),
            16 => (16, 8),
            _ => (10, DECIMAL_LIMIT),
        };
        self.base = base;
        self.limit = limit;
        let value = self.getnum();
        self.set_display_value(value);
        self.modifiers = 0;
    }

    fn set_precision(&mut self, precision: i32) {
        self.precision = precision.clamp(0, 16) as usize;
        let value = self.getnum();
        self.set_display_value(value);
        self.modifiers = 0;
    }

    fn cycle_exponent(&mut self) {
        self.exponent = match self.exponent {
            Exponent::Maybe => Exponent::Always,
            Exponent::Always => Exponent::Never,
            Exponent::Never => Exponent::Maybe,
        };
        let value = self.getnum();
        self.set_display_value(value);
        self.modifiers = 0;
    }

    fn lparen(&mut self) {
        if self.opsp + 1 >= STACK as isize {
            self.show_error();
            return;
        }
        self.opsp += 1;
        self.opstack[self.opsp as usize] = Dy::LPar;
        self.setnum(0.0);
        self.set_display_value(0.0);
        self.modifiers = 0;
    }

    fn rparen(&mut self) {
        while self.opsp >= 0 {
            let op = self.opstack[self.opsp as usize];
            self.opsp -= 1;
            if op == Dy::LPar {
                break;
            }
            self.dyop(op);
        }
        let value = self.getnum();
        self.set_display_value(value);
        self.modifiers = 0;
    }

    fn dyadic(&mut self, op: Dy) {
        let push = self.opsp < 0
            || self.opstack[self.opsp as usize] == Dy::LPar
            || priority(op) > priority(self.opstack[self.opsp as usize]);
        if push {
            if self.opsp + 1 >= STACK as isize {
                self.show_error();
                return;
            }
            if !self.pushnum(self.getnum()) {
                return;
            }
            self.opsp += 1;
            self.opstack[self.opsp as usize] = op;
        } else {
            let pending = self.opstack[self.opsp as usize];
            self.dyop(pending);
            if !self.pushnum(self.getnum()) {
                return;
            }
            self.opstack[self.opsp as usize] = op;
        }
        self.modifiers = 0;
    }

    fn evaluate(&mut self) {
        while self.opsp >= 0 {
            let op = self.opstack[self.opsp as usize];
            self.opsp -= 1;
            if op != Dy::LPar {
                self.dyop(op);
            }
        }
        let value = self.getnum();
        self.set_display_value(value);
        self.modifiers = 0;
    }

    fn dyop(&mut self, op: Dy) {
        let val = self.popnum();
        let acc = self.getnum();
        let acc = match op {
            Dy::Or => f64::from(as_u32(acc) | as_u32(val)),
            Dy::Xor => f64::from(as_u32(acc) ^ as_u32(val)),
            Dy::And => f64::from(as_u32(acc) & as_u32(val)),
            Dy::Sub => acc - val,
            Dy::Add => acc + val,
            Dy::Mod => acc % val.abs(),
            Dy::Idiv => (acc / val).trunc(),
            Dy::Div => acc / val,
            Dy::Mul => acc * val,
            Dy::XToY => acc.powf(val),
            Dy::XToInvY => acc.powf(1.0 / val),
            Dy::Per => permutations(acc, val),
            Dy::Com => combinations(acc, val),
            Dy::LPar | Dy::RPar => acc,
        };
        self.setnum(acc);
        self.set_display_value(acc);
        self.modifiers = 0;
    }

    fn unary(&mut self, op: Un) {
        let val = self.getnum();
        let acc = match op {
            Un::Not => f64::from(!as_u32(val)),
            Un::Neg => -val,
            Un::Shl => f64::from(as_u32(val) << 1),
            Un::Shr => f64::from(as_u32(val) >> 1),
            Un::Sar => f64::from((as_u32(val) >> 1) | (as_u32(val) & 0x8000_0000)),
            Un::Recip => 1.0 / val,
            Un::Fac => factorial(val),
            Un::Sqrt => val.sqrt(),
            Un::Square => val * val,
            Un::Log2 => val.log2(),
            Un::Exp2 => 2.0_f64.powf(val),
            Un::Log => val.log10(),
            Un::Exp10 => 10.0_f64.powf(val),
            Un::Ln => val.ln(),
            Un::Exp => val.exp(),
        };
        self.setnum(acc);
        self.set_display_value(acc);
        self.modifiers = 0;
    }

    fn unary_trig(&mut self, which: Trig) {
        let val = self.getnum();
        let inv = self.modifiers & MOD_INV != 0;
        let hyp = self.modifiers & MOD_HYP != 0;
        let acc = match (which, inv, hyp) {
            (Trig::Sin, false, false) => self.trig_arg(val).sin(),
            (Trig::Cos, false, false) => self.trig_arg(val).cos(),
            (Trig::Tan, false, false) => self.trig_arg(val).tan(),
            (Trig::Sin, true, false) => self.trig_res(val.asin()),
            (Trig::Cos, true, false) => self.trig_res(val.acos()),
            (Trig::Tan, true, false) => self.trig_res(val.atan()),
            (Trig::Sin, false, true) => val.sinh(),
            (Trig::Cos, false, true) => val.cosh(),
            (Trig::Tan, false, true) => val.tanh(),
            (Trig::Sin, true, true) => (val + (val * val + 1.0).sqrt()).ln(),
            (Trig::Cos, true, true) => (val + (val * val - 1.0).sqrt()).ln(),
            (Trig::Tan, true, true) => 0.5 * ((1.0 + val) / (1.0 - val)).ln(),
        };
        self.setnum(acc);
        self.set_display_value(acc);
        self.modifiers = 0;
    }

    fn unary_log(&mut self, natural: bool) {
        let inverse = self.modifiers & MOD_INV != 0;
        let op = match (natural, inverse) {
            (false, false) => Un::Log,
            (false, true) => Un::Exp10,
            (true, false) => Un::Ln,
            (true, true) => Un::Exp,
        };
        self.unary(op);
    }

    fn constant(&mut self) {
        let inv = self.modifiers & MOD_INV != 0;
        let hyp = self.modifiers & MOD_HYP != 0;
        let value = match (hyp, inv) {
            (false, false) => std::f64::consts::PI,
            (false, true) => std::f64::consts::E,
            (true, false) => 1.618_033_988_749_895,
            (true, true) => 1.0 / 1.618_033_988_749_895,
        };
        self.setnum(value);
        self.set_display_value(value);
        self.modifiers = 0;
    }

    fn trig_arg(&self, ang: f64) -> f64 {
        match self.angles {
            Angle::Deg => ang.to_radians(),
            Angle::Gra => ang * (std::f64::consts::PI / 200.0),
            Angle::Rad => ang,
        }
    }

    fn trig_res(&self, res: f64) -> f64 {
        match self.angles {
            Angle::Deg => res.to_degrees(),
            Angle::Gra => res * (200.0 / std::f64::consts::PI),
            Angle::Rad => res,
        }
    }

    fn parse_display(&self) -> f64 {
        if self.base == 10 {
            self.display.parse::<f64>().unwrap_or(0.0)
        } else {
            u32::from_str_radix(self.display.trim(), self.base).unwrap_or(0) as f64
        }
    }

    fn set_display_value(&mut self, mut val: f64) {
        if val.is_nan() {
            self.show_error();
            return;
        }
        if val.is_infinite() {
            self.display = if val.is_sign_negative() {
                "-INF".into()
            } else {
                "+INF".into()
            };
            self.beep();
            return;
        }
        if self.base == 10 {
            if val == 0.0 {
                val = 0.0;
            }
            self.display = format_decimal(val, self.precision, self.exponent);
        } else {
            self.display = format_integer(as_u32(val), self.base);
        }
    }

    fn show_error(&mut self) {
        self.display = "ERROR".into();
        self.beep();
    }

    fn beep(&self) {
        if self.beep {
            eprint!("\u{7}");
        }
    }

    fn decimal(&self) -> bool {
        self.base == 10
    }
}

#[derive(Clone, Copy)]
enum Un {
    Not,
    Neg,
    Shl,
    Shr,
    Sar,
    Recip,
    Fac,
    Sqrt,
    Square,
    Log2,
    Exp2,
    Log,
    Exp10,
    Ln,
    Exp,
}

#[derive(Clone, Copy)]
enum Trig {
    Sin,
    Cos,
    Tan,
}

fn priority(op: Dy) -> u8 {
    match op {
        Dy::Or | Dy::Xor | Dy::And => 1,
        Dy::Sub | Dy::Add => 2,
        Dy::Mod | Dy::Idiv | Dy::Div | Dy::Mul => 3,
        Dy::XToY | Dy::XToInvY => 4,
        Dy::Per | Dy::Com => 5,
        Dy::LPar | Dy::RPar => 8,
    }
}

fn as_u32(val: f64) -> u32 {
    let n = val.floor();
    if !n.is_finite() {
        return 0;
    }
    n.rem_euclid(4_294_967_296.0) as u32
}

fn factorial(n: f64) -> f64 {
    let mut num = n.floor();
    if num < 0.0 || num != n {
        return f64::NAN;
    }
    let mut result: f64 = 1.0;
    while num > 0.0 {
        if !result.is_finite() {
            break;
        }
        result *= num;
        num -= 1.0;
    }
    result
}

fn permutations(n: f64, r: f64) -> f64 {
    let mut num = n.floor();
    let mut den = r.floor();
    if num < 0.0 || den < 0.0 || den > num || num != n || den != r {
        return f64::NAN;
    }
    let mut result: f64 = 1.0;
    while den > 0.0 {
        if !result.is_finite() {
            break;
        }
        result *= num;
        num -= 1.0;
        den -= 1.0;
    }
    result
}

fn combinations(n: f64, r: f64) -> f64 {
    let mut num = n.floor();
    let mut den = r.floor();
    if num < 0.0 || den < 0.0 || den > num || num != n || den != r {
        return f64::NAN;
    }
    let mut numerator: f64 = 1.0;
    let mut denominator: f64 = 1.0;
    while den > 0.0 {
        if !numerator.is_finite() {
            break;
        }
        numerator *= num;
        denominator *= den;
        num -= 1.0;
        den -= 1.0;
    }
    numerator / denominator
}

fn format_integer(value: u32, base: u32) -> String {
    match base {
        2 => format!("{value:b}"),
        8 => format!("{value:o}"),
        16 => format!("{value:X}"),
        _ => format!("{value}"),
    }
}

fn format_decimal(val: f64, precision: usize, exponent: Exponent) -> String {
    if val == 0.0 {
        return "0".into();
    }
    let prec = precision.min(16);
    let scientific = match exponent {
        Exponent::Always => true,
        Exponent::Never => false,
        Exponent::Maybe => {
            let abs = val.abs();
            abs >= 10f64.powi(prec as i32) || abs < 1e-4
        }
    };
    if scientific {
        normalize_exponent(&format!("{val:.prec$e}"))
    } else {
        trim_trailing_zeros(&format!("{val:.prec$}"))
    }
}

fn normalize_exponent(text: &str) -> String {
    let Some(mark) = text.find('e').or_else(|| text.find('E')) else {
        return text.to_owned();
    };
    let (mant, exp) = text.split_at(mark);
    let exp = &exp[1..];
    let (sign, digits) = if let Some(rest) = exp.strip_prefix('+') {
        ("+", rest)
    } else if let Some(rest) = exp.strip_prefix('-') {
        ("-", rest)
    } else {
        ("+", exp)
    };
    format!("{mant}E{sign}{digits}")
}

fn trim_trailing_zeros(text: &str) -> String {
    let Some((head, frac)) = text.split_once('.') else {
        return text.to_owned();
    };
    let frac = frac.trim_end_matches('0');
    if frac.is_empty() {
        head.to_owned()
    } else {
        format!("{head}.{frac}")
    }
}

#[derive(Default)]
struct Calculator {
    display: foxtk::TextField,
    memory: foxtk::Label,
    bases: [foxtk::Button; 4],
    angles: [foxtk::Button; 3],
    inverse: foxtk::Button,
    hyper: foxtk::Button,
    recip: foxtk::Button,
    power: foxtk::Button,
    sqrt: foxtk::Button,
    log2: foxtk::Button,
    pi: foxtk::Button,
    shift_right: foxtk::Button,
    factorial: foxtk::Button,
    permut: foxtk::Button,
    combin: foxtk::Button,
    sin: foxtk::Button,
    cos: foxtk::Button,
    tan: foxtk::Button,
    log: foxtk::Button,
    ln: foxtk::Button,
    point: foxtk::Button,
    exp: foxtk::Button,
    modulo: foxtk::Button,
    digits: [foxtk::Button; 16],
    exponent_mode: foxtk::Button,
    precision: foxtk::Spinner,
    beep: foxtk::CheckButton,
}

impl Component for Calculator {
    type Event = Event;
    type State = Model;

    fn handle(
        event: Self::Event,
        model: &mut Self::State,
        _: Sender<Self::Event>,
    ) -> ComponentOutcome {
        match event {
            Event::Key(key) => model.press(key),
            Event::Precision(precision) => model.set_precision(precision),
            Event::Exponent => model.cycle_exponent(),
            Event::Beep(beep) => model.beep = beep,
        }
        ComponentOutcome::Changed
    }

    fn update(&self, model: &Self::State) {
        self.display.update(&model.display);
        let memory = if model.recall == 0.0 {
            String::new()
        } else {
            format!(
                "M {}",
                format_decimal(model.recall, model.precision, model.exponent)
            )
        };
        if self.memory.text().trim_start_matches('&') != memory {
            self.memory.set_text(&memory);
        }
        let decimal = model.decimal();
        let inv = model.modifiers & MOD_INV != 0;
        let hyp = model.modifiers & MOD_HYP != 0;
        engage(&self.inverse, inv);
        engage(&self.hyper, hyp);
        label(&self.recip, "1/x", decimal);
        label(&self.power, if inv { "x^1/y" } else { "x^y" }, decimal);
        label(&self.sqrt, if inv { "x^2" } else { "sqrt" }, decimal);
        label(&self.log2, if inv { "2^x" } else { "2log" }, decimal);
        label(
            &self.pi,
            match (hyp, inv) {
                (false, false) => "pi",
                (false, true) => "e",
                (true, false) => "phi",
                (true, true) => "1/phi",
            },
            decimal,
        );
        label(&self.shift_right, if inv { "SAR" } else { "SHR" }, true);
        label(&self.factorial, "x!", decimal);
        label(&self.permut, "nPr", decimal);
        label(&self.combin, "nCr", decimal);
        label(&self.sin, trig_label("sin", inv, hyp), decimal);
        label(&self.cos, trig_label("cos", inv, hyp), decimal);
        label(&self.tan, trig_label("tan", inv, hyp), decimal);
        label(&self.log, if inv { "10^x" } else { "log" }, decimal);
        label(&self.ln, if inv { "e^x" } else { "ln" }, decimal);
        label(&self.point, ".", decimal);
        label(&self.exp, "EXP", decimal);
        label(&self.modulo, if inv { "div" } else { "mod" }, decimal);
        for (index, button) in self.digits.iter().enumerate() {
            button.set_enable((index as u32) < model.base);
        }
        for (button, base) in self.bases.iter().zip([16, 10, 8, 2]) {
            engage(button, model.base == base);
        }
        for (button, angle) in self.angles.iter().zip([Angle::Deg, Angle::Rad, Angle::Gra]) {
            engage(button, model.angles == angle);
        }
        label(
            &self.exponent_mode,
            match model.exponent {
                Exponent::Maybe => "exp: auto",
                Exponent::Always => "exp: sci",
                Exponent::Never => "exp: fix",
            },
            true,
        );
        self.precision.update(model.precision as i32);
        if self.beep.check() != model.beep {
            self.beep.set_check(model.beep);
        }
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        let app = parent.app();
        let big = foxtk::Icon::from_gif(&app, include_bytes!("icons/bigcalc.gif"));
        let mini = foxtk::Icon::from_gif(&app, include_bytes!("icons/tinycalc.gif"));
        let question = foxtk::Icon::from_gif(&app, include_bytes!("icons/question.gif"));
        let memory_icon = foxtk::Icon::from_bmp(&app, include_bytes!("icons/constmem.bmp"));
        parent.set_window_icon(&big);
        parent.set_window_mini_icon(&mini);
        foxtk::VerticalFrame::new(parent)
            .with_spacing(4)
            .inside(|parent| {
                foxtk::HorizontalFrame::new(parent).inside(|parent| {
                    let title = foxtk::Label::new(parent, "FOX Calculator").with_icon(&big);
                    title.set_text("FOX Calculator");
                    foxtk::Button::new(parent, "Help")
                        .with_icon(&question)
                        .with_callback(|button| {
                            button.message(
                                MessageBox::Ok,
                                "Digits, operators, and functions follow the FOX calculator.\n\
                                 inv and hyp change the key above them. = evaluates the stack.\n\
                                 C clears the entry. AC clears the stack. Bases: Hex Dec Oct Bin.",
                                Message::Information,
                            );
                            false
                        });
                    self.display = foxtk::TextField::new(parent)
                        .with_editable(false)
                        .with_layout(Layout::Fill);
                    self.memory = foxtk::Label::new(parent, "").with_icon(&memory_icon);
                });
                foxtk::HorizontalFrame::new(parent).inside(|parent| {
                    self.precision = foxtk::Spinner::new(parent)
                        .with_range(0, 16)
                        .with_callback({
                            let sender = sender.clone();
                            move |spinner| {
                                if spinner.has_focus() {
                                    let _ = sender.send(Event::Precision(spinner.value()));
                                }
                                false
                            }
                        });
                    self.exponent_mode = key(parent, "exp: auto", &sender, Event::Exponent);
                    self.beep = foxtk::CheckButton::new(parent, "Beep")
                        .with_check(true)
                        .with_callback({
                            let sender = sender.clone();
                            move |check| {
                                let _ = sender.send(Event::Beep(check.check()));
                                false
                            }
                        });
                });
                foxtk::HorizontalFrame::new(parent).inside(|parent| {
                    for (slot, (title, base)) in ["Hex", "Dec", "Oct", "Bin"]
                        .into_iter()
                        .zip([16, 10, 8, 2])
                        .enumerate()
                    {
                        self.bases[slot] = key(parent, title, &sender, Event::Key(Key::Base(base)));
                    }
                    for (slot, (title, angle)) in ["Deg", "Rad", "Gra"]
                        .into_iter()
                        .zip([Angle::Deg, Angle::Rad, Angle::Gra])
                        .enumerate()
                    {
                        self.angles[slot] =
                            key(parent, title, &sender, Event::Key(Key::Angle(angle)));
                    }
                });
                foxtk::HorizontalFrame::new(parent)
                    .with_height(0)
                    .inside(|parent| {
                        let functions = foxtk::Matrix::new(parent, 6);
                        self.inverse = key(&functions, "inv", &sender, Event::Key(Key::Inverse));
                        key(&functions, "+/-", &sender, Event::Key(Key::PlusMin));
                        self.recip = key(&functions, "1/x", &sender, Event::Key(Key::Recip));
                        self.power = key(&functions, "x^y", &sender, Event::Key(Key::Power));
                        self.sqrt = key(&functions, "sqrt", &sender, Event::Key(Key::Sqrt));
                        self.log2 = key(&functions, "2log", &sender, Event::Key(Key::Log2));

                        self.pi = key(&functions, "pi", &sender, Event::Key(Key::Pi));
                        key(&functions, "SHL", &sender, Event::Key(Key::Shl));
                        self.shift_right = key(&functions, "SHR", &sender, Event::Key(Key::Shr));
                        self.factorial = key(&functions, "x!", &sender, Event::Key(Key::Fac));
                        self.permut = key(&functions, "nPr", &sender, Event::Key(Key::Per));
                        self.combin = key(&functions, "nCr", &sender, Event::Key(Key::Com));

                        self.hyper = key(&functions, "hyp", &sender, Event::Key(Key::Hyper));
                        self.sin = key(&functions, "sin", &sender, Event::Key(Key::Sin));
                        self.cos = key(&functions, "cos", &sender, Event::Key(Key::Cos));
                        self.tan = key(&functions, "tan", &sender, Event::Key(Key::Tan));
                        self.log = key(&functions, "log", &sender, Event::Key(Key::Log));
                        self.ln = key(&functions, "ln", &sender, Event::Key(Key::Ln));

                        for (offset, title) in
                            ["A", "B", "C", "D", "E", "F"].into_iter().enumerate()
                        {
                            self.digits[10 + offset] = key(
                                &functions,
                                title,
                                &sender,
                                Event::Key(Key::Digit((10 + offset) as u32)),
                            );
                        }

                        let keys = foxtk::Matrix::new(parent, 5);
                        let column = [
                            (Key::MemRec, "MR"),
                            (Key::Digit(7), "7"),
                            (Key::Digit(4), "4"),
                            (Key::Digit(1), "1"),
                            (Key::Digit(0), "0"),
                            (Key::MemAdd, "M+"),
                            (Key::Digit(8), "8"),
                            (Key::Digit(5), "5"),
                            (Key::Digit(2), "2"),
                            (Key::Point, "."),
                            (Key::MemSub, "M-"),
                            (Key::Digit(9), "9"),
                            (Key::Digit(6), "6"),
                            (Key::Digit(3), "3"),
                            (Key::Exp, "EXP"),
                            (Key::MemClr, "MC"),
                            (Key::LPar, "("),
                            (Key::Mul, "*"),
                            (Key::Add, "+"),
                            (Key::Enter, "="),
                            (Key::Clear, "C"),
                            (Key::RPar, ")"),
                            (Key::Div, "/"),
                            (Key::Sub, "-"),
                            (Key::Mod, "mod"),
                            (Key::ClearAll, "AC"),
                            (Key::And, "AND"),
                            (Key::Or, "OR"),
                            (Key::Xor, "XOR"),
                            (Key::Not, "NOT"),
                        ];
                        for (key_id, title) in column {
                            let button = key(&keys, title, &sender, Event::Key(key_id));
                            match key_id {
                                Key::Digit(digit) => self.digits[digit as usize] = button,
                                Key::Point => self.point = button,
                                Key::Exp => self.exp = button,
                                Key::Mod => self.modulo = button,
                                _ => {}
                            }
                        }
                    });
            });
    }
}

fn key(
    parent: &impl CompositeExt,
    title: &str,
    sender: &Sender<Event>,
    event: Event,
) -> foxtk::Button {
    foxtk::Button::new(parent, title)
        .with_layout(Layout::Fill)
        .with_callback({
            let sender = sender.clone();
            move |_| {
                let _ = sender.send(event);
                false
            }
        })
}

fn label(button: &foxtk::Button, text: &str, enabled: bool) {
    if button.text() != text {
        button.set_text(text);
    }
    button.set_enable(enabled);
}

fn engage(button: &foxtk::Button, down: bool) {
    button.set_state(if down {
        ButtonState::Engaged
    } else {
        ButtonState::Up
    });
}

fn trig_label(name: &str, inv: bool, hyp: bool) -> String {
    let mut label = String::new();
    if inv {
        label.push('a');
    }
    label.push_str(name);
    if hyp {
        label.push('h');
    }
    label
}

fn main() {
    Calculator::run("Calculator", "FOX", "FOX Calculator", 760, 480);
}

#[cfg(test)]
mod tests {
    use super::{Angle, Key, Model};

    fn run(keys: &[Key]) -> Model {
        let mut model = Model::default();
        for key in keys {
            model.press(*key);
        }
        model
    }

    #[test]
    fn addition_and_multiplication_honor_precedence() {
        let model = run(&[
            Key::Digit(2),
            Key::Add,
            Key::Digit(3),
            Key::Mul,
            Key::Digit(4),
            Key::Enter,
        ]);
        assert_eq!(model.getnum(), 14.0);
    }

    #[test]
    fn parentheses_group_before_multiplication() {
        let model = run(&[
            Key::LPar,
            Key::Digit(2),
            Key::Add,
            Key::Digit(3),
            Key::RPar,
            Key::Mul,
            Key::Digit(4),
            Key::Enter,
        ]);
        assert_eq!(model.getnum(), 20.0);
    }

    #[test]
    fn inverse_square_root_is_square() {
        let model = run(&[Key::Digit(9), Key::Inverse, Key::Sqrt]);
        assert_eq!(model.getnum(), 81.0);
    }

    #[test]
    fn factorial_of_eight() {
        let model = run(&[Key::Digit(8), Key::Fac]);
        assert_eq!(model.getnum(), 40_320.0);
    }

    #[test]
    fn hex_digits_and_base_change() {
        let mut model = run(&[Key::Base(16), Key::Digit(15), Key::Digit(15)]);
        assert_eq!(model.display, "FF");
        assert_eq!(model.getnum(), 255.0);
        model.press(Key::Base(10));
        assert_eq!(model.getnum(), 255.0);
        assert!(model.display.starts_with("255"));
    }

    #[test]
    fn sine_of_zero_radians_is_zero() {
        let model = run(&[Key::Angle(Angle::Rad), Key::Digit(0), Key::Sin]);
        assert_eq!(model.getnum(), 0.0);
    }

    #[test]
    fn memory_adds_and_recalls() {
        let model = run(&[
            Key::Digit(4),
            Key::MemAdd,
            Key::Clear,
            Key::Digit(1),
            Key::MemAdd,
            Key::MemRec,
        ]);
        assert_eq!(model.getnum(), 5.0);
    }
}
