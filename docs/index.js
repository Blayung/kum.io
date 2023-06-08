var canvasContext;

var serverAddress;
var serverName;

var sessionToken;
var nick;

function throwError(message) {
    alert("Error: "+message);
    location.reload(true); // Delete the "true" in prod
}

function keepAliveLoop() {
    $.post(serverAddress + "/keep_alive", nick+","+sessionToken, function(data, status) {
        if (status != "success") {
            throwError("Couldn't connect to server.");
        }
    }).fail(function(error, status) {
        throwError("Couldn't connect to server.");
    });
}

function mainLoop() {
    canvasContext.fillStyle = "#ff0000";
    canvasContext.fillRect(0, 0, 35, 35);
}

$(document).ready(function() {
    do {
        serverAddress=prompt("Enter the IP of the server you want to connect to.", "193.107.8.49:8888");
    } while (serverAddress==null);

    // It's going to be https in prod
    serverAddress="http://"+serverAddress;

    document.getElementById("header").innerHTML = "CONNECTING...";
    document.getElementById("loading_circle").classList = ["loading_circle"];
    
    $.get(serverAddress+"/server_name", function(data, status) {
        if (status == "success" && typeof data === "string") {
            serverName=data;

            document.getElementById("header").innerHTML = "KUM.IO";
            document.getElementById("loading_circle").classList = [];

            do {
                nick=prompt("Enter your nick."); 
            } while (nick==null);

            $.post(serverAddress + "/register", nick, function(data2, status2) {
                if (status2 == "success") {
                    sessionToken=data2;
                } else {
                    throwError("Couldn't connect to server.");
                }
            }).fail(function(error, status) {
                throwError("Couldn't connect to server.");
            });

            let canvas = document.getElementById("main_canvas");
            canvas.classList.add("canvas_loaded");
            canvasContext = canvas.getContext("2d");

            setInterval(mainLoop, 32);
            setInterval(keepAliveLoop, 15000);
        } else {
            throwError("Couldn't connect to server.");
        }
    }).fail(function(error, status) {
        throwError("Couldn't connect to server.");
    });
});
