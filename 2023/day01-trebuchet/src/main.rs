fn main() -> Result<(), std::io::Error> {
    let lines = std::io::stdin().lines();

    let m = lines
        .into_iter()
        .map(|v| v.unwrap())
        .map(|lines| {
            let mut vs = Vec::new();
            for c in lines.chars() {
                let m = c.to_digit(10);
                if let Some(x) = m {
                    vs.push(x);
                }
            }
            if vs.len() == 1 {
                vs.push(vs[0])
            }
            vs
        })
        .map(|vs| vs[0] * 10 + vs[vs.len() - 1]);

    println!("The sum is {:?}", m.sum::<u32>());

    Ok(())
}
