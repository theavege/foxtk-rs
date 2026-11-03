//! FOX desktop calculator, as a [`Component`](foxtk::prelude::Component).
//!
//! The keypad and the operator stack follow the FOX 1.6 calculator: scientific
//! functions, bases, angle units, memory, and parentheses. The window, title,
//! help, and memory icons are the original GIF and BMP files. Settings are
//! stored in the FOX registry under the same `SETTINGS` keys as the C++
//! calculator, except the display font, which is saved as a family and a size.
//! The window position and size use `x`, `y`, `w`, and `h`.
//! Double-click a color well to edit it.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

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

#[derive(Clone)]
enum Event {
    Key(Key),
    Precision(i32),
    Exponent,
    Beep(bool),
    Saved(Saved),
    Colors(Colors),
    Font(String, i32),
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Colors {
    digit: u32,
    hex: u32,
    operator: u32,
    function: u32,
    memory: u32,
    inverse: u32,
    hyper: u32,
    clear: u32,
    clear_all: u32,
    display: u32,
    number: u32,
    base: u32,
    angle: u32,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            digit: Color::from_rgb(94, 209, 204).bits(),
            hex: Color::from_rgb(151, 189, 206).bits(),
            operator: Color::from_rgb(255, 222, 163).bits(),
            function: Color::from_rgb(158, 213, 188).bits(),
            memory: Color::from_rgb(181, 207, 227).bits(),
            inverse: Color::from_rgb(224, 222, 69).bits(),
            hyper: Color::from_rgb(224, 222, 69).bits(),
            clear: Color::from_rgb(238, 148, 0).bits(),
            clear_all: Color::from_rgb(238, 118, 0).bits(),
            display: Color::from_rgb(255, 255, 255).bits(),
            number: Color::from_rgb(0, 0, 0).bits(),
            base: Color::from_rgb(203, 203, 203).bits(),
            angle: Color::from_rgb(203, 203, 203).bits(),
        }
    }
}

#[derive(Clone)]
struct Saved {
    colors: Colors,
    base: u32,
    angles: Angle,
    precision: usize,
    exponent: Exponent,
    beep: bool,
    recall: f64,
    font_family: String,
    font_size: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Saved {
    fn read(app: &foxtk::App) -> Self {
        let colors = Colors::default();
        Self {
            colors: Colors {
                digit: app
                    .read_color("SETTINGS", "digitcolor", Color::from_bits(colors.digit))
                    .bits(),
                hex: app
                    .read_color("SETTINGS", "hexdigitcolor", Color::from_bits(colors.hex))
                    .bits(),
                operator: app
                    .read_color(
                        "SETTINGS",
                        "operatorcolor",
                        Color::from_bits(colors.operator),
                    )
                    .bits(),
                function: app
                    .read_color(
                        "SETTINGS",
                        "functioncolor",
                        Color::from_bits(colors.function),
                    )
                    .bits(),
                memory: app
                    .read_color("SETTINGS", "memorycolor", Color::from_bits(colors.memory))
                    .bits(),
                inverse: app
                    .read_color("SETTINGS", "inversecolor", Color::from_bits(colors.inverse))
                    .bits(),
                hyper: app
                    .read_color("SETTINGS", "hypercolor", Color::from_bits(colors.hyper))
                    .bits(),
                clear: app
                    .read_color("SETTINGS", "clearcolor", Color::from_bits(colors.clear))
                    .bits(),
                clear_all: app
                    .read_color(
                        "SETTINGS",
                        "clearallcolor",
                        Color::from_bits(colors.clear_all),
                    )
                    .bits(),
                display: app
                    .read_color("SETTINGS", "displaycolor", Color::from_bits(colors.display))
                    .bits(),
                number: app
                    .read_color(
                        "SETTINGS",
                        "displaynumbercolor",
                        Color::from_bits(colors.number),
                    )
                    .bits(),
                base: app
                    .read_color("SETTINGS", "numbasecolor", Color::from_bits(colors.base))
                    .bits(),
                angle: app
                    .read_color("SETTINGS", "anglemodecolor", Color::from_bits(colors.angle))
                    .bits(),
            },
            base: app.read_int("SETTINGS", "base", 10) as u32,
            angles: angle_from_registry(app.read_int("SETTINGS", "angles", 1)),
            precision: app.read_int("SETTINGS", "precision", 16).clamp(0, 16) as usize,
            exponent: exponent_from_registry(app.read_int("SETTINGS", "exponent", 2)),
            beep: app.read_int("SETTINGS", "beep", 1) != 0,
            recall: app.read_real("SETTINGS", "memory", 0.0),
            font_family: app.read_string("SETTINGS", "displayfontfamily", ""),
            font_size: app.read_int("SETTINGS", "displayfontsize", 0),
            x: app.read_int("SETTINGS", "x", 50),
            y: app.read_int("SETTINGS", "y", 50),
            w: app.read_int("SETTINGS", "w", 0),
            h: app.read_int("SETTINGS", "h", 0),
        }
    }
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
    colors: Colors,
    font_family: String,
    font_size: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
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
            colors: Colors::default(),
            font_family: String::new(),
            font_size: 0,
            x: 50,
            y: 50,
            w: 0,
            h: 0,
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

    fn restore(&mut self, saved: Saved) {
        self.colors = saved.colors;
        self.recall = saved.recall;
        self.beep = saved.beep;
        self.exponent = saved.exponent;
        self.angles = saved.angles;
        self.font_family = saved.font_family;
        self.font_size = saved.font_size;
        self.x = saved.x;
        self.y = saved.y;
        self.w = saved.w;
        self.h = saved.h;
        self.precision = saved.precision.clamp(0, 16);
        self.set_base(saved.base);
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
    plus_min: foxtk::Button,
    shift_left: foxtk::Button,
    operators: [foxtk::Button; 14],
    memory_keys: [foxtk::Button; 4],
    clear: foxtk::Button,
    clear_all: foxtk::Button,
    live_colors: Rc<RefCell<Colors>>,
    live_font: Rc<RefCell<(String, i32)>>,
    updates: Cell<u8>,
    placed: Cell<bool>,
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
            Event::Saved(saved) => model.restore(saved),
            Event::Colors(colors) => model.colors = colors,
            Event::Font(family, size) => {
                model.font_family = family;
                model.font_size = size;
            }
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
        label(&self.sin, &trig_label("sin", inv, hyp), decimal);
        label(&self.cos, &trig_label("cos", inv, hyp), decimal);
        label(&self.tan, &trig_label("tan", inv, hyp), decimal);
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
        self.paint(model);
        self.place(model);
        self.persist(model);
    }

    fn paint(&self, model: &Model) {
        let colors = model.colors;
        for button in &self.digits[..10] {
            paint(button, colors.digit);
        }
        for button in &self.digits[10..] {
            paint(button, colors.hex);
        }
        for button in [
            &self.plus_min,
            &self.recip,
            &self.power,
            &self.sqrt,
            &self.log2,
            &self.pi,
            &self.shift_left,
            &self.shift_right,
            &self.factorial,
            &self.permut,
            &self.combin,
            &self.sin,
            &self.cos,
            &self.tan,
            &self.log,
            &self.ln,
        ] {
            paint(button, colors.function);
        }
        for button in &self.operators {
            paint(button, colors.operator);
        }
        for button in &self.memory_keys {
            paint(button, colors.memory);
        }
        paint(&self.inverse, colors.inverse);
        paint(&self.hyper, colors.hyper);
        paint(&self.clear, colors.clear);
        paint(&self.clear_all, colors.clear_all);
        for button in &self.bases {
            paint_group(button, colors.base);
        }
        for button in &self.angles {
            paint_group(button, colors.angle);
        }
        let display = Color::from_bits(colors.display);
        self.display.set_back_color(display);
        self.display.set_hilite_color(display.hilite());
        self.display.set_shadow_color(display.shadow());
        self.display.set_text_color(Color::from_bits(colors.number));
        if !model.font_family.is_empty() && model.font_size > 0 {
            self.display.set_font(&model.font_family, model.font_size);
        }
        *self.live_colors.borrow_mut() = colors;
        *self.live_font.borrow_mut() = (model.font_family.clone(), model.font_size);
    }

    fn place(&self, model: &Model) {
        if self.placed.get() || self.updates.get() == 0 {
            return;
        }
        self.placed.set(true);
        self.display.place_shell(model.x, model.y, model.w, model.h);
    }

    fn persist(&self, model: &Model) {
        if self.updates.get() == 0 {
            self.updates.set(1);
            return;
        }
        let app = self.display.app();
        let colors = model.colors;
        app.write_int("SETTINGS", "base", model.base as i32);
        app.write_int("SETTINGS", "angles", model.angles as i32);
        app.write_int("SETTINGS", "exponent", exponent_to_registry(model.exponent));
        app.write_int("SETTINGS", "precision", model.precision as i32);
        app.write_int("SETTINGS", "beep", i32::from(model.beep));
        app.write_real("SETTINGS", "memory", model.recall);
        app.write_color("SETTINGS", "digitcolor", Color::from_bits(colors.digit));
        app.write_color("SETTINGS", "hexdigitcolor", Color::from_bits(colors.hex));
        app.write_color(
            "SETTINGS",
            "operatorcolor",
            Color::from_bits(colors.operator),
        );
        app.write_color(
            "SETTINGS",
            "functioncolor",
            Color::from_bits(colors.function),
        );
        app.write_color("SETTINGS", "memorycolor", Color::from_bits(colors.memory));
        app.write_color("SETTINGS", "inversecolor", Color::from_bits(colors.inverse));
        app.write_color("SETTINGS", "hypercolor", Color::from_bits(colors.hyper));
        app.write_color("SETTINGS", "clearcolor", Color::from_bits(colors.clear));
        app.write_color(
            "SETTINGS",
            "clearallcolor",
            Color::from_bits(colors.clear_all),
        );
        app.write_color("SETTINGS", "displaycolor", Color::from_bits(colors.display));
        app.write_color(
            "SETTINGS",
            "displaynumbercolor",
            Color::from_bits(colors.number),
        );
        app.write_color("SETTINGS", "numbasecolor", Color::from_bits(colors.base));
        app.write_color("SETTINGS", "anglemodecolor", Color::from_bits(colors.angle));
        app.write_string("SETTINGS", "displayfontfamily", &model.font_family);
        app.write_int("SETTINGS", "displayfontsize", model.font_size);
        write_geometry(&app, &self.display);
    }

    fn view(&mut self, parent: &impl CompositeExt, sender: Sender<Self::Event>) {
        let app = parent.app();
        let big = foxtk::Icon::from_gif(&app, include_bytes!("icons/bigcalc.gif"));
        let mini = foxtk::Icon::from_gif(&app, include_bytes!("icons/tinycalc.gif"));
        let question = foxtk::Icon::from_gif(&app, include_bytes!("icons/question.gif"));
        let memory_icon = foxtk::Icon::from_bmp(&app, include_bytes!("icons/constmem.bmp"));
        let palette = foxtk::Icon::from_gif(&app, include_bytes!("icons/colors.gif"));
        let about = foxtk::Icon::from_gif(&app, include_bytes!("icons/information.gif"));
        parent.set_window_icon(&big);
        parent.set_window_mini_icon(&mini);
        parent.on_close(|window| write_geometry(&window.app(), &window));
        let live_colors = self.live_colors.clone();
        let live_font = self.live_font.clone();
        foxtk::VerticalFrame::new(parent)
            .with_spacing(4)
            .inside(|parent| {
                foxtk::HorizontalFrame::new(parent).inside(|parent| {
                    let title = foxtk::Button::new(parent, "FOX Calculator").with_icon(&big);
                    title.set_text("FOX Calculator");
                    title.with_callback({
                        let sender = sender.clone();
                        let live_colors = live_colors.clone();
                        let live_font = live_font.clone();
                        let big = same_icon(&big);
                        let palette = same_icon(&palette);
                        let about = same_icon(&about);
                        move |button| {
                            let colors = *live_colors.borrow();
                            let font = live_font.borrow().clone();
                            if let Some((colors, family, size)) =
                                edit_preferences(&button, colors, font, &big, &palette, &about)
                            {
                                let _ = sender.send(Event::Colors(colors));
                                let _ = sender.send(Event::Font(family, size));
                            }
                            false
                        }
                    });
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
                        self.plus_min = key(&functions, "+/-", &sender, Event::Key(Key::PlusMin));
                        self.recip = key(&functions, "1/x", &sender, Event::Key(Key::Recip));
                        self.power = key(&functions, "x^y", &sender, Event::Key(Key::Power));
                        self.sqrt = key(&functions, "sqrt", &sender, Event::Key(Key::Sqrt));
                        self.log2 = key(&functions, "2log", &sender, Event::Key(Key::Log2));

                        self.pi = key(&functions, "pi", &sender, Event::Key(Key::Pi));
                        self.shift_left = key(&functions, "SHL", &sender, Event::Key(Key::Shl));
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
                                Key::Point => self.operators[0] = button,
                                Key::Exp => self.operators[1] = button,
                                Key::LPar => self.operators[2] = button,
                                Key::Mul => self.operators[3] = button,
                                Key::Add => self.operators[4] = button,
                                Key::Enter => self.operators[5] = button,
                                Key::RPar => self.operators[6] = button,
                                Key::Div => self.operators[7] = button,
                                Key::Sub => self.operators[8] = button,
                                Key::Mod => self.operators[9] = button,
                                Key::And => self.operators[10] = button,
                                Key::Or => self.operators[11] = button,
                                Key::Xor => self.operators[12] = button,
                                Key::Not => self.operators[13] = button,
                                Key::MemRec => self.memory_keys[0] = button,
                                Key::MemAdd => self.memory_keys[1] = button,
                                Key::MemSub => self.memory_keys[2] = button,
                                Key::MemClr => self.memory_keys[3] = button,
                                Key::Clear => self.clear = button,
                                Key::ClearAll => self.clear_all = button,
                                _ => {}
                            }
                        }
                        self.point = same_button(&self.operators[0]);
                        self.exp = same_button(&self.operators[1]);
                        self.modulo = same_button(&self.operators[9]);
                    });
            });
        let _ = sender.send(Event::Saved(Saved::read(&parent.app())));
    }
}

fn write_geometry(app: &foxtk::App, window: &impl WindowExt) {
    let (x, y, width, height) = window.shell_geometry();
    if width <= 0 || height <= 0 {
        return;
    }
    app.write_int("SETTINGS", "x", x);
    app.write_int("SETTINGS", "y", y);
    app.write_int("SETTINGS", "w", width);
    app.write_int("SETTINGS", "h", height);
}

fn same_button(button: &foxtk::Button) -> foxtk::Button {
    unsafe { foxtk::Button::from_raw(button.as_raw()) }
}

fn same_icon(icon: &foxtk::Icon) -> foxtk::Icon {
    unsafe { foxtk::Icon::from_raw(icon.as_raw()) }
}

fn same_switcher(switcher: &foxtk::Switcher) -> foxtk::Switcher {
    unsafe { foxtk::Switcher::from_raw(switcher.as_raw()) }
}

fn paint(button: &foxtk::Button, bits: u32) {
    let color = Color::from_bits(bits);
    button.set_back_color(color);
    button.set_hilite_color(color.hilite());
    button.set_shadow_color(color.shadow());
}

fn paint_group(button: &foxtk::Button, bits: u32) {
    let color = Color::from_bits(bits);
    let raised = |channel: u32| channel + ((255 - channel) * 3) / 8;
    button.set_back_color(color);
    button.set_hilite_color(Color::from_rgb(
        raised(color.red()),
        raised(color.green()),
        raised(color.blue()),
    ));
    button.set_shadow_color(color.shadow());
}

fn angle_from_registry(value: i32) -> Angle {
    match value {
        0 => Angle::Deg,
        2 => Angle::Gra,
        _ => Angle::Rad,
    }
}

fn exponent_from_registry(value: i32) -> Exponent {
    match value {
        1 => Exponent::Always,
        0 => Exponent::Never,
        _ => Exponent::Maybe,
    }
}

fn exponent_to_registry(exponent: Exponent) -> i32 {
    match exponent {
        Exponent::Always => 1,
        Exponent::Never => 0,
        Exponent::Maybe => 2,
    }
}

fn edit_preferences(
    owner: &impl WindowExt,
    colors: Colors,
    font: (String, i32),
    calculator_icon: &foxtk::Icon,
    palette_icon: &foxtk::Icon,
    about_icon: &foxtk::Icon,
) -> Option<(Colors, String, i32)> {
    let dialog = foxtk::DialogBox::new(owner, "Calculator Preferences");
    let body = foxtk::VerticalFrame::new(&dialog).with_spacing(4);
    let close_row = foxtk::HorizontalFrame::new(&body);
    foxtk::Button::new(&close_row, "Close").with_callback(|button| {
        button.accept_dialog();
        false
    });
    foxtk::HorizontalSeparator::new(&body);
    let pages = foxtk::HorizontalFrame::new(&body).with_height(0);
    let side = foxtk::VerticalFrame::new(&pages);
    let switcher = foxtk::Switcher::new(&pages);

    let settings = foxtk::VerticalFrame::new(&switcher);
    foxtk::Label::new(&settings, "Calculator settings");
    foxtk::HorizontalSeparator::new(&settings);
    let font_choice = Rc::new(RefCell::new(font));
    let chosen = font_choice.clone();
    foxtk::Button::new(&settings, "Display font...").with_callback(move |button| {
        let dialog = foxtk::FontDialog::new(&button, "Change Display Font");
        let (family, size) = chosen.borrow().clone();
        if !family.is_empty() && size > 0 {
            dialog.set_font_selection(&family, size);
        }
        if dialog.execute() != 0 {
            *chosen.borrow_mut() = dialog.font_selection();
        }
        false
    });

    let colors_page = foxtk::VerticalFrame::new(&switcher);
    foxtk::Label::new(&colors_page, "Color settings");
    foxtk::HorizontalSeparator::new(&colors_page);
    let wells = [
        color_well(&colors_page, "Digits:", colors.digit),
        color_well(&colors_page, "Hex digits:", colors.hex),
        color_well(&colors_page, "Operators:", colors.operator),
        color_well(&colors_page, "Functions:", colors.function),
        color_well(&colors_page, "Memory:", colors.memory),
        color_well(&colors_page, "Inverse:", colors.inverse),
        color_well(&colors_page, "Hyperbolic:", colors.hyper),
        color_well(&colors_page, "Clear:", colors.clear),
        color_well(&colors_page, "Clear all:", colors.clear_all),
        color_well(&colors_page, "Display:", colors.display),
        color_well(&colors_page, "Display number:", colors.number),
        color_well(&colors_page, "Number base:", colors.base),
        color_well(&colors_page, "Angle mode:", colors.angle),
    ];

    let about_page = foxtk::VerticalFrame::new(&switcher);
    foxtk::Label::new(&about_page, "About FOX Calculator");
    foxtk::HorizontalSeparator::new(&about_page);
    let about_row = foxtk::HorizontalFrame::new(&about_page);
    foxtk::Label::new(&about_row, "").with_icon(calculator_icon);
    foxtk::Label::new(
        &about_row,
        "A programmer's desktop calculator.\nFOX Toolkit 1.6.",
    );

    let calculator_page = same_switcher(&switcher);
    let color_page = same_switcher(&switcher);
    let info_page = same_switcher(&switcher);
    foxtk::Button::new(&side, "Calculator")
        .with_icon(calculator_icon)
        .with_callback(move |_| {
            calculator_page.set_current(0);
            false
        });
    foxtk::Button::new(&side, "Colors")
        .with_icon(palette_icon)
        .with_callback(move |_| {
            color_page.set_current(1);
            false
        });
    foxtk::Button::new(&side, "About")
        .with_icon(about_icon)
        .with_callback(move |_| {
            info_page.set_current(2);
            false
        });

    if !dialog.execute() {
        return None;
    }
    let mut colors = colors;
    let values = wells.each_ref().map(|well| well.rgba().bits());
    colors.digit = values[0];
    colors.hex = values[1];
    colors.operator = values[2];
    colors.function = values[3];
    colors.memory = values[4];
    colors.inverse = values[5];
    colors.hyper = values[6];
    colors.clear = values[7];
    colors.clear_all = values[8];
    colors.display = values[9];
    colors.number = values[10];
    colors.base = values[11];
    colors.angle = values[12];
    let (family, size) = font_choice.borrow().clone();
    Some((colors, family, size))
}

fn color_well(parent: &impl CompositeExt, title: &str, bits: u32) -> foxtk::ColorWell {
    let row = foxtk::HorizontalFrame::new(parent);
    foxtk::Label::new(&row, title);
    let well = foxtk::ColorWell::new(&row);
    well.set_rgba(Color::from_bits(bits));
    well.set_help_text("Double-click to change this color");
    well
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
                let _ = sender.send(event.clone());
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
    Calculator::run("Calculator", "", "FOX Calculator", 760, 480);
}

#[cfg(test)]
mod tests {
    use super::{
        Angle, Exponent, Key, Model, angle_from_registry, exponent_from_registry,
        exponent_to_registry,
    };

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
    fn registry_exponent_values_match_fox() {
        assert_eq!(exponent_from_registry(1), Exponent::Always);
        assert_eq!(exponent_from_registry(0), Exponent::Never);
        assert_eq!(exponent_from_registry(2), Exponent::Maybe);
        assert_eq!(exponent_to_registry(Exponent::Always), 1);
        assert_eq!(exponent_to_registry(Exponent::Never), 0);
        assert_eq!(exponent_to_registry(Exponent::Maybe), 2);
        assert_eq!(angle_from_registry(0), Angle::Deg);
        assert_eq!(angle_from_registry(1), Angle::Rad);
        assert_eq!(angle_from_registry(2), Angle::Gra);
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
