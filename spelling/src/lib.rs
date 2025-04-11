pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let units = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    fn under_thousand(n: u64, units: &[&str], teens: &[&str], tens: &[&str]) -> String {
        let mut parts = vec![];

        let hundred = n / 100;
        let rem = n % 100;

        if hundred > 0 {
            parts.push(format!("{} hundred", units[hundred as usize]));
        }

        if rem >= 10 && rem < 20 {
            parts.push(teens[(rem - 10) as usize].to_string());
        } else {
            let ten = rem / 10;
            let unit = rem % 10;

            if ten > 0 {
                if unit > 0 {
                    parts.push(format!("{}-{}", tens[ten as usize], units[unit as usize]));
                } else {
                    parts.push(tens[ten as usize].to_string());
                }
            } else if unit > 0 {
                parts.push(units[unit as usize].to_string());
            }
        }

        parts.join(" ")
    }

    let mut result = vec![];

    let million = n / 1_000_000;
    let thousand = (n % 1_000_000) / 1_000;
    let rest = n % 1_000;

    if million > 0 {
        result.push(format!("{} million", spell(million)));
    }

    if thousand > 0 {
        result.push(format!("{} thousand", under_thousand(thousand, &units, &teens, &tens)));
    }

    if rest > 0 {
        result.push(under_thousand(rest, &units, &teens, &tens));
    }

    result.join(" ")
}
