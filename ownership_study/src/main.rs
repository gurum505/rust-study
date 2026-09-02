//https://doc.rust-kr.org/ch04-01-what-is-ownership.html



fn main() {
    // 스택 과 힙 : runtime시 이용하게 될 메모리 영역.
    // 힙(Heap) 자체가 무언가를 반환한다기보다는, 메모리 할당자(Allocator)가 힙 영역에 공간을 할당한 뒤 그 시작 주소(포인터)를 반환.
    // ownership의 목표는 힙 데이터의 관리 : : 힙의 어떤 데이터를 사용하는 지 추적하고 중복되는 데이터 최소화, 쓰지않는 데이터 청소 =>ownership의 역할
    // 메모리 내부를 이리저리 왔다갔다하는 작업이 적을수록 빨라짐
    // 힙 영역처럼 서로 떨어져있다면 느리고, 스택처럼 데이터가 붙어잇다면 빠르다.

    // 소유권 규칙
    // 1. Rust의 모든 값은 오직 하나의 owner를 가진다.
    // 2. owner가 스코프를 벗어나면 값은 drop되어 메모리에서 해제된다.
    // 3. 값의 소유권은 여럿 존재할 수 없다. 단, 참조자를 통해서만 가능하다.

    // 참조자 규칙
    // 1. 여러분은 단 하나의 가변 참조자만 갖거나, 여러 개의 불변 참조자를 가질 수 있습니다.
    // 2. 참조자는 항상 유효해야 합니다.


    let _s = "hello"; // 하드코딩된 리터럴. 불변
    // 힙에 저장된다는 건, 알 수 없는 크기의 데이터를 저장할 수 있다는 의미. 힙은 런타임에 크기가 결정됨.
    let _s = String::from("hello"); // String은 힙에 저장됨 // String 은 가변적이므로, 런타임에 크기가 결정됨. // String외 다른 복잡한 데이터 형식도 비슷

    let x = 5; // 스택에 저장됨. 정적 크기이므로, 런타임에 크기가 결정되지 않음.
    let y = x; // 스택에 저장된 데이터는 복사됨. 즉, x와 y는 독립적인 값이 됨.
    println!("x: {}, y: {}", x, y);



    let _s1 = String::from("hello"); 
    // String 변수 자체(메타데이터)는 스택(Stack)에 저장되는데, 내부에 다음 세 가지 정보를 가지고 있습니다.
    // 1. 포인터(pointer): 힙 영역에 저장된 실제 문자열 데이터를 가리킴
    // 2. 길이(length): 문자열의 길이
    // 3. 용량(capacity): 문자열이 저장될 수 있는 총 용량
    // 즉, "스택에 있는 포인터가 힙에 있는 실제 데이터의 주소를 가리키고 있다"
    let _s2 = _s1; 
    // _s2와 _s1은 같은 힙 영역의 데이터를 가리키게 됨. 
    // 즉, let _s2 = _s1; 라인 뒤로는 _s1이 더 이상 유효하지 않다고 판단합니다.
    // _s1의 소유권이 _s2로 이동됨. 이제 _s1은 더 이상 유효하지 않음.
    // println!("_s1: {}", _s1); // error: value borrowed here after move
    println!("_s2: {}", _s2);



    // move vs shallow copy : 기존변수를 무효화하기 때문에 rust에서는 move라고 표현.
    // 러스트는 절대 자동으로 ‘깊은’ 복사로 데이터를 복사하는 일이 없습니다. 따라서, 러스트가 자동으로 수행하는 모든 복사는 런타임 성능 측면에서 효율적이라 할 수 있습니다.
    // 깊은 복사는 clone()으로만 가능.
    // 일반적으로 단순한 스칼라 값의 묶음은 Copy 가능하고, 할당이 필요하거나 리소스의 일종인 경우엔 불가능합니다. 
    


    // 함수로 값을 전달하는 메커니즘은 변수에 값을 넣을 때와 같습니다. 즉, 스택에 저장된 값은 복사되고, 힙에 저장된 값은 소유권이 이동합니다.
    // 이동이나 복사가 이루어짐
    // 함수에 전달된 값은 함수가 끝나면 drop되어 메모리에서 해제됨.
    takes_ownership(_s2); // _s2의 소유권이 함수로 이동됨. 이제 _s2는 더 이상 유효하지 않음.
    makes_copy(x); // x는 스택에 저장된 값이므로, 복사됨. 이제 x는 여전히 유효함.



    let _s4 = give_ownership(); // give_ownership() 함수가 반환한 String의 소유권이 _s4로 이동됨.
    let _s5 = String::from("hello"); // _s5가 스코프에 들어옴
    let _s6 = takes_and_gives_back(_s5); // _s5의 소유권이 함수로 이동됨. 이제 _s5는 더 이상 유효하지 않음.



    // 그럼, 함수가 값을 사용할 수 있도록 하되 소유권은 가져가지 않도록 하고 싶다면 어떻게 해야 할까요?
    // 참조자를 만드는 행위를 대여 (borrow) 라고 합니다. 
    let _s7 = String::from("hello"); 
    let len = calculate_length(&_s7); 
    // &_s7는 _s7에 대한 포인터를 전달하는 것과 같습니다. 
    // _s7의 소유권을 빼앗지 않고, 참조자(&)를 전달함. 이제 _s7은 여전히 유효함.
    println!("The length of '{}' is {}.", _s7, len);

    // 참조자도 마찬가지로 참조하는 것을 수정할 수 없습니다. caculate_length() 함수에서 _s7의 길이를 계산할 수는 있지만, _s7을 수정할 수는 없습니다.
    // 그럼 참조를 해서 수정할 수 없다가 아니라 애초에 기본적으로 immutable해서 수정할 수 없다가 맞다.
    let mut _s8 = String::from("hello");
    change(&mut _s8); // 가변 참조자를 전달함. 이제 _s8을 수정할 수 있음.
    
    // 가변 참조자는 한 번에 하나만 존재할 수 있습니다.
    // &mut _s8를 통해 _s8을 가변 참조하고 있는 동안에는, 다른 가변 참조자나 불변 참조자를 만들 수 없습니다.
    // 이미 가변으로 독점 대여 중. 다른 데이터가 중간에 수정되면 안됨.
    // 가변 참조자(&mut)가 살아있는 동안에는 그 변수를 읽는 것(& 대여)조차 금지
    
    //let _s9 = &mut _s8; 
    // error: cannot borrow `_s8` as mutable more than once at a time
    //cannot borrow `_s8` as immutable because it is also borrowed as mutable 

    // 가변 참조자와 불변 참조자를 혼용할때도 마찬가지.
    // 사용중 값이 도중에 바뀌면 안되므로.

    // 주의: 실제로 _s9를 사용하지 않으면, _s9가 컴파일러에 의해 바로 drop되므로, _s8를 다시 가변 참조할 수 있습니다.
    // 러스트의 NLL스코프 : 사실상 자신이 마지막 쓰일때까지가 scope. 그 이후에서야 drop됨. 
    println!("_s8: {}", _s8);
    //println!("_s9: {}", _s9);



}

fn takes_ownership(some_string: String) { // some_string이 스코프에 들어옴
    println!("{}", some_string);
} // some_string이 스코프를 벗어나고, drop됨. 메모리 해제

fn makes_copy(some_integer: i32) { // some_integer가 스코프에 들어옴
    println!("{}", some_integer);
} // some_integer가 스코프를 벗어나고, drop됨. 하지만 i32는 Copy이므로, 메모리 해제되지 않음.

fn give_ownership() -> String { // give_ownership() 함수가 반환한 String의 소유권이 호출한 곳으로 이동됨.
    let some_string = String::from("hello"); // some_string이 스코프에 들어옴
    some_string // some_string이 반환되고, 소유권이 호출한 곳으로 이동됨.
} // some_string이 스코프를 벗어나고, drop되지 않음. 이미 소유권이 이동했기 때문.

fn takes_and_gives_back(some_string: String) -> String { // some_string이 스코프에 들어옴
    some_string // some_string이 반환되고, 소유권이 호출한 곳으로 이동됨.
} // some_string이 스코프를 벗어나고, drop되지 않음. 이미 소유권이 이동했기 때문.

fn calculate_length(s: &String) -> usize { // s는 String에 대한 참조자(&)이므로, 소유권을 가져가지 않음.
    s.len() // 참조자를 통해 String의 길이를 반환함.
} // s가 스코프를 벗어나도, 소유권을 가져가지 않았으므로 drop되지 않음.

fn change(some_string: &mut String) { // some_string은 String에 대한 가변 참조자(&mut)이므로, 소유권을 가져가지 않음.
    some_string.push_str(", world"); // 가변 참조자를 통해 String을 수정함.
} // some_string이 스코프를 벗어나도, 소유권을 가져가지 않았으므로 drop되지 않음.
