# TODO 
1. a load balancer in L3/L4 which forwards the requests to the backend servers. this should use tcp/udp ports. this should handle http2 connections
2. a TLS on load balancer that encrypts the traffic between client and load balancer
3. set up or configure multiple instances of gprc
4. an algorithm for the load balancer to choose or balance the load between these instances
5. an algorithm to check the health of gprc health

6. a process to keep the connection alive