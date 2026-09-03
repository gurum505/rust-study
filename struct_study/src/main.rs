fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // String 타입은 move되고 나머지 필드들은 복사됨. 따라서 user1은 더 이상 유효하지 않음.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

}

struct User {
    active: bool,
    username: String, // 예시를 위해 의도적으로 &str 문자열 슬라이스 대신 구조체가 소유권을 갖는 String 타입을 사용
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// 튜플 구조체 : 필드이름 없음
struct Color(i32, i32, i32);

// 유사 유닛 구조체 (unit-like structs)
struct AlwaysEqual;