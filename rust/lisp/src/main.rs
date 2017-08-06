use std::io::Write;
use std::io;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
struct LRef(i64);

struct Arena { last: i64, data: HashMap<LRef, LObj> }

impl Arena {
    fn new() -> Arena {
        Arena { last: 0, data: HashMap::new() }
    }
    fn get(&self, key: &LRef)-> LObj {
        match self.data.get(&key) {
            Some(val) => val.clone(),
            _ => LObj::Nil,
        }
    }
    fn set(&mut self, key: LRef, val: LObj) {
        self.data.insert(key, val);
    }
    fn make(&mut self, obj: LObj) -> LRef {
        self.last += 1;
        self.data.insert(LRef(self.last), obj);
        LRef(self.last)
    }
    fn to_string(&self, key: &LRef) -> String {
        match self.get(key) {
            LObj::Nil => "nil".to_string(),
            LObj::Sym(ref symbol) => symbol.clone(),
            LObj::Num(number) => format!("{}", number),
            LObj::Subr(_) => "<subr>".to_string(),
            LObj::Expr(_, _) => "<expr>".to_string(),
            LObj::Cons(_, _) => format!("({})", self.to_string(key)),
        }
    }
    fn to_list_string(&self, key: &LRef) -> String {
        match self.get(key) {
            LObj::Cons(ref car, ref cdr) =>
                format!("{}{}", self.to_string(car), match self.get(cdr) {
                    LObj::Nil => "".to_string(),
                    LObj::Cons(_, _) => format!(" {}", self.to_string(cdr)),
                    _ => format!(" . {}", self.to_string(cdr))
                }),
            _ => "<internal error>".to_string(),
        }
    }
    fn nreverse(&mut self, lst: LRef) -> LRef {
        let mut lst = lst;
        let mut ret = self.make(LObj::Nil);
        while let LObj::Cons(car, cdr) = self.get(&lst) {
            self.set(lst.clone(), LObj::Cons(car, ret));
            ret = lst;
            lst = cdr;
        }
        ret
    }
    fn pairlis(&mut self, lst1: LRef, lst2: LRef) -> LRef {
        let mut lst1 = lst1;
        let mut lst2 = lst2;
        let mut ret = self.make(LObj::Nil);
        while let LObj::Cons(car1, cdr1) = self.get(&lst1) {
            if let LObj::Cons(car2, cdr2) = self.get(&lst2) {
                let car = self.make(LObj::Cons(car1, car2));
                ret = self.make(LObj::Cons(car, ret));
                lst1 = cdr1;
                lst2 = cdr2;
                continue;
            }
            break;
        }
        self.nreverse(ret)
    }
}

#[derive(Clone, PartialEq)]
enum LObj {
    Nil,
    Sym(String),
    Num(i64),
    Subr(SubFn),
    Expr(LRef, LRef),
    Cons(LRef, LRef),
}

impl LObj {
    fn sym(s: &str) -> LObj {
        LObj::Sym(s.into())
    }
    fn t() -> LObj {
        LObj::sym("t")
    }
    fn car(&self, arena: &Arena) -> LObj {
        match self {
            &LObj::Cons(ref x, _) => arena.get(x),
            _ => LObj::Nil,
         }
    }
    fn cdr(&self, arena: &Arena) -> LObj {
        match self {
            &LObj::Cons(_, ref x) => arena.get(x),
            _ => LObj::Nil,
        }
    }
}

#[derive(Clone, PartialEq)]
enum SubFn {
    Car,
    Cdr,
    Cons,
    Eq,
    Atom,
    Numberp,
    Symbolp,
    Add,
    Mul,
    Sub,
    Div,
    Mod,
    T,
}

impl SubFn {
     fn call(&self, arena: &mut Arena, args: &LRef) -> LObj {
         if let LObj::Cons(ref car, ref cdr) = arena.get(args) {
             let cdar = arena.get(cdr).car(&arena);
             let cdar = arena.make(cdar);
             return match *self {
                 SubFn::Car => arena.get(car).car(&arena),
                 SubFn::Cdr => arena.get(car).cdr(&arena),
                 SubFn::Cons => LObj::Cons(car.clone(), cdar),
                 SubFn::Eq => match arena.get(car) == arena.get(&cdar) {
                     true => LObj::t(),
                     _ => LObj::Nil,
                 },
                 SubFn::Atom => match arena.get(car) {
                     LObj::Cons(_, _) => LObj::Nil,
                     _ => LObj::t(),
                 },
                 SubFn::Numberp => match arena.get(car) {
                     LObj::Num(_) => LObj::t(),
                     _ => LObj::Nil,
                 },
                 SubFn::Symbolp => match arena.get(car) {
                     LObj::Num(_) => LObj::t(),
                     _ => LObj::Nil,
                 },


             };
         }
         LObj::Nil
     }
     fn all() -> Vec<(SubFn, &'static str)> {
         vec![
             (SubFn::Car, "car"),
             (SubFn::Cdr, "cdr"),
             (SubFn::Cons, "cons"),
             (SubFn::Eq, "eq"),
             (SubFn::Atom, "atom"),
             (SubFn::Numberp, "numberp"),
             (SubFn::Symbolp, "symbolp"),
             (SubFn::Add, "+"),
             (SubFn::Sub, "-"),
             (SubFn::Mul, "*"),
             (SubFn::Div, "/"),
             (SubFn::Mod, "mod"),
             (SubFn::T, "t"),
            ]
     }
     fn fold(arena: &mut Arena, car: &LRef, cdr: &LRef, f: &Fn(i64, i64) -> i64) -> LObj {
         if let LObj::Num(x) = arena.get(car) {
             if let LObj::Cons(ref cdar, ref cddr) = arena.get(cdr) {
                 if let LObj::Num(y) = arena.get(cdar) {
                     let sum = &arena.make(LObj::Num(f(x, y)));
                     return Self::fold(arena, sum, cddr, f);
                 }
             }
         } else {
             return arena.get(car);
         }
         LObj::Nil
     }
}

struct Reader<'a> { next: &'a str }

struct Evaluator { arena: Arena, genv: LRef}

// impl<'a> Reader<'a> {
//     fn make_num_or_sym(s: &str) -> LObj {
//         match s.parse::<i64>() {
//             Ok(n) => LObj::Num(n),
//             _ => LObj::Sym(s),
//         }
//     }
// }

fn main() {
    loop {
        print!(">> ");
        if let Err(e) = io::stdout().flush() { panic!(e); }
        let mut line = "".to_string();
        if let Err(_) = io::stdin().read_line(&mut line) { break; }
        if line.is_empty() { break; }
    }
}