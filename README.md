# YAK - "Yet another Kafka"-like message broker
YAK aims to be a fully-functional clone of the first version of Kafka presented in the paper. This project is just for learning and not meant to be deployed in production. 

Outlined kafka paper description lives in [this blog post](https://uniform-lima.pages.dev/paper_notes/kafka-paper-notes/). Feel free to peruse it. 

### Tasks / Todos:
- [ ] Initialize respective Go Repositories for each project. 
- [ ] Initialize dockerfiles and docker-compose for each project.
- [ ] Broker components: 
    - [ ] Bootstrap - join with etcd cluster for registering self
    - [ ] Ingest data over HTTP 
    - [ ] Segment file storage
    - [ ] In-memory index for segment file and offsets
- [ ] Consumers component:
    - [ ] Boostrap - join eith etcd cluster for registering self
    - [ ] Scan etcd directory for available brokers, ownership information and offset information