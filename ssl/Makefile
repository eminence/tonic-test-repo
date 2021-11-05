ca.key:
	openssl genrsa -out ca.key 4096
ca.crt: ca.key
	openssl req -new -x509 -key ca.key -out ca.crt -days 365 -subj '/C=US/ST=Rhode Island/L=Providence/O=tonic-test'

server.key:
	openssl genrsa -out server.key 4096
server.csr: server.key server.conf
	openssl req -new -key server.key -out server.csr -config server.conf
server.crt: ca.crt ca.key server.csr server.conf
	openssl x509 -req -days 365 -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -extfile server.conf -extensions my_extensions


bundle: server.crt ca.crt
	cat server.crt ca.crt > server.bundle.crt

all: bundle

clean:
	rm -f server.bundle.crt client.bundle.crt server.crt server.csr client.csr server.key client.key ca.key ca.crt