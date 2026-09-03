fn main() {
    let home = IpAddrKind::V4(String::from("::1"));

    // 러스트는 이 타입들을 추론할 수 있습니다.
    let some_number = Some(5); //some_number의 타입은 Option<i32>입니다. 
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y; //error


}



// 열거형 vs 구조체
// struct 키워드를 사용하지 않는다는 것과 모든 배리언트가 Message 타입으로 묶인다는 것입니다. 
// 구조체(struct): 정의된 모든 필드가 항상 함께 존재합니다.
// 열거형(enum): 여러 모양 중 "딱 하나만" 선택해서 가질 수 있는 상자입니다. (예: Message는 Quit이거나, Move이거나, Write 중 단 하나의 상태만 가집니다.)

// 러스트의 각 배리언트(Variant)는 실제로 구조체(Struct)나 튜플 구조체의 문법을 그대로 품을 수 있습니다.
// variant : 열거형에 정의된 각기 다른 값
// 여기서 IpAddrKind라는 이름으로 네임스페이스(공간)가 생성됩니다.
// 그 안에 들어있는 V4와 V6는 IpAddrKind라는 폴더 안에 들어있는 값이 됩니다.
enum IpAddrKind {
    V4(String),
    V6,
}

enum Message {
    Quit,                         // 데이터가 없는 형태 (기본 enum)
    Move { x: i32, y: i32 },     // 💡 이름이 있는 필드 (struct 모양!)
    Write(String),                // 💡 괄호가 있는 데이터 (튜플 struct 모양!)
    ChangeColor(i32, i32, i32),   // 💡 여러 개의 값을 담은 튜플 모양
}

impl Message {
    fn call(&self) {
    }
}



// 러스트는 다른 언어들에서 흔하게 볼 수 있는 널 (null) 개념이 없습니다.
// ‘현재 어떠한 이유로 인해 유효하지 않거나, 존재하지 않는 하나의 값’이라는 널이 표현하려고 하는 개념은 여전히 유용
// 널의 문제는 실제 개념에 있기보다, 특정 구현에 있다고 보는 것이 Rust의 관점입니다. 
// Optional<T> 열거형은 T 타입의 값이 있거나 없음을 나타내는 열거형입니다.

// 만약 변수가 그냥 String 타입이라면, 그 안에는 무조건 진짜 문자열이 들어있음이 보장됩니다. (절대 빈값이 아님)
// 약 값이 없을 수도 있는 변수라면 반드시 Option<String>으로 선언해야 합니다.
// 덕분에 "NullPointerException" 같은 런타임 에러가 러스트에서는 원천적으로 발생하지 않습니다.


