function outputToConsole(text) {
    let para = document.createElement("p");
    let node = document.createTextNode(text);
    para.appendChild(node);
    document.getElementById("console").prepend(para);
    para.scrollIntoView();
}

function tableOutputToConsole(json_data, row_header) {
    let datensaetze = json_data;
    let tbl = document.createElement("table");
    let tblBody = document.createElement("tbody");
    let tblheader = document.createElement("thead");
    let tblh_tr = document.createElement("tr");

    for (let x = 0; x < row_header.length; x++) {
        const row_header_text = document.createTextNode(row_header[x]);
        let tblh_element = document.createElement("th");
        tblh_element.appendChild(row_header_text);
        tblh_tr.appendChild(tblh_element);
    }

    tblheader.appendChild(tblh_tr);
    tbl.appendChild(tblheader);

    for(let i = 0; i < datensaetze.length; i++) {
        const row = document.createElement("tr");

        for (let j = 0; j < datensaetze[i].length; j++) {
            const cell = document.createElement("td");
            const cellText = document.createTextNode(datensaetze[i][j] );
            cell.appendChild(cellText);
            row.appendChild(cell);
        }
        tblBody.appendChild(row);
    }
    tbl.appendChild(tblBody);
    tbl.setAttribute("border", "2");
    tbl.setAttribute("id","output_table");
    document.getElementById("console").prepend(tbl);
}


function httpGet(endpoint_name, variable_context, response_handler) {
    let xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            response_handler(this.responseText);         
        }
    };
    let endpoint = "http://" + location.hostname + ":8000/" + endpoint_name;
    xmlHttp.open("GET", endpoint, true);
    xmlHttp.send(null);
}

function httpPost(endpoint_name, param_name, param_value, variable_context, response_handler) {
    let xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            response_handler(this.responseText);
        }
    };
    let endpoint = "http://" + location.hostname + ":8000/" + endpoint_name;
    xmlHttp.open("POST", endpoint, true);
    xmlHttp.setRequestHeader('Content-type', 'application/json');
    let param_json = "{ \"" + param_name + "\" : \"" + param_value + "\" }";
    xmlHttp.send(param_json);
}

function httpGetTest() {
    let variable_context = "get test: ";
    let response_handler = (response_text) => {
        let json_data = JSON.parse(response_text);
        outputToConsole(variable_context + json_data);
    };
    httpGet("get_test", variable_context, response_handler);
}

function httpPostSearch() {
    let end_point_name = "api/search";
    let variable_context = "post search: ";
    let search_input = document.getElementById("search_input").value;
    outputToConsole(search_input);
    let param_name = "search_input";
    let response_handler = (response_text) => {
        let table_header = ["path", "rank"];
        let json_data = JSON.parse(response_text);
        outputToConsole(variable_context);
        tableOutputToConsole(json_data, table_header);
    };
    httpPost(end_point_name, param_name, search_input, variable_context, response_handler);
}


function clearConsole() {
    document.getElementById("console").innerHTML = "";
}

document.getElementById("clear_button").onclick = function() {
    clearConsole();
};

document.getElementById("get_test_button").onclick = function() {
    httpGetTest();
};

document.getElementById("search_button").onclick = function() {
    httpPostSearch();
};
