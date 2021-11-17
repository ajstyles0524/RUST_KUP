use log::*;
mod complex;
mod student_score;
fn main() {
    env_logger::init();
    let complex_1 = complex::Complex {
        real: 2.0,
        img: 3.0,
    };
    let complex_2 = complex::Complex {
        real: 3.0,
        img: 2.0,
    };
    complex::addition(&complex_1, &complex_2);
    info!("addition of two complex number is performed");
    complex::subtraction(&complex_1, &complex_2);
    info!("subtraction of two complex number is performed");
    complex::multiplication(&complex_1, &complex_2);
    info!("multiplication of two complex number is performed");
    // Student-1
    let student_1 = student_score::Student::new(
        "Prem".to_string(),
        77,
        student_score::Score {
            hindi: 31.5,
            english: 32.5,
            maths: 33.5,
            science: 34.0,
        },
        "Civil".to_string(),
        "JRS".to_string(),
    );
    // Student-2
    let student_2 = student_score::Student::new(
        "Prem".to_string(),
        77,
        student_score::Score {
            hindi: 32.5,
            english: 25.5,
            maths: 25.5,
            science: 30.0,
        },
        "Civil".to_string(),
        "JRS".to_string(),
    );
    // To get average of all scores.
    debug!(
        "average of all scores is {}",
         student_score::Score {
            hindi: 31.5,
            english: 32.5,
            maths: 33.5,
            science: 34.0,
        }
            .get_average()
    );
    info!("average of all score is displayed");
    // Add numbers to student’s subject score if score is less than 35.
    student_score::Score {
        hindi: 31.5,
        english: 32.5,
        maths: 33.5,
        science: 34.0,
    }
        .pass_student();
    // Print difference of each subject’s score.
    student_score::Student::compare_student(&student_1, &student_2);
    info!("difference between of each subject's score is displayed");
}

