/** This is the jobs file that contains
  * the struct for a job and its implementation.
  */
// Each job will have an id,
// a job_title, an hourly rate,
// and whether the user has applied
// to the job or not.
//#[derive(Copy, Clone)] // Allows copies of struct objects
#[derive(Clone)]
pub struct Job {
    //_job_id: u32,
    title: String,
    hourly: f32,
    applied: u32,
}

impl Job {
    // Constructor:
    //pub fn new(job_id: u32, title: String, hourly: f32) -> Self {
    pub fn new(title: String, hourly: f32, applied: u32) -> Self {
        Self {
            //job_id,
            title,
            hourly,
            applied
        } // Return self
    }

    // Display job info:
    pub fn _display(&self) -> String {
        //println!("Job ID: {}. Title: {}. Pay: {}. Applied: {}", 
        //self.job_id, self.title, self.hourly, self.applied);
        //format!("Job ID: {0}. Title: {1}. Pay: {2}. Applied: {3}", 
        format!("Title: {0}. Pay: {1}. Applied: {2}", 
        //self.job_id, self.title, self.hourly, self.applied).to_string() // Return the String
        self.title, self.hourly, self.applied).to_string() // Return the String
    }

    // Update application status:
    pub fn _applied(&mut self,  applied: u32) {
        self.applied = applied;
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_hourly(&self) -> f32 {
        self.hourly.clone()
    }
    
    pub fn get_applied(&self) -> u32 {
        self.applied.clone()
    }
}
