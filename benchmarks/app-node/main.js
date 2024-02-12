var http = require('http');

http.createServer(function (req, res) {
    res.writeHead(200, {'Content-Type': 'text/plain'});
    res.end('Hello from node!\n');
}).listen(8080, "0.0.0.0");
console.log('Server running at http://0.0.0.0:8080/');

var url = process.argv[2];

if (url === undefined) {
    console.log("Missing callback. Exiting...");
    process.exit()
}

if (url == "ignore") {
    console.log("Ignoring callback. Continuing...");
} else {
    console.log('Calling ready callback: ', url);
    http.get(url, (resp) => {
        console.log('Response', resp.statusCode);
    }).on("error", (err) => {
        console.log('Error: ' + err.message);
    });
}
