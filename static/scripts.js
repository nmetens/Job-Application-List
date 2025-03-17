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

function toggleAppliedStatus(element) {
    const jobId = element.dataset.jobId;
    const currentStatus = element.textContent.trim(); // Get current text (Yes/No)
    const newStatus = currentStatus === "Yes" ? "No" : "Yes"; // Toggle between Yes/No

    // Update the UI immediately
    element.textContent = newStatus;

    // Get the parent row and change the color based on the status
    const row = element.closest('tr');
    if (newStatus === "Yes") {
        row.classList.add('highlight-green');
        row.classList.remove('highlight-red');
    } else {
        row.classList.add('highlight-red');
        row.classList.remove('highlight-green');
    }

    // Send an AJAX request to update the database
    fetch("/update", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id: parseInt(jobId), applied: newStatus === "Yes" }) // Send boolean
    })
    .then(response => response.json()) // Ensure response is parsed as JSON
    .then(data => {
        if (data.success) {
            console.log("Status updated successfully on the server.");
        } else {
            alert("Failed to update status.");
            element.textContent = currentStatus; // Revert if the update fails
            // Revert row color if needed
            if (currentStatus === "Yes") {
                row.classList.add('highlight-green');
                row.classList.remove('highlight-red');
            } else {
                row.classList.add('highlight-red');
                row.classList.remove('highlight-green');
            }
        }
    })
    .catch(error => {
        console.error("Error updating applied status:", error);
        alert("Failed to update status.");
        element.textContent = currentStatus; // Revert if there's an error
        // Revert row color if needed
        if (currentStatus === "Yes") {
            row.classList.add('highlight-green');
            row.classList.remove('highlight-red');
        } else {
            row.classList.add('highlight-red');
            row.classList.remove('highlight-green');
        }
    });
}

