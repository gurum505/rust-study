fn dangle() -> &String { // dangling 함수는 String에 대한 참조자를 반환하려고 함.
    let s = String::from("hello"); // s가 스코프에 들어옴
    &s // s에 대한 참조자를 반환함. 하지만 s는 이 함수가 끝나면 drop됨.
} // s가 스코프를 벗어나고, drop됨. 이제 dangling 함수는 유효하지 않은 참조자를 반환하게 됨.

fn no_dangle() -> String { // dangling 함수는 String에 대한 참조자를 반환하려고 함.
    let s = String::from("hello"); // s가 스코프에 들어옴
    s // s를 반환함. 이제 dangling 함수는 유효한 String을 반환하게 됨.
} // s가 스코프를 벗어나고, drop되지 않음. 이미 소유권이 이동했기 때문.