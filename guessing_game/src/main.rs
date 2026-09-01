// 프렐루드 (prelude) : use로 가져오지 않아도 사용가능한 요소 모음
// 그 외에는 use를 통해 가져와야 한다.
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //i32가 기본

    println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        // &는 참조자(reference)로 기본적으로 불변.
        // &mut는 가변 참조자(mutable reference)로, 참조한 값을 변경할 수 있음.
        io::stdin()
            .read_line(&mut guess) 
            // read_line()은 Result(enum, 열거형)를 반환하므로, expect()를 통해 에러 처리.
            .expect("Failed to read line");
    
        // shadowing : 이전에 선언한 변수와 같은 이름으로 새 변수를 선언하여, 이전 변수를 가림.
        // 어떤 한 타입의 값을 다른 타입으로 바꾸고 싶을 때 자주 사용되는 기능
        // trim()은 read_line의 결과값에 포함된 개행문자를 제거하는 메서드
        // 문자열의 parse 메서드는 문자열을 다른 타입으로 바꿔줍니다
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,  
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
