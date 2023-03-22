function outputToConsole(text) {
    var para = document.createElement("p");
    var node = document.createTextNode(text);
    para.appendChild(node);
    document.getElementById("console").prepend(para);
    para.scrollIntoView();
}

function httpGet(endpoint_name, variable_context, response_handler) {
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            response_handler(this.responseText);         
        }
    };
    var endpoint = "http://" + location.hostname + ":8000/" + endpoint_name;
    xmlHttp.open("GET", endpoint, true);
    xmlHttp.send(null);
}

function httpPost(endpoint_name, param_name, param_value, variable_context, response_handler) {
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            response_handler(this.responseText);
        }
    };
    var endpoint = "http://" + location.hostname + ":8000/" + endpoint_name;
    xmlHttp.open("POST", endpoint, true);
    xmlHttp.setRequestHeader('Content-type', 'application/json');
    var param_json = "{ \"" + param_name + "\" : \"" + param_value + "\" }";
    xmlHttp.send(param_json);
}

function httpGetTest() {
    var variable_context = "get test: ";
    var response_handler = (response_text) => {
        var json_data = JSON.parse(response_text);
        outputToConsole(variable_context + json_data);
    };
    httpGet("get_test", variable_context, response_handler);
}

function httpPostSearch() {
    var end_point_name = "api/search";
    var variable_context = "post search: ";
    var search_input = document.getElementById("search_input").value;
    outputToConsole(search_input);
    var param_name = "search_input";
    var response_handler = (response_text) => {
        var json_data = JSON.parse(response_text);
        outputToConsole(variable_context + json_data);
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
