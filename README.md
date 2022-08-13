# YAK - yet another kafka 

Yet another Kafka is a personal undertaking to replace a push-based architecture
in a software I use with a pull-based architecture. The motivation for the project is
as follows: 
1. Push-based architecture causes queueing on the overall system, which ultimately 
halts the software
2. Disk can be sacrificed in cases where memory queues can become expensive.
3. Dedicated disk I/O with sequential files will be faster than random disk seeks when
queues are flushed as pages to disk (random) 

The architecture works as follows: 
1. YAK server is a single broker that listens to messages, and responds to message requests.
2. Each message has the following - a key (defaults to timestamp when not supplied)
3. The key is used to maintain an in-memory B-Tree 
4. Consumers can request for a range of keys to consume. The messages are buffered in memory 
for some time, after which they are flushed. The message sending happens with `sendfile` api.
5. After clients "ack" a message key range, the messages are flushed from memory. 
6. Any un-flushed messages are stored in the memory and can be replayed using a different script. 
7. Each message is associated with a Topic. If the topic does not exist, then it is created.
8. Each topic is associated with a LSM Tree in memory. 
s
