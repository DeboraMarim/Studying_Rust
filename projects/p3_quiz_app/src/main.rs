struct Question {
    text: String,
    answer: String,
}

fn main() {
    let quiz = initialize_quiz();

    println!("Welcome to the Quiz App!");

    let mut score = 0;
    for question in &quiz {
        println!("{}", question.text);

        let mut user_answer = String::new();
        std::io::stdin().read_line(&mut user_answer).expect("Failed to read line");

        if user_answer.trim() == question.answer {
            println!("Correct!\n");
            score += 1;
        } else {
            println!("Incorrect. The correct answer is: {}\n", question.answer);
        }
    }

    println!("Quiz completed! Your score: {}/{}", score, quiz.len());
}

fn initialize_quiz() -> Vec<Question> {
    vec![
        Question {
            text: String::from("What is the capital of France?"),
            answer: String::from("Paris"),
        },
        Question {
            text: String::from("Which planet is known as the Red Planet?"),
            answer: String::from("Mars"),
        },
        // Add more questions and answers here
    ]
}
