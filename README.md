# jobrunner
a job runner client and server within limited cpu numbers

# install
`cargo install sjqjobrunner`

# usage:
- in server: `sjqjobrunner [-m max_cpu_num] [-b server_addr:port]`
- in client: `sjqjobsender [-s server_addr:port] jobs..`
- in client query the current status:`sjqjobnum [-s server_addr:port]`
the server addr will be 0.0.0.0:5233, in client, it will connect to 127.0.0.1:5233 by default

# example
- in server: `sjqjobrunner -m 12`
- in client: `sjqjobsender "echo hello;sleep 10;echo done;" "echo job2;sleep 20;echo done;"`
