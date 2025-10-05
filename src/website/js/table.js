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

const addTransactionBtn = document.getElementById("add-transaction-btn");

addTransactionBtn.onclick = () => {
    fetch("/components/add_single_transaction")
        .then((res) => res.text())
        .then((html) => {
            addHTML(html, "transaction");
            //default date picker to now
            document.getElementById("transaction-date").valueAsDate =
                new Date();
            document.getElementById("close-transaction").onclick = () => {
                var els = document.getElementsByClassName(
                    "added-by-add-transaction-btn"
                );
                console.log(els);

                while (els[0]) {
                    els[0].parentNode.removeChild(els[0]);
                }
            };
            document
                .getElementById("add-single-transaction")
                .classList.add("shown");
        });
};

function editTransaction(event) {
    event.preventDefault();
    const row = event.currentTarget.closest(".transaction-row");
    fetch("/components/edit_transaction", {
        method: "POST",
        body: JSON.stringify(row.getElementsByClassName("uuid")[0].innerText),
        headers: {
            authorization: "Bearer " + getCookie("token"),
            "Content-Type": "application/json",
        },
    })
        .then((res) => res.text())
        .then((html) => {
            addHTML(html, "transaction");
            document.getElementById("close-transaction").onclick = () => {
                var els = document.getElementsByClassName(
                    "added-by-transaction"
                );
                console.log(els);

                while (els[0]) {
                    els[0].parentNode.removeChild(els[0]);
                }
            };
        });
}
