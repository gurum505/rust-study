// cargo check를 주기적으로 한다음, 필요할 때 cargo build를 한다. 
// cargo build --release를 사용하여 릴리즈 빌드를 생성할 수 있습니다.
// 작동 속도를 벤치마킹할 시에는 릴리즈 빌드를 기준으로 해야 한다는 것도 알아두시기 바랍니다.
fn main() {
    println!("Hello, world!");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //let mut spaces였다면 변수타입을 바꿀 수 없어, spaces.len()를 사용할 수 없었을 것이다.
    let spaces = "   ";
    let spaces = spaces.len(); //let으로 변수 shadowing을 통해 spaces를 문자열에서 정수로 바꿔줌

    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);
    println!("spaces: {}", spaces);

    //러스트는 정적 타입의 (statically typed) 언어라는 점을 주지하세요. 이게 의미하는 바는 모든 변수의 타입이 컴파일 시점에 반드시 정해져 있어야 한다는 겁니다.

    // 정수 오버플로우가 발생하면 즉시 패닉(Panic)을 일으켜 프로그램을 중단시키는 것이 기본 동작입니다.

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;

    //러스트는 여러분의 함수 위치를 고려하지 않으며, 호출하는 쪽에서 볼 수 있는 스코프 어딘가에 정의만 되어있으면 됩니다.
    another_function(five_hundred);

    //구문은 값을 반환하지 않습니다
    let y = 6;

    //표현식은 값을 반환합니다.(let y 뒤의 {...}가 표현식)
    let y = {
        let x = 3;
        x + 1 // 마지막 줄에는 세미콜론이 없으므로, 이 블록은 4를 반환합니다.
    };

    // if - elseif - else
    // 한줄 if문도 가능
    let condition = true;
    let number = if condition { 5 } else { 6 };

    let counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //반환값. 구문(statement)이다. 
        }
    };

    //대부분은 for사용
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


//함수가 매개변수를 갖고 있으면 이 매개변수에 대한 구체적인 값을 전달할 수 있습니다. 엄밀하게는 이러한 구체적인 값을 인수 (argument) 라고 부르지만 용어를 혼용하는 경향이 있습니다.
fn another_function(x: i32) { // 함수 시그니처에서는 각 매개변수의 타입을 반드시 선언해야 합니다. 이는 러스트를 설계하면서 신중하게 내린 결정 사항입니다: 
    println!("Another function.");
}