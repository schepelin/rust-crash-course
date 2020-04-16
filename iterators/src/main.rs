struct Empty;

impl Iterator for Empty {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

struct TheAnswer;

impl Iterator for TheAnswer {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        Some(42)
    }
}

struct Counter(usize);

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 > 10 {
            return None;
        }
        let res = Some(self.0);
        self.0 += 1;
        res
    }
}

struct Fibonacci {
    current: usize,
    next: usize,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let orig_cur = self.current;
        let orig_next = self.next;

        match orig_cur.checked_add(orig_next) {
            None => None,
            Some(new_next) => {
                self.current = orig_next;
                self.next = new_next;
                Some(orig_cur)
            }
        }
    }
}

struct Doubler<I> {
    iter: I,
}

impl<I> Iterator for Doubler<I>
where
    I: Iterator,
    I::Item: std::ops::Add<Output = I::Item> + Copy,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => Some(val + val),
            None => None,
        }
    }
}

struct InfiniteUnit;

impl IntoIterator for InfiniteUnit {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![0; 10].into_iter()
    }
}

fn main() {
    for _i in Empty {
        unimplemented!("never happen");
    }
    println!("Hello, world!");

    for i in TheAnswer.take(10) {
        println!("The answer to life, the universe, and everything is {}", i);
    }

    for i in Counter(5) {
        println!("counter: {}", i);
    }
    let fib = Fibonacci::new();
    for num in fib.take(10) {
        println!("next fib is: {}", num);
    }

    let doubler = Doubler {
        iter: Fibonacci::new(),
    };

    for num in doubler.take(10) {
        println!("Doubled: {}", num);
    }

    let mut count = 0;
    for _ in InfiniteUnit {
        count += 1;
        println!("count == {}", count);
        if count >= 5 {
            break;
        }
    }
}
