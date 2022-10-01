fn main() {
    let mut numbers = vec![1, 3, 5, 7];

    numbers.push(9);
    numbers.push(12);

    println!("{:?}", numbers); // [1, 3, 5, 7, 9, 12]

    numbers.pop();
    println!("{:?}", numbers.len()); // 5

    for number in numbers {
        println!("{:?}", number);
    }


    struct Test {
        score: i32,
    }

    let my_scores = vec![
        Test {score: 95},
        Test {score: 43},
        Test {score: 86},
        Test {score: 79},
    ];

    for my_score in my_scores {
        println!("{:?}", my_score.score);
    }
}