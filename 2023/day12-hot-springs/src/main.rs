#[derive(Debug)]
struct Record {
    record: Vec<char>,
    truth: Vec<i32>,
}

#[derive(Debug)]
struct GroupedRecord {
    record: Vec<Vec<char>>,
    truth: Vec<i32>,
}

fn parse() -> Vec<Record> {
    std::io::stdin()
        .lines()
        .map(|v| v.unwrap())
        .map(|line| {
            let mut iter = line.split(' ');
            let record = iter.next().unwrap().chars().collect::<Vec<char>>();
            let truth = iter
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Record {
                record: record,
                truth: truth,
            }
        })
        .collect::<Vec<Record>>()
}

fn grouping(record: &Record) -> GroupedRecord {
    let s = String::from_iter(record.record.iter());
    let grouped_record = s
        .split('.')
        .filter(|s| s.len() != 0)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    GroupedRecord {
        record: grouped_record,
        truth: record.truth.clone(),
    }
}

fn remove_exacts(record: &GroupedRecord) -> GroupedRecord {}

fn main() {
    println!("Hello, world!");

    let record = parse();
    println!("{:?}", record);

    let record_phase1 = record.iter().map(|r| grouping(&r)).collect::<Vec<_>>();
    println!("{:?}", record_phase1);

    // example input
    // ???.### 1,1,3
    // phase1, split into pieces  => [???, ###], [1,1,3]
    // phase2, remove exact matches => [???] [1,1]
    // phase3, apply and get result 1

    // example input
    // ????.#...#... 4,1,1
}
