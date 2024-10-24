## Overview

Quick/small/dirty hack for running an echo server on all the ports possible. Hopefully useful as a diagnostic tool in certain cases.

Warning: this will likely need a higher `ulimit -n` than default.

## Why the name ?

I like puns and because this tool calls listen() a lot I googled what animal has the best hearing.

## Usage

used interactively, will print out each port as it attempts to bind to it including success/failure

e.g.

```
<snip>
binding to port: 1022 unsuccessfully :(
binding to port: 1023 unsuccessfully :(
binding to port: 1024 successfully!
binding to port: 1025 successfully!
<snip>
```

in addition to functioning as an echo server, prints a message about which port you've connected to, both on the server stdout and to the client.

e.g. on stdout of the server itself:

```
<snip>
binding to port: 65535 successfully!
accepted a connection to 1024
<snip>
```

and the client:

```
% echo foo | nc localhost 1024
welcome to port 1024
foo
```

## TODO

As one of the use-cases is running in a container, future work is to build this statically and have a scratch container running it and pushing that somewhere nice.
I.e, this is likely going to be my inspiration to dive into Github Actions and GHCR.

