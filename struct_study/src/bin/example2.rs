fn main(){
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );



    let square = Rectangle::square(30);
    println!(
        "The area of the square is {} square pixels.",
        square.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 메서드 (method) : 구조체에 연관된 함수. 첫 번째 매개변수로 self를 받음. self는 구조체 인스턴스 자신을 의미.
// 메서드는 다른 매개변수가 그런 것처럼 self의 소유권을 가져올 수도, 지금처럼 self를 불변으로 빌려올 수도, 가변으로 빌려올 수도 있습니
// self를 선택한 이유는 데이터를 읽는 것뿐이니까요
impl Rectangle {
    //연관 함수 (associated function)라 부른다. impl 뒤에 나오는 타입과 모두 연관된 함수이기 때문
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // new는 이 언어에서 특별한 이름 혹은 키워드가 아닙니다.
    // 동작하는 데 해당 타입의 인스턴스가 필요하지 않다면 self를 첫 매개변수로 갖지 않는 (따라서 메서드가 아닌) 연관 함수를 정의할 수도 있습니다
    fn square(size: u32) -> Self { // Self 키워드는 impl 키워드 뒤에 적혀있는 타입의 별칭으로서, 여기서는 Rectangle이 되겠습니다.
        Self { width: size, height: size }
    }
}



// 러스트에는 자동 참조 및 역참조 (automatic referencing and dereferencing) 라는 기능이 있고, 메서드 호출에 이 기능이 포함
// 자동 참조 동작 : *p1을 안 쓰는 이유: p1이 일반 변수면 *를 못 쓰고, 참조자여도 러스트가 알아서 처리해주기 때문입니다(Autoderef).
// p1.distance(&p2);
// (&p1).distance(&p2);
// 

