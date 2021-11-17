use log::*;
/// Score Structure
pub struct Score {
    pub hindi: f32,
    pub english: f32,
    pub maths: f32,
    pub science: f32,
}
/// Student Structure
pub struct Student {
    pub name: String,
    pub roll_no: i32,
    pub scoreofeachsubject: Score,
    pub department: String,
    pub school: String,
}

impl Student {
    /// The new method initializes Student objects
    ///
    /// #Arguments
    ///
    /// Student Structure
    ///
    /// #Return
    ///
    /// Return the Student type objects
    pub fn new(
        string: String,
        roll: i32,
        score_subject: Score,
        department_name: String,
        school_name: String,
    ) -> Student {
        Student {
            name: string,
            roll_no: roll,
            scoreofeachsubject: score_subject,
            department: department_name,
            school: school_name,
        }
    }
    /// compare_student method is to print difference of each subject's score b/w students
    ///
    /// #Arguments
    ///
    /// Student Structure
    ///
    /// #Return
    ///
    /// No Return
    pub fn compare_student(&self, another_student: &Student) {
        debug!(
            "Difference of Hindi score is {}",
            self.scoreofeachsubject.hindi - another_student.scoreofeachsubject.hindi
        );
        debug!(
            "Difference of English score is {}",
            self.scoreofeachsubject.english - another_student.scoreofeachsubject.english
        );
        debug!(
            "Difference of Maths score is {}",
            self.scoreofeachsubject.maths - another_student.scoreofeachsubject.maths
        );
        debug!(
            "Difference of Science score is {}",
            self.scoreofeachsubject.science - another_student.scoreofeachsubject.science
        );
    }
}

impl Score {
    /// get_average method is to get average of all scores
    ///
    /// #Arguments
    ///
    /// Score Structure
    ///
    /// #Return
    ///
    /// Return the average of marks
    pub fn get_average(&self) -> f32 {
        info!("average of marks of student is returned");
        (self.hindi + self.english + self.maths + self.science) / 4.0

    }
    /// pass_student method add numbers to student's score if score < 35
    ///
    /// #Arguments
    ///
    /// Score Structure
    ///
    /// #Return
    ///
    /// No return
   pub fn pass_student(&mut self) {
        let passing_score = 35.0;
        let mut less_score: f32 = passing_score - self.hindi;
        if less_score > 0.0 {
            self.hindi += less_score;
            debug!(
                "Updated Hindi score is {} and less score is {}",
                self.hindi, less_score
            );
        }
        less_score = passing_score - self.english;
        if less_score > 0.0 {
            self.english  += less_score;
            debug!(
                "Updated English score is {} and less score is {}",
                self.english, less_score
            );
        }
        less_score = passing_score - self.maths;
        if less_score > 0.0 {
            self.maths += less_score;
            debug!(
                "Updated Maths score is {} and less score is {}",
                self.maths, less_score
            );
        }
        less_score = passing_score - self.science;
        if less_score > 0.0 {
            self.science  +=  less_score;
            debug!(
                "Updated Science score is {} and less score is {}",
                self.science, less_score
            );
        }
    }
}