import http.server, ssl, socketserver, sys # for the https server
#from pyautogui import*

from pynput.keyboard import Controller,Key
keyboard = Controller()


# HTTPS SERVER
DIRECTORY = sys.argv[1]
context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
context.load_cert_chain(certfile='certificate.pem', keyfile='private_key.pem')
server_address = ("", 4443) # CHANGE THIS IP & PORT




class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def do_GET(self):
        if "/?v" in self.path:
            x,y,z = [int(float(v)*100) for v in self.path.replace("/?v=","").split(",")]
            #print("x: ",x,"   y : ",y,"   z : ",z)
            if z>30000:
                keyboard.press(Key.right)
                keyboard.release(Key.left)  
                
            elif 0<z<3000:
                keyboard.press(Key.left) 
                keyboard.release(Key.right)

            else:
                keyboard.release(Key.left)  
                keyboard.release(Key.right)

            if y>12:
                keyboard.press('z')
                keyboard.release('s')  
            elif y<-30:
                keyboard.press('s')  
                keyboard.release('z')  
            else:
                keyboard.release('z')  
                keyboard.release('s')

        else:
            #print("GET request,\nPath: %s\nHeaders:\n%s\n", str(self.path), str(self.headers))
            super().do_GET()
            #print("GET request for {}".format(self.path).encode('utf-8'))
    def end_headers(self):
        # Add the Content-Security-Policy header with the upgrade-insecure-requests directive
        self.send_header('Content-Security-Policy', "default-src *; style-src 'self' 'unsafe-inline'; script-src * 'unsafe-inline' 'unsafe-eval'")
        super().end_headers()

http_server = socketserver.TCPServer(server_address, Handler)
http_server.socket = context.wrap_socket(http_server.socket, server_side=True)

http_server.serve_forever()