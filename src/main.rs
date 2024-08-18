use std::io;
use std::io::Write;
use std::process;

fn main() {

    loop {
        let input: u32 = fibonacci_input();
        // println!("input number : {input}");    
        calculate_fibonacci(input);
    }
}

fn fibonacci_input() -> u32{
    // 프로그램 안내문 출력
    println!("본 프로그램은 피보나치 수열의 n번째에 어떤 숫자가 있는 지 알려주는 프로그램입니다.");

    loop{
        // 입력 안내문 출력
        print!("알고싶은 피보나치의 수의 위치 n(자연수)를 입력해주세요(종료하려면 0) : ");
        io::stdout().flush().expect("failed to flush stdout");
    
        // 입력
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
    
        // 유효성 검사
        // 0을 제외한 자연수는 반환 후 함수 종료
        // 0이 나오면 프로그램 종료
        // 그외의 입력은 실패시키고 다시 안내문 출력으로 회귀
        match input.trim().parse::<u32>() {
            Ok(natual_number) if natual_number == 0 => {
                println!("프로그램을 종료합니다.");
                process::exit(0);
            },
            Ok(natual_number) => return natual_number,
            _ => println!("유효하지 않은 입력입니다. 표시할 피보나치 수의 위치 n을 자연수로 입력해주세요."),
        }
    }
}

fn calculate_fibonacci(input: u32){
    // let mut fibonacci: [u32; 3] = [0,1,1];
    // let num = match input {
    //     1 | 2 => 1,
    //     _ => {
    //         for i in 2..input {
    //             fibonacci[0] = fibonacci[1];
    //             fibonacci[1] = fibonacci[2];
    //             fibonacci[2] = fibonacci[0] + fibonacci[1];
    //         }
    //         fibonacci[2]
    //     }
    // };

    // println!("{input}번째 피보나치의 수 : {num}");

    let num = match input {
        1 | 2 => 1, // 1번과 2번 피보나치의 수는 1로 동일
        _ => {
            let mut prev = 1;
            let mut curr = 1;
            for _ in 3..=input {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            curr
        }
    };
    println!("{input}번째 피보나치의 수 : {num}");
}