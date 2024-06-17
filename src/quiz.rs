use xml::writer::{EmitterConfig, XmlEvent};
use std::fs::File;
use crate::question::Question;
use std::fmt;

#[derive(Debug)]
pub enum QuizError{
    WriterError(xml::writer::Error),
    EmptyError(String)
}
impl From<xml::writer::Error> for QuizError{
    fn from(e: xml::writer::Error) -> Self {
        QuizError::WriterError(e)
    }
}
impl From<EmptyError> for QuizError{
    fn from(e: EmptyError) -> Self {
        QuizError::EmptyError(e.to_string())
    }
}

#[derive(Debug)]
pub struct EmptyError;

impl fmt::Display for EmptyError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Quiz questions or answer is empty")
    }
}

pub struct Quiz{
    category: String,
    path: Option<String>,
    questions: Vec<Question>
}
impl Quiz{
    /// Creates a new quiz instance with the specified moodle category and an optional path.
    /// If a path is provided, it is stored in the `path` field.
    /// If no path is specified, the `path` field is set to `None`.
    /// Initializes an empty vector for the `questions` field.
    pub fn new(new_category: String, new_path: Option<String>) -> Self {
        Self { 
            category: new_category,
            path: new_path,
            questions: Vec::new() }
    }
    /// Adds a new question to the quiz.
    ///
    /// # Arguments
    ///
    /// * `question` - The `Question` instance to be added to the quiz.
    ///
    pub fn add_question(&mut self, question: Question){
        let new_question = Question{
            name: question.name,
            description: question.description,
            question_type: question.question_type,
            answers: question.answers
        };
        self.questions.push(new_question);
    }

    pub fn add_questions(&mut self, questions: Vec<Question>){
        for question in questions{
          let new_question = Question{
            name: question.name,
            description: question.description,
            question_type: question.question_type,
            answers: question.answers
            };
            self.questions.push(new_question);  
        }
        
    }
    /// Creates an XML file from quiz object, containing question and answer objects.
    ///
    /// # Arguments
    ///
    /// - `filepath`: The path where the XML file will be saved. Use empty string to store in same folder.
    /// - `filename`: The name of the XML file.
    ///
    /// # Errors
    ///
    /// Returns an QuizError which can be of type
    /// WriterError - `xml::writer::Error` if there's an issue writing the XML data.
    /// EmptyError - if question or answers are empty. 
    pub fn quiz_xml(&mut self, filepath: String, filename: String) -> Result<(), QuizError>{
        // Specify the file and its path
        let mut file_path = filepath; 
        let file = filename;
        file_path.push_str(&file);
        let output: File = File::create(file_path).expect("Bad file path");

        let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(output);

        writer.write(XmlEvent::start_element("quiz"))?;
        writer.write(XmlEvent::start_element("question").attr("type", "category"))?;
        writer.write(XmlEvent::start_element("category"))?;
        writer.write(XmlEvent::start_element("text"))?;
        if self.path.is_none(){
            let string = "$course$/".to_owned();
            let categorypath: &str = self.category.as_str(); 
            let together = string + categorypath;
            writer.write(XmlEvent::characters(together.as_str()))?;
        } else {
            let string = "$course$/".to_owned();
            let categorypath: &str = self.category.as_str();
            let cpath = self.path.as_mut().expect("not run");
            let coursepath = cpath.as_str();
            let together = string + coursepath + categorypath;
            writer.write(XmlEvent::characters(together.as_str()))?;
        }
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        if self.questions.is_empty(){
            return Err(EmptyError.into())
        }
        for question in &mut self.questions{
            question.question_xml(&mut writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

