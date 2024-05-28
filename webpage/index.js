
// stolen from tsoding :), https://github.com/tsoding/seroost/blob/main/src/index.js

async function search(prompt) {
    console.log("searching");

    const results = document.getElementById("results")
    results.innerHTML = "";

    const response = await fetch("/api/search", {
        method: 'POST',
        headers: {'Content-Type': 'text/plain'},
        body: prompt,
    });

    const json = await response.json();

    for ([path, rank] of json) {
        let item = document.createElement("span");
        item.appendChild(document.createTextNode(path));
        item.appendChild(document.createElement("br"));
        results.appendChild(item);
    }

    if (results.innerHTML == "") {
        results.innerHTML = "No results";
    }
}

let query = document.getElementById("query");
let currentSearch = Promise.resolve()

query.addEventListener("keypress", (e) => {
    if (e.key == "Enter") {
        currentSearch.then(() => search(query.value));
    }
})


