Map loading by UUID

Client asks all peers for load 
All peers respond if they can

Match # of peers respond
0 Peers respond. Wait some # of time and attempt to reload.
1 Peer responds. Respond with request to load map with UUID.
N Peers respond. Respond to one with  