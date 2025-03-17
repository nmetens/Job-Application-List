$(document).ready(function() {
    // Click event for the "Add a Job" button
    $('#addJobBtn').click(function() {
        // Hide the "Remove a Job" form if it is visible
        $('#removeJobForm').hide();
        
        // Toggle the visibility of the "Add a Job" form
        $('#addJobForm').toggle();
    });

    // Click event for the "Remove a Job" button
    $('#removeJobBtn').click(function() {
        // Hide the "Add a Job" form if it is visible
        $('#addJobForm').hide();
        
        // Toggle the visibility of the "Remove a Job" form
        $('#removeJobForm').toggle();
    });
});

