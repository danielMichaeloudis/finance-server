const applyFilterBtn = document.getElementById("filter-button");

applyFilterBtn.onclick = () => {
    const startDate = document.getElementById("start-date").value;
    const endDate = document.getElementById("end-date").value;
    const buyer = document.getElementById("buyer").value;
    const tags = document.getElementById("tags").value;
    const bought_for = document.getElementById("bought-for").value;
    const searchParams = {};
    if (startDate != "") searchParams["start"] = startDate;
    if (endDate != "") searchParams["end"] = endDate;
    if (buyer != "") searchParams["buyer"] = buyer.replaceAll(/\s*,\s*/g, ",");
    if (tags != "") searchParams["tags"] = tags.replaceAll(/\s*,\s*/g, ",");
    if (bought_for != "") {
        searchParams["item_bought_for"] = bought_for.replaceAll(
            /\s*,\s*/g,
            ","
        );
    }
    location.assign("/table?" + new URLSearchParams(searchParams).toString());
};

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

const filtersContainer = document.getElementById("filters-header");

filtersContainer.onclick = (event) => {
    const container = event.currentTarget.closest("#filters");
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

const addContainer = document.getElementById("add-container");
const addBtn = document.getElementById("add-btn");

addBtn.onmouseover = (event) => {
    event.currentTarget.parentElement.classList.add("open");
};

addContainer.onmouseleave = (event) => {
    event.currentTarget.classList.remove("open");
};

//default date picker to now
document.getElementById("date").valueAsDate = new Date();
