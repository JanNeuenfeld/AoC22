use std::fs::File;
use std::io::Read;
use std::collections::{HashSet, HashMap};
use std::str::Chars;
use itertools::Itertools;
use std::time::Instant;
use std::thread;

fn day1_1() -> i32 {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    let mut amount = 0;
    let mut amounts: Vec<i32> = Vec::new();

    for line in contents.split("\n") {
        if line != "" {
            amount += line.parse::<i32>().unwrap();
        }
        else {
            amounts.push(amount);
            amount = 0;
        }
    }
    
    *amounts.iter().max().unwrap()
}

fn day1_2() -> i32 {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    let mut amount = 0;
    let mut amounts: Vec<i32> = Vec::new();

    for line in contents.split("\n") {
        if line != "" {
            amount += line.parse::<i32>().unwrap();
        }
        else {
            amounts.push(amount);
            amount = 0;
        }
    }
    
    amounts.sort();

    amounts[amounts.len() - 3..amounts.len()].into_iter().sum::<i32>()
}

fn day2_1() -> i32 {
    let mut file = File::open("input2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut points = 0;

    for i in contents.split("\n") {
        let mut signs = i.split(" ");
        match i {
            "A Y" | "B Z" | "C X" => points += 6,
            "A X" | "B Y" | "C Z" => points += 3,
            _ => ()
        }
        match signs.nth(1).unwrap() {
            "X" => points += 1,
            "Y" => points += 2,
            "Z" => points += 3,
            _ => panic!("no clue")
        }
    }

    points
}

fn day2_2() -> i32 {
    fn play(with: &str, win: &str) -> i32 {
        let (a, b, c);
        match win {
            "X" => (a, b, c) = (3, 1, 2),
            "Y" => (a, b, c) = (4, 5, 6),
            "Z" => (a, b, c) = (8, 9, 7),
            _ => panic!()
        }
        match with {
            "A" => return a,
            "B" => return b,
            "C" => return c,
            _ => panic!("asd")
        }
    }

    let mut file = File::open("input2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut points = 0;

    for i in contents.split("\n") {
        let mut signs = i.split(" ");

        points += play(signs.clone().nth(0).unwrap(), signs.nth(1).unwrap());
    }

    points
}

fn day3_1() -> usize {
    let mut file = File::open("input3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut points = 0;
    let abc: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();


    for i in contents.split("\n") {
        let mut items = HashSet::new();
        for item in i.chars().into_iter().take(i.len() / 2) {
            items.insert(item);
        }
        for item in i.chars().into_iter().rev().take(i.len() / 2) {
            if items.contains(&item) {
                points += abc.iter().position(|&r| r == item).unwrap() + 1;
                break;
            }
        }
    }

    points
}

fn day3_2() -> usize {
    let mut file = File::open("input3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut contents = contents.split("\n").into_iter();

    let mut points = 0;
    let abc: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    while let Some((g1, g2, g3)) = contents.next_tuple() {
        for i in g1.chars() {
            if g2.contains(i) && g3.contains(i) {
                points += abc.iter().position(|&r| r == i).unwrap() + 1;
                break;
            }
        }
    }

    points
}

fn day4_1() -> i32 {
    let mut file = File::open("input4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents = contents.split("\n").into_iter();

    let mut amount = 0;

    for pair in contents {
        let mut elves = pair.split(",");
        let e1 = (elves.clone().nth(0).unwrap().split("-").nth(0).unwrap().parse::<i32>().unwrap(), elves.clone().nth(0).unwrap().split("-").nth(1).unwrap().parse::<i32>().unwrap());
        let e2 = (elves.clone().nth(1).unwrap().split("-").nth(0).unwrap().parse::<i32>().unwrap(), elves.nth(1).unwrap().split("-").nth(1).unwrap().parse::<i32>().unwrap());
        if (e1.0 >= e2.0 && e1.1 <= e2.1) || (e2.0 >= e1.0 && e2.1 <= e1.1) {
            amount += 1;
        }
    }

    amount
}

fn day4_2() -> i32 {
    let mut file = File::open("input4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents = contents.split("\n").into_iter();

    let mut amount = 0;

    for pair in contents {
        let mut elves = pair.split(",");
        let mut e1: HashSet<i32> = HashSet::new();
        
        for i in elves.clone().nth(0).unwrap().split("-").nth(0).unwrap().parse::<i32>().unwrap()..elves.clone().nth(0).unwrap().split("-").nth(1).unwrap().parse::<i32>().unwrap() + 1 {
            e1.insert(i);
        }

        for i in elves.clone().nth(1).unwrap().split("-").nth(0).unwrap().parse::<i32>().unwrap()..elves.nth(1).unwrap().split("-").nth(1).unwrap().parse::<i32>().unwrap() + 1 {
            if e1.contains(&i) {
                amount += 1;
                break;
            }
        }
    }

    amount
}

fn day5_1() -> String {
    let mut file = File::open("input5.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut contents = contents.split("\n\n");

    let stacks = contents.nth(0).unwrap();

    let mut actual_stacks: Vec<Vec<char>> = Vec::new();

    for i in stacks.split("\n") {
        if i.starts_with(" 1") {
            break;
        }
        let mut stack = 1;
        for o in i.chars().into_iter().tuples::<(_, _, _ ,_)>() {
            if actual_stacks.len() < stack {
                actual_stacks.push(Vec::new())
            }
            if o.1 != ' ' {
                actual_stacks[stack - 1].insert(0, o.1);
            }
            stack += 1;
        }
    }

    for i in contents.nth(0).unwrap().split("\n") {
        let (_, mv, _, fm, _, to) = i.split(" ").into_iter().next_tuple().unwrap();

        for _ in 0..mv.parse().unwrap() {
            let asd = actual_stacks[fm.parse::<usize>().unwrap() - 1].pop().unwrap();
            actual_stacks[to.parse::<usize>().unwrap() - 1].push(asd);
        }
    }

    let mut output = String::new();

    for mut i in actual_stacks {
        output += &i.pop().unwrap().to_string();
    }

    output
}

fn day5_2(f: &str) -> String {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut contents = contents.split("\n\n");

    let stacks = contents.nth(0).unwrap();

    let mut actual_stacks: Vec<Vec<char>> = Vec::new();

    for i in stacks.split("\n") {
        if i.starts_with(" 1") {
            break;
        }
        let mut stack = 1;
        for o in i.chars().into_iter().tuples::<(_, _, _ ,_)>() {
            if actual_stacks.len() < stack {
                actual_stacks.push(Vec::new())
            }
            if o.1 != ' ' {
                actual_stacks[stack - 1].insert(0, o.1);
            }
            stack += 1;
        }
    }

    for i in contents.nth(0).unwrap().split("\n") {
        let (_, mv, _, fm, _, to) = i.split(" ").into_iter().next_tuple().unwrap();
        let mut asdf = Vec::new();

        for _ in 0..mv.parse().unwrap() {
            asdf.insert(0, actual_stacks[fm.parse::<usize>().unwrap() - 1].pop().unwrap())
        }
        for i in asdf {
            actual_stacks[to.parse::<usize>().unwrap() - 1].push(i);
        }
    }

    let mut output = String::new();

    for mut i in actual_stacks {
        output += &i.pop().unwrap().to_string();
    }

    output
}

fn day6_1(f: &str) -> usize {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for (index, i) in contents.chars().into_iter().tuple_windows::<(_, _, _, _)>().enumerate() {
        if (i.0 != i.1 && i.0 != i.2 && i.0 != i.3) && (i.1 != i.2 && i.1 != i.3) && (i.2 != i.3) {
            return index + 4
        }
    }
    0
}

fn day6_2(f: &str) -> usize {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for (index, _) in contents.chars().enumerate() {
        if contents[index..index + 14].chars().all_unique() {
            return index + 14
        }
    }

    0
}

fn day7_1(f: &str) -> i32 {
    #[derive(Clone)]
    struct Folder {
        subfolders: Vec<Vec<String>>,
        size: i32,
    }

    impl Folder {
        fn get_size(&mut self, folders: &mut HashMap<Vec<String>, Folder>) -> i32 {
            let mut sum = self.size;
            for folder in &self.subfolders {
                //println!("{:?}", folder);
                let mut some_folder = folders.get_mut(&folder.clone()).unwrap().clone();
                let some_sum = some_folder.get_size(folders);
                folders.get_mut(folder).unwrap().size = some_sum;
                //println!("somesum: {}", some_sum);
                sum += some_sum;
            }
            self.size = sum;
            sum
        }
    }

    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut folders: HashMap<Vec<String>, Folder> = HashMap::new();
    let mut current_folder: Vec<String> = Vec::new();
    
    folders.insert(vec!["/".to_string()], Folder { subfolders: Vec::new(), size: 0});
    //println!("{:?}", current_folder);

    for line in contents.split("\n") {
        match line.split(" ").nth(0).unwrap() {
            "$" => {
                match line.split(" ").nth(1).unwrap() {
                    "cd" => {
                        if line.split(" ").nth(2).unwrap().to_string() == ".." {
                            current_folder.pop();
                        } else {
                            //if line.split(" ").nth(2).unwrap() == "/" {
                            //    current_folder = vec!["/".to_string()]
                            //}
                            current_folder.push(line.split(" ").nth(2).unwrap().to_string());
                        }
                    },
                    _ => ()
                }
            },
            "dir" => {
                let asd = current_folder.clone();
                let mut asdf = asd.clone();
                //println!("{:?}", asd);
                //println!("{:?}", folders.keys());
                asdf.push(line.split(" ").nth(1).unwrap().to_string());
                folders.get_mut(&asd).unwrap().subfolders.push(asdf.clone());
                folders.insert(asdf, Folder { subfolders: Vec::new(), size: 0});
            },
            a => {
                folders.get_mut(&current_folder).unwrap_or_else(|| panic!("error at: {:?}", current_folder)).size += a.parse::<i32>().unwrap_or_else(|_| panic!("{}", a));
            }
        }
    }

    
    let mut some_folder = folders.get_mut(&vec!["/".to_string()]).unwrap().clone();
    some_folder.get_size(&mut folders);
    let mut out = 0;
    for (_, f) in folders {
        if f.size <= 100000 {
            out += f.size;
        }
    }
    out
}

fn day7_2(f: &str) -> i32 {
    #[derive(Clone)]
    struct Folder {
        subfolders: Vec<Vec<String>>,
        size: i32,
    }

    impl Folder {
        fn get_size(&mut self, folders: &mut HashMap<Vec<String>, Folder>) -> i32 {
            let mut sum = self.size;
            for folder in &self.subfolders {
                let mut some_folder = folders.get_mut(&folder.clone()).unwrap().clone();
                let some_sum = some_folder.get_size(folders);
                folders.get_mut(folder).unwrap().size = some_sum;
                sum += some_sum;
            }
            self.size = sum;
            sum
        }
    }

    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut folders: HashMap<Vec<String>, Folder> = HashMap::new();
    let mut current_folder: Vec<String> = Vec::new();
    
    folders.insert(vec!["/".to_string()], Folder { subfolders: Vec::new(), size: 0});

    for line in contents.split("\n") {
        match line.split(" ").nth(0).unwrap() {
            "$" => {
                match line.split(" ").nth(1).unwrap() {
                    "cd" => {
                        if line.split(" ").nth(2).unwrap().to_string() == ".." {
                            current_folder.pop();
                        } else {
                            current_folder.push(line.split(" ").nth(2).unwrap().to_string());
                        }
                    },
                    _ => ()
                }
            },
            "dir" => {
                let asd = current_folder.clone();
                let mut asdf = asd.clone();
                asdf.push(line.split(" ").nth(1).unwrap().to_string());
                folders.get_mut(&asd).unwrap().subfolders.push(asdf.clone());
                folders.insert(asdf, Folder { subfolders: Vec::new(), size: 0});
            },
            a => {
                folders.get_mut(&current_folder).unwrap_or_else(|| panic!("error at: {:?}", current_folder)).size += a.parse::<i32>().unwrap_or_else(|_| panic!("{}", a));
            }
        }
    }

    let mut some_folder = folders.get_mut(&vec!["/".to_string()]).unwrap().clone();
    let asdsad = some_folder.get_size(&mut folders);
    folders.get_mut(&vec!["/".to_string()]).unwrap().size = asdsad;
    let total_size = folders.values().sorted_by(|a, b| Ord::cmp(&a.size, &b.size)).nth(folders.len() - 1).unwrap();
    //println!("{}", total_size.size);
    let mut possible = Vec::new();
    for i in folders.values() {
        if i.size >= total_size.size - 40000000 {
            possible.push(i.size.clone());
        }
    }
    possible.sort();
    let asldhaskljhkjfsdf = possible.clone();
    *asldhaskljhkjfsdf.get(0).unwrap()
}

fn day9_1(f: &str) -> usize {
    #[derive(Debug)]
    struct Knot {
        x: i32,
        y: i32
    }
    impl Knot {
        fn distance(&self, other: &Knot) -> (i32, i32) {
            (self.x - other.x, self.y - other.y)
        }
    }

    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert((0, 0));
    let mut head = Knot {x: 0, y: 0};
    let mut tail = Knot {x: 0, y: 0};

    for i in contents.split("\n") {
        //println!("NEW!");
        let (direction, amount) = i.split(" ").into_iter().next_tuple().unwrap();
        
        for _ in 0..amount.parse::<i32>().unwrap() {
            match direction {
                "R" => head.x += 1,
                "L" => head.x -= 1,
                "U" => head.y += 1,
                "D" => head.y -= 1,
                _ => panic!("sth went wrong ig")
            }

            let d = head.distance(&tail);
            match d {
                (2, 1) | (1, 2) => {tail.x += 1; tail.y += 1},
                (-2, -1) | (-1, -2) => {tail.x -= 1; tail.y -= 1},
                (-2, 1) | (-1, 2) => {tail.x -= 1; tail.y += 1},
                (2, -1) | (1, -2) => {tail.x += 1; tail.y -= 1},
                (2, 0) => tail.x += 1,
                (-2, 0) => tail.x -= 1,
                (0, 2) => tail.y += 1,
                (0, -2) => tail.y -= 1,
                _ => ()
            }


            //println!("head: {:?}, tail: {:?}, distance: {:?}", head, tail, d);

            positions.insert((tail.x, tail.y));
        }
    }

    //println!("{:?}", positions);
    positions.len()
}

fn day9_2(f: &str) -> usize {
    #[derive(Debug, Clone, Copy)]
    struct Knot {
        x: i32,
        y: i32
    }
    impl Knot {
        fn distance(&self, other: &Knot) -> (i32, i32) {
            (self.x - other.x, self.y - other.y)
        }
    }

    fn mv(s: &mut Knot, o: &Knot) -> bool {
        let d = o.distance(&s);
        //println!("{:?}", d);
        match d {
            (2, 1) | (1, 2) | (2, 2) => {s.x += 1; s.y += 1},
            (-2, -1) | (-1, -2) | (-2, -2) => {s.x -= 1; s.y -= 1},
            (-2, 1) | (-1, 2) | (-2, 2) => {s.x -= 1; s.y += 1},
            (2, -1) | (1, -2) | (2, -2)=> {s.x += 1; s.y -= 1},
            (2, 0) => s.x += 1,
            (-2, 0) => s.x -= 1,
            (0, 2) => s.y += 1,
            (0, -2) => s.y -= 1,
            _ => return false
        }
        true
    }

    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert((0, 0));
    let mut head = Knot {x: 0, y: 0};
    let mut tail = Knot {x: 0, y: 0};
    let mut knots = HashMap::new();

    for i in 0..8 {
        knots.insert(i, Knot {x: 0, y: 0});
    }

    for i in contents.split("\n") {
        let (direction, amount) = i.split(" ").into_iter().next_tuple().unwrap();
        
        for _ in 0..amount.parse::<i32>().unwrap() {
            match direction {
                "R" => head.x += 1,
                "L" => head.x -= 1,
                "U" => head.y += 1,
                "D" => head.y -= 1,
                _ => panic!("sth went wrong ig")
            }

            mv(knots.get_mut(&0).unwrap(), &head);

            for k in 1..8 {
                let ks = knots.clone();
                mv(knots.get_mut(&k).unwrap(), ks.get(&(k - 1)).unwrap());
            }

            mv(&mut tail, knots.get(&(&7)).unwrap());

            positions.insert((tail.x, tail.y));
        }
    }

    //println!("{:?}", positions);
    positions.len()
}

fn day10_1(f: &str) -> i32 {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut cycle = 0;
    let mut x = 1;
    let mut out = 0;
    let mut stall = 0;
    let mut to_add = 0;
    let mut commands = contents.split("\n").into_iter();

    loop {
        if (cycle + 20) % 40 == 0 {out += cycle * x;}
        if stall == 0 {
            if to_add != 0 {x += to_add; to_add = 0}
            if let Some(c) = commands.next() {
                if c.starts_with("addx") {
                    stall = 1;
                    to_add = c.split(" ").nth(1).unwrap().parse::<i32>().unwrap_or_else(|_| panic!("{:?}", c));
                }
            } else {break;}
        } else {stall -= 1}
        cycle += 1;
    }
    out
}

fn day10_2(f: &str) -> String {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut cycle = 0;
    let mut x = 1;
    let mut out = String::new();
    let mut stall = 0;
    let mut to_add = 0;
    let mut commands = contents.split("\n").into_iter();

    loop {
        if stall == 0 {
            if to_add != 0 {x += to_add; to_add = 0;}
            if let Some(c) = commands.next() {
                if c.starts_with("addx") {
                    stall = 1;
                    to_add = c.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                }
            } else {break;}
        } else {stall -= 1}

        let x_pos = cycle % 40;
        if x_pos == 0 {out += "\n"}
        if x_pos >= x - 1 && x_pos <= x + 1 {out += "█"}
        else {out += " ";}
        cycle += 1;
    }
    out
}

fn day11_1(f: &str) -> i128 {
    #[derive(Debug, Clone)]
    struct Monkey {
        items: Vec<i128>,
        times_inspected: i128,
        operation: String,
        op_num: i128,
        test: i128,
        yes: usize,
        no: usize
    }

    impl Monkey {
        fn inspect(&mut self, monkeys: &mut HashMap<usize, Monkey>) {
            for item in self.items.clone().into_iter() {
                let mut amount = self.op_num;
                if self.op_num == -1 {amount = item}
                if self.operation == "*" {
                    self.items[0] = (item * amount) / 3;
                } else {
                    self.items[0] = (item + amount) / 3;
                }
                if self.items[0] % self.test == 0 {
                    monkeys.get_mut(&self.yes).unwrap().items.push(self.items.remove(0));
                } else {
                    monkeys.get_mut(&self.no).unwrap().items.push(self.items.remove(0));
                }
                self.times_inspected += 1;
            }
        }
    }

    
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    for (monkey, stats) in contents.split("\n\n").enumerate() {
        let mut s = stats.split("\n");
        s.next();
        let mut items = Vec::new();
        for i in s.next().unwrap().split(": ").nth(1).unwrap().split(", ") {
            items.push(i.parse::<i128>().unwrap());
        }
        let mut thing = s.next().unwrap().split("= old ").nth(1).unwrap().split(" ");

        let operation = thing.nth(0).unwrap().to_string();
        let op_num = thing.nth(0).unwrap().parse::<i128>().unwrap_or(-1);

        monkeys.insert(monkey, Monkey {
            items: items,
            times_inspected: 0,
            operation: operation,
            op_num: op_num,
            test: s.next().unwrap().split(" ").last().unwrap().parse::<i128>().unwrap(),
            yes: s.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap(),
            no: s.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap()
        });
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut asd = monkeys.clone();
            let m = asd.get_mut(&i).unwrap();
            m.inspect(&mut monkeys);
            monkeys.insert(i, asd.get(&i).unwrap().clone());
        }
    }

    let mut values = Vec::new();

    for i in monkeys.values() {
        values.push(i.times_inspected);
    }
    values.sort();

    values.last().unwrap() * values[values.len() - 2]
}

fn day11_2(f: &str) -> i128 {
    #[derive(Debug, Clone)]
    struct Monkey {
        items: Vec<i128>,
        times_inspected: i128,
        operation: String,
        op_num: i128,
        test: i128,
        yes: usize,
        no: usize
    }

    impl Monkey {
        fn inspect(&mut self, monkeys: &mut HashMap<usize, Monkey>) {
            for item in self.items.clone().into_iter() {
                let mut amount = self.op_num;
                if self.op_num == -1 {amount = item}
                if self.operation == "*" {
                    self.items[0] = (item * amount) % (11*17*5*13*19*2*3*7);
                } else {
                    self.items[0] = (item + amount) % (11*17*5*13*19*2*3*7);
                }
                if self.items[0] % self.test == 0 {
                    monkeys.get_mut(&self.yes).unwrap().items.push(self.items.remove(0));
                } else {
                    monkeys.get_mut(&self.no).unwrap().items.push(self.items.remove(0));
                }
                self.times_inspected += 1;
            }
        }
    }

    
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    for (monkey, stats) in contents.split("\n\n").enumerate() {
        let mut s = stats.split("\n");
        s.next();
        let mut items = Vec::new();
        for i in s.next().unwrap().split(": ").nth(1).unwrap().split(", ") {
            items.push(i.parse::<i128>().unwrap());
        }
        let mut thing = s.next().unwrap().split("= old ").nth(1).unwrap().split(" ");

        let operation = thing.nth(0).unwrap().to_string();
        let op_num = thing.nth(0).unwrap().parse::<i128>().unwrap_or(-1);

        monkeys.insert(monkey, Monkey {
            items: items,
            times_inspected: 0,
            operation: operation,
            op_num: op_num,
            test: s.next().unwrap().split(" ").last().unwrap().parse::<i128>().unwrap(),
            yes: s.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap(),
            no: s.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap()
        });
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut asd = monkeys.clone();
            let m = asd.get_mut(&i).unwrap();
            m.inspect(&mut monkeys);
            monkeys.insert(i, asd.get(&i).unwrap().clone());
        }
    }

    let mut values = Vec::new();

    for i in monkeys.values() {
        values.push(i.times_inspected);
    }
    values.sort();

    values.last().unwrap() * values[values.len() - 2]
}

fn _day12_1(f: &str) -> i32 { // I really like this code even though its way too slow so i can't delete it
    fn go(pos: (usize, usize), mut map: Vec<Vec<char>>, steps: i32) -> Result<i32, ()> {
        //println!("{steps}");
        //println!("{:?}", pos);
        let abc = "abcdefghijklmnopqrstuvwxyzE.";
        let others = vec![(pos.0, pos.1 + 1), (pos.0 + 1, pos.1), (pos.0 - 1, pos.1), (pos.0, pos.1 - 1)];
        let mut s_o = Vec::new();
        let mut done = true;
        //for i in map.clone() {println!("{:?}", i)}
        for i in others {
            if (i.1 < map.len()) && (i.0 < map[0].len()) {
                //println!("{:?}", map[i.1][i.0]);
                let abcd = abc.clone().chars().into_iter();
                //println!("{}", abcd.clone().position(|a| a == map[i.1][i.0]).unwrap());
                let next_pos = abcd.clone().position(|a| a == map[i.1][i.0]).unwrap();
                let current_pos = abcd.clone().into_iter().position(|a| a == map[pos.1][pos.0]).unwrap();
                if map[i.1][i.0] != '.' && (next_pos == current_pos || next_pos == current_pos + 1) {
                    if map[i.1][i.0] == 'E' {
                        //println!("FOUND IT in {} with {:?}", steps + 1, visited);
                        return Ok(steps + 1)
                    }
                    done = false;
                    s_o.push(i);
                }
            }
        }
        map.get_mut(pos.1).unwrap()[pos.0] = '.';
        if done {return Err(())}
        let mut result = i32::MAX;
        let mut results_j = Vec::new();
        let mut results = Vec::new();

        for i in s_o {
            let s = map.clone();
            if steps < 5 {
                results_j.push(thread::spawn(move || go(i, s, steps + 1)));
            } else {
                results.push(go(i, s, steps + 1))
            }
        }
        for i in results_j {
            if let Ok(a) = i.join().unwrap() {
                if a < result {
                    result = a;
                }
            }
        }

        for i in results {
            if let Ok(a) = i {
                if a < result {
                    result = a;
                }
            }
        }
        if result != i32::MAX {return Ok(result)}
        else {return Err(())}
    }

    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut points = Vec::new();
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..contents.split("\n").nth(0).unwrap().chars().collect::<Vec<char>>().len() + 2 {points.push('.');}
    map.push(points.clone());
    let mut start = (0, 0);

    for (index, i) in contents.split("\n").enumerate() {
        let mut buffer = vec!['.'];
        for (ind, c) in i.chars().enumerate() {
            if c == 'S' {start = (ind + 1, index + 1)}
            buffer.push(c);
        }
        buffer.push('.');
        map.push(buffer);
    }
    map.push(points);
    //println!("{:?}", map);
    map.get_mut(start.1).unwrap()[start.0] = 'a';

    go(start, map.clone(), 0).unwrap()
}

fn day12_1(f: &str) -> i32 {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (index, i) in contents.split("\n").enumerate() {
        let mut buffer = Vec::new();
        for (ind, c) in i.chars().enumerate() {
            if c == 'S' {start = (ind, index)}
            if c == 'E' {end = (ind, index)}
            buffer.push(c);
        }
        map.push(buffer);
    }
    //println!("{:?}", map);
    map.get_mut(start.1).unwrap()[start.0] = 'a';
    map.get_mut(end.1).unwrap()[end.0] = 'z';

    let mut visited = vec![start];
    let mut news: Vec<(usize, usize)> = vec![start];

    for i in 0.. {
        let abc = "abcdefghijklmnopqrstuvwxyz";
        let s_news = news.clone();
        if news.is_empty() {break;}
        news = Vec::new();
        for o in &s_news.clone() {
            if o == &end {return i;}
            let pos_current = abc.chars().into_iter().position(|a| a == map[o.1][o.0]).unwrap();

            if o.1 < map.len() - 1 {
                let pos = abc.chars().into_iter().position(|a| a == map[o.1 + 1][o.0]).unwrap();
                if !visited.contains(&(o.0, o.1 + 1)) && pos <= pos_current + 1 {
                    visited.push((o.0, o.1 + 1));
                    news.push((o.0, o.1 + 1));
                }
            }
            
            if o.1 > 0 {
                let pos = abc.chars().into_iter().position(|a| a == map[o.1 - 1][o.0]).unwrap();
                if !visited.contains(&(o.0, o.1 - 1)) && pos <= pos_current + 1 {
                    visited.push((o.0, o.1 - 1));
                    news.push((o.0, o.1 - 1));
                }
            }

            if o.0 < map[0].len() - 1 {
                let pos = abc.chars().into_iter().position(|a| a == map[o.1][o.0 + 1]).unwrap();
                if !visited.contains(&(o.0 + 1, o.1)) && pos <= pos_current + 1 {
                    visited.push((o.0 + 1, o.1));
                    news.push((o.0 + 1, o.1));
                }
            }
            
            if o.0 > 0 {
                let pos = abc.chars().into_iter().position(|a| a == map[o.1][o.0 - 1]).unwrap();
                if !visited.contains(&(o.0 - 1, o.1)) && pos <= pos_current + 1 {
                    visited.push((o.0 - 1, o.1));
                    news.push((o.0 - 1, o.1));
                }
            }
        }
    }
    return 0
}

fn day12_2(f: &str) -> i32 {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (index, i) in contents.split("\n").enumerate() {
        let mut buffer = Vec::new();
        for (ind, c) in i.chars().enumerate() {
            if c == 'S' {start = (ind, index)}
            if c == 'E' {end = (ind, index)}
            buffer.push(c);
        }
        map.push(buffer);
    }

    map.get_mut(start.1).unwrap()[start.0] = 'a';
    map.get_mut(end.1).unwrap()[end.0] = 'z';
    let mut possible = Vec::new();

    for (y, line) in map.clone().iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'a' {
                possible.push((x, y));
            }
        }
    }

    let mut visited = possible.clone();
    let mut news: Vec<(usize, usize)> = possible;

    for i in 0.. {
        let abc = "abcdefghijklmnopqrstuvwxyz";
        let s_news = news.clone();
        if news.is_empty() {break;}
        news = Vec::new();
        for o in &s_news.clone() {
            if o == &end {return i;}
            let pos_current = abc.chars().into_iter().position(|a| a == map[o.1][o.0]).unwrap();

            if o.1 < map.len() - 1 {
                let pos = abc.chars().into_iter().position(|a| a == map[o.1 + 1][o.0]).unwrap();
                if !visited.contains(&(o.0, o.1 + 1)) && pos <= pos_current + 1 {
                    visited.push((o.0, o.1 + 1));
                    news.push((o.0, o.1 + 1));
                }
            }
            
            if o.1 > 0 {
                let pos = abc.chars().into_iter().position(|a| a == map[o.1 - 1][o.0]).unwrap();
                if !visited.contains(&(o.0, o.1 - 1)) && pos <= pos_current + 1 {
                    visited.push((o.0, o.1 - 1));
                    news.push((o.0, o.1 - 1));
                }
            }

            if o.0 < map[0].len() - 1 {
                let pos = abc.chars().into_iter().position(|a| a == map[o.1][o.0 + 1]).unwrap();
                if !visited.contains(&(o.0 + 1, o.1)) && pos <= pos_current + 1 {
                    visited.push((o.0 + 1, o.1));
                    news.push((o.0 + 1, o.1));
                }
            }
            
            if o.0 > 0 {
                let pos = abc.chars().into_iter().position(|a| a == map[o.1][o.0 - 1]).unwrap();
                if !visited.contains(&(o.0 - 1, o.1)) && pos <= pos_current + 1 {
                    visited.push((o.0 - 1, o.1));
                    news.push((o.0 - 1, o.1));
                }
            }
        }
    }
    return 0
}

fn day13_1(f: &str) -> usize {
    #[derive(PartialEq)]
    enum Thing {
        Yes,
        Maybe,
        No
    }

    #[derive(Debug)]
    enum Content {
        List(Vec<Content>),
        Val(usize)
    }

    impl Content {
        fn eval(&self, other: &Content) -> Thing{
            // println!("Compare {:?} vs {:?}", self, other);
            match (self, other) {
                (Content::Val(num), Content::Val(num_o)) => {
                    if num < num_o {/*println!("Left side is smaller, so inputs are in the right order"); */ return Thing::Yes}
                    else if num > num_o {/*println!("Right side is smaller, so inputs are not in the right order"); */ return Thing::No;}
                    else {return Thing::Maybe;}
                },
                (Content::List(s), Content::List(s_o)) => {
                    for (i, cont) in s.iter().enumerate() {
                        if i >= s_o.len() {/*println!("Right side ran out of items, so inputs are not in the right order"); */ return Thing::No;}
                        let a = cont.eval(s_o.get(i).unwrap());
                        if a == Thing::Yes || a == Thing::No {return a}
                    }
                    if s.len() < s_o.len() {/*println!("Left side ran out of items, so inputs are in the right order"); */ return Thing::Yes;}
                    else {return Thing::Maybe}
                },
                (Content::List(_), Content::Val(s_o)) => {
                    // println!("Mixed types; convert right to [{}] and retry comparison", s_o);
                    return self.eval(&Content::List(vec![Content::Val(*s_o)]))
                },
                (Content::Val(s), Content::List(_)) => {
                    // println!("Mixed types; convert left to [{}] and retry comparison", s);
                    return Content::List(vec![Content::Val(*s)]).eval(other)
                }      
            }
        }
    }

    fn get_items(mut list: Chars) -> (Vec<Content>, Chars) {
        let mut items: Vec<Content> = Vec::new();
        let mut last_was_num = false;

        while let Some(a) = list.next() {
            if let Ok(n) = a.to_string().parse::<usize>() {
                if !last_was_num {
                    items.push(Content::Val(n));
                } else {
                    if let Content::Val(j) = items.pop().unwrap() {
                        items.push(Content::Val((j.to_string() + &a.to_string()).parse::<usize>().unwrap()));
                    } else {panic!("asdasdasd")} 
                }
                last_was_num = true;
            } else if a == '[' {
                let s = get_items(list);
                list = s.1;
                items.push(Content::List(s.0));
                last_was_num = false;
            } else if a == ']' {
                return (items, list)
            } else {
                last_was_num = false;
            }
        }
        return (items, list)
    }

    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let pairs_raw = contents.split("\n\n");
    let mut pairs: Vec<(Content, Content)> = Vec::new();
    let mut out: usize = 0; // change to 0 again

    for pair in pairs_raw {
        let left_raw = pair.split("\n").nth(0).unwrap();
        let right_raw = pair.split("\n").nth(1).unwrap();
        pairs.push((Content::List(get_items(left_raw.chars()).0), Content::List(get_items(right_raw.chars()).0)))
    }

    for (i, pair) in pairs.iter().enumerate() {
        // println!("\n== Pair {} ==", i + 1);
        if pair.0.eval(&pair.1) == Thing::Yes {out += i + 1}
    } 
    out
}

fn day13_2(f: &str) -> usize {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum Thing {
        Yes,
        Maybe,
        No
    }

    #[derive(Debug, Clone, PartialEq)]
    enum Content {
        List(Vec<Content>),
        Val(usize)
    }

    impl Content {
        fn eval(&self, other: &Content) -> Thing{
            // println!("Compare {:?} vs {:?}", self, other);
            match (self, other) {
                (Content::Val(num), Content::Val(num_o)) => {
                    if num < num_o {/*println!("Left side is smaller, so inputs are in the right order"); */ return Thing::Yes}
                    else if num > num_o {/*println!("Right side is smaller, so inputs are not in the right order"); */ return Thing::No;}
                    else {return Thing::Maybe;}
                },
                (Content::List(s), Content::List(s_o)) => {
                    for (i, cont) in s.iter().enumerate() {
                        if i >= s_o.len() {/*println!("Right side ran out of items, so inputs are not in the right order"); */ return Thing::No;}
                        let a = cont.eval(s_o.get(i).unwrap());
                        if a == Thing::Yes || a == Thing::No {return a}
                    }
                    if s.len() < s_o.len() {/*println!("Left side ran out of items, so inputs are in the right order"); */ return Thing::Yes;}
                    else {return Thing::Maybe}
                },
                (Content::List(_), Content::Val(s_o)) => {
                    // println!("Mixed types; convert right to [{}] and retry comparison", s_o);
                    return self.eval(&Content::List(vec![Content::Val(*s_o)]))
                },
                (Content::Val(s), Content::List(_)) => {
                    // println!("Mixed types; convert left to [{}] and retry comparison", s);
                    return Content::List(vec![Content::Val(*s)]).eval(other)
                }      
            }
        }
    }

    fn get_items(mut list: Chars) -> (Vec<Content>, Chars) {
        let mut items: Vec<Content> = Vec::new();
        let mut last_was_num = false;

        while let Some(a) = list.next() {
            if let Ok(n) = a.to_string().parse::<usize>() {
                if !last_was_num {
                    items.push(Content::Val(n));
                } else {
                    if let Content::Val(j) = items.pop().unwrap() {
                        items.push(Content::Val((j.to_string() + &a.to_string()).parse::<usize>().unwrap()));
                    } else {panic!("asdasdasd")} 
                }
                last_was_num = true;
            } else if a == '[' {
                let s = get_items(list);
                list = s.1;
                items.push(Content::List(s.0));
                last_was_num = false;
            } else if a == ']' {
                return (items, list)
            } else {
                last_was_num = false;
            }
        }
        return (items, list)
    }

    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut out = 1;

    let mut packets = Vec::new();
    let mut sorted_packets = Vec::new();
    sorted_packets.push(Content::List(get_items("[[2]]".chars()).0));
    sorted_packets.push(Content::List(get_items("[[6]]".chars()).0));


    for packet in contents.split("\n") {
        if packet == "" {continue;}
        packets.push(Content::List(get_items(packet.chars()).0))
    }

    for packet in packets {
        let mut put_in = false;
        for (i, p) in sorted_packets.clone().iter().enumerate() {
            if packet.eval(p) == Thing::Yes {sorted_packets.insert(i, packet.clone()); put_in = true; break;}
        }
        if !put_in {sorted_packets.push(packet)}
    }

    out *= sorted_packets.iter().position(|a| a == &Content::List(get_items("[[6]]".chars()).0)).unwrap() + 1;
    out * (sorted_packets.iter().position(|a| a == &Content::List(get_items("[[2]]".chars()).0)).unwrap() + 1)
}

fn day14_1(f: &str) -> i32 {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map = HashSet::new();

    for i in contents.split("\n") {
        for s in i.split(" -> ").tuple_windows::<(_, _)>() {
            let s1: (&str, &str) = s.0.split(",").into_iter().next_tuple().unwrap();
            let s2: (&str, &str) = s.1.split(",").into_iter().next_tuple().unwrap();

            if s1.0.parse::<usize>().unwrap() <= s2.0.parse::<usize>().unwrap() {
                for o in s1.0.parse::<usize>().unwrap()..=s2.0.parse::<usize>().unwrap() {
                    map.insert((o, s2.1.parse::<usize>().unwrap()));
                }
            } else {
                for o in s2.0.parse::<usize>().unwrap()..=s1.0.parse::<usize>().unwrap() {
                    map.insert((o, s2.1.parse::<usize>().unwrap()));
                }
            }

            for o in s1.0.parse::<usize>().unwrap()..=s2.0.parse::<usize>().unwrap() {
                map.insert((o, s2.1.parse::<usize>().unwrap()));
            }
            if s1.1.parse::<usize>().unwrap() <= s2.1.parse::<usize>().unwrap() {
                for o in s1.1.parse::<usize>().unwrap()..=s2.1.parse::<usize>().unwrap() {
                    map.insert((s2.0.parse::<usize>().unwrap(), o));
                }
            } else {
                for o in s2.1.parse::<usize>().unwrap()..=s1.1.parse::<usize>().unwrap() {
                    map.insert((s2.0.parse::<usize>().unwrap(), o));
                }
            }
        }
    }

    let lowest = 10000;

    for i in 1.. {
        let mut done = false;
        let mut pos = (500, 0);
        while !done {
            if pos.1 > lowest {return i - 1}
            if !map.contains(&(pos.0, pos.1 + 1)) {
                pos.1 += 1;
            } else if !map.contains(&(pos.0 - 1, pos.1 + 1)) {
                pos = (pos.0 - 1, pos.1 + 1);
            } else if !map.contains(&(pos.0 + 1, pos.1 + 1)) {
                pos = (pos.0 + 1, pos.1 + 1);
            } else {done = true; map.insert(pos);}
        }
    };

    0
}

fn day14_2(f: &str) -> i32 {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map = HashSet::new();

    for i in contents.split("\n") {
        for s in i.split(" -> ").tuple_windows::<(_, _)>() {
            let s1: (&str, &str) = s.0.split(",").into_iter().next_tuple().unwrap();
            let s2: (&str, &str) = s.1.split(",").into_iter().next_tuple().unwrap();

            if s1.0.parse::<usize>().unwrap() <= s2.0.parse::<usize>().unwrap() {
                for o in s1.0.parse::<usize>().unwrap()..=s2.0.parse::<usize>().unwrap() {
                    map.insert((o, s2.1.parse::<usize>().unwrap()));
                }
            } else {
                for o in s2.0.parse::<usize>().unwrap()..=s1.0.parse::<usize>().unwrap() {
                    map.insert((o, s2.1.parse::<usize>().unwrap()));
                }
            }

            for o in s1.0.parse::<usize>().unwrap()..=s2.0.parse::<usize>().unwrap() {
                map.insert((o, s2.1.parse::<usize>().unwrap()));
            }
            if s1.1.parse::<usize>().unwrap() <= s2.1.parse::<usize>().unwrap() {
                for o in s1.1.parse::<usize>().unwrap()..=s2.1.parse::<usize>().unwrap() {
                    map.insert((s2.0.parse::<usize>().unwrap(), o));
                }
            } else {
                for o in s2.1.parse::<usize>().unwrap()..=s1.1.parse::<usize>().unwrap() {
                    map.insert((s2.0.parse::<usize>().unwrap(), o));
                }
            }
        }
    }

    let mut lowest = 0;
    for i in map.clone() {
        if i.1 > lowest {lowest = i.1}
    }
    lowest += 2;

    for i in 1.. {
        let mut done = false;
        let mut pos = (500, 0);
        if map.contains(&pos) {return i - 1}
        while !done {
            if pos.1 == lowest - 1 {map.insert(pos); break;}
            if !map.contains(&(pos.0, pos.1 + 1)) {
                pos.1 += 1;
            } else if !map.contains(&(pos.0 - 1, pos.1 + 1)) {
                pos = (pos.0 - 1, pos.1 + 1);
            } else if !map.contains(&(pos.0 + 1, pos.1 + 1)) {
                pos = (pos.0 + 1, pos.1 + 1);
            } else {done = true; map.insert(pos);}
        }
    };

    0
}

fn _day15_1(f: &str, l: i32) -> usize {
    let mut contents = String::new();
    File::open(f).unwrap().read_to_string(&mut contents).unwrap();

    let mut sensors: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut beacons = HashSet::new();
    let mut leftmost = i32::MAX;
    let mut rightmost = i32::MIN;

    for line in contents.split("\n") {
        let pos = (
            line.split("=").nth(1).unwrap().split(",").nth(0).unwrap().parse::<i32>().unwrap(),
            line.split("=").nth(2).unwrap().split(":").nth(0).unwrap().parse::<i32>().unwrap()
        );

        let pos_beacon = (
            line.split("=").nth(3).unwrap().split(",").nth(0).unwrap().parse::<i32>().unwrap(),
            line.split("=").nth(4).unwrap().parse::<i32>().unwrap()
        );

        beacons.insert(pos_beacon);

        let sensor = (pos.0, pos.1, (pos.0.abs_diff(pos_beacon.0) + pos.1.abs_diff(pos_beacon.1)) as i32);

        if sensor.0 - sensor.2 < leftmost {leftmost = sensor.0 - sensor.2}
        else if sensor.0 + sensor.2 > rightmost {rightmost = sensor.0 + sensor.2}

        sensors.insert(sensor);
    }

    let mut out = 0;

    for i in leftmost - 1..=rightmost + 1 {
        for s in sensors.clone() {
            if (s.0.abs_diff(i) + s.1.abs_diff(l)) as i32 <= s.2 {
                if !beacons.contains(&(i, l)) {
                    out += 1;
                    break;
                }
            }
        }
    }

    out
}

fn _day15_2(f: &str, max: i32) -> i64 {
    let mut contents = String::new();
    File::open(f).unwrap().read_to_string(&mut contents).unwrap();

    let mut sensors: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut leftmost = i32::MAX;
    let mut rightmost = i32::MIN;

    for line in contents.split("\n") {
        let pos = (
            line.split("=").nth(1).unwrap().split(",").nth(0).unwrap().parse::<i32>().unwrap(),
            line.split("=").nth(2).unwrap().split(":").nth(0).unwrap().parse::<i32>().unwrap()
        );

        let pos_beacon = (
            line.split("=").nth(3).unwrap().split(",").nth(0).unwrap().parse::<i32>().unwrap(),
            line.split("=").nth(4).unwrap().parse::<i32>().unwrap()
        );

        let sensor = (pos.0, pos.1, (pos.0.abs_diff(pos_beacon.0) + pos.1.abs_diff(pos_beacon.1)) as i32);

        if sensor.0 - sensor.2 < leftmost {leftmost = sensor.0 - sensor.2}
        else if sensor.0 + sensor.2 > rightmost {rightmost = sensor.0 + sensor.2}

        sensors.insert(sensor);
    }

    for s in sensors.clone() {
        if !(s.0 - s.2 <= max && s.0 + s.2 >= 0) && (s.1 - s.2 <= max && s.1 + s.2 >= 0) {
            sensors.remove(&s);
        }
    }


    for i in 0..=max {
        let asd = sensors.clone();

        let mut possible_sensors = HashSet::new();
        for s in asd.clone() {
            if (s.0 - s.2 <= max && s.0 + s.2 >= 0) && (s.1 - s.2 <= i && s.1 + s.2 >= i) {possible_sensors.insert(s);}
        }

        let mut l = 0;
        while l <= max {
            let mut found = true;
            for s in possible_sensors.clone() {
                if (s.0.abs_diff(l) as usize + s.1.abs_diff(i) as usize) as i64 <= s.2 as i64 {
                    found = false;
                    l = s.0 + s.2 - (i - s.1);
                    break;
                }
            }
            if found {
                return (l as i64 * 4000000) + i as i64
            }
            l += 1;
        }
    }
    666
}

fn _day16_1(f: &str) -> usize {
    #[derive(Debug, Clone, PartialEq)]
    struct Valve {
        flow_rate: usize,
        goes_to: Vec<String>,
        name: String
    }

    impl Valve {
        fn eval(&self, valves: &HashMap<String, Valve>, thing: usize) -> usize {
            let mut out = 0;
            let mut starts = vec![self.clone()];
            for i in thing.. {
                let mut visited = Vec::new();
                let mut new_starts = Vec::new();

                for v in starts.clone() {
                    if !visited.contains(&v) {
                        out += v.flow_rate / i;
                        visited.push(v.clone());
                    }
                    for s in v.goes_to {
                        new_starts.push(valves.get(&s).unwrap().clone());
                    }
                }
                if new_starts.is_empty() {
                    break;
                }
                starts.clear();
                starts = new_starts;
            }
            out
        }

        
    }

    let mut contents = String::new();
    File::open(f).unwrap().read_to_string(&mut contents).unwrap();

    let mut valves = HashMap::new();

    for i in contents.split("\n") {
        valves.insert(i.split(" ").nth(1).unwrap().to_string(), Valve {
            flow_rate: i.split("=").nth(1).unwrap().split(";").nth(0).unwrap().parse::<usize>().unwrap(),
            goes_to: i.split("valves ").nth(1).unwrap().split(", ").map_into().collect_vec(),
            name: i.split(" ").nth(1).unwrap().to_string()
        });
    }
    let t = Instant::now();
    // let out = valves.clone().get(&"AA".to_string()).unwrap().go(&valves, 30, 0, HashSet::new());
    let mut out = 0;
    let mut current = valves.get("AA").unwrap();

    for i in 0..30 {
        //let mut possibilities = Vec::new();
        for v in &current.goes_to {
            
        }
    }
    println!("{:?}", t.elapsed());
    return out
} 
fn main() {
    let time = Instant::now();

    println!("Day 1_1: {}", day1_1());
    println!("Day 1_2: {}", day1_2());
    println!("Day 2_1: {}", day2_1());
    println!("Day 2_2: {}", day2_2());
    println!("Day 3_1: {}", day3_1());
    println!("Day 3_2: {}", day3_2());
    println!("Day 4_1: {}", day4_1());
    println!("Day 4_2: {}", day4_2());
    println!("Day 5_1: {}", day5_1());
    println!("Day 5_2: {}", day5_2("input5.txt"));
    println!("Day 6_1: {}", day6_1("input6.txt"));
    println!("Day 6_2: {}", day6_2("input6.txt"));
    println!("Day 7_1: {}", day7_1("input7.txt"));
    println!("Day 7_2: {}", day7_2("input7.txt"));
    // Day 8 on my tablet in Python
    println!("Day 9_1: {}", day9_1("input9.txt"));
    println!("Day 9_2: {}", day9_2("input9.txt"));
    println!("Day 10_1: {}", day10_1("input10.txt"));
    println!("Day 10_2: {}", day10_2("input10.txt"));
    println!("Day 11_1: {}", day11_1("input11.txt"));
    println!("Day 11_2: {}", day11_2("input11.txt"));
    println!("Day 12_1: {}", day12_1("input12.txt"));
    println!("Day 12_2: {}", day12_2("input12.txt"));
    println!("Day 13_1: {}", day13_1("input13.txt"));
    println!("Day 13_2: {}", day13_2("input13.txt"));
    println!("Day 14_1: {}", day14_1("input14.txt"));
    println!("Day 14_2: {}", day14_2("input14.txt"));
    // println!("Day 15_1: {}", _day15_1("input15.txt", 2000000));
    // println!("Day 15_2: {}", _day15_2("input15.txt", 4000000));
    // println!("Day 16_1: {}", _day16_1("input16.txt"));

    println!("\nfinished in {:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test5_2() {
        assert_eq!(day5_2("test5.txt"), "MCD")
    }

    #[test]
    fn test6_1() {
        assert_eq!(day6_1("test6.txt"), 10)
    }

    #[test]
    fn test7_1() {
        assert_eq!(day7_1("test7.txt"), 95437)
    }

    #[test]
    fn test7_2() {
        assert_eq!(day7_2("test7.txt"), 24933642)
    }

    #[test]
    fn test9_1() {
        assert_eq!(day9_1("test9.txt"), 13)
    }

    #[test]
    fn test9_2() {
        assert_eq!(day9_2("test9_2.txt"), 36)
    }

    #[test]
    fn test10_1() {
        assert_eq!(day10_1("test10.txt"), 13140)
    }

    #[test]
    fn test12_1() {
        assert_eq!(day12_1("test12.txt"), 31)
    }

    #[test]
    fn test12_2() {
        assert_eq!(day12_2("test12.txt"), 29)
    }

    #[test]
    fn test13_1() {
        assert_eq!(day13_1("test13.txt"), 13)
    }

    #[test]
    fn test13_2() {
        assert_eq!(day13_2("test13.txt"), 140)
    }

    #[test]
    fn test14_1() {
        assert_eq!(day14_1("test14.txt"), 24)
    }

    #[test]
    fn test14_2() {
        assert_eq!(day14_2("test14.txt"), 93)
    }

    #[test]
    fn test15_1() {
        assert_eq!(_day15_1("test15.txt", 10), 26)
    }

    #[test]
    fn test15_2() {
        assert_eq!(_day15_2("test15.txt", 20), 56000011)
    }

    // #[test]
    // fn test16_1() {
    //     assert_eq!(_day16_1("test16.txt"), 1651)
    // }
}