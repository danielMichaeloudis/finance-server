const applyFilterBtn = document.getElementById("filter-button");

applyFilterBtn.onclick = () => {
    const startDate = document.getElementById("filter-start-date").value;
    const endDate = document.getElementById("filter-end-date").value;
    const buyer = document.getElementById("filter-buyer").value;
    const tags = document.getElementById("filter-tags").value;
    const bought_for = document.getElementById("filter-bought-for").value;
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
    location.assign(
        location.pathname + "?" + new URLSearchParams(searchParams).toString()
    );
};

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
