/* Insert a new job into the jobs table in the database: */
/* INSERT INTO jobs (job_title, hourly_rate, applied) VALUES ('Software Engineer', 60, 1); */

/* Delete a job from the jobs table in the database: */
/*DELETE FROM jobs WHERE job_title = 'Waiter';*/

/* Display all the jobs applied to in the jobs table: */
/*SELECT * FROM jobs WHERE applied = 1;*/

/* Display all the jobs not applied to: */
/*SELECT * FROM jobs WHERE applied = 0;*/

/* Get the highest paying job in the table: */
/*SELECT * FROM jobs ORDER BY hourly_rate DESC LIMIT 1;*/

/* Count the total number of jobs in the table: */
/*SELECT COUNT(*) FROM jobs;*/

/* Count the number of jobs applied to: */
/*SELECT COUNT(*) FROM jobs WHERE applied = 1;*/

/* List all the jobs with an hourly rate greater than $20: */
/*SELECT * FROM jobs WHERE hourly_rate > 20;*/

/* Sort the jobs by hourly rate from highest to lowest: */
SELECT * FROM jobs ORDER BY hourly_rate DESC;
