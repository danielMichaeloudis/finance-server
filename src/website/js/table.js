const transactionRows = document.getElementsByClassName("transaction-row");

for (row of transactionRows) {
    row.onclick = (event) => {
        const container = event.currentTarget.closest(".transaction-container");
        const dropdown = container.querySelector(".dropdown");

        const isOpen = container.classList.toggle("open");

        if (isOpen) {
            dropdown.style.maxHeight = dropdown.scrollHeight + "px";
            dropdown.style.opacity = "1";
        } else {
            dropdown.style.maxHeight = "0";
            dropdown.style.opacity = "0";
        }
    };
}

const addContainer = document.getElementById("add-container");
const addBtn = document.getElementById("add-btn");

addBtn.onmouseover = (event) => {
    event.currentTarget.parentElement.classList.add("open");
};

addContainer.onmouseleave = (event) => {
    event.currentTarget.classList.remove("open");
};

//default date picker to now
document.getElementById("transaction-date").valueAsDate = new Date();
