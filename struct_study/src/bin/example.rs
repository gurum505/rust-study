fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );



    let rect2 = (30,50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect2)
    );



    let rect3 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect3)
    );



    //println!("rect3 is {}", rect3);
    //println!("rect3 is {:?}", rect3); // println debug trait 사용. 디버그용으로 구조체를 출력할 수 있음. 단, 구조체에 #[derive(Debug)] 어트리뷰트를 붙여야 함.
    println!("rect3 is {:#?}", rect3); // pretty print. 디버그용으로 구조체를 출력할 수 있음. 단, 구조체에 #[derive(Debug)] 어트리뷰트를 붙여야 함.
}

// 러스트는 전통적인 의미의 메소드 오버로딩을 직접 지원하지 않습니다.
// generic을 사용하여 함수의 시그니처를 다르게 하거나, trait를 사용하여 다른 타입에 대해 다른 구현을 제공하는 방식으로 오버로딩을 흉내낼 수 있습니다.

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

