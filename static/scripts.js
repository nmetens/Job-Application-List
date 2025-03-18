// AJAX scripting for the clicking of buttons:
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

    // Get references to elements for the Modal:
    const addJobBtn = document.getElementById("addJobBtn");
    const removeJobBtn = document.getElementById("removeJobBtn");
    const modalOverlay = document.getElementById("modalOverlay");
    const closeModalBtn = document.getElementById("closeModalBtn");
    const addJobForm = document.getElementById("addJobForm");
    const removeJobForm = document.getElementById("removeJobForm");

    // Show the Add Job form in the modal:
    addJobBtn.addEventListener("click", () => {
        modalOverlay.style.display = "flex";
        addJobForm.style.display = "block";
        removeJobForm.style.display = "none"; // Hide remove job form again.
    });

    // Show the Remove Job form in the modal:
    removeJobBtn.addEventListener("click", () => {
        modalOverlay.style.display = "flex";
        addJobForm.style.display = "none";
        removeJobForm.style.display = "block";
    });

    // Close the modal when the close button is clicked:
    closeModalBtn.addEventListener("click", () => {
        modalOverlay.style.display = "none";
    });

    // Close the modal if the background is clicked:
    modalOverlay.addEventListener("click", (event) => {
        if (event.target === modalOverlay) {
            modalOverlay.style.display = "none";
        }
    });
});

// Method to change row color when a job has been applied to or not:
function toggleAppliedStatus(element) {
    const jobId = element.dataset.jobId;
    const currentStatus = element.textContent.trim(); // Get current text (Yes/No).
    const newStatus = currentStatus === "Yes" ? "No" : "Yes"; // Toggle between Yes/No on click.

    // Update the UI immediately:
    element.textContent = newStatus;

    // Get the parent row and change the color based on the status:
    const row = element.closest('tr');
    if (newStatus === "Yes") { // Green color for Yes.
        row.classList.add('highlight-green');
        row.classList.remove('highlight-red');
    } else { // Red color for No.
        row.classList.add('highlight-red');
        row.classList.remove('highlight-green');
    }

    // Send an AJAX request to update the database:
    fetch("/update", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id: parseInt(jobId), applied: newStatus === "Yes" }) // Send boolean caught in rust.
    })
    .then(response => response.json()) // Ensure response is parsed as JSON.
    .then(data => {
        if (data.success) {
            console.log("Status updated successfully on the server.");
        } else {
            console.log("Status updated unsuccessfully on the server.");
        }
    })
}
