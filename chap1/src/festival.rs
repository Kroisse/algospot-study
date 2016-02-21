/*
    https://algospot.com/judge/problem/read/FESTIVAL

    - 문제 풀이보다 입출력과 테스트 코드 짜는 데에 시간을 더 잡아먹는 것 같다.
    - 일단 단순무식한 방법부터 시도
      - 채점 결과 오답
      - edge case를 잘못 파악하고 있는 것 같음
      - 더 무식한 방법 시도: 모든 begin/end 쌍을 시도해 보고 길이가 n_team보다 짧으면 continue
      - 그래도 오답. 출력 오류를 의심해봐야 하나?
      - ...진짜다. 자릿수를 11자리까지 찍게 하니 정답 처리됨;;
    - 복붙하기 껄끄러우니 테스트 코드를 별도 파일로 분리함.
    - 이제 시간을 줄여 보자. 앞서 푼 사람들의 기록을 보면 10ms 이내에 달성 가능함.
 */
mod festival_test;

use std::io;
use std::cmp;

#[derive(Clone, Copy)]
pub struct Fraction { pub numerator: u32, pub denominator: u32 }

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Fraction {
        Fraction { numerator: numerator, denominator: denominator }
    }
}

impl cmp::PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl cmp::Eq for Fraction { }

impl cmp::PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Fraction) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Fraction {
    fn cmp(&self, other: &Fraction) -> cmp::Ordering {
        let Fraction { numerator: a, denominator: b} = *self;
        let Fraction { numerator: c, denominator: d} = *other;
        cmp::Ord::cmp(&(a * d), &(c * b))
    }
}

impl From<Fraction> for f64 {
    fn from(frac: Fraction) -> f64 {
        frac.numerator as f64 / frac.denominator as f64
    }
}

/*
    calculate_v1 + Fraction + calculate_v3!
    28ms.
 */
fn partial_sum(costs: &[u32]) -> Vec<u32> {
    let mut partial_sum = Vec::with_capacity(costs.len() + 1);
    partial_sum.push(0);
    partial_sum.extend(costs.iter().scan(0, |state, &x| { *state += x; Some(*state) }));
    partial_sum
}

fn calculate(costs: &[u32], n_team: u32) -> f64 {
    let infinity = Fraction::new(1, 0);
    assert!(n_team as usize <= costs.len());
    let partial_sums = partial_sum(costs);
    let mut min_avg_cost = infinity;
    for i in 0..(costs.len() - n_team as usize + 1) {
        let optional_begins = i + n_team as usize - 1;
        let sum = unsafe {
            partial_sums.get_unchecked(optional_begins) - partial_sums.get_unchecked(i)
        };
        let optional = &costs[optional_begins..costs.len()];
        let mut cost = Fraction::new(sum, n_team - 1);
        for &c in optional {
            cost.numerator += c;
            cost.denominator += 1;
            if min_avg_cost > cost {
                min_avg_cost = cost;
            }
        }
    }
    f64::from(min_avg_cost)
}

/*
    이쪽도 개선해봅시다. 쓸데없는 string split, 메모리 할당 등이 많았음.
    ...구현하기 엄청 짜증남.

    36ms. 효과 없음.
 */
fn read_u32(read: &mut io::BufRead) -> io::Result<u32> {
    let mut result: Option<u32> = None;
    let mut count;
    'a: loop {
        count = 0;
        {
            let buf = try!(read.fill_buf());
            if buf.len() == 0 {
                break 'a;
            }
            for &b in buf {
                result = match (b, result) {
                    (b'0' ... b'9', Some(r)) => Some(r * 10 + (b - b'0') as u32),
                    (b'0' ... b'9', None)    => Some((b - b'0') as u32),
                    _ => { break 'a; }
                };
                count += 1;
            }
        }
        read.consume(count);
    }
    read.consume(count);
    if let Some(r) = result {
        Ok(r)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "failed to read"))
    }
}

fn consume<'a, F>(read: &mut io::BufRead, mut pred: F) -> io::Result<usize> where F: FnMut(u8) -> bool + 'a {
    let mut count = 0;
    let mut c;
    'c: loop {
        c = 0;
        {
            let buf = try!(read.fill_buf());
            if buf.len() == 0 {
                break 'c;
            }
            for &b in buf {
                if pred(b) {
                    /* just pass it */
                } else {
                    break 'c;
                }
                c += 1;
            }
        }
        read.consume(c);
        count += c;
    }
    read.consume(c);
    Ok(count + c)
}

fn process(input: &mut io::BufRead, output: &mut io::Write) {
    let cases: u32 = read_u32(input).unwrap();
    consume(input, |b| b == b'\n').unwrap();
    for _ in 0..cases {
        let days = read_u32(input).unwrap() as usize;
        consume(input, |b| b == b' ').unwrap();
        let teams = read_u32(input).unwrap();
        consume(input, |b| b == b'\n').unwrap();
        let mut costs = Vec::with_capacity(days);
        for _ in 0..days {
            costs.push(read_u32(input).unwrap());
            consume(input, |b| b == b' ').unwrap();
        }
        consume(input, |b| b == b'\n').unwrap();
        let result = calculate(&costs, teams);
        writeln!(output, "{:.11}", result).unwrap();
    }
}


fn main() {
    let stdin = io::stdin();
    process(&mut stdin.lock(), &mut io::stdout());
}

#[allow(dead_code)]
mod old {
    use std;
    use std::io;
    use std::cmp;

    /*
        calculate_v1 + Fraction
        34ms...
     */
    fn calculate_v4(costs: &[u32], n_team: u32) -> f64 {
        let infinity = Fraction::new(1, 0);
        assert!(n_team as usize <= costs.len());
        let mut min_avg_cost = infinity;
        for i in 0..(costs.len() - n_team as usize + 1) {
            let partial = &costs[i..costs.len()];
            let (mandatory, optional) = partial.split_at(n_team as usize - 1);
            let sum = mandatory.iter().fold(0, |acc, x| acc + x);
            let mut cost = Fraction::new(sum, n_team - 1);
            for &c in optional {
                cost.numerator += c;
                cost.denominator += 1;
                if min_avg_cost > cost {
                    min_avg_cost = cost;
                }
            }
        }
        f64::from(min_avg_cost)
    }

    /*
        실수 연산을 배제해 봅시다.
        36ms!
     */
    #[derive(Clone, Copy)]
    pub struct Fraction { pub numerator: u32, pub denominator: u32 }

    impl Fraction {
        pub fn new(numerator: u32, denominator: u32) -> Fraction {
            Fraction { numerator: numerator, denominator: denominator }
        }
    }

    impl cmp::PartialEq for Fraction {
        fn eq(&self, other: &Fraction) -> bool {
            self.cmp(other) == cmp::Ordering::Equal
        }
    }

    impl cmp::Eq for Fraction { }

    impl cmp::PartialOrd for Fraction {
        fn partial_cmp(&self, other: &Fraction) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl cmp::Ord for Fraction {
        fn cmp(&self, other: &Fraction) -> cmp::Ordering {
            let Fraction { numerator: a, denominator: b} = *self;
            let Fraction { numerator: c, denominator: d} = *other;
            cmp::Ord::cmp(&(a * d), &(c * b))
        }
    }

    impl From<Fraction> for f64 {
        fn from(frac: Fraction) -> f64 {
            frac.numerator as f64 / frac.denominator as f64
        }
    }

    /*
        위험한 짓을 해보자.
        29ms.
     */
    fn calculate_v3(costs: &[u32], n_team: u32) -> f64 {
        let n_team = n_team as usize;
        assert!(n_team <= costs.len());
        let mut min_avg_cost = Fraction::new(1, 0); // infinity
        let mut partial_sum = Vec::with_capacity(costs.len() + 1);
        partial_sum.push(0);
        partial_sum.extend(costs.iter().scan(0, |state, &x| { *state += x; Some(*state) }));
        let partial_sum = partial_sum;
        for begin in 0..(costs.len() - n_team + 1) {
            for end in (begin + n_team)..(costs.len() + 1) {
                let sum = unsafe {
                    partial_sum.get_unchecked(end) - partial_sum.get_unchecked(begin)
                };
                let avg_cost = Fraction::new(sum, (end - begin) as u32);
                if min_avg_cost > avg_cost {
                    min_avg_cost = avg_cost;
                }
            }
        }
        f64::from(min_avg_cost)
    }

    /*
        비용 배열   1  2  3  1  2  3
        비용 부분합 1  3  6  7  9 12

        범위 합 연산을 부분합 인덱스 두 번 + 뺄셈 한 번으로 해결할 수 있다.

        하지만 133ms -> 75ms
        - Vec::with_capacity()의 유무가 이렇게 큰 차이를 만듭니다.
     */
    fn calculate_v2(prices: &[u32], n_team: u32) -> f64 {
        let n_team = n_team as usize;
        assert!(n_team <= prices.len());
        let mut min_avg_price = std::f64::MAX;
        let mut partial_sum = Vec::with_capacity(prices.len() + 1);
        partial_sum.push(0);
        partial_sum.extend(prices.iter().scan(0, |state, &x| { *state += x; Some(*state) }));
        let partial_sum = partial_sum;
        for begin in 0..(prices.len() - n_team + 1) {
            for end in (begin + n_team)..(prices.len() + 1) {
                let sum = partial_sum[end] - partial_sum[begin];
                let avg_price = sum as f64 / (end - begin) as f64;
                if min_avg_price > avg_price {
                    min_avg_price = avg_price;
                }
            }
        }
        min_avg_price
    }

    // O(n^2), 83ms
    fn calculate_v1(prices: &[u32], n_team: u32) -> f64 {
        let n_team = n_team as usize;
        assert!(n_team <= prices.len());
        let mut min_avg_price = std::f64::MAX;
        for i in 0..(prices.len() - n_team + 1) {
            let partial = &prices[i..prices.len()];
            let scaned = partial.iter().zip(1..).scan(0, |state, (&x, n)| {
                *state += x;
                Some(if n < n_team { None } else { Some(*state as f64 / n as f64) })
            }).skip_while(Option::is_none).map(Option::unwrap);
            let avg_price = scaned.fold(std::f64::MAX, |acc, x| if acc > x { x } else { acc });
            if min_avg_price > avg_price {
                min_avg_price = avg_price
            }
        }
        min_avg_price
    }

    // O(n^3), 약 700ms
    // 여기서 적어도 매 begin-end 쌍마다 range 안에서 또 루프를 도는 건 피할 수 있을 것 같다.
    fn calculate_v0(prices: &[u32], n_team: u32) -> f64 {
        if n_team == 0 { return 0. }
        let n_team = n_team as usize;
        assert!(n_team <= prices.len());
        let mut min_avg_price = std::f64::MAX;
        for begin in 0..prices.len() {
            for end in (begin + 1)..(prices.len() + 1) {
                if end - begin < n_team { continue; }
                let schedule = &prices[begin..end];
                let avg_price = schedule.iter().fold(0, |acc, &x| acc + x) as f64 / schedule.len() as f64;
                if min_avg_price > avg_price {
                    min_avg_price = avg_price;
                }
            }
        }
        min_avg_price
    }

    fn process_v1(input: &mut io::BufRead, output: &mut io::Write) {
        let mut line = String::new();
        input.read_line(&mut line).unwrap();
        let cases: u32 = line.trim().parse().unwrap();
        for _ in 0..cases {
            line.clear();
            input.read_line(&mut line).unwrap();
            let mut info = Vec::with_capacity(2);
            info.extend(line.trim().split(' ').map(|e| e.parse::<u32>().unwrap()));
            let (days, teams) = (info[0] as usize, info[1]);
            line.clear();
            input.read_line(&mut line).unwrap();
            let mut prices = Vec::with_capacity(days);
            prices.extend(line.trim().split(' ').map(|e| e.parse::<u32>().unwrap()));
            assert!(days as usize == prices.len());
            let result = super::calculate(&prices, teams);
            writeln!(output, "{:.11}", result).unwrap();
        }
    }
}
