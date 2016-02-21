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
use std::io;
use std::cmp;

/*
    실수 연산을 배제해 봅시다.
    36ms!
 */
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

fn calculate(costs: &[u32], n_team: u32) -> f64 {
    let n_team = n_team as usize;
    assert!(n_team <= costs.len());
    let mut min_avg_cost = Fraction::new(1, 0); // infinity
    let mut partial_sum = Vec::with_capacity(costs.len() + 1);
    partial_sum.push(0);
    partial_sum.extend(costs.iter().scan(0, |state, &x| { *state += x; Some(*state) }));
    let partial_sum = partial_sum;
    for begin in 0..(costs.len() - n_team + 1) {
        for end in (begin + n_team)..(costs.len() + 1) {
            let sum = partial_sum[end] - partial_sum[begin];
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


fn process(input: &mut io::BufRead, output: &mut io::Write) {
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
        let result = calculate(&prices, teams);
        writeln!(output, "{:.11}", result).unwrap();
    }
}


fn main() {
    let stdin = io::stdin();
    process(&mut stdin.lock(), &mut io::stdout());
}


mod festival_test;
