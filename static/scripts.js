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
    const currentStatus = element.innerText.trim(); // Trim to avoid whitespace issues
    const newStatus = currentStatus === "Yes" ? "No" : "Yes"; // Toggle between Yes/No

    fetch("/update", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id: parseInt(jobId), applied: newStatus === "Yes" }) // Send boolean
    })
    .then(response => response.json()) // ✅ Ensure response is parsed as JSON
    .then(data => {
        if (data.success) {
            element.innerText = newStatus; // ✅ Update UI immediately
        } else {
            alert("Failed to update status."); // ❌ This means server returned `success: false`
        }
    })
    .catch(error => {
        console.error("Error updating applied status:", error);
        alert("Failed to update status.");
    });
}

