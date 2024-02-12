from http.server import BaseHTTPRequestHandler, HTTPServer
import http.client
import time
import sys
from urllib.parse import urlparse

hostName = "0.0.0.0"
serverPort = 8080

def callback(url):
    u = urlparse(url)
    print("Calling ready callback ", url)
    connection = http.client.HTTPConnection(u.netloc)
    connection.request("GET", u.path)
    response = connection.getresponse()
    print("Status: {}".format(response.status))
    connection.close()

class Server(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-type", "text/text")
        self.end_headers()
        self.wfile.write(bytes("Hello from python", "utf-8"))

if __name__ == "__main__":        
    # webServer = HTTPServer((hostName, serverPort), Server)
    print("Server listening on %s:%s" % (hostName, serverPort))
    callback(sys.argv[1])
    try:
        # webServer.serve_forever()
        pass
    except KeyboardInterrupt:
        pass

    # webServer.server_close()
    print("Server stopped.")
